# Detectors
We designed a set of detectors for `ink!` smart contracts. We ran these
detectors on both the vulnerable and the remediated smart contracts we
prepared. The detectors are good in detecting the vulnerabilities they should
detect and and have no false positives on the remediated examples.

We selected a set of tools which implement techniques that are widely used for detecting vulnerabilities in source code (not necessarily smart contracts). Furthermore, the tools selected are open source, well maintained and can be easily configured/adapted to detect `ink!` vulnerabilities.

There follows a [description of the tools](#analysis-techniques-and-tools) we used and the criteria for their selection, and a list of the vulnerabilities detected and the [detectors](#implementation) used to do so.

Briefly speaking, for every vulnerability in our [list](../vulnerabilities/README.md),
we were able to construct linters with Dylint, verifying that
the detectors effectively recognized the issues in the vulnerable code and
that no false positives occurred on the remediated examples.
We used semgrep. Semgrep is a text search utility that understands --to some extent--
the programming language semantics, thus queries can go beyond searching for
regular expressions or navigating over abstract syntax trees to include conditions
on the role that particular strings have in the code (e.g., name of a function
as opposed to the name of a variable).
Finally, we managed to use cargo-fuzz to detect the [integer overflow](../vulnerabilities/examples/integer-overflow-or-underflow/README.md)
and the [unauthorized set contract storage vulnerability](../vulnerabilities/examples/set-contract-storage/README.md),
where input variation seemed like a possible application of this technique.


## Analysis Techniques and Tools
Being Ink! A Rust-based language, we looked at static analysis tools that
can analyze Rust code, first aiming at linters and then more precise static
analysis tools. Finally, we looked to some extent at dynamic analysis tools,
favouring fuzzers.

We selected:
- [dylint](https://github.com/trailofbits/dylint): A linter that allows for
quickly implementing the detection of programming errors. Quickness may
come at the cost of [precision and recall](https://en.wikipedia.org/wiki/Precision_and_recall).
- [semgrep](https://github.com/returntocorp/semgrep): A static analysis tool
supporting many languages, including Rust, that attempts to be a text search
command-line utility that is aware of source code semantics. Thus, this tools
allows for improving over dylint in terms of precision/recall.
- [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) is a subcommand for
fuzzing using the in-process, coverage-guided, evolutionary fuzzing engine
called [libFuzz](https://llvm.org/docs/LibFuzzer.html).


## Implementation
For each vulnerability in our list, we explain what tools and techniques were
applied for their detection, mentioning implementation caveats.

### 1. Integer Overflow and Integer Underflow
We based our analysis for overflow or underflow detection on the
[vulnerability example associated to this issue](../vulnerabilities/examples/integer-overflow-or-underflow/).

For this vulnerability, we were able to produce successful detectors using
[Dylint](./dylint/smart-contract-linters/integer-overflow-or-underflow/) and
[Cargo-fuzz](./cargo-fuzz/integer-overflow-or-underflow/), we detail their
implementation below.

#### Dylint
Our detector checks for integer arithmetic operations which could overflow or
panic. Specifically, it checks for any operators (+, -, &ast, <<, etc) which
are capable of overflowing according to the Rust Reference, or which can panic
(/, %). No bounds analysis or other more sophisticated reasoning is attempted.

__Implementation__:
In order to implement this detector we developed the following functions of the
[LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html)
trait:
- `check_body`
- `check_body_post`
- `check_expr`
- `check_expr_post`

In particular, we used these functions to check for every expression in the
analyzed code, and to determine whether it contains one the operations that
generate overflows/underflows. We also validate that the types being handled
are integers.

__Caveats__:
Rust includes a runtime check for integer overflows and underflows, which panics if any of these operations are detected. Adding this detector to the code will move the check to compile time, allowing the developer to handle the error in a more appropriate way.

#### Cargo-Fuzz
__Description__:
This detector uses fuzzing to find valid inputs that generate overflow or
underlfow when fed into the smart contract.

__Implementation__:
Using `ink::env test module`, we implemented a `fuzz_target` to execute the
different tests of the contract using input values generated with
`libfuzzer_sys`.

__Caveats__:
The fuzzer built for this example was written from the tests of the vulnerability
example and therefore is bound to be imprecise against other instances of this
vulnerability class. More work is needed against a wider set of examples to
improve precision.

### 2. Set contract storage
We based our analysis for set-contract-storage detection on the
[vulnerability example associated to this issue](../vulnerabilities/examples/set-contract-storage/).

For this vulnerability, we were able to produce successful detectors using
[Dylint](./dylint/smart-contract-linters/set-contract-storage/) and
[Cargo-fuzz](./cargo-fuzz/set-contract-storage/), we detail their
implementation below.

#### Dylint
__Description__:
This detector checks for calls to `env::set_contract_storage()` from arbitrary users.

__Implementation__:
In order to implement this detector we developed the following functions of the
[LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html)
trait:
- `check_fn`

In particular, we used this function to check for every expression in the
analyzed code, and to determine whether it calls the function
`env::set_contract_storage()`. When this happens we check if the function call
is performed within an `if` statement that determines whether the caller is the
contract owner, in which case no warning is raised.

__Caveats__:
If owner validation is performed with an auxiliary function, this detector will
not recognize the vulnerability.

#### Cargo-Fuzz
__Description__:
This detector uses fuzzing to find the storage key for
`env::set_contract_storage()` and user accounts used in this exploit.

__Implementation__:
Using `ink::env` test module, we implemented a `fuzz_target` to execute the
different tests of the contract using input values generated with
`libfuzzer_sys`.

__Caveats__:
The fuzzer built for this vulnerability was written using the tests of the
vulnerability example that we worked with. Therefore it only works for this
contract. Furthermore, the bytesize of the arguments makes it very hard to
find them using this technique.

### 3. Reentrancy
We based our analysis for set-contract-storage detection on the
[vulnerability example associated to this issue](../vulnerabilities/examples/reentrancy/).

For this vulnerability, we were able to produce successfull detectors using
[Dylint](./dylint/smart-contract-linters/reentrancy/) we detail the implementation below.

#### Dylint
__Description__:
This detector checks the usage of the flag `set_allow_reentry(true)`, followed
by an `invoke_contract_call()` and changes in contract state performed by
assignments or inserts in mappings.

__Implementation__:
In order to implement this detector we developed the following functions of the
[LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html) trait:
- `check_fn`

In particular, we used this function to check for every expression in the
analyzed code, and to determine whether it calls the `flag set_allow_reentry(true)`
and the function `invoke_contract_call()`. The `check_fn` function is also used to
 detect for assignments (`=`, `+=`, `-=`, etc) and calls to the `insert()` function.

__Caveats__:
If the usage of `set_allow_reentry(true)` or later state changes are performed in
an auxiliary function, this detector will not detect the reentrancy.

### 4. Panic error
We based our analysis for set-contract-storage detection on the
[vulnerability example associated to this issue](../vulnerabilities/examples/panic-error/).

For this vulnerability, we were able to produce successful detectors using
[Dylint](./dylint/smart-contract-linters/panic-error/) we detail the implementation below.

#### Dylint
__Description__:
This detector checks the usage of the `panic!` macro.

__Implementation__:
In order to implement this detector we developed the following functions of the
[LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html)
trait:
- `check_expr`

In particular, we used this function to check for every expression in the analyzed code, and
to determine whether it uses the `panic!` macro.

__Caveats__:
None.

### 5. Unused return enum
We based our analysis for unused-return-enum detection on the
[vulnerability example associated to this issue](/vulnerabilities/examples/unused-return-enum/).

For this vulnerability, we were able to produce successful detectors using
[Dylint](./dylint/smart-contract-linters/unused-return-enum/).

#### Dylint
__Description__:
This detector checks that if the function return value is of type `Result` then
there exists at least one return value that uses `Err` and another return value
that uses `Ok`.

__Implementation__:
In order to implement this detector we developed the following functions of the
[LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html)
trait:
- `check_fn`
- `visitor`

In particular, we used this function together with a visitor to check for every
expression of a function with return type `Result` whether its returns values
are at least an `Err` and an `Ok`.

__Caveats__:
If definitions of Err() and/or Ok() are in the code but do not flow to the return value due to the definition of a variable or because they are defined in a dead code block, the warning will not be shown. If the definitions are made in an auxiliary method, the warning will be shown, resulting in a false positive.

### 6. DoS Unbounded Operation
We based our analysis for dos-unbounded-operation detection on the
[vulnerability example associated to this issue](../vulnerabilities/examples/dos-unexpected-revert-with-vector/).

For this vulnerability, we were able to produce successful detectors using
[Dylint](./dylint/smart-contract-linters/dos-unbounded-operation/).

#### Dylint
__Description__:
This detector checks that when using for or while loops, their conditions limit the execution to a constant number of iterations.

__Implementation__:
In order to implement this detector we developed the following functions of the
[LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html)
trait:
- `check_expr`

In particular, we have used this function to search for every for or while loop
through the code's expressions and determine if their conditions contain variables or
function calls.

__Caveats__:
False positives are to be expected when using variables that can only be set using controlled flows that limit the values within acceptable ranges. These cases can be detected by using tainting techniques and/or interprocedural dataflow analysis.


### 7. DoS Unexpected Revert With Vector
We based our analysis for set-contract-storage detection on the
[vulnerability example associated to this issue](../vulnerabilities/examples/dos-unexpected-revert-with-vector/).

For this vulnerability, we were able to produce successful detectors using
[Dylint](./dylint/smart_contracts_linters/dos-unexpected-revert),
we detail the implementation below.

#### Dylint
__Description__:
This detector checks that only the owner can manipulate vectors' content.

__Implementation__:
In order to implement this detector we developed the following functions of the
[LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html)
trait:
- `check_fn`

In particular, we used this function to check for every expression in the
analyzed code, and to determine whether it allows users to modify vectors
without being the contract owners.

__Caveats__:
If the owner validation is performed in an auxiliary function, this detector
will not detect the unexpected revert.








## Detection of Vulnerability Examples with Tools
The following table summarizes our work on building detectors to identify
vulnerabilities in our list of vulnerability examples.

We use ✅ to indicate that the vulnerability was detected in the vulnerable
example (vuln.), ❎ to indicate that the vulnerability was not detected in
the remediated example (remed.), and empty cells in cases where no detectors
have been built.

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
