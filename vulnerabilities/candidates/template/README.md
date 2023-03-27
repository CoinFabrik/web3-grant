# Vulnerability Name

## Configuration

- Detector ID: `detector-name`
- Analysis Category: `Analysis Category`
- Severity: `Severity`

## Description

Some description of the vulnerability.

## Exploit Scenario

An exploit scenario with code snippets describing the vulnerability.

```rust
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
```

### Deployment

[Optional] Observations about deployment.

### Tutorial

[Optional] Reference to video tutorial deploying, running and explaining the exploit.

## Remediation

Explanation with code snippets on how to remediate the vulnerability.

## References

- Url to some useful reference to this type of vulnereability
- Another url to some reference
