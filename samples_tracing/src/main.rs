#![feature(error_generic_member_access)]
//! An Example Binary (CLI app) using Example Library in Example Workspace
//!

mod error {
        pub type Result<T> = core::result::Result<T, Error>;
        pub type Error = Box<dyn std::error::Error>;
}
mod support_tracing {
        use tracing_subscriber::EnvFilter;

        pub fn tracing_subscribe_boilerplate(env_min: impl Into<String>) {
                let filter = EnvFilter::try_new(
                    std::env::var("RUST_LOG").unwrap_or_else(|_| env_min.into()),
                )
                .expect("Valid filter input provided.");

                tracing_subscriber::fmt().pretty()
                                         .with_env_filter(filter)
                                         .with_file(true)
                                         .with_line_number(true)
                                         .with_thread_ids(true)
                                         .with_target(true)
                                         .init();
        }
}
use clap::Parser;
use error::Result;

fn main() -> Result<()> {
        support_tracing::tracing_subscribe_boilerplate("warn");
        let args = Args::parse();
        println!("Script 1, running...");
        Ok(())
}

/// CLI Args
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
        /// Ints, xp.
        #[arg(short, long)]
        repeats: u8,
}
