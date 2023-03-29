# Scout: Security Analysis Tool

![https://img.shields.io/badge/license-MIT-green](https://img.shields.io/badge/license-MIT-green)

Towards the aim of building extensible open-source set of tools to assist 
`ink!` smart contract developers in the detection of security issues, we
worked on three deliverables as committed in the proposal:
- A [curated list of ink! Vulnerabilities](./curated-list-of-vulnerabilities/README.md).
- A [set of smart contracts written in ink! including the above vulnerabilities](./vulnerabilities/README.md), and
- [Proof-of-concept tools detecting the vulnerabilities in these smart contracts](./detectors/README.md)

In short, we worked to have a list of relevant vulnerabilities that represents the
security issues that can be found in `ink!` smart contracts. This list is not 
intended to be exhaustive nor ordered by any other criteria. We then worked in
finding (smnall) smart contracts with these vulnerabilities, often introducing
ourselves the vulnerabilities by modifying a seemingly-safe contract. Finally,
we selected a set of static/dynamic analysis tools that are effective in analyzing
Rust code, open source, well maintained, and (more importantly) can be used to
detect vulnerabilities in `ink!` smart contracts. We went to implement detectors
for all the vulnerabilities we had compiled, sometimes having more than one 
detector (tool) for a vulnerability. These tools/detectors are checked to have
a quality consistent with a proof-of-concept and are probably not product-ready.

The table below summarizes the vulnerability classes we selected, the specific
vulnerable smart contracts, and the detectors we designed. Details follow in the 
folder/repo for each deliverable ([1](./curated-list-of-vulnerabilities/README.md),
[2](./vulnerabilities/README.md), and [3](./detectors/README.md)).
We end this document with a [Future Work](#future-work) section suggesting the
work ahead. 

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


## Future Work
In the month this project lasted we had limited time to develop vulnerable smart 
contracts, learn how to build detectors for these vulnerabilities using the 
selected tools (dylint, semgrep & cargo-fuzz). 
The precision of the tool we have built should be analyzed, and the detectors
tuned to improve precision and recall. 
This amounts to compiling a larger set of smart contracts, vulnerable and not, 
and having the detectors detect vulnerabilities effectively while not generating 
false positives. 

This can be attained by different means. As there is not a large size of security
audit reports available for public ink! smart contracts, an annotated base of
vulnerable smart contracts would be very useful in this endeavor.
We suggest that spending time in analyzing the ink! smart contracts that are 
being deployed with our toolset and a manual/assisted audit process. Hence,
we would find false positives, false negatives, true positives (and true negatives)
for our toolset and work on adding detectors or tuning the existing ones in order
to improve the overall precision and recall. Integrating the tools into a single
prototype and making it available to ink! developers would also help the community
in finding bugs and us builders in improving the tool. 

Then we would turn this prototype into a robust product-quality tool, integrated 
with IDEs, and improve its documentation accordingly.

