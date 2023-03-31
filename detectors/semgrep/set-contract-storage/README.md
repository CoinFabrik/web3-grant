# Unauthorized Set Contract Storage
These semgrep rules are designed to identify potential misuse of the 
`env::set_contract_storage` function by unauthorized account. 

# Syntactic rule
This semgrep rule is designed to identify potential misuse of the 
`env::set_contract_storage` function. 

The rule consists of several patterns that are combined using logical operators
(pattern-inside, pattern-not-inside). 
The first pattern (pattern-inside) matches a function definition that has a 
parameter `$IK` of some type `$T`. 
The function may have other parameters and a function body, which are not relevant
for this rule.

The second and third patterns (pattern-not-inside) are used to exclude certain 
conditions that could authorize the use of `set_contract_storage` by anyone other
than the contract owner. 

The fourth pattern (pattern) matches a call to the `env::set_contract_storage` 
function, passing the variable `$IK` as the first argument. 
This function is used to write data to the contract storage, which can be critical
for the contract's integrity.
If the pattern matches and no authorization condition is found, a warning message is 
issued.

## Caveats
Conceptually, this detector should detect a problem in the information flow: 
user-provided data being used for the invocation of the set_contract_storage 
function without prior sanitization. We assume that if the data is entered by 
the contract owner, it has been sanitized beforehand. If ownership validation 
is performed in an auxiliary function, the linter will not be able to identify
it, and the warning will be indicated as a false positive.

# Tainting rule 
This semgrep rule is similar to the previous one, but it uses taint tracking to 
identify potential security issues in Rust smart contracts. Here's how it works:

The rule starts with a source pattern (pattern-sources), which matches a function
definition that has a parameter `$IK` of some type `$T`. This is similar to the first 
pattern in the previous rule.

The next pattern (pattern-sinks) matches a call to the `env::set_contract_storage` 
function, passing the variable `$IK` as the first argument. This is also similar to 
the fourth pattern in the previous rule.

However, instead of using exclusion patterns to identify authorized use of 
`set_contract_storage`, this rule uses sanitizers (pattern-sanitizers) to remove taint
from the variable `$IK` under certain conditions. Specifically, the sanitizers check
whether there is an if statement inside the function body that compares the caller 
of the contract (`self.env().caller()`) with the contract owner (`self.owner`) using
either `==` or `!=` operators. If such a condition is found, the variable `$IK` is
considered to be sanitized, meaning that it is no longer considered a potential
 security issue.

Finally, if the `env::set_contract_storage` function is called with an unsanitized
`$IK` variable, a warning message is issued (message), explaining that the parameter 
`$IK` is user-controlled and can potentially corrupt the contract storage. The 
message advises that only the contract owner should be allowed to perform this 
operation.

## Caveats
If ownership validation is performed in an auxiliary function, the linter will 
not be able to identify it, and the warning will be indicated as a false positive.
