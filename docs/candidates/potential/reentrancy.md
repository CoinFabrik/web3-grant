# Reentrancy

## Configuration

* Detector ID: `reentrancy`
* Analysis Category: `Reentrancy`
* Severity: `High`

## Description

Smart contracts can call other contracts and send tokens to them. These operations imply external calls where control flow is passed to the called contract until the execution of the called code is over. Then the control is delivered back to the caller.

External calls, therefore, could open the opportunity for a malicious contract to execute any arbitrary code. This includes calling back the caller contract, an attack known as reentrancy. This kind of attack was used in Ethereum for the infamous [DAO Hack](https://blog.chain.link/reentrancy-attacks-and-the-dao-hack/).

## Exploit Scenario

Working on this ....

See this [preliminary tutorial](https://drive.google.com/file/d/1xdd3sECx0_qwVmwTpqs2zHNdKjghAae3/view?usp=share_link) to see where we are currently stuck.

Vault and exploit files can be found under the directories ./example/exploit and ./example/vault.

### Deployment


### Tutorial

See this preliminary [tutorialV1](https://drive.google.com/file/d/1xdd3sECx0_qwVmwTpqs2zHNdKjghAae3/view?usp=share_link) (in Spanish) showing this exploit in action.

## Recommendation

Reentrancy can be addressed with the Check-Effect-Interaction pattern, a best practice that indicates that external calls should be the last thing to be executed in a function.

In our example, this means to set the balance of the message sender before transfering them the tokens. Another approach is to use a [reentrancy guard](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/security/reentrancy_guard) like the one offered by [OpenBrush](https://github.com/Supercolony-net/openbrush-contracts).

## References
* https://use.ink/datastructures/storage-layout
* https://consensys.github.io/smart-contract-best-practices/attacks/reentrancy/
* https://dasp.co/#item-1
* https://blog.sigmaprime.io/solidity-security.html#SP-1
* https://docs.soliditylang.org/en/develop/security-considerations.html#re-entrancy
* [Ethernaut: Reentrancy](https://ethernaut.openzeppelin.com/level/0xe6BA07257a9321e755184FB2F995e0600E78c16D)
* [SWC-107](https://swcregistry.io/docs/SWC-107)
* [Slither: Reentrancy vulnerabilities (theft of ethers)](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities)
* [Slither: Reentrancy vulnerabilities (no theft of ethers)](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-1)
* [Slither: Benign reentrancy vulnerabilities](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-2)
* [Slither: Reentrancy vulnerabilities leading to out-of-order Events](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-3)
* [Slither: Reentrancy vulnerabilities through send and transfer](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-4)



