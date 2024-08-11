//! Another Example Script using Example Library in Example Workspace

fn main() {
        println!("Script 2, running...");

        let mut count = 0;
        let mut state_curried = || inner_state_hello(&mut count);
        for _ in 0..plus3(3) {
                state_curried();
        }
        println!();
        let mut count = 0;
        let state_curried = || inner_state_hello(&mut count); // not mut declaration, because mutable taken in function
        libraire::repeat_function_mutable(state_curried, plus3(2) as u8); // input is taken as mutable, despite original declaration
}

/// Curying `add` for '3' (left, though symmetric)
fn plus3(n: u32) -> u64 {
        libraire::add_ample_room_example(3, n)
}

fn inner_state_hello(state_holder: &mut u64) {
        *state_holder += 1;
        println!("Hello for the {}th time", state_holder)
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn test_plus3() {
                let result = plus3(2);
                assert_eq!(result, 5);
        }
}
