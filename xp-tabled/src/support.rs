//! Support code for package: **xp-tabled**

mod error;
mod subscriber;

pub use error::ErrWrapperXpTabled;
pub use subscriber::active_global_default_tracing_subscriber;

pub type Result<T> = std::result::Result<T, ErrWrapperXpTabled>;
pub type Error = ErrWrapperXpTabled;
