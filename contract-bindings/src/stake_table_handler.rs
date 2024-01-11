pub use stake_table_handler::*;
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
pub mod stake_table_handler {
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
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_users"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Address,),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address[]"),
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
                    ::std::borrow::ToOwned::to_owned("genClientWallet"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("genClientWallet"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("seed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
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
                                name: ::std::string::String::new(),
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
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isUserRegistered"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isUserRegistered"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("userIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("registered"),
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
                    ::std::borrow::ToOwned::to_owned("numberUsers"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("numberUsers"),
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
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("userIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("registrationSuccessful"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registrationSuccessful",),
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
                    ::std::borrow::ToOwned::to_owned("requestExitKeys"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestExitKeys"),
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
                    ::std::borrow::ToOwned::to_owned("requestExitSuccessful"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestExitSuccessful",),
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
                    ::std::borrow::ToOwned::to_owned("userIndexFromVk"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("userIndexFromVk"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("vkHash"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("userIndex"),
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
    pub static STAKETABLEHANDLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15b\0\0.W`\0\x80\xFD[P`@Qb\0/n8\x03\x80b\0/n\x839\x81\x01`@\x81\x90Rb\0\0Q\x91b\0\x01\x84V[`\x1B\x80T`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x1D\x80T\x86\x84\x16\x90\x83\x16\x17\x90U`\x1C\x80T\x87\x84\x16\x90\x83\x16\x17\x90U` \x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x81U\x81Qb\0\0\xAF\x91`!\x91\x90\x84\x01\x90b\0\0\xC2V[PP`!T`#UPb\0\x02\xB0\x92PPPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15b\0\x01\x1AW\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\x01\x1AW\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90b\0\0\xE3V[Pb\0\x01(\x92\x91Pb\0\x01,V[P\x90V[[\x80\x82\x11\x15b\0\x01(W`\0\x81U`\x01\x01b\0\x01-V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01YW`\0\x80\xFD[PV[\x80Qb\0\x01i\x81b\0\x01CV[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01\x9DW`\0\x80\xFD[\x85Qb\0\x01\xAA\x81b\0\x01CV[\x80\x95PP` \x80\x87\x01Qb\0\x01\xBF\x81b\0\x01CV[`@\x88\x01Q\x90\x95Pb\0\x01\xD2\x81b\0\x01CV[``\x88\x01Q\x90\x94Pb\0\x01\xE5\x81b\0\x01CV[`\x80\x88\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x02\x03W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12b\0\x02\x18W`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x02-Wb\0\x02-b\0\x01nV[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15b\0\x02UWb\0\x02Ub\0\x01nV[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x8C\x83\x11\x15b\0\x02tW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15b\0\x02\x9DWb\0\x02\x8D\x85b\0\x01\\V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92b\0\x02yV[\x80\x96PPPPPPP\x92\x95P\x92\x95\x90\x93PV[a,\xAE\x80b\0\x02\xC0`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02=W`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\x01;W\x80c\xBC\xC4\xF0\xDD\x11a\0\xB8W\x80c\xE3Y%R\x11a\0|W\x80c\xE3Y%R\x14a\x05^W\x80c\xF5\xA0n\x9D\x14a\x05\x93W\x80c\xFA\x18/\xA1\x14a\x05\xA0W\x80c\xFAv&\xD4\x14a\x05\xB3W\x80c\xFC\x0CTj\x14a\x05\xC0W`\0\x80\xFD[\x80c\xBC\xC4\xF0\xDD\x14a\x04\xF3W\x80c\xC2;\xBA\xC5\x14a\x05\x06W\x80c\xDB\x84%,\x14a\x05 W\x80c\xE0\xA5\x8E\xE1\x14a\x053W\x80c\xE2\x0C\x9Fq\x14a\x05VW`\0\x80\xFD[\x80c\x9601h\x11a\0\xFFW\x80c\x9601h\x14a\x04\xADW\x80c\xB5P\x8A\xA9\x14a\x04\xC7W\x80c\xB5p\x0Eh\x14a\x04\xCFW\x80c\xBAAO\xA6\x14a\x04\xE2W\x80c\xBBG\x10\xC5\x14a\x04\xEAW`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x040W\x80c\x88M\xA7}\x14a\x04EW\x80c\x8Cb\xC3\xAD\x14a\x04XW\x80c\x91j\x17\xC6\x14a\x04\x8BW\x80c\x92H\xDDD\x14a\x04\x93W`\0\x80\xFD[\x80c?r\x86\xF4\x11a\x01\xC9W\x80ck\xAD9\x12\x11a\x01\x8DW\x80ck\xAD9\x12\x14a\x03\xBBW\x80cr\x1Ce\x13\x14a\x03\xDDW\x80cvg\x18\x08\x14a\x03\xF0W\x80c}U.\xA6\x14a\x04\nW\x80c\x7F\xAE\xB4\xEF\x14a\x04\x1DW`\0\x80\xFD[\x80c?r\x86\xF4\x14a\x03EW\x80c?\xCC\xF1\x0F\x14a\x03MW\x80cRu/\xCE\x14a\x03{W\x80c\\\x05\x03G\x14a\x03\x93W\x80cf\xD9\xA9\xA0\x14a\x03\xA6W`\0\x80\xFD[\x80c*\x1B\xF7d\x11a\x02\x10W\x80c*\x1B\xF7d\x14a\x02\xBDW\x80c,\xB4\x8A\x9E\x14a\x02\xE1W\x80c6[\x98\xB2\x14a\x03\nW\x80c<\xF8\x0El\x14a\x035W\x80c>^<#\x14a\x03=W`\0\x80\xFD[\x80c\x01v\xA3\xE4\x14a\x02BW\x80c\x15]\xD5\xEE\x14a\x02yW\x80c\x1E\xD7\x83\x1C\x14a\x02\x8EW\x80c%\xA2\xC5\x9B\x14a\x02\xA3W[`\0\x80\xFD[`$Ta\x02\\\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x8Ca\x02\x876`\x04a$uV[a\x05\xD3V[\0[a\x02\x96a\t.V[`@Qa\x02p\x91\x90a$\x8EV[`%Ta\x02\\\x90`\x01`\x88\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`&Ta\x02\xD1\x90`\x01`\x80\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02pV[a\x02\\a\x02\xEF6`\x04a$uV[`'` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x03\x1Da\x03\x186`\x04a$uV[a\t\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02pV[a\x02\x8Ca\t\xBAV[a\x02\x96a\n\x93V[a\x02\x96a\n\xF3V[a\x03ma\x03[6`\x04a$uV[`\"` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\x02pV[`%Ta\x02\\\x90a\x01\0\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x1BTa\x03\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xAEa\x0BSV[`@Qa\x02p\x91\x90a$\xDBV[a\x03\xCEa\x03\xC96`\x04a%\xB9V[a\x0CBV[`@Qa\x02p\x93\x92\x91\x90a%\xEEV[a\x02\x8Ca\x03\xEB6`\x04a$uV[a\x0FAV[`&Ta\x02\\\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x02\x8Ca\x04\x186`\x04a&RV[a\x14\xE5V[a\x02\x8Ca\x04+6`\x04a&\x82V[a\x16\xC5V[a\x048a\x18\xFBV[`@Qa\x02p\x91\x90a&\xFEV[`&Ta\x02\\\x90`\x01`\x01`@\x1B\x03\x16\x81V[a\x04ka\x04f6`\x04a$uV[a\x19\xCBV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x02pV[a\x03\xAEa\x1A\x05V[`%Ta\x02\\\x90`\x01`H\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`$Ta\x02\\\x90`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x048a\x1A\xEBV[` Ta\x03\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xD1a\x1B\xBBV[a\x03m`#T\x81V[`$Ta\x02\\\x90`\x01`\x01`@\x1B\x03\x16\x81V[`$Ta\x02\\\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x1CTa\x03\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xD1a\x05A6`\x04a$uV[`(` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\x96a\x1C\xE6V[a\x04ka\x05l6`\x04a$uV[`\x1E` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x92\x90\x91\x84V[`%Ta\x02\xD1\x90`\xFF\x16\x81V[a\x04ka\x05\xAE6`\x04a$uV[a\x1DFV[`\0Ta\x02\xD1\x90`\xFF\x16\x81V[`\x1DTa\x03\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`)T`\0\x03a\x05\xE0WPV[`\0a\x05\xFF\x82`\0`\x01`)\x80T\x90Pa\x05\xFA\x91\x90a'xV[a\x1DVV[\x90P`\0`)\x82\x81T\x81\x10a\x06\x16Wa\x06\x16a'\x91V[`\0\x91\x82R` \x80\x83 `@\x80Q`\x80\x81\x01\x82R`\x04\x94\x85\x02\x90\x92\x01\x80T\x83R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x83\x01T\x82\x82\x01R`\x03\x90\x92\x01T``\x82\x01R`\x1BT\x91QcM\x98R\xF3`\xE1\x1B\x81R\x90\x94P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x9B0\xA5\xE6\x91a\x06\x86\x91\x86\x91\x01a'\xA7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC7\x91\x90a'\xD2V[`\0\x81\x81R`'` R`@\x81 T\x91\x92P`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`d\x90a\x06\xF2\x82\x84a'\xEBV[` T`@Qc9I\xD1\xE9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c9I\xD1\xE9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07UW=`\0\x80>=`\0\xFD[PPP`\0\x85\x81R`\"` R`@\x90 T`!\x80T\x91\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xCAf\x9F\xA7\x91\x90\x84\x90\x81\x10a\x07\x9BWa\x07\x9Ba'\x91V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xFCW=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\x01\x84\x95\xE3`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\x0C$\xAF\x18\x91Pa\x080\x90\x89\x90`\x04\x01a'\xA7V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08s\x91\x90a(\x12V[P`)\x80Ta\x08\x84\x90`\x01\x90a'xV[\x81T\x81\x10a\x08\x94Wa\x08\x94a'\x91V[\x90`\0R` `\0 \x90`\x04\x02\x01`)\x88\x81T\x81\x10a\x08\xB5Wa\x08\xB5a'\x91V[`\0\x91\x82R` \x90\x91 \x82T`\x04\x90\x92\x02\x01\x90\x81U`\x01\x80\x83\x01T\x90\x82\x01U`\x02\x80\x83\x01T\x90\x82\x01U`\x03\x91\x82\x01T\x91\x01U`)\x80T\x80a\x08\xF8Wa\x08\xF8a(/V[`\0\x82\x81R` \x81 `\x04`\0\x19\x90\x93\x01\x92\x83\x02\x01\x81\x81U`\x01\x81\x01\x82\x90U`\x02\x81\x01\x82\x90U`\x03\x01U\x90UPPPPPPPPV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x86W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\thW[PPPPP\x90P\x90V[`!\x81\x81T\x81\x10a\t\xA0W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[` \x80T`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c9I\xD1\xE9\x92\x84\x92cvg\x18\x08\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n-\x91\x90a(\x12V[a\n8\x90`\x01a'\xEBV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\nyW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\x8DW=`\0\x80>=`\0\xFD[PPPPV[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x86W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\thWPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x86W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\thWPPPPP\x90P\x90V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0C9W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0C!W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0B\xE3W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0BwV[PPPP\x90P\x90V[a\x0Cm`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x85R\x82\x81R\x80\x82\x01\x83\x90R\x84Q`\x04\x80\x82R`\xA0\x82\x01\x90\x96R\x93\x94\x90\x93\x91\x82\x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xA9W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x19\x1AY\x99\x8B]\x19\\\xDD`\xBA\x1B\x81RP\x81`\0\x81Q\x81\x10a\x0C\xF5Wa\x0C\xF5a'\x91V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x19\xD9[\x8BX\xDB\x1AY[\x9D\x0B]\xD8[\x1B\x19]`z\x1B\x81RP\x81`\x01\x81Q\x81\x10a\r=Wa\r=a'\x91V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc+e1\x1F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cV\xCAb>\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\xC8\x91\x90\x81\x01\x90a(\xCEV[\x81`\x02\x81Q\x81\x10a\r\xDBWa\r\xDBa'\x91V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E`\x91\x90\x81\x01\x90a(\xCEV[\x81`\x03\x81Q\x81\x10a\x0EsWa\x0Esa'\x91V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x89\x16\x04g`\xE0\x1B\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x89\x16\x04g\x90a\x0E\xB8\x90\x85\x90`\x04\x01a&\xFEV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\xFF\x91\x90\x81\x01\x90a(\xCEV[\x90P`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x0F\x1B\x91\x90a)\xC9V[`@\x80Q\x80\x82\x01\x90\x91R\x91\x82R` \x82\x01R\x90\x9AP\x98P\x96PPPPPPP\x92P\x92P\x92V[`&\x80T`\xFF`\x80\x1B\x19\x16\x90U`#T`\0\x90a\x0Fh\x90\x83\x90\x83\x90a\x05\xFA\x90`\x01\x90a'xV[`\x1BT`@\x80Qc;\t\xC2g`\xE0\x1B\x81R\x81Q\x93\x94P`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c;\t\xC2g\x92`\x04\x80\x82\x01\x93\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0F\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xD5\x91\x90a*\x0FV[`%\x80Tp\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16`\x01`H\x1B`\x01`\x01`@\x1B\x03\x93\x84\x16\x02h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16\x17a\x01\0\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U`\x1BT`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cvg\x18\x08\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x10cW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x87\x91\x90a(\x12V[`&\x80T`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90U`!\x80Tsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xCAf\x9F\xA7\x91\x84\x90\x81\x10a\x10\xE8Wa\x10\xE8a'\x91V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x115W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11IW=`\0\x80>=`\0\xFD[PPP`\0\x82\x81R`\x1E` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x83\x83\x01R`\x03\x01T``\x83\x01R`\x1BT\x90QcJ\xA7\xC2\x7F`\xE0\x1B\x81R\x91\x93P`\x01`\x01`\xA0\x1B\x03\x16\x90cJ\xA7\xC2\x7F\x90a\x11\xB7\x90\x85\x90`\x04\x01a'\xA7V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xFA\x91\x90a*>V[\x90P\x80\x15a\x14\xC4W`\x1BT`@QcM\x98R\xF3`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B0\xA5\xE6\x90a\x123\x90\x86\x90`\x04\x01a'\xA7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12t\x91\x90a'\xD2V[`\x1BT`@Qc\xD8ni}`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xD8ni}\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xEF\x91\x90a*`V[P\x94P\x94P\x94P\x94P\x94P\x80`'`\0\x88\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`)\x88\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x04\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01UPP`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xE43=\xB5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xFF\x91\x90a(\x12V[`%`\x11a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xD6{l\xA5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x9C\x91\x90a(\x12V[`&\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPP[`&\x80T\x91\x15\x15`\x01`\x80\x1B\x02`\xFF`\x80\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UPPPV[a\x14\xFA\x82`\0`\x01`#Ta\x05\xFA\x91\x90a'xV[`\0\x81\x81R`(` R`@\x90 T\x90\x92P`\xFF\x16a\x15\x17WPPV[a\x15-\x81`\x01`\x01`@\x1B\x03\x16`\x01`\na\x1DVV[\x90P`\0`\x1E`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPP\x90P\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xCAf\x9F\xA7`!\x85\x81T\x81\x10a\x15\xBEWa\x15\xBEa'\x91V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x0BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\x1FW=`\0\x80>=`\0\xFD[PP`\x1BT`@\x80Qc\x1D\xC7\xDB\xD1`\xE2\x1B\x81R\x85Q`\x04\x82\x01R` \x86\x01Q`$\x82\x01R\x90\x85\x01Q`D\x82\x01R``\x85\x01Q`d\x82\x01R`\x01`\x01`@\x1B\x03\x86\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcw\x1FoD\x91P`\xA4\x01`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x16\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xBE\x91\x90a*\x0FV[PPPPPV[a\x16\xDD\x82`\xFF\x16`\0`\x01`#Ta\x05\xFA\x91\x90a'xV[`\x1BT`@\x80Qc\x0B\x14\xC1a`\xE2\x1B\x81R\x81Q\x93\x95P`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c,S\x05\x84\x92`\x04\x80\x82\x01\x93\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x17&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17J\x91\x90a*\x0FV[`$\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`@\x1B`\x01`\x01`@\x1B\x03\x93\x84\x16\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x92\x82\x16\x92\x90\x92\x17\x90\x91U`\x1CT`\0\x91a\x17\xA8\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\xFF\x86\x16\x90\x85\x16a\x1D\x9AV[\x90P`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xA6\xE2\xE3\xDC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18!\x91\x90a(\x12V[`$`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x16\xFE\xFE\xD7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xBE\x91\x90a(\x12V[`$\x80T`\x01`\x01`\xC0\x1B\x03\x16`\x01`\xC0\x1B`\x01`\x01`@\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90U`%\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UPPV[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0C9W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x19>\x90a*\xE7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19j\x90a*\xE7V[\x80\x15a\x19\xB7W\x80`\x1F\x10a\x19\x8CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x19\xB7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19\x9AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x19\x1FV[`)\x81\x81T\x81\x10a\x19\xDBW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x93P\x91\x90\x84V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0C9W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x1A\xD3W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1A\x95W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1A)V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0C9W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x1B.\x90a*\xE7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1BZ\x90a*\xE7V[\x80\x15a\x1B\xA7W\x80`\x1F\x10a\x1B|Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1B\xA7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B\x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1B\x0FV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x1B\xDBWP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x1C\xE1W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x1Ci\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a+!V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1C\x83\x91a+RV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x1C\xC0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1C\xC5V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x1C\xDD\x91\x90a*>V[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x86W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\thWPPPPP\x90P\x90V[`\x1F\x81\x81T\x81\x10a\x19\xDBW`\0\x80\xFD[`\0a\x1Dc\x84\x84\x84a\"\x06V[\x90Pa\x1D\x93`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x14\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82a#\xCEV[\x93\x92PPPV[`\0\x80\x83\x90P`\0`!\x85\x81T\x81\x10a\x1D\xB5Wa\x1D\xB5a'\x91V[`\0\x91\x82R` \x82 \x01T`@Qc5\xD6\x9C\x89`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01\x81\x90R`\xFF\x85\x16`$\x83\x01R\x92P\x81\x90\x81\x900\x90ck\xAD9\x12\x90`D\x01a\x01\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1E\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E=\x91\x90a+nV[\x92P\x92P\x92P`\0a\x1ER\x88`\x01`\na\x1DVV[`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x16`\x04\x82\x01R\x90\x91Pb\x01\x86\xA0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E\xAFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\xC3W=`\0\x80>=`\0\xFD[PP`\x1DT`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`\x01`\x01`@\x1B\x03\x87\x16`$\x83\x01R\x90\x91\x16\x92Pc\xA9\x05\x9C\xBB\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1F\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FF\x91\x90a*>V[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\x9CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F\xB0W=`\0\x80>=`\0\xFD[PP`\x1DT`\x1BT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`\x01`\x01`@\x1B\x03\x87\x16`$\x82\x01R\x91\x16\x92Pc\t^\xA7\xB3\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a \x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a 5\x91\x90a*>V[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \x8BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a \x9FW=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xC7,\xC7\x17`\xE0\x1B\x81R`\0\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xC7,\xC7\x17\x90a \xE0\x90\x89\x90\x89\x90\x88\x90\x87\x90\x8B\x90\x8A\x90`\x04\x01a+\xB5V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a \xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!#\x91\x90a*>V[\x90P\x80\x15a!\xF7W`\0\x8B\x81R`(` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x1BT\x90QcM\x98R\xF3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9B0\xA5\xE6\x90a!t\x90\x8A\x90`\x04\x01a'\xA7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xB5\x91\x90a'\xD2V[`\0\x90\x81R`\"` \x90\x81R`@\x80\x83 \x8F\x90U\x8E\x83R`\x1E\x82R\x91\x82\x90 \x89Q\x81U\x90\x89\x01Q`\x01\x82\x01U\x90\x88\x01Q`\x02\x82\x01U``\x88\x01Q`\x03\x90\x91\x01UP[\x9B\x9APPPPPPPPPPPV[`\0\x81\x83\x11\x15a\"\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\"\x92WP\x81\x84\x11\x15[\x15a\"\x9EWP\x82a\x1D\x93V[`\0a\"\xAA\x84\x84a'xV[a\"\xB5\x90`\x01a,JV[\x90P`\x03\x85\x11\x15\x80\x15a\"\xC7WP\x84\x81\x11[\x15a\"\xDEWa\"\xD6\x85\x85a,JV[\x91PPa\x1D\x93V[a\"\xEB`\x03`\0\x19a'xV[\x85\x10\x15\x80\x15a#\x04WPa#\x01\x85`\0\x19a'xV[\x81\x11[\x15a#\x1FWa#\x15\x85`\0\x19a'xV[a\"\xD6\x90\x84a'xV[\x82\x85\x11\x15a#uW`\0a#3\x84\x87a'xV[\x90P`\0a#A\x83\x83a,]V[\x90P\x80`\0\x03a#VW\x84\x93PPPPa\x1D\x93V[`\x01a#b\x82\x88a,JV[a#l\x91\x90a'xV[\x93PPPa#\xC6V[\x83\x85\x10\x15a#\xC6W`\0a#\x89\x86\x86a'xV[\x90P`\0a#\x97\x83\x83a,]V[\x90P\x80`\0\x03a#\xACW\x85\x93PPPPa\x1D\x93V[a#\xB6\x81\x86a'xV[a#\xC1\x90`\x01a,JV[\x93PPP[P\x93\x92PPPV[`\0jconsole.log`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Q`$\x01a#\xF8\x92\x91\x90a,\x7FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90RQa$-\x91\x90a+RV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a$hW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a$mV[``\x91P[PPPPPPV[`\0` \x82\x84\x03\x12\x15a$\x87W`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a$\xCFW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a$\xAAV[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a%\x81W\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a%lW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a%BV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a%\x05V[P\x91\x99\x98PPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a%\xA5W`\0\x80\xFD[PV[\x805`\xFF\x81\x16\x81\x14a\x1C\xE1W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a%\xCCW`\0\x80\xFD[\x825a%\xD7\x81a%\x90V[\x91Pa%\xE5` \x84\x01a%\xA8V[\x90P\x92P\x92\x90PV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R`@\x80\x86\x01Q\x90\x83\x01R``\x80\x86\x01Q\x90\x83\x01R\x83Q`\x80\x83\x01R\x83\x81\x01Q`\xA0\x83\x01R\x82Q`\xC0\x83\x01R\x82\x01Q`\xE0\x82\x01Ra\x01\0\x81\x01[\x94\x93PPPPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a%\xA5W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a&eW`\0\x80\xFD[\x825\x91P` \x83\x015a&w\x81a&=V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a&\x95W`\0\x80\xFD[a&\x9E\x83a%\xA8V[\x91P` \x83\x015a&w\x81a&=V[`\0[\x83\x81\x10\x15a&\xC9W\x81\x81\x01Q\x83\x82\x01R` \x01a&\xB1V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra&\xEA\x81` \x86\x01` \x86\x01a&\xAEV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a'UW`?\x19\x88\x86\x03\x01\x84Ra'C\x85\x83Qa&\xD2V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a''V[P\x92\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a'\x8BWa'\x8Ba'bV[\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x81\x01a'\x8BV[`\0` \x82\x84\x03\x12\x15a'\xE4W`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a(\x0BWa(\x0Ba'bV[P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a($W`\0\x80\xFD[\x81Qa\x1D\x93\x81a&=V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\x01`\x01`@\x1B\x03\x80\x84\x11\x15a(uWa(ua(EV[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a(\x9DWa(\x9Da(EV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a(\xB6W`\0\x80\xFD[a(\xC4\x86` \x83\x01\x87a&\xAEV[PPP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a(\xE0W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xF6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a)\x07W`\0\x80\xFD[a&5\x84\x82Q` \x84\x01a([V[`\0`@\x82\x84\x03\x12\x15a)(W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a)JWa)Ja(EV[`@R\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a)vW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a)\x98Wa)\x98a(EV[\x80`@RP\x80\x91P\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01RP\x92\x91PPV[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a)\xE0W`\0\x80\xFD[a)\xEA\x86\x86a)\x16V[\x93Pa)\xF9\x86`@\x87\x01a)dV[`\xC0\x86\x01Q`\xE0\x90\x96\x01Q\x94\x97\x90\x96P\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a*\"W`\0\x80\xFD[\x82Qa*-\x81a&=V[` \x84\x01Q\x90\x92Pa&w\x81a&=V[`\0` \x82\x84\x03\x12\x15a*PW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1D\x93W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xE0\x87\x89\x03\x12\x15a*yW`\0\x80\xFD[\x86Qa*\x84\x81a%\x90V[` \x88\x01Q\x90\x96P`\x02\x81\x10a*\x99W`\0\x80\xFD[`@\x88\x01Q\x90\x95Pa*\xAA\x81a&=V[``\x88\x01Q\x90\x94Pa*\xBB\x81a&=V[`\x80\x88\x01Q\x90\x93Pa*\xCC\x81a&=V[\x91Pa*\xDB\x88`\xA0\x89\x01a)\x16V[\x90P\x92\x95P\x92\x95P\x92\x95V[`\x01\x81\x81\x1C\x90\x82\x16\x80a*\xFBW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a+\x1BWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a+D\x81`\x04\x85\x01` \x87\x01a&\xAEV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa+d\x81\x84` \x87\x01a&\xAEV[\x91\x90\x91\x01\x92\x91PPV[`\0\x80`\0a\x01\0\x84\x86\x03\x12\x15a+\x84W`\0\x80\xFD[a+\x8E\x85\x85a)dV[\x92Pa+\x9D\x85`\x80\x86\x01a)\x16V[\x91Pa+\xAC\x85`\xC0\x86\x01a)\x16V[\x90P\x92P\x92P\x92V[\x86Q\x81R` \x80\x88\x01Q\x90\x82\x01R`@\x80\x88\x01Q\x90\x82\x01R``\x80\x88\x01Q\x90\x82\x01Ra\x01`\x81\x01\x86Q`\x80\x83\x01R` \x87\x01Q`\xA0\x83\x01R`\x01`\x01`@\x1B\x03\x80\x87\x16`\xC0\x84\x01R`\x02\x86\x10a,\x1BWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\xE0\x83\x01\x86\x90R\x84Qa\x01\0\x84\x01R` \x85\x01Qa\x01 \x84\x01R\x80\x84\x16a\x01@\x84\x01RP\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a'\x8BWa'\x8Ba'bV[`\0\x82a,zWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`@\x81R`\0a,\x92`@\x83\x01\x85a&\xD2V[\x90P\x82` \x83\x01R\x93\x92PPPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static STAKETABLEHANDLER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02=W`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\x01;W\x80c\xBC\xC4\xF0\xDD\x11a\0\xB8W\x80c\xE3Y%R\x11a\0|W\x80c\xE3Y%R\x14a\x05^W\x80c\xF5\xA0n\x9D\x14a\x05\x93W\x80c\xFA\x18/\xA1\x14a\x05\xA0W\x80c\xFAv&\xD4\x14a\x05\xB3W\x80c\xFC\x0CTj\x14a\x05\xC0W`\0\x80\xFD[\x80c\xBC\xC4\xF0\xDD\x14a\x04\xF3W\x80c\xC2;\xBA\xC5\x14a\x05\x06W\x80c\xDB\x84%,\x14a\x05 W\x80c\xE0\xA5\x8E\xE1\x14a\x053W\x80c\xE2\x0C\x9Fq\x14a\x05VW`\0\x80\xFD[\x80c\x9601h\x11a\0\xFFW\x80c\x9601h\x14a\x04\xADW\x80c\xB5P\x8A\xA9\x14a\x04\xC7W\x80c\xB5p\x0Eh\x14a\x04\xCFW\x80c\xBAAO\xA6\x14a\x04\xE2W\x80c\xBBG\x10\xC5\x14a\x04\xEAW`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x040W\x80c\x88M\xA7}\x14a\x04EW\x80c\x8Cb\xC3\xAD\x14a\x04XW\x80c\x91j\x17\xC6\x14a\x04\x8BW\x80c\x92H\xDDD\x14a\x04\x93W`\0\x80\xFD[\x80c?r\x86\xF4\x11a\x01\xC9W\x80ck\xAD9\x12\x11a\x01\x8DW\x80ck\xAD9\x12\x14a\x03\xBBW\x80cr\x1Ce\x13\x14a\x03\xDDW\x80cvg\x18\x08\x14a\x03\xF0W\x80c}U.\xA6\x14a\x04\nW\x80c\x7F\xAE\xB4\xEF\x14a\x04\x1DW`\0\x80\xFD[\x80c?r\x86\xF4\x14a\x03EW\x80c?\xCC\xF1\x0F\x14a\x03MW\x80cRu/\xCE\x14a\x03{W\x80c\\\x05\x03G\x14a\x03\x93W\x80cf\xD9\xA9\xA0\x14a\x03\xA6W`\0\x80\xFD[\x80c*\x1B\xF7d\x11a\x02\x10W\x80c*\x1B\xF7d\x14a\x02\xBDW\x80c,\xB4\x8A\x9E\x14a\x02\xE1W\x80c6[\x98\xB2\x14a\x03\nW\x80c<\xF8\x0El\x14a\x035W\x80c>^<#\x14a\x03=W`\0\x80\xFD[\x80c\x01v\xA3\xE4\x14a\x02BW\x80c\x15]\xD5\xEE\x14a\x02yW\x80c\x1E\xD7\x83\x1C\x14a\x02\x8EW\x80c%\xA2\xC5\x9B\x14a\x02\xA3W[`\0\x80\xFD[`$Ta\x02\\\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x8Ca\x02\x876`\x04a$uV[a\x05\xD3V[\0[a\x02\x96a\t.V[`@Qa\x02p\x91\x90a$\x8EV[`%Ta\x02\\\x90`\x01`\x88\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`&Ta\x02\xD1\x90`\x01`\x80\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02pV[a\x02\\a\x02\xEF6`\x04a$uV[`'` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x03\x1Da\x03\x186`\x04a$uV[a\t\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02pV[a\x02\x8Ca\t\xBAV[a\x02\x96a\n\x93V[a\x02\x96a\n\xF3V[a\x03ma\x03[6`\x04a$uV[`\"` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\x02pV[`%Ta\x02\\\x90a\x01\0\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x1BTa\x03\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xAEa\x0BSV[`@Qa\x02p\x91\x90a$\xDBV[a\x03\xCEa\x03\xC96`\x04a%\xB9V[a\x0CBV[`@Qa\x02p\x93\x92\x91\x90a%\xEEV[a\x02\x8Ca\x03\xEB6`\x04a$uV[a\x0FAV[`&Ta\x02\\\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x02\x8Ca\x04\x186`\x04a&RV[a\x14\xE5V[a\x02\x8Ca\x04+6`\x04a&\x82V[a\x16\xC5V[a\x048a\x18\xFBV[`@Qa\x02p\x91\x90a&\xFEV[`&Ta\x02\\\x90`\x01`\x01`@\x1B\x03\x16\x81V[a\x04ka\x04f6`\x04a$uV[a\x19\xCBV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x02pV[a\x03\xAEa\x1A\x05V[`%Ta\x02\\\x90`\x01`H\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`$Ta\x02\\\x90`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x048a\x1A\xEBV[` Ta\x03\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xD1a\x1B\xBBV[a\x03m`#T\x81V[`$Ta\x02\\\x90`\x01`\x01`@\x1B\x03\x16\x81V[`$Ta\x02\\\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x1CTa\x03\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xD1a\x05A6`\x04a$uV[`(` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\x96a\x1C\xE6V[a\x04ka\x05l6`\x04a$uV[`\x1E` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x92\x90\x91\x84V[`%Ta\x02\xD1\x90`\xFF\x16\x81V[a\x04ka\x05\xAE6`\x04a$uV[a\x1DFV[`\0Ta\x02\xD1\x90`\xFF\x16\x81V[`\x1DTa\x03\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`)T`\0\x03a\x05\xE0WPV[`\0a\x05\xFF\x82`\0`\x01`)\x80T\x90Pa\x05\xFA\x91\x90a'xV[a\x1DVV[\x90P`\0`)\x82\x81T\x81\x10a\x06\x16Wa\x06\x16a'\x91V[`\0\x91\x82R` \x80\x83 `@\x80Q`\x80\x81\x01\x82R`\x04\x94\x85\x02\x90\x92\x01\x80T\x83R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x83\x01T\x82\x82\x01R`\x03\x90\x92\x01T``\x82\x01R`\x1BT\x91QcM\x98R\xF3`\xE1\x1B\x81R\x90\x94P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x9B0\xA5\xE6\x91a\x06\x86\x91\x86\x91\x01a'\xA7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC7\x91\x90a'\xD2V[`\0\x81\x81R`'` R`@\x81 T\x91\x92P`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`d\x90a\x06\xF2\x82\x84a'\xEBV[` T`@Qc9I\xD1\xE9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c9I\xD1\xE9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07UW=`\0\x80>=`\0\xFD[PPP`\0\x85\x81R`\"` R`@\x90 T`!\x80T\x91\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xCAf\x9F\xA7\x91\x90\x84\x90\x81\x10a\x07\x9BWa\x07\x9Ba'\x91V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xFCW=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\x01\x84\x95\xE3`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\x0C$\xAF\x18\x91Pa\x080\x90\x89\x90`\x04\x01a'\xA7V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08s\x91\x90a(\x12V[P`)\x80Ta\x08\x84\x90`\x01\x90a'xV[\x81T\x81\x10a\x08\x94Wa\x08\x94a'\x91V[\x90`\0R` `\0 \x90`\x04\x02\x01`)\x88\x81T\x81\x10a\x08\xB5Wa\x08\xB5a'\x91V[`\0\x91\x82R` \x90\x91 \x82T`\x04\x90\x92\x02\x01\x90\x81U`\x01\x80\x83\x01T\x90\x82\x01U`\x02\x80\x83\x01T\x90\x82\x01U`\x03\x91\x82\x01T\x91\x01U`)\x80T\x80a\x08\xF8Wa\x08\xF8a(/V[`\0\x82\x81R` \x81 `\x04`\0\x19\x90\x93\x01\x92\x83\x02\x01\x81\x81U`\x01\x81\x01\x82\x90U`\x02\x81\x01\x82\x90U`\x03\x01U\x90UPPPPPPPPV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x86W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\thW[PPPPP\x90P\x90V[`!\x81\x81T\x81\x10a\t\xA0W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[` \x80T`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c9I\xD1\xE9\x92\x84\x92cvg\x18\x08\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\n\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n-\x91\x90a(\x12V[a\n8\x90`\x01a'\xEBV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\nyW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\x8DW=`\0\x80>=`\0\xFD[PPPPV[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x86W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\thWPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x86W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\thWPPPPP\x90P\x90V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0C9W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0C!W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0B\xE3W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0BwV[PPPP\x90P\x90V[a\x0Cm`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x85R\x82\x81R\x80\x82\x01\x83\x90R\x84Q`\x04\x80\x82R`\xA0\x82\x01\x90\x96R\x93\x94\x90\x93\x91\x82\x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xA9W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x19\x1AY\x99\x8B]\x19\\\xDD`\xBA\x1B\x81RP\x81`\0\x81Q\x81\x10a\x0C\xF5Wa\x0C\xF5a'\x91V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x19\xD9[\x8BX\xDB\x1AY[\x9D\x0B]\xD8[\x1B\x19]`z\x1B\x81RP\x81`\x01\x81Q\x81\x10a\r=Wa\r=a'\x91V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc+e1\x1F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cV\xCAb>\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\xC8\x91\x90\x81\x01\x90a(\xCEV[\x81`\x02\x81Q\x81\x10a\r\xDBWa\r\xDBa'\x91V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E`\x91\x90\x81\x01\x90a(\xCEV[\x81`\x03\x81Q\x81\x10a\x0EsWa\x0Esa'\x91V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x89\x16\x04g`\xE0\x1B\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x89\x16\x04g\x90a\x0E\xB8\x90\x85\x90`\x04\x01a&\xFEV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\xFF\x91\x90\x81\x01\x90a(\xCEV[\x90P`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x0F\x1B\x91\x90a)\xC9V[`@\x80Q\x80\x82\x01\x90\x91R\x91\x82R` \x82\x01R\x90\x9AP\x98P\x96PPPPPPP\x92P\x92P\x92V[`&\x80T`\xFF`\x80\x1B\x19\x16\x90U`#T`\0\x90a\x0Fh\x90\x83\x90\x83\x90a\x05\xFA\x90`\x01\x90a'xV[`\x1BT`@\x80Qc;\t\xC2g`\xE0\x1B\x81R\x81Q\x93\x94P`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c;\t\xC2g\x92`\x04\x80\x82\x01\x93\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0F\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xD5\x91\x90a*\x0FV[`%\x80Tp\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16`\x01`H\x1B`\x01`\x01`@\x1B\x03\x93\x84\x16\x02h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16\x17a\x01\0\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U`\x1BT`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cvg\x18\x08\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x10cW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x87\x91\x90a(\x12V[`&\x80T`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90U`!\x80Tsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xCAf\x9F\xA7\x91\x84\x90\x81\x10a\x10\xE8Wa\x10\xE8a'\x91V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x115W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11IW=`\0\x80>=`\0\xFD[PPP`\0\x82\x81R`\x1E` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x83\x83\x01R`\x03\x01T``\x83\x01R`\x1BT\x90QcJ\xA7\xC2\x7F`\xE0\x1B\x81R\x91\x93P`\x01`\x01`\xA0\x1B\x03\x16\x90cJ\xA7\xC2\x7F\x90a\x11\xB7\x90\x85\x90`\x04\x01a'\xA7V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xFA\x91\x90a*>V[\x90P\x80\x15a\x14\xC4W`\x1BT`@QcM\x98R\xF3`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9B0\xA5\xE6\x90a\x123\x90\x86\x90`\x04\x01a'\xA7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12t\x91\x90a'\xD2V[`\x1BT`@Qc\xD8ni}`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xD8ni}\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xEF\x91\x90a*`V[P\x94P\x94P\x94P\x94P\x94P\x80`'`\0\x88\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`)\x88\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x04\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01UPP`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xE43=\xB5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xFF\x91\x90a(\x12V[`%`\x11a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xD6{l\xA5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x9C\x91\x90a(\x12V[`&\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPP[`&\x80T\x91\x15\x15`\x01`\x80\x1B\x02`\xFF`\x80\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UPPPV[a\x14\xFA\x82`\0`\x01`#Ta\x05\xFA\x91\x90a'xV[`\0\x81\x81R`(` R`@\x90 T\x90\x92P`\xFF\x16a\x15\x17WPPV[a\x15-\x81`\x01`\x01`@\x1B\x03\x16`\x01`\na\x1DVV[\x90P`\0`\x1E`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPP\x90P\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xCAf\x9F\xA7`!\x85\x81T\x81\x10a\x15\xBEWa\x15\xBEa'\x91V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x0BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\x1FW=`\0\x80>=`\0\xFD[PP`\x1BT`@\x80Qc\x1D\xC7\xDB\xD1`\xE2\x1B\x81R\x85Q`\x04\x82\x01R` \x86\x01Q`$\x82\x01R\x90\x85\x01Q`D\x82\x01R``\x85\x01Q`d\x82\x01R`\x01`\x01`@\x1B\x03\x86\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcw\x1FoD\x91P`\xA4\x01`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x16\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xBE\x91\x90a*\x0FV[PPPPPV[a\x16\xDD\x82`\xFF\x16`\0`\x01`#Ta\x05\xFA\x91\x90a'xV[`\x1BT`@\x80Qc\x0B\x14\xC1a`\xE2\x1B\x81R\x81Q\x93\x95P`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c,S\x05\x84\x92`\x04\x80\x82\x01\x93\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x17&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17J\x91\x90a*\x0FV[`$\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`@\x1B`\x01`\x01`@\x1B\x03\x93\x84\x16\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x92\x82\x16\x92\x90\x92\x17\x90\x91U`\x1CT`\0\x91a\x17\xA8\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\xFF\x86\x16\x90\x85\x16a\x1D\x9AV[\x90P`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xA6\xE2\xE3\xDC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18!\x91\x90a(\x12V[`$`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x16\xFE\xFE\xD7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xBE\x91\x90a(\x12V[`$\x80T`\x01`\x01`\xC0\x1B\x03\x16`\x01`\xC0\x1B`\x01`\x01`@\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90U`%\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UPPV[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0C9W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x19>\x90a*\xE7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19j\x90a*\xE7V[\x80\x15a\x19\xB7W\x80`\x1F\x10a\x19\x8CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x19\xB7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19\x9AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x19\x1FV[`)\x81\x81T\x81\x10a\x19\xDBW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x93P\x91\x90\x84V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0C9W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x1A\xD3W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1A\x95W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1A)V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0C9W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x1B.\x90a*\xE7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1BZ\x90a*\xE7V[\x80\x15a\x1B\xA7W\x80`\x1F\x10a\x1B|Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1B\xA7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B\x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1B\x0FV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x1B\xDBWP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x1C\xE1W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x1Ci\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a+!V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1C\x83\x91a+RV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x1C\xC0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1C\xC5V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x1C\xDD\x91\x90a*>V[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x86W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\thWPPPPP\x90P\x90V[`\x1F\x81\x81T\x81\x10a\x19\xDBW`\0\x80\xFD[`\0a\x1Dc\x84\x84\x84a\"\x06V[\x90Pa\x1D\x93`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x14\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82a#\xCEV[\x93\x92PPPV[`\0\x80\x83\x90P`\0`!\x85\x81T\x81\x10a\x1D\xB5Wa\x1D\xB5a'\x91V[`\0\x91\x82R` \x82 \x01T`@Qc5\xD6\x9C\x89`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01\x81\x90R`\xFF\x85\x16`$\x83\x01R\x92P\x81\x90\x81\x900\x90ck\xAD9\x12\x90`D\x01a\x01\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1E\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E=\x91\x90a+nV[\x92P\x92P\x92P`\0a\x1ER\x88`\x01`\na\x1DVV[`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x16`\x04\x82\x01R\x90\x91Pb\x01\x86\xA0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E\xAFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\xC3W=`\0\x80>=`\0\xFD[PP`\x1DT`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`\x01`\x01`@\x1B\x03\x87\x16`$\x83\x01R\x90\x91\x16\x92Pc\xA9\x05\x9C\xBB\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1F\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FF\x91\x90a*>V[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\x9CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F\xB0W=`\0\x80>=`\0\xFD[PP`\x1DT`\x1BT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`\x01`\x01`@\x1B\x03\x87\x16`$\x82\x01R\x91\x16\x92Pc\t^\xA7\xB3\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a \x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a 5\x91\x90a*>V[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \x8BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a \x9FW=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xC7,\xC7\x17`\xE0\x1B\x81R`\0\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xC7,\xC7\x17\x90a \xE0\x90\x89\x90\x89\x90\x88\x90\x87\x90\x8B\x90\x8A\x90`\x04\x01a+\xB5V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a \xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!#\x91\x90a*>V[\x90P\x80\x15a!\xF7W`\0\x8B\x81R`(` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x1BT\x90QcM\x98R\xF3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9B0\xA5\xE6\x90a!t\x90\x8A\x90`\x04\x01a'\xA7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xB5\x91\x90a'\xD2V[`\0\x90\x81R`\"` \x90\x81R`@\x80\x83 \x8F\x90U\x8E\x83R`\x1E\x82R\x91\x82\x90 \x89Q\x81U\x90\x89\x01Q`\x01\x82\x01U\x90\x88\x01Q`\x02\x82\x01U``\x88\x01Q`\x03\x90\x91\x01UP[\x9B\x9APPPPPPPPPPPV[`\0\x81\x83\x11\x15a\"\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\"\x92WP\x81\x84\x11\x15[\x15a\"\x9EWP\x82a\x1D\x93V[`\0a\"\xAA\x84\x84a'xV[a\"\xB5\x90`\x01a,JV[\x90P`\x03\x85\x11\x15\x80\x15a\"\xC7WP\x84\x81\x11[\x15a\"\xDEWa\"\xD6\x85\x85a,JV[\x91PPa\x1D\x93V[a\"\xEB`\x03`\0\x19a'xV[\x85\x10\x15\x80\x15a#\x04WPa#\x01\x85`\0\x19a'xV[\x81\x11[\x15a#\x1FWa#\x15\x85`\0\x19a'xV[a\"\xD6\x90\x84a'xV[\x82\x85\x11\x15a#uW`\0a#3\x84\x87a'xV[\x90P`\0a#A\x83\x83a,]V[\x90P\x80`\0\x03a#VW\x84\x93PPPPa\x1D\x93V[`\x01a#b\x82\x88a,JV[a#l\x91\x90a'xV[\x93PPPa#\xC6V[\x83\x85\x10\x15a#\xC6W`\0a#\x89\x86\x86a'xV[\x90P`\0a#\x97\x83\x83a,]V[\x90P\x80`\0\x03a#\xACW\x85\x93PPPPa\x1D\x93V[a#\xB6\x81\x86a'xV[a#\xC1\x90`\x01a,JV[\x93PPP[P\x93\x92PPPV[`\0jconsole.log`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Q`$\x01a#\xF8\x92\x91\x90a,\x7FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90RQa$-\x91\x90a+RV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a$hW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a$mV[``\x91P[PPPPPPV[`\0` \x82\x84\x03\x12\x15a$\x87W`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a$\xCFW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a$\xAAV[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a%\x81W\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a%lW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a%BV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a%\x05V[P\x91\x99\x98PPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a%\xA5W`\0\x80\xFD[PV[\x805`\xFF\x81\x16\x81\x14a\x1C\xE1W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a%\xCCW`\0\x80\xFD[\x825a%\xD7\x81a%\x90V[\x91Pa%\xE5` \x84\x01a%\xA8V[\x90P\x92P\x92\x90PV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R`@\x80\x86\x01Q\x90\x83\x01R``\x80\x86\x01Q\x90\x83\x01R\x83Q`\x80\x83\x01R\x83\x81\x01Q`\xA0\x83\x01R\x82Q`\xC0\x83\x01R\x82\x01Q`\xE0\x82\x01Ra\x01\0\x81\x01[\x94\x93PPPPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a%\xA5W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a&eW`\0\x80\xFD[\x825\x91P` \x83\x015a&w\x81a&=V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a&\x95W`\0\x80\xFD[a&\x9E\x83a%\xA8V[\x91P` \x83\x015a&w\x81a&=V[`\0[\x83\x81\x10\x15a&\xC9W\x81\x81\x01Q\x83\x82\x01R` \x01a&\xB1V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra&\xEA\x81` \x86\x01` \x86\x01a&\xAEV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a'UW`?\x19\x88\x86\x03\x01\x84Ra'C\x85\x83Qa&\xD2V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a''V[P\x92\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a'\x8BWa'\x8Ba'bV[\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x81\x01a'\x8BV[`\0` \x82\x84\x03\x12\x15a'\xE4W`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a(\x0BWa(\x0Ba'bV[P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a($W`\0\x80\xFD[\x81Qa\x1D\x93\x81a&=V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\x01`\x01`@\x1B\x03\x80\x84\x11\x15a(uWa(ua(EV[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a(\x9DWa(\x9Da(EV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a(\xB6W`\0\x80\xFD[a(\xC4\x86` \x83\x01\x87a&\xAEV[PPP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a(\xE0W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xF6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a)\x07W`\0\x80\xFD[a&5\x84\x82Q` \x84\x01a([V[`\0`@\x82\x84\x03\x12\x15a)(W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a)JWa)Ja(EV[`@R\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a)vW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a)\x98Wa)\x98a(EV[\x80`@RP\x80\x91P\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01RP\x92\x91PPV[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a)\xE0W`\0\x80\xFD[a)\xEA\x86\x86a)\x16V[\x93Pa)\xF9\x86`@\x87\x01a)dV[`\xC0\x86\x01Q`\xE0\x90\x96\x01Q\x94\x97\x90\x96P\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a*\"W`\0\x80\xFD[\x82Qa*-\x81a&=V[` \x84\x01Q\x90\x92Pa&w\x81a&=V[`\0` \x82\x84\x03\x12\x15a*PW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1D\x93W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xE0\x87\x89\x03\x12\x15a*yW`\0\x80\xFD[\x86Qa*\x84\x81a%\x90V[` \x88\x01Q\x90\x96P`\x02\x81\x10a*\x99W`\0\x80\xFD[`@\x88\x01Q\x90\x95Pa*\xAA\x81a&=V[``\x88\x01Q\x90\x94Pa*\xBB\x81a&=V[`\x80\x88\x01Q\x90\x93Pa*\xCC\x81a&=V[\x91Pa*\xDB\x88`\xA0\x89\x01a)\x16V[\x90P\x92\x95P\x92\x95P\x92\x95V[`\x01\x81\x81\x1C\x90\x82\x16\x80a*\xFBW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a+\x1BWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a+D\x81`\x04\x85\x01` \x87\x01a&\xAEV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa+d\x81\x84` \x87\x01a&\xAEV[\x91\x90\x91\x01\x92\x91PPV[`\0\x80`\0a\x01\0\x84\x86\x03\x12\x15a+\x84W`\0\x80\xFD[a+\x8E\x85\x85a)dV[\x92Pa+\x9D\x85`\x80\x86\x01a)\x16V[\x91Pa+\xAC\x85`\xC0\x86\x01a)\x16V[\x90P\x92P\x92P\x92V[\x86Q\x81R` \x80\x88\x01Q\x90\x82\x01R`@\x80\x88\x01Q\x90\x82\x01R``\x80\x88\x01Q\x90\x82\x01Ra\x01`\x81\x01\x86Q`\x80\x83\x01R` \x87\x01Q`\xA0\x83\x01R`\x01`\x01`@\x1B\x03\x80\x87\x16`\xC0\x84\x01R`\x02\x86\x10a,\x1BWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\xE0\x83\x01\x86\x90R\x84Qa\x01\0\x84\x01R` \x85\x01Qa\x01 \x84\x01R\x80\x84\x16a\x01@\x84\x01RP\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a'\x8BWa'\x8Ba'bV[`\0\x82a,zWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`@\x81R`\0a,\x92`@\x83\x01\x85a&\xD2V[\x90P\x82` \x83\x01R\x93\x92PPPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static STAKETABLEHANDLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct StakeTableHandler<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StakeTableHandler<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StakeTableHandler<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StakeTableHandler<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StakeTableHandler<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StakeTableHandler))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StakeTableHandler<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                STAKETABLEHANDLER_ABI.clone(),
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
                STAKETABLEHANDLER_ABI.clone(),
                STAKETABLEHANDLER_BYTECODE.clone().into(),
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
        ///Calls the contract's `genClientWallet` (0x6bad3912) function
        pub fn gen_client_wallet(
            &self,
            sender: ::ethers::core::types::Address,
            seed: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, (G2Point, EdOnBN254Point, G1Point)>
        {
            self.0
                .method_hash([107, 173, 57, 18], (sender, seed))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isUserRegistered` (0xe0a58ee1) function
        pub fn is_user_registered(
            &self,
            user_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([224, 165, 142, 225], user_index)
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
        ///Calls the contract's `numberUsers` (0xbb4710c5) function
        pub fn number_users(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([187, 71, 16, 197], ())
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
        ///Calls the contract's `register` (0x7faeb4ef) function
        pub fn register(
            &self,
            user_index: u8,
            amount: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 174, 180, 239], (user_index, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registrationSuccessful` (0xf5a06e9d) function
        pub fn registration_successful(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([245, 160, 110, 157], ())
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
        ///Calls the contract's `requestExitKeys` (0x8c62c3ad) function
        pub fn request_exit_keys(
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
                .method_hash([140, 98, 195, 173], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestExitSuccessful` (0x2a1bf764) function
        pub fn request_exit_successful(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([42, 27, 247, 100], ())
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
        ///Calls the contract's `userIndexFromVk` (0x3fccf10f) function
        pub fn user_index_from_vk(
            &self,
            vk_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([63, 204, 241, 15], vk_hash)
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeTableHandlerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for StakeTableHandler<M>
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
    pub enum StakeTableHandlerEvents {
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
    impl ::ethers::contract::EthLogDecode for StakeTableHandlerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(StakeTableHandlerEvents::LogsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StakeTableHandlerEvents {
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
    impl ::core::convert::From<LogFilter> for StakeTableHandlerEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for StakeTableHandlerEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogArray1Filter> for StakeTableHandlerEvents {
        fn from(value: LogArray1Filter) -> Self {
            Self::LogArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogArray2Filter> for StakeTableHandlerEvents {
        fn from(value: LogArray2Filter) -> Self {
            Self::LogArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogArray3Filter> for StakeTableHandlerEvents {
        fn from(value: LogArray3Filter) -> Self {
            Self::LogArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for StakeTableHandlerEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for StakeTableHandlerEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for StakeTableHandlerEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedAddressFilter> for StakeTableHandlerEvents {
        fn from(value: LogNamedAddressFilter) -> Self {
            Self::LogNamedAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray1Filter> for StakeTableHandlerEvents {
        fn from(value: LogNamedArray1Filter) -> Self {
            Self::LogNamedArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray2Filter> for StakeTableHandlerEvents {
        fn from(value: LogNamedArray2Filter) -> Self {
            Self::LogNamedArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray3Filter> for StakeTableHandlerEvents {
        fn from(value: LogNamedArray3Filter) -> Self {
            Self::LogNamedArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytesFilter> for StakeTableHandlerEvents {
        fn from(value: LogNamedBytesFilter) -> Self {
            Self::LogNamedBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytes32Filter> for StakeTableHandlerEvents {
        fn from(value: LogNamedBytes32Filter) -> Self {
            Self::LogNamedBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalIntFilter> for StakeTableHandlerEvents {
        fn from(value: LogNamedDecimalIntFilter) -> Self {
            Self::LogNamedDecimalIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalUintFilter> for StakeTableHandlerEvents {
        fn from(value: LogNamedDecimalUintFilter) -> Self {
            Self::LogNamedDecimalUintFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedIntFilter> for StakeTableHandlerEvents {
        fn from(value: LogNamedIntFilter) -> Self {
            Self::LogNamedIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedStringFilter> for StakeTableHandlerEvents {
        fn from(value: LogNamedStringFilter) -> Self {
            Self::LogNamedStringFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedUintFilter> for StakeTableHandlerEvents {
        fn from(value: LogNamedUintFilter) -> Self {
            Self::LogNamedUintFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for StakeTableHandlerEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for StakeTableHandlerEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    impl ::core::convert::From<LogsFilter> for StakeTableHandlerEvents {
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
    ///Container type for all input parameters for the `genClientWallet` function with signature `genClientWallet(address,uint8)` and selector `0x6bad3912`
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
    #[ethcall(name = "genClientWallet", abi = "genClientWallet(address,uint8)")]
    pub struct GenClientWalletCall {
        pub sender: ::ethers::core::types::Address,
        pub seed: u8,
    }
    ///Container type for all input parameters for the `isUserRegistered` function with signature `isUserRegistered(uint256)` and selector `0xe0a58ee1`
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
    #[ethcall(name = "isUserRegistered", abi = "isUserRegistered(uint256)")]
    pub struct IsUserRegisteredCall {
        pub user_index: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `numberUsers` function with signature `numberUsers()` and selector `0xbb4710c5`
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
    #[ethcall(name = "numberUsers", abi = "numberUsers()")]
    pub struct NumberUsersCall;
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
    ///Container type for all input parameters for the `register` function with signature `register(uint8,uint64)` and selector `0x7faeb4ef`
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
    #[ethcall(name = "register", abi = "register(uint8,uint64)")]
    pub struct RegisterCall {
        pub user_index: u8,
        pub amount: u64,
    }
    ///Container type for all input parameters for the `registrationSuccessful` function with signature `registrationSuccessful()` and selector `0xf5a06e9d`
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
    #[ethcall(name = "registrationSuccessful", abi = "registrationSuccessful()")]
    pub struct RegistrationSuccessfulCall;
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
    ///Container type for all input parameters for the `requestExitKeys` function with signature `requestExitKeys(uint256)` and selector `0x8c62c3ad`
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
    #[ethcall(name = "requestExitKeys", abi = "requestExitKeys(uint256)")]
    pub struct RequestExitKeysCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `requestExitSuccessful` function with signature `requestExitSuccessful()` and selector `0x2a1bf764`
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
    #[ethcall(name = "requestExitSuccessful", abi = "requestExitSuccessful()")]
    pub struct RequestExitSuccessfulCall;
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
    ///Container type for all input parameters for the `userIndexFromVk` function with signature `userIndexFromVk(bytes32)` and selector `0x3fccf10f`
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
    #[ethcall(name = "userIndexFromVk", abi = "userIndexFromVk(bytes32)")]
    pub struct UserIndexFromVkCall {
        pub vk_hash: [u8; 32],
    }
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
    pub enum StakeTableHandlerCalls {
        IsTest(IsTestCall),
        AdvanceEpoch(AdvanceEpochCall),
        CurrentEpoch(CurrentEpochCall),
        Deposit(DepositCall),
        ExcludeArtifacts(ExcludeArtifactsCall),
        ExcludeContracts(ExcludeContractsCall),
        ExcludeSenders(ExcludeSendersCall),
        ExitEpochForBlsVK(ExitEpochForBlsVKCall),
        Failed(FailedCall),
        GenClientWallet(GenClientWalletCall),
        IsUserRegistered(IsUserRegisteredCall),
        LightClient(LightClientCall),
        NextExitEpochBefore(NextExitEpochBeforeCall),
        NextRegistrationEpochBefore(NextRegistrationEpochBeforeCall),
        NumberUsers(NumberUsersCall),
        PendingExitsBefore(PendingExitsBeforeCall),
        PendingRegistrationsBefore(PendingRegistrationsBeforeCall),
        Register(RegisterCall),
        RegistrationSuccessful(RegistrationSuccessfulCall),
        RequestExit(RequestExitCall),
        RequestExitKeys(RequestExitKeysCall),
        RequestExitSuccessful(RequestExitSuccessfulCall),
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
        UserIndexFromVk(UserIndexFromVkCall),
        Users(UsersCall),
        Vks(VksCall),
        VksWithdraw(VksWithdrawCall),
        WithdrawFunds(WithdrawFundsCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakeTableHandlerCalls {
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
            if let Ok(decoded) =
                <GenClientWalletCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GenClientWallet(decoded));
            }
            if let Ok(decoded) =
                <IsUserRegisteredCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsUserRegistered(decoded));
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
            if let Ok(decoded) = <NumberUsersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NumberUsers(decoded));
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
                <RegistrationSuccessfulCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegistrationSuccessful(decoded));
            }
            if let Ok(decoded) = <RequestExitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RequestExit(decoded));
            }
            if let Ok(decoded) =
                <RequestExitKeysCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestExitKeys(decoded));
            }
            if let Ok(decoded) =
                <RequestExitSuccessfulCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestExitSuccessful(decoded));
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
            if let Ok(decoded) =
                <UserIndexFromVkCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UserIndexFromVk(decoded));
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
    impl ::ethers::core::abi::AbiEncode for StakeTableHandlerCalls {
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
                Self::GenClientWallet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsUserRegistered(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LightClient(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NextExitEpochBefore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NextRegistrationEpochBefore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NumberUsers(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingExitsBefore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingRegistrationsBefore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Register(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegistrationSuccessful(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestExit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestExitKeys(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestExitSuccessful(element) => {
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
                Self::UserIndexFromVk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Users(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Vks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VksWithdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawFunds(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for StakeTableHandlerCalls {
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
                Self::GenClientWallet(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsUserRegistered(element) => ::core::fmt::Display::fmt(element, f),
                Self::LightClient(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextExitEpochBefore(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextRegistrationEpochBefore(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumberUsers(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingExitsBefore(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingRegistrationsBefore(element) => ::core::fmt::Display::fmt(element, f),
                Self::Register(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistrationSuccessful(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestExit(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestExitKeys(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestExitSuccessful(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::UserIndexFromVk(element) => ::core::fmt::Display::fmt(element, f),
                Self::Users(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vks(element) => ::core::fmt::Display::fmt(element, f),
                Self::VksWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFunds(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsTestCall> for StakeTableHandlerCalls {
        fn from(value: IsTestCall) -> Self {
            Self::IsTest(value)
        }
    }
    impl ::core::convert::From<AdvanceEpochCall> for StakeTableHandlerCalls {
        fn from(value: AdvanceEpochCall) -> Self {
            Self::AdvanceEpoch(value)
        }
    }
    impl ::core::convert::From<CurrentEpochCall> for StakeTableHandlerCalls {
        fn from(value: CurrentEpochCall) -> Self {
            Self::CurrentEpoch(value)
        }
    }
    impl ::core::convert::From<DepositCall> for StakeTableHandlerCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<ExcludeArtifactsCall> for StakeTableHandlerCalls {
        fn from(value: ExcludeArtifactsCall) -> Self {
            Self::ExcludeArtifacts(value)
        }
    }
    impl ::core::convert::From<ExcludeContractsCall> for StakeTableHandlerCalls {
        fn from(value: ExcludeContractsCall) -> Self {
            Self::ExcludeContracts(value)
        }
    }
    impl ::core::convert::From<ExcludeSendersCall> for StakeTableHandlerCalls {
        fn from(value: ExcludeSendersCall) -> Self {
            Self::ExcludeSenders(value)
        }
    }
    impl ::core::convert::From<ExitEpochForBlsVKCall> for StakeTableHandlerCalls {
        fn from(value: ExitEpochForBlsVKCall) -> Self {
            Self::ExitEpochForBlsVK(value)
        }
    }
    impl ::core::convert::From<FailedCall> for StakeTableHandlerCalls {
        fn from(value: FailedCall) -> Self {
            Self::Failed(value)
        }
    }
    impl ::core::convert::From<GenClientWalletCall> for StakeTableHandlerCalls {
        fn from(value: GenClientWalletCall) -> Self {
            Self::GenClientWallet(value)
        }
    }
    impl ::core::convert::From<IsUserRegisteredCall> for StakeTableHandlerCalls {
        fn from(value: IsUserRegisteredCall) -> Self {
            Self::IsUserRegistered(value)
        }
    }
    impl ::core::convert::From<LightClientCall> for StakeTableHandlerCalls {
        fn from(value: LightClientCall) -> Self {
            Self::LightClient(value)
        }
    }
    impl ::core::convert::From<NextExitEpochBeforeCall> for StakeTableHandlerCalls {
        fn from(value: NextExitEpochBeforeCall) -> Self {
            Self::NextExitEpochBefore(value)
        }
    }
    impl ::core::convert::From<NextRegistrationEpochBeforeCall> for StakeTableHandlerCalls {
        fn from(value: NextRegistrationEpochBeforeCall) -> Self {
            Self::NextRegistrationEpochBefore(value)
        }
    }
    impl ::core::convert::From<NumberUsersCall> for StakeTableHandlerCalls {
        fn from(value: NumberUsersCall) -> Self {
            Self::NumberUsers(value)
        }
    }
    impl ::core::convert::From<PendingExitsBeforeCall> for StakeTableHandlerCalls {
        fn from(value: PendingExitsBeforeCall) -> Self {
            Self::PendingExitsBefore(value)
        }
    }
    impl ::core::convert::From<PendingRegistrationsBeforeCall> for StakeTableHandlerCalls {
        fn from(value: PendingRegistrationsBeforeCall) -> Self {
            Self::PendingRegistrationsBefore(value)
        }
    }
    impl ::core::convert::From<RegisterCall> for StakeTableHandlerCalls {
        fn from(value: RegisterCall) -> Self {
            Self::Register(value)
        }
    }
    impl ::core::convert::From<RegistrationSuccessfulCall> for StakeTableHandlerCalls {
        fn from(value: RegistrationSuccessfulCall) -> Self {
            Self::RegistrationSuccessful(value)
        }
    }
    impl ::core::convert::From<RequestExitCall> for StakeTableHandlerCalls {
        fn from(value: RequestExitCall) -> Self {
            Self::RequestExit(value)
        }
    }
    impl ::core::convert::From<RequestExitKeysCall> for StakeTableHandlerCalls {
        fn from(value: RequestExitKeysCall) -> Self {
            Self::RequestExitKeys(value)
        }
    }
    impl ::core::convert::From<RequestExitSuccessfulCall> for StakeTableHandlerCalls {
        fn from(value: RequestExitSuccessfulCall) -> Self {
            Self::RequestExitSuccessful(value)
        }
    }
    impl ::core::convert::From<StakeTableCall> for StakeTableHandlerCalls {
        fn from(value: StakeTableCall) -> Self {
            Self::StakeTable(value)
        }
    }
    impl ::core::convert::From<StakeTableFirstAvailableExitEpochCall> for StakeTableHandlerCalls {
        fn from(value: StakeTableFirstAvailableExitEpochCall) -> Self {
            Self::StakeTableFirstAvailableExitEpoch(value)
        }
    }
    impl ::core::convert::From<StakeTableFirstAvailableRegistrationEpochCall>
        for StakeTableHandlerCalls
    {
        fn from(value: StakeTableFirstAvailableRegistrationEpochCall) -> Self {
            Self::StakeTableFirstAvailableRegistrationEpoch(value)
        }
    }
    impl ::core::convert::From<StakeTableNumPendingExitsCall> for StakeTableHandlerCalls {
        fn from(value: StakeTableNumPendingExitsCall) -> Self {
            Self::StakeTableNumPendingExits(value)
        }
    }
    impl ::core::convert::From<StakeTableNumPendingRegistrationsCall> for StakeTableHandlerCalls {
        fn from(value: StakeTableNumPendingRegistrationsCall) -> Self {
            Self::StakeTableNumPendingRegistrations(value)
        }
    }
    impl ::core::convert::From<TargetArtifactSelectorsCall> for StakeTableHandlerCalls {
        fn from(value: TargetArtifactSelectorsCall) -> Self {
            Self::TargetArtifactSelectors(value)
        }
    }
    impl ::core::convert::From<TargetArtifactsCall> for StakeTableHandlerCalls {
        fn from(value: TargetArtifactsCall) -> Self {
            Self::TargetArtifacts(value)
        }
    }
    impl ::core::convert::From<TargetContractsCall> for StakeTableHandlerCalls {
        fn from(value: TargetContractsCall) -> Self {
            Self::TargetContracts(value)
        }
    }
    impl ::core::convert::From<TargetSelectorsCall> for StakeTableHandlerCalls {
        fn from(value: TargetSelectorsCall) -> Self {
            Self::TargetSelectors(value)
        }
    }
    impl ::core::convert::From<TargetSendersCall> for StakeTableHandlerCalls {
        fn from(value: TargetSendersCall) -> Self {
            Self::TargetSenders(value)
        }
    }
    impl ::core::convert::From<TokenCall> for StakeTableHandlerCalls {
        fn from(value: TokenCall) -> Self {
            Self::Token(value)
        }
    }
    impl ::core::convert::From<TokenCreatorCall> for StakeTableHandlerCalls {
        fn from(value: TokenCreatorCall) -> Self {
            Self::TokenCreator(value)
        }
    }
    impl ::core::convert::From<UserIndexFromVkCall> for StakeTableHandlerCalls {
        fn from(value: UserIndexFromVkCall) -> Self {
            Self::UserIndexFromVk(value)
        }
    }
    impl ::core::convert::From<UsersCall> for StakeTableHandlerCalls {
        fn from(value: UsersCall) -> Self {
            Self::Users(value)
        }
    }
    impl ::core::convert::From<VksCall> for StakeTableHandlerCalls {
        fn from(value: VksCall) -> Self {
            Self::Vks(value)
        }
    }
    impl ::core::convert::From<VksWithdrawCall> for StakeTableHandlerCalls {
        fn from(value: VksWithdrawCall) -> Self {
            Self::VksWithdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawFundsCall> for StakeTableHandlerCalls {
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
    ///Container type for all return fields from the `genClientWallet` function with signature `genClientWallet(address,uint8)` and selector `0x6bad3912`
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
    pub struct GenClientWalletReturn(pub G2Point, pub EdOnBN254Point, pub G1Point);
    ///Container type for all return fields from the `isUserRegistered` function with signature `isUserRegistered(uint256)` and selector `0xe0a58ee1`
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
    pub struct IsUserRegisteredReturn {
        pub registered: bool,
    }
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
    ///Container type for all return fields from the `numberUsers` function with signature `numberUsers()` and selector `0xbb4710c5`
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
    pub struct NumberUsersReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `registrationSuccessful` function with signature `registrationSuccessful()` and selector `0xf5a06e9d`
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
    pub struct RegistrationSuccessfulReturn(pub bool);
    ///Container type for all return fields from the `requestExitKeys` function with signature `requestExitKeys(uint256)` and selector `0x8c62c3ad`
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
    pub struct RequestExitKeysReturn {
        pub x_0: ::ethers::core::types::U256,
        pub x_1: ::ethers::core::types::U256,
        pub y_0: ::ethers::core::types::U256,
        pub y_1: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `requestExitSuccessful` function with signature `requestExitSuccessful()` and selector `0x2a1bf764`
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
    pub struct RequestExitSuccessfulReturn(pub bool);
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
    ///Container type for all return fields from the `userIndexFromVk` function with signature `userIndexFromVk(bytes32)` and selector `0x3fccf10f`
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
    pub struct UserIndexFromVkReturn {
        pub user_index: ::ethers::core::types::U256,
    }
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
    ///`FuzzSelector(address,bytes4[])`
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
    pub struct FuzzSelector {
        pub addr: ::ethers::core::types::Address,
        pub selectors: ::std::vec::Vec<[u8; 4]>,
    }
}
