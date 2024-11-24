//! drops (and consts)

use xp_drop::*;

fn main() {
        // unsafe {
        //         std::env::set_var("RUST_LOG", "trace");
        // }
        tracing_subscriber::fmt::init();
        tracing::info!("Starting...");

        {
                println!();
                tracing::info!("Running create_and_drop fn...");
                create_zero_with_destructor();
                tracing::info!("End of create_and_drop fn.");
                tracing::info!("^________________________^");
        }
        {
                println!();
                tracing::warn!("Assignining const from create_and_drop fn...");
                let assigned = create_zero_with_destructor();
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
