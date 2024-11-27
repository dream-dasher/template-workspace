//! Playing mutability

use tracing::{Level, event, instrument, span};

fn main()
{
        tracing_subscriber::fmt::init();
        let vectorio = vec![1, 2, 3, 4, 5];
        let vectorio_reference_1 = &vectorio;
        let vectorio_reference_2 = &vectorio;
        event!(Level::INFO, ?vectorio);
        event!(Level::INFO, ?vectorio_reference_1);
        event!(Level::INFO, ?vectorio_reference_2);

        let mut vectart = vec![5, 4, 3, 2, 1];
        let mut vectart_mut_reference_1 = &mut vectart;
        // let vectart_mut_reference_2 = &mut vectart;
        // event!(Level::INFO, ?vectart);
        event!(Level::INFO, ?vectart_mut_reference_1);
        // event!(Level::INFO, ?vectart_mut_reference_2);
        let vectart_mut_reference_1_mutref = &mut vectart_mut_reference_1;
        event!(Level::INFO, ?vectart_mut_reference_1_mutref);
        event!(Level::INFO, ?vectart_mut_reference_1);
        // event!(Level::INFO, ?vectart_mut_reference_1_mutref);
        event!(Level::INFO, ?vectart);

        let mut vectaline = vec![2, 1, 5, 1, 2];
        let vectaline_taker_1 = vectaline;
        // let vectaline_taker_2 = vectaline;
        // event!(Level::INFO, ?vectaline);
        event!(Level::INFO, ?vectaline_taker_1);
        // event!(Level::INFO, ?vectaline_taker_2);
        // let vectaline_taker_2 = vectaline;
}

#[instrument]
fn _refcell_example()
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
