//! An Example Script using Example Library in Example Workspace

use libraire::{repeat_function, say_hi};

fn main() {
    println!("Script 1, running...");
    repeat_function(say_hi, 7);
}
