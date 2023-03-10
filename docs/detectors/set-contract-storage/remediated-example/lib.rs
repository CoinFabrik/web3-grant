#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod template {

    #[ink(storage)]
    pub struct Template {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl Template {
        /// Creates a new instance of Template contract.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Returns the current value of the stored `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let template = Template::new(false);
            assert_eq!(template.get(), false);
        }

        #[ink::test]
        fn flips_correctly() {
            let mut template = Template::new(false);
            assert_eq!(template.get(), false);
            template.flip();
            assert_eq!(template.get(), true);
        }
    }
}
