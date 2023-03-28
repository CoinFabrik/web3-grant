#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod panic_error {

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
        pub fn add(&mut self, value: u32)   {
            match self.value.checked_add(value) {
                Some(v) => self.value = v,
                None => panic!("Overflow error"),
            };
        }
    }
}