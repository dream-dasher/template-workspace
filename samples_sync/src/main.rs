//! Playing with sync and relateed objects

use std::{sync::{Arc, Barrier},
          thread};

fn main()
{
        barrier_example(10, 20);
}

/// Example use of barrier.
/// Given a value, barrier will block thread progression until it has received that number of wait calls.
///
/// ## Note
/// if `to_generate` % `to_wait_for` != 0 then the function will not terminate.
fn barrier_example(to_wait_for: usize, to_generate: usize)
{
        let n = to_generate;
        let mut handles = Vec::with_capacity(n);
        let barrier = Arc::new(Barrier::new(to_wait_for));
        println!("To wait for: {} -- To generate: {}\n", to_wait_for, to_generate);
        for i in 0..n {
                let c = Arc::clone(&barrier);
                // The same messages will be printed together.
                // You will NOT see any interleaving.
                handles.push(thread::spawn(move || {
                               println!("{}: before wait", i);
                               c.wait();
                               println!("{}: after wait", i);
                       }));
        }
        // Wait for other threads to finish.
        for handle in handles {
                handle.join().unwrap();
        }
        println!("\nTo wait for: {} -- To generate: {}", to_wait_for, to_generate);
}
