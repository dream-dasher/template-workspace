//! Tracing - with Jon Gj.
//!
//! clear; RUST_LOG=trace carrbn samples_tracing  bb ccc dddd

use std::io::Read;

use tracing::{Level, debug, error, info, span, trace, warn};
fn main() {
        tracing_subscriber::fmt::init();
        let span = span!(Level::INFO, "main");
        let _guard = span.enter();
        for file in std::env::args().skip(1) {
                let span = span!(Level::INFO, "file", fname = %file);
                let _guard = span.enter();
                info!("opening the file");
                // let mut file = std::fs::File::open(file).unwrap();
                info!("reading file contents");
                // let mut bytes = Vec::new();
                // file.read_exact(&mut bytes).unwrap();
                info!("parsing");
                // ..
                info!("done with file");
        }
}
