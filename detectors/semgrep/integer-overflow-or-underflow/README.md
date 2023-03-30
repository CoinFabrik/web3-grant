# Integer Overflow and Integer Underflow
This semgrep rule checks for potential integer overflows or underflows in Rust
code by looking for arithmetic operations that could cause such issues. Here's
how it works:

The rule starts with a pattern-either block, which contains four patterns 
(pattern), each of which matches a different type of arithmetic operation. 
In each case, `$VAL1` and `$VAL2` are variables that represent integer values,
and the arithmetic operation is performed between them.

If any of these patterns matches in the Rust code, the rule issues a warning
message (message) indicating that an arithmetic 
operation may cause an integer overflow or underflow. 


## Approach and limitations
This is a linter-based approach that checks for the use of operations that 
might lead to unmanaged overflow and would accept code that uses Rust libraries
and/or  configurations  flags that would treat overflow during runtime as a
managed exception. Note that these checks would yield false positives if:
- there was user code that prevents overflows/underflows or, 
- values that may take operands during runtime would never produce over/underflows. 
 
Also, if even dynamically catching overflows were not a valid approach in a given
circumstance, a much more sophisticated static mechanism would be required to avoid
false negatives.     
