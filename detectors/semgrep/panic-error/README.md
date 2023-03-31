# panic error
This semgrep rule checks for instances of the `panic!` macro in `ink!` contracts.

The rule consists of a single pattern (pattern) that matches the `panic!` macro, 
which takes an error message as an argument and abruptly terminates the program 
when executed. 

## Approach and limitations
While this rule detects explicit use of panic! macro, there are some ways to make the program panic such as unwrap() or expect().