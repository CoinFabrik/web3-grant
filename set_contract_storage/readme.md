## Example of incorrect use of the `set_contract_storage` function

the user should not have control over the key because it implies writing any value of a mapping, lazy, or the main struct of the contract located in position 0 of the storage.

To compile this example, `cargo-contract` v2.0.1 is required.