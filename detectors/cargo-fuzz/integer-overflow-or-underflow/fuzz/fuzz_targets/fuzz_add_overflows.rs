#![no_main]
#![feature(core_panic)]

use arbitrary::Arbitrary;
use integer_overflow_or_underflow::integer_overflow_underflow::test_utils;
use libfuzzer_sys::fuzz_target;

#[derive(Clone, Debug, Arbitrary)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Input {
    pub value_to_add: u8,
}

fuzz_target!(|input: Input| {
    ink::env::test::run_test::<ink::env::DefaultEnvironment, _>(|_| {
        test_utils::add_overflows(u8::MAX, input.value_to_add);
        Ok(())
    })
    .unwrap();
});
