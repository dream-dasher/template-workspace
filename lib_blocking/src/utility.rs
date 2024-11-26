//! Utility functions for the project.

/// Prints "Hi!" to screen.
/// Trivial Fn() function
#[tracing::instrument]
pub fn say_hi()
{
        tracing::info!("Saying hi!");
        println!("Hi!");
}
