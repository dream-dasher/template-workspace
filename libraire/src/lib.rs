//! Example Library in an Example Workspace

use tracing;

/// Prints "Hi!" to screen.
#[tracing::instrument]
pub fn say_hi() {
        println!("Hi!");
}

/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `libraire` crate:
///
/// ```
/// let result = libraire::add_example(2, 3);
/// assert_eq!(result, 5);
/// ```
#[tracing::instrument]
pub fn add_example(a: u32, b: u32) -> u64 {
        a as u64 + b as u64
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
///
/// The next function divides two numbers.
///
/// # Examples
///
/// ```
/// let result = libraire::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// libraire::div(10, 0);
/// ```
#[tracing::instrument]
pub fn div(a: u64, b: u64) -> u64 {
        if b == 0 {
                panic!("Divide-by-zero error");
        }

        a / b
}

/// Repeats an immutable function multiple times
///
/// Function does not directly take inputs.
/// Currying?
#[tracing::instrument(skip(f))]
pub fn repeat_function<F>(f: F, times: u8)
        where F: Fn() {
        for _ in 0..times {
                f();
        }
}

/// Repeats a potentially mutable function multiple times
///
/// Function does not directly take inputs.
/// Currying?
#[tracing::instrument(skip(m))]
pub fn repeat_function_mutable<F>(mut m: F, times: u8)
        where F: FnMut() {
        for _ in 0..times {
                m();
        }
}

#[cfg(test)]
mod tests {
        use quickcheck::{self, TestResult};
        use quickcheck_macros::quickcheck;
        use test_log::test;

        use super::*;

        #[test]
        fn it_works() {
                let result = add_example(2, 2);
                assert_eq!(result, 4);
        }

        /// Proptest example using discards to get subset of inputs.
        #[quickcheck]
        fn prop_plus_x(a: u32, b: u32) -> bool {
                (a as u64 + b as u64) == add_example(a, b)
        }
}
