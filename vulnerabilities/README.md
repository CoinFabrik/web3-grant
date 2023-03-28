# Code Examples and Snippets: From Candidates to Vulnerability Examples

As we analyzed different audited and deployed projects, we came about with various vulnerability and enhancement ideas. Most of these could not be realized within the reviewed code, but could be present in other projects without the proper checks.

After studying Substrate and analyzing the mentioned projects, we started building candidate examples for possible vulnerabilities. These candidates consisted of:
- A vulnerable smart contract file lib.rs with its associated Cargo.toml
- If necessary, an exploit smart contract that would perform the attack on the vulnerable smart contract, also with its lib.rs and Cargo.toml associated files.

If the idea behind the vulnerability was verified upon its implementation and manual deployment into a local substrate node, the candidate was then confirmed as a vulnerability example, adding the following components to the example:
- Classification: The issue was named and classified according to our analyses categories, establishing as well the severity of the problem. For the scope of this Proof of Concept, the probability of occurrence and detection confidence were not considered.
- Documentation: A README.md file exposing the classification in the Configuration section, followed by the sections: Description, Exploit Scenario and Remediation. When relevant, subsections about Deployment, video Tutorials and further References where also included.
- Testing: Together with the lib.rs and Cargo.toml files associated with the vulnerability and exploiting contract, integration and end-to-end tests were provided in order to simplify and document the realization of the vulnerability. Files relevant for the vulnerable examples were kept under the directory vulnerable-example, while those associated with its remediation can be found under the directory remediated-example.

Therefore, once a vulnerability example has been completely tested and documented, it is presented in our repository under the structure:
- REAMDE.md
- /vulnerable-example
- /remediated-example

In order to organize our work, candidates were first pushed into our repository under the directory vulnerabilities/candidates, where the missing components were added to the example following the vulnerabilities/candidates/template  as a guideline. Once confirmed and completed, candidates were migrated to vulnerabilities/examples.