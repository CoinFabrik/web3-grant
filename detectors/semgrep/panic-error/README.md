# panic error
This semgrep rule checks for instances of the `panic!` macro in Ink! contracts.

The rule consists of a single pattern (pattern) that matches the `panic!` macro, 
which takes an error message as an argument and abruptly terminates the program 
when executed. 

