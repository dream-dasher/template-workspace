//! Type Conversion
//!
//! `as` (!): xu32 as yi32
//!
//! `from()`: i32::from(xi16)
//! `into()`: let new: i32 = x16.into()
//!
//! `try_from()`: i32::from(xi16)?
//! `try_into()`: let new: i32 = x16.into()?
//!
//! clear; RUST_LOG=samples_conversion=trace carrbn samples_conversion

use std::convert::{TryFrom, TryInto};

use tracing::{Level, debug, error, info, info_span, span, trace, warn};

type Error = Box<dyn core::error::Error>;
type Result<T> = core::result::Result<T, Error>;

// #[unsafe(no_mangle)]
pub fn square(num: i32) -> i32
{
        num * num
}

// #[unsafe(no_mangle)]
pub fn mult_same(i1: i32, i2: i32) -> i32
{
        i1 * i2
}
// #[unsafe(no_mangle)]
pub fn mult_as(i1: i32, u: u32) -> i32
{
        let i2 = u as i32;
        i1 * i2
}

// #[unsafe(no_mangle)]
pub fn mult_into_easy(i1: i32, ismall: i16) -> i32
{
        let i2: i32 = ismall.into();
        i1 * i2
}

// #[unsafe(no_mangle)]
pub fn mult_from_easy(i1: i32, ismall: i16) -> i32
{
        let i2 = i32::from(ismall);
        i1 * i2
}

// #[unsafe(no_mangle)]
pub fn mult_into(i1: i32, u: u32) -> i32
{
        let i2: i32 = u.try_into().unwrap();
        i1 * i2
}

// #[unsafe(no_mangle)]
pub fn mult_from(i1: i32, u: u32) -> i32
{
        let i2 = i32::try_from(u).unwrap();
        i1 * i2
}

// #[unsafe(no_mangle)]
pub fn mult_into_res(i1: i32, u: u32) -> Result<i32>
{
        let i2: i32 = u.try_into()?;
        Ok(i1 * i2)
}

// #[unsafe(no_mangle)]
pub fn mult_from_res(i1: i32, u: u32) -> Result<i32>
{
        let i2 = i32::try_from(u)?;
        Ok(i1 * i2)
}

fn main()
{
        tracing_subscriber::fmt::init();
}
