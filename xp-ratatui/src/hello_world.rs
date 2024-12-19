//! # Hello_World interface for code in package: **xp-ratatui**
//!
//! ## Strategy
//! [hello world link](https://ratatui.rs/tutorials/hello-world/)
//!
//! ## Caveat
//!
//! RUST_LOG=error cargo run --package xp-ratatui hello_world

use ratatui::{DefaultTerminal,
              crossterm::event::{self, KeyCode, KeyEventKind},
              style::Stylize,
              widgets::Paragraph};
use tracing::{self as tea, Level, instrument};

use crate::Result;
/// Run a simple hello world terminal UI application
///
/// # Arguments
///
/// * `terminal` - The terminal interface to render to
///
/// # Returns
///
/// Returns `Ok(())` on successful completion (when user presses 'q')
#[instrument(skip(terminal), ret(level = Level::DEBUG))]
pub fn run_hello_world(mut terminal: DefaultTerminal) -> Result<()> {
        loop {
                tea::trace!("draw frame");
                terminal.draw(|frame| {
                        let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)").black().on_blue();
                        frame.render_widget(greeting, frame.area());
                })?;
                tea::trace!("look for input events");
                if let event::Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                                return Ok(());
                        }
                }
        }
}
