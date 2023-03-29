# Relevant ink! Vulnerabilities
In this section we describe our effort to discover relevant security-related 
issues introduced during the development of smart contracts in Substrate Ink!. 
Many do generalize to substate-based networks, but that need not be the rule. 
There follows a list containing some security issues that we identified. 
The list is, of course, not exhaustive but all of these are very relevant. 
In each case we assign a severity label according to the following taxonomy:

## Vulnerability Severity
* __Critical__: These issues compromise the system seriously. They must be 
fixed immediately.
* __Medium__: These are potentially exploitable issues which might represent 
a security risk in the near future. We suggest fixing them as soon as possible.
* __Minor__: These issues represent problems that are relatively small or 
difficult to take advantage of, but might be exploited in combination with 
other issues. These kinds of issues do not block deployments in production 
environments. They should be taken into account and be fixed when possible.
* __Enhcancemet__: This class relates to issues related to deviations from 
best practices or stylistic which could become higher-priority issues with 
other changes, e.g., may lead to development errors in an future update.

## Vulnerability Categories
We follow with a taxonomy of Vulnerabilities. Many "top vulnerability" lists 
can be found covering Ethereum/Solidity smart contracts. This list below is 
used by the Coinfabrik Audit Team, when source code (security) audits in
Ethereum/Solidity, Stacks/Clarity, Algorand/PyTEAL /TEAL, Solana/RUST, etc.
The team discusses the creation of the list in this 
[blogpost](https://blog.coinfabrik.com/vulnerability-categories/).

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

## A Curated List of Security Vulnerabilities
As a result of our analysis, we were able to produce seven examples of 
vulnerabilities which fall under the following categories: 
Arithmetic, 
Authorization, 
Denial of Service, 
Reentrancy, and 
Validations and error handling. 
Full documentation of each vulnerability, as well as associated smart contract 
files can be found in the [vulnerabilities folder](../vulnerabilities/examples/).

### 1 - Integer Overflow and Integer Underflow
This type of vulnerability occurs when an arithmetic operation attempts to 
create a numeric value that is outside the valid range in substrate, e.g, 
a `u8` unsigned integer can be at most M:=2**8-1=255, hence the sum *M+1* 
produces an overflow. 

An overflow/underflow is typically caught and generates an error. When it 
is not caught, the operation will result in an inexact result which could 
lead to serious problems. We classified this type of vulnerability under 
the [Arithmetic Category](#vulnerability-categories) type anhd assinged it a 
High Severity.

In the context of Substrate, we found that this vulnerability could only be 
realized if overflow and underflow checks are disabled during compilation. 
Notwithstanding, there are contexts where developers do turn off checks for 
valid reasons and hence the reason for including this vulnerability in the 
list. Check the following code snippet and 
[documentation](https://github.com/CoinFabrik/web3-grant/tree/main/vulnerabilities/examples/integer-overflow-or-underflow). 

### 2 - Unauthotized Set Contract Storage
Smart contract can store important information in memory which changes 
through the contract's lifecycle. Changes happen via user interaction with 
the smart contract. An _unauthorized set contract storage_ vulnerability 
happens when a smart contract call allows a user to set or modify contract 
memory when he was not supposed to be authorized.

Common practice is to have functions with the ability to change 
security-relevant values in memory to be only accessible to specific roles, 
e.g, only an admin can call the function `reset()` which resets auction values.
When this does not happen, arbitrary users may alter memory which may impose 
great damage to the smart contract users. We classified this vulnerability 
under the [Authorization Category](#vulnerability-categories) and assigned it a 
High Severity.

In `ink!` the function `set_contract_storage(key: &K, value: &V)` can be used 
to modify the contract storage under a given key. When a smart contract uses 
this function, the contract needs to check if the caller should be able to 
alter this storage. If this does not happen, an arbitary caller may modify 
balances and other relevant

### 3 - Reentrancy
An `ink!` smart contract can interact with other smart contracts. These 
operations imply (external) calls where control flow is passed to the called 
contract until the execution of the called code is over, then the control is 
delivered back to the caller. A _reentrancy_ vulnerability may happen when a 
user calls a function, this function calls a malicious contract which again 
calls this same function, and this 'reentrancy' has unexpected reprecussions 
to the contract. 
This kind of attack was used in Ethereum for
[the infamous DAO Hack](https://www.economist.com/finance-and-economics/2016/05/19/the-dao-of-accrue).

This vulnerability may be prevented with the use of the Check-Effect-Interaction
pattern that dictates that we first evaluate (check) if the necessary conditions
are granted, next we record the effects of the interaction and finally we 
execute the interaction (e.g., check if the user has funds, substract the funds 
from the records, then transfer the funds). There's also so-called 
_reentrancy guards_ which prevent the marked piece of code to be called twice 
from the same contract call. When the vulnerability may be exercised, the 
successive calls to the contract may allow the malicious contract to execute a
function partially many times, e.g., transfering funds many times but 
substracting the funds only once. 
This vulnerability is of the [Reentrancy Category](#vulnerability-categories) and 
assign it a High Severity.

In the context of `ink!` Substrate smart contracts there are controls 
preventing reentrancy which could be turned off (validly) using the flag
`set_allow_reentry(true)`.

### 4 - Panic error
The use of the `panic!` macro to stop execution when a condition is not met is 
useful for testing and prototyping but should be avoided in production code. 
Using `Result` as the return type for functions that can fail is the idiomatic
way to handle errors in Rust.

We classified this issue, a deviation for best practices which could have 
security implications, under the [Validations and Error Handling Category](#vulnerability-categories)
with the severity of an Enhancement.

### 5 - Unused Return enum
`Ink!` messages can return a `Result` `enum` with a custom error type. This is
useful for the caller to know what went wrong when the message fails. The 
definition of the `Result` type enum consists of two variants: Ok and Err. If 
any of the variants is not used, the code could be simplified or it could imply
a bug.

We put this vulnerability under the [Validations and Error Handling Category](#vulnerability-categories) 
with a Low Severity.

In our example, we see how lack of revision on the usage of both types (`Ok`
and `Err`) leads to code where its intended functionality is not realized.

### 6 - DoS Unbounded Operation With Vector
Each block in a Substrate Blockchain has an upper bound on the amount of gas
that can be spent, and thus the amount of computation that can be done. This 
is the Block Gas Limit. If the gas spent by a function call on an `ink!` smart
contract exceeds this limit, the transaction will fail. Sometimes it is the
case that the contract logic allows a malicious user to modify conditions
so that other users are forced to exahust gas on standard function calls.

In order to prevent a single transaction from consuming all the gas in a block, 
unbounded operations must be avoided. This includes loops that do not have a 
bounded number of iterations, and recursive calls. This vulnerability falls
under the [Denial of Service Category](#vulnerability-categories) and has a Medium
Severity.
A denial of service vulnerability allows the exploiter to hamper the 
availability of a service rendered by the smart contract. In the context 
of `ink!` smart contracts, it can be caused by the exhaustion of gas,
starage space, or other failures in the contract's logic.

Needless to say, there are many different ways to cause a DOS vulnerability.
This case is relevant and introduced repeteadly by the developer untrained in
web3 environments. 

### 7 - DoS Unexpected Revert
Another type of Denial of Service attack is called unexpected revert. It occurs
by preventing transactions by other users from being successfully executed 
forcing the blockchain state to revert to its original state.

This vulnerability again falls under the [Denial of Service Category](#vulnerability-categories)
and similarly has a Medium Severity.

In this particular example, a Denial of Service through unexpected revert is 
accomplished by exploiting a smart contract that does not manage storage size 
errors correctly. It can be prevented by using Mapping instead of Vec to avoid
storage limit problems.


<!-- 
## Discussion
### Methodology and Analysis
Today, smart contracts in Substrate Ink! have a short history when compared to
longer-lived peers like Ethereum. Therefore, the job of listing and 
prioritizing Substrate issues cannot be made by a straight-forward sampling of
the issues found in some Ink! smart contracts.

Moreover, variations in pallet configuration across different parachains adds 
another layer of analysis on the runtime. We analyzed the different pallet 
configurations on existing projects, but determined that vulnerabilities native
to the runtime were beyond the scope of the tools and techniques studied for
this PoC. Therefore, our search for vulnerabilities was focused on Substrate 
Ink! smart contracts.

We based our work on three sources in order to produce a list of vulnerable 
examples. First, we searched for audit reports on Substrate in order to list 
common vulnerabilities, analysis tools and analysis techniques. Second, we 
used our previous auditing experience and categorization of issues present 
in other blockchains in order to consider possible vulnerabilities that 
could be easily mapped to Substrate. Finally, we performed a manual review 
of a few deployed Substrate Ink! smart contracts, looking for issues 
particular to Substrate as well as issues common to other networks.

## Analysis of Audited Projects
When analyzing public audit reports on Substrate projects, we focused on 
identifying found vulnerabilities; we also classified theand tools used for
security automatic analysis.

Out of a list of 10 initial public audit reports on projects developed with
Substrate, we found no audits dedicated specifically to Ink! smart contracts.
All reports were focused on pallet configuration, runtime logic or parachain 
interactions.

Nevertheless, we did find some recurring types of vulnerabilities and tool
usage across the analyzed audits. We list our findings below:

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






## Analysis of Deployed Projects

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

### Discarded Vulnerabilities

During the duration of this milestone we were not able to officially discard a particular type of vulnerability present in other blockchains but not feasible for Substrate Ink!. Even in examples like #3 - reentrancy, were the vulnerability is contingent on the use of the set_allow_reentry(true) flag, or #1 - integer-overflow-or-underflow, where the issue was only possible if overflows were allowed at compilation, we found ways for the vulnerabilities to effectively take place in Substrate.

All the vulnerabilities that we pushed as candidates to vulnerabilities/candidates were feasible and manually tested before further development.

-->