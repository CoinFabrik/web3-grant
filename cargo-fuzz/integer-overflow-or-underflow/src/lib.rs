#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
pub mod integer_overflow_underflow {

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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new() {
            let contract = IntegerOverflowUnderflow::new(42);
            assert_eq!(42, contract.value);
        }

        #[test]
        fn test_add() {
            let mut contract = IntegerOverflowUnderflow::new(42);
            contract.add(42);
            assert_eq!(84, contract.value);
        }

        #[test]
        fn test_sub() {
            let mut contract = IntegerOverflowUnderflow::new(42);
            contract.sub(10);
            assert_eq!(32, contract.value);
        }

        #[test]
        fn test_get() {
            let contract = IntegerOverflowUnderflow::new(42);
            assert_eq!(42, contract.get());
        }

    }

}


