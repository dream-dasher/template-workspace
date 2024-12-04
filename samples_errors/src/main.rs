#![feature(error_generic_member_access)]
//! Itertools
//!
//! ## Docs
//! you likely want to start at:
//! [trait Itertools::Itertools](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.tuple_windows)
//!
//!
//! clear; RUST_LOG=samples_itertools=trace carrbn samples_itertools

use std::{backtrace, num};

use derive_more::{Display, Error, derive::From};
// use thiserror::Error;

// #[derive(Debug, Error)]
// #[error("spanner error: {source}")]
// struct Spanner {
//         #[from]
//         source: MyError,
//         // spantrace: tracing_error::SpanTrace,
// }
// // use thiserror::Error;
// #[derive(Debug, Error)]
// pub enum MyError {
//         #[error("parse error: {source}")]
//         ParseError {
//                 #[from]
//                 #[backtrace]
//                 source: std::num::ParseIntError,
//                 // spantrace: tracing_error::SpanTrace,
//                 // backtrace: std::backtrace::Backtrace,
//         },
// }

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, Error, From)]
pub enum MyError {
        #[display("parse error: {}", source)]
        // #[from(forward)]
        #[from]
        ParseError {
                source: num::ParseIntError,
                // spantrace: tracing_error::SpanTrace,
                // #[from(ignore)]
                // backtrace: backtrace::Backtrace,
        },
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
        source:    MyError,
        spantrace: tracing_error::SpanTrace,
        backtrace: backtrace::Backtrace,
}
impl<T> From<T> for MyErrorWrapper
where
        T: Into<MyError>,
{
        fn from(error: T) -> Self {
                Self {
                        source:    error.into(),
                        spantrace: tracing_error::SpanTrace::capture(),
                        backtrace: backtrace::Backtrace::capture(),
                }
        }
}

use itertools::{Itertools, iproduct};
use tracing::{Level, debug, error, info, info_span, level_filters::LevelFilter, span, trace, warn};
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, prelude::*};

#[tracing::instrument]
fn make_error(say: &str) -> Result<(), MyErrorWrapper> {
        println!("say: {}", say);
        let x: u32 = "12".parse()?;
        let y: u32 = "-12".parse()?;
        let y: u32 = "zzz".parse()?;
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

        let _enter = span!(Level::INFO, "main").entered();
        let _enter = span!(Level::INFO, "main2").entered();
        match make_error("hi") {
                Ok(_) => info!("no error"),
                Err(e) => error!("error: {:#}", e),
                // Ok(_) => println!("no error"),
                // Err(e) => println!("error: {:#}", e),
        }
        {
                let _enter = span!(Level::INFO, "---special scope---").entered();
                let y: u32 = "-12".parse()?;
                let _enter = span!(Level::INFO, "---special scope222222---").entered();
        }
        let _enter = span!(Level::INFO, "end!!! (shouldn't see?)").entered();

        Ok(())
}
