#![no_main]
#![feature(core_panic)]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use set_contract_storage::erc20::test_utils;

#[derive(Clone, Debug, Arbitrary)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Input {
    pub initial_supply: u128,
    bob_initial_allowance: u128,
    bob_exploited_allowance: u128,
}

fuzz_target!(|input: Input| {
    ink::env::test::run_test::<::ink::env::DefaultEnvironment, _>(|_| {
        test_utils::misuse_contract_storage(input.initial_supply, input.bob_initial_allowance, input.bob_exploited_allowance);
        Ok(())
    })
    .unwrap();
});
