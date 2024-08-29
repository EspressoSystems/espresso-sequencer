pub use light_client_mock::*;
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
pub mod light_client_mock {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("genesis"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        ],),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("struct LightClient.LightClientState",),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("numBlockPerEpoch"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint32"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("UPGRADE_INTERFACE_VERSION"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("UPGRADE_INTERFACE_VERSION",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("blocksPerEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("blocksPerEpoch"),
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
                    ::std::borrow::ToOwned::to_owned("computeStakeTableComm"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computeStakeTableComm",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("state"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct LightClient.LightClientState",
                                ),
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
                    ::std::borrow::ToOwned::to_owned("disablePermissionedProverMode"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("disablePermissionedProverMode",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("frozenStakeTableCommitment"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("frozenStakeTableCommitment",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("frozenThreshold"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("frozenThreshold"),
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
                    ::std::borrow::ToOwned::to_owned("getFinalizedState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getFinalizedState"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct LightClient.LightClientState",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getGenesisState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getGenesisState"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct LightClient.LightClientState",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getHotShotBlockCommitmentsCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getHotShotBlockCommitmentsCount",),
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
                    ::std::borrow::ToOwned::to_owned("getHotShotCommitment"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getHotShotCommitment",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("hotShotBlockHeight",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct LightClient.HotShotCommitment",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStateUpdateBlockNumbersCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStateUpdateBlockNumbersCount",),
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
                    ::std::borrow::ToOwned::to_owned("getVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getVersion"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("majorVersion"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("minorVersion"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("patchVersion"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hotShotCommitments"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("hotShotCommitments"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockHeight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockCommRoot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.ScalarField"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("genesis"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct LightClient.LightClientState",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("numBlocksPerEpoch"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lagOverEscapeHatchThreshold"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lagOverEscapeHatchThreshold",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("threshold"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("newFinalizedState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("newFinalizedState"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newState"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct LightClient.LightClientState",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proof"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IPlonkVerifier.PlonkProof",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("permissionedProver"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("permissionedProver"),
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
                    ::std::borrow::ToOwned::to_owned("permissionedProverEnabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("permissionedProverEnabled",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setCurrentEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setCurrentEpoch"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newEpoch"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFinalizedState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFinalizedState"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("state"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct LightClient.LightClientState",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setHotShotCommitments"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setHotShotCommitments",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("values"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct LightClient.HotShotCommitment[]",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setHotShotDownSince"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setHotShotDownSince",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("l1Height"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setHotShotUp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setHotShotUp"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPermissionedProver"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPermissionedProver",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("prover"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setStateUpdateBlockNumbers"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setStateUpdateBlockNumbers",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("values"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stateUpdateBlockNumbers"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stateUpdateBlockNumbers",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("states"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("states"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("viewNum"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockHeight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockCommRoot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.ScalarField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("feeLedgerComm"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.ScalarField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakeTableBlsKeyComm",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.ScalarField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakeTableSchnorrKeyComm",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.ScalarField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakeTableAmountComm",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.ScalarField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("threshold"),
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("votingStakeTableCommitment"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("votingStakeTableCommitment",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("votingThreshold"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("votingThreshold"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EpochChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EpochChanged"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialized"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewState"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewState"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("viewNum"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("blockHeight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("blockCommRoot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PermissionedProverNotRequired"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PermissionedProverNotRequired",),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PermissionedProverRequired"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PermissionedProverRequired",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("permissionedProver",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Upgrade"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Upgrade"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("target"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1967InvalidImplementation"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1967InvalidImplementation",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientSnapshotHistory"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientSnapshotHistory",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidAddress"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidArgs"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidArgs"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidHotShotBlockForCommitmentCheck"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "InvalidHotShotBlockForCommitmentCheck",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInitialization"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidInitialization",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidProof"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidProof"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MissingLastBlockForCurrentEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MissingLastBlockForCurrentEpoch",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("expectedBlockHeight",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoChangeRequired"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NoChangeRequired"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotInitializing"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotInitializing"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutdatedState"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OutdatedState"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PermissionedProverNotSet"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PermissionedProverNotSet",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProverNotPermissioned"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ProverNotPermissioned",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UUPSUnauthorizedCallContext"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UUPSUnauthorizedCallContext",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UUPSUnsupportedProxiableUUID"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UUPSUnsupportedProxiableUUID",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("slot"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnsupportedDegree"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UnsupportedDegree"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongPlonkVK"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongPlonkVK"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongStakeTableUsed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongStakeTableUsed",),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIGHTCLIENTMOCK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R4\x80\x15b\0\0\x15W`\0\x80\xFD[P`@Qb\0Q\x8A8\x03\x80b\0Q\x8A\x839\x81\x01`@\x81\x90Rb\0\08\x91b\0\x04\xECV[b\0\0Bb\0\0VV[b\0\0N\x82\x82b\0\x01\nV[PPb\0\x05\xBFV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15b\0\0\xA7W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14b\0\x01\x07W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[\x81Q`\x01`\x01`@\x1B\x03\x16\x15\x15\x80b\0\x01/WP` \x82\x01Q`\x01`\x01`@\x1B\x03\x16\x15\x15[\x80b\0\x01=WP`\x80\x82\x01Q\x15[\x80b\0\x01KWP`\xA0\x82\x01Q\x15[\x80b\0\x01YWP`\xC0\x82\x01Q\x15[\x80b\0\x01gWP`\xE0\x82\x01Q\x15[\x80b\0\x01wWPc\xFF\xFF\xFF\xFF\x81\x16\x15[\x15b\0\x01\x96W`@QcP\xDD\x03\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x05`\0\x80`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x08a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01U``\x82\x01Q\x81`\x02\x01U`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01U`\xE0\x82\x01Q\x81`\x06\x01U\x90PP\x81`\x05`\0\x80`\x08\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x08a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01U``\x82\x01Q\x81`\x02\x01U`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01U`\xE0\x82\x01Q\x81`\x06\x01U\x90PP`\0\x80`\x0Ca\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x80`\0\x80a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0b\0\x03\x84\x83b\0\x04r` \x1B` \x1CV[`\x01\x81\x81U`\xE0\x85\x01Q`\x02\x81\x81U`\x03\x93\x90\x93U`\x04U`\x07\x80T\x80\x83\x01\x82U`\0\x91\x82RC\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88\x90\x91\x01U`@\x80Q\x80\x82\x01\x82R` \x80\x89\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x83R\x92\x90\x98\x01Q\x97\x81\x01\x97\x88R`\x08\x80T\x94\x85\x01\x81U\x90\x92R\x90Q\x7F\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE3\x92\x90\x93\x02\x91\x82\x01\x80T`\x01`\x01`@\x1B\x03\x19\x16\x93\x90\x91\x16\x92\x90\x92\x17\x90\x91U\x92Q\x7F\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE4\x90\x93\x01\x92\x90\x92UPPV[`\x80\x80\x82\x01Q`\xA0\x83\x01Q`\xC0\x84\x01Q`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x80Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x14b\0\x04\xD2W`\0\x80\xFD[\x91\x90PV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x04\xD2W`\0\x80\xFD[`\0\x80\x82\x84\x03a\x01 \x81\x12\x15b\0\x05\x02W`\0\x80\xFD[a\x01\0\x80\x82\x12\x15b\0\x05\x13W`\0\x80\xFD[`@Q\x91P\x80\x82\x01`\x01`\x01`@\x1B\x03\x81\x11\x83\x82\x10\x17\x15b\0\x05EWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Rb\0\x05S\x85b\0\x04\xBAV[\x82Rb\0\x05c` \x86\x01b\0\x04\xBAV[` \x83\x01R`@\x85\x01Q`@\x83\x01R``\x85\x01Q``\x83\x01R`\x80\x85\x01Q`\x80\x83\x01R`\xA0\x85\x01Q`\xA0\x83\x01R`\xC0\x85\x01Q`\xC0\x83\x01R`\xE0\x85\x01Q`\xE0\x83\x01R\x81\x93Pb\0\x05\xB4\x81\x86\x01b\0\x04\xD7V[\x92PPP\x92P\x92\x90PV[`\x80QaK\xA1b\0\x05\xE9`\09`\0\x81\x81a\x14\xAC\x01R\x81\x81a\x14\xD5\x01Ra\x16A\x01RaK\xA1`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\x94W`\x005`\xE0\x1C\x80c\x01?\xA5\xFC\x14a\x01\x99W\x80c\r\x8En,\x14a\x01\xBBW\x80c *\n\xDB\x14a\x01\xEDW\x80c-R\xAA\xD6\x14a\x02\x96W\x80c1=\xF7\xB1\x14a\x02\xC3W\x80c8+!Z\x14a\x02\xF0W\x80c9\x194\x0F\x14a\x03\x14W\x80c9I\xD1\xE9\x14a\x034W\x80c@\x999\xB7\x14a\x03{W\x80cHG\xAE]\x14a\x03\x9BW\x80cO\x1E\xF2\x86\x14a\x04\x1DW\x80cR\xD1\x90-\x14a\x040W\x80cS\x0C\xA7\x8F\x14a\x04EW\x80cTd`\x85\x14a\x04eW\x80cb\x82w3\x14a\x04zW\x80ci\xCCj\x04\x14a\x04\x90W\x80cpS\xFCQ\x14a\x04\xA5W\x80cqP\x18\xA6\x14a\x04\xBAW\x80cvg\x18\x08\x14a\x04\xCFW\x80cv\xB6\xB7\xCB\x14a\x05\x03W\x80c\x7F\x17\xBA\xAD\x14a\x05\x19W\x80c\x82\xD0\x7F\xF3\x14a\x05\xCCW\x80c\x85\x84\xD2?\x14a\x05\xE1W\x80c\x8D\xA5\xCB[\x14a\x06%W\x80c\xA2D\xD5\x96\x14a\x06:W\x80c\xA5\x1Eo\xEA\x14a\x06ZW\x80c\xAA\x92'2\x14a\x06zW\x80c\xAD<\xB1\xCC\x14a\x06\x9AW\x80c\xBD2Q\x9A\x14a\x06\xD8W\x80c\xC8\xE5\xE4\x98\x14a\x07\tW\x80c\xCAo\xE8U\x14a\x07%W\x80c\xDB\x13\xB6\n\x14a\x07;W\x80c\xE003\x01\x14a\x07zW\x80c\xF0h T\x14a\x07\x9AW\x80c\xF2\xFD\xE3\x8B\x14a\x07\xCCW[`\0\x80\xFD[4\x80\x15a\x01\xA5W`\0\x80\xFD[Pa\x01\xB9a\x01\xB46`\x04aB\x87V[a\x07\xECV[\0[4\x80\x15a\x01\xC7W`\0\x80\xFD[P`@\x80Q`\x01\x81R`\0` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF9W`\0\x80\xFD[Pa\x01\xB9a\x02\x086`\x04aC\xE7V[`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` \x81\x81R`@\x93\x84\x90 \x85Q\x81T\x92\x87\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x95\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x94\x16\x93\x90\x93\x17\x17\x82U\x91\x83\x01Q`\x01\x82\x01U``\x83\x01Q`\x02\x82\x01U`\x80\x83\x01Q`\x03\x82\x01U`\xA0\x83\x01Q`\x04\x82\x01U`\xC0\x83\x01Q\x91\x81\x01\x91\x90\x91U`\xE0\x90\x91\x01Q`\x06\x90\x91\x01UV[4\x80\x15a\x02\xA2W`\0\x80\xFD[Pa\x01\xB9a\x02\xB16`\x04aD\x04V[`\t\x80T`\xFF\x19\x16`\x01\x17\x90U`\nUV[4\x80\x15a\x02\xCFW`\0\x80\xFD[P`\x06Ta\x02\xE3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qa\x01\xE4\x91\x90aD\x1DV[4\x80\x15a\x02\xFCW`\0\x80\xFD[Pa\x03\x06`\x03T\x81V[`@Q\x90\x81R` \x01a\x01\xE4V[4\x80\x15a\x03 W`\0\x80\xFD[Pa\x01\xB9a\x03/6`\x04aDTV[a\x08\xACV[4\x80\x15a\x03@W`\0\x80\xFD[Pa\x01\xB9a\x03O6`\x04aD\xE9V[`\0\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01``\x1B\x02`\x01``\x1B`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x80\x15a\x03\x87W`\0\x80\xFD[Pa\x01\xB9a\x03\x966`\x04aE4V[a\x08\xCFV[4\x80\x15a\x03\xA7W`\0\x80\xFD[Pa\x03\xB0a\x0B\xFAV[`@Qa\x01\xE4\x91\x90`\0a\x01\0\x82\x01\x90P`\x01\x80`@\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01R\x92\x91PPV[a\x01\xB9a\x04+6`\x04aF\xEEV[a\x0C\x8CV[4\x80\x15a\x04<W`\0\x80\xFD[Pa\x03\x06a\x0C\xA7V[4\x80\x15a\x04QW`\0\x80\xFD[Pa\x01\xB9a\x04`6`\x04aG\x93V[a\x0C\xC4V[4\x80\x15a\x04qW`\0\x80\xFD[P`\x08Ta\x03\x06V[4\x80\x15a\x04\x86W`\0\x80\xFD[Pa\x03\x06`\x02T\x81V[4\x80\x15a\x04\x9CW`\0\x80\xFD[Pa\x01\xB9a\r@V[4\x80\x15a\x04\xB1W`\0\x80\xFD[P`\x07Ta\x03\x06V[4\x80\x15a\x04\xC6W`\0\x80\xFD[Pa\x01\xB9a\r\xB0V[4\x80\x15a\x04\xDBW`\0\x80\xFD[P`\0Ta\x04\xF6\x90`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Qa\x01\xE4\x91\x90aHDV[4\x80\x15a\x05\x0FW`\0\x80\xFD[Pa\x03\x06`\x01T\x81V[4\x80\x15a\x05%W`\0\x80\xFD[Pa\x05\x86a\x0546`\x04aHlV[`\x05` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T\x95\x85\x01T`\x06\x90\x95\x01T`\x01`\x01`@\x1B\x03\x80\x86\x16\x97`\x01`@\x1B\x90\x96\x04\x16\x95\x93\x94\x92\x93\x91\x92\x91\x90\x88V[`@\x80Q`\x01`\x01`@\x1B\x03\x99\x8A\x16\x81R\x98\x90\x97\x16` \x89\x01R\x95\x87\x01\x94\x90\x94R``\x86\x01\x92\x90\x92R`\x80\x85\x01R`\xA0\x84\x01R`\xC0\x83\x01R`\xE0\x82\x01Ra\x01\0\x01a\x01\xE4V[4\x80\x15a\x05\xD8W`\0\x80\xFD[Pa\x03\xB0a\r\xC2V[4\x80\x15a\x05\xEDW`\0\x80\xFD[Pa\x06\x01a\x05\xFC6`\x04aD\x04V[a\x0ERV[`@\x80Q\x82Q`\x01`\x01`@\x1B\x03\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x01\xE4V[4\x80\x15a\x061W`\0\x80\xFD[Pa\x02\xE3a\x0F\xACV[4\x80\x15a\x06FW`\0\x80\xFD[Pa\x01\xB9a\x06U6`\x04aH\x87V[a\x0F\xC7V[4\x80\x15a\x06fW`\0\x80\xFD[Pa\x03\x06a\x06u6`\x04aD\x04V[a\x10\xF2V[4\x80\x15a\x06\x86W`\0\x80\xFD[Pa\x03\x06a\x06\x956`\x04aC\xE7V[a\x11\x13V[4\x80\x15a\x06\xA6W`\0\x80\xFD[Pa\x06\xCB`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x01\xE4\x91\x90aH\xF2V[4\x80\x15a\x06\xE4W`\0\x80\xFD[P`\x06Ta\x06\xF9\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xE4V[4\x80\x15a\x07\x15W`\0\x80\xFD[Pa\x01\xB9`\t\x80T`\xFF\x19\x16\x90UV[4\x80\x15a\x071W`\0\x80\xFD[Pa\x03\x06`\x04T\x81V[4\x80\x15a\x07GW`\0\x80\xFD[Pa\x07[a\x07V6`\x04aD\x04V[a\x11[V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xE4V[4\x80\x15a\x07\x86W`\0\x80\xFD[Pa\x06\xF9a\x07\x956`\x04aI%V[a\x11\x93V[4\x80\x15a\x07\xA6W`\0\x80\xFD[P`\0Ta\x07\xB7\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xE4V[4\x80\x15a\x07\xD8W`\0\x80\xFD[Pa\x01\xB9a\x07\xE76`\x04aB\x87V[a\x11\xCBV[a\x07\xF4a\x12\tV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x08\x1BW`@Qc\xE6\xC4${`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x08JW`@Qc\xA8c\xAE\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x91\x82\x90U`@Q\x7F\x80\x17\xBB\x88\x7F\xDF\x8F\xCAC\x14\xA9\xD4\x0Fns\xB3\xB8\x10\x02\xD6~\\\xFA\x85\xD8\x81s\xAFj\xA4`r\x92a\x08\xA1\x92\x16\x90aD\x1DV[`@Q\x80\x91\x03\x90\xA1PV[a\x08\xB8`\x07`\0a?\xA6V[\x80Qa\x08\xCB\x90`\x07\x90` \x84\x01\x90a?\xC4V[PPV[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x80\x15a\x08\xF3WP`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\t:W`\x06T`\x01`\x01`\xA0\x1B\x03\x16a\t!W`@Qc\x12\xE6\xD1\xE7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x01GL\x8F`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\tBa\r\xC2V[Q\x82Q`\x01`\x01`@\x1B\x03\x91\x82\x16\x91\x16\x11\x15\x80a\t\x80WPa\tba\r\xC2V[` \x01Q`\x01`\x01`@\x1B\x03\x16\x82` \x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15[\x15a\t\x9EW`@Qc\x05\x1CF\xEF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\t\xC3\x90c\xFF\xFF\xFF\xFF\x81\x16\x90`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aI]V[`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` R`@\x90\x91 T\x91\x92P\x90\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x82\x16\x14\x80\x15\x81a\n\x19WP\x81`\x01`\x01`@\x1B\x03\x16\x84` \x01Q`\x01`\x01`@\x1B\x03\x16\x11[\x15a\nBW\x81`@Qc\x03df\xBF`\xE3\x1B\x81R`\x04\x01a\n9\x91\x90aHDV[`@Q\x80\x91\x03\x90\xFD[a\nO\x84`@\x01Qa\x12;V[a\n\\\x84``\x01Qa\x12;V[a\ni\x84`\x80\x01Qa\x12;V[a\nv\x84`\xA0\x01Qa\x12;V[a\n\x83\x84`\xC0\x01Qa\x12;V[\x80\x15a\n\x91Wa\n\x91a\x12\x97V[a\n\x9B\x84\x84a\x13\xE2V[`\0\x80T`\x01`@\x1B\x90\x81\x90\x04c\xFF\xFF\xFF\xFF\x16\x82R`\x05` \x81\x81R`@\x80\x85 \x89Q\x81T\x8B\x85\x01\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x91\x83\x16\x90\x97\x02\x17\x82U\x8A\x83\x01\x80Q`\x01\x80\x85\x01\x91\x90\x91U``\x8D\x01Q`\x02\x80\x86\x01\x91\x90\x91U`\x80\x8E\x01Q`\x03\x86\x01U`\xA0\x8E\x01Q`\x04\x86\x01U`\xC0\x8E\x01Q\x97\x85\x01\x97\x90\x97U`\xE0\x8D\x01Q`\x06\x90\x94\x01\x93\x90\x93U`\x07\x80T\x80\x85\x01\x82U\x90\x89RC`\0\x80Q` aJu\x839\x81Q\x91R\x90\x91\x01U\x83Q\x80\x85\x01\x85R\x87Q\x83\x16\x81R\x81Q\x81\x87\x01\x90\x81R`\x08\x80T\x95\x86\x01\x81U\x90\x99RQ`\0\x80Q` aJ\xF5\x839\x81Q\x91R\x93\x90\x96\x02\x92\x83\x01\x80T`\x01`\x01`@\x1B\x03\x19\x16\x96\x83\x16\x96\x90\x96\x17\x90\x95U\x95Q`\0\x80Q` aK5\x839\x81Q\x91R\x90\x91\x01U\x92Q\x88Q\x92Q\x93Q\x93\x84R\x84\x16\x93\x91\x90\x91\x16\x91\x7F\xA0Jw9$PZA\x85d67%\xF5h2\xF5w.k\x8D\r\xBDn\xFC\xE7$\xDF\xE8\x03\xDA\xE6\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPV[a\x0C\x02a@\x0FV[P`\0\x80T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81R`\x05` \x81\x81R`@\x92\x83\x90 \x83Qa\x01\0\x81\x01\x85R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x90\x91\x04\x16\x92\x81\x01\x92\x90\x92R`\x01\x81\x01T\x93\x82\x01\x93\x90\x93R`\x02\x83\x01T``\x82\x01R`\x03\x83\x01T`\x80\x82\x01R`\x04\x83\x01T`\xA0\x82\x01R\x90\x82\x01T`\xC0\x82\x01R`\x06\x90\x91\x01T`\xE0\x82\x01R\x90V[a\x0C\x94a\x14\xA1V[a\x0C\x9D\x82a\x15FV[a\x08\xCB\x82\x82a\x15}V[`\0a\x0C\xB1a\x166V[P`\0\x80Q` aJ\xB5\x839\x81Q\x91R\x90V[a\x0C\xD0`\x08`\0a@fV[`\0[\x81Q\x81\x10\x15a\x08\xCBW`\x08\x82\x82\x81Q\x81\x10a\x0C\xF0Wa\x0C\xF0aI\x80V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x80\x82\x01\x85U`\0\x94\x85R\x93\x83\x90 \x82Q`\x02\x90\x92\x02\x01\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x81U\x91\x01Q\x90\x82\x01U\x01a\x0C\xD3V[a\rHa\x12\tV[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\r\x95W`\x06\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x90U`@Q\x7F\x9A_W\xDE\x85m\xD6h\xC5M\xD9^\\U\xDF\x93C!q\xCB\xCAI\xA8wmV \xEAY\xC0$P\x90`\0\x90\xA1V[`@Qc\xA8c\xAE\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\r\xB8a\x12\tV[a\r\xAE`\0a\x16\x7FV[a\r\xCAa@\x0FV[P`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` \x81\x81R`@\x93\x84\x90 \x84Qa\x01\0\x81\x01\x86R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R\x95\x90\x04\x90\x94\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x83\x01T``\x83\x01R`\x03\x83\x01T`\x80\x83\x01R`\x04\x83\x01T`\xA0\x83\x01R\x82\x01T`\xC0\x82\x01R`\x06\x90\x91\x01T`\xE0\x82\x01R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x08\x80T\x90a\x0Ev`\x01\x83aI\x96V[\x81T\x81\x10a\x0E\x86Wa\x0E\x86aI\x80V[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x83\x10a\x0E\xBFW`@Qc\x18V\xA4\x99`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x0FTW\x83`\x08\x82\x81T\x81\x10a\x0E\xDEWa\x0E\xDEaI\x80V[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0FLW`\x08\x81\x81T\x81\x10a\x0F\x11Wa\x0F\x11aI\x80V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80T`\x01`\x01`@\x1B\x03\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`\x01\x01a\x0E\xC2V[P`\x08a\x0Fb`\x01\x83aI\x96V[\x81T\x81\x10a\x0FrWa\x0FraI\x80V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80T`\x01`\x01`@\x1B\x03\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80a\x0F\xB7a\x16\xDBV[T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[`\0a\x0F\xD1a\x16\xFFV[\x80T\x90\x91P`\xFF`\x01`@\x1B\x82\x04\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15a\x0F\xF8WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x10\x14WP0;\x15[\x90P\x81\x15\x80\x15a\x10\"WP\x80\x15[\x15a\x10@W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84T`\x01`\x01`@\x1B\x03\x19\x16`\x01\x17\x85U\x83\x15a\x10iW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x10r\x86a\x17#V[a\x10za\x174V[`\0\x80T`\x01` \x1B`\x01``\x1B\x03\x19\x16`\x01`@\x1B\x17\x90Ua\x10\x9D\x88\x88a\x17<V[\x83\x15a\x10\xE8W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90a\x10\xDF\x90`\x01\x90aHDV[`@Q\x80\x91\x03\x90\xA1[PPPPPPPPV[`\x07\x81\x81T\x81\x10a\x11\x02W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\x80\x80\x82\x01Q`\xA0\x83\x01Q`\xC0\x84\x01Q`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x08\x81\x81T\x81\x10a\x11kW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01\x80T`\x01\x90\x91\x01T`\x01`\x01`@\x1B\x03\x90\x91\x16\x91P\x82V[`\tT`\0\x90`\xFF\x16\x15a\x11\xB8W\x81`\nT\x84a\x11\xB0\x91\x90aI\x96V[\x11\x90Pa\x11\xC5V[a\x11\xC2\x83\x83a\x1A_V[\x90P[\x92\x91PPV[a\x11\xD3a\x12\tV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x11\xFDW`\0`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\x04\x01a\n9\x91\x90aD\x1DV[a\x12\x06\x81a\x16\x7FV[PV[3a\x12\x12a\x0F\xACV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\r\xAEW3`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R`\x04\x01a\n9\x91\x90aD\x1DV[`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x81\x10\x80a\x08\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x10\x9B\x8C\x8DM\x0E\x88\x1A[\x9D\x98[\x1AY\x08\x1C\xD8\xD8[\x18\\\x88\x19\x9AY[\x19`*\x1B`D\x82\x01R`d\x01a\n9V[`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` \x81\x81R`@\x80\x85 \x81Qa\x01\0\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R\x96\x90\x04\x90\x95\x16\x92\x85\x01\x92\x90\x92R`\x01\x82\x01T\x90\x84\x01R`\x02\x81\x01T``\x84\x01R`\x03\x81\x01T`\x80\x84\x01R`\x04\x81\x01T`\xA0\x84\x01R\x90\x81\x01T`\xC0\x83\x01R`\x06\x01T`\xE0\x82\x01Ra\x13\x1F\x90a\x11\x13V[`\x03\x80T`\x01\x90\x81U\x90\x82\x90U`\x04\x80T`\x02U`\0\x80T`\x01`@\x1B\x81\x04c\xFF\xFF\xFF\xFF\x16\x82R`\x05` R`@\x82 `\x06\x01T\x90\x92U\x92\x93P\x90\x91\x90`\x0C\x90a\x13z\x90\x84\x90`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aI\xA9V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x7F\xDB5X%\x9E\x03\x9D~P\xE8\x16\xB9\xDC\xCE0\xFB\x11M\x8A\x9C\x86\xEC\xA5\xAB\x14\xB6\x01\x94\xD6\x94]?`\0`\x0C\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16`@Qa\x08\xA1\x91\x90aHDV[`\0a\x13\xECa\x1B@V[\x90Pa\x13\xF6a@\x87V[`\x02T\x81R\x83Q`\x01`\x01`@\x1B\x03\x90\x81\x16` \x80\x84\x01\x91\x90\x91R\x80\x86\x01Q\x90\x91\x16`@\x80\x84\x01\x91\x90\x91R\x80\x86\x01Q``\x80\x85\x01\x91\x90\x91R\x86\x01Q`\x80\x84\x01R`\0\x80T`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x80\x82R`\x05\x80\x85R\x92\x82 `\x03\x81\x01T`\xA0\x87\x01R`\x04\x81\x01T`\xC0\x87\x01R\x91R\x91\x81\x90R\x01T`\xE0\x82\x01Ra\x14~\x82\x82\x85a!oV[a\x14\x9BW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x15(WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x15\x1C`\0\x80Q` aJ\xB5\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\r\xAEW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15Na\x12\tV[\x7F\xF7\x87!\"n\xFE\x9A\x1B\xB6x\x18\x9A\x16\xD1UI(\xB9\xF2\x19.,\xB9>\xED\xA8;y\xFA@\0}\x81`@Qa\x08\xA1\x91\x90aD\x1DV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x15\xD7WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x15\xD4\x91\x81\x01\x90aI\xD0V[`\x01[a\x15\xF6W\x81`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x04\x01a\n9\x91\x90aD\x1DV[`\0\x80Q` aJ\xB5\x839\x81Q\x91R\x81\x14a\x16'W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\n9V[a\x161\x83\x83a!\xEDV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\r\xAEW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x16\x89a\x16\xDBV[\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x93\x94P\x91\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPPV[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90V[a\x17+a\"CV[a\x12\x06\x81a\"hV[a\r\xAEa\"CV[\x81Q`\x01`\x01`@\x1B\x03\x16\x15\x15\x80a\x17`WP` \x82\x01Q`\x01`\x01`@\x1B\x03\x16\x15\x15[\x80a\x17mWP`\x80\x82\x01Q\x15[\x80a\x17zWP`\xA0\x82\x01Q\x15[\x80a\x17\x87WP`\xC0\x82\x01Q\x15[\x80a\x17\x94WP`\xE0\x82\x01Q\x15[\x80a\x17\xA3WPc\xFF\xFF\xFF\xFF\x81\x16\x15[\x15a\x17\xC1W`@QcP\xDD\x03\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x05`\0\x80`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x08a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01U``\x82\x01Q\x81`\x02\x01U`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01U`\xE0\x82\x01Q\x81`\x06\x01U\x90PP\x81`\x05`\0\x80`\x08\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x08a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01U``\x82\x01Q\x81`\x02\x01U`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01U`\xE0\x82\x01Q\x81`\x06\x01U\x90PP`\0\x80`\x0Ca\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x80`\0\x80a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0a\x19\xA7\x83a\x11\x13V[`\x01\x81\x81U`\xE0\x85\x01Q`\x02\x81\x81U`\x03\x93\x90\x93U`\x04U`\x07\x80T\x80\x83\x01\x82U`\0\x91\x82RC`\0\x80Q` aJu\x839\x81Q\x91R\x90\x91\x01U`@\x80Q\x80\x82\x01\x82R` \x80\x89\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x83R\x92\x90\x98\x01Q\x97\x81\x01\x97\x88R`\x08\x80T\x94\x85\x01\x81U\x90\x92R\x90Q`\0\x80Q` aJ\xF5\x839\x81Q\x91R\x92\x90\x93\x02\x91\x82\x01\x80T`\x01`\x01`@\x1B\x03\x19\x16\x93\x90\x91\x16\x92\x90\x92\x17\x90\x91U\x92Q`\0\x80Q` aK5\x839\x81Q\x91R\x90\x93\x01\x92\x90\x92UPPV[`\x07T`\0\x90C\x84\x11\x80a\x1AsWP`\x03\x81\x10[\x15a\x1A\x91W`@Qc\xB0\xB48w`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80a\x1A\xA0`\x01\x85aI\x96V[\x90P[\x81a\x1B\x0BW\x86`\x07\x82\x81T\x81\x10a\x1A\xBCWa\x1A\xBCaI\x80V[\x90`\0R` `\0 \x01T\x11a\x1A\xF1W`\x01\x91P`\x07\x81\x81T\x81\x10a\x1A\xE3Wa\x1A\xE3aI\x80V[\x90`\0R` `\0 \x01T\x92P[`\x02\x81\x10a\x1B\x0BW\x80a\x1B\x03\x81aI\xE9V[\x91PPa\x1A\xA3V[\x81a\x1B)W`@Qc\xB0\xB48w`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85a\x1B4\x84\x89aI\x96V[\x11\x97\x96PPPPPPPV[a\x1BHa@\xA6V[b\x01\0\0\x81R`\x08` \x82\x01R\x7F\x01=\x1DKBQy%\x8BWx`9yU\xCB\xFA\x08\x16\xE3+\x1C%\xA1\xFDsL\x91\xB9Q\xEE\x81`@\x82\x01QR\x7F\x16\xB8\x8D\xC7C\x9Am\x84\x1E\x1A\x11\x03\xF5\xA3\xD2\xD2D\x017\xF1\x8D\x02v5\x03\xBA\xC7\xB4]\xCB\x98;` `@\x83\x01Q\x01R\x7F\x0C<\x86O\x19_Y\x11\x99'\xF58W\xF1\xDE\x8B\xF5u\x94\x17H\xB755\x1F\xD3\x13s\xC7\x87\\-``\x82\x01QR\x7F\x16\x9B\xA1Q\x07\xF2\xEF\xF9\xB94\x1B\xF3\x07B\xA88\xD2}\xBDi\xE8\x8B#S\xDC\xA8Y/\x15\xF1\x11\x1C` ``\x83\x01Q\x01R\x7F\x11\xD4\xCE\xB1Ya\xD1\x0BaV\xAE=\t\xBBx\xB4\xDFE\xFB\x85C\x06\x08\x84\xE7\xD4\0u[\xEBJ\xC8`\x80\x82\x01QR\x7F\x03&\xFF\x069\x1E\xD5\xD2n\xC1\xBC\x08\x0B\x8DF\x01N\xE2,\x0Ch\xED\x02/\x16 \xC4\xD9\xD3\x847\xD3` `\x80\x83\x01Q\x01R\x7F#a\x0C\xB4>!\x03<6\x8A\x93b-\xD4\x05\xB9\x05\xA0\xEB4L\x98\xB9\xD7\xCF\x08\xB0\xC5\xEB\xF7\xC89`\xA0\x82\x01QR~\x13y4*Mw\xD4p\x87C\xAF\xF0\x1F\xF2z\xA1\x19\x17G\x8F\xDC\x8E+}F0\x81sWr\xEA` `\xA0\x83\x01Q\x01R\x7F\x19M\xAF\x85\xD9\xEE\xD9\x93{(\xE2\xA6\x80\xFC\xC5\xA7i\"\xC1\\\xD3\x1D\xC4\xF6\0\xE1I9\xB8 \x0C\xE7`\xC0\x82\x01QR\x7F%(\x0B\x12F$\x91\x1C\x7F\x87\xB4\xC2\xD8\x7FY\xC6\xC0~>\xEE\xB1\raM\xA2\x16\xF6!\x9F\xFEP\xB6` `\xC0\x83\x01Q\x01R\x7F\x04\x88.\xF3\x98\x99\xEA8\xC9gzH\xB8\xF8\xCCjg(N\x17\xFF\x94\x02\x89\xFA\xAA5\x9E\xEC\x9B3\xA6`\xE0\x82\x01QR\x7F\x1B\xAE\x9F6\xE6\x19\x078\xC7\x11P\x1B\xE5?)\x9B\xF6\x13H\xE6\x1E.\xF9\xD5wv\x0Ed\xF6)6\x8D` `\xE0\x83\x01Q\x01R\x7F-\x81\r0\x12\x0C\xB9>\x1A%K\x89\xED\n\xE8lv\x1FI\xB4\xF1)E\x9C\xD54\xF9U\x18Q5\x0Fa\x01\0\x82\x01QR\x7F\x0B%9M\xA5\xA1\xD45\xDA\xCC\xC2\xEA\xDD\x03\x9E,'\t\xF5\xF4/\xAB\xD9\xAF\xBA\x81^\xD6-j\xF3k` a\x01\0\x83\x01Q\x01R\x7F\x1C,\xE7\xBEW\x0B\xEA\x9EC\xF3\xD3\xD7\xCB\xCA\x84\xBD\xB4\xFC\x89\xB5:\xE6WS\x1D\xE7&p\xA6\x10^\x08a\x01 \x82\x01QR\x7F\x1A\x02U\xEC\x8C|\x87i3[\xC9\xDCk\"*\xC6\xA0Nvm\x08\xB4\\\x8C\xC5sY,\x05\xBC\x9C\x14` a\x01 \x83\x01Q\x01R\x7F\x1C\x16AY\x13k\x8F[Gs\xE13\xF4\x83\xA8\xA1\x92\xAB\x15\xD6\xD3\xEE\x01/\x17\x1B=\x02\xFDE\x06\xE7a\x01@\x82\x01QR\x7F'.\xB7\xD63\xCE\xDBh\xCE\x01\x13\xF4B\n\xB5a\x0B\x81\xB8\xBA\x1A\xB94\x8D\xB1Wa\xD4\x0E\x8D\xF5\xBA` a\x01@\x83\x01Q\x01R\x7F\x0EDf9\xAAl\xAF%\xE9>\xF7G\x08N9\xB8\xEA\x90\xAB\xF2;\xB4\x8C(\xFD_\x9B\xA7\xBAeP\"a\x01`\x82\x01QR\x7F\x03>\x19Z\x9E\xA3\xA9\xCE@\xB7+g:\xFBEDL\xA1\xB1_\x05C\xF4M\x10\xF5\xC6@\xA7\x80go` a\x01`\x83\x01Q\x01R\x7F\x0E\x8D\xB2\xB2\x89=\xF2=<U\xF0\xB3\xA3\xB5J\xB2\xE2\xEDw[1\xC4\xC9\x0EG.\xB3\x15\x82X-\xF2a\x01\x80\x82\x01QR\x7F\x0FLi\x89TQ\xAF\x15\x05*\xA8\x1A\x03\xEB\xA9u,\x9Ex\x91\xDD\x08\xE2d\xE0\xBDY=\x03W\x85\x8E` a\x01\x80\x83\x01Q\x01R\x7F+\xC0\x91\xFE\xFE$\x16\xEC\x83\xC6\xFB\xA8\xFB\x82\xFBH\xCB\x9C\xA0\x01\x82\xECy,!+\xF5S)X\xC8\xF7a\x01\xA0\x82\x01QR\x7F\x1FM\xCFM\xD4\x84\xE3\xAB\x1A\xE4\x87`=\"E9z\xC7\x8F\xE6\xCAu\x84d^\xBA9&\xD3\x83\x90+` a\x01\xA0\x83\x01Q\x01R\x7F-\xAFx\xC5\xA2\x82\x9A\x94\x18\x08\x1D\xD7\xE8t:h>\xD6\x81y\x96\xF7_\x10\0\x9D\x99\"\x07\x93\xECsa\x01\xC0\x82\x01QR\x7F\x19\xEB\x12\xA7\x82|\r\xDFc\x83\xFE\x80l9S\xBD\x06\xB0\x8A\xAE{\xF2\xA0\x1FU\xC9\x86\xA8OP\xCC(` a\x01\xC0\x83\x01Q\x01R\x7F\x01V\x910\x88F\xE6\x8E\xA8V\xA2\xCB$\xC9\x90?\x0C\x86\x05\xDE\xA1\x90\x82\x91\x80\xFFk\xDD\x1Ce\x08\x03a\x01\xE0\x82\x01QR\x7F\x1F\xFDx\x9B\x15[\x8A\xCB\x13\xE0\xF6\xA4\x8BP\xF7\xAA\x80\x92T\x08\x88\xD0\t\x14\x10W\xD4V\x90\x91X$` a\x01\xE0\x83\x01Q\x01R\x7F\x05E\xACz\xA6m\xCF7\x19\x98\x848\xC8\x06\xFCbM\xE5z\xB4?\x85\x809/\x88\xC8l\x13x\xCEJa\x02\0\x82\x01QR\x7F\x16\xB7\xF2P\x84.\xCFN6\x90pj\x1E\x15-zW\xF7\x0FUo\x92\x07m\xA7\x85\xFD\xD3c\xC1\x9F\xCF` a\x02\0\x83\x01Q\x01R\x7F \xCB\x7F\xF3Z\x83\xA7\xDC1@6\xE4p\xF1L0\xFB\x0E\x98\xD3]f;$;\",\xAAo\xC7\xDBDa\x02 \x82\x01QR\x7F\x14\x9FAWDpth\xBD\xAAN\x85E \x1A\xB4\r\x191\xA7\xD3\x1F#v\x8F\xA7\xC6Ut\xEE>\xAB` a\x02 \x83\x01Q\x01R\x7F\n%\xC1\xB7W9\x06\xDCN\x19;N\xA8/\xD1\xFE|\xCE\xBCM\x92]\xAD&\xF0\xFF\t\xC8L\x9F\x1Aua\x02@\x82\x01QR\x7F\nR\x1F\xF3\x0C\x8F6fy\x8F\x84|]L7\x96X\xFB\xA1\x01V\xE7\xA9I\x9F'\x13\xFA\xE9\xBF+\xE1` a\x02@\x83\x01Q\x01R\x7F\x03\xDBe\x10\xC3\xF16)\xFD\xED\x9AZ-AeK\xBC\xE4\xEFm\x02L\xADS\x10\0Q\xD4\xA3\xF3\xEB\xC9a\x02`\x82\x01QR\x7F\x08\xE8\n\\\x8EL\x9B\x9F&\xF3\0<\xC5\x94\x03\xA1\x8D16\xAF\xD00\x86\x8D%\xCC\x8B\x80~*\xB3p` a\x02`\x83\x01Q\x01R\x7F\xB0\x83\x88\x93\xEC\x1F#~\x8B\x072;\x07DY\x9FN\x97\xB5\x98\xB3\xB5\x89\xBC\xC2\xBC7\xB8\xD5\xC4\x18\x01a\x02\x80\x82\x01R\x7F\xC1\x83\x93\xC0\xFA0\xFEN\x8B\x03\x8E5z\xD8Q\xEA\xE8\xDE\x91\x07XN\xFF\xE7\xC7\xF1\xF6Q\xB2\x01\x0E&a\x02\xA0\x82\x01R\x90V[`\0a!z\x82a\"pV[a!\x8B\x83`\0[` \x02\x01Qa\x12;V[a!\x96\x83`\x01a!\x81V[a!\xA1\x83`\x02a!\x81V[a!\xAC\x83`\x03a!\x81V[a!\xB7\x83`\x04a!\x81V[a!\xC2\x83`\x05a!\x81V[a!\xCD\x83`\x06a!\x81V[a!\xD8\x83`\x07a!\x81V[a!\xE3\x84\x84\x84a#\xA8V[\x90P[\x93\x92PPPV[a!\xF6\x82a%\x86V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\";Wa\x161\x82\x82a%\xE2V[a\x08\xCBa&XV[a\"Ka&wV[a\r\xAEW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\xD3a\"CV[\x80Qa\"{\x90a&\x91V[a\"\x88\x81` \x01Qa&\x91V[a\"\x95\x81`@\x01Qa&\x91V[a\"\xA2\x81``\x01Qa&\x91V[a\"\xAF\x81`\x80\x01Qa&\x91V[a\"\xBC\x81`\xA0\x01Qa&\x91V[a\"\xC9\x81`\xC0\x01Qa&\x91V[a\"\xD6\x81`\xE0\x01Qa&\x91V[a\"\xE4\x81a\x01\0\x01Qa&\x91V[a\"\xF2\x81a\x01 \x01Qa&\x91V[a#\0\x81a\x01@\x01Qa&\x91V[a#\x0E\x81a\x01`\x01Qa&\x91V[a#\x1C\x81a\x01\x80\x01Qa&\x91V[a#*\x81a\x01\xA0\x01Qa\x12;V[a#8\x81a\x01\xC0\x01Qa\x12;V[a#F\x81a\x01\xE0\x01Qa\x12;V[a#T\x81a\x02\0\x01Qa\x12;V[a#b\x81a\x02 \x01Qa\x12;V[a#p\x81a\x02@\x01Qa\x12;V[a#~\x81a\x02`\x01Qa\x12;V[a#\x8C\x81a\x02\x80\x01Qa\x12;V[a#\x9A\x81a\x02\xA0\x01Qa\x12;V[a\x12\x06\x81a\x02\xC0\x01Qa\x12;V[`\0\x83` \x01Q`\x08\x14a#\xCFW`@Qc \xFA\x9D\x89`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a#\xDC\x85\x85\x85a'#V[\x90P`\0a#\xED\x86`\0\x01Qa,\x9AV[\x90P`\0a$\0\x82\x84`\xA0\x01Q\x88a0\xF1V[\x90Pa$\naA\xB9V[a$\x12aA\xB9V[a$3\x87a\x01`\x01Qa$.\x89a\x01\x80\x01Q\x88`\xE0\x01Qa1QV[a1\xE5V[\x91P`\0\x80a$D\x8B\x88\x87\x8Ca2\x80V[\x91P\x91Pa$U\x81a$.\x84a4\x84V[\x92Pa$n\x83a$.\x8Ba\x01`\x01Q\x8A`\xA0\x01Qa1QV[`\xA0\x88\x01Q`@\x88\x01Q` \x01Q\x91\x94P`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x91\x82\x90\x82\t\x90P\x81`\xE0\x8A\x01Q\x82\t\x90Pa$\xB2\x85a$.\x8Da\x01\x80\x01Q\x84a1QV[\x94P`\0`@Q\x80`\x80\x01`@R\x80\x7F\x01\x18\xC4\xD5\xB87\xBC\xC2\xBC\x89\xB5\xB3\x98\xB5\x97N\x9FYD\x07;2\x07\x8B~#\x1F\xEC\x93\x88\x83\xB0\x81R` \x01\x7F&\x0E\x01\xB2Q\xF6\xF1\xC7\xE7\xFFNX\x07\x91\xDE\xE8\xEAQ\xD8z5\x8E\x03\x8BN\xFE0\xFA\xC0\x93\x83\xC1\x81R` \x01\x7F\"\xFE\xBD\xA3\xC0\xC0c*VG[B\x14\xE5a^\x11\xE6\xDD?\x96\xE6\xCE\xA2\x85J\x87\xD4\xDA\xCC^U\x81R` \x01\x7F\x04\xFCci\xF7\x11\x0F\xE3\xD2QV\xC1\xBB\x9Ar\x85\x9C\xF2\xA0FA\xF9\x9B\xA4\xEEA<\x80\xDAj_\xE4\x81RP\x90Pa%t\x87\x82a%g\x89a4\x84V[a%oa4\xF3V[a5\xC4V[\x9E\x9DPPPPPPPPPPPPPPV[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a%\xB3W\x80`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x04\x01a\n9\x91\x90aD\x1DV[`\0\x80Q` aJ\xB5\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa%\xFF\x91\x90aJ\0V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a&:W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a&?V[``\x91P[P\x91P\x91Pa&O\x85\x83\x83a6\xA7V[\x95\x94PPPPPV[4\x15a\r\xAEW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a&\x81a\x16\xFFV[T`\x01`@\x1B\x90\x04`\xFF\x16\x91\x90PV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` aJU\x839\x81Q\x91R\x91\x15\x90\x15\x16\x15a&\xB9WPPPV[\x82Q` \x84\x01Q\x82`\x03\x84\x85\x85\x86\t\x85\t\x08\x83\x82\x83\t\x14\x83\x82\x10\x84\x84\x10\x16\x16\x93PPP\x81a\x161W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv\x10\x9B\x8C\x8DM\x0E\x88\x1A[\x9D\x98[\x1AY\x08\x11\xCCH\x1C\x1B\xDA[\x9D`J\x1B`D\x82\x01R`d\x01a\n9V[a'k`@Q\x80a\x01\0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x90P`@Q` \x81\x01`\0\x81R`\xFE`\xE0\x1B\x81R\x86Q`\xC0\x1B`\x04\x82\x01R` \x87\x01Q`\xC0\x1B`\x0C\x82\x01Ra\x02\x80\x87\x01Q` \x82\x01Ra\x02\xA0\x87\x01Q`@\x82\x01R`\x01``\x82\x01R`\0\x80Q` aKu\x839\x81Q\x91R`\x80\x82\x01R`\0\x80Q` aK\x15\x839\x81Q\x91R`\xA0\x82\x01R`\0\x80Q` aKU\x839\x81Q\x91R`\xC0\x82\x01R`\0\x80Q` aJ\x95\x839\x81Q\x91R`\xE0\x82\x01R`\xE0\x87\x01Q\x80Qa\x01\0\x83\x01R` \x81\x01Qa\x01 \x83\x01RPa\x01\0\x87\x01Q\x80Qa\x01@\x83\x01R` \x81\x01Qa\x01`\x83\x01RPa\x01 \x87\x01Q\x80Qa\x01\x80\x83\x01R` \x81\x01Qa\x01\xA0\x83\x01RPa\x01@\x87\x01Q\x80Qa\x01\xC0\x83\x01R` \x81\x01Qa\x01\xE0\x83\x01RPa\x01`\x87\x01Q\x80Qa\x02\0\x83\x01R` \x81\x01Qa\x02 \x83\x01RPa\x01\x80\x87\x01Q\x80Qa\x02@\x83\x01R` \x81\x01Qa\x02`\x83\x01RPa\x01\xE0\x87\x01Q\x80Qa\x02\x80\x83\x01R` \x81\x01Qa\x02\xA0\x83\x01RPa\x02\0\x87\x01Q\x80Qa\x02\xC0\x83\x01R` \x81\x01Qa\x02\xE0\x83\x01RPa\x02 \x87\x01Q\x80Qa\x03\0\x83\x01R` \x81\x01Qa\x03 \x83\x01RPa\x02@\x87\x01Q\x80Qa\x03@\x83\x01R` \x81\x01Qa\x03`\x83\x01RPa\x01\xA0\x87\x01Q\x80Qa\x03\x80\x83\x01R` \x81\x01Qa\x03\xA0\x83\x01RPa\x01\xC0\x87\x01Q\x80Qa\x03\xC0\x83\x01R` \x81\x01Qa\x03\xE0\x83\x01RPa\x02`\x87\x01Q\x80Qa\x04\0\x83\x01R` \x81\x01Qa\x04 \x83\x01RP`@\x87\x01Q\x80Qa\x04@\x83\x01R` \x81\x01Qa\x04`\x83\x01RP``\x87\x01Q\x80Qa\x04\x80\x83\x01R` \x81\x01Qa\x04\xA0\x83\x01RP`\x80\x87\x01Q\x80Qa\x04\xC0\x83\x01R` \x81\x01Qa\x04\xE0\x83\x01RP`\xA0\x87\x01Q\x80Qa\x05\0\x83\x01R` \x81\x01Qa\x05 \x83\x01RP`\xC0\x87\x01Q\x80Qa\x05@\x83\x01R` \x81\x01Qa\x05`\x83\x01RP\x85Qa\x05\x80\x82\x01R` \x86\x01Qa\x05\xA0\x82\x01R`@\x86\x01Qa\x05\xC0\x82\x01R``\x86\x01Qa\x05\xE0\x82\x01R`\x80\x86\x01Qa\x06\0\x82\x01R`\xA0\x86\x01Qa\x06 \x82\x01R`\xC0\x86\x01Qa\x06@\x82\x01R`\xE0\x86\x01Qa\x06`\x82\x01R\x84Q\x80Qa\x06\x80\x83\x01R` \x81\x01Qa\x06\xA0\x83\x01RP` \x85\x01Q\x80Qa\x06\xC0\x83\x01R` \x81\x01Qa\x06\xE0\x83\x01RP`@\x85\x01Q\x80Qa\x07\0\x83\x01R` \x81\x01Qa\x07 \x83\x01RP``\x85\x01Q\x80Qa\x07@\x83\x01R` \x81\x01Qa\x07`\x83\x01RP`\x80\x85\x01Q\x80Qa\x07\x80\x83\x01R` \x81\x01Qa\x07\xA0\x83\x01RP`\0\x82Ra\x07\xE0\x82 a\x07\xC0\x82\x01Ra\x07\xC0\x81\x01\x91P` \x82\x01\x90P\x82\x82Q\x06``\x85\x01R` \x82 \x81R\x80\x91P` \x82\x01\x90P\x82\x82Q\x06`\x80\x85\x01R`\xA0\x85\x01Q\x80Q\x82R` \x81\x01Q` \x83\x01RP``\x82 `@\x82\x01\x92P\x80\x83R` \x83\x01\x91P\x83\x81\x06\x85R\x83\x81\x82\t\x84\x82\x82\t\x91P\x80` \x87\x01RP\x80`@\x86\x01RP`\xC0\x85\x01Q\x80Q\x82R` \x81\x01Q` \x83\x01RP`\xE0\x85\x01Q\x80Q`@\x83\x01R` \x81\x01Q``\x83\x01RPa\x01\0\x85\x01Q\x80Q`\x80\x83\x01R` \x81\x01Q`\xA0\x83\x01RPa\x01 \x85\x01Q\x80Q`\xC0\x83\x01R` \x81\x01Q`\xE0\x83\x01RPa\x01@\x85\x01Q\x80Qa\x01\0\x83\x01R` \x81\x01Qa\x01 \x83\x01RPa\x01`\x82 a\x01@\x82\x01Ra\x01@\x81\x01\x91P` \x82\x01\x90P\x82\x82Q\x06`\xA0\x85\x01Ra\x01\xA0\x85\x01Q\x81Ra\x01\xC0\x85\x01Q` \x82\x01Ra\x01\xE0\x85\x01Q`@\x82\x01Ra\x02\0\x85\x01Q``\x82\x01Ra\x02 \x85\x01Q`\x80\x82\x01Ra\x02@\x85\x01Q`\xA0\x82\x01Ra\x02`\x85\x01Q`\xC0\x82\x01Ra\x02\x80\x85\x01Q`\xE0\x82\x01Ra\x02\xA0\x85\x01Qa\x01\0\x82\x01Ra\x02\xC0\x85\x01Qa\x01 \x82\x01Ra\x01`\x82 a\x01@\x82\x01Ra\x01@\x81\x01\x91P` \x82\x01\x90P\x82\x82Q\x06`\xC0\x85\x01Ra\x01`\x85\x01Q\x80Q\x82R` \x81\x01Q` \x83\x01RPa\x01\x80\x85\x01Q\x80Q`@\x83\x01R` \x81\x01Q``\x83\x01RPP`\xA0\x81 \x82\x81\x06`\xE0\x85\x01RPPP\x93\x92PPPV[a,\xA2aA\xD3V[\x81b\x01\0\0\x03a.\x08W`@Q\x80``\x01`@R\x80`\x10\x81R` \x01\x7F0d\x1E\x0E\x92\xBE\xBE\xF8\x18&\x8Df;\xCA\xD6\xDB\xCF\xD6\xC0\x14\x91p\xF6\xD7\xD3P\xB1\xB1\xFAl\x10\x01\x81R` \x01`@Q\x80a\x01\0\x01`@R\x80`\x01\x81R` \x01~\xEE\xB2\xCBY\x81\xEDEd\x9A\xBE\xBD\xE0\x81\xDC\xFF\x16\xC8`\x1D\xE44~}\xD1b\x8B\xA2\xDA\xACC\xB7\x81R` \x01\x7F-\x1B\xA6oYA\xDC\x91\x01qq\xFAi\xEC+\xD0\x02**-A\x15\xA0\t\xA94X\xFDN&\xEC\xFB\x81R` \x01\x7F\x08h\x12\xA0\n\xC4>\xA8\x01f\x9Cd\x01q <A\xA4\x96g\x1B\xFB\xC0e\xAC\x8D\xB2MR\xCF1\xE5\x81R` \x01\x7F-\x96VQ\xCD\xD9\xE4\x81\x1FNQ\xB8\r\xDC\xA8\xA8\xB4\xA9>\xE1t \xAA\xE6\xAD\xAA\x01\xC2a|n\x85\x81R` \x01\x7F\x12YzV\xC2\xE48b\x0B\x90A\xB9\x89\x92\xAE\rNp[x\0W\xBFwf\xA2v|\xEC\xE1n\x1D\x81R` \x01\x7F\x02\xD9A\x17\xCD\x17\xBC\xF1)\x0F\xD6|\x01\x15]\xD4\x08\x07\x85}\xFFJZ\x0BM\xC6{\xEF\xA8\xAA4\xFD\x81R` \x01\x7F\x15\xEE$u\xBE\xE5\x17\xC4\xEE\x05\xE5\x1F\xA1\xEEs\x12\xA87:\x0B\x13\xDB\x8CQ\xBA\xF0L\xB2\xE9\x9B\xD2\xBD\x81RP\x81RP\x90P\x91\x90PV[\x81b\x10\0\0\x03a/oW`@Q\x80``\x01`@R\x80`\x14\x81R` \x01\x7F0dKl\x9CJr\x16\x9EM\xAA1}%\xF0E\x12\xAE\x15\xC5;4\xE8\xF5\xAC\xD8\xE1U\xD0\xA6\xC1\x01\x81R` \x01`@Q\x80a\x01\0\x01`@R\x80`\x01\x81R` \x01\x7F&\x12]\xA1\n\x0E\xD0c'P\x8A\xBA\x06\xD1\xE3\x03\xACaf2\xDB\xED4\x9FSB-\xA9S3xW\x81R` \x01\x7F\"`\xE7$\x84K\xCARQ\x82\x93S\x96\x8EI\x150RXA\x83WG:\\\x1DY\x7Fa?l\xBD\x81R` \x01\x7F \x87\xEA,\xD6d'\x86\x08\xFB\x0E\xBD\xB8 \x90\x7FY\x85\x02\xC8\x1Bf\x90\xC1\x85\xE2\xBF\x15\xCB\x93_B\x81R` \x01\x7F\x19\xDD\xBC\xAF:\x8DF\xC1\\\x01v\xFB\xB5\xB9^M\xC5p\x88\xFF\x13\xF4\xD1\xBD\x84\xC6\xBF\xA5}\xCD\xC0\xE0\x81R` \x01\x7F\x05\xA2\xC8\\\xFCY\x17\x89`\\\xAE\x81\x8E7\xDDAa\xEE\xF9\xAAfk\xECo\xE4(\x8D\t\xE6\xD24\x18\x81R` \x01\x7F\x11\xF7\x0ESc%\x8F\xF4\xF0\xD7\x16\xA6S\xE1\xDCA\xF1\xC6D\x84\xD7\xF4\xB6\xE2\x19\xD67v\x14\xA3\x90\\\x81R` \x01\x7F)\xE8AC\xF5\x87\rGv\xA9-\xF8\xDA\x8Cl\x93\x03\xD5\x90\x88\xF3{\xA8_@\xCFo\xD1Be\xB4\xBC\x81RP\x81RP\x90P\x91\x90PV[\x81` \x03a0\xD3W`@Q\x80``\x01`@R\x80`\x05\x81R` \x01\x7F.\xE1+\xFFJ(\x13(j\x8D\xC3\x88\xCDuM\x9A>\xF2I\x065\xEB\xA5\x0C\xB9\xC2\xE5\xE7P\x80\0\x01\x81R` \x01`@Q\x80a\x01\0\x01`@R\x80`\x01\x81R` \x01\x7F\t\xC52\xC60k\x93\xD2\x96x \rG\xC0\xB2\xA9\x9C\x18\xD5\x1B\x83\x8E\xEB\x1D>\xEDLS;\xB5\x12\xD0\x81R` \x01\x7F!\x08,\xA2\x16\xCB\xBFN\x1CnOE\x94\xDDP\x8C\x99m\xFB\xE1\x17N\xFB\x98\xB1\x15\t\xC6\xE3\x06F\x0B\x81R` \x01\x7F\x12w\xAEd\x15\xF0\xEF\x18\xF2\xBA_\xB1b\xC3\x9E\xB71\x1F8n-&\xD6D\x01\xF4\xA2]\xA7|%;\x81R` \x01\x7F+3}\xE1\xC8\xC1O\"\xEC\x9B\x9E/\x96\xAF\xEF6Rbsf\xF8\x17\n\n\x94\x8D\xADJ\xC1\xBD^\x80\x81R` \x01\x7F/\xBDM\xD2\x97k\xE5]\x1A\x16:\xA9\x82\x0F\xB8\x8D\xFA\xC5\xDD\xCEw\xE1\x87.\x90c '2z^\xBE\x81R` \x01\x7F\x10z\xABI\xE6Zg\xF9\xDA\x9C\xD2\xAB\xF7\x8B\xE3\x8B\xD9\xDC\x1D]\xB3\x9F\x81\xDE6\xBC\xFA[K\x03\x90C\x81R` \x01~\xE1Kcd\xA4~\x9CB\x84\xA9\xF8\n_\xC4\x1C\xD2\x12\xB0\xD4\xDB\xF8\xA5p7p\xA4\n\x9A49\x90\x81RP\x81RP\x90P\x91\x90PV[`@Qc\xE2\xEF\t\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a1\x15`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a1\x1F\x84\x84a6\xFAV[\x80\x82Ra1/\x90\x85\x90\x85\x90a7NV[` \x82\x01R\x80Qa1E\x90\x85\x90\x84\x90\x86\x90a7\xC2V[`@\x82\x01R\x93\x92PPPV[a1YaA\xB9V[a1aaA\xF9V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x81\x01\x83\x90R`\0``\x83`\x80\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a1\x93W`\0\x80\xFD[P\x80a1\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01RxBn254: scalar mul failed!`8\x1B`D\x82\x01R`d\x01a\n9V[PP\x92\x91PPV[a1\xEDaA\xB9V[a1\xF5aB\x17V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x83\x01R\x83\x01Q``\x80\x83\x01\x91\x90\x91R`\0\x90\x83`\xC0\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a22W`\0\x80\xFD[P\x80a1\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FBn254: group addition failed!\0\0\0`D\x82\x01R`d\x01a\n9V[a2\x88aA\xB9V[a2\x90aA\xB9V[`\0a2\x9E\x87\x87\x87\x87a9\x18V[\x90P`\0\x80Q` aJ\xD5\x839\x81Q\x91R`\0a2\xBC\x88\x87\x89a=\x94V[\x90Pa2\xC8\x81\x83aI\x96V[`\xC0\x89\x01Qa\x01\xA0\x88\x01Q\x91\x92P\x90\x81\x90\x84\x90\x81\x90\x83\t\x84\x08\x92Pa2\xF5\x85a$.\x8A`\0\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x01\xC0\x8A\x01Q\x83\t\x84\x08\x92Pa3\x1D\x86a$.\x8A` \x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x01\xE0\x8A\x01Q\x83\t\x84\x08\x92Pa3E\x86a$.\x8A`@\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x02\0\x8A\x01Q\x83\t\x84\x08\x92Pa3m\x86a$.\x8A``\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x02 \x8A\x01Q\x83\t\x84\x08\x92Pa3\x95\x86a$.\x8A`\x80\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x02@\x8A\x01Q\x83\t\x84\x08\x92Pa3\xBD\x86a$.\x8D`@\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x02`\x8A\x01Q\x83\t\x84\x08\x92Pa3\xE5\x86a$.\x8D``\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x02\x80\x8A\x01Q\x83\t\x84\x08\x92Pa4\r\x86a$.\x8D`\x80\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x02\xA0\x8A\x01Q\x83\t\x84\x08\x92Pa45\x86a$.\x8D`\xA0\x01Q\x84a1QV[\x95P`\0\x8A`\xE0\x01Q\x90P\x84\x85a\x02\xC0\x8B\x01Q\x83\t\x85\x08\x93Pa4`\x87a$.\x8B`\xA0\x01Q\x84a1QV[\x96Pa4sa4ma>\x84V[\x85a1QV[\x97PPPPPPP\x94P\x94\x92PPPV[a4\x8CaA\xB9V[\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a4\xA0WP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aJU\x839\x81Q\x91R\x84` \x01Qa4\xD3\x91\x90aJ2V[a4\xEB\x90`\0\x80Q` aJU\x839\x81Q\x91RaI\x96V[\x90R\x92\x91PPV[a5\x1E`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R` \x87\x01Q`@\x82\x01R\x86Q``\x82\x01R``\x87\x01Q`\x80\x82\x01R`@\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R` \x85\x01Qa\x01\0\x82\x01R\x84Qa\x01 \x82\x01R``\x85\x01Qa\x01@\x82\x01R`@\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a6\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R{Bn254: Pairing check failed!` \x1B`D\x82\x01R`d\x01a\n9V[P\x15\x15\x90P[\x94\x93PPPPV[``\x82a6\xBCWa6\xB7\x82a>\xA5V[a!\xE6V[\x81Q\x15\x80\x15a6\xD3WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a6\xF3W\x83`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x04\x01a\n9\x91\x90aD\x1DV[P\x80a!\xE6V[\x81Q`\0\x90`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x90\x83\x80\x15a7>W\x84\x93P`\0[\x82\x81\x10\x15a72W\x83\x85\x86\t\x94P`\x01\x01a7\x1CV[P`\x01\x84\x03\x93Pa7EV[`\x01\x83\x03\x93P[PPP\x92\x91PPV[`\0\x82`\x01\x03a7`WP`\x01a!\xE6V[\x81`\0\x03a7pWP`\0a!\xE6V[` \x84\x01Q`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x90`\0\x90\x82\x81\x86\t\x90P\x85\x80\x15a7\xA0W`\x01\x87\x03\x92Pa7\xA7V[`\x01\x84\x03\x92P[Pa7\xB1\x82a>\xCEV[\x91P\x82\x82\x82\t\x97\x96PPPPPPPV[`\0`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x82\x82\x03a8?W`\x01`\0[`\x08\x81\x10\x15a83W\x81\x86\x03a8\x10W\x86\x81`\x08\x81\x10a8\x01Wa8\x01aI\x80V[` \x02\x01Q\x93PPPPa6\x9FV[\x82\x80a8\x1EWa8\x1EaJ\x1CV[`@\x89\x01Q` \x01Q\x83\t\x91P`\x01\x01a7\xDFV[P`\0\x92PPPa6\x9FV[a8Ga@\x87V[`@\x87\x01Q`\x01`\xE0\x83\x81\x01\x82\x81R\x92\x01\x90\x80[`\x08\x81\x10\x15a8\x88W` \x84\x03\x93P\x85\x86\x8A\x85Q\x89\x03\x08\x83\t\x80\x85R`\x1F\x19\x90\x93\x01\x92\x91P`\x01\x01a8[V[PPPP`\0\x80`\0\x90P`\x01\x83\x89`@\x8C\x01Q`\0[`\x08\x81\x10\x15a8\xDFW\x88\x82Q\x8A\x85Q\x8C\x88Q\x8A\t\t\t\x89\x81\x88\x08\x96PP\x88\x89\x8D\x84Q\x8C\x03\x08\x86\t\x94P` \x93\x84\x01\x93\x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a8\x9FV[PPPP\x80\x92PP`\0a8\xF2\x83a>\xCEV[\x90P` \x8A\x01Q\x85\x81\x89\t\x96PP\x84\x81\x87\t\x95P\x84\x82\x87\t\x9A\x99PPPPPPPPPPV[a9 aA\xB9V[`\0\x80`\0\x80`\0`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x90P`\x80\x89\x01Q\x81` \x8A\x01Q` \x8C\x01Q\t\x95P\x89Q\x94P\x81`\xA0\x8B\x01Q``\x8C\x01Q\t\x93P\x81a\x01\xA0\x89\x01Q\x85\x08\x92P\x81\x81\x84\x08\x92P\x81\x85\x84\t\x94P\x81`\0\x80Q` aKu\x839\x81Q\x91R\x85\t\x92P\x81a\x01\xC0\x89\x01Q\x84\x08\x92P\x81\x81\x84\x08\x92P\x81\x85\x84\t\x94P\x81`\0\x80Q` aK\x15\x839\x81Q\x91R\x85\t\x92P\x81a\x01\xE0\x89\x01Q\x84\x08\x92P\x81\x81\x84\x08\x92P\x81\x85\x84\t\x94P\x81`\0\x80Q` aKU\x839\x81Q\x91R\x85\t\x92P\x81a\x02\0\x89\x01Q\x84\x08\x92P\x81\x81\x84\x08\x92P\x81\x85\x84\t\x94P\x81`\0\x80Q` aJ\x95\x839\x81Q\x91R\x85\t\x92P\x81a\x02 \x89\x01Q\x84\x08\x92P\x81\x81\x84\x08\x92PP\x80\x84\x83\t\x93P\x80\x84\x86\x08\x94Pa:6\x87`\xA0\x01Q\x86a1QV[\x95P\x88Q``\x8A\x01Q`\x80\x8B\x01Q\x83\x82\x84\t\x97P\x83a\x02\xC0\x8B\x01Q\x89\t\x97P\x83a\x02@\x8B\x01Q\x83\t\x95P\x83a\x01\xA0\x8B\x01Q\x87\x08\x95P\x83\x81\x87\x08\x95P\x83\x86\x89\t\x97P\x83a\x02`\x8B\x01Q\x83\t\x95P\x83a\x01\xC0\x8B\x01Q\x87\x08\x95P\x83\x81\x87\x08\x95P\x83\x86\x89\t\x97P\x83a\x02\x80\x8B\x01Q\x83\t\x95P\x83a\x01\xE0\x8B\x01Q\x87\x08\x95P\x83\x81\x87\x08\x95P\x83\x86\x89\t\x97P\x83a\x02\xA0\x8B\x01Q\x83\t\x95P\x83a\x02\0\x8B\x01Q\x87\x08\x95P\x83\x81\x87\x08\x95PPPP\x80\x83\x86\t\x94Pa:\xFD\x86a$.\x8C`\xC0\x01Q\x88\x85a:\xF8\x91\x90aI\x96V[a1QV[\x95Pa;\x16\x86a$.\x8C`\xE0\x01Q\x8Aa\x01\xA0\x01Qa1QV[\x95Pa;0\x86a$.\x8Ca\x01\0\x01Q\x8Aa\x01\xC0\x01Qa1QV[\x95Pa;J\x86a$.\x8Ca\x01 \x01Q\x8Aa\x01\xE0\x01Qa1QV[\x95Pa;d\x86a$.\x8Ca\x01@\x01Q\x8Aa\x02\0\x01Qa1QV[\x95P\x80a\x01\xC0\x88\x01Qa\x01\xA0\x89\x01Q\t\x92Pa;\x89\x86a$.\x8Ca\x01`\x01Q\x86a1QV[\x95P\x80a\x02\0\x88\x01Qa\x01\xE0\x89\x01Q\t\x92Pa;\xAE\x86a$.\x8Ca\x01\x80\x01Q\x86a1QV[\x95Pa\x01\xA0\x87\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92Pa;\xDD\x86a$.\x8Ca\x01\xE0\x01Q\x86a1QV[\x95Pa\x01\xC0\x87\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92Pa<\x0C\x86a$.\x8Ca\x02\0\x01Q\x86a1QV[\x95Pa\x01\xE0\x87\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92Pa<;\x86a$.\x8Ca\x02 \x01Q\x86a1QV[\x95Pa\x02\0\x87\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92Pa<j\x86a$.\x8Ca\x02@\x01Q\x86a1QV[\x95Pa<\x87\x86a$.\x8Ca\x01\xA0\x01Qa:\xF8\x8Ba\x02 \x01Qa?tV[\x95Pa<\x98\x86\x8Ba\x01\xC0\x01Qa1\xE5V[\x95P\x80a\x01\xC0\x88\x01Qa\x01\xA0\x89\x01Q\t\x92P\x80a\x01\xE0\x88\x01Q\x84\t\x92P\x80a\x02\0\x88\x01Q\x84\t\x92P\x80a\x02 \x88\x01Q\x84\t\x92Pa<\xDE\x86a$.\x8Ca\x02`\x01Q\x86a1QV[\x95Pa<\xED\x88`\0\x01Qa?tV[\x94Pa=\x01\x86a$.\x89`\xC0\x01Q\x88a1QV[\x95P\x80`\x01\x89Q\x08`\xA0\x8A\x01Q\x90\x93P\x81\x90\x80\t\x91P\x80\x82\x84\t\x92P\x80\x83\x86\t\x94Pa=5\x86a$.\x89`\xE0\x01Q\x88a1QV[\x95P\x80\x83\x86\t\x94Pa=P\x86a$.\x89a\x01\0\x01Q\x88a1QV[\x95P\x80\x83\x86\t\x94Pa=k\x86a$.\x89a\x01 \x01Q\x88a1QV[\x95P\x80\x83\x86\t\x94Pa=\x86\x86a$.\x89a\x01@\x01Q\x88a1QV[\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x90P`\0\x83` \x01Q\x90P`\0\x84`@\x01Q\x90P`\0`\x01\x90P``\x88\x01Q`\x80\x89\x01Qa\x01\xA0\x89\x01Qa\x02@\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x01\xC0\x89\x01Qa\x02`\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x01\xE0\x89\x01Qa\x02\x80\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x02\0\x89\x01Qa\x02\xA0\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x02 \x89\x01Q\x91Pa\x02\xC0\x89\x01Q\x86\x87\x82\x89\x85\x87\x08\t\x85\t\x93PPPP\x87Q` \x89\x01Q\x85\x86\x86\x83\t\x87\x03\x85\x08\x96PP\x84\x85\x83\x83\t\x86\x03\x87\x08\x99\x98PPPPPPPPPV[a>\x8CaA\xB9V[P`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x02` \x82\x01R\x90V[\x80Q\x15a>\xB5W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01R`\x02\x82\x03`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x92PP`\0Q\x92P\x81a?mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FBn254: pow precompile failed!\0\0\0`D\x82\x01R`d\x01a\n9V[PP\x91\x90PV[`\0a?\x8E`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x83aJ2V[a\x11\xC5\x90`\0\x80Q` aJ\xD5\x839\x81Q\x91RaI\x96V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x12\x06\x91\x90aB5V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a?\xFFW\x91` \x02\x82\x01[\x82\x81\x11\x15a?\xFFW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a?\xE4V[Pa@\x0B\x92\x91PaB5V[P\x90V[`@Q\x80a\x01\0\x01`@R\x80`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P\x80T`\0\x82U`\x02\x02\x90`\0R` `\0 \x90\x81\x01\x90a\x12\x06\x91\x90aBJV[`@Q\x80a\x01\0\x01`@R\x80`\x08\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x02\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01a@\xC8aA\xB9V[\x81R` \x01a@\xD5aA\xB9V[\x81R` \x01a@\xE2aA\xB9V[\x81R` \x01a@\xEFaA\xB9V[\x81R` \x01a@\xFCaA\xB9V[\x81R` \x01aA\taA\xB9V[\x81R` \x01aA\x16aA\xB9V[\x81R` \x01aA#aA\xB9V[\x81R` \x01aA0aA\xB9V[\x81R` \x01aA=aA\xB9V[\x81R` \x01aAJaA\xB9V[\x81R` \x01aAWaA\xB9V[\x81R` \x01aAdaA\xB9V[\x81R` \x01aAqaA\xB9V[\x81R` \x01aA~aA\xB9V[\x81R` \x01aA\x8BaA\xB9V[\x81R` \x01aA\x98aA\xB9V[\x81R` \x01aA\xA5aA\xB9V[\x81R`\0` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01aA\xF4a@\x87V[\x90R\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a@\x0BW`\0\x81U`\x01\x01aB6V[[\x80\x82\x11\x15a@\x0BW\x80T`\x01`\x01`@\x1B\x03\x19\x16\x81U`\0`\x01\x82\x01U`\x02\x01aBKV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a0\xECW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aB\x99W`\0\x80\xFD[a\x11\xC2\x82aBpV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\xDAWaB\xDAaB\xA2V[`@R\x90V[`@Qa\x02\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\xDAWaB\xDAaB\xA2V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC+WaC+aB\xA2V[`@R\x91\x90PV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a0\xECW`\0\x80\xFD[`\0a\x01\0\x80\x83\x85\x03\x12\x15aC^W`\0\x80\xFD[`@Q\x90\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x81\x83\x10\x17\x15aC\x80WaC\x80aB\xA2V[\x81`@R\x80\x92PaC\x90\x84aC3V[\x81RaC\x9E` \x85\x01aC3V[` \x82\x01R`@\x84\x015`@\x82\x01R``\x84\x015``\x82\x01R`\x80\x84\x015`\x80\x82\x01R`\xA0\x84\x015`\xA0\x82\x01R`\xC0\x84\x015`\xC0\x82\x01R`\xE0\x84\x015`\xE0\x82\x01RPP\x92\x91PPV[`\0a\x01\0\x82\x84\x03\x12\x15aC\xFAW`\0\x80\xFD[a\x11\xC2\x83\x83aCJV[`\0` \x82\x84\x03\x12\x15aD\x16W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aDJWaDJaB\xA2V[P`\x05\x1B` \x01\x90V[`\0` \x80\x83\x85\x03\x12\x15aDgW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aD}W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aD\x8EW`\0\x80\xFD[\x805aD\xA1aD\x9C\x82aD1V[aC\x03V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aD\xC0W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aD\xDEW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aD\xC5V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15aD\xFBW`\0\x80\xFD[a\x11\xC2\x82aC3V[`\0`@\x82\x84\x03\x12\x15aE\x16W`\0\x80\xFD[aE\x1EaB\xB8V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80\x82\x84\x03a\x05\x80\x81\x12\x15aEIW`\0\x80\xFD[aES\x85\x85aCJV[\x92Pa\x01\0a\x04\x80\x80`\xFF\x19\x84\x01\x12\x15aElW`\0\x80\xFD[aEtaB\xE0V[\x92PaE\x82\x87\x83\x88\x01aE\x04V[\x83Ra\x01@aE\x93\x88\x82\x89\x01aE\x04V[` \x85\x01Ra\x01\x80aE\xA7\x89\x82\x8A\x01aE\x04V[`@\x86\x01Ra\x01\xC0aE\xBB\x8A\x82\x8B\x01aE\x04V[``\x87\x01Ra\x02\0aE\xCF\x8B\x82\x8C\x01aE\x04V[`\x80\x88\x01Ra\x02@aE\xE3\x8C\x82\x8D\x01aE\x04V[`\xA0\x89\x01Ra\x02\x80aE\xF7\x8D\x82\x8E\x01aE\x04V[`\xC0\x8A\x01Ra\x02\xC0aF\x0B\x8E\x82\x8F\x01aE\x04V[`\xE0\x8B\x01RaF\x1E\x8Ea\x03\0\x8F\x01aE\x04V[\x89\x8B\x01RaF0\x8Ea\x03@\x8F\x01aE\x04V[a\x01 \x8B\x01RaFD\x8Ea\x03\x80\x8F\x01aE\x04V[\x87\x8B\x01RaFV\x8Ea\x03\xC0\x8F\x01aE\x04V[a\x01`\x8B\x01RaFj\x8Ea\x04\0\x8F\x01aE\x04V[\x86\x8B\x01Ra\x04@\x8D\x015a\x01\xA0\x8B\x01Ra\x04`\x8D\x015\x85\x8B\x01R\x87\x8D\x015a\x01\xE0\x8B\x01Ra\x04\xA0\x8D\x015\x84\x8B\x01Ra\x04\xC0\x8D\x015a\x02 \x8B\x01Ra\x04\xE0\x8D\x015\x83\x8B\x01Ra\x05\0\x8D\x015a\x02`\x8B\x01Ra\x05 \x8D\x015\x82\x8B\x01Ra\x05@\x8D\x015a\x02\xA0\x8B\x01Ra\x05`\x8D\x015\x81\x8B\x01RPPPPPPPPP\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aG\x01W`\0\x80\xFD[aG\n\x83aBpV[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aG'W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12aG;W`\0\x80\xFD[\x815\x81\x81\x11\x15aGMWaGMaB\xA2V[aG_`\x1F\x82\x01`\x1F\x19\x16\x85\x01aC\x03V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15aGuW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0` \x80\x83\x85\x03\x12\x15aG\xA6W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xBCW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aG\xCDW`\0\x80\xFD[\x805aG\xDBaD\x9C\x82aD1V[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aG\xFAW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aD\xDEW`@\x84\x89\x03\x12\x15aH\x18W`\0\x80\x81\xFD[aH aB\xB8V[aH)\x85aC3V[\x81R\x84\x86\x015\x86\x82\x01R\x82R`@\x90\x93\x01\x92\x90\x84\x01\x90aG\xFFV[`\x01`\x01`@\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a0\xECW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aH~W`\0\x80\xFD[a\x11\xC2\x82aHXV[`\0\x80`\0a\x01@\x84\x86\x03\x12\x15aH\x9DW`\0\x80\xFD[aH\xA7\x85\x85aCJV[\x92PaH\xB6a\x01\0\x85\x01aHXV[\x91PaH\xC5a\x01 \x85\x01aBpV[\x90P\x92P\x92P\x92V[`\0[\x83\x81\x10\x15aH\xE9W\x81\x81\x01Q\x83\x82\x01R` \x01aH\xD1V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01RaI\x11\x81`@\x85\x01` \x87\x01aH\xCEV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aI8W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x80\x82\x16\x91\x90\x82\x81\x14a1\xDDWa1\xDDaIGV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x11\xC5Wa\x11\xC5aIGV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15aI\xC9WaI\xC9aIGV[P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aI\xE2W`\0\x80\xFD[PQ\x91\x90PV[`\0\x81aI\xF8WaI\xF8aIGV[P`\0\x19\x01\x90V[`\0\x82QaJ\x12\x81\x84` \x87\x01aH\xCEV[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aJOWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88.+\x91Ea\x03i\x8A\xDFW\xB7\x99\x96\x9D\xEA\x1C\x8Fs\x9D\xA5\xD8\xD4\r\xD3\xEB\x92\"\xDB|\x81\xE8\x816\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE3\x1E\xE6x\xA0G\nu\xA6\xEA\xA8\xFE\x83p`I\x8B\xA8(\xA3p;1\x1D\x0Fw\xF0\x10BJ\xFE\xB0%\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE4 B\xA5\x87\xA9\x0C\x18{\n\x08|\x03\xE2\x9C\x96\x8B\x95\x0B\x1D\xB2m\\\x82\xD6f\x90Zh\x95y\x0C\n/\x8D\xD1\xF1\xA7X<B\xC4\xE1*D\xE1\x10@Ls\xCAl\x94\x81?\x85\x83]\xA4\xFB{\xB10\x1DJ\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static LIGHTCLIENTMOCK_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\x94W`\x005`\xE0\x1C\x80c\x01?\xA5\xFC\x14a\x01\x99W\x80c\r\x8En,\x14a\x01\xBBW\x80c *\n\xDB\x14a\x01\xEDW\x80c-R\xAA\xD6\x14a\x02\x96W\x80c1=\xF7\xB1\x14a\x02\xC3W\x80c8+!Z\x14a\x02\xF0W\x80c9\x194\x0F\x14a\x03\x14W\x80c9I\xD1\xE9\x14a\x034W\x80c@\x999\xB7\x14a\x03{W\x80cHG\xAE]\x14a\x03\x9BW\x80cO\x1E\xF2\x86\x14a\x04\x1DW\x80cR\xD1\x90-\x14a\x040W\x80cS\x0C\xA7\x8F\x14a\x04EW\x80cTd`\x85\x14a\x04eW\x80cb\x82w3\x14a\x04zW\x80ci\xCCj\x04\x14a\x04\x90W\x80cpS\xFCQ\x14a\x04\xA5W\x80cqP\x18\xA6\x14a\x04\xBAW\x80cvg\x18\x08\x14a\x04\xCFW\x80cv\xB6\xB7\xCB\x14a\x05\x03W\x80c\x7F\x17\xBA\xAD\x14a\x05\x19W\x80c\x82\xD0\x7F\xF3\x14a\x05\xCCW\x80c\x85\x84\xD2?\x14a\x05\xE1W\x80c\x8D\xA5\xCB[\x14a\x06%W\x80c\xA2D\xD5\x96\x14a\x06:W\x80c\xA5\x1Eo\xEA\x14a\x06ZW\x80c\xAA\x92'2\x14a\x06zW\x80c\xAD<\xB1\xCC\x14a\x06\x9AW\x80c\xBD2Q\x9A\x14a\x06\xD8W\x80c\xC8\xE5\xE4\x98\x14a\x07\tW\x80c\xCAo\xE8U\x14a\x07%W\x80c\xDB\x13\xB6\n\x14a\x07;W\x80c\xE003\x01\x14a\x07zW\x80c\xF0h T\x14a\x07\x9AW\x80c\xF2\xFD\xE3\x8B\x14a\x07\xCCW[`\0\x80\xFD[4\x80\x15a\x01\xA5W`\0\x80\xFD[Pa\x01\xB9a\x01\xB46`\x04aB\x87V[a\x07\xECV[\0[4\x80\x15a\x01\xC7W`\0\x80\xFD[P`@\x80Q`\x01\x81R`\0` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF9W`\0\x80\xFD[Pa\x01\xB9a\x02\x086`\x04aC\xE7V[`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` \x81\x81R`@\x93\x84\x90 \x85Q\x81T\x92\x87\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x95\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x94\x16\x93\x90\x93\x17\x17\x82U\x91\x83\x01Q`\x01\x82\x01U``\x83\x01Q`\x02\x82\x01U`\x80\x83\x01Q`\x03\x82\x01U`\xA0\x83\x01Q`\x04\x82\x01U`\xC0\x83\x01Q\x91\x81\x01\x91\x90\x91U`\xE0\x90\x91\x01Q`\x06\x90\x91\x01UV[4\x80\x15a\x02\xA2W`\0\x80\xFD[Pa\x01\xB9a\x02\xB16`\x04aD\x04V[`\t\x80T`\xFF\x19\x16`\x01\x17\x90U`\nUV[4\x80\x15a\x02\xCFW`\0\x80\xFD[P`\x06Ta\x02\xE3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qa\x01\xE4\x91\x90aD\x1DV[4\x80\x15a\x02\xFCW`\0\x80\xFD[Pa\x03\x06`\x03T\x81V[`@Q\x90\x81R` \x01a\x01\xE4V[4\x80\x15a\x03 W`\0\x80\xFD[Pa\x01\xB9a\x03/6`\x04aDTV[a\x08\xACV[4\x80\x15a\x03@W`\0\x80\xFD[Pa\x01\xB9a\x03O6`\x04aD\xE9V[`\0\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01``\x1B\x02`\x01``\x1B`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x80\x15a\x03\x87W`\0\x80\xFD[Pa\x01\xB9a\x03\x966`\x04aE4V[a\x08\xCFV[4\x80\x15a\x03\xA7W`\0\x80\xFD[Pa\x03\xB0a\x0B\xFAV[`@Qa\x01\xE4\x91\x90`\0a\x01\0\x82\x01\x90P`\x01\x80`@\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01R\x92\x91PPV[a\x01\xB9a\x04+6`\x04aF\xEEV[a\x0C\x8CV[4\x80\x15a\x04<W`\0\x80\xFD[Pa\x03\x06a\x0C\xA7V[4\x80\x15a\x04QW`\0\x80\xFD[Pa\x01\xB9a\x04`6`\x04aG\x93V[a\x0C\xC4V[4\x80\x15a\x04qW`\0\x80\xFD[P`\x08Ta\x03\x06V[4\x80\x15a\x04\x86W`\0\x80\xFD[Pa\x03\x06`\x02T\x81V[4\x80\x15a\x04\x9CW`\0\x80\xFD[Pa\x01\xB9a\r@V[4\x80\x15a\x04\xB1W`\0\x80\xFD[P`\x07Ta\x03\x06V[4\x80\x15a\x04\xC6W`\0\x80\xFD[Pa\x01\xB9a\r\xB0V[4\x80\x15a\x04\xDBW`\0\x80\xFD[P`\0Ta\x04\xF6\x90`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Qa\x01\xE4\x91\x90aHDV[4\x80\x15a\x05\x0FW`\0\x80\xFD[Pa\x03\x06`\x01T\x81V[4\x80\x15a\x05%W`\0\x80\xFD[Pa\x05\x86a\x0546`\x04aHlV[`\x05` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T\x95\x85\x01T`\x06\x90\x95\x01T`\x01`\x01`@\x1B\x03\x80\x86\x16\x97`\x01`@\x1B\x90\x96\x04\x16\x95\x93\x94\x92\x93\x91\x92\x91\x90\x88V[`@\x80Q`\x01`\x01`@\x1B\x03\x99\x8A\x16\x81R\x98\x90\x97\x16` \x89\x01R\x95\x87\x01\x94\x90\x94R``\x86\x01\x92\x90\x92R`\x80\x85\x01R`\xA0\x84\x01R`\xC0\x83\x01R`\xE0\x82\x01Ra\x01\0\x01a\x01\xE4V[4\x80\x15a\x05\xD8W`\0\x80\xFD[Pa\x03\xB0a\r\xC2V[4\x80\x15a\x05\xEDW`\0\x80\xFD[Pa\x06\x01a\x05\xFC6`\x04aD\x04V[a\x0ERV[`@\x80Q\x82Q`\x01`\x01`@\x1B\x03\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x01\xE4V[4\x80\x15a\x061W`\0\x80\xFD[Pa\x02\xE3a\x0F\xACV[4\x80\x15a\x06FW`\0\x80\xFD[Pa\x01\xB9a\x06U6`\x04aH\x87V[a\x0F\xC7V[4\x80\x15a\x06fW`\0\x80\xFD[Pa\x03\x06a\x06u6`\x04aD\x04V[a\x10\xF2V[4\x80\x15a\x06\x86W`\0\x80\xFD[Pa\x03\x06a\x06\x956`\x04aC\xE7V[a\x11\x13V[4\x80\x15a\x06\xA6W`\0\x80\xFD[Pa\x06\xCB`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x01\xE4\x91\x90aH\xF2V[4\x80\x15a\x06\xE4W`\0\x80\xFD[P`\x06Ta\x06\xF9\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xE4V[4\x80\x15a\x07\x15W`\0\x80\xFD[Pa\x01\xB9`\t\x80T`\xFF\x19\x16\x90UV[4\x80\x15a\x071W`\0\x80\xFD[Pa\x03\x06`\x04T\x81V[4\x80\x15a\x07GW`\0\x80\xFD[Pa\x07[a\x07V6`\x04aD\x04V[a\x11[V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xE4V[4\x80\x15a\x07\x86W`\0\x80\xFD[Pa\x06\xF9a\x07\x956`\x04aI%V[a\x11\x93V[4\x80\x15a\x07\xA6W`\0\x80\xFD[P`\0Ta\x07\xB7\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xE4V[4\x80\x15a\x07\xD8W`\0\x80\xFD[Pa\x01\xB9a\x07\xE76`\x04aB\x87V[a\x11\xCBV[a\x07\xF4a\x12\tV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x08\x1BW`@Qc\xE6\xC4${`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x08JW`@Qc\xA8c\xAE\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x91\x82\x90U`@Q\x7F\x80\x17\xBB\x88\x7F\xDF\x8F\xCAC\x14\xA9\xD4\x0Fns\xB3\xB8\x10\x02\xD6~\\\xFA\x85\xD8\x81s\xAFj\xA4`r\x92a\x08\xA1\x92\x16\x90aD\x1DV[`@Q\x80\x91\x03\x90\xA1PV[a\x08\xB8`\x07`\0a?\xA6V[\x80Qa\x08\xCB\x90`\x07\x90` \x84\x01\x90a?\xC4V[PPV[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x80\x15a\x08\xF3WP`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\t:W`\x06T`\x01`\x01`\xA0\x1B\x03\x16a\t!W`@Qc\x12\xE6\xD1\xE7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x01GL\x8F`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\tBa\r\xC2V[Q\x82Q`\x01`\x01`@\x1B\x03\x91\x82\x16\x91\x16\x11\x15\x80a\t\x80WPa\tba\r\xC2V[` \x01Q`\x01`\x01`@\x1B\x03\x16\x82` \x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15[\x15a\t\x9EW`@Qc\x05\x1CF\xEF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\t\xC3\x90c\xFF\xFF\xFF\xFF\x81\x16\x90`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aI]V[`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` R`@\x90\x91 T\x91\x92P\x90\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x82\x16\x14\x80\x15\x81a\n\x19WP\x81`\x01`\x01`@\x1B\x03\x16\x84` \x01Q`\x01`\x01`@\x1B\x03\x16\x11[\x15a\nBW\x81`@Qc\x03df\xBF`\xE3\x1B\x81R`\x04\x01a\n9\x91\x90aHDV[`@Q\x80\x91\x03\x90\xFD[a\nO\x84`@\x01Qa\x12;V[a\n\\\x84``\x01Qa\x12;V[a\ni\x84`\x80\x01Qa\x12;V[a\nv\x84`\xA0\x01Qa\x12;V[a\n\x83\x84`\xC0\x01Qa\x12;V[\x80\x15a\n\x91Wa\n\x91a\x12\x97V[a\n\x9B\x84\x84a\x13\xE2V[`\0\x80T`\x01`@\x1B\x90\x81\x90\x04c\xFF\xFF\xFF\xFF\x16\x82R`\x05` \x81\x81R`@\x80\x85 \x89Q\x81T\x8B\x85\x01\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x91\x83\x16\x90\x97\x02\x17\x82U\x8A\x83\x01\x80Q`\x01\x80\x85\x01\x91\x90\x91U``\x8D\x01Q`\x02\x80\x86\x01\x91\x90\x91U`\x80\x8E\x01Q`\x03\x86\x01U`\xA0\x8E\x01Q`\x04\x86\x01U`\xC0\x8E\x01Q\x97\x85\x01\x97\x90\x97U`\xE0\x8D\x01Q`\x06\x90\x94\x01\x93\x90\x93U`\x07\x80T\x80\x85\x01\x82U\x90\x89RC`\0\x80Q` aJu\x839\x81Q\x91R\x90\x91\x01U\x83Q\x80\x85\x01\x85R\x87Q\x83\x16\x81R\x81Q\x81\x87\x01\x90\x81R`\x08\x80T\x95\x86\x01\x81U\x90\x99RQ`\0\x80Q` aJ\xF5\x839\x81Q\x91R\x93\x90\x96\x02\x92\x83\x01\x80T`\x01`\x01`@\x1B\x03\x19\x16\x96\x83\x16\x96\x90\x96\x17\x90\x95U\x95Q`\0\x80Q` aK5\x839\x81Q\x91R\x90\x91\x01U\x92Q\x88Q\x92Q\x93Q\x93\x84R\x84\x16\x93\x91\x90\x91\x16\x91\x7F\xA0Jw9$PZA\x85d67%\xF5h2\xF5w.k\x8D\r\xBDn\xFC\xE7$\xDF\xE8\x03\xDA\xE6\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPV[a\x0C\x02a@\x0FV[P`\0\x80T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81R`\x05` \x81\x81R`@\x92\x83\x90 \x83Qa\x01\0\x81\x01\x85R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x90\x91\x04\x16\x92\x81\x01\x92\x90\x92R`\x01\x81\x01T\x93\x82\x01\x93\x90\x93R`\x02\x83\x01T``\x82\x01R`\x03\x83\x01T`\x80\x82\x01R`\x04\x83\x01T`\xA0\x82\x01R\x90\x82\x01T`\xC0\x82\x01R`\x06\x90\x91\x01T`\xE0\x82\x01R\x90V[a\x0C\x94a\x14\xA1V[a\x0C\x9D\x82a\x15FV[a\x08\xCB\x82\x82a\x15}V[`\0a\x0C\xB1a\x166V[P`\0\x80Q` aJ\xB5\x839\x81Q\x91R\x90V[a\x0C\xD0`\x08`\0a@fV[`\0[\x81Q\x81\x10\x15a\x08\xCBW`\x08\x82\x82\x81Q\x81\x10a\x0C\xF0Wa\x0C\xF0aI\x80V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x80\x82\x01\x85U`\0\x94\x85R\x93\x83\x90 \x82Q`\x02\x90\x92\x02\x01\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x81U\x91\x01Q\x90\x82\x01U\x01a\x0C\xD3V[a\rHa\x12\tV[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\r\x95W`\x06\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x90U`@Q\x7F\x9A_W\xDE\x85m\xD6h\xC5M\xD9^\\U\xDF\x93C!q\xCB\xCAI\xA8wmV \xEAY\xC0$P\x90`\0\x90\xA1V[`@Qc\xA8c\xAE\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\r\xB8a\x12\tV[a\r\xAE`\0a\x16\x7FV[a\r\xCAa@\x0FV[P`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` \x81\x81R`@\x93\x84\x90 \x84Qa\x01\0\x81\x01\x86R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R\x95\x90\x04\x90\x94\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x83\x01T``\x83\x01R`\x03\x83\x01T`\x80\x83\x01R`\x04\x83\x01T`\xA0\x83\x01R\x82\x01T`\xC0\x82\x01R`\x06\x90\x91\x01T`\xE0\x82\x01R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x08\x80T\x90a\x0Ev`\x01\x83aI\x96V[\x81T\x81\x10a\x0E\x86Wa\x0E\x86aI\x80V[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x83\x10a\x0E\xBFW`@Qc\x18V\xA4\x99`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x0FTW\x83`\x08\x82\x81T\x81\x10a\x0E\xDEWa\x0E\xDEaI\x80V[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0FLW`\x08\x81\x81T\x81\x10a\x0F\x11Wa\x0F\x11aI\x80V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80T`\x01`\x01`@\x1B\x03\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`\x01\x01a\x0E\xC2V[P`\x08a\x0Fb`\x01\x83aI\x96V[\x81T\x81\x10a\x0FrWa\x0FraI\x80V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80T`\x01`\x01`@\x1B\x03\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80a\x0F\xB7a\x16\xDBV[T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[`\0a\x0F\xD1a\x16\xFFV[\x80T\x90\x91P`\xFF`\x01`@\x1B\x82\x04\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15a\x0F\xF8WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x10\x14WP0;\x15[\x90P\x81\x15\x80\x15a\x10\"WP\x80\x15[\x15a\x10@W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84T`\x01`\x01`@\x1B\x03\x19\x16`\x01\x17\x85U\x83\x15a\x10iW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x10r\x86a\x17#V[a\x10za\x174V[`\0\x80T`\x01` \x1B`\x01``\x1B\x03\x19\x16`\x01`@\x1B\x17\x90Ua\x10\x9D\x88\x88a\x17<V[\x83\x15a\x10\xE8W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90a\x10\xDF\x90`\x01\x90aHDV[`@Q\x80\x91\x03\x90\xA1[PPPPPPPPV[`\x07\x81\x81T\x81\x10a\x11\x02W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\x80\x80\x82\x01Q`\xA0\x83\x01Q`\xC0\x84\x01Q`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x08\x81\x81T\x81\x10a\x11kW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01\x80T`\x01\x90\x91\x01T`\x01`\x01`@\x1B\x03\x90\x91\x16\x91P\x82V[`\tT`\0\x90`\xFF\x16\x15a\x11\xB8W\x81`\nT\x84a\x11\xB0\x91\x90aI\x96V[\x11\x90Pa\x11\xC5V[a\x11\xC2\x83\x83a\x1A_V[\x90P[\x92\x91PPV[a\x11\xD3a\x12\tV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x11\xFDW`\0`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\x04\x01a\n9\x91\x90aD\x1DV[a\x12\x06\x81a\x16\x7FV[PV[3a\x12\x12a\x0F\xACV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\r\xAEW3`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R`\x04\x01a\n9\x91\x90aD\x1DV[`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x81\x10\x80a\x08\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x10\x9B\x8C\x8DM\x0E\x88\x1A[\x9D\x98[\x1AY\x08\x1C\xD8\xD8[\x18\\\x88\x19\x9AY[\x19`*\x1B`D\x82\x01R`d\x01a\n9V[`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` \x81\x81R`@\x80\x85 \x81Qa\x01\0\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R\x96\x90\x04\x90\x95\x16\x92\x85\x01\x92\x90\x92R`\x01\x82\x01T\x90\x84\x01R`\x02\x81\x01T``\x84\x01R`\x03\x81\x01T`\x80\x84\x01R`\x04\x81\x01T`\xA0\x84\x01R\x90\x81\x01T`\xC0\x83\x01R`\x06\x01T`\xE0\x82\x01Ra\x13\x1F\x90a\x11\x13V[`\x03\x80T`\x01\x90\x81U\x90\x82\x90U`\x04\x80T`\x02U`\0\x80T`\x01`@\x1B\x81\x04c\xFF\xFF\xFF\xFF\x16\x82R`\x05` R`@\x82 `\x06\x01T\x90\x92U\x92\x93P\x90\x91\x90`\x0C\x90a\x13z\x90\x84\x90`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aI\xA9V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x7F\xDB5X%\x9E\x03\x9D~P\xE8\x16\xB9\xDC\xCE0\xFB\x11M\x8A\x9C\x86\xEC\xA5\xAB\x14\xB6\x01\x94\xD6\x94]?`\0`\x0C\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16`@Qa\x08\xA1\x91\x90aHDV[`\0a\x13\xECa\x1B@V[\x90Pa\x13\xF6a@\x87V[`\x02T\x81R\x83Q`\x01`\x01`@\x1B\x03\x90\x81\x16` \x80\x84\x01\x91\x90\x91R\x80\x86\x01Q\x90\x91\x16`@\x80\x84\x01\x91\x90\x91R\x80\x86\x01Q``\x80\x85\x01\x91\x90\x91R\x86\x01Q`\x80\x84\x01R`\0\x80T`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x80\x82R`\x05\x80\x85R\x92\x82 `\x03\x81\x01T`\xA0\x87\x01R`\x04\x81\x01T`\xC0\x87\x01R\x91R\x91\x81\x90R\x01T`\xE0\x82\x01Ra\x14~\x82\x82\x85a!oV[a\x14\x9BW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x15(WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x15\x1C`\0\x80Q` aJ\xB5\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\r\xAEW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15Na\x12\tV[\x7F\xF7\x87!\"n\xFE\x9A\x1B\xB6x\x18\x9A\x16\xD1UI(\xB9\xF2\x19.,\xB9>\xED\xA8;y\xFA@\0}\x81`@Qa\x08\xA1\x91\x90aD\x1DV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x15\xD7WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x15\xD4\x91\x81\x01\x90aI\xD0V[`\x01[a\x15\xF6W\x81`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x04\x01a\n9\x91\x90aD\x1DV[`\0\x80Q` aJ\xB5\x839\x81Q\x91R\x81\x14a\x16'W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\n9V[a\x161\x83\x83a!\xEDV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\r\xAEW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x16\x89a\x16\xDBV[\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x93\x94P\x91\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPPV[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90V[a\x17+a\"CV[a\x12\x06\x81a\"hV[a\r\xAEa\"CV[\x81Q`\x01`\x01`@\x1B\x03\x16\x15\x15\x80a\x17`WP` \x82\x01Q`\x01`\x01`@\x1B\x03\x16\x15\x15[\x80a\x17mWP`\x80\x82\x01Q\x15[\x80a\x17zWP`\xA0\x82\x01Q\x15[\x80a\x17\x87WP`\xC0\x82\x01Q\x15[\x80a\x17\x94WP`\xE0\x82\x01Q\x15[\x80a\x17\xA3WPc\xFF\xFF\xFF\xFF\x81\x16\x15[\x15a\x17\xC1W`@QcP\xDD\x03\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x05`\0\x80`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x08a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01U``\x82\x01Q\x81`\x02\x01U`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01U`\xE0\x82\x01Q\x81`\x06\x01U\x90PP\x81`\x05`\0\x80`\x08\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x08a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01U``\x82\x01Q\x81`\x02\x01U`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01U`\xE0\x82\x01Q\x81`\x06\x01U\x90PP`\0\x80`\x0Ca\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x80`\0\x80a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0a\x19\xA7\x83a\x11\x13V[`\x01\x81\x81U`\xE0\x85\x01Q`\x02\x81\x81U`\x03\x93\x90\x93U`\x04U`\x07\x80T\x80\x83\x01\x82U`\0\x91\x82RC`\0\x80Q` aJu\x839\x81Q\x91R\x90\x91\x01U`@\x80Q\x80\x82\x01\x82R` \x80\x89\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x83R\x92\x90\x98\x01Q\x97\x81\x01\x97\x88R`\x08\x80T\x94\x85\x01\x81U\x90\x92R\x90Q`\0\x80Q` aJ\xF5\x839\x81Q\x91R\x92\x90\x93\x02\x91\x82\x01\x80T`\x01`\x01`@\x1B\x03\x19\x16\x93\x90\x91\x16\x92\x90\x92\x17\x90\x91U\x92Q`\0\x80Q` aK5\x839\x81Q\x91R\x90\x93\x01\x92\x90\x92UPPV[`\x07T`\0\x90C\x84\x11\x80a\x1AsWP`\x03\x81\x10[\x15a\x1A\x91W`@Qc\xB0\xB48w`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80a\x1A\xA0`\x01\x85aI\x96V[\x90P[\x81a\x1B\x0BW\x86`\x07\x82\x81T\x81\x10a\x1A\xBCWa\x1A\xBCaI\x80V[\x90`\0R` `\0 \x01T\x11a\x1A\xF1W`\x01\x91P`\x07\x81\x81T\x81\x10a\x1A\xE3Wa\x1A\xE3aI\x80V[\x90`\0R` `\0 \x01T\x92P[`\x02\x81\x10a\x1B\x0BW\x80a\x1B\x03\x81aI\xE9V[\x91PPa\x1A\xA3V[\x81a\x1B)W`@Qc\xB0\xB48w`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85a\x1B4\x84\x89aI\x96V[\x11\x97\x96PPPPPPPV[a\x1BHa@\xA6V[b\x01\0\0\x81R`\x08` \x82\x01R\x7F\x01=\x1DKBQy%\x8BWx`9yU\xCB\xFA\x08\x16\xE3+\x1C%\xA1\xFDsL\x91\xB9Q\xEE\x81`@\x82\x01QR\x7F\x16\xB8\x8D\xC7C\x9Am\x84\x1E\x1A\x11\x03\xF5\xA3\xD2\xD2D\x017\xF1\x8D\x02v5\x03\xBA\xC7\xB4]\xCB\x98;` `@\x83\x01Q\x01R\x7F\x0C<\x86O\x19_Y\x11\x99'\xF58W\xF1\xDE\x8B\xF5u\x94\x17H\xB755\x1F\xD3\x13s\xC7\x87\\-``\x82\x01QR\x7F\x16\x9B\xA1Q\x07\xF2\xEF\xF9\xB94\x1B\xF3\x07B\xA88\xD2}\xBDi\xE8\x8B#S\xDC\xA8Y/\x15\xF1\x11\x1C` ``\x83\x01Q\x01R\x7F\x11\xD4\xCE\xB1Ya\xD1\x0BaV\xAE=\t\xBBx\xB4\xDFE\xFB\x85C\x06\x08\x84\xE7\xD4\0u[\xEBJ\xC8`\x80\x82\x01QR\x7F\x03&\xFF\x069\x1E\xD5\xD2n\xC1\xBC\x08\x0B\x8DF\x01N\xE2,\x0Ch\xED\x02/\x16 \xC4\xD9\xD3\x847\xD3` `\x80\x83\x01Q\x01R\x7F#a\x0C\xB4>!\x03<6\x8A\x93b-\xD4\x05\xB9\x05\xA0\xEB4L\x98\xB9\xD7\xCF\x08\xB0\xC5\xEB\xF7\xC89`\xA0\x82\x01QR~\x13y4*Mw\xD4p\x87C\xAF\xF0\x1F\xF2z\xA1\x19\x17G\x8F\xDC\x8E+}F0\x81sWr\xEA` `\xA0\x83\x01Q\x01R\x7F\x19M\xAF\x85\xD9\xEE\xD9\x93{(\xE2\xA6\x80\xFC\xC5\xA7i\"\xC1\\\xD3\x1D\xC4\xF6\0\xE1I9\xB8 \x0C\xE7`\xC0\x82\x01QR\x7F%(\x0B\x12F$\x91\x1C\x7F\x87\xB4\xC2\xD8\x7FY\xC6\xC0~>\xEE\xB1\raM\xA2\x16\xF6!\x9F\xFEP\xB6` `\xC0\x83\x01Q\x01R\x7F\x04\x88.\xF3\x98\x99\xEA8\xC9gzH\xB8\xF8\xCCjg(N\x17\xFF\x94\x02\x89\xFA\xAA5\x9E\xEC\x9B3\xA6`\xE0\x82\x01QR\x7F\x1B\xAE\x9F6\xE6\x19\x078\xC7\x11P\x1B\xE5?)\x9B\xF6\x13H\xE6\x1E.\xF9\xD5wv\x0Ed\xF6)6\x8D` `\xE0\x83\x01Q\x01R\x7F-\x81\r0\x12\x0C\xB9>\x1A%K\x89\xED\n\xE8lv\x1FI\xB4\xF1)E\x9C\xD54\xF9U\x18Q5\x0Fa\x01\0\x82\x01QR\x7F\x0B%9M\xA5\xA1\xD45\xDA\xCC\xC2\xEA\xDD\x03\x9E,'\t\xF5\xF4/\xAB\xD9\xAF\xBA\x81^\xD6-j\xF3k` a\x01\0\x83\x01Q\x01R\x7F\x1C,\xE7\xBEW\x0B\xEA\x9EC\xF3\xD3\xD7\xCB\xCA\x84\xBD\xB4\xFC\x89\xB5:\xE6WS\x1D\xE7&p\xA6\x10^\x08a\x01 \x82\x01QR\x7F\x1A\x02U\xEC\x8C|\x87i3[\xC9\xDCk\"*\xC6\xA0Nvm\x08\xB4\\\x8C\xC5sY,\x05\xBC\x9C\x14` a\x01 \x83\x01Q\x01R\x7F\x1C\x16AY\x13k\x8F[Gs\xE13\xF4\x83\xA8\xA1\x92\xAB\x15\xD6\xD3\xEE\x01/\x17\x1B=\x02\xFDE\x06\xE7a\x01@\x82\x01QR\x7F'.\xB7\xD63\xCE\xDBh\xCE\x01\x13\xF4B\n\xB5a\x0B\x81\xB8\xBA\x1A\xB94\x8D\xB1Wa\xD4\x0E\x8D\xF5\xBA` a\x01@\x83\x01Q\x01R\x7F\x0EDf9\xAAl\xAF%\xE9>\xF7G\x08N9\xB8\xEA\x90\xAB\xF2;\xB4\x8C(\xFD_\x9B\xA7\xBAeP\"a\x01`\x82\x01QR\x7F\x03>\x19Z\x9E\xA3\xA9\xCE@\xB7+g:\xFBEDL\xA1\xB1_\x05C\xF4M\x10\xF5\xC6@\xA7\x80go` a\x01`\x83\x01Q\x01R\x7F\x0E\x8D\xB2\xB2\x89=\xF2=<U\xF0\xB3\xA3\xB5J\xB2\xE2\xEDw[1\xC4\xC9\x0EG.\xB3\x15\x82X-\xF2a\x01\x80\x82\x01QR\x7F\x0FLi\x89TQ\xAF\x15\x05*\xA8\x1A\x03\xEB\xA9u,\x9Ex\x91\xDD\x08\xE2d\xE0\xBDY=\x03W\x85\x8E` a\x01\x80\x83\x01Q\x01R\x7F+\xC0\x91\xFE\xFE$\x16\xEC\x83\xC6\xFB\xA8\xFB\x82\xFBH\xCB\x9C\xA0\x01\x82\xECy,!+\xF5S)X\xC8\xF7a\x01\xA0\x82\x01QR\x7F\x1FM\xCFM\xD4\x84\xE3\xAB\x1A\xE4\x87`=\"E9z\xC7\x8F\xE6\xCAu\x84d^\xBA9&\xD3\x83\x90+` a\x01\xA0\x83\x01Q\x01R\x7F-\xAFx\xC5\xA2\x82\x9A\x94\x18\x08\x1D\xD7\xE8t:h>\xD6\x81y\x96\xF7_\x10\0\x9D\x99\"\x07\x93\xECsa\x01\xC0\x82\x01QR\x7F\x19\xEB\x12\xA7\x82|\r\xDFc\x83\xFE\x80l9S\xBD\x06\xB0\x8A\xAE{\xF2\xA0\x1FU\xC9\x86\xA8OP\xCC(` a\x01\xC0\x83\x01Q\x01R\x7F\x01V\x910\x88F\xE6\x8E\xA8V\xA2\xCB$\xC9\x90?\x0C\x86\x05\xDE\xA1\x90\x82\x91\x80\xFFk\xDD\x1Ce\x08\x03a\x01\xE0\x82\x01QR\x7F\x1F\xFDx\x9B\x15[\x8A\xCB\x13\xE0\xF6\xA4\x8BP\xF7\xAA\x80\x92T\x08\x88\xD0\t\x14\x10W\xD4V\x90\x91X$` a\x01\xE0\x83\x01Q\x01R\x7F\x05E\xACz\xA6m\xCF7\x19\x98\x848\xC8\x06\xFCbM\xE5z\xB4?\x85\x809/\x88\xC8l\x13x\xCEJa\x02\0\x82\x01QR\x7F\x16\xB7\xF2P\x84.\xCFN6\x90pj\x1E\x15-zW\xF7\x0FUo\x92\x07m\xA7\x85\xFD\xD3c\xC1\x9F\xCF` a\x02\0\x83\x01Q\x01R\x7F \xCB\x7F\xF3Z\x83\xA7\xDC1@6\xE4p\xF1L0\xFB\x0E\x98\xD3]f;$;\",\xAAo\xC7\xDBDa\x02 \x82\x01QR\x7F\x14\x9FAWDpth\xBD\xAAN\x85E \x1A\xB4\r\x191\xA7\xD3\x1F#v\x8F\xA7\xC6Ut\xEE>\xAB` a\x02 \x83\x01Q\x01R\x7F\n%\xC1\xB7W9\x06\xDCN\x19;N\xA8/\xD1\xFE|\xCE\xBCM\x92]\xAD&\xF0\xFF\t\xC8L\x9F\x1Aua\x02@\x82\x01QR\x7F\nR\x1F\xF3\x0C\x8F6fy\x8F\x84|]L7\x96X\xFB\xA1\x01V\xE7\xA9I\x9F'\x13\xFA\xE9\xBF+\xE1` a\x02@\x83\x01Q\x01R\x7F\x03\xDBe\x10\xC3\xF16)\xFD\xED\x9AZ-AeK\xBC\xE4\xEFm\x02L\xADS\x10\0Q\xD4\xA3\xF3\xEB\xC9a\x02`\x82\x01QR\x7F\x08\xE8\n\\\x8EL\x9B\x9F&\xF3\0<\xC5\x94\x03\xA1\x8D16\xAF\xD00\x86\x8D%\xCC\x8B\x80~*\xB3p` a\x02`\x83\x01Q\x01R\x7F\xB0\x83\x88\x93\xEC\x1F#~\x8B\x072;\x07DY\x9FN\x97\xB5\x98\xB3\xB5\x89\xBC\xC2\xBC7\xB8\xD5\xC4\x18\x01a\x02\x80\x82\x01R\x7F\xC1\x83\x93\xC0\xFA0\xFEN\x8B\x03\x8E5z\xD8Q\xEA\xE8\xDE\x91\x07XN\xFF\xE7\xC7\xF1\xF6Q\xB2\x01\x0E&a\x02\xA0\x82\x01R\x90V[`\0a!z\x82a\"pV[a!\x8B\x83`\0[` \x02\x01Qa\x12;V[a!\x96\x83`\x01a!\x81V[a!\xA1\x83`\x02a!\x81V[a!\xAC\x83`\x03a!\x81V[a!\xB7\x83`\x04a!\x81V[a!\xC2\x83`\x05a!\x81V[a!\xCD\x83`\x06a!\x81V[a!\xD8\x83`\x07a!\x81V[a!\xE3\x84\x84\x84a#\xA8V[\x90P[\x93\x92PPPV[a!\xF6\x82a%\x86V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\";Wa\x161\x82\x82a%\xE2V[a\x08\xCBa&XV[a\"Ka&wV[a\r\xAEW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\xD3a\"CV[\x80Qa\"{\x90a&\x91V[a\"\x88\x81` \x01Qa&\x91V[a\"\x95\x81`@\x01Qa&\x91V[a\"\xA2\x81``\x01Qa&\x91V[a\"\xAF\x81`\x80\x01Qa&\x91V[a\"\xBC\x81`\xA0\x01Qa&\x91V[a\"\xC9\x81`\xC0\x01Qa&\x91V[a\"\xD6\x81`\xE0\x01Qa&\x91V[a\"\xE4\x81a\x01\0\x01Qa&\x91V[a\"\xF2\x81a\x01 \x01Qa&\x91V[a#\0\x81a\x01@\x01Qa&\x91V[a#\x0E\x81a\x01`\x01Qa&\x91V[a#\x1C\x81a\x01\x80\x01Qa&\x91V[a#*\x81a\x01\xA0\x01Qa\x12;V[a#8\x81a\x01\xC0\x01Qa\x12;V[a#F\x81a\x01\xE0\x01Qa\x12;V[a#T\x81a\x02\0\x01Qa\x12;V[a#b\x81a\x02 \x01Qa\x12;V[a#p\x81a\x02@\x01Qa\x12;V[a#~\x81a\x02`\x01Qa\x12;V[a#\x8C\x81a\x02\x80\x01Qa\x12;V[a#\x9A\x81a\x02\xA0\x01Qa\x12;V[a\x12\x06\x81a\x02\xC0\x01Qa\x12;V[`\0\x83` \x01Q`\x08\x14a#\xCFW`@Qc \xFA\x9D\x89`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a#\xDC\x85\x85\x85a'#V[\x90P`\0a#\xED\x86`\0\x01Qa,\x9AV[\x90P`\0a$\0\x82\x84`\xA0\x01Q\x88a0\xF1V[\x90Pa$\naA\xB9V[a$\x12aA\xB9V[a$3\x87a\x01`\x01Qa$.\x89a\x01\x80\x01Q\x88`\xE0\x01Qa1QV[a1\xE5V[\x91P`\0\x80a$D\x8B\x88\x87\x8Ca2\x80V[\x91P\x91Pa$U\x81a$.\x84a4\x84V[\x92Pa$n\x83a$.\x8Ba\x01`\x01Q\x8A`\xA0\x01Qa1QV[`\xA0\x88\x01Q`@\x88\x01Q` \x01Q\x91\x94P`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x91\x82\x90\x82\t\x90P\x81`\xE0\x8A\x01Q\x82\t\x90Pa$\xB2\x85a$.\x8Da\x01\x80\x01Q\x84a1QV[\x94P`\0`@Q\x80`\x80\x01`@R\x80\x7F\x01\x18\xC4\xD5\xB87\xBC\xC2\xBC\x89\xB5\xB3\x98\xB5\x97N\x9FYD\x07;2\x07\x8B~#\x1F\xEC\x93\x88\x83\xB0\x81R` \x01\x7F&\x0E\x01\xB2Q\xF6\xF1\xC7\xE7\xFFNX\x07\x91\xDE\xE8\xEAQ\xD8z5\x8E\x03\x8BN\xFE0\xFA\xC0\x93\x83\xC1\x81R` \x01\x7F\"\xFE\xBD\xA3\xC0\xC0c*VG[B\x14\xE5a^\x11\xE6\xDD?\x96\xE6\xCE\xA2\x85J\x87\xD4\xDA\xCC^U\x81R` \x01\x7F\x04\xFCci\xF7\x11\x0F\xE3\xD2QV\xC1\xBB\x9Ar\x85\x9C\xF2\xA0FA\xF9\x9B\xA4\xEEA<\x80\xDAj_\xE4\x81RP\x90Pa%t\x87\x82a%g\x89a4\x84V[a%oa4\xF3V[a5\xC4V[\x9E\x9DPPPPPPPPPPPPPPV[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a%\xB3W\x80`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x04\x01a\n9\x91\x90aD\x1DV[`\0\x80Q` aJ\xB5\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa%\xFF\x91\x90aJ\0V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a&:W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a&?V[``\x91P[P\x91P\x91Pa&O\x85\x83\x83a6\xA7V[\x95\x94PPPPPV[4\x15a\r\xAEW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a&\x81a\x16\xFFV[T`\x01`@\x1B\x90\x04`\xFF\x16\x91\x90PV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` aJU\x839\x81Q\x91R\x91\x15\x90\x15\x16\x15a&\xB9WPPPV[\x82Q` \x84\x01Q\x82`\x03\x84\x85\x85\x86\t\x85\t\x08\x83\x82\x83\t\x14\x83\x82\x10\x84\x84\x10\x16\x16\x93PPP\x81a\x161W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv\x10\x9B\x8C\x8DM\x0E\x88\x1A[\x9D\x98[\x1AY\x08\x11\xCCH\x1C\x1B\xDA[\x9D`J\x1B`D\x82\x01R`d\x01a\n9V[a'k`@Q\x80a\x01\0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x90P`@Q` \x81\x01`\0\x81R`\xFE`\xE0\x1B\x81R\x86Q`\xC0\x1B`\x04\x82\x01R` \x87\x01Q`\xC0\x1B`\x0C\x82\x01Ra\x02\x80\x87\x01Q` \x82\x01Ra\x02\xA0\x87\x01Q`@\x82\x01R`\x01``\x82\x01R`\0\x80Q` aKu\x839\x81Q\x91R`\x80\x82\x01R`\0\x80Q` aK\x15\x839\x81Q\x91R`\xA0\x82\x01R`\0\x80Q` aKU\x839\x81Q\x91R`\xC0\x82\x01R`\0\x80Q` aJ\x95\x839\x81Q\x91R`\xE0\x82\x01R`\xE0\x87\x01Q\x80Qa\x01\0\x83\x01R` \x81\x01Qa\x01 \x83\x01RPa\x01\0\x87\x01Q\x80Qa\x01@\x83\x01R` \x81\x01Qa\x01`\x83\x01RPa\x01 \x87\x01Q\x80Qa\x01\x80\x83\x01R` \x81\x01Qa\x01\xA0\x83\x01RPa\x01@\x87\x01Q\x80Qa\x01\xC0\x83\x01R` \x81\x01Qa\x01\xE0\x83\x01RPa\x01`\x87\x01Q\x80Qa\x02\0\x83\x01R` \x81\x01Qa\x02 \x83\x01RPa\x01\x80\x87\x01Q\x80Qa\x02@\x83\x01R` \x81\x01Qa\x02`\x83\x01RPa\x01\xE0\x87\x01Q\x80Qa\x02\x80\x83\x01R` \x81\x01Qa\x02\xA0\x83\x01RPa\x02\0\x87\x01Q\x80Qa\x02\xC0\x83\x01R` \x81\x01Qa\x02\xE0\x83\x01RPa\x02 \x87\x01Q\x80Qa\x03\0\x83\x01R` \x81\x01Qa\x03 \x83\x01RPa\x02@\x87\x01Q\x80Qa\x03@\x83\x01R` \x81\x01Qa\x03`\x83\x01RPa\x01\xA0\x87\x01Q\x80Qa\x03\x80\x83\x01R` \x81\x01Qa\x03\xA0\x83\x01RPa\x01\xC0\x87\x01Q\x80Qa\x03\xC0\x83\x01R` \x81\x01Qa\x03\xE0\x83\x01RPa\x02`\x87\x01Q\x80Qa\x04\0\x83\x01R` \x81\x01Qa\x04 \x83\x01RP`@\x87\x01Q\x80Qa\x04@\x83\x01R` \x81\x01Qa\x04`\x83\x01RP``\x87\x01Q\x80Qa\x04\x80\x83\x01R` \x81\x01Qa\x04\xA0\x83\x01RP`\x80\x87\x01Q\x80Qa\x04\xC0\x83\x01R` \x81\x01Qa\x04\xE0\x83\x01RP`\xA0\x87\x01Q\x80Qa\x05\0\x83\x01R` \x81\x01Qa\x05 \x83\x01RP`\xC0\x87\x01Q\x80Qa\x05@\x83\x01R` \x81\x01Qa\x05`\x83\x01RP\x85Qa\x05\x80\x82\x01R` \x86\x01Qa\x05\xA0\x82\x01R`@\x86\x01Qa\x05\xC0\x82\x01R``\x86\x01Qa\x05\xE0\x82\x01R`\x80\x86\x01Qa\x06\0\x82\x01R`\xA0\x86\x01Qa\x06 \x82\x01R`\xC0\x86\x01Qa\x06@\x82\x01R`\xE0\x86\x01Qa\x06`\x82\x01R\x84Q\x80Qa\x06\x80\x83\x01R` \x81\x01Qa\x06\xA0\x83\x01RP` \x85\x01Q\x80Qa\x06\xC0\x83\x01R` \x81\x01Qa\x06\xE0\x83\x01RP`@\x85\x01Q\x80Qa\x07\0\x83\x01R` \x81\x01Qa\x07 \x83\x01RP``\x85\x01Q\x80Qa\x07@\x83\x01R` \x81\x01Qa\x07`\x83\x01RP`\x80\x85\x01Q\x80Qa\x07\x80\x83\x01R` \x81\x01Qa\x07\xA0\x83\x01RP`\0\x82Ra\x07\xE0\x82 a\x07\xC0\x82\x01Ra\x07\xC0\x81\x01\x91P` \x82\x01\x90P\x82\x82Q\x06``\x85\x01R` \x82 \x81R\x80\x91P` \x82\x01\x90P\x82\x82Q\x06`\x80\x85\x01R`\xA0\x85\x01Q\x80Q\x82R` \x81\x01Q` \x83\x01RP``\x82 `@\x82\x01\x92P\x80\x83R` \x83\x01\x91P\x83\x81\x06\x85R\x83\x81\x82\t\x84\x82\x82\t\x91P\x80` \x87\x01RP\x80`@\x86\x01RP`\xC0\x85\x01Q\x80Q\x82R` \x81\x01Q` \x83\x01RP`\xE0\x85\x01Q\x80Q`@\x83\x01R` \x81\x01Q``\x83\x01RPa\x01\0\x85\x01Q\x80Q`\x80\x83\x01R` \x81\x01Q`\xA0\x83\x01RPa\x01 \x85\x01Q\x80Q`\xC0\x83\x01R` \x81\x01Q`\xE0\x83\x01RPa\x01@\x85\x01Q\x80Qa\x01\0\x83\x01R` \x81\x01Qa\x01 \x83\x01RPa\x01`\x82 a\x01@\x82\x01Ra\x01@\x81\x01\x91P` \x82\x01\x90P\x82\x82Q\x06`\xA0\x85\x01Ra\x01\xA0\x85\x01Q\x81Ra\x01\xC0\x85\x01Q` \x82\x01Ra\x01\xE0\x85\x01Q`@\x82\x01Ra\x02\0\x85\x01Q``\x82\x01Ra\x02 \x85\x01Q`\x80\x82\x01Ra\x02@\x85\x01Q`\xA0\x82\x01Ra\x02`\x85\x01Q`\xC0\x82\x01Ra\x02\x80\x85\x01Q`\xE0\x82\x01Ra\x02\xA0\x85\x01Qa\x01\0\x82\x01Ra\x02\xC0\x85\x01Qa\x01 \x82\x01Ra\x01`\x82 a\x01@\x82\x01Ra\x01@\x81\x01\x91P` \x82\x01\x90P\x82\x82Q\x06`\xC0\x85\x01Ra\x01`\x85\x01Q\x80Q\x82R` \x81\x01Q` \x83\x01RPa\x01\x80\x85\x01Q\x80Q`@\x83\x01R` \x81\x01Q``\x83\x01RPP`\xA0\x81 \x82\x81\x06`\xE0\x85\x01RPPP\x93\x92PPPV[a,\xA2aA\xD3V[\x81b\x01\0\0\x03a.\x08W`@Q\x80``\x01`@R\x80`\x10\x81R` \x01\x7F0d\x1E\x0E\x92\xBE\xBE\xF8\x18&\x8Df;\xCA\xD6\xDB\xCF\xD6\xC0\x14\x91p\xF6\xD7\xD3P\xB1\xB1\xFAl\x10\x01\x81R` \x01`@Q\x80a\x01\0\x01`@R\x80`\x01\x81R` \x01~\xEE\xB2\xCBY\x81\xEDEd\x9A\xBE\xBD\xE0\x81\xDC\xFF\x16\xC8`\x1D\xE44~}\xD1b\x8B\xA2\xDA\xACC\xB7\x81R` \x01\x7F-\x1B\xA6oYA\xDC\x91\x01qq\xFAi\xEC+\xD0\x02**-A\x15\xA0\t\xA94X\xFDN&\xEC\xFB\x81R` \x01\x7F\x08h\x12\xA0\n\xC4>\xA8\x01f\x9Cd\x01q <A\xA4\x96g\x1B\xFB\xC0e\xAC\x8D\xB2MR\xCF1\xE5\x81R` \x01\x7F-\x96VQ\xCD\xD9\xE4\x81\x1FNQ\xB8\r\xDC\xA8\xA8\xB4\xA9>\xE1t \xAA\xE6\xAD\xAA\x01\xC2a|n\x85\x81R` \x01\x7F\x12YzV\xC2\xE48b\x0B\x90A\xB9\x89\x92\xAE\rNp[x\0W\xBFwf\xA2v|\xEC\xE1n\x1D\x81R` \x01\x7F\x02\xD9A\x17\xCD\x17\xBC\xF1)\x0F\xD6|\x01\x15]\xD4\x08\x07\x85}\xFFJZ\x0BM\xC6{\xEF\xA8\xAA4\xFD\x81R` \x01\x7F\x15\xEE$u\xBE\xE5\x17\xC4\xEE\x05\xE5\x1F\xA1\xEEs\x12\xA87:\x0B\x13\xDB\x8CQ\xBA\xF0L\xB2\xE9\x9B\xD2\xBD\x81RP\x81RP\x90P\x91\x90PV[\x81b\x10\0\0\x03a/oW`@Q\x80``\x01`@R\x80`\x14\x81R` \x01\x7F0dKl\x9CJr\x16\x9EM\xAA1}%\xF0E\x12\xAE\x15\xC5;4\xE8\xF5\xAC\xD8\xE1U\xD0\xA6\xC1\x01\x81R` \x01`@Q\x80a\x01\0\x01`@R\x80`\x01\x81R` \x01\x7F&\x12]\xA1\n\x0E\xD0c'P\x8A\xBA\x06\xD1\xE3\x03\xACaf2\xDB\xED4\x9FSB-\xA9S3xW\x81R` \x01\x7F\"`\xE7$\x84K\xCARQ\x82\x93S\x96\x8EI\x150RXA\x83WG:\\\x1DY\x7Fa?l\xBD\x81R` \x01\x7F \x87\xEA,\xD6d'\x86\x08\xFB\x0E\xBD\xB8 \x90\x7FY\x85\x02\xC8\x1Bf\x90\xC1\x85\xE2\xBF\x15\xCB\x93_B\x81R` \x01\x7F\x19\xDD\xBC\xAF:\x8DF\xC1\\\x01v\xFB\xB5\xB9^M\xC5p\x88\xFF\x13\xF4\xD1\xBD\x84\xC6\xBF\xA5}\xCD\xC0\xE0\x81R` \x01\x7F\x05\xA2\xC8\\\xFCY\x17\x89`\\\xAE\x81\x8E7\xDDAa\xEE\xF9\xAAfk\xECo\xE4(\x8D\t\xE6\xD24\x18\x81R` \x01\x7F\x11\xF7\x0ESc%\x8F\xF4\xF0\xD7\x16\xA6S\xE1\xDCA\xF1\xC6D\x84\xD7\xF4\xB6\xE2\x19\xD67v\x14\xA3\x90\\\x81R` \x01\x7F)\xE8AC\xF5\x87\rGv\xA9-\xF8\xDA\x8Cl\x93\x03\xD5\x90\x88\xF3{\xA8_@\xCFo\xD1Be\xB4\xBC\x81RP\x81RP\x90P\x91\x90PV[\x81` \x03a0\xD3W`@Q\x80``\x01`@R\x80`\x05\x81R` \x01\x7F.\xE1+\xFFJ(\x13(j\x8D\xC3\x88\xCDuM\x9A>\xF2I\x065\xEB\xA5\x0C\xB9\xC2\xE5\xE7P\x80\0\x01\x81R` \x01`@Q\x80a\x01\0\x01`@R\x80`\x01\x81R` \x01\x7F\t\xC52\xC60k\x93\xD2\x96x \rG\xC0\xB2\xA9\x9C\x18\xD5\x1B\x83\x8E\xEB\x1D>\xEDLS;\xB5\x12\xD0\x81R` \x01\x7F!\x08,\xA2\x16\xCB\xBFN\x1CnOE\x94\xDDP\x8C\x99m\xFB\xE1\x17N\xFB\x98\xB1\x15\t\xC6\xE3\x06F\x0B\x81R` \x01\x7F\x12w\xAEd\x15\xF0\xEF\x18\xF2\xBA_\xB1b\xC3\x9E\xB71\x1F8n-&\xD6D\x01\xF4\xA2]\xA7|%;\x81R` \x01\x7F+3}\xE1\xC8\xC1O\"\xEC\x9B\x9E/\x96\xAF\xEF6Rbsf\xF8\x17\n\n\x94\x8D\xADJ\xC1\xBD^\x80\x81R` \x01\x7F/\xBDM\xD2\x97k\xE5]\x1A\x16:\xA9\x82\x0F\xB8\x8D\xFA\xC5\xDD\xCEw\xE1\x87.\x90c '2z^\xBE\x81R` \x01\x7F\x10z\xABI\xE6Zg\xF9\xDA\x9C\xD2\xAB\xF7\x8B\xE3\x8B\xD9\xDC\x1D]\xB3\x9F\x81\xDE6\xBC\xFA[K\x03\x90C\x81R` \x01~\xE1Kcd\xA4~\x9CB\x84\xA9\xF8\n_\xC4\x1C\xD2\x12\xB0\xD4\xDB\xF8\xA5p7p\xA4\n\x9A49\x90\x81RP\x81RP\x90P\x91\x90PV[`@Qc\xE2\xEF\t\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a1\x15`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a1\x1F\x84\x84a6\xFAV[\x80\x82Ra1/\x90\x85\x90\x85\x90a7NV[` \x82\x01R\x80Qa1E\x90\x85\x90\x84\x90\x86\x90a7\xC2V[`@\x82\x01R\x93\x92PPPV[a1YaA\xB9V[a1aaA\xF9V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x81\x01\x83\x90R`\0``\x83`\x80\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a1\x93W`\0\x80\xFD[P\x80a1\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01RxBn254: scalar mul failed!`8\x1B`D\x82\x01R`d\x01a\n9V[PP\x92\x91PPV[a1\xEDaA\xB9V[a1\xF5aB\x17V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x83\x01R\x83\x01Q``\x80\x83\x01\x91\x90\x91R`\0\x90\x83`\xC0\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a22W`\0\x80\xFD[P\x80a1\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FBn254: group addition failed!\0\0\0`D\x82\x01R`d\x01a\n9V[a2\x88aA\xB9V[a2\x90aA\xB9V[`\0a2\x9E\x87\x87\x87\x87a9\x18V[\x90P`\0\x80Q` aJ\xD5\x839\x81Q\x91R`\0a2\xBC\x88\x87\x89a=\x94V[\x90Pa2\xC8\x81\x83aI\x96V[`\xC0\x89\x01Qa\x01\xA0\x88\x01Q\x91\x92P\x90\x81\x90\x84\x90\x81\x90\x83\t\x84\x08\x92Pa2\xF5\x85a$.\x8A`\0\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x01\xC0\x8A\x01Q\x83\t\x84\x08\x92Pa3\x1D\x86a$.\x8A` \x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x01\xE0\x8A\x01Q\x83\t\x84\x08\x92Pa3E\x86a$.\x8A`@\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x02\0\x8A\x01Q\x83\t\x84\x08\x92Pa3m\x86a$.\x8A``\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x02 \x8A\x01Q\x83\t\x84\x08\x92Pa3\x95\x86a$.\x8A`\x80\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x02@\x8A\x01Q\x83\t\x84\x08\x92Pa3\xBD\x86a$.\x8D`@\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x02`\x8A\x01Q\x83\t\x84\x08\x92Pa3\xE5\x86a$.\x8D``\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x02\x80\x8A\x01Q\x83\t\x84\x08\x92Pa4\r\x86a$.\x8D`\x80\x01Q\x84a1QV[\x95P\x83\x82\x82\t\x90P\x83\x84a\x02\xA0\x8A\x01Q\x83\t\x84\x08\x92Pa45\x86a$.\x8D`\xA0\x01Q\x84a1QV[\x95P`\0\x8A`\xE0\x01Q\x90P\x84\x85a\x02\xC0\x8B\x01Q\x83\t\x85\x08\x93Pa4`\x87a$.\x8B`\xA0\x01Q\x84a1QV[\x96Pa4sa4ma>\x84V[\x85a1QV[\x97PPPPPPP\x94P\x94\x92PPPV[a4\x8CaA\xB9V[\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a4\xA0WP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aJU\x839\x81Q\x91R\x84` \x01Qa4\xD3\x91\x90aJ2V[a4\xEB\x90`\0\x80Q` aJU\x839\x81Q\x91RaI\x96V[\x90R\x92\x91PPV[a5\x1E`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R` \x87\x01Q`@\x82\x01R\x86Q``\x82\x01R``\x87\x01Q`\x80\x82\x01R`@\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R` \x85\x01Qa\x01\0\x82\x01R\x84Qa\x01 \x82\x01R``\x85\x01Qa\x01@\x82\x01R`@\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a6\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R{Bn254: Pairing check failed!` \x1B`D\x82\x01R`d\x01a\n9V[P\x15\x15\x90P[\x94\x93PPPPV[``\x82a6\xBCWa6\xB7\x82a>\xA5V[a!\xE6V[\x81Q\x15\x80\x15a6\xD3WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a6\xF3W\x83`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x04\x01a\n9\x91\x90aD\x1DV[P\x80a!\xE6V[\x81Q`\0\x90`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x90\x83\x80\x15a7>W\x84\x93P`\0[\x82\x81\x10\x15a72W\x83\x85\x86\t\x94P`\x01\x01a7\x1CV[P`\x01\x84\x03\x93Pa7EV[`\x01\x83\x03\x93P[PPP\x92\x91PPV[`\0\x82`\x01\x03a7`WP`\x01a!\xE6V[\x81`\0\x03a7pWP`\0a!\xE6V[` \x84\x01Q`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x90`\0\x90\x82\x81\x86\t\x90P\x85\x80\x15a7\xA0W`\x01\x87\x03\x92Pa7\xA7V[`\x01\x84\x03\x92P[Pa7\xB1\x82a>\xCEV[\x91P\x82\x82\x82\t\x97\x96PPPPPPPV[`\0`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x82\x82\x03a8?W`\x01`\0[`\x08\x81\x10\x15a83W\x81\x86\x03a8\x10W\x86\x81`\x08\x81\x10a8\x01Wa8\x01aI\x80V[` \x02\x01Q\x93PPPPa6\x9FV[\x82\x80a8\x1EWa8\x1EaJ\x1CV[`@\x89\x01Q` \x01Q\x83\t\x91P`\x01\x01a7\xDFV[P`\0\x92PPPa6\x9FV[a8Ga@\x87V[`@\x87\x01Q`\x01`\xE0\x83\x81\x01\x82\x81R\x92\x01\x90\x80[`\x08\x81\x10\x15a8\x88W` \x84\x03\x93P\x85\x86\x8A\x85Q\x89\x03\x08\x83\t\x80\x85R`\x1F\x19\x90\x93\x01\x92\x91P`\x01\x01a8[V[PPPP`\0\x80`\0\x90P`\x01\x83\x89`@\x8C\x01Q`\0[`\x08\x81\x10\x15a8\xDFW\x88\x82Q\x8A\x85Q\x8C\x88Q\x8A\t\t\t\x89\x81\x88\x08\x96PP\x88\x89\x8D\x84Q\x8C\x03\x08\x86\t\x94P` \x93\x84\x01\x93\x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a8\x9FV[PPPP\x80\x92PP`\0a8\xF2\x83a>\xCEV[\x90P` \x8A\x01Q\x85\x81\x89\t\x96PP\x84\x81\x87\t\x95P\x84\x82\x87\t\x9A\x99PPPPPPPPPPV[a9 aA\xB9V[`\0\x80`\0\x80`\0`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x90P`\x80\x89\x01Q\x81` \x8A\x01Q` \x8C\x01Q\t\x95P\x89Q\x94P\x81`\xA0\x8B\x01Q``\x8C\x01Q\t\x93P\x81a\x01\xA0\x89\x01Q\x85\x08\x92P\x81\x81\x84\x08\x92P\x81\x85\x84\t\x94P\x81`\0\x80Q` aKu\x839\x81Q\x91R\x85\t\x92P\x81a\x01\xC0\x89\x01Q\x84\x08\x92P\x81\x81\x84\x08\x92P\x81\x85\x84\t\x94P\x81`\0\x80Q` aK\x15\x839\x81Q\x91R\x85\t\x92P\x81a\x01\xE0\x89\x01Q\x84\x08\x92P\x81\x81\x84\x08\x92P\x81\x85\x84\t\x94P\x81`\0\x80Q` aKU\x839\x81Q\x91R\x85\t\x92P\x81a\x02\0\x89\x01Q\x84\x08\x92P\x81\x81\x84\x08\x92P\x81\x85\x84\t\x94P\x81`\0\x80Q` aJ\x95\x839\x81Q\x91R\x85\t\x92P\x81a\x02 \x89\x01Q\x84\x08\x92P\x81\x81\x84\x08\x92PP\x80\x84\x83\t\x93P\x80\x84\x86\x08\x94Pa:6\x87`\xA0\x01Q\x86a1QV[\x95P\x88Q``\x8A\x01Q`\x80\x8B\x01Q\x83\x82\x84\t\x97P\x83a\x02\xC0\x8B\x01Q\x89\t\x97P\x83a\x02@\x8B\x01Q\x83\t\x95P\x83a\x01\xA0\x8B\x01Q\x87\x08\x95P\x83\x81\x87\x08\x95P\x83\x86\x89\t\x97P\x83a\x02`\x8B\x01Q\x83\t\x95P\x83a\x01\xC0\x8B\x01Q\x87\x08\x95P\x83\x81\x87\x08\x95P\x83\x86\x89\t\x97P\x83a\x02\x80\x8B\x01Q\x83\t\x95P\x83a\x01\xE0\x8B\x01Q\x87\x08\x95P\x83\x81\x87\x08\x95P\x83\x86\x89\t\x97P\x83a\x02\xA0\x8B\x01Q\x83\t\x95P\x83a\x02\0\x8B\x01Q\x87\x08\x95P\x83\x81\x87\x08\x95PPPP\x80\x83\x86\t\x94Pa:\xFD\x86a$.\x8C`\xC0\x01Q\x88\x85a:\xF8\x91\x90aI\x96V[a1QV[\x95Pa;\x16\x86a$.\x8C`\xE0\x01Q\x8Aa\x01\xA0\x01Qa1QV[\x95Pa;0\x86a$.\x8Ca\x01\0\x01Q\x8Aa\x01\xC0\x01Qa1QV[\x95Pa;J\x86a$.\x8Ca\x01 \x01Q\x8Aa\x01\xE0\x01Qa1QV[\x95Pa;d\x86a$.\x8Ca\x01@\x01Q\x8Aa\x02\0\x01Qa1QV[\x95P\x80a\x01\xC0\x88\x01Qa\x01\xA0\x89\x01Q\t\x92Pa;\x89\x86a$.\x8Ca\x01`\x01Q\x86a1QV[\x95P\x80a\x02\0\x88\x01Qa\x01\xE0\x89\x01Q\t\x92Pa;\xAE\x86a$.\x8Ca\x01\x80\x01Q\x86a1QV[\x95Pa\x01\xA0\x87\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92Pa;\xDD\x86a$.\x8Ca\x01\xE0\x01Q\x86a1QV[\x95Pa\x01\xC0\x87\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92Pa<\x0C\x86a$.\x8Ca\x02\0\x01Q\x86a1QV[\x95Pa\x01\xE0\x87\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92Pa<;\x86a$.\x8Ca\x02 \x01Q\x86a1QV[\x95Pa\x02\0\x87\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92Pa<j\x86a$.\x8Ca\x02@\x01Q\x86a1QV[\x95Pa<\x87\x86a$.\x8Ca\x01\xA0\x01Qa:\xF8\x8Ba\x02 \x01Qa?tV[\x95Pa<\x98\x86\x8Ba\x01\xC0\x01Qa1\xE5V[\x95P\x80a\x01\xC0\x88\x01Qa\x01\xA0\x89\x01Q\t\x92P\x80a\x01\xE0\x88\x01Q\x84\t\x92P\x80a\x02\0\x88\x01Q\x84\t\x92P\x80a\x02 \x88\x01Q\x84\t\x92Pa<\xDE\x86a$.\x8Ca\x02`\x01Q\x86a1QV[\x95Pa<\xED\x88`\0\x01Qa?tV[\x94Pa=\x01\x86a$.\x89`\xC0\x01Q\x88a1QV[\x95P\x80`\x01\x89Q\x08`\xA0\x8A\x01Q\x90\x93P\x81\x90\x80\t\x91P\x80\x82\x84\t\x92P\x80\x83\x86\t\x94Pa=5\x86a$.\x89`\xE0\x01Q\x88a1QV[\x95P\x80\x83\x86\t\x94Pa=P\x86a$.\x89a\x01\0\x01Q\x88a1QV[\x95P\x80\x83\x86\t\x94Pa=k\x86a$.\x89a\x01 \x01Q\x88a1QV[\x95P\x80\x83\x86\t\x94Pa=\x86\x86a$.\x89a\x01@\x01Q\x88a1QV[\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x90P`\0\x83` \x01Q\x90P`\0\x84`@\x01Q\x90P`\0`\x01\x90P``\x88\x01Q`\x80\x89\x01Qa\x01\xA0\x89\x01Qa\x02@\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x01\xC0\x89\x01Qa\x02`\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x01\xE0\x89\x01Qa\x02\x80\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x02\0\x89\x01Qa\x02\xA0\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x02 \x89\x01Q\x91Pa\x02\xC0\x89\x01Q\x86\x87\x82\x89\x85\x87\x08\t\x85\t\x93PPPP\x87Q` \x89\x01Q\x85\x86\x86\x83\t\x87\x03\x85\x08\x96PP\x84\x85\x83\x83\t\x86\x03\x87\x08\x99\x98PPPPPPPPPV[a>\x8CaA\xB9V[P`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x02` \x82\x01R\x90V[\x80Q\x15a>\xB5W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01R`\x02\x82\x03`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x92PP`\0Q\x92P\x81a?mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FBn254: pow precompile failed!\0\0\0`D\x82\x01R`d\x01a\n9V[PP\x91\x90PV[`\0a?\x8E`\0\x80Q` aJ\xD5\x839\x81Q\x91R\x83aJ2V[a\x11\xC5\x90`\0\x80Q` aJ\xD5\x839\x81Q\x91RaI\x96V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x12\x06\x91\x90aB5V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a?\xFFW\x91` \x02\x82\x01[\x82\x81\x11\x15a?\xFFW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a?\xE4V[Pa@\x0B\x92\x91PaB5V[P\x90V[`@Q\x80a\x01\0\x01`@R\x80`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[P\x80T`\0\x82U`\x02\x02\x90`\0R` `\0 \x90\x81\x01\x90a\x12\x06\x91\x90aBJV[`@Q\x80a\x01\0\x01`@R\x80`\x08\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x02\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01a@\xC8aA\xB9V[\x81R` \x01a@\xD5aA\xB9V[\x81R` \x01a@\xE2aA\xB9V[\x81R` \x01a@\xEFaA\xB9V[\x81R` \x01a@\xFCaA\xB9V[\x81R` \x01aA\taA\xB9V[\x81R` \x01aA\x16aA\xB9V[\x81R` \x01aA#aA\xB9V[\x81R` \x01aA0aA\xB9V[\x81R` \x01aA=aA\xB9V[\x81R` \x01aAJaA\xB9V[\x81R` \x01aAWaA\xB9V[\x81R` \x01aAdaA\xB9V[\x81R` \x01aAqaA\xB9V[\x81R` \x01aA~aA\xB9V[\x81R` \x01aA\x8BaA\xB9V[\x81R` \x01aA\x98aA\xB9V[\x81R` \x01aA\xA5aA\xB9V[\x81R`\0` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01aA\xF4a@\x87V[\x90R\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a@\x0BW`\0\x81U`\x01\x01aB6V[[\x80\x82\x11\x15a@\x0BW\x80T`\x01`\x01`@\x1B\x03\x19\x16\x81U`\0`\x01\x82\x01U`\x02\x01aBKV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a0\xECW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aB\x99W`\0\x80\xFD[a\x11\xC2\x82aBpV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\xDAWaB\xDAaB\xA2V[`@R\x90V[`@Qa\x02\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\xDAWaB\xDAaB\xA2V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC+WaC+aB\xA2V[`@R\x91\x90PV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a0\xECW`\0\x80\xFD[`\0a\x01\0\x80\x83\x85\x03\x12\x15aC^W`\0\x80\xFD[`@Q\x90\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x81\x83\x10\x17\x15aC\x80WaC\x80aB\xA2V[\x81`@R\x80\x92PaC\x90\x84aC3V[\x81RaC\x9E` \x85\x01aC3V[` \x82\x01R`@\x84\x015`@\x82\x01R``\x84\x015``\x82\x01R`\x80\x84\x015`\x80\x82\x01R`\xA0\x84\x015`\xA0\x82\x01R`\xC0\x84\x015`\xC0\x82\x01R`\xE0\x84\x015`\xE0\x82\x01RPP\x92\x91PPV[`\0a\x01\0\x82\x84\x03\x12\x15aC\xFAW`\0\x80\xFD[a\x11\xC2\x83\x83aCJV[`\0` \x82\x84\x03\x12\x15aD\x16W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aDJWaDJaB\xA2V[P`\x05\x1B` \x01\x90V[`\0` \x80\x83\x85\x03\x12\x15aDgW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aD}W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aD\x8EW`\0\x80\xFD[\x805aD\xA1aD\x9C\x82aD1V[aC\x03V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aD\xC0W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aD\xDEW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aD\xC5V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15aD\xFBW`\0\x80\xFD[a\x11\xC2\x82aC3V[`\0`@\x82\x84\x03\x12\x15aE\x16W`\0\x80\xFD[aE\x1EaB\xB8V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80\x82\x84\x03a\x05\x80\x81\x12\x15aEIW`\0\x80\xFD[aES\x85\x85aCJV[\x92Pa\x01\0a\x04\x80\x80`\xFF\x19\x84\x01\x12\x15aElW`\0\x80\xFD[aEtaB\xE0V[\x92PaE\x82\x87\x83\x88\x01aE\x04V[\x83Ra\x01@aE\x93\x88\x82\x89\x01aE\x04V[` \x85\x01Ra\x01\x80aE\xA7\x89\x82\x8A\x01aE\x04V[`@\x86\x01Ra\x01\xC0aE\xBB\x8A\x82\x8B\x01aE\x04V[``\x87\x01Ra\x02\0aE\xCF\x8B\x82\x8C\x01aE\x04V[`\x80\x88\x01Ra\x02@aE\xE3\x8C\x82\x8D\x01aE\x04V[`\xA0\x89\x01Ra\x02\x80aE\xF7\x8D\x82\x8E\x01aE\x04V[`\xC0\x8A\x01Ra\x02\xC0aF\x0B\x8E\x82\x8F\x01aE\x04V[`\xE0\x8B\x01RaF\x1E\x8Ea\x03\0\x8F\x01aE\x04V[\x89\x8B\x01RaF0\x8Ea\x03@\x8F\x01aE\x04V[a\x01 \x8B\x01RaFD\x8Ea\x03\x80\x8F\x01aE\x04V[\x87\x8B\x01RaFV\x8Ea\x03\xC0\x8F\x01aE\x04V[a\x01`\x8B\x01RaFj\x8Ea\x04\0\x8F\x01aE\x04V[\x86\x8B\x01Ra\x04@\x8D\x015a\x01\xA0\x8B\x01Ra\x04`\x8D\x015\x85\x8B\x01R\x87\x8D\x015a\x01\xE0\x8B\x01Ra\x04\xA0\x8D\x015\x84\x8B\x01Ra\x04\xC0\x8D\x015a\x02 \x8B\x01Ra\x04\xE0\x8D\x015\x83\x8B\x01Ra\x05\0\x8D\x015a\x02`\x8B\x01Ra\x05 \x8D\x015\x82\x8B\x01Ra\x05@\x8D\x015a\x02\xA0\x8B\x01Ra\x05`\x8D\x015\x81\x8B\x01RPPPPPPPPP\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aG\x01W`\0\x80\xFD[aG\n\x83aBpV[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aG'W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12aG;W`\0\x80\xFD[\x815\x81\x81\x11\x15aGMWaGMaB\xA2V[aG_`\x1F\x82\x01`\x1F\x19\x16\x85\x01aC\x03V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15aGuW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0` \x80\x83\x85\x03\x12\x15aG\xA6W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xBCW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aG\xCDW`\0\x80\xFD[\x805aG\xDBaD\x9C\x82aD1V[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aG\xFAW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aD\xDEW`@\x84\x89\x03\x12\x15aH\x18W`\0\x80\x81\xFD[aH aB\xB8V[aH)\x85aC3V[\x81R\x84\x86\x015\x86\x82\x01R\x82R`@\x90\x93\x01\x92\x90\x84\x01\x90aG\xFFV[`\x01`\x01`@\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a0\xECW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aH~W`\0\x80\xFD[a\x11\xC2\x82aHXV[`\0\x80`\0a\x01@\x84\x86\x03\x12\x15aH\x9DW`\0\x80\xFD[aH\xA7\x85\x85aCJV[\x92PaH\xB6a\x01\0\x85\x01aHXV[\x91PaH\xC5a\x01 \x85\x01aBpV[\x90P\x92P\x92P\x92V[`\0[\x83\x81\x10\x15aH\xE9W\x81\x81\x01Q\x83\x82\x01R` \x01aH\xD1V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01RaI\x11\x81`@\x85\x01` \x87\x01aH\xCEV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aI8W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x80\x82\x16\x91\x90\x82\x81\x14a1\xDDWa1\xDDaIGV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x11\xC5Wa\x11\xC5aIGV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15aI\xC9WaI\xC9aIGV[P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aI\xE2W`\0\x80\xFD[PQ\x91\x90PV[`\0\x81aI\xF8WaI\xF8aIGV[P`\0\x19\x01\x90V[`\0\x82QaJ\x12\x81\x84` \x87\x01aH\xCEV[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aJOWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88.+\x91Ea\x03i\x8A\xDFW\xB7\x99\x96\x9D\xEA\x1C\x8Fs\x9D\xA5\xD8\xD4\r\xD3\xEB\x92\"\xDB|\x81\xE8\x816\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE3\x1E\xE6x\xA0G\nu\xA6\xEA\xA8\xFE\x83p`I\x8B\xA8(\xA3p;1\x1D\x0Fw\xF0\x10BJ\xFE\xB0%\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE4 B\xA5\x87\xA9\x0C\x18{\n\x08|\x03\xE2\x9C\x96\x8B\x95\x0B\x1D\xB2m\\\x82\xD6f\x90Zh\x95y\x0C\n/\x8D\xD1\xF1\xA7X<B\xC4\xE1*D\xE1\x10@Ls\xCAl\x94\x81?\x85\x83]\xA4\xFB{\xB10\x1DJ\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static LIGHTCLIENTMOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LightClientMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LightClientMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LightClientMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LightClientMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LightClientMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LightClientMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LightClientMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LIGHTCLIENTMOCK_ABI.clone(),
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
                LIGHTCLIENTMOCK_ABI.clone(),
                LIGHTCLIENTMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `UPGRADE_INTERFACE_VERSION` (0xad3cb1cc) function
        pub fn upgrade_interface_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([173, 60, 177, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blocksPerEpoch` (0xf0682054) function
        pub fn blocks_per_epoch(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([240, 104, 32, 84], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeStakeTableComm` (0xaa922732) function
        pub fn compute_stake_table_comm(
            &self,
            state: LightClientState,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([170, 146, 39, 50], (state,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentEpoch` (0x76671808) function
        pub fn current_epoch(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([118, 103, 24, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disablePermissionedProverMode` (0x69cc6a04) function
        pub fn disable_permissioned_prover_mode(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 204, 106, 4], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `frozenStakeTableCommitment` (0x382b215a) function
        pub fn frozen_stake_table_commitment(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([56, 43, 33, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `frozenThreshold` (0xca6fe855) function
        pub fn frozen_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 111, 232, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFinalizedState` (0x82d07ff3) function
        pub fn get_finalized_state(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, LightClientState> {
            self.0
                .method_hash([130, 208, 127, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGenesisState` (0x4847ae5d) function
        pub fn get_genesis_state(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, LightClientState> {
            self.0
                .method_hash([72, 71, 174, 93], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHotShotBlockCommitmentsCount` (0x54646085) function
        pub fn get_hot_shot_block_commitments_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 100, 96, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHotShotCommitment` (0x8584d23f) function
        pub fn get_hot_shot_commitment(
            &self,
            hot_shot_block_height: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, HotShotCommitment> {
            self.0
                .method_hash([133, 132, 210, 63], hot_shot_block_height)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStateUpdateBlockNumbersCount` (0x7053fc51) function
        pub fn get_state_update_block_numbers_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 83, 252, 81], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVersion` (0x0d8e6e2c) function
        pub fn get_version(&self) -> ::ethers::contract::builders::ContractCall<M, (u8, u8, u8)> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hotShotCommitments` (0xdb13b60a) function
        pub fn hot_shot_commitments(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, ::ethers::core::types::U256)>
        {
            self.0
                .method_hash([219, 19, 182, 10], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xa244d596) function
        pub fn initialize(
            &self,
            genesis: LightClientState,
            num_blocks_per_epoch: u32,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 68, 213, 150], (genesis, num_blocks_per_epoch, owner))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lagOverEscapeHatchThreshold` (0xe0303301) function
        pub fn lag_over_escape_hatch_threshold(
            &self,
            block_number: ::ethers::core::types::U256,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([224, 48, 51, 1], (block_number, threshold))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `newFinalizedState` (0x409939b7) function
        pub fn new_finalized_state(
            &self,
            new_state: LightClientState,
            proof: PlonkProof,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 153, 57, 183], (new_state, proof))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permissionedProver` (0x313df7b1) function
        pub fn permissioned_prover(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([49, 61, 247, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permissionedProverEnabled` (0xbd32519a) function
        pub fn permissioned_prover_enabled(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([189, 50, 81, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCurrentEpoch` (0x3949d1e9) function
        pub fn set_current_epoch(
            &self,
            new_epoch: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 73, 209, 233], new_epoch)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFinalizedState` (0x202a0adb) function
        pub fn set_finalized_state(
            &self,
            state: LightClientState,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 42, 10, 219], (state,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setHotShotCommitments` (0x530ca78f) function
        pub fn set_hot_shot_commitments(
            &self,
            values: ::std::vec::Vec<HotShotCommitment>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 12, 167, 143], values)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setHotShotDownSince` (0x2d52aad6) function
        pub fn set_hot_shot_down_since(
            &self,
            l_1_height: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 82, 170, 214], l_1_height)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setHotShotUp` (0xc8e5e498) function
        pub fn set_hot_shot_up(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 229, 228, 152], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPermissionedProver` (0x013fa5fc) function
        pub fn set_permissioned_prover(
            &self,
            prover: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 63, 165, 252], prover)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStateUpdateBlockNumbers` (0x3919340f) function
        pub fn set_state_update_block_numbers(
            &self,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 25, 52, 15], values)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stateUpdateBlockNumbers` (0xa51e6fea) function
        pub fn state_update_block_numbers(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([165, 30, 111, 234], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `states` (0x7f17baad) function
        pub fn states(
            &self,
            index: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u64,
                u64,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([127, 23, 186, 173], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeToAndCall` (0x4f1ef286) function
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `votingStakeTableCommitment` (0x76b6b7cb) function
        pub fn voting_stake_table_commitment(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([118, 182, 183, 203], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `votingThreshold` (0x62827733) function
        pub fn voting_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([98, 130, 119, 51], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EpochChanged` event
        pub fn epoch_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EpochChangedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `NewState` event
        pub fn new_state_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewStateFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PermissionedProverNotRequired` event
        pub fn permissioned_prover_not_required_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PermissionedProverNotRequiredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PermissionedProverRequired` event
        pub fn permissioned_prover_required_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PermissionedProverRequiredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Upgrade` event
        pub fn upgrade_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradeFilter> {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LightClientMockEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for LightClientMock<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
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
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1967InvalidImplementation` with signature `ERC1967InvalidImplementation(address)` and selector `0x4c9c8ce3`
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
        name = "ERC1967InvalidImplementation",
        abi = "ERC1967InvalidImplementation(address)"
    )]
    pub struct ERC1967InvalidImplementation {
        pub implementation: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1967NonPayable` with signature `ERC1967NonPayable()` and selector `0xb398979f`
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
    #[etherror(name = "ERC1967NonPayable", abi = "ERC1967NonPayable()")]
    pub struct ERC1967NonPayable;
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
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
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
    ///Custom Error type `InsufficientSnapshotHistory` with signature `InsufficientSnapshotHistory()` and selector `0xb0b43877`
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
        name = "InsufficientSnapshotHistory",
        abi = "InsufficientSnapshotHistory()"
    )]
    pub struct InsufficientSnapshotHistory;
    ///Custom Error type `InvalidAddress` with signature `InvalidAddress()` and selector `0xe6c4247b`
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
    #[etherror(name = "InvalidAddress", abi = "InvalidAddress()")]
    pub struct InvalidAddress;
    ///Custom Error type `InvalidArgs` with signature `InvalidArgs()` and selector `0xa1ba07ee`
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
    #[etherror(name = "InvalidArgs", abi = "InvalidArgs()")]
    pub struct InvalidArgs;
    ///Custom Error type `InvalidHotShotBlockForCommitmentCheck` with signature `InvalidHotShotBlockForCommitmentCheck()` and selector `0x615a9264`
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
        name = "InvalidHotShotBlockForCommitmentCheck",
        abi = "InvalidHotShotBlockForCommitmentCheck()"
    )]
    pub struct InvalidHotShotBlockForCommitmentCheck;
    ///Custom Error type `InvalidInitialization` with signature `InvalidInitialization()` and selector `0xf92ee8a9`
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
    #[etherror(name = "InvalidInitialization", abi = "InvalidInitialization()")]
    pub struct InvalidInitialization;
    ///Custom Error type `InvalidProof` with signature `InvalidProof()` and selector `0x09bde339`
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
    #[etherror(name = "InvalidProof", abi = "InvalidProof()")]
    pub struct InvalidProof;
    ///Custom Error type `MissingLastBlockForCurrentEpoch` with signature `MissingLastBlockForCurrentEpoch(uint64)` and selector `0x1b2335f8`
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
        name = "MissingLastBlockForCurrentEpoch",
        abi = "MissingLastBlockForCurrentEpoch(uint64)"
    )]
    pub struct MissingLastBlockForCurrentEpoch {
        pub expected_block_height: u64,
    }
    ///Custom Error type `NoChangeRequired` with signature `NoChangeRequired()` and selector `0xa863aec9`
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
    #[etherror(name = "NoChangeRequired", abi = "NoChangeRequired()")]
    pub struct NoChangeRequired;
    ///Custom Error type `NotInitializing` with signature `NotInitializing()` and selector `0xd7e6bcf8`
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
    #[etherror(name = "NotInitializing", abi = "NotInitializing()")]
    pub struct NotInitializing;
    ///Custom Error type `OutdatedState` with signature `OutdatedState()` and selector `0x051c46ef`
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
    #[etherror(name = "OutdatedState", abi = "OutdatedState()")]
    pub struct OutdatedState;
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
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
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
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
        name = "OwnableUnauthorizedAccount",
        abi = "OwnableUnauthorizedAccount(address)"
    )]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `PermissionedProverNotSet` with signature `PermissionedProverNotSet()` and selector `0x25cda3ce`
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
    #[etherror(name = "PermissionedProverNotSet", abi = "PermissionedProverNotSet()")]
    pub struct PermissionedProverNotSet;
    ///Custom Error type `ProverNotPermissioned` with signature `ProverNotPermissioned()` and selector `0xa3a64780`
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
    #[etherror(name = "ProverNotPermissioned", abi = "ProverNotPermissioned()")]
    pub struct ProverNotPermissioned;
    ///Custom Error type `UUPSUnauthorizedCallContext` with signature `UUPSUnauthorizedCallContext()` and selector `0xe07c8dba`
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
        name = "UUPSUnauthorizedCallContext",
        abi = "UUPSUnauthorizedCallContext()"
    )]
    pub struct UUPSUnauthorizedCallContext;
    ///Custom Error type `UUPSUnsupportedProxiableUUID` with signature `UUPSUnsupportedProxiableUUID(bytes32)` and selector `0xaa1d49a4`
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
        name = "UUPSUnsupportedProxiableUUID",
        abi = "UUPSUnsupportedProxiableUUID(bytes32)"
    )]
    pub struct UUPSUnsupportedProxiableUUID {
        pub slot: [u8; 32],
    }
    ///Custom Error type `UnsupportedDegree` with signature `UnsupportedDegree()` and selector `0xe2ef09e5`
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
    #[etherror(name = "UnsupportedDegree", abi = "UnsupportedDegree()")]
    pub struct UnsupportedDegree;
    ///Custom Error type `WrongPlonkVK` with signature `WrongPlonkVK()` and selector `0x41f53b12`
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
    #[etherror(name = "WrongPlonkVK", abi = "WrongPlonkVK()")]
    pub struct WrongPlonkVK;
    ///Custom Error type `WrongStakeTableUsed` with signature `WrongStakeTableUsed()` and selector `0x51618089`
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
    #[etherror(name = "WrongStakeTableUsed", abi = "WrongStakeTableUsed()")]
    pub struct WrongStakeTableUsed;
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
    pub enum LightClientMockErrors {
        AddressEmptyCode(AddressEmptyCode),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedInnerCall(FailedInnerCall),
        InsufficientSnapshotHistory(InsufficientSnapshotHistory),
        InvalidAddress(InvalidAddress),
        InvalidArgs(InvalidArgs),
        InvalidHotShotBlockForCommitmentCheck(InvalidHotShotBlockForCommitmentCheck),
        InvalidInitialization(InvalidInitialization),
        InvalidProof(InvalidProof),
        MissingLastBlockForCurrentEpoch(MissingLastBlockForCurrentEpoch),
        NoChangeRequired(NoChangeRequired),
        NotInitializing(NotInitializing),
        OutdatedState(OutdatedState),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        PermissionedProverNotSet(PermissionedProverNotSet),
        ProverNotPermissioned(ProverNotPermissioned),
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        UnsupportedDegree(UnsupportedDegree),
        WrongPlonkVK(WrongPlonkVK),
        WrongStakeTableUsed(WrongStakeTableUsed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LightClientMockErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) =
                <ERC1967InvalidImplementation as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1967InvalidImplementation(decoded));
            }
            if let Ok(decoded) = <ERC1967NonPayable as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1967NonPayable(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) =
                <InsufficientSnapshotHistory as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientSnapshotHistory(decoded));
            }
            if let Ok(decoded) = <InvalidAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidAddress(decoded));
            }
            if let Ok(decoded) = <InvalidArgs as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidArgs(decoded));
            }
            if let Ok(decoded) =
                <InvalidHotShotBlockForCommitmentCheck as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::InvalidHotShotBlockForCommitmentCheck(decoded));
            }
            if let Ok(decoded) =
                <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) = <InvalidProof as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidProof(decoded));
            }
            if let Ok(decoded) =
                <MissingLastBlockForCurrentEpoch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MissingLastBlockForCurrentEpoch(decoded));
            }
            if let Ok(decoded) = <NoChangeRequired as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NoChangeRequired(decoded));
            }
            if let Ok(decoded) = <NotInitializing as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotInitializing(decoded));
            }
            if let Ok(decoded) = <OutdatedState as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutdatedState(decoded));
            }
            if let Ok(decoded) =
                <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) =
                <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) =
                <PermissionedProverNotSet as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PermissionedProverNotSet(decoded));
            }
            if let Ok(decoded) =
                <ProverNotPermissioned as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProverNotPermissioned(decoded));
            }
            if let Ok(decoded) =
                <UUPSUnauthorizedCallContext as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UUPSUnauthorizedCallContext(decoded));
            }
            if let Ok(decoded) =
                <UUPSUnsupportedProxiableUUID as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UUPSUnsupportedProxiableUUID(decoded));
            }
            if let Ok(decoded) = <UnsupportedDegree as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsupportedDegree(decoded));
            }
            if let Ok(decoded) = <WrongPlonkVK as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongPlonkVK(decoded));
            }
            if let Ok(decoded) =
                <WrongStakeTableUsed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WrongStakeTableUsed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LightClientMockErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC1967InvalidImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967NonPayable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedInnerCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InsufficientSnapshotHistory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidArgs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidHotShotBlockForCommitmentCheck(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidProof(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MissingLastBlockForCurrentEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoChangeRequired(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotInitializing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OutdatedState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PermissionedProverNotSet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProverNotPermissioned(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UUPSUnauthorizedCallContext(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsupportedDegree(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongPlonkVK(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongStakeTableUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LightClientMockErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967InvalidImplementation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967NonPayable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientSnapshotHistory as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidArgs as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidHotShotBlockForCommitmentCheck as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidProof as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <MissingLastBlockForCurrentEpoch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoChangeRequired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotInitializing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutdatedState as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PermissionedProverNotSet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ProverNotPermissioned as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UUPSUnauthorizedCallContext as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UUPSUnsupportedProxiableUUID as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnsupportedDegree as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <WrongPlonkVK as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <WrongStakeTableUsed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LightClientMockErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1967InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967NonPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientSnapshotHistory(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidArgs(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidHotShotBlockForCommitmentCheck(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidInitialization(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::MissingLastBlockForCurrentEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoChangeRequired(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotInitializing(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutdatedState(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableInvalidOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableUnauthorizedAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermissionedProverNotSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProverNotPermissioned(element) => ::core::fmt::Display::fmt(element, f),
                Self::UUPSUnauthorizedCallContext(element) => ::core::fmt::Display::fmt(element, f),
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnsupportedDegree(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongPlonkVK(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongStakeTableUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LightClientMockErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for LightClientMockErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation> for LightClientMockErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for LightClientMockErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for LightClientMockErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<InsufficientSnapshotHistory> for LightClientMockErrors {
        fn from(value: InsufficientSnapshotHistory) -> Self {
            Self::InsufficientSnapshotHistory(value)
        }
    }
    impl ::core::convert::From<InvalidAddress> for LightClientMockErrors {
        fn from(value: InvalidAddress) -> Self {
            Self::InvalidAddress(value)
        }
    }
    impl ::core::convert::From<InvalidArgs> for LightClientMockErrors {
        fn from(value: InvalidArgs) -> Self {
            Self::InvalidArgs(value)
        }
    }
    impl ::core::convert::From<InvalidHotShotBlockForCommitmentCheck> for LightClientMockErrors {
        fn from(value: InvalidHotShotBlockForCommitmentCheck) -> Self {
            Self::InvalidHotShotBlockForCommitmentCheck(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for LightClientMockErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<InvalidProof> for LightClientMockErrors {
        fn from(value: InvalidProof) -> Self {
            Self::InvalidProof(value)
        }
    }
    impl ::core::convert::From<MissingLastBlockForCurrentEpoch> for LightClientMockErrors {
        fn from(value: MissingLastBlockForCurrentEpoch) -> Self {
            Self::MissingLastBlockForCurrentEpoch(value)
        }
    }
    impl ::core::convert::From<NoChangeRequired> for LightClientMockErrors {
        fn from(value: NoChangeRequired) -> Self {
            Self::NoChangeRequired(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for LightClientMockErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<OutdatedState> for LightClientMockErrors {
        fn from(value: OutdatedState) -> Self {
            Self::OutdatedState(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for LightClientMockErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for LightClientMockErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<PermissionedProverNotSet> for LightClientMockErrors {
        fn from(value: PermissionedProverNotSet) -> Self {
            Self::PermissionedProverNotSet(value)
        }
    }
    impl ::core::convert::From<ProverNotPermissioned> for LightClientMockErrors {
        fn from(value: ProverNotPermissioned) -> Self {
            Self::ProverNotPermissioned(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext> for LightClientMockErrors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID> for LightClientMockErrors {
        fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
            Self::UUPSUnsupportedProxiableUUID(value)
        }
    }
    impl ::core::convert::From<UnsupportedDegree> for LightClientMockErrors {
        fn from(value: UnsupportedDegree) -> Self {
            Self::UnsupportedDegree(value)
        }
    }
    impl ::core::convert::From<WrongPlonkVK> for LightClientMockErrors {
        fn from(value: WrongPlonkVK) -> Self {
            Self::WrongPlonkVK(value)
        }
    }
    impl ::core::convert::From<WrongStakeTableUsed> for LightClientMockErrors {
        fn from(value: WrongStakeTableUsed) -> Self {
            Self::WrongStakeTableUsed(value)
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
    #[ethevent(name = "EpochChanged", abi = "EpochChanged(uint64)")]
    pub struct EpochChangedFilter(pub u64);
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint64)")]
    pub struct InitializedFilter {
        pub version: u64,
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
    #[ethevent(name = "NewState", abi = "NewState(uint64,uint64,uint256)")]
    pub struct NewStateFilter {
        #[ethevent(indexed)]
        pub view_num: u64,
        #[ethevent(indexed)]
        pub block_height: u64,
        pub block_comm_root: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "PermissionedProverNotRequired",
        abi = "PermissionedProverNotRequired()"
    )]
    pub struct PermissionedProverNotRequiredFilter;
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
        name = "PermissionedProverRequired",
        abi = "PermissionedProverRequired(address)"
    )]
    pub struct PermissionedProverRequiredFilter {
        pub permissioned_prover: ::ethers::core::types::Address,
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
    #[ethevent(name = "Upgrade", abi = "Upgrade(address)")]
    pub struct UpgradeFilter {
        pub implementation: ::ethers::core::types::Address,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
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
    pub enum LightClientMockEvents {
        EpochChangedFilter(EpochChangedFilter),
        InitializedFilter(InitializedFilter),
        NewStateFilter(NewStateFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PermissionedProverNotRequiredFilter(PermissionedProverNotRequiredFilter),
        PermissionedProverRequiredFilter(PermissionedProverRequiredFilter),
        UpgradeFilter(UpgradeFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for LightClientMockEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EpochChangedFilter::decode_log(log) {
                return Ok(LightClientMockEvents::EpochChangedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(LightClientMockEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = NewStateFilter::decode_log(log) {
                return Ok(LightClientMockEvents::NewStateFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(LightClientMockEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PermissionedProverNotRequiredFilter::decode_log(log) {
                return Ok(LightClientMockEvents::PermissionedProverNotRequiredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PermissionedProverRequiredFilter::decode_log(log) {
                return Ok(LightClientMockEvents::PermissionedProverRequiredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UpgradeFilter::decode_log(log) {
                return Ok(LightClientMockEvents::UpgradeFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(LightClientMockEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LightClientMockEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EpochChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewStateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermissionedProverNotRequiredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PermissionedProverRequiredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EpochChangedFilter> for LightClientMockEvents {
        fn from(value: EpochChangedFilter) -> Self {
            Self::EpochChangedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for LightClientMockEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<NewStateFilter> for LightClientMockEvents {
        fn from(value: NewStateFilter) -> Self {
            Self::NewStateFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for LightClientMockEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PermissionedProverNotRequiredFilter> for LightClientMockEvents {
        fn from(value: PermissionedProverNotRequiredFilter) -> Self {
            Self::PermissionedProverNotRequiredFilter(value)
        }
    }
    impl ::core::convert::From<PermissionedProverRequiredFilter> for LightClientMockEvents {
        fn from(value: PermissionedProverRequiredFilter) -> Self {
            Self::PermissionedProverRequiredFilter(value)
        }
    }
    impl ::core::convert::From<UpgradeFilter> for LightClientMockEvents {
        fn from(value: UpgradeFilter) -> Self {
            Self::UpgradeFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for LightClientMockEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `UPGRADE_INTERFACE_VERSION` function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`
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
        name = "UPGRADE_INTERFACE_VERSION",
        abi = "UPGRADE_INTERFACE_VERSION()"
    )]
    pub struct UpgradeInterfaceVersionCall;
    ///Container type for all input parameters for the `blocksPerEpoch` function with signature `blocksPerEpoch()` and selector `0xf0682054`
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
    #[ethcall(name = "blocksPerEpoch", abi = "blocksPerEpoch()")]
    pub struct BlocksPerEpochCall;
    ///Container type for all input parameters for the `computeStakeTableComm` function with signature `computeStakeTableComm((uint64,uint64,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0xaa922732`
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
        name = "computeStakeTableComm",
        abi = "computeStakeTableComm((uint64,uint64,uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct ComputeStakeTableCommCall {
        pub state: LightClientState,
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
    ///Container type for all input parameters for the `disablePermissionedProverMode` function with signature `disablePermissionedProverMode()` and selector `0x69cc6a04`
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
        name = "disablePermissionedProverMode",
        abi = "disablePermissionedProverMode()"
    )]
    pub struct DisablePermissionedProverModeCall;
    ///Container type for all input parameters for the `frozenStakeTableCommitment` function with signature `frozenStakeTableCommitment()` and selector `0x382b215a`
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
        name = "frozenStakeTableCommitment",
        abi = "frozenStakeTableCommitment()"
    )]
    pub struct FrozenStakeTableCommitmentCall;
    ///Container type for all input parameters for the `frozenThreshold` function with signature `frozenThreshold()` and selector `0xca6fe855`
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
    #[ethcall(name = "frozenThreshold", abi = "frozenThreshold()")]
    pub struct FrozenThresholdCall;
    ///Container type for all input parameters for the `getFinalizedState` function with signature `getFinalizedState()` and selector `0x82d07ff3`
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
    #[ethcall(name = "getFinalizedState", abi = "getFinalizedState()")]
    pub struct GetFinalizedStateCall;
    ///Container type for all input parameters for the `getGenesisState` function with signature `getGenesisState()` and selector `0x4847ae5d`
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
    #[ethcall(name = "getGenesisState", abi = "getGenesisState()")]
    pub struct GetGenesisStateCall;
    ///Container type for all input parameters for the `getHotShotBlockCommitmentsCount` function with signature `getHotShotBlockCommitmentsCount()` and selector `0x54646085`
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
        name = "getHotShotBlockCommitmentsCount",
        abi = "getHotShotBlockCommitmentsCount()"
    )]
    pub struct GetHotShotBlockCommitmentsCountCall;
    ///Container type for all input parameters for the `getHotShotCommitment` function with signature `getHotShotCommitment(uint256)` and selector `0x8584d23f`
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
    #[ethcall(name = "getHotShotCommitment", abi = "getHotShotCommitment(uint256)")]
    pub struct GetHotShotCommitmentCall {
        pub hot_shot_block_height: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getStateUpdateBlockNumbersCount` function with signature `getStateUpdateBlockNumbersCount()` and selector `0x7053fc51`
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
        name = "getStateUpdateBlockNumbersCount",
        abi = "getStateUpdateBlockNumbersCount()"
    )]
    pub struct GetStateUpdateBlockNumbersCountCall;
    ///Container type for all input parameters for the `getVersion` function with signature `getVersion()` and selector `0x0d8e6e2c`
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
    #[ethcall(name = "getVersion", abi = "getVersion()")]
    pub struct GetVersionCall;
    ///Container type for all input parameters for the `hotShotCommitments` function with signature `hotShotCommitments(uint256)` and selector `0xdb13b60a`
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
    #[ethcall(name = "hotShotCommitments", abi = "hotShotCommitments(uint256)")]
    pub struct HotShotCommitmentsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `initialize` function with signature `initialize((uint64,uint64,uint256,uint256,uint256,uint256,uint256,uint256),uint32,address)` and selector `0xa244d596`
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
        name = "initialize",
        abi = "initialize((uint64,uint64,uint256,uint256,uint256,uint256,uint256,uint256),uint32,address)"
    )]
    pub struct InitializeCall {
        pub genesis: LightClientState,
        pub num_blocks_per_epoch: u32,
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lagOverEscapeHatchThreshold` function with signature `lagOverEscapeHatchThreshold(uint256,uint256)` and selector `0xe0303301`
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
        name = "lagOverEscapeHatchThreshold",
        abi = "lagOverEscapeHatchThreshold(uint256,uint256)"
    )]
    pub struct LagOverEscapeHatchThresholdCall {
        pub block_number: ::ethers::core::types::U256,
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `newFinalizedState` function with signature `newFinalizedState((uint64,uint64,uint256,uint256,uint256,uint256,uint256,uint256),((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x409939b7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[ethcall(
        name = "newFinalizedState",
        abi = "newFinalizedState((uint64,uint64,uint256,uint256,uint256,uint256,uint256,uint256),((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct NewFinalizedStateCall {
        pub new_state: LightClientState,
        pub proof: PlonkProof,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `permissionedProver` function with signature `permissionedProver()` and selector `0x313df7b1`
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
    #[ethcall(name = "permissionedProver", abi = "permissionedProver()")]
    pub struct PermissionedProverCall;
    ///Container type for all input parameters for the `permissionedProverEnabled` function with signature `permissionedProverEnabled()` and selector `0xbd32519a`
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
        name = "permissionedProverEnabled",
        abi = "permissionedProverEnabled()"
    )]
    pub struct PermissionedProverEnabledCall;
    ///Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setCurrentEpoch` function with signature `setCurrentEpoch(uint64)` and selector `0x3949d1e9`
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
    #[ethcall(name = "setCurrentEpoch", abi = "setCurrentEpoch(uint64)")]
    pub struct SetCurrentEpochCall {
        pub new_epoch: u64,
    }
    ///Container type for all input parameters for the `setFinalizedState` function with signature `setFinalizedState((uint64,uint64,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x202a0adb`
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
        name = "setFinalizedState",
        abi = "setFinalizedState((uint64,uint64,uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct SetFinalizedStateCall {
        pub state: LightClientState,
    }
    ///Container type for all input parameters for the `setHotShotCommitments` function with signature `setHotShotCommitments((uint64,uint256)[])` and selector `0x530ca78f`
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
        name = "setHotShotCommitments",
        abi = "setHotShotCommitments((uint64,uint256)[])"
    )]
    pub struct SetHotShotCommitmentsCall {
        pub values: ::std::vec::Vec<HotShotCommitment>,
    }
    ///Container type for all input parameters for the `setHotShotDownSince` function with signature `setHotShotDownSince(uint256)` and selector `0x2d52aad6`
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
    #[ethcall(name = "setHotShotDownSince", abi = "setHotShotDownSince(uint256)")]
    pub struct SetHotShotDownSinceCall {
        pub l_1_height: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setHotShotUp` function with signature `setHotShotUp()` and selector `0xc8e5e498`
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
    #[ethcall(name = "setHotShotUp", abi = "setHotShotUp()")]
    pub struct SetHotShotUpCall;
    ///Container type for all input parameters for the `setPermissionedProver` function with signature `setPermissionedProver(address)` and selector `0x013fa5fc`
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
    #[ethcall(name = "setPermissionedProver", abi = "setPermissionedProver(address)")]
    pub struct SetPermissionedProverCall {
        pub prover: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setStateUpdateBlockNumbers` function with signature `setStateUpdateBlockNumbers(uint256[])` and selector `0x3919340f`
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
        name = "setStateUpdateBlockNumbers",
        abi = "setStateUpdateBlockNumbers(uint256[])"
    )]
    pub struct SetStateUpdateBlockNumbersCall {
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `stateUpdateBlockNumbers` function with signature `stateUpdateBlockNumbers(uint256)` and selector `0xa51e6fea`
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
        name = "stateUpdateBlockNumbers",
        abi = "stateUpdateBlockNumbers(uint256)"
    )]
    pub struct StateUpdateBlockNumbersCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `states` function with signature `states(uint32)` and selector `0x7f17baad`
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
    #[ethcall(name = "states", abi = "states(uint32)")]
    pub struct StatesCall {
        pub index: u32,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`
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
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `votingStakeTableCommitment` function with signature `votingStakeTableCommitment()` and selector `0x76b6b7cb`
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
        name = "votingStakeTableCommitment",
        abi = "votingStakeTableCommitment()"
    )]
    pub struct VotingStakeTableCommitmentCall;
    ///Container type for all input parameters for the `votingThreshold` function with signature `votingThreshold()` and selector `0x62827733`
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
    #[ethcall(name = "votingThreshold", abi = "votingThreshold()")]
    pub struct VotingThresholdCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize)]
    pub enum LightClientMockCalls {
        UpgradeInterfaceVersion(UpgradeInterfaceVersionCall),
        BlocksPerEpoch(BlocksPerEpochCall),
        ComputeStakeTableComm(ComputeStakeTableCommCall),
        CurrentEpoch(CurrentEpochCall),
        DisablePermissionedProverMode(DisablePermissionedProverModeCall),
        FrozenStakeTableCommitment(FrozenStakeTableCommitmentCall),
        FrozenThreshold(FrozenThresholdCall),
        GetFinalizedState(GetFinalizedStateCall),
        GetGenesisState(GetGenesisStateCall),
        GetHotShotBlockCommitmentsCount(GetHotShotBlockCommitmentsCountCall),
        GetHotShotCommitment(GetHotShotCommitmentCall),
        GetStateUpdateBlockNumbersCount(GetStateUpdateBlockNumbersCountCall),
        GetVersion(GetVersionCall),
        HotShotCommitments(HotShotCommitmentsCall),
        Initialize(InitializeCall),
        LagOverEscapeHatchThreshold(LagOverEscapeHatchThresholdCall),
        NewFinalizedState(NewFinalizedStateCall),
        Owner(OwnerCall),
        PermissionedProver(PermissionedProverCall),
        PermissionedProverEnabled(PermissionedProverEnabledCall),
        ProxiableUUID(ProxiableUUIDCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetCurrentEpoch(SetCurrentEpochCall),
        SetFinalizedState(SetFinalizedStateCall),
        SetHotShotCommitments(SetHotShotCommitmentsCall),
        SetHotShotDownSince(SetHotShotDownSinceCall),
        SetHotShotUp(SetHotShotUpCall),
        SetPermissionedProver(SetPermissionedProverCall),
        SetStateUpdateBlockNumbers(SetStateUpdateBlockNumbersCall),
        StateUpdateBlockNumbers(StateUpdateBlockNumbersCall),
        States(StatesCall),
        TransferOwnership(TransferOwnershipCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        VotingStakeTableCommitment(VotingStakeTableCommitmentCall),
        VotingThreshold(VotingThresholdCall),
    }
    impl ::ethers::core::abi::AbiDecode for LightClientMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <UpgradeInterfaceVersionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeInterfaceVersion(decoded));
            }
            if let Ok(decoded) =
                <BlocksPerEpochCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BlocksPerEpoch(decoded));
            }
            if let Ok(decoded) =
                <ComputeStakeTableCommCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeStakeTableComm(decoded));
            }
            if let Ok(decoded) = <CurrentEpochCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CurrentEpoch(decoded));
            }
            if let Ok(decoded) =
                <DisablePermissionedProverModeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DisablePermissionedProverMode(decoded));
            }
            if let Ok(decoded) =
                <FrozenStakeTableCommitmentCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FrozenStakeTableCommitment(decoded));
            }
            if let Ok(decoded) =
                <FrozenThresholdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FrozenThreshold(decoded));
            }
            if let Ok(decoded) =
                <GetFinalizedStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetFinalizedState(decoded));
            }
            if let Ok(decoded) =
                <GetGenesisStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetGenesisState(decoded));
            }
            if let Ok(decoded) =
                <GetHotShotBlockCommitmentsCountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetHotShotBlockCommitmentsCount(decoded));
            }
            if let Ok(decoded) =
                <GetHotShotCommitmentCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetHotShotCommitment(decoded));
            }
            if let Ok(decoded) =
                <GetStateUpdateBlockNumbersCountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetStateUpdateBlockNumbersCount(decoded));
            }
            if let Ok(decoded) = <GetVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetVersion(decoded));
            }
            if let Ok(decoded) =
                <HotShotCommitmentsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HotShotCommitments(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LagOverEscapeHatchThresholdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LagOverEscapeHatchThreshold(decoded));
            }
            if let Ok(decoded) =
                <NewFinalizedStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NewFinalizedState(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <PermissionedProverCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PermissionedProver(decoded));
            }
            if let Ok(decoded) =
                <PermissionedProverEnabledCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PermissionedProverEnabled(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetCurrentEpochCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetCurrentEpoch(decoded));
            }
            if let Ok(decoded) =
                <SetFinalizedStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFinalizedState(decoded));
            }
            if let Ok(decoded) =
                <SetHotShotCommitmentsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetHotShotCommitments(decoded));
            }
            if let Ok(decoded) =
                <SetHotShotDownSinceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetHotShotDownSince(decoded));
            }
            if let Ok(decoded) = <SetHotShotUpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetHotShotUp(decoded));
            }
            if let Ok(decoded) =
                <SetPermissionedProverCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPermissionedProver(decoded));
            }
            if let Ok(decoded) =
                <SetStateUpdateBlockNumbersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetStateUpdateBlockNumbers(decoded));
            }
            if let Ok(decoded) =
                <StateUpdateBlockNumbersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StateUpdateBlockNumbers(decoded));
            }
            if let Ok(decoded) = <StatesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::States(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) =
                <VotingStakeTableCommitmentCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VotingStakeTableCommitment(decoded));
            }
            if let Ok(decoded) =
                <VotingThresholdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VotingThreshold(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LightClientMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::UpgradeInterfaceVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlocksPerEpoch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComputeStakeTableComm(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentEpoch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DisablePermissionedProverMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FrozenStakeTableCommitment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FrozenThreshold(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetFinalizedState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetGenesisState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetHotShotBlockCommitmentsCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHotShotCommitment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStateUpdateBlockNumbersCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HotShotCommitments(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LagOverEscapeHatchThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NewFinalizedState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PermissionedProver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PermissionedProverEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxiableUUID(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetCurrentEpoch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFinalizedState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetHotShotCommitments(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetHotShotDownSince(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetHotShotUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPermissionedProver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStateUpdateBlockNumbers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StateUpdateBlockNumbers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::States(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeToAndCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VotingStakeTableCommitment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VotingThreshold(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LightClientMockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::UpgradeInterfaceVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlocksPerEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeStakeTableComm(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisablePermissionedProverMode(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FrozenStakeTableCommitment(element) => ::core::fmt::Display::fmt(element, f),
                Self::FrozenThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFinalizedState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGenesisState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHotShotBlockCommitmentsCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetHotShotCommitment(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStateUpdateBlockNumbersCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::HotShotCommitments(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LagOverEscapeHatchThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewFinalizedState(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermissionedProver(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermissionedProverEnabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCurrentEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFinalizedState(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetHotShotCommitments(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetHotShotDownSince(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetHotShotUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPermissionedProver(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStateUpdateBlockNumbers(element) => ::core::fmt::Display::fmt(element, f),
                Self::StateUpdateBlockNumbers(element) => ::core::fmt::Display::fmt(element, f),
                Self::States(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotingStakeTableCommitment(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotingThreshold(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall> for LightClientMockCalls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<BlocksPerEpochCall> for LightClientMockCalls {
        fn from(value: BlocksPerEpochCall) -> Self {
            Self::BlocksPerEpoch(value)
        }
    }
    impl ::core::convert::From<ComputeStakeTableCommCall> for LightClientMockCalls {
        fn from(value: ComputeStakeTableCommCall) -> Self {
            Self::ComputeStakeTableComm(value)
        }
    }
    impl ::core::convert::From<CurrentEpochCall> for LightClientMockCalls {
        fn from(value: CurrentEpochCall) -> Self {
            Self::CurrentEpoch(value)
        }
    }
    impl ::core::convert::From<DisablePermissionedProverModeCall> for LightClientMockCalls {
        fn from(value: DisablePermissionedProverModeCall) -> Self {
            Self::DisablePermissionedProverMode(value)
        }
    }
    impl ::core::convert::From<FrozenStakeTableCommitmentCall> for LightClientMockCalls {
        fn from(value: FrozenStakeTableCommitmentCall) -> Self {
            Self::FrozenStakeTableCommitment(value)
        }
    }
    impl ::core::convert::From<FrozenThresholdCall> for LightClientMockCalls {
        fn from(value: FrozenThresholdCall) -> Self {
            Self::FrozenThreshold(value)
        }
    }
    impl ::core::convert::From<GetFinalizedStateCall> for LightClientMockCalls {
        fn from(value: GetFinalizedStateCall) -> Self {
            Self::GetFinalizedState(value)
        }
    }
    impl ::core::convert::From<GetGenesisStateCall> for LightClientMockCalls {
        fn from(value: GetGenesisStateCall) -> Self {
            Self::GetGenesisState(value)
        }
    }
    impl ::core::convert::From<GetHotShotBlockCommitmentsCountCall> for LightClientMockCalls {
        fn from(value: GetHotShotBlockCommitmentsCountCall) -> Self {
            Self::GetHotShotBlockCommitmentsCount(value)
        }
    }
    impl ::core::convert::From<GetHotShotCommitmentCall> for LightClientMockCalls {
        fn from(value: GetHotShotCommitmentCall) -> Self {
            Self::GetHotShotCommitment(value)
        }
    }
    impl ::core::convert::From<GetStateUpdateBlockNumbersCountCall> for LightClientMockCalls {
        fn from(value: GetStateUpdateBlockNumbersCountCall) -> Self {
            Self::GetStateUpdateBlockNumbersCount(value)
        }
    }
    impl ::core::convert::From<GetVersionCall> for LightClientMockCalls {
        fn from(value: GetVersionCall) -> Self {
            Self::GetVersion(value)
        }
    }
    impl ::core::convert::From<HotShotCommitmentsCall> for LightClientMockCalls {
        fn from(value: HotShotCommitmentsCall) -> Self {
            Self::HotShotCommitments(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for LightClientMockCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LagOverEscapeHatchThresholdCall> for LightClientMockCalls {
        fn from(value: LagOverEscapeHatchThresholdCall) -> Self {
            Self::LagOverEscapeHatchThreshold(value)
        }
    }
    impl ::core::convert::From<NewFinalizedStateCall> for LightClientMockCalls {
        fn from(value: NewFinalizedStateCall) -> Self {
            Self::NewFinalizedState(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LightClientMockCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PermissionedProverCall> for LightClientMockCalls {
        fn from(value: PermissionedProverCall) -> Self {
            Self::PermissionedProver(value)
        }
    }
    impl ::core::convert::From<PermissionedProverEnabledCall> for LightClientMockCalls {
        fn from(value: PermissionedProverEnabledCall) -> Self {
            Self::PermissionedProverEnabled(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for LightClientMockCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for LightClientMockCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetCurrentEpochCall> for LightClientMockCalls {
        fn from(value: SetCurrentEpochCall) -> Self {
            Self::SetCurrentEpoch(value)
        }
    }
    impl ::core::convert::From<SetFinalizedStateCall> for LightClientMockCalls {
        fn from(value: SetFinalizedStateCall) -> Self {
            Self::SetFinalizedState(value)
        }
    }
    impl ::core::convert::From<SetHotShotCommitmentsCall> for LightClientMockCalls {
        fn from(value: SetHotShotCommitmentsCall) -> Self {
            Self::SetHotShotCommitments(value)
        }
    }
    impl ::core::convert::From<SetHotShotDownSinceCall> for LightClientMockCalls {
        fn from(value: SetHotShotDownSinceCall) -> Self {
            Self::SetHotShotDownSince(value)
        }
    }
    impl ::core::convert::From<SetHotShotUpCall> for LightClientMockCalls {
        fn from(value: SetHotShotUpCall) -> Self {
            Self::SetHotShotUp(value)
        }
    }
    impl ::core::convert::From<SetPermissionedProverCall> for LightClientMockCalls {
        fn from(value: SetPermissionedProverCall) -> Self {
            Self::SetPermissionedProver(value)
        }
    }
    impl ::core::convert::From<SetStateUpdateBlockNumbersCall> for LightClientMockCalls {
        fn from(value: SetStateUpdateBlockNumbersCall) -> Self {
            Self::SetStateUpdateBlockNumbers(value)
        }
    }
    impl ::core::convert::From<StateUpdateBlockNumbersCall> for LightClientMockCalls {
        fn from(value: StateUpdateBlockNumbersCall) -> Self {
            Self::StateUpdateBlockNumbers(value)
        }
    }
    impl ::core::convert::From<StatesCall> for LightClientMockCalls {
        fn from(value: StatesCall) -> Self {
            Self::States(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LightClientMockCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for LightClientMockCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<VotingStakeTableCommitmentCall> for LightClientMockCalls {
        fn from(value: VotingStakeTableCommitmentCall) -> Self {
            Self::VotingStakeTableCommitment(value)
        }
    }
    impl ::core::convert::From<VotingThresholdCall> for LightClientMockCalls {
        fn from(value: VotingThresholdCall) -> Self {
            Self::VotingThreshold(value)
        }
    }
    ///Container type for all return fields from the `UPGRADE_INTERFACE_VERSION` function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`
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
    pub struct UpgradeInterfaceVersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `blocksPerEpoch` function with signature `blocksPerEpoch()` and selector `0xf0682054`
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
    pub struct BlocksPerEpochReturn(pub u32);
    ///Container type for all return fields from the `computeStakeTableComm` function with signature `computeStakeTableComm((uint64,uint64,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0xaa922732`
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
    pub struct ComputeStakeTableCommReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `frozenStakeTableCommitment` function with signature `frozenStakeTableCommitment()` and selector `0x382b215a`
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
    pub struct FrozenStakeTableCommitmentReturn(pub [u8; 32]);
    ///Container type for all return fields from the `frozenThreshold` function with signature `frozenThreshold()` and selector `0xca6fe855`
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
    pub struct FrozenThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getFinalizedState` function with signature `getFinalizedState()` and selector `0x82d07ff3`
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
    pub struct GetFinalizedStateReturn(pub LightClientState);
    ///Container type for all return fields from the `getGenesisState` function with signature `getGenesisState()` and selector `0x4847ae5d`
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
    pub struct GetGenesisStateReturn(pub LightClientState);
    ///Container type for all return fields from the `getHotShotBlockCommitmentsCount` function with signature `getHotShotBlockCommitmentsCount()` and selector `0x54646085`
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
    pub struct GetHotShotBlockCommitmentsCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getHotShotCommitment` function with signature `getHotShotCommitment(uint256)` and selector `0x8584d23f`
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
    pub struct GetHotShotCommitmentReturn(pub HotShotCommitment);
    ///Container type for all return fields from the `getStateUpdateBlockNumbersCount` function with signature `getStateUpdateBlockNumbersCount()` and selector `0x7053fc51`
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
    pub struct GetStateUpdateBlockNumbersCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVersion` function with signature `getVersion()` and selector `0x0d8e6e2c`
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
    pub struct GetVersionReturn {
        pub major_version: u8,
        pub minor_version: u8,
        pub patch_version: u8,
    }
    ///Container type for all return fields from the `hotShotCommitments` function with signature `hotShotCommitments(uint256)` and selector `0xdb13b60a`
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
    pub struct HotShotCommitmentsReturn {
        pub block_height: u64,
        pub block_comm_root: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `lagOverEscapeHatchThreshold` function with signature `lagOverEscapeHatchThreshold(uint256,uint256)` and selector `0xe0303301`
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
    pub struct LagOverEscapeHatchThresholdReturn(pub bool);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `permissionedProver` function with signature `permissionedProver()` and selector `0x313df7b1`
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
    pub struct PermissionedProverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `permissionedProverEnabled` function with signature `permissionedProverEnabled()` and selector `0xbd32519a`
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
    pub struct PermissionedProverEnabledReturn(pub bool);
    ///Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
    ///Container type for all return fields from the `stateUpdateBlockNumbers` function with signature `stateUpdateBlockNumbers(uint256)` and selector `0xa51e6fea`
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
    pub struct StateUpdateBlockNumbersReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `states` function with signature `states(uint32)` and selector `0x7f17baad`
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
    pub struct StatesReturn {
        pub view_num: u64,
        pub block_height: u64,
        pub block_comm_root: ::ethers::core::types::U256,
        pub fee_ledger_comm: ::ethers::core::types::U256,
        pub stake_table_bls_key_comm: ::ethers::core::types::U256,
        pub stake_table_schnorr_key_comm: ::ethers::core::types::U256,
        pub stake_table_amount_comm: ::ethers::core::types::U256,
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `votingStakeTableCommitment` function with signature `votingStakeTableCommitment()` and selector `0x76b6b7cb`
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
    pub struct VotingStakeTableCommitmentReturn(pub [u8; 32]);
    ///Container type for all return fields from the `votingThreshold` function with signature `votingThreshold()` and selector `0x62827733`
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
    pub struct VotingThresholdReturn(pub ::ethers::core::types::U256);
}
