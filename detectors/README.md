# Detectors
We designed a set of detectors for `ink!` smart contracts. We ran these
detectors on both the vulnerable and the remediated smart contracts we
prepared. The detectors are good in detecting the vulnerabilities they should
detect and have no false positives on the remediated examples.

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
[Dylint](./dylint/smart-contract-linters/integer-overflow-or-underflow/),
[Semgrep](./semgrep/integer-overflow-or-underflow/) and
[Cargo-fuzz](./cargo-fuzz/integer-overflow-or-underflow/), we detail their
implementation below.

#### Dylint
Our detector checks for integer arithmetic operations which could overflow or
panic. Specifically, it checks for any operators (+, -, *, <<, etc) which 
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

#### Semgrep
__Description__:
This semgrep rule checks for potential integer overflows or underflows in `Ink!` contracts, by looking for arithmetic operations that could cause such issues.

__Implementation__:
The rule starts with a pattern-either block, which contains four patterns (pattern), 
each of which matches a different type of arithmetic operation. 
In each case, `$VAL1` and `$VAL2` are variables that represent integer values, and the arithmetic operation is performed between them.
If any of these patterns matches in the Rust code, the rule issues a warning message (message) indicating that an arithmetic 
operation may cause an integer overflow or underflow. 

__Caveats__:
This is a linter-based approach that checks for the use of operations that might lead to unmanaged overflow and would accept code that uses Rust libraries and/or configurations flags that would treat overflow during runtime as a managed exception. Note that these checks would yield false positives if:
- there was user code that prevents overflows/underflows or, 
- values that may take operands during runtime would never produce over/underflows.

### 2. Set contract storage
We based our analysis for set-contract-storage detection on the
[vulnerability example associated to this issue](../vulnerabilities/examples/set-contract-storage/).

For this vulnerability, we were able to produce successful detectors using
[Dylint](./dylint/smart-contract-linters/set-contract-storage/),
[Semgrep](./semgrep/set-contract-storage/) and
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

#### Semgrep
__Description__:
These semgrep rules are designed to identify potential misuse of the `env::set_contract_storage` function by unauthorized account.

__Implementation__:
__Syntactic rule__
This semgrep rule is designed to identify potential misuse of the `env::set_contract_storage` function. 

The rule consists of several patterns that are combined using logical operators (pattern-inside, pattern-not-inside). 
The first pattern (pattern-inside) matches a function definition that has a parameter $IK of some type $T. 
The function may have other parameters and a function body, which are not relevant for this rule.

The second and third patterns (pattern-not-inside) are used to exclude certain conditions that could authorize 
the use of `set_contract_storage` by anyone other than the contract owner. 

The fourth pattern (pattern) matches a call to the env::set_contract_storage function, passing the variable $IK as the first argument. 
This function is used to write data to the contract storage, which can be critical for the contract's integrity.
If the pattern matches and no authorization condition is found, a warning message is issued.

__Tainting rule__

This semgrep rule is similar to the previous one, but it uses taint tracking to identify potential security issues in Rust smart contracts. Here's how it works:

The rule starts with a source pattern (pattern-sources), which matches a function definition that has a parameter $IK of some type $T. This is similar to the first pattern in the previous rule.

The next pattern (pattern-sinks) matches a call to the env::set_contract_storage function, passing the variable $IK as the first argument. This is also similar to the fourth pattern in the previous rule.

However, instead of using exclusion patterns to identify authorized use of `set_contract_storage()`, this rule uses sanitizers (pattern-sanitizers) to remove taint from the variable $IK under certain conditions. Specifically, the sanitizers check whether there is an if statement inside the function body that compares the caller of the contract (`self.env().caller()`) with the contract owner (`self.owner`) using either == or != operators. If such a condition is found, the variable $IK is considered to be sanitized, meaning that it is no longer considered a potential security issue.

Finally, if the `env::set_contract_storage` function is called with an unsanitized $IK variable, a warning message is issued (message), explaining that the parameter $IK is user-controlled and can potentially corrupt the contract storage. The message advises that only the contract owner should be allowed to perform this operation.

