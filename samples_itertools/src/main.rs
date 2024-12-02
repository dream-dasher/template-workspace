//! Itertools
//!
//! ## Docs
//! you likely want to start at:
//! [trait Itertools::Itertools](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.tuple_windows)
//!
//! ## Syntax Ref
//!
//! ## Gotchas
//!
//! ## Special Notes
//!
//!
//! clear; RUST_LOG=samples_itertools=trace carrbn samples_itertools

use itertools::Itertools;
use tracing::{Level, debug, error, info, info_span, level_filters::LevelFilter, span, trace, warn};
use tracing_subscriber::{EnvFilter, prelude::*};

fn main() {
        tracing_subscriber::Registry::default()
                .with(tracing_tree::HierarchicalLayer::new(2)
                        .with_timer(tracing_tree::time::Uptime::default())
                        .with_span_modes(true)
                        .with_indent_lines(true))
                .with(EnvFilter::builder()
                        .with_default_directive(LevelFilter::ERROR.into())
                        .from_env_lossy())
                // .with(tracing_subscriber::fmt::Layer::default())
                .init();

        // Window: type annotations, get tuple
        {
                let _enter = info_span!("Rolling Window over data").entered();
                let names = 0..;
                let mut rolling_win = names.clone().tuple_windows::<(_, _, _, _, _)>();
                for tuple in rolling_win.take(5) {
                        debug!(?tuple);
                }
        }

        // Chunk: makes iterator of iterators; all of which require `into_iter()`
        {
                let _enter = info_span!("Fixed Chunking of data").entered();
                let names = 0..;
                let mut fixed_chunks = names.clone().chunks(5);
                for chunk in fixed_chunks.into_iter().take(5) {
                        let vec: Vec<_> = chunk.into_iter().collect();
                        debug!(?vec);
                }
        }

        // Chunk_By: whatever yields same-ity
        {
                let _enter = info_span!("Custom Chunking").entered();
                let names = &[
                        "Bob", "Brandy", "Bobby", "Brenda", "Cal", "Connie", "Doyle", "Brendan", "Doyle2", "Delilah",
                ];
                let mut fixed_chunks = names.iter().chunk_by(|name| name.chars().next());
                for (comparison_key, chunk) in fixed_chunks.into_iter().take(5) {
                        let vec: Vec<_> = chunk.into_iter().collect();
                        debug!(?comparison_key, ?vec);
                }
        }
}
