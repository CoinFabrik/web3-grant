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
        pub fn add(&mut self, value: u8) {
            self.value += value;
        }

        #[ink(message)]
        pub fn sub(&mut self, value: u8) {
            self.value -= value;
        }

        #[ink(message)]
        pub fn get(&self) -> u8 {
            self.value
        }
    }
}