# UNUSED_RETURN_ENUM

### What it does
It warns if a fuction that returns a Result type does not return a Result enum variant (Ok/Err)


### Why is this bad?
If any of the variants (Ok/Err) is not used, the code could be simplified or it could imply a bug.


### Known problems
TODO.

### Example
```rust
// example code where a warning is issued
    #![cfg_attr(not(feature = "std"), no_std)]
    pub enum TradingPairErrors {
        Overflow,
    }
    (...)
    #[ink(message)]
    pub fn get_percentage_difference(&mut self, value1: Balance, value2: Balance) -> Result<Balance, TradingPairErrors>  {
        let absolute_difference = value1.abs_diff(value2);
        let sum = value1 + value2;
        let percentage_difference =
        match 100u128.checked_mul(absolute_difference / sum) {
           Some(result) => result,
           None => Err(TradingPairErrors::Overflow),
        }
    }
```
Use instead:
```rust
// example code that does not raise a warning
    #![cfg_attr(not(feature = "std"), no_std)]
    pub enum TradingPairErrors {
        Overflow,
    }
    (...)
    #[ink(message)]
    pub fn get_percentage_difference(&mut self, value1: Balance, value2: Balance) -> Result<Balance, TradingPairErrors>  {
        let absolute_difference = value1.abs_diff(value2);
        let sum = value1 + value2;
        let percentage_difference =
        match 100u128.checked_mul(absolute_difference / sum) {
           Some(result) => Ok(result),
           None => panic!("overflow!"),
        };
        return Err(TradingPairErrors::Overflow);
    }
```
