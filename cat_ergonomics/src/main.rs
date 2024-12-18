//! # CLI interface for code in package: **cat-ergonomics**
//!
//! ## Strategy
//! <placeholder code>
//! Principally convenience access to code in the `lib.rs`

use std::{path::PathBuf, time::Duration};

use cat_ergonomics::{Result, active_global_default_tracing_subscriber};
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize as _;
use tracing::{Level, instrument, trace, warn};

/// Package: **cat-ergonomics**'s convenience CLI interface.
#[derive(Parser, Debug)]
#[command(version, about, long_about, disable_help_subcommand = true)]
pub struct Args {
        /// Action to take.
        #[command(subcommand)]
        action: Option<DoAction>,
}
#[derive(Debug, Clone, Subcommand)]
pub enum DoAction {
        /// Enter input via the terminal
        #[command(alias = "preset", alias = "example", alias = "ex", alias = "pset")]
        PreSetExample { num_hi_s: Option<usize> },
        /// Custom command1
        #[command(alias = "1", alias = "i", alias = "I", alias = "one")]
        Custom1 { len: Option<u32> },
        /// Custom command 2
        #[command(alias = "2", alias = "ii", alias = "II", alias = "two")]
        Custom2 { path: PathBuf },
}

#[instrument(skip_all, ret(level = Level::DEBUG))]
fn main() -> Result<()> {
        let _tracing_writer_guard = active_global_default_tracing_subscriber()?;
        let args = Args::parse();
        trace!(?args);

        let do_action = args.action.unwrap_or_else(|| {
                warn!("No Action selected, using PreSetExample behavior!");
                DoAction::PreSetExample { num_hi_s: None }
        });
        match do_action {
                DoAction::PreSetExample { num_hi_s } => match num_hi_s {
                        Some(hi_s) => run_preset_example(hi_s, None)?,
                        None => {
                                let def = 10;
                                warn!(
                                        r#"No "number of hi's" selected, using default value: {}"#,
                                        def
                                );
                                run_preset_example(def, None)?
                        }
                },
                DoAction::Custom1 { len } => println!("Custom1: {:?}", len),
                DoAction::Custom2 { path } => {
                        let read_string = std::fs::read_to_string(path)?;
                        println!("Custom1: {}", read_string)
                }
        };
        trace!("finishing main()");
        Ok(())
}

/// Say hi a few times
#[instrument]
fn run_preset_example(num_hi_s: usize, opt_delay: Option<Duration>) -> Result<()> {
        const DEF_DUR: Duration = Duration::from_millis(300);
        const HI: &str = "hi";
        if num_hi_s == 0 {
                Err("No hi's to print!".to_string())?
        }
        let delay = opt_delay.unwrap_or(DEF_DUR);
        let mut hi_s: String = String::from("");
        for i in 0..num_hi_s {
                hi_s.push_str(HI);
                if i % 2 == 0 {
                        println!("{}", hi_s.blue());
                } else if i % 7 == 0 {
                        println!("{}", hi_s.red());
                } else {
                        println!("{}", hi_s.green());
                }
                std::thread::sleep(delay);
        }
        Ok(())
}
