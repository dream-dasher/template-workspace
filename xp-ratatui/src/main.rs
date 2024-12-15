//! # CLI interface for code in package: **xp-ratatui**
//!
//! ## Strategy
//! <placeholder code>
//!
//! ## Caveat
//! - tracing events on screen sseem to shift the whole screen!
//!
//! RUST_LOG=error cargo run --package xp-ratatui
//!

//! # CLI interface for code in package: **temp**
//!
//! ## Strategy
//! <placeholder code>
//! Principally convenience access to code in the `lib.rs`

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing::{self as tea, Level, instrument};
use xp_ratatui::{Result, active_global_default_tracing_subscriber, run_counter_app, run_hello_world};

/// Interface to choose a Ratatui tutorial program to run.
#[derive(Parser, Debug)]
#[command(
        version,
        about,
        long_about,
        disable_help_subcommand = true,
        subcommand_help_heading = "input source"
)]
pub struct Args {
        /// Ratatui Tutorial
        #[command(subcommand)]
        sub_program: TuiProgram,
}
#[derive(Debug, Clone, Subcommand)]
pub enum TuiProgram {
        /// Enter input via the terminal
        HelloWorld,
        /// Use custom file for input
        CounterApp,
        /// Provide a FilePath to use as input
        JsonEditor { path: PathBuf },
}

#[instrument(skip_all, ret(level = Level::DEBUG))]
fn main() -> Result<()> {
        let _tracing_writer_guard = active_global_default_tracing_subscriber()?;
        let args = Args::parse();
        tea::trace!(?args);

        let mut terminal = ratatui::init();
        terminal.clear()?;

        match args.sub_program {
                TuiProgram::HelloWorld => run_hello_world(terminal)?,
                TuiProgram::CounterApp => run_counter_app(terminal)?,
                TuiProgram::JsonEditor { path: _ } => todo!(),
        }
        Ok(())
}

// use ratatui::{DefaultTerminal,
//               crossterm::event::{self, KeyCode, KeyEventKind},
//               style::Stylize,
//               widgets::Paragraph};
// use tracing::{self as tea, Level, instrument, trace};
// #[expect(unused_imports)]
// use xp_ratatui::{_CUSTOM_INPUT_PATH, Result, active_global_default_tracing_subscriber};
// const _DEFAULT_INPUT: &str = "default input";

// #[instrument(skip_all, ret(level = Level::DEBUG))]
// fn main() -> Result<()> {
//         let _tracing_writer_guard = active_global_default_tracing_subscriber()?;

//         let mut terminal = ratatui::init();
//         terminal.clear()?;
//         let app_result = run(terminal);
//         ratatui::restore();
//         tea::trace!("finishing main()");
//         app_result
// }

// #[instrument(skip(terminal), ret(level = Level::DEBUG))]
// fn run(mut terminal: DefaultTerminal) -> Result<()> {
//         loop {
//                 trace!("draw frame");
//                 terminal.draw(|frame| {
//                         let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)").black().on_blue();
//                         frame.render_widget(greeting, frame.area());
//                 })?;
//                 trace!("look for input events");
//                 if let event::Event::Key(key) = event::read()? {
//                         if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
//                                 return Ok(());
//                         }
//                 }
//         }
// }

// // /// Package: **xp-ratatui**'s convenience CLI interface.
// // #[derive(Parser, Debug)]
// // #[command(
// //         version,
// //         about,
// //         long_about,
// //         disable_help_subcommand = true,
// //         subcommand_help_heading = "input source"
// // )]
// // pub struct Args {
// //         /// Action to take.
// //         action: Action,
// //         /// Input to use.
// //         #[command(subcommand)]
// //         input:  Option<Input>,
// // }
// // #[derive(Debug, Clone, ValueEnum)]
// // pub enum Action {
// //         /// Action 1
// //         #[value(alias = "1", alias = "i", alias = "I", alias = "one")]
// //         Action1,
// //         /// Action 2
// //         #[value(alias = "2", alias = "ii", alias = "II", alias = "two")]
// //         Action2,
// // }
// // #[derive(Debug, Clone, Subcommand)]
// // pub enum Input {
// //         /// Enter input via the terminal
// //         CLI { string: String },
// //         /// Use custom file for input
// //         Custom,
// //         /// Provide a FilePath to use as input
// //         FilePath { path: PathBuf },
// //         /// Use default input.
// //         Default,
// // }
