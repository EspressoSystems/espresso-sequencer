pub use light_client::*;
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
pub mod light_client {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
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
    pub static LIGHTCLIENT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R4\x80\x15b\0\0\x15W`\0\x80\xFD[Pb\0\0 b\0\0&V[b\0\0\xDAV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15b\0\0wW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14b\0\0\xD7W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80QaN\x98b\0\x01\x04`\09`\0\x81\x81a\x13\x01\x01R\x81\x81a\x13*\x01Ra\x14\x96\x01RaN\x98`\0\xF3\xFE`\x80`@R`\x046\x10a\x01RW`\x005`\xE0\x1C\x80c\x01?\xA5\xFC\x14a\x01WW\x80c\r\x8En,\x14a\x01yW\x80c1=\xF7\xB1\x14a\x01\xABW\x80c8+!Z\x14a\x01\xD8W\x80c@\x999\xB7\x14a\x01\xFCW\x80cHG\xAE]\x14a\x02\x1CW\x80cO\x1E\xF2\x86\x14a\x02\x9EW\x80cR\xD1\x90-\x14a\x02\xB1W\x80cTd`\x85\x14a\x02\xC6W\x80cb\x82w3\x14a\x02\xDBW\x80ci\xCCj\x04\x14a\x02\xF1W\x80cpS\xFCQ\x14a\x03\x06W\x80cqP\x18\xA6\x14a\x03\x1BW\x80cvg\x18\x08\x14a\x030W\x80cv\xB6\xB7\xCB\x14a\x03dW\x80c\x7F\x17\xBA\xAD\x14a\x03zW\x80c\x82\xD0\x7F\xF3\x14a\x04-W\x80c\x85\x84\xD2?\x14a\x04BW\x80c\x8D\xA5\xCB[\x14a\x04\x86W\x80c\xA2D\xD5\x96\x14a\x04\x9BW\x80c\xA5\x1Eo\xEA\x14a\x04\xBBW\x80c\xAA\x92'2\x14a\x04\xDBW\x80c\xAD<\xB1\xCC\x14a\x04\xFBW\x80c\xBD2Q\x9A\x14a\x059W\x80c\xCAo\xE8U\x14a\x05jW\x80c\xDB\x13\xB6\n\x14a\x05\x80W\x80c\xE003\x01\x14a\x05\xBFW\x80c\xF0h T\x14a\x05\xDFW\x80c\xF2\xFD\xE3\x8B\x14a\x06\x11W[`\0\x80\xFD[4\x80\x15a\x01cW`\0\x80\xFD[Pa\x01wa\x01r6`\x04aF\xFAV[a\x061V[\0[4\x80\x15a\x01\x85W`\0\x80\xFD[P`@\x80Q`\x01\x81R`\0` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xB7W`\0\x80\xFD[P`\x06Ta\x01\xCB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qa\x01\xA2\x91\x90aG\x15V[4\x80\x15a\x01\xE4W`\0\x80\xFD[Pa\x01\xEE`\x03T\x81V[`@Q\x90\x81R` \x01a\x01\xA2V[4\x80\x15a\x02\x08W`\0\x80\xFD[Pa\x01wa\x02\x176`\x04aH\x9AV[a\x06\xF1V[4\x80\x15a\x02(W`\0\x80\xFD[Pa\x021a\n\x1CV[`@Qa\x01\xA2\x91\x90`\0a\x01\0\x82\x01\x90P`\x01\x80`@\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01R\x92\x91PPV[a\x01wa\x02\xAC6`\x04aJTV[a\n\xAEV[4\x80\x15a\x02\xBDW`\0\x80\xFD[Pa\x01\xEEa\n\xCDV[4\x80\x15a\x02\xD2W`\0\x80\xFD[P`\x08Ta\x01\xEEV[4\x80\x15a\x02\xE7W`\0\x80\xFD[Pa\x01\xEE`\x02T\x81V[4\x80\x15a\x02\xFDW`\0\x80\xFD[Pa\x01wa\n\xEAV[4\x80\x15a\x03\x12W`\0\x80\xFD[P`\x07Ta\x01\xEEV[4\x80\x15a\x03'W`\0\x80\xFD[Pa\x01wa\x0BZV[4\x80\x15a\x03<W`\0\x80\xFD[P`\0Ta\x03W\x90`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Qa\x01\xA2\x91\x90aJ\xF9V[4\x80\x15a\x03pW`\0\x80\xFD[Pa\x01\xEE`\x01T\x81V[4\x80\x15a\x03\x86W`\0\x80\xFD[Pa\x03\xE7a\x03\x956`\x04aK!V[`\x05` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T\x95\x85\x01T`\x06\x90\x95\x01T`\x01`\x01`@\x1B\x03\x80\x86\x16\x97`\x01`@\x1B\x90\x96\x04\x16\x95\x93\x94\x92\x93\x91\x92\x91\x90\x88V[`@\x80Q`\x01`\x01`@\x1B\x03\x99\x8A\x16\x81R\x98\x90\x97\x16` \x89\x01R\x95\x87\x01\x94\x90\x94R``\x86\x01\x92\x90\x92R`\x80\x85\x01R`\xA0\x84\x01R`\xC0\x83\x01R`\xE0\x82\x01Ra\x01\0\x01a\x01\xA2V[4\x80\x15a\x049W`\0\x80\xFD[Pa\x021a\x0BlV[4\x80\x15a\x04NW`\0\x80\xFD[Pa\x04ba\x04]6`\x04aK<V[a\x0B\xFCV[`@\x80Q\x82Q`\x01`\x01`@\x1B\x03\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x01\xA2V[4\x80\x15a\x04\x92W`\0\x80\xFD[Pa\x01\xCBa\rVV[4\x80\x15a\x04\xA7W`\0\x80\xFD[Pa\x01wa\x04\xB66`\x04aKUV[a\rqV[4\x80\x15a\x04\xC7W`\0\x80\xFD[Pa\x01\xEEa\x04\xD66`\x04aK<V[a\x0E\x9CV[4\x80\x15a\x04\xE7W`\0\x80\xFD[Pa\x01\xEEa\x04\xF66`\x04aK\x9CV[a\x0E\xBDV[4\x80\x15a\x05\x07W`\0\x80\xFD[Pa\x05,`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x01\xA2\x91\x90aK\xDDV[4\x80\x15a\x05EW`\0\x80\xFD[P`\x06Ta\x05Z\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xA2V[4\x80\x15a\x05vW`\0\x80\xFD[Pa\x01\xEE`\x04T\x81V[4\x80\x15a\x05\x8CW`\0\x80\xFD[Pa\x05\xA0a\x05\x9B6`\x04aK<V[a\x0F\x05V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xA2V[4\x80\x15a\x05\xCBW`\0\x80\xFD[Pa\x05Za\x05\xDA6`\x04aL\x10V[a\x0F=V[4\x80\x15a\x05\xEBW`\0\x80\xFD[P`\0Ta\x05\xFC\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xA2V[4\x80\x15a\x06\x1DW`\0\x80\xFD[Pa\x01wa\x06,6`\x04aF\xFAV[a\x10 V[a\x069a\x10^V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06`W`@Qc\xE6\xC4${`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x06\x8FW`@Qc\xA8c\xAE\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x91\x82\x90U`@Q\x7F\x80\x17\xBB\x88\x7F\xDF\x8F\xCAC\x14\xA9\xD4\x0Fns\xB3\xB8\x10\x02\xD6~\\\xFA\x85\xD8\x81s\xAFj\xA4`r\x92a\x06\xE6\x92\x16\x90aG\x15V[`@Q\x80\x91\x03\x90\xA1PV[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x80\x15a\x07\x15WP`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x07\\W`\x06T`\x01`\x01`\xA0\x1B\x03\x16a\x07CW`@Qc\x12\xE6\xD1\xE7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x01GL\x8F`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07da\x0BlV[Q\x82Q`\x01`\x01`@\x1B\x03\x91\x82\x16\x91\x16\x11\x15\x80a\x07\xA2WPa\x07\x84a\x0BlV[` \x01Q`\x01`\x01`@\x1B\x03\x16\x82` \x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15[\x15a\x07\xC0W`@Qc\x05\x1CF\xEF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\x07\xE5\x90c\xFF\xFF\xFF\xFF\x81\x16\x90`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aLHV[`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` R`@\x90\x91 T\x91\x92P\x90\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x82\x16\x14\x80\x15\x81a\x08;WP\x81`\x01`\x01`@\x1B\x03\x16\x84` \x01Q`\x01`\x01`@\x1B\x03\x16\x11[\x15a\x08dW\x81`@Qc\x03df\xBF`\xE3\x1B\x81R`\x04\x01a\x08[\x91\x90aJ\xF9V[`@Q\x80\x91\x03\x90\xFD[a\x08q\x84`@\x01Qa\x10\x90V[a\x08~\x84``\x01Qa\x10\x90V[a\x08\x8B\x84`\x80\x01Qa\x10\x90V[a\x08\x98\x84`\xA0\x01Qa\x10\x90V[a\x08\xA5\x84`\xC0\x01Qa\x10\x90V[\x80\x15a\x08\xB3Wa\x08\xB3a\x10\xECV[a\x08\xBD\x84\x84a\x127V[`\0\x80T`\x01`@\x1B\x90\x81\x90\x04c\xFF\xFF\xFF\xFF\x16\x82R`\x05` \x81\x81R`@\x80\x85 \x89Q\x81T\x8B\x85\x01\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x91\x83\x16\x90\x97\x02\x17\x82U\x8A\x83\x01\x80Q`\x01\x80\x85\x01\x91\x90\x91U``\x8D\x01Q`\x02\x80\x86\x01\x91\x90\x91U`\x80\x8E\x01Q`\x03\x86\x01U`\xA0\x8E\x01Q`\x04\x86\x01U`\xC0\x8E\x01Q\x97\x85\x01\x97\x90\x97U`\xE0\x8D\x01Q`\x06\x90\x94\x01\x93\x90\x93U`\x07\x80T\x80\x85\x01\x82U\x90\x89RC`\0\x80Q` aMl\x839\x81Q\x91R\x90\x91\x01U\x83Q\x80\x85\x01\x85R\x87Q\x83\x16\x81R\x81Q\x81\x87\x01\x90\x81R`\x08\x80T\x95\x86\x01\x81U\x90\x99RQ`\0\x80Q` aM\xEC\x839\x81Q\x91R\x93\x90\x96\x02\x92\x83\x01\x80T`\x01`\x01`@\x1B\x03\x19\x16\x96\x83\x16\x96\x90\x96\x17\x90\x95U\x95Q`\0\x80Q` aN,\x839\x81Q\x91R\x90\x91\x01U\x92Q\x88Q\x92Q\x93Q\x93\x84R\x84\x16\x93\x91\x90\x91\x16\x91\x7F\xA0Jw9$PZA\x85d67%\xF5h2\xF5w.k\x8D\r\xBDn\xFC\xE7$\xDF\xE8\x03\xDA\xE6\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPV[a\n$aD\xB4V[P`\0\x80T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81R`\x05` \x81\x81R`@\x92\x83\x90 \x83Qa\x01\0\x81\x01\x85R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x90\x91\x04\x16\x92\x81\x01\x92\x90\x92R`\x01\x81\x01T\x93\x82\x01\x93\x90\x93R`\x02\x83\x01T``\x82\x01R`\x03\x83\x01T`\x80\x82\x01R`\x04\x83\x01T`\xA0\x82\x01R\x90\x82\x01T`\xC0\x82\x01R`\x06\x90\x91\x01T`\xE0\x82\x01R\x90V[a\n\xB6a\x12\xF6V[a\n\xBF\x82a\x13\x9BV[a\n\xC9\x82\x82a\x13\xD2V[PPV[`\0a\n\xD7a\x14\x8BV[P`\0\x80Q` aM\xAC\x839\x81Q\x91R\x90V[a\n\xF2a\x10^V[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0B?W`\x06\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x90U`@Q\x7F\x9A_W\xDE\x85m\xD6h\xC5M\xD9^\\U\xDF\x93C!q\xCB\xCAI\xA8wmV \xEAY\xC0$P\x90`\0\x90\xA1V[`@Qc\xA8c\xAE\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\x0Bba\x10^V[a\x0BX`\0a\x14\xD4V[a\x0BtaD\xB4V[P`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` \x81\x81R`@\x93\x84\x90 \x84Qa\x01\0\x81\x01\x86R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R\x95\x90\x04\x90\x94\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x83\x01T``\x83\x01R`\x03\x83\x01T`\x80\x83\x01R`\x04\x83\x01T`\xA0\x83\x01R\x82\x01T`\xC0\x82\x01R`\x06\x90\x91\x01T`\xE0\x82\x01R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x08\x80T\x90a\x0C `\x01\x83aLkV[\x81T\x81\x10a\x0C0Wa\x0C0aL~V[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x83\x10a\x0CiW`@Qc\x18V\xA4\x99`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x0C\xFEW\x83`\x08\x82\x81T\x81\x10a\x0C\x88Wa\x0C\x88aL~V[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0C\xF6W`\x08\x81\x81T\x81\x10a\x0C\xBBWa\x0C\xBBaL~V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80T`\x01`\x01`@\x1B\x03\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`\x01\x01a\x0ClV[P`\x08a\r\x0C`\x01\x83aLkV[\x81T\x81\x10a\r\x1CWa\r\x1CaL~V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80T`\x01`\x01`@\x1B\x03\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80a\raa\x150V[T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[`\0a\r{a\x15TV[\x80T\x90\x91P`\xFF`\x01`@\x1B\x82\x04\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15a\r\xA2WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\r\xBEWP0;\x15[\x90P\x81\x15\x80\x15a\r\xCCWP\x80\x15[\x15a\r\xEAW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84T`\x01`\x01`@\x1B\x03\x19\x16`\x01\x17\x85U\x83\x15a\x0E\x13W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x0E\x1C\x86a\x15xV[a\x0E$a\x15\x89V[`\0\x80T`\x01` \x1B`\x01``\x1B\x03\x19\x16`\x01`@\x1B\x17\x90Ua\x0EG\x88\x88a\x15\x91V[\x83\x15a\x0E\x92W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90a\x0E\x89\x90`\x01\x90aJ\xF9V[`@Q\x80\x91\x03\x90\xA1[PPPPPPPPV[`\x07\x81\x81T\x81\x10a\x0E\xACW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\x80\x80\x82\x01Q`\xA0\x83\x01Q`\xC0\x84\x01Q`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x08\x81\x81T\x81\x10a\x0F\x15W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01\x80T`\x01\x90\x91\x01T`\x01`\x01`@\x1B\x03\x90\x91\x16\x91P\x82V[`\x07T`\0\x90C\x84\x11\x80a\x0FQWP`\x03\x81\x10[\x15a\x0FoW`@Qc\xB0\xB48w`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80a\x0F~`\x01\x85aLkV[\x90P[\x81a\x0F\xE9W\x86`\x07\x82\x81T\x81\x10a\x0F\x9AWa\x0F\x9AaL~V[\x90`\0R` `\0 \x01T\x11a\x0F\xCFW`\x01\x91P`\x07\x81\x81T\x81\x10a\x0F\xC1Wa\x0F\xC1aL~V[\x90`\0R` `\0 \x01T\x92P[`\x02\x81\x10a\x0F\xE9W\x80a\x0F\xE1\x81aL\x94V[\x91PPa\x0F\x81V[\x81a\x10\x07W`@Qc\xB0\xB48w`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85a\x10\x12\x84\x89aLkV[\x11\x94PPPPP[\x92\x91PPV[a\x10(a\x10^V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10RW`\0`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\x04\x01a\x08[\x91\x90aG\x15V[a\x10[\x81a\x14\xD4V[PV[3a\x10ga\rVV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0BXW3`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R`\x04\x01a\x08[\x91\x90aG\x15V[`\0\x80Q` aM\xCC\x839\x81Q\x91R\x81\x10\x80a\n\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x10\x9B\x8C\x8DM\x0E\x88\x1A[\x9D\x98[\x1AY\x08\x1C\xD8\xD8[\x18\\\x88\x19\x9AY[\x19`*\x1B`D\x82\x01R`d\x01a\x08[V[`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` \x81\x81R`@\x80\x85 \x81Qa\x01\0\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R\x96\x90\x04\x90\x95\x16\x92\x85\x01\x92\x90\x92R`\x01\x82\x01T\x90\x84\x01R`\x02\x81\x01T``\x84\x01R`\x03\x81\x01T`\x80\x84\x01R`\x04\x81\x01T`\xA0\x84\x01R\x90\x81\x01T`\xC0\x83\x01R`\x06\x01T`\xE0\x82\x01Ra\x11t\x90a\x0E\xBDV[`\x03\x80T`\x01\x90\x81U\x90\x82\x90U`\x04\x80T`\x02U`\0\x80T`\x01`@\x1B\x81\x04c\xFF\xFF\xFF\xFF\x16\x82R`\x05` R`@\x82 `\x06\x01T\x90\x92U\x92\x93P\x90\x91\x90`\x0C\x90a\x11\xCF\x90\x84\x90`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aL\xABV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x7F\xDB5X%\x9E\x03\x9D~P\xE8\x16\xB9\xDC\xCE0\xFB\x11M\x8A\x9C\x86\xEC\xA5\xAB\x14\xB6\x01\x94\xD6\x94]?`\0`\x0C\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16`@Qa\x06\xE6\x91\x90aJ\xF9V[`\0a\x12Aa\x18\xB4V[\x90Pa\x12KaE\x0BV[`\x02T\x81R\x83Q`\x01`\x01`@\x1B\x03\x90\x81\x16` \x80\x84\x01\x91\x90\x91R\x80\x86\x01Q\x90\x91\x16`@\x80\x84\x01\x91\x90\x91R\x80\x86\x01Q``\x80\x85\x01\x91\x90\x91R\x86\x01Q`\x80\x84\x01R`\0\x80T`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x80\x82R`\x05\x80\x85R\x92\x82 `\x03\x81\x01T`\xA0\x87\x01R`\x04\x81\x01T`\xC0\x87\x01R\x91R\x91\x81\x90R\x01T`\xE0\x82\x01Ra\x12\xD3\x82\x82\x85a\x1E\xE4V[a\x12\xF0W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x13}WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x13q`\0\x80Q` aM\xAC\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0BXW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13\xA3a\x10^V[\x7F\xF7\x87!\"n\xFE\x9A\x1B\xB6x\x18\x9A\x16\xD1UI(\xB9\xF2\x19.,\xB9>\xED\xA8;y\xFA@\0}\x81`@Qa\x06\xE6\x91\x90aG\x15V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x14,WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x14)\x91\x81\x01\x90aL\xCBV[`\x01[a\x14KW\x81`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x04\x01a\x08[\x91\x90aG\x15V[`\0\x80Q` aM\xAC\x839\x81Q\x91R\x81\x14a\x14|W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08[V[a\x14\x86\x83\x83a\x1FpV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0BXW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\xDEa\x150V[\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x93\x94P\x91\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPPV[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90V[a\x15\x80a\x1F\xC6V[a\x10[\x81a\x1F\xEBV[a\x0BXa\x1F\xC6V[\x81Q`\x01`\x01`@\x1B\x03\x16\x15\x15\x80a\x15\xB5WP` \x82\x01Q`\x01`\x01`@\x1B\x03\x16\x15\x15[\x80a\x15\xC2WP`\x80\x82\x01Q\x15[\x80a\x15\xCFWP`\xA0\x82\x01Q\x15[\x80a\x15\xDCWP`\xC0\x82\x01Q\x15[\x80a\x15\xE9WP`\xE0\x82\x01Q\x15[\x80a\x15\xF8WPc\xFF\xFF\xFF\xFF\x81\x16\x15[\x15a\x16\x16W`@QcP\xDD\x03\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x05`\0\x80`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x08a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01U``\x82\x01Q\x81`\x02\x01U`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01U`\xE0\x82\x01Q\x81`\x06\x01U\x90PP\x81`\x05`\0\x80`\x08\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x08a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01U``\x82\x01Q\x81`\x02\x01U`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01U`\xE0\x82\x01Q\x81`\x06\x01U\x90PP`\0\x80`\x0Ca\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x80`\0\x80a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0a\x17\xFC\x83a\x0E\xBDV[`\x01\x81\x81U`\xE0\x85\x01Q`\x02\x81\x81U`\x03\x93\x90\x93U`\x04U`\x07\x80T\x80\x83\x01\x82U`\0\x91\x82RC`\0\x80Q` aMl\x839\x81Q\x91R\x90\x91\x01U`@\x80Q\x80\x82\x01\x82R` \x80\x89\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x83R\x92\x90\x98\x01Q\x97\x81\x01\x97\x88R`\x08\x80T\x94\x85\x01\x81U\x90\x92R\x90Q`\0\x80Q` aM\xEC\x839\x81Q\x91R\x92\x90\x93\x02\x91\x82\x01\x80T`\x01`\x01`@\x1B\x03\x19\x16\x93\x90\x91\x16\x92\x90\x92\x17\x90\x91U\x92Q`\0\x80Q` aN,\x839\x81Q\x91R\x90\x93\x01\x92\x90\x92UPPV[a\x18\xBCaE*V[b\x10\0\0\x81R`\x08` \x82\x01R\x7F \xC9@13\xDF\xDE\x9A\x9D8-\xF7o\xB0R5qd\x87%\xAB\xC0\xA7\xC1(0\xBBi\x0E\xC8;3`@\x82\x01QR\x7F\x03\xA0\xA9\xAC\xC3\xE3\x81Z~\xD6\xCB\x13y\xF7\xD1W\xE641dr\x93v9*i:\xCB\xD3\xEC(<` `@\x83\x01Q\x01R\x7F(f\xC1\x8A\xD1\xDF\x10\xEF\x13T,\xCEbP\xCE\x02\xCB*kr\xAE\0\xA9\x85.'\x11\x87\xE9\xE4\xE0\xDB``\x82\x01QR\x7F!\xBE#*B$jVc\xEB\xF4\x83G\x0C\xCAfo\xFE\x9DO\x0Ec\xB9)\xC5\x96\xA7e\x87\x14\xE9p` ``\x83\x01Q\x01R\x7F\x07\xD7xs\xB9\x86\0t\x11\x8Eu\x80\x8CyF\x8B\x83\xC8\xEDd\xBA\x14\xDB\\\xB5\xAF\xA8\xE54\xDE{\x99`\x80\x82\x01QR\x7F\x0B\xE0\xF4H\x83\x90\x80\x13-G\xDE\x17\xDE\0\x99\xB4\xCDt\xAE\x1Ekq\xCD\xDA\x06\xCD\xEB\xB8h\xA5\x0Cm` `\x80\x83\x01Q\x01R\x7F\x13\xBDE\xA0#I\x1E\xAD\xEAD\xCC?$\xCF\xBD\x17\x96\xEA\xDE\x9C\x0E9\xEE\x81\xD9\xF6>\xA0\xA5\x80f%`\xA0\x82\x01QR\x7F\x18\xF9\\\xDD\xA4,\xE1\x1D\x9D\x10\xA3\xB35\xAC\xC2\x14\xE3\x80|W\x8CSY@]\x81\x0C \x8D\xF6\0\x93` `\xA0\x83\x01Q\x01R\x7F\tp\xD9xv4a\xF0\x9E\x9E\xC64T\x074\x978nM(/\xED\xC2\xAC[\x96|\xB9\xFD?\xA8\xA9`\xC0\x82\x01QR\x7F(\xC2!\x7F{\xAC\xF6\xF8\xB2\xB8\xEEJ\x90\xFC\xF8\xB5\xBC\xA0B\x05\xEA\x84\xE8\xE1\xEBT\xB8]\xD4\x1B\xDE(` `\xC0\x83\x01Q\x01R\x7F\x02\xFE=\x02\x98\x8D\xB7\x188\0R\x97\n\xBAF\xA3)m\xF5\xF2\x9Bsk\xA1\xF2\xC4\xCC\xFF\xC8\xB5\x96\x93`\xE0\x82\x01QR\x7F ,>9\x0C\xEE|\\\x85%\xDA#)\xA1\x9FI6\xF6\xF7\x1C\xA9}\xDElo\xA3+8-Z\xCC\x03` `\xE0\x83\x01Q\x01R\x7F#\xAC\x10\xAEl\xA5\xCA\xCE\xE8tK\xB99\xAA\xA859\tT\xB9\x1A\xE6h\xA2\xC8\xD0\xED\xDAU\x8A\x89\xE7a\x01\0\x82\x01QR\x7F\x1C\x8C+\x85l\xDA\xDE%k\xA3#\x7F9\xAF\xD5\xE1p\xA9S \x12\xF7\xAE\xCA\xE4\x9DE\x9B)\xF6\xF6\xAD` a\x01\0\x83\x01Q\x01R\x7F\x16\xEC\x03\xD2`\xBDz\xC1\xC5\x0F\xFAcV]Rt\xB4X,\xEE\xA5/\xF4\x0B\x81\xCD\xFE\x8FDO\x01\xE4a\x01 \x82\x01QR\x7F)9!Rr0\x97\xE0q\x13\xC3\xD7xm$^\xC4\x0C0\x92\x80\x15\xCDP\xB5f\x8AON\xA1p1` a\x01 \x83\x01Q\x01R\x7F,\xDB\xFB:@S\xC8H\x9B\x0C\x94\xE7C8\xAC\x19\x11\x8D\xF7\xA0k\xC5k\x1E\xB4\xD0\xE0\xDCN\xAErHa\x01@\x82\x01QR\x7F\x07\xFE\xA1'\xDA\xE9C\xB8\xDC\x14\x8F\x14\x08\xD4\x0C\xFFF\\\x9CG!\x946i\xB1\xE4\xFDZ9\xDBp6` a\x01@\x83\x01Q\x01R\x7F\x03\x14U\xA7\x9A.\x0C\xE7\x8Al\xB55&\xEC\x04\xAC\x19qj\x86\xB0\x8A\x93\xDFH\xD1x\xF8\xB7~V\x19a\x01`\x82\x01QR\x7F\x11\x86#\xE6\xBC\x13n\xE6\xD3\xF9\x90|\xD4\xAD\x04\xA9A\x8E\xA0;\xA9\x9A\xD7S\"|\xDF\xEEY\x8E\x84\x15` a\x01`\x83\x01Q\x01R\x7F\x08a\xD1\x99wa\xA8R\"j\xAC{\xA9q{\xF6\xAEVE\x10\x99\xBEwL\xDF\x02\xEF5*X\xCB\xC8a\x01\x80\x82\x01QR\x7F\x08\x05\xE3\x92\xBC\xBC\x12\xE4\nr'xc-s\xFE\x98\x1EK\xC6\xFAm\x11x\xB7\n\xF7\xBE\x1C\xB9\xA3\xA3` a\x01\x80\x83\x01Q\x01R\x7F\x10\x1D\x1E9x\xCB\x9F\x1E0=A1D\xEB\xE6v\x82\xC9\xEB\x0C\xFE\x11$)Y\xAA`)\xD7\x8C\xDB\xBCa\x01\xA0\x82\x01QR\x7F\x08\x9E\xB9\xC7'\xE6\xCB\x07\x08+\xC3\xE6\xF4\x0C\xF0OC\x9F\xE4\x80\0`+XGt\xDA\xD7\xEF\xC6`|` a\x01\xA0\x83\x01Q\x01R\x7F-H\x9F$\x93&:\xA8s\xBC\xD9O!\xEF\xB4[\xF2W\xA6\x1D\x81\xC0\xC9\\2\x97\x91e\x06e;@a\x01\xC0\x82\x01QR\x7F\x18\xE4]bz\xAD\xD4\xDF'\x94\xEC\xD9\x90\x9F\xAC\x1Au?\x0Co\xA8\xA9\xC6eJzX\xB0\x91/\xFF\xD5` a\x01\xC0\x83\x01Q\x01R\x7F\x0EC\xE3\xA4\xB1<\xB48\xE2\xAD\x92F\x14&\x1A\xD0$\x02\x14\xFA\x1C\x83\xFC\xDAj\x0B\xF7y\xEB9\xFF\xC5a\x01\xE0\x82\x01QR\x7F\x0E\xAB\xA9\xF4)\xC5\xF6\xFC1\x03\xD4\xCC@V\xC5\0\xFFBB]\x8Ede\xC5\xB8\xE1E!\x9F\x9C\\\xD3` a\x01\xE0\x83\x01Q\x01R\x7F)\xAE5\x1D\t\xDC\xF4\x1C\n\x80\xAB\x059785\x8B\xAA\xB3~o\xBCFK;\xB12X\x99J\x1F\xA4a\x02\0\x82\x01QR\x7F+{\xC7F\x08\xD7\xEC}\xAD\xD0Y}j@\x10\xD8\xBF\xC2\xB3\x19\0(\x19\x01\xCE\xDCB\xBD\xBB\x0F\xB8\xFC` a\x02\0\x83\x01Q\x01R\x7F\x06h\x02\xC7\xCE\xB9\xE9\x13\xD4\xF6T3\xA2\x06a\xE0\x97\xAC\xAC\x1A\xFF\xEC\xBBSJT\xF7j)x\"&a\x02 \x82\x01QR\x7F'\xEC\x80\xE8\x11\xE66\xF34\x82g\x92<\x8Ed\x1B\xD9\x8A~7\xC5!fp\xCB\xFF\x14\xAE2?\x9E\x0E` a\x02 \x83\x01Q\x01R\x7F\x12`M\x1F\x87\xC5\x83\xF6\xC9q\x0Cs\xEA\xF5\x90\xAF\x9D\x07\xAAt=\x13\x81\xD0\xE9\xDF\xF0\xEA\xB2aB9a\x02@\x82\x01QR\x7F\x15\x88W\x9El3x\xEA2\xCBd\x12\x05\xEFv*c\xCD5:\x0B\xD6p9E(\xAD \x81\xEE\x8D\xD4` a\x02@\x83\x01Q\x01R\x7F$}e&\x1D:J\xB0B\xBA\x93s1\xF6\xD0\xC0\xC5\xEB\x9E\xA7\x87S\xA9 \x84\xDB\x1Ai9\xE1\x9E\x82a\x02`\x82\x01QR\x7F,\xE6\xCCfJ2\x14{\xFEj\x0C\x94\xA9[\xF0Ify@\\\xCA\xE0\x16H\xCDN\xC0!\x14Q \xD5` a\x02`\x83\x01Q\x01R\x7F\xB0\x83\x88\x93\xEC\x1F#~\x8B\x072;\x07DY\x9FN\x97\xB5\x98\xB3\xB5\x89\xBC\xC2\xBC7\xB8\xD5\xC4\x18\x01a\x02\x80\x82\x01R\x7F\xC1\x83\x93\xC0\xFA0\xFEN\x8B\x03\x8E5z\xD8Q\xEA\xE8\xDE\x91\x07XN\xFF\xE7\xC7\xF1\xF6Q\xB2\x01\x0E&a\x02\xA0\x82\x01R\x90V[`\0a\x1E\xEF\x82a\x1F\xF3V[a\x1F\0\x83`\0[` \x02\x01Qa\x10\x90V[a\x1F\x0B\x83`\x01a\x1E\xF6V[a\x1F\x16\x83`\x02a\x1E\xF6V[a\x1F!\x83`\x03a\x1E\xF6V[a\x1F,\x83`\x04a\x1E\xF6V[a\x1F7\x83`\x05a\x1E\xF6V[a\x1FB\x83`\x06a\x1E\xF6V[a\x1FM\x83`\x07a\x1E\xF6V[`\0a\x1FZ\x85\x85\x85a!+V[\x90Pa\x1Fe\x81a\"PV[\x91PP[\x93\x92PPPV[a\x1Fy\x82a&\xA3V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\x1F\xBEWa\x14\x86\x82\x82a&\xFFV[a\n\xC9a'uV[a\x1F\xCEa'\x94V[a\x0BXW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10(a\x1F\xC6V[\x80Qa\x1F\xFE\x90a'\xAEV[a \x0B\x81` \x01Qa'\xAEV[a \x18\x81`@\x01Qa'\xAEV[a %\x81``\x01Qa'\xAEV[a 2\x81`\x80\x01Qa'\xAEV[a ?\x81`\xA0\x01Qa'\xAEV[a L\x81`\xC0\x01Qa'\xAEV[a Y\x81`\xE0\x01Qa'\xAEV[a g\x81a\x01\0\x01Qa'\xAEV[a u\x81a\x01 \x01Qa'\xAEV[a \x83\x81a\x01@\x01Qa'\xAEV[a \x91\x81a\x01`\x01Qa'\xAEV[a \x9F\x81a\x01\x80\x01Qa'\xAEV[a \xAD\x81a\x01\xA0\x01Qa\x10\x90V[a \xBB\x81a\x01\xC0\x01Qa\x10\x90V[a \xC9\x81a\x01\xE0\x01Qa\x10\x90V[a \xD7\x81a\x02\0\x01Qa\x10\x90V[a \xE5\x81a\x02 \x01Qa\x10\x90V[a \xF3\x81a\x02@\x01Qa\x10\x90V[a!\x01\x81a\x02`\x01Qa\x10\x90V[a!\x0F\x81a\x02\x80\x01Qa\x10\x90V[a!\x1D\x81a\x02\xA0\x01Qa\x10\x90V[a\x10[\x81a\x02\xC0\x01Qa\x10\x90V[a!3aF=V[`\0a!@\x85\x85\x85a(@V[\x90P`\0a!Q\x86`\0\x01Qa-\xB7V[\x90P`\0a!d\x82\x84`\xA0\x01Q\x88a/=V[`@\x80Q`\x1E\x80\x82Ra\x03\xE0\x82\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01a\x03\xC0\x806\x837PP`@\x80Q`\x1E\x80\x82Ra\x03\xE0\x82\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x82\x01[a!\xAFaF\x8DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a!\xA7W\x90PP\x90P`\0a!\xD4\x8A\x85\x8A\x89\x87\x87a/\x9DV[`\xA0\x87\x01Q`@\x87\x01Q\x91\x92P\x90`\0\x80Q` aM\xCC\x839\x81Q\x91R`\0\x81\x83\x85\t`@\x80Qa\x01\0\x81\x01\x82R`\xE0\x9C\x8D\x01Q\x81R` \x81\x01\x96\x90\x96R\x85\x01RPPP``\x81\x01\x91\x90\x91R`\x80\x81\x01\x92\x90\x92R`\xA0\x82\x01Ra\x01`\x86\x01Q`\xC0\x82\x01Ra\x01\x80\x90\x95\x01Q\x92\x85\x01\x92\x90\x92RP\x91\x94\x93PPPPV[`\0`\0\x80Q` aM\xCC\x839\x81Q\x91Ra\"iaF\x8DV[a\"qaF\x8DV[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R`\0\x92` \x83\x01\x90\x806\x837PP`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x82\x01[a\"\xB3aF\x8DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\"\xABW\x90PP\x90P`\0`\x01\x90P\x80\x83`\0\x81Q\x81\x10a\"\xE2Wa\"\xE2aL~V[` \x02` \x01\x01\x81\x81RPP\x87`\xC0\x01Q\x82`\0\x81Q\x81\x10a#\x06Wa#\x06aL~V[` \x02` \x01\x01\x81\x90RP\x87`\0\x01Q\x83`\x01\x81Q\x81\x10a#)Wa#)aL~V[` \x02` \x01\x01\x81\x81RPP\x87`\xE0\x01Q\x82`\x01\x81Q\x81\x10a#MWa#MaL~V[` \x02` \x01\x01\x81\x90RPa#b\x82\x84a/\xD2V[`\x80\x89\x01QQ\x90\x95P``\x93P\x83\x92P\x90P`\0a#\x81\x82`\x02aL\xE4V[a#\x8C\x90`\x01aL\xE4V[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xA6Wa#\xA6aG)V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#\xCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x93P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xEAWa#\xEAaG)V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$#W\x81` \x01[a$\x10aF\x8DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a$\x08W\x90P[P\x92PPP`\0\x80[\x88`\x80\x01QQ\x81\x10\x15a$\xC5W\x88`\x80\x01Q\x81\x81Q\x81\x10a$OWa$OaL~V[` \x02` \x01\x01Q\x84\x83\x81Q\x81\x10a$iWa$iaL~V[` \x02` \x01\x01\x81\x81RPP\x88`\xA0\x01Q\x81\x81Q\x81\x10a$\x8BWa$\x8BaL~V[` \x02` \x01\x01Q\x83\x83\x81Q\x81\x10a$\xA5Wa$\xA5aL~V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra$\xBB`\x01\x83aL\xE4V[\x91P`\x01\x01a$,V[P\x87` \x01Q\x83\x82\x81Q\x81\x10a$\xDDWa$\xDDaL~V[` \x02` \x01\x01\x81\x81RPP\x87`\xC0\x01Q\x82\x82\x81Q\x81\x10a%\0Wa%\0aL~V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra%\x16`\x01\x82aL\xE4V[\x88Q`@\x8A\x01Q\x91\x92P\x90`\0\x88\x82\x84\t\x90P\x80\x86\x85\x81Q\x81\x10a%<Wa%<aL~V[` \x02` \x01\x01\x81\x81RPPPPP\x87`\xE0\x01Q\x82\x82\x81Q\x81\x10a%bWa%baL~V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra%x`\x01\x82aL\xE4V[\x90Pa%\x87\x88``\x01Qa0\xC0V[\x83\x82\x81Q\x81\x10a%\x99Wa%\x99aL~V[` \x02` \x01\x01\x81\x81RPPa%\xADa0\xF2V[\x82\x82\x81Q\x81\x10a%\xBFWa%\xBFaL~V[` \x02` \x01\x01\x81\x90RPa%\xDCa%\xD7\x83\x85a/\xD2V[a1\x13V[\x93PPPP`\0`@Q\x80`\x80\x01`@R\x80\x7F\x01\x18\xC4\xD5\xB87\xBC\xC2\xBC\x89\xB5\xB3\x98\xB5\x97N\x9FYD\x07;2\x07\x8B~#\x1F\xEC\x93\x88\x83\xB0\x81R` \x01\x7F&\x0E\x01\xB2Q\xF6\xF1\xC7\xE7\xFFNX\x07\x91\xDE\xE8\xEAQ\xD8z5\x8E\x03\x8BN\xFE0\xFA\xC0\x93\x83\xC1\x81R` \x01\x7F\"\xFE\xBD\xA3\xC0\xC0c*VG[B\x14\xE5a^\x11\xE6\xDD?\x96\xE6\xCE\xA2\x85J\x87\xD4\xDA\xCC^U\x81R` \x01\x7F\x04\xFCci\xF7\x11\x0F\xE3\xD2QV\xC1\xBB\x9Ar\x85\x9C\xF2\xA0FA\xF9\x9B\xA4\xEEA<\x80\xDAj_\xE4\x81RP\x90Pa&\x99\x83\x82\x84a&\x94a1\x82V[a2SV[\x96\x95PPPPPPV[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a&\xD0W\x80`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x04\x01a\x08[\x91\x90aG\x15V[`\0\x80Q` aM\xAC\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa'\x1C\x91\x90aL\xF7V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a'WW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a'\\V[``\x91P[P\x91P\x91Pa'l\x85\x83\x83a36V[\x95\x94PPPPPV[4\x15a\x0BXW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a'\x9Ea\x15TV[T`\x01`@\x1B\x90\x04`\xFF\x16\x91\x90PV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` aML\x839\x81Q\x91R\x91\x15\x90\x15\x16\x15a'\xD6WPPPV[\x82Q` \x84\x01Q\x82`\x03\x84\x85\x85\x86\t\x85\t\x08\x83\x82\x83\t\x14\x83\x82\x10\x84\x84\x10\x16\x16\x93PPP\x81a\x14\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv\x10\x9B\x8C\x8DM\x0E\x88\x1A[\x9D\x98[\x1AY\x08\x11\xCCH\x1C\x1B\xDA[\x9D`J\x1B`D\x82\x01R`d\x01a\x08[V[a(\x88`@Q\x80a\x01\0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0`\0\x80Q` aM\xCC\x839\x81Q\x91R\x90P`@Q` \x81\x01`\0\x81R`\xFE`\xE0\x1B\x81R\x86Q`\xC0\x1B`\x04\x82\x01R` \x87\x01Q`\xC0\x1B`\x0C\x82\x01Ra\x02\x80\x87\x01Q` \x82\x01Ra\x02\xA0\x87\x01Q`@\x82\x01R`\x01``\x82\x01R`\0\x80Q` aNl\x839\x81Q\x91R`\x80\x82\x01R`\0\x80Q` aN\x0C\x839\x81Q\x91R`\xA0\x82\x01R`\0\x80Q` aNL\x839\x81Q\x91R`\xC0\x82\x01R`\0\x80Q` aM\x8C\x839\x81Q\x91R`\xE0\x82\x01R`\xE0\x87\x01Q\x80Qa\x01\0\x83\x01R` \x81\x01Qa\x01 \x83\x01RPa\x01\0\x87\x01Q\x80Qa\x01@\x83\x01R` \x81\x01Qa\x01`\x83\x01RPa\x01 \x87\x01Q\x80Qa\x01\x80\x83\x01R` \x81\x01Qa\x01\xA0\x83\x01RPa\x01@\x87\x01Q\x80Qa\x01\xC0\x83\x01R` \x81\x01Qa\x01\xE0\x83\x01RPa\x01`\x87\x01Q\x80Qa\x02\0\x83\x01R` \x81\x01Qa\x02 \x83\x01RPa\x01\x80\x87\x01Q\x80Qa\x02@\x83\x01R` \x81\x01Qa\x02`\x83\x01RPa\x01\xE0\x87\x01Q\x80Qa\x02\x80\x83\x01R` \x81\x01Qa\x02\xA0\x83\x01RPa\x02\0\x87\x01Q\x80Qa\x02\xC0\x83\x01R` \x81\x01Qa\x02\xE0\x83\x01RPa\x02 \x87\x01Q\x80Qa\x03\0\x83\x01R` \x81\x01Qa\x03 \x83\x01RPa\x02@\x87\x01Q\x80Qa\x03@\x83\x01R` \x81\x01Qa\x03`\x83\x01RPa\x01\xA0\x87\x01Q\x80Qa\x03\x80\x83\x01R` \x81\x01Qa\x03\xA0\x83\x01RPa\x01\xC0\x87\x01Q\x80Qa\x03\xC0\x83\x01R` \x81\x01Qa\x03\xE0\x83\x01RPa\x02`\x87\x01Q\x80Qa\x04\0\x83\x01R` \x81\x01Qa\x04 \x83\x01RP`@\x87\x01Q\x80Qa\x04@\x83\x01R` \x81\x01Qa\x04`\x83\x01RP``\x87\x01Q\x80Qa\x04\x80\x83\x01R` \x81\x01Qa\x04\xA0\x83\x01RP`\x80\x87\x01Q\x80Qa\x04\xC0\x83\x01R` \x81\x01Qa\x04\xE0\x83\x01RP`\xA0\x87\x01Q\x80Qa\x05\0\x83\x01R` \x81\x01Qa\x05 \x83\x01RP`\xC0\x87\x01Q\x80Qa\x05@\x83\x01R` \x81\x01Qa\x05`\x83\x01RP\x85Qa\x05\x80\x82\x01R` \x86\x01Qa\x05\xA0\x82\x01R`@\x86\x01Qa\x05\xC0\x82\x01R``\x86\x01Qa\x05\xE0\x82\x01R`\x80\x86\x01Qa\x06\0\x82\x01R`\xA0\x86\x01Qa\x06 \x82\x01R`\xC0\x86\x01Qa\x06@\x82\x01R`\xE0\x86\x01Qa\x06`\x82\x01R\x84Q\x80Qa\x06\x80\x83\x01R` \x81\x01Qa\x06\xA0\x83\x01RP` \x85\x01Q\x80Qa\x06\xC0\x83\x01R` \x81\x01Qa\x06\xE0\x83\x01RP`@\x85\x01Q\x80Qa\x07\0\x83\x01R` \x81\x01Qa\x07 \x83\x01RP``\x85\x01Q\x80Qa\x07@\x83\x01R` \x81\x01Qa\x07`\x83\x01RP`\x80\x85\x01Q\x80Qa\x07\x80\x83\x01R` \x81\x01Qa\x07\xA0\x83\x01RP`\0\x82Ra\x07\xE0\x82 a\x07\xC0\x82\x01Ra\x07\xC0\x81\x01\x91P` \x82\x01\x90P\x82\x82Q\x06``\x85\x01R` \x82 \x81R\x80\x91P` \x82\x01\x90P\x82\x82Q\x06`\x80\x85\x01R`\xA0\x85\x01Q\x80Q\x82R` \x81\x01Q` \x83\x01RP``\x82 `@\x82\x01\x92P\x80\x83R` \x83\x01\x91P\x83\x81\x06\x85R\x83\x81\x82\t\x84\x82\x82\t\x91P\x80` \x87\x01RP\x80`@\x86\x01RP`\xC0\x85\x01Q\x80Q\x82R` \x81\x01Q` \x83\x01RP`\xE0\x85\x01Q\x80Q`@\x83\x01R` \x81\x01Q``\x83\x01RPa\x01\0\x85\x01Q\x80Q`\x80\x83\x01R` \x81\x01Q`\xA0\x83\x01RPa\x01 \x85\x01Q\x80Q`\xC0\x83\x01R` \x81\x01Q`\xE0\x83\x01RPa\x01@\x85\x01Q\x80Qa\x01\0\x83\x01R` \x81\x01Qa\x01 \x83\x01RPa\x01`\x82 a\x01@\x82\x01Ra\x01@\x81\x01\x91P` \x82\x01\x90P\x82\x82Q\x06`\xA0\x85\x01Ra\x01\xA0\x85\x01Q\x81Ra\x01\xC0\x85\x01Q` \x82\x01Ra\x01\xE0\x85\x01Q`@\x82\x01Ra\x02\0\x85\x01Q``\x82\x01Ra\x02 \x85\x01Q`\x80\x82\x01Ra\x02@\x85\x01Q`\xA0\x82\x01Ra\x02`\x85\x01Q`\xC0\x82\x01Ra\x02\x80\x85\x01Q`\xE0\x82\x01Ra\x02\xA0\x85\x01Qa\x01\0\x82\x01Ra\x02\xC0\x85\x01Qa\x01 \x82\x01Ra\x01`\x82 a\x01@\x82\x01Ra\x01@\x81\x01\x91P` \x82\x01\x90P\x82\x82Q\x06`\xC0\x85\x01Ra\x01`\x85\x01Q\x80Q\x82R` \x81\x01Q` \x83\x01RPa\x01\x80\x85\x01Q\x80Q`@\x83\x01R` \x81\x01Q``\x83\x01RPP`\xA0\x81 \x82\x81\x06`\xE0\x85\x01RPPP\x93\x92PPPV[a-\xDB`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81b\x01\0\0\x03a.GW`@Q\x80``\x01`@R\x80`\x10\x81R` \x01\x7F0d\x1E\x0E\x92\xBE\xBE\xF8\x18&\x8Df;\xCA\xD6\xDB\xCF\xD6\xC0\x14\x91p\xF6\xD7\xD3P\xB1\xB1\xFAl\x10\x01\x81R` \x01~\xEE\xB2\xCBY\x81\xEDEd\x9A\xBE\xBD\xE0\x81\xDC\xFF\x16\xC8`\x1D\xE44~}\xD1b\x8B\xA2\xDA\xACC\xB7\x81RP\x90P\x91\x90PV[\x81b\x10\0\0\x03a.\xB4W`@Q\x80``\x01`@R\x80`\x14\x81R` \x01\x7F0dKl\x9CJr\x16\x9EM\xAA1}%\xF0E\x12\xAE\x15\xC5;4\xE8\xF5\xAC\xD8\xE1U\xD0\xA6\xC1\x01\x81R` \x01\x7F&\x12]\xA1\n\x0E\xD0c'P\x8A\xBA\x06\xD1\xE3\x03\xACaf2\xDB\xED4\x9FSB-\xA9S3xW\x81RP\x90P\x91\x90PV[\x81` \x03a/\x1FW`@Q\x80``\x01`@R\x80`\x05\x81R` \x01\x7F.\xE1+\xFFJ(\x13(j\x8D\xC3\x88\xCDuM\x9A>\xF2I\x065\xEB\xA5\x0C\xB9\xC2\xE5\xE7P\x80\0\x01\x81R` \x01\x7F\t\xC52\xC60k\x93\xD2\x96x \rG\xC0\xB2\xA9\x9C\x18\xD5\x1B\x83\x8E\xEB\x1D>\xEDLS;\xB5\x12\xD0\x81RP\x90P\x91\x90PV[`@Qc\xE2\xEF\t\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a/a`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a/k\x84\x84a3\x89V[\x80\x82Ra/{\x90\x85\x90\x85\x90a3\xDDV[` \x82\x01R\x80Qa/\x91\x90\x85\x90\x84\x90\x86\x90a4QV[`@\x82\x01R\x93\x92PPPV[`\0\x80a/\xAB\x85\x87\x89a6\x02V[\x90Pa/\xBB\x88\x86\x89\x89\x88\x88a6\xF2V[a/\xC6\x81\x87\x86a9\xF8V[\x98\x97PPPPPPPPV[a/\xDAaF\x8DV[\x82Q\x82Q\x14a0+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FMSM error: length does not match`D\x82\x01R`d\x01a\x08[V[a0i\x83`\0\x81Q\x81\x10a0AWa0AaL~V[` \x02` \x01\x01Q\x83`\0\x81Q\x81\x10a0\\Wa0\\aL~V[` \x02` \x01\x01Qa:HV[\x90P`\x01[\x82Q\x81\x10\x15a0\xB9Wa0\xAF\x82a0\xAA\x86\x84\x81Q\x81\x10a0\x90Wa0\x90aL~V[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a0\\Wa0\\aL~V[a:\xDCV[\x91P`\x01\x01a0nV[P\x92\x91PPV[`\0a0\xDA`\0\x80Q` aM\xCC\x839\x81Q\x91R\x83aM)V[a\x10\x1A\x90`\0\x80Q` aM\xCC\x839\x81Q\x91RaLkV[a0\xFAaF\x8DV[P`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x02` \x82\x01R\x90V[a1\x1BaF\x8DV[\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a1/WP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aML\x839\x81Q\x91R\x84` \x01Qa1b\x91\x90aM)V[a1z\x90`\0\x80Q` aML\x839\x81Q\x91RaLkV[\x90R\x92\x91PPV[a1\xAD`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R` \x87\x01Q`@\x82\x01R\x86Q``\x82\x01R``\x87\x01Q`\x80\x82\x01R`@\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R` \x85\x01Qa\x01\0\x82\x01R\x84Qa\x01 \x82\x01R``\x85\x01Qa\x01@\x82\x01R`@\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a3(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R{Bn254: Pairing check failed!` \x1B`D\x82\x01R`d\x01a\x08[V[P\x15\x15\x90P[\x94\x93PPPPV[``\x82a3KWa3F\x82a;wV[a\x1FiV[\x81Q\x15\x80\x15a3bWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a3\x82W\x83`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x04\x01a\x08[\x91\x90aG\x15V[P\x80a\x1FiV[\x81Q`\0\x90`\0\x80Q` aM\xCC\x839\x81Q\x91R\x90\x83\x80\x15a3\xCDW\x84\x93P`\0[\x82\x81\x10\x15a3\xC1W\x83\x85\x86\t\x94P`\x01\x01a3\xABV[P`\x01\x84\x03\x93Pa3\xD4V[`\x01\x83\x03\x93P[PPP\x92\x91PPV[`\0\x82`\x01\x03a3\xEFWP`\x01a\x1FiV[\x81`\0\x03a3\xFFWP`\0a\x1FiV[` \x84\x01Q`\0\x80Q` aM\xCC\x839\x81Q\x91R\x90`\0\x90\x82\x81\x86\t\x90P\x85\x80\x15a4/W`\x01\x87\x03\x92Pa46V[`\x01\x84\x03\x92P[Pa4@\x82a;\xA0V[\x91P\x82\x82\x82\t\x97\x96PPPPPPPV[`\0`\0\x80Q` aM\xCC\x839\x81Q\x91R\x82\x82\x03a4\xCAW`\x01`\0[`\x08\x81\x10\x15a4\xBEW\x81\x86\x03a4\x9FW\x86\x81`\x08\x81\x10a4\x90Wa4\x90aL~V[` \x02\x01Q\x93PPPPa3.V[\x82\x80a4\xADWa4\xADaM\x13V[\x88`@\x01Q\x83\t\x91P`\x01\x01a4nV[P`\0\x92PPPa3.V[a4\xD2aE\x0BV[`@\x87\x01Q`\x01\x80\x83R\x83\x82\x82\t\x90P\x80` \x84\x01R\x83\x82\x82\t\x90P\x80`@\x84\x01R\x83\x82\x82\t\x90P\x80``\x84\x01R\x83\x82\x82\t\x90P\x80`\x80\x84\x01R\x83\x82\x82\t\x90P\x80`\xA0\x84\x01R\x83\x82\x82\t\x90P\x80`\xC0\x84\x01R\x83\x82\x82\t`\xE0\x84\x01RPa56aE\x0BV[`\x01`\xE0\x82\x81\x01\x82\x81R\x91\x90\x85\x01\x90\x80[`\x08\x81\x10\x15a5tW` \x84\x03\x93P\x87\x88\x8C\x85Q\x8B\x03\x08\x83\t\x80\x85R`\x1F\x19\x90\x93\x01\x92\x91P`\x01\x01a5GV[PPPP`\0\x80`\0\x90P`\x01\x83\x8B\x87`\0[`\x08\x81\x10\x15a5\xC7W\x8A\x82Q\x8C\x85Q\x8E\x88Q\x8A\t\t\t\x8B\x81\x88\x08\x96PP\x8A\x8B\x8F\x84Q\x8E\x03\x08\x86\t\x94P` \x93\x84\x01\x93\x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a5\x87V[PPPP\x80\x92PP`\0a5\xDA\x83a;\xA0V[\x90P` \x8C\x01Q\x87\x81\x8B\t\x98PP\x86\x81\x89\t\x97P\x86\x82\x89\t\x9C\x9BPPPPPPPPPPPPV[`\0\x80`\0\x80Q` aM\xCC\x839\x81Q\x91R\x90P`\0\x83` \x01Q\x90P`\0\x84`@\x01Q\x90P`\0`\x01\x90P``\x88\x01Q`\x80\x89\x01Qa\x01\xA0\x89\x01Qa\x02@\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x01\xC0\x89\x01Qa\x02`\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x01\xE0\x89\x01Qa\x02\x80\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x02\0\x89\x01Qa\x02\xA0\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x02 \x89\x01Q\x91Pa\x02\xC0\x89\x01Q\x86\x87\x82\x89\x85\x87\x08\t\x85\t\x93PPPP\x87Q` \x89\x01Q\x85\x86\x86\x83\t\x87\x03\x85\x08\x96PP\x84\x85\x83\x83\t\x86\x03\x87\x08\x99\x98PPPPPPPPPV[a7\0\x86\x86\x86\x86\x85\x87a<FV[`\xC0\x85\x01Q\x82Q`\0\x80Q` aM\xCC\x839\x81Q\x91R\x91\x90\x81\x90\x81\x90\x86\x90`\x14\x90\x81\x10a7/Wa7/aL~V[` \x02` \x01\x01\x81\x81RPP\x85`\0\x01Q\x84`\x14\x81Q\x81\x10a7SWa7SaL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x15\x81Q\x81\x10a7xWa7xaL~V[` \x02` \x01\x01\x81\x81RPP\x85` \x01Q\x84`\x15\x81Q\x81\x10a7\x9CWa7\x9CaL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x16\x81Q\x81\x10a7\xC1Wa7\xC1aL~V[` \x02` \x01\x01\x81\x81RPP\x85`@\x01Q\x84`\x16\x81Q\x81\x10a7\xE5Wa7\xE5aL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x17\x81Q\x81\x10a8\nWa8\naL~V[` \x02` \x01\x01\x81\x81RPP\x85``\x01Q\x84`\x17\x81Q\x81\x10a8.Wa8.aL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x18\x81Q\x81\x10a8SWa8SaL~V[` \x02` \x01\x01\x81\x81RPP\x85`\x80\x01Q\x84`\x18\x81Q\x81\x10a8wWa8waL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x19\x81Q\x81\x10a8\x9CWa8\x9CaL~V[` \x02` \x01\x01\x81\x81RPP\x88`@\x01Q\x84`\x19\x81Q\x81\x10a8\xC0Wa8\xC0aL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x1A\x81Q\x81\x10a8\xE5Wa8\xE5aL~V[` \x02` \x01\x01\x81\x81RPP\x88``\x01Q\x84`\x1A\x81Q\x81\x10a9\tWa9\taL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x1B\x81Q\x81\x10a9.Wa9.aL~V[` \x02` \x01\x01\x81\x81RPP\x88`\x80\x01Q\x84`\x1B\x81Q\x81\x10a9RWa9RaL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x1C\x81Q\x81\x10a9wWa9waL~V[` \x02` \x01\x01\x81\x81RPP\x88`\xA0\x01Q\x84`\x1C\x81Q\x81\x10a9\x9BWa9\x9BaL~V[` \x02` \x01\x01\x81\x90RP\x87`\xE0\x01Q\x85`\x1D\x81Q\x81\x10a9\xBEWa9\xBEaL~V[` \x02` \x01\x01\x81\x81RPP\x85`\xA0\x01Q\x84`\x1D\x81Q\x81\x10a9\xE2Wa9\xE2aL~V[` \x02` \x01\x01\x81\x90RPPPPPPPPPPV[`\0\x80Q` aM\xCC\x839\x81Q\x91R\x83\x81\x03\x90`\0[`\n\x81\x10\x15a:?W` `\x15\x82\x01\x02\x84\x01Q` \x82\x02a\x01\xA0\x01\x86\x01Q\x83\x84\x82\x84\t\x86\x08\x94PPP`\x01\x01a:\x0EV[PP\x93\x92PPPV[a:PaF\x8DV[a:XaF\xA7V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x81\x01\x83\x90R`\0``\x83`\x80\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a:\x8AW`\0\x80\xFD[P\x80a:\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01RxBn254: scalar mul failed!`8\x1B`D\x82\x01R`d\x01a\x08[V[PP\x92\x91PPV[a:\xE4aF\x8DV[a:\xECaF\xC5V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x83\x01R\x83\x01Q``\x80\x83\x01\x91\x90\x91R`\0\x90\x83`\xC0\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a;)W`\0\x80\xFD[P\x80a:\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FBn254: group addition failed!\0\0\0`D\x82\x01R`d\x01a\x08[V[\x80Q\x15a;\x87W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0`\0\x80Q` aM\xCC\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01R`\x02\x82\x03`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x92PP`\0Q\x92P\x81a<?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FBn254: pow precompile failed!\0\0\0`D\x82\x01R`d\x01a\x08[V[PP\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80Q` aM\xCC\x839\x81Q\x91R\x90P\x80` \x8B\x01Q` \x8D\x01Q\t\x95P\x8AQ\x93P\x80`\xA0\x8C\x01Q``\x8D\x01Q\t\x92P\x80a\x01\xA0\x8A\x01Q\x84\x08\x91P\x80`\x80\x8C\x01Q\x83\x08\x91P\x80\x84\x83\t\x93P\x80`\0\x80Q` aNl\x839\x81Q\x91R\x84\t\x91P\x80a\x01\xC0\x8A\x01Q\x83\x08\x91P\x80`\x80\x8C\x01Q\x83\x08\x91P\x80\x84\x83\t\x93P\x80`\0\x80Q` aN\x0C\x839\x81Q\x91R\x84\t\x91P\x80a\x01\xE0\x8A\x01Q\x83\x08\x91P\x80`\x80\x8C\x01Q\x83\x08\x91P\x80\x84\x83\t\x93P\x80`\0\x80Q` aNL\x839\x81Q\x91R\x84\t\x91P\x80a\x02\0\x8A\x01Q\x83\x08\x91P\x80`\x80\x8C\x01Q\x83\x08\x91P\x80\x84\x83\t\x93P\x80`\0\x80Q` aM\x8C\x839\x81Q\x91R\x84\t\x91P\x80a\x02 \x8A\x01Q\x83\x08\x91P\x80`\x80\x8C\x01Q\x83\x08\x91P\x80\x84\x83\t\x93P\x80\x84\x87\x08\x95P\x88`\xA0\x01Q\x88`\0\x81Q\x81\x10a=uWa=uaL~V[` \x02` \x01\x01\x81\x90RP\x85\x87`\0\x81Q\x81\x10a=\x94Wa=\x94aL~V[` \x02` \x01\x01\x81\x81RPP\x80``\x8C\x01Q\x8CQ\t\x94P\x80a\x02\xC0\x8A\x01Q\x86\t\x94P\x80a\x02@\x8A\x01Q``\x8D\x01Q\t\x92P\x80a\x01\xA0\x8A\x01Q\x84\x08\x92P\x80`\x80\x8C\x01Q\x84\x08\x92P\x80\x83\x86\t\x94P\x80a\x02`\x8A\x01Q``\x8D\x01Q\t\x92P\x80a\x01\xC0\x8A\x01Q\x84\x08\x92P\x80`\x80\x8C\x01Q\x84\x08\x92P\x80\x83\x86\t\x94P\x80a\x02\x80\x8A\x01Q``\x8D\x01Q\t\x92P\x80a\x01\xE0\x8A\x01Q\x84\x08\x92P\x80`\x80\x8C\x01Q\x84\x08\x92P\x80\x83\x86\t\x94P\x80a\x02\xA0\x8A\x01Q``\x8D\x01Q\t\x92P\x80a\x02\0\x8A\x01Q\x84\x08\x92P\x80`\x80\x8C\x01Q\x84\x08\x92P\x80\x83\x86\t\x94P\x8B`\xC0\x01Q\x88`\x01\x81Q\x81\x10a>vWa>vaL~V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra>\x8B\x85\x82aLkV[\x87`\x01\x81Q\x81\x10a>\x9EWa>\x9EaL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x01\xA0\x01Q\x87`\x02\x81Q\x81\x10a>\xC3Wa>\xC3aL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x01\xC0\x01Q\x87`\x03\x81Q\x81\x10a>\xE8Wa>\xE8aL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x01\xE0\x01Q\x87`\x04\x81Q\x81\x10a?\rWa?\raL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x02\0\x01Q\x87`\x05\x81Q\x81\x10a?2Wa?2aL~V[` \x02` \x01\x01\x81\x81RPP\x8B`\xE0\x01Q\x88`\x02\x81Q\x81\x10a?VWa?VaL~V[` \x02` \x01\x01\x81\x90RP\x8Ba\x01\0\x01Q\x88`\x03\x81Q\x81\x10a?zWa?zaL~V[` \x02` \x01\x01\x81\x90RP\x8Ba\x01 \x01Q\x88`\x04\x81Q\x81\x10a?\x9EWa?\x9EaL~V[` \x02` \x01\x01\x81\x90RP\x8Ba\x01@\x01Q\x88`\x05\x81Q\x81\x10a?\xC2Wa?\xC2aL~V[` \x02` \x01\x01\x81\x90RP\x80a\x01\xC0\x8A\x01Qa\x01\xA0\x8B\x01Q\t\x92P\x82\x87`\x06\x81Q\x81\x10a?\xF1Wa?\xF1aL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x01`\x01Q\x88`\x06\x81Q\x81\x10a@\x16Wa@\x16aL~V[` \x02` \x01\x01\x81\x90RP\x80a\x02\0\x8A\x01Qa\x01\xE0\x8B\x01Q\t\x92P\x82\x87`\x07\x81Q\x81\x10a@EWa@EaL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x01\x80\x01Q\x88`\x07\x81Q\x81\x10a@jWa@jaL~V[` \x02` \x01\x01\x81\x90RPa\x01\xA0\x89\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92P\x82\x87`\x08\x81Q\x81\x10a@\xA3Wa@\xA3aL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x01\xE0\x01Q\x88`\x08\x81Q\x81\x10a@\xC8Wa@\xC8aL~V[` \x02` \x01\x01\x81\x90RPa\x01\xC0\x89\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92P\x82\x87`\t\x81Q\x81\x10aA\x01WaA\x01aL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x02\0\x01Q\x88`\t\x81Q\x81\x10aA&WaA&aL~V[` \x02` \x01\x01\x81\x90RPa\x01\xE0\x89\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92P\x82\x87`\n\x81Q\x81\x10aA_WaA_aL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x02 \x01Q\x88`\n\x81Q\x81\x10aA\x84WaA\x84aL~V[` \x02` \x01\x01\x81\x90RPa\x02\0\x89\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92P\x82\x87`\x0B\x81Q\x81\x10aA\xBDWaA\xBDaL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x02@\x01Q\x88`\x0B\x81Q\x81\x10aA\xE2WaA\xE2aL~V[` \x02` \x01\x01\x81\x90RP\x88a\x02 \x01Q\x81aA\xFE\x91\x90aLkV[\x87`\x0C\x81Q\x81\x10aB\x11WaB\x11aL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x01\xA0\x01Q\x88`\x0C\x81Q\x81\x10aB6WaB6aL~V[` \x02` \x01\x01\x81\x90RP`\x01\x87`\r\x81Q\x81\x10aBVWaBVaL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x01\xC0\x01Q\x88`\r\x81Q\x81\x10aB{WaB{aL~V[` \x02` \x01\x01\x81\x90RP\x80a\x01\xC0\x8A\x01Qa\x01\xA0\x8B\x01Q\t\x92P\x80a\x01\xE0\x8A\x01Q\x84\t\x92P\x80a\x02\0\x8A\x01Q\x84\t\x92P\x80a\x02 \x8A\x01Q\x84\t\x92P\x82\x87`\x0E\x81Q\x81\x10aB\xCBWaB\xCBaL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x02`\x01Q\x88`\x0E\x81Q\x81\x10aB\xF0WaB\xF0aL~V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x89QaC\x07\x90\x82aLkV[\x87`\x0F\x81Q\x81\x10aC\x1AWaC\x1AaL~V[` \x02` \x01\x01\x81\x81RPP\x88`\xC0\x01Q\x88`\x0F\x81Q\x81\x10aC>WaC>aL~V[` \x02` \x01\x01\x81\x90RP\x80`\x01\x8BQ\x08`\xA0\x8C\x01Q\x90\x93P\x81\x90\x80\t\x91P\x80\x82\x84\t\x92P\x80\x83` `\x10\x02\x89\x01Q\t\x91P\x81\x87`\x10\x81Q\x81\x10aC\x84WaC\x84aL~V[` \x02` \x01\x01\x81\x81RPP\x88`\xE0\x01Q\x88`\x10\x81Q\x81\x10aC\xA8WaC\xA8aL~V[` \x02` \x01\x01\x81\x90RP\x80\x83` `\x11\x02\x89\x01Q\t\x91P\x81\x87`\x11\x81Q\x81\x10aC\xD4WaC\xD4aL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x01\0\x01Q\x88`\x11\x81Q\x81\x10aC\xF9WaC\xF9aL~V[` \x02` \x01\x01\x81\x90RP\x80\x83` `\x12\x02\x89\x01Q\t\x91P\x81\x87`\x12\x81Q\x81\x10aD%WaD%aL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x01 \x01Q\x88`\x12\x81Q\x81\x10aDJWaDJaL~V[` \x02` \x01\x01\x81\x90RP\x80\x83` `\x13\x02\x89\x01Q\t\x91P\x81\x87`\x13\x81Q\x81\x10aDvWaDvaL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x01@\x01Q\x88`\x13\x81Q\x81\x10aD\x9BWaD\x9BaL~V[` \x02` \x01\x01\x81\x90RPPPPPPPPPPPPPV[`@Q\x80a\x01\0\x01`@R\x80`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80a\x01\0\x01`@R\x80`\x08\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x02\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01aELaF\x8DV[\x81R` \x01aEYaF\x8DV[\x81R` \x01aEfaF\x8DV[\x81R` \x01aEsaF\x8DV[\x81R` \x01aE\x80aF\x8DV[\x81R` \x01aE\x8DaF\x8DV[\x81R` \x01aE\x9AaF\x8DV[\x81R` \x01aE\xA7aF\x8DV[\x81R` \x01aE\xB4aF\x8DV[\x81R` \x01aE\xC1aF\x8DV[\x81R` \x01aE\xCEaF\x8DV[\x81R` \x01aE\xDBaF\x8DV[\x81R` \x01aE\xE8aF\x8DV[\x81R` \x01aE\xF5aF\x8DV[\x81R` \x01aF\x02aF\x8DV[\x81R` \x01aF\x0FaF\x8DV[\x81R` \x01aF\x1CaF\x8DV[\x81R` \x01aF)aF\x8DV[\x81R`\0` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80a\x01\0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81R` \x01``\x81R` \x01aF{aF\x8DV[\x81R` \x01aF\x88aF\x8DV[\x90R\x90V[`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a/8W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aG\x0CW`\0\x80\xFD[a\x1Fi\x82aF\xE3V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x02\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aGbWaGbaG)V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aG\x90WaG\x90aG)V[`@R\x91\x90PV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a/8W`\0\x80\xFD[`\0a\x01\0\x80\x83\x85\x03\x12\x15aG\xC3W`\0\x80\xFD[`@Q\x90\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x81\x83\x10\x17\x15aG\xE5WaG\xE5aG)V[\x81`@R\x80\x92PaG\xF5\x84aG\x98V[\x81RaH\x03` \x85\x01aG\x98V[` \x82\x01R`@\x84\x015`@\x82\x01R``\x84\x015``\x82\x01R`\x80\x84\x015`\x80\x82\x01R`\xA0\x84\x015`\xA0\x82\x01R`\xC0\x84\x015`\xC0\x82\x01R`\xE0\x84\x015`\xE0\x82\x01RPP\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15aH^W`\0\x80\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH\x80WaH\x80aG)V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x80\x82\x84\x03a\x05\x80\x81\x12\x15aH\xAFW`\0\x80\xFD[aH\xB9\x85\x85aG\xAFV[\x92Pa\x01\0a\x04\x80\x80`\xFF\x19\x84\x01\x12\x15aH\xD2W`\0\x80\xFD[aH\xDAaG?V[\x92PaH\xE8\x87\x83\x88\x01aHLV[\x83Ra\x01@aH\xF9\x88\x82\x89\x01aHLV[` \x85\x01Ra\x01\x80aI\r\x89\x82\x8A\x01aHLV[`@\x86\x01Ra\x01\xC0aI!\x8A\x82\x8B\x01aHLV[``\x87\x01Ra\x02\0aI5\x8B\x82\x8C\x01aHLV[`\x80\x88\x01Ra\x02@aII\x8C\x82\x8D\x01aHLV[`\xA0\x89\x01Ra\x02\x80aI]\x8D\x82\x8E\x01aHLV[`\xC0\x8A\x01Ra\x02\xC0aIq\x8E\x82\x8F\x01aHLV[`\xE0\x8B\x01RaI\x84\x8Ea\x03\0\x8F\x01aHLV[\x89\x8B\x01RaI\x96\x8Ea\x03@\x8F\x01aHLV[a\x01 \x8B\x01RaI\xAA\x8Ea\x03\x80\x8F\x01aHLV[\x87\x8B\x01RaI\xBC\x8Ea\x03\xC0\x8F\x01aHLV[a\x01`\x8B\x01RaI\xD0\x8Ea\x04\0\x8F\x01aHLV[\x86\x8B\x01Ra\x04@\x8D\x015a\x01\xA0\x8B\x01Ra\x04`\x8D\x015\x85\x8B\x01R\x87\x8D\x015a\x01\xE0\x8B\x01Ra\x04\xA0\x8D\x015\x84\x8B\x01Ra\x04\xC0\x8D\x015a\x02 \x8B\x01Ra\x04\xE0\x8D\x015\x83\x8B\x01Ra\x05\0\x8D\x015a\x02`\x8B\x01Ra\x05 \x8D\x015\x82\x8B\x01Ra\x05@\x8D\x015a\x02\xA0\x8B\x01Ra\x05`\x8D\x015\x81\x8B\x01RPPPPPPPPP\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aJgW`\0\x80\xFD[aJp\x83aF\xE3V[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aJ\x8DW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12aJ\xA1W`\0\x80\xFD[\x815\x81\x81\x11\x15aJ\xB3WaJ\xB3aG)V[aJ\xC5`\x1F\x82\x01`\x1F\x19\x16\x85\x01aGhV[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15aJ\xDBW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\x01`\x01`@\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a/8W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aK3W`\0\x80\xFD[a\x1Fi\x82aK\rV[`\0` \x82\x84\x03\x12\x15aKNW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0a\x01@\x84\x86\x03\x12\x15aKkW`\0\x80\xFD[aKu\x85\x85aG\xAFV[\x92PaK\x84a\x01\0\x85\x01aK\rV[\x91PaK\x93a\x01 \x85\x01aF\xE3V[\x90P\x92P\x92P\x92V[`\0a\x01\0\x82\x84\x03\x12\x15aK\xAFW`\0\x80\xFD[a\x1Fi\x83\x83aG\xAFV[`\0[\x83\x81\x10\x15aK\xD4W\x81\x81\x01Q\x83\x82\x01R` \x01aK\xBCV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01RaK\xFC\x81`@\x85\x01` \x87\x01aK\xB9V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aL#W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x80\x82\x16\x91\x90\x82\x81\x14a:\xD4Wa:\xD4aL2V[\x81\x81\x03\x81\x81\x11\x15a\x10\x1AWa\x10\x1AaL2V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81aL\xA3WaL\xA3aL2V[P`\0\x19\x01\x90V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a0\xB9Wa0\xB9aL2V[`\0` \x82\x84\x03\x12\x15aL\xDDW`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x10\x1AWa\x10\x1AaL2V[`\0\x82QaM\t\x81\x84` \x87\x01aK\xB9V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aMFWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88.+\x91Ea\x03i\x8A\xDFW\xB7\x99\x96\x9D\xEA\x1C\x8Fs\x9D\xA5\xD8\xD4\r\xD3\xEB\x92\"\xDB|\x81\xE8\x816\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE3\x1E\xE6x\xA0G\nu\xA6\xEA\xA8\xFE\x83p`I\x8B\xA8(\xA3p;1\x1D\x0Fw\xF0\x10BJ\xFE\xB0%\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE4 B\xA5\x87\xA9\x0C\x18{\n\x08|\x03\xE2\x9C\x96\x8B\x95\x0B\x1D\xB2m\\\x82\xD6f\x90Zh\x95y\x0C\n/\x8D\xD1\xF1\xA7X<B\xC4\xE1*D\xE1\x10@Ls\xCAl\x94\x81?\x85\x83]\xA4\xFB{\xB10\x1DJ\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static LIGHTCLIENT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01RW`\x005`\xE0\x1C\x80c\x01?\xA5\xFC\x14a\x01WW\x80c\r\x8En,\x14a\x01yW\x80c1=\xF7\xB1\x14a\x01\xABW\x80c8+!Z\x14a\x01\xD8W\x80c@\x999\xB7\x14a\x01\xFCW\x80cHG\xAE]\x14a\x02\x1CW\x80cO\x1E\xF2\x86\x14a\x02\x9EW\x80cR\xD1\x90-\x14a\x02\xB1W\x80cTd`\x85\x14a\x02\xC6W\x80cb\x82w3\x14a\x02\xDBW\x80ci\xCCj\x04\x14a\x02\xF1W\x80cpS\xFCQ\x14a\x03\x06W\x80cqP\x18\xA6\x14a\x03\x1BW\x80cvg\x18\x08\x14a\x030W\x80cv\xB6\xB7\xCB\x14a\x03dW\x80c\x7F\x17\xBA\xAD\x14a\x03zW\x80c\x82\xD0\x7F\xF3\x14a\x04-W\x80c\x85\x84\xD2?\x14a\x04BW\x80c\x8D\xA5\xCB[\x14a\x04\x86W\x80c\xA2D\xD5\x96\x14a\x04\x9BW\x80c\xA5\x1Eo\xEA\x14a\x04\xBBW\x80c\xAA\x92'2\x14a\x04\xDBW\x80c\xAD<\xB1\xCC\x14a\x04\xFBW\x80c\xBD2Q\x9A\x14a\x059W\x80c\xCAo\xE8U\x14a\x05jW\x80c\xDB\x13\xB6\n\x14a\x05\x80W\x80c\xE003\x01\x14a\x05\xBFW\x80c\xF0h T\x14a\x05\xDFW\x80c\xF2\xFD\xE3\x8B\x14a\x06\x11W[`\0\x80\xFD[4\x80\x15a\x01cW`\0\x80\xFD[Pa\x01wa\x01r6`\x04aF\xFAV[a\x061V[\0[4\x80\x15a\x01\x85W`\0\x80\xFD[P`@\x80Q`\x01\x81R`\0` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xB7W`\0\x80\xFD[P`\x06Ta\x01\xCB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qa\x01\xA2\x91\x90aG\x15V[4\x80\x15a\x01\xE4W`\0\x80\xFD[Pa\x01\xEE`\x03T\x81V[`@Q\x90\x81R` \x01a\x01\xA2V[4\x80\x15a\x02\x08W`\0\x80\xFD[Pa\x01wa\x02\x176`\x04aH\x9AV[a\x06\xF1V[4\x80\x15a\x02(W`\0\x80\xFD[Pa\x021a\n\x1CV[`@Qa\x01\xA2\x91\x90`\0a\x01\0\x82\x01\x90P`\x01\x80`@\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01R\x92\x91PPV[a\x01wa\x02\xAC6`\x04aJTV[a\n\xAEV[4\x80\x15a\x02\xBDW`\0\x80\xFD[Pa\x01\xEEa\n\xCDV[4\x80\x15a\x02\xD2W`\0\x80\xFD[P`\x08Ta\x01\xEEV[4\x80\x15a\x02\xE7W`\0\x80\xFD[Pa\x01\xEE`\x02T\x81V[4\x80\x15a\x02\xFDW`\0\x80\xFD[Pa\x01wa\n\xEAV[4\x80\x15a\x03\x12W`\0\x80\xFD[P`\x07Ta\x01\xEEV[4\x80\x15a\x03'W`\0\x80\xFD[Pa\x01wa\x0BZV[4\x80\x15a\x03<W`\0\x80\xFD[P`\0Ta\x03W\x90`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Qa\x01\xA2\x91\x90aJ\xF9V[4\x80\x15a\x03pW`\0\x80\xFD[Pa\x01\xEE`\x01T\x81V[4\x80\x15a\x03\x86W`\0\x80\xFD[Pa\x03\xE7a\x03\x956`\x04aK!V[`\x05` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T\x95\x85\x01T`\x06\x90\x95\x01T`\x01`\x01`@\x1B\x03\x80\x86\x16\x97`\x01`@\x1B\x90\x96\x04\x16\x95\x93\x94\x92\x93\x91\x92\x91\x90\x88V[`@\x80Q`\x01`\x01`@\x1B\x03\x99\x8A\x16\x81R\x98\x90\x97\x16` \x89\x01R\x95\x87\x01\x94\x90\x94R``\x86\x01\x92\x90\x92R`\x80\x85\x01R`\xA0\x84\x01R`\xC0\x83\x01R`\xE0\x82\x01Ra\x01\0\x01a\x01\xA2V[4\x80\x15a\x049W`\0\x80\xFD[Pa\x021a\x0BlV[4\x80\x15a\x04NW`\0\x80\xFD[Pa\x04ba\x04]6`\x04aK<V[a\x0B\xFCV[`@\x80Q\x82Q`\x01`\x01`@\x1B\x03\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x01\xA2V[4\x80\x15a\x04\x92W`\0\x80\xFD[Pa\x01\xCBa\rVV[4\x80\x15a\x04\xA7W`\0\x80\xFD[Pa\x01wa\x04\xB66`\x04aKUV[a\rqV[4\x80\x15a\x04\xC7W`\0\x80\xFD[Pa\x01\xEEa\x04\xD66`\x04aK<V[a\x0E\x9CV[4\x80\x15a\x04\xE7W`\0\x80\xFD[Pa\x01\xEEa\x04\xF66`\x04aK\x9CV[a\x0E\xBDV[4\x80\x15a\x05\x07W`\0\x80\xFD[Pa\x05,`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x01\xA2\x91\x90aK\xDDV[4\x80\x15a\x05EW`\0\x80\xFD[P`\x06Ta\x05Z\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xA2V[4\x80\x15a\x05vW`\0\x80\xFD[Pa\x01\xEE`\x04T\x81V[4\x80\x15a\x05\x8CW`\0\x80\xFD[Pa\x05\xA0a\x05\x9B6`\x04aK<V[a\x0F\x05V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xA2V[4\x80\x15a\x05\xCBW`\0\x80\xFD[Pa\x05Za\x05\xDA6`\x04aL\x10V[a\x0F=V[4\x80\x15a\x05\xEBW`\0\x80\xFD[P`\0Ta\x05\xFC\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xA2V[4\x80\x15a\x06\x1DW`\0\x80\xFD[Pa\x01wa\x06,6`\x04aF\xFAV[a\x10 V[a\x069a\x10^V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06`W`@Qc\xE6\xC4${`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x06\x8FW`@Qc\xA8c\xAE\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x91\x82\x90U`@Q\x7F\x80\x17\xBB\x88\x7F\xDF\x8F\xCAC\x14\xA9\xD4\x0Fns\xB3\xB8\x10\x02\xD6~\\\xFA\x85\xD8\x81s\xAFj\xA4`r\x92a\x06\xE6\x92\x16\x90aG\x15V[`@Q\x80\x91\x03\x90\xA1PV[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x80\x15a\x07\x15WP`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x07\\W`\x06T`\x01`\x01`\xA0\x1B\x03\x16a\x07CW`@Qc\x12\xE6\xD1\xE7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x01GL\x8F`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07da\x0BlV[Q\x82Q`\x01`\x01`@\x1B\x03\x91\x82\x16\x91\x16\x11\x15\x80a\x07\xA2WPa\x07\x84a\x0BlV[` \x01Q`\x01`\x01`@\x1B\x03\x16\x82` \x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15[\x15a\x07\xC0W`@Qc\x05\x1CF\xEF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\x07\xE5\x90c\xFF\xFF\xFF\xFF\x81\x16\x90`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aLHV[`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` R`@\x90\x91 T\x91\x92P\x90\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x82\x16\x14\x80\x15\x81a\x08;WP\x81`\x01`\x01`@\x1B\x03\x16\x84` \x01Q`\x01`\x01`@\x1B\x03\x16\x11[\x15a\x08dW\x81`@Qc\x03df\xBF`\xE3\x1B\x81R`\x04\x01a\x08[\x91\x90aJ\xF9V[`@Q\x80\x91\x03\x90\xFD[a\x08q\x84`@\x01Qa\x10\x90V[a\x08~\x84``\x01Qa\x10\x90V[a\x08\x8B\x84`\x80\x01Qa\x10\x90V[a\x08\x98\x84`\xA0\x01Qa\x10\x90V[a\x08\xA5\x84`\xC0\x01Qa\x10\x90V[\x80\x15a\x08\xB3Wa\x08\xB3a\x10\xECV[a\x08\xBD\x84\x84a\x127V[`\0\x80T`\x01`@\x1B\x90\x81\x90\x04c\xFF\xFF\xFF\xFF\x16\x82R`\x05` \x81\x81R`@\x80\x85 \x89Q\x81T\x8B\x85\x01\x80Q`\x01`\x01`@\x1B\x03\x93\x84\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x91\x83\x16\x90\x97\x02\x17\x82U\x8A\x83\x01\x80Q`\x01\x80\x85\x01\x91\x90\x91U``\x8D\x01Q`\x02\x80\x86\x01\x91\x90\x91U`\x80\x8E\x01Q`\x03\x86\x01U`\xA0\x8E\x01Q`\x04\x86\x01U`\xC0\x8E\x01Q\x97\x85\x01\x97\x90\x97U`\xE0\x8D\x01Q`\x06\x90\x94\x01\x93\x90\x93U`\x07\x80T\x80\x85\x01\x82U\x90\x89RC`\0\x80Q` aMl\x839\x81Q\x91R\x90\x91\x01U\x83Q\x80\x85\x01\x85R\x87Q\x83\x16\x81R\x81Q\x81\x87\x01\x90\x81R`\x08\x80T\x95\x86\x01\x81U\x90\x99RQ`\0\x80Q` aM\xEC\x839\x81Q\x91R\x93\x90\x96\x02\x92\x83\x01\x80T`\x01`\x01`@\x1B\x03\x19\x16\x96\x83\x16\x96\x90\x96\x17\x90\x95U\x95Q`\0\x80Q` aN,\x839\x81Q\x91R\x90\x91\x01U\x92Q\x88Q\x92Q\x93Q\x93\x84R\x84\x16\x93\x91\x90\x91\x16\x91\x7F\xA0Jw9$PZA\x85d67%\xF5h2\xF5w.k\x8D\r\xBDn\xFC\xE7$\xDF\xE8\x03\xDA\xE6\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPV[a\n$aD\xB4V[P`\0\x80T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81R`\x05` \x81\x81R`@\x92\x83\x90 \x83Qa\x01\0\x81\x01\x85R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x90\x91\x04\x16\x92\x81\x01\x92\x90\x92R`\x01\x81\x01T\x93\x82\x01\x93\x90\x93R`\x02\x83\x01T``\x82\x01R`\x03\x83\x01T`\x80\x82\x01R`\x04\x83\x01T`\xA0\x82\x01R\x90\x82\x01T`\xC0\x82\x01R`\x06\x90\x91\x01T`\xE0\x82\x01R\x90V[a\n\xB6a\x12\xF6V[a\n\xBF\x82a\x13\x9BV[a\n\xC9\x82\x82a\x13\xD2V[PPV[`\0a\n\xD7a\x14\x8BV[P`\0\x80Q` aM\xAC\x839\x81Q\x91R\x90V[a\n\xF2a\x10^V[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0B?W`\x06\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x90U`@Q\x7F\x9A_W\xDE\x85m\xD6h\xC5M\xD9^\\U\xDF\x93C!q\xCB\xCAI\xA8wmV \xEAY\xC0$P\x90`\0\x90\xA1V[`@Qc\xA8c\xAE\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\x0Bba\x10^V[a\x0BX`\0a\x14\xD4V[a\x0BtaD\xB4V[P`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` \x81\x81R`@\x93\x84\x90 \x84Qa\x01\0\x81\x01\x86R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R\x95\x90\x04\x90\x94\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x83\x01T``\x83\x01R`\x03\x83\x01T`\x80\x83\x01R`\x04\x83\x01T`\xA0\x83\x01R\x82\x01T`\xC0\x82\x01R`\x06\x90\x91\x01T`\xE0\x82\x01R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x08\x80T\x90a\x0C `\x01\x83aLkV[\x81T\x81\x10a\x0C0Wa\x0C0aL~V[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x83\x10a\x0CiW`@Qc\x18V\xA4\x99`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x0C\xFEW\x83`\x08\x82\x81T\x81\x10a\x0C\x88Wa\x0C\x88aL~V[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0C\xF6W`\x08\x81\x81T\x81\x10a\x0C\xBBWa\x0C\xBBaL~V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80T`\x01`\x01`@\x1B\x03\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`\x01\x01a\x0ClV[P`\x08a\r\x0C`\x01\x83aLkV[\x81T\x81\x10a\r\x1CWa\r\x1CaL~V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80T`\x01`\x01`@\x1B\x03\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80a\raa\x150V[T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[`\0a\r{a\x15TV[\x80T\x90\x91P`\xFF`\x01`@\x1B\x82\x04\x16\x15\x90`\x01`\x01`@\x1B\x03\x16`\0\x81\x15\x80\x15a\r\xA2WP\x82[\x90P`\0\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\r\xBEWP0;\x15[\x90P\x81\x15\x80\x15a\r\xCCWP\x80\x15[\x15a\r\xEAW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84T`\x01`\x01`@\x1B\x03\x19\x16`\x01\x17\x85U\x83\x15a\x0E\x13W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x0E\x1C\x86a\x15xV[a\x0E$a\x15\x89V[`\0\x80T`\x01` \x1B`\x01``\x1B\x03\x19\x16`\x01`@\x1B\x17\x90Ua\x0EG\x88\x88a\x15\x91V[\x83\x15a\x0E\x92W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90a\x0E\x89\x90`\x01\x90aJ\xF9V[`@Q\x80\x91\x03\x90\xA1[PPPPPPPPV[`\x07\x81\x81T\x81\x10a\x0E\xACW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\x80\x80\x82\x01Q`\xA0\x83\x01Q`\xC0\x84\x01Q`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x08\x81\x81T\x81\x10a\x0F\x15W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01\x80T`\x01\x90\x91\x01T`\x01`\x01`@\x1B\x03\x90\x91\x16\x91P\x82V[`\x07T`\0\x90C\x84\x11\x80a\x0FQWP`\x03\x81\x10[\x15a\x0FoW`@Qc\xB0\xB48w`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80a\x0F~`\x01\x85aLkV[\x90P[\x81a\x0F\xE9W\x86`\x07\x82\x81T\x81\x10a\x0F\x9AWa\x0F\x9AaL~V[\x90`\0R` `\0 \x01T\x11a\x0F\xCFW`\x01\x91P`\x07\x81\x81T\x81\x10a\x0F\xC1Wa\x0F\xC1aL~V[\x90`\0R` `\0 \x01T\x92P[`\x02\x81\x10a\x0F\xE9W\x80a\x0F\xE1\x81aL\x94V[\x91PPa\x0F\x81V[\x81a\x10\x07W`@Qc\xB0\xB48w`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85a\x10\x12\x84\x89aLkV[\x11\x94PPPPP[\x92\x91PPV[a\x10(a\x10^V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10RW`\0`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\x04\x01a\x08[\x91\x90aG\x15V[a\x10[\x81a\x14\xD4V[PV[3a\x10ga\rVV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0BXW3`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R`\x04\x01a\x08[\x91\x90aG\x15V[`\0\x80Q` aM\xCC\x839\x81Q\x91R\x81\x10\x80a\n\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01Rz\x10\x9B\x8C\x8DM\x0E\x88\x1A[\x9D\x98[\x1AY\x08\x1C\xD8\xD8[\x18\\\x88\x19\x9AY[\x19`*\x1B`D\x82\x01R`d\x01a\x08[V[`\0\x80Tc\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x16\x82R`\x05` \x81\x81R`@\x80\x85 \x81Qa\x01\0\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R\x96\x90\x04\x90\x95\x16\x92\x85\x01\x92\x90\x92R`\x01\x82\x01T\x90\x84\x01R`\x02\x81\x01T``\x84\x01R`\x03\x81\x01T`\x80\x84\x01R`\x04\x81\x01T`\xA0\x84\x01R\x90\x81\x01T`\xC0\x83\x01R`\x06\x01T`\xE0\x82\x01Ra\x11t\x90a\x0E\xBDV[`\x03\x80T`\x01\x90\x81U\x90\x82\x90U`\x04\x80T`\x02U`\0\x80T`\x01`@\x1B\x81\x04c\xFF\xFF\xFF\xFF\x16\x82R`\x05` R`@\x82 `\x06\x01T\x90\x92U\x92\x93P\x90\x91\x90`\x0C\x90a\x11\xCF\x90\x84\x90`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aL\xABV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x7F\xDB5X%\x9E\x03\x9D~P\xE8\x16\xB9\xDC\xCE0\xFB\x11M\x8A\x9C\x86\xEC\xA5\xAB\x14\xB6\x01\x94\xD6\x94]?`\0`\x0C\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16`@Qa\x06\xE6\x91\x90aJ\xF9V[`\0a\x12Aa\x18\xB4V[\x90Pa\x12KaE\x0BV[`\x02T\x81R\x83Q`\x01`\x01`@\x1B\x03\x90\x81\x16` \x80\x84\x01\x91\x90\x91R\x80\x86\x01Q\x90\x91\x16`@\x80\x84\x01\x91\x90\x91R\x80\x86\x01Q``\x80\x85\x01\x91\x90\x91R\x86\x01Q`\x80\x84\x01R`\0\x80T`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x80\x82R`\x05\x80\x85R\x92\x82 `\x03\x81\x01T`\xA0\x87\x01R`\x04\x81\x01T`\xC0\x87\x01R\x91R\x91\x81\x90R\x01T`\xE0\x82\x01Ra\x12\xD3\x82\x82\x85a\x1E\xE4V[a\x12\xF0W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x13}WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x13q`\0\x80Q` aM\xAC\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0BXW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13\xA3a\x10^V[\x7F\xF7\x87!\"n\xFE\x9A\x1B\xB6x\x18\x9A\x16\xD1UI(\xB9\xF2\x19.,\xB9>\xED\xA8;y\xFA@\0}\x81`@Qa\x06\xE6\x91\x90aG\x15V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x14,WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x14)\x91\x81\x01\x90aL\xCBV[`\x01[a\x14KW\x81`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x04\x01a\x08[\x91\x90aG\x15V[`\0\x80Q` aM\xAC\x839\x81Q\x91R\x81\x14a\x14|W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x08[V[a\x14\x86\x83\x83a\x1FpV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0BXW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\xDEa\x150V[\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x93\x94P\x91\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPPV[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90V[a\x15\x80a\x1F\xC6V[a\x10[\x81a\x1F\xEBV[a\x0BXa\x1F\xC6V[\x81Q`\x01`\x01`@\x1B\x03\x16\x15\x15\x80a\x15\xB5WP` \x82\x01Q`\x01`\x01`@\x1B\x03\x16\x15\x15[\x80a\x15\xC2WP`\x80\x82\x01Q\x15[\x80a\x15\xCFWP`\xA0\x82\x01Q\x15[\x80a\x15\xDCWP`\xC0\x82\x01Q\x15[\x80a\x15\xE9WP`\xE0\x82\x01Q\x15[\x80a\x15\xF8WPc\xFF\xFF\xFF\xFF\x81\x16\x15[\x15a\x16\x16W`@QcP\xDD\x03\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x05`\0\x80`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x08a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01U``\x82\x01Q\x81`\x02\x01U`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01U`\xE0\x82\x01Q\x81`\x06\x01U\x90PP\x81`\x05`\0\x80`\x08\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x08a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01U``\x82\x01Q\x81`\x02\x01U`\x80\x82\x01Q\x81`\x03\x01U`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01U`\xE0\x82\x01Q\x81`\x06\x01U\x90PP`\0\x80`\x0Ca\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x80`\0\x80a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0a\x17\xFC\x83a\x0E\xBDV[`\x01\x81\x81U`\xE0\x85\x01Q`\x02\x81\x81U`\x03\x93\x90\x93U`\x04U`\x07\x80T\x80\x83\x01\x82U`\0\x91\x82RC`\0\x80Q` aMl\x839\x81Q\x91R\x90\x91\x01U`@\x80Q\x80\x82\x01\x82R` \x80\x89\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x83R\x92\x90\x98\x01Q\x97\x81\x01\x97\x88R`\x08\x80T\x94\x85\x01\x81U\x90\x92R\x90Q`\0\x80Q` aM\xEC\x839\x81Q\x91R\x92\x90\x93\x02\x91\x82\x01\x80T`\x01`\x01`@\x1B\x03\x19\x16\x93\x90\x91\x16\x92\x90\x92\x17\x90\x91U\x92Q`\0\x80Q` aN,\x839\x81Q\x91R\x90\x93\x01\x92\x90\x92UPPV[a\x18\xBCaE*V[b\x10\0\0\x81R`\x08` \x82\x01R\x7F \xC9@13\xDF\xDE\x9A\x9D8-\xF7o\xB0R5qd\x87%\xAB\xC0\xA7\xC1(0\xBBi\x0E\xC8;3`@\x82\x01QR\x7F\x03\xA0\xA9\xAC\xC3\xE3\x81Z~\xD6\xCB\x13y\xF7\xD1W\xE641dr\x93v9*i:\xCB\xD3\xEC(<` `@\x83\x01Q\x01R\x7F(f\xC1\x8A\xD1\xDF\x10\xEF\x13T,\xCEbP\xCE\x02\xCB*kr\xAE\0\xA9\x85.'\x11\x87\xE9\xE4\xE0\xDB``\x82\x01QR\x7F!\xBE#*B$jVc\xEB\xF4\x83G\x0C\xCAfo\xFE\x9DO\x0Ec\xB9)\xC5\x96\xA7e\x87\x14\xE9p` ``\x83\x01Q\x01R\x7F\x07\xD7xs\xB9\x86\0t\x11\x8Eu\x80\x8CyF\x8B\x83\xC8\xEDd\xBA\x14\xDB\\\xB5\xAF\xA8\xE54\xDE{\x99`\x80\x82\x01QR\x7F\x0B\xE0\xF4H\x83\x90\x80\x13-G\xDE\x17\xDE\0\x99\xB4\xCDt\xAE\x1Ekq\xCD\xDA\x06\xCD\xEB\xB8h\xA5\x0Cm` `\x80\x83\x01Q\x01R\x7F\x13\xBDE\xA0#I\x1E\xAD\xEAD\xCC?$\xCF\xBD\x17\x96\xEA\xDE\x9C\x0E9\xEE\x81\xD9\xF6>\xA0\xA5\x80f%`\xA0\x82\x01QR\x7F\x18\xF9\\\xDD\xA4,\xE1\x1D\x9D\x10\xA3\xB35\xAC\xC2\x14\xE3\x80|W\x8CSY@]\x81\x0C \x8D\xF6\0\x93` `\xA0\x83\x01Q\x01R\x7F\tp\xD9xv4a\xF0\x9E\x9E\xC64T\x074\x978nM(/\xED\xC2\xAC[\x96|\xB9\xFD?\xA8\xA9`\xC0\x82\x01QR\x7F(\xC2!\x7F{\xAC\xF6\xF8\xB2\xB8\xEEJ\x90\xFC\xF8\xB5\xBC\xA0B\x05\xEA\x84\xE8\xE1\xEBT\xB8]\xD4\x1B\xDE(` `\xC0\x83\x01Q\x01R\x7F\x02\xFE=\x02\x98\x8D\xB7\x188\0R\x97\n\xBAF\xA3)m\xF5\xF2\x9Bsk\xA1\xF2\xC4\xCC\xFF\xC8\xB5\x96\x93`\xE0\x82\x01QR\x7F ,>9\x0C\xEE|\\\x85%\xDA#)\xA1\x9FI6\xF6\xF7\x1C\xA9}\xDElo\xA3+8-Z\xCC\x03` `\xE0\x83\x01Q\x01R\x7F#\xAC\x10\xAEl\xA5\xCA\xCE\xE8tK\xB99\xAA\xA859\tT\xB9\x1A\xE6h\xA2\xC8\xD0\xED\xDAU\x8A\x89\xE7a\x01\0\x82\x01QR\x7F\x1C\x8C+\x85l\xDA\xDE%k\xA3#\x7F9\xAF\xD5\xE1p\xA9S \x12\xF7\xAE\xCA\xE4\x9DE\x9B)\xF6\xF6\xAD` a\x01\0\x83\x01Q\x01R\x7F\x16\xEC\x03\xD2`\xBDz\xC1\xC5\x0F\xFAcV]Rt\xB4X,\xEE\xA5/\xF4\x0B\x81\xCD\xFE\x8FDO\x01\xE4a\x01 \x82\x01QR\x7F)9!Rr0\x97\xE0q\x13\xC3\xD7xm$^\xC4\x0C0\x92\x80\x15\xCDP\xB5f\x8AON\xA1p1` a\x01 \x83\x01Q\x01R\x7F,\xDB\xFB:@S\xC8H\x9B\x0C\x94\xE7C8\xAC\x19\x11\x8D\xF7\xA0k\xC5k\x1E\xB4\xD0\xE0\xDCN\xAErHa\x01@\x82\x01QR\x7F\x07\xFE\xA1'\xDA\xE9C\xB8\xDC\x14\x8F\x14\x08\xD4\x0C\xFFF\\\x9CG!\x946i\xB1\xE4\xFDZ9\xDBp6` a\x01@\x83\x01Q\x01R\x7F\x03\x14U\xA7\x9A.\x0C\xE7\x8Al\xB55&\xEC\x04\xAC\x19qj\x86\xB0\x8A\x93\xDFH\xD1x\xF8\xB7~V\x19a\x01`\x82\x01QR\x7F\x11\x86#\xE6\xBC\x13n\xE6\xD3\xF9\x90|\xD4\xAD\x04\xA9A\x8E\xA0;\xA9\x9A\xD7S\"|\xDF\xEEY\x8E\x84\x15` a\x01`\x83\x01Q\x01R\x7F\x08a\xD1\x99wa\xA8R\"j\xAC{\xA9q{\xF6\xAEVE\x10\x99\xBEwL\xDF\x02\xEF5*X\xCB\xC8a\x01\x80\x82\x01QR\x7F\x08\x05\xE3\x92\xBC\xBC\x12\xE4\nr'xc-s\xFE\x98\x1EK\xC6\xFAm\x11x\xB7\n\xF7\xBE\x1C\xB9\xA3\xA3` a\x01\x80\x83\x01Q\x01R\x7F\x10\x1D\x1E9x\xCB\x9F\x1E0=A1D\xEB\xE6v\x82\xC9\xEB\x0C\xFE\x11$)Y\xAA`)\xD7\x8C\xDB\xBCa\x01\xA0\x82\x01QR\x7F\x08\x9E\xB9\xC7'\xE6\xCB\x07\x08+\xC3\xE6\xF4\x0C\xF0OC\x9F\xE4\x80\0`+XGt\xDA\xD7\xEF\xC6`|` a\x01\xA0\x83\x01Q\x01R\x7F-H\x9F$\x93&:\xA8s\xBC\xD9O!\xEF\xB4[\xF2W\xA6\x1D\x81\xC0\xC9\\2\x97\x91e\x06e;@a\x01\xC0\x82\x01QR\x7F\x18\xE4]bz\xAD\xD4\xDF'\x94\xEC\xD9\x90\x9F\xAC\x1Au?\x0Co\xA8\xA9\xC6eJzX\xB0\x91/\xFF\xD5` a\x01\xC0\x83\x01Q\x01R\x7F\x0EC\xE3\xA4\xB1<\xB48\xE2\xAD\x92F\x14&\x1A\xD0$\x02\x14\xFA\x1C\x83\xFC\xDAj\x0B\xF7y\xEB9\xFF\xC5a\x01\xE0\x82\x01QR\x7F\x0E\xAB\xA9\xF4)\xC5\xF6\xFC1\x03\xD4\xCC@V\xC5\0\xFFBB]\x8Ede\xC5\xB8\xE1E!\x9F\x9C\\\xD3` a\x01\xE0\x83\x01Q\x01R\x7F)\xAE5\x1D\t\xDC\xF4\x1C\n\x80\xAB\x059785\x8B\xAA\xB3~o\xBCFK;\xB12X\x99J\x1F\xA4a\x02\0\x82\x01QR\x7F+{\xC7F\x08\xD7\xEC}\xAD\xD0Y}j@\x10\xD8\xBF\xC2\xB3\x19\0(\x19\x01\xCE\xDCB\xBD\xBB\x0F\xB8\xFC` a\x02\0\x83\x01Q\x01R\x7F\x06h\x02\xC7\xCE\xB9\xE9\x13\xD4\xF6T3\xA2\x06a\xE0\x97\xAC\xAC\x1A\xFF\xEC\xBBSJT\xF7j)x\"&a\x02 \x82\x01QR\x7F'\xEC\x80\xE8\x11\xE66\xF34\x82g\x92<\x8Ed\x1B\xD9\x8A~7\xC5!fp\xCB\xFF\x14\xAE2?\x9E\x0E` a\x02 \x83\x01Q\x01R\x7F\x12`M\x1F\x87\xC5\x83\xF6\xC9q\x0Cs\xEA\xF5\x90\xAF\x9D\x07\xAAt=\x13\x81\xD0\xE9\xDF\xF0\xEA\xB2aB9a\x02@\x82\x01QR\x7F\x15\x88W\x9El3x\xEA2\xCBd\x12\x05\xEFv*c\xCD5:\x0B\xD6p9E(\xAD \x81\xEE\x8D\xD4` a\x02@\x83\x01Q\x01R\x7F$}e&\x1D:J\xB0B\xBA\x93s1\xF6\xD0\xC0\xC5\xEB\x9E\xA7\x87S\xA9 \x84\xDB\x1Ai9\xE1\x9E\x82a\x02`\x82\x01QR\x7F,\xE6\xCCfJ2\x14{\xFEj\x0C\x94\xA9[\xF0Ify@\\\xCA\xE0\x16H\xCDN\xC0!\x14Q \xD5` a\x02`\x83\x01Q\x01R\x7F\xB0\x83\x88\x93\xEC\x1F#~\x8B\x072;\x07DY\x9FN\x97\xB5\x98\xB3\xB5\x89\xBC\xC2\xBC7\xB8\xD5\xC4\x18\x01a\x02\x80\x82\x01R\x7F\xC1\x83\x93\xC0\xFA0\xFEN\x8B\x03\x8E5z\xD8Q\xEA\xE8\xDE\x91\x07XN\xFF\xE7\xC7\xF1\xF6Q\xB2\x01\x0E&a\x02\xA0\x82\x01R\x90V[`\0a\x1E\xEF\x82a\x1F\xF3V[a\x1F\0\x83`\0[` \x02\x01Qa\x10\x90V[a\x1F\x0B\x83`\x01a\x1E\xF6V[a\x1F\x16\x83`\x02a\x1E\xF6V[a\x1F!\x83`\x03a\x1E\xF6V[a\x1F,\x83`\x04a\x1E\xF6V[a\x1F7\x83`\x05a\x1E\xF6V[a\x1FB\x83`\x06a\x1E\xF6V[a\x1FM\x83`\x07a\x1E\xF6V[`\0a\x1FZ\x85\x85\x85a!+V[\x90Pa\x1Fe\x81a\"PV[\x91PP[\x93\x92PPPV[a\x1Fy\x82a&\xA3V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\x1F\xBEWa\x14\x86\x82\x82a&\xFFV[a\n\xC9a'uV[a\x1F\xCEa'\x94V[a\x0BXW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10(a\x1F\xC6V[\x80Qa\x1F\xFE\x90a'\xAEV[a \x0B\x81` \x01Qa'\xAEV[a \x18\x81`@\x01Qa'\xAEV[a %\x81``\x01Qa'\xAEV[a 2\x81`\x80\x01Qa'\xAEV[a ?\x81`\xA0\x01Qa'\xAEV[a L\x81`\xC0\x01Qa'\xAEV[a Y\x81`\xE0\x01Qa'\xAEV[a g\x81a\x01\0\x01Qa'\xAEV[a u\x81a\x01 \x01Qa'\xAEV[a \x83\x81a\x01@\x01Qa'\xAEV[a \x91\x81a\x01`\x01Qa'\xAEV[a \x9F\x81a\x01\x80\x01Qa'\xAEV[a \xAD\x81a\x01\xA0\x01Qa\x10\x90V[a \xBB\x81a\x01\xC0\x01Qa\x10\x90V[a \xC9\x81a\x01\xE0\x01Qa\x10\x90V[a \xD7\x81a\x02\0\x01Qa\x10\x90V[a \xE5\x81a\x02 \x01Qa\x10\x90V[a \xF3\x81a\x02@\x01Qa\x10\x90V[a!\x01\x81a\x02`\x01Qa\x10\x90V[a!\x0F\x81a\x02\x80\x01Qa\x10\x90V[a!\x1D\x81a\x02\xA0\x01Qa\x10\x90V[a\x10[\x81a\x02\xC0\x01Qa\x10\x90V[a!3aF=V[`\0a!@\x85\x85\x85a(@V[\x90P`\0a!Q\x86`\0\x01Qa-\xB7V[\x90P`\0a!d\x82\x84`\xA0\x01Q\x88a/=V[`@\x80Q`\x1E\x80\x82Ra\x03\xE0\x82\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01a\x03\xC0\x806\x837PP`@\x80Q`\x1E\x80\x82Ra\x03\xE0\x82\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x82\x01[a!\xAFaF\x8DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a!\xA7W\x90PP\x90P`\0a!\xD4\x8A\x85\x8A\x89\x87\x87a/\x9DV[`\xA0\x87\x01Q`@\x87\x01Q\x91\x92P\x90`\0\x80Q` aM\xCC\x839\x81Q\x91R`\0\x81\x83\x85\t`@\x80Qa\x01\0\x81\x01\x82R`\xE0\x9C\x8D\x01Q\x81R` \x81\x01\x96\x90\x96R\x85\x01RPPP``\x81\x01\x91\x90\x91R`\x80\x81\x01\x92\x90\x92R`\xA0\x82\x01Ra\x01`\x86\x01Q`\xC0\x82\x01Ra\x01\x80\x90\x95\x01Q\x92\x85\x01\x92\x90\x92RP\x91\x94\x93PPPPV[`\0`\0\x80Q` aM\xCC\x839\x81Q\x91Ra\"iaF\x8DV[a\"qaF\x8DV[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R`\0\x92` \x83\x01\x90\x806\x837PP`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x82\x01[a\"\xB3aF\x8DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\"\xABW\x90PP\x90P`\0`\x01\x90P\x80\x83`\0\x81Q\x81\x10a\"\xE2Wa\"\xE2aL~V[` \x02` \x01\x01\x81\x81RPP\x87`\xC0\x01Q\x82`\0\x81Q\x81\x10a#\x06Wa#\x06aL~V[` \x02` \x01\x01\x81\x90RP\x87`\0\x01Q\x83`\x01\x81Q\x81\x10a#)Wa#)aL~V[` \x02` \x01\x01\x81\x81RPP\x87`\xE0\x01Q\x82`\x01\x81Q\x81\x10a#MWa#MaL~V[` \x02` \x01\x01\x81\x90RPa#b\x82\x84a/\xD2V[`\x80\x89\x01QQ\x90\x95P``\x93P\x83\x92P\x90P`\0a#\x81\x82`\x02aL\xE4V[a#\x8C\x90`\x01aL\xE4V[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xA6Wa#\xA6aG)V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#\xCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x93P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xEAWa#\xEAaG)V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$#W\x81` \x01[a$\x10aF\x8DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a$\x08W\x90P[P\x92PPP`\0\x80[\x88`\x80\x01QQ\x81\x10\x15a$\xC5W\x88`\x80\x01Q\x81\x81Q\x81\x10a$OWa$OaL~V[` \x02` \x01\x01Q\x84\x83\x81Q\x81\x10a$iWa$iaL~V[` \x02` \x01\x01\x81\x81RPP\x88`\xA0\x01Q\x81\x81Q\x81\x10a$\x8BWa$\x8BaL~V[` \x02` \x01\x01Q\x83\x83\x81Q\x81\x10a$\xA5Wa$\xA5aL~V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra$\xBB`\x01\x83aL\xE4V[\x91P`\x01\x01a$,V[P\x87` \x01Q\x83\x82\x81Q\x81\x10a$\xDDWa$\xDDaL~V[` \x02` \x01\x01\x81\x81RPP\x87`\xC0\x01Q\x82\x82\x81Q\x81\x10a%\0Wa%\0aL~V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra%\x16`\x01\x82aL\xE4V[\x88Q`@\x8A\x01Q\x91\x92P\x90`\0\x88\x82\x84\t\x90P\x80\x86\x85\x81Q\x81\x10a%<Wa%<aL~V[` \x02` \x01\x01\x81\x81RPPPPP\x87`\xE0\x01Q\x82\x82\x81Q\x81\x10a%bWa%baL~V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra%x`\x01\x82aL\xE4V[\x90Pa%\x87\x88``\x01Qa0\xC0V[\x83\x82\x81Q\x81\x10a%\x99Wa%\x99aL~V[` \x02` \x01\x01\x81\x81RPPa%\xADa0\xF2V[\x82\x82\x81Q\x81\x10a%\xBFWa%\xBFaL~V[` \x02` \x01\x01\x81\x90RPa%\xDCa%\xD7\x83\x85a/\xD2V[a1\x13V[\x93PPPP`\0`@Q\x80`\x80\x01`@R\x80\x7F\x01\x18\xC4\xD5\xB87\xBC\xC2\xBC\x89\xB5\xB3\x98\xB5\x97N\x9FYD\x07;2\x07\x8B~#\x1F\xEC\x93\x88\x83\xB0\x81R` \x01\x7F&\x0E\x01\xB2Q\xF6\xF1\xC7\xE7\xFFNX\x07\x91\xDE\xE8\xEAQ\xD8z5\x8E\x03\x8BN\xFE0\xFA\xC0\x93\x83\xC1\x81R` \x01\x7F\"\xFE\xBD\xA3\xC0\xC0c*VG[B\x14\xE5a^\x11\xE6\xDD?\x96\xE6\xCE\xA2\x85J\x87\xD4\xDA\xCC^U\x81R` \x01\x7F\x04\xFCci\xF7\x11\x0F\xE3\xD2QV\xC1\xBB\x9Ar\x85\x9C\xF2\xA0FA\xF9\x9B\xA4\xEEA<\x80\xDAj_\xE4\x81RP\x90Pa&\x99\x83\x82\x84a&\x94a1\x82V[a2SV[\x96\x95PPPPPPV[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a&\xD0W\x80`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x04\x01a\x08[\x91\x90aG\x15V[`\0\x80Q` aM\xAC\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa'\x1C\x91\x90aL\xF7V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a'WW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a'\\V[``\x91P[P\x91P\x91Pa'l\x85\x83\x83a36V[\x95\x94PPPPPV[4\x15a\x0BXW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a'\x9Ea\x15TV[T`\x01`@\x1B\x90\x04`\xFF\x16\x91\x90PV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` aML\x839\x81Q\x91R\x91\x15\x90\x15\x16\x15a'\xD6WPPPV[\x82Q` \x84\x01Q\x82`\x03\x84\x85\x85\x86\t\x85\t\x08\x83\x82\x83\t\x14\x83\x82\x10\x84\x84\x10\x16\x16\x93PPP\x81a\x14\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv\x10\x9B\x8C\x8DM\x0E\x88\x1A[\x9D\x98[\x1AY\x08\x11\xCCH\x1C\x1B\xDA[\x9D`J\x1B`D\x82\x01R`d\x01a\x08[V[a(\x88`@Q\x80a\x01\0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0`\0\x80Q` aM\xCC\x839\x81Q\x91R\x90P`@Q` \x81\x01`\0\x81R`\xFE`\xE0\x1B\x81R\x86Q`\xC0\x1B`\x04\x82\x01R` \x87\x01Q`\xC0\x1B`\x0C\x82\x01Ra\x02\x80\x87\x01Q` \x82\x01Ra\x02\xA0\x87\x01Q`@\x82\x01R`\x01``\x82\x01R`\0\x80Q` aNl\x839\x81Q\x91R`\x80\x82\x01R`\0\x80Q` aN\x0C\x839\x81Q\x91R`\xA0\x82\x01R`\0\x80Q` aNL\x839\x81Q\x91R`\xC0\x82\x01R`\0\x80Q` aM\x8C\x839\x81Q\x91R`\xE0\x82\x01R`\xE0\x87\x01Q\x80Qa\x01\0\x83\x01R` \x81\x01Qa\x01 \x83\x01RPa\x01\0\x87\x01Q\x80Qa\x01@\x83\x01R` \x81\x01Qa\x01`\x83\x01RPa\x01 \x87\x01Q\x80Qa\x01\x80\x83\x01R` \x81\x01Qa\x01\xA0\x83\x01RPa\x01@\x87\x01Q\x80Qa\x01\xC0\x83\x01R` \x81\x01Qa\x01\xE0\x83\x01RPa\x01`\x87\x01Q\x80Qa\x02\0\x83\x01R` \x81\x01Qa\x02 \x83\x01RPa\x01\x80\x87\x01Q\x80Qa\x02@\x83\x01R` \x81\x01Qa\x02`\x83\x01RPa\x01\xE0\x87\x01Q\x80Qa\x02\x80\x83\x01R` \x81\x01Qa\x02\xA0\x83\x01RPa\x02\0\x87\x01Q\x80Qa\x02\xC0\x83\x01R` \x81\x01Qa\x02\xE0\x83\x01RPa\x02 \x87\x01Q\x80Qa\x03\0\x83\x01R` \x81\x01Qa\x03 \x83\x01RPa\x02@\x87\x01Q\x80Qa\x03@\x83\x01R` \x81\x01Qa\x03`\x83\x01RPa\x01\xA0\x87\x01Q\x80Qa\x03\x80\x83\x01R` \x81\x01Qa\x03\xA0\x83\x01RPa\x01\xC0\x87\x01Q\x80Qa\x03\xC0\x83\x01R` \x81\x01Qa\x03\xE0\x83\x01RPa\x02`\x87\x01Q\x80Qa\x04\0\x83\x01R` \x81\x01Qa\x04 \x83\x01RP`@\x87\x01Q\x80Qa\x04@\x83\x01R` \x81\x01Qa\x04`\x83\x01RP``\x87\x01Q\x80Qa\x04\x80\x83\x01R` \x81\x01Qa\x04\xA0\x83\x01RP`\x80\x87\x01Q\x80Qa\x04\xC0\x83\x01R` \x81\x01Qa\x04\xE0\x83\x01RP`\xA0\x87\x01Q\x80Qa\x05\0\x83\x01R` \x81\x01Qa\x05 \x83\x01RP`\xC0\x87\x01Q\x80Qa\x05@\x83\x01R` \x81\x01Qa\x05`\x83\x01RP\x85Qa\x05\x80\x82\x01R` \x86\x01Qa\x05\xA0\x82\x01R`@\x86\x01Qa\x05\xC0\x82\x01R``\x86\x01Qa\x05\xE0\x82\x01R`\x80\x86\x01Qa\x06\0\x82\x01R`\xA0\x86\x01Qa\x06 \x82\x01R`\xC0\x86\x01Qa\x06@\x82\x01R`\xE0\x86\x01Qa\x06`\x82\x01R\x84Q\x80Qa\x06\x80\x83\x01R` \x81\x01Qa\x06\xA0\x83\x01RP` \x85\x01Q\x80Qa\x06\xC0\x83\x01R` \x81\x01Qa\x06\xE0\x83\x01RP`@\x85\x01Q\x80Qa\x07\0\x83\x01R` \x81\x01Qa\x07 \x83\x01RP``\x85\x01Q\x80Qa\x07@\x83\x01R` \x81\x01Qa\x07`\x83\x01RP`\x80\x85\x01Q\x80Qa\x07\x80\x83\x01R` \x81\x01Qa\x07\xA0\x83\x01RP`\0\x82Ra\x07\xE0\x82 a\x07\xC0\x82\x01Ra\x07\xC0\x81\x01\x91P` \x82\x01\x90P\x82\x82Q\x06``\x85\x01R` \x82 \x81R\x80\x91P` \x82\x01\x90P\x82\x82Q\x06`\x80\x85\x01R`\xA0\x85\x01Q\x80Q\x82R` \x81\x01Q` \x83\x01RP``\x82 `@\x82\x01\x92P\x80\x83R` \x83\x01\x91P\x83\x81\x06\x85R\x83\x81\x82\t\x84\x82\x82\t\x91P\x80` \x87\x01RP\x80`@\x86\x01RP`\xC0\x85\x01Q\x80Q\x82R` \x81\x01Q` \x83\x01RP`\xE0\x85\x01Q\x80Q`@\x83\x01R` \x81\x01Q``\x83\x01RPa\x01\0\x85\x01Q\x80Q`\x80\x83\x01R` \x81\x01Q`\xA0\x83\x01RPa\x01 \x85\x01Q\x80Q`\xC0\x83\x01R` \x81\x01Q`\xE0\x83\x01RPa\x01@\x85\x01Q\x80Qa\x01\0\x83\x01R` \x81\x01Qa\x01 \x83\x01RPa\x01`\x82 a\x01@\x82\x01Ra\x01@\x81\x01\x91P` \x82\x01\x90P\x82\x82Q\x06`\xA0\x85\x01Ra\x01\xA0\x85\x01Q\x81Ra\x01\xC0\x85\x01Q` \x82\x01Ra\x01\xE0\x85\x01Q`@\x82\x01Ra\x02\0\x85\x01Q``\x82\x01Ra\x02 \x85\x01Q`\x80\x82\x01Ra\x02@\x85\x01Q`\xA0\x82\x01Ra\x02`\x85\x01Q`\xC0\x82\x01Ra\x02\x80\x85\x01Q`\xE0\x82\x01Ra\x02\xA0\x85\x01Qa\x01\0\x82\x01Ra\x02\xC0\x85\x01Qa\x01 \x82\x01Ra\x01`\x82 a\x01@\x82\x01Ra\x01@\x81\x01\x91P` \x82\x01\x90P\x82\x82Q\x06`\xC0\x85\x01Ra\x01`\x85\x01Q\x80Q\x82R` \x81\x01Q` \x83\x01RPa\x01\x80\x85\x01Q\x80Q`@\x83\x01R` \x81\x01Q``\x83\x01RPP`\xA0\x81 \x82\x81\x06`\xE0\x85\x01RPPP\x93\x92PPPV[a-\xDB`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81b\x01\0\0\x03a.GW`@Q\x80``\x01`@R\x80`\x10\x81R` \x01\x7F0d\x1E\x0E\x92\xBE\xBE\xF8\x18&\x8Df;\xCA\xD6\xDB\xCF\xD6\xC0\x14\x91p\xF6\xD7\xD3P\xB1\xB1\xFAl\x10\x01\x81R` \x01~\xEE\xB2\xCBY\x81\xEDEd\x9A\xBE\xBD\xE0\x81\xDC\xFF\x16\xC8`\x1D\xE44~}\xD1b\x8B\xA2\xDA\xACC\xB7\x81RP\x90P\x91\x90PV[\x81b\x10\0\0\x03a.\xB4W`@Q\x80``\x01`@R\x80`\x14\x81R` \x01\x7F0dKl\x9CJr\x16\x9EM\xAA1}%\xF0E\x12\xAE\x15\xC5;4\xE8\xF5\xAC\xD8\xE1U\xD0\xA6\xC1\x01\x81R` \x01\x7F&\x12]\xA1\n\x0E\xD0c'P\x8A\xBA\x06\xD1\xE3\x03\xACaf2\xDB\xED4\x9FSB-\xA9S3xW\x81RP\x90P\x91\x90PV[\x81` \x03a/\x1FW`@Q\x80``\x01`@R\x80`\x05\x81R` \x01\x7F.\xE1+\xFFJ(\x13(j\x8D\xC3\x88\xCDuM\x9A>\xF2I\x065\xEB\xA5\x0C\xB9\xC2\xE5\xE7P\x80\0\x01\x81R` \x01\x7F\t\xC52\xC60k\x93\xD2\x96x \rG\xC0\xB2\xA9\x9C\x18\xD5\x1B\x83\x8E\xEB\x1D>\xEDLS;\xB5\x12\xD0\x81RP\x90P\x91\x90PV[`@Qc\xE2\xEF\t\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a/a`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a/k\x84\x84a3\x89V[\x80\x82Ra/{\x90\x85\x90\x85\x90a3\xDDV[` \x82\x01R\x80Qa/\x91\x90\x85\x90\x84\x90\x86\x90a4QV[`@\x82\x01R\x93\x92PPPV[`\0\x80a/\xAB\x85\x87\x89a6\x02V[\x90Pa/\xBB\x88\x86\x89\x89\x88\x88a6\xF2V[a/\xC6\x81\x87\x86a9\xF8V[\x98\x97PPPPPPPPV[a/\xDAaF\x8DV[\x82Q\x82Q\x14a0+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FMSM error: length does not match`D\x82\x01R`d\x01a\x08[V[a0i\x83`\0\x81Q\x81\x10a0AWa0AaL~V[` \x02` \x01\x01Q\x83`\0\x81Q\x81\x10a0\\Wa0\\aL~V[` \x02` \x01\x01Qa:HV[\x90P`\x01[\x82Q\x81\x10\x15a0\xB9Wa0\xAF\x82a0\xAA\x86\x84\x81Q\x81\x10a0\x90Wa0\x90aL~V[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a0\\Wa0\\aL~V[a:\xDCV[\x91P`\x01\x01a0nV[P\x92\x91PPV[`\0a0\xDA`\0\x80Q` aM\xCC\x839\x81Q\x91R\x83aM)V[a\x10\x1A\x90`\0\x80Q` aM\xCC\x839\x81Q\x91RaLkV[a0\xFAaF\x8DV[P`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x02` \x82\x01R\x90V[a1\x1BaF\x8DV[\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a1/WP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aML\x839\x81Q\x91R\x84` \x01Qa1b\x91\x90aM)V[a1z\x90`\0\x80Q` aML\x839\x81Q\x91RaLkV[\x90R\x92\x91PPV[a1\xAD`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R` \x87\x01Q`@\x82\x01R\x86Q``\x82\x01R``\x87\x01Q`\x80\x82\x01R`@\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R` \x85\x01Qa\x01\0\x82\x01R\x84Qa\x01 \x82\x01R``\x85\x01Qa\x01@\x82\x01R`@\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a3(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R{Bn254: Pairing check failed!` \x1B`D\x82\x01R`d\x01a\x08[V[P\x15\x15\x90P[\x94\x93PPPPV[``\x82a3KWa3F\x82a;wV[a\x1FiV[\x81Q\x15\x80\x15a3bWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a3\x82W\x83`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x04\x01a\x08[\x91\x90aG\x15V[P\x80a\x1FiV[\x81Q`\0\x90`\0\x80Q` aM\xCC\x839\x81Q\x91R\x90\x83\x80\x15a3\xCDW\x84\x93P`\0[\x82\x81\x10\x15a3\xC1W\x83\x85\x86\t\x94P`\x01\x01a3\xABV[P`\x01\x84\x03\x93Pa3\xD4V[`\x01\x83\x03\x93P[PPP\x92\x91PPV[`\0\x82`\x01\x03a3\xEFWP`\x01a\x1FiV[\x81`\0\x03a3\xFFWP`\0a\x1FiV[` \x84\x01Q`\0\x80Q` aM\xCC\x839\x81Q\x91R\x90`\0\x90\x82\x81\x86\t\x90P\x85\x80\x15a4/W`\x01\x87\x03\x92Pa46V[`\x01\x84\x03\x92P[Pa4@\x82a;\xA0V[\x91P\x82\x82\x82\t\x97\x96PPPPPPPV[`\0`\0\x80Q` aM\xCC\x839\x81Q\x91R\x82\x82\x03a4\xCAW`\x01`\0[`\x08\x81\x10\x15a4\xBEW\x81\x86\x03a4\x9FW\x86\x81`\x08\x81\x10a4\x90Wa4\x90aL~V[` \x02\x01Q\x93PPPPa3.V[\x82\x80a4\xADWa4\xADaM\x13V[\x88`@\x01Q\x83\t\x91P`\x01\x01a4nV[P`\0\x92PPPa3.V[a4\xD2aE\x0BV[`@\x87\x01Q`\x01\x80\x83R\x83\x82\x82\t\x90P\x80` \x84\x01R\x83\x82\x82\t\x90P\x80`@\x84\x01R\x83\x82\x82\t\x90P\x80``\x84\x01R\x83\x82\x82\t\x90P\x80`\x80\x84\x01R\x83\x82\x82\t\x90P\x80`\xA0\x84\x01R\x83\x82\x82\t\x90P\x80`\xC0\x84\x01R\x83\x82\x82\t`\xE0\x84\x01RPa56aE\x0BV[`\x01`\xE0\x82\x81\x01\x82\x81R\x91\x90\x85\x01\x90\x80[`\x08\x81\x10\x15a5tW` \x84\x03\x93P\x87\x88\x8C\x85Q\x8B\x03\x08\x83\t\x80\x85R`\x1F\x19\x90\x93\x01\x92\x91P`\x01\x01a5GV[PPPP`\0\x80`\0\x90P`\x01\x83\x8B\x87`\0[`\x08\x81\x10\x15a5\xC7W\x8A\x82Q\x8C\x85Q\x8E\x88Q\x8A\t\t\t\x8B\x81\x88\x08\x96PP\x8A\x8B\x8F\x84Q\x8E\x03\x08\x86\t\x94P` \x93\x84\x01\x93\x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a5\x87V[PPPP\x80\x92PP`\0a5\xDA\x83a;\xA0V[\x90P` \x8C\x01Q\x87\x81\x8B\t\x98PP\x86\x81\x89\t\x97P\x86\x82\x89\t\x9C\x9BPPPPPPPPPPPPV[`\0\x80`\0\x80Q` aM\xCC\x839\x81Q\x91R\x90P`\0\x83` \x01Q\x90P`\0\x84`@\x01Q\x90P`\0`\x01\x90P``\x88\x01Q`\x80\x89\x01Qa\x01\xA0\x89\x01Qa\x02@\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x01\xC0\x89\x01Qa\x02`\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x01\xE0\x89\x01Qa\x02\x80\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x02\0\x89\x01Qa\x02\xA0\x8A\x01Q\x87\x88\x89\x83\x87\t\x8A\x86\x86\x08\x08\x86\t\x94PPPa\x02 \x89\x01Q\x91Pa\x02\xC0\x89\x01Q\x86\x87\x82\x89\x85\x87\x08\t\x85\t\x93PPPP\x87Q` \x89\x01Q\x85\x86\x86\x83\t\x87\x03\x85\x08\x96PP\x84\x85\x83\x83\t\x86\x03\x87\x08\x99\x98PPPPPPPPPV[a7\0\x86\x86\x86\x86\x85\x87a<FV[`\xC0\x85\x01Q\x82Q`\0\x80Q` aM\xCC\x839\x81Q\x91R\x91\x90\x81\x90\x81\x90\x86\x90`\x14\x90\x81\x10a7/Wa7/aL~V[` \x02` \x01\x01\x81\x81RPP\x85`\0\x01Q\x84`\x14\x81Q\x81\x10a7SWa7SaL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x15\x81Q\x81\x10a7xWa7xaL~V[` \x02` \x01\x01\x81\x81RPP\x85` \x01Q\x84`\x15\x81Q\x81\x10a7\x9CWa7\x9CaL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x16\x81Q\x81\x10a7\xC1Wa7\xC1aL~V[` \x02` \x01\x01\x81\x81RPP\x85`@\x01Q\x84`\x16\x81Q\x81\x10a7\xE5Wa7\xE5aL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x17\x81Q\x81\x10a8\nWa8\naL~V[` \x02` \x01\x01\x81\x81RPP\x85``\x01Q\x84`\x17\x81Q\x81\x10a8.Wa8.aL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x18\x81Q\x81\x10a8SWa8SaL~V[` \x02` \x01\x01\x81\x81RPP\x85`\x80\x01Q\x84`\x18\x81Q\x81\x10a8wWa8waL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x19\x81Q\x81\x10a8\x9CWa8\x9CaL~V[` \x02` \x01\x01\x81\x81RPP\x88`@\x01Q\x84`\x19\x81Q\x81\x10a8\xC0Wa8\xC0aL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x1A\x81Q\x81\x10a8\xE5Wa8\xE5aL~V[` \x02` \x01\x01\x81\x81RPP\x88``\x01Q\x84`\x1A\x81Q\x81\x10a9\tWa9\taL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x1B\x81Q\x81\x10a9.Wa9.aL~V[` \x02` \x01\x01\x81\x81RPP\x88`\x80\x01Q\x84`\x1B\x81Q\x81\x10a9RWa9RaL~V[` \x02` \x01\x01\x81\x90RP\x82\x82\x82\t\x90P\x80\x85`\x1C\x81Q\x81\x10a9wWa9waL~V[` \x02` \x01\x01\x81\x81RPP\x88`\xA0\x01Q\x84`\x1C\x81Q\x81\x10a9\x9BWa9\x9BaL~V[` \x02` \x01\x01\x81\x90RP\x87`\xE0\x01Q\x85`\x1D\x81Q\x81\x10a9\xBEWa9\xBEaL~V[` \x02` \x01\x01\x81\x81RPP\x85`\xA0\x01Q\x84`\x1D\x81Q\x81\x10a9\xE2Wa9\xE2aL~V[` \x02` \x01\x01\x81\x90RPPPPPPPPPPV[`\0\x80Q` aM\xCC\x839\x81Q\x91R\x83\x81\x03\x90`\0[`\n\x81\x10\x15a:?W` `\x15\x82\x01\x02\x84\x01Q` \x82\x02a\x01\xA0\x01\x86\x01Q\x83\x84\x82\x84\t\x86\x08\x94PPP`\x01\x01a:\x0EV[PP\x93\x92PPPV[a:PaF\x8DV[a:XaF\xA7V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x81\x01\x83\x90R`\0``\x83`\x80\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a:\x8AW`\0\x80\xFD[P\x80a:\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01RxBn254: scalar mul failed!`8\x1B`D\x82\x01R`d\x01a\x08[V[PP\x92\x91PPV[a:\xE4aF\x8DV[a:\xECaF\xC5V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x83\x01R\x83\x01Q``\x80\x83\x01\x91\x90\x91R`\0\x90\x83`\xC0\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a;)W`\0\x80\xFD[P\x80a:\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FBn254: group addition failed!\0\0\0`D\x82\x01R`d\x01a\x08[V[\x80Q\x15a;\x87W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0`\0\x80Q` aM\xCC\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01R`\x02\x82\x03`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x92PP`\0Q\x92P\x81a<?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FBn254: pow precompile failed!\0\0\0`D\x82\x01R`d\x01a\x08[V[PP\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80Q` aM\xCC\x839\x81Q\x91R\x90P\x80` \x8B\x01Q` \x8D\x01Q\t\x95P\x8AQ\x93P\x80`\xA0\x8C\x01Q``\x8D\x01Q\t\x92P\x80a\x01\xA0\x8A\x01Q\x84\x08\x91P\x80`\x80\x8C\x01Q\x83\x08\x91P\x80\x84\x83\t\x93P\x80`\0\x80Q` aNl\x839\x81Q\x91R\x84\t\x91P\x80a\x01\xC0\x8A\x01Q\x83\x08\x91P\x80`\x80\x8C\x01Q\x83\x08\x91P\x80\x84\x83\t\x93P\x80`\0\x80Q` aN\x0C\x839\x81Q\x91R\x84\t\x91P\x80a\x01\xE0\x8A\x01Q\x83\x08\x91P\x80`\x80\x8C\x01Q\x83\x08\x91P\x80\x84\x83\t\x93P\x80`\0\x80Q` aNL\x839\x81Q\x91R\x84\t\x91P\x80a\x02\0\x8A\x01Q\x83\x08\x91P\x80`\x80\x8C\x01Q\x83\x08\x91P\x80\x84\x83\t\x93P\x80`\0\x80Q` aM\x8C\x839\x81Q\x91R\x84\t\x91P\x80a\x02 \x8A\x01Q\x83\x08\x91P\x80`\x80\x8C\x01Q\x83\x08\x91P\x80\x84\x83\t\x93P\x80\x84\x87\x08\x95P\x88`\xA0\x01Q\x88`\0\x81Q\x81\x10a=uWa=uaL~V[` \x02` \x01\x01\x81\x90RP\x85\x87`\0\x81Q\x81\x10a=\x94Wa=\x94aL~V[` \x02` \x01\x01\x81\x81RPP\x80``\x8C\x01Q\x8CQ\t\x94P\x80a\x02\xC0\x8A\x01Q\x86\t\x94P\x80a\x02@\x8A\x01Q``\x8D\x01Q\t\x92P\x80a\x01\xA0\x8A\x01Q\x84\x08\x92P\x80`\x80\x8C\x01Q\x84\x08\x92P\x80\x83\x86\t\x94P\x80a\x02`\x8A\x01Q``\x8D\x01Q\t\x92P\x80a\x01\xC0\x8A\x01Q\x84\x08\x92P\x80`\x80\x8C\x01Q\x84\x08\x92P\x80\x83\x86\t\x94P\x80a\x02\x80\x8A\x01Q``\x8D\x01Q\t\x92P\x80a\x01\xE0\x8A\x01Q\x84\x08\x92P\x80`\x80\x8C\x01Q\x84\x08\x92P\x80\x83\x86\t\x94P\x80a\x02\xA0\x8A\x01Q``\x8D\x01Q\t\x92P\x80a\x02\0\x8A\x01Q\x84\x08\x92P\x80`\x80\x8C\x01Q\x84\x08\x92P\x80\x83\x86\t\x94P\x8B`\xC0\x01Q\x88`\x01\x81Q\x81\x10a>vWa>vaL~V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra>\x8B\x85\x82aLkV[\x87`\x01\x81Q\x81\x10a>\x9EWa>\x9EaL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x01\xA0\x01Q\x87`\x02\x81Q\x81\x10a>\xC3Wa>\xC3aL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x01\xC0\x01Q\x87`\x03\x81Q\x81\x10a>\xE8Wa>\xE8aL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x01\xE0\x01Q\x87`\x04\x81Q\x81\x10a?\rWa?\raL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x02\0\x01Q\x87`\x05\x81Q\x81\x10a?2Wa?2aL~V[` \x02` \x01\x01\x81\x81RPP\x8B`\xE0\x01Q\x88`\x02\x81Q\x81\x10a?VWa?VaL~V[` \x02` \x01\x01\x81\x90RP\x8Ba\x01\0\x01Q\x88`\x03\x81Q\x81\x10a?zWa?zaL~V[` \x02` \x01\x01\x81\x90RP\x8Ba\x01 \x01Q\x88`\x04\x81Q\x81\x10a?\x9EWa?\x9EaL~V[` \x02` \x01\x01\x81\x90RP\x8Ba\x01@\x01Q\x88`\x05\x81Q\x81\x10a?\xC2Wa?\xC2aL~V[` \x02` \x01\x01\x81\x90RP\x80a\x01\xC0\x8A\x01Qa\x01\xA0\x8B\x01Q\t\x92P\x82\x87`\x06\x81Q\x81\x10a?\xF1Wa?\xF1aL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x01`\x01Q\x88`\x06\x81Q\x81\x10a@\x16Wa@\x16aL~V[` \x02` \x01\x01\x81\x90RP\x80a\x02\0\x8A\x01Qa\x01\xE0\x8B\x01Q\t\x92P\x82\x87`\x07\x81Q\x81\x10a@EWa@EaL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x01\x80\x01Q\x88`\x07\x81Q\x81\x10a@jWa@jaL~V[` \x02` \x01\x01\x81\x90RPa\x01\xA0\x89\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92P\x82\x87`\x08\x81Q\x81\x10a@\xA3Wa@\xA3aL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x01\xE0\x01Q\x88`\x08\x81Q\x81\x10a@\xC8Wa@\xC8aL~V[` \x02` \x01\x01\x81\x90RPa\x01\xC0\x89\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92P\x82\x87`\t\x81Q\x81\x10aA\x01WaA\x01aL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x02\0\x01Q\x88`\t\x81Q\x81\x10aA&WaA&aL~V[` \x02` \x01\x01\x81\x90RPa\x01\xE0\x89\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92P\x82\x87`\n\x81Q\x81\x10aA_WaA_aL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x02 \x01Q\x88`\n\x81Q\x81\x10aA\x84WaA\x84aL~V[` \x02` \x01\x01\x81\x90RPa\x02\0\x89\x01Q\x92P\x80\x83\x84\t\x91P\x80\x82\x83\t\x91P\x80\x82\x84\t\x92P\x82\x87`\x0B\x81Q\x81\x10aA\xBDWaA\xBDaL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x02@\x01Q\x88`\x0B\x81Q\x81\x10aA\xE2WaA\xE2aL~V[` \x02` \x01\x01\x81\x90RP\x88a\x02 \x01Q\x81aA\xFE\x91\x90aLkV[\x87`\x0C\x81Q\x81\x10aB\x11WaB\x11aL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x01\xA0\x01Q\x88`\x0C\x81Q\x81\x10aB6WaB6aL~V[` \x02` \x01\x01\x81\x90RP`\x01\x87`\r\x81Q\x81\x10aBVWaBVaL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x01\xC0\x01Q\x88`\r\x81Q\x81\x10aB{WaB{aL~V[` \x02` \x01\x01\x81\x90RP\x80a\x01\xC0\x8A\x01Qa\x01\xA0\x8B\x01Q\t\x92P\x80a\x01\xE0\x8A\x01Q\x84\t\x92P\x80a\x02\0\x8A\x01Q\x84\t\x92P\x80a\x02 \x8A\x01Q\x84\t\x92P\x82\x87`\x0E\x81Q\x81\x10aB\xCBWaB\xCBaL~V[` \x02` \x01\x01\x81\x81RPP\x8Ba\x02`\x01Q\x88`\x0E\x81Q\x81\x10aB\xF0WaB\xF0aL~V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x89QaC\x07\x90\x82aLkV[\x87`\x0F\x81Q\x81\x10aC\x1AWaC\x1AaL~V[` \x02` \x01\x01\x81\x81RPP\x88`\xC0\x01Q\x88`\x0F\x81Q\x81\x10aC>WaC>aL~V[` \x02` \x01\x01\x81\x90RP\x80`\x01\x8BQ\x08`\xA0\x8C\x01Q\x90\x93P\x81\x90\x80\t\x91P\x80\x82\x84\t\x92P\x80\x83` `\x10\x02\x89\x01Q\t\x91P\x81\x87`\x10\x81Q\x81\x10aC\x84WaC\x84aL~V[` \x02` \x01\x01\x81\x81RPP\x88`\xE0\x01Q\x88`\x10\x81Q\x81\x10aC\xA8WaC\xA8aL~V[` \x02` \x01\x01\x81\x90RP\x80\x83` `\x11\x02\x89\x01Q\t\x91P\x81\x87`\x11\x81Q\x81\x10aC\xD4WaC\xD4aL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x01\0\x01Q\x88`\x11\x81Q\x81\x10aC\xF9WaC\xF9aL~V[` \x02` \x01\x01\x81\x90RP\x80\x83` `\x12\x02\x89\x01Q\t\x91P\x81\x87`\x12\x81Q\x81\x10aD%WaD%aL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x01 \x01Q\x88`\x12\x81Q\x81\x10aDJWaDJaL~V[` \x02` \x01\x01\x81\x90RP\x80\x83` `\x13\x02\x89\x01Q\t\x91P\x81\x87`\x13\x81Q\x81\x10aDvWaDvaL~V[` \x02` \x01\x01\x81\x81RPP\x88a\x01@\x01Q\x88`\x13\x81Q\x81\x10aD\x9BWaD\x9BaL~V[` \x02` \x01\x01\x81\x90RPPPPPPPPPPPPPV[`@Q\x80a\x01\0\x01`@R\x80`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80a\x01\0\x01`@R\x80`\x08\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x02\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01aELaF\x8DV[\x81R` \x01aEYaF\x8DV[\x81R` \x01aEfaF\x8DV[\x81R` \x01aEsaF\x8DV[\x81R` \x01aE\x80aF\x8DV[\x81R` \x01aE\x8DaF\x8DV[\x81R` \x01aE\x9AaF\x8DV[\x81R` \x01aE\xA7aF\x8DV[\x81R` \x01aE\xB4aF\x8DV[\x81R` \x01aE\xC1aF\x8DV[\x81R` \x01aE\xCEaF\x8DV[\x81R` \x01aE\xDBaF\x8DV[\x81R` \x01aE\xE8aF\x8DV[\x81R` \x01aE\xF5aF\x8DV[\x81R` \x01aF\x02aF\x8DV[\x81R` \x01aF\x0FaF\x8DV[\x81R` \x01aF\x1CaF\x8DV[\x81R` \x01aF)aF\x8DV[\x81R`\0` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80a\x01\0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81R` \x01``\x81R` \x01aF{aF\x8DV[\x81R` \x01aF\x88aF\x8DV[\x90R\x90V[`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a/8W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aG\x0CW`\0\x80\xFD[a\x1Fi\x82aF\xE3V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x02\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aGbWaGbaG)V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aG\x90WaG\x90aG)V[`@R\x91\x90PV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a/8W`\0\x80\xFD[`\0a\x01\0\x80\x83\x85\x03\x12\x15aG\xC3W`\0\x80\xFD[`@Q\x90\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x81\x83\x10\x17\x15aG\xE5WaG\xE5aG)V[\x81`@R\x80\x92PaG\xF5\x84aG\x98V[\x81RaH\x03` \x85\x01aG\x98V[` \x82\x01R`@\x84\x015`@\x82\x01R``\x84\x015``\x82\x01R`\x80\x84\x015`\x80\x82\x01R`\xA0\x84\x015`\xA0\x82\x01R`\xC0\x84\x015`\xC0\x82\x01R`\xE0\x84\x015`\xE0\x82\x01RPP\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15aH^W`\0\x80\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH\x80WaH\x80aG)V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x80\x82\x84\x03a\x05\x80\x81\x12\x15aH\xAFW`\0\x80\xFD[aH\xB9\x85\x85aG\xAFV[\x92Pa\x01\0a\x04\x80\x80`\xFF\x19\x84\x01\x12\x15aH\xD2W`\0\x80\xFD[aH\xDAaG?V[\x92PaH\xE8\x87\x83\x88\x01aHLV[\x83Ra\x01@aH\xF9\x88\x82\x89\x01aHLV[` \x85\x01Ra\x01\x80aI\r\x89\x82\x8A\x01aHLV[`@\x86\x01Ra\x01\xC0aI!\x8A\x82\x8B\x01aHLV[``\x87\x01Ra\x02\0aI5\x8B\x82\x8C\x01aHLV[`\x80\x88\x01Ra\x02@aII\x8C\x82\x8D\x01aHLV[`\xA0\x89\x01Ra\x02\x80aI]\x8D\x82\x8E\x01aHLV[`\xC0\x8A\x01Ra\x02\xC0aIq\x8E\x82\x8F\x01aHLV[`\xE0\x8B\x01RaI\x84\x8Ea\x03\0\x8F\x01aHLV[\x89\x8B\x01RaI\x96\x8Ea\x03@\x8F\x01aHLV[a\x01 \x8B\x01RaI\xAA\x8Ea\x03\x80\x8F\x01aHLV[\x87\x8B\x01RaI\xBC\x8Ea\x03\xC0\x8F\x01aHLV[a\x01`\x8B\x01RaI\xD0\x8Ea\x04\0\x8F\x01aHLV[\x86\x8B\x01Ra\x04@\x8D\x015a\x01\xA0\x8B\x01Ra\x04`\x8D\x015\x85\x8B\x01R\x87\x8D\x015a\x01\xE0\x8B\x01Ra\x04\xA0\x8D\x015\x84\x8B\x01Ra\x04\xC0\x8D\x015a\x02 \x8B\x01Ra\x04\xE0\x8D\x015\x83\x8B\x01Ra\x05\0\x8D\x015a\x02`\x8B\x01Ra\x05 \x8D\x015\x82\x8B\x01Ra\x05@\x8D\x015a\x02\xA0\x8B\x01Ra\x05`\x8D\x015\x81\x8B\x01RPPPPPPPPP\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aJgW`\0\x80\xFD[aJp\x83aF\xE3V[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aJ\x8DW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12aJ\xA1W`\0\x80\xFD[\x815\x81\x81\x11\x15aJ\xB3WaJ\xB3aG)V[aJ\xC5`\x1F\x82\x01`\x1F\x19\x16\x85\x01aGhV[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15aJ\xDBW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\x01`\x01`@\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a/8W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aK3W`\0\x80\xFD[a\x1Fi\x82aK\rV[`\0` \x82\x84\x03\x12\x15aKNW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0a\x01@\x84\x86\x03\x12\x15aKkW`\0\x80\xFD[aKu\x85\x85aG\xAFV[\x92PaK\x84a\x01\0\x85\x01aK\rV[\x91PaK\x93a\x01 \x85\x01aF\xE3V[\x90P\x92P\x92P\x92V[`\0a\x01\0\x82\x84\x03\x12\x15aK\xAFW`\0\x80\xFD[a\x1Fi\x83\x83aG\xAFV[`\0[\x83\x81\x10\x15aK\xD4W\x81\x81\x01Q\x83\x82\x01R` \x01aK\xBCV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01RaK\xFC\x81`@\x85\x01` \x87\x01aK\xB9V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aL#W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x80\x82\x16\x91\x90\x82\x81\x14a:\xD4Wa:\xD4aL2V[\x81\x81\x03\x81\x81\x11\x15a\x10\x1AWa\x10\x1AaL2V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81aL\xA3WaL\xA3aL2V[P`\0\x19\x01\x90V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a0\xB9Wa0\xB9aL2V[`\0` \x82\x84\x03\x12\x15aL\xDDW`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x10\x1AWa\x10\x1AaL2V[`\0\x82QaM\t\x81\x84` \x87\x01aK\xB9V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aMFWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88.+\x91Ea\x03i\x8A\xDFW\xB7\x99\x96\x9D\xEA\x1C\x8Fs\x9D\xA5\xD8\xD4\r\xD3\xEB\x92\"\xDB|\x81\xE8\x816\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE3\x1E\xE6x\xA0G\nu\xA6\xEA\xA8\xFE\x83p`I\x8B\xA8(\xA3p;1\x1D\x0Fw\xF0\x10BJ\xFE\xB0%\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE4 B\xA5\x87\xA9\x0C\x18{\n\x08|\x03\xE2\x9C\x96\x8B\x95\x0B\x1D\xB2m\\\x82\xD6f\x90Zh\x95y\x0C\n/\x8D\xD1\xF1\xA7X<B\xC4\xE1*D\xE1\x10@Ls\xCAl\x94\x81?\x85\x83]\xA4\xFB{\xB10\x1DJ\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static LIGHTCLIENT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LightClient<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LightClient<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LightClient<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LightClient<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LightClient<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LightClient))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LightClient<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LIGHTCLIENT_ABI.clone(),
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
                LIGHTCLIENT_ABI.clone(),
                LIGHTCLIENT_BYTECODE.clone().into(),
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
        ///Calls the contract's `setPermissionedProver` (0x013fa5fc) function
        pub fn set_permissioned_prover(
            &self,
            prover: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 63, 165, 252], prover)
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LightClientEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for LightClient<M> {
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
    pub enum LightClientErrors {
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
    impl ::ethers::core::abi::AbiDecode for LightClientErrors {
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
    impl ::ethers::core::abi::AbiEncode for LightClientErrors {
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
    impl ::ethers::contract::ContractRevert for LightClientErrors {
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
    impl ::core::fmt::Display for LightClientErrors {
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
    impl ::core::convert::From<::std::string::String> for LightClientErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for LightClientErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation> for LightClientErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for LightClientErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for LightClientErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<InsufficientSnapshotHistory> for LightClientErrors {
        fn from(value: InsufficientSnapshotHistory) -> Self {
            Self::InsufficientSnapshotHistory(value)
        }
    }
    impl ::core::convert::From<InvalidAddress> for LightClientErrors {
        fn from(value: InvalidAddress) -> Self {
            Self::InvalidAddress(value)
        }
    }
    impl ::core::convert::From<InvalidArgs> for LightClientErrors {
        fn from(value: InvalidArgs) -> Self {
            Self::InvalidArgs(value)
        }
    }
    impl ::core::convert::From<InvalidHotShotBlockForCommitmentCheck> for LightClientErrors {
        fn from(value: InvalidHotShotBlockForCommitmentCheck) -> Self {
            Self::InvalidHotShotBlockForCommitmentCheck(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for LightClientErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<InvalidProof> for LightClientErrors {
        fn from(value: InvalidProof) -> Self {
            Self::InvalidProof(value)
        }
    }
    impl ::core::convert::From<MissingLastBlockForCurrentEpoch> for LightClientErrors {
        fn from(value: MissingLastBlockForCurrentEpoch) -> Self {
            Self::MissingLastBlockForCurrentEpoch(value)
        }
    }
    impl ::core::convert::From<NoChangeRequired> for LightClientErrors {
        fn from(value: NoChangeRequired) -> Self {
            Self::NoChangeRequired(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for LightClientErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<OutdatedState> for LightClientErrors {
        fn from(value: OutdatedState) -> Self {
            Self::OutdatedState(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for LightClientErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for LightClientErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<PermissionedProverNotSet> for LightClientErrors {
        fn from(value: PermissionedProverNotSet) -> Self {
            Self::PermissionedProverNotSet(value)
        }
    }
    impl ::core::convert::From<ProverNotPermissioned> for LightClientErrors {
        fn from(value: ProverNotPermissioned) -> Self {
            Self::ProverNotPermissioned(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext> for LightClientErrors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID> for LightClientErrors {
        fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
            Self::UUPSUnsupportedProxiableUUID(value)
        }
    }
    impl ::core::convert::From<UnsupportedDegree> for LightClientErrors {
        fn from(value: UnsupportedDegree) -> Self {
            Self::UnsupportedDegree(value)
        }
    }
    impl ::core::convert::From<WrongPlonkVK> for LightClientErrors {
        fn from(value: WrongPlonkVK) -> Self {
            Self::WrongPlonkVK(value)
        }
    }
    impl ::core::convert::From<WrongStakeTableUsed> for LightClientErrors {
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
    pub enum LightClientEvents {
        EpochChangedFilter(EpochChangedFilter),
        InitializedFilter(InitializedFilter),
        NewStateFilter(NewStateFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PermissionedProverNotRequiredFilter(PermissionedProverNotRequiredFilter),
        PermissionedProverRequiredFilter(PermissionedProverRequiredFilter),
        UpgradeFilter(UpgradeFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for LightClientEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EpochChangedFilter::decode_log(log) {
                return Ok(LightClientEvents::EpochChangedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(LightClientEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = NewStateFilter::decode_log(log) {
                return Ok(LightClientEvents::NewStateFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(LightClientEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PermissionedProverNotRequiredFilter::decode_log(log) {
                return Ok(LightClientEvents::PermissionedProverNotRequiredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PermissionedProverRequiredFilter::decode_log(log) {
                return Ok(LightClientEvents::PermissionedProverRequiredFilter(decoded));
            }
            if let Ok(decoded) = UpgradeFilter::decode_log(log) {
                return Ok(LightClientEvents::UpgradeFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(LightClientEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LightClientEvents {
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
    impl ::core::convert::From<EpochChangedFilter> for LightClientEvents {
        fn from(value: EpochChangedFilter) -> Self {
            Self::EpochChangedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for LightClientEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<NewStateFilter> for LightClientEvents {
        fn from(value: NewStateFilter) -> Self {
            Self::NewStateFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for LightClientEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PermissionedProverNotRequiredFilter> for LightClientEvents {
        fn from(value: PermissionedProverNotRequiredFilter) -> Self {
            Self::PermissionedProverNotRequiredFilter(value)
        }
    }
    impl ::core::convert::From<PermissionedProverRequiredFilter> for LightClientEvents {
        fn from(value: PermissionedProverRequiredFilter) -> Self {
            Self::PermissionedProverRequiredFilter(value)
        }
    }
    impl ::core::convert::From<UpgradeFilter> for LightClientEvents {
        fn from(value: UpgradeFilter) -> Self {
            Self::UpgradeFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for LightClientEvents {
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
    pub enum LightClientCalls {
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
        SetPermissionedProver(SetPermissionedProverCall),
        StateUpdateBlockNumbers(StateUpdateBlockNumbersCall),
        States(StatesCall),
        TransferOwnership(TransferOwnershipCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        VotingStakeTableCommitment(VotingStakeTableCommitmentCall),
        VotingThreshold(VotingThresholdCall),
    }
    impl ::ethers::core::abi::AbiDecode for LightClientCalls {
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
                <SetPermissionedProverCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPermissionedProver(decoded));
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
    impl ::ethers::core::abi::AbiEncode for LightClientCalls {
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
                Self::SetPermissionedProver(element) => {
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
    impl ::core::fmt::Display for LightClientCalls {
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
                Self::SetPermissionedProver(element) => ::core::fmt::Display::fmt(element, f),
                Self::StateUpdateBlockNumbers(element) => ::core::fmt::Display::fmt(element, f),
                Self::States(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotingStakeTableCommitment(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotingThreshold(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall> for LightClientCalls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<BlocksPerEpochCall> for LightClientCalls {
        fn from(value: BlocksPerEpochCall) -> Self {
            Self::BlocksPerEpoch(value)
        }
    }
    impl ::core::convert::From<ComputeStakeTableCommCall> for LightClientCalls {
        fn from(value: ComputeStakeTableCommCall) -> Self {
            Self::ComputeStakeTableComm(value)
        }
    }
    impl ::core::convert::From<CurrentEpochCall> for LightClientCalls {
        fn from(value: CurrentEpochCall) -> Self {
            Self::CurrentEpoch(value)
        }
    }
    impl ::core::convert::From<DisablePermissionedProverModeCall> for LightClientCalls {
        fn from(value: DisablePermissionedProverModeCall) -> Self {
            Self::DisablePermissionedProverMode(value)
        }
    }
    impl ::core::convert::From<FrozenStakeTableCommitmentCall> for LightClientCalls {
        fn from(value: FrozenStakeTableCommitmentCall) -> Self {
            Self::FrozenStakeTableCommitment(value)
        }
    }
    impl ::core::convert::From<FrozenThresholdCall> for LightClientCalls {
        fn from(value: FrozenThresholdCall) -> Self {
            Self::FrozenThreshold(value)
        }
    }
    impl ::core::convert::From<GetFinalizedStateCall> for LightClientCalls {
        fn from(value: GetFinalizedStateCall) -> Self {
            Self::GetFinalizedState(value)
        }
    }
    impl ::core::convert::From<GetGenesisStateCall> for LightClientCalls {
        fn from(value: GetGenesisStateCall) -> Self {
            Self::GetGenesisState(value)
        }
    }
    impl ::core::convert::From<GetHotShotBlockCommitmentsCountCall> for LightClientCalls {
        fn from(value: GetHotShotBlockCommitmentsCountCall) -> Self {
            Self::GetHotShotBlockCommitmentsCount(value)
        }
    }
    impl ::core::convert::From<GetHotShotCommitmentCall> for LightClientCalls {
        fn from(value: GetHotShotCommitmentCall) -> Self {
            Self::GetHotShotCommitment(value)
        }
    }
    impl ::core::convert::From<GetStateUpdateBlockNumbersCountCall> for LightClientCalls {
        fn from(value: GetStateUpdateBlockNumbersCountCall) -> Self {
            Self::GetStateUpdateBlockNumbersCount(value)
        }
    }
    impl ::core::convert::From<GetVersionCall> for LightClientCalls {
        fn from(value: GetVersionCall) -> Self {
            Self::GetVersion(value)
        }
    }
    impl ::core::convert::From<HotShotCommitmentsCall> for LightClientCalls {
        fn from(value: HotShotCommitmentsCall) -> Self {
            Self::HotShotCommitments(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for LightClientCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LagOverEscapeHatchThresholdCall> for LightClientCalls {
        fn from(value: LagOverEscapeHatchThresholdCall) -> Self {
            Self::LagOverEscapeHatchThreshold(value)
        }
    }
    impl ::core::convert::From<NewFinalizedStateCall> for LightClientCalls {
        fn from(value: NewFinalizedStateCall) -> Self {
            Self::NewFinalizedState(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LightClientCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PermissionedProverCall> for LightClientCalls {
        fn from(value: PermissionedProverCall) -> Self {
            Self::PermissionedProver(value)
        }
    }
    impl ::core::convert::From<PermissionedProverEnabledCall> for LightClientCalls {
        fn from(value: PermissionedProverEnabledCall) -> Self {
            Self::PermissionedProverEnabled(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for LightClientCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for LightClientCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetPermissionedProverCall> for LightClientCalls {
        fn from(value: SetPermissionedProverCall) -> Self {
            Self::SetPermissionedProver(value)
        }
    }
    impl ::core::convert::From<StateUpdateBlockNumbersCall> for LightClientCalls {
        fn from(value: StateUpdateBlockNumbersCall) -> Self {
            Self::StateUpdateBlockNumbers(value)
        }
    }
    impl ::core::convert::From<StatesCall> for LightClientCalls {
        fn from(value: StatesCall) -> Self {
            Self::States(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LightClientCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for LightClientCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<VotingStakeTableCommitmentCall> for LightClientCalls {
        fn from(value: VotingStakeTableCommitmentCall) -> Self {
            Self::VotingStakeTableCommitment(value)
        }
    }
    impl ::core::convert::From<VotingThresholdCall> for LightClientCalls {
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
