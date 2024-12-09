//! Support files, less connected to crates core logic.

mod error;
mod subscriber;

pub use error::{Error, ErrorKind, Result};
pub use subscriber::{generate_tracing_subscriber, tracing_subscribe_boilerplate};
