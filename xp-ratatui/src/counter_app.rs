//! # Counter_app interface for code in package: **xp-ratatui**
//!
//! ## Strategy
//! [counter app link](https://ratatui.rs/tutorials/counter-app/)
//!
//! ## Caveat
//!
//! RUST_LOG=error cargo run --package xp-ratatui counter_app

use std::io;

use crossterm::event::{Event, KeyEvent};
// use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame,
              buffer::Buffer,
              crossterm::event::{self, KeyCode, KeyEventKind},
              layout::Rect,
              style::Stylize,
              symbols::border,
              text::{Line, Text},
              widgets::{Block, Paragraph, Widget}};
use tracing::instrument;

use crate::Result;

#[instrument(skip_all)]
pub fn run_counter_app(mut terminal: DefaultTerminal) -> Result<()> {
        let app_result = App::default().run(&mut terminal);
        ratatui::restore();

        app_result
}

#[derive(Default, Debug)]
pub struct App {
        count: u32,
        exit:  bool,
}

impl App {
        #[instrument(skip_all)]
        pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
                while !self.exit {
                        terminal.draw(|frame| self.draw(frame))?;
                        self.handle_events()?;
                }
                Ok(())
        }

        #[instrument(skip_all)]
        fn draw(&self, frame: &mut Frame) {
                frame.render_widget(self, frame.area());
        }

        #[instrument(skip_all)]
        fn handle_events(&mut self) -> Result<()> {
                match event::read()? {
                        // it's important to check that the event is a key press event as
                        // crossterm also emits key release and repeat events on Windows.
                        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                                self.handle_key_event(key_event)
                        }
                        _ => {}
                };
                Ok(())
        }

        fn handle_key_event(&mut self, key_event: KeyEvent) {
                match key_event.code {
                        KeyCode::Char('q') => self.exit(),
                        KeyCode::Left => self.decrement_counter(),
                        KeyCode::Right => self.increment_counter(),
                        _ => {}
                }
        }

        fn exit(&mut self) {
                self.exit = true;
        }

        fn increment_counter(&mut self) {
                self.count = self.count.wrapping_add(1);
        }

        fn decrement_counter(&mut self) {
                self.count = self.count.wrapping_sub(1);
        }
}

impl Widget for &App {
        #[instrument(skip_all)]
        fn render(self, area: Rect, buf: &mut Buffer) {
                let title = Line::from(" Counter App Tutorial ".bold());
                let instructions = Line::from(vec![
                        " Decrement ".into(),
                        "<Left>".blue().bold(),
                        " Increment ".into(),
                        "<Right>".blue().bold(),
                        " Quit ".into(),
                        "<Q> ".blue().bold(),
                ]);
                let block = Block::bordered()
                        .title(title.centered())
                        .title_bottom(instructions.centered())
                        .border_set(border::THICK);

                let counter_text = Text::from(vec![Line::from(vec![
                        "Value: ".into(),
                        self.count.to_string().yellow(),
                ])]);
                Paragraph::new(counter_text).centered().block(block).render(area, buf);
        }
}

#[cfg(test)]
mod tests {
        use pretty_assertions::assert_eq;
        use ratatui::style::Style;

        use super::*;

        #[test]
        fn render() {
                let app = App::default();
                let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

                app.render(buf.area, &mut buf);

                let mut expected = Buffer::with_lines(vec![
                        "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
                        "┃                    Value: 0                    ┃",
                        "┃                                                ┃",
                        "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
                ]);
                let title_style = Style::new().bold();
                let counter_style = Style::new().yellow();
                let key_style = Style::new().blue().bold();
                expected.set_style(Rect::new(14, 0, 22, 1), title_style);
                expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
                expected.set_style(Rect::new(13, 3, 6, 1), key_style);
                expected.set_style(Rect::new(30, 3, 7, 1), key_style);
                expected.set_style(Rect::new(43, 3, 4, 1), key_style);

                assert_eq!(buf, expected);
        }

        #[test]
        fn handle_key_event() -> io::Result<()> {
                let mut app = App::default();
                app.handle_key_event(KeyCode::Right.into());
                assert_eq!(app.count, 1);

                app.handle_key_event(KeyCode::Left.into());
                assert_eq!(app.count, 0);

                app.handle_key_event(KeyCode::Left.into());
                assert_eq!(app.count, u32::MAX);

                let mut app = App::default();
                app.handle_key_event(KeyCode::Char('q').into());
                assert!(app.exit);

                Ok(())
        }
}
