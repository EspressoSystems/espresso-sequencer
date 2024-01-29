pub use fee_contract_test::*;
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
pub mod fee_contract_test {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
                    ::std::borrow::ToOwned::to_owned("feeContract"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("feeContract"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract FeeContract"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setUp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setUp"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("testFuzz_deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("testFuzz_deposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("testFuzz_depositEvent"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("testFuzz_depositEvent",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("testFuzz_depositTwice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("testFuzz_depositTwice",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("user1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("user2"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount2"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("testFuzz_newUserHasZeroBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("testFuzz_newUserHasZeroBalance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
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
                    ::std::borrow::ToOwned::to_owned("testFuzz_noFunction"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("testFuzz_noFunction",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("testFuzz_nonExistentFunction"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("testFuzz_nonExistentFunction",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("test_depositMaxAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("test_depositMaxAmount",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("test_depositMinAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("test_depositMinAmount",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("test_invalidUserAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("test_invalidUserAddress",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
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
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
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
    pub static FEECONTRACTTEST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0-W`\0\x80\xFD[Pa1\xFD\x80a\0=`\09`\0\xF3\xFE`\x80`@R`\x046\x10b\0\x01CW`\x005`\xE0\x1C\x80cq\xC9\xCE\x1C\x11b\0\0\xB9W\x80c\xA5\xCA\xE1\xCC\x11b\0\0xW\x80c\xA5\xCA\xE1\xCC\x14b\0\x03\x02W\x80c\xA6\xCB\x0C\xDA\x14b\0\x03\x0CW\x80c\xB5P\x8A\xA9\x14b\0\x03$W\x80c\xBAAO\xA6\x14b\0\x03<W\x80c\xE2\x0C\x9Fq\x14b\0\x03eW\x80c\xFAv&\xD4\x14b\0\x03}W`\0\x80\xFD[\x80cq\xC9\xCE\x1C\x14b\0\x02\x8AW\x80cs\xA7dQ\x14b\0\x02\x94W\x80c\x85\"l\x81\x14b\0\x02\xABW\x80c\x8E\xC33\xB2\x14b\0\x02\xD2W\x80c\x91j\x17\xC6\x14b\0\x02\xEAW`\0\x80\xFD[\x80c?r\x86\xF4\x11b\0\x01\x06W\x80c?r\x86\xF4\x14b\0\x01\xF7W\x80cI\x18\xC1\xAD\x14b\0\x02\x0FW\x80cS\xB6\x87\xB2\x14b\0\x02'W\x80cc\x8E\xE8\x1C\x14b\0\x02>W\x80cf\xD9\xA9\xA0\x14b\0\x02cW`\0\x80\xFD[\x80c\x06\xE2\x97\x12\x14b\0\x01HW\x80c\n\x92T\xE4\x14b\0\x01\x87W\x80c\x1E\xD7\x83\x1C\x14b\0\x01\xA1W\x80c&\x12\xD5\x80\x14b\0\x01\xC8W\x80c>^<#\x14b\0\x01\xDFW[`\0\x80\xFD[4\x80\x15b\0\x01UW`\0\x80\xFD[P`\x1BTb\0\x01j\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15b\0\x01\x94W`\0\x80\xFD[Pb\0\x01\x9Fb\0\x03\x99V[\0[4\x80\x15b\0\x01\xAEW`\0\x80\xFD[Pb\0\x01\xB9b\0\x03\xE7V[`@Qb\0\x01~\x91\x90b\0\x1F\xB1V[b\0\x01\x9Fb\0\x01\xD96`\x04b\0 \x16V[b\0\x04KV[4\x80\x15b\0\x01\xECW`\0\x80\xFD[Pb\0\x01\xB9b\0\x07\xF9V[4\x80\x15b\0\x02\x04W`\0\x80\xFD[Pb\0\x01\xB9b\0\x08[V[4\x80\x15b\0\x02\x1CW`\0\x80\xFD[Pb\0\x01\x9Fb\0\x08\xBDV[b\0\x01\x9Fb\0\x0286`\x04b\0 EV[b\0\nKV[4\x80\x15b\0\x02KW`\0\x80\xFD[Pb\0\x01\x9Fb\0\x02]6`\x04b\0 \x90V[b\0\x0F)V[4\x80\x15b\0\x02pW`\0\x80\xFD[Pb\0\x02{b\0\x10\x1AV[`@Qb\0\x01~\x91\x90b\0 \xB0V[b\0\x01\x9Fb\0\x11\rV[b\0\x01\x9Fb\0\x02\xA56`\x04b\0 \x16V[b\0\x11\x94V[4\x80\x15b\0\x02\xB8W`\0\x80\xFD[Pb\0\x02\xC3b\0\x13\x05V[`@Qb\0\x01~\x91\x90b\0!\xBDV[4\x80\x15b\0\x02\xDFW`\0\x80\xFD[Pb\0\x01\x9Fb\0\x13\xDFV[4\x80\x15b\0\x02\xF7W`\0\x80\xFD[Pb\0\x02{b\0\x14\xD0V[b\0\x01\x9Fb\0\x15\xBAV[4\x80\x15b\0\x03\x19W`\0\x80\xFD[Pb\0\x01\x9Fb\0\x16&V[4\x80\x15b\0\x031W`\0\x80\xFD[Pb\0\x02\xC3b\0\x16mV[4\x80\x15b\0\x03IW`\0\x80\xFD[Pb\0\x03Tb\0\x17GV[`@Q\x90\x15\x15\x81R` \x01b\0\x01~V[4\x80\x15b\0\x03rW`\0\x80\xFD[Pb\0\x01\xB9b\0\x18rV[4\x80\x15b\0\x03\x8AW`\0\x80\xFD[P`\0Tb\0\x03T\x90`\xFF\x16\x81V[`@Qb\0\x03\xA7\x90b\0\x1F\xA3V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\x03\xC4W=`\0\x80>=`\0\xFD[P`\x1B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x04AW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x04\"W[PPPPP\x90P\x90V[`@Qc&1\xF2\xB1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x15`\x04\x82\x01R`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90cLc\xE5b\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x04\x9CW`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\x04\xB1W=`\0\x80>=`\0\xFD[PPPPb\0\x05\xB4\x81`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xA3\x0F\xEF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x054\x91\x90b\0\"%V[`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cL4\xA9\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x05\xAE\x91\x90b\0\"%V[b\0\x18\xD4V[`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x92\x93P`\0\x92\x90\x91\x16\x90c\xF8\xB2\xCBO\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x06\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06+\x91\x90b\0\"%V[`@Qc$\x8Ec\xE1`\xE1\x1B\x81R`\x01`\x04\x82\x01\x81\x90R`\0`$\x83\x01\x81\x90R`D\x83\x01R`d\x82\x01R\x90\x91P`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90cI\x1C\xC7\xC2\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x06\x8CW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x06\xA1W=`\0\x80>=`\0\xFD[PPPP\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C\x83`@Qb\0\x06\xE1\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x1BT`@Qc\xF3@\xFA\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xF3@\xFA\x01\x90\x84\x90`$\x01[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15b\0\x073W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x07HW=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R`\0\x95P\x90\x91\x16\x92Pc\xF8\xB2\xCBO\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x07\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x07\xC1\x91\x90b\0\"%V[\x90Pb\0\x07\xDA\x81b\0\x07\xD4\x85\x85b\0\"UV[b\0\x19\x1CV[`\x1BTb\0\x07\xF3\x90`\x01`\x01`\xA0\x1B\x03\x161\x84b\0\x19\x1CV[PPPPV[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x04AW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x04\"WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x04AW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x04\"WPPPPP\x90P\x90V[`\0b\0\x08\xE6`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c:\xB9\xB2\xB9`\xE1\x1B\x81RPb\0\x1AGV[\x90P`\0f#\x86\xF2o\xC1\0\0`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xA3\x0F\xEF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\tFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\tl\x91\x90b\0\"%V[b\0\tx\x91\x90b\0\"qV[`@Qc\x06\x18\xF5\x87`\xE5\x1B\x81Rck\xA4\xA1\xC7`\xE0\x1B`\x04\x82\x01R\x90\x91P`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xC3\x1E\xB0\xE0\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\t\xCBW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\t\xE0W=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF3@\xFA\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x90\x91\x16\x92Pc\xF3@\xFA\x01\x91P\x83\x90`$\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15b\0\n-W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\nBW=`\0\x80>=`\0\xFD[PPPPPPPV[`\0\x80Q` b\x001\xD1\x839\x81Q\x91RcLc\xE5b`\x01`\x01`\xA0\x1B\x03\x86\x16\x15\x80\x15\x90b\0\n\x81WP`\x01`\x01`\xA0\x1B\x03\x85\x16\x15\x15[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\n\xA2\x91\x15\x15\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\n\xBBW`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\n\xD0W=`\0\x80>=`\0\xFD[PPPPb\0\x0B-\x82`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xA3\x0F\xEF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05\x0EW=`\0\x80>=`\0\xFD[\x91Pb\0\x0B\x88\x81`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xA3\x0F\xEF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05\x0EW=`\0\x80>=`\0\xFD[`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R\x90\x91P`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0B\xDCW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0B\xF1W=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R`\0\x94P\x90\x91\x16\x91Pc\xF8\xB2\xCBO\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0CCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0Ci\x91\x90b\0\"%V[`\x1BT`@Qc\xF3@\xFA\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xF3@\xFA\x01\x90\x85\x90`$\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15b\0\x0C\xB4W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0C\xC9W=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`\0\x95P\x90\x91\x16\x92Pc\xF8\xB2\xCBO\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\r\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\rB\x91\x90b\0\"%V[\x90Pb\0\rU\x81b\0\x07\xD4\x86\x85b\0\"UV[`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\r\xA6W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\r\xBBW=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R`\0\x94P\x90\x91\x16\x91Pc\xF8\xB2\xCBO\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0E\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0E3\x91\x90b\0\"%V[`\x1BT`@Qc\xF3@\xFA\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xF3@\xFA\x01\x90\x86\x90`$\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15b\0\x0E~W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0E\x93W=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16`\x04\x83\x01R`\0\x95P\x90\x91\x16\x92Pc\xF8\xB2\xCBO\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0E\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0F\x0C\x91\x90b\0\"%V[\x90Pb\0\x0F\x1F\x81b\0\x07\xD4\x87\x85b\0\"UV[PPPPPPPPV[`@Qc&1\xF2\xB1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15`\x04\x82\x01R`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90cLc\xE5b\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x0FzW`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\x0F\x8FW=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`\0\x94P\x90\x91\x16\x91Pc\xF8\xB2\xCBO\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0F\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x10\x07\x91\x90b\0\"%V[\x90Pb\0\x10\x16\x81`\0b\0\x19\x1CV[PPV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\x11\x04W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15b\0\x10\xEBW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11b\0\x10\xACW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90b\0\x10>V[PPPP\x90P\x90V[`\x1BT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\0\x90\x82\x904\x90[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14b\0\x11fW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x11kV[``\x91P[PP\x90Pb\0\x11z\x81b\0\x1A[V[`\x1BTb\0\x10\x16\x90`\x01`\x01`\xA0\x1B\x03\x161`\0b\0\x19\x1CV[`@Qc&1\xF2\xB1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x15`\x04\x82\x01R`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90cLc\xE5b\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x11\xE5W`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\x11\xFAW=`\0\x80>=`\0\xFD[PPPPb\0\x12W\x81`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xA3\x0F\xEF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05\x0EW=`\0\x80>=`\0\xFD[`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x92\x93P`\0\x92\x90\x91\x16\x90c\xF8\xB2\xCBO\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x12\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x12\xCE\x91\x90b\0\"%V[`\x1BT`@Qc\xF3@\xFA\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xF3@\xFA\x01\x90\x84\x90`$\x01b\0\x07\x19V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\x11\x04W\x83\x82\x90`\0R` `\0 \x01\x80Tb\0\x13K\x90b\0\"\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x13y\x90b\0\"\x87V[\x80\x15b\0\x13\xCAW\x80`\x1F\x10b\0\x13\x9EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x13\xCAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x13\xACW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90b\0\x13)V[`\0b\0\x14\x08`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c:\xB9\xB2\xB9`\xE1\x1B\x81RPb\0\x1AGV[\x90P`\0`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cL4\xA9\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x14`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x14\x86\x91\x90b\0\"%V[b\0\x14\x93\x90`\x01b\0\"UV[`@Qc\x06\x18\xF5\x87`\xE5\x1B\x81Rc\xC5mF\xD3`\xE0\x1B`\x04\x82\x01R\x90\x91P`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xC3\x1E\xB0\xE0\x90`$\x01b\0\t\xB0V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\x11\x04W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15b\0\x15\xA1W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11b\0\x15bW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90b\0\x14\xF4V[`\x1BT`@Q` `$\x82\x01R`\x02`D\x82\x01Ra\x06\x0F`\xF3\x1B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\0\x90\x82\x904\x90`\x84\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cQ\xCF\xF8\xD9`\xE0\x1B\x17\x90RQb\0\x11'\x91\x90b\0\"\xC3V[`@Qc\x06\x18\xF5\x87`\xE5\x1B\x81Rc\x07\x02\xB3\xD9`\xE4\x1B`\x04\x82\x01R`\0\x90g\x06\xF0[Y\xD3\xB2\0\0\x90`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xC3\x1E\xB0\xE0\x90`$\x01b\0\t\xB0V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\x11\x04W\x83\x82\x90`\0R` `\0 \x01\x80Tb\0\x16\xB3\x90b\0\"\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x16\xE1\x90b\0\"\x87V[\x80\x15b\0\x172W\x80`\x1F\x10b\0\x17\x06Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x172V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x17\x14W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90b\0\x16\x91V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15b\0\x17hWP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0`\0\x80Q` b\x001\xD1\x839\x81Q\x91R;\x15b\0\x18mW`@\x80Q`\0\x80Q` b\x001\xD1\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91b\0\x17\xEF\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01b\0\"\xE1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x18\x0B\x91b\0\"\xC3V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14b\0\x18JW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x18OV[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90b\0\x18i\x91\x90b\0#\x14V[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x04AW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x04\"WPPPPP\x90P\x90V[`\0b\0\x18\xE3\x84\x84\x84b\0\x1AjV[\x90Pb\0\x19\x15`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x14\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82b\0\x1C`V[\x93\x92PPPV[\x80\x82\x14b\0\x10\x16W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qb\0\x19\x8F\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1b\0\x10\x16b\0\x1D\rV[`\0b\0\x1AT\x82b\0\x1E\x16V[P\x92\x91PPV[b\0\x1Ag\x81\x15b\0\x1F+V[PV[`\0\x81\x83\x11\x15b\0\x1A\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15b\0\x1A\xF8WP\x81\x84\x11\x15[\x15b\0\x1B\x06WP\x82b\0\x19\x15V[`\0b\0\x1B\x14\x84\x84b\0\"qV[b\0\x1B!\x90`\x01b\0\"UV[\x90P`\x03\x85\x11\x15\x80\x15b\0\x1B4WP\x84\x81\x11[\x15b\0\x1BOWb\0\x1BF\x85\x85b\0\"UV[\x91PPb\0\x19\x15V[b\0\x1B^`\x03`\0\x19b\0\"qV[\x85\x10\x15\x80\x15b\0\x1BzWPb\0\x1Bw\x85`\0\x19b\0\"qV[\x81\x11[\x15b\0\x1B\x9AWb\0\x1B\x8E\x85`\0\x19b\0\"qV[b\0\x1BF\x90\x84b\0\"qV[\x82\x85\x11\x15b\0\x1B\xFCW`\0b\0\x1B\xB1\x84\x87b\0\"qV[\x90P`\0b\0\x1B\xC1\x83\x83b\0#8V[\x90P\x80`\0\x03b\0\x1B\xD8W\x84\x93PPPPb\0\x19\x15V[`\x01b\0\x1B\xE6\x82\x88b\0\"UV[b\0\x1B\xF2\x91\x90b\0\"qV[\x93PPPb\0\x1CXV[\x83\x85\x10\x15b\0\x1CXW`\0b\0\x1C\x13\x86\x86b\0\"qV[\x90P`\0b\0\x1C#\x83\x83b\0#8V[\x90P\x80`\0\x03b\0\x1C:W\x85\x93PPPPb\0\x19\x15V[b\0\x1CF\x81\x86b\0\"qV[b\0\x1CS\x90`\x01b\0\"UV[\x93PPP[P\x93\x92PPPV[`\0jconsole.log`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Q`$\x01b\0\x1C\x8C\x92\x91\x90b\0#[V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90RQb\0\x1C\xC3\x91\x90b\0\"\xC3V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14b\0\x1D\0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x1D\x05V[``\x91P[PPPPPPV[`\0\x80Q` b\x001\xD1\x839\x81Q\x91R;\x15b\0\x1E\x05W`@\x80Q`\0\x80Q` b\x001\xD1\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x1D\xA0\x92\x91` \x01b\0\"\xE1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x1D\xBC\x91b\0\"\xC3V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14b\0\x1D\xFBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x1E\0V[``\x91P[PPPP[`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[`\0\x80\x82`@Q` \x01b\0\x1E,\x91\x90b\0\"\xC3V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01b^y\xB7`\xE0\x1B\x03\x19\x82R`\x04\x82\x01\x81\x90R\x91P`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xFF\xA1\x86I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x1E\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x1E\xB9\x91\x90b\0#\x7FV[`@Qc\x18\xCA\xF8\xE3`\xE3\x1B\x81R\x90\x92P`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xC6W\xC7\x18\x90b\0\x1E\xF2\x90\x85\x90\x87\x90`\x04\x01b\0#\x9FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x1F\rW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x1F\"W=`\0\x80>=`\0\xFD[PPPP\x91P\x91V[\x80b\0\x1AgW\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qb\0\x1F\x91\x90` \x80\x82R`\x17\x90\x82\x01R\x7FError: Assertion Failed\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1b\0\x1Agb\0\x1D\rV[a\x0E\x03\x80b\0#\xCE\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15b\0\x1F\xF4W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01b\0\x1F\xCDV[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x1AgW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15b\0 *W`\0\x80\xFD[\x825b\0 7\x81b\0 \0V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0 \\W`\0\x80\xFD[\x845b\0 i\x81b\0 \0V[\x93P` \x85\x015b\0 {\x81b\0 \0V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0` \x82\x84\x03\x12\x15b\0 \xA3W`\0\x80\xFD[\x815b\0\x19\x15\x81b\0 \0V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15b\0!ZW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15b\0!DW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90b\0!\x18V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01b\0 \xDAV[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15b\0!\x86W\x81\x81\x01Q\x83\x82\x01R` \x01b\0!lV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Rb\0!\xA9\x81` \x86\x01` \x86\x01b\0!iV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15b\0\"\x18W`?\x19\x88\x86\x03\x01\x84Rb\0\"\x05\x85\x83Qb\0!\x8FV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01b\0!\xE6V[P\x92\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15b\0\"8W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15b\0\"kWb\0\"kb\0\"?V[\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15b\0\"kWb\0\"kb\0\"?V[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\"\x9CW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\"\xBDWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x82Qb\0\"\xD7\x81\x84` \x87\x01b\0!iV[\x91\x90\x91\x01\x92\x91PPV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90b\0#\x06\x81`\x04\x85\x01` \x87\x01b\0!iV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15b\0#'W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14b\0\x19\x15W`\0\x80\xFD[`\0\x82b\0#VWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`@\x81R`\0b\0#p`@\x83\x01\x85b\0!\x8FV[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15b\0#\x92W`\0\x80\xFD[\x81Qb\0\x19\x15\x81b\0 \0V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90b\0#\xC5\x90\x83\x01\x84b\0!\x8FV[\x94\x93PPPPV\xFE`\xE0`@R0`\x80Rg\r\xE0\xB6\xB3\xA7d\0\0`\xA0Rg\x01cEx]\x8A\0\0`\xC0R4\x80\x15a\0,W`\0\x80\xFD[Pa\x005a\0:V[a\0\xECV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\0\x8AW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a\0\xE9W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80Q`\xA0Q`\xC0Qa\x0C\xCDa\x016`\09`\0\x81\x81`\xF4\x01Ra\x04\x83\x01R`\0\x81\x81a\x01h\x01Ra\x04\xC4\x01R`\0\x81\x81a\x05\xE8\x01R\x81\x81a\x06\x11\x01Ra\x07\x96\x01Ra\x0C\xCD`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xABW`\x005`\xE0\x1C\x80c\x81)\xFC\x1C\x11a\0dW\x80c\x81)\xFC\x1C\x14a\x01\xC9W\x80c\x8D\xA5\xCB[\x14a\x01\xDEW\x80c\xAD<\xB1\xCC\x14a\x02%W\x80c\xF2\xFD\xE3\x8B\x14a\x02cW\x80c\xF3@\xFA\x01\x14a\x02\x83W\x80c\xF8\xB2\xCBO\x14a\x02\x96Wa\0\xC9V[\x80c\x1E\xA3\x0F\xEF\x14a\0\xE2W\x80c'\xE25\xE3\x14a\x01)W\x80cL4\xA9\x82\x14a\x01VW\x80cO\x1E\xF2\x86\x14a\x01\x8AW\x80cR\xD1\x90-\x14a\x01\x9FW\x80cqP\x18\xA6\x14a\x01\xB4Wa\0\xC9V[6a\0\xC9W`@Qc\xBC\x8E\xCA\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x155\xAC_`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[4\x80\x15a\0\xEEW`\0\x80\xFD[Pa\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x015W`\0\x80\xFD[Pa\x01\x16a\x01D6`\x04a\x0B\0V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[4\x80\x15a\x01bW`\0\x80\xFD[Pa\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x9Da\x01\x986`\x04a\x0B1V[a\x02\xB6V[\0[4\x80\x15a\x01\xABW`\0\x80\xFD[Pa\x01\x16a\x02\xD5V[4\x80\x15a\x01\xC0W`\0\x80\xFD[Pa\x01\x9Da\x02\xF2V[4\x80\x15a\x01\xD5W`\0\x80\xFD[Pa\x01\x9Da\x03\x06V[4\x80\x15a\x01\xEAW`\0\x80\xFD[P\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01 V[4\x80\x15a\x021W`\0\x80\xFD[Pa\x02V`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x01 \x91\x90a\x0C\x17V[4\x80\x15a\x02oW`\0\x80\xFD[Pa\x01\x9Da\x02~6`\x04a\x0B\0V[a\x04\x1DV[a\x01\x9Da\x02\x916`\x04a\x0B\0V[a\x04`V[4\x80\x15a\x02\xA2W`\0\x80\xFD[Pa\x01\x16a\x02\xB16`\x04a\x0B\0V[a\x05\x98V[a\x02\xBEa\x05\xDDV[a\x02\xC7\x82a\x06\x82V[a\x02\xD1\x82\x82a\x06\xC9V[PPV[`\0a\x02\xDFa\x07\x8BV[P`\0\x80Q` a\x0C\xA1\x839\x81Q\x91R\x90V[a\x02\xFAa\x07\xD4V[a\x03\x04`\0a\x08/V[V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x81\x15\x80\x15a\x03LWP\x82[\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x03iWP0;\x15[\x90P\x81\x15\x80\x15a\x03wWP\x80\x15[\x15a\x03\x95W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x03\xBFW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x03\xC83a\x08\xA0V[a\x03\xD0a\x08\xB1V[\x83\x15a\x04\x16W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[a\x04%a\x07\xD4V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04TW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x04]\x81a\x08/V[PV[4`\0\x03a\x04\x81W`@Qc\x9E\xD2 \xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x004\x10\x15a\x04\xC2W`@Qck\xA4\xA1\xC7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x004\x11\x15a\x05\x03W`@Qc\xC5mF\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05*W`@Qc\x07\x02\xB3\xD9`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T4\x92\x90a\x05R\x90\x84\x90a\x0CJV[\x90\x91UPP`@Q4\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05\xC1W`@Qc\x07\x02\xB3\xD9`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x06dWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x06X`\0\x80Q` a\x0C\xA1\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x03\x04W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\x8Aa\x07\xD4V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7F\xF7\x87!\"n\xFE\x9A\x1B\xB6x\x18\x9A\x16\xD1UI(\xB9\xF2\x19.,\xB9>\xED\xA8;y\xFA@\0}\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x07#WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x07 \x91\x81\x01\x90a\x0CkV[`\x01[a\x07KW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04KV[`\0\x80Q` a\x0C\xA1\x839\x81Q\x91R\x81\x14a\x07|W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04KV[a\x07\x86\x83\x83a\x08\xB9V[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03\x04W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\x08\x06\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\x04W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x04KV[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPPV[a\x08\xA8a\t\x0FV[a\x04]\x81a\tXV[a\x03\x04a\t\x0FV[a\x08\xC2\x82a\t`V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\t\x07Wa\x07\x86\x82\x82a\t\xC5V[a\x02\xD1a\n=V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\x03\x04W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04%a\t\x0FV[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a\t\x96W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x04KV[`\0\x80Q` a\x0C\xA1\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\t\xE2\x91\x90a\x0C\x84V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\n\x1DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\"V[``\x91P[P\x91P\x91Pa\n2\x85\x83\x83a\n\\V[\x92PPP[\x92\x91PPV[4\x15a\x03\x04W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\nqWa\nl\x82a\n\xBBV[a\n\xB4V[\x81Q\x15\x80\x15a\n\x88WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\n\xB1W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x04KV[P\x80[\x93\x92PPPV[\x80Q\x15a\n\xCBW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xFBW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\x12W`\0\x80\xFD[a\n\xB4\x82a\n\xE4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0BDW`\0\x80\xFD[a\x0BM\x83a\n\xE4V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0BjW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0B~W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0B\x90Wa\x0B\x90a\x0B\x1BV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x0B\xB8Wa\x0B\xB8a\x0B\x1BV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x0B\xD1W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x0C\x0EW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\xF6V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0C6\x81`@\x85\x01` \x87\x01a\x0B\xF3V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\n7WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0C}W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82Qa\x0C\x96\x81\x84` \x87\x01a\x0B\xF3V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA1dsolcC\0\x08\x17\0\n\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static FEECONTRACTTEST_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10b\0\x01CW`\x005`\xE0\x1C\x80cq\xC9\xCE\x1C\x11b\0\0\xB9W\x80c\xA5\xCA\xE1\xCC\x11b\0\0xW\x80c\xA5\xCA\xE1\xCC\x14b\0\x03\x02W\x80c\xA6\xCB\x0C\xDA\x14b\0\x03\x0CW\x80c\xB5P\x8A\xA9\x14b\0\x03$W\x80c\xBAAO\xA6\x14b\0\x03<W\x80c\xE2\x0C\x9Fq\x14b\0\x03eW\x80c\xFAv&\xD4\x14b\0\x03}W`\0\x80\xFD[\x80cq\xC9\xCE\x1C\x14b\0\x02\x8AW\x80cs\xA7dQ\x14b\0\x02\x94W\x80c\x85\"l\x81\x14b\0\x02\xABW\x80c\x8E\xC33\xB2\x14b\0\x02\xD2W\x80c\x91j\x17\xC6\x14b\0\x02\xEAW`\0\x80\xFD[\x80c?r\x86\xF4\x11b\0\x01\x06W\x80c?r\x86\xF4\x14b\0\x01\xF7W\x80cI\x18\xC1\xAD\x14b\0\x02\x0FW\x80cS\xB6\x87\xB2\x14b\0\x02'W\x80cc\x8E\xE8\x1C\x14b\0\x02>W\x80cf\xD9\xA9\xA0\x14b\0\x02cW`\0\x80\xFD[\x80c\x06\xE2\x97\x12\x14b\0\x01HW\x80c\n\x92T\xE4\x14b\0\x01\x87W\x80c\x1E\xD7\x83\x1C\x14b\0\x01\xA1W\x80c&\x12\xD5\x80\x14b\0\x01\xC8W\x80c>^<#\x14b\0\x01\xDFW[`\0\x80\xFD[4\x80\x15b\0\x01UW`\0\x80\xFD[P`\x1BTb\0\x01j\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15b\0\x01\x94W`\0\x80\xFD[Pb\0\x01\x9Fb\0\x03\x99V[\0[4\x80\x15b\0\x01\xAEW`\0\x80\xFD[Pb\0\x01\xB9b\0\x03\xE7V[`@Qb\0\x01~\x91\x90b\0\x1F\xB1V[b\0\x01\x9Fb\0\x01\xD96`\x04b\0 \x16V[b\0\x04KV[4\x80\x15b\0\x01\xECW`\0\x80\xFD[Pb\0\x01\xB9b\0\x07\xF9V[4\x80\x15b\0\x02\x04W`\0\x80\xFD[Pb\0\x01\xB9b\0\x08[V[4\x80\x15b\0\x02\x1CW`\0\x80\xFD[Pb\0\x01\x9Fb\0\x08\xBDV[b\0\x01\x9Fb\0\x0286`\x04b\0 EV[b\0\nKV[4\x80\x15b\0\x02KW`\0\x80\xFD[Pb\0\x01\x9Fb\0\x02]6`\x04b\0 \x90V[b\0\x0F)V[4\x80\x15b\0\x02pW`\0\x80\xFD[Pb\0\x02{b\0\x10\x1AV[`@Qb\0\x01~\x91\x90b\0 \xB0V[b\0\x01\x9Fb\0\x11\rV[b\0\x01\x9Fb\0\x02\xA56`\x04b\0 \x16V[b\0\x11\x94V[4\x80\x15b\0\x02\xB8W`\0\x80\xFD[Pb\0\x02\xC3b\0\x13\x05V[`@Qb\0\x01~\x91\x90b\0!\xBDV[4\x80\x15b\0\x02\xDFW`\0\x80\xFD[Pb\0\x01\x9Fb\0\x13\xDFV[4\x80\x15b\0\x02\xF7W`\0\x80\xFD[Pb\0\x02{b\0\x14\xD0V[b\0\x01\x9Fb\0\x15\xBAV[4\x80\x15b\0\x03\x19W`\0\x80\xFD[Pb\0\x01\x9Fb\0\x16&V[4\x80\x15b\0\x031W`\0\x80\xFD[Pb\0\x02\xC3b\0\x16mV[4\x80\x15b\0\x03IW`\0\x80\xFD[Pb\0\x03Tb\0\x17GV[`@Q\x90\x15\x15\x81R` \x01b\0\x01~V[4\x80\x15b\0\x03rW`\0\x80\xFD[Pb\0\x01\xB9b\0\x18rV[4\x80\x15b\0\x03\x8AW`\0\x80\xFD[P`\0Tb\0\x03T\x90`\xFF\x16\x81V[`@Qb\0\x03\xA7\x90b\0\x1F\xA3V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\x03\xC4W=`\0\x80>=`\0\xFD[P`\x1B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x04AW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x04\"W[PPPPP\x90P\x90V[`@Qc&1\xF2\xB1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x15`\x04\x82\x01R`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90cLc\xE5b\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x04\x9CW`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\x04\xB1W=`\0\x80>=`\0\xFD[PPPPb\0\x05\xB4\x81`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xA3\x0F\xEF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x054\x91\x90b\0\"%V[`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cL4\xA9\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x05\xAE\x91\x90b\0\"%V[b\0\x18\xD4V[`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x92\x93P`\0\x92\x90\x91\x16\x90c\xF8\xB2\xCBO\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x06\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06+\x91\x90b\0\"%V[`@Qc$\x8Ec\xE1`\xE1\x1B\x81R`\x01`\x04\x82\x01\x81\x90R`\0`$\x83\x01\x81\x90R`D\x83\x01R`d\x82\x01R\x90\x91P`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90cI\x1C\xC7\xC2\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x06\x8CW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x06\xA1W=`\0\x80>=`\0\xFD[PPPP\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C\x83`@Qb\0\x06\xE1\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x1BT`@Qc\xF3@\xFA\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xF3@\xFA\x01\x90\x84\x90`$\x01[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15b\0\x073W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x07HW=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R`\0\x95P\x90\x91\x16\x92Pc\xF8\xB2\xCBO\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x07\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x07\xC1\x91\x90b\0\"%V[\x90Pb\0\x07\xDA\x81b\0\x07\xD4\x85\x85b\0\"UV[b\0\x19\x1CV[`\x1BTb\0\x07\xF3\x90`\x01`\x01`\xA0\x1B\x03\x161\x84b\0\x19\x1CV[PPPPV[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x04AW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x04\"WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x04AW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x04\"WPPPPP\x90P\x90V[`\0b\0\x08\xE6`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c:\xB9\xB2\xB9`\xE1\x1B\x81RPb\0\x1AGV[\x90P`\0f#\x86\xF2o\xC1\0\0`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xA3\x0F\xEF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\tFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\tl\x91\x90b\0\"%V[b\0\tx\x91\x90b\0\"qV[`@Qc\x06\x18\xF5\x87`\xE5\x1B\x81Rck\xA4\xA1\xC7`\xE0\x1B`\x04\x82\x01R\x90\x91P`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xC3\x1E\xB0\xE0\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\t\xCBW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\t\xE0W=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF3@\xFA\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x90\x91\x16\x92Pc\xF3@\xFA\x01\x91P\x83\x90`$\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15b\0\n-W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\nBW=`\0\x80>=`\0\xFD[PPPPPPPV[`\0\x80Q` b\x001\xD1\x839\x81Q\x91RcLc\xE5b`\x01`\x01`\xA0\x1B\x03\x86\x16\x15\x80\x15\x90b\0\n\x81WP`\x01`\x01`\xA0\x1B\x03\x85\x16\x15\x15[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\n\xA2\x91\x15\x15\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\n\xBBW`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\n\xD0W=`\0\x80>=`\0\xFD[PPPPb\0\x0B-\x82`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xA3\x0F\xEF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05\x0EW=`\0\x80>=`\0\xFD[\x91Pb\0\x0B\x88\x81`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xA3\x0F\xEF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05\x0EW=`\0\x80>=`\0\xFD[`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R\x90\x91P`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0B\xDCW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0B\xF1W=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R`\0\x94P\x90\x91\x16\x91Pc\xF8\xB2\xCBO\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0CCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0Ci\x91\x90b\0\"%V[`\x1BT`@Qc\xF3@\xFA\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xF3@\xFA\x01\x90\x85\x90`$\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15b\0\x0C\xB4W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0C\xC9W=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`\0\x95P\x90\x91\x16\x92Pc\xF8\xB2\xCBO\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\r\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\rB\x91\x90b\0\"%V[\x90Pb\0\rU\x81b\0\x07\xD4\x86\x85b\0\"UV[`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\r\xA6W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\r\xBBW=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R`\0\x94P\x90\x91\x16\x91Pc\xF8\xB2\xCBO\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0E\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0E3\x91\x90b\0\"%V[`\x1BT`@Qc\xF3@\xFA\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xF3@\xFA\x01\x90\x86\x90`$\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15b\0\x0E~W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0E\x93W=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16`\x04\x83\x01R`\0\x95P\x90\x91\x16\x92Pc\xF8\xB2\xCBO\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0E\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0F\x0C\x91\x90b\0\"%V[\x90Pb\0\x0F\x1F\x81b\0\x07\xD4\x87\x85b\0\"UV[PPPPPPPPV[`@Qc&1\xF2\xB1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15`\x04\x82\x01R`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90cLc\xE5b\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x0FzW`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\x0F\x8FW=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`\0\x94P\x90\x91\x16\x91Pc\xF8\xB2\xCBO\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x0F\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x10\x07\x91\x90b\0\"%V[\x90Pb\0\x10\x16\x81`\0b\0\x19\x1CV[PPV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\x11\x04W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15b\0\x10\xEBW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11b\0\x10\xACW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90b\0\x10>V[PPPP\x90P\x90V[`\x1BT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\0\x90\x82\x904\x90[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14b\0\x11fW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x11kV[``\x91P[PP\x90Pb\0\x11z\x81b\0\x1A[V[`\x1BTb\0\x10\x16\x90`\x01`\x01`\xA0\x1B\x03\x161`\0b\0\x19\x1CV[`@Qc&1\xF2\xB1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x15`\x04\x82\x01R`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90cLc\xE5b\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x11\xE5W`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\x11\xFAW=`\0\x80>=`\0\xFD[PPPPb\0\x12W\x81`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x1E\xA3\x0F\xEF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05\x0EW=`\0\x80>=`\0\xFD[`\x1BT`@Qc\xF8\xB2\xCBO`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x92\x93P`\0\x92\x90\x91\x16\x90c\xF8\xB2\xCBO\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x12\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x12\xCE\x91\x90b\0\"%V[`\x1BT`@Qc\xF3@\xFA\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xF3@\xFA\x01\x90\x84\x90`$\x01b\0\x07\x19V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\x11\x04W\x83\x82\x90`\0R` `\0 \x01\x80Tb\0\x13K\x90b\0\"\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x13y\x90b\0\"\x87V[\x80\x15b\0\x13\xCAW\x80`\x1F\x10b\0\x13\x9EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x13\xCAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x13\xACW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90b\0\x13)V[`\0b\0\x14\x08`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c:\xB9\xB2\xB9`\xE1\x1B\x81RPb\0\x1AGV[\x90P`\0`\x1B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cL4\xA9\x82`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x14`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x14\x86\x91\x90b\0\"%V[b\0\x14\x93\x90`\x01b\0\"UV[`@Qc\x06\x18\xF5\x87`\xE5\x1B\x81Rc\xC5mF\xD3`\xE0\x1B`\x04\x82\x01R\x90\x91P`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xC3\x1E\xB0\xE0\x90`$\x01b\0\t\xB0V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\x11\x04W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15b\0\x15\xA1W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11b\0\x15bW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90b\0\x14\xF4V[`\x1BT`@Q` `$\x82\x01R`\x02`D\x82\x01Ra\x06\x0F`\xF3\x1B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\0\x90\x82\x904\x90`\x84\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cQ\xCF\xF8\xD9`\xE0\x1B\x17\x90RQb\0\x11'\x91\x90b\0\"\xC3V[`@Qc\x06\x18\xF5\x87`\xE5\x1B\x81Rc\x07\x02\xB3\xD9`\xE4\x1B`\x04\x82\x01R`\0\x90g\x06\xF0[Y\xD3\xB2\0\0\x90`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xC3\x1E\xB0\xE0\x90`$\x01b\0\t\xB0V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15b\0\x11\x04W\x83\x82\x90`\0R` `\0 \x01\x80Tb\0\x16\xB3\x90b\0\"\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x16\xE1\x90b\0\"\x87V[\x80\x15b\0\x172W\x80`\x1F\x10b\0\x17\x06Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x172V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x17\x14W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90b\0\x16\x91V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15b\0\x17hWP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0`\0\x80Q` b\x001\xD1\x839\x81Q\x91R;\x15b\0\x18mW`@\x80Q`\0\x80Q` b\x001\xD1\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91b\0\x17\xEF\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01b\0\"\xE1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x18\x0B\x91b\0\"\xC3V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14b\0\x18JW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x18OV[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90b\0\x18i\x91\x90b\0#\x14V[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15b\0\x04AW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11b\0\x04\"WPPPPP\x90P\x90V[`\0b\0\x18\xE3\x84\x84\x84b\0\x1AjV[\x90Pb\0\x19\x15`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x14\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82b\0\x1C`V[\x93\x92PPPV[\x80\x82\x14b\0\x10\x16W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qb\0\x19\x8F\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1b\0\x10\x16b\0\x1D\rV[`\0b\0\x1AT\x82b\0\x1E\x16V[P\x92\x91PPV[b\0\x1Ag\x81\x15b\0\x1F+V[PV[`\0\x81\x83\x11\x15b\0\x1A\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15b\0\x1A\xF8WP\x81\x84\x11\x15[\x15b\0\x1B\x06WP\x82b\0\x19\x15V[`\0b\0\x1B\x14\x84\x84b\0\"qV[b\0\x1B!\x90`\x01b\0\"UV[\x90P`\x03\x85\x11\x15\x80\x15b\0\x1B4WP\x84\x81\x11[\x15b\0\x1BOWb\0\x1BF\x85\x85b\0\"UV[\x91PPb\0\x19\x15V[b\0\x1B^`\x03`\0\x19b\0\"qV[\x85\x10\x15\x80\x15b\0\x1BzWPb\0\x1Bw\x85`\0\x19b\0\"qV[\x81\x11[\x15b\0\x1B\x9AWb\0\x1B\x8E\x85`\0\x19b\0\"qV[b\0\x1BF\x90\x84b\0\"qV[\x82\x85\x11\x15b\0\x1B\xFCW`\0b\0\x1B\xB1\x84\x87b\0\"qV[\x90P`\0b\0\x1B\xC1\x83\x83b\0#8V[\x90P\x80`\0\x03b\0\x1B\xD8W\x84\x93PPPPb\0\x19\x15V[`\x01b\0\x1B\xE6\x82\x88b\0\"UV[b\0\x1B\xF2\x91\x90b\0\"qV[\x93PPPb\0\x1CXV[\x83\x85\x10\x15b\0\x1CXW`\0b\0\x1C\x13\x86\x86b\0\"qV[\x90P`\0b\0\x1C#\x83\x83b\0#8V[\x90P\x80`\0\x03b\0\x1C:W\x85\x93PPPPb\0\x19\x15V[b\0\x1CF\x81\x86b\0\"qV[b\0\x1CS\x90`\x01b\0\"UV[\x93PPP[P\x93\x92PPPV[`\0jconsole.log`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Q`$\x01b\0\x1C\x8C\x92\x91\x90b\0#[V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90RQb\0\x1C\xC3\x91\x90b\0\"\xC3V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14b\0\x1D\0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x1D\x05V[``\x91P[PPPPPPV[`\0\x80Q` b\x001\xD1\x839\x81Q\x91R;\x15b\0\x1E\x05W`@\x80Q`\0\x80Q` b\x001\xD1\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x1D\xA0\x92\x91` \x01b\0\"\xE1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x1D\xBC\x91b\0\"\xC3V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14b\0\x1D\xFBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x1E\0V[``\x91P[PPPP[`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[`\0\x80\x82`@Q` \x01b\0\x1E,\x91\x90b\0\"\xC3V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01b^y\xB7`\xE0\x1B\x03\x19\x82R`\x04\x82\x01\x81\x90R\x91P`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xFF\xA1\x86I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x1E\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x1E\xB9\x91\x90b\0#\x7FV[`@Qc\x18\xCA\xF8\xE3`\xE3\x1B\x81R\x90\x92P`\0\x80Q` b\x001\xD1\x839\x81Q\x91R\x90c\xC6W\xC7\x18\x90b\0\x1E\xF2\x90\x85\x90\x87\x90`\x04\x01b\0#\x9FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x1F\rW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x1F\"W=`\0\x80>=`\0\xFD[PPPP\x91P\x91V[\x80b\0\x1AgW\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qb\0\x1F\x91\x90` \x80\x82R`\x17\x90\x82\x01R\x7FError: Assertion Failed\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1b\0\x1Agb\0\x1D\rV[a\x0E\x03\x80b\0#\xCE\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15b\0\x1F\xF4W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01b\0\x1F\xCDV[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x1AgW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15b\0 *W`\0\x80\xFD[\x825b\0 7\x81b\0 \0V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0 \\W`\0\x80\xFD[\x845b\0 i\x81b\0 \0V[\x93P` \x85\x015b\0 {\x81b\0 \0V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0` \x82\x84\x03\x12\x15b\0 \xA3W`\0\x80\xFD[\x815b\0\x19\x15\x81b\0 \0V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15b\0!ZW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15b\0!DW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90b\0!\x18V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01b\0 \xDAV[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15b\0!\x86W\x81\x81\x01Q\x83\x82\x01R` \x01b\0!lV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Rb\0!\xA9\x81` \x86\x01` \x86\x01b\0!iV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15b\0\"\x18W`?\x19\x88\x86\x03\x01\x84Rb\0\"\x05\x85\x83Qb\0!\x8FV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01b\0!\xE6V[P\x92\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15b\0\"8W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15b\0\"kWb\0\"kb\0\"?V[\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15b\0\"kWb\0\"kb\0\"?V[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\"\x9CW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\"\xBDWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x82Qb\0\"\xD7\x81\x84` \x87\x01b\0!iV[\x91\x90\x91\x01\x92\x91PPV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90b\0#\x06\x81`\x04\x85\x01` \x87\x01b\0!iV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15b\0#'W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14b\0\x19\x15W`\0\x80\xFD[`\0\x82b\0#VWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`@\x81R`\0b\0#p`@\x83\x01\x85b\0!\x8FV[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15b\0#\x92W`\0\x80\xFD[\x81Qb\0\x19\x15\x81b\0 \0V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90b\0#\xC5\x90\x83\x01\x84b\0!\x8FV[\x94\x93PPPPV\xFE`\xE0`@R0`\x80Rg\r\xE0\xB6\xB3\xA7d\0\0`\xA0Rg\x01cEx]\x8A\0\0`\xC0R4\x80\x15a\0,W`\0\x80\xFD[Pa\x005a\0:V[a\0\xECV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\0\x8AW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a\0\xE9W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80Q`\xA0Q`\xC0Qa\x0C\xCDa\x016`\09`\0\x81\x81`\xF4\x01Ra\x04\x83\x01R`\0\x81\x81a\x01h\x01Ra\x04\xC4\x01R`\0\x81\x81a\x05\xE8\x01R\x81\x81a\x06\x11\x01Ra\x07\x96\x01Ra\x0C\xCD`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xABW`\x005`\xE0\x1C\x80c\x81)\xFC\x1C\x11a\0dW\x80c\x81)\xFC\x1C\x14a\x01\xC9W\x80c\x8D\xA5\xCB[\x14a\x01\xDEW\x80c\xAD<\xB1\xCC\x14a\x02%W\x80c\xF2\xFD\xE3\x8B\x14a\x02cW\x80c\xF3@\xFA\x01\x14a\x02\x83W\x80c\xF8\xB2\xCBO\x14a\x02\x96Wa\0\xC9V[\x80c\x1E\xA3\x0F\xEF\x14a\0\xE2W\x80c'\xE25\xE3\x14a\x01)W\x80cL4\xA9\x82\x14a\x01VW\x80cO\x1E\xF2\x86\x14a\x01\x8AW\x80cR\xD1\x90-\x14a\x01\x9FW\x80cqP\x18\xA6\x14a\x01\xB4Wa\0\xC9V[6a\0\xC9W`@Qc\xBC\x8E\xCA\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x155\xAC_`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[4\x80\x15a\0\xEEW`\0\x80\xFD[Pa\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x015W`\0\x80\xFD[Pa\x01\x16a\x01D6`\x04a\x0B\0V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[4\x80\x15a\x01bW`\0\x80\xFD[Pa\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x9Da\x01\x986`\x04a\x0B1V[a\x02\xB6V[\0[4\x80\x15a\x01\xABW`\0\x80\xFD[Pa\x01\x16a\x02\xD5V[4\x80\x15a\x01\xC0W`\0\x80\xFD[Pa\x01\x9Da\x02\xF2V[4\x80\x15a\x01\xD5W`\0\x80\xFD[Pa\x01\x9Da\x03\x06V[4\x80\x15a\x01\xEAW`\0\x80\xFD[P\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01 V[4\x80\x15a\x021W`\0\x80\xFD[Pa\x02V`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x01 \x91\x90a\x0C\x17V[4\x80\x15a\x02oW`\0\x80\xFD[Pa\x01\x9Da\x02~6`\x04a\x0B\0V[a\x04\x1DV[a\x01\x9Da\x02\x916`\x04a\x0B\0V[a\x04`V[4\x80\x15a\x02\xA2W`\0\x80\xFD[Pa\x01\x16a\x02\xB16`\x04a\x0B\0V[a\x05\x98V[a\x02\xBEa\x05\xDDV[a\x02\xC7\x82a\x06\x82V[a\x02\xD1\x82\x82a\x06\xC9V[PPV[`\0a\x02\xDFa\x07\x8BV[P`\0\x80Q` a\x0C\xA1\x839\x81Q\x91R\x90V[a\x02\xFAa\x07\xD4V[a\x03\x04`\0a\x08/V[V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x81\x15\x80\x15a\x03LWP\x82[\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x03iWP0;\x15[\x90P\x81\x15\x80\x15a\x03wWP\x80\x15[\x15a\x03\x95W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x03\xBFW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x03\xC83a\x08\xA0V[a\x03\xD0a\x08\xB1V[\x83\x15a\x04\x16W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[a\x04%a\x07\xD4V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04TW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x04]\x81a\x08/V[PV[4`\0\x03a\x04\x81W`@Qc\x9E\xD2 \xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x004\x10\x15a\x04\xC2W`@Qck\xA4\xA1\xC7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x004\x11\x15a\x05\x03W`@Qc\xC5mF\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05*W`@Qc\x07\x02\xB3\xD9`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T4\x92\x90a\x05R\x90\x84\x90a\x0CJV[\x90\x91UPP`@Q4\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05\xC1W`@Qc\x07\x02\xB3\xD9`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x06dWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x06X`\0\x80Q` a\x0C\xA1\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x03\x04W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\x8Aa\x07\xD4V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7F\xF7\x87!\"n\xFE\x9A\x1B\xB6x\x18\x9A\x16\xD1UI(\xB9\xF2\x19.,\xB9>\xED\xA8;y\xFA@\0}\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x07#WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x07 \x91\x81\x01\x90a\x0CkV[`\x01[a\x07KW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04KV[`\0\x80Q` a\x0C\xA1\x839\x81Q\x91R\x81\x14a\x07|W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04KV[a\x07\x86\x83\x83a\x08\xB9V[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03\x04W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\x08\x06\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\x04W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x04KV[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPPV[a\x08\xA8a\t\x0FV[a\x04]\x81a\tXV[a\x03\x04a\t\x0FV[a\x08\xC2\x82a\t`V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\t\x07Wa\x07\x86\x82\x82a\t\xC5V[a\x02\xD1a\n=V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\x03\x04W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04%a\t\x0FV[\x80`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a\t\x96W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x04KV[`\0\x80Q` a\x0C\xA1\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\t\xE2\x91\x90a\x0C\x84V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\n\x1DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\"V[``\x91P[P\x91P\x91Pa\n2\x85\x83\x83a\n\\V[\x92PPP[\x92\x91PPV[4\x15a\x03\x04W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\nqWa\nl\x82a\n\xBBV[a\n\xB4V[\x81Q\x15\x80\x15a\n\x88WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\n\xB1W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x04KV[P\x80[\x93\x92PPPV[\x80Q\x15a\n\xCBW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xFBW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\x12W`\0\x80\xFD[a\n\xB4\x82a\n\xE4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0BDW`\0\x80\xFD[a\x0BM\x83a\n\xE4V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0BjW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0B~W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0B\x90Wa\x0B\x90a\x0B\x1BV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x0B\xB8Wa\x0B\xB8a\x0B\x1BV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x0B\xD1W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x0C\x0EW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\xF6V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0C6\x81`@\x85\x01` \x87\x01a\x0B\xF3V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\n7WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0C}W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82Qa\x0C\x96\x81\x84` \x87\x01a\x0B\xF3V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA1dsolcC\0\x08\x17\0\n\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static FEECONTRACTTEST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct FeeContractTest<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FeeContractTest<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FeeContractTest<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FeeContractTest<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FeeContractTest<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FeeContractTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FeeContractTest<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                FEECONTRACTTEST_ABI.clone(),
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
                FEECONTRACTTEST_ABI.clone(),
                FEECONTRACTTEST_BYTECODE.clone().into(),
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
        ///Calls the contract's `failed` (0xba414fa6) function
        pub fn failed(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeContract` (0x06e29712) function
        pub fn fee_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([6, 226, 151, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUp` (0x0a9254e4) function
        pub fn set_up(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
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
        ///Calls the contract's `testFuzz_deposit` (0x73a76451) function
        pub fn test_fuzz_deposit(
            &self,
            user: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 167, 100, 81], (user, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testFuzz_depositEvent` (0x2612d580) function
        pub fn test_fuzz_deposit_event(
            &self,
            user: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 18, 213, 128], (user, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testFuzz_depositTwice` (0x53b687b2) function
        pub fn test_fuzz_deposit_twice(
            &self,
            user_1: ::ethers::core::types::Address,
            user_2: ::ethers::core::types::Address,
            amount_1: ::ethers::core::types::U256,
            amount_2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 182, 135, 178], (user_1, user_2, amount_1, amount_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testFuzz_newUserHasZeroBalance` (0x638ee81c) function
        pub fn test_fuzz_new_user_has_zero_balance(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 142, 232, 28], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testFuzz_noFunction` (0x71c9ce1c) function
        pub fn test_fuzz_no_function(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 201, 206, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testFuzz_nonExistentFunction` (0xa5cae1cc) function
        pub fn test_fuzz_non_existent_function(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 202, 225, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `test_depositMaxAmount` (0x8ec333b2) function
        pub fn test_deposit_max_amount(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 195, 51, 178], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `test_depositMinAmount` (0x4918c1ad) function
        pub fn test_deposit_min_amount(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 24, 193, 173], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `test_invalidUserAddress` (0xa6cb0cda) function
        pub fn test_invalid_user_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 203, 12, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeeContractTestEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for FeeContractTest<M>
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
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
    pub enum FeeContractTestEvents {
        DepositFilter(DepositFilter),
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
    impl ::ethers::contract::EthLogDecode for FeeContractTestEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(FeeContractTestEvents::LogsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for FeeContractTestEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<DepositFilter> for FeeContractTestEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<LogFilter> for FeeContractTestEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for FeeContractTestEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogArray1Filter> for FeeContractTestEvents {
        fn from(value: LogArray1Filter) -> Self {
            Self::LogArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogArray2Filter> for FeeContractTestEvents {
        fn from(value: LogArray2Filter) -> Self {
            Self::LogArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogArray3Filter> for FeeContractTestEvents {
        fn from(value: LogArray3Filter) -> Self {
            Self::LogArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for FeeContractTestEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for FeeContractTestEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for FeeContractTestEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedAddressFilter> for FeeContractTestEvents {
        fn from(value: LogNamedAddressFilter) -> Self {
            Self::LogNamedAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray1Filter> for FeeContractTestEvents {
        fn from(value: LogNamedArray1Filter) -> Self {
            Self::LogNamedArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray2Filter> for FeeContractTestEvents {
        fn from(value: LogNamedArray2Filter) -> Self {
            Self::LogNamedArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray3Filter> for FeeContractTestEvents {
        fn from(value: LogNamedArray3Filter) -> Self {
            Self::LogNamedArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytesFilter> for FeeContractTestEvents {
        fn from(value: LogNamedBytesFilter) -> Self {
            Self::LogNamedBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytes32Filter> for FeeContractTestEvents {
        fn from(value: LogNamedBytes32Filter) -> Self {
            Self::LogNamedBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalIntFilter> for FeeContractTestEvents {
        fn from(value: LogNamedDecimalIntFilter) -> Self {
            Self::LogNamedDecimalIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalUintFilter> for FeeContractTestEvents {
        fn from(value: LogNamedDecimalUintFilter) -> Self {
            Self::LogNamedDecimalUintFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedIntFilter> for FeeContractTestEvents {
        fn from(value: LogNamedIntFilter) -> Self {
            Self::LogNamedIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedStringFilter> for FeeContractTestEvents {
        fn from(value: LogNamedStringFilter) -> Self {
            Self::LogNamedStringFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedUintFilter> for FeeContractTestEvents {
        fn from(value: LogNamedUintFilter) -> Self {
            Self::LogNamedUintFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for FeeContractTestEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for FeeContractTestEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    impl ::core::convert::From<LogsFilter> for FeeContractTestEvents {
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
    ///Container type for all input parameters for the `feeContract` function with signature `feeContract()` and selector `0x06e29712`
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
    #[ethcall(name = "feeContract", abi = "feeContract()")]
    pub struct FeeContractCall;
    ///Container type for all input parameters for the `setUp` function with signature `setUp()` and selector `0x0a9254e4`
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
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
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
    ///Container type for all input parameters for the `testFuzz_deposit` function with signature `testFuzz_deposit(address,uint256)` and selector `0x73a76451`
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
    #[ethcall(name = "testFuzz_deposit", abi = "testFuzz_deposit(address,uint256)")]
    pub struct TestFuzzDepositCall {
        pub user: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `testFuzz_depositEvent` function with signature `testFuzz_depositEvent(address,uint256)` and selector `0x2612d580`
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
        name = "testFuzz_depositEvent",
        abi = "testFuzz_depositEvent(address,uint256)"
    )]
    pub struct TestFuzzDepositEventCall {
        pub user: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `testFuzz_depositTwice` function with signature `testFuzz_depositTwice(address,address,uint256,uint256)` and selector `0x53b687b2`
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
        name = "testFuzz_depositTwice",
        abi = "testFuzz_depositTwice(address,address,uint256,uint256)"
    )]
    pub struct TestFuzzDepositTwiceCall {
        pub user_1: ::ethers::core::types::Address,
        pub user_2: ::ethers::core::types::Address,
        pub amount_1: ::ethers::core::types::U256,
        pub amount_2: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `testFuzz_newUserHasZeroBalance` function with signature `testFuzz_newUserHasZeroBalance(address)` and selector `0x638ee81c`
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
        name = "testFuzz_newUserHasZeroBalance",
        abi = "testFuzz_newUserHasZeroBalance(address)"
    )]
    pub struct TestFuzzNewUserHasZeroBalanceCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `testFuzz_noFunction` function with signature `testFuzz_noFunction()` and selector `0x71c9ce1c`
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
    #[ethcall(name = "testFuzz_noFunction", abi = "testFuzz_noFunction()")]
    pub struct TestFuzzNoFunctionCall;
    ///Container type for all input parameters for the `testFuzz_nonExistentFunction` function with signature `testFuzz_nonExistentFunction()` and selector `0xa5cae1cc`
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
        name = "testFuzz_nonExistentFunction",
        abi = "testFuzz_nonExistentFunction()"
    )]
    pub struct TestFuzzNonExistentFunctionCall;
    ///Container type for all input parameters for the `test_depositMaxAmount` function with signature `test_depositMaxAmount()` and selector `0x8ec333b2`
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
    #[ethcall(name = "test_depositMaxAmount", abi = "test_depositMaxAmount()")]
    pub struct TestDepositMaxAmountCall;
    ///Container type for all input parameters for the `test_depositMinAmount` function with signature `test_depositMinAmount()` and selector `0x4918c1ad`
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
    #[ethcall(name = "test_depositMinAmount", abi = "test_depositMinAmount()")]
    pub struct TestDepositMinAmountCall;
    ///Container type for all input parameters for the `test_invalidUserAddress` function with signature `test_invalidUserAddress()` and selector `0xa6cb0cda`
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
    #[ethcall(name = "test_invalidUserAddress", abi = "test_invalidUserAddress()")]
    pub struct TestInvalidUserAddressCall;
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
    pub enum FeeContractTestCalls {
        IsTest(IsTestCall),
        ExcludeArtifacts(ExcludeArtifactsCall),
        ExcludeContracts(ExcludeContractsCall),
        ExcludeSenders(ExcludeSendersCall),
        Failed(FailedCall),
        FeeContract(FeeContractCall),
        SetUp(SetUpCall),
        TargetArtifactSelectors(TargetArtifactSelectorsCall),
        TargetArtifacts(TargetArtifactsCall),
        TargetContracts(TargetContractsCall),
        TargetSelectors(TargetSelectorsCall),
        TargetSenders(TargetSendersCall),
        TestFuzzDeposit(TestFuzzDepositCall),
        TestFuzzDepositEvent(TestFuzzDepositEventCall),
        TestFuzzDepositTwice(TestFuzzDepositTwiceCall),
        TestFuzzNewUserHasZeroBalance(TestFuzzNewUserHasZeroBalanceCall),
        TestFuzzNoFunction(TestFuzzNoFunctionCall),
        TestFuzzNonExistentFunction(TestFuzzNonExistentFunctionCall),
        TestDepositMaxAmount(TestDepositMaxAmountCall),
        TestDepositMinAmount(TestDepositMinAmountCall),
        TestInvalidUserAddress(TestInvalidUserAddressCall),
    }
    impl ::ethers::core::abi::AbiDecode for FeeContractTestCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsTestCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsTest(decoded));
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
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Failed(decoded));
            }
            if let Ok(decoded) = <FeeContractCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeContract(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetUp(decoded));
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
            if let Ok(decoded) =
                <TestFuzzDepositCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestFuzzDeposit(decoded));
            }
            if let Ok(decoded) =
                <TestFuzzDepositEventCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestFuzzDepositEvent(decoded));
            }
            if let Ok(decoded) =
                <TestFuzzDepositTwiceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestFuzzDepositTwice(decoded));
            }
            if let Ok(decoded) =
                <TestFuzzNewUserHasZeroBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestFuzzNewUserHasZeroBalance(decoded));
            }
            if let Ok(decoded) =
                <TestFuzzNoFunctionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestFuzzNoFunction(decoded));
            }
            if let Ok(decoded) =
                <TestFuzzNonExistentFunctionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestFuzzNonExistentFunction(decoded));
            }
            if let Ok(decoded) =
                <TestDepositMaxAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestDepositMaxAmount(decoded));
            }
            if let Ok(decoded) =
                <TestDepositMinAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestDepositMinAmount(decoded));
            }
            if let Ok(decoded) =
                <TestInvalidUserAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestInvalidUserAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FeeContractTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsTest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeArtifacts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeContracts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeSenders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Failed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeContract(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetArtifactSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetArtifacts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetContracts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetSelectors(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetSenders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TestFuzzDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TestFuzzDepositEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TestFuzzDepositTwice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TestFuzzNewUserHasZeroBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TestFuzzNoFunction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TestFuzzNonExistentFunction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TestDepositMaxAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TestDepositMinAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TestInvalidUserAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FeeContractTestCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsTest(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::Failed(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifactSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestFuzzDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestFuzzDepositEvent(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestFuzzDepositTwice(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestFuzzNewUserHasZeroBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TestFuzzNoFunction(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestFuzzNonExistentFunction(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestDepositMaxAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestDepositMinAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestInvalidUserAddress(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsTestCall> for FeeContractTestCalls {
        fn from(value: IsTestCall) -> Self {
            Self::IsTest(value)
        }
    }
    impl ::core::convert::From<ExcludeArtifactsCall> for FeeContractTestCalls {
        fn from(value: ExcludeArtifactsCall) -> Self {
            Self::ExcludeArtifacts(value)
        }
    }
    impl ::core::convert::From<ExcludeContractsCall> for FeeContractTestCalls {
        fn from(value: ExcludeContractsCall) -> Self {
            Self::ExcludeContracts(value)
        }
    }
    impl ::core::convert::From<ExcludeSendersCall> for FeeContractTestCalls {
        fn from(value: ExcludeSendersCall) -> Self {
            Self::ExcludeSenders(value)
        }
    }
    impl ::core::convert::From<FailedCall> for FeeContractTestCalls {
        fn from(value: FailedCall) -> Self {
            Self::Failed(value)
        }
    }
    impl ::core::convert::From<FeeContractCall> for FeeContractTestCalls {
        fn from(value: FeeContractCall) -> Self {
            Self::FeeContract(value)
        }
    }
    impl ::core::convert::From<SetUpCall> for FeeContractTestCalls {
        fn from(value: SetUpCall) -> Self {
            Self::SetUp(value)
        }
    }
    impl ::core::convert::From<TargetArtifactSelectorsCall> for FeeContractTestCalls {
        fn from(value: TargetArtifactSelectorsCall) -> Self {
            Self::TargetArtifactSelectors(value)
        }
    }
    impl ::core::convert::From<TargetArtifactsCall> for FeeContractTestCalls {
        fn from(value: TargetArtifactsCall) -> Self {
            Self::TargetArtifacts(value)
        }
    }
    impl ::core::convert::From<TargetContractsCall> for FeeContractTestCalls {
        fn from(value: TargetContractsCall) -> Self {
            Self::TargetContracts(value)
        }
    }
    impl ::core::convert::From<TargetSelectorsCall> for FeeContractTestCalls {
        fn from(value: TargetSelectorsCall) -> Self {
            Self::TargetSelectors(value)
        }
    }
    impl ::core::convert::From<TargetSendersCall> for FeeContractTestCalls {
        fn from(value: TargetSendersCall) -> Self {
            Self::TargetSenders(value)
        }
    }
    impl ::core::convert::From<TestFuzzDepositCall> for FeeContractTestCalls {
        fn from(value: TestFuzzDepositCall) -> Self {
            Self::TestFuzzDeposit(value)
        }
    }
    impl ::core::convert::From<TestFuzzDepositEventCall> for FeeContractTestCalls {
        fn from(value: TestFuzzDepositEventCall) -> Self {
            Self::TestFuzzDepositEvent(value)
        }
    }
    impl ::core::convert::From<TestFuzzDepositTwiceCall> for FeeContractTestCalls {
        fn from(value: TestFuzzDepositTwiceCall) -> Self {
            Self::TestFuzzDepositTwice(value)
        }
    }
    impl ::core::convert::From<TestFuzzNewUserHasZeroBalanceCall> for FeeContractTestCalls {
        fn from(value: TestFuzzNewUserHasZeroBalanceCall) -> Self {
            Self::TestFuzzNewUserHasZeroBalance(value)
        }
    }
    impl ::core::convert::From<TestFuzzNoFunctionCall> for FeeContractTestCalls {
        fn from(value: TestFuzzNoFunctionCall) -> Self {
            Self::TestFuzzNoFunction(value)
        }
    }
    impl ::core::convert::From<TestFuzzNonExistentFunctionCall> for FeeContractTestCalls {
        fn from(value: TestFuzzNonExistentFunctionCall) -> Self {
            Self::TestFuzzNonExistentFunction(value)
        }
    }
    impl ::core::convert::From<TestDepositMaxAmountCall> for FeeContractTestCalls {
        fn from(value: TestDepositMaxAmountCall) -> Self {
            Self::TestDepositMaxAmount(value)
        }
    }
    impl ::core::convert::From<TestDepositMinAmountCall> for FeeContractTestCalls {
        fn from(value: TestDepositMinAmountCall) -> Self {
            Self::TestDepositMinAmount(value)
        }
    }
    impl ::core::convert::From<TestInvalidUserAddressCall> for FeeContractTestCalls {
        fn from(value: TestInvalidUserAddressCall) -> Self {
            Self::TestInvalidUserAddress(value)
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
    ///Container type for all return fields from the `feeContract` function with signature `feeContract()` and selector `0x06e29712`
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
    pub struct FeeContractReturn(pub ::ethers::core::types::Address);
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
}
