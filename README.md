# Scout: Security Analysis Tool

![https://img.shields.io/badge/license-MIT-green](https://img.shields.io/badge/license-MIT-green)

We built an extensible open-source set of tools to assist `ink!` (and Rust 
Polkadot / Kusama) smart contract developers to detect common security issues 
and deviations from best practices. 
We use tools implementing both static and dynamic analysis techniques to have
both coverage and precision/recall.


<p align="center">
  <img src="/assets/scout.png" alt="Scout in a Dark Forest" width="300" center  />
</p>

## Detectors

| Detector ID                   | Category                       | Description                                                                                                                                                                                        | Severity      | 
| ----------------------------- | ------------------------------ | ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | 
| integer-overflow-or-underflow | Arithmetic                     | [An arithmetic operation overflows or underflows the available memory allocated to the variable.](https://github.com/CoinFabrik/web3-grant/tree/main/vulnerabilities/examples/integer-overflow-or-underflow) | High          |
| set-contract-storage          | Authorization                  |  [Insufficient access control on set_contract_storage() function.](https://github.com/CoinFabrik/web3-grant/tree/main/vulnerabilities/examples/set-contract-storage)                                          | High          |
| reentrancy                    | Reentrancy                     | [Consistency of contract state under recursive calls.](https://github.com/CoinFabrik/web3-grant/tree/main/vulnerabilities/examples/reentrancy)                                                               | High          |
| panic-error                   | Validations and error handling |  [Code panics on error instead of using descriptive enum](https://github.com/CoinFabrik/web3-grant/tree/main/vulnerabilities/examples/panic-error)                                                            | Informational |
| unused-return-enum            | Validations and error handling |  [Return enum from a function is not completely used](https://github.com/CoinFabrik/web3-grant/tree/main/vulnerabilities/examples/unused-return-enum)                                                         | Low           |
| dos-unbounded-operation-with-vector       | Denial of Service               | [Return enum from a function is not completely used](https://github.com/CoinFabrik/web3-grant/tree/main/vulnerabilities/examples/dos-unbounded-operation-with-vector)                                                    | High          |
| dos-unexpected-revert         | Denial of Service              |  [DoS due to improper storage.](https://github.com/CoinFabrik/web3-grant/tree/main/vulnerabilities/examples/dos-unexpected-revert)                                                                                | High          |

## References
- [Clippy](https://github.com/rust-lang/rust-clippy): Rust linting tool, static set of lints.
- [Dylint](https://github.com/trailofbits/dylint): Rust linting tool, dynamic set of lints. Based on Clippy, it can replicate Clippy.
- [Semgrep](https://github.com/returntocorp/semgrep): A lightweight static analysis tool with beta support for Rust.
- [Cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz): A cargo subcommand for fuzzing with libFuzzer.


## Further Work
For the current list of vulnerable examples, Dylint seems to be a good option to use in the construction of a security analysis tool for Substrate Ink!. As new vulnerability examples are added to the list, Cargo-fuzz and Semgrep can be considered in parallel, especially in cases where the confidence of detectors implemented in Dylint is not satisfactory.

In particular, as Semgrep improves its Rust compatibility, it could be considered for detector building due to its ease of use and tainting capabilities.

