//! An Example Binary (CLI app) using Example Library in Example Workspace
//!

mod support;

use std::{io::Write, time::Duration};

use clap::{Parser, Subcommand};
use lib_blocking::{repeat_function, utility::say_hi}; // workspace
use tracing::{self as tea, instrument};

use crate::support::Result;

/// CLI Args
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
        #[command(subcommand)]
        subcmnd: SubCommandEnum,
}

#[derive(Debug, Clone, Subcommand)]
enum SubCommandEnum {
        /// Ints, xp.
        RepeatHi {
                repetitions: u8,
        },

        // Dance for x seconds.
        Dance {
                seconds: u8,
        },
}

fn main() -> Result<()> {
        tracing::subscriber::set_global_default(support::generate_tracing_fmt_subscriber("warn").finish())?;

        tea::info!("----Tracing Active----");
        let args = Args::parse();
        match args.subcmnd {
                SubCommandEnum::RepeatHi { repetitions } => {
                        println!("Script 1: Repeats, running...");
                        repeat_function(say_hi, repetitions)
                }
                SubCommandEnum::Dance { seconds } => {
                        println!("Script 2: Dance, running...");
                        dance(seconds)
                }
        }
        Ok(())
}

/// Raw Animation.
/// Using ANSI-escape code to clear screen.
/// And manual idexing to place wide utf8 chars in a [u8] array.
fn dance(seconds: u8) {
        const DA_LEN: usize = 100;
        const FRAME_RATE: u8 = 30;
        const MS_TO_WAIT: u64 = 1000 / FRAME_RATE as u64;
        const DANCE_ARRAY: [u8; DA_LEN] = [b'_'; DA_LEN];
        const BORING_BUF: [u8; 4] = [b'_'; 4];
        // const DANCER_BUF: [u8; 4] = [0xF0, 0x9F, 0x95, 0xBA]; // UTF-8 for 🕺
        let mut dancer_buf: [u8; 4] = BORING_BUF; // not sure how to null init
        '🕺'.encode_utf8(&mut dancer_buf);
        let dancer_buf = dancer_buf;

        let mut idx: usize = 0;
        let mut dance_array = DANCE_ARRAY;
        dance_array[0..4].copy_from_slice(&dancer_buf);

        let mut countdown = seconds as u64;
        let start_time = std::time::Instant::now();
        loop {
                // 0 1 2 3 4 5 6 7 8 9
                // 0 1 2 3 4 5
                let idx_next = (idx + 1).rem_euclid(DA_LEN - 4);
                dance_array.copy_within(idx..idx + 4, idx_next);
                if idx == 0 {
                        dance_array[DA_LEN - 4..DA_LEN].copy_from_slice(&BORING_BUF);
                }
                dance_array[idx] = b'_';
                idx = idx_next;

                clear_screen_ansi();
                tea::info!(?idx, ?idx_next, ?dance_array);
                println!("Dancing for more {} seconds...", countdown);
                println!("{:?}", &dance_array);
                println!("{}", String::from_utf8_lossy(&dance_array));
                println!("{:?}", &dance_array);

                let current_time = std::time::Instant::now();
                if current_time.duration_since(start_time).as_secs() > (seconds as u64 - countdown) {
                        countdown -= 1;
                }
                if current_time.duration_since(start_time).as_secs() > seconds as u64 {
                        clear_screen_ansi();
                        println!("Dance is Done!");
                        println!("Dance is Done!");
                        println!("Dance is Done!");
                        std::thread::sleep(Duration::from_secs(1));
                        clear_screen_ansi();
                        break;
                }
                std::thread::sleep(Duration::from_millis(MS_TO_WAIT));
        }
}

/// Clear terminal screen using ANSI escape code.
///
/// Not the most robust, but decent in a pinch.
#[instrument]
fn clear_screen_ansi() {
        // There are ANSI escape codes that can be used to clear the screen!
        const ANSI_CLEAR_SCREEN: &str = "\x1B[2J\x1B[H";
        print!("{}", ANSI_CLEAR_SCREEN);
        std::io::stdout().flush().unwrap();
}

#[cfg(test)]
mod tests {

        use test_log::test;

        #[test]
        fn trivially_true_in_practice() {
                assert_eq!(2 + 3, 3 + 2);
        }
}
