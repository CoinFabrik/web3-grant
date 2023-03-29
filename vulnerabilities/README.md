# Code Snippets and Smart Contract Examples
## Examples
According to the 
[curated list of vulnerabilities](../curated-list-of-vulnerabilities/README.md)
we prepared smart contracts where the following vulnerabilities are present:
- [integer-overflow-or-underflow](./examples/integer-overflow-or-underflow/README.md)
- [set-contract-storage](./examples/set-contract-storage/README.md)
- [reentrancy](./examples/reentrancy/README.md)
- [panic-error](./examples/panic-error/README.md)
- [unused-return-enum](./examples/unused-return-enum/README.md)
- [dos-unbounded-operation](./examples/dos-unbounded-operation/README.md)
- [dos-unexpected-revert-with-vector](./examples/dos-unexpected-revert-with-vector/README.md)

We created one folder for each of the above vulnerabilities. 
Each folder consists of: 
- **Documentation**: A `README.md` file with a detailed description on how the
vulnerability may be exercised (Exploit Section) and how it can be prevented
(Remediation Section). Evenmore, in some cases we included a Deployment 
subsection and References.
- A **vulnerable smart contract** folder including a `lib.rs` file with the
smart contract's code and the associated `Cargo.toml`.
    - If necessary, an exploit smart contract that would perform the attack on
    the vulnerable smart contract, also with its `lib.rs` and `Cargo.toml` 
    associated files.
    - Integration and end-to-end tests were provided in order to simplify and
    document the realization of the vulnerability. 
- A **remediated smart contract** folder which includes a version of the vulnerable
example but fixed. This folder including the same `lib.rs`and `Cargo.toml`
files.

## From Nothing to Vulnerability Examples
A problem we discovered in `ink!` and more genreally Substrate-based networks
security is the lack of public vulnerabilities disclosed, e.g., as part of 
security audits of deployed smart contracts. Ideally, we would compile a 
nicely-sized set of smart contracts with documented vulnerabilities, grow a 
database from there and use this database as a source to extract smippets,
classify vulnerabilities, and develop and tune our detection tools on this 
snippets. With this missing, we could not come up with a reasonable-sized 
list of vulnerabilities in real-life smart contracts. 

The second best option we came up with was to recrate this vulnerable smart 
contracts from other sources. As we analyzed different audited and deployed 
`ink!`smart contracts, we came upon various pieces of code that were _almost_
vulnerable. That is, maybe the smart contracts were not vulnerable in their 
form, but could become vulnerable after some changes, e.g., removing checks 
that were in place. These modified smart contracts were consistent with some
vulnerable contracts that we found while auditing smart contracts on other
blockchains.

This is how we created the above examples.
