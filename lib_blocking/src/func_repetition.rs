//! Function running functions.

/// Repeats an immutable function multiple times
///
/// Function does not directly take inputs.
/// Currying?
#[tracing::instrument(skip(f))]
pub fn repeat_function<F>(f: F, times: u8)
where F: Fn()
{
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
where F: FnMut()
{
        for _ in 0..times {
                m();
        }
}

#[cfg(test)]
mod tests
{}
