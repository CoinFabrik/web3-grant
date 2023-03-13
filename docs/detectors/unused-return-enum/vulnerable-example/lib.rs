#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod unused_return_enum {

    #[ink(storage)]
    pub struct UnusedReturnEnum {}

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum TradingPairErrors {
        Overflow,
    }

    impl UnusedReturnEnum {

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_percentage_difference(
            &mut self,
            value1: Balance,
            value2: Balance
        ) -> Result<Balance, TradingPairErrors>  {
            let absolute_difference = value1.abs_diff(value2);
            let sum = value1 + value2;
            let percentage_difference =
                match 100u128.checked_mul(absolute_difference / sum) {
                    Some(result) => result,
                    None => panic!("overflow!"),
            };
            return Err(TradingPairErrors::Overflow);
        }
    }
}