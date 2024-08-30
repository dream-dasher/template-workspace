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
        speed: Mode,

        #[arg(default_value_t = 2020)]
        /// Network port: Optional (w/ default)
        #[arg(value_parser = clap::value_parser!(u16).range(1..))]
        port: u16,

        /// Name: Optional
        name: Option<String>,

        /// flag_Repeats: Optional
        #[arg(short, long)]
        repeats: Option<u8>,

        /// Sets a custom config file
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,

        /// Turn debugging information on
        #[arg(short, long, action = clap::ArgAction::Count)]
        debug: u8,

        /// You can repeat me
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,

        /// Some sort of sub-command
        #[command(subcommand)]
        subcommand: Option<SubCommands>,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode {
        /// Run swiftly
        Fast,
        /// Crawl slowly but steadily
        ///
        /// This paragraph is ignored because there is no long help text for possible values.
        Slow,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
        /// does testing things
        LightSwitch {
                /// lists test values
                #[arg(short, long)]
                toggle: bool,
        },
}

fn main() -> Result<()> {
        support_tracing::tracing_subscribe_boilerplate("info");
        tracing::event!(Level::DEBUG, "Script 1, starting...");
        let args = Args::parse();
        println!("Script 1, running... {}", args.repeats.unwrap_or(0));
        info!(?args);

        match &args.speed {
                Mode::Fast => {
                        println!("Hare");
                }
                Mode::Slow => {
                        println!("Tortoise");
                }
        }

        match &args.subcommand {
                Some(SubCommands::LightSwitch { toggle: list_bool }) => {
                        event!(Level::INFO, "Subcommand Provided.");
                        match list_bool {
                                true => event!(Level::INFO, "Toggle is set to  ^^true^^."),
                                false => event!(Level::INFO, "Toggle is set to  v-false-v."),
                        }
                }
                None => event!(Level::WARN, "SubCommand not given."),
        }

        Ok(())
}

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
