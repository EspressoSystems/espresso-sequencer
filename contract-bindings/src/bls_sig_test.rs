pub use bls_sig_test::*;
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
pub mod bls_sig_test {
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
                    ::std::borrow::ToOwned::to_owned("testFuzz_BLS_hashes_computation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("testFuzz_BLS_hashes_computation",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "testFuzz_RevertWhen_SignatureIsAnInvalidPoint",
                    ),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "testFuzz_RevertWhen_SignatureIsAnInvalidPoint",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("testFuzz_RevertWhen_SignatureIsInvalid"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "testFuzz_RevertWhen_SignatureIsInvalid",
                        ),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("exp"),
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
                    ::std::borrow::ToOwned::to_owned("testFuzz_RevertWhen_usingWrongVK"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("testFuzz_RevertWhen_usingWrongVK",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("exp"),
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
                    ::std::borrow::ToOwned::to_owned("test_SigVerification_Succeeds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("test_SigVerification_Succeeds",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("wrapVerifyBlsSig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("wrapVerifyBlsSig"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("message"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sig"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("vk"),
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
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
    pub static BLSSIG_TEST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0-W`\0\x80\xFD[Pa/\x81\x80a\0=`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\0W`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\x97W\x80c\xE1\xDC}\x94\x11a\0fW\x80c\xE1\xDC}\x94\x14a\x01\xC8W\x80c\xE2\x0C\x9Fq\x14a\x01\xDBW\x80c\xF2\x92\x92n\x14a\x01\xE3W\x80c\xFAv&\xD4\x14a\x01\xF6W`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x01\x8BW\x80c\x91j\x17\xC6\x14a\x01\xA0W\x80c\xB5P\x8A\xA9\x14a\x01\xA8W\x80c\xBAAO\xA6\x14a\x01\xB0W`\0\x80\xFD[\x80c?r\x86\xF4\x11a\0\xD3W\x80c?r\x86\xF4\x14a\x01HW\x80cJ\xB8\x94\xB6\x14a\x01PW\x80cf\xD9\xA9\xA0\x14a\x01cW\x80cwh+q\x14a\x01xW`\0\x80\xFD[\x80c\x0B\xF5\x0Fo\x14a\x01\x05W\x80c\x1E\xD7\x83\x1C\x14a\x01\x0FW\x80c#\xB8\xCD\xED\x14a\x01-W\x80c>^<#\x14a\x01@W[`\0\x80\xFD[a\x01\ra\x02\x03V[\0[a\x01\x17a\x02<V[`@Qa\x01$\x91\x90a$sV[`@Q\x80\x91\x03\x90\xF3[a\x01\ra\x01;6`\x04a%\xD1V[a\x02\x9EV[a\x01\x17a\x02\xA9V[a\x01\x17a\x03\tV[a\x01\ra\x01^6`\x04a&\x85V[a\x03iV[a\x01ka\x06[V[`@Qa\x01$\x91\x90a&\xAFV[a\x01\ra\x01\x866`\x04a'dV[a\x07JV[a\x01\x93a\x08qV[`@Qa\x01$\x91\x90a'\xCDV[a\x01ka\tAV[a\x01\x93a\n'V[a\x01\xB8a\n\xF7V[`@Q\x90\x15\x15\x81R` \x01a\x01$V[a\x01\ra\x01\xD66`\x04a(1V[a\x0C\x16V[a\x01\x17a\x0E_V[a\x01\ra\x01\xF16`\x04a(nV[a\x0E\xBFV[`\0Ta\x01\xB8\x90`\xFF\x16\x81V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaHi`\xF0\x1B` \x82\x01R`\0\x80a\x02(\x83a\x10\x8EV[\x91P\x91Pa\x027\x83\x82\x84a\x12\xB5V[PPPV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02\x94W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02vW[PPPPP\x90P\x90V[a\x027\x83\x83\x83a\x12\xB5V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02\x94W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02vWPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02\x94W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02vWPPPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaHi`\xF0\x1B` \x82\x01Ra\x03\x9Ag\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x90`\x01\x90a\x13MV[\x91P`\0\x80a\x03\xA8\x83a\x10\x8EV[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P`\0\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03\xC7W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x19\x1AY\x99\x8B]\x19\\\xDD`\xBA\x1B\x81RP\x81`\0\x81Q\x81\x10a\x04\x13Wa\x04\x13a(\x90V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x19\xD9[\x8B\\\x98[\x99\x1B\xDBKY\xCC\x8B\\\x1B\xDA[\x9D`j\x1B\x81RP\x81`\x01\x81Q\x81\x10a\x04]Wa\x04]a(\x90V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc4\x80Q\xD7`\xE1\x1B\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\0\x80Q` a/5\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xE3\x91\x90\x81\x01\x90a(\xD6V[\x81`\x02\x81Q\x81\x10a\x04\xF6Wa\x04\xF6a(\x90V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x89\x16\x04g`\xE0\x1B\x81R`\0\x90`\0\x80Q` a/5\x839\x81Q\x91R\x90c\x89\x16\x04g\x90a\x055\x90\x85\x90`\x04\x01a'\xCDV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05TW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05|\x91\x90\x81\x01\x90a(\xD6V[\x90P`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x05\x94\x91\x90a)cV[`@Qc\x06\x18\xF5\x87`\xE5\x1B\x81Rb\xCE\xD3\xE5`\xE4\x1B`\x04\x82\x01R\x90\x91P`\0\x80Q` a/5\x839\x81Q\x91R\x90c\xC3\x1E\xB0\xE0\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xE3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xF7W=`\0\x80>=`\0\xFD[PP`@Qc#\xB8\xCD\xED`\xE0\x1B\x81R0\x92Pc#\xB8\xCD\xED\x91Pa\x06\"\x90\x89\x90\x88\x90\x86\x90`\x04\x01a)\x7FV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06:W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x06NW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x07AW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x07)W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x06\xEBW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x06\x7FV[PPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaHi`\xF0\x1B` \x82\x01R\x81`\0a\x07\x92`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90Pa\x07\x9E\x81\x83a\x13\x91V[\x90P`\0a\x07\xAB\x84a\x10\x8EV[P`@Qc\x06\x18\xF5\x87`\xE5\x1B\x81Rb\xCE\xD3\xE5`\xE4\x1B`\x04\x82\x01R\x90\x91P`\0\x80Q` a/5\x839\x81Q\x91R\x90c\xC3\x1E\xB0\xE0\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xFBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x0FW=`\0\x80>=`\0\xFD[PP`@Qc#\xB8\xCD\xED`\xE0\x1B\x81R0\x92Pc#\xB8\xCD\xED\x91Pa\x08:\x90\x87\x90\x86\x90\x86\x90`\x04\x01a)\x7FV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08RW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x08fW=`\0\x80>=`\0\xFD[PPPPPPPPPV[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x07AW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x08\xB4\x90a)\xD8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xE0\x90a)\xD8V[\x80\x15a\t-W\x80`\x1F\x10a\t\x02Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t-V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x10W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x08\x95V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x07AW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\n\x0FW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\t\xD1W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\teV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x07AW\x83\x82\x90`\0R` `\0 \x01\x80Ta\nj\x90a)\xD8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x96\x90a)\xD8V[\x80\x15a\n\xE3W\x80`\x1F\x10a\n\xB8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xE3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xC6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\nKV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x0B\x17WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0`\0\x80Q` a/5\x839\x81Q\x91R;\x15a\x0C\x11W`@\x80Q`\0\x80Q` a/5\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x0B\x99\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a*\x12V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0B\xB3\x91a*CV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0B\xF0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xF5V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x0C\r\x91\x90a*_V[\x91PP[\x91\x90PV[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R`\0\x91\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C.W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x19\x1AY\x99\x8B]\x19\\\xDD`\xBA\x1B\x81RP\x81`\0\x81Q\x81\x10a\x0CzWa\x0Cza(\x90V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mgen-bls-hashes`\x90\x1B\x81RP\x81`\x01\x81Q\x81\x10a\x0C\xBFWa\x0C\xBFa(\x90V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qcq\xAA\xD1\r`\xE0\x1B\x81R`\0\x80Q` a/5\x839\x81Q\x91R\x90cq\xAA\xD1\r\x90a\x0C\xFB\x90\x85\x90`\x04\x01a*\x81V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r@\x91\x90\x81\x01\x90a(\xD6V[\x81`\x02\x81Q\x81\x10a\rSWa\rSa(\x90V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x89\x16\x04g`\xE0\x1B\x81R`\0\x90`\0\x80Q` a/5\x839\x81Q\x91R\x90c\x89\x16\x04g\x90a\r\x92\x90\x85\x90`\x04\x01a'\xCDV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\xD9\x91\x90\x81\x01\x90a(\xD6V[\x90P`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\r\xF2\x91\x90a*\xC4V[\x91P\x91Pa\x0E\x08\x82a\x0E\x03\x87a\x14:V[a\x16NV[a\x0EX\x81`@Q` \x01a\x0E\x1C\x91\x90a*\xF1V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0E4\x87a\x17AV[`@Q` \x01a\x0ED\x91\x90a*\xF1V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x183V[PPPPPV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02\x94W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02vWPPPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaHi`\xF0\x1B` \x82\x01R`\0a\x0E\xE3\x82a\x10\x8EV[P\x90P`\0a\x0F\x03\x85`\0\x80Q` a/U\x839\x81Q\x91R`\0\x19a\x13MV[\x90P`\0a\x0F\"\x85`\0\x80Q` a/U\x839\x81Q\x91R`\0\x19a\x13MV[`@\x80Q\x80\x82\x01\x82R\x84\x81R`\x02` \x82\x01R\x90Qc\xF2\x8D\xCE\xB3`\xE0\x1B\x81R\x91\x92P\x90`\0\x80Q` a/5\x839\x81Q\x91R\x90c\xF2\x8D\xCE\xB3\x90a\x0Fg\x90`\x04\x01a+\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\x81W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x95W=`\0\x80>=`\0\xFD[PP`@Qc#\xB8\xCD\xED`\xE0\x1B\x81R0\x92Pc#\xB8\xCD\xED\x91Pa\x0F\xC0\x90\x88\x90\x85\x90\x89\x90`\x04\x01a)\x7FV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xD8W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0F\xECW=`\0\x80>=`\0\xFD[PP`@\x80Q\x80\x82\x01\x82R`\n\x81R` \x81\x01\x86\x90R\x90Qc\xF2\x8D\xCE\xB3`\xE0\x1B\x81R\x90\x93P`\0\x80Q` a/5\x839\x81Q\x91R\x92Pc\xF2\x8D\xCE\xB3\x91Pa\x105\x90`\x04\x01a+\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10OW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10cW=`\0\x80>=`\0\xFD[PP`@Qc#\xB8\xCD\xED`\xE0\x1B\x81R0\x92Pc#\xB8\xCD\xED\x91Pa\x06\"\x90\x88\x90\x85\x90\x89\x90`\x04\x01a)\x7FV[a\x10\xB9`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x82Q`\x03\x80\x82R`\x80\x82\x01\x90\x94R\x91\x92\x90\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10\xE6W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x19\x1AY\x99\x8B]\x19\\\xDD`\xBA\x1B\x81RP\x81`\0\x81Q\x81\x10a\x112Wa\x112a(\x90V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jgen-bls-sig`\xA8\x1B\x81RP\x81`\x01\x81Q\x81\x10a\x11tWa\x11ta(\x90V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qcq\xAA\xD1\r`\xE0\x1B\x81R`\0\x80Q` a/5\x839\x81Q\x91R\x90cq\xAA\xD1\r\x90a\x11\xB0\x90\x87\x90`\x04\x01a*\x81V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\xF5\x91\x90\x81\x01\x90a(\xD6V[\x81`\x02\x81Q\x81\x10a\x12\x08Wa\x12\x08a(\x90V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x89\x16\x04g`\xE0\x1B\x81R`\0\x90`\0\x80Q` a/5\x839\x81Q\x91R\x90c\x89\x16\x04g\x90a\x12G\x90\x85\x90`\x04\x01a'\xCDV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\x8E\x91\x90\x81\x01\x90a(\xD6V[\x90P`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x12\xA7\x91\x90a+EV[\x90\x98\x90\x97P\x95PPPPPPV[a\x12\xBE\x82a\x18=V[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a/\x11`$\x919\x90P`\0\x84\x82`@Q` \x01a\x12\xF0\x92\x91\x90a+qV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\x13\x0C\x82a\x17AV[\x90Pa\x13)\x81\x85a\x13\x1C\x88a\x18\xA5V[a\x13$a\x19 V[a\x19\xF1V[a\x13EW`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0a\x13Z\x84\x84\x84a\x1A\xD3V[\x90Pa\x13\x8A`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x14\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82a\x1C\x97V[\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13\xADa$UV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x81\x01\x83\x90R`\0``\x83`\x80\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x13\xDFW`\0\x80\xFD[P\x80a\x142W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FBn254: scalar mul failed!\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[`\0\x80a\x14F\x83a\x1D1V[\x80Q\x90\x91P`0\x81\x14a\x14[Wa\x14[a+\xA0V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14vWa\x14va$\xC0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x14\xA0W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x15\x11W\x83`\x01a\x14\xBB\x83\x86a+\xCCV[a\x14\xC5\x91\x90a+\xCCV[\x81Q\x81\x10a\x14\xD5Wa\x14\xD5a(\x90V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x14\xF2Wa\x14\xF2a(\x90V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x14\xA6V[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x15\xA3W\x83\x81a\x15O\x85\x88a+\xCCV[a\x15Y\x91\x90a+\xDFV[\x81Q\x81\x10a\x15iWa\x15ia(\x90V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x15\x89Wa\x15\x89a(\x90V[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x15;V[P`\0a\x15\xAF\x82a \x85V[\x90Pa\x01\0`\0\x80Q` a/U\x839\x81Q\x91R`\0a\x15\xCF\x86\x89a+\xCCV[\x90P`\0[\x81\x81\x10\x15a\x16>W`\0\x88`\x01a\x15\xEB\x84\x86a+\xCCV[a\x15\xF5\x91\x90a+\xCCV[\x81Q\x81\x10a\x16\x05Wa\x16\x05a(\x90V[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x16\x1DWa\x16\x1Da+\xF2V[\x85\x87\t\x95P\x83\x80a\x160Wa\x160a+\xF2V[\x81\x87\x08\x95PP`\x01\x01a\x15\xD4V[P\x92\x9A\x99PPPPPPPPPPV[\x80\x82\x14a\x17=W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x16\xBF\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x81`@Qa\x16\xF6\x91\x90a,\x08V[`@Q\x80\x91\x03\x90\xA1\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82`@Qa\x17-\x91\x90a,@V[`@Q\x80\x91\x03\x90\xA1a\x17=a \xEDV[PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0a\x17`\x83a\x14:V[\x90P`\0\x80Q` a/U\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\x17\x89Wa\x17\x89a+\xF2V[\x84\x82\t\x90P\x82\x80a\x17\x9CWa\x17\x9Ca+\xF2V[\x82\x82\x08\x90P`\0\x80a\x17\xAD\x83a!\xEDV[\x92P\x90P[\x80a\x18\x16W\x84\x80a\x17\xC5Wa\x17\xC5a+\xF2V[`\x01\x87\x08\x95P\x84\x80a\x17\xD9Wa\x17\xD9a+\xF2V[\x86\x87\t\x92P\x84\x80a\x17\xECWa\x17\xECa+\xF2V[\x86\x84\t\x92P\x84\x80a\x17\xFFWa\x17\xFFa+\xF2V[\x84\x84\x08\x92Pa\x18\r\x83a!\xEDV[\x92P\x90Pa\x17\xB2V[P`@\x80Q\x80\x82\x01\x90\x91R\x94\x85R` \x85\x01RP\x91\x94\x93PPPPV[a\x17=\x82\x82a\"\xEBV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a/U\x839\x81Q\x91R\x91\x15\x90\x15\x16\x15a\x18eWPPPV[\x82Q` \x84\x01Q\x82`\x03\x84\x85\x85\x86\t\x85\t\x08\x83\x82\x83\t\x14\x83\x82\x10\x84\x84\x10\x16\x16\x93PPP\x81a\x027W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x14)\x90a+\x08V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x18\xCDWP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a/U\x839\x81Q\x91R\x84` \x01Qa\x19\0\x91\x90a,jV[a\x19\x18\x90`\0\x80Q` a/U\x839\x81Q\x91Ra+\xCCV[\x90R\x92\x91PPV[a\x19K`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R` \x87\x01Q`@\x82\x01R\x86Q``\x82\x01R``\x87\x01Q`\x80\x82\x01R`@\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R` \x85\x01Qa\x01\0\x82\x01R\x84Qa\x01 \x82\x01R``\x85\x01Qa\x01@\x82\x01R`@\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x1A\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x14)V[P\x15\x15\x95\x94PPPPPV[`\0\x81\x83\x11\x15a\x1BKW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01a\x14)V[\x82\x84\x10\x15\x80\x15a\x1B[WP\x81\x84\x11\x15[\x15a\x1BgWP\x82a\x13\x8AV[`\0a\x1Bs\x84\x84a+\xCCV[a\x1B~\x90`\x01a+\xDFV[\x90P`\x03\x85\x11\x15\x80\x15a\x1B\x90WP\x84\x81\x11[\x15a\x1B\xA7Wa\x1B\x9F\x85\x85a+\xDFV[\x91PPa\x13\x8AV[a\x1B\xB4`\x03`\0\x19a+\xCCV[\x85\x10\x15\x80\x15a\x1B\xCDWPa\x1B\xCA\x85`\0\x19a+\xCCV[\x81\x11[\x15a\x1B\xE8Wa\x1B\xDE\x85`\0\x19a+\xCCV[a\x1B\x9F\x90\x84a+\xCCV[\x82\x85\x11\x15a\x1C>W`\0a\x1B\xFC\x84\x87a+\xCCV[\x90P`\0a\x1C\n\x83\x83a,jV[\x90P\x80`\0\x03a\x1C\x1FW\x84\x93PPPPa\x13\x8AV[`\x01a\x1C+\x82\x88a+\xDFV[a\x1C5\x91\x90a+\xCCV[\x93PPPa\x1C\x8FV[\x83\x85\x10\x15a\x1C\x8FW`\0a\x1CR\x86\x86a+\xCCV[\x90P`\0a\x1C`\x83\x83a,jV[\x90P\x80`\0\x03a\x1CuW\x85\x93PPPPa\x13\x8AV[a\x1C\x7F\x81\x86a+\xCCV[a\x1C\x8A\x90`\x01a+\xDFV[\x93PPP[P\x93\x92PPPV[`\0jconsole.log`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Q`$\x01a\x1C\xC1\x92\x91\x90a,\x8CV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90RQa\x1C\xF6\x91\x90a*CV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x13EW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x13EV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x1Dr\x92\x91\x90a+qV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x1D\x99\x92\x91\x90a,\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x1D\xBB\x91\x90a,\xE2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x1D\xE5\x90\x83\x90\x83\x90` \x01a-\x04V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1EVWa\x1EVa$\xC0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1E\x80W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x1E\x98\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x1F\x03W\x81\x81\x81Q\x81\x10a\x1E\xC7Wa\x1E\xC7a(\x90V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x1E\xE4Wa\x1E\xE4a(\x90V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x1E\xACV[P`\0\x84`@Q` \x01a\x1F\x19\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x1F\xADW`\0\x83\x82\x81Q\x81\x10a\x1FTWa\x1FTa(\x90V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x1FqWa\x1Fqa(\x90V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x1F\x92\x92\x91\x90a-3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x98PP`\x01\x01a\x1F8V[P\x86\x88\x87`@Q` \x01a\x1F\xC3\x93\x92\x91\x90a-bV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x1F\xF1\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a \x12\x8A`\xFF\x8D\x16a+\xCCV[\x81\x10\x15a tW\x82\x81\x81Q\x81\x10a +Wa +a(\x90V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a E\x83\x8Da+\xDFV[\x81Q\x81\x10a UWa Ua(\x90V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a \x05V[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a \xE6W\x83\x81\x81Q\x81\x10a \xA5Wa \xA5a(\x90V[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a \xBD\x91\x90a-\xA3V[a \xC8\x90`\x02a.\x9EV[a \xD2\x91\x90a-\xA3V[a \xDC\x90\x83a+\xDFV[\x91P`\x01\x01a \x8AV[P\x92\x91PPV[`\0\x80Q` a/5\x839\x81Q\x91R;\x15a!\xDCW`@\x80Q`\0\x80Q` a/5\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra!{\x92\x91` \x01a*\x12V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra!\x95\x91a*CV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a!\xD2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a!\xD7V[``\x91P[PPPP[`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[`\0\x80`\0\x80`\0\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a/U\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x87``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x94PP`\0Q\x92P\x83a\"\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x14)V[\x80`\x01\x84\x90\x1B\x11\x15a\"\xCAWa\"\xC7\x83\x82a+\xCCV[\x92P[\x80\x80a\"\xD8Wa\"\xD8a+\xF2V[\x83\x84\t\x96\x90\x96\x14\x96\x91\x95P\x90\x93PPPPV[a\"\xF5\x82\x82a#\xD2V[a\x17=W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa#d\x90` \x80\x82R`#\x90\x82\x01R\x7FError: a == b not satisfied [byt`@\x82\x01Rbes]`\xE8\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xD2n\x16\xCA\xD4T\x87\x05\xE4\xC9\xE2\xD9O\x98\xEE\x91\xC2\x89\x08^\xE4%YO\xD5c_\xA2\x96L\xCF\x18\x81`@Qa#\x9B\x91\x90a.\xAAV[`@Q\x80\x91\x03\x90\xA1\x7F\xD2n\x16\xCA\xD4T\x87\x05\xE4\xC9\xE2\xD9O\x98\xEE\x91\xC2\x89\x08^\xE4%YO\xD5c_\xA2\x96L\xCF\x18\x82`@Qa\x17-\x91\x90a.\xE6V[\x80Q\x82Q`\x01\x91\x90\x03a$KW`\0[\x83Q\x81\x10\x15a$EW\x82\x81\x81Q\x81\x10a#\xFDWa#\xFDa(\x90V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x84\x82\x81Q\x81\x10a$$Wa$$a(\x90V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x14a$=W`\0\x91P[`\x01\x01a#\xE2V[Pa$OV[P`\0[\x92\x91PPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a$\xB4W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a$\x8FV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\xF9Wa$\xF9a$\xC0V[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\xF9Wa$\xF9a$\xC0V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%KWa%Ka$\xC0V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%mWa%ma$\xC0V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a%\x8CW`\0\x80\xFD[\x815a%\x9Fa%\x9A\x82a%SV[a%\"V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a%\xB4W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a%\xE7W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xFEW`\0\x80\xFD[a&\n\x87\x82\x88\x01a%{V[\x94PP`@`\x1F\x19\x82\x01\x12\x15a&\x1FW`\0\x80\xFD[a&'a$\xD6V[` \x86\x81\x015\x82R`@\x87\x015\x90\x82\x01R\x92P`\x80`_\x19\x82\x01\x12\x15a&LW`\0\x80\xFD[Pa&Ua$\xFFV[``\x85\x015\x81R`\x80\x85\x015` \x82\x01R`\xA0\x85\x015`@\x82\x01R`\xC0\x85\x015``\x82\x01R\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a&\x97W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x13\x8AW`\0\x80\xFD[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a'UW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a'@W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a'\x16V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a&\xD9V[P\x91\x99\x98PPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a'vW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a'\x98W\x81\x81\x01Q\x83\x82\x01R` \x01a'\x80V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra'\xB9\x81` \x86\x01` \x86\x01a'}V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a($W`?\x19\x88\x86\x03\x01\x84Ra(\x12\x85\x83Qa'\xA1V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a'\xF6V[P\x92\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a(CW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(ZW`\0\x80\xFD[a(f\x84\x82\x85\x01a%{V[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a(\x81W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0a(\xB4a%\x9A\x84a%SV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a(\xC8W`\0\x80\xFD[a\x13\x8A\x83` \x83\x01\x84a'}V[`\0` \x82\x84\x03\x12\x15a(\xE8W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\xFFW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a)\x10W`\0\x80\xFD[a(f\x84\x82Q` \x84\x01a(\xA6V[`\0`\x80\x82\x84\x03\x12\x15a)1W`\0\x80\xFD[a)9a$\xFFV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a)uW`\0\x80\xFD[a\x13\x8A\x83\x83a)\x1FV[`\xE0\x81R`\0a)\x92`\xE0\x83\x01\x86a'\xA1V[\x90Pa)\xAB` \x83\x01\x85\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x82Q``\x83\x01R` \x83\x01Q`\x80\x83\x01R`@\x83\x01Q`\xA0\x83\x01R``\x83\x01Q`\xC0\x83\x01R\x94\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a)\xECW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a*\x0CWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a*5\x81`\x04\x85\x01` \x87\x01a'}V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa*U\x81\x84` \x87\x01a'}V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a*qW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x13\x8AW`\0\x80\xFD[` \x81R`\0a\x13\x8A` \x83\x01\x84a'\xA1V[`\0`@\x82\x84\x03\x12\x15a*\xA6W`\0\x80\xFD[a*\xAEa$\xD6V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15a*\xD7W`\0\x80\xFD[\x82Q\x91Pa*\xE8\x84` \x85\x01a*\x94V[\x90P\x92P\x92\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a$OV[` \x81R`\0a$O` \x83\x01`\x17\x81R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0` \x82\x01R`@\x01\x90V[`\0\x80`\xC0\x83\x85\x03\x12\x15a+XW`\0\x80\xFD[a+b\x84\x84a)\x1FV[\x91Pa*\xE8\x84`\x80\x85\x01a*\x94V[`\0\x83Qa+\x83\x81\x84` \x88\x01a'}V[\x83Q\x90\x83\x01\x90a+\x97\x81\x83` \x88\x01a'}V[\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a$OWa$Oa+\xB6V[\x80\x82\x01\x80\x82\x11\x15a$OWa$Oa+\xB6V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`@\x81R`\0a,2`@\x83\x01`\n\x81Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B` \x82\x01R`@\x01\x90V[\x90P\x82` \x83\x01R\x92\x91PPV[`@\x81R`\0a,2`@\x83\x01`\n\x81Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B` \x82\x01R`@\x01\x90V[`\0\x82a,\x87WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`@\x81R`\0a,\x9F`@\x83\x01\x85a'\xA1V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x83Qa,\xC0\x81\x84` \x88\x01a'}V[`\0\x92\x01\x91\x82RP`\x01`\x01`\xF8\x1B\x03\x19\x91\x90\x91\x16`\x01\x82\x01R`\x02\x01\x91\x90PV[`\0\x82Qa,\xF4\x81\x84` \x87\x01a'}V[`\0\x92\x01\x91\x82RP`\x01\x01\x91\x90PV[`\0\x83Qa-\x16\x81\x84` \x88\x01a'}V[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x02\x01\x92\x91PPV[`\0\x83Qa-E\x81\x84` \x88\x01a'}V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x01\x01\x92\x91PPV[`\0\x84Qa-t\x81\x84` \x89\x01a'}V[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x91\x90\x93\x01\x90\x81R`\x01`\x01`\xF0\x1B\x03\x19\x91\x90\x91\x16`\x01\x82\x01R`\x03\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a$OWa$Oa+\xB6V[`\x01\x81\x81[\x80\x85\x11\x15a-\xF5W\x81`\0\x19\x04\x82\x11\x15a-\xDBWa-\xDBa+\xB6V[\x80\x85\x16\x15a-\xE8W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a-\xBFV[P\x92P\x92\x90PV[`\0\x82a.\x0CWP`\x01a$OV[\x81a.\x19WP`\0a$OV[\x81`\x01\x81\x14a./W`\x02\x81\x14a.9Wa.UV[`\x01\x91PPa$OV[`\xFF\x84\x11\x15a.JWa.Ja+\xB6V[PP`\x01\x82\x1Ba$OV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a.xWP\x81\x81\na$OV[a.\x82\x83\x83a-\xBAV[\x80`\0\x19\x04\x82\x11\x15a.\x96Wa.\x96a+\xB6V[\x02\x93\x92PPPV[`\0a\x13\x8A\x83\x83a-\xFDV[`@\x81R`\0a.\xD4`@\x83\x01`\n\x81Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B` \x82\x01R`@\x01\x90V[\x82\x81\x03` \x84\x01Ra(f\x81\x85a'\xA1V[`@\x81R`\0a.\xD4`@\x83\x01`\n\x81Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B` \x82\x01R`@\x01\x90V\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static BLSSIG_TEST_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\0W`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\x97W\x80c\xE1\xDC}\x94\x11a\0fW\x80c\xE1\xDC}\x94\x14a\x01\xC8W\x80c\xE2\x0C\x9Fq\x14a\x01\xDBW\x80c\xF2\x92\x92n\x14a\x01\xE3W\x80c\xFAv&\xD4\x14a\x01\xF6W`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x01\x8BW\x80c\x91j\x17\xC6\x14a\x01\xA0W\x80c\xB5P\x8A\xA9\x14a\x01\xA8W\x80c\xBAAO\xA6\x14a\x01\xB0W`\0\x80\xFD[\x80c?r\x86\xF4\x11a\0\xD3W\x80c?r\x86\xF4\x14a\x01HW\x80cJ\xB8\x94\xB6\x14a\x01PW\x80cf\xD9\xA9\xA0\x14a\x01cW\x80cwh+q\x14a\x01xW`\0\x80\xFD[\x80c\x0B\xF5\x0Fo\x14a\x01\x05W\x80c\x1E\xD7\x83\x1C\x14a\x01\x0FW\x80c#\xB8\xCD\xED\x14a\x01-W\x80c>^<#\x14a\x01@W[`\0\x80\xFD[a\x01\ra\x02\x03V[\0[a\x01\x17a\x02<V[`@Qa\x01$\x91\x90a$sV[`@Q\x80\x91\x03\x90\xF3[a\x01\ra\x01;6`\x04a%\xD1V[a\x02\x9EV[a\x01\x17a\x02\xA9V[a\x01\x17a\x03\tV[a\x01\ra\x01^6`\x04a&\x85V[a\x03iV[a\x01ka\x06[V[`@Qa\x01$\x91\x90a&\xAFV[a\x01\ra\x01\x866`\x04a'dV[a\x07JV[a\x01\x93a\x08qV[`@Qa\x01$\x91\x90a'\xCDV[a\x01ka\tAV[a\x01\x93a\n'V[a\x01\xB8a\n\xF7V[`@Q\x90\x15\x15\x81R` \x01a\x01$V[a\x01\ra\x01\xD66`\x04a(1V[a\x0C\x16V[a\x01\x17a\x0E_V[a\x01\ra\x01\xF16`\x04a(nV[a\x0E\xBFV[`\0Ta\x01\xB8\x90`\xFF\x16\x81V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaHi`\xF0\x1B` \x82\x01R`\0\x80a\x02(\x83a\x10\x8EV[\x91P\x91Pa\x027\x83\x82\x84a\x12\xB5V[PPPV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02\x94W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02vW[PPPPP\x90P\x90V[a\x027\x83\x83\x83a\x12\xB5V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02\x94W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02vWPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02\x94W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02vWPPPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaHi`\xF0\x1B` \x82\x01Ra\x03\x9Ag\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x90`\x01\x90a\x13MV[\x91P`\0\x80a\x03\xA8\x83a\x10\x8EV[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P`\0\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03\xC7W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x19\x1AY\x99\x8B]\x19\\\xDD`\xBA\x1B\x81RP\x81`\0\x81Q\x81\x10a\x04\x13Wa\x04\x13a(\x90V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x19\xD9[\x8B\\\x98[\x99\x1B\xDBKY\xCC\x8B\\\x1B\xDA[\x9D`j\x1B\x81RP\x81`\x01\x81Q\x81\x10a\x04]Wa\x04]a(\x90V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc4\x80Q\xD7`\xE1\x1B\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\0\x80Q` a/5\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xE3\x91\x90\x81\x01\x90a(\xD6V[\x81`\x02\x81Q\x81\x10a\x04\xF6Wa\x04\xF6a(\x90V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x89\x16\x04g`\xE0\x1B\x81R`\0\x90`\0\x80Q` a/5\x839\x81Q\x91R\x90c\x89\x16\x04g\x90a\x055\x90\x85\x90`\x04\x01a'\xCDV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05TW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05|\x91\x90\x81\x01\x90a(\xD6V[\x90P`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x05\x94\x91\x90a)cV[`@Qc\x06\x18\xF5\x87`\xE5\x1B\x81Rb\xCE\xD3\xE5`\xE4\x1B`\x04\x82\x01R\x90\x91P`\0\x80Q` a/5\x839\x81Q\x91R\x90c\xC3\x1E\xB0\xE0\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xE3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xF7W=`\0\x80>=`\0\xFD[PP`@Qc#\xB8\xCD\xED`\xE0\x1B\x81R0\x92Pc#\xB8\xCD\xED\x91Pa\x06\"\x90\x89\x90\x88\x90\x86\x90`\x04\x01a)\x7FV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06:W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x06NW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x07AW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x07)W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x06\xEBW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x06\x7FV[PPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaHi`\xF0\x1B` \x82\x01R\x81`\0a\x07\x92`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90Pa\x07\x9E\x81\x83a\x13\x91V[\x90P`\0a\x07\xAB\x84a\x10\x8EV[P`@Qc\x06\x18\xF5\x87`\xE5\x1B\x81Rb\xCE\xD3\xE5`\xE4\x1B`\x04\x82\x01R\x90\x91P`\0\x80Q` a/5\x839\x81Q\x91R\x90c\xC3\x1E\xB0\xE0\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xFBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x0FW=`\0\x80>=`\0\xFD[PP`@Qc#\xB8\xCD\xED`\xE0\x1B\x81R0\x92Pc#\xB8\xCD\xED\x91Pa\x08:\x90\x87\x90\x86\x90\x86\x90`\x04\x01a)\x7FV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08RW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x08fW=`\0\x80>=`\0\xFD[PPPPPPPPPV[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x07AW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x08\xB4\x90a)\xD8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xE0\x90a)\xD8V[\x80\x15a\t-W\x80`\x1F\x10a\t\x02Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t-V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x10W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x08\x95V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x07AW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\n\x0FW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\t\xD1W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\teV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x07AW\x83\x82\x90`\0R` `\0 \x01\x80Ta\nj\x90a)\xD8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x96\x90a)\xD8V[\x80\x15a\n\xE3W\x80`\x1F\x10a\n\xB8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xE3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xC6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\nKV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x0B\x17WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0`\0\x80Q` a/5\x839\x81Q\x91R;\x15a\x0C\x11W`@\x80Q`\0\x80Q` a/5\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x0B\x99\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a*\x12V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0B\xB3\x91a*CV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0B\xF0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xF5V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x0C\r\x91\x90a*_V[\x91PP[\x91\x90PV[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R`\0\x91\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C.W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x19\x1AY\x99\x8B]\x19\\\xDD`\xBA\x1B\x81RP\x81`\0\x81Q\x81\x10a\x0CzWa\x0Cza(\x90V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mgen-bls-hashes`\x90\x1B\x81RP\x81`\x01\x81Q\x81\x10a\x0C\xBFWa\x0C\xBFa(\x90V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qcq\xAA\xD1\r`\xE0\x1B\x81R`\0\x80Q` a/5\x839\x81Q\x91R\x90cq\xAA\xD1\r\x90a\x0C\xFB\x90\x85\x90`\x04\x01a*\x81V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r@\x91\x90\x81\x01\x90a(\xD6V[\x81`\x02\x81Q\x81\x10a\rSWa\rSa(\x90V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x89\x16\x04g`\xE0\x1B\x81R`\0\x90`\0\x80Q` a/5\x839\x81Q\x91R\x90c\x89\x16\x04g\x90a\r\x92\x90\x85\x90`\x04\x01a'\xCDV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\xD9\x91\x90\x81\x01\x90a(\xD6V[\x90P`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\r\xF2\x91\x90a*\xC4V[\x91P\x91Pa\x0E\x08\x82a\x0E\x03\x87a\x14:V[a\x16NV[a\x0EX\x81`@Q` \x01a\x0E\x1C\x91\x90a*\xF1V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0E4\x87a\x17AV[`@Q` \x01a\x0ED\x91\x90a*\xF1V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x183V[PPPPPV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02\x94W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02vWPPPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaHi`\xF0\x1B` \x82\x01R`\0a\x0E\xE3\x82a\x10\x8EV[P\x90P`\0a\x0F\x03\x85`\0\x80Q` a/U\x839\x81Q\x91R`\0\x19a\x13MV[\x90P`\0a\x0F\"\x85`\0\x80Q` a/U\x839\x81Q\x91R`\0\x19a\x13MV[`@\x80Q\x80\x82\x01\x82R\x84\x81R`\x02` \x82\x01R\x90Qc\xF2\x8D\xCE\xB3`\xE0\x1B\x81R\x91\x92P\x90`\0\x80Q` a/5\x839\x81Q\x91R\x90c\xF2\x8D\xCE\xB3\x90a\x0Fg\x90`\x04\x01a+\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\x81W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x95W=`\0\x80>=`\0\xFD[PP`@Qc#\xB8\xCD\xED`\xE0\x1B\x81R0\x92Pc#\xB8\xCD\xED\x91Pa\x0F\xC0\x90\x88\x90\x85\x90\x89\x90`\x04\x01a)\x7FV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xD8W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0F\xECW=`\0\x80>=`\0\xFD[PP`@\x80Q\x80\x82\x01\x82R`\n\x81R` \x81\x01\x86\x90R\x90Qc\xF2\x8D\xCE\xB3`\xE0\x1B\x81R\x90\x93P`\0\x80Q` a/5\x839\x81Q\x91R\x92Pc\xF2\x8D\xCE\xB3\x91Pa\x105\x90`\x04\x01a+\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10OW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10cW=`\0\x80>=`\0\xFD[PP`@Qc#\xB8\xCD\xED`\xE0\x1B\x81R0\x92Pc#\xB8\xCD\xED\x91Pa\x06\"\x90\x88\x90\x85\x90\x89\x90`\x04\x01a)\x7FV[a\x10\xB9`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x82Q`\x03\x80\x82R`\x80\x82\x01\x90\x94R\x91\x92\x90\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10\xE6W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x19\x1AY\x99\x8B]\x19\\\xDD`\xBA\x1B\x81RP\x81`\0\x81Q\x81\x10a\x112Wa\x112a(\x90V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jgen-bls-sig`\xA8\x1B\x81RP\x81`\x01\x81Q\x81\x10a\x11tWa\x11ta(\x90V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qcq\xAA\xD1\r`\xE0\x1B\x81R`\0\x80Q` a/5\x839\x81Q\x91R\x90cq\xAA\xD1\r\x90a\x11\xB0\x90\x87\x90`\x04\x01a*\x81V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\xF5\x91\x90\x81\x01\x90a(\xD6V[\x81`\x02\x81Q\x81\x10a\x12\x08Wa\x12\x08a(\x90V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x89\x16\x04g`\xE0\x1B\x81R`\0\x90`\0\x80Q` a/5\x839\x81Q\x91R\x90c\x89\x16\x04g\x90a\x12G\x90\x85\x90`\x04\x01a'\xCDV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\x8E\x91\x90\x81\x01\x90a(\xD6V[\x90P`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x12\xA7\x91\x90a+EV[\x90\x98\x90\x97P\x95PPPPPPV[a\x12\xBE\x82a\x18=V[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a/\x11`$\x919\x90P`\0\x84\x82`@Q` \x01a\x12\xF0\x92\x91\x90a+qV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\x13\x0C\x82a\x17AV[\x90Pa\x13)\x81\x85a\x13\x1C\x88a\x18\xA5V[a\x13$a\x19 V[a\x19\xF1V[a\x13EW`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0a\x13Z\x84\x84\x84a\x1A\xD3V[\x90Pa\x13\x8A`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x14\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82a\x1C\x97V[\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13\xADa$UV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x81\x01\x83\x90R`\0``\x83`\x80\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x13\xDFW`\0\x80\xFD[P\x80a\x142W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FBn254: scalar mul failed!\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[`\0\x80a\x14F\x83a\x1D1V[\x80Q\x90\x91P`0\x81\x14a\x14[Wa\x14[a+\xA0V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14vWa\x14va$\xC0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x14\xA0W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x15\x11W\x83`\x01a\x14\xBB\x83\x86a+\xCCV[a\x14\xC5\x91\x90a+\xCCV[\x81Q\x81\x10a\x14\xD5Wa\x14\xD5a(\x90V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x14\xF2Wa\x14\xF2a(\x90V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x14\xA6V[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x15\xA3W\x83\x81a\x15O\x85\x88a+\xCCV[a\x15Y\x91\x90a+\xDFV[\x81Q\x81\x10a\x15iWa\x15ia(\x90V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x15\x89Wa\x15\x89a(\x90V[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x15;V[P`\0a\x15\xAF\x82a \x85V[\x90Pa\x01\0`\0\x80Q` a/U\x839\x81Q\x91R`\0a\x15\xCF\x86\x89a+\xCCV[\x90P`\0[\x81\x81\x10\x15a\x16>W`\0\x88`\x01a\x15\xEB\x84\x86a+\xCCV[a\x15\xF5\x91\x90a+\xCCV[\x81Q\x81\x10a\x16\x05Wa\x16\x05a(\x90V[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x16\x1DWa\x16\x1Da+\xF2V[\x85\x87\t\x95P\x83\x80a\x160Wa\x160a+\xF2V[\x81\x87\x08\x95PP`\x01\x01a\x15\xD4V[P\x92\x9A\x99PPPPPPPPPPV[\x80\x82\x14a\x17=W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x16\xBF\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x81`@Qa\x16\xF6\x91\x90a,\x08V[`@Q\x80\x91\x03\x90\xA1\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82`@Qa\x17-\x91\x90a,@V[`@Q\x80\x91\x03\x90\xA1a\x17=a \xEDV[PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0a\x17`\x83a\x14:V[\x90P`\0\x80Q` a/U\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\x17\x89Wa\x17\x89a+\xF2V[\x84\x82\t\x90P\x82\x80a\x17\x9CWa\x17\x9Ca+\xF2V[\x82\x82\x08\x90P`\0\x80a\x17\xAD\x83a!\xEDV[\x92P\x90P[\x80a\x18\x16W\x84\x80a\x17\xC5Wa\x17\xC5a+\xF2V[`\x01\x87\x08\x95P\x84\x80a\x17\xD9Wa\x17\xD9a+\xF2V[\x86\x87\t\x92P\x84\x80a\x17\xECWa\x17\xECa+\xF2V[\x86\x84\t\x92P\x84\x80a\x17\xFFWa\x17\xFFa+\xF2V[\x84\x84\x08\x92Pa\x18\r\x83a!\xEDV[\x92P\x90Pa\x17\xB2V[P`@\x80Q\x80\x82\x01\x90\x91R\x94\x85R` \x85\x01RP\x91\x94\x93PPPPV[a\x17=\x82\x82a\"\xEBV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a/U\x839\x81Q\x91R\x91\x15\x90\x15\x16\x15a\x18eWPPPV[\x82Q` \x84\x01Q\x82`\x03\x84\x85\x85\x86\t\x85\t\x08\x83\x82\x83\t\x14\x83\x82\x10\x84\x84\x10\x16\x16\x93PPP\x81a\x027W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x14)\x90a+\x08V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x18\xCDWP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a/U\x839\x81Q\x91R\x84` \x01Qa\x19\0\x91\x90a,jV[a\x19\x18\x90`\0\x80Q` a/U\x839\x81Q\x91Ra+\xCCV[\x90R\x92\x91PPV[a\x19K`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R` \x87\x01Q`@\x82\x01R\x86Q``\x82\x01R``\x87\x01Q`\x80\x82\x01R`@\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R` \x85\x01Qa\x01\0\x82\x01R\x84Qa\x01 \x82\x01R``\x85\x01Qa\x01@\x82\x01R`@\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x1A\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x14)V[P\x15\x15\x95\x94PPPPPV[`\0\x81\x83\x11\x15a\x1BKW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01a\x14)V[\x82\x84\x10\x15\x80\x15a\x1B[WP\x81\x84\x11\x15[\x15a\x1BgWP\x82a\x13\x8AV[`\0a\x1Bs\x84\x84a+\xCCV[a\x1B~\x90`\x01a+\xDFV[\x90P`\x03\x85\x11\x15\x80\x15a\x1B\x90WP\x84\x81\x11[\x15a\x1B\xA7Wa\x1B\x9F\x85\x85a+\xDFV[\x91PPa\x13\x8AV[a\x1B\xB4`\x03`\0\x19a+\xCCV[\x85\x10\x15\x80\x15a\x1B\xCDWPa\x1B\xCA\x85`\0\x19a+\xCCV[\x81\x11[\x15a\x1B\xE8Wa\x1B\xDE\x85`\0\x19a+\xCCV[a\x1B\x9F\x90\x84a+\xCCV[\x82\x85\x11\x15a\x1C>W`\0a\x1B\xFC\x84\x87a+\xCCV[\x90P`\0a\x1C\n\x83\x83a,jV[\x90P\x80`\0\x03a\x1C\x1FW\x84\x93PPPPa\x13\x8AV[`\x01a\x1C+\x82\x88a+\xDFV[a\x1C5\x91\x90a+\xCCV[\x93PPPa\x1C\x8FV[\x83\x85\x10\x15a\x1C\x8FW`\0a\x1CR\x86\x86a+\xCCV[\x90P`\0a\x1C`\x83\x83a,jV[\x90P\x80`\0\x03a\x1CuW\x85\x93PPPPa\x13\x8AV[a\x1C\x7F\x81\x86a+\xCCV[a\x1C\x8A\x90`\x01a+\xDFV[\x93PPP[P\x93\x92PPPV[`\0jconsole.log`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Q`$\x01a\x1C\xC1\x92\x91\x90a,\x8CV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90RQa\x1C\xF6\x91\x90a*CV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x13EW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x13EV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x1Dr\x92\x91\x90a+qV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x1D\x99\x92\x91\x90a,\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x1D\xBB\x91\x90a,\xE2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x1D\xE5\x90\x83\x90\x83\x90` \x01a-\x04V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1EVWa\x1EVa$\xC0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1E\x80W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x1E\x98\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x1F\x03W\x81\x81\x81Q\x81\x10a\x1E\xC7Wa\x1E\xC7a(\x90V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x1E\xE4Wa\x1E\xE4a(\x90V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x1E\xACV[P`\0\x84`@Q` \x01a\x1F\x19\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x1F\xADW`\0\x83\x82\x81Q\x81\x10a\x1FTWa\x1FTa(\x90V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x1FqWa\x1Fqa(\x90V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x1F\x92\x92\x91\x90a-3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x98PP`\x01\x01a\x1F8V[P\x86\x88\x87`@Q` \x01a\x1F\xC3\x93\x92\x91\x90a-bV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x1F\xF1\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a \x12\x8A`\xFF\x8D\x16a+\xCCV[\x81\x10\x15a tW\x82\x81\x81Q\x81\x10a +Wa +a(\x90V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a E\x83\x8Da+\xDFV[\x81Q\x81\x10a UWa Ua(\x90V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a \x05V[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a \xE6W\x83\x81\x81Q\x81\x10a \xA5Wa \xA5a(\x90V[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a \xBD\x91\x90a-\xA3V[a \xC8\x90`\x02a.\x9EV[a \xD2\x91\x90a-\xA3V[a \xDC\x90\x83a+\xDFV[\x91P`\x01\x01a \x8AV[P\x92\x91PPV[`\0\x80Q` a/5\x839\x81Q\x91R;\x15a!\xDCW`@\x80Q`\0\x80Q` a/5\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra!{\x92\x91` \x01a*\x12V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra!\x95\x91a*CV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a!\xD2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a!\xD7V[``\x91P[PPPP[`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[`\0\x80`\0\x80`\0\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a/U\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x87``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x94PP`\0Q\x92P\x83a\"\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x14)V[\x80`\x01\x84\x90\x1B\x11\x15a\"\xCAWa\"\xC7\x83\x82a+\xCCV[\x92P[\x80\x80a\"\xD8Wa\"\xD8a+\xF2V[\x83\x84\t\x96\x90\x96\x14\x96\x91\x95P\x90\x93PPPPV[a\"\xF5\x82\x82a#\xD2V[a\x17=W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa#d\x90` \x80\x82R`#\x90\x82\x01R\x7FError: a == b not satisfied [byt`@\x82\x01Rbes]`\xE8\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xD2n\x16\xCA\xD4T\x87\x05\xE4\xC9\xE2\xD9O\x98\xEE\x91\xC2\x89\x08^\xE4%YO\xD5c_\xA2\x96L\xCF\x18\x81`@Qa#\x9B\x91\x90a.\xAAV[`@Q\x80\x91\x03\x90\xA1\x7F\xD2n\x16\xCA\xD4T\x87\x05\xE4\xC9\xE2\xD9O\x98\xEE\x91\xC2\x89\x08^\xE4%YO\xD5c_\xA2\x96L\xCF\x18\x82`@Qa\x17-\x91\x90a.\xE6V[\x80Q\x82Q`\x01\x91\x90\x03a$KW`\0[\x83Q\x81\x10\x15a$EW\x82\x81\x81Q\x81\x10a#\xFDWa#\xFDa(\x90V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x84\x82\x81Q\x81\x10a$$Wa$$a(\x90V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x14a$=W`\0\x91P[`\x01\x01a#\xE2V[Pa$OV[P`\0[\x92\x91PPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a$\xB4W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a$\x8FV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\xF9Wa$\xF9a$\xC0V[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\xF9Wa$\xF9a$\xC0V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%KWa%Ka$\xC0V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%mWa%ma$\xC0V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a%\x8CW`\0\x80\xFD[\x815a%\x9Fa%\x9A\x82a%SV[a%\"V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a%\xB4W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a%\xE7W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xFEW`\0\x80\xFD[a&\n\x87\x82\x88\x01a%{V[\x94PP`@`\x1F\x19\x82\x01\x12\x15a&\x1FW`\0\x80\xFD[a&'a$\xD6V[` \x86\x81\x015\x82R`@\x87\x015\x90\x82\x01R\x92P`\x80`_\x19\x82\x01\x12\x15a&LW`\0\x80\xFD[Pa&Ua$\xFFV[``\x85\x015\x81R`\x80\x85\x015` \x82\x01R`\xA0\x85\x015`@\x82\x01R`\xC0\x85\x015``\x82\x01R\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a&\x97W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x13\x8AW`\0\x80\xFD[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a'UW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a'@W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a'\x16V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a&\xD9V[P\x91\x99\x98PPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a'vW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a'\x98W\x81\x81\x01Q\x83\x82\x01R` \x01a'\x80V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra'\xB9\x81` \x86\x01` \x86\x01a'}V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a($W`?\x19\x88\x86\x03\x01\x84Ra(\x12\x85\x83Qa'\xA1V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a'\xF6V[P\x92\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a(CW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(ZW`\0\x80\xFD[a(f\x84\x82\x85\x01a%{V[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a(\x81W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0a(\xB4a%\x9A\x84a%SV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a(\xC8W`\0\x80\xFD[a\x13\x8A\x83` \x83\x01\x84a'}V[`\0` \x82\x84\x03\x12\x15a(\xE8W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\xFFW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a)\x10W`\0\x80\xFD[a(f\x84\x82Q` \x84\x01a(\xA6V[`\0`\x80\x82\x84\x03\x12\x15a)1W`\0\x80\xFD[a)9a$\xFFV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a)uW`\0\x80\xFD[a\x13\x8A\x83\x83a)\x1FV[`\xE0\x81R`\0a)\x92`\xE0\x83\x01\x86a'\xA1V[\x90Pa)\xAB` \x83\x01\x85\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x82Q``\x83\x01R` \x83\x01Q`\x80\x83\x01R`@\x83\x01Q`\xA0\x83\x01R``\x83\x01Q`\xC0\x83\x01R\x94\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a)\xECW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a*\x0CWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a*5\x81`\x04\x85\x01` \x87\x01a'}V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa*U\x81\x84` \x87\x01a'}V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a*qW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x13\x8AW`\0\x80\xFD[` \x81R`\0a\x13\x8A` \x83\x01\x84a'\xA1V[`\0`@\x82\x84\x03\x12\x15a*\xA6W`\0\x80\xFD[a*\xAEa$\xD6V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15a*\xD7W`\0\x80\xFD[\x82Q\x91Pa*\xE8\x84` \x85\x01a*\x94V[\x90P\x92P\x92\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a$OV[` \x81R`\0a$O` \x83\x01`\x17\x81R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0` \x82\x01R`@\x01\x90V[`\0\x80`\xC0\x83\x85\x03\x12\x15a+XW`\0\x80\xFD[a+b\x84\x84a)\x1FV[\x91Pa*\xE8\x84`\x80\x85\x01a*\x94V[`\0\x83Qa+\x83\x81\x84` \x88\x01a'}V[\x83Q\x90\x83\x01\x90a+\x97\x81\x83` \x88\x01a'}V[\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a$OWa$Oa+\xB6V[\x80\x82\x01\x80\x82\x11\x15a$OWa$Oa+\xB6V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`@\x81R`\0a,2`@\x83\x01`\n\x81Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B` \x82\x01R`@\x01\x90V[\x90P\x82` \x83\x01R\x92\x91PPV[`@\x81R`\0a,2`@\x83\x01`\n\x81Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B` \x82\x01R`@\x01\x90V[`\0\x82a,\x87WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`@\x81R`\0a,\x9F`@\x83\x01\x85a'\xA1V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x83Qa,\xC0\x81\x84` \x88\x01a'}V[`\0\x92\x01\x91\x82RP`\x01`\x01`\xF8\x1B\x03\x19\x91\x90\x91\x16`\x01\x82\x01R`\x02\x01\x91\x90PV[`\0\x82Qa,\xF4\x81\x84` \x87\x01a'}V[`\0\x92\x01\x91\x82RP`\x01\x01\x91\x90PV[`\0\x83Qa-\x16\x81\x84` \x88\x01a'}V[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x02\x01\x92\x91PPV[`\0\x83Qa-E\x81\x84` \x88\x01a'}V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x01\x01\x92\x91PPV[`\0\x84Qa-t\x81\x84` \x89\x01a'}V[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x91\x90\x93\x01\x90\x81R`\x01`\x01`\xF0\x1B\x03\x19\x91\x90\x91\x16`\x01\x82\x01R`\x03\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a$OWa$Oa+\xB6V[`\x01\x81\x81[\x80\x85\x11\x15a-\xF5W\x81`\0\x19\x04\x82\x11\x15a-\xDBWa-\xDBa+\xB6V[\x80\x85\x16\x15a-\xE8W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a-\xBFV[P\x92P\x92\x90PV[`\0\x82a.\x0CWP`\x01a$OV[\x81a.\x19WP`\0a$OV[\x81`\x01\x81\x14a./W`\x02\x81\x14a.9Wa.UV[`\x01\x91PPa$OV[`\xFF\x84\x11\x15a.JWa.Ja+\xB6V[PP`\x01\x82\x1Ba$OV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a.xWP\x81\x81\na$OV[a.\x82\x83\x83a-\xBAV[\x80`\0\x19\x04\x82\x11\x15a.\x96Wa.\x96a+\xB6V[\x02\x93\x92PPPV[`\0a\x13\x8A\x83\x83a-\xFDV[`@\x81R`\0a.\xD4`@\x83\x01`\n\x81Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B` \x82\x01R`@\x01\x90V[\x82\x81\x03` \x84\x01Ra(f\x81\x85a'\xA1V[`@\x81R`\0a.\xD4`@\x83\x01`\n\x81Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B` \x82\x01R`@\x01\x90V\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static BLSSIG_TEST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct BLSSig_Test<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BLSSig_Test<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BLSSig_Test<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BLSSig_Test<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BLSSig_Test<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BLSSig_Test))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BLSSig_Test<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                BLSSIG_TEST_ABI.clone(),
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
                BLSSIG_TEST_ABI.clone(),
                BLSSIG_TEST_BYTECODE.clone().into(),
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
        ///Calls the contract's `testFuzz_BLS_hashes_computation` (0xe1dc7d94) function
        pub fn test_fuzz_bls_hashes_computation(
            &self,
            input: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 220, 125, 148], input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testFuzz_RevertWhen_SignatureIsAnInvalidPoint` (0xf292926e) function
        pub fn test_fuzz_revert_when_signature_is_an_invalid_point(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 146, 146, 110], (x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testFuzz_RevertWhen_SignatureIsInvalid` (0x77682b71) function
        pub fn test_fuzz_revert_when_signature_is_invalid(
            &self,
            exp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([119, 104, 43, 113], exp)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testFuzz_RevertWhen_usingWrongVK` (0x4ab894b6) function
        pub fn test_fuzz_revert_when_using_wrong_vk(
            &self,
            exp: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 184, 148, 182], exp)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `test_SigVerification_Succeeds` (0x0bf50f6f) function
        pub fn test_sig_verification_succeeds(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 245, 15, 111], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wrapVerifyBlsSig` (0x23b8cded) function
        pub fn wrap_verify_bls_sig(
            &self,
            message: ::ethers::core::types::Bytes,
            sig: G1Point,
            vk: G2Point,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 205, 237], (message, sig, vk))
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BLSSig_TestEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for BLSSig_Test<M> {
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
    pub enum BLSSig_TestEvents {
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
    impl ::ethers::contract::EthLogDecode for BLSSig_TestEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(BLSSig_TestEvents::LogsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for BLSSig_TestEvents {
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
    impl ::core::convert::From<LogFilter> for BLSSig_TestEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for BLSSig_TestEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogArray1Filter> for BLSSig_TestEvents {
        fn from(value: LogArray1Filter) -> Self {
            Self::LogArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogArray2Filter> for BLSSig_TestEvents {
        fn from(value: LogArray2Filter) -> Self {
            Self::LogArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogArray3Filter> for BLSSig_TestEvents {
        fn from(value: LogArray3Filter) -> Self {
            Self::LogArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for BLSSig_TestEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for BLSSig_TestEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for BLSSig_TestEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedAddressFilter> for BLSSig_TestEvents {
        fn from(value: LogNamedAddressFilter) -> Self {
            Self::LogNamedAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray1Filter> for BLSSig_TestEvents {
        fn from(value: LogNamedArray1Filter) -> Self {
            Self::LogNamedArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray2Filter> for BLSSig_TestEvents {
        fn from(value: LogNamedArray2Filter) -> Self {
            Self::LogNamedArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray3Filter> for BLSSig_TestEvents {
        fn from(value: LogNamedArray3Filter) -> Self {
            Self::LogNamedArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytesFilter> for BLSSig_TestEvents {
        fn from(value: LogNamedBytesFilter) -> Self {
            Self::LogNamedBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytes32Filter> for BLSSig_TestEvents {
        fn from(value: LogNamedBytes32Filter) -> Self {
            Self::LogNamedBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalIntFilter> for BLSSig_TestEvents {
        fn from(value: LogNamedDecimalIntFilter) -> Self {
            Self::LogNamedDecimalIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalUintFilter> for BLSSig_TestEvents {
        fn from(value: LogNamedDecimalUintFilter) -> Self {
            Self::LogNamedDecimalUintFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedIntFilter> for BLSSig_TestEvents {
        fn from(value: LogNamedIntFilter) -> Self {
            Self::LogNamedIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedStringFilter> for BLSSig_TestEvents {
        fn from(value: LogNamedStringFilter) -> Self {
            Self::LogNamedStringFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedUintFilter> for BLSSig_TestEvents {
        fn from(value: LogNamedUintFilter) -> Self {
            Self::LogNamedUintFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for BLSSig_TestEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for BLSSig_TestEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    impl ::core::convert::From<LogsFilter> for BLSSig_TestEvents {
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
    ///Container type for all input parameters for the `testFuzz_BLS_hashes_computation` function with signature `testFuzz_BLS_hashes_computation(bytes)` and selector `0xe1dc7d94`
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
        name = "testFuzz_BLS_hashes_computation",
        abi = "testFuzz_BLS_hashes_computation(bytes)"
    )]
    pub struct TestFuzzBLSHashesComputationCall {
        pub input: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `testFuzz_RevertWhen_SignatureIsAnInvalidPoint` function with signature `testFuzz_RevertWhen_SignatureIsAnInvalidPoint(uint256,uint256)` and selector `0xf292926e`
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
        name = "testFuzz_RevertWhen_SignatureIsAnInvalidPoint",
        abi = "testFuzz_RevertWhen_SignatureIsAnInvalidPoint(uint256,uint256)"
    )]
    pub struct TestFuzzRevertWhenSignatureIsAnInvalidPointCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `testFuzz_RevertWhen_SignatureIsInvalid` function with signature `testFuzz_RevertWhen_SignatureIsInvalid(uint256)` and selector `0x77682b71`
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
        name = "testFuzz_RevertWhen_SignatureIsInvalid",
        abi = "testFuzz_RevertWhen_SignatureIsInvalid(uint256)"
    )]
    pub struct TestFuzzRevertWhenSignatureIsInvalidCall {
        pub exp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `testFuzz_RevertWhen_usingWrongVK` function with signature `testFuzz_RevertWhen_usingWrongVK(uint64)` and selector `0x4ab894b6`
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
        name = "testFuzz_RevertWhen_usingWrongVK",
        abi = "testFuzz_RevertWhen_usingWrongVK(uint64)"
    )]
    pub struct TestFuzzRevertWhenUsingWrongVKCall {
        pub exp: u64,
    }
    ///Container type for all input parameters for the `test_SigVerification_Succeeds` function with signature `test_SigVerification_Succeeds()` and selector `0x0bf50f6f`
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
        name = "test_SigVerification_Succeeds",
        abi = "test_SigVerification_Succeeds()"
    )]
    pub struct TestSigVerificationSucceedsCall;
    ///Container type for all input parameters for the `wrapVerifyBlsSig` function with signature `wrapVerifyBlsSig(bytes,(uint256,uint256),(uint256,uint256,uint256,uint256))` and selector `0x23b8cded`
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
        name = "wrapVerifyBlsSig",
        abi = "wrapVerifyBlsSig(bytes,(uint256,uint256),(uint256,uint256,uint256,uint256))"
    )]
    pub struct WrapVerifyBlsSigCall {
        pub message: ::ethers::core::types::Bytes,
        pub sig: G1Point,
        pub vk: G2Point,
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
    pub enum BLSSig_TestCalls {
        IsTest(IsTestCall),
        ExcludeArtifacts(ExcludeArtifactsCall),
        ExcludeContracts(ExcludeContractsCall),
        ExcludeSenders(ExcludeSendersCall),
        Failed(FailedCall),
        TargetArtifactSelectors(TargetArtifactSelectorsCall),
        TargetArtifacts(TargetArtifactsCall),
        TargetContracts(TargetContractsCall),
        TargetSelectors(TargetSelectorsCall),
        TargetSenders(TargetSendersCall),
        TestFuzzBLSHashesComputation(TestFuzzBLSHashesComputationCall),
        TestFuzzRevertWhenSignatureIsAnInvalidPoint(
            TestFuzzRevertWhenSignatureIsAnInvalidPointCall,
        ),
        TestFuzzRevertWhenSignatureIsInvalid(TestFuzzRevertWhenSignatureIsInvalidCall),
        TestFuzzRevertWhenUsingWrongVK(TestFuzzRevertWhenUsingWrongVKCall),
        TestSigVerificationSucceeds(TestSigVerificationSucceedsCall),
        WrapVerifyBlsSig(WrapVerifyBlsSigCall),
    }
    impl ::ethers::core::abi::AbiDecode for BLSSig_TestCalls {
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
                <TestFuzzBLSHashesComputationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestFuzzBLSHashesComputation(decoded));
            }
            if let Ok(decoded) = <TestFuzzRevertWhenSignatureIsAnInvalidPointCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TestFuzzRevertWhenSignatureIsAnInvalidPoint(decoded));
            }
            if let Ok(decoded) =
                <TestFuzzRevertWhenSignatureIsInvalidCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TestFuzzRevertWhenSignatureIsInvalid(decoded));
            }
            if let Ok(decoded) =
                <TestFuzzRevertWhenUsingWrongVKCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestFuzzRevertWhenUsingWrongVK(decoded));
            }
            if let Ok(decoded) =
                <TestSigVerificationSucceedsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestSigVerificationSucceeds(decoded));
            }
            if let Ok(decoded) =
                <WrapVerifyBlsSigCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WrapVerifyBlsSig(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BLSSig_TestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsTest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeArtifacts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeContracts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeSenders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Failed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetArtifactSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetArtifacts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetContracts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetSelectors(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetSenders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TestFuzzBLSHashesComputation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TestFuzzRevertWhenSignatureIsAnInvalidPoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TestFuzzRevertWhenSignatureIsInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TestFuzzRevertWhenUsingWrongVK(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TestSigVerificationSucceeds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrapVerifyBlsSig(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for BLSSig_TestCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsTest(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::Failed(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifactSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestFuzzBLSHashesComputation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TestFuzzRevertWhenSignatureIsAnInvalidPoint(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TestFuzzRevertWhenSignatureIsInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TestFuzzRevertWhenUsingWrongVK(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TestSigVerificationSucceeds(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrapVerifyBlsSig(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsTestCall> for BLSSig_TestCalls {
        fn from(value: IsTestCall) -> Self {
            Self::IsTest(value)
        }
    }
    impl ::core::convert::From<ExcludeArtifactsCall> for BLSSig_TestCalls {
        fn from(value: ExcludeArtifactsCall) -> Self {
            Self::ExcludeArtifacts(value)
        }
    }
    impl ::core::convert::From<ExcludeContractsCall> for BLSSig_TestCalls {
        fn from(value: ExcludeContractsCall) -> Self {
            Self::ExcludeContracts(value)
        }
    }
    impl ::core::convert::From<ExcludeSendersCall> for BLSSig_TestCalls {
        fn from(value: ExcludeSendersCall) -> Self {
            Self::ExcludeSenders(value)
        }
    }
    impl ::core::convert::From<FailedCall> for BLSSig_TestCalls {
        fn from(value: FailedCall) -> Self {
            Self::Failed(value)
        }
    }
    impl ::core::convert::From<TargetArtifactSelectorsCall> for BLSSig_TestCalls {
        fn from(value: TargetArtifactSelectorsCall) -> Self {
            Self::TargetArtifactSelectors(value)
        }
    }
    impl ::core::convert::From<TargetArtifactsCall> for BLSSig_TestCalls {
        fn from(value: TargetArtifactsCall) -> Self {
            Self::TargetArtifacts(value)
        }
    }
    impl ::core::convert::From<TargetContractsCall> for BLSSig_TestCalls {
        fn from(value: TargetContractsCall) -> Self {
            Self::TargetContracts(value)
        }
    }
    impl ::core::convert::From<TargetSelectorsCall> for BLSSig_TestCalls {
        fn from(value: TargetSelectorsCall) -> Self {
            Self::TargetSelectors(value)
        }
    }
    impl ::core::convert::From<TargetSendersCall> for BLSSig_TestCalls {
        fn from(value: TargetSendersCall) -> Self {
            Self::TargetSenders(value)
        }
    }
    impl ::core::convert::From<TestFuzzBLSHashesComputationCall> for BLSSig_TestCalls {
        fn from(value: TestFuzzBLSHashesComputationCall) -> Self {
            Self::TestFuzzBLSHashesComputation(value)
        }
    }
    impl ::core::convert::From<TestFuzzRevertWhenSignatureIsAnInvalidPointCall> for BLSSig_TestCalls {
        fn from(value: TestFuzzRevertWhenSignatureIsAnInvalidPointCall) -> Self {
            Self::TestFuzzRevertWhenSignatureIsAnInvalidPoint(value)
        }
    }
    impl ::core::convert::From<TestFuzzRevertWhenSignatureIsInvalidCall> for BLSSig_TestCalls {
        fn from(value: TestFuzzRevertWhenSignatureIsInvalidCall) -> Self {
            Self::TestFuzzRevertWhenSignatureIsInvalid(value)
        }
    }
    impl ::core::convert::From<TestFuzzRevertWhenUsingWrongVKCall> for BLSSig_TestCalls {
        fn from(value: TestFuzzRevertWhenUsingWrongVKCall) -> Self {
            Self::TestFuzzRevertWhenUsingWrongVK(value)
        }
    }
    impl ::core::convert::From<TestSigVerificationSucceedsCall> for BLSSig_TestCalls {
        fn from(value: TestSigVerificationSucceedsCall) -> Self {
            Self::TestSigVerificationSucceeds(value)
        }
    }
    impl ::core::convert::From<WrapVerifyBlsSigCall> for BLSSig_TestCalls {
        fn from(value: WrapVerifyBlsSigCall) -> Self {
            Self::WrapVerifyBlsSig(value)
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
