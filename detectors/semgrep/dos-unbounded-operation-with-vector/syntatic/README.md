This semgrep rule is designed to identify potentially unbounded loops in `Ink!` contracts, which can lead to a denial of service (DoS) error.


The rule contains a pattern that is designed to match any of the following constructs:

- A for loop (for $X in $START..$END {...}) with an inclusive range where either the start or end is not a known constant value.
- A function with at least one parameter that has a type of $ENDTYPE, where the name of the parameter is $END and it is used in a similar range expression (pub fn $FN_NAME(...,$END:$ENDTYPE,...) {...}).
- A function that takes `self` as an argument and contains a for loop where the end of the range expression is a field of self rather than a known constant value (pub fn $FN_NAME(&mut self,...) {...} for $X in $START..self.$FIELD {...}).

Note that this rule may generate false positives or false negatives depending on the specific code being analyzed, and it should be used as a guide to identify potentially problematic loops that require further investigation.

Approach and limitations: Conceptually, the issue should be warned when there is no guarantee that the expression used as an exit condition of a loop evaluates into either a known interval or a known subset of values.  

This simple linter-based approach detects loops in which the upper bound is given by an expression that explicitly uses a contract state field or a formal parameter. Thus, known false negatives are formal parameter/field data non-trivially flowing into the upper bound expression (this could be improved by either further restricting expressions used in loops or by using static inter-procedural information-flow or tainting). Among false positives, it could be mentioned code using contract fields that actually take only known values (some type information might be used to improve this) or any sort of sanitization mechanism of formal parameters (e.g., only loop if the used formal parameter is less than a given value). A static analysis of possible values of relevant expressions in an abstract domain might be the adequate approach to address these situations.  
