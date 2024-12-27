//! Support code for package: **xp-snapshot**

mod error;
mod subscriber;

pub use error::ErrWrapperXpSnapshot;
pub use subscriber::active_global_default_tracing_subscriber;

pub type Result<T> = std::result::Result<T, ErrWrapperXpSnapshot>;
pub type Error = ErrWrapperXpSnapshot;
