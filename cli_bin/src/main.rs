#![feature(error_generic_member_access)]
//! An Example Binary (CLI app) using Example Library in Example Workspace
//!

mod error;
mod support_tracing;

use clap::Parser;
use error::Result;
use libraire_lib::{repeat_function, utility::say_hi};
// use rename_files::{app, error::Result, logging, Args};

fn main() -> Result<()> {
        support_tracing::tracing_subscribe_boilerplate("warn");
        let args = Args::parse();
        println!("Script 1, running...");
        repeat_function(say_hi, args.repeats);
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

#[cfg(test)]
mod tests {

        use test_log::test;

        #[test]
        fn trivially_true_in_practice() {
                assert_eq!(2 + 3, 3 + 2);
        }
}
