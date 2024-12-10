//! Playing with ptr types

use tracing::{self as tea, instrument};

fn main() {
        tracing_subscriber::fmt::init();

        let smallest = Matroschka::new(Color::Red);
        tea::error!(?smallest);
        let second = smallest.add_shell(Color::Green);
        tea::info!(?second);
        let third = second.add_shell(Color::Blue);
        tea::debug!(?third);

        // NOTE: We are putting the method on `Box`, NOT on `Matroschka`, but it seems to operate as if it were on `Matroschka`
        let extraction = third.smaller_me.unwrap();
        tea::trace!(?extraction);
        let new_second = extraction.add_shell(Color::Yellow);
        tea::warn!(?new_second);
}

#[derive(Debug)]
struct Matroschka {
        #[expect(dead_code, reason = "incorrect flag, it is used ...")]
        color:      Color,
        size:       usize,
        smaller_me: Option<Box<Matroschka>>,
}

impl Matroschka {
        #[instrument]
        fn new(color: Color) -> Self {
                Self {
                        color,
                        size: 0,
                        smaller_me: None,
                }
        }

        #[instrument]
        fn add_shell(self, color: Color) -> Self {
                let size = self.size + 1;
                Self {
                        color,
                        size,
                        smaller_me: Some(Box::new(self)),
                }
        }
}

#[derive(Debug)]
enum Color {
        Red,
        Green,
        Blue,
        Yellow,
}
