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
            constructor: ::core::option::Option::None,
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
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
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
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
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
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
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
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
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
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
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
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STAKETABLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\n\x9D\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80cH\x8B\xDA\xBC\x11a\0\x8CW\x80c\xBF\x82H\xDD\x11a\0fW\x80c\xBF\x82H\xDD\x14a\x01\xACW\x80c\xCD\xF7x\x8F\x14a\x01\xC4W\x80c\xD4\xB8?E\x14a\x01\xD7W\x80c\xF7\xBF\xB0\x1C\x14a\x01\xEAW`\0\x80\xFD[\x80cH\x8B\xDA\xBC\x14a\x01YW\x80cR\xF9/\xC4\x14a\x01qW\x80c\x8B\x0E\x9F?\x14a\x01\x91W`\0\x80\xFD[\x80c\to\xBA\xE3\x14a\0\xD4W\x80c\x16-N|\x14a\0\xFCW\x80c\x16\xFE\xFE\xD7\x14a\x01'W\x80c,S\x05\x84\x14a\x018W\x80c;\t\xC2g\x14a\x01@W\x80cC\x17\xD0\x0B\x14a\x01HW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x06\xE7V[a\x02\x1DV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x0Fa\x01\n6`\x04a\x07\xB3V[a\x03\xB6V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF3V[`\x05T`\x01`\x01`@\x1B\x03\x16a\x01\x0FV[a\x01\x0Fa\x03\xE9V[a\x01\x0Fa\x04\nV[`\x04T`@Q\x90\x81R` \x01a\0\xF3V[`\x03T`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xF3V[a\x01\x84a\x01\x7F6`\x04a\x07\xB3V[a\x04,V[`@Qa\0\xF3\x91\x90a\x07\xECV[`\x01T`\x02T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xF3V[`\x05T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x01\x0FV[a\x01\x0Fa\x01\xD26`\x04a\x07\xB3V[a\x05?V[a\0\xE7a\x01\xE56`\x04a\x07\xB3V[a\x05\xB9V[a\x01\xFDa\x01\xF86`\x04a\x08\x89V[a\x05\xF6V[`@\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\0\xF3V[`\0\x80`\0\x90P`\0`@Q\x80`\xC0\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01\x81\x11\x15a\x02RWa\x02Ra\x07\xD6V[\x81R`\x01`\x01`@\x1B\x03\x80\x8B\x16` \x83\x01R\x84\x81\x16`@\x83\x01R\x86\x16``\x82\x01R`\x80\x01a\x02\x856\x8C\x90\x03\x8C\x01\x8Ca\x08\xF7V[\x90R\x90P\x85\x85`\0\x81a\x02\x9AWa\x02\x9Aa\t)V[\x90\x91\x015`\x01`\x01`\xF8\x1B\x03\x19\x16`\0\x03\x90Pa\x02\xBCW`\0\x92PPPa\x03\xABV[`\0a\x02\xC7\x8Ba\x06jV[`\0\x81\x81R` \x81\x81R`@\x90\x91 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x83U\x92\x86\x01Q\x93\x94P\x85\x93\x91\x92\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\x03#Wa\x03#a\x07\xD6V[\x02\x17\x90UP`@\x82\x01Q\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17\x82U``\x83\x01Q`\x01\x80\x84\x01\x80T`\x80\x87\x01Q\x93\x85\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x17`\x01`@\x1B\x93\x90\x94\x16\x92\x90\x92\x02\x92\x90\x92\x17\x90U`\xA0\x90\x92\x01Q\x80Q`\x02\x83\x01U` \x01Q`\x03\x90\x91\x01U\x93PPPP[\x97\x96PPPPPPPV[`\0\x80a\x03\xC2\x83a\x06jV[`\0\x90\x81R` \x81\x90R`@\x81 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16\x90U\x93\x92PPPV[`\x05T`\0\x90`\x01`\x01`@\x1B\x03\x16\x81\x03a\x04\x04WP`\0\x90V[P`\x01\x90V[`\x05T`\0\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81\x03a\x04\x04WP`\0\x90V[a\x04q`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\xA0\x82\x01R\x90V[`\0a\x04|\x83a\x06jV[`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xC0\x81\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x84R\x93\x94P\x91\x92\x90\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x04\xC7Wa\x04\xC7a\x07\xD6V[`\x01\x81\x11\x15a\x04\xD8Wa\x04\xD8a\x07\xD6V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x93\x92PPPV[`@Qc\x14\xBEK\xF1`\xE2\x1B\x81R\x815`\x04\x82\x01R` \x82\x015`$\x82\x01R`\0\x90\x81\x900\x90cR\xF9/\xC4\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xAE\x91\x90a\tzV[`@\x01Q\x93\x92PPPV[`\0\x80a\x05\xC5\x83a\x06jV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x90\x81\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16\x90U\x93\x92PPPV[`\0\x80`\0a\x06\x04\x85a\x06jV[`\0\x81\x81R` \x81\x90R`@\x90 \x80T\x91\x92P\x85\x91`\x15\x90a\x067\x90\x84\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\n9V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\0\x80\x92P\x92PP\x92P\x92\x90PV[`@\x80Q\x825` \x80\x83\x01\x91\x90\x91R\x92\x83\x015\x81\x83\x01R\x81Q\x80\x82\x03\x83\x01\x81R``\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`\0`@\x82\x84\x03\x12\x15a\x06\xACW`\0\x80\xFD[P\x91\x90PV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x06\xC7W`\0\x80\xFD[PV[\x805a\x06\xD5\x81a\x06\xB2V[\x91\x90PV[`\x02\x81\x10a\x06\xC7W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0a\x01\0\x88\x8A\x03\x12\x15a\x07\x03W`\0\x80\xFD[a\x07\r\x89\x89a\x06\x9AV[\x96Pa\x07\x1C\x89`@\x8A\x01a\x06\x9AV[\x95P`\x80\x88\x015a\x07,\x81a\x06\xB2V[\x94P`\xA0\x88\x015a\x07<\x81a\x06\xDAV[\x93P`\xC0\x88\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x07XW`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x07lW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07{W`\0\x80\xFD[\x8B` \x82\x85\x01\x01\x11\x15a\x07\x8DW`\0\x80\xFD[` \x83\x01\x95P\x80\x94PPPPa\x07\xA5`\xE0\x89\x01a\x06\xCAV[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0`@\x82\x84\x03\x12\x15a\x07\xC5W`\0\x80\xFD[a\x07\xCF\x83\x83a\x06\x9AV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x82\x01Q`\xE0\x82\x01\x90`\x02\x81\x10a\x08!WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x80` \x84\x01RP`@\x83\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x86\x01Q\x16``\x85\x01RPP`\x80\x83\x01Qa\x08e`\x80\x84\x01\x82`\x01`\x01`@\x1B\x03\x16\x90RV[P`\xA0\x83\x01Qa\x08\x82`\xA0\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15a\x08\x9CW`\0\x80\xFD[a\x08\xA6\x84\x84a\x06\x9AV[\x91P`@\x83\x015a\x08\xB6\x81a\x06\xB2V[\x80\x91PP\x92P\x92\x90PV[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x08\xF1WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0`@\x82\x84\x03\x12\x15a\t\tW`\0\x80\xFD[a\t\x11a\x08\xC1V[\x825\x81R` \x83\x015` \x82\x01R\x80\x91PP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80Qa\x06\xD5\x81a\x06\xB2V[`\0`@\x82\x84\x03\x12\x15a\t\\W`\0\x80\xFD[a\tda\x08\xC1V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15a\t\x8CW`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\t\xBCWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xD6W`\0\x80\xFD[\x81R` \x83\x01Qa\t\xE6\x81a\x06\xDAV[` \x82\x01R`@\x83\x01Qa\t\xF9\x81a\x06\xB2V[`@\x82\x01Ra\n\n``\x84\x01a\t?V[``\x82\x01Ra\n\x1B`\x80\x84\x01a\t?V[`\x80\x82\x01Ra\n-\x84`\xA0\x85\x01a\tJV[`\xA0\x82\x01R\x93\x92PPPV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x08\x82WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 h8\xE4\x1C\x15(>?;\x8CAa\xB6\t\xE70/\xA7\xAAF3\xC9m\xC3\x1A\xE6\x9AeW\xCCoGdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static STAKETABLE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80cH\x8B\xDA\xBC\x11a\0\x8CW\x80c\xBF\x82H\xDD\x11a\0fW\x80c\xBF\x82H\xDD\x14a\x01\xACW\x80c\xCD\xF7x\x8F\x14a\x01\xC4W\x80c\xD4\xB8?E\x14a\x01\xD7W\x80c\xF7\xBF\xB0\x1C\x14a\x01\xEAW`\0\x80\xFD[\x80cH\x8B\xDA\xBC\x14a\x01YW\x80cR\xF9/\xC4\x14a\x01qW\x80c\x8B\x0E\x9F?\x14a\x01\x91W`\0\x80\xFD[\x80c\to\xBA\xE3\x14a\0\xD4W\x80c\x16-N|\x14a\0\xFCW\x80c\x16\xFE\xFE\xD7\x14a\x01'W\x80c,S\x05\x84\x14a\x018W\x80c;\t\xC2g\x14a\x01@W\x80cC\x17\xD0\x0B\x14a\x01HW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x06\xE7V[a\x02\x1DV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x0Fa\x01\n6`\x04a\x07\xB3V[a\x03\xB6V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF3V[`\x05T`\x01`\x01`@\x1B\x03\x16a\x01\x0FV[a\x01\x0Fa\x03\xE9V[a\x01\x0Fa\x04\nV[`\x04T`@Q\x90\x81R` \x01a\0\xF3V[`\x03T`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xF3V[a\x01\x84a\x01\x7F6`\x04a\x07\xB3V[a\x04,V[`@Qa\0\xF3\x91\x90a\x07\xECV[`\x01T`\x02T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xF3V[`\x05T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x01\x0FV[a\x01\x0Fa\x01\xD26`\x04a\x07\xB3V[a\x05?V[a\0\xE7a\x01\xE56`\x04a\x07\xB3V[a\x05\xB9V[a\x01\xFDa\x01\xF86`\x04a\x08\x89V[a\x05\xF6V[`@\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\0\xF3V[`\0\x80`\0\x90P`\0`@Q\x80`\xC0\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01\x81\x11\x15a\x02RWa\x02Ra\x07\xD6V[\x81R`\x01`\x01`@\x1B\x03\x80\x8B\x16` \x83\x01R\x84\x81\x16`@\x83\x01R\x86\x16``\x82\x01R`\x80\x01a\x02\x856\x8C\x90\x03\x8C\x01\x8Ca\x08\xF7V[\x90R\x90P\x85\x85`\0\x81a\x02\x9AWa\x02\x9Aa\t)V[\x90\x91\x015`\x01`\x01`\xF8\x1B\x03\x19\x16`\0\x03\x90Pa\x02\xBCW`\0\x92PPPa\x03\xABV[`\0a\x02\xC7\x8Ba\x06jV[`\0\x81\x81R` \x81\x81R`@\x90\x91 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x83U\x92\x86\x01Q\x93\x94P\x85\x93\x91\x92\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\x03#Wa\x03#a\x07\xD6V[\x02\x17\x90UP`@\x82\x01Q\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17\x82U``\x83\x01Q`\x01\x80\x84\x01\x80T`\x80\x87\x01Q\x93\x85\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x17`\x01`@\x1B\x93\x90\x94\x16\x92\x90\x92\x02\x92\x90\x92\x17\x90U`\xA0\x90\x92\x01Q\x80Q`\x02\x83\x01U` \x01Q`\x03\x90\x91\x01U\x93PPPP[\x97\x96PPPPPPPV[`\0\x80a\x03\xC2\x83a\x06jV[`\0\x90\x81R` \x81\x90R`@\x81 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16\x90U\x93\x92PPPV[`\x05T`\0\x90`\x01`\x01`@\x1B\x03\x16\x81\x03a\x04\x04WP`\0\x90V[P`\x01\x90V[`\x05T`\0\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81\x03a\x04\x04WP`\0\x90V[a\x04q`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\xA0\x82\x01R\x90V[`\0a\x04|\x83a\x06jV[`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xC0\x81\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x84R\x93\x94P\x91\x92\x90\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x04\xC7Wa\x04\xC7a\x07\xD6V[`\x01\x81\x11\x15a\x04\xD8Wa\x04\xD8a\x07\xD6V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x93\x92PPPV[`@Qc\x14\xBEK\xF1`\xE2\x1B\x81R\x815`\x04\x82\x01R` \x82\x015`$\x82\x01R`\0\x90\x81\x900\x90cR\xF9/\xC4\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xAE\x91\x90a\tzV[`@\x01Q\x93\x92PPPV[`\0\x80a\x05\xC5\x83a\x06jV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x90\x81\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16\x90U\x93\x92PPPV[`\0\x80`\0a\x06\x04\x85a\x06jV[`\0\x81\x81R` \x81\x90R`@\x90 \x80T\x91\x92P\x85\x91`\x15\x90a\x067\x90\x84\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\n9V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\0\x80\x92P\x92PP\x92P\x92\x90PV[`@\x80Q\x825` \x80\x83\x01\x91\x90\x91R\x92\x83\x015\x81\x83\x01R\x81Q\x80\x82\x03\x83\x01\x81R``\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`\0`@\x82\x84\x03\x12\x15a\x06\xACW`\0\x80\xFD[P\x91\x90PV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x06\xC7W`\0\x80\xFD[PV[\x805a\x06\xD5\x81a\x06\xB2V[\x91\x90PV[`\x02\x81\x10a\x06\xC7W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0a\x01\0\x88\x8A\x03\x12\x15a\x07\x03W`\0\x80\xFD[a\x07\r\x89\x89a\x06\x9AV[\x96Pa\x07\x1C\x89`@\x8A\x01a\x06\x9AV[\x95P`\x80\x88\x015a\x07,\x81a\x06\xB2V[\x94P`\xA0\x88\x015a\x07<\x81a\x06\xDAV[\x93P`\xC0\x88\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x07XW`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x07lW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07{W`\0\x80\xFD[\x8B` \x82\x85\x01\x01\x11\x15a\x07\x8DW`\0\x80\xFD[` \x83\x01\x95P\x80\x94PPPPa\x07\xA5`\xE0\x89\x01a\x06\xCAV[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0`@\x82\x84\x03\x12\x15a\x07\xC5W`\0\x80\xFD[a\x07\xCF\x83\x83a\x06\x9AV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x82\x01Q`\xE0\x82\x01\x90`\x02\x81\x10a\x08!WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x80` \x84\x01RP`@\x83\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x86\x01Q\x16``\x85\x01RPP`\x80\x83\x01Qa\x08e`\x80\x84\x01\x82`\x01`\x01`@\x1B\x03\x16\x90RV[P`\xA0\x83\x01Qa\x08\x82`\xA0\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15a\x08\x9CW`\0\x80\xFD[a\x08\xA6\x84\x84a\x06\x9AV[\x91P`@\x83\x015a\x08\xB6\x81a\x06\xB2V[\x80\x91PP\x92P\x92\x90PV[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x08\xF1WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0`@\x82\x84\x03\x12\x15a\t\tW`\0\x80\xFD[a\t\x11a\x08\xC1V[\x825\x81R` \x83\x015` \x82\x01R\x80\x91PP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80Qa\x06\xD5\x81a\x06\xB2V[`\0`@\x82\x84\x03\x12\x15a\t\\W`\0\x80\xFD[a\tda\x08\xC1V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15a\t\x8CW`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\t\xBCWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xD6W`\0\x80\xFD[\x81R` \x83\x01Qa\t\xE6\x81a\x06\xDAV[` \x82\x01R`@\x83\x01Qa\t\xF9\x81a\x06\xB2V[`@\x82\x01Ra\n\n``\x84\x01a\t?V[``\x82\x01Ra\n\x1B`\x80\x84\x01a\t?V[`\x80\x82\x01Ra\n-\x84`\xA0\x85\x01a\tJV[`\xA0\x82\x01R\x93\x92PPPV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x08\x82WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 h8\xE4\x1C\x15(>?;\x8CAa\xB6\t\xE70/\xA7\xAAF3\xC9m\xC3\x1A\xE6\x9AeW\xCCoGdsolcC\0\x08\x14\x003";
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
        ///Calls the contract's `deposit` (0xf7bfb01c) function
        pub fn deposit(
            &self,
            bls_vk: G1Point,
            amount: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u64)> {
            self.0
                .method_hash([247, 191, 176, 28], (bls_vk, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lookupNode` (0x52f92fc4) function
        pub fn lookup_node(
            &self,
            bls_vk: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, Node> {
            self.0
                .method_hash([82, 249, 47, 196], (bls_vk,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lookupStake` (0xcdf7788f) function
        pub fn lookup_stake(
            &self,
            bls_vk: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([205, 247, 120, 143], (bls_vk,))
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
        ///Calls the contract's `register` (0x096fbae3) function
        pub fn register(
            &self,
            bls_vk: G1Point,
            schnorr_vk: EdOnBN254Point,
            amount: u64,
            stake_type: u8,
            bls_sig: ::ethers::core::types::Bytes,
            valid_until_epoch: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [9, 111, 186, 227],
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
        ///Calls the contract's `requestExit` (0xd4b83f45) function
        pub fn request_exit(
            &self,
            bls_vk: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([212, 184, 63, 69], (bls_vk,))
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
        ///Calls the contract's `withdrawFunds` (0x162d4e7c) function
        pub fn withdraw_funds(
            &self,
            bls_vk: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([22, 45, 78, 124], (bls_vk,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for StakeTable<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit((uint256,uint256),uint64)` and selector `0xf7bfb01c`
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
    #[ethcall(name = "deposit", abi = "deposit((uint256,uint256),uint64)")]
    pub struct DepositCall {
        pub bls_vk: G1Point,
        pub amount: u64,
    }
    ///Container type for all input parameters for the `lookupNode` function with signature `lookupNode((uint256,uint256))` and selector `0x52f92fc4`
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
    #[ethcall(name = "lookupNode", abi = "lookupNode((uint256,uint256))")]
    pub struct LookupNodeCall {
        pub bls_vk: G1Point,
    }
    ///Container type for all input parameters for the `lookupStake` function with signature `lookupStake((uint256,uint256))` and selector `0xcdf7788f`
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
    #[ethcall(name = "lookupStake", abi = "lookupStake((uint256,uint256))")]
    pub struct LookupStakeCall {
        pub bls_vk: G1Point,
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
    ///Container type for all input parameters for the `register` function with signature `register((uint256,uint256),(uint256,uint256),uint64,uint8,bytes,uint64)` and selector `0x096fbae3`
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
        abi = "register((uint256,uint256),(uint256,uint256),uint64,uint8,bytes,uint64)"
    )]
    pub struct RegisterCall {
        pub bls_vk: G1Point,
        pub schnorr_vk: EdOnBN254Point,
        pub amount: u64,
        pub stake_type: u8,
        pub bls_sig: ::ethers::core::types::Bytes,
        pub valid_until_epoch: u64,
    }
    ///Container type for all input parameters for the `requestExit` function with signature `requestExit((uint256,uint256))` and selector `0xd4b83f45`
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
    #[ethcall(name = "requestExit", abi = "requestExit((uint256,uint256))")]
    pub struct RequestExitCall {
        pub bls_vk: G1Point,
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
    ///Container type for all input parameters for the `withdrawFunds` function with signature `withdrawFunds((uint256,uint256))` and selector `0x162d4e7c`
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
    #[ethcall(name = "withdrawFunds", abi = "withdrawFunds((uint256,uint256))")]
    pub struct WithdrawFundsCall {
        pub bls_vk: G1Point,
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
    ///Container type for all return fields from the `deposit` function with signature `deposit((uint256,uint256),uint64)` and selector `0xf7bfb01c`
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
    ///Container type for all return fields from the `lookupNode` function with signature `lookupNode((uint256,uint256))` and selector `0x52f92fc4`
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
    ///Container type for all return fields from the `lookupStake` function with signature `lookupStake((uint256,uint256))` and selector `0xcdf7788f`
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
    ///Container type for all return fields from the `register` function with signature `register((uint256,uint256),(uint256,uint256),uint64,uint8,bytes,uint64)` and selector `0x096fbae3`
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
    ///Container type for all return fields from the `requestExit` function with signature `requestExit((uint256,uint256))` and selector `0xd4b83f45`
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
    ///Container type for all return fields from the `withdrawFunds` function with signature `withdrawFunds((uint256,uint256))` and selector `0x162d4e7c`
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
