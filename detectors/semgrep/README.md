# Semgrep
Semgrep is a fast, open source static analysis tool for finding bugs, detecting
vulnerabilities in third-party dependencies, and enforcing code standards.

## Installation (macOS)

`$ brew install semgrep`

## Usage

`$ semgrep --config=<rule.yaml> /PATH/TO/SRC` 

### Examples

`$ semgrep --config="panic-error/panic-error.yaml" ../../vulnerabilities/examples/panic-error/vulnerable-example/lib.rs`

## Rules

The following rules are written into yaml files:

* dos-unbounded-operation/uncontrolled-var.yaml
* integer-overflow-or-underflow/integer-overflow-or-underflow.yaml
* panic-error/panic-error.yaml
* set-contract-storage/syntactic/unprotected-use.yaml
* set-contract-storage/tainting/unprotected-use.yaml

## Executing each rule

### dos-unbounded-operation/uncontrolled-var.yaml

`$ semgrep --config="dos-unbounded-operation/uncontrolled-var.yaml" ../../vulnerabilities/examples/dos-unbounded-operation/vulnerable-example/lib.rs`

### integer-overflow-or-underflow/integer-overflow-or-underflow.yaml

`$ semgrep --config="integer-overflow-or-underflow/integer-overflow-or-underflow.yaml" ../../vulnerabilities/examples/integer-overflow-or-underflow/vulnerable-example/lib.rs`

### panic-error/panic-error.yaml
  
`$ semgrep --config="panic-error/panic-error.yaml" ../../vulnerabilities/examples/panic-error/vulnerable-example/lib.rs`

### set-contract-storage/syntactic/unprotected-use.yaml

`$ semgrep --config="set-contract-storage/syntactic/unprotected-use.yaml" ../../vulnerabilities/examples/set-contract-storage/vulnerable-example/lib.rs`

### set-contract-storage/tainting/unprotected-use.yaml

`$ semgrep --config="set-contract-storage/tainting/unprotected-use.yaml" ../../vulnerabilities/examples/set-contract-storage/vulnerable-example/lib.rs`


## References
- [Semgrep](https://semgrep.dev/): Static semantics/syntax checker

