//! Support code for package: **{{ project-name | kebab_case }}**

mod error;
mod subscriber;

pub use error::ErrWrapper{{ project-name | upper_camel_case }};
pub use subscriber::active_global_default_tracing_subscriber;

pub type Result<T> = std::result::Result<T, ErrWrapper{{ project-name | upper_camel_case }}>;
pub type Error = ErrWrapper{{ project-name | upper_camel_case }};
