# Curated List of Vulnerabilities

Smart contracts deployed in the Polkadot or Kusama networks are liable to exploitation as it happens with all running code. In this section we describe our effort to discover relevant security-related issues introduced during the development of smart contracts in Substrate Ink!. Some issues are particular to Substrate while others are common across multiple networks. While focusing on security vulnerabilities, we also look for issues related to best practices and enhancements. 

# Methodology and Analysis

Today, smart contracts in Substrate Ink! have a short history when compared to longer-lived peers like Ethereum. Therefore, the job of listing and prioritizing Substrate issues cannot be made by a straight-forward sampling of the issues found in some Ink! smart contracts.

Moreover, variations in pallet configuration across different parachains adds another layer of analysis on the runtime. We analyzed the different pallet configurations on existing projects, but determined that vulnerabilities native to the runtime were beyond the scope of the tools and techniques studied for this PoC. Therefore, our search for vulnerabilities was focused on Substrate Ink! smart contracts.

We based our work on three sources in order to produce a list of vulnerable examples. First, we searched for audit reports on Substrate in order to list common vulnerabilities, analysis tools and analysis techniques. Second, we used our previous auditing experience and categorization of issues present in other blockchains in order to consider possible vulnerabilities that could be easily mapped to Substrate. Finally, we performed a manual review of a few deployed Substrate Ink! smart contracts, looking for issues particular to Substrate as well as issues common to other networks.

## Analysis of Audited Projects

When analyzing public audit reports on Substrate projects, we focused on identifying found vulnerabilities; we also classified theand tools used for securityautomatic analysis.

Out of a list of 10 initial public audit reports on projects developed with Substrate, we found no audits dedicated specifically to Ink! smart contracts. All reports were focused on pallet configuration, runtime logic or parachain interactions.

Nevertheless, we did find some recurring types of vulnerabilities and tool usage across the analyzed audits. We list our findings below:

| Types of Vulnerabilities      |
| -----------------------------|
| Overflow/Underflow           |
| Dependency versions          |
| Lack of validations and error handling |
| Cross chain replay            |


| Analysis Tools |
| -------------- |
| Dylint         |
| Cargo audit    |
| Tarpaulin      |
| Eslint         |
| Semgrep        |
| Rustsec        |
| Cargo-geiger   |
| Cargo-audit    |
| Rust-clippy    |


## Analysis Categories

