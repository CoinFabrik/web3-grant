# template

# unexpected_revert_warning

### What it does
Checks for array pushes without access control.

### Why is this bad?
Arrays have a maximum size according to the storage cell. If the array is full, the push will revert. This can be used to prevent the execution of a function.

### Known problems
Only check the function call, so false positives could result.

### Example
```rust
// example code where a warning is issued
```
Use instead:
```rust
// example code that does not raise a warning
```
