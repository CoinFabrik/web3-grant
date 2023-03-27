## Detectors Under Review by Coinfabrik

| Detector ID | Category | Source | Description | Severity | Reviewing | Status | 
|----------------------|------------------|--------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------|----------|----------|---------------------------|
| weak-randomness       | Block attributes | Analysis Categories Wiki                        | [Insecure source of randomness through block attributes.](https://gitlab.com/coinfabrik-private/coinfabrik-wiki/-/wikis/Auditing/Analyses/Block-attributes/Use-of-insufficiently-random-values)                                                                         | ?        | FALSE    | 2-Under Review Coinfabrik |
| time-manipulation     | Block attributes | Analysis Categories Wiki                        | [Using block attributes in order to determine time can lead to manipulation by miners.](https://gitlab.com/coinfabrik-private/coinfabrik-wiki/-/wikis/Auditing/Analyses/Block-attributes/Time-manipulation)                                        | ?        | FALSE    | 2-Under Review Coinfabrik |

We prioritize analysis on issues that appear on deployed projects or audits.


## Methodology

We have three sources for candidate for examples to be detected (detectors):

1. Audits of Substrate Polkadot projects by auditing companies. In order to get candidates we look at the audit's findings and the associated github repo.

2. Deployed ink! projects.

3. [Analysis Categories](https://blog.coinfabrik.com/analysis-categories/). We generate examples from classic examples in other blockchains.

