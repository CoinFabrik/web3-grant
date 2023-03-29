# Analysis Techniques and Tools

Our primary focus was set on analyzing the viability of using static analysis tools to detect security issues in smart contracts developed in Substrate Ink!. In other words, tools in which the analysis is performed without executing the smart contract, as opposed to dynamic program analysis, which is performed during and after their execution. Being Ink! A Rust-based language, we looked at static analysis tools that already work for Rust and evaluated the possibility of using them to analyze programs written with Substrate.

On the other hand, we also analyzed fuzzing techniques and tools for particular cases where these techniques were deemed appropriate.

## Static Analysis

Static analysis is an automated analysis technique used to examine issues and potential vulnerabilities in code without performing its execution. Considering the successful adoption of this technique for vulnerability detectors in other blockchains (eg: Slither for Solidity and Rustle for NEAR), and available tools for static analysis in Rust like Clippy and Dylint, we reviewed the latter and their applicability for Substrate Ink! contracts. We also partially reviewed the static analyzer Semgrep, even though its support for Rust is currently experimental.

### Clippy

Clippy is a static code analysis tool that supports detection of programming errors via analysis of the abstract syntax tree of Rust code. The tool currently supports over 600 lints that produce correctness, stylistic, and performance warnings (amongst others). Available at github under an Apache v2 / MIT license, Clippy is actively maintained and used by Rust programmers. Furthermore, there is good documentation on how to add new lints.

Adding rules (lints) to Clippy involves adding to the Clippy codebase and recompiling. For a stable set of rules for Substrate Ink! contracts this is not a problem. However, if application-specific rules are to be added or removed frequently, this can be quite tedious.

### Dylint

Dylint is also a static analysis linting tool for Rust. Available at github under an Apache v2 / MIT license, it was developed by Trail of Bits to be able to write Rust lints without forking Clippy. It is also actively maintained and well documented.

Whereas Clippy runs a predetermined, static set of lints, Dylint runs lints from user-specified, dynamic libraries. Thus, Dylint allows developers to maintain their own personal lint collections, making it a better option than Clippy for the growing list of vulnerability detectors that we intend to have in our tool.

### Semgrep

Semgrep is a text search utility that understands –to some extent– the programming language semantics, thus queries can go beyond searching for regular expressions or navigating over abstract syntax trees to include conditions on the role that particular strings have in the code (e.g., name of a function as opposed to the name of a variable).

Semgrep comes with an intra-procedural data-flow engine that supports taint analysis. Thus, analysis of the flow of data from user defined sources to sinks without passing through sanitizers is possible.

Adding rules works rather differently than in Clippy. Instead of modifying the source code and recompiling, Semgrep is simply called providing as a parameter the user-defined rules to be checked. This makes prototyping and adding application specific rules very easy and potentially all issues detected by Clippy and Dylint could be also detected by Semgrep.

The core Semgrep tool is available on github on GNU Lesser general public license and is actively maintained. Although Semgrep supports multiple languages, support for Rust is beta at the moment. Amongst the various Rust-specific problems we encountered, not supporting detection of macro applications, constrains some of the potential of the tool for detecting problems in Substrate smart contracts. Therefore, in order to have Semgrep detect some of the issues that involve macros either Semgrep must be extended to support macros or the code must be instrumented to change the syntax of the macros to a text format accepted by Semgrep.

Writing rules in Semgrep is straightforward, and rules that may require some work to program in Clippy or Dylint can be specified quite simply in Semgrep.

### Conclusion

All in all, whereas Clippy runs a predetermined, static set of lints, Dylint runs lints from user-specified, dynamic libraries. Thus, Dylint allows developers to maintain their own personal lint collections, making it a better option for the growing list of vulnerability detectors that we intend to have in our tool.

Nevertheless, for some issues the detection by syntactic rules might lead to imprecision or missing bugs as some of them require more complex reasoning like computing control and data dependencies or symbolic reasoning, possibly interprocedural, that are beyond the capabilities of both Clippy and Dylint.

On the other hand, while taint analysis is supported in Semgrep, and inclusion of new rules is straightforward, its beta support for Rust makes it less preferable at this stage.

## Dynamic Analysis

