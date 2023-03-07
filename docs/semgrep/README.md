# Semgrep

Semgrep is a fast, open source static analysis tool for finding bugs, detecting vulnerabilities in third-party dependencies, and enforcing code standards.

## Installation (macOS)

`$ brew install semgrep`

## Usage

`$ semgrep --rule="<rule.yaml> /PATH/TO/SRC` 

### Examples

`$ semgrep --rule="panic-error.yaml" panic-error.rs`

## Rules

The following rules are written into yaml files:

* integer-overflow-or-underflow.yaml
* panic-error.yaml 
* set-contract-storage.yaml

## Samples

The following RUST files are provided as examples:

* integer-overflow-or-underflow.rs (and integer-overflow-or-underflow-fixed.rs)
* panic-error.rs (and panic-error-fixed.rs)
* set-contract-storage.rs




## References


- [Semgrep](https://semgrep.dev/): Static semantics/syntax checker

