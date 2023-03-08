# Using cargo-fuzz

[Cargo-Fuzz](https://github.com/rust-fuzz/cargo-fuzz) is a tool to invoke a fuzzer. 
Currently, the only fuzzer it supports 
is [libFuzzer](https://rust-fuzz.github.io/book/cargo-fuzz.html) 
(through the [libfuzzer-sys](https://github.com/rust-fuzz/libfuzzer-sys) crate), but it could be extended to support other fuzzers in the future.

## Requirements

LibFuzzer needs LLVM sanitizer support, so this only works on x86-64 Linux, x86-64 macOS and Apple-Silicon (aarch64) macOS for now. Requires a C++ compiler with C++11 support. Rust provides multiple compilers. This project requires the nightly compiler since it uses the -Z compiler flag to provide address sanitization. Assuming you used rustup to install Rust, you can check your default compiler with:


```
$ rustup default
stable-x86_64-unknown-linux-gnu (default) # Not the compiler we want.
```

To change to the nightly compiler:


```
$ rustup install nightly
$ rustup default nightly
nightly-x86_64-unknown-linux-gnu (default) # The correct compiler.
```

## Installing

`$ cargo install cargo-fuzz`

## Upgrading

`$ cargo install --force cargo-fuzz`

## Executing LibFuzzer with cargo-fuzz

A manually written fuzz target can be found in fuzz/fuzz_targets/fuzz_target_1.rs.

### Inputs
First we define the input as three unsigned 8-bit integers.
LibFuzzer will generate values for these inputs
```
#[derive(Clone, Debug, Arbitrary)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Input {
  pub valueForNew: u8,
  pub valueForAdd: u8,
  pub valueForSub: u8,
}
```

### Fuzz Target

Then, we define the fuzz target that LibFuzzer is going to invoke with the generated input value.
```
fuzz_target!(|input: Input| {
    let mut contract = integer_overflow_underflow::IntegerOverflowUnderflow::new(input.valueForNew);
    contract.add(input.valueForAdd);
    contract.sub(input.valueForSub);
    contract.get();
});
```

Finally, we execute the fuzz target with libFuzzer using cargo-fuzz

`
$ cargo fuzz run fuzz_target_1
`

## Ouput

When executed, LibFuzzer will signal a panic due to an
'attempt to add with overflow' 

```
thread '<unnamed>' panicked at 'attempt to add with overflow', /Users/jgaleotti/CLionProjects/web3-grant/cargo-fuzz/integer-overflow-or-underflow/src/lib.rs:19:13
```

The report will be completed with the used input. For example:
```
Output of `std::fmt::Debug`:

	Input {
	    valueForNew: 246,
	    valueForAdd: 246,
	    valueForSub: 242,
	}
```

## References

* [Fuzzing with cargo-fuzz](https://rust-fuzz.github.io/book/cargo-fuzz.html)