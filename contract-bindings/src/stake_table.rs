pub use stake_table::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod stake_table {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blsVK"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G2Point"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lookupNode"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lookupNode"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("blsVK"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BN254.G2Point"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IStakeTable.Node"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lookupStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lookupStake"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("blsVK"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BN254.G2Point"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nextExitEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nextExitEpoch"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nextRegistrationEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nextRegistrationEpoch",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("numPendingExit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("numPendingExit"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("numPendingRegistrations"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("numPendingRegistrations",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("register"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("register"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blsVK"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G2Point"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("schnorrVK"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct EdOnBN254.EdOnBN254Point",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakeType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("enum IStakeTable.StakeType",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blsSig"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("validUntilEpoch"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestExit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestExit"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("blsVK"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BN254.G2Point"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalKeys"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalKeys"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalStake"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalVotingStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalVotingStake"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawFunds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawFunds"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("blsVK"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BN254.G2Point"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("Register"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("Register"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                            ],),
                            indexed: false,
                        },
                    ],
                    anonymous: false,
                },],
            )]),
            errors: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("BLSSigVerificationFailed"),
                ::std::vec![::ethers::core::abi::ethabi::AbiError {
                    name: ::std::borrow::ToOwned::to_owned("BLSSigVerificationFailed",),
                    inputs: ::std::vec![],
                },],
            )]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STAKETABLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PC`\x06Ua\x1C\x1C\x80a\0$`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80cH\x8B\xDA\xBC\x11a\0\x8CW\x80c\x8B\x0E\x9F?\x11a\0fW\x80c\x8B\x0E\x9F?\x14a\x01\xC4W\x80c\xBF\x82H\xDD\x14a\x01\xDFW\x80c\xC7,\xC7\x17\x14a\x01\xF7W\x80c\xDD.\xD3\xEC\x14a\x02\nW`\0\x80\xFD[\x80cH\x8B\xDA\xBC\x14a\x01VW\x80cJ\xA7\xC2\x7F\x14a\x01nW\x80cw\x1FoD\x14a\x01\x91W`\0\x80\xFD[\x80c\x0C$\xAF\x18\x14a\0\xD4W\x80c\x16\xFE\xFE\xD7\x14a\x01\x04W\x80c*\xDD\xA1\xC1\x14a\x01\x15W\x80c,S\x05\x84\x14a\x015W\x80c;\t\xC2g\x14a\x01=W\x80cC\x17\xD0\x0B\x14a\x01EW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x14fV[a\x02\x1DV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x05T`\x01`\x01`@\x1B\x03\x16a\0\xE7V[a\x01(a\x01#6`\x04a\x14fV[a\x02PV[`@Qa\0\xFB\x91\x90a\x15 V[a\0\xE7a\x03cV[a\0\xE7a\x03}V[`\x04T`@Q\x90\x81R` \x01a\0\xFBV[`\x03T`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xFBV[a\x01\x81a\x01|6`\x04a\x14fV[a\x03\xA5V[`@Q\x90\x15\x15\x81R` \x01a\0\xFBV[a\x01\xA4a\x01\x9F6`\x04a\x15LV[a\x03\xE2V[`@\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\0\xFBV[`\x01T`\x02T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xFBV[`\x05T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\0\xE7V[a\x01\x81a\x02\x056`\x04a\x15\xA3V[a\x04VV[a\0\xE7a\x02\x186`\x04a\x14fV[a\x08 V[`\0\x80a\x02)\x83a\x08\x94V[`\0\x90\x81R` \x81\x90R`@\x81 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16\x90U\x93\x92PPPV[a\x02\x95`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\xA0\x82\x01R\x90V[`\0a\x02\xA0\x83a\x08\x94V[`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xC0\x81\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x84R\x93\x94P\x91\x92\x90\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x02\xEBWa\x02\xEBa\x14\x89V[`\x01\x81\x11\x15a\x02\xFCWa\x02\xFCa\x14\x89V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x93\x92PPPV[`\0a\x03ma\x08\xDFV[a\x03x\x90`\x01a\x16;V[\x90P\x90V[`\x05T`\0\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81\x03a\x03\x9FWP`\0\x90V[P`\x01\x90V[`\0\x80a\x03\xB1\x83a\x08\x94V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x90\x81\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16\x90U\x93\x92PPPV[`\0\x80`\0a\x03\xF0\x85a\x08\x94V[`\0\x81\x81R` \x81\x90R`@\x90 \x80T\x91\x92P\x85\x91`\x15\x90a\x04#\x90\x84\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x16;V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\0\x80\x92P\x92PP\x92P\x92\x90PV[`\0\x80a\x04b\x88a\x08\x94V[`\0\x81\x81R` \x81\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x04\xAEWa\x04\xAEa\x14\x89V[`\x01\x81\x11\x15a\x04\xBFWa\x04\xBFa\x14\x89V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x05\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FThe node has already been regist`D\x82\x01Rc\x19\\\x99Y`\xE2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R`\0\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90Pa\x05\xD2\x81a\x05\xBE6\x89\x90\x03\x89\x01\x89a\x16\xEDV[a\x05\xCD6\x8E\x90\x03\x8E\x01\x8Ea\x17\tV[a\x08\xFCV[`\x000`\x01`\x01`\xA0\x1B\x03\x16c,S\x05\x84`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x066\x91\x90a\x17\x8CV[\x90P\x85`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\x06\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid next registration epoch.`D\x82\x01R`d\x01a\x05\x82V[3\x83R`\x01`\x01`@\x1B\x03\x89\x16`@\x84\x01R` \x83\x01\x88`\x01\x81\x11\x15a\x06\xC2Wa\x06\xC2a\x14\x89V[\x90\x81`\x01\x81\x11\x15a\x06\xD5Wa\x06\xD5a\x14\x89V[\x90RPa\x06\xE76\x8B\x90\x03\x8B\x01\x8Ba\x16\xEDV[`\xA0\x84\x01R`\x01`\x01`@\x1B\x03\x81\x16``\x84\x01R`\0\x84\x81R` \x81\x81R`@\x90\x91 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x83U\x92\x86\x01Q\x86\x93\x90\x91\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\x07UWa\x07Ua\x14\x89V[\x02\x17\x90UP`@\x82\x81\x01Q\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17\x83U``\x84\x01Q`\x01\x84\x01\x80T`\x80\x87\x01Q\x92\x84\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x17`\x01`@\x1B\x92\x90\x93\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90U`\xA0\x90\x92\x01Q\x80Q`\x02\x83\x01U` \x01Q`\x03\x90\x91\x01UQ\x7F\xD1\xFE\xEEYM\x06\x1A\xD2\x81\x03\xA0\xC2\x13\xB9#\x14\x81K\x16\xF2\xFE\xC4\xC61\xE8\xDEm\x10\xBA\xFA\xC7\xDE\x90a\x08\x07\x90\x8D\x90\x86\x90a\x17\xA9V[`@Q\x80\x91\x03\x90\xA1P`\x01\x9A\x99PPPPPPPPPPV[`@Qc*\xDD\xA1\xC1`\xE0\x1B\x81R`\0\x90\x81\x900\x90c*\xDD\xA1\xC1\x90a\x08H\x90\x86\x90`\x04\x01a\x17\xDDV[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x89\x91\x90a\x188V[`@\x01Q\x93\x92PPPV[`@\x80Q\x825` \x80\x83\x01\x91\x90\x91R\x80\x84\x015\x82\x84\x01R\x83\x83\x015``\x80\x84\x01\x91\x90\x91R\x90\x93\x015`\x80\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`\x06T`\0\x90`\n\x90a\x08\xF2\x90Ca\x18\xF7V[a\x03x\x91\x90a\x19 V[a\t\x05\x82a\t\xAEV[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a\x1B\xA3`$\x919\x90P`\0\x84\x82`@Q` \x01a\t7\x92\x91\x90a\x19dV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80a\tT\x83a\n=V[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91Pa\t\x88\x81\x87a\t{\x8Aa\x0B\x10V[a\t\x83a\x0B\x8BV[a\x0C\\V[a\t\xA4W`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a\x1B\xC7\x839\x81Q\x91R\x91\x82`\x03\x81\x80\x85\x80\t\x85\t\x08\x83\x82\x83\t\x14\x81\x15\x83\x15\x17\x19\x84\x83\x10\x85\x85\x10\x16\x16\x16\x93PPP\x81a\n8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x82V[PPPV[`\0\x80`\0a\nK\x84a\r>V[\x90P`\0\x80Q` a\x1B\xC7\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\ntWa\nta\x19\nV[\x84\x82\t\x90P\x82\x80a\n\x87Wa\n\x87a\x19\nV[\x82\x82\x08\x90P`\0\x80a\n\x98\x83a\x0FpV[\x92P\x90P[\x80a\x0B\x01W\x84\x80a\n\xB0Wa\n\xB0a\x19\nV[`\x01\x87\x08\x95P\x84\x80a\n\xC4Wa\n\xC4a\x19\nV[\x86\x87\t\x92P\x84\x80a\n\xD7Wa\n\xD7a\x19\nV[\x86\x84\t\x92P\x84\x80a\n\xEAWa\n\xEAa\x19\nV[\x84\x84\x08\x92Pa\n\xF8\x83a\x0FpV[\x92P\x90Pa\n\x9DV[P\x93\x97\x93\x96P\x92\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x0B8WP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a\x1B\xC7\x839\x81Q\x91R\x84` \x01Qa\x0Bk\x91\x90a\x19\x81V[a\x0B\x83\x90`\0\x80Q` a\x1B\xC7\x839\x81Q\x91Ra\x18\xF7V[\x90R\x92\x91PPV[a\x0B\xB6`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R\x86Q`@\x82\x01R` \x87\x01Q``\x82\x01R`@\x87\x01Q`\x80\x82\x01R``\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R` \x85\x01Qa\x01 \x82\x01R`@\x85\x01Qa\x01@\x82\x01R``\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\r2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x05\x82V[P\x15\x15\x95\x94PPPPPV[`\0\x80a\rJ\x83a\x10kV[\x80Q\x90\x91P`0\x81\x14a\r_Wa\r_a\x19\x95V[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\ryWa\rya\x16qV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\r\xA3W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0E\x1EW\x83`\x01a\r\xBE\x83\x86a\x18\xF7V[a\r\xC8\x91\x90a\x18\xF7V[\x81Q\x81\x10a\r\xD8Wa\r\xD8a\x16[V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\r\xF5Wa\r\xF5a\x16[V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x0E\x16\x81a\x19\xABV[\x91PPa\r\xA9V[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x0E\xBAW\x83\x81a\x0E\\\x85\x88a\x18\xF7V[a\x0Ef\x91\x90a\x19\xC4V[\x81Q\x81\x10a\x0EvWa\x0Eva\x16[V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x0E\x96Wa\x0E\x96a\x16[V[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0E\xB2\x81a\x19\xABV[\x91PPa\x0EHV[P`\0a\x0E\xC6\x82a\x13\xDCV[\x90Pa\x01\0`\0\x80Q` a\x1B\xC7\x839\x81Q\x91R`\0a\x0E\xE6\x86\x89a\x18\xF7V[\x90P`\0[\x81\x81\x10\x15a\x0F`W`\0\x88`\x01a\x0F\x02\x84\x86a\x18\xF7V[a\x0F\x0C\x91\x90a\x18\xF7V[\x81Q\x81\x10a\x0F\x1CWa\x0F\x1Ca\x16[V[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x0F4Wa\x0F4a\x19\nV[\x85\x87\t\x95P\x83\x80a\x0FGWa\x0FGa\x19\nV[\x81\x87\x08\x95PP\x80\x80a\x0FX\x90a\x19\xABV[\x91PPa\x0E\xEBV[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a\x1B\xC7\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x86``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x93PP`\0Q\x93P\x82a\x102W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x82V[\x80`\x01\x85\x90\x1B\x11\x15a\x10KWa\x10H\x84\x82a\x18\xF7V[\x93P[\x80\x80a\x10YWa\x10Ya\x19\nV[\x84\x85\t\x91P\x85\x82\x14\x94PPPP\x91P\x91V[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x10\xAC\x92\x91\x90a\x19dV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x10\xD3\x92\x91\x90a\x19\xD7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x10\xF5\x91\x90a\x1A\x03V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x11\x1F\x90\x83\x90\x83\x90` \x01a\x1A\x1DV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x8FWa\x11\x8Fa\x16qV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x11\xB9W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x11\xD1\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x12FW\x81\x81\x81Q\x81\x10a\x12\0Wa\x12\0a\x16[V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x12\x1DWa\x12\x1Da\x16[V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x12>\x81a\x19\xABV[\x91PPa\x11\xE5V[P`\0\x84`@Q` \x01a\x12\\\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x12\xFAW`\0\x83\x82\x81Q\x81\x10a\x12\x97Wa\x12\x97a\x16[V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x12\xB4Wa\x12\xB4a\x16[V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x12\xD5\x92\x91\x90a\x1ABV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x98PP\x80\x80a\x12\xF2\x90a\x19\xABV[\x91PPa\x12{V[P\x86\x88\x87`@Q` \x01a\x13\x10\x93\x92\x91\x90a\x1AgV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x13>\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x13_\x8A`\xFF\x8D\x16a\x18\xF7V[\x81\x10\x15a\x13\xCBW\x82\x81\x81Q\x81\x10a\x13xWa\x13xa\x16[V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x13\x92\x83\x8Da\x19\xC4V[\x81Q\x81\x10a\x13\xA2Wa\x13\xA2a\x16[V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x13\xC3\x81a\x19\xABV[\x91PPa\x13RV[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a\x14GW\x83\x81\x81Q\x81\x10a\x13\xFCWa\x13\xFCa\x16[V[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x14\x14\x91\x90a\x1A\x9BV[a\x14\x1F\x90`\x02a\x1B\x96V[a\x14)\x91\x90a\x1A\x9BV[a\x143\x90\x83a\x19\xC4V[\x91P\x80a\x14?\x81a\x19\xABV[\x91PPa\x13\xE1V[P\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x14`W`\0\x80\xFD[P\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x14xW`\0\x80\xFD[a\x14\x82\x83\x83a\x14NV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x01Q`\x02\x81\x10a\x14\xCFWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x80` \x84\x01RP`@\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\xA0\x81\x01Q\x80Q`\xA0\x84\x01R` \x81\x01Q`\xC0\x84\x01RPPPV[`\xE0\x81\x01a\x15.\x82\x84a\x14\x9FV[\x92\x91PPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x15IW`\0\x80\xFD[PV[`\0\x80`\xA0\x83\x85\x03\x12\x15a\x15_W`\0\x80\xFD[a\x15i\x84\x84a\x14NV[\x91P`\x80\x83\x015a\x15y\x81a\x154V[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15a\x14`W`\0\x80\xFD[`\x02\x81\x10a\x15IW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15a\x15\xBDW`\0\x80\xFD[a\x15\xC7\x88\x88a\x14NV[\x95Pa\x15\xD6\x88`\x80\x89\x01a\x15\x84V[\x94P`\xC0\x87\x015a\x15\xE6\x81a\x154V[\x93P`\xE0\x87\x015a\x15\xF6\x81a\x15\x96V[\x92Pa\x16\x06\x88a\x01\0\x89\x01a\x15\x84V[\x91Pa\x01@\x87\x015a\x16\x17\x81a\x154V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x14GWa\x14Ga\x16%V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x16\xB7WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0`@\x82\x84\x03\x12\x15a\x16\xCFW`\0\x80\xFD[a\x16\xD7a\x16\x87V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x16\xFFW`\0\x80\xFD[a\x14\x82\x83\x83a\x16\xBDV[`\0`\x80\x82\x84\x03\x12\x15a\x17\x1BW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x17KWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[\x80Qa\x17\x87\x81a\x154V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x17\x9EW`\0\x80\xFD[\x81Qa\x14\x82\x81a\x154V[\x825\x81R` \x80\x84\x015\x90\x82\x01R`@\x80\x84\x015\x90\x82\x01R``\x80\x84\x015\x90\x82\x01Ra\x01`\x81\x01a\x14\x82`\x80\x83\x01\x84a\x14\x9FV[\x815\x81R` \x80\x83\x015\x90\x82\x01R`@\x80\x83\x015\x90\x82\x01R``\x80\x83\x015\x90\x82\x01R`\x80\x81\x01a\x15.V[`\0`@\x82\x84\x03\x12\x15a\x18\x1AW`\0\x80\xFD[a\x18\"a\x16\x87V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15a\x18JW`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x18zWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\x94W`\0\x80\xFD[\x81R` \x83\x01Qa\x18\xA4\x81a\x15\x96V[` \x82\x01R`@\x83\x01Qa\x18\xB7\x81a\x154V[`@\x82\x01Ra\x18\xC8``\x84\x01a\x17|V[``\x82\x01Ra\x18\xD9`\x80\x84\x01a\x17|V[`\x80\x82\x01Ra\x18\xEB\x84`\xA0\x85\x01a\x18\x08V[`\xA0\x82\x01R\x93\x92PPPV[\x81\x81\x03\x81\x81\x11\x15a\x15.Wa\x15.a\x16%V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x19/Wa\x19/a\x19\nV[P\x04\x90V[`\0\x81Q`\0[\x81\x81\x10\x15a\x19UW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x19;V[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x19ya\x19s\x83\x86a\x194V[\x84a\x194V[\x94\x93PPPPV[`\0\x82a\x19\x90Wa\x19\x90a\x19\nV[P\x06\x90V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x19\xBDWa\x19\xBDa\x16%V[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x15.Wa\x15.a\x16%V[`\0a\x19\xE3\x82\x85a\x194V[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a\x1A\x0F\x82\x84a\x194V[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a\x1A)\x82\x85a\x194V[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a\x1AN\x82\x85a\x194V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a\x1As\x82\x86a\x194V[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x15.Wa\x15.a\x16%V[`\x01\x81\x81[\x80\x85\x11\x15a\x1A\xEDW\x81`\0\x19\x04\x82\x11\x15a\x1A\xD3Wa\x1A\xD3a\x16%V[\x80\x85\x16\x15a\x1A\xE0W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x1A\xB7V[P\x92P\x92\x90PV[`\0\x82a\x1B\x04WP`\x01a\x15.V[\x81a\x1B\x11WP`\0a\x15.V[\x81`\x01\x81\x14a\x1B'W`\x02\x81\x14a\x1B1Wa\x1BMV[`\x01\x91PPa\x15.V[`\xFF\x84\x11\x15a\x1BBWa\x1BBa\x16%V[PP`\x01\x82\x1Ba\x15.V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1BpWP\x81\x81\na\x15.V[a\x1Bz\x83\x83a\x1A\xB2V[\x80`\0\x19\x04\x82\x11\x15a\x1B\x8EWa\x1B\x8Ea\x16%V[\x02\x93\x92PPPV[`\0a\x14\x82\x83\x83a\x1A\xF5V\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 P\xC1\x0E\xE2\x80`\x05\x9B\xD3#j<\xF18\x1FNk\xAD\xFE\xF5\x1C\x81o\x0BZR\x83_\x97\x111\xB7dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static STAKETABLE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80cH\x8B\xDA\xBC\x11a\0\x8CW\x80c\x8B\x0E\x9F?\x11a\0fW\x80c\x8B\x0E\x9F?\x14a\x01\xC4W\x80c\xBF\x82H\xDD\x14a\x01\xDFW\x80c\xC7,\xC7\x17\x14a\x01\xF7W\x80c\xDD.\xD3\xEC\x14a\x02\nW`\0\x80\xFD[\x80cH\x8B\xDA\xBC\x14a\x01VW\x80cJ\xA7\xC2\x7F\x14a\x01nW\x80cw\x1FoD\x14a\x01\x91W`\0\x80\xFD[\x80c\x0C$\xAF\x18\x14a\0\xD4W\x80c\x16\xFE\xFE\xD7\x14a\x01\x04W\x80c*\xDD\xA1\xC1\x14a\x01\x15W\x80c,S\x05\x84\x14a\x015W\x80c;\t\xC2g\x14a\x01=W\x80cC\x17\xD0\x0B\x14a\x01EW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x14fV[a\x02\x1DV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x05T`\x01`\x01`@\x1B\x03\x16a\0\xE7V[a\x01(a\x01#6`\x04a\x14fV[a\x02PV[`@Qa\0\xFB\x91\x90a\x15 V[a\0\xE7a\x03cV[a\0\xE7a\x03}V[`\x04T`@Q\x90\x81R` \x01a\0\xFBV[`\x03T`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xFBV[a\x01\x81a\x01|6`\x04a\x14fV[a\x03\xA5V[`@Q\x90\x15\x15\x81R` \x01a\0\xFBV[a\x01\xA4a\x01\x9F6`\x04a\x15LV[a\x03\xE2V[`@\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\0\xFBV[`\x01T`\x02T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xFBV[`\x05T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\0\xE7V[a\x01\x81a\x02\x056`\x04a\x15\xA3V[a\x04VV[a\0\xE7a\x02\x186`\x04a\x14fV[a\x08 V[`\0\x80a\x02)\x83a\x08\x94V[`\0\x90\x81R` \x81\x90R`@\x81 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16\x90U\x93\x92PPPV[a\x02\x95`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\xA0\x82\x01R\x90V[`\0a\x02\xA0\x83a\x08\x94V[`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xC0\x81\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x84R\x93\x94P\x91\x92\x90\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x02\xEBWa\x02\xEBa\x14\x89V[`\x01\x81\x11\x15a\x02\xFCWa\x02\xFCa\x14\x89V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x93\x92PPPV[`\0a\x03ma\x08\xDFV[a\x03x\x90`\x01a\x16;V[\x90P\x90V[`\x05T`\0\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81\x03a\x03\x9FWP`\0\x90V[P`\x01\x90V[`\0\x80a\x03\xB1\x83a\x08\x94V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x90\x81\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16\x90U\x93\x92PPPV[`\0\x80`\0a\x03\xF0\x85a\x08\x94V[`\0\x81\x81R` \x81\x90R`@\x90 \x80T\x91\x92P\x85\x91`\x15\x90a\x04#\x90\x84\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x16;V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\0\x80\x92P\x92PP\x92P\x92\x90PV[`\0\x80a\x04b\x88a\x08\x94V[`\0\x81\x81R` \x81\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x04\xAEWa\x04\xAEa\x14\x89V[`\x01\x81\x11\x15a\x04\xBFWa\x04\xBFa\x14\x89V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x05\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FThe node has already been regist`D\x82\x01Rc\x19\\\x99Y`\xE2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R`\0\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90Pa\x05\xD2\x81a\x05\xBE6\x89\x90\x03\x89\x01\x89a\x16\xEDV[a\x05\xCD6\x8E\x90\x03\x8E\x01\x8Ea\x17\tV[a\x08\xFCV[`\x000`\x01`\x01`\xA0\x1B\x03\x16c,S\x05\x84`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x066\x91\x90a\x17\x8CV[\x90P\x85`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\x06\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid next registration epoch.`D\x82\x01R`d\x01a\x05\x82V[3\x83R`\x01`\x01`@\x1B\x03\x89\x16`@\x84\x01R` \x83\x01\x88`\x01\x81\x11\x15a\x06\xC2Wa\x06\xC2a\x14\x89V[\x90\x81`\x01\x81\x11\x15a\x06\xD5Wa\x06\xD5a\x14\x89V[\x90RPa\x06\xE76\x8B\x90\x03\x8B\x01\x8Ba\x16\xEDV[`\xA0\x84\x01R`\x01`\x01`@\x1B\x03\x81\x16``\x84\x01R`\0\x84\x81R` \x81\x81R`@\x90\x91 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x83U\x92\x86\x01Q\x86\x93\x90\x91\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\x07UWa\x07Ua\x14\x89V[\x02\x17\x90UP`@\x82\x81\x01Q\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17\x83U``\x84\x01Q`\x01\x84\x01\x80T`\x80\x87\x01Q\x92\x84\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x17`\x01`@\x1B\x92\x90\x93\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90U`\xA0\x90\x92\x01Q\x80Q`\x02\x83\x01U` \x01Q`\x03\x90\x91\x01UQ\x7F\xD1\xFE\xEEYM\x06\x1A\xD2\x81\x03\xA0\xC2\x13\xB9#\x14\x81K\x16\xF2\xFE\xC4\xC61\xE8\xDEm\x10\xBA\xFA\xC7\xDE\x90a\x08\x07\x90\x8D\x90\x86\x90a\x17\xA9V[`@Q\x80\x91\x03\x90\xA1P`\x01\x9A\x99PPPPPPPPPPV[`@Qc*\xDD\xA1\xC1`\xE0\x1B\x81R`\0\x90\x81\x900\x90c*\xDD\xA1\xC1\x90a\x08H\x90\x86\x90`\x04\x01a\x17\xDDV[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x89\x91\x90a\x188V[`@\x01Q\x93\x92PPPV[`@\x80Q\x825` \x80\x83\x01\x91\x90\x91R\x80\x84\x015\x82\x84\x01R\x83\x83\x015``\x80\x84\x01\x91\x90\x91R\x90\x93\x015`\x80\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`\x06T`\0\x90`\n\x90a\x08\xF2\x90Ca\x18\xF7V[a\x03x\x91\x90a\x19 V[a\t\x05\x82a\t\xAEV[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a\x1B\xA3`$\x919\x90P`\0\x84\x82`@Q` \x01a\t7\x92\x91\x90a\x19dV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80a\tT\x83a\n=V[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91Pa\t\x88\x81\x87a\t{\x8Aa\x0B\x10V[a\t\x83a\x0B\x8BV[a\x0C\\V[a\t\xA4W`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a\x1B\xC7\x839\x81Q\x91R\x91\x82`\x03\x81\x80\x85\x80\t\x85\t\x08\x83\x82\x83\t\x14\x81\x15\x83\x15\x17\x19\x84\x83\x10\x85\x85\x10\x16\x16\x16\x93PPP\x81a\n8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x82V[PPPV[`\0\x80`\0a\nK\x84a\r>V[\x90P`\0\x80Q` a\x1B\xC7\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\ntWa\nta\x19\nV[\x84\x82\t\x90P\x82\x80a\n\x87Wa\n\x87a\x19\nV[\x82\x82\x08\x90P`\0\x80a\n\x98\x83a\x0FpV[\x92P\x90P[\x80a\x0B\x01W\x84\x80a\n\xB0Wa\n\xB0a\x19\nV[`\x01\x87\x08\x95P\x84\x80a\n\xC4Wa\n\xC4a\x19\nV[\x86\x87\t\x92P\x84\x80a\n\xD7Wa\n\xD7a\x19\nV[\x86\x84\t\x92P\x84\x80a\n\xEAWa\n\xEAa\x19\nV[\x84\x84\x08\x92Pa\n\xF8\x83a\x0FpV[\x92P\x90Pa\n\x9DV[P\x93\x97\x93\x96P\x92\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x0B8WP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a\x1B\xC7\x839\x81Q\x91R\x84` \x01Qa\x0Bk\x91\x90a\x19\x81V[a\x0B\x83\x90`\0\x80Q` a\x1B\xC7\x839\x81Q\x91Ra\x18\xF7V[\x90R\x92\x91PPV[a\x0B\xB6`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R\x86Q`@\x82\x01R` \x87\x01Q``\x82\x01R`@\x87\x01Q`\x80\x82\x01R``\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R` \x85\x01Qa\x01 \x82\x01R`@\x85\x01Qa\x01@\x82\x01R``\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\r2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x05\x82V[P\x15\x15\x95\x94PPPPPV[`\0\x80a\rJ\x83a\x10kV[\x80Q\x90\x91P`0\x81\x14a\r_Wa\r_a\x19\x95V[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\ryWa\rya\x16qV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\r\xA3W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0E\x1EW\x83`\x01a\r\xBE\x83\x86a\x18\xF7V[a\r\xC8\x91\x90a\x18\xF7V[\x81Q\x81\x10a\r\xD8Wa\r\xD8a\x16[V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\r\xF5Wa\r\xF5a\x16[V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x0E\x16\x81a\x19\xABV[\x91PPa\r\xA9V[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x0E\xBAW\x83\x81a\x0E\\\x85\x88a\x18\xF7V[a\x0Ef\x91\x90a\x19\xC4V[\x81Q\x81\x10a\x0EvWa\x0Eva\x16[V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x0E\x96Wa\x0E\x96a\x16[V[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0E\xB2\x81a\x19\xABV[\x91PPa\x0EHV[P`\0a\x0E\xC6\x82a\x13\xDCV[\x90Pa\x01\0`\0\x80Q` a\x1B\xC7\x839\x81Q\x91R`\0a\x0E\xE6\x86\x89a\x18\xF7V[\x90P`\0[\x81\x81\x10\x15a\x0F`W`\0\x88`\x01a\x0F\x02\x84\x86a\x18\xF7V[a\x0F\x0C\x91\x90a\x18\xF7V[\x81Q\x81\x10a\x0F\x1CWa\x0F\x1Ca\x16[V[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x0F4Wa\x0F4a\x19\nV[\x85\x87\t\x95P\x83\x80a\x0FGWa\x0FGa\x19\nV[\x81\x87\x08\x95PP\x80\x80a\x0FX\x90a\x19\xABV[\x91PPa\x0E\xEBV[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a\x1B\xC7\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x86``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x93PP`\0Q\x93P\x82a\x102W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x82V[\x80`\x01\x85\x90\x1B\x11\x15a\x10KWa\x10H\x84\x82a\x18\xF7V[\x93P[\x80\x80a\x10YWa\x10Ya\x19\nV[\x84\x85\t\x91P\x85\x82\x14\x94PPPP\x91P\x91V[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x10\xAC\x92\x91\x90a\x19dV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x10\xD3\x92\x91\x90a\x19\xD7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x10\xF5\x91\x90a\x1A\x03V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x11\x1F\x90\x83\x90\x83\x90` \x01a\x1A\x1DV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x8FWa\x11\x8Fa\x16qV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x11\xB9W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x11\xD1\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x12FW\x81\x81\x81Q\x81\x10a\x12\0Wa\x12\0a\x16[V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x12\x1DWa\x12\x1Da\x16[V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x12>\x81a\x19\xABV[\x91PPa\x11\xE5V[P`\0\x84`@Q` \x01a\x12\\\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x12\xFAW`\0\x83\x82\x81Q\x81\x10a\x12\x97Wa\x12\x97a\x16[V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x12\xB4Wa\x12\xB4a\x16[V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x12\xD5\x92\x91\x90a\x1ABV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x98PP\x80\x80a\x12\xF2\x90a\x19\xABV[\x91PPa\x12{V[P\x86\x88\x87`@Q` \x01a\x13\x10\x93\x92\x91\x90a\x1AgV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x13>\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x13_\x8A`\xFF\x8D\x16a\x18\xF7V[\x81\x10\x15a\x13\xCBW\x82\x81\x81Q\x81\x10a\x13xWa\x13xa\x16[V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x13\x92\x83\x8Da\x19\xC4V[\x81Q\x81\x10a\x13\xA2Wa\x13\xA2a\x16[V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x13\xC3\x81a\x19\xABV[\x91PPa\x13RV[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a\x14GW\x83\x81\x81Q\x81\x10a\x13\xFCWa\x13\xFCa\x16[V[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x14\x14\x91\x90a\x1A\x9BV[a\x14\x1F\x90`\x02a\x1B\x96V[a\x14)\x91\x90a\x1A\x9BV[a\x143\x90\x83a\x19\xC4V[\x91P\x80a\x14?\x81a\x19\xABV[\x91PPa\x13\xE1V[P\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x14`W`\0\x80\xFD[P\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x14xW`\0\x80\xFD[a\x14\x82\x83\x83a\x14NV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x01Q`\x02\x81\x10a\x14\xCFWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x80` \x84\x01RP`@\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\xA0\x81\x01Q\x80Q`\xA0\x84\x01R` \x81\x01Q`\xC0\x84\x01RPPPV[`\xE0\x81\x01a\x15.\x82\x84a\x14\x9FV[\x92\x91PPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x15IW`\0\x80\xFD[PV[`\0\x80`\xA0\x83\x85\x03\x12\x15a\x15_W`\0\x80\xFD[a\x15i\x84\x84a\x14NV[\x91P`\x80\x83\x015a\x15y\x81a\x154V[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15a\x14`W`\0\x80\xFD[`\x02\x81\x10a\x15IW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15a\x15\xBDW`\0\x80\xFD[a\x15\xC7\x88\x88a\x14NV[\x95Pa\x15\xD6\x88`\x80\x89\x01a\x15\x84V[\x94P`\xC0\x87\x015a\x15\xE6\x81a\x154V[\x93P`\xE0\x87\x015a\x15\xF6\x81a\x15\x96V[\x92Pa\x16\x06\x88a\x01\0\x89\x01a\x15\x84V[\x91Pa\x01@\x87\x015a\x16\x17\x81a\x154V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x14GWa\x14Ga\x16%V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x16\xB7WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0`@\x82\x84\x03\x12\x15a\x16\xCFW`\0\x80\xFD[a\x16\xD7a\x16\x87V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x16\xFFW`\0\x80\xFD[a\x14\x82\x83\x83a\x16\xBDV[`\0`\x80\x82\x84\x03\x12\x15a\x17\x1BW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x17KWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[\x80Qa\x17\x87\x81a\x154V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x17\x9EW`\0\x80\xFD[\x81Qa\x14\x82\x81a\x154V[\x825\x81R` \x80\x84\x015\x90\x82\x01R`@\x80\x84\x015\x90\x82\x01R``\x80\x84\x015\x90\x82\x01Ra\x01`\x81\x01a\x14\x82`\x80\x83\x01\x84a\x14\x9FV[\x815\x81R` \x80\x83\x015\x90\x82\x01R`@\x80\x83\x015\x90\x82\x01R``\x80\x83\x015\x90\x82\x01R`\x80\x81\x01a\x15.V[`\0`@\x82\x84\x03\x12\x15a\x18\x1AW`\0\x80\xFD[a\x18\"a\x16\x87V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15a\x18JW`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x18zWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\x94W`\0\x80\xFD[\x81R` \x83\x01Qa\x18\xA4\x81a\x15\x96V[` \x82\x01R`@\x83\x01Qa\x18\xB7\x81a\x154V[`@\x82\x01Ra\x18\xC8``\x84\x01a\x17|V[``\x82\x01Ra\x18\xD9`\x80\x84\x01a\x17|V[`\x80\x82\x01Ra\x18\xEB\x84`\xA0\x85\x01a\x18\x08V[`\xA0\x82\x01R\x93\x92PPPV[\x81\x81\x03\x81\x81\x11\x15a\x15.Wa\x15.a\x16%V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x19/Wa\x19/a\x19\nV[P\x04\x90V[`\0\x81Q`\0[\x81\x81\x10\x15a\x19UW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x19;V[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x19ya\x19s\x83\x86a\x194V[\x84a\x194V[\x94\x93PPPPV[`\0\x82a\x19\x90Wa\x19\x90a\x19\nV[P\x06\x90V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x19\xBDWa\x19\xBDa\x16%V[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x15.Wa\x15.a\x16%V[`\0a\x19\xE3\x82\x85a\x194V[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a\x1A\x0F\x82\x84a\x194V[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a\x1A)\x82\x85a\x194V[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a\x1AN\x82\x85a\x194V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a\x1As\x82\x86a\x194V[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x15.Wa\x15.a\x16%V[`\x01\x81\x81[\x80\x85\x11\x15a\x1A\xEDW\x81`\0\x19\x04\x82\x11\x15a\x1A\xD3Wa\x1A\xD3a\x16%V[\x80\x85\x16\x15a\x1A\xE0W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x1A\xB7V[P\x92P\x92\x90PV[`\0\x82a\x1B\x04WP`\x01a\x15.V[\x81a\x1B\x11WP`\0a\x15.V[\x81`\x01\x81\x14a\x1B'W`\x02\x81\x14a\x1B1Wa\x1BMV[`\x01\x91PPa\x15.V[`\xFF\x84\x11\x15a\x1BBWa\x1BBa\x16%V[PP`\x01\x82\x1Ba\x15.V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1BpWP\x81\x81\na\x15.V[a\x1Bz\x83\x83a\x1A\xB2V[\x80`\0\x19\x04\x82\x11\x15a\x1B\x8EWa\x1B\x8Ea\x16%V[\x02\x93\x92PPPV[`\0a\x14\x82\x83\x83a\x1A\xF5V\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 P\xC1\x0E\xE2\x80`\x05\x9B\xD3#j<\xF18\x1FNk\xAD\xFE\xF5\x1C\x81o\x0BZR\x83_\x97\x111\xB7dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static STAKETABLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct StakeTable<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StakeTable<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StakeTable<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StakeTable<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StakeTable<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StakeTable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StakeTable<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                STAKETABLE_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                STAKETABLE_ABI.clone(),
                STAKETABLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `deposit` (0x771f6f44) function
        pub fn deposit(
            &self,
            bls_vk: G2Point,
            amount: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u64)> {
            self.0
                .method_hash([119, 31, 111, 68], (bls_vk, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lookupNode` (0x2adda1c1) function
        pub fn lookup_node(
            &self,
            bls_vk: G2Point,
        ) -> ::ethers::contract::builders::ContractCall<M, Node> {
            self.0
                .method_hash([42, 221, 161, 193], (bls_vk,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lookupStake` (0xdd2ed3ec) function
        pub fn lookup_stake(
            &self,
            bls_vk: G2Point,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([221, 46, 211, 236], (bls_vk,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nextExitEpoch` (0x3b09c267) function
        pub fn next_exit_epoch(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([59, 9, 194, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nextRegistrationEpoch` (0x2c530584) function
        pub fn next_registration_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([44, 83, 5, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numPendingExit` (0xbf8248dd) function
        pub fn num_pending_exit(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([191, 130, 72, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numPendingRegistrations` (0x16fefed7) function
        pub fn num_pending_registrations(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([22, 254, 254, 215], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `register` (0xc72cc717) function
        pub fn register(
            &self,
            bls_vk: G2Point,
            schnorr_vk: EdOnBN254Point,
            amount: u64,
            stake_type: u8,
            bls_sig: G1Point,
            valid_until_epoch: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [199, 44, 199, 23],
                    (
                        bls_vk,
                        schnorr_vk,
                        amount,
                        stake_type,
                        bls_sig,
                        valid_until_epoch,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestExit` (0x4aa7c27f) function
        pub fn request_exit(
            &self,
            bls_vk: G2Point,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([74, 167, 194, 127], (bls_vk,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalKeys` (0x488bdabc) function
        pub fn total_keys(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([72, 139, 218, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalStake` (0x8b0e9f3f) function
        pub fn total_stake(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([139, 14, 159, 63], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalVotingStake` (0x4317d00b) function
        pub fn total_voting_stake(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([67, 23, 208, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawFunds` (0x0c24af18) function
        pub fn withdraw_funds(
            &self,
            bls_vk: G2Point,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([12, 36, 175, 24], (bls_vk,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Register` event
        pub fn register_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RegisterFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RegisterFilter> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for StakeTable<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BLSSigVerificationFailed` with signature `BLSSigVerificationFailed()` and selector `0x0ced3e50`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BLSSigVerificationFailed", abi = "BLSSigVerificationFailed()")]
    pub struct BLSSigVerificationFailed;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Register",
        abi = "Register((uint256,uint256,uint256,uint256),(address,uint8,uint64,uint64,uint64,(uint256,uint256)))"
    )]
    pub struct RegisterFilter(pub G2Point, pub Node);
    ///Container type for all input parameters for the `deposit` function with signature `deposit((uint256,uint256,uint256,uint256),uint64)` and selector `0x771f6f44`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "deposit",
        abi = "deposit((uint256,uint256,uint256,uint256),uint64)"
    )]
    pub struct DepositCall {
        pub bls_vk: G2Point,
        pub amount: u64,
    }
    ///Container type for all input parameters for the `lookupNode` function with signature `lookupNode((uint256,uint256,uint256,uint256))` and selector `0x2adda1c1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "lookupNode",
        abi = "lookupNode((uint256,uint256,uint256,uint256))"
    )]
    pub struct LookupNodeCall {
        pub bls_vk: G2Point,
    }
    ///Container type for all input parameters for the `lookupStake` function with signature `lookupStake((uint256,uint256,uint256,uint256))` and selector `0xdd2ed3ec`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "lookupStake",
        abi = "lookupStake((uint256,uint256,uint256,uint256))"
    )]
    pub struct LookupStakeCall {
        pub bls_vk: G2Point,
    }
    ///Container type for all input parameters for the `nextExitEpoch` function with signature `nextExitEpoch()` and selector `0x3b09c267`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "nextExitEpoch", abi = "nextExitEpoch()")]
    pub struct NextExitEpochCall;
    ///Container type for all input parameters for the `nextRegistrationEpoch` function with signature `nextRegistrationEpoch()` and selector `0x2c530584`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "nextRegistrationEpoch", abi = "nextRegistrationEpoch()")]
    pub struct NextRegistrationEpochCall;
    ///Container type for all input parameters for the `numPendingExit` function with signature `numPendingExit()` and selector `0xbf8248dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "numPendingExit", abi = "numPendingExit()")]
    pub struct NumPendingExitCall;
    ///Container type for all input parameters for the `numPendingRegistrations` function with signature `numPendingRegistrations()` and selector `0x16fefed7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "numPendingRegistrations", abi = "numPendingRegistrations()")]
    pub struct NumPendingRegistrationsCall;
    ///Container type for all input parameters for the `register` function with signature `register((uint256,uint256,uint256,uint256),(uint256,uint256),uint64,uint8,(uint256,uint256),uint64)` and selector `0xc72cc717`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "register",
        abi = "register((uint256,uint256,uint256,uint256),(uint256,uint256),uint64,uint8,(uint256,uint256),uint64)"
    )]
    pub struct RegisterCall {
        pub bls_vk: G2Point,
        pub schnorr_vk: EdOnBN254Point,
        pub amount: u64,
        pub stake_type: u8,
        pub bls_sig: G1Point,
        pub valid_until_epoch: u64,
    }
    ///Container type for all input parameters for the `requestExit` function with signature `requestExit((uint256,uint256,uint256,uint256))` and selector `0x4aa7c27f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "requestExit",
        abi = "requestExit((uint256,uint256,uint256,uint256))"
    )]
    pub struct RequestExitCall {
        pub bls_vk: G2Point,
    }
    ///Container type for all input parameters for the `totalKeys` function with signature `totalKeys()` and selector `0x488bdabc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalKeys", abi = "totalKeys()")]
    pub struct TotalKeysCall;
    ///Container type for all input parameters for the `totalStake` function with signature `totalStake()` and selector `0x8b0e9f3f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalStake", abi = "totalStake()")]
    pub struct TotalStakeCall;
    ///Container type for all input parameters for the `totalVotingStake` function with signature `totalVotingStake()` and selector `0x4317d00b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalVotingStake", abi = "totalVotingStake()")]
    pub struct TotalVotingStakeCall;
    ///Container type for all input parameters for the `withdrawFunds` function with signature `withdrawFunds((uint256,uint256,uint256,uint256))` and selector `0x0c24af18`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "withdrawFunds",
        abi = "withdrawFunds((uint256,uint256,uint256,uint256))"
    )]
    pub struct WithdrawFundsCall {
        pub bls_vk: G2Point,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum StakeTableCalls {
        Deposit(DepositCall),
        LookupNode(LookupNodeCall),
        LookupStake(LookupStakeCall),
        NextExitEpoch(NextExitEpochCall),
        NextRegistrationEpoch(NextRegistrationEpochCall),
        NumPendingExit(NumPendingExitCall),
        NumPendingRegistrations(NumPendingRegistrationsCall),
        Register(RegisterCall),
        RequestExit(RequestExitCall),
        TotalKeys(TotalKeysCall),
        TotalStake(TotalStakeCall),
        TotalVotingStake(TotalVotingStakeCall),
        WithdrawFunds(WithdrawFundsCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakeTableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <LookupNodeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LookupNode(decoded));
            }
            if let Ok(decoded) = <LookupStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LookupStake(decoded));
            }
            if let Ok(decoded) = <NextExitEpochCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NextExitEpoch(decoded));
            }
            if let Ok(decoded) =
                <NextRegistrationEpochCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NextRegistrationEpoch(decoded));
            }
            if let Ok(decoded) =
                <NumPendingExitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NumPendingExit(decoded));
            }
            if let Ok(decoded) =
                <NumPendingRegistrationsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NumPendingRegistrations(decoded));
            }
            if let Ok(decoded) = <RegisterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Register(decoded));
            }
            if let Ok(decoded) = <RequestExitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RequestExit(decoded));
            }
            if let Ok(decoded) = <TotalKeysCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalKeys(decoded));
            }
            if let Ok(decoded) = <TotalStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalStake(decoded));
            }
            if let Ok(decoded) =
                <TotalVotingStakeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalVotingStake(decoded));
            }
            if let Ok(decoded) = <WithdrawFundsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawFunds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakeTableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LookupNode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LookupStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NextExitEpoch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NextRegistrationEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NumPendingExit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NumPendingRegistrations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Register(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestExit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalKeys(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalVotingStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawFunds(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for StakeTableCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::LookupNode(element) => ::core::fmt::Display::fmt(element, f),
                Self::LookupStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextExitEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextRegistrationEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumPendingExit(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumPendingRegistrations(element) => ::core::fmt::Display::fmt(element, f),
                Self::Register(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestExit(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalKeys(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalVotingStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFunds(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositCall> for StakeTableCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<LookupNodeCall> for StakeTableCalls {
        fn from(value: LookupNodeCall) -> Self {
            Self::LookupNode(value)
        }
    }
    impl ::core::convert::From<LookupStakeCall> for StakeTableCalls {
        fn from(value: LookupStakeCall) -> Self {
            Self::LookupStake(value)
        }
    }
    impl ::core::convert::From<NextExitEpochCall> for StakeTableCalls {
        fn from(value: NextExitEpochCall) -> Self {
            Self::NextExitEpoch(value)
        }
    }
    impl ::core::convert::From<NextRegistrationEpochCall> for StakeTableCalls {
        fn from(value: NextRegistrationEpochCall) -> Self {
            Self::NextRegistrationEpoch(value)
        }
    }
    impl ::core::convert::From<NumPendingExitCall> for StakeTableCalls {
        fn from(value: NumPendingExitCall) -> Self {
            Self::NumPendingExit(value)
        }
    }
    impl ::core::convert::From<NumPendingRegistrationsCall> for StakeTableCalls {
        fn from(value: NumPendingRegistrationsCall) -> Self {
            Self::NumPendingRegistrations(value)
        }
    }
    impl ::core::convert::From<RegisterCall> for StakeTableCalls {
        fn from(value: RegisterCall) -> Self {
            Self::Register(value)
        }
    }
    impl ::core::convert::From<RequestExitCall> for StakeTableCalls {
        fn from(value: RequestExitCall) -> Self {
            Self::RequestExit(value)
        }
    }
    impl ::core::convert::From<TotalKeysCall> for StakeTableCalls {
        fn from(value: TotalKeysCall) -> Self {
            Self::TotalKeys(value)
        }
    }
    impl ::core::convert::From<TotalStakeCall> for StakeTableCalls {
        fn from(value: TotalStakeCall) -> Self {
            Self::TotalStake(value)
        }
    }
    impl ::core::convert::From<TotalVotingStakeCall> for StakeTableCalls {
        fn from(value: TotalVotingStakeCall) -> Self {
            Self::TotalVotingStake(value)
        }
    }
    impl ::core::convert::From<WithdrawFundsCall> for StakeTableCalls {
        fn from(value: WithdrawFundsCall) -> Self {
            Self::WithdrawFunds(value)
        }
    }
    ///Container type for all return fields from the `deposit` function with signature `deposit((uint256,uint256,uint256,uint256),uint64)` and selector `0x771f6f44`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositReturn(pub u64, pub u64);
    ///Container type for all return fields from the `lookupNode` function with signature `lookupNode((uint256,uint256,uint256,uint256))` and selector `0x2adda1c1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LookupNodeReturn(pub Node);
    ///Container type for all return fields from the `lookupStake` function with signature `lookupStake((uint256,uint256,uint256,uint256))` and selector `0xdd2ed3ec`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LookupStakeReturn(pub u64);
    ///Container type for all return fields from the `nextExitEpoch` function with signature `nextExitEpoch()` and selector `0x3b09c267`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NextExitEpochReturn(pub u64);
    ///Container type for all return fields from the `nextRegistrationEpoch` function with signature `nextRegistrationEpoch()` and selector `0x2c530584`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NextRegistrationEpochReturn(pub u64);
    ///Container type for all return fields from the `numPendingExit` function with signature `numPendingExit()` and selector `0xbf8248dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NumPendingExitReturn(pub u64);
    ///Container type for all return fields from the `numPendingRegistrations` function with signature `numPendingRegistrations()` and selector `0x16fefed7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NumPendingRegistrationsReturn(pub u64);
    ///Container type for all return fields from the `register` function with signature `register((uint256,uint256,uint256,uint256),(uint256,uint256),uint64,uint8,(uint256,uint256),uint64)` and selector `0xc72cc717`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RegisterReturn(pub bool);
    ///Container type for all return fields from the `requestExit` function with signature `requestExit((uint256,uint256,uint256,uint256))` and selector `0x4aa7c27f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RequestExitReturn(pub bool);
    ///Container type for all return fields from the `totalKeys` function with signature `totalKeys()` and selector `0x488bdabc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalKeysReturn(pub u32);
    ///Container type for all return fields from the `totalStake` function with signature `totalStake()` and selector `0x8b0e9f3f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalStakeReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `totalVotingStake` function with signature `totalVotingStake()` and selector `0x4317d00b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalVotingStakeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `withdrawFunds` function with signature `withdrawFunds((uint256,uint256,uint256,uint256))` and selector `0x0c24af18`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WithdrawFundsReturn(pub u64);
}
