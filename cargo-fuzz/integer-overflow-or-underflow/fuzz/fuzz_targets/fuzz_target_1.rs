#![no_main]
#![feature(core_panic)]

use libfuzzer_sys::fuzz_target;
use integer_overflow_or_underflow::integer_overflow_underflow::IntegerOverflowUnderflow;
use arbitrary::Arbitrary;

#[derive(Clone, Debug, Arbitrary)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Input {

}

fuzz_target!(|input: Input| {

    // do nothing

});


