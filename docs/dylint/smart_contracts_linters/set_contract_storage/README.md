# template

# set_contract_storage_warning

### What it does
Checks for calls to env::set_contract_storage.

### Why is this bad?
Functions using keys as variables without proper access control or input sanitation can allow users to perform changes in arbitrary memory locations.

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
