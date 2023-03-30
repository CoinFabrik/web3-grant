# Dylint based detectors
We would like to introduce a new set of detectors that are powered by 
[Dylint](https://github.com/trailofbits/dylint) - a Rust linting tool. 
Similar to Clippy, Dylint can run lints to help identify potential issues
in code. However, unlike Clippy, Dylint can run lints from user-specified 
dynamic libraries instead of just a statically predetermined set.

This unique feature of Dylint makes it easier for developers to extend and
customize their own personal lint collections, leading to reduced compile 
and run cycles.

## Running lints
The next three steps, describe how to run lints from a dynamic library.

1. Install `cargo-dylint` and `dylint-link`:

```sh
$ cargo install cargo-dylint dylint-link
```

2. Add the following to the project you want to run the lint on:

```sh
[workspace.metadata.dylint]
libraries = [
    { git = "{{TBD_GITHUB_REPO}}", pattern = "detectors/dylint/smart_contract_linters/*" },
]
```

3. Run `cargo-dylint`:

```sh
$ cargo dylint --all --workspace
```

## References
- [Dylint](https://github.com/trailofbits/dylint): running Rust lints from dynamic libraries
