//! Organizes and Exposes code internally & externally
//!
//! Notice that `tracing_boilerplate` is **not** present here.
//! That would belong to binary code.  It *could* exist here as support for example files.

#![feature(error_generic_member_access)]

// Expose specific items
mod error;
pub use error::{Error, Result};

// Flatten
mod func_repetition;
pub use func_repetition::*;

// Expose module
pub mod arithmetic;
pub mod utility;
