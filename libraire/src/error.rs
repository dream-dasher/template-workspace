//! Error Handling for libraire

// EARLY_DEV: non-specific error & result types for use while exploring new code.
pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;
