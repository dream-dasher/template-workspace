//! # Error & Result type for package: **{{ project-name | kebab_case }}**
//!
//! ## General Strategies
//! - Enum for errors that may appear
//!   - `Error` & `From` auto-derived, allowing `?` based conversion of errors
//!   - 'Garbage' `OtherDynError` type for development and exploration purposes
//!     - redundant `ToOther` trait & `ErrKind::make_other_error(_)` to help grab these errors in the dyn category
//! - Struct Wrapper that adds a tracing SpanTrace and can accomodate a backtrace (later is auto-derivable)
//!   - *custom* `From` for the Wrapper, so that `?` conversion will generate a spantrace or otherwise populate additional values.
//!   - *custom* `Debug` for the Wrapper, so that the SpanTrace is auto-printed (using the Display value).
//!
//!
//! ## Utility reference
//! For adding backtrace to errors:
//! `#![feature(error_generic_member_access)]`
//! `use std::backtrace;`

use std::io;

use derive_more::{Display, Error, From};
use tracing::subscriber::SetGlobalDefaultError; // !

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, From, Error)]
pub enum ErrKind{{ project-name | upper_camel_case }} {
        #[display("io error: {}", source)]
        Io { source: io::Error },
        #[display("Error setting tracing subscriber default: {}", source)]
        TracingSubscriber { source: SetGlobalDefaultError },
        #[from(ignore)]
        #[display("Unlabelled error (dyn error object): {}", source)]
        OtherDynError {
                source: Box<dyn std::error::Error + Send + Sync>,
        },
        // #[display("Error extracting lines from input: {}", source_input)]
        // NoInputLines { source_input: String },
        // #[from(ignore)]
        // #[display("error parsing char: {}", uninterpretable_char)]
        // CharParse { uninterpretable_char: char },
        // #[display("parse error: {}", source)]
        // ParseInt { source: num::ParseIntError },
        // #[display("env variable error: {}", source)]
        // Env { source: env::VarError },
}
impl ErrKind{{ project-name | upper_camel_case }} {
        pub fn make_other_error<E>(error: E) -> Self
        where
                E: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
                Self::OtherDynError { source: error.into() }
        }
}

#[derive(Display, Error, From)]
#[display(
        "error: {:#}\n\n\nspantrace capture: {:?}\n\n\nspantrace: {:#}",
        source,
        spantrace.status(),
        spantrace,
)]
pub struct ErrWrapper{{ project-name | upper_camel_case }} {
        source:    ErrKind{{ project-name | upper_camel_case }},
        spantrace: tracing_error::SpanTrace,
        // backtrace: backtrace::Backtrace,
}
impl<T> From<T> for ErrWrapper{{ project-name | upper_camel_case }}
where
        T: Into<ErrKind{{ project-name | upper_camel_case }}>,
{
        fn from(error: T) -> Self {
                Self {
                        source:    error.into(),
                        spantrace: tracing_error::SpanTrace::capture(),
                        // backtrace: backtrace::Backtrace::capture(),
                }
        }
}
// Using custom display as debug so we can get SpanTrace auto printed.
impl std::fmt::Debug for ErrWrapper{{ project-name | upper_camel_case }} {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
        }
}

#[expect(dead_code)]
trait ToOther {
        fn to_other(self) -> ErrWrapper{{ project-name | upper_camel_case }};
}
impl<E> ToOther for E
where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
        fn to_other(self) -> ErrWrapper{{ project-name | upper_camel_case }} {
                ErrKind{{ project-name | upper_camel_case }}::OtherDynError { source: self.into() }.into()
        }
}
