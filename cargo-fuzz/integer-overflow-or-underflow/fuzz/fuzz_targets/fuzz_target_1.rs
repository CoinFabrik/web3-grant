#![no_main]

use libfuzzer_sys::fuzz_target;
use integer_overflow_or_underflow::integer_overflow_underflow;
use arbitrary::Arbitrary;

#[derive(Clone, Debug, Arbitrary)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Input {
    pub valueForNew: u8,
    pub valueForAdd: u8,
    pub valueForSub: u8,
}


fuzz_target!(|input: Input| {
    let mut contract = integer_overflow_underflow::IntegerOverflowUnderflow::new(input.valueForNew);
    contract.add(input.valueForAdd);
    contract.sub(input.valueForSub);
    contract.get();
});


