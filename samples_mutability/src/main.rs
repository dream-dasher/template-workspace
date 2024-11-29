//! Playing mutability
//!
//! clear; RUST_LOG=samples_mutability=trace carrbn samples_mutability

use std::cell::RefCell;

use tracing::{Level, debug, error, event, info, instrument, level_filters::LevelFilter, span, trace, warn};
use tracing_subscriber::{EnvFilter, prelude::*};
use tracing_tree::time::Uptime;

fn main()
{
        tracing_subscriber::Registry::default()
                .with(
                        tracing_tree::HierarchicalLayer::new(2), // .with_timer(Uptime::default())
                                                                 // .with_span_modes(true),
                                                                 // .with_indent_lines(true)
                )
                .with(EnvFilter::builder().with_default_directive(LevelFilter::ERROR.into()).from_env_lossy())
                .init();

        let span = span!(Level::INFO, "View-Only Multi-Borrow");
        {
                let _guard = span.enter();
                let vectorio = vec![1, 2, 3, 4, 5];
                let vectorio_reference_1 = &vectorio;
                let vectorio_reference_2 = &vectorio;
                debug!(?vectorio);
                debug!(?vectorio_reference_1);
                debug!(?vectorio_reference_2);
        }

        let span = span!(Level::INFO, "Mutable-Access Exclusive-Borrow");
        {
                let _guard = span.enter();
                let mut vectart = vec![5, 4, 3, 2, 1];
                let mut vectart_mut_reference_1 = &mut vectart;
                // let vectart_mut_reference_2 = &mut vectart;
                // debug!(?vectart);
                debug!(?vectart_mut_reference_1);
                // debug!(?vectart_mut_reference_2);
                let vectart_mut_reference_1_mutref = &mut vectart_mut_reference_1;
                debug!(?vectart_mut_reference_1_mutref);
                debug!(?vectart_mut_reference_1);
                // debug!(?vectart_mut_reference_1_mutref);
                debug!(?vectart);
        }

        let span = span!(Level::INFO, "Responsibility Transfer, Exclusive");
        {
                let _guard = span.enter();
                let mut vectaline = vec![2, 1, 5, 1, 2];
                let vectaline_taker_1 = vectaline;
                // let vectaline_taker_2 = vectaline;
                // debug!(?vectaline);
                debug!(?vectaline_taker_1);
                // debug!(?vectaline_taker_2);
                // let vectaline_taker_2 = vectaline;
        }

        let span = span!(Level::INFO, "RefCell Exploration...");
        {
                let _guard = span.enter();
                let vecto = vec!['a', 'b', 'c', 'd', 'e'];
                let vecto_refcell = RefCell::new(vecto);
                let span = span!(Level::INFO, "RefCell -- Immutable Borrow, Multiple");
                {
                        let _guard = span.enter();
                        let ref_borrow_a = vecto_refcell.borrow();
                        let ref_borrow_b = vecto_refcell.borrow();
                        let ref_borrow_c = vecto_refcell.borrow();
                        debug!(?ref_borrow_a);
                        debug!(?ref_borrow_b);
                        debug!(?ref_borrow_c);
                }
                let span = span!(Level::INFO, "RefCell -- Muttable Borrow, Exclusive");
                {
                        let _guard = span.enter();
                        warn!(
                                "NOTE: we had to force scope to drop immutable borrows, BEFORE, taking immutable borrow.  No non-lexical scope inference."
                        );
                        let ref_mutborrow = vecto_refcell.borrow_mut();
                        debug!(?ref_mutborrow);
                }
        }
}

#[instrument]
fn _refcell_example()
{
        use std::{cell::{RefCell, RefMut},
                  collections::HashMap,
                  rc::Rc};

        debug!("create shared map");
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
        debug!("total: {total}");
        debug!("map: {:?}", shared_map.borrow());

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
        debug!("old total: {total}");
        debug!("old map: {:?}", shared_map.borrow());
        let total: i32 = shared_map.borrow().values().sum();
        debug!("total: {total}");
        debug!("map: {:?}", shared_map.borrow());
}
