//! An Example Script using Example Library in Example Workspace

use libraire::{repeat_function, say_hi};

fn main() {
    println!("Script 1, running...");
    repeat_function(say_hi, 7);
}

#[cfg(test)]
mod tests {

    #[test]
    fn trivially_true_in_practice() {
        assert_eq!(2 + 3, 3 + 2);
    }
}
