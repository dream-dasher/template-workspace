//! # CLI interface for code in package: **{{ project-name | kebab_case }}**
//!
//! ## Strategy
//! <placeholder code>
//! Principally convenience access to code in the `lib.rs`

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use tracing::{self as tea, Level, instrument};

use {{ project-name | snake_case }}::{CUSTOM_INPUT_PATH, Result, active_global_default_tracing_subscriber};

/// Package: **{{ project-name | kebab_case }}**'s convenience CLI interface.
#[derive(Parser, Debug)]
#[command(
        version,
        about,
        long_about,
        disable_help_subcommand = true,
        subcommand_help_heading = "input source"
)]
pub struct Args {
        /// Which Part to Run
        part:  Part,
        /// Input to use.
        #[command(subcommand)]
        input: Option<Input>,
}
#[derive(Debug, Clone, ValueEnum)]
pub enum Part {
        /// Part 1
        #[value(alias = "1", alias = "i", alias = "I", alias = "one")]
        Part1,
        /// Part 2
        #[value(alias = "2", alias = "ii", alias = "II", alias = "two")]
        Part2,
}
#[derive(Debug, Clone, Subcommand)]
pub enum Input {
        /// Use the example input.
        Example,
        /// Use the full problem input.
        Full,
        /// Use a custom input.
        Custom,
        /// Use custom input; please give a path.
        Other { path: PathBuf },
}

#[instrument(skip_all, ret(level = Level::DEBUG))]
fn main() -> Result<()> {
        let _tracing_writer_guard = active_global_default_tracing_subscriber()?;

        let _enter = tea::debug_span!("main()").entered();
        tea::trace!("tracing subscriber set");
        let cli_user_args = Args::parse();
        tea::trace!(?cli_user_args);
        let part = cli_user_args.part;
        let inp = cli_user_args.input.unwrap_or_else(|| {
                tea::warn!("-- No input given.  Using Example input. -- ");
                Input::Example
        });
        tea::trace!(?part, ?inp);

        match (part, inp) {
                (Part::Part1, inp) => main_part1(inp),
                (Part::Part2, inp) => main_part2(inp),
        }?;
        tea::trace!("finishing main()");
        Ok(())
}
