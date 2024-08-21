//! drops (and consts)

use const_drop::ZERO_WITH_DESTRUCTOR;

fn main() {
        std::env::set_var("RUST_LOG", "trace");
        tracing_subscriber::fmt::init();
        tracing::info!("Starting...");

        {
                println!();
                tracing::info!("Running create_and_drop fn...");
                const_drop::create_zero_with_destructor();
                tracing::info!("End of create_and_drop fn.");
                tracing::info!("^________________________^");
        }
        {
                println!();
                tracing::warn!("Assignining const from create_and_drop fn...");
                let assigned = const_drop::create_zero_with_destructor();
                tracing::warn!(?assigned, "Assigned const from create_and_drop fn");
                tracing::warn!("^________________________^");
        }
        {
                println!();
                tracing::debug!("Creating struct with const field...");
                let some_struct = HasConstField { constantine: ZERO_WITH_DESTRUCTOR, field: 42 };
                tracing::warn!(?some_struct, "Assigned const from create_and_drop fn");
                tracing::debug!("^________________________^");
        }
}

#[derive(Debug)]
struct HasConstField {
        constantine: const_drop::TypeWithDestructor,
        field:       i32,
}

mod const_drop {
        pub const ZERO_WITH_DESTRUCTOR: TypeWithDestructor = TypeWithDestructor(0);

        #[derive(Debug)]
        pub struct TypeWithDestructor(i32);
        impl Drop for TypeWithDestructor {
                fn drop(&mut self) {
                        // tracing::error!("Dropped. Held {}.", self.0);
                        println!(" --> Dropped. Held {}.", self.0);
                }
        }

        #[tracing::instrument]
        pub fn create_zero_with_destructor() -> TypeWithDestructor {
                let x = ZERO_WITH_DESTRUCTOR;
                tracing::trace!("Val of 'x': {}", x.0);
                x
        }
}
