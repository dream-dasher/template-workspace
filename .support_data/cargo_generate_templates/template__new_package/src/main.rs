//! # CLI interface for code in package: **{{ project-name | kebab_case }}**
//!
//! ## Strategy
//! <placeholder code>
//! Principally convenience access to code in the `lib.rs`

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use tracing::{self as tea, Level, instrument};
use {{ project-name | snake_case }}::{CUSTOM_INPUT_PATH, Result, active_global_default_tracing_subscriber};

const DEFAULT_INPUT: &str = "default input";

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
        /// Action to take.
        action: Action,
        /// Input to use.
        #[command(subcommand)]
        input:  Option<Input>,
}
#[derive(Debug, Clone, ValueEnum)]
pub enum Action {
        /// Action 1
        #[value(alias = "1", alias = "i", alias = "I", alias = "one")]
        Action1,
        /// Action 2
        #[value(alias = "2", alias = "ii", alias = "II", alias = "two")]
        Action2,
}
#[derive(Debug, Clone, Subcommand)]
pub enum Input {
        /// Enter input via the terminal
        CLI { string: String },
        /// Use custom file for input
        Custom,
        /// Provide a FilePath to use as input
        FilePath { path: PathBuf },
        /// Use default input.
        Default,
}

#[instrument(skip_all, ret(level = Level::DEBUG))]
fn main() -> Result<()> {
        let _tracing_writer_guard = active_global_default_tracing_subscriber()?;
        let args = Args::parse();
        tea::trace!(?args);

        let input = match args.input {
                Some(Input::CLI { string }) => string,
                Some(Input::Custom) => CUSTOM_INPUT_PATH.to_string(),
                Some(Input::FilePath { path }) => std::fs::read_to_string(path)?,
                Some(Input::Default) | None => DEFAULT_INPUT.to_string(),
        };
        tea::trace!(?input);
        tea::trace!("finishing main()");
        Ok(())
}