Dynamic analysis refers to the analysis of a program's behavior during runtime. In the case of smart contracts, it involves the deployment of smart contracts on a local node. In contrast to static analysis which involves analyzing the source code without executing it, dynamic analysis involves running the program and analyzing it in real time. For this Proof of Concept, we explored the usage of fuzzing for the detection of some vulnerabilities.

Fuzzing is performed by generating a set of input test cases that are fed into the program in order to detect potential issues, usually in edge scenarios. This is particularly relevant in smart contracts in the context of input validation errors and possible integer overflows or underflows.

For this milestone, we focused on well-established fuzzers with production support for Rust.

### Cargo-fuzz

Cargo-fuzz is a tool used to invoke a fuzzer. Even though it could be extended in the future for other fuzzers, it currently only supports libFuzzer through the libfuzzer-sys crate.

### Conclusion

For this Proof of Concept, we decided to use the most mature tool, Cargo-fuzz, in order to analyze some vulnerabilities associated to integer overflow and underflow, as well as problems related to input validation and contract storage.

# Implementation

For each vulnerability in our list, we explain what tools and techniques were applied for their detection, mentioning implementation caveats.

## 1. Integer Overflow or Underflow

We based our analysis for overflow or underflow detection on the [vulnerability example associated to this issue](/vulnerabilities/examples/integer-overflow-or-underflow).

For this vulnerability, we were able to produce successfull detectors using [Dylint](/detectors/dylint/smart_contracts_linters/integer-overflow-or-underflow) and [Cargo-fuzz](/detectors/cargo-fuzz/integer-overflow-or-underflow), we detail their implementation below.

### Dylint

#### Description

Our detector checks for integer arithmetic operations which could overflow or panic. Specifically, it checks for any operators (+, -, \*, <<, etc) which are capable of overflowing according to the Rust Reference, or which can panic (/, %). No bounds analysis or sophisticated reasoning is attempted.

#### Implementation

In order to implement this detector we developed the following functions of the [LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html) trait:

- `check_body`
- `check_body_post`
- `check_expr`
- `check_expr_post`

In particular, we used these functions to check for every expression in the analyzed code, and to determine whether it contains one the operations that generate overflows/underflows. We also validate that the types being handled are integers.

#### Caveats

Rust includes a runtime check for integer overflows and underflows, which panics if any of these operations are detected. Adding this detector to the code will move the check to compile time, allowing the developer to handle the error in a more appropriate way.

### Cargo-Fuzz

#### Description

This detector uses fuzzing to find valid inputs that generate overflow or underlfow errors when fed into the smart contract.

#### Implementation

Using ink::env test module, we implemented a fuzz_target to execute the different tests of the contract using input values generated with libfuzzer_sys.

#### Caveats

The fuzzer built for this vulnerability was written using the tests of the vulnerability example that we worked with. Therefore it only works for this contract.

## 2. Set Contract Storage

We based our analysis for set-contract-storage detection on the [vulnerability example associated to this issue](/vulnerabilities/examples/set-contract-storage).

For this vulnerability, we were able to produce successfull detectors using [Dylint](/detectors/dylint/smart_contracts_linters/set-contract-storge) and [Cargo-fuzz](/detectors/cargo-fuzz/set-contract-storage), we detail their implementation below.

### Dylint

#### Description

This detector checks for calls to env::set_contract_storage().

#### Implementation

In order to implement this detector we developed the following functions of the [LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html) trait:

- `check_fn`

In particular, we used this function to check for every expression in the analyzed code, and to determine whether it calls the function env::set_contract_storage(). We also check if the function call is performed within an `if` statement that determines whether the caller is the contract owner, in the latter case no warning is shown.

#### Caveats

If ownership validation is performed in an auxiliary function, the linter will not be able to identify it, and the warning will be indicated as a false positive.
Conceptually, this detector should detect a problem in the information flow: user-provided data being used for the invocation of the set_contract_storage function without prior sanitization. We assume that if the data is entered by the contract owner, it has been sanitized beforehand.

### Cargo-Fuzz

#### Description

This detector uses fuzzing to find the storage key for env::set_contract_storage() and user accounts used in this exploit.

