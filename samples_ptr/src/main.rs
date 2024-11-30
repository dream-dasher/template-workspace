//! Playing with ptr types

use tracing::{Level, debug, error, event, info, instrument, trace, warn};

fn main() {
        tracing_subscriber::fmt::init();

        let smallest = Matroschka::new(Color::Red);
        error!(?smallest);
        let second = smallest.add_shell(Color::Green);
        info!(?second);
        let third = second.add_shell(Color::Blue);
        debug!(?third);

        // NOTE: We are putting the method on `Box`, NOT on `Matroschka`, but it seems to operate as if it were on `Matroschka`
        let extraction = third.smaller_me.unwrap();
        trace!(?extraction);
        let new_second = extraction.add_shell(Color::Yellow);
        warn!(?new_second);
}

#[derive(Debug)]
struct Matroschka {
        color:      Color,
        size:       usize,
        smaller_me: Option<Box<Matroschka>>,
}

impl Matroschka {
        fn new(color: Color) -> Self {
                Self {
                        color,
                        size: 0,
                        smaller_me: None,
                }
        }

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
