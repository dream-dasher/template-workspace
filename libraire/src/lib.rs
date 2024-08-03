//! Example Library in an Example Workspace

/// Adds two numbers together.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Prints "Hi!" to screen.
pub fn say_hi() {
    println!("Hi!");
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
