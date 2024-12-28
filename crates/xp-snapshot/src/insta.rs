#![expect(unused)]
//! insta snapshot
//! https://github.com/mitsuhiko/insta

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub fn wobble(s: &str) -> String {
        s.to_string()
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn insta_wobble_test() {
                insta::assert_snapshot!("Hello World", @"Hello World");
        }
}
