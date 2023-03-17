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
use my_contract::erc20::Error;
use my_contract::erc20::MisusedSetContractStorage;

#[derive(Clone, Debug, Arbitrary)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Input {}

type Environment = <Erc20 as ::ink::env::ContractEnv>::Env;
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


    fn set_caller(sender: AccountId) {
        ink::env::test::set_caller::<Environment>(sender);
    }

    fuzz_target!(|input: Input| {
          ::ink::env::test::run_test::<
                ::ink::env::DefaultEnvironment,
                _,

         >(|_| {

                            {
                        {
                            {
                                let mut contract = Erc20::new(100);
                                let alice_account_id: AccountId = ink_e2e::alice::<
                                    ink_e2e::PolkadotConfig,
                                >()
                                    .account_id()
                                    .0
                                    .into();
                                let bob_account_id: AccountId = ink_e2e::bob::<
                                    ink_e2e::PolkadotConfig,
                                >()
                                    .account_id()
                                    .0
                                    .into();
                                let allowance = contract
                                    .allowance(alice_account_id, bob_account_id);
                                match (&allowance, &0) {
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
                                ink::env::test::set_caller::<
                                    ::ink::env::DefaultEnvironment,
                                >(alice_account_id);
                                contract
                                    .approve(bob_account_id, 10)
                                    .expect("Approve failed");
                                let allowance = contract
                                    .allowance(alice_account_id, bob_account_id);
                                match (&allowance, &10) {
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
                                ink::env::test::set_caller::<
                                    ::ink::env::DefaultEnvironment,
                                >(bob_account_id);
                                contract
                                    .misused_set_contract_storage(
                                        [
                                            255,
                                            0,
                                            0,
                                            0,
                                            212,
                                            53,
                                            147,
                                            199,
                                            21,
                                            253,
                                            211,
                                            28,
                                            97,
                                            20,
                                            26,
                                            189,
                                            4,
                                            169,
                                            159,
                                            214,
                                            130,
                                            44,
                                            133,
                                            88,
                                            133,
                                            76,
                                            205,
                                            227,
                                            154,
                                            86,
                                            132,
                                            231,
                                            165,
                                            109,
                                            162,
                                            125,
                                            142,
                                            175,
                                            4,
                                            21,
                                            22,
                                            135,
                                            115,
                                            99,
                                            38,
                                            201,
                                            254,
                                            161,
                                            126,
                                            37,
                                            252,
                                            82,
                                            135,
                                            97,
                                            54,
                                            147,
                                            201,
                                            18,
                                            144,
                                            156,
                                            178,
                                            38,
                                            170,
                                            71,
                                            148,
                                            242,
                                            106,
                                            72,
                                        ],
                                        1000,
                                    )
                                    .expect("Set contract storage failed");
                                let allowance = contract
                                    .allowance(alice_account_id, bob_account_id);
                                match (&allowance, &1000) {
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
}