//! Support code for package: **xp-ratatui**

mod error;
mod subscriber;

pub use error::ErrWrapperXpRatatui;
pub use subscriber::active_global_default_tracing_subscriber;

pub type Result<T> = std::result::Result<T, ErrWrapperXpRatatui>;
pub type Error = ErrWrapperXpRatatui;
