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
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_lightClientAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("churnRate"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint64"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("_hashBlsKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("_hashBlsKey"),
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
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("currentEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("currentEpoch"),
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
                    ::std::borrow::ToOwned::to_owned("exitEscrowPeriod"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("exitEscrowPeriod"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("node"),
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
                                ::std::borrow::ToOwned::to_owned("struct AbstractStakeTable.Node",),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("firstAvailableExitEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("firstAvailableExitEpoch",),
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
                    ::std::borrow::ToOwned::to_owned("firstAvailableRegistrationEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("firstAvailableRegistrationEpoch",),
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
                    ::std::borrow::ToOwned::to_owned("lightClient"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lightClient"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract LightClient"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                                ::std::borrow::ToOwned::to_owned("struct AbstractStakeTable.Node",),
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
                    ::std::borrow::ToOwned::to_owned("maxChurnRate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("maxChurnRate"),
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
                    ::std::borrow::ToOwned::to_owned("nextExitEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nextExitEpoch"),
                        inputs: ::std::vec![],
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nextRegistrationEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nextRegistrationEpoch",),
                        inputs: ::std::vec![],
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nodes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nodes"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("keyHash"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakeType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum AbstractStakeTable.StakeType",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("registerEpoch"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("exitEpoch"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
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
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("numPendingExits"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("numPendingExits"),
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
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum AbstractStakeTable.StakeType",
                                    ),
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
                        outputs: ::std::vec![],
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("totalNativeStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalNativeStake"),
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
                    ::std::borrow::ToOwned::to_owned("totalRestakedStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalRestakedStake"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Deposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("blsVKhash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Exit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Exit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("blsVKhash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("exitEpoch"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Registered"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Registered"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("blsVKhash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("registerEpoch"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("stakeType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amountDeposited"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BLSSigVerificationFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BLSSigVerificationFailed",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExitRequestInProgress"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ExitRequestInProgress",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidNextRegistrationEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidNextRegistrationEpoch",),
                        inputs: ::std::vec![
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NodeAlreadyRegistered"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NodeAlreadyRegistered",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PrematureDeposit"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PrematureDeposit"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PrematureExit"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PrematureExit"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PrematureWithdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PrematureWithdrawal",),
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
                (
                    ::std::borrow::ToOwned::to_owned("Unauthenticated"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Unauthenticated"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0(\xBD8\x03\x80b\0(\xBD\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0\xE3V[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90U`\x06\x80T`\x07\x80T`\x01`\x01`\xE0\x1B\x03\x19\x90\x92\x16\x94\x90\x95\x16\x93\x90\x93\x17`\x01`\xA0\x1B\x17\x90U`\x01`\x80\x1B`\x01`\xC0\x1B\x03\x19`\x01`\x01`\x80\x1B\x03\x19`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`\xC0\x1B\x02\x91\x90\x91\x16`\x01`\x80\x1B`\x01`\xC0\x1B\x03\x90\x92\x16\x91\x90\x91\x17h\x01\0\0\0\0\0\0\0\0\x17\x16\x90Ub\0\x01=V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xDEW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\0\xF9W`\0\x80\xFD[b\0\x01\x04\x84b\0\0\xC6V[\x92Pb\0\x01\x14` \x85\x01b\0\0\xC6V[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x16\x81\x14b\0\x012W`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[a'p\x80b\0\x01M`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80cn\x8ENj\x11a\0\xC3W\x80c\xB5p\x0Eh\x11a\0|W\x80c\xB5p\x0Eh\x14a\x03\x13W\x80c\xC7,\xC7\x17\x14a\x03&W\x80c\xC8L\x7F\xA1\x14a\x039W\x80c\xD6{l\xA5\x14a\x03LW\x80c\xD8ni}\x14a\x03dW\x80c\xDD.\xD3\xEC\x14a\x03\xF2W`\0\x80\xFD[\x80cn\x8ENj\x14a\x02\x96W\x80cvg\x18\x08\x14a\x02\x9FW\x80cw\x1FoD\x14a\x02\xA7W\x80c\x8B\x0E\x9F?\x14a\x02\xBAW\x80c\x9B0\xA5\xE6\x14a\x02\xD5W\x80c\x9Dv\xEAX\x14a\x02\xE8W`\0\x80\xFD[\x80c,p\x12i\x11a\x01\x15W\x80c,p\x12i\x14a\x02\x1AW\x80c;\t\xC2g\x14a\x024W\x80cC\x17\xD0\x0B\x14a\x02<W\x80cH\x8B\xDA\xBC\x14a\x02SW\x80cJ\xA7\xC2\x7F\x14a\x02xW\x80cTL-v\x14a\x02\x8DW`\0\x80\xFD[\x80c\x0C$\xAF\x18\x14a\x01]W\x80c\x10\x9E;\xE3\x14a\x01\x8DW\x80c\x16\xFE\xFE\xD7\x14a\x01\xA7W\x80c!\xBF\xD5\x15\x14a\x01\xB8W\x80c*\xDD\xA1\xC1\x14a\x01\xD2W\x80c,S\x05\x84\x14a\x01\xF2W[`\0\x80\xFD[a\x01pa\x01k6`\x04a \tV[a\x04\x05V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x07Ta\x01p\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x07T`\x01`\x01`@\x1B\x03\x16a\x01pV[`\x07Ta\x01p\x90`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x01\xE5a\x01\xE06`\x04a \tV[a\x05\x91V[`@Qa\x01\x84\x91\x90a dV[a\x01\xFAa\x06\xA7V[`@\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x01\x84V[`\x06Ta\x01p\x90`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x01\xFAa\x07\\V[a\x02E`\x01T\x81V[`@Q\x90\x81R` \x01a\x01\x84V[`\0Ta\x02c\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x84V[a\x02\x8Ba\x02\x866`\x04a \tV[a\x08\x02V[\0[a\x02E`\x03T\x81V[a\x02E`\x04T\x81V[a\x01pa\n]V[a\x01\xFAa\x02\xB56`\x04a \xECV[a\n\xD0V[`\x03T`\x04T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x84V[a\x02Ea\x02\xE36`\x04a \tV[a\r;V[`\x05Ta\x02\xFB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x84V[`\x06Ta\x02\xFB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\x8Ba\x0346`\x04a!aV[a\r\x97V[a\x01pa\x03G6`\x04a\"\x19V[a\x11qV[`\x07T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x01pV[a\x03\xE0a\x03r6`\x04a\"\xA5V[`\x02` \x81\x81R`\0\x92\x83R`@\x92\x83\x90 \x80T`\x01\x82\x01T\x85Q\x80\x87\x01\x90\x96R\x93\x82\x01T\x85R`\x03\x90\x91\x01T\x91\x84\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x81\x16\x92`\xFF`\x01`\xA0\x1B\x83\x04\x16\x92`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x93\x04\x83\x16\x92\x81\x81\x16\x92`\x01`@\x1B\x90\x92\x04\x16\x90\x86V[`@Qa\x01\x84\x96\x95\x94\x93\x92\x91\x90a\"\xBEV[a\x01pa\x04\x006`\x04a \tV[a\x11\x99V[`\0\x80a\x04\x11\x83a\r;V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x04_Wa\x04_a ,V[`\x01\x81\x11\x15a\x04pWa\x04pa ,V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x90Pa\x04\xDB\x81a\x11qV[\x81`\x80\x01Qa\x04\xEA\x91\x90a#,V[`\x01`\x01`@\x1B\x03\x16a\x04\xFBa\n]V[`\x01`\x01`@\x1B\x03\x16\x10\x15a\x05#W`@QcZwCW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80\x82\x01Q`\0\x84\x81R`\x02` \x81\x90R\x92\x81 \x80T`\x01`\x01`\xE8\x1B\x03\x19\x16\x81U`\x01\x81\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90U\x92\x83\x01\x81\x90U`\x03\x90\x92\x01\x91\x90\x91U`\x05T\x82Qa\x05\x89\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\x01`@\x1B\x03\x84\x16a\x12'V[\x94\x93PPPPV[a\x05\xD6`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\xA0\x82\x01R\x90V[`\x02`\0a\x05\xE3\x84a\r;V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x91\x92\x90\x91\x90\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x060Wa\x060a ,V[`\x01\x81\x11\x15a\x06AWa\x06Aa ,V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x92\x91PPV[`\0\x80`\0\x80a\x06\xB5a\n]V[a\x06\xC0\x90`\x01a#,V[`\x06T`\x01`\x01`@\x1B\x03\x91\x82\x16`\x01`\xA0\x1B\x90\x91\x04\x90\x91\x16\x10\x15a\x06\xFDWa\x06\xE7a\n]V[a\x06\xF2\x90`\x01a#,V[\x91P`\0\x90Pa\x07SV[`\x07T`\x01`\x01`@\x1B\x03`\x01`\xC0\x1B\x82\x04\x81\x16\x91\x16\x10a\x076W`\x06Ta\x06\xF2\x90`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16`\x01a#,V[PP`\x06T`\x07T`\x01`\x01`@\x1B\x03`\x01`\xA0\x1B\x90\x92\x04\x82\x16\x91\x16[\x90\x93\x90\x92P\x90PV[`\0\x80`\0\x80a\x07ja\n]V[a\x07u\x90`\x01a#,V[`\x07T`\x01`\x01`@\x1B\x03\x91\x82\x16`\x01`@\x1B\x90\x91\x04\x90\x91\x16\x10\x15a\x07\x9CWa\x06\xE7a\n]V[`\x07T`\x01`\x01`@\x1B\x03`\x01`\xC0\x1B\x82\x04\x81\x16`\x01`\x80\x1B\x90\x92\x04\x16\x10a\x07\xDCW`\x07Ta\x06\xF2\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16`\x01a#,V[PP`\x07T`\x01`\x01`@\x1B\x03`\x01`@\x1B\x82\x04\x81\x16\x94`\x01`\x80\x1B\x90\x92\x04\x16\x92P\x90PV[`\0a\x08\r\x82a\r;V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x08[Wa\x08[a ,V[`\x01\x81\x11\x15a\x08lWa\x08la ,V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xF9W`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x15a\t%W`@Qc7\xA8>\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81\x01Qa\t5\x90`\x01a#,V[`\x01`\x01`@\x1B\x03\x16a\tFa\n]V[`\x01`\x01`@\x1B\x03\x16\x10\x15a\tnW`@Qcxz\xEBS`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x800`\x01`\x01`\xA0\x1B\x03\x16c;\t\xC2g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xAEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xD2\x91\x90a#LV[`\0\x86\x81R`\x02` R`@\x90 `\x01\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19\x16`\x01`@\x1B`\x01`\x01`@\x1B\x03\x85\x16\x02\x17\x90U\x90\x92P\x90Pa\n\x15\x82\x82a\x12\xAEV[`@\x80Q\x85\x81R`\x01`\x01`@\x1B\x03\x84\x16` \x82\x01R\x7F\x1D\xA0\xACf\xB9>\xA1\xC8\xD3%\x14$\xE6W(\xBC\xEEZ\x7F\xB7Y\xA9G\x15c\xEF\x8Ef\x82\xBB\xD7\x11\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x06T`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cvg\x18\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xCB\x91\x90a#{V[\x90P\x90V[`\0\x80`\0a\n\xDE\x85a\r;V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x0B,Wa\x0B,a ,V[`\x01\x81\x11\x15a\x0B=Wa\x0B=a ,V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xCAW`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0B\xD4a\n]V[\x90P\x81``\x01Q`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x11a\x0C\x0CW`@Qc)\x8Be\xF3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x82\x01Q`\x01`\x01`@\x1B\x03\x16\x15a\x0C8W`@Qc7\xA8>\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R`\x02` R`@\x90 \x80T\x87\x91\x90`\x15\x90a\x0Ci\x90\x84\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a#,V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa\x0C\xB7`\x05`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x1630\x89`\x01`\x01`@\x1B\x03\x16a\x13\x08V[\x7F\x98\xE7\x83\xC3\x86K\xBFtJ\x05~\xF6\x05\xA2\xA6\x17\x01\xC3\xB6+^\xD6\x8B7E\xB9\x90\x94I}\xAF\x1Fa\x0C\xE1\x88a\r;V[`@\x80Q\x91\x82R`\x01`\x01`@\x1B\x03\x89\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1`\0a\r\r\x82`\x01a#,V[`\0\x94\x85R`\x02` R`@\x90\x94 T`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x98\x93\x97P\x92\x95PPPPPPV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\rz\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x83`\x01\x81\x11\x15a\r\xABWa\r\xABa ,V[\x14a\r\xC8W`@Qb\x11\xD7\xFB`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\r\xF2\x81\x84\x89a\x13\xA4V[`\0\x800`\x01`\x01`\xA0\x1B\x03\x16c,S\x05\x84`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EV\x91\x90a#LV[\x91P\x91P\x83`\x01`\x01`@\x1B\x03\x16\x82`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0E\xA5W`@Qc!\xDF\x8B\xC3`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x80\x84\x16`\x04\x83\x01R\x85\x16`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[a\x0E\xAF\x82\x82a\x14<V[`\0a\x0E\xBA\x8Aa\r;V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x0F\x08Wa\x0F\x08a ,V[`\x01\x81\x11\x15a\x0F\x19Wa\x0F\x19a ,V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0F\xA5W`@Qc\x0E\xB0\xD3\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3\x81R`\x01`\x01`@\x1B\x03\x89\x16`@\x82\x01R` \x81\x01\x88`\x01\x81\x11\x15a\x0F\xCDWa\x0F\xCDa ,V[\x90\x81`\x01\x81\x11\x15a\x0F\xE0Wa\x0F\xE0a ,V[\x90RP`\xA0\x81\x01\x8A\x90R`\x01`\x01`@\x1B\x03\x84\x16``\x82\x01R`\0\x82\x81R`\x02` \x90\x81R`@\x90\x91 \x82Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x83U\x92\x84\x01Q\x84\x93\x90\x91\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\x10UWa\x10Ua ,V[\x02\x17\x90UP`@\x82\x01Q\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17\x82U``\x83\x01Q`\x01\x83\x01\x80T`\x80\x86\x01Q\x92\x84\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17`\x01`@\x1B\x92\x90\x93\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90U`\xA0\x90\x91\x01Q\x80Q`\x02\x83\x01U` \x01Q`\x03\x90\x91\x01U`\0\x88`\x01\x81\x11\x15a\x10\xE1Wa\x10\xE1a ,V[\x03a\x11'W\x88`\x01`\x01`@\x1B\x03\x16`\x03`\0\x82\x82Ta\x11\x01\x91\x90a#\x98V[\x90\x91UPP`\x05Ta\x11'\x90`\x01`\x01`\xA0\x1B\x03\x1630`\x01`\x01`@\x1B\x03\x8D\x16a\x13\x08V[\x7F\x8C\"\xDC2\xA9\xEE;6$\xF3\xF9\xF4\xF9\xBE\x14\x8Dxb^\xCD\x89<5\xE6\x9A\xB2\xFF\x0E\x0B\xFF\xC00\x82\x85\x8A\x8C`@Qa\x11\\\x94\x93\x92\x91\x90a#\xB1V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPV[`\0`d\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\x11\x91WP`\n\x91\x90PV[P`\x05\x91\x90PV[`@\x80Qc*\xDD\xA1\xC1`\xE0\x1B\x81R\x82Q`\x04\x82\x01R` \x83\x01Q`$\x82\x01R\x90\x82\x01Q`D\x82\x01R``\x82\x01Q`d\x82\x01R`\0\x90\x81\x900\x90c*\xDD\xA1\xC1\x90`\x84\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x1C\x91\x90a#\xE8V[`@\x01Q\x93\x92PPPV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x12\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x0E\x9CV[PPPPV[`\x07\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19\x16`\x01`@\x1B`\x01`\x01`@\x1B\x03\x85\x16\x02\x17\x90Ua\x12\xDE\x81`\x01a#,V[`\x07`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPV[`\0`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x13\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x0E\x9CV[PPPPPV[a\x13\xAD\x82a\x14\x91V[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a' `$\x919\x90P`\0\x84\x82`@Q` \x01a\x13\xDF\x92\x91\x90a$\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\x13\xFB\x82a\x15.V[\x90Pa\x14\x18\x81\x85a\x14\x0B\x88a\x16 V[a\x14\x13a\x16\x9BV[a\x17lV[a\x144W`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\x06\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B`\x01`\x01`@\x1B\x03\x85\x16\x02\x17\x90Ua\x14l\x81`\x01a#,V[`\x07\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a'D\x839\x81Q\x91R\x91\x15\x90\x15\x16\x15a\x14\xB9WPPPV[\x82Q` \x84\x01Q\x82`\x03\x84\x85\x85\x86\t\x85\t\x08\x83\x82\x83\t\x14\x83\x82\x10\x84\x84\x10\x16\x16\x93PPP\x81a\x15)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0E\x9CV[PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0a\x15M\x83a\x18NV[\x90P`\0\x80Q` a'D\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\x15vWa\x15va$\xDDV[\x84\x82\t\x90P\x82\x80a\x15\x89Wa\x15\x89a$\xDDV[\x82\x82\x08\x90P`\0\x80a\x15\x9A\x83a\x1AaV[\x92P\x90P[\x80a\x16\x03W\x84\x80a\x15\xB2Wa\x15\xB2a$\xDDV[`\x01\x87\x08\x95P\x84\x80a\x15\xC6Wa\x15\xC6a$\xDDV[\x86\x87\t\x92P\x84\x80a\x15\xD9Wa\x15\xD9a$\xDDV[\x86\x84\t\x92P\x84\x80a\x15\xECWa\x15\xECa$\xDDV[\x84\x84\x08\x92Pa\x15\xFA\x83a\x1AaV[\x92P\x90Pa\x15\x9FV[P`@\x80Q\x80\x82\x01\x90\x91R\x94\x85R` \x85\x01RP\x91\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x16HWP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a'D\x839\x81Q\x91R\x84` \x01Qa\x16{\x91\x90a$\xF3V[a\x16\x93\x90`\0\x80Q` a'D\x839\x81Q\x91Ra%\x15V[\x90R\x92\x91PPV[a\x16\xC6`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R` \x87\x01Q`@\x82\x01R\x86Q``\x82\x01R``\x87\x01Q`\x80\x82\x01R`@\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R` \x85\x01Qa\x01\0\x82\x01R\x84Qa\x01 \x82\x01R``\x85\x01Qa\x01@\x82\x01R`@\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x18BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x0E\x9CV[P\x15\x15\x95\x94PPPPPV[`\0\x80a\x18Z\x83a\x1B_V[\x80Q\x90\x91P`0\x81\x14a\x18oWa\x18oa%(V[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x89Wa\x18\x89a\x1F\x1AV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x18\xB3W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x19$W\x83`\x01a\x18\xCE\x83\x86a%\x15V[a\x18\xD8\x91\x90a%\x15V[\x81Q\x81\x10a\x18\xE8Wa\x18\xE8a%>V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x19\x05Wa\x19\x05a%>V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x18\xB9V[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x19\xB6W\x83\x81a\x19b\x85\x88a%\x15V[a\x19l\x91\x90a#\x98V[\x81Q\x81\x10a\x19|Wa\x19|a%>V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x19\x9CWa\x19\x9Ca%>V[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x19NV[P`\0a\x19\xC2\x82a\x1E\xB2V[\x90Pa\x01\0`\0\x80Q` a'D\x839\x81Q\x91R`\0a\x19\xE2\x86\x89a%\x15V[\x90P`\0[\x81\x81\x10\x15a\x1AQW`\0\x88`\x01a\x19\xFE\x84\x86a%\x15V[a\x1A\x08\x91\x90a%\x15V[\x81Q\x81\x10a\x1A\x18Wa\x1A\x18a%>V[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x1A0Wa\x1A0a$\xDDV[\x85\x87\t\x95P\x83\x80a\x1ACWa\x1ACa$\xDDV[\x81\x87\x08\x95PP`\x01\x01a\x19\xE7V[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80`\0\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a'D\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x87``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x94PP`\0Q\x92P\x83a\x1B%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x0E\x9CV[\x80`\x01\x84\x90\x1B\x11\x15a\x1B>Wa\x1B;\x83\x82a%\x15V[\x92P[\x80\x80a\x1BLWa\x1BLa$\xDDV[\x83\x84\t\x96\x90\x96\x14\x96\x91\x95P\x90\x93PPPPV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x1B\xA0\x92\x91\x90a$\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x1B\xC7\x92\x91\x90a%TV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x1B\xE9\x91\x90a%\x80V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x1C\x13\x90\x83\x90\x83\x90` \x01a%\x9AV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\x83Wa\x1C\x83a\x1F\x1AV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1C\xADW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x1C\xC5\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x1D0W\x81\x81\x81Q\x81\x10a\x1C\xF4Wa\x1C\xF4a%>V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x1D\x11Wa\x1D\x11a%>V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x1C\xD9V[P`\0\x84`@Q` \x01a\x1DF\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x1D\xDAW`\0\x83\x82\x81Q\x81\x10a\x1D\x81Wa\x1D\x81a%>V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x1D\x9EWa\x1D\x9Ea%>V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x1D\xBF\x92\x91\x90a%\xBFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x98PP`\x01\x01a\x1DeV[P\x86\x88\x87`@Q` \x01a\x1D\xF0\x93\x92\x91\x90a%\xE4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x1E\x1E\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x1E?\x8A`\xFF\x8D\x16a%\x15V[\x81\x10\x15a\x1E\xA1W\x82\x81\x81Q\x81\x10a\x1EXWa\x1EXa%>V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x1Er\x83\x8Da#\x98V[\x81Q\x81\x10a\x1E\x82Wa\x1E\x82a%>V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x1E2V[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a\x1F\x13W\x83\x81\x81Q\x81\x10a\x1E\xD2Wa\x1E\xD2a%>V[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x1E\xEA\x91\x90a&\x18V[a\x1E\xF5\x90`\x02a'\x13V[a\x1E\xFF\x91\x90a&\x18V[a\x1F\t\x90\x83a#\x98V[\x91P`\x01\x01a\x1E\xB7V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1F`WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1F`WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\x80\x82\x84\x03\x12\x15a\x1F\xA8W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1F\xD8WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a \x1BW`\0\x80\xFD[a %\x83\x83a\x1F\x96V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a `WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Q`\xE0\x83\x01\x91a \x88\x90\x84\x01\x82a BV[P`@\x83\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x86\x01Q\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\xA0\x83\x01Qa\x1F\x13`\xA0\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a \xE9W`\0\x80\xFD[PV[`\0\x80`\xA0\x83\x85\x03\x12\x15a \xFFW`\0\x80\xFD[a!\t\x84\x84a\x1F\x96V[\x91P`\x80\x83\x015a!\x19\x81a \xD4V[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15a!6W`\0\x80\xFD[a!>a\x1F0V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\x02\x81\x10a \xE9W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x01`\x81\x12\x15a!|W`\0\x80\xFD[a!\x86\x89\x89a\x1F\x96V[\x96Pa!\x95\x89`\x80\x8A\x01a!$V[\x95P`\xC0\x88\x015a!\xA5\x81a \xD4V[\x94P`\xE0\x88\x015a!\xB5\x81a!TV[\x93P`@`\xFF\x19\x82\x01\x12\x15a!\xC9W`\0\x80\xFD[Pa!\xD2a\x1F0V[a\x01\0\x88\x015\x81Ra\x01 \x88\x015` \x82\x01R\x91Pa\x01@\x87\x015a!\xF6\x81a \xD4V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a \xE9W`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15a\"+W`\0\x80\xFD[a\"3a\x1FfV[\x825a\">\x81a\"\x04V[\x81R` \x83\x015a\"N\x81a!TV[` \x82\x01R`@\x83\x015a\"a\x81a \xD4V[`@\x82\x01R``\x83\x015a\"t\x81a \xD4V[``\x82\x01R`\x80\x83\x015a\"\x87\x81a \xD4V[`\x80\x82\x01Ra\"\x99\x84`\xA0\x85\x01a!$V[`\xA0\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\"\xB7W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x87\x16\x81R`\xE0\x81\x01a\"\xDB` \x83\x01\x88a BV[`\x01`\x01`@\x1B\x03\x86\x81\x16`@\x84\x01R\x85\x81\x16``\x84\x01R\x84\x16`\x80\x83\x01R\x82Q`\xA0\x83\x01R` \x83\x01Q`\xC0\x83\x01R\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x1F\x13Wa\x1F\x13a#\x16V[`\0\x80`@\x83\x85\x03\x12\x15a#_W`\0\x80\xFD[\x82Qa#j\x81a \xD4V[` \x84\x01Q\x90\x92Pa!\x19\x81a \xD4V[`\0` \x82\x84\x03\x12\x15a#\x8DW`\0\x80\xFD[\x81Qa %\x81a \xD4V[\x80\x82\x01\x80\x82\x11\x15a#\xABWa#\xABa#\x16V[\x92\x91PPV[\x84\x81R`\x01`\x01`@\x1B\x03\x84\x81\x16` \x83\x01R`\x80\x82\x01\x90a#\xD6`@\x84\x01\x86a BV[\x80\x84\x16``\x84\x01RP\x95\x94PPPPPV[`\0\x81\x83\x03`\xE0\x81\x12\x15a#\xFBW`\0\x80\xFD[a$\x03a\x1FfV[\x83Qa$\x0E\x81a\"\x04V[\x81R` \x84\x01Qa$\x1E\x81a!TV[` \x82\x01R`@\x84\x01Qa$1\x81a \xD4V[`@\x82\x01R``\x84\x01Qa$D\x81a \xD4V[``\x82\x01R`\x80\x84\x01Qa$W\x81a \xD4V[`\x80\x82\x01R`@`\x9F\x19\x83\x01\x12\x15a$nW`\0\x80\xFD[a$va\x1F0V[`\xA0\x85\x81\x01Q\x82R`\xC0\x90\x95\x01Q` \x82\x01R\x93\x81\x01\x93\x90\x93RP\x90\x92\x91PPV[`\0\x81Q`\0[\x81\x81\x10\x15a$\xB9W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a$\x9FV[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x05\x89a$\xD7\x83\x86a$\x98V[\x84a$\x98V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a%\x10WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x81\x81\x03\x81\x81\x11\x15a#\xABWa#\xABa#\x16V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0a%`\x82\x85a$\x98V[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a%\x8C\x82\x84a$\x98V[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a%\xA6\x82\x85a$\x98V[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a%\xCB\x82\x85a$\x98V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a%\xF0\x82\x86a$\x98V[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a#\xABWa#\xABa#\x16V[`\x01\x81\x81[\x80\x85\x11\x15a&jW\x81`\0\x19\x04\x82\x11\x15a&PWa&Pa#\x16V[\x80\x85\x16\x15a&]W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a&4V[P\x92P\x92\x90PV[`\0\x82a&\x81WP`\x01a#\xABV[\x81a&\x8EWP`\0a#\xABV[\x81`\x01\x81\x14a&\xA4W`\x02\x81\x14a&\xAEWa&\xCAV[`\x01\x91PPa#\xABV[`\xFF\x84\x11\x15a&\xBFWa&\xBFa#\x16V[PP`\x01\x82\x1Ba#\xABV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a&\xEDWP\x81\x81\na#\xABV[a&\xF7\x83\x83a&/V[\x80`\0\x19\x04\x82\x11\x15a'\x0BWa'\x0Ba#\x16V[\x02\x93\x92PPPV[`\0a %\x83\x83a&rV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static STAKETABLE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80cn\x8ENj\x11a\0\xC3W\x80c\xB5p\x0Eh\x11a\0|W\x80c\xB5p\x0Eh\x14a\x03\x13W\x80c\xC7,\xC7\x17\x14a\x03&W\x80c\xC8L\x7F\xA1\x14a\x039W\x80c\xD6{l\xA5\x14a\x03LW\x80c\xD8ni}\x14a\x03dW\x80c\xDD.\xD3\xEC\x14a\x03\xF2W`\0\x80\xFD[\x80cn\x8ENj\x14a\x02\x96W\x80cvg\x18\x08\x14a\x02\x9FW\x80cw\x1FoD\x14a\x02\xA7W\x80c\x8B\x0E\x9F?\x14a\x02\xBAW\x80c\x9B0\xA5\xE6\x14a\x02\xD5W\x80c\x9Dv\xEAX\x14a\x02\xE8W`\0\x80\xFD[\x80c,p\x12i\x11a\x01\x15W\x80c,p\x12i\x14a\x02\x1AW\x80c;\t\xC2g\x14a\x024W\x80cC\x17\xD0\x0B\x14a\x02<W\x80cH\x8B\xDA\xBC\x14a\x02SW\x80cJ\xA7\xC2\x7F\x14a\x02xW\x80cTL-v\x14a\x02\x8DW`\0\x80\xFD[\x80c\x0C$\xAF\x18\x14a\x01]W\x80c\x10\x9E;\xE3\x14a\x01\x8DW\x80c\x16\xFE\xFE\xD7\x14a\x01\xA7W\x80c!\xBF\xD5\x15\x14a\x01\xB8W\x80c*\xDD\xA1\xC1\x14a\x01\xD2W\x80c,S\x05\x84\x14a\x01\xF2W[`\0\x80\xFD[a\x01pa\x01k6`\x04a \tV[a\x04\x05V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x07Ta\x01p\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x07T`\x01`\x01`@\x1B\x03\x16a\x01pV[`\x07Ta\x01p\x90`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x01\xE5a\x01\xE06`\x04a \tV[a\x05\x91V[`@Qa\x01\x84\x91\x90a dV[a\x01\xFAa\x06\xA7V[`@\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x01\x84V[`\x06Ta\x01p\x90`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x01\xFAa\x07\\V[a\x02E`\x01T\x81V[`@Q\x90\x81R` \x01a\x01\x84V[`\0Ta\x02c\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x84V[a\x02\x8Ba\x02\x866`\x04a \tV[a\x08\x02V[\0[a\x02E`\x03T\x81V[a\x02E`\x04T\x81V[a\x01pa\n]V[a\x01\xFAa\x02\xB56`\x04a \xECV[a\n\xD0V[`\x03T`\x04T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x84V[a\x02Ea\x02\xE36`\x04a \tV[a\r;V[`\x05Ta\x02\xFB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x84V[`\x06Ta\x02\xFB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\x8Ba\x0346`\x04a!aV[a\r\x97V[a\x01pa\x03G6`\x04a\"\x19V[a\x11qV[`\x07T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x01pV[a\x03\xE0a\x03r6`\x04a\"\xA5V[`\x02` \x81\x81R`\0\x92\x83R`@\x92\x83\x90 \x80T`\x01\x82\x01T\x85Q\x80\x87\x01\x90\x96R\x93\x82\x01T\x85R`\x03\x90\x91\x01T\x91\x84\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x81\x16\x92`\xFF`\x01`\xA0\x1B\x83\x04\x16\x92`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x93\x04\x83\x16\x92\x81\x81\x16\x92`\x01`@\x1B\x90\x92\x04\x16\x90\x86V[`@Qa\x01\x84\x96\x95\x94\x93\x92\x91\x90a\"\xBEV[a\x01pa\x04\x006`\x04a \tV[a\x11\x99V[`\0\x80a\x04\x11\x83a\r;V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x04_Wa\x04_a ,V[`\x01\x81\x11\x15a\x04pWa\x04pa ,V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x90Pa\x04\xDB\x81a\x11qV[\x81`\x80\x01Qa\x04\xEA\x91\x90a#,V[`\x01`\x01`@\x1B\x03\x16a\x04\xFBa\n]V[`\x01`\x01`@\x1B\x03\x16\x10\x15a\x05#W`@QcZwCW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80\x82\x01Q`\0\x84\x81R`\x02` \x81\x90R\x92\x81 \x80T`\x01`\x01`\xE8\x1B\x03\x19\x16\x81U`\x01\x81\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90U\x92\x83\x01\x81\x90U`\x03\x90\x92\x01\x91\x90\x91U`\x05T\x82Qa\x05\x89\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\x01`@\x1B\x03\x84\x16a\x12'V[\x94\x93PPPPV[a\x05\xD6`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\xA0\x82\x01R\x90V[`\x02`\0a\x05\xE3\x84a\r;V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x91\x92\x90\x91\x90\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x060Wa\x060a ,V[`\x01\x81\x11\x15a\x06AWa\x06Aa ,V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x92\x91PPV[`\0\x80`\0\x80a\x06\xB5a\n]V[a\x06\xC0\x90`\x01a#,V[`\x06T`\x01`\x01`@\x1B\x03\x91\x82\x16`\x01`\xA0\x1B\x90\x91\x04\x90\x91\x16\x10\x15a\x06\xFDWa\x06\xE7a\n]V[a\x06\xF2\x90`\x01a#,V[\x91P`\0\x90Pa\x07SV[`\x07T`\x01`\x01`@\x1B\x03`\x01`\xC0\x1B\x82\x04\x81\x16\x91\x16\x10a\x076W`\x06Ta\x06\xF2\x90`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16`\x01a#,V[PP`\x06T`\x07T`\x01`\x01`@\x1B\x03`\x01`\xA0\x1B\x90\x92\x04\x82\x16\x91\x16[\x90\x93\x90\x92P\x90PV[`\0\x80`\0\x80a\x07ja\n]V[a\x07u\x90`\x01a#,V[`\x07T`\x01`\x01`@\x1B\x03\x91\x82\x16`\x01`@\x1B\x90\x91\x04\x90\x91\x16\x10\x15a\x07\x9CWa\x06\xE7a\n]V[`\x07T`\x01`\x01`@\x1B\x03`\x01`\xC0\x1B\x82\x04\x81\x16`\x01`\x80\x1B\x90\x92\x04\x16\x10a\x07\xDCW`\x07Ta\x06\xF2\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16`\x01a#,V[PP`\x07T`\x01`\x01`@\x1B\x03`\x01`@\x1B\x82\x04\x81\x16\x94`\x01`\x80\x1B\x90\x92\x04\x16\x92P\x90PV[`\0a\x08\r\x82a\r;V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x08[Wa\x08[a ,V[`\x01\x81\x11\x15a\x08lWa\x08la ,V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xF9W`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x15a\t%W`@Qc7\xA8>\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81\x01Qa\t5\x90`\x01a#,V[`\x01`\x01`@\x1B\x03\x16a\tFa\n]V[`\x01`\x01`@\x1B\x03\x16\x10\x15a\tnW`@Qcxz\xEBS`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x800`\x01`\x01`\xA0\x1B\x03\x16c;\t\xC2g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xAEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xD2\x91\x90a#LV[`\0\x86\x81R`\x02` R`@\x90 `\x01\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19\x16`\x01`@\x1B`\x01`\x01`@\x1B\x03\x85\x16\x02\x17\x90U\x90\x92P\x90Pa\n\x15\x82\x82a\x12\xAEV[`@\x80Q\x85\x81R`\x01`\x01`@\x1B\x03\x84\x16` \x82\x01R\x7F\x1D\xA0\xACf\xB9>\xA1\xC8\xD3%\x14$\xE6W(\xBC\xEEZ\x7F\xB7Y\xA9G\x15c\xEF\x8Ef\x82\xBB\xD7\x11\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x06T`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cvg\x18\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xCB\x91\x90a#{V[\x90P\x90V[`\0\x80`\0a\n\xDE\x85a\r;V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x0B,Wa\x0B,a ,V[`\x01\x81\x11\x15a\x0B=Wa\x0B=a ,V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xCAW`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0B\xD4a\n]V[\x90P\x81``\x01Q`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x11a\x0C\x0CW`@Qc)\x8Be\xF3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x82\x01Q`\x01`\x01`@\x1B\x03\x16\x15a\x0C8W`@Qc7\xA8>\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R`\x02` R`@\x90 \x80T\x87\x91\x90`\x15\x90a\x0Ci\x90\x84\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a#,V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa\x0C\xB7`\x05`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x1630\x89`\x01`\x01`@\x1B\x03\x16a\x13\x08V[\x7F\x98\xE7\x83\xC3\x86K\xBFtJ\x05~\xF6\x05\xA2\xA6\x17\x01\xC3\xB6+^\xD6\x8B7E\xB9\x90\x94I}\xAF\x1Fa\x0C\xE1\x88a\r;V[`@\x80Q\x91\x82R`\x01`\x01`@\x1B\x03\x89\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1`\0a\r\r\x82`\x01a#,V[`\0\x94\x85R`\x02` R`@\x90\x94 T`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x98\x93\x97P\x92\x95PPPPPPV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\rz\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x83`\x01\x81\x11\x15a\r\xABWa\r\xABa ,V[\x14a\r\xC8W`@Qb\x11\xD7\xFB`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\r\xF2\x81\x84\x89a\x13\xA4V[`\0\x800`\x01`\x01`\xA0\x1B\x03\x16c,S\x05\x84`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EV\x91\x90a#LV[\x91P\x91P\x83`\x01`\x01`@\x1B\x03\x16\x82`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0E\xA5W`@Qc!\xDF\x8B\xC3`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x80\x84\x16`\x04\x83\x01R\x85\x16`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[a\x0E\xAF\x82\x82a\x14<V[`\0a\x0E\xBA\x8Aa\r;V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x0F\x08Wa\x0F\x08a ,V[`\x01\x81\x11\x15a\x0F\x19Wa\x0F\x19a ,V[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0F\xA5W`@Qc\x0E\xB0\xD3\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3\x81R`\x01`\x01`@\x1B\x03\x89\x16`@\x82\x01R` \x81\x01\x88`\x01\x81\x11\x15a\x0F\xCDWa\x0F\xCDa ,V[\x90\x81`\x01\x81\x11\x15a\x0F\xE0Wa\x0F\xE0a ,V[\x90RP`\xA0\x81\x01\x8A\x90R`\x01`\x01`@\x1B\x03\x84\x16``\x82\x01R`\0\x82\x81R`\x02` \x90\x81R`@\x90\x91 \x82Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x83U\x92\x84\x01Q\x84\x93\x90\x91\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\x10UWa\x10Ua ,V[\x02\x17\x90UP`@\x82\x01Q\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17\x82U``\x83\x01Q`\x01\x83\x01\x80T`\x80\x86\x01Q\x92\x84\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17`\x01`@\x1B\x92\x90\x93\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90U`\xA0\x90\x91\x01Q\x80Q`\x02\x83\x01U` \x01Q`\x03\x90\x91\x01U`\0\x88`\x01\x81\x11\x15a\x10\xE1Wa\x10\xE1a ,V[\x03a\x11'W\x88`\x01`\x01`@\x1B\x03\x16`\x03`\0\x82\x82Ta\x11\x01\x91\x90a#\x98V[\x90\x91UPP`\x05Ta\x11'\x90`\x01`\x01`\xA0\x1B\x03\x1630`\x01`\x01`@\x1B\x03\x8D\x16a\x13\x08V[\x7F\x8C\"\xDC2\xA9\xEE;6$\xF3\xF9\xF4\xF9\xBE\x14\x8Dxb^\xCD\x89<5\xE6\x9A\xB2\xFF\x0E\x0B\xFF\xC00\x82\x85\x8A\x8C`@Qa\x11\\\x94\x93\x92\x91\x90a#\xB1V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPV[`\0`d\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\x11\x91WP`\n\x91\x90PV[P`\x05\x91\x90PV[`@\x80Qc*\xDD\xA1\xC1`\xE0\x1B\x81R\x82Q`\x04\x82\x01R` \x83\x01Q`$\x82\x01R\x90\x82\x01Q`D\x82\x01R``\x82\x01Q`d\x82\x01R`\0\x90\x81\x900\x90c*\xDD\xA1\xC1\x90`\x84\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x1C\x91\x90a#\xE8V[`@\x01Q\x93\x92PPPV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x12\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x0E\x9CV[PPPPV[`\x07\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19\x16`\x01`@\x1B`\x01`\x01`@\x1B\x03\x85\x16\x02\x17\x90Ua\x12\xDE\x81`\x01a#,V[`\x07`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPV[`\0`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x13\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x0E\x9CV[PPPPPV[a\x13\xAD\x82a\x14\x91V[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a' `$\x919\x90P`\0\x84\x82`@Q` \x01a\x13\xDF\x92\x91\x90a$\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\x13\xFB\x82a\x15.V[\x90Pa\x14\x18\x81\x85a\x14\x0B\x88a\x16 V[a\x14\x13a\x16\x9BV[a\x17lV[a\x144W`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\x06\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B`\x01`\x01`@\x1B\x03\x85\x16\x02\x17\x90Ua\x14l\x81`\x01a#,V[`\x07\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a'D\x839\x81Q\x91R\x91\x15\x90\x15\x16\x15a\x14\xB9WPPPV[\x82Q` \x84\x01Q\x82`\x03\x84\x85\x85\x86\t\x85\t\x08\x83\x82\x83\t\x14\x83\x82\x10\x84\x84\x10\x16\x16\x93PPP\x81a\x15)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0E\x9CV[PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0a\x15M\x83a\x18NV[\x90P`\0\x80Q` a'D\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\x15vWa\x15va$\xDDV[\x84\x82\t\x90P\x82\x80a\x15\x89Wa\x15\x89a$\xDDV[\x82\x82\x08\x90P`\0\x80a\x15\x9A\x83a\x1AaV[\x92P\x90P[\x80a\x16\x03W\x84\x80a\x15\xB2Wa\x15\xB2a$\xDDV[`\x01\x87\x08\x95P\x84\x80a\x15\xC6Wa\x15\xC6a$\xDDV[\x86\x87\t\x92P\x84\x80a\x15\xD9Wa\x15\xD9a$\xDDV[\x86\x84\t\x92P\x84\x80a\x15\xECWa\x15\xECa$\xDDV[\x84\x84\x08\x92Pa\x15\xFA\x83a\x1AaV[\x92P\x90Pa\x15\x9FV[P`@\x80Q\x80\x82\x01\x90\x91R\x94\x85R` \x85\x01RP\x91\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x16HWP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a'D\x839\x81Q\x91R\x84` \x01Qa\x16{\x91\x90a$\xF3V[a\x16\x93\x90`\0\x80Q` a'D\x839\x81Q\x91Ra%\x15V[\x90R\x92\x91PPV[a\x16\xC6`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R` \x87\x01Q`@\x82\x01R\x86Q``\x82\x01R``\x87\x01Q`\x80\x82\x01R`@\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R` \x85\x01Qa\x01\0\x82\x01R\x84Qa\x01 \x82\x01R``\x85\x01Qa\x01@\x82\x01R`@\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x18BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x0E\x9CV[P\x15\x15\x95\x94PPPPPV[`\0\x80a\x18Z\x83a\x1B_V[\x80Q\x90\x91P`0\x81\x14a\x18oWa\x18oa%(V[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x89Wa\x18\x89a\x1F\x1AV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x18\xB3W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x19$W\x83`\x01a\x18\xCE\x83\x86a%\x15V[a\x18\xD8\x91\x90a%\x15V[\x81Q\x81\x10a\x18\xE8Wa\x18\xE8a%>V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x19\x05Wa\x19\x05a%>V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x18\xB9V[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x19\xB6W\x83\x81a\x19b\x85\x88a%\x15V[a\x19l\x91\x90a#\x98V[\x81Q\x81\x10a\x19|Wa\x19|a%>V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x19\x9CWa\x19\x9Ca%>V[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x19NV[P`\0a\x19\xC2\x82a\x1E\xB2V[\x90Pa\x01\0`\0\x80Q` a'D\x839\x81Q\x91R`\0a\x19\xE2\x86\x89a%\x15V[\x90P`\0[\x81\x81\x10\x15a\x1AQW`\0\x88`\x01a\x19\xFE\x84\x86a%\x15V[a\x1A\x08\x91\x90a%\x15V[\x81Q\x81\x10a\x1A\x18Wa\x1A\x18a%>V[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x1A0Wa\x1A0a$\xDDV[\x85\x87\t\x95P\x83\x80a\x1ACWa\x1ACa$\xDDV[\x81\x87\x08\x95PP`\x01\x01a\x19\xE7V[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80`\0\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a'D\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x87``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x94PP`\0Q\x92P\x83a\x1B%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x0E\x9CV[\x80`\x01\x84\x90\x1B\x11\x15a\x1B>Wa\x1B;\x83\x82a%\x15V[\x92P[\x80\x80a\x1BLWa\x1BLa$\xDDV[\x83\x84\t\x96\x90\x96\x14\x96\x91\x95P\x90\x93PPPPV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x1B\xA0\x92\x91\x90a$\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x1B\xC7\x92\x91\x90a%TV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x1B\xE9\x91\x90a%\x80V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x1C\x13\x90\x83\x90\x83\x90` \x01a%\x9AV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\x83Wa\x1C\x83a\x1F\x1AV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1C\xADW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x1C\xC5\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x1D0W\x81\x81\x81Q\x81\x10a\x1C\xF4Wa\x1C\xF4a%>V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x1D\x11Wa\x1D\x11a%>V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x1C\xD9V[P`\0\x84`@Q` \x01a\x1DF\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x1D\xDAW`\0\x83\x82\x81Q\x81\x10a\x1D\x81Wa\x1D\x81a%>V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x1D\x9EWa\x1D\x9Ea%>V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x1D\xBF\x92\x91\x90a%\xBFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x98PP`\x01\x01a\x1DeV[P\x86\x88\x87`@Q` \x01a\x1D\xF0\x93\x92\x91\x90a%\xE4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x1E\x1E\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x1E?\x8A`\xFF\x8D\x16a%\x15V[\x81\x10\x15a\x1E\xA1W\x82\x81\x81Q\x81\x10a\x1EXWa\x1EXa%>V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x1Er\x83\x8Da#\x98V[\x81Q\x81\x10a\x1E\x82Wa\x1E\x82a%>V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x1E2V[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a\x1F\x13W\x83\x81\x81Q\x81\x10a\x1E\xD2Wa\x1E\xD2a%>V[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x1E\xEA\x91\x90a&\x18V[a\x1E\xF5\x90`\x02a'\x13V[a\x1E\xFF\x91\x90a&\x18V[a\x1F\t\x90\x83a#\x98V[\x91P`\x01\x01a\x1E\xB7V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1F`WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1F`WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\x80\x82\x84\x03\x12\x15a\x1F\xA8W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1F\xD8WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a \x1BW`\0\x80\xFD[a %\x83\x83a\x1F\x96V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a `WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Q`\xE0\x83\x01\x91a \x88\x90\x84\x01\x82a BV[P`@\x83\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x86\x01Q\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\xA0\x83\x01Qa\x1F\x13`\xA0\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a \xE9W`\0\x80\xFD[PV[`\0\x80`\xA0\x83\x85\x03\x12\x15a \xFFW`\0\x80\xFD[a!\t\x84\x84a\x1F\x96V[\x91P`\x80\x83\x015a!\x19\x81a \xD4V[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15a!6W`\0\x80\xFD[a!>a\x1F0V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\x02\x81\x10a \xE9W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x01`\x81\x12\x15a!|W`\0\x80\xFD[a!\x86\x89\x89a\x1F\x96V[\x96Pa!\x95\x89`\x80\x8A\x01a!$V[\x95P`\xC0\x88\x015a!\xA5\x81a \xD4V[\x94P`\xE0\x88\x015a!\xB5\x81a!TV[\x93P`@`\xFF\x19\x82\x01\x12\x15a!\xC9W`\0\x80\xFD[Pa!\xD2a\x1F0V[a\x01\0\x88\x015\x81Ra\x01 \x88\x015` \x82\x01R\x91Pa\x01@\x87\x015a!\xF6\x81a \xD4V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a \xE9W`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15a\"+W`\0\x80\xFD[a\"3a\x1FfV[\x825a\">\x81a\"\x04V[\x81R` \x83\x015a\"N\x81a!TV[` \x82\x01R`@\x83\x015a\"a\x81a \xD4V[`@\x82\x01R``\x83\x015a\"t\x81a \xD4V[``\x82\x01R`\x80\x83\x015a\"\x87\x81a \xD4V[`\x80\x82\x01Ra\"\x99\x84`\xA0\x85\x01a!$V[`\xA0\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\"\xB7W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x87\x16\x81R`\xE0\x81\x01a\"\xDB` \x83\x01\x88a BV[`\x01`\x01`@\x1B\x03\x86\x81\x16`@\x84\x01R\x85\x81\x16``\x84\x01R\x84\x16`\x80\x83\x01R\x82Q`\xA0\x83\x01R` \x83\x01Q`\xC0\x83\x01R\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x1F\x13Wa\x1F\x13a#\x16V[`\0\x80`@\x83\x85\x03\x12\x15a#_W`\0\x80\xFD[\x82Qa#j\x81a \xD4V[` \x84\x01Q\x90\x92Pa!\x19\x81a \xD4V[`\0` \x82\x84\x03\x12\x15a#\x8DW`\0\x80\xFD[\x81Qa %\x81a \xD4V[\x80\x82\x01\x80\x82\x11\x15a#\xABWa#\xABa#\x16V[\x92\x91PPV[\x84\x81R`\x01`\x01`@\x1B\x03\x84\x81\x16` \x83\x01R`\x80\x82\x01\x90a#\xD6`@\x84\x01\x86a BV[\x80\x84\x16``\x84\x01RP\x95\x94PPPPPV[`\0\x81\x83\x03`\xE0\x81\x12\x15a#\xFBW`\0\x80\xFD[a$\x03a\x1FfV[\x83Qa$\x0E\x81a\"\x04V[\x81R` \x84\x01Qa$\x1E\x81a!TV[` \x82\x01R`@\x84\x01Qa$1\x81a \xD4V[`@\x82\x01R``\x84\x01Qa$D\x81a \xD4V[``\x82\x01R`\x80\x84\x01Qa$W\x81a \xD4V[`\x80\x82\x01R`@`\x9F\x19\x83\x01\x12\x15a$nW`\0\x80\xFD[a$va\x1F0V[`\xA0\x85\x81\x01Q\x82R`\xC0\x90\x95\x01Q` \x82\x01R\x93\x81\x01\x93\x90\x93RP\x90\x92\x91PPV[`\0\x81Q`\0[\x81\x81\x10\x15a$\xB9W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a$\x9FV[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x05\x89a$\xD7\x83\x86a$\x98V[\x84a$\x98V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a%\x10WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x81\x81\x03\x81\x81\x11\x15a#\xABWa#\xABa#\x16V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0a%`\x82\x85a$\x98V[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a%\x8C\x82\x84a$\x98V[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a%\xA6\x82\x85a$\x98V[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a%\xCB\x82\x85a$\x98V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a%\xF0\x82\x86a$\x98V[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a#\xABWa#\xABa#\x16V[`\x01\x81\x81[\x80\x85\x11\x15a&jW\x81`\0\x19\x04\x82\x11\x15a&PWa&Pa#\x16V[\x80\x85\x16\x15a&]W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a&4V[P\x92P\x92\x90PV[`\0\x82a&\x81WP`\x01a#\xABV[\x81a&\x8EWP`\0a#\xABV[\x81`\x01\x81\x14a&\xA4W`\x02\x81\x14a&\xAEWa&\xCAV[`\x01\x91PPa#\xABV[`\xFF\x84\x11\x15a&\xBFWa&\xBFa#\x16V[PP`\x01\x82\x1Ba#\xABV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a&\xEDWP\x81\x81\na#\xABV[a&\xF7\x83\x83a&/V[\x80`\0\x19\x04\x82\x11\x15a'\x0BWa'\x0Ba#\x16V[\x02\x93\x92PPPV[`\0a %\x83\x83a&rV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA1dsolcC\0\x08\x17\0\n";
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
        ///Calls the contract's `_hashBlsKey` (0x9b30a5e6) function
        pub fn hash_bls_key(
            &self,
            bls_vk: G2Point,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([155, 48, 165, 230], (bls_vk,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentEpoch` (0x76671808) function
        pub fn current_epoch(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([118, 103, 24, 8], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `exitEscrowPeriod` (0xc84c7fa1) function
        pub fn exit_escrow_period(
            &self,
            node: Node,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([200, 76, 127, 161], (node,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `firstAvailableExitEpoch` (0x109e3be3) function
        pub fn first_available_exit_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([16, 158, 59, 227], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `firstAvailableRegistrationEpoch` (0x2c701269) function
        pub fn first_available_registration_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([44, 112, 18, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lightClient` (0xb5700e68) function
        pub fn light_client(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([181, 112, 14, 104], ())
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
        ///Calls the contract's `maxChurnRate` (0x21bfd515) function
        pub fn max_churn_rate(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([33, 191, 213, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nextExitEpoch` (0x3b09c267) function
        pub fn next_exit_epoch(&self) -> ::ethers::contract::builders::ContractCall<M, (u64, u64)> {
            self.0
                .method_hash([59, 9, 194, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nextRegistrationEpoch` (0x2c530584) function
        pub fn next_registration_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u64)> {
            self.0
                .method_hash([44, 83, 5, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nodes` (0xd86e697d) function
        pub fn nodes(
            &self,
            key_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                u8,
                u64,
                u64,
                u64,
                EdOnBN254Point,
            ),
        > {
            self.0
                .method_hash([216, 110, 105, 125], key_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numPendingExits` (0xd67b6ca5) function
        pub fn num_pending_exits(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([214, 123, 108, 165], ())
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
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 167, 194, 127], (bls_vk,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenAddress` (0x9d76ea58) function
        pub fn token_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([157, 118, 234, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalKeys` (0x488bdabc) function
        pub fn total_keys(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([72, 139, 218, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalNativeStake` (0x544c2d76) function
        pub fn total_native_stake(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 76, 45, 118], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalRestakedStake` (0x6e8e4e6a) function
        pub fn total_restaked_stake(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([110, 142, 78, 106], ())
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
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `Exit` event
        pub fn exit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ExitFilter> {
            self.0.event()
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeTableEvents> {
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
    ///Custom Error type `ExitRequestInProgress` with signature `ExitRequestInProgress()` and selector `0x37a83ed5`
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
    #[etherror(name = "ExitRequestInProgress", abi = "ExitRequestInProgress()")]
    pub struct ExitRequestInProgress;
    ///Custom Error type `InvalidNextRegistrationEpoch` with signature `InvalidNextRegistrationEpoch(uint64,uint64)` and selector `0x43bf1786`
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
    #[etherror(
        name = "InvalidNextRegistrationEpoch",
        abi = "InvalidNextRegistrationEpoch(uint64,uint64)"
    )]
    pub struct InvalidNextRegistrationEpoch(pub u64, pub u64);
    ///Custom Error type `NodeAlreadyRegistered` with signature `NodeAlreadyRegistered()` and selector `0x1d61a626`
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
    #[etherror(name = "NodeAlreadyRegistered", abi = "NodeAlreadyRegistered()")]
    pub struct NodeAlreadyRegistered;
    ///Custom Error type `PrematureDeposit` with signature `PrematureDeposit()` and selector `0x5316cbe6`
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
    #[etherror(name = "PrematureDeposit", abi = "PrematureDeposit()")]
    pub struct PrematureDeposit;
    ///Custom Error type `PrematureExit` with signature `PrematureExit()` and selector `0x787aeb53`
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
    #[etherror(name = "PrematureExit", abi = "PrematureExit()")]
    pub struct PrematureExit;
    ///Custom Error type `PrematureWithdrawal` with signature `PrematureWithdrawal()` and selector `0x5a774357`
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
    #[etherror(name = "PrematureWithdrawal", abi = "PrematureWithdrawal()")]
    pub struct PrematureWithdrawal;
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
    ///Custom Error type `Unauthenticated` with signature `Unauthenticated()` and selector `0xc8759c17`
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
    #[etherror(name = "Unauthenticated", abi = "Unauthenticated()")]
    pub struct Unauthenticated;
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
        ExitRequestInProgress(ExitRequestInProgress),
        InvalidNextRegistrationEpoch(InvalidNextRegistrationEpoch),
        NodeAlreadyRegistered(NodeAlreadyRegistered),
        PrematureDeposit(PrematureDeposit),
        PrematureExit(PrematureExit),
        PrematureWithdrawal(PrematureWithdrawal),
        RestakingNotImplemented(RestakingNotImplemented),
        Unauthenticated(Unauthenticated),
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
                <ExitRequestInProgress as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExitRequestInProgress(decoded));
            }
            if let Ok(decoded) =
                <InvalidNextRegistrationEpoch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidNextRegistrationEpoch(decoded));
            }
            if let Ok(decoded) =
                <NodeAlreadyRegistered as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NodeAlreadyRegistered(decoded));
            }
            if let Ok(decoded) = <PrematureDeposit as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrematureDeposit(decoded));
            }
            if let Ok(decoded) = <PrematureExit as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PrematureExit(decoded));
            }
            if let Ok(decoded) =
                <PrematureWithdrawal as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrematureWithdrawal(decoded));
            }
            if let Ok(decoded) =
                <RestakingNotImplemented as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RestakingNotImplemented(decoded));
            }
            if let Ok(decoded) = <Unauthenticated as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unauthenticated(decoded));
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
                Self::ExitRequestInProgress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidNextRegistrationEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NodeAlreadyRegistered(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrematureDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrematureExit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrematureWithdrawal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RestakingNotImplemented(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthenticated(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                    == <ExitRequestInProgress as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InvalidNextRegistrationEpoch as ::ethers::contract::EthError>::selector(
                    ) =>
                {
                    true
                }
                _ if selector
                    == <NodeAlreadyRegistered as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <PrematureDeposit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <PrematureExit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PrematureWithdrawal as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <RestakingNotImplemented as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <Unauthenticated as ::ethers::contract::EthError>::selector() => {
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
                Self::ExitRequestInProgress(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidNextRegistrationEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NodeAlreadyRegistered(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrematureDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrematureExit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrematureWithdrawal(element) => ::core::fmt::Display::fmt(element, f),
                Self::RestakingNotImplemented(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthenticated(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<ExitRequestInProgress> for StakeTableErrors {
        fn from(value: ExitRequestInProgress) -> Self {
            Self::ExitRequestInProgress(value)
        }
    }
    impl ::core::convert::From<InvalidNextRegistrationEpoch> for StakeTableErrors {
        fn from(value: InvalidNextRegistrationEpoch) -> Self {
            Self::InvalidNextRegistrationEpoch(value)
        }
    }
    impl ::core::convert::From<NodeAlreadyRegistered> for StakeTableErrors {
        fn from(value: NodeAlreadyRegistered) -> Self {
            Self::NodeAlreadyRegistered(value)
        }
    }
    impl ::core::convert::From<PrematureDeposit> for StakeTableErrors {
        fn from(value: PrematureDeposit) -> Self {
            Self::PrematureDeposit(value)
        }
    }
    impl ::core::convert::From<PrematureExit> for StakeTableErrors {
        fn from(value: PrematureExit) -> Self {
            Self::PrematureExit(value)
        }
    }
    impl ::core::convert::From<PrematureWithdrawal> for StakeTableErrors {
        fn from(value: PrematureWithdrawal) -> Self {
            Self::PrematureWithdrawal(value)
        }
    }
    impl ::core::convert::From<RestakingNotImplemented> for StakeTableErrors {
        fn from(value: RestakingNotImplemented) -> Self {
            Self::RestakingNotImplemented(value)
        }
    }
    impl ::core::convert::From<Unauthenticated> for StakeTableErrors {
        fn from(value: Unauthenticated) -> Self {
            Self::Unauthenticated(value)
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
    #[ethevent(name = "Deposit", abi = "Deposit(bytes32,uint256)")]
    pub struct DepositFilter {
        pub bls_v_khash: [u8; 32],
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "Exit", abi = "Exit(bytes32,uint64)")]
    pub struct ExitFilter {
        pub bls_v_khash: [u8; 32],
        pub exit_epoch: u64,
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
    pub struct RegisteredFilter {
        pub bls_v_khash: [u8; 32],
        pub register_epoch: u64,
        pub stake_type: u8,
        pub amount_deposited: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
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
    pub enum StakeTableEvents {
        DepositFilter(DepositFilter),
        ExitFilter(ExitFilter),
        RegisteredFilter(RegisteredFilter),
    }
    impl ::ethers::contract::EthLogDecode for StakeTableEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(StakeTableEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = ExitFilter::decode_log(log) {
                return Ok(StakeTableEvents::ExitFilter(decoded));
            }
            if let Ok(decoded) = RegisteredFilter::decode_log(log) {
                return Ok(StakeTableEvents::RegisteredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StakeTableEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExitFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositFilter> for StakeTableEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<ExitFilter> for StakeTableEvents {
        fn from(value: ExitFilter) -> Self {
            Self::ExitFilter(value)
        }
    }
    impl ::core::convert::From<RegisteredFilter> for StakeTableEvents {
        fn from(value: RegisteredFilter) -> Self {
            Self::RegisteredFilter(value)
        }
    }
    ///Container type for all input parameters for the `_hashBlsKey` function with signature `_hashBlsKey((uint256,uint256,uint256,uint256))` and selector `0x9b30a5e6`
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
        name = "_hashBlsKey",
        abi = "_hashBlsKey((uint256,uint256,uint256,uint256))"
    )]
    pub struct HashBlsKeyCall {
        pub bls_vk: G2Point,
    }
    ///Container type for all input parameters for the `currentEpoch` function with signature `currentEpoch()` and selector `0x76671808`
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
    #[ethcall(name = "currentEpoch", abi = "currentEpoch()")]
    pub struct CurrentEpochCall;
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
    ///Container type for all input parameters for the `exitEscrowPeriod` function with signature `exitEscrowPeriod((address,uint8,uint64,uint64,uint64,(uint256,uint256)))` and selector `0xc84c7fa1`
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
        name = "exitEscrowPeriod",
        abi = "exitEscrowPeriod((address,uint8,uint64,uint64,uint64,(uint256,uint256)))"
    )]
    pub struct ExitEscrowPeriodCall {
        pub node: Node,
    }
    ///Container type for all input parameters for the `firstAvailableExitEpoch` function with signature `firstAvailableExitEpoch()` and selector `0x109e3be3`
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
    #[ethcall(name = "firstAvailableExitEpoch", abi = "firstAvailableExitEpoch()")]
    pub struct FirstAvailableExitEpochCall;
    ///Container type for all input parameters for the `firstAvailableRegistrationEpoch` function with signature `firstAvailableRegistrationEpoch()` and selector `0x2c701269`
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
        name = "firstAvailableRegistrationEpoch",
        abi = "firstAvailableRegistrationEpoch()"
    )]
    pub struct FirstAvailableRegistrationEpochCall;
    ///Container type for all input parameters for the `lightClient` function with signature `lightClient()` and selector `0xb5700e68`
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
    #[ethcall(name = "lightClient", abi = "lightClient()")]
    pub struct LightClientCall;
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
    ///Container type for all input parameters for the `maxChurnRate` function with signature `maxChurnRate()` and selector `0x21bfd515`
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
    #[ethcall(name = "maxChurnRate", abi = "maxChurnRate()")]
    pub struct MaxChurnRateCall;
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
    ///Container type for all input parameters for the `nodes` function with signature `nodes(bytes32)` and selector `0xd86e697d`
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
    #[ethcall(name = "nodes", abi = "nodes(bytes32)")]
    pub struct NodesCall {
        pub key_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `numPendingExits` function with signature `numPendingExits()` and selector `0xd67b6ca5`
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
    #[ethcall(name = "numPendingExits", abi = "numPendingExits()")]
    pub struct NumPendingExitsCall;
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
    ///Container type for all input parameters for the `tokenAddress` function with signature `tokenAddress()` and selector `0x9d76ea58`
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
    #[ethcall(name = "tokenAddress", abi = "tokenAddress()")]
    pub struct TokenAddressCall;
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
    ///Container type for all input parameters for the `totalNativeStake` function with signature `totalNativeStake()` and selector `0x544c2d76`
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
    #[ethcall(name = "totalNativeStake", abi = "totalNativeStake()")]
    pub struct TotalNativeStakeCall;
    ///Container type for all input parameters for the `totalRestakedStake` function with signature `totalRestakedStake()` and selector `0x6e8e4e6a`
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
    #[ethcall(name = "totalRestakedStake", abi = "totalRestakedStake()")]
    pub struct TotalRestakedStakeCall;
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
        HashBlsKey(HashBlsKeyCall),
        CurrentEpoch(CurrentEpochCall),
        Deposit(DepositCall),
        ExitEscrowPeriod(ExitEscrowPeriodCall),
        FirstAvailableExitEpoch(FirstAvailableExitEpochCall),
        FirstAvailableRegistrationEpoch(FirstAvailableRegistrationEpochCall),
        LightClient(LightClientCall),
        LookupNode(LookupNodeCall),
        LookupStake(LookupStakeCall),
        MaxChurnRate(MaxChurnRateCall),
        NextExitEpoch(NextExitEpochCall),
        NextRegistrationEpoch(NextRegistrationEpochCall),
        Nodes(NodesCall),
        NumPendingExits(NumPendingExitsCall),
        NumPendingRegistrations(NumPendingRegistrationsCall),
        Register(RegisterCall),
        RequestExit(RequestExitCall),
        TokenAddress(TokenAddressCall),
        TotalKeys(TotalKeysCall),
        TotalNativeStake(TotalNativeStakeCall),
        TotalRestakedStake(TotalRestakedStakeCall),
        TotalStake(TotalStakeCall),
        TotalVotingStake(TotalVotingStakeCall),
        WithdrawFunds(WithdrawFundsCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakeTableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <HashBlsKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HashBlsKey(decoded));
            }
            if let Ok(decoded) = <CurrentEpochCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CurrentEpoch(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) =
                <ExitEscrowPeriodCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExitEscrowPeriod(decoded));
            }
            if let Ok(decoded) =
                <FirstAvailableExitEpochCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FirstAvailableExitEpoch(decoded));
            }
            if let Ok(decoded) =
                <FirstAvailableRegistrationEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::FirstAvailableRegistrationEpoch(decoded));
            }
            if let Ok(decoded) = <LightClientCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LightClient(decoded));
            }
            if let Ok(decoded) = <LookupNodeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LookupNode(decoded));
            }
            if let Ok(decoded) = <LookupStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LookupStake(decoded));
            }
            if let Ok(decoded) = <MaxChurnRateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxChurnRate(decoded));
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
            if let Ok(decoded) = <NodesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nodes(decoded));
            }
            if let Ok(decoded) =
                <NumPendingExitsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NumPendingExits(decoded));
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
            if let Ok(decoded) = <TokenAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TokenAddress(decoded));
            }
            if let Ok(decoded) = <TotalKeysCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalKeys(decoded));
            }
            if let Ok(decoded) =
                <TotalNativeStakeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalNativeStake(decoded));
            }
            if let Ok(decoded) =
                <TotalRestakedStakeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalRestakedStake(decoded));
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
                Self::HashBlsKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CurrentEpoch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExitEscrowPeriod(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FirstAvailableExitEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FirstAvailableRegistrationEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LightClient(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LookupNode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LookupStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxChurnRate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NextExitEpoch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NextRegistrationEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nodes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NumPendingExits(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NumPendingRegistrations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Register(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestExit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalKeys(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalNativeStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalRestakedStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalVotingStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawFunds(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for StakeTableCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::HashBlsKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExitEscrowPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::FirstAvailableExitEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::FirstAvailableRegistrationEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LightClient(element) => ::core::fmt::Display::fmt(element, f),
                Self::LookupNode(element) => ::core::fmt::Display::fmt(element, f),
                Self::LookupStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxChurnRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextExitEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextRegistrationEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nodes(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumPendingExits(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumPendingRegistrations(element) => ::core::fmt::Display::fmt(element, f),
                Self::Register(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestExit(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalKeys(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalNativeStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalRestakedStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalVotingStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFunds(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<HashBlsKeyCall> for StakeTableCalls {
        fn from(value: HashBlsKeyCall) -> Self {
            Self::HashBlsKey(value)
        }
    }
    impl ::core::convert::From<CurrentEpochCall> for StakeTableCalls {
        fn from(value: CurrentEpochCall) -> Self {
            Self::CurrentEpoch(value)
        }
    }
    impl ::core::convert::From<DepositCall> for StakeTableCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<ExitEscrowPeriodCall> for StakeTableCalls {
        fn from(value: ExitEscrowPeriodCall) -> Self {
            Self::ExitEscrowPeriod(value)
        }
    }
    impl ::core::convert::From<FirstAvailableExitEpochCall> for StakeTableCalls {
        fn from(value: FirstAvailableExitEpochCall) -> Self {
            Self::FirstAvailableExitEpoch(value)
        }
    }
    impl ::core::convert::From<FirstAvailableRegistrationEpochCall> for StakeTableCalls {
        fn from(value: FirstAvailableRegistrationEpochCall) -> Self {
            Self::FirstAvailableRegistrationEpoch(value)
        }
    }
    impl ::core::convert::From<LightClientCall> for StakeTableCalls {
        fn from(value: LightClientCall) -> Self {
            Self::LightClient(value)
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
    impl ::core::convert::From<MaxChurnRateCall> for StakeTableCalls {
        fn from(value: MaxChurnRateCall) -> Self {
            Self::MaxChurnRate(value)
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
    impl ::core::convert::From<NodesCall> for StakeTableCalls {
        fn from(value: NodesCall) -> Self {
            Self::Nodes(value)
        }
    }
    impl ::core::convert::From<NumPendingExitsCall> for StakeTableCalls {
        fn from(value: NumPendingExitsCall) -> Self {
            Self::NumPendingExits(value)
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
    impl ::core::convert::From<TokenAddressCall> for StakeTableCalls {
        fn from(value: TokenAddressCall) -> Self {
            Self::TokenAddress(value)
        }
    }
    impl ::core::convert::From<TotalKeysCall> for StakeTableCalls {
        fn from(value: TotalKeysCall) -> Self {
            Self::TotalKeys(value)
        }
    }
    impl ::core::convert::From<TotalNativeStakeCall> for StakeTableCalls {
        fn from(value: TotalNativeStakeCall) -> Self {
            Self::TotalNativeStake(value)
        }
    }
    impl ::core::convert::From<TotalRestakedStakeCall> for StakeTableCalls {
        fn from(value: TotalRestakedStakeCall) -> Self {
            Self::TotalRestakedStake(value)
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
    ///Container type for all return fields from the `_hashBlsKey` function with signature `_hashBlsKey((uint256,uint256,uint256,uint256))` and selector `0x9b30a5e6`
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
    pub struct HashBlsKeyReturn(pub [u8; 32]);
    ///Container type for all return fields from the `currentEpoch` function with signature `currentEpoch()` and selector `0x76671808`
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
    pub struct CurrentEpochReturn(pub u64);
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
    ///Container type for all return fields from the `exitEscrowPeriod` function with signature `exitEscrowPeriod((address,uint8,uint64,uint64,uint64,(uint256,uint256)))` and selector `0xc84c7fa1`
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
    pub struct ExitEscrowPeriodReturn(pub u64);
    ///Container type for all return fields from the `firstAvailableExitEpoch` function with signature `firstAvailableExitEpoch()` and selector `0x109e3be3`
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
    pub struct FirstAvailableExitEpochReturn(pub u64);
    ///Container type for all return fields from the `firstAvailableRegistrationEpoch` function with signature `firstAvailableRegistrationEpoch()` and selector `0x2c701269`
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
    pub struct FirstAvailableRegistrationEpochReturn(pub u64);
    ///Container type for all return fields from the `lightClient` function with signature `lightClient()` and selector `0xb5700e68`
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
    pub struct LightClientReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `maxChurnRate` function with signature `maxChurnRate()` and selector `0x21bfd515`
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
    pub struct MaxChurnRateReturn(pub u64);
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
    pub struct NextExitEpochReturn(pub u64, pub u64);
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
    pub struct NextRegistrationEpochReturn(pub u64, pub u64);
    ///Container type for all return fields from the `nodes` function with signature `nodes(bytes32)` and selector `0xd86e697d`
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
    pub struct NodesReturn {
        pub account: ::ethers::core::types::Address,
        pub stake_type: u8,
        pub balance: u64,
        pub register_epoch: u64,
        pub exit_epoch: u64,
        pub schnorr_vk: EdOnBN254Point,
    }
    ///Container type for all return fields from the `numPendingExits` function with signature `numPendingExits()` and selector `0xd67b6ca5`
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
    pub struct NumPendingExitsReturn(pub u64);
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
    ///Container type for all return fields from the `tokenAddress` function with signature `tokenAddress()` and selector `0x9d76ea58`
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
    pub struct TokenAddressReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `totalNativeStake` function with signature `totalNativeStake()` and selector `0x544c2d76`
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
    pub struct TotalNativeStakeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalRestakedStake` function with signature `totalRestakedStake()` and selector `0x6e8e4e6a`
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
    pub struct TotalRestakedStakeReturn(pub ::ethers::core::types::U256);
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
