#![feature(error_generic_member_access)]
//!
//! clear; RUST_LOG=samples_errors=trace carrbn samples_errors

use std::{backtrace, env, fs, io, num};

use derive_more::{Display, Error, derive::From}; // !
use tracing::{self as tea, Level, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, prelude::*};

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, Error, From)]
pub enum MyErrorSource {
        #[display("parse error: {}", source)]
        ParseError { source: num::ParseIntError },
        #[display("env variable error: {}", source)]
        EnvError { source: env::VarError },
        #[display("io error: {}", source)]
        IoError { source: io::Error },
        #[display("Uncategorized error: {}", source)]
        #[from(ignore)]
        OtherError {
                source: Box<dyn std::error::Error + Send + Sync>,
        },
}
impl MyErrorSource {
        pub fn make_other_error<E>(error: E) -> Self
        where
                E: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
                Self::OtherError { source: error.into() }
        }
}
#[derive(Debug, Display, Error, From)]
#[display(
        "error: {:#}\n\n\nspantrace capture: {:?}\n\n\nspantrace: {:#}\n\n\nbacktrace: {:#}",
        source,
        spantrace.status(),
        spantrace,
        backtrace
)]
pub struct MyErrorWrapper {
        source:    MyErrorSource,
        spantrace: tracing_error::SpanTrace,
        backtrace: backtrace::Backtrace,
}
impl<T> From<T> for MyErrorWrapper
where
        T: Into<MyErrorSource>,
{
        fn from(error: T) -> Self {
                Self {
                        source:    error.into(),
                        spantrace: tracing_error::SpanTrace::capture(),
                        backtrace: backtrace::Backtrace::capture(),
                }
        }
}

trait ToOther {
        fn to_other(self) -> MyErrorWrapper;
}

impl<E> ToOther for E
where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
        fn to_other(self) -> MyErrorWrapper {
                MyErrorSource::OtherError { source: self.into() }.into()
        }
}
#[tracing::instrument]
fn make_error(ch: char) -> Result<(), MyErrorWrapper> {
        match ch {
                // ParseIntError
                'p' => {
                        let y = "-12".parse::<u32>();
                        y?;
                }
                // io::Error
                'f' => {
                        let f = fs::read_to_string("non-present-file.txt");
                        f?;
                }
                // unstructured that can be converted to a dyn error object
                _ => {
                        let x = "just a random string".to_other();
                        Err(x)?;
                }
        };
        Ok(())
}

fn main() -> Result<(), MyErrorWrapper> {
        tracing_subscriber::Registry::default()
                .with(tracing_tree::HierarchicalLayer::new(2)
                        .with_timer(tracing_tree::time::Uptime::default())
                        .with_span_modes(true)
                        .with_indent_lines(true))
                .with(EnvFilter::builder()
                        .with_default_directive(LevelFilter::DEBUG.into())
                        .from_env_lossy())
                .with(ErrorLayer::default())
                // .with(tracing_subscriber::fmt::Layer::default())
                .init();

        // const A_CHAR: char = 'p';
        // const A_CHAR: char = 'f';
        const A_CHAR: char = 'x';

        let _enter = tea::span!(Level::INFO, "main").entered();
        let _enter = tea::span!(Level::INFO, "main2").entered();
        match make_error(A_CHAR) {
                Ok(_) => tea::info!("no error"),
                Err(e) => tea::error!("error: {:#}", e),
        }
        {
                let _enter = tea::span!(Level::INFO, "---special scope---").entered();
                make_error(A_CHAR)?;
                let _enter = tea::span!(Level::INFO, "---special scope222222---").entered();
        }
        let _enter = tea::span!(Level::INFO, "end!!! (shouldn't see?)").entered();

        Ok(())
}
