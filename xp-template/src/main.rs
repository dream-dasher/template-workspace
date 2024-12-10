//! main - cli entry point to various example templating code

mod error;
mod handlebars_ex;
mod liquid_ex;
mod minijinja_ex;
mod support;

use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use error::Result;
use handlebars_ex::handlebars_example;
use liquid_ex::liquid_rust_example;
use minijinja_ex::mini_jinja_example;
use tracing::{self as tea, Level};

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
        Minijinja,
        /// LiquidRust
        LiquidRust,
        /// Handlebars
        Handlebars,
}

fn main() -> Result<()> {
        let args = Args::parse();
        match &args.verbose {
                0 => support::tracing_subscribe_boilerplate("error"),
                1 => support::tracing_subscribe_boilerplate("warn"),
                2 => support::tracing_subscribe_boilerplate("info"),
                3 => support::tracing_subscribe_boilerplate("debug"),
                _ => support::tracing_subscribe_boilerplate("trace"),
        }
        tracing::event!(Level::DEBUG, "Script 1, starting...");
        tea::info!(?args);

        match &args.templater {
                TemplateSystem::None => {
                        tea::event!(Level::INFO, "No templating system selected.");
                        println!("No templating system selected.");
                }
                TemplateSystem::Minijinja => {
                        tea::event!(Level::INFO, "Using MiniJinja templating system.");
                        println!("Using MiniJinja templating system.");
                        mini_jinja_example()?;
                }
                TemplateSystem::LiquidRust => {
                        tea::event!(Level::INFO, "Using LiquidRust templating system.");
                        println!("Using LiquidRust templating system.");
                        liquid_rust_example()?;
                }
                TemplateSystem::Handlebars => {
                        tea::event!(Level::INFO, "Using Handlebars templating system.");
                        println!("Using Handlebars templating system.");
                        handlebars_example()?;
                }
        }

        Ok(())
}

// /////////////////////////////////////////////////////////////////////////////////////// //
