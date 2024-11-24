use std::io::Read;

use tracing::{Level, debug, error, info, span, trace, warn};
fn main() {
        for file in std::env::args() {
                let span = span!(Level::INFO, "file");
                let _guard = span.enter();
                info!("opening the file");
                let mut file = std::fs::File::open(file).unwrap();
                info!("reading file contents");
                let mut bytes = Vec::new();
                file.read_exact(&mut bytes).unwrap();
                info!("parsing");
                // ..
                info!("done with file");
        }
}
