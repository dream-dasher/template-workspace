#![allow(unused)]
//! insta snapshot
//! https://github.com/mitsuhiko/insta
//!
//! any of:
//! - `cargo insta test -p xp-snapshot`
//! - `cargo test -p xp-snapshot`
//! - `cargo nextest run -p xp-snapshot`
//!
//! along with:
//! - `cargo insta review`

use rand::{SeedableRng, prelude::*};
use rand_chacha::ChaCha12Rng; // used for reproducible rng

const INSECURE_SEED: u64 = 849275923947769452;
const CHANCE_UPPERCASE: f64 = 0.5;
const CHANCE_REPLACE: f64 = 0.8;
pub fn wobble(s: &str) -> String {
        // standard rand not recommended for reproducibility
        // ChaCha specifically suggested as an example alternative
        let mut rng = ChaCha12Rng::seed_from_u64(INSECURE_SEED);
        s.chars().fold(String::new(), |mut acc, c| {
                if rng.gen_bool(CHANCE_REPLACE) {
                        acc.push('~');
                        return acc;
                } else if rng.gen_bool(CHANCE_UPPERCASE) {
                        acc.extend(c.to_uppercase())
                } else {
                        acc.extend(c.to_lowercase())
                };
                acc
        })
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn insta_wobble_test() {
                insta::assert_snapshot!(wobble("Hello World"), @"H~~l~~w~~~~");
        }
        #[test]
        fn insta_wobble2_test() {
                insta::assert_snapshot!(wobble("abcde1234!@"), @"A~~d~~2~~~~");
        }
}
