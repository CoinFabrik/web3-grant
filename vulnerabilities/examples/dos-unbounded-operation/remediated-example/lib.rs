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
            self.next_payee_ix.checked_add(1).unwrap();

            // Return the index of the new payee
            self.next_payee_ix.checked_sub(1).unwrap()
        }

        #[ink(message)]
        pub fn get_payee(&self, id: u128) -> Option<Payee> {
            self.payees.get(id)
        }

        #[ink(message)]
        pub fn pay_out(&mut self, payee: u128) {
            let payee = self.payees.get(&payee).unwrap();
            self.env().transfer(payee.address, payee.value).unwrap();
        }
    }
}
