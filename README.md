# Scout

Scout is an extensible open-source tool to assist Rust Polkadot / Kusama smart contract developers to detect common security issues and deviations from best practices.

## Detectors

| Num | Detector ID | Category | Source | Description| Severity | Confidence | Reviewed | Linter |
| ------ | ------ | ------ | ------| ------| ------ | ------ | ------ | ------ |
| 1 | integer-overflow-or-underflow | Arithmetic | Analysis Categories Wiki | [An arithmetic operation overflows or underflows the available memory allocated to the variable.](https://github.com/CoinFabrik/web3-grant/blob/main/docs/detectors/integer-overflow-or-underflow/integer-overflow-or-underflow.md) | High | ? | Working vulnerability Agus | None |
| 10 | set_contract_storage | ? | Tiki | [Insufficient access control on set_contract_storage() function.](https://github.com/CoinFabrik/web3-grant/blob/main/docs/detectors/set-contract-storage/set-contract-storage.md)  | High | ? | Tiki | None |

## Detectors Under Review

| Num | Detector ID | Category | Source | Description| Severity | Confidence | Reviewed | Linter |
| ------ | ------ | ------ | ------| ------| ------ | ------ | ------ | ------ |
| 2 | precision-loss | Arithmetic |  Analysis Categories Wiki |  [Order of multiplications and divisions are important to ensure numerical precision.](https://gitlab.com/coinfabrik-private/web3-grant/scout/-/wikis/Scout/Detectors/Precision-Loss) | High | ? | False | None |
| 3 | incorrect-shift | Assembly Usage |  Analysis Categories Wiki |  [Arithmetic shift is performed with an incorrect or unexpected value.](https://gitlab.com/coinfabrik-private/web3-grant/scout/-/wikis/Scout/Detectors/Incorrect-shift) | High | ? | False | None |
| 4 | priviledged-roles | Authorization |  Analysis Categories Wiki |  [A vulnerability can arise if the roles and permissions are not properly defined or implemented, which can lead to unauthorized access to privileged functions and data.](https://gitlab.com/coinfabrik-private/web3-grant/scout/-/wikis/Scout/Detectors/Priviledged-roles) | ? | ? | False | None |\
| 5 | timeout | Validations and error handling |  [Audit-4](https://blog.quarkslab.com/resources/2022-02-27-xcmv2-audit/21-12-908-REP.pdf) |  ? | ? | ? | False | None |\
| 6 | pending-queries | Validations and error handling |  [Audit-4](https://blog.quarkslab.com/resources/2022-02-27-xcmv2-audit/21-12-908-REP.pdf) |  ? | ? | ? | False | None |\
| 7 | response-handling | Validations and error handling |  [Audit-4](https://blog.quarkslab.com/resources/2022-02-27-xcmv2-audit/21-12-908-REP.pdf) | ? | ? | ? | False | None |\
| 8 | threshold-not-enforced | Validations and error handling |  [Audit-7](https://raw.githubusercontent.com/parallel-finance/auditing-report/main/Halborn_Parallel_fi_Loans_Pallet_Substrate_Pallet_Security_Audit_Report_Final.pdf) | ? | ? | ? | False | None |
| 9 | missing-zero-check | Validations and error handling |  [Audit-7](https://raw.githubusercontent.com/parallel-finance/auditing-report/main/Halborn_Parallel_fi_Loans_Pallet_Substrate_Pallet_Security_Audit_Report_Final.pdf) | ? | ? | ? | False | None |

## Discarded Detectors

These candidates did not make it to the Detectors list because they were not relevant to substrate.

| Detector ID | Category | Source | Description| Reviewed |
| ------ | ------ | ------ | ------ | ------ |
| right-to-left-override-character | Assembly Usage |  Analysis Categories Wiki |  [An attacker can manipulate the logic of the contract by using a right-to-left-override character](banana) | False |
| tx.origin-vs-msg.sender | Authorization |  Analysis Categories Wiki |  [A contract using tx.origin to validate a user's identity is potentially insecure, since any call by an intermediate contract that the user interacts with would have the same value of tx.origin.](https://gitlab.com/coinfabrik-private/web3-grant/scout/-/wikis/Scout/Discarded-Detectors/Tx.origin-vs-msg.sender) | False |
| delegatecall | Authorization |  Analysis Categories Wiki | [Delegatecall allows a contract to call another contract's function while preserving context.](https://gitlab.com/coinfabrik-private/web3-grant/scout/-/wikis/Scout/Discarded-Detectors/Delegatecall) | False |

## Methodology

We have two sources for candidate for examples to be detected (detectors):

1. The [list of audits of Substrate Polkadot projects by auditing companies](https://docs.google.com/spreadsheets/d/1xQ-RTui38vTAXKIbBOLZmbUEvHjTGrbdRvbG12c7n-8/edit#gid=0). In order to get candidates we look at the audit's findings and the associated github repo.

2. The [Analysis Categories Wiki](https://gitlab.com/coinfabrik-private/coinfabrik-wiki/-/wikis/Auditing/Analyses). In order to get candidates we search the wiki for examples in solidity and try to generate the candidate in ink! using ChatGPT with the following prompt below. If the type of vulnerability is possible, a new line is added to the Detectors list. If the type of vulnerability is not possible, a new line is added to the Discarded Detectors list.

Chat GPT Prompt:

Determine whether vulnerabilities associated to <eg: priviledged roles> are possible in polkadot substrate.

If this type of vulenerability is possible, provide a simple code example of a smart contract in ink!, where the only vulnerability found is associated to <eg: priviledged roles>. Provide details on how to deploy this smart contract, interact with it, and exploit its vulnerability. Indicate the severity of this vulnerability."

If this type of vulnerability is not possible, explain why it is not possible in substrate polkadot.


## References

Similar Tools from other Blockchains:
- [Slither](https://github.com/crytic/slither): Static analyzer for solidity.\
- [Rustle](https://github.com/blocksecteam/rustle): Static analyzer for Near.\

Candidate tools to fork/work with:\
- [Dylint](https://github.com/trailofbits/dylint): Rust linting tool, dynamic set of lints. Based on Clippy, it can replicate Clippy.\
- [Clippy](https://github.com/rust-lang/rust-clippy): Rust linting tool, static set of lints.


