//! Error & Result type.
//!
//! To add backtraces see:
//! #![feature(error_generic_member_access)]
//! use std::backtrace;

use derive_more::{Display, Error, From};
use tracing::subscriber::SetGlobalDefaultError;

#[derive(Debug, Display, Error, From)]
#[display(
        "error: {:#}\n\n\nspantrace capture: {:?}\n\n\nspantrace: {:#}",
        source,
        spantrace.status(),
        spantrace,
)]
pub struct CliErrorWrapper {
        source:    CliErrorKind,
        spantrace: tracing_error::SpanTrace,
        // backtrace: backtrace::Backtrace,
}

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, derive_more::Error, From)]
pub enum CliErrorKind {
        // #[display("parse error: {}", source)]
        // ParseError { source: num::ParseIntError },
        // #[display("env variable error: {}", source)]
        // EnvError { source: env::VarError },
        // #[display("Error setting tracing subscriber default: {}", source)]
        // TracingSubscriber { source: SetGlobalDefaultError },
        // #[display("io error: {}", source)]
        // Io { source: io::Error },
        #[display("Error setting tracing subscriber default: {}", source)]
        TracingSubscriber { source: SetGlobalDefaultError },
        #[from(ignore)]
        #[display("Unlabelled error (dyn error object): {}", source)]
        OtherDynError {
                source: Box<dyn std::error::Error + Send + Sync>,
        },
}
impl CliErrorKind {
        /// Redundant with `ToOther(trait)::make_other_dyn_error`
        // #[expect(
        //         clippy::allow_attributes,
        //         reason = "On or Off expect throws a warning. Lint warning or unfulfilled lint expectation warning."
        // )]
        // #[allow(dead_code, reason = "Boiler plate available for use.")]
        #[expect(dead_code, reason = "Boiler plate available for use.")]
        pub fn make_other_dyn_error<E>(error: E) -> Self
        where
                E: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
                Self::OtherDynError { source: error.into() }
        }
}

impl<T> From<T> for CliErrorWrapper
where
        T: Into<CliErrorKind>,
{
        fn from(error: T) -> Self {
                Self {
                        source:    error.into(),
                        spantrace: tracing_error::SpanTrace::capture(),
                        // backtrace: backtrace::Backtrace::capture(),
                }
        }
}

#[expect(unused, reason = "Boiler plate available for use.")]
/// Redundant with `CliErrorKind(error Enum)::make_other_dyn_error`
pub trait ToOther {
        fn to_other(self) -> CliErrorWrapper;
}
impl<E> ToOther for E
where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
        fn to_other(self) -> CliErrorWrapper {
                CliErrorKind::OtherDynError { source: self.into() }.into()
        }
}
