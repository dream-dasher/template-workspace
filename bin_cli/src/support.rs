//! Support files, less connected to crates core logic.

pub mod error;
mod subscriber;

use error::CliErrorWrapper;
pub use subscriber::*;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = CliErrorWrapper;