__Caveats__:
If ownership validation is performed in an auxiliary function, the linter will not be able to identify it, and the warning will be indicated as a false positive.

### 3. Reentrancy
We based our analysis for reentancy detection on the 
[vulnerability example associated to this issue](../vulnerabilities/examples/reentrancy/).

For this vulnerability, we were able to produce successfull detectors using 
[Dylint](./dylint/smart-contract-linters/reentrancy/), we detail the implementation below.

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
an auxiliary function, this detector will not detect the reentrancy. Also, we miss
to analyze if the `call` variable that is passed to `invoke_contract_call()` is
associated to `set_allow_reentry(true)`.

### 4. Panic error
We based our analysis for panic error detection on the 
[vulnerability example associated to this issue](../vulnerabilities/examples/panic-error/).

For this vulnerability, we were able to produce successful detectors using
[Dylint](./dylint/smart-contract-linters/panic-error/) and
[Semgrep](./semgrep/panic-error/), we detail the implementation below.

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
While this lint detects explicit use of panic! macro, there are some ways to make the program panic such as unwrap() or expect().

#### Semgrep
__Description__:
This detector checks the usage of the `panic!` macro.

__Implementation__:
The rule consists of a single pattern (pattern) that matches the `panic!` macro, which takes an error message as an argument and abruptly terminates the program when executed.

__Caveats__:
There are some ways to make the program panic such as unwrap() or expect() which are not handled by this rule.

### 5. Unused return enum
We based our analysis for unused return enum detection on the 
[vulnerability example associated to this issue](../vulnerabilities/examples/unused-return-enum/).

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

In particular, we used this function together with a visitor to check for every
expression of a function with return type `Result` whether its returns values
are at least an `Err` and an `Ok`.

__Caveats__:
If definitions of Err() and/or Ok() are in the code but do not flow to the return value due to the definition of a variable or because they are defined in a dead code block, the warning will not be shown. If the definitions are made in an auxiliary method, the warning will be shown, resulting in a false positive.

### 6. DoS Unbounded Operation
We based our analysis for DoS Unbounded Operation detection on the 
[vulnerability example associated to this issue](../vulnerabilities/examples/dos-unexpected-revert-with-vector/).

For this vulnerability, we were able to produce successful detectors using
[Dylint](./dylint/smart-contract-linters/dos-unbounded-operation/) and
[Semgrep](./semgrep/dos-unbounded-operation/).

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

#### Semgrep
__Description__:
This semgrep rule is designed to identify potentially unbounded loops in `Ink!` contracts, which can lead to a denial of service (DoS) error.

__Implementation__:
The rule contains a pattern that is designed to match any of the following constructs:

- A for loop (for $X in $START..$END {...}) with an inclusive range where either the start or end is not a known constant value.
- A function with at least one parameter that has a type of $ENDTYPE, where the name of the parameter is $END and it is used in a similar range expression (pub fn $FN_NAME(...,$END:$ENDTYPE,...) {...}).
- A function that takes `self` as an argument and contains a for loop where the end of the range expression is a field of self rather than a known constant value (pub fn $FN_NAME(&mut self,...) {...} for $X in $START..self.$FIELD {...}).

__Caveats__:
Known false negatives are formal parameter/field data non-trivially flowing into the upper bound expression
Among false positives, it could be mentioned code using contract fields that actually take only known values or any sort of sanitization mechanism of formal parameters.

### 7. DoS Unexpected Revert With Vector
We based our analysis for DoS unexpected revert with vector detection on the 
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
    </tr>
    <tr>
      <td>#4</td>
      <td>panic-error</td>
      <td>Validations and error handling</td>
      <td>✅</td>
      <td>❎</td>
      <td></td>
      <td></td>
      <td>✅</td>
      <td>❎</td>
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
    </tr>
    <tr>
      <td>#6</td>
      <td>dos-unbounded-operation</td>
      <td>DoS</td>
      <td>✅</td>
      <td>❎</td>
      <td></td>
      <td></td>
      <td>✅</td>
      <td>❎</td>
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
    </tr>
  </tbody>
</table>
