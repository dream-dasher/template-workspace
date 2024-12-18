//! # expect_test
//!
//! ## Description
//! Very simple, heuristic snapthotting.
//! `expect!()`/`expect_file!()` take a *string literal* and compare that to the **debug** value of something else.
//! They also keep track of their location in file.
//! They note when the assert fails (like pretty_assert).
//! And then they can be updated by manually running with rust analyzer (?!)
//! or by updating *everything* by running the program with an envvar set.
//!   - `UPDATE_EXPECT=1` -- updates **ALL** tests (often not what you want)
//!

#[cfg(test)]
mod tests {

        use expect_test::{Expect, expect, expect_file};

        #[derive(Debug)]
        #[expect(dead_code)]
        struct Foo {
                value: i32,
        }

        #[test]
        fn test_foo() {
                let actual = Foo { value: 92 };
                let expected = expect![["
            Foo {
                value: 92,
            }
        "]];
                expected.assert_debug_eq(&actual);
        }

        fn check(actual: i32, expect: Expect) {
                let actual = actual.to_string();
                expect.assert_eq(&actual);
        }

        #[test]
        fn test_addition() { check(90 + 2, expect![[]]); }

        #[test]
        fn test_multiplication() { check(46 * 2, expect![[]]); }

        #[test]
        fn test_large_value() {
                let actual = 42;
                let expected = expect_file!["./data/expect_snapshots/the-answer.txt"];
                expected.assert_eq(&actual.to_string());
        }
}
