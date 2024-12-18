//! Tracing Subscriber configuration.

use std::io::Stderr;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, Registry,
                         fmt::{SubscriberBuilder, format::Pretty},
                         layer::Layered,
                         prelude::*};
use tracing_tree::HierarchicalLayer;

// workaround to hairy return type
type SpecificLayered = Layered<
        EnvFilter,
        Layered<HierarchicalLayer<fn() -> Stderr, tracing_tree::time::Uptime>, Registry>,
>;

/// Generates a tracing_subcsriber.  (Convenience function.)
#[expect(dead_code, reason = "Boiler plate available for use.")]
pub fn generate_tracing_tree_subscriber() -> SpecificLayered {
        let tree_layer = tracing_tree::HierarchicalLayer::new(3)
                .with_timer(tracing_tree::time::Uptime::default())
                // .with_span_modes(true)
                .with_indent_lines(true);
        let envfilter_layer = EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .from_env_lossy();
        Registry::default().with(tree_layer).with(envfilter_layer)
}

/// Basic boilerplate logging initialization.
// #[expect(dead_code, reason = "Boiler plate available for use.")]
pub fn generate_tracing_fmt_subscriber(
        env_min: impl Into<String>,
) -> SubscriberBuilder<Pretty, tracing_subscriber::fmt::format::Format<Pretty>, EnvFilter> {
        let env_filter =
                EnvFilter::try_new(std::env::var("RUST_LOG").unwrap_or_else(|_| env_min.into()))
                        .expect("Valid filter input provided.");

        tracing_subscriber::fmt()
                .pretty()
                .with_env_filter(env_filter)
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_target(true)
}
