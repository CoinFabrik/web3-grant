# Scout
<img src="./scout.png" alt="Scout in a Dark Forest" width="500"/>

Scout is an extensible open-source tool to assist Rust Polkadot / Kusama smart contract developers to detect common security issues and deviations from best practices.

## Detectors

| Detector ID | Category | Source | Description| Severity | Reviewed |
| ------ | ------ | ------| ------| ------ | ------ |
| integer-overflow-or-underflow | Arithmetic | Analysis Categories Wiki | [An arithmetic operation overflows or underflows the available memory allocated to the variable.](https://github.com/CoinFabrik/web3-grant/blob/main/docs/detectors/integer-overflow-or-underflow/integer-overflow-or-underflow.md) | High | Agus |
| set-contract-storage | Authorization | Tiki | [Insufficient access control on set_contract_storage() function.](https://github.com/CoinFabrik/web3-grant/blob/main/docs/detectors/set-contract-storage/set-contract-storage.md) | High | Tiki, Turi |
| reentrancy            | Reentrancy       | Analysis Categories Wiki & Deployed ink! projects | [Consistency of contract state under recursive calls.](https://github.com/CoinFabrik/web3-grant/tree/main/docs/detectors/reentrancy/reentrancy.md)                                                                         | High     | Tiki, Turi |
| panic-error            | Validations and error handling       | Analysis Categories Wiki & Deployed ink! projects | [Code panics on error instead of using descriptive enum](https://github.com/CoinFabrik/web3-grant/tree/main/docs/detectors/panic-error/panic-error.md)                                                                         | Informational     | Agus |


## References

Similar Tools from other Blockchains:
- [Slither](https://github.com/crytic/slither): Static analyzer for solidity.
- [Rustle](https://github.com/blocksecteam/rustle): Static analyzer for Near.

Candidate tools to fork/work with:
- [Dylint](https://github.com/trailofbits/dylint): Rust linting tool, dynamic set of lints. Based on Clippy, it can replicate Clippy.
- [Clippy](https://github.com/rust-lang/rust-clippy): Rust linting tool, static set of lints.


