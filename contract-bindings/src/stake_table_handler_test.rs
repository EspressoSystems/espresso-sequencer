pub use stake_table_handler_test::*;
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
pub mod stake_table_handler_test {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_stakeTable"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract StakeTable"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_tokenCreator"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_token"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ExampleToken"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_lightClient"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract LightClientTest"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_TEST"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("IS_TEST"),
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
                    ::std::borrow::ToOwned::to_owned("advanceEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("advanceEpoch"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                name: ::std::borrow::ToOwned::to_owned("userIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("excludedArtifacts_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::String,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeContracts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("excludeContracts"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("excludedContracts_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeSenders"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("excludeSenders"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("excludedSenders_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exitEpochForBlsVK"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("exitEpochForBlsVK"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("blsKeyHash"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("exitEpoch"),
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
                    ::std::borrow::ToOwned::to_owned("failed"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("failed"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("lightClient"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lightClient"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract LightClientTest"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nextExitEpochBefore"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nextExitEpochBefore",),
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
                    ::std::borrow::ToOwned::to_owned("nextRegistrationEpochBefore"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nextRegistrationEpochBefore",),
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
                    ::std::borrow::ToOwned::to_owned("pendingExitsBefore"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingExitsBefore"),
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
                    ::std::borrow::ToOwned::to_owned("pendingRegistrationsBefore"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingRegistrationsBefore",),
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
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("registrationCalledAtLeastOnce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registrationCalledAtLeastOnce",),
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
                    ::std::borrow::ToOwned::to_owned("requestExit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestExit"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("rand"),
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
                    ::std::borrow::ToOwned::to_owned("requestExitCalledAtLeastOnce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestExitCalledAtLeastOnce",),
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
                    ::std::borrow::ToOwned::to_owned("requestExitKeysIndexes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestExitKeysIndexes",),
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
                    ::std::borrow::ToOwned::to_owned("stakeTable"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeTable"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract StakeTable"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stakeTableFirstAvailableExitEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeTableFirstAvailableExitEpoch",),
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
                    ::std::borrow::ToOwned::to_owned("stakeTableFirstAvailableRegistrationEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "stakeTableFirstAvailableRegistrationEpoch",
                        ),
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
                    ::std::borrow::ToOwned::to_owned("stakeTableNumPendingExits"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeTableNumPendingExits",),
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
                    ::std::borrow::ToOwned::to_owned("stakeTableNumPendingRegistrations"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeTableNumPendingRegistrations",),
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
                    ::std::borrow::ToOwned::to_owned("targetArtifactSelectors"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("targetArtifactSelectors",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("targetedArtifactSelectors_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                    4usize
                                                ),
                                            ),
                                        ),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct StdInvariant.FuzzSelector[]",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("targetedArtifacts_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::String,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetContracts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("targetContracts"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("targetedContracts_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetSelectors"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("targetSelectors"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("targetedSelectors_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                    4usize
                                                ),
                                            ),
                                        ),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct StdInvariant.FuzzSelector[]",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetSenders"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("targetSenders"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("targetedSenders_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("token"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("token"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract ExampleToken"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenCreator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenCreator"),
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
                    ::std::borrow::ToOwned::to_owned("users"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("users"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("vks"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("vks"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("vksWithdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("vksWithdraw"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawFunds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawFunds"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("rand"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_address"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_address"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                indexed: false,
                            },],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    ),
                                ),
                                indexed: false,
                            },],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                indexed: false,
                            },],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_bytes"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes32"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_bytes32"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_int"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_int"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_address"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_address"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_int"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_decimal_int",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("decimals"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_uint"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_decimal_uint",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("decimals"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_int"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_int"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_string"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_string"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_uint"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_uint"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_string"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_string"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_uint"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_uint"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("logs"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("logs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STAKETABLEHANDLERTEST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15b\0\0.W`\0\x80\xFD[P`@Qb\x002\t8\x03\x80b\x002\t\x839\x81\x01`@\x81\x90Rb\0\0Q\x91b\0\0\xD4V[`\x1B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x17\x90\x91U`\x1D\x80T\x82\x16\x93\x86\x16\x93\x90\x93\x17\x90\x92U`\x1C\x80T\x83\x16\x93\x85\x16\x93\x90\x93\x17\x90\x92U` \x80T\x90\x91\x16\x91\x90\x92\x16\x17\x90U`$\x80T`\xFF`\x80\x1B\x19\x16\x90U`#\x80T`\xFF\x19\x16\x90Ub\0\x01<V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xD1W`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\0\xEBW`\0\x80\xFD[\x84Qb\0\0\xF8\x81b\0\0\xBBV[` \x86\x01Q\x90\x94Pb\0\x01\x0B\x81b\0\0\xBBV[`@\x86\x01Q\x90\x93Pb\0\x01\x1E\x81b\0\0\xBBV[``\x86\x01Q\x90\x92Pb\0\x011\x81b\0\0\xBBV[\x93\x96\x92\x95P\x90\x93PPV[a0\xBD\x80b\0\x01L`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x11W`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\x01%W\x80c\xBC\xC4\xF0\xDD\x11a\0\xADW\x80c\xE3Y%R\x11a\0|W\x80c\xE3Y%R\x14a\x04\x9EW\x80c\xF5\x16Xc\x14a\x04\xF3W\x80c\xFA\x18/\xA1\x14a\x05\x06W\x80c\xFAv&\xD4\x14a\x05\x19W\x80c\xFC\x0CTj\x14a\x05&W`\0\x80\xFD[\x80c\xBC\xC4\xF0\xDD\x14a\x04VW\x80c\xC2;\xBA\xC5\x14a\x04iW\x80c\xDB\x84%,\x14a\x04\x83W\x80c\xE2\x0C\x9Fq\x14a\x04\x96W`\0\x80\xFD[\x80c\x92H\xDDD\x11a\0\xF4W\x80c\x92H\xDDD\x14a\x03\xFFW\x80c\x9601h\x14a\x04\x19W\x80c\xB5P\x8A\xA9\x14a\x043W\x80c\xB5p\x0Eh\x14a\x04;W\x80c\xBAAO\xA6\x14a\x04NW`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x03\xAEW\x80c\x88M\xA7}\x14a\x03\xC3W\x80c\x91j\x17\xC6\x14a\x03\xD6W\x80c\x91\x83\x14\x10\x14a\x03\xDEW`\0\x80\xFD[\x80c?r\x86\xF4\x11a\x01\xA8W\x80cp\xA2\xBC\xA5\x11a\x01wW\x80cp\xA2\xBC\xA5\x14a\x03=W\x80cr\x1Ce\x13\x14a\x03aW\x80cvg\x18\x08\x14a\x03tW\x80c}U.\xA6\x14a\x03\x8EW\x80c\x82>&?\x14a\x03\xA1W`\0\x80\xFD[\x80c?r\x86\xF4\x14a\x02\xF5W\x80cRu/\xCE\x14a\x02\xFDW\x80c\\\x05\x03G\x14a\x03\x15W\x80cf\xD9\xA9\xA0\x14a\x03(W`\0\x80\xFD[\x80c,\xB4\x8A\x9E\x11a\x01\xE4W\x80c,\xB4\x8A\x9E\x14a\x02\x91W\x80c6[\x98\xB2\x14a\x02\xBAW\x80c<\xF8\x0El\x14a\x02\xE5W\x80c>^<#\x14a\x02\xEDW`\0\x80\xFD[\x80c\x01v\xA3\xE4\x14a\x02\x16W\x80c\x15]\xD5\xEE\x14a\x02MW\x80c\x1E\xD7\x83\x1C\x14a\x02bW\x80c%\xA2\xC5\x9B\x14a\x02wW[`\0\x80\xFD[`\"Ta\x020\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02`a\x02[6`\x04a(\xA1V[a\x059V[\0[a\x02ja\x08\x8DV[`@Qa\x02D\x91\x90a(\xBAV[`#Ta\x020\x90`\x01`\x88\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x020a\x02\x9F6`\x04a(\xA1V[`%` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x02\xCDa\x02\xC86`\x04a(\xA1V[a\x08\xEFV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02DV[a\x02`a\t\x19V[a\x02ja\t\xF2V[a\x02ja\nRV[`#Ta\x020\x90a\x01\0\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x1BTa\x02\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x030a\n\xB2V[`@Qa\x02D\x91\x90a)\x07V[`$Ta\x03Q\x90`\x01`\x80\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02DV[a\x02`a\x03o6`\x04a(\xA1V[a\x0B\xA1V[`$Ta\x020\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x02`a\x03\x9C6`\x04a)\xD4V[a\x10\x94V[`#Ta\x03Q\x90`\xFF\x16\x81V[a\x03\xB6a\x14\xF1V[`@Qa\x02D\x91\x90a*TV[`$Ta\x020\x90`\x01`\x01`@\x1B\x03\x16\x81V[a\x030a\x15\xC1V[a\x03\xF1a\x03\xEC6`\x04a(\xA1V[a\x16\xA7V[`@Q\x90\x81R` \x01a\x02DV[`#Ta\x020\x90`\x01`H\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\"Ta\x020\x90`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x03\xB6a\x16\xC8V[` Ta\x02\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03Qa\x17\x98V[`\"Ta\x020\x90`\x01`\x01`@\x1B\x03\x16\x81V[`\"Ta\x020\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x1CTa\x02\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02ja\x18\xB7V[a\x04\xD3a\x04\xAC6`\x04a(\xA1V[`\x1E` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x92\x90\x91\x84V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x02DV[a\x02`a\x05\x016`\x04a*\xB8V[a\x19\x17V[a\x04\xD3a\x05\x146`\x04a(\xA1V[a\x1C}V[`\0Ta\x03Q\x90`\xFF\x16\x81V[`\x1DTa\x02\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`&T`\0\x03a\x05FWPV[`\0a\x05e\x82`\0`\x01`&\x80T\x90Pa\x05`\x91\x90a*\xEBV[a\x1C\xB7V[\x90P`\0`\x1E`\0`&\x84\x81T\x81\x10a\x05\x80Wa\x05\x80a+\x04V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T\x83R\x82\x81\x01\x93\x90\x93R`@\x91\x82\x01\x81 \x82Q`\x80\x81\x01\x84R\x81T\x81R`\x01\x82\x01T\x94\x81\x01\x94\x90\x94R`\x02\x81\x01T\x84\x84\x01R`\x03\x01T``\x84\x01R`\x1BT\x91QcM\x98R\xF3`\xE1\x1B\x81R\x92\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9B0\xA5\xE6\x90a\x05\xFB\x90\x85\x90`\x04\x01a+\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06<\x91\x90a+EV[`\0\x81\x81R`%` R`@\x81 T\x91\x92P`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`d\x90a\x06g\x82\x84a+^V[` T`@Qc9I\xD1\xE9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c9I\xD1\xE9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xB6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xCAW=`\0\x80>=`\0\xFD[PPPP`\0`&\x87\x81T\x81\x10a\x06\xE3Wa\x06\xE3a+\x04V[\x90`\0R` `\0 \x01T\x90P`\0`!\x82\x81T\x81\x10a\x07\x05Wa\x07\x05a+\x04V[`\0\x91\x82R` \x90\x91 \x01T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01\x81\x90R\x91P`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07eW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07yW=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\x01\x84\x95\xE3`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\x0C$\xAF\x18\x91Pa\x07\xAD\x90\x8A\x90`\x04\x01a+\x1AV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xF0\x91\x90a+~V[P`&\x80Ta\x08\x01\x90`\x01\x90a*\xEBV[\x81T\x81\x10a\x08\x11Wa\x08\x11a+\x04V[\x90`\0R` `\0 \x01T`&\x89\x81T\x81\x10a\x08/Wa\x08/a+\x04V[`\0\x91\x82R` \x90\x91 \x01U`&\x80T\x80a\x08LWa\x08La+\x9BV[`\0\x82\x81R` \x80\x82 \x83\x01`\0\x19\x90\x81\x01\x83\x90U\x90\x92\x01\x90\x92U\x96\x81R`%\x90\x96RPP`@\x90\x93 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90UPPPPPPV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xE5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\xC7W[PPPPP\x90P\x90V[`!\x81\x81T\x81\x10a\x08\xFFW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[` \x80T`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c9I\xD1\xE9\x92\x84\x92cvg\x18\x08\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\thW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x8C\x91\x90a+~V[a\t\x97\x90`\x01a+^V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xD8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xECW=`\0\x80>=`\0\xFD[PPPPV[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xE5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\xC7WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xE5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\xC7WPPPPP\x90P\x90V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\x98W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0B\x80W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0BBW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\n\xD6V[PPPP\x90P\x90V[`!T`\0\x03a\x0B\xAEWPV[`\0a\x0B\xC8\x82`\0`\x01`!\x80T\x90Pa\x05`\x91\x90a*\xEBV[\x90P`\0\x80a\x0B\xD6\x83a\x1C\xFBV[\x91P\x91P\x81`\x80\x01Q`\x01`\x01`@\x1B\x03\x16`\0\x14a\x0B\xF5WPPPPV[``\x82\x01Qa\x0C\x05\x90`\x01a+^V[`$T`\x01`\x01`@\x1B\x03\x91\x82\x16`\x01`@\x1B\x90\x91\x04\x90\x91\x16\x10\x15a\x0C*WPPPPV[`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cvg\x18\x08`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xA1\x91\x90a+~V[`$\x80T`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90U`\x1BT`@\x80Qc;\t\xC2g`\xE0\x1B\x81R\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c;\t\xC2g\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r=\x91\x90a+\xB1V[`#\x80Tp\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16`\x01`H\x1B`\x01`\x01`@\x1B\x03\x93\x84\x16\x02h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16\x17a\x01\0\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U`!\x80T`\0\x80Q` a0\x91\x839\x81Q\x91R\x91c\xCAf\x9F\xA7\x91\x86\x90\x81\x10a\r\xABWa\r\xABa+\x04V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xF8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x0CW=`\0\x80>=`\0\xFD[PP`\x1BT`@QcJ\xA7\xC2\x7F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92PcJ\xA7\xC2\x7F\x91Pa\x0E@\x90\x84\x90`\x04\x01a+\x1AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0EZW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0EnW=`\0\x80>=`\0\xFD[PPPP`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x10\x9E;\xE3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xE9\x91\x90a+~V[`#`\x11a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xD6{l\xA5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FbW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x86\x91\x90a+~V[`$\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\x1BT`@QcM\x98R\xF3`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B0\xA5\xE6\x90a\x0F\xD8\x90\x85\x90`\x04\x01a+\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x19\x91\x90a+EV[`\x80\x90\x93\x01Q`\0\x93\x84R`%` R`@\x84 \x80T`\x01`\x01`@\x1B\x03\x90\x92\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17\x90UPP`&\x80T`\x01\x81\x01\x82U\x91R\x7FtJ,\xF8\xFDp\x08\xE3\xD5;g\x91nsF\r\xF9\xFAR\x14\xE3\xEF#\xDDBY\xCA\tI:5\x94\x01UP`$\x80T`\xFF`\x80\x1B\x19\x16`\x01`\x80\x1B\x17\x90UV[`!T`\0\x03a\x10\xA2WPPV[a\x10\xBA\x82`\0`\x01`!\x80T\x90Pa\x05`\x91\x90a*\xEBV[\x91P`\0\x80a\x10\xC8\x84a\x1C\xFBV[\x91P\x91P\x81``\x01Q`\x01`\x01`@\x1B\x03\x16`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cvg\x18\x08`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11Q\x91\x90a+~V[`\x01`\x01`@\x1B\x03\x16\x11a\x11eWPPPPV[`\x80\x82\x01Q`\x01`\x01`@\x1B\x03\x16\x15a\x11~WPPPPV[a\x11\x94\x83`\x01`\x01`@\x1B\x03\x16`\x01`\na\x1C\xB7V[`\x1DT`\x1CT`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x95P`\x01`\x01`@\x1B\x03\x86\x16\x92\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x12\x91\x90a+EV[\x10\x15a\x12\x1EWPPPPV[`\x1CT`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12qW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\x85W=`\0\x80>=`\0\xFD[PP`\x1DT\x84Q`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93Pc\xA9\x05\x9C\xBB\x92Pa\x12\xBB\x91\x87\x90`\x04\x01a+\xE0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xFE\x91\x90a,\x02V[P\x81Q`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13QW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13eW=`\0\x80>=`\0\xFD[PP`\x1DT`\x1BT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\x13\xA0\x92\x90\x91\x16\x90\x87\x90`\x04\x01a+\xE0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x13\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xE3\x91\x90a,\x02V[P\x81Q`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x146W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14JW=`\0\x80>=`\0\xFD[PP`\x1BT`@\x80Qc\x1D\xC7\xDB\xD1`\xE2\x1B\x81R\x85Q`\x04\x82\x01R` \x86\x01Q`$\x82\x01R\x90\x85\x01Q`D\x82\x01R``\x85\x01Q`d\x82\x01R`\x01`\x01`@\x1B\x03\x87\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcw\x1FoD\x91P`\xA4\x01`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xE9\x91\x90a+\xB1V[PPPPPPV[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\x98W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x154\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15`\x90a,$V[\x80\x15a\x15\xADW\x80`\x1F\x10a\x15\x82Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xADV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x90W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x15\x15V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\x98W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x16\x8FW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x16QW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x15\xE5V[`&\x81\x81T\x81\x10a\x16\xB7W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\x98W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x17\x0B\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x177\x90a,$V[\x80\x15a\x17\x84W\x80`\x1F\x10a\x17YWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\x84V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17gW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x16\xECV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x17\xB8WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0`\0\x80Q` a0\x91\x839\x81Q\x91R;\x15a\x18\xB2W`@\x80Q`\0\x80Q` a0\x91\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x18:\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a,^V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x18T\x91a,\x8FV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x18\x91W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18\x96V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x18\xAE\x91\x90a,\x02V[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xE5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\xC7WPPPPP\x90P\x90V[`!T`@\x03a\x19$WPV[`!T`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90`\0\x80Q` a0\x91\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19\x9C\x91\x90\x81\x01\x90a-~V[`@Q` \x01a\x19\xAC\x91\x90a-\xCEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\x19\xC8\x82a\x1F\x08V[`@Qc\x18\xCA\xF8\xE3`\xE3\x1B\x81R\x90\x91P`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xC6W\xC7\x18\x90a\x19\xFE\x90\x84\x90\x86\x90`\x04\x01a-\xFAV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\x18W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A,W=`\0\x80>=`\0\xFD[PP`!\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x7F:cW\x01,\x1A:\xE0\xA1}0L\x99 1\x03\x82\xD9h\xEB\xCCK\x17q\xF4\x1Ck0B\x05\xB5p\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x91\x90\x91\x17\x90\x91U`\x1BT`@\x80Qc\x0B\x14\xC1a`\xE2\x1B\x81R\x81Q\x92\x90\x93\x16\x94Pc,S\x05\x84\x93P`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1A\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xE4\x91\x90a+\xB1V[`\"\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`@\x1B`\x01`\x01`@\x1B\x03\x93\x84\x16\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x92\x82\x16\x92\x90\x92\x17\x90\x91Ua\x1B/\x90\x84\x90\x86\x16a\x1F\x1AV[`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c,p\x12i`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xA6\x91\x90a+~V[`\"`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x16\xFE\xFE\xD7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CC\x91\x90a+~V[`\"\x80T`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UPP`#\x80T`\xFF\x19\x16`\x01\x17\x90UPPV[`\x1F\x81\x81T\x81\x10a\x1C\x8DW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x93P\x91\x90\x84V[`\0a\x1C\xC4\x84\x84\x84a\"HV[\x90Pa\x1C\xF4`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x14\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82a$\x10V[\x93\x92PPPV[a\x1D@`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\xA0\x82\x01R\x90V[a\x1Dk`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x83\x81R`\x1E` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x83\x83\x01R`\x03\x01T``\x83\x01R`\x1BT\x90QcM\x98R\xF3`\xE1\x1B\x81R\x91\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9B0\xA5\xE6\x90a\x1D\xD8\x90\x85\x90`\x04\x01a+\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x19\x91\x90a+EV[`\x1BT`@Qc\xD8ni}`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD8ni}\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1EpW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x94\x91\x90a.5V[\x95P\x95P\x95P\x95P\x95P\x95P`\0`@Q\x80`\xC0\x01`@R\x80\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\x01\x81\x11\x15a\x1E\xCEWa\x1E\xCEa.\xDBV[\x81R`\x01`\x01`@\x1B\x03\x96\x87\x16` \x82\x01R\x94\x86\x16`@\x86\x01R\x92\x90\x94\x16``\x84\x01R`\x80\x90\x92\x01\x91\x90\x91R\x98\x94\x97P\x93\x95PPPPPPV[`\0a\x1F\x13\x82a$\xAAV[P\x92\x91PPV[`\0\x82\x90P`\0`!\x84\x81T\x81\x10a\x1F4Wa\x1F4a+\x04V[`\0\x91\x82R` \x82 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x80\x80a\x1FV\x84\x86a%\xB4V[\x92P\x92P\x92P`\0a\x1Fk\x87`\x01`\na\x1C\xB7V[`\x1CT`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91Pb\x01\x86\xA0\x90`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xC6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F\xDAW=`\0\x80>=`\0\xFD[PP`\x1DT`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\xA9\x05\x9C\xBB\x91Pa \x10\x90\x89\x90\x86\x90`\x04\x01a+\xE0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a /W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a S\x91\x90a,\x02V[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \xA3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a \xB7W=`\0\x80>=`\0\xFD[PP`\x1DT`\x1BT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa \xF2\x92\x90\x91\x16\x90\x86\x90`\x04\x01a+\xE0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a!\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!5\x91\x90a,\x02V[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\x85W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\x99W=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xC7,\xC7\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\xC7,\xC7\x17\x91Pa!\xD8\x90\x88\x90\x88\x90\x87\x90`\0\x90\x8A\x90\x89\x90`\x04\x01a.\xF1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xF2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\"\x06W=`\0\x80>=`\0\xFD[PPP`\0\x99\x8ARPP`\x1E` \x90\x81R`@\x98\x89\x90 \x85Q\x81U\x90\x85\x01Q`\x01\x82\x01U\x97\x84\x01Q`\x02\x89\x01UPPP``\x01Q`\x03\x90\x94\x01\x93\x90\x93UPPPV[`\0\x81\x83\x11\x15a\"\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\"\xD4WP\x81\x84\x11\x15[\x15a\"\xE0WP\x82a\x1C\xF4V[`\0a\"\xEC\x84\x84a*\xEBV[a\"\xF7\x90`\x01a/\x85V[\x90P`\x03\x85\x11\x15\x80\x15a#\tWP\x84\x81\x11[\x15a# Wa#\x18\x85\x85a/\x85V[\x91PPa\x1C\xF4V[a#-`\x03`\0\x19a*\xEBV[\x85\x10\x15\x80\x15a#FWPa#C\x85`\0\x19a*\xEBV[\x81\x11[\x15a#aWa#W\x85`\0\x19a*\xEBV[a#\x18\x90\x84a*\xEBV[\x82\x85\x11\x15a#\xB7W`\0a#u\x84\x87a*\xEBV[\x90P`\0a#\x83\x83\x83a/\x98V[\x90P\x80`\0\x03a#\x98W\x84\x93PPPPa\x1C\xF4V[`\x01a#\xA4\x82\x88a/\x85V[a#\xAE\x91\x90a*\xEBV[\x93PPPa$\x08V[\x83\x85\x10\x15a$\x08W`\0a#\xCB\x86\x86a*\xEBV[\x90P`\0a#\xD9\x83\x83a/\x98V[\x90P\x80`\0\x03a#\xEEW\x85\x93PPPPa\x1C\xF4V[a#\xF8\x81\x86a*\xEBV[a$\x03\x90`\x01a/\x85V[\x93PPP[P\x93\x92PPPV[`\0jconsole.log`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Q`$\x01a$:\x92\x91\x90a/\xBAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90RQa$o\x91\x90a,\x8FV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x14\xE9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x14\xE9V[`\0\x80\x82`@Q` \x01a$\xBE\x91\x90a,\x8FV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01b^y\xB7`\xE0\x1B\x03\x19\x82R`\x04\x82\x01\x81\x90R\x91P`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xFF\xA1\x86I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%G\x91\x90a/\xDCV[`@Qc\x18\xCA\xF8\xE3`\xE3\x1B\x81R\x90\x92P`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xC6W\xC7\x18\x90a%}\x90\x85\x90\x87\x90`\x04\x01a-\xFAV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a%\x97W`\0\x80\xFD[PZ\xF1\x15\x80\x15a%\xABW=`\0\x80>=`\0\xFD[PPPP\x91P\x91V[a%\xDF`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x85R\x82\x81R\x80\x82\x01\x83\x90R\x84Q`\x04\x80\x82R`\xA0\x82\x01\x90\x96R\x93\x94\x90\x93\x91\x82\x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a&\x1BW\x90PP\x90P`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x19\x1AY\x99\x8B]\x19\\\xDD`\xBA\x1B\x81RP\x81`\0\x81Q\x81\x10a&gWa&ga+\x04V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x19\xD9[\x8BX\xDB\x1AY[\x9D\x0B]\xD8[\x1B\x19]`z\x1B\x81RP\x81`\x01\x81Q\x81\x10a&\xAFWa&\xAFa+\x04V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc+e1\x1F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90cV\xCAb>\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'4\x91\x90\x81\x01\x90a-~V[\x81`\x02\x81Q\x81\x10a'GWa'Ga+\x04V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'\xC6\x91\x90\x81\x01\x90a-~V[\x81`\x03\x81Q\x81\x10a'\xD9Wa'\xD9a+\x04V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x89\x16\x04g`\xE0\x1B\x81R`\0\x90`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\x89\x16\x04g\x90a(\x18\x90\x85\x90`\x04\x01a*TV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a(7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(_\x91\x90\x81\x01\x90a-~V[\x90P`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a({\x91\x90a/\xF7V[`@\x80Q\x80\x82\x01\x90\x91R\x91\x82R` \x82\x01R\x90\x9AP\x98P\x96PPPPPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a(\xB3W`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a(\xFBW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a(\xD6V[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a)\xADW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a)\x98W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a)nV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a)1V[P\x91\x99\x98PPPPPPPPPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a)\xD1W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a)\xE7W`\0\x80\xFD[\x825\x91P` \x83\x015a)\xF9\x81a)\xBCV[\x80\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a*\x1FW\x81\x81\x01Q\x83\x82\x01R` \x01a*\x07V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra*@\x81` \x86\x01` \x86\x01a*\x04V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a*\xABW`?\x19\x88\x86\x03\x01\x84Ra*\x99\x85\x83Qa*(V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a*}V[P\x92\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a*\xCAW`\0\x80\xFD[\x815a\x1C\xF4\x81a)\xBCV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a*\xFEWa*\xFEa*\xD5V[\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x81\x01a*\xFEV[`\0` \x82\x84\x03\x12\x15a+WW`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x1F\x13Wa\x1F\x13a*\xD5V[`\0` \x82\x84\x03\x12\x15a+\x90W`\0\x80\xFD[\x81Qa\x1C\xF4\x81a)\xBCV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a+\xC4W`\0\x80\xFD[\x82Qa+\xCF\x81a)\xBCV[` \x84\x01Q\x90\x92Pa)\xF9\x81a)\xBCV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01`@\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\0` \x82\x84\x03\x12\x15a,\x14W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1C\xF4W`\0\x80\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a,8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a,XWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a,\x81\x81`\x04\x85\x01` \x87\x01a*\x04V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa,\xA1\x81\x84` \x87\x01a*\x04V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a,\xE3Wa,\xE3a,\xABV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a,\xE3Wa,\xE3a,\xABV[`\0`\x01`\x01`@\x1B\x03\x80\x84\x11\x15a-%Wa-%a,\xABV[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a-MWa-Ma,\xABV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a-fW`\0\x80\xFD[a-t\x86` \x83\x01\x87a*\x04V[PPP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a-\x90W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xA6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a-\xB7W`\0\x80\xFD[a-\xC6\x84\x82Q` \x84\x01a-\x0BV[\x94\x93PPPPV[c:\xB9\xB2\xB9`\xE1\x1B\x81R`\0\x82Qa-\xED\x81`\x04\x85\x01` \x87\x01a*\x04V[\x91\x90\x91\x01`\x04\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a-\xC6\x90\x83\x01\x84a*(V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\xB2W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80\x86\x88\x03`\xE0\x81\x12\x15a.OW`\0\x80\xFD[a.X\x88a.\x1EV[\x96P` \x88\x01Q`\x02\x81\x10a.lW`\0\x80\xFD[`@\x89\x01Q\x90\x96Pa.}\x81a)\xBCV[``\x89\x01Q\x90\x95Pa.\x8E\x81a)\xBCV[`\x80\x89\x01Q\x90\x94Pa.\x9F\x81a)\xBCV[\x92P`@`\x9F\x19\x82\x01\x12\x15a.\xB3W`\0\x80\xFD[Pa.\xBCa,\xC1V[`\xA0\x88\x01Q\x81R`\xC0\x88\x01Q` \x82\x01R\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x86Q\x81R` \x80\x88\x01Q\x90\x82\x01R`@\x80\x88\x01Q\x90\x82\x01R``\x80\x88\x01Q\x90\x82\x01Ra\x01`\x81\x01\x86Q`\x80\x83\x01R` \x87\x01Q`\xA0\x83\x01R`\x01`\x01`@\x1B\x03\x80\x87\x16`\xC0\x84\x01R`\x02\x86\x10a/WWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x85`\xE0\x84\x01R\x84Qa\x01\0\x84\x01R` \x85\x01Qa\x01 \x84\x01R\x80\x84\x16a\x01@\x84\x01RP\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a*\xFEWa*\xFEa*\xD5V[`\0\x82a/\xB5WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`@\x81R`\0a/\xCD`@\x83\x01\x85a*(V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a/\xEEW`\0\x80\xFD[a\x1C\xF4\x82a.\x1EV[`\0\x80`\0\x80\x84\x86\x03a\x01\0\x81\x12\x15a0\x0FW`\0\x80\xFD[`@\x81\x12\x15a0\x1DW`\0\x80\xFD[a0%a,\xC1V[\x86Q\x81R` \x80\x88\x01Q\x90\x82\x01R\x94P`\x80`?\x19\x82\x01\x12\x15a0GW`\0\x80\xFD[Pa0Pa,\xE9V[`@\x86\x01Q\x81R``\x86\x01Q` \x82\x01R`\x80\x86\x01Q`@\x82\x01R`\xA0\x86\x01Q``\x82\x01R\x80\x93PP`\xC0\x85\x01Q\x91P`\xE0\x85\x01Q\x90P\x92\x95\x91\x94P\x92PV\xFE\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static STAKETABLEHANDLERTEST_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x11W`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\x01%W\x80c\xBC\xC4\xF0\xDD\x11a\0\xADW\x80c\xE3Y%R\x11a\0|W\x80c\xE3Y%R\x14a\x04\x9EW\x80c\xF5\x16Xc\x14a\x04\xF3W\x80c\xFA\x18/\xA1\x14a\x05\x06W\x80c\xFAv&\xD4\x14a\x05\x19W\x80c\xFC\x0CTj\x14a\x05&W`\0\x80\xFD[\x80c\xBC\xC4\xF0\xDD\x14a\x04VW\x80c\xC2;\xBA\xC5\x14a\x04iW\x80c\xDB\x84%,\x14a\x04\x83W\x80c\xE2\x0C\x9Fq\x14a\x04\x96W`\0\x80\xFD[\x80c\x92H\xDDD\x11a\0\xF4W\x80c\x92H\xDDD\x14a\x03\xFFW\x80c\x9601h\x14a\x04\x19W\x80c\xB5P\x8A\xA9\x14a\x043W\x80c\xB5p\x0Eh\x14a\x04;W\x80c\xBAAO\xA6\x14a\x04NW`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x03\xAEW\x80c\x88M\xA7}\x14a\x03\xC3W\x80c\x91j\x17\xC6\x14a\x03\xD6W\x80c\x91\x83\x14\x10\x14a\x03\xDEW`\0\x80\xFD[\x80c?r\x86\xF4\x11a\x01\xA8W\x80cp\xA2\xBC\xA5\x11a\x01wW\x80cp\xA2\xBC\xA5\x14a\x03=W\x80cr\x1Ce\x13\x14a\x03aW\x80cvg\x18\x08\x14a\x03tW\x80c}U.\xA6\x14a\x03\x8EW\x80c\x82>&?\x14a\x03\xA1W`\0\x80\xFD[\x80c?r\x86\xF4\x14a\x02\xF5W\x80cRu/\xCE\x14a\x02\xFDW\x80c\\\x05\x03G\x14a\x03\x15W\x80cf\xD9\xA9\xA0\x14a\x03(W`\0\x80\xFD[\x80c,\xB4\x8A\x9E\x11a\x01\xE4W\x80c,\xB4\x8A\x9E\x14a\x02\x91W\x80c6[\x98\xB2\x14a\x02\xBAW\x80c<\xF8\x0El\x14a\x02\xE5W\x80c>^<#\x14a\x02\xEDW`\0\x80\xFD[\x80c\x01v\xA3\xE4\x14a\x02\x16W\x80c\x15]\xD5\xEE\x14a\x02MW\x80c\x1E\xD7\x83\x1C\x14a\x02bW\x80c%\xA2\xC5\x9B\x14a\x02wW[`\0\x80\xFD[`\"Ta\x020\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02`a\x02[6`\x04a(\xA1V[a\x059V[\0[a\x02ja\x08\x8DV[`@Qa\x02D\x91\x90a(\xBAV[`#Ta\x020\x90`\x01`\x88\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x020a\x02\x9F6`\x04a(\xA1V[`%` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x02\xCDa\x02\xC86`\x04a(\xA1V[a\x08\xEFV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02DV[a\x02`a\t\x19V[a\x02ja\t\xF2V[a\x02ja\nRV[`#Ta\x020\x90a\x01\0\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x1BTa\x02\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x030a\n\xB2V[`@Qa\x02D\x91\x90a)\x07V[`$Ta\x03Q\x90`\x01`\x80\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02DV[a\x02`a\x03o6`\x04a(\xA1V[a\x0B\xA1V[`$Ta\x020\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x02`a\x03\x9C6`\x04a)\xD4V[a\x10\x94V[`#Ta\x03Q\x90`\xFF\x16\x81V[a\x03\xB6a\x14\xF1V[`@Qa\x02D\x91\x90a*TV[`$Ta\x020\x90`\x01`\x01`@\x1B\x03\x16\x81V[a\x030a\x15\xC1V[a\x03\xF1a\x03\xEC6`\x04a(\xA1V[a\x16\xA7V[`@Q\x90\x81R` \x01a\x02DV[`#Ta\x020\x90`\x01`H\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\"Ta\x020\x90`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x03\xB6a\x16\xC8V[` Ta\x02\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03Qa\x17\x98V[`\"Ta\x020\x90`\x01`\x01`@\x1B\x03\x16\x81V[`\"Ta\x020\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x1CTa\x02\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02ja\x18\xB7V[a\x04\xD3a\x04\xAC6`\x04a(\xA1V[`\x1E` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x92\x90\x91\x84V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x02DV[a\x02`a\x05\x016`\x04a*\xB8V[a\x19\x17V[a\x04\xD3a\x05\x146`\x04a(\xA1V[a\x1C}V[`\0Ta\x03Q\x90`\xFF\x16\x81V[`\x1DTa\x02\xCD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`&T`\0\x03a\x05FWPV[`\0a\x05e\x82`\0`\x01`&\x80T\x90Pa\x05`\x91\x90a*\xEBV[a\x1C\xB7V[\x90P`\0`\x1E`\0`&\x84\x81T\x81\x10a\x05\x80Wa\x05\x80a+\x04V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T\x83R\x82\x81\x01\x93\x90\x93R`@\x91\x82\x01\x81 \x82Q`\x80\x81\x01\x84R\x81T\x81R`\x01\x82\x01T\x94\x81\x01\x94\x90\x94R`\x02\x81\x01T\x84\x84\x01R`\x03\x01T``\x84\x01R`\x1BT\x91QcM\x98R\xF3`\xE1\x1B\x81R\x92\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9B0\xA5\xE6\x90a\x05\xFB\x90\x85\x90`\x04\x01a+\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06<\x91\x90a+EV[`\0\x81\x81R`%` R`@\x81 T\x91\x92P`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`d\x90a\x06g\x82\x84a+^V[` T`@Qc9I\xD1\xE9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c9I\xD1\xE9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xB6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xCAW=`\0\x80>=`\0\xFD[PPPP`\0`&\x87\x81T\x81\x10a\x06\xE3Wa\x06\xE3a+\x04V[\x90`\0R` `\0 \x01T\x90P`\0`!\x82\x81T\x81\x10a\x07\x05Wa\x07\x05a+\x04V[`\0\x91\x82R` \x90\x91 \x01T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01\x81\x90R\x91P`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07eW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07yW=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\x01\x84\x95\xE3`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\x0C$\xAF\x18\x91Pa\x07\xAD\x90\x8A\x90`\x04\x01a+\x1AV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xF0\x91\x90a+~V[P`&\x80Ta\x08\x01\x90`\x01\x90a*\xEBV[\x81T\x81\x10a\x08\x11Wa\x08\x11a+\x04V[\x90`\0R` `\0 \x01T`&\x89\x81T\x81\x10a\x08/Wa\x08/a+\x04V[`\0\x91\x82R` \x90\x91 \x01U`&\x80T\x80a\x08LWa\x08La+\x9BV[`\0\x82\x81R` \x80\x82 \x83\x01`\0\x19\x90\x81\x01\x83\x90U\x90\x92\x01\x90\x92U\x96\x81R`%\x90\x96RPP`@\x90\x93 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90UPPPPPPV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xE5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\xC7W[PPPPP\x90P\x90V[`!\x81\x81T\x81\x10a\x08\xFFW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[` \x80T`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c9I\xD1\xE9\x92\x84\x92cvg\x18\x08\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\thW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x8C\x91\x90a+~V[a\t\x97\x90`\x01a+^V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xD8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xECW=`\0\x80>=`\0\xFD[PPPPV[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xE5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\xC7WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xE5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\xC7WPPPPP\x90P\x90V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\x98W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0B\x80W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0BBW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\n\xD6V[PPPP\x90P\x90V[`!T`\0\x03a\x0B\xAEWPV[`\0a\x0B\xC8\x82`\0`\x01`!\x80T\x90Pa\x05`\x91\x90a*\xEBV[\x90P`\0\x80a\x0B\xD6\x83a\x1C\xFBV[\x91P\x91P\x81`\x80\x01Q`\x01`\x01`@\x1B\x03\x16`\0\x14a\x0B\xF5WPPPPV[``\x82\x01Qa\x0C\x05\x90`\x01a+^V[`$T`\x01`\x01`@\x1B\x03\x91\x82\x16`\x01`@\x1B\x90\x91\x04\x90\x91\x16\x10\x15a\x0C*WPPPPV[`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cvg\x18\x08`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xA1\x91\x90a+~V[`$\x80T`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90U`\x1BT`@\x80Qc;\t\xC2g`\xE0\x1B\x81R\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c;\t\xC2g\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\r\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r=\x91\x90a+\xB1V[`#\x80Tp\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16`\x01`H\x1B`\x01`\x01`@\x1B\x03\x93\x84\x16\x02h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16\x17a\x01\0\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U`!\x80T`\0\x80Q` a0\x91\x839\x81Q\x91R\x91c\xCAf\x9F\xA7\x91\x86\x90\x81\x10a\r\xABWa\r\xABa+\x04V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xF8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x0CW=`\0\x80>=`\0\xFD[PP`\x1BT`@QcJ\xA7\xC2\x7F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92PcJ\xA7\xC2\x7F\x91Pa\x0E@\x90\x84\x90`\x04\x01a+\x1AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0EZW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0EnW=`\0\x80>=`\0\xFD[PPPP`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x10\x9E;\xE3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xE9\x91\x90a+~V[`#`\x11a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xD6{l\xA5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FbW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x86\x91\x90a+~V[`$\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\x1BT`@QcM\x98R\xF3`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B0\xA5\xE6\x90a\x0F\xD8\x90\x85\x90`\x04\x01a+\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x19\x91\x90a+EV[`\x80\x90\x93\x01Q`\0\x93\x84R`%` R`@\x84 \x80T`\x01`\x01`@\x1B\x03\x90\x92\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17\x90UPP`&\x80T`\x01\x81\x01\x82U\x91R\x7FtJ,\xF8\xFDp\x08\xE3\xD5;g\x91nsF\r\xF9\xFAR\x14\xE3\xEF#\xDDBY\xCA\tI:5\x94\x01UP`$\x80T`\xFF`\x80\x1B\x19\x16`\x01`\x80\x1B\x17\x90UV[`!T`\0\x03a\x10\xA2WPPV[a\x10\xBA\x82`\0`\x01`!\x80T\x90Pa\x05`\x91\x90a*\xEBV[\x91P`\0\x80a\x10\xC8\x84a\x1C\xFBV[\x91P\x91P\x81``\x01Q`\x01`\x01`@\x1B\x03\x16`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cvg\x18\x08`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11Q\x91\x90a+~V[`\x01`\x01`@\x1B\x03\x16\x11a\x11eWPPPPV[`\x80\x82\x01Q`\x01`\x01`@\x1B\x03\x16\x15a\x11~WPPPPV[a\x11\x94\x83`\x01`\x01`@\x1B\x03\x16`\x01`\na\x1C\xB7V[`\x1DT`\x1CT`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x95P`\x01`\x01`@\x1B\x03\x86\x16\x92\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x12\x91\x90a+EV[\x10\x15a\x12\x1EWPPPPV[`\x1CT`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12qW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\x85W=`\0\x80>=`\0\xFD[PP`\x1DT\x84Q`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93Pc\xA9\x05\x9C\xBB\x92Pa\x12\xBB\x91\x87\x90`\x04\x01a+\xE0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xFE\x91\x90a,\x02V[P\x81Q`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13QW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13eW=`\0\x80>=`\0\xFD[PP`\x1DT`\x1BT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\x13\xA0\x92\x90\x91\x16\x90\x87\x90`\x04\x01a+\xE0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x13\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xE3\x91\x90a,\x02V[P\x81Q`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x146W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14JW=`\0\x80>=`\0\xFD[PP`\x1BT`@\x80Qc\x1D\xC7\xDB\xD1`\xE2\x1B\x81R\x85Q`\x04\x82\x01R` \x86\x01Q`$\x82\x01R\x90\x85\x01Q`D\x82\x01R``\x85\x01Q`d\x82\x01R`\x01`\x01`@\x1B\x03\x87\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcw\x1FoD\x91P`\xA4\x01`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xE9\x91\x90a+\xB1V[PPPPPPV[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\x98W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x154\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15`\x90a,$V[\x80\x15a\x15\xADW\x80`\x1F\x10a\x15\x82Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xADV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x90W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x15\x15V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\x98W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x16\x8FW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x16QW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x15\xE5V[`&\x81\x81T\x81\x10a\x16\xB7W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\x98W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x17\x0B\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x177\x90a,$V[\x80\x15a\x17\x84W\x80`\x1F\x10a\x17YWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\x84V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17gW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x16\xECV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x17\xB8WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0`\0\x80Q` a0\x91\x839\x81Q\x91R;\x15a\x18\xB2W`@\x80Q`\0\x80Q` a0\x91\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x18:\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a,^V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x18T\x91a,\x8FV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x18\x91W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18\x96V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x18\xAE\x91\x90a,\x02V[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xE5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\xC7WPPPPP\x90P\x90V[`!T`@\x03a\x19$WPV[`!T`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90`\0\x80Q` a0\x91\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19\x9C\x91\x90\x81\x01\x90a-~V[`@Q` \x01a\x19\xAC\x91\x90a-\xCEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\x19\xC8\x82a\x1F\x08V[`@Qc\x18\xCA\xF8\xE3`\xE3\x1B\x81R\x90\x91P`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xC6W\xC7\x18\x90a\x19\xFE\x90\x84\x90\x86\x90`\x04\x01a-\xFAV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\x18W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A,W=`\0\x80>=`\0\xFD[PP`!\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x7F:cW\x01,\x1A:\xE0\xA1}0L\x99 1\x03\x82\xD9h\xEB\xCCK\x17q\xF4\x1Ck0B\x05\xB5p\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x91\x90\x91\x17\x90\x91U`\x1BT`@\x80Qc\x0B\x14\xC1a`\xE2\x1B\x81R\x81Q\x92\x90\x93\x16\x94Pc,S\x05\x84\x93P`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1A\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xE4\x91\x90a+\xB1V[`\"\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`@\x1B`\x01`\x01`@\x1B\x03\x93\x84\x16\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x92\x82\x16\x92\x90\x92\x17\x90\x91Ua\x1B/\x90\x84\x90\x86\x16a\x1F\x1AV[`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c,p\x12i`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xA6\x91\x90a+~V[`\"`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x16\xFE\xFE\xD7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CC\x91\x90a+~V[`\"\x80T`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UPP`#\x80T`\xFF\x19\x16`\x01\x17\x90UPPV[`\x1F\x81\x81T\x81\x10a\x1C\x8DW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x93P\x91\x90\x84V[`\0a\x1C\xC4\x84\x84\x84a\"HV[\x90Pa\x1C\xF4`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x14\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82a$\x10V[\x93\x92PPPV[a\x1D@`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90`\xA0\x82\x01R\x90V[a\x1Dk`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x83\x81R`\x1E` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x83\x83\x01R`\x03\x01T``\x83\x01R`\x1BT\x90QcM\x98R\xF3`\xE1\x1B\x81R\x91\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9B0\xA5\xE6\x90a\x1D\xD8\x90\x85\x90`\x04\x01a+\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x19\x91\x90a+EV[`\x1BT`@Qc\xD8ni}`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD8ni}\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1EpW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x94\x91\x90a.5V[\x95P\x95P\x95P\x95P\x95P\x95P`\0`@Q\x80`\xC0\x01`@R\x80\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\x01\x81\x11\x15a\x1E\xCEWa\x1E\xCEa.\xDBV[\x81R`\x01`\x01`@\x1B\x03\x96\x87\x16` \x82\x01R\x94\x86\x16`@\x86\x01R\x92\x90\x94\x16``\x84\x01R`\x80\x90\x92\x01\x91\x90\x91R\x98\x94\x97P\x93\x95PPPPPPV[`\0a\x1F\x13\x82a$\xAAV[P\x92\x91PPV[`\0\x82\x90P`\0`!\x84\x81T\x81\x10a\x1F4Wa\x1F4a+\x04V[`\0\x91\x82R` \x82 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x80\x80a\x1FV\x84\x86a%\xB4V[\x92P\x92P\x92P`\0a\x1Fk\x87`\x01`\na\x1C\xB7V[`\x1CT`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91Pb\x01\x86\xA0\x90`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xC6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F\xDAW=`\0\x80>=`\0\xFD[PP`\x1DT`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\xA9\x05\x9C\xBB\x91Pa \x10\x90\x89\x90\x86\x90`\x04\x01a+\xE0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a /W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a S\x91\x90a,\x02V[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \xA3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a \xB7W=`\0\x80>=`\0\xFD[PP`\x1DT`\x1BT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa \xF2\x92\x90\x91\x16\x90\x86\x90`\x04\x01a+\xE0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a!\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!5\x91\x90a,\x02V[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\x85W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\x99W=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xC7,\xC7\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\xC7,\xC7\x17\x91Pa!\xD8\x90\x88\x90\x88\x90\x87\x90`\0\x90\x8A\x90\x89\x90`\x04\x01a.\xF1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xF2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\"\x06W=`\0\x80>=`\0\xFD[PPP`\0\x99\x8ARPP`\x1E` \x90\x81R`@\x98\x89\x90 \x85Q\x81U\x90\x85\x01Q`\x01\x82\x01U\x97\x84\x01Q`\x02\x89\x01UPPP``\x01Q`\x03\x90\x94\x01\x93\x90\x93UPPPV[`\0\x81\x83\x11\x15a\"\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\"\xD4WP\x81\x84\x11\x15[\x15a\"\xE0WP\x82a\x1C\xF4V[`\0a\"\xEC\x84\x84a*\xEBV[a\"\xF7\x90`\x01a/\x85V[\x90P`\x03\x85\x11\x15\x80\x15a#\tWP\x84\x81\x11[\x15a# Wa#\x18\x85\x85a/\x85V[\x91PPa\x1C\xF4V[a#-`\x03`\0\x19a*\xEBV[\x85\x10\x15\x80\x15a#FWPa#C\x85`\0\x19a*\xEBV[\x81\x11[\x15a#aWa#W\x85`\0\x19a*\xEBV[a#\x18\x90\x84a*\xEBV[\x82\x85\x11\x15a#\xB7W`\0a#u\x84\x87a*\xEBV[\x90P`\0a#\x83\x83\x83a/\x98V[\x90P\x80`\0\x03a#\x98W\x84\x93PPPPa\x1C\xF4V[`\x01a#\xA4\x82\x88a/\x85V[a#\xAE\x91\x90a*\xEBV[\x93PPPa$\x08V[\x83\x85\x10\x15a$\x08W`\0a#\xCB\x86\x86a*\xEBV[\x90P`\0a#\xD9\x83\x83a/\x98V[\x90P\x80`\0\x03a#\xEEW\x85\x93PPPPa\x1C\xF4V[a#\xF8\x81\x86a*\xEBV[a$\x03\x90`\x01a/\x85V[\x93PPP[P\x93\x92PPPV[`\0jconsole.log`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Q`$\x01a$:\x92\x91\x90a/\xBAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90RQa$o\x91\x90a,\x8FV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x14\xE9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x14\xE9V[`\0\x80\x82`@Q` \x01a$\xBE\x91\x90a,\x8FV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01b^y\xB7`\xE0\x1B\x03\x19\x82R`\x04\x82\x01\x81\x90R\x91P`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xFF\xA1\x86I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%G\x91\x90a/\xDCV[`@Qc\x18\xCA\xF8\xE3`\xE3\x1B\x81R\x90\x92P`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\xC6W\xC7\x18\x90a%}\x90\x85\x90\x87\x90`\x04\x01a-\xFAV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a%\x97W`\0\x80\xFD[PZ\xF1\x15\x80\x15a%\xABW=`\0\x80>=`\0\xFD[PPPP\x91P\x91V[a%\xDF`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x85R\x82\x81R\x80\x82\x01\x83\x90R\x84Q`\x04\x80\x82R`\xA0\x82\x01\x90\x96R\x93\x94\x90\x93\x91\x82\x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a&\x1BW\x90PP\x90P`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x19\x1AY\x99\x8B]\x19\\\xDD`\xBA\x1B\x81RP\x81`\0\x81Q\x81\x10a&gWa&ga+\x04V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x19\xD9[\x8BX\xDB\x1AY[\x9D\x0B]\xD8[\x1B\x19]`z\x1B\x81RP\x81`\x01\x81Q\x81\x10a&\xAFWa&\xAFa+\x04V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc+e1\x1F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90cV\xCAb>\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'4\x91\x90\x81\x01\x90a-~V[\x81`\x02\x81Q\x81\x10a'GWa'Ga+\x04V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R`\0\x80Q` a0\x91\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'\xC6\x91\x90\x81\x01\x90a-~V[\x81`\x03\x81Q\x81\x10a'\xD9Wa'\xD9a+\x04V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x89\x16\x04g`\xE0\x1B\x81R`\0\x90`\0\x80Q` a0\x91\x839\x81Q\x91R\x90c\x89\x16\x04g\x90a(\x18\x90\x85\x90`\x04\x01a*TV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a(7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(_\x91\x90\x81\x01\x90a-~V[\x90P`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a({\x91\x90a/\xF7V[`@\x80Q\x80\x82\x01\x90\x91R\x91\x82R` \x82\x01R\x90\x9AP\x98P\x96PPPPPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a(\xB3W`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a(\xFBW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a(\xD6V[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a)\xADW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a)\x98W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a)nV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a)1V[P\x91\x99\x98PPPPPPPPPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a)\xD1W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a)\xE7W`\0\x80\xFD[\x825\x91P` \x83\x015a)\xF9\x81a)\xBCV[\x80\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a*\x1FW\x81\x81\x01Q\x83\x82\x01R` \x01a*\x07V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra*@\x81` \x86\x01` \x86\x01a*\x04V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a*\xABW`?\x19\x88\x86\x03\x01\x84Ra*\x99\x85\x83Qa*(V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a*}V[P\x92\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a*\xCAW`\0\x80\xFD[\x815a\x1C\xF4\x81a)\xBCV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a*\xFEWa*\xFEa*\xD5V[\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x81\x01a*\xFEV[`\0` \x82\x84\x03\x12\x15a+WW`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x1F\x13Wa\x1F\x13a*\xD5V[`\0` \x82\x84\x03\x12\x15a+\x90W`\0\x80\xFD[\x81Qa\x1C\xF4\x81a)\xBCV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a+\xC4W`\0\x80\xFD[\x82Qa+\xCF\x81a)\xBCV[` \x84\x01Q\x90\x92Pa)\xF9\x81a)\xBCV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01`@\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\0` \x82\x84\x03\x12\x15a,\x14W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1C\xF4W`\0\x80\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a,8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a,XWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a,\x81\x81`\x04\x85\x01` \x87\x01a*\x04V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa,\xA1\x81\x84` \x87\x01a*\x04V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a,\xE3Wa,\xE3a,\xABV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a,\xE3Wa,\xE3a,\xABV[`\0`\x01`\x01`@\x1B\x03\x80\x84\x11\x15a-%Wa-%a,\xABV[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a-MWa-Ma,\xABV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a-fW`\0\x80\xFD[a-t\x86` \x83\x01\x87a*\x04V[PPP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a-\x90W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xA6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a-\xB7W`\0\x80\xFD[a-\xC6\x84\x82Q` \x84\x01a-\x0BV[\x94\x93PPPPV[c:\xB9\xB2\xB9`\xE1\x1B\x81R`\0\x82Qa-\xED\x81`\x04\x85\x01` \x87\x01a*\x04V[\x91\x90\x91\x01`\x04\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a-\xC6\x90\x83\x01\x84a*(V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\xB2W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80\x86\x88\x03`\xE0\x81\x12\x15a.OW`\0\x80\xFD[a.X\x88a.\x1EV[\x96P` \x88\x01Q`\x02\x81\x10a.lW`\0\x80\xFD[`@\x89\x01Q\x90\x96Pa.}\x81a)\xBCV[``\x89\x01Q\x90\x95Pa.\x8E\x81a)\xBCV[`\x80\x89\x01Q\x90\x94Pa.\x9F\x81a)\xBCV[\x92P`@`\x9F\x19\x82\x01\x12\x15a.\xB3W`\0\x80\xFD[Pa.\xBCa,\xC1V[`\xA0\x88\x01Q\x81R`\xC0\x88\x01Q` \x82\x01R\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x86Q\x81R` \x80\x88\x01Q\x90\x82\x01R`@\x80\x88\x01Q\x90\x82\x01R``\x80\x88\x01Q\x90\x82\x01Ra\x01`\x81\x01\x86Q`\x80\x83\x01R` \x87\x01Q`\xA0\x83\x01R`\x01`\x01`@\x1B\x03\x80\x87\x16`\xC0\x84\x01R`\x02\x86\x10a/WWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x85`\xE0\x84\x01R\x84Qa\x01\0\x84\x01R` \x85\x01Qa\x01 \x84\x01R\x80\x84\x16a\x01@\x84\x01RP\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a*\xFEWa*\xFEa*\xD5V[`\0\x82a/\xB5WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`@\x81R`\0a/\xCD`@\x83\x01\x85a*(V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a/\xEEW`\0\x80\xFD[a\x1C\xF4\x82a.\x1EV[`\0\x80`\0\x80\x84\x86\x03a\x01\0\x81\x12\x15a0\x0FW`\0\x80\xFD[`@\x81\x12\x15a0\x1DW`\0\x80\xFD[a0%a,\xC1V[\x86Q\x81R` \x80\x88\x01Q\x90\x82\x01R\x94P`\x80`?\x19\x82\x01\x12\x15a0GW`\0\x80\xFD[Pa0Pa,\xE9V[`@\x86\x01Q\x81R``\x86\x01Q` \x82\x01R`\x80\x86\x01Q`@\x82\x01R`\xA0\x86\x01Q``\x82\x01R\x80\x93PP`\xC0\x85\x01Q\x91P`\xE0\x85\x01Q\x90P\x92\x95\x91\x94P\x92PV\xFE\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static STAKETABLEHANDLERTEST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct StakeTableHandlerTest<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StakeTableHandlerTest<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StakeTableHandlerTest<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StakeTableHandlerTest<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StakeTableHandlerTest<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StakeTableHandlerTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StakeTableHandlerTest<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                STAKETABLEHANDLERTEST_ABI.clone(),
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
                STAKETABLEHANDLERTEST_ABI.clone(),
                STAKETABLEHANDLERTEST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `IS_TEST` (0xfa7626d4) function
        pub fn is_test(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `advanceEpoch` (0x3cf80e6c) function
        pub fn advance_epoch(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 248, 14, 108], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentEpoch` (0x76671808) function
        pub fn current_epoch(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([118, 103, 24, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x7d552ea6) function
        pub fn deposit(
            &self,
            user_index: ::ethers::core::types::U256,
            amount: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([125, 85, 46, 166], (user_index, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeArtifacts` (0xb5508aa9) function
        pub fn exclude_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<::std::string::String>>
        {
            self.0
                .method_hash([181, 80, 138, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeContracts` (0xe20c9f71) function
        pub fn exclude_contracts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([226, 12, 159, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeSenders` (0x1ed7831c) function
        pub fn exclude_senders(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([30, 215, 131, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exitEpochForBlsVK` (0x2cb48a9e) function
        pub fn exit_epoch_for_bls_vk(
            &self,
            bls_key_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([44, 180, 138, 158], bls_key_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failed` (0xba414fa6) function
        pub fn failed(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
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
        ///Calls the contract's `nextExitEpochBefore` (0x52752fce) function
        pub fn next_exit_epoch_before(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([82, 117, 47, 206], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nextRegistrationEpochBefore` (0xbcc4f0dd) function
        pub fn next_registration_epoch_before(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([188, 196, 240, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingExitsBefore` (0x9248dd44) function
        pub fn pending_exits_before(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([146, 72, 221, 68], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingRegistrationsBefore` (0xc23bbac5) function
        pub fn pending_registrations_before(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([194, 59, 186, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `register` (0xf5165863) function
        pub fn register(&self, amount: u64) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 22, 88, 99], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registrationCalledAtLeastOnce` (0x823e263f) function
        pub fn registration_called_at_least_once(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([130, 62, 38, 63], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestExit` (0x721c6513) function
        pub fn request_exit(
            &self,
            rand: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 28, 101, 19], rand)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestExitCalledAtLeastOnce` (0x70a2bca5) function
        pub fn request_exit_called_at_least_once(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([112, 162, 188, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestExitKeysIndexes` (0x91831410) function
        pub fn request_exit_keys_indexes(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([145, 131, 20, 16], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeTable` (0x5c050347) function
        pub fn stake_table(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([92, 5, 3, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeTableFirstAvailableExitEpoch` (0x25a2c59b) function
        pub fn stake_table_first_available_exit_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([37, 162, 197, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeTableFirstAvailableRegistrationEpoch` (0x0176a3e4) function
        pub fn stake_table_first_available_registration_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([1, 118, 163, 228], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeTableNumPendingExits` (0x884da77d) function
        pub fn stake_table_num_pending_exits(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([136, 77, 167, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeTableNumPendingRegistrations` (0x96303168) function
        pub fn stake_table_num_pending_registrations(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([150, 48, 49, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetArtifactSelectors` (0x66d9a9a0) function
        pub fn target_artifact_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<FuzzSelector>> {
            self.0
                .method_hash([102, 217, 169, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetArtifacts` (0x85226c81) function
        pub fn target_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<::std::string::String>>
        {
            self.0
                .method_hash([133, 34, 108, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetContracts` (0x3f7286f4) function
        pub fn target_contracts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([63, 114, 134, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetSelectors` (0x916a17c6) function
        pub fn target_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<FuzzSelector>> {
            self.0
                .method_hash([145, 106, 23, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetSenders` (0x3e5e3c23) function
        pub fn target_senders(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([62, 94, 60, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token` (0xfc0c546a) function
        pub fn token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenCreator` (0xdb84252c) function
        pub fn token_creator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([219, 132, 37, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `users` (0x365b98b2) function
        pub fn users(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([54, 91, 152, 178], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vks` (0xe3592552) function
        pub fn vks(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([227, 89, 37, 82], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vksWithdraw` (0xfa182fa1) function
        pub fn vks_withdraw(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([250, 24, 47, 161], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawFunds` (0x155dd5ee) function
        pub fn withdraw_funds(
            &self,
            rand: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 93, 213, 238], rand)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `log` event
        pub fn log_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_address` event
        pub fn log_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogAddressFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogArray1Filter> {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogArray2Filter> {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogArray3Filter> {
            self.0.event()
        }
        ///Gets the contract's `log_bytes` event
        pub fn log_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogBytesFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_bytes32` event
        pub fn log_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogBytes32Filter> {
            self.0.event()
        }
        ///Gets the contract's `log_int` event
        pub fn log_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogIntFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_named_address` event
        pub fn log_named_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedAddressFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedArray1Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedArray2Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedArray3Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes` event
        pub fn log_named_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedBytesFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes32` event
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedBytes32Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_int` event
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedDecimalIntFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_uint` event
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedDecimalUintFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_int` event
        pub fn log_named_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedIntFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_string` event
        pub fn log_named_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedStringFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_uint` event
        pub fn log_named_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedUintFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_string` event
        pub fn log_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogStringFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_uint` event
        pub fn log_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogUintFilter> {
            self.0.event()
        }
        ///Gets the contract's `logs` event
        pub fn logs_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogsFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeTableHandlerTestEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for StakeTableHandlerTest<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(name = "log", abi = "log(string)")]
    pub struct LogFilter(pub ::std::string::String);
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
    #[ethevent(name = "log_address", abi = "log_address(address)")]
    pub struct LogAddressFilter(pub ::ethers::core::types::Address);
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
    #[ethevent(name = "log_array", abi = "log_array(uint256[])")]
    pub struct LogArray1Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
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
    #[ethevent(name = "log_array", abi = "log_array(int256[])")]
    pub struct LogArray2Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
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
    #[ethevent(name = "log_array", abi = "log_array(address[])")]
    pub struct LogArray3Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
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
    #[ethevent(name = "log_bytes", abi = "log_bytes(bytes)")]
    pub struct LogBytesFilter(pub ::ethers::core::types::Bytes);
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
    #[ethevent(name = "log_bytes32", abi = "log_bytes32(bytes32)")]
    pub struct LogBytes32Filter(pub [u8; 32]);
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
    #[ethevent(name = "log_int", abi = "log_int(int256)")]
    pub struct LogIntFilter(pub ::ethers::core::types::I256);
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
    #[ethevent(name = "log_named_address", abi = "log_named_address(string,address)")]
    pub struct LogNamedAddressFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Address,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,uint256[])")]
    pub struct LogNamedArray1Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,int256[])")]
    pub struct LogNamedArray2Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,address[])")]
    pub struct LogNamedArray3Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
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
    #[ethevent(name = "log_named_bytes", abi = "log_named_bytes(string,bytes)")]
    pub struct LogNamedBytesFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "log_named_bytes32", abi = "log_named_bytes32(string,bytes32)")]
    pub struct LogNamedBytes32Filter {
        pub key: ::std::string::String,
        pub val: [u8; 32],
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
        name = "log_named_decimal_int",
        abi = "log_named_decimal_int(string,int256,uint256)"
    )]
    pub struct LogNamedDecimalIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
        pub decimals: ::ethers::core::types::U256,
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
        name = "log_named_decimal_uint",
        abi = "log_named_decimal_uint(string,uint256,uint256)"
    )]
    pub struct LogNamedDecimalUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
        pub decimals: ::ethers::core::types::U256,
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
    #[ethevent(name = "log_named_int", abi = "log_named_int(string,int256)")]
    pub struct LogNamedIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
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
    #[ethevent(name = "log_named_string", abi = "log_named_string(string,string)")]
    pub struct LogNamedStringFilter {
        pub key: ::std::string::String,
        pub val: ::std::string::String,
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
    #[ethevent(name = "log_named_uint", abi = "log_named_uint(string,uint256)")]
    pub struct LogNamedUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
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
    #[ethevent(name = "log_string", abi = "log_string(string)")]
    pub struct LogStringFilter(pub ::std::string::String);
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
    #[ethevent(name = "log_uint", abi = "log_uint(uint256)")]
    pub struct LogUintFilter(pub ::ethers::core::types::U256);
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
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ::ethers::core::types::Bytes);
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
    pub enum StakeTableHandlerTestEvents {
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogArray1Filter(LogArray1Filter),
        LogArray2Filter(LogArray2Filter),
        LogArray3Filter(LogArray3Filter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedArray1Filter(LogNamedArray1Filter),
        LogNamedArray2Filter(LogNamedArray2Filter),
        LogNamedArray3Filter(LogNamedArray3Filter),
        LogNamedBytesFilter(LogNamedBytesFilter),
        LogNamedBytes32Filter(LogNamedBytes32Filter),
        LogNamedDecimalIntFilter(LogNamedDecimalIntFilter),
        LogNamedDecimalUintFilter(LogNamedDecimalUintFilter),
        LogNamedIntFilter(LogNamedIntFilter),
        LogNamedStringFilter(LogNamedStringFilter),
        LogNamedUintFilter(LogNamedUintFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
        LogsFilter(LogsFilter),
    }
    impl ::ethers::contract::EthLogDecode for StakeTableHandlerTestEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogNamedDecimalIntFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogNamedDecimalUintFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(StakeTableHandlerTestEvents::LogsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StakeTableHandlerTestEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LogFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogAddressFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray3Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes32Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedAddressFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedArray1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedArray2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedArray3Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedBytesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedBytes32Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedDecimalIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedDecimalUintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedStringFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedUintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogStringFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogUintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogsFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LogFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogArray1Filter> for StakeTableHandlerTestEvents {
        fn from(value: LogArray1Filter) -> Self {
            Self::LogArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogArray2Filter> for StakeTableHandlerTestEvents {
        fn from(value: LogArray2Filter) -> Self {
            Self::LogArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogArray3Filter> for StakeTableHandlerTestEvents {
        fn from(value: LogArray3Filter) -> Self {
            Self::LogArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for StakeTableHandlerTestEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedAddressFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogNamedAddressFilter) -> Self {
            Self::LogNamedAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray1Filter> for StakeTableHandlerTestEvents {
        fn from(value: LogNamedArray1Filter) -> Self {
            Self::LogNamedArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray2Filter> for StakeTableHandlerTestEvents {
        fn from(value: LogNamedArray2Filter) -> Self {
            Self::LogNamedArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray3Filter> for StakeTableHandlerTestEvents {
        fn from(value: LogNamedArray3Filter) -> Self {
            Self::LogNamedArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytesFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogNamedBytesFilter) -> Self {
            Self::LogNamedBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytes32Filter> for StakeTableHandlerTestEvents {
        fn from(value: LogNamedBytes32Filter) -> Self {
            Self::LogNamedBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalIntFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogNamedDecimalIntFilter) -> Self {
            Self::LogNamedDecimalIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalUintFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogNamedDecimalUintFilter) -> Self {
            Self::LogNamedDecimalUintFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedIntFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogNamedIntFilter) -> Self {
            Self::LogNamedIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedStringFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogNamedStringFilter) -> Self {
            Self::LogNamedStringFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedUintFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogNamedUintFilter) -> Self {
            Self::LogNamedUintFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    impl ::core::convert::From<LogsFilter> for StakeTableHandlerTestEvents {
        fn from(value: LogsFilter) -> Self {
            Self::LogsFilter(value)
        }
    }
    ///Container type for all input parameters for the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
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
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    ///Container type for all input parameters for the `advanceEpoch` function with signature `advanceEpoch()` and selector `0x3cf80e6c`
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
    #[ethcall(name = "advanceEpoch", abi = "advanceEpoch()")]
    pub struct AdvanceEpochCall;
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
    ///Container type for all input parameters for the `deposit` function with signature `deposit(uint256,uint64)` and selector `0x7d552ea6`
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
    #[ethcall(name = "deposit", abi = "deposit(uint256,uint64)")]
    pub struct DepositCall {
        pub user_index: ::ethers::core::types::U256,
        pub amount: u64,
    }
    ///Container type for all input parameters for the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `0xb5508aa9`
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
    #[ethcall(name = "excludeArtifacts", abi = "excludeArtifacts()")]
    pub struct ExcludeArtifactsCall;
    ///Container type for all input parameters for the `excludeContracts` function with signature `excludeContracts()` and selector `0xe20c9f71`
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
    #[ethcall(name = "excludeContracts", abi = "excludeContracts()")]
    pub struct ExcludeContractsCall;
    ///Container type for all input parameters for the `excludeSenders` function with signature `excludeSenders()` and selector `0x1ed7831c`
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
    #[ethcall(name = "excludeSenders", abi = "excludeSenders()")]
    pub struct ExcludeSendersCall;
    ///Container type for all input parameters for the `exitEpochForBlsVK` function with signature `exitEpochForBlsVK(bytes32)` and selector `0x2cb48a9e`
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
    #[ethcall(name = "exitEpochForBlsVK", abi = "exitEpochForBlsVK(bytes32)")]
    pub struct ExitEpochForBlsVKCall {
        pub bls_key_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `failed` function with signature `failed()` and selector `0xba414fa6`
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
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
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
    ///Container type for all input parameters for the `nextExitEpochBefore` function with signature `nextExitEpochBefore()` and selector `0x52752fce`
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
    #[ethcall(name = "nextExitEpochBefore", abi = "nextExitEpochBefore()")]
    pub struct NextExitEpochBeforeCall;
    ///Container type for all input parameters for the `nextRegistrationEpochBefore` function with signature `nextRegistrationEpochBefore()` and selector `0xbcc4f0dd`
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
        name = "nextRegistrationEpochBefore",
        abi = "nextRegistrationEpochBefore()"
    )]
    pub struct NextRegistrationEpochBeforeCall;
    ///Container type for all input parameters for the `pendingExitsBefore` function with signature `pendingExitsBefore()` and selector `0x9248dd44`
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
    #[ethcall(name = "pendingExitsBefore", abi = "pendingExitsBefore()")]
    pub struct PendingExitsBeforeCall;
    ///Container type for all input parameters for the `pendingRegistrationsBefore` function with signature `pendingRegistrationsBefore()` and selector `0xc23bbac5`
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
        name = "pendingRegistrationsBefore",
        abi = "pendingRegistrationsBefore()"
    )]
    pub struct PendingRegistrationsBeforeCall;
    ///Container type for all input parameters for the `register` function with signature `register(uint64)` and selector `0xf5165863`
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
    #[ethcall(name = "register", abi = "register(uint64)")]
    pub struct RegisterCall {
        pub amount: u64,
    }
    ///Container type for all input parameters for the `registrationCalledAtLeastOnce` function with signature `registrationCalledAtLeastOnce()` and selector `0x823e263f`
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
        name = "registrationCalledAtLeastOnce",
        abi = "registrationCalledAtLeastOnce()"
    )]
    pub struct RegistrationCalledAtLeastOnceCall;
    ///Container type for all input parameters for the `requestExit` function with signature `requestExit(uint256)` and selector `0x721c6513`
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
    #[ethcall(name = "requestExit", abi = "requestExit(uint256)")]
    pub struct RequestExitCall {
        pub rand: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `requestExitCalledAtLeastOnce` function with signature `requestExitCalledAtLeastOnce()` and selector `0x70a2bca5`
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
        name = "requestExitCalledAtLeastOnce",
        abi = "requestExitCalledAtLeastOnce()"
    )]
    pub struct RequestExitCalledAtLeastOnceCall;
    ///Container type for all input parameters for the `requestExitKeysIndexes` function with signature `requestExitKeysIndexes(uint256)` and selector `0x91831410`
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
        name = "requestExitKeysIndexes",
        abi = "requestExitKeysIndexes(uint256)"
    )]
    pub struct RequestExitKeysIndexesCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `stakeTable` function with signature `stakeTable()` and selector `0x5c050347`
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
    #[ethcall(name = "stakeTable", abi = "stakeTable()")]
    pub struct StakeTableCall;
    ///Container type for all input parameters for the `stakeTableFirstAvailableExitEpoch` function with signature `stakeTableFirstAvailableExitEpoch()` and selector `0x25a2c59b`
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
        name = "stakeTableFirstAvailableExitEpoch",
        abi = "stakeTableFirstAvailableExitEpoch()"
    )]
    pub struct StakeTableFirstAvailableExitEpochCall;
    ///Container type for all input parameters for the `stakeTableFirstAvailableRegistrationEpoch` function with signature `stakeTableFirstAvailableRegistrationEpoch()` and selector `0x0176a3e4`
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
        name = "stakeTableFirstAvailableRegistrationEpoch",
        abi = "stakeTableFirstAvailableRegistrationEpoch()"
    )]
    pub struct StakeTableFirstAvailableRegistrationEpochCall;
    ///Container type for all input parameters for the `stakeTableNumPendingExits` function with signature `stakeTableNumPendingExits()` and selector `0x884da77d`
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
        name = "stakeTableNumPendingExits",
        abi = "stakeTableNumPendingExits()"
    )]
    pub struct StakeTableNumPendingExitsCall;
    ///Container type for all input parameters for the `stakeTableNumPendingRegistrations` function with signature `stakeTableNumPendingRegistrations()` and selector `0x96303168`
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
        name = "stakeTableNumPendingRegistrations",
        abi = "stakeTableNumPendingRegistrations()"
    )]
    pub struct StakeTableNumPendingRegistrationsCall;
    ///Container type for all input parameters for the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`
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
    #[ethcall(name = "targetArtifactSelectors", abi = "targetArtifactSelectors()")]
    pub struct TargetArtifactSelectorsCall;
    ///Container type for all input parameters for the `targetArtifacts` function with signature `targetArtifacts()` and selector `0x85226c81`
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
    #[ethcall(name = "targetArtifacts", abi = "targetArtifacts()")]
    pub struct TargetArtifactsCall;
    ///Container type for all input parameters for the `targetContracts` function with signature `targetContracts()` and selector `0x3f7286f4`
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
    #[ethcall(name = "targetContracts", abi = "targetContracts()")]
    pub struct TargetContractsCall;
    ///Container type for all input parameters for the `targetSelectors` function with signature `targetSelectors()` and selector `0x916a17c6`
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
    #[ethcall(name = "targetSelectors", abi = "targetSelectors()")]
    pub struct TargetSelectorsCall;
    ///Container type for all input parameters for the `targetSenders` function with signature `targetSenders()` and selector `0x3e5e3c23`
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
    #[ethcall(name = "targetSenders", abi = "targetSenders()")]
    pub struct TargetSendersCall;
    ///Container type for all input parameters for the `token` function with signature `token()` and selector `0xfc0c546a`
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
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    ///Container type for all input parameters for the `tokenCreator` function with signature `tokenCreator()` and selector `0xdb84252c`
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
    #[ethcall(name = "tokenCreator", abi = "tokenCreator()")]
    pub struct TokenCreatorCall;
    ///Container type for all input parameters for the `users` function with signature `users(uint256)` and selector `0x365b98b2`
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
    #[ethcall(name = "users", abi = "users(uint256)")]
    pub struct UsersCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `vks` function with signature `vks(uint256)` and selector `0xe3592552`
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
    #[ethcall(name = "vks", abi = "vks(uint256)")]
    pub struct VksCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `vksWithdraw` function with signature `vksWithdraw(uint256)` and selector `0xfa182fa1`
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
    #[ethcall(name = "vksWithdraw", abi = "vksWithdraw(uint256)")]
    pub struct VksWithdrawCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `withdrawFunds` function with signature `withdrawFunds(uint256)` and selector `0x155dd5ee`
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
    #[ethcall(name = "withdrawFunds", abi = "withdrawFunds(uint256)")]
    pub struct WithdrawFundsCall {
        pub rand: ::ethers::core::types::U256,
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
    pub enum StakeTableHandlerTestCalls {
        IsTest(IsTestCall),
        AdvanceEpoch(AdvanceEpochCall),
        CurrentEpoch(CurrentEpochCall),
        Deposit(DepositCall),
        ExcludeArtifacts(ExcludeArtifactsCall),
        ExcludeContracts(ExcludeContractsCall),
        ExcludeSenders(ExcludeSendersCall),
        ExitEpochForBlsVK(ExitEpochForBlsVKCall),
        Failed(FailedCall),
        LightClient(LightClientCall),
        NextExitEpochBefore(NextExitEpochBeforeCall),
        NextRegistrationEpochBefore(NextRegistrationEpochBeforeCall),
        PendingExitsBefore(PendingExitsBeforeCall),
        PendingRegistrationsBefore(PendingRegistrationsBeforeCall),
        Register(RegisterCall),
        RegistrationCalledAtLeastOnce(RegistrationCalledAtLeastOnceCall),
        RequestExit(RequestExitCall),
        RequestExitCalledAtLeastOnce(RequestExitCalledAtLeastOnceCall),
        RequestExitKeysIndexes(RequestExitKeysIndexesCall),
        StakeTable(StakeTableCall),
        StakeTableFirstAvailableExitEpoch(StakeTableFirstAvailableExitEpochCall),
        StakeTableFirstAvailableRegistrationEpoch(StakeTableFirstAvailableRegistrationEpochCall),
        StakeTableNumPendingExits(StakeTableNumPendingExitsCall),
        StakeTableNumPendingRegistrations(StakeTableNumPendingRegistrationsCall),
        TargetArtifactSelectors(TargetArtifactSelectorsCall),
        TargetArtifacts(TargetArtifactsCall),
        TargetContracts(TargetContractsCall),
        TargetSelectors(TargetSelectorsCall),
        TargetSenders(TargetSendersCall),
        Token(TokenCall),
        TokenCreator(TokenCreatorCall),
        Users(UsersCall),
        Vks(VksCall),
        VksWithdraw(VksWithdrawCall),
        WithdrawFunds(WithdrawFundsCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakeTableHandlerTestCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsTestCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsTest(decoded));
            }
            if let Ok(decoded) = <AdvanceEpochCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AdvanceEpoch(decoded));
            }
            if let Ok(decoded) = <CurrentEpochCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CurrentEpoch(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) =
                <ExcludeArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExcludeArtifacts(decoded));
            }
            if let Ok(decoded) =
                <ExcludeContractsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExcludeContracts(decoded));
            }
            if let Ok(decoded) =
                <ExcludeSendersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExcludeSenders(decoded));
            }
            if let Ok(decoded) =
                <ExitEpochForBlsVKCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExitEpochForBlsVK(decoded));
            }
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Failed(decoded));
            }
            if let Ok(decoded) = <LightClientCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LightClient(decoded));
            }
            if let Ok(decoded) =
                <NextExitEpochBeforeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NextExitEpochBefore(decoded));
            }
            if let Ok(decoded) =
                <NextRegistrationEpochBeforeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NextRegistrationEpochBefore(decoded));
            }
            if let Ok(decoded) =
                <PendingExitsBeforeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingExitsBefore(decoded));
            }
            if let Ok(decoded) =
                <PendingRegistrationsBeforeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingRegistrationsBefore(decoded));
            }
            if let Ok(decoded) = <RegisterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Register(decoded));
            }
            if let Ok(decoded) =
                <RegistrationCalledAtLeastOnceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegistrationCalledAtLeastOnce(decoded));
            }
            if let Ok(decoded) = <RequestExitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RequestExit(decoded));
            }
            if let Ok(decoded) =
                <RequestExitCalledAtLeastOnceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestExitCalledAtLeastOnce(decoded));
            }
            if let Ok(decoded) =
                <RequestExitKeysIndexesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestExitKeysIndexes(decoded));
            }
            if let Ok(decoded) = <StakeTableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StakeTable(decoded));
            }
            if let Ok(decoded) =
                <StakeTableFirstAvailableExitEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::StakeTableFirstAvailableExitEpoch(decoded));
            }
            if let Ok(decoded) = <StakeTableFirstAvailableRegistrationEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakeTableFirstAvailableRegistrationEpoch(decoded));
            }
            if let Ok(decoded) =
                <StakeTableNumPendingExitsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeTableNumPendingExits(decoded));
            }
            if let Ok(decoded) =
                <StakeTableNumPendingRegistrationsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::StakeTableNumPendingRegistrations(decoded));
            }
            if let Ok(decoded) =
                <TargetArtifactSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TargetArtifactSelectors(decoded));
            }
            if let Ok(decoded) =
                <TargetArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TargetArtifacts(decoded));
            }
            if let Ok(decoded) =
                <TargetContractsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TargetContracts(decoded));
            }
            if let Ok(decoded) =
                <TargetSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TargetSelectors(decoded));
            }
            if let Ok(decoded) = <TargetSendersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TargetSenders(decoded));
            }
            if let Ok(decoded) = <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token(decoded));
            }
            if let Ok(decoded) = <TokenCreatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TokenCreator(decoded));
            }
            if let Ok(decoded) = <UsersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Users(decoded));
            }
            if let Ok(decoded) = <VksCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Vks(decoded));
            }
            if let Ok(decoded) = <VksWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VksWithdraw(decoded));
            }
            if let Ok(decoded) = <WithdrawFundsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawFunds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakeTableHandlerTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsTest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AdvanceEpoch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CurrentEpoch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeArtifacts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeContracts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeSenders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExitEpochForBlsVK(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Failed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LightClient(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NextExitEpochBefore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NextRegistrationEpochBefore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingExitsBefore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingRegistrationsBefore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Register(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegistrationCalledAtLeastOnce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestExit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestExitCalledAtLeastOnce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestExitKeysIndexes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeTable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeTableFirstAvailableExitEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeTableFirstAvailableRegistrationEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeTableNumPendingExits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeTableNumPendingRegistrations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetArtifactSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetArtifacts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetContracts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetSelectors(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetSenders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenCreator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Users(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Vks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VksWithdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawFunds(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for StakeTableHandlerTestCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsTest(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdvanceEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExitEpochForBlsVK(element) => ::core::fmt::Display::fmt(element, f),
                Self::Failed(element) => ::core::fmt::Display::fmt(element, f),
                Self::LightClient(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextExitEpochBefore(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextRegistrationEpochBefore(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingExitsBefore(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingRegistrationsBefore(element) => ::core::fmt::Display::fmt(element, f),
                Self::Register(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistrationCalledAtLeastOnce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestExit(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestExitCalledAtLeastOnce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestExitKeysIndexes(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTable(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTableFirstAvailableExitEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeTableFirstAvailableRegistrationEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeTableNumPendingExits(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTableNumPendingRegistrations(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TargetArtifactSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenCreator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Users(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vks(element) => ::core::fmt::Display::fmt(element, f),
                Self::VksWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFunds(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsTestCall> for StakeTableHandlerTestCalls {
        fn from(value: IsTestCall) -> Self {
            Self::IsTest(value)
        }
    }
    impl ::core::convert::From<AdvanceEpochCall> for StakeTableHandlerTestCalls {
        fn from(value: AdvanceEpochCall) -> Self {
            Self::AdvanceEpoch(value)
        }
    }
    impl ::core::convert::From<CurrentEpochCall> for StakeTableHandlerTestCalls {
        fn from(value: CurrentEpochCall) -> Self {
            Self::CurrentEpoch(value)
        }
    }
    impl ::core::convert::From<DepositCall> for StakeTableHandlerTestCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<ExcludeArtifactsCall> for StakeTableHandlerTestCalls {
        fn from(value: ExcludeArtifactsCall) -> Self {
            Self::ExcludeArtifacts(value)
        }
    }
    impl ::core::convert::From<ExcludeContractsCall> for StakeTableHandlerTestCalls {
        fn from(value: ExcludeContractsCall) -> Self {
            Self::ExcludeContracts(value)
        }
    }
    impl ::core::convert::From<ExcludeSendersCall> for StakeTableHandlerTestCalls {
        fn from(value: ExcludeSendersCall) -> Self {
            Self::ExcludeSenders(value)
        }
    }
    impl ::core::convert::From<ExitEpochForBlsVKCall> for StakeTableHandlerTestCalls {
        fn from(value: ExitEpochForBlsVKCall) -> Self {
            Self::ExitEpochForBlsVK(value)
        }
    }
    impl ::core::convert::From<FailedCall> for StakeTableHandlerTestCalls {
        fn from(value: FailedCall) -> Self {
            Self::Failed(value)
        }
    }
    impl ::core::convert::From<LightClientCall> for StakeTableHandlerTestCalls {
        fn from(value: LightClientCall) -> Self {
            Self::LightClient(value)
        }
    }
    impl ::core::convert::From<NextExitEpochBeforeCall> for StakeTableHandlerTestCalls {
        fn from(value: NextExitEpochBeforeCall) -> Self {
            Self::NextExitEpochBefore(value)
        }
    }
    impl ::core::convert::From<NextRegistrationEpochBeforeCall> for StakeTableHandlerTestCalls {
        fn from(value: NextRegistrationEpochBeforeCall) -> Self {
            Self::NextRegistrationEpochBefore(value)
        }
    }
    impl ::core::convert::From<PendingExitsBeforeCall> for StakeTableHandlerTestCalls {
        fn from(value: PendingExitsBeforeCall) -> Self {
            Self::PendingExitsBefore(value)
        }
    }
    impl ::core::convert::From<PendingRegistrationsBeforeCall> for StakeTableHandlerTestCalls {
        fn from(value: PendingRegistrationsBeforeCall) -> Self {
            Self::PendingRegistrationsBefore(value)
        }
    }
    impl ::core::convert::From<RegisterCall> for StakeTableHandlerTestCalls {
        fn from(value: RegisterCall) -> Self {
            Self::Register(value)
        }
    }
    impl ::core::convert::From<RegistrationCalledAtLeastOnceCall> for StakeTableHandlerTestCalls {
        fn from(value: RegistrationCalledAtLeastOnceCall) -> Self {
            Self::RegistrationCalledAtLeastOnce(value)
        }
    }
    impl ::core::convert::From<RequestExitCall> for StakeTableHandlerTestCalls {
        fn from(value: RequestExitCall) -> Self {
            Self::RequestExit(value)
        }
    }
    impl ::core::convert::From<RequestExitCalledAtLeastOnceCall> for StakeTableHandlerTestCalls {
        fn from(value: RequestExitCalledAtLeastOnceCall) -> Self {
            Self::RequestExitCalledAtLeastOnce(value)
        }
    }
    impl ::core::convert::From<RequestExitKeysIndexesCall> for StakeTableHandlerTestCalls {
        fn from(value: RequestExitKeysIndexesCall) -> Self {
            Self::RequestExitKeysIndexes(value)
        }
    }
    impl ::core::convert::From<StakeTableCall> for StakeTableHandlerTestCalls {
        fn from(value: StakeTableCall) -> Self {
            Self::StakeTable(value)
        }
    }
    impl ::core::convert::From<StakeTableFirstAvailableExitEpochCall> for StakeTableHandlerTestCalls {
        fn from(value: StakeTableFirstAvailableExitEpochCall) -> Self {
            Self::StakeTableFirstAvailableExitEpoch(value)
        }
    }
    impl ::core::convert::From<StakeTableFirstAvailableRegistrationEpochCall>
        for StakeTableHandlerTestCalls
    {
        fn from(value: StakeTableFirstAvailableRegistrationEpochCall) -> Self {
            Self::StakeTableFirstAvailableRegistrationEpoch(value)
        }
    }
    impl ::core::convert::From<StakeTableNumPendingExitsCall> for StakeTableHandlerTestCalls {
        fn from(value: StakeTableNumPendingExitsCall) -> Self {
            Self::StakeTableNumPendingExits(value)
        }
    }
    impl ::core::convert::From<StakeTableNumPendingRegistrationsCall> for StakeTableHandlerTestCalls {
        fn from(value: StakeTableNumPendingRegistrationsCall) -> Self {
            Self::StakeTableNumPendingRegistrations(value)
        }
    }
    impl ::core::convert::From<TargetArtifactSelectorsCall> for StakeTableHandlerTestCalls {
        fn from(value: TargetArtifactSelectorsCall) -> Self {
            Self::TargetArtifactSelectors(value)
        }
    }
    impl ::core::convert::From<TargetArtifactsCall> for StakeTableHandlerTestCalls {
        fn from(value: TargetArtifactsCall) -> Self {
            Self::TargetArtifacts(value)
        }
    }
    impl ::core::convert::From<TargetContractsCall> for StakeTableHandlerTestCalls {
        fn from(value: TargetContractsCall) -> Self {
            Self::TargetContracts(value)
        }
    }
    impl ::core::convert::From<TargetSelectorsCall> for StakeTableHandlerTestCalls {
        fn from(value: TargetSelectorsCall) -> Self {
            Self::TargetSelectors(value)
        }
    }
    impl ::core::convert::From<TargetSendersCall> for StakeTableHandlerTestCalls {
        fn from(value: TargetSendersCall) -> Self {
            Self::TargetSenders(value)
        }
    }
    impl ::core::convert::From<TokenCall> for StakeTableHandlerTestCalls {
        fn from(value: TokenCall) -> Self {
            Self::Token(value)
        }
    }
    impl ::core::convert::From<TokenCreatorCall> for StakeTableHandlerTestCalls {
        fn from(value: TokenCreatorCall) -> Self {
            Self::TokenCreator(value)
        }
    }
    impl ::core::convert::From<UsersCall> for StakeTableHandlerTestCalls {
        fn from(value: UsersCall) -> Self {
            Self::Users(value)
        }
    }
    impl ::core::convert::From<VksCall> for StakeTableHandlerTestCalls {
        fn from(value: VksCall) -> Self {
            Self::Vks(value)
        }
    }
    impl ::core::convert::From<VksWithdrawCall> for StakeTableHandlerTestCalls {
        fn from(value: VksWithdrawCall) -> Self {
            Self::VksWithdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawFundsCall> for StakeTableHandlerTestCalls {
        fn from(value: WithdrawFundsCall) -> Self {
            Self::WithdrawFunds(value)
        }
    }
    ///Container type for all return fields from the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
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
    pub struct IsTestReturn(pub bool);
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
    ///Container type for all return fields from the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `0xb5508aa9`
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
    pub struct ExcludeArtifactsReturn {
        pub excluded_artifacts: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `excludeContracts` function with signature `excludeContracts()` and selector `0xe20c9f71`
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
    pub struct ExcludeContractsReturn {
        pub excluded_contracts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `excludeSenders` function with signature `excludeSenders()` and selector `0x1ed7831c`
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
    pub struct ExcludeSendersReturn {
        pub excluded_senders: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `exitEpochForBlsVK` function with signature `exitEpochForBlsVK(bytes32)` and selector `0x2cb48a9e`
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
    pub struct ExitEpochForBlsVKReturn {
        pub exit_epoch: u64,
    }
    ///Container type for all return fields from the `failed` function with signature `failed()` and selector `0xba414fa6`
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
    pub struct FailedReturn(pub bool);
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
    ///Container type for all return fields from the `nextExitEpochBefore` function with signature `nextExitEpochBefore()` and selector `0x52752fce`
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
    pub struct NextExitEpochBeforeReturn(pub u64);
    ///Container type for all return fields from the `nextRegistrationEpochBefore` function with signature `nextRegistrationEpochBefore()` and selector `0xbcc4f0dd`
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
    pub struct NextRegistrationEpochBeforeReturn(pub u64);
    ///Container type for all return fields from the `pendingExitsBefore` function with signature `pendingExitsBefore()` and selector `0x9248dd44`
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
    pub struct PendingExitsBeforeReturn(pub u64);
    ///Container type for all return fields from the `pendingRegistrationsBefore` function with signature `pendingRegistrationsBefore()` and selector `0xc23bbac5`
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
    pub struct PendingRegistrationsBeforeReturn(pub u64);
    ///Container type for all return fields from the `registrationCalledAtLeastOnce` function with signature `registrationCalledAtLeastOnce()` and selector `0x823e263f`
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
    pub struct RegistrationCalledAtLeastOnceReturn(pub bool);
    ///Container type for all return fields from the `requestExitCalledAtLeastOnce` function with signature `requestExitCalledAtLeastOnce()` and selector `0x70a2bca5`
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
    pub struct RequestExitCalledAtLeastOnceReturn(pub bool);
    ///Container type for all return fields from the `requestExitKeysIndexes` function with signature `requestExitKeysIndexes(uint256)` and selector `0x91831410`
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
    pub struct RequestExitKeysIndexesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stakeTable` function with signature `stakeTable()` and selector `0x5c050347`
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
    pub struct StakeTableReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `stakeTableFirstAvailableExitEpoch` function with signature `stakeTableFirstAvailableExitEpoch()` and selector `0x25a2c59b`
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
    pub struct StakeTableFirstAvailableExitEpochReturn(pub u64);
    ///Container type for all return fields from the `stakeTableFirstAvailableRegistrationEpoch` function with signature `stakeTableFirstAvailableRegistrationEpoch()` and selector `0x0176a3e4`
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
    pub struct StakeTableFirstAvailableRegistrationEpochReturn(pub u64);
    ///Container type for all return fields from the `stakeTableNumPendingExits` function with signature `stakeTableNumPendingExits()` and selector `0x884da77d`
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
    pub struct StakeTableNumPendingExitsReturn(pub u64);
    ///Container type for all return fields from the `stakeTableNumPendingRegistrations` function with signature `stakeTableNumPendingRegistrations()` and selector `0x96303168`
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
    pub struct StakeTableNumPendingRegistrationsReturn(pub u64);
    ///Container type for all return fields from the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`
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
    pub struct TargetArtifactSelectorsReturn {
        pub targeted_artifact_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    ///Container type for all return fields from the `targetArtifacts` function with signature `targetArtifacts()` and selector `0x85226c81`
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
    pub struct TargetArtifactsReturn {
        pub targeted_artifacts: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `targetContracts` function with signature `targetContracts()` and selector `0x3f7286f4`
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
    pub struct TargetContractsReturn {
        pub targeted_contracts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `targetSelectors` function with signature `targetSelectors()` and selector `0x916a17c6`
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
    pub struct TargetSelectorsReturn {
        pub targeted_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    ///Container type for all return fields from the `targetSenders` function with signature `targetSenders()` and selector `0x3e5e3c23`
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
    pub struct TargetSendersReturn {
        pub targeted_senders: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `token` function with signature `token()` and selector `0xfc0c546a`
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
    pub struct TokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokenCreator` function with signature `tokenCreator()` and selector `0xdb84252c`
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
    pub struct TokenCreatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `users` function with signature `users(uint256)` and selector `0x365b98b2`
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
    pub struct UsersReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `vks` function with signature `vks(uint256)` and selector `0xe3592552`
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
    pub struct VksReturn {
        pub x_0: ::ethers::core::types::U256,
        pub x_1: ::ethers::core::types::U256,
        pub y_0: ::ethers::core::types::U256,
        pub y_1: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `vksWithdraw` function with signature `vksWithdraw(uint256)` and selector `0xfa182fa1`
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
    pub struct VksWithdrawReturn {
        pub x_0: ::ethers::core::types::U256,
        pub x_1: ::ethers::core::types::U256,
        pub y_0: ::ethers::core::types::U256,
        pub y_1: ::ethers::core::types::U256,
    }
}
