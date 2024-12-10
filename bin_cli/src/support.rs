//! Support files, less connected to crates core logic.

mod error;
mod subscriber;

use error::CliErrorWrapper;
pub use subscriber::{generate_tracing_subscriber, tracing_subscribe_boilerplate};

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = CliErrorWrapper;
