# Scout
<img src="/assets/scout.png" alt="Scout in a Dark Forest" width="500"/>

Scout is an extensible open-source tool to assist Rust Polkadot / Kusama smart contract developers to detect common security issues and deviations from best practices.

## Detectors

| Detector ID | Category | Source | Description| Severity | Reviewed |
| ------ | ------ | ------| ------| ------ | ------ |
| integer-overflow-or-underflow | Arithmetic | Analysis Categories Wiki | [An arithmetic operation overflows or underflows the available memory allocated to the variable.](https://github.com/CoinFabrik/web3-grant/tree/main/docs/detectors/integer-overflow-or-underflow) | High | Agus |
| set-contract-storage | Authorization | Tiki | [Insufficient access control on set_contract_storage() function.](https://github.com/CoinFabrik/web3-grant/tree/main/docs/detectors/set-contract-storage) | High | Tiki, Turi |
| reentrancy            | Reentrancy       | Analysis Categories Wiki & Deployed ink! projects | [Consistency of contract state under recursive calls.](https://github.com/CoinFabrik/web3-grant/tree/main/docs/detectors/reentrancy)                                                                         | High     | Tiki, Turi |
| panic-error            | Validations and error handling       | Analysis Categories Wiki & Deployed ink! projects | [Code panics on error instead of using descriptive enum](https://github.com/CoinFabrik/web3-grant/tree/main/docs/detectors/panic-error)                                                                         | Informational     | Agus |
| unused-return-enum            | Validations and error handling       | Analysis Categories Wiki & Deployed ink! projects | [Return enum from a function is not completely used](https://github.com/CoinFabrik/web3-grant/tree/main/docs/detectors/unused-return-enum) | Low     | Agus |
| dos-unbounded-operation | DoS | Analysis Categories Wiki & Deployed ink! projects | [Return enum from a function is not completely used](https://github.com/CoinFabrik/web3-grant/tree/main/docs/detectors/dos-unbounded-operation) | High | Agus |
| dos-unexpected-revert | DoS | Analysis Categories Wiki & Deployed ink! projects | [DoS due to improper storage.](https://github.com/CoinFabrik/web3-grant/tree/main/docs/detectors/unexpected-revert) | High | Tiki, Turi |

## References

Similar Tools from other Blockchains:
- [Slither](https://github.com/crytic/slither): Static analyzer for solidity.
- [Rustle](https://github.com/blocksecteam/rustle): Static analyzer for Near.

Candidate tools to fork/work with:
- [Dylint](https://github.com/trailofbits/dylint): Rust linting tool, dynamic set of lints. Based on Clippy, it can replicate Clippy.
- [Clippy](https://github.com/rust-lang/rust-clippy): Rust linting tool, static set of lints.
