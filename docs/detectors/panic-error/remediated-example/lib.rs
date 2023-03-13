#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod panic_error {

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// An overflow was produced while adding
        OverflowError,
    }

    #[ink(storage)]
    pub struct PanicError {
        value: u32,
    }

    impl PanicError {
        #[ink(constructor)]
        pub fn new(value: u32) -> Self {
            Self { value }
        }

        #[ink(message)]
        pub fn add(&mut self, value: u32) -> Result<(), Error>  {
            match self.value.checked_add(value) {
                Some(v) => self.value = v,
                None => return Err(Error::OverflowError),
            };
            Ok(())
        }
    }
}