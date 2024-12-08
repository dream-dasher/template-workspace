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
//! clear; RUST_LOG=samples_regex=trace carrbn samples_regex
//! if you want to see Regex output:
//! clear; RUST_LOG=trace carrbn samples_regex

use indoc::indoc;
use regex::{self, Regex};
use tracing::{self as tea, Level, instrument::WithSubscriber, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan, prelude::*};

const REGEX_PATH_SPLIT: &str = r"^(?m)^(?<path>[^:]+):(?<line_number>[0-9]+):(?<title>.+)$";
const HAY_PATHS: &str = indoc!("
path/to/foo:54:Blue Harvest
path/to/bar:90:Something, Something, Something, Dark Side
path/to/baz:3:It's a Trap!
");
const REGEX_DATE_SPLIT_TOO_RESTRICTIVE: &str = r"^(?<year>\d{4})-(?<month>)\d{2}-(?<day>\d{2})$";
const REGEX_DATE_SPLIT: &str = r"(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})";
const HAY_DATES: &str = indoc!("
What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?
1973-01-05, 1975-08-25 and 1980-10-18
What do 18653-04-14, 1891-07-033, 1901-09-06 and 1963-11-22 have in common?
What do 1865-04-14, 1214-91-13, 1214-22-12, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?
");
fn main() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::Registry::default()
                // .with(tracing_tree::HierarchicalLayer::new(2)
                //         .with_span_retrace(true)
                //         .with_targets(true)
                //         .with_bracketed_fields(true))
                .with(tracing_subscriber::fmt::Layer::default()
                        .with_thread_names(true)
                        .with_target(true)
                        .with_line_number(true)
                        .with_span_events(FmtSpan::ENTER)
                        // .with_file(true)
                        // .with_span_events(FmtSpan::ENTER)
                        .without_time())
                .with(EnvFilter::builder()
                        .with_default_directive(LevelFilter::ERROR.into())
                        .from_env_lossy())
                // .with(tracing_subscriber::fmt::Layer::default())
                .init();

        {
                let _enter = tea::debug_span!("CapNumFixed").entered();
                let re = Regex::new(REGEX_PATH_SPLIT).expect("string should be valid regex");
                tea::info!(?re, "Regex runtime construction completed.");

                {
                        let _enter = tea::debug_span!("Parsing").entered();
                        for (i, line) in HAY_PATHS.lines().enumerate() {
                                let (raw, [path, lineno, title]) = re.captures(line).unwrap().extract();
                                tea::info!(path, lineno, title, raw, i);
                        }
                }
        }

        {
                let _enter = tea::debug_span!("CapNumFixed").entered();
                let re = Regex::new(REGEX_DATE_SPLIT).expect("string should be valid regex");
                tea::info!(?re, "Regex runtime construction completed.");
                {
                        {
                                let _enter = tea::debug_span!("Parsing").entered();
                                for (i, line) in HAY_DATES.lines().enumerate() {
                                        let _enter = tea::warn_span!("Line", i).entered();
                                        for (i2, cap) in re.captures_iter(line).enumerate() {
                                                let (raw, [year, month, day]) = cap.extract();
                                                tea::info!(?raw, ?year, ?month, ?day, idx = ?(i, i2));
                                        }
                                }
                        }
                        let vec_per_line: Vec<_> = {
                                let _enter = tea::debug_span!("Parsing").entered();
                                HAY_DATES
                                        .lines()
                                        .enumerate()
                                        .flat_map(|(i, line)| {
                                                re.captures_iter(line).enumerate().map(|(i2, cap)| {
                                                        let (raw, [year, month, day]) = cap.extract();
                                                        tea::info!(?raw, ?year, ?month, ?day, i2);
                                                        (year, month, day)
                                                })
                                        })
                                        .collect()
                        };
                        tea::info!(?vec_per_line);
                }
        }
        Ok(())
}
