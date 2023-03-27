#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::integer_arithmetic)]

#[ink::contract]
mod integer_overflow_underflow {

    #[ink(storage)]
    pub struct IntegerOverflowUnderflow {
        value: u8,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// An overflow was produced while adding
        OverflowError,
        /// An underflow was produced while substracting
        UnderflowError,
    }

    impl IntegerOverflowUnderflow {
        #[ink(constructor)]
        pub fn new(value: u8) -> Self {
            Self { value }
        }

        #[ink(message)]
        pub fn add(&mut self, value: u8) -> Result<(), Error> {
            match self.value.checked_add(value) {
                Some(v) => self.value = v,
                None => return Err(Error::OverflowError),
            };
            Ok(())
        }

        #[ink(message)]
        pub fn sub(&mut self, value: u8) -> Result<(), Error> {
            match self.value.checked_sub(value) {
                Some(v) => self.value = v,
                None => return Err(Error::UnderflowError),
            };
            Ok(())
        }

        #[ink(message)]
        pub fn get(&self) -> u8 {
            self.value
        }
    }
}
