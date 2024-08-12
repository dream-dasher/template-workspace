//! Arithmetic with various range and overflow considerations.

use crate::error::{Error, Result};

/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `lib_blocking` crate:
///
/// ```
/// use lib_blocking::arithmetic::add_ample_room;
///
/// let result = add_ample_room(u32::MAX, u32::MAX);
/// assert_eq!(result, 2*u32::MAX as u64);
/// ```
#[tracing::instrument]
pub fn add_ample_room(a: u32, b: u32) -> u64 {
        a as u64 + b as u64
}

/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `lib_blocking` crate:
///
/// ```
/// use lib_blocking::arithmetic::add_can_overflow;
///
/// let result = add_can_overflow(u32::MAX, 1);
/// assert_eq!(result, 0);
/// ```
#[tracing::instrument]
pub fn add_can_overflow(a: u32, b: u32) -> u32 {
        a.wrapping_add(b)
}

/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `lib_blocking` crate:
///
/// ```
/// use lib_blocking::arithmetic::add_can_err;
///
/// let result = add_can_err(u32::MAX, 1);
/// assert!(result.is_err());
/// ```
#[tracing::instrument]
pub fn add_can_err(a: u32, b: u32) -> Result<u32> {
        a.checked_add(b).ok_or(Error::Simple)
}

/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `lib_blocking` crate:
///
/// ```
/// use lib_blocking::arithmetic::mult;
///
/// let result = mult(2, 3);
/// assert_eq!(result, 6);
///
/// let result = mult(u32::MAX, u32::MAX);
/// assert_eq!(result, (u64::MAX - 2u64.pow(33) + 1 + 1));
/// ```
#[tracing::instrument]
pub fn mult(a: u32, b: u32) -> u64 {
        (a as u64) * (b as u64)
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
///
/// The next function divides two numbers.
///
/// # Examples
///
/// ```
/// use lib_blocking::arithmetic::div;
///
/// let result =div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// use lib_blocking::arithmetic::div;
///
/// // panics on division by zero
/// div(10, 0);
/// ```
#[tracing::instrument]
pub fn div(a: u64, b: u64) -> u64 {
        if b == 0 {
                panic!("Divide-by-zero error");
        }

        a / b
}

#[cfg(test)]
mod tests {
        use quickcheck::{self, TestResult};
        use quickcheck_macros::quickcheck;
        use test_log::test;

        use super::*;

        #[test]
        fn spotcheck_add_example() {
                let result = add_ample_room(2, 2);
                assert_eq!(result, 4);
        }

        #[quickcheck]
        fn prop_add_ample_room(a: u32, b: u32) -> bool {
                (a as u64 + b as u64) == add_ample_room(a, b)
        }

        /// Proptest example; matches wrap on full range
        #[quickcheck]
        fn prop_add_can_overflow_full_wrapping_add(a: u32, b: u32) -> TestResult {
                TestResult::from_bool((b.wrapping_add(a)) == add_can_overflow(a, b))
        }

        /// Proptest example; matches regular add on restricted range
        #[quickcheck]
        fn prop_add_can_overflow_restricted_add(a: u32, b: u32) -> TestResult {
                if a > u32::MAX / 2 || b > u32::MAX / 2 {
                        return TestResult::discard();
                }

                TestResult::from_bool((a + b) == add_can_overflow(a, b))
        }
        ///
        #[quickcheck]
        fn prop_mult(a: u32, b: u32) -> bool {
                (a as u64 * b as u64) == mult(a, b)
        }
}
