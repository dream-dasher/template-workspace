//! Support code for package: **cat-ergonomics**

mod error;
mod subscriber;

pub use error::ErrWrapperCatErgonomics;
pub use subscriber::active_global_default_tracing_subscriber;

pub type Result<T> = std::result::Result<T, ErrWrapperCatErgonomics>;
pub type Error = ErrWrapperCatErgonomics;
