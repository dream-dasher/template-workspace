//! # Drop w/ Const & Static
//!
//! ## Notes
//!  - **Clone**
//!   - vibe:
//!     - immutable (immutable ref like)
//!     - config & reference values w/ only compile-time-info
//!   - inline
//!   - creates "temporary values", which *do* run `drop()`
//!
//!  - **Static**
//!    - vibe:
//!      - mutable-ish (often used with `interior-mutability` pattern; also mutable via `unsafe`)
//!      - shared values w/ runtime info
//!    - not inlined; typically referenced
//!    - cloning creates a **non**-static opy
//!
/*!
## Utility Lines:
clear; RUST_LOG=xp_drop=trace cargo run --quiet --bin xp-drop
!*/

use tracing::{self as tea, Level, instrument};
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

const CONST_ZERO_WITH_DESTRUCTOR: TypeWithDestructor = TypeWithDestructor(0);
static STATIC_ZERO_WITH_DESTRUCTOR: TypeWithDestructor = TypeWithDestructor(111);

#[derive(Debug, Clone)]
struct TypeWithDestructor(i32);
impl Drop for TypeWithDestructor {
        #[instrument]
        fn drop(&mut self) {
                println!(" --> Dropped. Held {}.", self.0);
        }
}

#[instrument]
fn create_zero_with_destructor() -> (TypeWithDestructor, TypeWithDestructor) {
        let x = CONST_ZERO_WITH_DESTRUCTOR;
        let y = STATIC_ZERO_WITH_DESTRUCTOR.clone();
        tracing::debug!("Val of 'x': {}", x.0);
        tracing::debug!("Val of 'y': {}", y.0);
        (x, y)
}

#[derive(Debug)]
#[expect(dead_code)]
struct HasConstField {
        pub constantine: TypeWithDestructor,
        pub field:       i32,
}

fn main() {
        tracing_subscriber::fmt()
                .with_env_filter(EnvFilter::from_default_env().add_directive(Level::DEBUG.into()))
                .with_span_events(FmtSpan::NONE)
                .init();
        tea::trace!("Starting...");

        println!("------------------------");
        {
                let _enter = tea::span!(Level::INFO, "NoAssignDrop").entered();
                tea::info!("v________________________v");
                create_zero_with_destructor();
                tea::info!("^________________________^");
        }

        println!("------------------------");
        {
                let _enter = tea::span!(Level::INFO, "ExplicitAssign").entered();
                tea::info!("v________________________v");
                let (assigned_const, assigned_static) = create_zero_with_destructor();
                tea::info!(?assigned_const, "Assigned const from create_and_drop fn");
                tea::info!(?assigned_static, "Assigned cloned static from create_and_drop fn");
                tea::info!("^________________________^");
        }

        println!("------------------------");
        {
                let _enter = tea::span!(Level::INFO, "StructFieldAssign").entered();
                tea::info!("v________________________v");
                let some_struct_with_const = HasConstField {
                        constantine: CONST_ZERO_WITH_DESTRUCTOR,
                        field:       100001,
                };
                let some_struct_with_static = HasConstField {
                        constantine: STATIC_ZERO_WITH_DESTRUCTOR.clone(),
                        field:       8118,
                };
                tea::info!(?some_struct_with_const, "Assigned const from create_and_drop fn");
                tea::info!(?some_struct_with_static, "Assigned cloned static from create_and_drop fn");
                tea::info!("^________________________^");
        }
        println!("------------------------");
}