#### Implementation

Using ink::env test module, we implemented a fuzz_target to execute the different tests of the contract using input values generated with libfuzzer_sys.

#### Caveats

The fuzzer built for this vulnerability was written using the tests of the vulnerability example that we worked with. Therefore it only works for this contract. Furthermore, the bytesize of the arguments makes it very hard to find them using this technique.

## 3. Reentrancy

We based our analysis for set-contract-storage detection on the [vulnerability example associated to this issue](/vulnerabilities/examples/reentrancy).

For this vulnerability, we were able to produce successfull detectors using [Dylint](/detectors/dylint/smart_contracts_linters/reentrancy) we detail the implementation below.

### Dylint

#### Description

Conceptually, the warning should be issued when there is some evidence that check-effect interaction pattern is not adequately followed by code invoking a contract that may call back the original one.

#### Implementation

In order to implement this detector we developed the following functions of the [LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html) trait:

- `check_fn`

In particular, we used this function to check for every expression in the analyzed code, and to determine whether it calls the function set_allow_reentry(true) and the function invoke_contract_call() then we check for subsequent state changes like assignments (=, +=, -=, etc) or calls to the insert() function of mappings.

#### Caveats

If called method does not perform a malicious reentrancy (i.e. known method from known contract) false positives will arise.
If the usage of set_allow_reentry(true) or later state changes are performed in an auxiliary function, this detector will not detect the reentrancy. To detect these cases, it is necessary to perform interprocedural dataflow analysis

## 4. Panic Error

We based our analysis for set-contract-storage detection on the [vulnerability example associated to this issue](/vulnerabilities/examples/panic-error).

For this vulnerability, we were able to produce successfull detectors using [Dylint](/detectors/dylint/smart_contracts_linters/panic-error) we detail the implementation below.

### Dylint

#### Description

This detector checks the usage of the panic! macro.

#### Implementation

In order to implement this detector we developed the following functions of the [LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html) trait:

- `check_expr`

In particular, we used this function to check for every expression in the analyzed code, and to determine whether it uses the panic! macro.

#### Caveats

While this linter detects explicit calls to panic!, there may be some ways to raise a panic such as unwrap() or expect().

## 5. Unused Return Enum

We based our analysis for set-contract-storage detection on the [vulnerability example associated to this issue](/vulnerabilities/examples/unused-return-enum).

For this vulnerability, we were able to produce successfull detectors using [Dylint](/detectors/dylint/smart_contracts_linters/unused-return-enum) we detail the implementation below.

### Dylint

#### Description

This detector checks that if the function return value is of type Result then there exists at least one return value that uses Err and another return value that uses Ok.

#### Implementation

In order to implement this detector we developed the following functions of the [LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html) trait:

- `check_fn`
- `visitor`

In particular, we used this function together with a visitor to check for every expression of a function with return type Result whether its returns values are at least one Err and one Ok.

#### Caveats

If definitions of Err() and/or Ok() are in the code but do not flow to the return value due to the definition of a variable or because they are defined in a dead code block, the warning will not be shown. If the definitions are made in an auxiliary method, the warning will be shown, resulting in a false positive.

## 6. DoS Unbounded Operation

We based our analysis for set-contract-storage detection on the [vulnerability example associated to this issue](/vulnerabilities/examples/dos-unbounded-operation).

For this vulnerability, we were able to produce successfull detectors using [Dylint](/detectors/dylint/smart_contracts_linters/dos-unbounded-operation), we detail the implementation below.

### Dylint

#### Description

This detector checks that when using for or while loops, their conditions limit the execution to a constant number of iterations.

#### Implementation

In order to implement this detector we developed the following functions of the [LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html) trait:

- `check_expr`

In particular, we have used this function to search for every for or while loop through the code's expressions and determine if their conditions contain variables or function calls.

#### Caveats

False positives are to be expected when using variables that can only be set using controlled flows that limit the values within acceptable ranges. These cases can be detected by using tainting techniques and/or interprocedural dataflow analysis.

## 7. DoS Unexpected Revert with Vector

We based our analysis for set-contract-storage detection on the [vulnerability example associated to this issue](/vulnerabilities/examples/dos-unexpected-revert-with-vector).

