//! Playing with sync and relateed objects

use tracing::{Level, event, instrument};

fn main()
{
        tracing_subscriber::fmt::init();

        // barrier_example(10, 20);
        refcell_example();
}

#[instrument]
fn refcell_example()
{
        use std::{cell::{RefCell, RefMut},
                  collections::HashMap,
                  rc::Rc};

        event!(Level::INFO, "create shared map");
        let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
        // Create a new block to limit the scope of the dynamic borrow
        {
                event!(Level::DEBUG, "mutable borrow of map");
                let mut map: RefMut<'_, _> = shared_map.borrow_mut();
                map.insert("A", 1);
                map.insert("BB", 4);
                map.insert("CCC", 9);
                map.insert("DDDD", 16);
        }

        // Note that if we had not let the previous borrow of the cache fall out
        // of scope then the subsequent borrow would cause a dynamic thread panic.
        // This is the major hazard of using `RefCell`.
        let total: i32 = shared_map.borrow().values().sum();
        event!(Level::INFO, "total: {total}");
        event!(Level::INFO, "map: {:?}", shared_map.borrow());

        // Create a new block to limit the scope of the dynamic borrow
        {
                event!(Level::DEBUG, "mutable borrow of map");
                let mut map: RefMut<'_, _> = shared_map.borrow_mut();
                map.insert("EEEEE", 25);
                map.insert("FFF_FFF", 36);
                map.insert("GGG_G_GGG", 49);
                map.insert("HHHH_HHHH", 64);
        }

        // Note that if we had not let the previous borrow of the cache fall out
        // of scope then the subsequent borrow would cause a dynamic thread panic.
        // This is the major hazard of using `RefCell`.
        event!(Level::INFO, "old total: {total}");
        event!(Level::INFO, "old map: {:?}", shared_map.borrow());
        let total: i32 = shared_map.borrow().values().sum();
        event!(Level::INFO, "total: {total}");
        event!(Level::INFO, "map: {:?}", shared_map.borrow());
}

/// Example use of barrier.
/// Given a value, barrier will block thread progression until it has received that number of wait calls.
///
/// ## Note
/// if `to_generate` % `to_wait_for` != 0 then the function will not terminate.
#[instrument]
fn barrier_example(to_wait_for: usize, to_generate: usize)
{
        use std::{sync::{Arc, Barrier},
                  thread};

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
