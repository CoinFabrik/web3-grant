# Scout

![https://img.shields.io/badge/license-MIT-green](https://img.shields.io/badge/license-MIT-green)

Scout is an extensible open-source tool to assist Rust Polkadot / Kusama smart contract developers to detect common security issues and deviations from best practices.

<p align="center">
  <img src="/assets/scout.png" alt="Scout in a Dark Forest" width="300" center  />
</p>

## Detectors

| Detector ID                       | Category                       | Source                                       | Description                                                                                                                                               | Severity      | Reviewed   |
| --------------------------------- | ------------------------------ | -------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- | ---------- |
| integer-overflow-or-underflow     | Arithmetic                     | Analysis Categories                          | [An arithmetic operation overflows or underflows the available memory allocated to the variable.](vulnerabilities/examples/integer-overflow-or-underflow) | High          | Agus       |
| set-contract-storage              | Authorization                  | Tiki                                         | [Insufficient access control on set_contract_storage() function.](vulnerabilities/examples/set-contract-storage)                                          | High          | Tiki, Turi |
| reentrancy                        | Reentrancy                     | Analysis Categories & Deployed ink! projects | [Consistency of contract state under recursive calls.](vulnerabilities/examples/reentrancy)                                                               | High          | Tiki, Turi |
| panic-error                       | Validations and error handling | Analysis Categories & Deployed ink! projects | [Code panics on error instead of using descriptive enum](vulnerabilities/examples/panic-error)                                                            | Informational | Agus       |
| unused-return-enum                | Validations and error handling | Analysis Categories & Deployed ink! projects | [Return enum from a function is not completely used](vulnerabilities/examples/unused-return-enum)                                                         | Low           | Agus       |
| dos-unbounded-operation           | DoS                            | Analysis Categories & Deployed ink! projects | [Return enum from a function is not completely used](vulnerabilities/examples/dos-unbounded-operation)                                                    | High          | Agus       |
| dos-unexpected-revert-with-vector | DoS                            | Analysis Categories & Deployed ink! projects | [DoS due to improper storage.](vulnerabilities/examples/dos-unexpected-revert-with-vector)                                                                | High          | Tiki, Turi |

## References

Similar tools from other blockchains:

- [Slither](https://github.com/crytic/slither): Static analyzer for Solidity.
- [Rustle](https://github.com/blocksecteam/rustle): Static analyzer for Near.

Candidate tools to fork/work with:

- [Clippy](https://github.com/rust-lang/rust-clippy): Rust linting tool, static set of lints.
- [Dylint](https://github.com/trailofbits/dylint): Rust linting tool, dynamic set of lints. Based on Clippy, it can replicate Clippy.
- [Semgrep](https://github.com/returntocorp/semgrep): A lightweight static analysis tool with beta support for Rust.
- [Cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz): A cargo subcommand for fuzzing with libFuzzer.
