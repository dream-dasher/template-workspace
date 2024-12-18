//! # Error & Result type for package: **xp-ratatui**
//!
//! ## General Strategies
//! - Enum for errors that may appear
//!   - `Error` & `From` auto-derived, allowing `?` based conversion of errors
//!   - 'Garbage' `OtherDynError` type for development and exploration purposes
//!     - redundant `ToOther` trait & `ErrKind::make_other_error(_)` to help grab these errors in the dyn category
//! - Struct Wrapper that adds a tracing SpanTrace and can accommodate a backtrace (later is auto-derivable)
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
pub enum ErrKindXpRatatui {
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
impl ErrKindXpRatatui {
        pub fn make_other_error<E>(error: E) -> Self
        where
                E: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
                Self::OtherDynError {
                        source: error.into(),
                }
        }
}

#[derive(Display, Error, From)]
#[display(
        "error: {:#}\n\n\nspantrace capture: {:?}\n\n\nspantrace: {:#}",
        source,
        spantrace.status(),
        spantrace,
)]
pub struct ErrWrapperXpRatatui {
        source:    ErrKindXpRatatui,
        spantrace: tracing_error::SpanTrace,
        // backtrace: backtrace::Backtrace,
}
impl<T> From<T> for ErrWrapperXpRatatui
where
        T: Into<ErrKindXpRatatui>,
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
impl std::fmt::Debug for ErrWrapperXpRatatui {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self) }
}

#[expect(dead_code)]
trait ToOther {
        fn to_other(self) -> ErrWrapperXpRatatui;
}
impl<E> ToOther for E
where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
        fn to_other(self) -> ErrWrapperXpRatatui {
                ErrKindXpRatatui::OtherDynError {
                        source: self.into(),
                }
                .into()
        }
}
