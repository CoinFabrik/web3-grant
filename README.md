# Scout

Scout is an extensible open-source tool to assist Rust Polkadot / Kusama smart contract developers to detect common security issues and deviations from best practices.

## Detectors

| Detector ID | Category | Source | Description| Severity | Reviewed |
| ------ | ------ | ------| ------| ------ | ------ |
| integer-overflow-or-underflow | Arithmetic | Analysis Categories Wiki | [An arithmetic operation overflows or underflows the available memory allocated to the variable.](https://github.com/CoinFabrik/web3-grant/blob/main/docs/detectors/integer-overflow-or-underflow/integer-overflow-or-underflow.md) | High | Agus |
| set-contract-storage | Authorization | Tiki | [Insufficient access control on set_contract_storage() function.](https://github.com/CoinFabrik/web3-grant/blob/main/docs/detectors/set-contract-storage/set-contract-storage.md) | High | Tiki |

## Detectors Under Review by Coinfabrik

| Detector ID | Category | Source | Description | Severity | Reviewing | Status | 
|----------------------|------------------|--------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------|----------|----------|---------------------------|
| reentrancy            | Reentrancy       | Analysis Categories Wiki & Deployed ink! projects | [Consistency of contract state under recursive calls.](https://github.com/CoinFabrik/web3-grant/tree/main/docs/candidates/potential/reentrancy)                                                                         | High     | Tiki, Turi | 3-Documenting             |
| unexpected-revert     | DoS              | Analysis Categories Wiki                        | [Unexpected revert occurs when the access to a function is blocked by forcing it to revert.](https://gitlab.com/coinfabrik-private/coinfabrik-wiki/-/wikis/Auditing/Analyses/DoS/Unexpected-revert)                                    | ?        | FALSE    | 2-Under Review Coinfabrik |
| block-gas-limit       | DoS              | Analysis Categories Wiki                        | [Block Gas Limit happens when an attacker floods the execution of a function with so much data that it hits the gas limit of the block, causing the transaction to revert.](https://gitlab.com/coinfabrik-private/coinfabrik-wiki/-/wikis/Auditing/Analyses/DoS/Block-gas-limit) | ?        | FALSE    | 2-Under Review Coinfabrik |
| weak-randomness       | Block attributes | Analysis Categories Wiki                        | [Insecure source of randomness through block attributes.](https://gitlab.com/coinfabrik-private/coinfabrik-wiki/-/wikis/Auditing/Analyses/Block-attributes/Use-of-insufficiently-random-values)                                                                         | ?        | FALSE    | 2-Under Review Coinfabrik |
| time-manipulation     | Block attributes | Analysis Categories Wiki                        | [Using block attributes in order to determine time can lead to manipulation by miners.](https://gitlab.com/coinfabrik-private/coinfabrik-wiki/-/wikis/Auditing/Analyses/Block-attributes/Time-manipulation)                                        | ?        | FALSE    | 2-Under Review Coinfabrik |

We prioritize analysis on issues that appear on [deployed projects](https://docs.google.com/spreadsheets/d/19fVqHwQwfhWUBfHppiEnr9yJ9Ep-qr_czGSSkTvKn5E/edit?usp=share_link) as potential vulnerabilities by chatGPT.

Take a look at [this](https://docs.google.com/spreadsheets/d/1mCE1KIXSngQP8VsY7fo4UqH_QL_7VKQ0JSUvhGuY4Rs/edit#gid=0) table for other detector candidates.

## Methodology

We have two sources for candidate for examples to be detected (detectors):

1. The [list of audits of Substrate Polkadot projects by auditing companies](https://docs.google.com/spreadsheets/d/1xQ-RTui38vTAXKIbBOLZmbUEvHjTGrbdRvbG12c7n-8/edit#gid=0). In order to get candidates we look at the audit's findings and the associated github repo.

2. Deployed [ink! projects](https://docs.google.com/spreadsheets/d/19fVqHwQwfhWUBfHppiEnr9yJ9Ep-qr_czGSSkTvKn5E/edit?usp=share_link).

3. The [Analysis Categories Wiki](https://gitlab.com/coinfabrik-private/coinfabrik-wiki/-/wikis/Auditing/Analyses). In order to get candidates we search the wiki for examples in solidity and try to generate the candidate in ink! using ChatGPT with the following prompt below. If the type of vulnerability is possible, a new line is added to the Detectors list. If the type of vulnerability is not possible, a new line is added to the [candidates table](https://docs.google.com/spreadsheets/d/1mCE1KIXSngQP8VsY7fo4UqH_QL_7VKQ0JSUvhGuY4Rs/edit#gid=0).

Chat GPT Prompt:

Determine whether vulnerabilities associated to <eg: priviledged roles> are possible in polkadot substrate.

If this type of vulenerability is possible, provide a simple code example of a smart contract in ink!, where the only vulnerability found is associated to <eg: priviledged roles>. Provide details on how to deploy this smart contract, interact with it, and exploit its vulnerability. Indicate the severity of this vulnerability."

If this type of vulnerability is not possible, explain why it is not possible in substrate polkadot.


## References

Similar Tools from other Blockchains:
- [Slither](https://github.com/crytic/slither): Static analyzer for solidity.
- [Rustle](https://github.com/blocksecteam/rustle): Static analyzer for Near.

Candidate tools to fork/work with:\
- [Dylint](https://github.com/trailofbits/dylint): Rust linting tool, dynamic set of lints. Based on Clippy, it can replicate Clippy.
- [Clippy](https://github.com/rust-lang/rust-clippy): Rust linting tool, static set of lints.


