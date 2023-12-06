pub use stake_table_mock::*;
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
pub mod stake_table_mock {
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
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MAX_CHURN_RATE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MAX_CHURN_RATE"),
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
                    ::std::borrow::ToOwned::to_owned("exitQueue"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("exitQueue"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("firstAvailableEpoch",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pendingRequests"),
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
                    ::std::borrow::ToOwned::to_owned("getQueueParameters"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getQueueParameters"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("queueType"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("enum StakeTable.QueueType",),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("nextEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nextEpoch"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("queueType"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("enum StakeTable.QueueType",),
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
                    ::std::borrow::ToOwned::to_owned("registrationQueue"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registrationQueue"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("firstAvailableEpoch",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pendingRequests"),
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
    pub static STAKETABLEMOCK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0)\xE58\x03\x80b\0)\xE5\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0\xC5V[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x06\x80T\x92\x90\x93\x16\x91\x16\x17\x90U`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01\x81\x90R`\x07\x80T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16\x90\x91U\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x92\x90\x91\x01R`\x08\x80T\x90\x91\x16\x90Ub\0\0\xFDV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xC0W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\xD9W`\0\x80\xFD[b\0\0\xE4\x83b\0\0\xA8V[\x91Pb\0\0\xF4` \x84\x01b\0\0\xA8V[\x90P\x92P\x92\x90PV[a(\xD8\x80b\0\x01\r`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80c\x8B\x0E\x9F?\x11a\0\xC3W\x80c\xC8L\x7F\xA1\x11a\0|W\x80c\xC8L\x7F\xA1\x14a\x03#W\x80c\xD8ni}\x14a\x036W\x80c\xDD.\xD3\xEC\x14a\x03\xC4W\x80c\xE6\xC7\x94\x84\x14a\x03\xD7W\x80c\xE9\x9Dp\x99\x14a\x03\xEAW\x80c\xFF\xEDK\xF5\x14a\x04\x08W`\0\x80\xFD[\x80c\x8B\x0E\x9F?\x14a\x02\x8CW\x80c\x9B0\xA5\xE6\x14a\x02\xA7W\x80c\x9Dv\xEAX\x14a\x02\xBAW\x80c\xB5p\x0Eh\x14a\x02\xE5W\x80c\xBF\x82H\xDD\x14a\x02\xF8W\x80c\xC7,\xC7\x17\x14a\x03\x10W`\0\x80\xFD[\x80cH\x8B\xDA\xBC\x11a\x01\x15W\x80cH\x8B\xDA\xBC\x14a\x02\x17W\x80cJ\xA7\xC2\x7F\x14a\x02<W\x80cTL-v\x14a\x02_W\x80cn\x8ENj\x14a\x02hW\x80cvg\x18\x08\x14a\x02qW\x80cw\x1FoD\x14a\x02yW`\0\x80\xFD[\x80c\x0C$\xAF\x18\x14a\x01]W\x80c\x10\x9A\xF5\x84\x14a\x01\x8DW\x80c\x16\xFE\xFE\xD7\x14a\x01\xC0W\x80c*\xDD\xA1\xC1\x14a\x01\xD8W\x80c9\xD0 \x9E\x14a\x01\xF8W\x80cC\x17\xD0\x0B\x14a\x02\x0EW[`\0\x80\xFD[a\x01pa\x01k6`\x04a!ZV[a\x04&V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xA0a\x01\x9B6`\x04a!\x8DV[a\x05\xAFV[`@\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x01\x84V[`\x07T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x01pV[a\x01\xEBa\x01\xE66`\x04a!ZV[a\x06\xA8V[`@Qa\x01\x84\x91\x90a!\xE2V[a\x02\0`\x14\x81V[`@Q\x90\x81R` \x01a\x01\x84V[a\x02\0`\x01T\x81V[`\0Ta\x02'\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x84V[a\x02Oa\x02J6`\x04a!ZV[a\x07\xBEV[`@Q\x90\x15\x15\x81R` \x01a\x01\x84V[a\x02\0`\x03T\x81V[a\x02\0`\x04T\x81V[a\x01pa\nsV[a\x01\xA0a\x02\x876`\x04a\"gV[a\n\xE6V[`\x03T`\x04T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x84V[a\x02\0a\x02\xB56`\x04a!ZV[a\r\xC8V[`\x05Ta\x02\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x84V[`\x06Ta\x02\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x08T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x01pV[a\x02Oa\x03\x1E6`\x04a\"\xCFV[a\x0E$V[a\x01pa\x0316`\x04a#fV[a\x11\x9DV[a\x03\xB2a\x03D6`\x04a#\xF2V[`\x02` \x81\x81R`\0\x92\x83R`@\x92\x83\x90 \x80T`\x01\x82\x01T\x85Q\x80\x87\x01\x90\x96R\x93\x82\x01T\x85R`\x03\x90\x91\x01T\x91\x84\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x81\x16\x92`\xFF`\x01`\xA0\x1B\x83\x04\x16\x92`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x93\x04\x83\x16\x92\x81\x81\x16\x92`\x01`@\x1B\x90\x92\x04\x16\x90\x86V[`@Qa\x01\x84\x96\x95\x94\x93\x92\x91\x90a$\x0BV[a\x01pa\x03\xD26`\x04a!ZV[a\x11\xC5V[a\x01pa\x03\xE56`\x04a!\x8DV[a\x12SV[`\x07Ta\x01\xA0\x90`\x01`\x01`@\x1B\x03\x80\x82\x16\x91`\x01`@\x1B\x90\x04\x16\x82V[`\x08Ta\x01\xA0\x90`\x01`\x01`@\x1B\x03\x80\x82\x16\x91`\x01`@\x1B\x90\x04\x16\x82V[`\0\x80a\x042\x83a\r\xC8V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x04\x80Wa\x04\x80a!\xAAV[`\x01\x81\x11\x15a\x04\x91Wa\x04\x91a!\xAAV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x90Pa\x04\xFC\x81a\x11\x9DV[\x81`\x80\x01Qa\x05\x0B\x91\x90a$yV[`\x01`\x01`@\x1B\x03\x16a\x05\x1Ca\nsV[`\x01`\x01`@\x1B\x03\x16\x10\x15a\x05DW`@QcZwCW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x81\x01Q`\x05T\x82Qa\x05k\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\x01`@\x1B\x03\x84\x16a\x12\x83V[`\0\x92\x83R`\x02` \x81\x90R`@\x84 \x80T`\x01`\x01`\xE8\x1B\x03\x19\x16\x81U`\x01\x81\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90U\x90\x81\x01\x84\x90U`\x03\x01\x92\x90\x92UP\x92\x91PPV[`\0\x80\x80\x80\x80\x85`\x01\x81\x11\x15a\x05\xC7Wa\x05\xC7a!\xAAV[\x03a\x067W0`\x01`\x01`\xA0\x1B\x03\x16c\xE9\x9Dp\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06-\x91\x90a$\x99V[\x90\x92P\x90Pa\x06\x9EV[0`\x01`\x01`\xA0\x1B\x03\x16c\xFF\xEDK\xF5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x98\x91\x90a$\x99V[\x90\x92P\x90P[\x90\x94\x90\x93P\x91PPV[a\x06\xED`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\xA0\x82\x01R\x90V[`\x02`\0a\x06\xFA\x84a\r\xC8V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x91\x92\x90\x91\x90\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x07GWa\x07Ga!\xAAV[`\x01\x81\x11\x15a\x07XWa\x07Xa!\xAAV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x92\x91PPV[`\0\x80a\x07\xCA\x83a\r\xC8V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x08\x18Wa\x08\x18a!\xAAV[`\x01\x81\x11\x15a\x08)Wa\x08)a!\xAAV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xB6W`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x15a\x08\xE2W`@Qc7\xA8>\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81\x01Qa\x08\xF2\x90`\x01a$yV[`\x01`\x01`@\x1B\x03\x16a\t\x03a\nsV[`\x01`\x01`@\x1B\x03\x16\x10\x15a\t+W`@Qcxz\xEBS`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t3a\x13\nV[`\x01`\x01`@\x1B\x03\x16`\x80\x82\x01R`\0\x82\x81R`\x02` \x90\x81R`@\x90\x91 \x82Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x83U\x92\x84\x01Q\x84\x93\x90\x91\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\t\x9DWa\t\x9Da!\xAAV[\x02\x17\x90UP`@\x82\x81\x01Q\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16`\x01`\xA8\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x90\x91\x16\x17\x83U``\x84\x01Q`\x01\x84\x01\x80T`\x80\x80\x88\x01Q\x85\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x92\x16\x93\x90\x94\x16\x92\x90\x92\x17\x91\x90\x91\x17\x90U`\xA0\x90\x93\x01Q\x80Q`\x02\x84\x01U` \x01Q`\x03\x90\x92\x01\x91\x90\x91U\x90\x82\x01Q\x90Q\x7F\x1D\xA0\xACf\xB9>\xA1\xC8\xD3%\x14$\xE6W(\xBC\xEEZ\x7F\xB7Y\xA9G\x15c\xEF\x8Ef\x82\xBB\xD7\x11\x91a\na\x91\x85\x91\x90\x91\x82R`\x01`\x01`@\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1P`\x01\x93\x92PPPV[`\x06T`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cvg\x18\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xE1\x91\x90a$\xC8V[\x90P\x90V[`\0\x80`\0a\n\xF4\x85a\r\xC8V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x0BBWa\x0BBa!\xAAV[`\x01\x81\x11\x15a\x0BSWa\x0BSa!\xAAV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xE0W`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0B\xEAa\nsV[\x90P\x81``\x01Q`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x11a\x0C\"W`@Qc)\x8Be\xF3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x82\x01Q`\x01`\x01`@\x1B\x03\x16\x15a\x0CNW`@Qc7\xA8>\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x82`@\x01\x81\x81Qa\x0C`\x91\x90a$yV[`\x01`\x01`@\x1B\x03\x16\x90RP`\0\x83\x81R`\x02` \x90\x81R`@\x90\x91 \x83Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x83U\x92\x85\x01Q\x85\x93\x90\x91\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\x0C\xC8Wa\x0C\xC8a!\xAAV[\x02\x17\x90UP`@\x82\x01Q\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17\x82U``\x83\x01Q`\x01\x83\x01\x80T`\x80\x86\x01Q\x92\x84\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17`\x01`@\x1B\x92\x84\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90U`\xA0\x90\x92\x01Q\x80Q`\x02\x83\x01U` \x01Q`\x03\x90\x91\x01U`\x05Ta\r]\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x903\x900\x90\x8A\x16a\x13\x16V[\x7F\x98\xE7\x83\xC3\x86K\xBFtJ\x05~\xF6\x05\xA2\xA6\x17\x01\xC3\xB6+^\xD6\x8B7E\xB9\x90\x94I}\xAF\x1Fa\r\x87\x88a\r\xC8V[`@\x80Q\x91\x82R`\x01`\x01`@\x1B\x03\x89\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1`\0a\r\xB3\x82`\x01a$yV[`@\x90\x93\x01Q\x95P\x91\x93PPPP\x92P\x92\x90PV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x0E\x07\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80\x84`\x01\x81\x11\x15a\x0E9Wa\x0E9a!\xAAV[\x14a\x0EVW`@Qb\x11\xD7\xFB`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x0E\x80\x81\x85\x8Aa\x13\xB2V[`\0a\x0E\x8Aa\x14dV[\x90P\x83`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0E\xD7W`@Qc!\xDF\x8B\xC3`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x80\x83\x16`\x04\x83\x01R\x85\x16`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x0E\xE2\x8Aa\r\xC8V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x0F0Wa\x0F0a!\xAAV[`\x01\x81\x11\x15a\x0FAWa\x0FAa!\xAAV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0F\xCDW`@Qc\x0E\xB0\xD3\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3\x81R`\x01`\x01`@\x1B\x03\x89\x16`@\x82\x01R` \x81\x01\x88`\x01\x81\x11\x15a\x0F\xF5Wa\x0F\xF5a!\xAAV[\x90\x81`\x01\x81\x11\x15a\x10\x08Wa\x10\x08a!\xAAV[\x90RP`\xA0\x81\x01\x8A\x90R`\x01`\x01`@\x1B\x03\x83\x16``\x82\x01R`\0\x82\x81R`\x02` \x90\x81R`@\x90\x91 \x82Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x83U\x92\x84\x01Q\x84\x93\x90\x91\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\x10}Wa\x10}a!\xAAV[\x02\x17\x90UP`@\x82\x01Q\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17\x82U``\x83\x01Q`\x01\x83\x01\x80T`\x80\x86\x01Q\x92\x84\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17`\x01`@\x1B\x92\x90\x93\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90U`\xA0\x90\x91\x01Q\x80Q`\x02\x83\x01U` \x01Q`\x03\x90\x91\x01U`\0\x88`\x01\x81\x11\x15a\x11\tWa\x11\ta!\xAAV[\x03a\x11OW\x88`\x01`\x01`@\x1B\x03\x16`\x03`\0\x82\x82Ta\x11)\x91\x90a$\xE5V[\x90\x91UPP`\x05Ta\x11O\x90`\x01`\x01`\xA0\x1B\x03\x1630`\x01`\x01`@\x1B\x03\x8D\x16a\x13\x16V[\x7F\x8C\"\xDC2\xA9\xEE;6$\xF3\xF9\xF4\xF9\xBE\x14\x8Dxb^\xCD\x89<5\xE6\x9A\xB2\xFF\x0E\x0B\xFF\xC00\x82\x84\x8A\x8C`@Qa\x11\x84\x94\x93\x92\x91\x90a$\xF8V[`@Q\x80\x91\x03\x90\xA1P`\x01\x9A\x99PPPPPPPPPPV[`\0`d\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\x11\xBDWP`\n\x91\x90PV[P`\x05\x91\x90PV[`@\x80Qc*\xDD\xA1\xC1`\xE0\x1B\x81R\x82Q`\x04\x82\x01R` \x83\x01Q`$\x82\x01R\x90\x82\x01Q`D\x82\x01R``\x82\x01Q`d\x82\x01R`\0\x90\x81\x900\x90c*\xDD\xA1\xC1\x90`\x84\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12H\x91\x90a%/V[`@\x01Q\x93\x92PPPV[`\0\x80\x82`\x01\x81\x11\x15a\x12hWa\x12ha!\xAAV[\x03a\x12{Wa\x12ua\x14dV[\x92\x91PPV[a\x12ua\x13\nV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x13\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x0E\xCEV[PPPPV[`\0a\n\xE1`\x01a\x14lV[`\0`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x13\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x0E\xCEV[PPPPPV[a\x13\xBB\x82a\x15\xCBV[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a(\x88`$\x919\x90P`\0\x84\x82`@Q` \x01a\x13\xED\x92\x91\x90a&\x0FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80a\x14\n\x83a\x16ZV[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91Pa\x14>\x81\x87a\x141\x8Aa\x17-V[a\x149a\x17\xA8V[a\x18yV[a\x14ZW`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0a\n\xE1`\0[`\0\x80\x80\x83`\x01\x81\x11\x15a\x14\x82Wa\x14\x82a!\xAAV[\x03a\x14\x8FWP`\x07a\x14\x93V[P`\x08[a\x14\x9Ba\nsV[a\x14\xA6\x90`\x01a$yV[\x81T`\x01`\x01`@\x1B\x03\x91\x82\x16\x91\x16\x10\x15a\x14\xF4Wa\x14\xC3a\nsV[a\x14\xCE\x90`\x01a$yV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x91\x90\x91\x16\x17`\x01`@\x1B\x17\x81Ua\x15\xBBV[\x80T`\x14`\x01`@\x1B\x90\x91\x04`\x01`\x01`@\x1B\x03\x16\x10a\x15qW\x80T`\x01\x90\x82\x90`\0\x90a\x15,\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a$yV[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x92\x83\x02\x91\x90\x92\x02\x19\x90\x91\x16\x17\x90UP\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1B\x17\x81Ua\x15\xBBV[\x80T`\x01\x90\x82\x90`\x08\x90a\x15\x96\x90\x84\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a$yV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP[T`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a(\xAC\x839\x81Q\x91R\x91\x82`\x03\x81\x80\x85\x80\t\x85\t\x08\x83\x82\x83\t\x14\x81\x15\x83\x15\x17\x19\x84\x83\x10\x85\x85\x10\x16\x16\x16\x93PPP\x81a\x16UW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0E\xCEV[PPPV[`\0\x80`\0a\x16h\x84a\x19[V[\x90P`\0\x80Q` a(\xAC\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\x16\x91Wa\x16\x91a&,V[\x84\x82\t\x90P\x82\x80a\x16\xA4Wa\x16\xA4a&,V[\x82\x82\x08\x90P`\0\x80a\x16\xB5\x83a\x1B\x8DV[\x92P\x90P[\x80a\x17\x1EW\x84\x80a\x16\xCDWa\x16\xCDa&,V[`\x01\x87\x08\x95P\x84\x80a\x16\xE1Wa\x16\xE1a&,V[\x86\x87\t\x92P\x84\x80a\x16\xF4Wa\x16\xF4a&,V[\x86\x84\t\x92P\x84\x80a\x17\x07Wa\x17\x07a&,V[\x84\x84\x08\x92Pa\x17\x15\x83a\x1B\x8DV[\x92P\x90Pa\x16\xBAV[P\x93\x97\x93\x96P\x92\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x17UWP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a(\xAC\x839\x81Q\x91R\x84` \x01Qa\x17\x88\x91\x90a&BV[a\x17\xA0\x90`\0\x80Q` a(\xAC\x839\x81Q\x91Ra&dV[\x90R\x92\x91PPV[a\x17\xD3`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R\x86Q`@\x82\x01R` \x87\x01Q``\x82\x01R`@\x87\x01Q`\x80\x82\x01R``\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R` \x85\x01Qa\x01 \x82\x01R`@\x85\x01Qa\x01@\x82\x01R``\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x19OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x0E\xCEV[P\x15\x15\x95\x94PPPPPV[`\0\x80a\x19g\x83a\x1C\x88V[\x80Q\x90\x91P`0\x81\x14a\x19|Wa\x19|a&wV[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\x96Wa\x19\x96a kV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x19\xC0W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x1A;W\x83`\x01a\x19\xDB\x83\x86a&dV[a\x19\xE5\x91\x90a&dV[\x81Q\x81\x10a\x19\xF5Wa\x19\xF5a&\x8DV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1A\x12Wa\x1A\x12a&\x8DV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x1A3\x81a&\xA3V[\x91PPa\x19\xC6V[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x1A\xD7W\x83\x81a\x1Ay\x85\x88a&dV[a\x1A\x83\x91\x90a$\xE5V[\x81Q\x81\x10a\x1A\x93Wa\x1A\x93a&\x8DV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x1A\xB3Wa\x1A\xB3a&\x8DV[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1A\xCF\x81a&\xA3V[\x91PPa\x1AeV[P`\0a\x1A\xE3\x82a\x1F\xF9V[\x90Pa\x01\0`\0\x80Q` a(\xAC\x839\x81Q\x91R`\0a\x1B\x03\x86\x89a&dV[\x90P`\0[\x81\x81\x10\x15a\x1B}W`\0\x88`\x01a\x1B\x1F\x84\x86a&dV[a\x1B)\x91\x90a&dV[\x81Q\x81\x10a\x1B9Wa\x1B9a&\x8DV[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x1BQWa\x1BQa&,V[\x85\x87\t\x95P\x83\x80a\x1BdWa\x1Bda&,V[\x81\x87\x08\x95PP\x80\x80a\x1Bu\x90a&\xA3V[\x91PPa\x1B\x08V[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a(\xAC\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x86``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x93PP`\0Q\x93P\x82a\x1COW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x0E\xCEV[\x80`\x01\x85\x90\x1B\x11\x15a\x1ChWa\x1Ce\x84\x82a&dV[\x93P[\x80\x80a\x1CvWa\x1Cva&,V[\x84\x85\t\x91P\x85\x82\x14\x94PPPP\x91P\x91V[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x1C\xC9\x92\x91\x90a&\x0FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x1C\xF0\x92\x91\x90a&\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x1D\x12\x91\x90a&\xE8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x1D<\x90\x83\x90\x83\x90` \x01a'\x02V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xACWa\x1D\xACa kV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1D\xD6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x1D\xEE\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x1EcW\x81\x81\x81Q\x81\x10a\x1E\x1DWa\x1E\x1Da&\x8DV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x1E:Wa\x1E:a&\x8DV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x1E[\x81a&\xA3V[\x91PPa\x1E\x02V[P`\0\x84`@Q` \x01a\x1Ey\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x1F\x17W`\0\x83\x82\x81Q\x81\x10a\x1E\xB4Wa\x1E\xB4a&\x8DV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x1E\xD1Wa\x1E\xD1a&\x8DV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x1E\xF2\x92\x91\x90a''V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x98PP\x80\x80a\x1F\x0F\x90a&\xA3V[\x91PPa\x1E\x98V[P\x86\x88\x87`@Q` \x01a\x1F-\x93\x92\x91\x90a'LV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x1F[\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x1F|\x8A`\xFF\x8D\x16a&dV[\x81\x10\x15a\x1F\xE8W\x82\x81\x81Q\x81\x10a\x1F\x95Wa\x1F\x95a&\x8DV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x1F\xAF\x83\x8Da$\xE5V[\x81Q\x81\x10a\x1F\xBFWa\x1F\xBFa&\x8DV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x1F\xE0\x81a&\xA3V[\x91PPa\x1FoV[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a dW\x83\x81\x81Q\x81\x10a \x19Wa \x19a&\x8DV[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a 1\x91\x90a'\x80V[a <\x90`\x02a({V[a F\x91\x90a'\x80V[a P\x90\x83a$\xE5V[\x91P\x80a \\\x81a&\xA3V[\x91PPa\x1F\xFEV[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a \xB1WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a \xB1WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\x80\x82\x84\x03\x12\x15a \xF9W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a!)WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a!lW`\0\x80\xFD[a!v\x83\x83a \xE7V[\x93\x92PPPV[`\x02\x81\x10a!\x8AW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a!\x9FW`\0\x80\xFD[\x815a!v\x81a!}V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a!\xDEWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Q`\xE0\x83\x01\x91a\"\x06\x90\x84\x01\x82a!\xC0V[P`@\x83\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x86\x01Q\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\xA0\x83\x01Qa d`\xA0\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a!\x8AW`\0\x80\xFD[`\0\x80`\xA0\x83\x85\x03\x12\x15a\"zW`\0\x80\xFD[a\"\x84\x84\x84a \xE7V[\x91P`\x80\x83\x015a\"\x94\x81a\"RV[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15a\"\xB1W`\0\x80\xFD[a\"\xB9a \x81V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15a\"\xE9W`\0\x80\xFD[a\"\xF3\x88\x88a \xE7V[\x95Pa#\x02\x88`\x80\x89\x01a\"\x9FV[\x94P`\xC0\x87\x015a#\x12\x81a\"RV[\x93P`\xE0\x87\x015a#\"\x81a!}V[\x92Pa#2\x88a\x01\0\x89\x01a\"\x9FV[\x91Pa\x01@\x87\x015a#C\x81a\"RV[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\x8AW`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15a#xW`\0\x80\xFD[a#\x80a \xB7V[\x825a#\x8B\x81a#QV[\x81R` \x83\x015a#\x9B\x81a!}V[` \x82\x01R`@\x83\x015a#\xAE\x81a\"RV[`@\x82\x01R``\x83\x015a#\xC1\x81a\"RV[``\x82\x01R`\x80\x83\x015a#\xD4\x81a\"RV[`\x80\x82\x01Ra#\xE6\x84`\xA0\x85\x01a\"\x9FV[`\xA0\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a$\x04W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x87\x16\x81R`\xE0\x81\x01a$(` \x83\x01\x88a!\xC0V[`\x01`\x01`@\x1B\x03\x86\x81\x16`@\x84\x01R\x85\x81\x16``\x84\x01R\x84\x16`\x80\x83\x01R\x82Q`\xA0\x83\x01R` \x83\x01Q`\xC0\x83\x01R\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a dWa da$cV[`\0\x80`@\x83\x85\x03\x12\x15a$\xACW`\0\x80\xFD[\x82Qa$\xB7\x81a\"RV[` \x84\x01Q\x90\x92Pa\"\x94\x81a\"RV[`\0` \x82\x84\x03\x12\x15a$\xDAW`\0\x80\xFD[\x81Qa!v\x81a\"RV[\x80\x82\x01\x80\x82\x11\x15a\x12uWa\x12ua$cV[\x84\x81R`\x01`\x01`@\x1B\x03\x84\x81\x16` \x83\x01R`\x80\x82\x01\x90a%\x1D`@\x84\x01\x86a!\xC0V[\x80\x84\x16``\x84\x01RP\x95\x94PPPPPV[`\0\x81\x83\x03`\xE0\x81\x12\x15a%BW`\0\x80\xFD[a%Ja \xB7V[\x83Qa%U\x81a#QV[\x81R` \x84\x01Qa%e\x81a!}V[` \x82\x01R`@\x84\x01Qa%x\x81a\"RV[`@\x82\x01R``\x84\x01Qa%\x8B\x81a\"RV[``\x82\x01R`\x80\x84\x01Qa%\x9E\x81a\"RV[`\x80\x82\x01R`@`\x9F\x19\x83\x01\x12\x15a%\xB5W`\0\x80\xFD[a%\xBDa \x81V[`\xA0\x85\x81\x01Q\x82R`\xC0\x90\x95\x01Q` \x82\x01R\x93\x81\x01\x93\x90\x93RP\x90\x92\x91PPV[`\0\x81Q`\0[\x81\x81\x10\x15a&\0W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a%\xE6V[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a&$a&\x1E\x83\x86a%\xDFV[\x84a%\xDFV[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a&_WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x81\x81\x03\x81\x81\x11\x15a\x12uWa\x12ua$cV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a&\xB5Wa&\xB5a$cV[P`\x01\x01\x90V[`\0a&\xC8\x82\x85a%\xDFV[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a&\xF4\x82\x84a%\xDFV[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a'\x0E\x82\x85a%\xDFV[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a'3\x82\x85a%\xDFV[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a'X\x82\x86a%\xDFV[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x12uWa\x12ua$cV[`\x01\x81\x81[\x80\x85\x11\x15a'\xD2W\x81`\0\x19\x04\x82\x11\x15a'\xB8Wa'\xB8a$cV[\x80\x85\x16\x15a'\xC5W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a'\x9CV[P\x92P\x92\x90PV[`\0\x82a'\xE9WP`\x01a\x12uV[\x81a'\xF6WP`\0a\x12uV[\x81`\x01\x81\x14a(\x0CW`\x02\x81\x14a(\x16Wa(2V[`\x01\x91PPa\x12uV[`\xFF\x84\x11\x15a('Wa('a$cV[PP`\x01\x82\x1Ba\x12uV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a(UWP\x81\x81\na\x12uV[a(_\x83\x83a'\x97V[\x80`\0\x19\x04\x82\x11\x15a(sWa(sa$cV[\x02\x93\x92PPPV[`\0a!v\x83\x83a'\xDAV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA1dsolcC\0\x08\x14\0\n";
    /// The bytecode of the contract.
    pub static STAKETABLEMOCK_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80c\x8B\x0E\x9F?\x11a\0\xC3W\x80c\xC8L\x7F\xA1\x11a\0|W\x80c\xC8L\x7F\xA1\x14a\x03#W\x80c\xD8ni}\x14a\x036W\x80c\xDD.\xD3\xEC\x14a\x03\xC4W\x80c\xE6\xC7\x94\x84\x14a\x03\xD7W\x80c\xE9\x9Dp\x99\x14a\x03\xEAW\x80c\xFF\xEDK\xF5\x14a\x04\x08W`\0\x80\xFD[\x80c\x8B\x0E\x9F?\x14a\x02\x8CW\x80c\x9B0\xA5\xE6\x14a\x02\xA7W\x80c\x9Dv\xEAX\x14a\x02\xBAW\x80c\xB5p\x0Eh\x14a\x02\xE5W\x80c\xBF\x82H\xDD\x14a\x02\xF8W\x80c\xC7,\xC7\x17\x14a\x03\x10W`\0\x80\xFD[\x80cH\x8B\xDA\xBC\x11a\x01\x15W\x80cH\x8B\xDA\xBC\x14a\x02\x17W\x80cJ\xA7\xC2\x7F\x14a\x02<W\x80cTL-v\x14a\x02_W\x80cn\x8ENj\x14a\x02hW\x80cvg\x18\x08\x14a\x02qW\x80cw\x1FoD\x14a\x02yW`\0\x80\xFD[\x80c\x0C$\xAF\x18\x14a\x01]W\x80c\x10\x9A\xF5\x84\x14a\x01\x8DW\x80c\x16\xFE\xFE\xD7\x14a\x01\xC0W\x80c*\xDD\xA1\xC1\x14a\x01\xD8W\x80c9\xD0 \x9E\x14a\x01\xF8W\x80cC\x17\xD0\x0B\x14a\x02\x0EW[`\0\x80\xFD[a\x01pa\x01k6`\x04a!ZV[a\x04&V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xA0a\x01\x9B6`\x04a!\x8DV[a\x05\xAFV[`@\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x01\x84V[`\x07T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x01pV[a\x01\xEBa\x01\xE66`\x04a!ZV[a\x06\xA8V[`@Qa\x01\x84\x91\x90a!\xE2V[a\x02\0`\x14\x81V[`@Q\x90\x81R` \x01a\x01\x84V[a\x02\0`\x01T\x81V[`\0Ta\x02'\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x84V[a\x02Oa\x02J6`\x04a!ZV[a\x07\xBEV[`@Q\x90\x15\x15\x81R` \x01a\x01\x84V[a\x02\0`\x03T\x81V[a\x02\0`\x04T\x81V[a\x01pa\nsV[a\x01\xA0a\x02\x876`\x04a\"gV[a\n\xE6V[`\x03T`\x04T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x84V[a\x02\0a\x02\xB56`\x04a!ZV[a\r\xC8V[`\x05Ta\x02\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x84V[`\x06Ta\x02\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x08T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x01pV[a\x02Oa\x03\x1E6`\x04a\"\xCFV[a\x0E$V[a\x01pa\x0316`\x04a#fV[a\x11\x9DV[a\x03\xB2a\x03D6`\x04a#\xF2V[`\x02` \x81\x81R`\0\x92\x83R`@\x92\x83\x90 \x80T`\x01\x82\x01T\x85Q\x80\x87\x01\x90\x96R\x93\x82\x01T\x85R`\x03\x90\x91\x01T\x91\x84\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x81\x16\x92`\xFF`\x01`\xA0\x1B\x83\x04\x16\x92`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x93\x04\x83\x16\x92\x81\x81\x16\x92`\x01`@\x1B\x90\x92\x04\x16\x90\x86V[`@Qa\x01\x84\x96\x95\x94\x93\x92\x91\x90a$\x0BV[a\x01pa\x03\xD26`\x04a!ZV[a\x11\xC5V[a\x01pa\x03\xE56`\x04a!\x8DV[a\x12SV[`\x07Ta\x01\xA0\x90`\x01`\x01`@\x1B\x03\x80\x82\x16\x91`\x01`@\x1B\x90\x04\x16\x82V[`\x08Ta\x01\xA0\x90`\x01`\x01`@\x1B\x03\x80\x82\x16\x91`\x01`@\x1B\x90\x04\x16\x82V[`\0\x80a\x042\x83a\r\xC8V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x04\x80Wa\x04\x80a!\xAAV[`\x01\x81\x11\x15a\x04\x91Wa\x04\x91a!\xAAV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x90Pa\x04\xFC\x81a\x11\x9DV[\x81`\x80\x01Qa\x05\x0B\x91\x90a$yV[`\x01`\x01`@\x1B\x03\x16a\x05\x1Ca\nsV[`\x01`\x01`@\x1B\x03\x16\x10\x15a\x05DW`@QcZwCW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x81\x01Q`\x05T\x82Qa\x05k\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\x01`@\x1B\x03\x84\x16a\x12\x83V[`\0\x92\x83R`\x02` \x81\x90R`@\x84 \x80T`\x01`\x01`\xE8\x1B\x03\x19\x16\x81U`\x01\x81\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90U\x90\x81\x01\x84\x90U`\x03\x01\x92\x90\x92UP\x92\x91PPV[`\0\x80\x80\x80\x80\x85`\x01\x81\x11\x15a\x05\xC7Wa\x05\xC7a!\xAAV[\x03a\x067W0`\x01`\x01`\xA0\x1B\x03\x16c\xE9\x9Dp\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06-\x91\x90a$\x99V[\x90\x92P\x90Pa\x06\x9EV[0`\x01`\x01`\xA0\x1B\x03\x16c\xFF\xEDK\xF5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x98\x91\x90a$\x99V[\x90\x92P\x90P[\x90\x94\x90\x93P\x91PPV[a\x06\xED`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\xA0\x82\x01R\x90V[`\x02`\0a\x06\xFA\x84a\r\xC8V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x91\x92\x90\x91\x90\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x07GWa\x07Ga!\xAAV[`\x01\x81\x11\x15a\x07XWa\x07Xa!\xAAV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x92\x91PPV[`\0\x80a\x07\xCA\x83a\r\xC8V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x08\x18Wa\x08\x18a!\xAAV[`\x01\x81\x11\x15a\x08)Wa\x08)a!\xAAV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xB6W`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x15a\x08\xE2W`@Qc7\xA8>\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81\x01Qa\x08\xF2\x90`\x01a$yV[`\x01`\x01`@\x1B\x03\x16a\t\x03a\nsV[`\x01`\x01`@\x1B\x03\x16\x10\x15a\t+W`@Qcxz\xEBS`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t3a\x13\nV[`\x01`\x01`@\x1B\x03\x16`\x80\x82\x01R`\0\x82\x81R`\x02` \x90\x81R`@\x90\x91 \x82Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x83U\x92\x84\x01Q\x84\x93\x90\x91\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\t\x9DWa\t\x9Da!\xAAV[\x02\x17\x90UP`@\x82\x81\x01Q\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16`\x01`\xA8\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x90\x91\x16\x17\x83U``\x84\x01Q`\x01\x84\x01\x80T`\x80\x80\x88\x01Q\x85\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x92\x16\x93\x90\x94\x16\x92\x90\x92\x17\x91\x90\x91\x17\x90U`\xA0\x90\x93\x01Q\x80Q`\x02\x84\x01U` \x01Q`\x03\x90\x92\x01\x91\x90\x91U\x90\x82\x01Q\x90Q\x7F\x1D\xA0\xACf\xB9>\xA1\xC8\xD3%\x14$\xE6W(\xBC\xEEZ\x7F\xB7Y\xA9G\x15c\xEF\x8Ef\x82\xBB\xD7\x11\x91a\na\x91\x85\x91\x90\x91\x82R`\x01`\x01`@\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1P`\x01\x93\x92PPPV[`\x06T`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cvg\x18\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xE1\x91\x90a$\xC8V[\x90P\x90V[`\0\x80`\0a\n\xF4\x85a\r\xC8V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x0BBWa\x0BBa!\xAAV[`\x01\x81\x11\x15a\x0BSWa\x0BSa!\xAAV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xE0W`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0B\xEAa\nsV[\x90P\x81``\x01Q`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x11a\x0C\"W`@Qc)\x8Be\xF3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x82\x01Q`\x01`\x01`@\x1B\x03\x16\x15a\x0CNW`@Qc7\xA8>\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x82`@\x01\x81\x81Qa\x0C`\x91\x90a$yV[`\x01`\x01`@\x1B\x03\x16\x90RP`\0\x83\x81R`\x02` \x90\x81R`@\x90\x91 \x83Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x83U\x92\x85\x01Q\x85\x93\x90\x91\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\x0C\xC8Wa\x0C\xC8a!\xAAV[\x02\x17\x90UP`@\x82\x01Q\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17\x82U``\x83\x01Q`\x01\x83\x01\x80T`\x80\x86\x01Q\x92\x84\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17`\x01`@\x1B\x92\x84\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90U`\xA0\x90\x92\x01Q\x80Q`\x02\x83\x01U` \x01Q`\x03\x90\x91\x01U`\x05Ta\r]\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x903\x900\x90\x8A\x16a\x13\x16V[\x7F\x98\xE7\x83\xC3\x86K\xBFtJ\x05~\xF6\x05\xA2\xA6\x17\x01\xC3\xB6+^\xD6\x8B7E\xB9\x90\x94I}\xAF\x1Fa\r\x87\x88a\r\xC8V[`@\x80Q\x91\x82R`\x01`\x01`@\x1B\x03\x89\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1`\0a\r\xB3\x82`\x01a$yV[`@\x90\x93\x01Q\x95P\x91\x93PPPP\x92P\x92\x90PV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x0E\x07\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80\x84`\x01\x81\x11\x15a\x0E9Wa\x0E9a!\xAAV[\x14a\x0EVW`@Qb\x11\xD7\xFB`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x0E\x80\x81\x85\x8Aa\x13\xB2V[`\0a\x0E\x8Aa\x14dV[\x90P\x83`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0E\xD7W`@Qc!\xDF\x8B\xC3`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x80\x83\x16`\x04\x83\x01R\x85\x16`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x0E\xE2\x8Aa\r\xC8V[`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x81Q`\xC0\x81\x01\x90\x92R\x80T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x0F0Wa\x0F0a!\xAAV[`\x01\x81\x11\x15a\x0FAWa\x0FAa!\xAAV[\x81R\x81T`\x01`\x01`@\x1B\x03`\x01`\xA8\x1B\x90\x91\x04\x81\x16` \x80\x84\x01\x91\x90\x91R`\x01\x84\x01T\x80\x83\x16`@\x80\x86\x01\x91\x90\x91R`\x01`@\x1B\x90\x91\x04\x90\x92\x16``\x84\x01R\x81Q\x80\x83\x01\x90\x92R`\x02\x84\x01T\x82R`\x03\x90\x93\x01T\x92\x81\x01\x92\x90\x92R`\x80\x01R\x80Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0F\xCDW`@Qc\x0E\xB0\xD3\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3\x81R`\x01`\x01`@\x1B\x03\x89\x16`@\x82\x01R` \x81\x01\x88`\x01\x81\x11\x15a\x0F\xF5Wa\x0F\xF5a!\xAAV[\x90\x81`\x01\x81\x11\x15a\x10\x08Wa\x10\x08a!\xAAV[\x90RP`\xA0\x81\x01\x8A\x90R`\x01`\x01`@\x1B\x03\x83\x16``\x82\x01R`\0\x82\x81R`\x02` \x90\x81R`@\x90\x91 \x82Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x83U\x92\x84\x01Q\x84\x93\x90\x91\x83\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x01\x81\x11\x15a\x10}Wa\x10}a!\xAAV[\x02\x17\x90UP`@\x82\x01Q\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17\x82U``\x83\x01Q`\x01\x83\x01\x80T`\x80\x86\x01Q\x92\x84\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17`\x01`@\x1B\x92\x90\x93\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90U`\xA0\x90\x91\x01Q\x80Q`\x02\x83\x01U` \x01Q`\x03\x90\x91\x01U`\0\x88`\x01\x81\x11\x15a\x11\tWa\x11\ta!\xAAV[\x03a\x11OW\x88`\x01`\x01`@\x1B\x03\x16`\x03`\0\x82\x82Ta\x11)\x91\x90a$\xE5V[\x90\x91UPP`\x05Ta\x11O\x90`\x01`\x01`\xA0\x1B\x03\x1630`\x01`\x01`@\x1B\x03\x8D\x16a\x13\x16V[\x7F\x8C\"\xDC2\xA9\xEE;6$\xF3\xF9\xF4\xF9\xBE\x14\x8Dxb^\xCD\x89<5\xE6\x9A\xB2\xFF\x0E\x0B\xFF\xC00\x82\x84\x8A\x8C`@Qa\x11\x84\x94\x93\x92\x91\x90a$\xF8V[`@Q\x80\x91\x03\x90\xA1P`\x01\x9A\x99PPPPPPPPPPV[`\0`d\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\x11\xBDWP`\n\x91\x90PV[P`\x05\x91\x90PV[`@\x80Qc*\xDD\xA1\xC1`\xE0\x1B\x81R\x82Q`\x04\x82\x01R` \x83\x01Q`$\x82\x01R\x90\x82\x01Q`D\x82\x01R``\x82\x01Q`d\x82\x01R`\0\x90\x81\x900\x90c*\xDD\xA1\xC1\x90`\x84\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12H\x91\x90a%/V[`@\x01Q\x93\x92PPPV[`\0\x80\x82`\x01\x81\x11\x15a\x12hWa\x12ha!\xAAV[\x03a\x12{Wa\x12ua\x14dV[\x92\x91PPV[a\x12ua\x13\nV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x13\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x0E\xCEV[PPPPV[`\0a\n\xE1`\x01a\x14lV[`\0`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x13\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x0E\xCEV[PPPPPV[a\x13\xBB\x82a\x15\xCBV[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a(\x88`$\x919\x90P`\0\x84\x82`@Q` \x01a\x13\xED\x92\x91\x90a&\x0FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80a\x14\n\x83a\x16ZV[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91Pa\x14>\x81\x87a\x141\x8Aa\x17-V[a\x149a\x17\xA8V[a\x18yV[a\x14ZW`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0a\n\xE1`\0[`\0\x80\x80\x83`\x01\x81\x11\x15a\x14\x82Wa\x14\x82a!\xAAV[\x03a\x14\x8FWP`\x07a\x14\x93V[P`\x08[a\x14\x9Ba\nsV[a\x14\xA6\x90`\x01a$yV[\x81T`\x01`\x01`@\x1B\x03\x91\x82\x16\x91\x16\x10\x15a\x14\xF4Wa\x14\xC3a\nsV[a\x14\xCE\x90`\x01a$yV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x91\x90\x91\x16\x17`\x01`@\x1B\x17\x81Ua\x15\xBBV[\x80T`\x14`\x01`@\x1B\x90\x91\x04`\x01`\x01`@\x1B\x03\x16\x10a\x15qW\x80T`\x01\x90\x82\x90`\0\x90a\x15,\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a$yV[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x92\x83\x02\x91\x90\x92\x02\x19\x90\x91\x16\x17\x90UP\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1B\x17\x81Ua\x15\xBBV[\x80T`\x01\x90\x82\x90`\x08\x90a\x15\x96\x90\x84\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a$yV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP[T`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a(\xAC\x839\x81Q\x91R\x91\x82`\x03\x81\x80\x85\x80\t\x85\t\x08\x83\x82\x83\t\x14\x81\x15\x83\x15\x17\x19\x84\x83\x10\x85\x85\x10\x16\x16\x16\x93PPP\x81a\x16UW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0E\xCEV[PPPV[`\0\x80`\0a\x16h\x84a\x19[V[\x90P`\0\x80Q` a(\xAC\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\x16\x91Wa\x16\x91a&,V[\x84\x82\t\x90P\x82\x80a\x16\xA4Wa\x16\xA4a&,V[\x82\x82\x08\x90P`\0\x80a\x16\xB5\x83a\x1B\x8DV[\x92P\x90P[\x80a\x17\x1EW\x84\x80a\x16\xCDWa\x16\xCDa&,V[`\x01\x87\x08\x95P\x84\x80a\x16\xE1Wa\x16\xE1a&,V[\x86\x87\t\x92P\x84\x80a\x16\xF4Wa\x16\xF4a&,V[\x86\x84\t\x92P\x84\x80a\x17\x07Wa\x17\x07a&,V[\x84\x84\x08\x92Pa\x17\x15\x83a\x1B\x8DV[\x92P\x90Pa\x16\xBAV[P\x93\x97\x93\x96P\x92\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x17UWP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a(\xAC\x839\x81Q\x91R\x84` \x01Qa\x17\x88\x91\x90a&BV[a\x17\xA0\x90`\0\x80Q` a(\xAC\x839\x81Q\x91Ra&dV[\x90R\x92\x91PPV[a\x17\xD3`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R\x86Q`@\x82\x01R` \x87\x01Q``\x82\x01R`@\x87\x01Q`\x80\x82\x01R``\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R` \x85\x01Qa\x01 \x82\x01R`@\x85\x01Qa\x01@\x82\x01R``\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x19OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x0E\xCEV[P\x15\x15\x95\x94PPPPPV[`\0\x80a\x19g\x83a\x1C\x88V[\x80Q\x90\x91P`0\x81\x14a\x19|Wa\x19|a&wV[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\x96Wa\x19\x96a kV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x19\xC0W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x1A;W\x83`\x01a\x19\xDB\x83\x86a&dV[a\x19\xE5\x91\x90a&dV[\x81Q\x81\x10a\x19\xF5Wa\x19\xF5a&\x8DV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1A\x12Wa\x1A\x12a&\x8DV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x1A3\x81a&\xA3V[\x91PPa\x19\xC6V[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x1A\xD7W\x83\x81a\x1Ay\x85\x88a&dV[a\x1A\x83\x91\x90a$\xE5V[\x81Q\x81\x10a\x1A\x93Wa\x1A\x93a&\x8DV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x1A\xB3Wa\x1A\xB3a&\x8DV[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1A\xCF\x81a&\xA3V[\x91PPa\x1AeV[P`\0a\x1A\xE3\x82a\x1F\xF9V[\x90Pa\x01\0`\0\x80Q` a(\xAC\x839\x81Q\x91R`\0a\x1B\x03\x86\x89a&dV[\x90P`\0[\x81\x81\x10\x15a\x1B}W`\0\x88`\x01a\x1B\x1F\x84\x86a&dV[a\x1B)\x91\x90a&dV[\x81Q\x81\x10a\x1B9Wa\x1B9a&\x8DV[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x1BQWa\x1BQa&,V[\x85\x87\t\x95P\x83\x80a\x1BdWa\x1Bda&,V[\x81\x87\x08\x95PP\x80\x80a\x1Bu\x90a&\xA3V[\x91PPa\x1B\x08V[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a(\xAC\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x86``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x93PP`\0Q\x93P\x82a\x1COW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x0E\xCEV[\x80`\x01\x85\x90\x1B\x11\x15a\x1ChWa\x1Ce\x84\x82a&dV[\x93P[\x80\x80a\x1CvWa\x1Cva&,V[\x84\x85\t\x91P\x85\x82\x14\x94PPPP\x91P\x91V[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x1C\xC9\x92\x91\x90a&\x0FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x1C\xF0\x92\x91\x90a&\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x1D\x12\x91\x90a&\xE8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x1D<\x90\x83\x90\x83\x90` \x01a'\x02V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xACWa\x1D\xACa kV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1D\xD6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x1D\xEE\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x1EcW\x81\x81\x81Q\x81\x10a\x1E\x1DWa\x1E\x1Da&\x8DV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x1E:Wa\x1E:a&\x8DV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x1E[\x81a&\xA3V[\x91PPa\x1E\x02V[P`\0\x84`@Q` \x01a\x1Ey\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x1F\x17W`\0\x83\x82\x81Q\x81\x10a\x1E\xB4Wa\x1E\xB4a&\x8DV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x1E\xD1Wa\x1E\xD1a&\x8DV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x1E\xF2\x92\x91\x90a''V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x98PP\x80\x80a\x1F\x0F\x90a&\xA3V[\x91PPa\x1E\x98V[P\x86\x88\x87`@Q` \x01a\x1F-\x93\x92\x91\x90a'LV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x1F[\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x1F|\x8A`\xFF\x8D\x16a&dV[\x81\x10\x15a\x1F\xE8W\x82\x81\x81Q\x81\x10a\x1F\x95Wa\x1F\x95a&\x8DV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x1F\xAF\x83\x8Da$\xE5V[\x81Q\x81\x10a\x1F\xBFWa\x1F\xBFa&\x8DV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x1F\xE0\x81a&\xA3V[\x91PPa\x1FoV[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a dW\x83\x81\x81Q\x81\x10a \x19Wa \x19a&\x8DV[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a 1\x91\x90a'\x80V[a <\x90`\x02a({V[a F\x91\x90a'\x80V[a P\x90\x83a$\xE5V[\x91P\x80a \\\x81a&\xA3V[\x91PPa\x1F\xFEV[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a \xB1WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a \xB1WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\x80\x82\x84\x03\x12\x15a \xF9W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a!)WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a!lW`\0\x80\xFD[a!v\x83\x83a \xE7V[\x93\x92PPPV[`\x02\x81\x10a!\x8AW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a!\x9FW`\0\x80\xFD[\x815a!v\x81a!}V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10a!\xDEWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Q`\xE0\x83\x01\x91a\"\x06\x90\x84\x01\x82a!\xC0V[P`@\x83\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x86\x01Q\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\xA0\x83\x01Qa d`\xA0\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a!\x8AW`\0\x80\xFD[`\0\x80`\xA0\x83\x85\x03\x12\x15a\"zW`\0\x80\xFD[a\"\x84\x84\x84a \xE7V[\x91P`\x80\x83\x015a\"\x94\x81a\"RV[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15a\"\xB1W`\0\x80\xFD[a\"\xB9a \x81V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15a\"\xE9W`\0\x80\xFD[a\"\xF3\x88\x88a \xE7V[\x95Pa#\x02\x88`\x80\x89\x01a\"\x9FV[\x94P`\xC0\x87\x015a#\x12\x81a\"RV[\x93P`\xE0\x87\x015a#\"\x81a!}V[\x92Pa#2\x88a\x01\0\x89\x01a\"\x9FV[\x91Pa\x01@\x87\x015a#C\x81a\"RV[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\x8AW`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15a#xW`\0\x80\xFD[a#\x80a \xB7V[\x825a#\x8B\x81a#QV[\x81R` \x83\x015a#\x9B\x81a!}V[` \x82\x01R`@\x83\x015a#\xAE\x81a\"RV[`@\x82\x01R``\x83\x015a#\xC1\x81a\"RV[``\x82\x01R`\x80\x83\x015a#\xD4\x81a\"RV[`\x80\x82\x01Ra#\xE6\x84`\xA0\x85\x01a\"\x9FV[`\xA0\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a$\x04W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x87\x16\x81R`\xE0\x81\x01a$(` \x83\x01\x88a!\xC0V[`\x01`\x01`@\x1B\x03\x86\x81\x16`@\x84\x01R\x85\x81\x16``\x84\x01R\x84\x16`\x80\x83\x01R\x82Q`\xA0\x83\x01R` \x83\x01Q`\xC0\x83\x01R\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a dWa da$cV[`\0\x80`@\x83\x85\x03\x12\x15a$\xACW`\0\x80\xFD[\x82Qa$\xB7\x81a\"RV[` \x84\x01Q\x90\x92Pa\"\x94\x81a\"RV[`\0` \x82\x84\x03\x12\x15a$\xDAW`\0\x80\xFD[\x81Qa!v\x81a\"RV[\x80\x82\x01\x80\x82\x11\x15a\x12uWa\x12ua$cV[\x84\x81R`\x01`\x01`@\x1B\x03\x84\x81\x16` \x83\x01R`\x80\x82\x01\x90a%\x1D`@\x84\x01\x86a!\xC0V[\x80\x84\x16``\x84\x01RP\x95\x94PPPPPV[`\0\x81\x83\x03`\xE0\x81\x12\x15a%BW`\0\x80\xFD[a%Ja \xB7V[\x83Qa%U\x81a#QV[\x81R` \x84\x01Qa%e\x81a!}V[` \x82\x01R`@\x84\x01Qa%x\x81a\"RV[`@\x82\x01R``\x84\x01Qa%\x8B\x81a\"RV[``\x82\x01R`\x80\x84\x01Qa%\x9E\x81a\"RV[`\x80\x82\x01R`@`\x9F\x19\x83\x01\x12\x15a%\xB5W`\0\x80\xFD[a%\xBDa \x81V[`\xA0\x85\x81\x01Q\x82R`\xC0\x90\x95\x01Q` \x82\x01R\x93\x81\x01\x93\x90\x93RP\x90\x92\x91PPV[`\0\x81Q`\0[\x81\x81\x10\x15a&\0W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a%\xE6V[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a&$a&\x1E\x83\x86a%\xDFV[\x84a%\xDFV[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a&_WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x81\x81\x03\x81\x81\x11\x15a\x12uWa\x12ua$cV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a&\xB5Wa&\xB5a$cV[P`\x01\x01\x90V[`\0a&\xC8\x82\x85a%\xDFV[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a&\xF4\x82\x84a%\xDFV[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a'\x0E\x82\x85a%\xDFV[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a'3\x82\x85a%\xDFV[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a'X\x82\x86a%\xDFV[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x12uWa\x12ua$cV[`\x01\x81\x81[\x80\x85\x11\x15a'\xD2W\x81`\0\x19\x04\x82\x11\x15a'\xB8Wa'\xB8a$cV[\x80\x85\x16\x15a'\xC5W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a'\x9CV[P\x92P\x92\x90PV[`\0\x82a'\xE9WP`\x01a\x12uV[\x81a'\xF6WP`\0a\x12uV[\x81`\x01\x81\x14a(\x0CW`\x02\x81\x14a(\x16Wa(2V[`\x01\x91PPa\x12uV[`\xFF\x84\x11\x15a('Wa('a$cV[PP`\x01\x82\x1Ba\x12uV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a(UWP\x81\x81\na\x12uV[a(_\x83\x83a'\x97V[\x80`\0\x19\x04\x82\x11\x15a(sWa(sa$cV[\x02\x93\x92PPPV[`\0a!v\x83\x83a'\xDAV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA1dsolcC\0\x08\x14\0\n";
    /// The deployed bytecode of the contract.
    pub static STAKETABLEMOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct StakeTableMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StakeTableMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StakeTableMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StakeTableMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StakeTableMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StakeTableMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StakeTableMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                STAKETABLEMOCK_ABI.clone(),
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
                STAKETABLEMOCK_ABI.clone(),
                STAKETABLEMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MAX_CHURN_RATE` (0x39d0209e) function
        pub fn max_churn_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([57, 208, 32, 158], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `exitQueue` (0xffed4bf5) function
        pub fn exit_queue(&self) -> ::ethers::contract::builders::ContractCall<M, (u64, u64)> {
            self.0
                .method_hash([255, 237, 75, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQueueParameters` (0x109af584) function
        pub fn get_queue_parameters(
            &self,
            queue_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u64)> {
            self.0
                .method_hash([16, 154, 245, 132], queue_type)
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
        ///Calls the contract's `nextEpoch` (0xe6c79484) function
        pub fn next_epoch(
            &self,
            queue_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([230, 199, 148, 132], queue_type)
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
        ///Calls the contract's `registrationQueue` (0xe99d7099) function
        pub fn registration_queue(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u64)> {
            self.0
                .method_hash([233, 157, 112, 153], ())
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeTableMockEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for StakeTableMock<M>
    {
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
    pub enum StakeTableMockErrors {
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
    impl ::ethers::core::abi::AbiDecode for StakeTableMockErrors {
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
    impl ::ethers::core::abi::AbiEncode for StakeTableMockErrors {
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
    impl ::ethers::contract::ContractRevert for StakeTableMockErrors {
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
    impl ::core::fmt::Display for StakeTableMockErrors {
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
    impl ::core::convert::From<::std::string::String> for StakeTableMockErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BLSSigVerificationFailed> for StakeTableMockErrors {
        fn from(value: BLSSigVerificationFailed) -> Self {
            Self::BLSSigVerificationFailed(value)
        }
    }
    impl ::core::convert::From<ExitRequestInProgress> for StakeTableMockErrors {
        fn from(value: ExitRequestInProgress) -> Self {
            Self::ExitRequestInProgress(value)
        }
    }
    impl ::core::convert::From<InvalidNextRegistrationEpoch> for StakeTableMockErrors {
        fn from(value: InvalidNextRegistrationEpoch) -> Self {
            Self::InvalidNextRegistrationEpoch(value)
        }
    }
    impl ::core::convert::From<NodeAlreadyRegistered> for StakeTableMockErrors {
        fn from(value: NodeAlreadyRegistered) -> Self {
            Self::NodeAlreadyRegistered(value)
        }
    }
    impl ::core::convert::From<PrematureDeposit> for StakeTableMockErrors {
        fn from(value: PrematureDeposit) -> Self {
            Self::PrematureDeposit(value)
        }
    }
    impl ::core::convert::From<PrematureExit> for StakeTableMockErrors {
        fn from(value: PrematureExit) -> Self {
            Self::PrematureExit(value)
        }
    }
    impl ::core::convert::From<PrematureWithdrawal> for StakeTableMockErrors {
        fn from(value: PrematureWithdrawal) -> Self {
            Self::PrematureWithdrawal(value)
        }
    }
    impl ::core::convert::From<RestakingNotImplemented> for StakeTableMockErrors {
        fn from(value: RestakingNotImplemented) -> Self {
            Self::RestakingNotImplemented(value)
        }
    }
    impl ::core::convert::From<Unauthenticated> for StakeTableMockErrors {
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
    pub enum StakeTableMockEvents {
        DepositFilter(DepositFilter),
        ExitFilter(ExitFilter),
        RegisteredFilter(RegisteredFilter),
    }
    impl ::ethers::contract::EthLogDecode for StakeTableMockEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(StakeTableMockEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = ExitFilter::decode_log(log) {
                return Ok(StakeTableMockEvents::ExitFilter(decoded));
            }
            if let Ok(decoded) = RegisteredFilter::decode_log(log) {
                return Ok(StakeTableMockEvents::RegisteredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StakeTableMockEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExitFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositFilter> for StakeTableMockEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<ExitFilter> for StakeTableMockEvents {
        fn from(value: ExitFilter) -> Self {
            Self::ExitFilter(value)
        }
    }
    impl ::core::convert::From<RegisteredFilter> for StakeTableMockEvents {
        fn from(value: RegisteredFilter) -> Self {
            Self::RegisteredFilter(value)
        }
    }
    ///Container type for all input parameters for the `MAX_CHURN_RATE` function with signature `MAX_CHURN_RATE()` and selector `0x39d0209e`
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
    #[ethcall(name = "MAX_CHURN_RATE", abi = "MAX_CHURN_RATE()")]
    pub struct MaxChurnRateCall;
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
    ///Container type for all input parameters for the `exitQueue` function with signature `exitQueue()` and selector `0xffed4bf5`
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
    #[ethcall(name = "exitQueue", abi = "exitQueue()")]
    pub struct ExitQueueCall;
    ///Container type for all input parameters for the `getQueueParameters` function with signature `getQueueParameters(uint8)` and selector `0x109af584`
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
    #[ethcall(name = "getQueueParameters", abi = "getQueueParameters(uint8)")]
    pub struct GetQueueParametersCall {
        pub queue_type: u8,
    }
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
    ///Container type for all input parameters for the `nextEpoch` function with signature `nextEpoch(uint8)` and selector `0xe6c79484`
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
    #[ethcall(name = "nextEpoch", abi = "nextEpoch(uint8)")]
    pub struct NextEpochCall {
        pub queue_type: u8,
    }
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
    ///Container type for all input parameters for the `registrationQueue` function with signature `registrationQueue()` and selector `0xe99d7099`
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
    #[ethcall(name = "registrationQueue", abi = "registrationQueue()")]
    pub struct RegistrationQueueCall;
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
    pub enum StakeTableMockCalls {
        MaxChurnRate(MaxChurnRateCall),
        HashBlsKey(HashBlsKeyCall),
        CurrentEpoch(CurrentEpochCall),
        Deposit(DepositCall),
        ExitEscrowPeriod(ExitEscrowPeriodCall),
        ExitQueue(ExitQueueCall),
        GetQueueParameters(GetQueueParametersCall),
        LightClient(LightClientCall),
        LookupNode(LookupNodeCall),
        LookupStake(LookupStakeCall),
        NextEpoch(NextEpochCall),
        Nodes(NodesCall),
        NumPendingExit(NumPendingExitCall),
        NumPendingRegistrations(NumPendingRegistrationsCall),
        Register(RegisterCall),
        RegistrationQueue(RegistrationQueueCall),
        RequestExit(RequestExitCall),
        TokenAddress(TokenAddressCall),
        TotalKeys(TotalKeysCall),
        TotalNativeStake(TotalNativeStakeCall),
        TotalRestakedStake(TotalRestakedStakeCall),
        TotalStake(TotalStakeCall),
        TotalVotingStake(TotalVotingStakeCall),
        WithdrawFunds(WithdrawFundsCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakeTableMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MaxChurnRateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxChurnRate(decoded));
            }
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
            if let Ok(decoded) = <ExitQueueCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExitQueue(decoded));
            }
            if let Ok(decoded) =
                <GetQueueParametersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetQueueParameters(decoded));
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
            if let Ok(decoded) = <NextEpochCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NextEpoch(decoded));
            }
            if let Ok(decoded) = <NodesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nodes(decoded));
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
            if let Ok(decoded) =
                <RegistrationQueueCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegistrationQueue(decoded));
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
    impl ::ethers::core::abi::AbiEncode for StakeTableMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxChurnRate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HashBlsKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CurrentEpoch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExitEscrowPeriod(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExitQueue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetQueueParameters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LightClient(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LookupNode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LookupStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NextEpoch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nodes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NumPendingExit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NumPendingRegistrations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Register(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegistrationQueue(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
    impl ::core::fmt::Display for StakeTableMockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxChurnRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashBlsKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExitEscrowPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExitQueue(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetQueueParameters(element) => ::core::fmt::Display::fmt(element, f),
                Self::LightClient(element) => ::core::fmt::Display::fmt(element, f),
                Self::LookupNode(element) => ::core::fmt::Display::fmt(element, f),
                Self::LookupStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nodes(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumPendingExit(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumPendingRegistrations(element) => ::core::fmt::Display::fmt(element, f),
                Self::Register(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistrationQueue(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<MaxChurnRateCall> for StakeTableMockCalls {
        fn from(value: MaxChurnRateCall) -> Self {
            Self::MaxChurnRate(value)
        }
    }
    impl ::core::convert::From<HashBlsKeyCall> for StakeTableMockCalls {
        fn from(value: HashBlsKeyCall) -> Self {
            Self::HashBlsKey(value)
        }
    }
    impl ::core::convert::From<CurrentEpochCall> for StakeTableMockCalls {
        fn from(value: CurrentEpochCall) -> Self {
            Self::CurrentEpoch(value)
        }
    }
    impl ::core::convert::From<DepositCall> for StakeTableMockCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<ExitEscrowPeriodCall> for StakeTableMockCalls {
        fn from(value: ExitEscrowPeriodCall) -> Self {
            Self::ExitEscrowPeriod(value)
        }
    }
    impl ::core::convert::From<ExitQueueCall> for StakeTableMockCalls {
        fn from(value: ExitQueueCall) -> Self {
            Self::ExitQueue(value)
        }
    }
    impl ::core::convert::From<GetQueueParametersCall> for StakeTableMockCalls {
        fn from(value: GetQueueParametersCall) -> Self {
            Self::GetQueueParameters(value)
        }
    }
    impl ::core::convert::From<LightClientCall> for StakeTableMockCalls {
        fn from(value: LightClientCall) -> Self {
            Self::LightClient(value)
        }
    }
    impl ::core::convert::From<LookupNodeCall> for StakeTableMockCalls {
        fn from(value: LookupNodeCall) -> Self {
            Self::LookupNode(value)
        }
    }
    impl ::core::convert::From<LookupStakeCall> for StakeTableMockCalls {
        fn from(value: LookupStakeCall) -> Self {
            Self::LookupStake(value)
        }
    }
    impl ::core::convert::From<NextEpochCall> for StakeTableMockCalls {
        fn from(value: NextEpochCall) -> Self {
            Self::NextEpoch(value)
        }
    }
    impl ::core::convert::From<NodesCall> for StakeTableMockCalls {
        fn from(value: NodesCall) -> Self {
            Self::Nodes(value)
        }
    }
    impl ::core::convert::From<NumPendingExitCall> for StakeTableMockCalls {
        fn from(value: NumPendingExitCall) -> Self {
            Self::NumPendingExit(value)
        }
    }
    impl ::core::convert::From<NumPendingRegistrationsCall> for StakeTableMockCalls {
        fn from(value: NumPendingRegistrationsCall) -> Self {
            Self::NumPendingRegistrations(value)
        }
    }
    impl ::core::convert::From<RegisterCall> for StakeTableMockCalls {
        fn from(value: RegisterCall) -> Self {
            Self::Register(value)
        }
    }
    impl ::core::convert::From<RegistrationQueueCall> for StakeTableMockCalls {
        fn from(value: RegistrationQueueCall) -> Self {
            Self::RegistrationQueue(value)
        }
    }
    impl ::core::convert::From<RequestExitCall> for StakeTableMockCalls {
        fn from(value: RequestExitCall) -> Self {
            Self::RequestExit(value)
        }
    }
    impl ::core::convert::From<TokenAddressCall> for StakeTableMockCalls {
        fn from(value: TokenAddressCall) -> Self {
            Self::TokenAddress(value)
        }
    }
    impl ::core::convert::From<TotalKeysCall> for StakeTableMockCalls {
        fn from(value: TotalKeysCall) -> Self {
            Self::TotalKeys(value)
        }
    }
    impl ::core::convert::From<TotalNativeStakeCall> for StakeTableMockCalls {
        fn from(value: TotalNativeStakeCall) -> Self {
            Self::TotalNativeStake(value)
        }
    }
    impl ::core::convert::From<TotalRestakedStakeCall> for StakeTableMockCalls {
        fn from(value: TotalRestakedStakeCall) -> Self {
            Self::TotalRestakedStake(value)
        }
    }
    impl ::core::convert::From<TotalStakeCall> for StakeTableMockCalls {
        fn from(value: TotalStakeCall) -> Self {
            Self::TotalStake(value)
        }
    }
    impl ::core::convert::From<TotalVotingStakeCall> for StakeTableMockCalls {
        fn from(value: TotalVotingStakeCall) -> Self {
            Self::TotalVotingStake(value)
        }
    }
    impl ::core::convert::From<WithdrawFundsCall> for StakeTableMockCalls {
        fn from(value: WithdrawFundsCall) -> Self {
            Self::WithdrawFunds(value)
        }
    }
    ///Container type for all return fields from the `MAX_CHURN_RATE` function with signature `MAX_CHURN_RATE()` and selector `0x39d0209e`
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
    pub struct MaxChurnRateReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `exitQueue` function with signature `exitQueue()` and selector `0xffed4bf5`
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
    pub struct ExitQueueReturn {
        pub first_available_epoch: u64,
        pub pending_requests: u64,
    }
    ///Container type for all return fields from the `getQueueParameters` function with signature `getQueueParameters(uint8)` and selector `0x109af584`
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
    pub struct GetQueueParametersReturn(pub u64, pub u64);
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
    ///Container type for all return fields from the `nextEpoch` function with signature `nextEpoch(uint8)` and selector `0xe6c79484`
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
    pub struct NextEpochReturn(pub u64);
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
    ///Container type for all return fields from the `registrationQueue` function with signature `registrationQueue()` and selector `0xe99d7099`
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
    pub struct RegistrationQueueReturn {
        pub first_available_epoch: u64,
        pub pending_requests: u64,
    }
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