During our smart contract audits at Coinfabrik, our auditors perform a manual review of the analyzed code, including security analyses that can be grouped into the following [categories](https://blog.coinfabrik.com/analysis-categories/):

| Category                  | Description                                                                                                      |
| -------------------------| -----------------------------------------------------------------------------------------------------------------|
| Arithmetic                | Proper use of arithmetic and number representation.                                                              |
| Assembly Usage            | Detailed analysis of implementations using assembly.                                                             |
| Authorization             | Vulnerabilities related to insufficient access control or incorrect authorization implementation.                |
| Best practices            | Conventions and best practices for improved code quality and vulnerability prevention.                           |
| Block attributes          | Appropriate usage of block attributes. In particular, when used as a source of randomness.                       |
| Centralization            | Analysis of centralization and single points of failure.                                                         |
| DoS                       | Denial of service attacks.                                                                                        |
| Gas Usage                 | Performance issues, enhancements and vulnerabilities related to use of gas.                                      |
| MEV                       | Patterns that could lead to the exploitation of Maximal Extractable Value.                                        |
| Privacy                   | Patterns revealing sensible user or state data.                                                                   |
| Reentrancy                | Consistency of contract state under recursive calls.                                                              |
| Unexpected transfers      | Contract behavior under unexpected or forced transfers of tokens.                                               |
| Upgradability             | Proxy patterns and upgradable smart contracts.                                                                   |
| Validations and error handling | Handling of errors, exceptions and parameters.                                                               |


We used the Analysis Categories above, and common examples of vulnerabilities detected within each category in other blockchains, as a guideline for finding and developing vulnerable examples of Substrate Ink! smart contracts.


## Analyses of Deployed Projects

Taking into consideration that a complete audit of a smart contract project takes significant time, usually in the order of weeks, we looked for a few Substrate Ink! smart contract projects deployed on Polkadot or Kusama to analyze for vulnerabilities. Our objective was to perform a quick evaluation of the projects during the course of a week, looking for possible vulnerabilities, enhancements or best practices that could be taken either directly or as inspiration for examples to include later in our list of vulnerabilities.

From this list of deployed projects, we analyzed the Panorama Swap smart contract repository and, less thoroughly, swanky-dapp’s implementation of UniswapV2.

This review consisted of:
- An initial definition of the analysis scope,
- A first walkthrough of the selected contracts looking for potential vulnerabilities and taking as a reference the analysis categories used in our smart contract audits, 
- A final revision, where these potential vulnerabilities were discarded or confirmed.

### Panorama Swap

Panorama swap is a decentralized exchange, currently running on Aleph Zero testnet. Project links and a description of involved contracts can be found at its Github repository.

Our analysis was focused on commit 797e41ece7e58778175ff7d01c5133b7b3769a46. The scope covered the following contract files:
- airdrop_contract/lib.rs
- pair_creator/lib.rs
- staking_contract/lib.rs
- trading_pair_azero/lib.rs
- trading_pair_psp22/lib.rs
- vesting_contract/lib.rs

During a first review of the contracts, we highlighted some potential issues associated with integer overflow, denial of service, reentrancy attacks, lack of input validation and error handling, timestamp manipulation and accurateness of vesting calculations. However, upon a closer look, most of these issues were marked as unfeasible and discarded.

We were able to confirm two issues related to input validation and error handling, which were informed to the Panorama team. These issues were taken into consideration for our list of vulnerabilities, we list our observations in the table below:



### Swanky Dapp's Uniswap V2

This implementation of Uniswap V2 is deployed on the Shibuya testnet. Specific deployments of the involved contracts are listed in its Github repository.

Our analysis of this project was based on commit 6bed95a925b532d912e25dd2b1b92f3bdb0e14f8. The scope of analysis covered the following contract files:
- uniswap-v2/contracts/factory/lib.rs
- uniswap-v2/contracts/pair/lib.rs
- uniswap-v2/contracts/psp22/lib.rs
- uniswap-v2/contracts/router/lib.rs
- uniswap-v2/contracts/wnative/lib.rs

Even though we identified some initial clues related to integer overflow and reentrancy attack, we could not confirm any vulnerabilities on this repo.

## Results 

As a result of our analysis, we were able to produce seven examples of vulnerabilities under the following analysis categories: Arithmetic, Authorization, DoS (Denial of Service), Reentrancy, and Validations and error handling.
Full documentation of each vulnerability, as well as associated smart contract files can be found at our repository at vulnerabilities/examples.

### Curated List of Vulnerabilities

We summarize below the different vulnerability examples produced, proving for each of them references to their complete documentation at our repository folder vulnerabilities/examples.

#### 1 - Integer overflow or underflow
These types of vulnerabilities are commonly referred to as "integer overflow" and "integer underflow" vulnerabilities, and they can occur when an arithmetic operation overflows or underflows the available memory allocated to the variable.

We classified this type of vulnerability under the ID integer-overflow-or-underflow. It is categorized as a vulnerability of Arithmetic type, with a High severity.

In the context of Substrate, we found that this vulnerability could only be realized if overflow and underflow checks are disabled at the time of compilation. We explain this vulnerability in further detail at its documentation.

#### 2 - Set contract Storage

Functions using keys as variables without proper access control or input sanitation can allow users to perform changes in arbitrary memory locations. 

We classified this type of vulnerability under the ID set-contract-storage. It is categorized as a vulnerability of Authorization type, with a High severity.

In the exploit example described in our repository, we highlight the importance of setting access control and proper authorization validation for the set_contract_storage() function, in order to prevent malicious users from changing their allowance in an ERC20 contract.

#### 3 - Reentrancy

Smart contracts can call other contracts and send tokens to them. These operations imply external calls where control flow is passed to the called contract until the execution of the called code is over. Then the control is delivered back to the caller.

External calls, therefore, could open the opportunity for a malicious contract to execute any arbitrary code. This includes calling back the caller contract, an attack known as reentrancy. This kind of attack was used in Ethereum for the infamous DAO Hack.

We classified this type of vulnerability under the ID reentrancy. It is categorized as a vulnerability of Reentrancy type, with a High severity.

Remediation of this type of vulnerability is generally addressed with the use of the Check-Effect-Interaction pattern, or the use of reentrancy guards in relevant code. In the vault smart contract we use for our example, we explain the usage of these patterns, but were only able to realize the vulnerability using the flag set_allow_reentry(true).

#### 4 - Panic error
The use of the panic! macro to stop execution when a condition is not met is useful for testing and prototyping, but should be avoided in production code. Using Result as return type for functions that can fail is the idiomatic way to handle errors in Rust.

We classified this type of vulnerability under the ID panic-error. It is categorized as a vulnerability of Validations and error handling type, with an Informational severity.

The proper usage of the Result type is available at the remediation of this example.

#### 5 - Unused return enum

Ink messages can return a Result enum with a custom error type. This is useful for the caller to know what went wrong when the message fails. The definition of the Result type enum consists of two variants: Ok and Err. If any of the variants is not used, the code could be simplified or it could imply a bug.

We classified this type of vulnerability under the ID unused-return-enum. It is categorized as a vulnerability of Validations and error handling type, with an Low severity.

In our example, we see how lack of revision on the usage of both types (Ok and Err) leads to code where its intended functionality is not realized.

#### 6 - DoS Unbounded operation with vector
A Denial of Service attack is an exploit where a malicious user prevents others from accessing or using a service. In the context of blockchain, it can be associated with the exhaustion of block gas limits and consequent transaction failures.

Each block in a Substrate Blockchain has an upper bound on the amount of gas that can be spent, and thus the amount of computation that can be done. This is the Block Gas Limit. If the gas spent exceeds this limit, the transaction will fail.

In order to prevent a single transaction from consuming all the gas in a block, unbounded operations must be avoided. This includes loops that do not have a fixed number of iterations, and recursive calls.
We classified this type of vulnerability under the ID dos-unbounded-operation-with-vector. It is categorized as a vulnerability of DoS type, with and High severity.

This type of DoS issue can be understood in our example, when iterating over a vector. It can be avoided by applying a pull over push pattern, or by ensuring that loops over an unbounded array can take place over multiple blocks, preventing the block gas limit from being reached.

#### 7 - DoS Unexpected revert
Another type of Denial of Service attack is called unexpected revert. It occurs by preventing transactions by other users from being successfully executed forcing the blockchain state to revert to its original state.

We classified this type of vulnerability under the ID dos-unexpected-revert. It is categorized as a vulnerability of DoS type, with and High severity.

In this particular example, a Denial of Service through unexpected revert is accomplished by exploiting a smart contract that does not manage storage size errors correctly. It can be prevented by using Mapping instead of Vec to avoid storage limit problems.


### Discarded Vulnerabilities

During the duration of this milestone we were not able to officially discard a particular type of vulnerability present in other blockchains but not feasible for Substrate Ink!. Even in examples like #3 - reentrancy, were the vulnerability is contingent on the use of the set_allow_reentry(true) flag, or #1 - integer-overflow-or-underflow, where the issue was only possible if overflows were allowed at compilation, we found ways for the vulnerabilities to effectively take place in Substrate.

All the vulnerabilities that we pushed as candidates to vulnerabilities/candidates were feasible and manually tested before further development.
