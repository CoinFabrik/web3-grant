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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            // Arrange
            let value = 42;

            // Act
            let contract = IntegerOverflowUnderflow::new(value);

            // Assert
            assert_eq!(contract.get(), value);
        }

        #[ink::test]
        fn add_overflows() {
            // Arrange
            let mut contract = IntegerOverflowUnderflow::new(u8::MAX);

            // Act
            contract.add(1);

            // Assert
            assert_eq!(contract.get(), u8::MIN);
        }

        #[ink::test]
        fn sub_underflows() {
            // Arrange
            let mut contract = IntegerOverflowUnderflow::new(u8::MIN);

            // Act
            contract.sub(1);

            // Assert
            assert_eq!(contract.get(), u8::MAX);
        }
    }
}
