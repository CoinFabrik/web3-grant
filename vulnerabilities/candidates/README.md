## Detectors Under Review by Coinfabrik

| Detector ID | Category | Source | Description | Severity | Reviewing | Status | 
|----------------------|------------------|--------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------|----------|----------|---------------------------|
| weak-randomness       | Block attributes | Analysis Categories Wiki                        | Insecure source of randomness through block attributes.                                                                         | ?        | FALSE    | 2-Under Review Coinfabrik |
| time-manipulation     | Block attributes | Analysis Categories Wiki                        | Using block attributes in order to determine time can lead to manipulation by miners.                                   | ?        | FALSE    | 2-Under Review Coinfabrik |

We prioritize analysis on issues that appear on deployed projects or audits.


## Methodology

We use three sources for candidate vulnerabilities:

1. Audits of Substrate Polkadot projects by auditing companies. In order to get candidates we look at the audit's findings and the associated github repo.

2. Deployed ink! projects. We look at the projects github repo.

3. [Analysis Categories](https://blog.coinfabrik.com/analysis-categories/). We generate examples from classic examples in other blockchains.

In order to submit a candidate:

1. Branch this repo into a new branch with the vulnerability's name.

2. Edit this README.md adding your candidate to the table.

3. Add a folder with your candidate's name to this directory, following the documentation guideline in the `template` folder.

4. Push your changes and make a pull request.

