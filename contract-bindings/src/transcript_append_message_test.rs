pub use transcript_append_message_test::*;
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
pub mod transcript_append_message_test {
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
                    ::std::borrow::ToOwned::to_owned("testFuzz_appendMessage_matches"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("testFuzz_appendMessage_matches",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transcript"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                        ),
                                        2usize,
                                    ),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct Transcript.TranscriptData",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("message"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
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
    pub static TRANSCRIPT_APPENDMESSAGE_TEST_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0-W`\0\x80\xFD[Pa\x15\xBB\x80a\0=`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0qW\x80c\x85\"l\x81\x14a\x01\x06W\x80c\x91j\x17\xC6\x14a\x01\x1BW\x80c\xB5P\x8A\xA9\x14a\x01#W\x80c\xBAAO\xA6\x14a\x01+W\x80c\xE2\x0C\x9Fq\x14a\x01CW\x80c\xFAv&\xD4\x14a\x01KW`\0\x80\xFD[\x80c\x1E\xD7\x83\x1C\x14a\0\xAEW\x80c4\xC6\xEE]\x14a\0\xCCW\x80c>^<#\x14a\0\xE1W\x80c?r\x86\xF4\x14a\0\xE9W\x80cf\xD9\xA9\xA0\x14a\0\xF1W[`\0\x80\xFD[a\0\xB6a\x01XV[`@Qa\0\xC3\x91\x90a\x0EgV[`@Q\x80\x91\x03\x90\xF3[a\0\xDFa\0\xDA6`\x04a\x0F\xA2V[a\x01\xBAV[\0[a\0\xB6a\x05\x07V[a\0\xB6a\x05gV[a\0\xF9a\x05\xC7V[`@Qa\0\xC3\x91\x90a\x10\x8BV[a\x01\x0Ea\x06\xB6V[`@Qa\0\xC3\x91\x90a\x11\x90V[a\0\xF9a\x07\x86V[a\x01\x0Ea\x08lV[a\x013a\t<V[`@Q\x90\x15\x15\x81R` \x01a\0\xC3V[a\0\xB6a\ngV[`\0Ta\x013\x90`\xFF\x16\x81V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92W[PPPPP\x90P\x90V[`@\x80Q`\x04\x80\x82R`\xA0\x82\x01\x90\x92R`\0\x91\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\xD2W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x19\x1AY\x99\x8B]\x19\\\xDD`\xBA\x1B\x81RP\x81`\0\x81Q\x81\x10a\x02\x1EWa\x02\x1Ea\x11\xF4V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01ttranscript-append-msg`X\x1B\x81RP\x81`\x01\x81Q\x81\x10a\x02jWa\x02ja\x11\xF4V[` \x02` \x01\x01\x81\x90RP\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cq\xAA\xD1\r\x84`@Q` \x01a\x02\xB8\x91\x90a\x12\nV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xE3\x91\x90a\x12^V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03(\x91\x90\x81\x01\x90a\x12\xA8V[\x81`\x02\x81Q\x81\x10a\x03;Wa\x03;a\x11\xF4V[` \x02` \x01\x01\x81\x90RP\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cq\xAA\xD1\r\x83`@Q` \x01a\x03\x89\x91\x90a\x12^V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xB4\x91\x90a\x12^V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xF9\x91\x90\x81\x01\x90a\x12\xA8V[\x81`\x03\x81Q\x81\x10a\x04\x0CWa\x04\x0Ca\x11\xF4V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x89\x16\x04g`\xE0\x1B\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x89\x16\x04g\x90a\x04Q\x90\x85\x90`\x04\x01a\x11\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\x98\x91\x90\x81\x01\x90a\x13\x19V[\x90P`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x04\xB0\x91\x90a\x13NV[\x90Pa\x04\xBC\x85\x85a\n\xC7V[\x80Q\x85Qa\x04\xCA\x91\x90a\n\xF1V[` \x81\x81\x01QQ\x90\x86\x01Qa\x04\xE7\x91\x90`\0[` \x02\x01Qa\n\xFFV[` \x81\x81\x01Q\x81\x01Q\x90\x86\x01Qa\x05\0\x91\x90`\x01a\x04\xDDV[PPPPPV[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92WPPPPP\x90P\x90V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xADW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x06\x95W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x06WW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x05\xEBV[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xADW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x06\xF9\x90a\x14\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07%\x90a\x14\x0EV[\x80\x15a\x07rW\x80`\x1F\x10a\x07GWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07rV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07UW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x06\xDAV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xADW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x08TW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x08\x16W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x07\xAAV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xADW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x08\xAF\x90a\x14\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xDB\x90a\x14\x0EV[\x80\x15a\t(W\x80`\x1F\x10a\x08\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t(V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x0BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x08\x90V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\t\\WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\nbW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\t\xEA\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x14HV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\n\x04\x91a\x14yV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\nAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\nFV[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\n^\x91\x90a\x14\x95V[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92WPPPPP\x90P\x90V[\x81Q`@Qa\n\xDB\x91\x90\x83\x90` \x01a\x14\xB7V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90\x91RPV[a\n\xFB\x82\x82a\x0B\xF1V[PPV[\x80\x82\x14a\n\xFBW\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x0Bs\x90` \x80\x82R`%\x90\x82\x01R\x7FError: a == b not satisfied [byt`@\x82\x01Rdes32]`\xD8\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xAF\xB7\x95\xC9\xC6\x1EO\xE7F\x8C8o\x92]zT)\xEC\xAD\x9C\x04\x95\xDD\xB8\xD3\x8Di\x06\x14\xD3/\x99\x81`@Qa\x0B\xAA\x91\x90a\x14\xE6V[`@Q\x80\x91\x03\x90\xA1\x7F\xAF\xB7\x95\xC9\xC6\x1EO\xE7F\x8C8o\x92]zT)\xEC\xAD\x9C\x04\x95\xDD\xB8\xD3\x8Di\x06\x14\xD3/\x99\x82`@Qa\x0B\xE1\x91\x90a\x15\x1EV[`@Q\x80\x91\x03\x90\xA1a\n\xFBa\x0C\xD8V[a\x0B\xFB\x82\x82a\r\xE4V[a\n\xFBW\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x0Cj\x90` \x80\x82R`#\x90\x82\x01R\x7FError: a == b not satisfied [byt`@\x82\x01Rbes]`\xE8\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xD2n\x16\xCA\xD4T\x87\x05\xE4\xC9\xE2\xD9O\x98\xEE\x91\xC2\x89\x08^\xE4%YO\xD5c_\xA2\x96L\xCF\x18\x81`@Qa\x0C\xA1\x91\x90a\x15HV[`@Q\x80\x91\x03\x90\xA1\x7F\xD2n\x16\xCA\xD4T\x87\x05\xE4\xC9\xE2\xD9O\x98\xEE\x91\xC2\x89\x08^\xE4%YO\xD5c_\xA2\x96L\xCF\x18\x82`@Qa\x0B\xE1\x91\x90a\x15\x84V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\r\xD3W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\rr\x92\x91` \x01a\x14HV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\x8C\x91a\x14yV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\r\xC9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xCEV[``\x91P[PPPP[`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[\x80Q\x82Q`\x01\x91\x90\x03a\x0E]W`\0[\x83Q\x81\x10\x15a\x0EWW\x82\x81\x81Q\x81\x10a\x0E\x0FWa\x0E\x0Fa\x11\xF4V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x84\x82\x81Q\x81\x10a\x0E6Wa\x0E6a\x11\xF4V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x14a\x0EOW`\0\x91P[`\x01\x01a\r\xF4V[Pa\x0EaV[P`\0[\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0E\xA8W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E\x83V[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\xEDWa\x0E\xEDa\x0E\xB4V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\x1CWa\x0F\x1Ca\x0E\xB4V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0F>Wa\x0F>a\x0E\xB4V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0F]W`\0\x80\xFD[\x815a\x0Fpa\x0Fk\x82a\x0F$V[a\x0E\xF3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0F\x85W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xB5W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\xCDW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x0F\xE1W`\0\x80\xFD[a\x0F\xE9a\x0E\xCAV[\x825\x82\x81\x11\x15a\x0F\xF8W`\0\x80\xFD[a\x10\x04\x88\x82\x86\x01a\x0FLV[\x82RP` \x87`?\x85\x01\x12a\x10\x18W`\0\x80\xFD[a\x10 a\x0E\xCAV[\x80``\x86\x01\x8A\x81\x11\x15a\x102W`\0\x80\xFD[` \x87\x01\x96P[\x80\x87\x10\x15a\x10RW\x865\x83R\x95\x83\x01\x95\x91\x83\x01\x91a\x109V[P\x80` \x85\x01RPPP\x80\x94PP` \x85\x015\x91P\x80\x82\x11\x15a\x10tW`\0\x80\xFD[Pa\x10\x81\x85\x82\x86\x01a\x0FLV[\x91PP\x92P\x92\x90PV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\x111W\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\x11\x1CW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a\x10\xF2V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\x10\xB5V[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15a\x11[W\x81\x81\x01Q\x83\x82\x01R` \x01a\x11CV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x11|\x81` \x86\x01` \x86\x01a\x11@V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a\x11\xE7W`?\x19\x88\x86\x03\x01\x84Ra\x11\xD5\x85\x83Qa\x11dV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x11\xB9V[P\x92\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x80\x83R\x83Q``` \x85\x01Ra\x12'`\x80\x85\x01\x82a\x11dV[\x90P` \x85\x01Q`@\x85\x01`\0[`\x02\x81\x10\x15a\x12RW\x82Q\x82R\x91\x84\x01\x91\x90\x84\x01\x90`\x01\x01a\x125V[P\x91\x96\x95PPPPPPV[` \x81R`\0a\x12q` \x83\x01\x84a\x11dV[\x93\x92PPPV[`\0a\x12\x86a\x0Fk\x84a\x0F$V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a\x12\x9AW`\0\x80\xFD[a\x12q\x83` \x83\x01\x84a\x11@V[`\0` \x82\x84\x03\x12\x15a\x12\xBAW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xD1W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x12\xE2W`\0\x80\xFD[a\x12\xF1\x84\x82Q` \x84\x01a\x12xV[\x94\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a\x13\nW`\0\x80\xFD[a\x12q\x83\x83Q` \x85\x01a\x12xV[`\0` \x82\x84\x03\x12\x15a\x13+W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13BW`\0\x80\xFD[a\x12\xF1\x84\x82\x85\x01a\x12\xF9V[`\0` \x80\x83\x85\x03\x12\x15a\x13aW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13yW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x13\x8DW`\0\x80\xFD[a\x13\x95a\x0E\xCAV[\x82Q\x82\x81\x11\x15a\x13\xA4W`\0\x80\xFD[a\x13\xB0\x88\x82\x86\x01a\x12\xF9V[\x82RP\x86`?\x84\x01\x12a\x13\xC2W`\0\x80\xFD[a\x13\xCAa\x0E\xCAV[\x91P\x81``\x84\x01\x88\x81\x11\x15a\x13\xDEW`\0\x80\xFD[` \x85\x01\x94P[\x80\x85\x10\x15a\x13\xFEW\x84Q\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x13\xE5V[P` \x82\x01R\x96\x95PPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x14\"W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x14BWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a\x14k\x81`\x04\x85\x01` \x87\x01a\x11@V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa\x14\x8B\x81\x84` \x87\x01a\x11@V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x14\xA7W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x12qW`\0\x80\xFD[`\0\x83Qa\x14\xC9\x81\x84` \x88\x01a\x11@V[\x83Q\x90\x83\x01\x90a\x14\xDD\x81\x83` \x88\x01a\x11@V[\x01\x94\x93PPPPV[`@\x81R`\0a\x15\x10`@\x83\x01`\n\x81Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B` \x82\x01R`@\x01\x90V[\x90P\x82` \x83\x01R\x92\x91PPV[`@\x81R`\0a\x15\x10`@\x83\x01`\n\x81Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B` \x82\x01R`@\x01\x90V[`@\x81R`\0a\x15r`@\x83\x01`\n\x81Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B` \x82\x01R`@\x01\x90V[\x82\x81\x03` \x84\x01Ra\x12\xF1\x81\x85a\x11dV[`@\x81R`\0a\x15r`@\x83\x01`\n\x81Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B` \x82\x01R`@\x01\x90V\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static TRANSCRIPT_APPENDMESSAGE_TEST_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0qW\x80c\x85\"l\x81\x14a\x01\x06W\x80c\x91j\x17\xC6\x14a\x01\x1BW\x80c\xB5P\x8A\xA9\x14a\x01#W\x80c\xBAAO\xA6\x14a\x01+W\x80c\xE2\x0C\x9Fq\x14a\x01CW\x80c\xFAv&\xD4\x14a\x01KW`\0\x80\xFD[\x80c\x1E\xD7\x83\x1C\x14a\0\xAEW\x80c4\xC6\xEE]\x14a\0\xCCW\x80c>^<#\x14a\0\xE1W\x80c?r\x86\xF4\x14a\0\xE9W\x80cf\xD9\xA9\xA0\x14a\0\xF1W[`\0\x80\xFD[a\0\xB6a\x01XV[`@Qa\0\xC3\x91\x90a\x0EgV[`@Q\x80\x91\x03\x90\xF3[a\0\xDFa\0\xDA6`\x04a\x0F\xA2V[a\x01\xBAV[\0[a\0\xB6a\x05\x07V[a\0\xB6a\x05gV[a\0\xF9a\x05\xC7V[`@Qa\0\xC3\x91\x90a\x10\x8BV[a\x01\x0Ea\x06\xB6V[`@Qa\0\xC3\x91\x90a\x11\x90V[a\0\xF9a\x07\x86V[a\x01\x0Ea\x08lV[a\x013a\t<V[`@Q\x90\x15\x15\x81R` \x01a\0\xC3V[a\0\xB6a\ngV[`\0Ta\x013\x90`\xFF\x16\x81V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92W[PPPPP\x90P\x90V[`@\x80Q`\x04\x80\x82R`\xA0\x82\x01\x90\x92R`\0\x91\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\xD2W\x90PP\x90P`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x19\x1AY\x99\x8B]\x19\\\xDD`\xBA\x1B\x81RP\x81`\0\x81Q\x81\x10a\x02\x1EWa\x02\x1Ea\x11\xF4V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01ttranscript-append-msg`X\x1B\x81RP\x81`\x01\x81Q\x81\x10a\x02jWa\x02ja\x11\xF4V[` \x02` \x01\x01\x81\x90RP\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cq\xAA\xD1\r\x84`@Q` \x01a\x02\xB8\x91\x90a\x12\nV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xE3\x91\x90a\x12^V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03(\x91\x90\x81\x01\x90a\x12\xA8V[\x81`\x02\x81Q\x81\x10a\x03;Wa\x03;a\x11\xF4V[` \x02` \x01\x01\x81\x90RP\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cq\xAA\xD1\r\x83`@Q` \x01a\x03\x89\x91\x90a\x12^V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xB4\x91\x90a\x12^V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xF9\x91\x90\x81\x01\x90a\x12\xA8V[\x81`\x03\x81Q\x81\x10a\x04\x0CWa\x04\x0Ca\x11\xF4V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x89\x16\x04g`\xE0\x1B\x81R`\0\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x89\x16\x04g\x90a\x04Q\x90\x85\x90`\x04\x01a\x11\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\x98\x91\x90\x81\x01\x90a\x13\x19V[\x90P`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x04\xB0\x91\x90a\x13NV[\x90Pa\x04\xBC\x85\x85a\n\xC7V[\x80Q\x85Qa\x04\xCA\x91\x90a\n\xF1V[` \x81\x81\x01QQ\x90\x86\x01Qa\x04\xE7\x91\x90`\0[` \x02\x01Qa\n\xFFV[` \x81\x81\x01Q\x81\x01Q\x90\x86\x01Qa\x05\0\x91\x90`\x01a\x04\xDDV[PPPPPV[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92WPPPPP\x90P\x90V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xADW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x06\x95W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x06WW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x05\xEBV[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xADW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x06\xF9\x90a\x14\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07%\x90a\x14\x0EV[\x80\x15a\x07rW\x80`\x1F\x10a\x07GWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07rV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07UW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x06\xDAV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xADW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x08TW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x08\x16W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x07\xAAV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xADW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x08\xAF\x90a\x14\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xDB\x90a\x14\x0EV[\x80\x15a\t(W\x80`\x1F\x10a\x08\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t(V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x0BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x08\x90V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\t\\WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\nbW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\t\xEA\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x14HV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\n\x04\x91a\x14yV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\nAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\nFV[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\n^\x91\x90a\x14\x95V[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92WPPPPP\x90P\x90V[\x81Q`@Qa\n\xDB\x91\x90\x83\x90` \x01a\x14\xB7V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90\x91RPV[a\n\xFB\x82\x82a\x0B\xF1V[PPV[\x80\x82\x14a\n\xFBW\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x0Bs\x90` \x80\x82R`%\x90\x82\x01R\x7FError: a == b not satisfied [byt`@\x82\x01Rdes32]`\xD8\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xAF\xB7\x95\xC9\xC6\x1EO\xE7F\x8C8o\x92]zT)\xEC\xAD\x9C\x04\x95\xDD\xB8\xD3\x8Di\x06\x14\xD3/\x99\x81`@Qa\x0B\xAA\x91\x90a\x14\xE6V[`@Q\x80\x91\x03\x90\xA1\x7F\xAF\xB7\x95\xC9\xC6\x1EO\xE7F\x8C8o\x92]zT)\xEC\xAD\x9C\x04\x95\xDD\xB8\xD3\x8Di\x06\x14\xD3/\x99\x82`@Qa\x0B\xE1\x91\x90a\x15\x1EV[`@Q\x80\x91\x03\x90\xA1a\n\xFBa\x0C\xD8V[a\x0B\xFB\x82\x82a\r\xE4V[a\n\xFBW\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x0Cj\x90` \x80\x82R`#\x90\x82\x01R\x7FError: a == b not satisfied [byt`@\x82\x01Rbes]`\xE8\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xD2n\x16\xCA\xD4T\x87\x05\xE4\xC9\xE2\xD9O\x98\xEE\x91\xC2\x89\x08^\xE4%YO\xD5c_\xA2\x96L\xCF\x18\x81`@Qa\x0C\xA1\x91\x90a\x15HV[`@Q\x80\x91\x03\x90\xA1\x7F\xD2n\x16\xCA\xD4T\x87\x05\xE4\xC9\xE2\xD9O\x98\xEE\x91\xC2\x89\x08^\xE4%YO\xD5c_\xA2\x96L\xCF\x18\x82`@Qa\x0B\xE1\x91\x90a\x15\x84V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\r\xD3W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\rr\x92\x91` \x01a\x14HV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\x8C\x91a\x14yV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\r\xC9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xCEV[``\x91P[PPPP[`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[\x80Q\x82Q`\x01\x91\x90\x03a\x0E]W`\0[\x83Q\x81\x10\x15a\x0EWW\x82\x81\x81Q\x81\x10a\x0E\x0FWa\x0E\x0Fa\x11\xF4V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x84\x82\x81Q\x81\x10a\x0E6Wa\x0E6a\x11\xF4V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x14a\x0EOW`\0\x91P[`\x01\x01a\r\xF4V[Pa\x0EaV[P`\0[\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0E\xA8W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E\x83V[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\xEDWa\x0E\xEDa\x0E\xB4V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\x1CWa\x0F\x1Ca\x0E\xB4V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0F>Wa\x0F>a\x0E\xB4V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0F]W`\0\x80\xFD[\x815a\x0Fpa\x0Fk\x82a\x0F$V[a\x0E\xF3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0F\x85W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xB5W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\xCDW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x0F\xE1W`\0\x80\xFD[a\x0F\xE9a\x0E\xCAV[\x825\x82\x81\x11\x15a\x0F\xF8W`\0\x80\xFD[a\x10\x04\x88\x82\x86\x01a\x0FLV[\x82RP` \x87`?\x85\x01\x12a\x10\x18W`\0\x80\xFD[a\x10 a\x0E\xCAV[\x80``\x86\x01\x8A\x81\x11\x15a\x102W`\0\x80\xFD[` \x87\x01\x96P[\x80\x87\x10\x15a\x10RW\x865\x83R\x95\x83\x01\x95\x91\x83\x01\x91a\x109V[P\x80` \x85\x01RPPP\x80\x94PP` \x85\x015\x91P\x80\x82\x11\x15a\x10tW`\0\x80\xFD[Pa\x10\x81\x85\x82\x86\x01a\x0FLV[\x91PP\x92P\x92\x90PV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\x111W\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\x11\x1CW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a\x10\xF2V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\x10\xB5V[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15a\x11[W\x81\x81\x01Q\x83\x82\x01R` \x01a\x11CV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x11|\x81` \x86\x01` \x86\x01a\x11@V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01`\0[\x82\x81\x10\x15a\x11\xE7W`?\x19\x88\x86\x03\x01\x84Ra\x11\xD5\x85\x83Qa\x11dV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x11\xB9V[P\x92\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x80\x83R\x83Q``` \x85\x01Ra\x12'`\x80\x85\x01\x82a\x11dV[\x90P` \x85\x01Q`@\x85\x01`\0[`\x02\x81\x10\x15a\x12RW\x82Q\x82R\x91\x84\x01\x91\x90\x84\x01\x90`\x01\x01a\x125V[P\x91\x96\x95PPPPPPV[` \x81R`\0a\x12q` \x83\x01\x84a\x11dV[\x93\x92PPPV[`\0a\x12\x86a\x0Fk\x84a\x0F$V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a\x12\x9AW`\0\x80\xFD[a\x12q\x83` \x83\x01\x84a\x11@V[`\0` \x82\x84\x03\x12\x15a\x12\xBAW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xD1W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x12\xE2W`\0\x80\xFD[a\x12\xF1\x84\x82Q` \x84\x01a\x12xV[\x94\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a\x13\nW`\0\x80\xFD[a\x12q\x83\x83Q` \x85\x01a\x12xV[`\0` \x82\x84\x03\x12\x15a\x13+W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13BW`\0\x80\xFD[a\x12\xF1\x84\x82\x85\x01a\x12\xF9V[`\0` \x80\x83\x85\x03\x12\x15a\x13aW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13yW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x13\x8DW`\0\x80\xFD[a\x13\x95a\x0E\xCAV[\x82Q\x82\x81\x11\x15a\x13\xA4W`\0\x80\xFD[a\x13\xB0\x88\x82\x86\x01a\x12\xF9V[\x82RP\x86`?\x84\x01\x12a\x13\xC2W`\0\x80\xFD[a\x13\xCAa\x0E\xCAV[\x91P\x81``\x84\x01\x88\x81\x11\x15a\x13\xDEW`\0\x80\xFD[` \x85\x01\x94P[\x80\x85\x10\x15a\x13\xFEW\x84Q\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x13\xE5V[P` \x82\x01R\x96\x95PPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x14\"W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x14BWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a\x14k\x81`\x04\x85\x01` \x87\x01a\x11@V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa\x14\x8B\x81\x84` \x87\x01a\x11@V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x14\xA7W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x12qW`\0\x80\xFD[`\0\x83Qa\x14\xC9\x81\x84` \x88\x01a\x11@V[\x83Q\x90\x83\x01\x90a\x14\xDD\x81\x83` \x88\x01a\x11@V[\x01\x94\x93PPPPV[`@\x81R`\0a\x15\x10`@\x83\x01`\n\x81Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B` \x82\x01R`@\x01\x90V[\x90P\x82` \x83\x01R\x92\x91PPV[`@\x81R`\0a\x15\x10`@\x83\x01`\n\x81Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B` \x82\x01R`@\x01\x90V[`@\x81R`\0a\x15r`@\x83\x01`\n\x81Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B` \x82\x01R`@\x01\x90V[\x82\x81\x03` \x84\x01Ra\x12\xF1\x81\x85a\x11dV[`@\x81R`\0a\x15r`@\x83\x01`\n\x81Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B` \x82\x01R`@\x01\x90V\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static TRANSCRIPT_APPENDMESSAGE_TEST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Transcript_appendMessage_Test<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Transcript_appendMessage_Test<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Transcript_appendMessage_Test<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Transcript_appendMessage_Test<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Transcript_appendMessage_Test<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Transcript_appendMessage_Test))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Transcript_appendMessage_Test<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                TRANSCRIPT_APPENDMESSAGE_TEST_ABI.clone(),
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
                TRANSCRIPT_APPENDMESSAGE_TEST_ABI.clone(),
                TRANSCRIPT_APPENDMESSAGE_TEST_BYTECODE.clone().into(),
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
        ///Calls the contract's `testFuzz_appendMessage_matches` (0x34c6ee5d) function
        pub fn test_fuzz_append_message_matches(
            &self,
            transcript: TranscriptData,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 198, 238, 93], (transcript, message))
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Transcript_appendMessage_TestEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for Transcript_appendMessage_Test<M>
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
    pub enum Transcript_appendMessage_TestEvents {
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
    impl ::ethers::contract::EthLogDecode for Transcript_appendMessage_TestEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogAddressFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogArray1Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogArray2Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogArray3Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogBytes32Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogNamedAddressFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogNamedArray1Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogNamedArray2Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogNamedArray3Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogNamedBytesFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogNamedBytes32Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogNamedIntFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogNamedStringFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogNamedUintFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogStringFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(Transcript_appendMessage_TestEvents::LogsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for Transcript_appendMessage_TestEvents {
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
    impl ::core::convert::From<LogFilter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogArray1Filter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogArray1Filter) -> Self {
            Self::LogArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogArray2Filter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogArray2Filter) -> Self {
            Self::LogArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogArray3Filter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogArray3Filter) -> Self {
            Self::LogArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedAddressFilter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogNamedAddressFilter) -> Self {
            Self::LogNamedAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray1Filter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogNamedArray1Filter) -> Self {
            Self::LogNamedArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray2Filter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogNamedArray2Filter) -> Self {
            Self::LogNamedArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray3Filter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogNamedArray3Filter) -> Self {
            Self::LogNamedArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytesFilter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogNamedBytesFilter) -> Self {
            Self::LogNamedBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytes32Filter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogNamedBytes32Filter) -> Self {
            Self::LogNamedBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalIntFilter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogNamedDecimalIntFilter) -> Self {
            Self::LogNamedDecimalIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalUintFilter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogNamedDecimalUintFilter) -> Self {
            Self::LogNamedDecimalUintFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedIntFilter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogNamedIntFilter) -> Self {
            Self::LogNamedIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedStringFilter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogNamedStringFilter) -> Self {
            Self::LogNamedStringFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedUintFilter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogNamedUintFilter) -> Self {
            Self::LogNamedUintFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for Transcript_appendMessage_TestEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    impl ::core::convert::From<LogsFilter> for Transcript_appendMessage_TestEvents {
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
    ///Container type for all input parameters for the `testFuzz_appendMessage_matches` function with signature `testFuzz_appendMessage_matches((bytes,bytes32[2]),bytes)` and selector `0x34c6ee5d`
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
        name = "testFuzz_appendMessage_matches",
        abi = "testFuzz_appendMessage_matches((bytes,bytes32[2]),bytes)"
    )]
    pub struct TestFuzzAppendMessageMatchesCall {
        pub transcript: TranscriptData,
        pub message: ::ethers::core::types::Bytes,
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
    pub enum Transcript_appendMessage_TestCalls {
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
        TestFuzzAppendMessageMatches(TestFuzzAppendMessageMatchesCall),
    }
    impl ::ethers::core::abi::AbiDecode for Transcript_appendMessage_TestCalls {
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
                <TestFuzzAppendMessageMatchesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestFuzzAppendMessageMatches(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for Transcript_appendMessage_TestCalls {
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
                Self::TestFuzzAppendMessageMatches(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for Transcript_appendMessage_TestCalls {
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
                Self::TestFuzzAppendMessageMatches(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<IsTestCall> for Transcript_appendMessage_TestCalls {
        fn from(value: IsTestCall) -> Self {
            Self::IsTest(value)
        }
    }
    impl ::core::convert::From<ExcludeArtifactsCall> for Transcript_appendMessage_TestCalls {
        fn from(value: ExcludeArtifactsCall) -> Self {
            Self::ExcludeArtifacts(value)
        }
    }
    impl ::core::convert::From<ExcludeContractsCall> for Transcript_appendMessage_TestCalls {
        fn from(value: ExcludeContractsCall) -> Self {
            Self::ExcludeContracts(value)
        }
    }
    impl ::core::convert::From<ExcludeSendersCall> for Transcript_appendMessage_TestCalls {
        fn from(value: ExcludeSendersCall) -> Self {
            Self::ExcludeSenders(value)
        }
    }
    impl ::core::convert::From<FailedCall> for Transcript_appendMessage_TestCalls {
        fn from(value: FailedCall) -> Self {
            Self::Failed(value)
        }
    }
    impl ::core::convert::From<TargetArtifactSelectorsCall> for Transcript_appendMessage_TestCalls {
        fn from(value: TargetArtifactSelectorsCall) -> Self {
            Self::TargetArtifactSelectors(value)
        }
    }
    impl ::core::convert::From<TargetArtifactsCall> for Transcript_appendMessage_TestCalls {
        fn from(value: TargetArtifactsCall) -> Self {
            Self::TargetArtifacts(value)
        }
    }
    impl ::core::convert::From<TargetContractsCall> for Transcript_appendMessage_TestCalls {
        fn from(value: TargetContractsCall) -> Self {
            Self::TargetContracts(value)
        }
    }
    impl ::core::convert::From<TargetSelectorsCall> for Transcript_appendMessage_TestCalls {
        fn from(value: TargetSelectorsCall) -> Self {
            Self::TargetSelectors(value)
        }
    }
    impl ::core::convert::From<TargetSendersCall> for Transcript_appendMessage_TestCalls {
        fn from(value: TargetSendersCall) -> Self {
            Self::TargetSenders(value)
        }
    }
    impl ::core::convert::From<TestFuzzAppendMessageMatchesCall>
        for Transcript_appendMessage_TestCalls
    {
        fn from(value: TestFuzzAppendMessageMatchesCall) -> Self {
            Self::TestFuzzAppendMessageMatches(value)
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