For this vulnerability, we were able to produce successfull detectors using [Dylint](/detectors/dylint/smart_contracts_linters/dos-unexpected-revert-with-vector), we detail the implementation below.

### Dylint

#### Description

This detector checks that only the owner can manipulate vectors' content.

#### Implementation

In order to implement this detector we developed the following functions of the [LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html) trait:

- `check_fn`

In particular, we used this function to check for every expression in the analyzed code, and to determine whether it allows users to modify vectors without being the contract owners.

#### Caveats

If the owner validation is performed in an auxiliary function, this detector will not detect the unexpected revert.

# Results

Summarizing, with the tools mentioned above, we attempted to build detectors that would detect the vulnerability examples in our list.

For all cases, we were able to construct linters with Dylint, verifying that the detectors effectively recognized the issues in the vulnerable code and that no false positives occurred on the remediated examples.

We also built fuzzers for vulnerabilities #1-integer-overflow-or-underflow and #2-set-contract-storage, where input variation seemed like a possible application of this technique.

Finally, we also constructed some detectors with Semgrep for vulnerabilities [listar vulnerabilidades con ejemplos en semgrep].

## Detection of Vulnerability Examples with Tools

The following table summarizes our work on building detectors to identify vulnerabilities in our list of vulnerability examples.

We use ✅ to indicate that the vulnerability was detected in the vulnerable example (vuln.), ❎ to indicate that the vulnerability was not detected in the remediated example (remed.), and empty cells in cases where no detectors have been built.

<table>
  <thead>
    <tr>
      <th rowspan="2">Num.</th>
      <th rowspan="2">ID</th>
      <th rowspan="2">Category</th>
      <th colspan="2">Detection tool</th>
      <th colspan="2">Dylint</th>
      <th colspan="2">Cargo-fuzz</th>
      <th colspan="2">Semgrep</th>
    </tr>
    <tr>
      <th>vuln.</th>
      <th>remed.</th>
      <th>vuln.</th>
      <th>remed.</th>
      <th>vuln.</th>
      <th>remed.</th>
      <th>vuln.</th>
      <th>remed.</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>#1</td>
      <td>integer-overflow-or-underflow</td>
      <td>Arithmetic</td>
      <td>✅</td>
      <td>❎</td>
      <td>✅</td>
      <td>❎</td>
      <td>✅</td>
      <td>❎</td>
      <td>✅</td>
      <td>❎</td>
    </tr>
    <tr>
      <td>#2</td>
      <td>set-contract-storage</td>
      <td>Authorization</td>
      <td>✅</td>
      <td>❎</td>
      <td>✅</td>
      <td>❎</td>
      <td>✅</td>
      <td>❎</td>
      <td>✅</td>
      <td>❎</td>
    </tr>
    <tr>
      <td>#3</td>
      <td>reentrancy</td>
      <td>Reentrancy</td>
      <td>✅</td>
      <td>❎</td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
    </tr>
    <tr>
      <td>#4</td>
      <td>panic-error</td>
      <td>Validations and error handling</td>
      <td>✅</td>
      <td>❎</td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
    </tr>
    <tr>
      <td>#5</td>
      <td>unused-return-enum</td>
      <td>Validations and error handling</td>
      <td>✅</td>
      <td>❎</td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
    </tr>
    <tr>
      <td>#6</td>
      <td>dos-unbounded-operation</td>
      <td>DoS</td>
      <td>✅</td>
      <td>❎</td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
    </tr>
    <tr>
      <td>#7</td>
      <td>dos-unexpected-revert-with-vector</td>
      <td>DoS</td>
      <td>✅</td>
      <td>❎</td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
      <td></td>
    </tr>
  </tbody>
</table>

## Further Work

For the current list of vulnerable examples, Dylint seems to be a good option to use in the construction of a security analysis tool for Substrate Ink!. As new vulnerability examples are added to the list, Cargo-fuzz and Semgrep can be considered in parallel, especially in cases where the confidence of detectors implemented in Dylint is not satisfactory.

In particular, as Semgrep improves its Rust compatibility, it could be considered for detector building due to its ease of use and tainting capabilities.
