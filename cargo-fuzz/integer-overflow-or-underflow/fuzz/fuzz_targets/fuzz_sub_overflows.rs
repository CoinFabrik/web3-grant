#![no_main]
#![feature(core_panic)]

use libfuzzer_sys::fuzz_target;
use integer_overflow_or_underflow::integer_overflow_underflow::IntegerOverflowUnderflow;
use arbitrary::Arbitrary;

#[derive(Clone, Debug, Arbitrary)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Input {
    pub value_for_sub: u8
}

fuzz_target!(|input: Input| {

          ::ink::env::test::run_test::<
                ::ink::env::DefaultEnvironment,
                _,
            >(|_| {
                    {
                        {
                            {
                                let mut contract = IntegerOverflowUnderflow::new(u8::MIN);
                                contract.sub(input.value_for_sub);
                                match (&contract.get(), &u8::MAX) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                            }
                        };
                        ::core::result::Result::Ok(())
                    }
                })
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    format_args!(
                        "{0}: {1:?}",
                        "# fn_name: the off-chain testing environment returned an error",
                        error
                    ),
                ));


});
