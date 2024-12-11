//! Rayon
//!

use core::fmt;
use std::path::PathBuf;

use tracing::{self as tea, Level, level_filters::LevelFilter, warn_span};
use tracing_subscriber::{EnvFilter, prelude::*};

fn main() {
        tracing_subscriber::Registry::default()
                // // Problem: tree seems to work poorly when non-monolinear thread
                // .with(tracing_tree::HierarchicalLayer::new(2)
                //         .with_timer(tracing_tree::time::Uptime::default())
                //         .with_span_modes(true)
                //         .with_indent_lines(true))
                .with(EnvFilter::builder()
                        .with_default_directive(LevelFilter::TRACE.into())
                        .from_env_lossy())
                .with(tracing_subscriber::fmt::Layer::default())
                .init();

        use std::sync::atomic::{AtomicUsize, Ordering};

        use rayon::prelude::*;

        let counter = AtomicUsize::new(0);
        let _tea = tea::warn_span!("ParentOfShared").entered();
        let span = tea::warn_span!("SharedSpan");
        let _tea = span.enter();
        let _tea = tea::warn_span!("NotShared").entered();
        tea::warn!(?counter, "Hello from outside of thread");
        let _tea = span.enter();
        let value = (0_i32..2048)
                .into_par_iter()
                .map(|x| {
                        let _tea = span.enter();
                        // let _tea = span.entered(); // <-- not allowed, moves span
                        // let _tea = span.clone().entered(); // <-- not needed, we can `.enter()` span
                        tea::info!(x, ?counter);
                        counter.fetch_add(1, Ordering::SeqCst);
                        // if x < 1024 { Some(x) } else { None }
                        if x <= 0 { Some(x) } else { None }
                })
                .while_some()
                .max();

        tea::info!(value);
        tea::info!(l = counter.load(Ordering::SeqCst)); // should not have visited every single one
}
