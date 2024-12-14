//! # CLI interface for code in package: **xp-ratatui**
//!
//! ## Strategy
//! <placeholder code>
//!
//! ## Caveat
//! - tracing events on screen sseem to shift the whole screen!
//!
//! RUST_LOG=error cargo run --package xp-ratatui

use std::io;

use ratatui::{DefaultTerminal,
              crossterm::event::{self, KeyCode, KeyEventKind},
              style::Stylize,
              widgets::Paragraph};
use tracing::{self as tea, Level, instrument, trace};
use xp_ratatui::{CUSTOM_INPUT_PATH, Result, active_global_default_tracing_subscriber};
const DEFAULT_INPUT: &str = "default input";

#[instrument(skip_all, ret(level = Level::DEBUG))]
fn main() -> Result<()> {
        let _tracing_writer_guard = active_global_default_tracing_subscriber()?;

        let mut terminal = ratatui::init();
        terminal.clear()?;
        let app_result = run(terminal);
        ratatui::restore();
        tea::trace!("finishing main()");
        app_result
}

#[instrument(skip(terminal), ret(level = Level::DEBUG))]
fn run(mut terminal: DefaultTerminal) -> Result<()> {
        loop {
                trace!("draw frame");
                terminal.draw(|frame| {
                        let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)").black().on_blue();
                        frame.render_widget(greeting, frame.area());
                })?;
                trace!("look for input events");
                if let event::Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                                return Ok(());
                        }
                }
        }
}

// /// Package: **xp-ratatui**'s convenience CLI interface.
// #[derive(Parser, Debug)]
// #[command(
//         version,
//         about,
//         long_about,
//         disable_help_subcommand = true,
//         subcommand_help_heading = "input source"
// )]
// pub struct Args {
//         /// Action to take.
//         action: Action,
//         /// Input to use.
//         #[command(subcommand)]
//         input:  Option<Input>,
// }
// #[derive(Debug, Clone, ValueEnum)]
// pub enum Action {
//         /// Action 1
//         #[value(alias = "1", alias = "i", alias = "I", alias = "one")]
//         Action1,
//         /// Action 2
//         #[value(alias = "2", alias = "ii", alias = "II", alias = "two")]
//         Action2,
// }
// #[derive(Debug, Clone, Subcommand)]
// pub enum Input {
//         /// Enter input via the terminal
//         CLI { string: String },
//         /// Use custom file for input
//         Custom,
//         /// Provide a FilePath to use as input
//         FilePath { path: PathBuf },
//         /// Use default input.
//         Default,
// }
