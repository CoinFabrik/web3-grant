# Dylint based detectors

We present a set of detectors that relie on the dylint tool.   
dylint is a Rust linting tool, similar to Clippy but instead  of running a statically predetermined set of lints, dylint runs lints from user-specified, dynamic libraries. 

dylint is easier to extend and reduce the compile/run cicle,  allowing developers to maintain their own personal lint collections.

## Installation

```sh
$ cargo install cargo-dylint dylint-link`
```

## Usage
1) Create your linter. For instance [set_contract_storage](https://github.com/CoinFabrik/web3-grant/tree/dylint/docs/dylint/set_contract_storage)

2) Go to the project you want to analyze and the following annotation in the Cargo.toml:

```sh
[workspace.metadata.dylint]
libraries = [
    { path = "/PATH/TO/LINTER/" },
]
```
and then run in the project folder

```sh
`$ cargo dylint --all --workspace` 
```

## References

- [Dylint](https://github.com/trailofbits/dylint): Running Rust lints from dynamic libraries
