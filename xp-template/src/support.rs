//! package support code
//!
/// Logging (tracing) related code.
use tracing_subscriber::EnvFilter;

/// Basic boilerplate logging initialization.
///
/// TODO/NOTE: `EnvFilter` provides builtins to do what we're already doing here / :shrug:
pub fn tracing_subscribe_boilerplate(env_min: impl Into<String>)
{
        let filter = EnvFilter::try_new(std::env::var("RUST_LOG").unwrap_or_else(|_| env_min.into()))
                .expect("Valid filter input provided.");

        tracing_subscriber::fmt()
                .pretty()
                .with_env_filter(filter)
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_target(true)
                .init();
}
