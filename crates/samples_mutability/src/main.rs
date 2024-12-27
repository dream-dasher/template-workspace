//! Playing mutability
//!
//! clear; RUST_LOG=samples_mutability=trace carrbn samples_mutability

use std::cell::RefCell;

use tracing::{self as tea, Level, instrument, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, prelude::*};

fn main() {
        tracing_subscriber::Registry::default()
                .with(tracing_tree::HierarchicalLayer::new(2))
                // .with(tracing_tree::HierarchicalLayer::new(2)
                //         .with_timer(Uptime::default())
                //         .with_span_modes(true)
                //         .with_indent_lines(true))
                .with(EnvFilter::builder()
                        .with_default_directive(LevelFilter::ERROR.into())
                        .from_env_lossy())
                .init();

        let span = tea::span!(Level::INFO, "View-Only Multi-Borrow");
        {
                let _guard = span.enter();
                let vectorio = vec![1, 2, 3, 4, 5];
                let vectorio_reference_1 = &vectorio;
                let vectorio_reference_2 = &vectorio;
                tea::debug!(?vectorio);
                tea::debug!(?vectorio_reference_1);
                tea::debug!(?vectorio_reference_2);
        }

        let span = tea::span!(Level::INFO, "Mutable-Access Exclusive-Borrow");
        {
                let _guard = span.enter();
                let mut vectart = vec![5, 4, 3, 2, 1];
                let mut vectart_mut_reference_1 = &mut vectart;
                // let vectart_mut_reference_2 = &mut vectart;
                // tea::debug!(?vectart);
                tea::debug!(?vectart_mut_reference_1);
                // tea::debug!(?vectart_mut_reference_2);
                let vectart_mut_reference_1_mutref = &mut vectart_mut_reference_1;
                tea::debug!(?vectart_mut_reference_1_mutref);
                tea::debug!(?vectart_mut_reference_1);
                // tea::debug!(?vectart_mut_reference_1_mutref);
                tea::debug!(?vectart);
        }

        let span = tea::span!(Level::INFO, "Responsibility Transfer, Exclusive");
        {
                let _guard = span.enter();
                let vectaline = vec![2, 1, 5, 1, 2];
                let vectaline_taker_1 = vectaline;
                // let vectaline_taker_2 = vectaline;
                // tea::debug!(?vectaline);
                tea::debug!(?vectaline_taker_1);
                // tea::debug!(?vectaline_taker_2);
                // let vectaline_taker_2 = vectaline;
        }

        let span = tea::span!(Level::INFO, "RefCell Exploration...");
        {
                let _guard = span.enter();
                let vecto = vec!['a', 'b', 'c', 'd', 'e'];
                let vecto_refcell = RefCell::new(vecto);
                let span = tea::span!(Level::INFO, "RefCell -- Immutable Borrow, Multiple");
                {
                        let _guard = span.enter();
                        let ref_borrow_a = vecto_refcell.borrow();
                        let ref_borrow_b = vecto_refcell.borrow();
                        let ref_borrow_c = vecto_refcell.borrow();
                        tea::debug!(?ref_borrow_a);
                        tea::debug!(?ref_borrow_b);
                        tea::debug!(?ref_borrow_c);
                }
                let span = tea::span!(Level::INFO, "RefCell -- Muttable Borrow, Exclusive");
                {
                        let _guard = span.enter();
                        tea::warn!(
                                "NOTE: we had to force scope to drop immutable borrows, BEFORE, taking immutable borrow.  No non-lexical scope inference."
                        );
                        let ref_mutborrow = vecto_refcell.borrow_mut();
                        tea::debug!(?ref_mutborrow);
                }
        }
}

#[instrument]
fn _refcell_example() {
        use std::{cell::{RefCell, RefMut},
                  collections::HashMap,
                  rc::Rc};

        tea::debug!("create shared map");
        let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
        // Create a new block to limit the scope of the dynamic borrow
        {
                tea::event!(Level::DEBUG, "mutable borrow of map");
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
        tea::debug!("total: {total}");
        tea::debug!("map: {:?}", shared_map.borrow());

        // Create a new block to limit the scope of the dynamic borrow
        {
                tea::event!(Level::DEBUG, "mutable borrow of map");
                let mut map: RefMut<'_, _> = shared_map.borrow_mut();
                map.insert("EEEEE", 25);
                map.insert("FFF_FFF", 36);
                map.insert("GGG_G_GGG", 49);
                map.insert("HHHH_HHHH", 64);
        }

        // Note that if we had not let the previous borrow of the cache fall out
        // of scope then the subsequent borrow would cause a dynamic thread panic.
        // This is the major hazard of using `RefCell`.
        tea::debug!("old total: {total}");
        tea::debug!("old map: {:?}", shared_map.borrow());
        let total: i32 = shared_map.borrow().values().sum();
        tea::debug!("total: {total}");
        tea::debug!("map: {:?}", shared_map.borrow());
}
