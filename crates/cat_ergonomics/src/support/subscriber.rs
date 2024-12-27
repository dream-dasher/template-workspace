//! # Tracing Subscriber configuration for package: **cat-ergonomics**
//!
//! ## Strategy
//! - A bunch of layers to choose from, in one function. (Runnable boilerplate receptacle.)
//! - sets default global subscriber itself (**Side Effect**)
//!   - (Previously passed a "Subscriber" that could be set locally; however `Subscriber` is not compatible with `dyn` & the returned elements had a combination Type that required manual extraction of type via CLI and function signature update.  (and not ammenable to a more dynamic function.))
//! - Returns a `WorkerGuard` for the writer.
//!   - This is useful for ensuring that the writer is not dropped before the program ends.
//!   - Writer is just stderr by default, but can be set to write to a file, for example.
//!
//! ## Caution
//! - Tracing is poorly documented and methods poorly named.  One can easily use, e.g., `::fmt()` instead of `::fmt` and be greeted with cryptic or even misdirecting errors.
//!   - I have no solution for this.  *Just be careful!*  It is very easy to lose a lot of time chain one's tail, on seemingly trivial configuration.

use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer;
// use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, prelude::*};

use crate::Result;

/// (Convenience function.) Generates a tracing_subcsriber and sets it as global default, while returning a writer guard.
///
/// # Caveat
///   - Side effect. (sets global default tracing subscriber)
///
/// # Use:
/// ```text
/// fn main() -> Result<()> {
///     let _tracing_writer_worker_guard = generate_tracing_subscriber()?;
///    // ...
///    Ok(())
/// }
/// ```
pub fn active_global_default_tracing_subscriber() -> Result<WorkerGuard> {
        // let tree_layer = tracing_tree::HierarchicalLayer::new(2)
        //         .with_timer(tracing_tree::time::Uptime::default())
        //         .with_span_modes(true)
        //         .with_indent_lines(true)
        //         .with_targets(true);

        let envfilter_filter = tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::TRACE.into())
                .from_env_lossy();

        let error_layer = ErrorLayer::default().with_filter(LevelFilter::TRACE);
        // let indicatif_layer = IndicatifLayer::new();
        // manually set writer to stderr.  (choose a line; different types)
        let writer = {
                std::io::stderr() // regular stderr
                // indicatif_layer.get_stderr_writer() // this prevents status bars from overwriting the output.
        };
        let (non_blocking_writer, trace_writer_guard) = tracing_appender::non_blocking(writer);
        let fmt_layer = tracing_subscriber::fmt::Layer::default()
                // .compact()
                // .pretty()
                // .with_timer(<timer>)
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_file(true)
                .with_line_number(true)
                .with_span_events(FmtSpan::NONE)
                .with_writer(non_blocking_writer)
                .with_filter(envfilter_filter);

        let subscriber = tracing_subscriber::Registry::default()
                .with(error_layer)
                // .with(indicatif_layer)
                .with(fmt_layer);
        // .with(fmt_layer.and_then(tree_layer).with_filter(envfilter_layer));

        tracing::subscriber::set_global_default(subscriber)?;
        Ok(trace_writer_guard)
}
