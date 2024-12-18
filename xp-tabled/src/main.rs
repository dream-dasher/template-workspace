//! # CLI interface for code in package: **xp-tabled**
//!
//! ## Strategy
//! <placeholder code>
//! Principally convenience access to code in the `lib.rs`

use std::{collections::BTreeMap, ops::AddAssign, path::PathBuf};

use clap::{Parser, ValueEnum};
use indoc::indoc;
use json_to_table::json_to_table;
use serde_json::json;
use tabled::{Table, Tabled,
             builder::Builder,
             settings::{Style, merge::Merge}};
use tracing::{self as tea, Level, instrument};
use xp_tabled::{Result, active_global_default_tracing_subscriber};

/// Package: **xp-tabled**'s convenience CLI interface.
#[derive(Parser, Debug, Tabled, derive_more::Display)]
#[command(
        version,
        about,
        long_about,
        disable_help_subcommand = true,
        subcommand_help_heading = "input source"
)]
#[display("Args: {:?} {:?} {:?}", input, path, words)]
pub struct Args {
        /// a thing
        input: Thing,
        // a path
        #[display("{:?}", path)]
        #[tabled(skip)]
        path:  PathBuf,
        /// some word
        #[display("{:?}", words)]
        #[tabled(format = "{:?}")]
        words: Option<String>,
}
#[derive(derive_more::Display, Debug, Clone, ValueEnum)]
pub enum Thing {
        /// 1
        #[value(alias = "1", alias = "i", alias = "I", alias = "one")]
        One,
        /// 2
        #[value(alias = "2", alias = "ii", alias = "II", alias = "two")]
        Two,
}

// use testing_table::assert_table;
#[derive(Tabled)]
struct WordInfo<'a> {
        word:             &'a str,
        frequency:        usize,
        length:           usize,
        most_common_char: char,
}

#[instrument(skip_all, ret(level = Level::DEBUG))]
fn main() -> Result<()> {
        let _tracing_writer_guard = active_global_default_tracing_subscriber()?;
        let args = Args::parse();

        let word_fmap = {
                let mut fmap: BTreeMap<&str, usize> = BTreeMap::new();
                let string = args.words.as_ref().map_or(LYRICS, |s| s);
                for line in string.lines() {
                        for word in line.split_whitespace() {
                                fmap.entry(word).or_default().add_assign(1);
                        }
                }
                fmap
        };

        let word_info_list = {
                let mut list = Vec::new();
                for (word, count) in word_fmap.iter() {
                        let mut char_fmap: BTreeMap<char, usize> = BTreeMap::new();
                        for c in word.chars() {
                                char_fmap.entry(c).or_default().add_assign(1);
                        }
                        let most_common_char = char_fmap
                                .into_iter()
                                .max_by_key(|&(_, count)| count)
                                .map(|(c, _)| c)
                                .unwrap_or_default();
                        list.push(WordInfo {
                                word,
                                frequency: *count,
                                length: word.len(),
                                most_common_char,
                        });
                }
                list
        };

        println!("--------------------------------------------------------------------------");
        tea::info!("std PrettyPrint Debugging");
        println!("{:#?}\n", args);

        tea::info!("Table from `::from(_)` using builder, from a BTreeMap");
        let mut table_from_fmap = Builder::from(word_fmap).build();
        table_from_fmap.with(Style::modern_rounded().remove_horizontal());
        println!("{}\n", table_from_fmap);

        tea::info!("Table from `::new(_)`, from a Vector of Struts");
        let mut word_info_table = Table::new(&word_info_list);
        word_info_table.with(Style::psql());
        println!("{}\n", word_info_table);

        tea::info!("Merging similar data cels together.");
        let data = [['A', 'B', 'B'], ['A', 'W', 'E'], ['Z', 'Z', 'Z']];
        let mut table_for_merge = Table::new(data);
        table_for_merge
                .with(Merge::horizontal())
                .with(Merge::vertical());
        println!("{}\n", table_for_merge);

        tea::info!("Merging similar data cels together.");
        let mut data_ = [['Q', 'A', 'Z'], ['A', 'A', 'A'], ['A', 'A', 'A'], ['A', 'Z', 'Z']];
        for bttm in data_.iter_mut().flat_map(|row| row.iter_mut()) {
                if *bttm != 'A' {
                        *bttm = '.';
                }
        }
        let table_no_merge = Table::new(data_);
        println!("horizontal\n{}\n", table_no_merge);
        let data_h = [['Q', 'A', 'Z'], ['A', 'A', 'A'], ['A', 'A', 'A'], ['A', 'Z', 'Z']];
        let mut table_hor_merge = Table::new(data_h);
        table_hor_merge.with(Merge::horizontal());
        println!("horizontal\n{}\n", table_hor_merge);
        let data_v = [['Q', 'A', 'Z'], ['A', 'A', 'A'], ['A', 'A', 'A'], ['A', 'Z', 'Z']];
        let mut table_vert_merge = Table::new(data_v);
        table_vert_merge.with(Merge::vertical());
        println!("vertical\n{}\n", table_vert_merge);
        let data_m = [['Q', 'A', 'Z'], ['A', 'A', 'A'], ['A', 'A', 'A'], ['A', 'Z', 'Z']];
        let mut table_attempt_mix_merge = Table::new(data_m);
        table_attempt_mix_merge
                .with(Merge::vertical())
                .with(Merge::horizontal())
                .with(Merge::vertical());
        println!("multiple, to no effect\n{}\n", table_attempt_mix_merge);

        tea::info!("Vector of Tabled Structs; combined as strings");
        let combined = &[
                table_no_merge.clone().to_string(),
                table_hor_merge.clone().to_string(),
                table_vert_merge.clone().to_string(),
                table_attempt_mix_merge.clone().to_string(),
        ];
        let table_from_combined = Table::new(combined);
        println!("{}\n", table_from_combined);

        tea::info!("Vector of Tabled Structs; combined as structs with macro");
        let combined_macro = tabled::col![
                tabled::row!["raw", "hor_merge", "vert_merge", "attempt_mix_merge"],
                tabled::row![
                        table_no_merge,
                        table_hor_merge,
                        table_vert_merge,
                        table_attempt_mix_merge,
                ]
        ];
        println!("{}\n", combined_macro);

        let serde_json_value = json!(
            [
                {
                    "name": "Aleix Melon",
                    "id": "E00245",
                    "role": ["Dev", "DBA"],
                    "age": 23,
                    "doj": "11-12-2019",
                    "married": false,
                    "address": {
                        "street": "32, Laham St.",
                        "city": "Innsbruck",
                        "country": "Austria"
                        },
                    "referred-by": "E0012"
                },
            ]
        );

        tea::warn!(r#"requires: `json_to_table = "0.6"`"#);
        let json_table = json_to_table(&serde_json_value).to_string();

        println!("JSON:!\n{}\n", serde_json_value);
        println!("JSON to Table!\n{}\n", json_table);

        tea::debug!(%json_table);
        Ok(())
}

const LYRICS: &str = indoc!(r#"
    … So, so you think you can tell heaven from hell?
    Blue skies from pain?
    Can you tell a green field from a cold steel rail?
    A smile from a veil?
    Do you think you can tell?
"#);
