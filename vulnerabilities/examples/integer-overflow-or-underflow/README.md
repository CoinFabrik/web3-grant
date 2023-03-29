# Integer Overflow or Underflow

## Configuration

* Detector ID: `integer-overflow-or-underflow`
* Analysis Category: `Arithmetic`
* Severity: `High`


## Description

These types of vulnerabilities are commonly referred to as "integer overflow" and "integer underflow" vulnerabilities, and they can occur when an arithmetic operation overflows or underflows the available memory allocated to the variable.

## Exploit Scenario

[Here's](vulnerable-example/lib.rs) an example of a simple ink! smart contract that could be vulnerable to an integer overflow vulnerability. The problematic functions are the following ones:

```rust
#[ink(message)]
pub fn add(&mut self, value: u8) {
    self.value += value;
}

#[ink(message)]
pub fn sub(&mut self, value: u8) {
    self.value -= value;
}
```

This contract stores a single value of type `u8` and provides three functions for interacting with it. The `add` function allows users to add a specified amount to the stored value, the `sub` function allows users to substract a specified amount, while the `get` function allows users to retrieve the current value.

However, this contract is vulnerable to an integer overflow attack if a user tries to add a value that exceeds the maximum value that can be stored in a `u8` variable. If the result of the addition operation overflows the available memory allocated to the variable, the value will wrap around to zero, potentially leading to unexpected behavior.

This vulnerability is **only** present if overflow and underflow checks are disabled at the time of compilation. We can disable it by adding to the `Cargo.toml` file the following configuration:

```toml
[profile.release]
overflow-checks = false
```

This way, the overflow checks will be disabled whenever the contract is built using the `release` profile. More info can be found [here](https://doc.rust-lang.org/cargo/reference/profiles.html).

To deploy this smart contract, you would need to compile it using the ink! compiler and deploy it to a Polkadot Substrate network using a suitable deployment tool such as Polkadot JS. Once deployed, users could interact with the contract by calling its functions using a compatible wallet or blockchain explorer.

### Deployment

Before deployment, the contract must be built using the tool `cargo-contract`:

```shell
cargo contract build --release
```

Following that, the contract can be deployed either by using `cargo-contract` or a GUI tool such as the one available on https://contracts-ui.substrate.io/:

```shell
cargo contract instantiate --constructor new --args 0 --suri //Alice
```

### Possible Prevention Tools

By leveraging `clippy`, Rust's linting tool, the user can enable a certain set of rules that disallows the usage of arithmetic operators (`+`, `-`, `*`, `/`).  The following lines could be added at the top of the contract file in order to enable them:

```rust
#![deny(clippy::integer_arithmetic)]
```

which triggers the following error:

```shell
❯ cargo clippy
    Checking integer-overflow v0.1.0 (/Users/agustin/work/coinfabrik/web3-grant/vulnerabilities/integer-overflow)
error: integer arithmetic detected
  --> lib.rs:20:13
   |
20 |             self.value += value;
   |             ^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#integer_arithmetic
note: the lint level is defined here
  --> lib.rs:2:9
   |
2  | #![deny(clippy::integer_arithmetic)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^

error: integer arithmetic detected
  --> lib.rs:25:13
   |
25 |             self.value -= value;
   |             ^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#integer_arithmetic

error: could not compile `integer-overflow` due to 2 previous errors
```

### Tutorial

See this [tutorial](https://drive.google.com/file/d/1B9SCFUok8Rxo6enIuz-f83fHPpS4jY1H/view?usp=share_link) (in Spanish) showing this exploit in action.


## Remediation

The code should then be changed to explicitly use checked, overflowing or saturating arithmetics, as can be seen [here](remediated-example/lib.rs).

Particularly an `Error` enum can be added:

```rust
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    /// An overflow was produced while adding
    OverflowError,
    /// An underflow was produced while substracting
    UnderflowError,
}
```

And the problematic functions can be changed to:

```rust
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
```

Other rules could be added to improve the checking. The set of rules can be found [here](https://rust-lang.github.io/rust-clippy/master/).
