pub use const_drop::*;

#[derive(Debug)]
pub struct HasConstField {
        pub constantine: TypeWithDestructor,
        pub field:       i32,
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
