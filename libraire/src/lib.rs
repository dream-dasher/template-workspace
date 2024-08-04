//! Example Library in an Example Workspace

/// Prints "Hi!" to screen.
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
/// let result = libraire::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: u64, b: u64) -> u64 {
    a + b
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
pub fn repeat_function<F>(f: F, times: u8)
where
    F: Fn(),
{
    for _ in 0..times {
        f();
    }
}

/// Repeats a potentially mutable function multiple times
///
/// Function does not directly take inputs.
/// Currying?
pub fn repeat_function_mutable<F>(mut m: F, times: u8)
where
    F: FnMut(),
{
    for _ in 0..times {
        m();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
