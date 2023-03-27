#![cfg_attr(not(feature = "std"), no_std)]




#[ink::contract]
mod integer_overflow_underflow {
    #[ink(storage)]
    pub struct IntegerOverflowUnderflow {
        value: u8,
    }

    impl IntegerOverflowUnderflow {
        #[ink(constructor)]
        pub fn new(value: u8) -> Self {
            Self { value }
        }

        #[ink(message)]
        pub fn get(&self) -> u8 {
            let x: u8 = read_data();
            eval_data(x);
            return x;
        }
    }
}
