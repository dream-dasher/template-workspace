//! Example Integration Test file for Example Library in an Example Workspace

use libraire::add_ample_room_example;

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn it_works_integration_version() {
                let result = add_ample_room_example(7, 9);
                assert_eq!(result, 16);
        }
}
