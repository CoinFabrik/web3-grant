#![no_main]
#![feature(core_panic)]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use set_contract_storage::erc20::test_utils;
use set_contract_storage::erc20::BaseErc20;

#[derive(Clone, Debug, Arbitrary)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Input {
    pub initial_supply: u128,
    bob_initial_allowance: u128,
    bob_exploited_allowance: u128,
    storage_location: [u8; 68],
}

fuzz_target!(|input: Input| {
    ink::env::test::run_test::<::ink::env::DefaultEnvironment, _>(|_| {
        let erc20 = test_utils::misuse_contract_storage(
            input.initial_supply,
            input.bob_initial_allowance,
            input.bob_exploited_allowance,
            input.storage_location,
        );
        let alice_account_id: ink::primitives::AccountId = [
            212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133,
            88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
        ]
        .into();
        let bob_account_id: ink::primitives::AccountId = [
            142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54,
            147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
        ]
        .into();
        let allowance = erc20.allowance(alice_account_id, bob_account_id);
        assert_eq!(
            allowance, input.bob_initial_allowance,
            "Fuzzer has found the invalid input"
        );
        Ok(())
    })
    .unwrap();
});
