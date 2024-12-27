//! Tracing - with Jon Gj.
//!
//! tracing & tracing-subscriber
//!
//! ## Syntax Ref
//! **event**!(<fields>, <message> & **span**!(<message>, <fields>) 🤷
//! Level: TRACE, DEBUG, INFO, WARN, ERROR
//! field_name = var||val : set field
//! ?var : use Debug implementation
//! %var : use Display implementation
//!
//! ## Gotchas
//! span entrance is **Thread LOCAL**
//! span & event macros have dyfferent argument ordering
//! may need: tracing_subscriber::prelude::*;
//! convenience subscriber syntax is often a trap -- avoid -- just creates syntactic confusion
//!
//! ## Special Notes
//! compile time filters: max_level_x && release_max_level_x
//! let _enter = span_level!(..).**entered()**; // single line span create and enter; consumes span
//!
//!
//! clear; RUST_LOG=trace carrbn samples_tracing  a bb ccc dddd

use core::fmt;
use std::path::PathBuf;

use tracing::{self as tea, Level, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, prelude::*};

#[derive(Debug)]
#[expect(dead_code)]
struct Foo {
        a: bool,
        b: u32,
}

fn main() {
        tracing_subscriber::Registry::default()
                // // Problem: tree seems to work poorly when non-monolinear thread
                // .with(tracing_tree::HierarchicalLayer::new(2)
                //         .with_timer(tracing_tree::time::Uptime::default())
                //         .with_span_modes(true)
                //         .with_indent_lines(true))
                .with(EnvFilter::builder()
                        .with_default_directive(LevelFilter::ERROR.into())
                        .from_env_lossy())
                .with(tracing_subscriber::fmt::Layer::default())
                .init();

        let _enter = tea::span!(Level::INFO, "main",).entered();
        let mut handles = vec![];
        tea::info!(args = ?std::env::args(), "about to start file loop");
        for file in std::env::args().skip(1) {
                handles.push(std::thread::spawn(move || on_thread(file)));
        }

        let _enter = tea::span!(Level::INFO, "joining").entered();
        for handle in handles {
                let span = tea::span!(Level::INFO, "indiv_join");
                let _guard = span.enter();
                tea::debug!("joining thread");
                handle.join().expect("thread join error");
                tea::debug!("thread joined");
        }
}

#[tracing::instrument]
fn on_thread<PATH>(file: PATH)
where
        PATH: Into<PathBuf> + std::fmt::Display + fmt::Debug,
{
        let _enter = tea::info_span!("file", fname = %file).entered();
        tea::warn!(parent: None, fname = %file, "opening the file");
        // let mut file = std::fs::File::open(file).unwrap();
        tea::info!("reading file contents");
        // let mut bytes = Vec::new();
        // file.read_exact(&mut bytes).unwrap();
        tea::info!(bytes = 0, "parsing");
        // ..
        #[expect(clippy::disallowed_names)]
        let foo: Foo = Foo { a: false, b: 12 };
        tea::info!(parsed = ?foo, "done with file");
}
