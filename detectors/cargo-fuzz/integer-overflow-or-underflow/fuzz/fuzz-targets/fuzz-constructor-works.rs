#![no_main]
#![feature(core_panic)]

use arbitrary::Arbitrary;
use integer_overflow_or_underflow::integer_overflow_underflow::test_utils;
use libfuzzer_sys::fuzz_target;

#[derive(Clone, Debug, Arbitrary)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Input {
    pub value_for_new: u8,
}

fuzz_target!(|input: Input| {
    ink::env::test::run_test::<ink::env::DefaultEnvironment, _>(|_| {
        test_utils::constructor_works(input.value_for_new);
        Ok(())
    })
    .unwrap();
});
