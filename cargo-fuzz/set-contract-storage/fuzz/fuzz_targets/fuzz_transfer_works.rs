#![no_main]
#![feature(core_panic)]
#![feature(prelude_import)]

#[prelude_import]
use my_contract::erc20::Erc20;
use my_contract::erc20::BaseErc20;
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use ink_env::topics::PrefixedValue;
use my_contract::erc20::Transfer;

#[derive(Clone, Debug, Arbitrary)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Input {}

type AccountId = <<Erc20 as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::AccountId;
type Balance = <<Erc20 as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Balance;
type Hash = <<Erc20 as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Hash;
type Event = <Erc20 as ::ink::reflect::ContractEventBase>::Type;

pub mod tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;
    use ink::{
        env::hash::{Blake2x256, CryptoHash, HashOutput},
        primitives::Clear,
    };

    fn assert_transfer_event(
        event: &ink::env::test::EmittedEvent,
        expected_from: Option<AccountId>,
        expected_to: Option<AccountId>,
        expected_value: Balance,
    ) {
        // let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
        //     .expect("encountered invalid contract event data buffer");
        //
        // if let Event::Transfer(Transfer {  from,to,value }) = decoded_event {
        //     match (&from, &expected_from) {
        //         (left_val, right_val) => {
        //             if !(*left_val == *right_val) {
        //                 let kind = ::core::panicking::AssertKind::Eq;
        //                 ::core::panicking::assert_failed(
        //                     kind,
        //                     &*left_val,
        //                     &*right_val,
        //                     ::core::option::Option::Some(
        //                         format_args!("encountered invalid Transfer.from"),
        //                     ),
        //                 );
        //             }
        //         }
        //     };
        //     match (&to, &expected_to) {
        //         (left_val, right_val) => {
        //             if !(*left_val == *right_val) {
        //                 let kind = ::core::panicking::AssertKind::Eq;
        //                 ::core::panicking::assert_failed(
        //                     kind,
        //                     &*left_val,
        //                     &*right_val,
        //                     ::core::option::Option::Some(
        //                         format_args!("encountered invalid Transfer.to"),
        //                     ),
        //                 );
        //             }
        //         }
        //     };
        //     match (&value, &expected_value) {
        //         (left_val, right_val) => {
        //             if !(*left_val == *right_val) {
        //                 let kind = ::core::panicking::AssertKind::Eq;
        //                 ::core::panicking::assert_failed(
        //                     kind,
        //                     &*left_val,
        //                     &*right_val,
        //                     ::core::option::Option::Some(
        //                         format_args!("encountered invalid Trasfer.value"),
        //                     ),
        //                 );
        //             }
        //         }
        //     };
        // } else {
        //     ::core::panicking::panic_fmt(
        //         format_args!(
        //             "encountered unexpected event kind: expected a Transfer event"
        //         ),
        //     )
        // }
        // fn encoded_into_hash<T>(entity: &T) -> Hash
        //     where
        //         T: scale::Encode,
        // {
        //     let mut result = Hash::CLEAR_HASH;
        //     let len_result = result.as_ref().len();
        //     let encoded = entity.encode();
        //     let len_encoded = encoded.len();
        //     if len_encoded <= len_result {
        //         result.as_mut()[..len_encoded].copy_from_slice(&encoded);
        //         return result;
        //     }
        //     let mut hash_output = <<Blake2x256 as HashOutput>::Type as Default>::default();
        //     <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
        //     let copy_len = core::cmp::min(hash_output.len(), len_result);
        //     result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
        //     result
        // }
        // let expected_topics = [
        //     encoded_into_hash(
        //         &PrefixedValue {
        //             prefix: b"",
        //             value: b"Erc20::Transfer",
        //         },
        //     ),
        //     encoded_into_hash(
        //         &PrefixedValue {
        //             prefix: b"Erc20::Transfer::from",
        //             value: &expected_from,
        //         },
        //     ),
        //     encoded_into_hash(
        //         &PrefixedValue {
        //             prefix: b"Erc20::Transfer::to",
        //             value: &expected_to,
        //         },
        //     ),
        //     encoded_into_hash(
        //         &PrefixedValue {
        //             prefix: b"Erc20::Transfer::value",
        //             value: &expected_value,
        //         },
        //     ),
        // ];
        // for (n, (actual_topic, expected_topic)) in event
        //     .topics
        //     .iter()
        //     .zip(expected_topics)
        //     .enumerate()
        // {
        //     let topic = <Hash as scale::Decode>::decode(&mut &actual_topic[..])
        //         .expect("encountered invalid topic encoding");
        //     match (&topic, &expected_topic) {
        //         (left_val, right_val) => {
        //             if !(*left_val == *right_val) {
        //                 let kind = ::core::panicking::AssertKind::Eq;
        //                 ::core::panicking::assert_failed(
        //                     kind,
        //                     &*left_val,
        //                     &*right_val,
        //                     ::core::option::Option::Some(
        //                         format_args!("encountered invalid topic at {0}", n),
        //                     ),
        //                 );
        //             }
        //         }
        //     };
        // }
    }


    fuzz_target!(|input: Input| {
          ::ink::env::test::run_test::<
                ::ink::env::DefaultEnvironment,
                _,

         >(|_| {
                    {
                        {
                            {
                                let initial_supply = 100;
                                let mut erc20 = Erc20::new(initial_supply);
                                let accounts = ink::env::test::default_accounts::<
                                    ink::env::DefaultEnvironment,
                                >();
                                match (&erc20.balance_of(accounts.bob), &0) {
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
                                match (&erc20.transfer(accounts.bob, 10), &Ok(())) {
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
                                match (&erc20.balance_of(accounts.bob), &10) {
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
                                let emitted_events = ink::env::test::recorded_events()
                                    .collect::<Vec<_>>();
                                match (&emitted_events.len(), &2) {
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
                                assert_transfer_event(
                                    &emitted_events[0],
                                    None,
                                    Some(AccountId::from([0x01; 32])),
                                    100,
                                );
                                assert_transfer_event(
                                    &emitted_events[1],
                                    Some(AccountId::from([0x01; 32])),
                                    Some(AccountId::from([0x02; 32])),
                                    10,
                                );
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
}