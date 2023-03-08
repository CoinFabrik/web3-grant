# Dylint

Dylint is a Rust linting tool, similar to Clippy. But whereas Clippy runs a predetermined, static set of lints, Dylint runs lints from user-specified, dynamic libraries. Thus, Dylint allows developers to maintain their own personal lint collections.

## Installation

`$ cargo install cargo-dylint dylint-link`

## Usage

Add the following in the Cargo.toml project to analyse:
[workspace.metadata.dylint]

`libraries = [
    { path = "/PATH/TO/LINTER/" },
]`

and then run in the project folder

`$ cargo dylint --all --workspace` 


## References

- [Dylint](https://github.com/trailofbits/dylint): Running Rust lints from dynamic libraries
