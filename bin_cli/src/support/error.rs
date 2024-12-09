//! Error & Result type.
//!
//! To add backtraces see:
//! #![feature(error_generic_member_access)]
//! use std::backtrace;

use std::io;

use derive_more::{Display, Error as DMError, From};
use tracing::subscriber::SetGlobalDefaultError;

pub type Result<T> = std::result::Result<T, ErrorKind>;
pub type Error = ErrWrapper;

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, derive_more::Error, From)]
pub enum ErrorKind {
        // #[display("parse error: {}", source)]
        // ParseError { source: num::ParseIntError },
        // #[display("env variable error: {}", source)]
        // EnvError { source: env::VarError },
        // #[display("Error setting tracing subscriber default: {}", source)]
        // TracingSubscriber { source: SetGlobalDefaultError },
        // #[display("io error: {}", source)]
        // Io { source: io::Error },
        #[from(ignore)]
        #[display("Unlabelled error (dyn error object): {}", source)]
        OtherDynError {
                source: Box<dyn std::error::Error + Send + Sync>,
        },
}
impl ErrorKind {
        pub fn make_other_error<E>(error: E) -> Self
        where
                E: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
                Self::OtherDynError { source: error.into() }
        }
}

#[derive(Debug, Display, DMError, From)]
#[display(
        "error: {:#}\n\n\nspantrace capture: {:?}\n\n\nspantrace: {:#}",
        source,
        spantrace.status(),
        spantrace,
)]
pub struct ErrWrapper {
        source:    ErrorKind,
        spantrace: tracing_error::SpanTrace,
        // backtrace: backtrace::Backtrace,
}
impl<T> From<T> for ErrWrapper
where
        T: Into<ErrorKind>,
{
        fn from(error: T) -> Self {
                Self {
                        source:    error.into(),
                        spantrace: tracing_error::SpanTrace::capture(),
                        // backtrace: backtrace::Backtrace::capture(),
                }
        }
}

#[expect(dead_code)]
trait ToOther {
        fn to_other(self) -> ErrWrapper;
}
impl<E> ToOther for E
where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
        fn to_other(self) -> ErrWrapper {
                ErrorKind::OtherDynError { source: self.into() }.into()
        }
}
