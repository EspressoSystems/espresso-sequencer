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
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
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
                ::std::borrow::ToOwned::to_owned("Registered"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("Registered"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                    ],
                    anonymous: false,
                },],
            )]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BLSSigVerificationFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BLSSigVerificationFailed",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RestakingNotImplemented"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RestakingNotImplemented",),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STAKETABLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x1Dz8\x03\x80a\x1Dz\x839\x81\x01`@\x81\x90Ra\0/\x91a\0XV[C`\x06U`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x88V[`\0` \x82\x84\x03\x12\x15a\0jW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x81W`\0\x80\xFD[\x93\x92PPPV[a\x1C\xE3\x80a\0\x97`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80cH\x8B\xDA\xBC\x11a\0\x8CW\x80c\x8B\x0E\x9F?\x11a\0fW\x80c\x8B\x0E\x9F?\x14a\x01\xC4W\x80c\xBF\x82H\xDD\x14a\x01\xDFW\x80c\xC7,\xC7\x17\x14a\x01\xF7W\x80c\xDD.\xD3\xEC\x14a\x02\nW`\0\x80\xFD[\x80cH\x8B\xDA\xBC\x14a\x01VW\x80cJ\xA7\xC2\x7F\x14a\x01nW\x80cw\x1FoD\x14a\x01\x91W`\0\x80\xFD[\x80c\x0C$\xAF\x18\x14a\0\xD4W\x80c\x16\xFE\xFE\xD7\x14a\x01\x04W\x80c*\xDD\xA1\xC1\x14a\x01\x15W\x80c,S\x05\x84\x14a\x015W\x80c;\t\xC2g\x14a\x01=W\x80cC\x17\xD0\x0B\x14a\x01EW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x16>V[a\x02\x1DV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x05T`\x01`\x01`@\x1B\x03\x16a\0\xE7V[a\x01(a\x01#6`\x04a\x16>V[a\x02PV[`@Qa\0\xFB\x91\x90a\x16\x99V[a\0\xE7a\x03eV[a\0\xE7a\x03\x7FV[`\x04T`@Q\x90\x81R` \x01a\0\xFBV[`\x03T`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xFBV[a\x01\x81a\x01|6`\x04a\x16>V[a\x03\xA7V[`@Q\x90\x15\x15\x81R` \x01a\0\xFBV[a\x01\xA4a\x01\x9F6`\x04a\x17\"V[a\x03\xE4V[`@\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\0\xFBV[`\x01T`\x02T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xFBV[`\x05T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\0\xE7V[a\x01\x81a\x02\x056`\x04a\x17\x97V[a\x04XV[a\0\xE7a\x02\x186`\x04a\x16>V[a\x08\x8AV[`\0\x80a\x02)\x83a\t\x18V[`\0\x90\x81R` \x81\x90R`@\x81 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16\x90U\x93\x92PPPV[a\x02\x95`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\xA0\x82\x01R\x90V[`\0\x80a\x02\xA1\x84a\t\x18V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x91\x92\x90\x91\x90\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x02\xEEWa\x02\xEEa\x16aV[`\x01\x81\x11\x15a\x02\xFFWa\x02\xFFa\x16aV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x92\x91PPV[`\0a\x03oa\ttV[a\x03z\x90`\x01a\x18/V[\x90P\x90V[`\x05T`\0\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81\x03a\x03\xA1WP`\0\x90V[P`\x01\x90V[`\0\x80a\x03\xB3\x83a\t\x18V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x90\x81\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16\x90U\x93\x92PPPV[`\0\x80`\0a\x03\xF2\x85a\t\x18V[`\0\x81\x81R` \x81\x90R`@\x90 \x80T\x91\x92P\x85\x91`\x15\x90a\x04%\x90\x84\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x18/V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\0\x80\x92P\x92PP\x92P\x92\x90PV[`\0\x80a\x04d\x88a\t\x18V[`\0\x81\x81R` \x81\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x04\xB0Wa\x04\xB0a\x16aV[`\x01\x81\x11\x15a\x04\xC1Wa\x04\xC1a\x16aV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x05\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FThe node has already been regist`D\x82\x01Rc\x19\\\x99Y`\xE2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x86`\x01\x81\x11\x15a\x05\xA1Wa\x05\xA1a\x16aV[\x14a\x05\xBEW`@Qb\x11\xD7\xFB`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x05\xE8\x81\x87\x8Ca\t\x91V[`\x000`\x01`\x01`\xA0\x1B\x03\x16c,S\x05\x84`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06L\x91\x90a\x18uV[\x90P\x85`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\x06\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid next registration epoch.`D\x82\x01R`d\x01a\x05\x84V[3\x83R`\x01`\x01`@\x1B\x03\x89\x16`@\x84\x01R` \x83\x01\x88`\x01\x81\x11\x15a\x06\xD8Wa\x06\xD8a\x16aV[\x90\x81`\x01\x81\x11\x15a\x06\xEBWa\x06\xEBa\x16aV[\x90RP`\xA0\x83\x01\x8A\x90R`\x01`\x01`@\x1B\x03\x81\x16``\x84\x01R`\0\x84\x81R` \x81\x81R`@\x90\x91 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x83U\x92\x86\x01Q\x86\x93\x90\x91\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\x07^Wa\x07^a\x16aV[\x02\x17\x90UP`@\x82\x01Q\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17\x82U``\x83\x01Q`\x01\x83\x01\x80T`\x80\x86\x01Q\x92\x84\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x17`\x01`@\x1B\x92\x90\x93\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90U`\xA0\x90\x91\x01Q\x80Q`\x02\x83\x01U` \x01Q`\x03\x90\x91\x01U`\0\x88`\x01\x81\x11\x15a\x07\xF3Wa\x07\xF3a\x16aV[\x03a\x08<W`\x01`\x01`@\x1B\x03\x89\x16`\x01`\0\x01`\0\x82\x82Ta\x08\x16\x91\x90a\x18\x92V[\x90\x91UPP`\x07Ta\x08<\x90`\x01`\x01`\xA0\x1B\x03\x1630`\x01`\x01`@\x1B\x03\x8D\x16a\nCV[\x7F\x8C\"\xDC2\xA9\xEE;6$\xF3\xF9\xF4\xF9\xBE\x14\x8Dxb^\xCD\x89<5\xE6\x9A\xB2\xFF\x0E\x0B\xFF\xC00\x84\x82\x8A\x8C`@Qa\x08q\x94\x93\x92\x91\x90a\x18\xABV[`@Q\x80\x91\x03\x90\xA1P`\x01\x9A\x99PPPPPPPPPPV[`@\x80Qc*\xDD\xA1\xC1`\xE0\x1B\x81R\x82Q`\x04\x82\x01R` \x83\x01Q`$\x82\x01R\x90\x82\x01Q`D\x82\x01R``\x82\x01Q`d\x82\x01R`\0\x90\x81\x900\x90c*\xDD\xA1\xC1\x90`\x84\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\r\x91\x90a\x19\x12V[`@\x01Q\x93\x92PPPV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\tW\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x06T`\0\x90`\n\x90a\t\x87\x90Ca\x19\xD1V[a\x03z\x91\x90a\x19\xFAV[a\t\x9A\x82a\n\xDFV[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a\x1Cj`$\x919\x90P`\0\x84\x82`@Q` \x01a\t\xCC\x92\x91\x90a\x1A>V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80a\t\xE9\x83a\x0BnV[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91Pa\n\x1D\x81\x87a\n\x10\x8Aa\x0CAV[a\n\x18a\x0C\xBCV[a\r\x8DV[a\n9W`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\n\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x05\x84V[PPPPPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a\x1C\x8E\x839\x81Q\x91R\x91\x82`\x03\x81\x80\x85\x80\t\x85\t\x08\x83\x82\x83\t\x14\x81\x15\x83\x15\x17\x19\x84\x83\x10\x85\x85\x10\x16\x16\x16\x93PPP\x81a\x0BiW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x84V[PPPV[`\0\x80`\0a\x0B|\x84a\x0EoV[\x90P`\0\x80Q` a\x1C\x8E\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\x0B\xA5Wa\x0B\xA5a\x19\xE4V[\x84\x82\t\x90P\x82\x80a\x0B\xB8Wa\x0B\xB8a\x19\xE4V[\x82\x82\x08\x90P`\0\x80a\x0B\xC9\x83a\x10\xA1V[\x92P\x90P[\x80a\x0C2W\x84\x80a\x0B\xE1Wa\x0B\xE1a\x19\xE4V[`\x01\x87\x08\x95P\x84\x80a\x0B\xF5Wa\x0B\xF5a\x19\xE4V[\x86\x87\t\x92P\x84\x80a\x0C\x08Wa\x0C\x08a\x19\xE4V[\x86\x84\t\x92P\x84\x80a\x0C\x1BWa\x0C\x1Ba\x19\xE4V[\x84\x84\x08\x92Pa\x0C)\x83a\x10\xA1V[\x92P\x90Pa\x0B\xCEV[P\x93\x97\x93\x96P\x92\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x0CiWP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a\x1C\x8E\x839\x81Q\x91R\x84` \x01Qa\x0C\x9C\x91\x90a\x1A[V[a\x0C\xB4\x90`\0\x80Q` a\x1C\x8E\x839\x81Q\x91Ra\x19\xD1V[\x90R\x92\x91PPV[a\x0C\xE7`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R\x86Q`@\x82\x01R` \x87\x01Q``\x82\x01R`@\x87\x01Q`\x80\x82\x01R``\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R` \x85\x01Qa\x01 \x82\x01R`@\x85\x01Qa\x01@\x82\x01R``\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x0EcW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x05\x84V[P\x15\x15\x95\x94PPPPPV[`\0\x80a\x0E{\x83a\x11\x9CV[\x80Q\x90\x91P`0\x81\x14a\x0E\x90Wa\x0E\x90a\x1AoV[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xAAWa\x0E\xAAa\x15\x7FV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\xD4W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0FOW\x83`\x01a\x0E\xEF\x83\x86a\x19\xD1V[a\x0E\xF9\x91\x90a\x19\xD1V[\x81Q\x81\x10a\x0F\tWa\x0F\ta\x18OV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0F&Wa\x0F&a\x18OV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x0FG\x81a\x1A\x85V[\x91PPa\x0E\xDAV[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x0F\xEBW\x83\x81a\x0F\x8D\x85\x88a\x19\xD1V[a\x0F\x97\x91\x90a\x18\x92V[\x81Q\x81\x10a\x0F\xA7Wa\x0F\xA7a\x18OV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x0F\xC7Wa\x0F\xC7a\x18OV[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0F\xE3\x81a\x1A\x85V[\x91PPa\x0FyV[P`\0a\x0F\xF7\x82a\x15\rV[\x90Pa\x01\0`\0\x80Q` a\x1C\x8E\x839\x81Q\x91R`\0a\x10\x17\x86\x89a\x19\xD1V[\x90P`\0[\x81\x81\x10\x15a\x10\x91W`\0\x88`\x01a\x103\x84\x86a\x19\xD1V[a\x10=\x91\x90a\x19\xD1V[\x81Q\x81\x10a\x10MWa\x10Ma\x18OV[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x10eWa\x10ea\x19\xE4V[\x85\x87\t\x95P\x83\x80a\x10xWa\x10xa\x19\xE4V[\x81\x87\x08\x95PP\x80\x80a\x10\x89\x90a\x1A\x85V[\x91PPa\x10\x1CV[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a\x1C\x8E\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x86``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x93PP`\0Q\x93P\x82a\x11cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x84V[\x80`\x01\x85\x90\x1B\x11\x15a\x11|Wa\x11y\x84\x82a\x19\xD1V[\x93P[\x80\x80a\x11\x8AWa\x11\x8Aa\x19\xE4V[\x84\x85\t\x91P\x85\x82\x14\x94PPPP\x91P\x91V[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x11\xDD\x92\x91\x90a\x1A>V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x12\x04\x92\x91\x90a\x1A\x9EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x12&\x91\x90a\x1A\xCAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x12P\x90\x83\x90\x83\x90` \x01a\x1A\xE4V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xC0Wa\x12\xC0a\x15\x7FV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12\xEAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x13\x02\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x13wW\x81\x81\x81Q\x81\x10a\x131Wa\x131a\x18OV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x13NWa\x13Na\x18OV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x13o\x81a\x1A\x85V[\x91PPa\x13\x16V[P`\0\x84`@Q` \x01a\x13\x8D\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x14+W`\0\x83\x82\x81Q\x81\x10a\x13\xC8Wa\x13\xC8a\x18OV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x13\xE5Wa\x13\xE5a\x18OV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x14\x06\x92\x91\x90a\x1B\tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x98PP\x80\x80a\x14#\x90a\x1A\x85V[\x91PPa\x13\xACV[P\x86\x88\x87`@Q` \x01a\x14A\x93\x92\x91\x90a\x1B.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x14o\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x14\x90\x8A`\xFF\x8D\x16a\x19\xD1V[\x81\x10\x15a\x14\xFCW\x82\x81\x81Q\x81\x10a\x14\xA9Wa\x14\xA9a\x18OV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x14\xC3\x83\x8Da\x18\x92V[\x81Q\x81\x10a\x14\xD3Wa\x14\xD3a\x18OV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x14\xF4\x81a\x1A\x85V[\x91PPa\x14\x83V[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a\x15xW\x83\x81\x81Q\x81\x10a\x15-Wa\x15-a\x18OV[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x15E\x91\x90a\x1BbV[a\x15P\x90`\x02a\x1C]V[a\x15Z\x91\x90a\x1BbV[a\x15d\x90\x83a\x18\x92V[\x91P\x80a\x15p\x81a\x1A\x85V[\x91PPa\x15\x12V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x15\xC5WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0`\x80\x82\x84\x03\x12\x15a\x15\xDDW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x16\rWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x16PW`\0\x80\xFD[a\x16Z\x83\x83a\x15\xCBV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a\x16\x95WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Q`\xE0\x83\x01\x91a\x16\xBD\x90\x84\x01\x82a\x16wV[P`@\x83\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x86\x01Q\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\xA0\x83\x01Q\x80Q`\xA0\x84\x01R` \x81\x01Q`\xC0\x84\x01RP\x92\x91PPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x17\x1FW`\0\x80\xFD[PV[`\0\x80`\xA0\x83\x85\x03\x12\x15a\x175W`\0\x80\xFD[a\x17?\x84\x84a\x15\xCBV[\x91P`\x80\x83\x015a\x17O\x81a\x17\nV[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15a\x17lW`\0\x80\xFD[a\x17ta\x15\x95V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\x02\x81\x10a\x17\x1FW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15a\x17\xB1W`\0\x80\xFD[a\x17\xBB\x88\x88a\x15\xCBV[\x95Pa\x17\xCA\x88`\x80\x89\x01a\x17ZV[\x94P`\xC0\x87\x015a\x17\xDA\x81a\x17\nV[\x93P`\xE0\x87\x015a\x17\xEA\x81a\x17\x8AV[\x92Pa\x17\xFA\x88a\x01\0\x89\x01a\x17ZV[\x91Pa\x01@\x87\x015a\x18\x0B\x81a\x17\nV[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x15xWa\x15xa\x18\x19V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80Qa\x18p\x81a\x17\nV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x18\x87W`\0\x80\xFD[\x81Qa\x16Z\x81a\x17\nV[\x80\x82\x01\x80\x82\x11\x15a\x18\xA5Wa\x18\xA5a\x18\x19V[\x92\x91PPV[\x84\x81R`\x01`\x01`@\x1B\x03\x84\x81\x16` \x83\x01R`\x80\x82\x01\x90a\x18\xD0`@\x84\x01\x86a\x16wV[\x80\x84\x16``\x84\x01RP\x95\x94PPPPPV[`\0`@\x82\x84\x03\x12\x15a\x18\xF4W`\0\x80\xFD[a\x18\xFCa\x15\x95V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15a\x19$W`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x19TWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x19nW`\0\x80\xFD[\x81R` \x83\x01Qa\x19~\x81a\x17\x8AV[` \x82\x01R`@\x83\x01Qa\x19\x91\x81a\x17\nV[`@\x82\x01Ra\x19\xA2``\x84\x01a\x18eV[``\x82\x01Ra\x19\xB3`\x80\x84\x01a\x18eV[`\x80\x82\x01Ra\x19\xC5\x84`\xA0\x85\x01a\x18\xE2V[`\xA0\x82\x01R\x93\x92PPPV[\x81\x81\x03\x81\x81\x11\x15a\x18\xA5Wa\x18\xA5a\x18\x19V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1A\tWa\x1A\ta\x19\xE4V[P\x04\x90V[`\0\x81Q`\0[\x81\x81\x10\x15a\x1A/W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x1A\x15V[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x1ASa\x1AM\x83\x86a\x1A\x0EV[\x84a\x1A\x0EV[\x94\x93PPPPV[`\0\x82a\x1AjWa\x1Aja\x19\xE4V[P\x06\x90V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1A\x97Wa\x1A\x97a\x18\x19V[P`\x01\x01\x90V[`\0a\x1A\xAA\x82\x85a\x1A\x0EV[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a\x1A\xD6\x82\x84a\x1A\x0EV[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a\x1A\xF0\x82\x85a\x1A\x0EV[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a\x1B\x15\x82\x85a\x1A\x0EV[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a\x1B:\x82\x86a\x1A\x0EV[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x18\xA5Wa\x18\xA5a\x18\x19V[`\x01\x81\x81[\x80\x85\x11\x15a\x1B\xB4W\x81`\0\x19\x04\x82\x11\x15a\x1B\x9AWa\x1B\x9Aa\x18\x19V[\x80\x85\x16\x15a\x1B\xA7W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x1B~V[P\x92P\x92\x90PV[`\0\x82a\x1B\xCBWP`\x01a\x18\xA5V[\x81a\x1B\xD8WP`\0a\x18\xA5V[\x81`\x01\x81\x14a\x1B\xEEW`\x02\x81\x14a\x1B\xF8Wa\x1C\x14V[`\x01\x91PPa\x18\xA5V[`\xFF\x84\x11\x15a\x1C\tWa\x1C\ta\x18\x19V[PP`\x01\x82\x1Ba\x18\xA5V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1C7WP\x81\x81\na\x18\xA5V[a\x1CA\x83\x83a\x1ByV[\x80`\0\x19\x04\x82\x11\x15a\x1CUWa\x1CUa\x18\x19V[\x02\x93\x92PPPV[`\0a\x16Z\x83\x83a\x1B\xBCV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 <\\\xE7\xF1Y\xAA\x16\x91vV|\xF2\x11n1\x95I\xE8\x03\xA5Jsmy6\xDA\x8E7\xA7\xA2s\xD5dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static STAKETABLE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80cH\x8B\xDA\xBC\x11a\0\x8CW\x80c\x8B\x0E\x9F?\x11a\0fW\x80c\x8B\x0E\x9F?\x14a\x01\xC4W\x80c\xBF\x82H\xDD\x14a\x01\xDFW\x80c\xC7,\xC7\x17\x14a\x01\xF7W\x80c\xDD.\xD3\xEC\x14a\x02\nW`\0\x80\xFD[\x80cH\x8B\xDA\xBC\x14a\x01VW\x80cJ\xA7\xC2\x7F\x14a\x01nW\x80cw\x1FoD\x14a\x01\x91W`\0\x80\xFD[\x80c\x0C$\xAF\x18\x14a\0\xD4W\x80c\x16\xFE\xFE\xD7\x14a\x01\x04W\x80c*\xDD\xA1\xC1\x14a\x01\x15W\x80c,S\x05\x84\x14a\x015W\x80c;\t\xC2g\x14a\x01=W\x80cC\x17\xD0\x0B\x14a\x01EW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x16>V[a\x02\x1DV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x05T`\x01`\x01`@\x1B\x03\x16a\0\xE7V[a\x01(a\x01#6`\x04a\x16>V[a\x02PV[`@Qa\0\xFB\x91\x90a\x16\x99V[a\0\xE7a\x03eV[a\0\xE7a\x03\x7FV[`\x04T`@Q\x90\x81R` \x01a\0\xFBV[`\x03T`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xFBV[a\x01\x81a\x01|6`\x04a\x16>V[a\x03\xA7V[`@Q\x90\x15\x15\x81R` \x01a\0\xFBV[a\x01\xA4a\x01\x9F6`\x04a\x17\"V[a\x03\xE4V[`@\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\0\xFBV[`\x01T`\x02T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xFBV[`\x05T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\0\xE7V[a\x01\x81a\x02\x056`\x04a\x17\x97V[a\x04XV[a\0\xE7a\x02\x186`\x04a\x16>V[a\x08\x8AV[`\0\x80a\x02)\x83a\t\x18V[`\0\x90\x81R` \x81\x90R`@\x81 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16\x90U\x93\x92PPPV[a\x02\x95`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\xA0\x82\x01R\x90V[`\0\x80a\x02\xA1\x84a\t\x18V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x91\x92\x90\x91\x90\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x02\xEEWa\x02\xEEa\x16aV[`\x01\x81\x11\x15a\x02\xFFWa\x02\xFFa\x16aV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x92\x91PPV[`\0a\x03oa\ttV[a\x03z\x90`\x01a\x18/V[\x90P\x90V[`\x05T`\0\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81\x03a\x03\xA1WP`\0\x90V[P`\x01\x90V[`\0\x80a\x03\xB3\x83a\t\x18V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x90\x81\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16\x90U\x93\x92PPPV[`\0\x80`\0a\x03\xF2\x85a\t\x18V[`\0\x81\x81R` \x81\x90R`@\x90 \x80T\x91\x92P\x85\x91`\x15\x90a\x04%\x90\x84\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x18/V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\0\x80\x92P\x92PP\x92P\x92\x90PV[`\0\x80a\x04d\x88a\t\x18V[`\0\x81\x81R` \x81\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x04\xB0Wa\x04\xB0a\x16aV[`\x01\x81\x11\x15a\x04\xC1Wa\x04\xC1a\x16aV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x05\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FThe node has already been regist`D\x82\x01Rc\x19\\\x99Y`\xE2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x86`\x01\x81\x11\x15a\x05\xA1Wa\x05\xA1a\x16aV[\x14a\x05\xBEW`@Qb\x11\xD7\xFB`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x05\xE8\x81\x87\x8Ca\t\x91V[`\x000`\x01`\x01`\xA0\x1B\x03\x16c,S\x05\x84`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06L\x91\x90a\x18uV[\x90P\x85`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\x06\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid next registration epoch.`D\x82\x01R`d\x01a\x05\x84V[3\x83R`\x01`\x01`@\x1B\x03\x89\x16`@\x84\x01R` \x83\x01\x88`\x01\x81\x11\x15a\x06\xD8Wa\x06\xD8a\x16aV[\x90\x81`\x01\x81\x11\x15a\x06\xEBWa\x06\xEBa\x16aV[\x90RP`\xA0\x83\x01\x8A\x90R`\x01`\x01`@\x1B\x03\x81\x16``\x84\x01R`\0\x84\x81R` \x81\x81R`@\x90\x91 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x83U\x92\x86\x01Q\x86\x93\x90\x91\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\x07^Wa\x07^a\x16aV[\x02\x17\x90UP`@\x82\x01Q\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17\x82U``\x83\x01Q`\x01\x83\x01\x80T`\x80\x86\x01Q\x92\x84\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x17`\x01`@\x1B\x92\x90\x93\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90U`\xA0\x90\x91\x01Q\x80Q`\x02\x83\x01U` \x01Q`\x03\x90\x91\x01U`\0\x88`\x01\x81\x11\x15a\x07\xF3Wa\x07\xF3a\x16aV[\x03a\x08<W`\x01`\x01`@\x1B\x03\x89\x16`\x01`\0\x01`\0\x82\x82Ta\x08\x16\x91\x90a\x18\x92V[\x90\x91UPP`\x07Ta\x08<\x90`\x01`\x01`\xA0\x1B\x03\x1630`\x01`\x01`@\x1B\x03\x8D\x16a\nCV[\x7F\x8C\"\xDC2\xA9\xEE;6$\xF3\xF9\xF4\xF9\xBE\x14\x8Dxb^\xCD\x89<5\xE6\x9A\xB2\xFF\x0E\x0B\xFF\xC00\x84\x82\x8A\x8C`@Qa\x08q\x94\x93\x92\x91\x90a\x18\xABV[`@Q\x80\x91\x03\x90\xA1P`\x01\x9A\x99PPPPPPPPPPV[`@\x80Qc*\xDD\xA1\xC1`\xE0\x1B\x81R\x82Q`\x04\x82\x01R` \x83\x01Q`$\x82\x01R\x90\x82\x01Q`D\x82\x01R``\x82\x01Q`d\x82\x01R`\0\x90\x81\x900\x90c*\xDD\xA1\xC1\x90`\x84\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\r\x91\x90a\x19\x12V[`@\x01Q\x93\x92PPPV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\tW\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x06T`\0\x90`\n\x90a\t\x87\x90Ca\x19\xD1V[a\x03z\x91\x90a\x19\xFAV[a\t\x9A\x82a\n\xDFV[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a\x1Cj`$\x919\x90P`\0\x84\x82`@Q` \x01a\t\xCC\x92\x91\x90a\x1A>V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80a\t\xE9\x83a\x0BnV[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91Pa\n\x1D\x81\x87a\n\x10\x8Aa\x0CAV[a\n\x18a\x0C\xBCV[a\r\x8DV[a\n9W`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\n\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x05\x84V[PPPPPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a\x1C\x8E\x839\x81Q\x91R\x91\x82`\x03\x81\x80\x85\x80\t\x85\t\x08\x83\x82\x83\t\x14\x81\x15\x83\x15\x17\x19\x84\x83\x10\x85\x85\x10\x16\x16\x16\x93PPP\x81a\x0BiW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x84V[PPPV[`\0\x80`\0a\x0B|\x84a\x0EoV[\x90P`\0\x80Q` a\x1C\x8E\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\x0B\xA5Wa\x0B\xA5a\x19\xE4V[\x84\x82\t\x90P\x82\x80a\x0B\xB8Wa\x0B\xB8a\x19\xE4V[\x82\x82\x08\x90P`\0\x80a\x0B\xC9\x83a\x10\xA1V[\x92P\x90P[\x80a\x0C2W\x84\x80a\x0B\xE1Wa\x0B\xE1a\x19\xE4V[`\x01\x87\x08\x95P\x84\x80a\x0B\xF5Wa\x0B\xF5a\x19\xE4V[\x86\x87\t\x92P\x84\x80a\x0C\x08Wa\x0C\x08a\x19\xE4V[\x86\x84\t\x92P\x84\x80a\x0C\x1BWa\x0C\x1Ba\x19\xE4V[\x84\x84\x08\x92Pa\x0C)\x83a\x10\xA1V[\x92P\x90Pa\x0B\xCEV[P\x93\x97\x93\x96P\x92\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x0CiWP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a\x1C\x8E\x839\x81Q\x91R\x84` \x01Qa\x0C\x9C\x91\x90a\x1A[V[a\x0C\xB4\x90`\0\x80Q` a\x1C\x8E\x839\x81Q\x91Ra\x19\xD1V[\x90R\x92\x91PPV[a\x0C\xE7`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R\x86Q`@\x82\x01R` \x87\x01Q``\x82\x01R`@\x87\x01Q`\x80\x82\x01R``\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R` \x85\x01Qa\x01 \x82\x01R`@\x85\x01Qa\x01@\x82\x01R``\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x0EcW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x05\x84V[P\x15\x15\x95\x94PPPPPV[`\0\x80a\x0E{\x83a\x11\x9CV[\x80Q\x90\x91P`0\x81\x14a\x0E\x90Wa\x0E\x90a\x1AoV[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xAAWa\x0E\xAAa\x15\x7FV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\xD4W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0FOW\x83`\x01a\x0E\xEF\x83\x86a\x19\xD1V[a\x0E\xF9\x91\x90a\x19\xD1V[\x81Q\x81\x10a\x0F\tWa\x0F\ta\x18OV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0F&Wa\x0F&a\x18OV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x0FG\x81a\x1A\x85V[\x91PPa\x0E\xDAV[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x0F\xEBW\x83\x81a\x0F\x8D\x85\x88a\x19\xD1V[a\x0F\x97\x91\x90a\x18\x92V[\x81Q\x81\x10a\x0F\xA7Wa\x0F\xA7a\x18OV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x0F\xC7Wa\x0F\xC7a\x18OV[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0F\xE3\x81a\x1A\x85V[\x91PPa\x0FyV[P`\0a\x0F\xF7\x82a\x15\rV[\x90Pa\x01\0`\0\x80Q` a\x1C\x8E\x839\x81Q\x91R`\0a\x10\x17\x86\x89a\x19\xD1V[\x90P`\0[\x81\x81\x10\x15a\x10\x91W`\0\x88`\x01a\x103\x84\x86a\x19\xD1V[a\x10=\x91\x90a\x19\xD1V[\x81Q\x81\x10a\x10MWa\x10Ma\x18OV[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x10eWa\x10ea\x19\xE4V[\x85\x87\t\x95P\x83\x80a\x10xWa\x10xa\x19\xE4V[\x81\x87\x08\x95PP\x80\x80a\x10\x89\x90a\x1A\x85V[\x91PPa\x10\x1CV[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a\x1C\x8E\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x86``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x93PP`\0Q\x93P\x82a\x11cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x84V[\x80`\x01\x85\x90\x1B\x11\x15a\x11|Wa\x11y\x84\x82a\x19\xD1V[\x93P[\x80\x80a\x11\x8AWa\x11\x8Aa\x19\xE4V[\x84\x85\t\x91P\x85\x82\x14\x94PPPP\x91P\x91V[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x11\xDD\x92\x91\x90a\x1A>V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x12\x04\x92\x91\x90a\x1A\x9EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x12&\x91\x90a\x1A\xCAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x12P\x90\x83\x90\x83\x90` \x01a\x1A\xE4V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xC0Wa\x12\xC0a\x15\x7FV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12\xEAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x13\x02\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x13wW\x81\x81\x81Q\x81\x10a\x131Wa\x131a\x18OV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x13NWa\x13Na\x18OV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x13o\x81a\x1A\x85V[\x91PPa\x13\x16V[P`\0\x84`@Q` \x01a\x13\x8D\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x14+W`\0\x83\x82\x81Q\x81\x10a\x13\xC8Wa\x13\xC8a\x18OV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x13\xE5Wa\x13\xE5a\x18OV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x14\x06\x92\x91\x90a\x1B\tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x98PP\x80\x80a\x14#\x90a\x1A\x85V[\x91PPa\x13\xACV[P\x86\x88\x87`@Q` \x01a\x14A\x93\x92\x91\x90a\x1B.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x14o\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x14\x90\x8A`\xFF\x8D\x16a\x19\xD1V[\x81\x10\x15a\x14\xFCW\x82\x81\x81Q\x81\x10a\x14\xA9Wa\x14\xA9a\x18OV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x14\xC3\x83\x8Da\x18\x92V[\x81Q\x81\x10a\x14\xD3Wa\x14\xD3a\x18OV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x14\xF4\x81a\x1A\x85V[\x91PPa\x14\x83V[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a\x15xW\x83\x81\x81Q\x81\x10a\x15-Wa\x15-a\x18OV[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x15E\x91\x90a\x1BbV[a\x15P\x90`\x02a\x1C]V[a\x15Z\x91\x90a\x1BbV[a\x15d\x90\x83a\x18\x92V[\x91P\x80a\x15p\x81a\x1A\x85V[\x91PPa\x15\x12V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x15\xC5WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0`\x80\x82\x84\x03\x12\x15a\x15\xDDW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x16\rWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x16PW`\0\x80\xFD[a\x16Z\x83\x83a\x15\xCBV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a\x16\x95WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Q`\xE0\x83\x01\x91a\x16\xBD\x90\x84\x01\x82a\x16wV[P`@\x83\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x86\x01Q\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\xA0\x83\x01Q\x80Q`\xA0\x84\x01R` \x81\x01Q`\xC0\x84\x01RP\x92\x91PPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x17\x1FW`\0\x80\xFD[PV[`\0\x80`\xA0\x83\x85\x03\x12\x15a\x175W`\0\x80\xFD[a\x17?\x84\x84a\x15\xCBV[\x91P`\x80\x83\x015a\x17O\x81a\x17\nV[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15a\x17lW`\0\x80\xFD[a\x17ta\x15\x95V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\x02\x81\x10a\x17\x1FW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15a\x17\xB1W`\0\x80\xFD[a\x17\xBB\x88\x88a\x15\xCBV[\x95Pa\x17\xCA\x88`\x80\x89\x01a\x17ZV[\x94P`\xC0\x87\x015a\x17\xDA\x81a\x17\nV[\x93P`\xE0\x87\x015a\x17\xEA\x81a\x17\x8AV[\x92Pa\x17\xFA\x88a\x01\0\x89\x01a\x17ZV[\x91Pa\x01@\x87\x015a\x18\x0B\x81a\x17\nV[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x15xWa\x15xa\x18\x19V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80Qa\x18p\x81a\x17\nV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x18\x87W`\0\x80\xFD[\x81Qa\x16Z\x81a\x17\nV[\x80\x82\x01\x80\x82\x11\x15a\x18\xA5Wa\x18\xA5a\x18\x19V[\x92\x91PPV[\x84\x81R`\x01`\x01`@\x1B\x03\x84\x81\x16` \x83\x01R`\x80\x82\x01\x90a\x18\xD0`@\x84\x01\x86a\x16wV[\x80\x84\x16``\x84\x01RP\x95\x94PPPPPV[`\0`@\x82\x84\x03\x12\x15a\x18\xF4W`\0\x80\xFD[a\x18\xFCa\x15\x95V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15a\x19$W`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x19TWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x19nW`\0\x80\xFD[\x81R` \x83\x01Qa\x19~\x81a\x17\x8AV[` \x82\x01R`@\x83\x01Qa\x19\x91\x81a\x17\nV[`@\x82\x01Ra\x19\xA2``\x84\x01a\x18eV[``\x82\x01Ra\x19\xB3`\x80\x84\x01a\x18eV[`\x80\x82\x01Ra\x19\xC5\x84`\xA0\x85\x01a\x18\xE2V[`\xA0\x82\x01R\x93\x92PPPV[\x81\x81\x03\x81\x81\x11\x15a\x18\xA5Wa\x18\xA5a\x18\x19V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1A\tWa\x1A\ta\x19\xE4V[P\x04\x90V[`\0\x81Q`\0[\x81\x81\x10\x15a\x1A/W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x1A\x15V[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x1ASa\x1AM\x83\x86a\x1A\x0EV[\x84a\x1A\x0EV[\x94\x93PPPPV[`\0\x82a\x1AjWa\x1Aja\x19\xE4V[P\x06\x90V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1A\x97Wa\x1A\x97a\x18\x19V[P`\x01\x01\x90V[`\0a\x1A\xAA\x82\x85a\x1A\x0EV[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a\x1A\xD6\x82\x84a\x1A\x0EV[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a\x1A\xF0\x82\x85a\x1A\x0EV[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a\x1B\x15\x82\x85a\x1A\x0EV[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a\x1B:\x82\x86a\x1A\x0EV[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x18\xA5Wa\x18\xA5a\x18\x19V[`\x01\x81\x81[\x80\x85\x11\x15a\x1B\xB4W\x81`\0\x19\x04\x82\x11\x15a\x1B\x9AWa\x1B\x9Aa\x18\x19V[\x80\x85\x16\x15a\x1B\xA7W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x1B~V[P\x92P\x92\x90PV[`\0\x82a\x1B\xCBWP`\x01a\x18\xA5V[\x81a\x1B\xD8WP`\0a\x18\xA5V[\x81`\x01\x81\x14a\x1B\xEEW`\x02\x81\x14a\x1B\xF8Wa\x1C\x14V[`\x01\x91PPa\x18\xA5V[`\xFF\x84\x11\x15a\x1C\tWa\x1C\ta\x18\x19V[PP`\x01\x82\x1Ba\x18\xA5V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1C7WP\x81\x81\na\x18\xA5V[a\x1CA\x83\x83a\x1ByV[\x80`\0\x19\x04\x82\x11\x15a\x1CUWa\x1CUa\x18\x19V[\x02\x93\x92PPPV[`\0a\x16Z\x83\x83a\x1B\xBCV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 <\\\xE7\xF1Y\xAA\x16\x91vV|\xF2\x11n1\x95I\xE8\x03\xA5Jsmy6\xDA\x8E7\xA7\xA2s\xD5dsolcC\0\x08\x14\x003";
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
        ///Gets the contract's `Registered` event
        pub fn registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RegisteredFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RegisteredFilter> {
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
    ///Custom Error type `RestakingNotImplemented` with signature `RestakingNotImplemented()` and selector `0x008ebfd8`
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
    #[etherror(name = "RestakingNotImplemented", abi = "RestakingNotImplemented()")]
    pub struct RestakingNotImplemented;
    ///Container type for all of the contract's custom errors
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
    pub enum StakeTableErrors {
        BLSSigVerificationFailed(BLSSigVerificationFailed),
        RestakingNotImplemented(RestakingNotImplemented),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for StakeTableErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <BLSSigVerificationFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BLSSigVerificationFailed(decoded));
            }
            if let Ok(decoded) =
                <RestakingNotImplemented as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RestakingNotImplemented(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakeTableErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BLSSigVerificationFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RestakingNotImplemented(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for StakeTableErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BLSSigVerificationFailed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <RestakingNotImplemented as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for StakeTableErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BLSSigVerificationFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RestakingNotImplemented(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for StakeTableErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BLSSigVerificationFailed> for StakeTableErrors {
        fn from(value: BLSSigVerificationFailed) -> Self {
            Self::BLSSigVerificationFailed(value)
        }
    }
    impl ::core::convert::From<RestakingNotImplemented> for StakeTableErrors {
        fn from(value: RestakingNotImplemented) -> Self {
            Self::RestakingNotImplemented(value)
        }
    }
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
    #[ethevent(name = "Registered", abi = "Registered(bytes32,uint64,uint8,uint256)")]
    pub struct RegisteredFilter(
        pub [u8; 32],
        pub u64,
        pub u8,
        pub ::ethers::core::types::U256,
    );
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
