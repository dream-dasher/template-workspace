//! Error Handling for libraire

use std::backtrace::Backtrace;

use derive_more::{Display, From};

// EARLY_DEV: non-specific error & result types for use while exploring new code.
pub type Result<T> = core::result::Result<T, Error>;
// pub type Error = Box<dyn std::error::Error>;

// Nightly requires enabling this feature (at crate root):
// #![feature(error_generic_member_access)]

// std::error::Error requires std::fmt::Debug and std::fmt::Display,
// so we can also use derive_more::Display for fully declarative
// error-type definitions.

// derive_more::From fits nicely into this pattern as well
#[derive(Debug, Display, derive_more::Error, From)]
pub enum Error {
        Simple,
        WithSource {
                source: Simple,
        },
        #[from(ignore)]
        WithBacktraceFromSource {
                #[error(backtrace)]
                source: Simple,
        },
        #[display("{source}")]
        WithDifferentBacktrace {
                source:    Simple,
                backtrace: Backtrace,
        },
        WithExplicitSource {
                #[error(source)]
                explicit_source: WithSource,
        },
        #[from(ignore)]
        WithBacktraceFromExplicitSource {
                #[error(backtrace, source)]
                explicit_source: WithSource,
        },
        Tuple(WithExplicitSource),
        WithoutSource(#[error(not(source))] Tuple),
}

#[derive(Default, Debug, Display, derive_more::Error)]
pub struct Simple;

#[derive(Default, Debug, Display, derive_more::Error)]
pub struct WithSource {
        source: Simple,
}
#[derive(Default, Debug, Display, derive_more::Error)]
pub struct WithExplicitSource {
        #[error(source)]
        explicit_source: Simple,
}

#[derive(Default, Debug, Display, derive_more::Error)]
pub struct Tuple(Simple);

#[derive(Default, Debug, Display, derive_more::Error)]
pub struct WithoutSource(#[error(not(source))] i32);

#[derive(Debug, Display, derive_more::Error)]
#[display("An error with a backtrace")]
pub struct WithSourceAndBacktrace {
        source:    Simple,
        backtrace: Backtrace,
}
