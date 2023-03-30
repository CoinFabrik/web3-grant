# Additional Information
In order to produce realistic smart contracts with documented vulnerabilities
we searched for public security audit reports for ink! smart contracts. We
found some audits on Substrate-based projects that we milked, with some effort,
to produce examples we could use. This also yielded some interesting information
laterally related to this project.
## Analysis of Audited Projects
Out of a list of 10 initial public audit reports on projects developed with
Substrate, we found no audits dedicated specifically to Ink! smart contracts.
All reports were focused on pallet configuration, runtime logic or parachain 
interactions.

## Analysis of Deployed Projects
Taking into consideration that a complete audit of a smart contract project
takes significant time, usually in the order of weeks, we looked for a few 
Substrate Ink! smart contract projects deployed on Polkadot or Kusama to 
analyze for vulnerabilities. 

In particular we looked at [Panorama Swap smart contract](https://github.com/RottenKiwi/Panorama-Swap-INK-SC).
The review, which should not be taken as a security audit, consisted of:
- An initial definition of the analysis scope,
- A first walkthrough of the selected contracts looking for potential
vulnerabilities and taking as a reference the analysis categories used in our
smart contract audits, 
- A final revision, where these potential vulnerabilities were discarded or 
confirmed.

Our analysis was focused on commit `797e41ece7e58778175ff7d01c5133b7b3769a46`.
The scope covered six contract files.

We were able to confirm two issues related to input validation and error 
handling, which were informed to the Panorama team. The issues wre fixed
and new versions of the smart contract deployed. These issues were taken
into consideration for our [list of vulnerabilities](../curated-list-of-vulnerabilities/README.md).

