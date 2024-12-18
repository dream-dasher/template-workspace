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

use itertools::{Itertools, iproduct};
use tracing::{self as tea, level_filters::LevelFilter};
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
                let _enter = tea::info_span!("Rolling Window over data").entered();
                let seq = 0..;
                let rolling_win = seq.clone().tuple_windows::<(_, _, _, _, _)>();
                for tuple in rolling_win.take(5) {
                        tea::debug!(?tuple);
                }
        }

        // Chunk: makes iterator of iterators; all of which require `into_iter()`
        {
                let _enter = tea::info_span!("Fixed Chunking of data").entered();
                let seq = 0..;
                let fixed_chunks = seq.clone().chunks(5);
                for chunk in fixed_chunks.into_iter().take(5) {
                        let vec: Vec<_> = chunk.into_iter().collect();
                        tea::debug!(?vec);
                }
        }

        // Chunk_By: whatever yields same-ity
        {
                let _enter = tea::info_span!("Custom Chunking").entered();
                let names =
                        &["Bob", "Brandy", "Bobby", "Brenda", "Cal", "Connie", "Doyle", "Brendan"];
                let custom_chunks = names.iter().chunk_by(|name| name.chars().next());
                for (comparison_key, chunk) in custom_chunks.into_iter().take(5) {
                        let vec: Vec<_> = chunk.into_iter().collect();
                        tea::debug!(?comparison_key, ?vec);
                }
        }

        // iProduct!: Cartesian_Product
        {
                let _enter = tea::info_span!("Cartesian Product").entered();
                let teams = &["Eagles", "Owls", "Vultures"];
                for (left, right) in iproduct!(teams.iter(), teams.iter()) {
                        tea::debug!(?left, ?right);
                }
        }

        // Multi_Cartesian_Product!: turns iterator of iterators into product
        {
                let _enter = tea::info_span!(
                        "Multi Cartesian Product: iterator of iterators into product"
                )
                .entered();
                let groupables = &["A_0", "B_0 ", "B_11", "C_0  ", "C_11 ", "C_222"];
                // chunks are kinda complicated and need some teasing
                let custom_chunks: Vec<Vec<&&str>> = groupables
                        .iter()
                        .chunk_by(|name| name.chars().next())
                        .into_iter()
                        .map(|(_, iter)| iter.collect())
                        .collect();
                // takes an iterators of iterators
                let mcp_vec: Vec<_> = custom_chunks.iter().multi_cartesian_product().collect();

                for combo in mcp_vec.iter() {
                        tea::debug!(?combo);
                }
        }
}
