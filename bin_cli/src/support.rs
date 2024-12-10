//! Support files, less connected to crates core logic.

pub mod error;
pub mod subscriber;

use error::CliErrorWrapper;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = CliErrorWrapper;
