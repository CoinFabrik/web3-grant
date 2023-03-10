# DoS Unbounded Operation

## Configuration

- Detector ID: `dos-unbounded-operation`
- Analysis Category: `DoS`
- Severity: `High`

## Description

Each block in a Substrate Blockchain has an upper bound on the amount of gas that can be spent, and thus the amount computation that can be done. This is the Block Gas Limit. If the gas spent exceeds this limit, the transaction will fail.

In order to prevent a single transaction from consuming all the gas in a block, unbounded operations must be avoided. This includes loops that do not have a fixed number of iterations, and recursive calls.

## Exploit Scenario

In the following example, a contract has a function `add_payee` that allows adding a new element to a vector. The function `pay_out` iterates over the vector and transfers the value to the payee's address. The problem is that the `pay_out` function does not have a fixed number of iterations, and thus it can consume all the gas in a block.

A malicious user could call `add_payee` a large number of times, thus populating the vector with a large number of elements. Then, the function `pay_out` when iterating over all the elements, will consume all the gas in a block, and the transaction will fail, successfully performing a DoS attack.

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod dos_unbounded_operation {
    use ink::storage::Mapping;

    #[derive(Debug, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Payee {
        pub address: AccountId,
        pub value: Balance,
    }

    #[ink(storage)]
    pub struct DosUnboundedOperation {
        payees: Mapping<u128, Payee>,
        next_payee_ix: u128,
    }

    impl DosUnboundedOperation {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                payees: Mapping::new(),
                next_payee_ix: 0,
            }
        }

        #[ink(message, payable)]
        pub fn add_payee(&mut self) -> u128 {
            let address = self.env().caller();
            let value = self.env().transferred_value();
            let new_payee = Payee { address, value };

            self.payees.insert(self.next_payee_ix, &new_payee);
            self.next_payee_ix += 1;

            // Return the index of the new payee
            self.next_payee_ix - 1
        }

        #[ink(message)]
        pub fn get_payee(&self, id: u128) -> Option<Payee> {
            self.payees.get(id)
        }

        #[ink(message)]
        pub fn pay_out(&mut self) {
            for i in 0..self.next_payee_ix {
                let payee = self.payees.get(&i).unwrap();
                self.env().transfer(payee.address, payee.value).unwrap();
            }
        }
    }
}
```

### Deployment

An example can be found under the directory [vulnerable-example](./vulnerable-example). The exploit can be tested by running the end-to-end test called `pay_out_runs_out_of_gas`.

## Recommendation

The main recommendation is to change the form of payments to favor pull over push. This way, the contract does not need to iterate over a vector of payees, and thus it does not need to consume all the gas in a block. The payee could instead call a function that will transfer the value to the payee's address.

If looping over an array of unknown size is absolutely necessary, then it should be planned to potentially take multiple blocks, and therefore require multiple transactions.

## References

- https://consensys.github.io/smart-contract-best-practices/attacks/denial-of-service
- https://consensys.github.io/smart-contract-best-practices/development-recommendations/general/external-calls/
