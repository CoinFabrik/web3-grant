# Cargo Fuzz based detectors

We present a new set of detectors based on [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz), a cargo subcommand for easy fuzzing using `libFuzzer`.

As a Rust library, cargo-fuzz is highly flexible and can be easily extended to support other fuzzers in the future. Its powerful features, combined with the support of the Rust community, make it an essential tool to improve code quality and reliability.

For now, the only fuzzers provided are the ones located in the current directory. This set of fuzzers is aimed to present a possible path of dynamic analysis of smart contracts.

## Running fuzzers

Given that `libFuzzer` needs LLVM sanitizer support, this will only work on the following platforms:

- x86-64 Linux
- x86-64 macOS
- Apple-Silicon (aarch64) macOS

A C++ compiler with C++11 support must also be present.

In order to run the fuzzers, you'll need to follow the next steps:

1. Install `cargo-fuzz`:

```sh
$ cargo install cargo-fuzz
```

2. Go to the detector you want to run, for example:

```sh
$ cd integer-overflow-or-underflow
```

3. Run one of the fuzz targets, for example:

```sh
$ cargo fuzz run fuzz-add-overflows
```

## References

- [Cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz): a cargo subcommand for fuzzing with libFuzzer
