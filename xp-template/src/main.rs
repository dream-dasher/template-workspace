use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use error::Result;
use tracing::{event, info, Level};

/// CLI Args
///
/// More words and lorems and so forth.
#[derive(Parser, Debug)]
#[command(version, about, long_about)] // Read from `Cargo.toml`
struct Args {
        /// Some kinda mode
        #[arg(value_enum)]
        templater: TemplateSystem,

        /// Name: Optional
        name: Option<String>,

        /// Optional Path to a template file
        #[arg(short, long, value_name = "FILE")]
        template_path: Option<PathBuf>,

        /// You can repeat me
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,
}
/// Templating Crate to use
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum TemplateSystem {
        /// no templating system
        None,
        /// minijinja
        MiniJinja,
        /// LiquidRust
        LiquidRust,
        /// Handlebars
        Handlebars,
}

fn main() -> Result<()> {
        let args = Args::parse();
        match &args.verbose {
                0 => support_tracing::tracing_subscribe_boilerplate("error"),
                1 => support_tracing::tracing_subscribe_boilerplate("warn"),
                2 => support_tracing::tracing_subscribe_boilerplate("info"),
                3 => support_tracing::tracing_subscribe_boilerplate("debug"),
                _ => support_tracing::tracing_subscribe_boilerplate("trace"),
        }
        tracing::event!(Level::DEBUG, "Script 1, starting...");
        info!(?args);

        Ok(())
}

// /////////////////////////////////////////////////////////////////////////////////////// //
/// EARLY_DEV: non-specific error & result types for use while exploring new code.
mod error {
        pub type Result<T> = core::result::Result<T, Error>;
        pub type Error = Box<dyn std::error::Error>;
}

/// Logging (tracing) related code.
mod support_tracing {
        use tracing_subscriber::EnvFilter;

        /// Basic boilerplate logging initialization.
        pub fn tracing_subscribe_boilerplate(env_min: impl Into<String>) {
                let filter = EnvFilter::try_new(
                        std::env::var("RUST_LOG").unwrap_or_else(|_| env_min.into()),
                        )
                        .expect("Valid filter input provided.");

                tracing_subscriber::fmt().pretty()
                                         .with_env_filter(filter)
                                         .with_file(true)
                                         .with_line_number(true)
                                         .with_thread_ids(true)
                                         .with_target(true)
                                         .init();
        }
}
