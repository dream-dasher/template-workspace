//! # Type Conversion
//!
//! ## Conversion Kinds:
//! ### Raw
//! `as` (!): xu32 as yi32
//!
//! ### Restricted (compiletime)
//! `from()`: i32::from(xi16)
//! `into()`: let new: i32 = x16.into()
//!
//! ### Checked (runtime)
//! `try_from()`: i32::try_from(xi16)?
//! `try_into()`: let new: i32 = x16.try_into()?
//!
//! ## GodBolt
//! target: `aarch64-apple-darwin`
//! compiler: nightly (>1.83)
//! edition: 2024
//!
//! clear; RUST_LOG=samples_conversion=trace carrbn samples_conversion

use std::{convert::{TryFrom, TryInto},
          num::TryFromIntError};

/// ```assembly
/// _square:
///         mul     w0, w0, w0
///         ret
/// ```
// #[unsafe(no_mangle)]
pub fn square(num: i32) -> i32 {
        num * num
}

/// ```assembly
/// _mult_same:
///         mul     w0, w1, w0
///         ret
/// ```
// #[unsafe(no_mangle)]
pub fn mult_same(i1: i32, i2: i32) -> i32 {
        i1 * i2
}

/// ```assembly
/// _mult_as:
///         mul     w0, w1, w0
///         ret
/// ```
// #[unsafe(no_mangle)]
pub fn mult_as(i1: i32, u: u32) -> i32 {
        let i2 = u as i32;
        i1 * i2
}

/// ```assembly
/// _mult_into_easy:
///         sxth    w8, w1
///         mul     w0, w8, w0
///         ret
/// ```
// #[unsafe(no_mangle)]
pub fn mult_into_easy(i1: i32, ismall: i16) -> i32 {
        let i2: i32 = ismall.into();
        i1 * i2
}

/// ```assembly
/// _mult_from_easy:
///         sxth    w8, w1
///         mul     w0, w8, w0
///         ret
/// ```
// #[unsafe(no_mangle)]
pub fn mult_from_easy(i1: i32, ismall: i16) -> i32 {
        let i2 = i32::from(ismall);
        i1 * i2
}

/// ```assembly
/// _mult_into_panic:
///         tbnz    w1, #31, LBB9_2
/// -->
/// _mult_into_panic:@55
///         mul     w0, w1, w0
///         ret
/// ```
// #[unsafe(no_mangle)]
pub fn mult_into_panic(i1: i32, u: u32) -> i32 {
        let i2: i32 = u.try_into().unwrap();
        i1 * i2
}

/// ```assembly
/// _mult_from_panic:
///        tbnz    w1, #31, LBB10_2
/// -->
/// _mult_from_panic:@72
///         mul     w0, w1, w0
///         ret
/// ```
// #[unsafe(no_mangle)]
pub fn mult_from_panic(i1: i32, u: u32) -> i32 {
        let i2 = i32::try_from(u).unwrap();
        i1 * i2
}

/// ```assembly
/// _mult_from_res_direct:
///        mul     w8, w1, w0
///        lsr     w0, w1, #31
///        mov     x1, x8
///        ret
/// ```
// #[unsafe(no_mangle)]
pub fn mult_into_res_direct(i1: i32, u: u32) -> core::result::Result<i32, TryFromIntError> {
        let i2: i32 = u.try_into()?;
        Ok(i1 * i2)
}

/// ```assembly
/// _mult_from_res_direct:
///         mul     w8, w1, w0
///         lsr     w0, w1, #31
///         mov     x1, x8
///         ret
/// ```
// #[unsafe(no_mangle)]
pub fn mult_from_res_direct(i1: i32, u: u32) -> core::result::Result<i32, TryFromIntError> {
        let i2 = i32::try_from(u)?;
        Ok(i1 * i2)
}

/// ```assembly
/// _mult_into_res_convert:
///         tbnz    w1, #31, LBB6_2
/// -->
/// _mult_into_res_convert:@45
///         mul     w10, w1, w0
///         str     w10, [x8, #8]
///         str     xzr, [x8]
///         ret
/// ```
// #[unsafe(no_mangle)]
pub fn mult_into_res_convert(
        i1: i32,
        u: u32,
) -> core::result::Result<i32, Box<dyn core::error::Error>> {
        let i2: i32 = u.try_into()?;
        Ok(i1 * i2)
}

/// ```assembly
/// _mult_from_res_convert:
///         tbnz    w1, #31, LBB11_2
/// -->
/// _mult_from_res_convert:@89
///         mul     w10, w1, w0
///         str     w10, [x8, #8]
///         str     xzr, [x8]
///         ret
/// ```
// #[unsafe(no_mangle)]
pub fn mult_from_res_convert(
        i1: i32,
        u: u32,
) -> core::result::Result<i32, Box<dyn core::error::Error>> {
        let i2 = i32::try_from(u)?;
        Ok(i1 * i2)
}

fn main() {
        tracing_subscriber::fmt::init();
}
