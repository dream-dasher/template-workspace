//! Tracing - with Jon Gj.
//!
//! tracing & tracing-subscriber
//!
//! (<fields>, <message>)
//! Level: TRACE, DEBUG, INFO, WARN, ERROR
//! field_name = var||val : set field
//! ?var : use Debug implementation
//! %var : use Display implementation
//!
//! ## Gotchas
//! - span entrance is **Thread LOCAL**
//! - order of fields in span! and event! macros is different (can be remarkably frustrating)
//!
//!
//! compile time filters: max_level_x && release_max_level_x
//!
//! clear; RUST_LOG=trace carrbn samples_tracing  a bb ccc dddd

use core::fmt;
use std::{error::Error,
          io::{self, Read},
          path::PathBuf,
          thread};

#[derive(Debug)]
struct Foo
{
        a: bool,
        b: u32,
}

use tracing::{Level, debug, error, info, info_span, span, trace, warn};
use tracing_error::{ErrorLayer, InstrumentError, InstrumentResult, SpanTrace, TracedError};
use tracing_subscriber::{EnvFilter, prelude::*};
// use tracing_subscriber::{EnvFilter, layer::SubscriberExt};
fn main() -> Result<(), TracedError<io::Error>>
{
        // tracing_subscriber::fmt::init();
        let subscriber =
                tracing_subscriber::Registry::default().with(tracing_subscriber::fmt::layer().with_target(true))
                                                       .with(EnvFilter::from_default_env())
                                                       .with(ErrorLayer::default());
        tracing::subscriber::set_global_default(subscriber);

        let x = 42;
        let y = 13;
        let span = span!(Level::INFO, "main", ?x, ?y);
        let _guard = span.enter();
        {
                let span = span!(Level::INFO, "instrumenting stuff!", ?x, ?y);
                let _guard = span.enter();

                // let read = InstrumentResult::in_current_span(std::fs::read_to_string("foo.txt"));
                // let read = std::fs::read_to_string("foo.txt")?;
                let read = std::fs::read_to_string("foo.txt").expect_err("");
                let read_span = std::fs::read_to_string("foo.txt").in_current_span().expect_err("");
                error!(?read, "read");
                eprintln!("read_span: {}", read_span);
                // NOTE: TracedError doesn't implement anything for Debug or Display.
                //       it just takes inner then error -- pulling out the standard error; no span-trace
                error!(?read_span, "read");
                // let st: SpanTrace = read_span.into();
        }

        let span = span!(Level::INFO, "main",);
        let _guard = span.enter();
        let mut handles = vec![];
        info!(args = ?std::env::args(), "about to start file loop");
        for file in std::env::args().skip(1) {
                handles.push(std::thread::spawn(move || on_thread(file)));
        }

        let span = span!(Level::INFO, "joining");
        let _guard = span.enter();
        for handle in handles {
                let span = span!(Level::INFO, "indiv_join");
                let _guard = span.enter();
                debug!("joining thread");
                handle.join().expect("thread join error");
                debug!("thread joined");
        }
        Ok(())
}

#[tracing::instrument]
fn on_thread<PATH>(file: PATH)
        where PATH: Into<PathBuf>+std::fmt::Display+fmt::Debug
{
        let span = info_span!("file", fname = %file);
        let _guard = span.enter();
        warn!(parent: None, fname = %file, "opening the file");
        // let mut file = std::fs::File::open(file).unwrap();
        info!("reading file contents");
        // let mut bytes = Vec::new();
        // file.read_exact(&mut bytes).unwrap();
        info!(bytes = 0, "parsing");
        // ..
        #[expect(clippy::disallowed_names)]
        let foo: Foo = Foo { a: false, b: 12 };
        info!(parsed = ?foo, "done with file");
}
