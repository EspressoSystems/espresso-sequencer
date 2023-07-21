pub use hot_shot_test::*;
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
pub mod hot_shot_test {
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
                    ::std::borrow::ToOwned::to_owned("hotshot"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("hotshot"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract HotShot"),
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
                    ::std::borrow::ToOwned::to_owned("testPublishCommitments"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("testPublishCommitments",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NewBlocks"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewBlocks"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("firstBlockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("numBlocks"),
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
    pub static HOTSHOTTEST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0-W`\0\x80\xFD[Pa3\x1C\x80a\0=`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\x8CW\x80c\xB5P\x8A\xA9\x11a\0fW\x80c\xB5P\x8A\xA9\x14a\x01qW\x80c\xBAAO\xA6\x14a\x01yW\x80c\xE2\x0C\x9Fq\x14a\x01\x91W\x80c\xFAv&\xD4\x14a\x01\x99W`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x01LW\x80c\x91j\x17\xC6\x14a\x01aW\x80c\x99\x97\xAF\xDB\x14a\x01iW`\0\x80\xFD[\x80c\n\x92T\xE4\x14a\0\xD4W\x80c\x1E\xD7\x83\x1C\x14a\0\xDEW\x80c*\xDC\x8Bv\x14a\0\xFCW\x80c>^<#\x14a\x01'W\x80c?r\x86\xF4\x14a\x01/W\x80cf\xD9\xA9\xA0\x14a\x017W[`\0\x80\xFD[a\0\xDCa\x01\xA6V[\0[a\0\xE6a\x01\xF1V[`@Qa\0\xF3\x91\x90a\x0E\x1CV[`@Q\x80\x91\x03\x90\xF3[`\x1BTa\x01\x0F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF3V[a\0\xE6a\x02SV[a\0\xE6a\x02\xB3V[a\x01?a\x03\x13V[`@Qa\0\xF3\x91\x90a\x0EiV[a\x01Ta\x04\x02V[`@Qa\0\xF3\x91\x90a\x0FlV[a\x01?a\x04\xD2V[a\0\xDCa\x05\xB8V[a\x01Ta\t\x86V[a\x01\x81a\nVV[`@Q\x90\x15\x15\x81R` \x01a\0\xF3V[a\0\xE6a\x0B\x81V[`\0Ta\x01\x81\x90`\xFF\x16\x81V[`@Qa\x01\xB2\x90a\x0E\x0FV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x01\xCEW=`\0\x80>=`\0\xFD[P`\x1B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+WPPPPP\x90P\x90V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x03\xE1W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x03\xA3W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x037V[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x04E\x90a\x0F\xCEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04q\x90a\x0F\xCEV[\x80\x15a\x04\xBEW\x80`\x1F\x10a\x04\x93Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xBEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xA1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04&V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x05\xA0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x05bW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04\xF6V[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R`\0\x92` \x83\x01\x90\x806\x837PP`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x82\x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05\xF2W\x90PP\x90Pd\x868#p\x95\x82`\0\x81Q\x81\x10a\x06\"Wa\x06\"a\x10\x08V[` \x02` \x01\x01\x81\x81RPP`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e0x3333`\xD0\x1B\x81RP\x81`\0\x81Q\x81\x10a\x06`Wa\x06`a\x10\x08V[` \x02` \x01\x01\x81\x90RPd6\x8B\xD51\xFE\x82`\x01\x81Q\x81\x10a\x06\x84Wa\x06\x84a\x10\x08V[` \x02` \x01\x01\x81\x81RPP`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e\x0C\x1E\r\r\r\r`\xD2\x1B\x81RP\x81`\x01\x81Q\x81\x10a\x06\xC2Wa\x06\xC2a\x10\x08V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x1BT`@Qc\x81\xBA\xD6\xF3`\xE0\x1B\x81R`\0`\x04\x82\x01\x81\x90R`$\x82\x01\x81\x90R`D\x82\x01R`\x01`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x84\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x81\xBA\xD6\xF3\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07VW=`\0\x80>=`\0\xFD[PP`@\x80Q`\0\x81R`\x02` \x82\x01R\x7F\x82\x03\xA2\x1EO\x95\xF7.P\x81\xD5\xE0\x92\x9B\x1A\x8CR\x14\x1E\x12?\x9A\x14\xE1\xE7K\x02`\xFA_R\xF1\x93P\x01\x90P`@Q\x80\x91\x03\x90\xA1`\x1BT`@Qc\x06v\x925`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cgi#P\x90a\x07\xC7\x90\x85\x90\x85\x90`\x04\x01a\x10\x1EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xF5W=`\0\x80>=`\0\xFD[PP`\x1BT`@QcI\xCE\x89\x97`\xE0\x1B\x81R`\0`\x04\x82\x01Ra\x08\x8A\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91PcI\xCE\x89\x97\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08j\x91\x90a\x10\xB6V[\x83`\0\x81Q\x81\x10a\x08}Wa\x08}a\x10\x08V[` \x02` \x01\x01Qa\x0B\xE1V[`\x1BT`@QcI\xCE\x89\x97`\xE0\x1B\x81R`\x01`\x04\x82\x01Ra\t\x0C\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cI\xCE\x89\x97\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xF9\x91\x90a\x10\xB6V[\x83`\x01\x81Q\x81\x10a\x08}Wa\x08}a\x10\x08V[`\x1BT`@QcI\xCE\x89\x97`\xE0\x1B\x81R`\x02`\x04\x82\x01Ra\t\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cI\xCE\x89\x97\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tWW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t{\x91\x90a\x10\xB6V[`\0a\x0B\xE1V[PPV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W\x83\x82\x90`\0R` `\0 \x01\x80Ta\t\xC9\x90a\x0F\xCEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xF5\x90a\x0F\xCEV[\x80\x15a\nBW\x80`\x1F\x10a\n\x17Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\nBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n%W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\t\xAAV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\nvWP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0B|W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x0B\x04\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x10\xCFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0B\x1E\x91a\x11\0V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0B[W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B`V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x0Bx\x91\x90a\x11\x1CV[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+WPPPPP\x90P\x90V[\x80\x82\x14a\t\x82W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x0CR\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1a\t\x82sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\r\xFEW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\x9D\x92\x91` \x01a\x10\xCFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\xB7\x91a\x11\0V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\r\xF4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xF9V[``\x91P[PPPP[`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[a!\xA1\x80a\x11F\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0E]W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E8V[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\x0F\rW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\x0E\xF8W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a\x0E\xCEV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\x0E\x91V[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15a\x0F7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0F\x1FV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0FX\x81` \x86\x01` \x86\x01a\x0F\x1CV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x0F\xC1W`?\x19\x88\x86\x03\x01\x84Ra\x0F\xAF\x85\x83Qa\x0F@V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x0F\x93V[P\x92\x97\x96PPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0F\xE2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x10\x02WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R`\0\x90` \x90``\x84\x01\x90\x82\x87\x01\x84[\x82\x81\x10\x15a\x10WW\x81Q\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x10;V[PPP\x83\x81\x03\x82\x85\x01R\x84Q\x80\x82R\x82\x82\x01\x90`\x05\x81\x90\x1B\x83\x01\x84\x01\x87\x85\x01`\0[\x83\x81\x10\x15a\x10\xA7W`\x1F\x19\x86\x84\x03\x01\x85Ra\x10\x95\x83\x83Qa\x0F@V[\x94\x87\x01\x94\x92P\x90\x86\x01\x90`\x01\x01a\x10yV[P\x90\x99\x98PPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x10\xC8W`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a\x10\xF2\x81`\x04\x85\x01` \x87\x01a\x0F\x1CV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa\x11\x12\x81\x84` \x87\x01a\x0F\x1CV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11.W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x11>W`\0\x80\xFD[\x93\x92PPPV\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa!\x81\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cgi#P\x11a\0[W\x80cgi#P\x14a\0\xD3W\x80cg\xA2\x1Ep\x14a\0\xE6W\x80c\xF1\xF4]\x99\x14a\x010W\x80c\xF4O\xF7\x12\x14a\x01CW`\0\x80\xFD[\x80c\x03@\x96\x1E\x14a\0\x82W\x80c&\x83=\xCC\x14a\0\x97W\x80cI\xCE\x89\x97\x14a\0\xB3W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x1B\xD0V[a\x01LV[\0[a\0\xA0a\x01\xF4\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA0a\0\xC16`\x04a\x1C\xA0V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\0\x95a\0\xE16`\x04a\x1C\xFEV[a\x03\xEBV[a\0\xF9a\0\xF46`\x04a\x1C\xA0V[a\x05SV[`@\x80Q\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R\x83\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x92\x83\x01Q\x92\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x01a\0\xAAV[a\0\x95a\x01>6`\x04a\x1DjV[a\x05\xE7V[a\0\xA0`\x01T\x81V[`\x03T\x82Q\x11\x15a\x01\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqbitmap is too long`p\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x81Q\x81\x10a\x01\xAEWa\x01\xAEa\x1D\xE2V[` \x02` \x01\x01Q\x15\x80\x15a\x01\xC3WP\x82Q\x81\x10[\x15a\x01\xDAW\x80a\x01\xD2\x81a\x1E\x0EV[\x91PPa\x01\x9CV[\x82Q\x81\x10a\x01\xFBW`@QcKe\x82-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81[\x84Q\x81\x10\x15a\x02UW\x84\x81\x81Q\x81\x10a\x02\x1AWa\x02\x1Aa\x1D\xE2V[` \x02` \x01\x01Q\x15a\x02CW`\0\x81\x81R`\x02` R`@\x90 Ta\x02@\x90\x83a\x1E'V[\x91P[\x80a\x02M\x81a\x1E\x0EV[\x91PPa\x01\xFFV[P\x82\x81\x10\x15a\x02wW`@Qc<)\x0BS`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x03\x83\x81T\x81\x10a\x02\x8CWa\x02\x8Ca\x1D\xE2V[\x90`\0R` `\0 \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPP\x90P`\0\x83`\x01a\x02\xDE\x91\x90a\x1E'V[\x90P[\x85Q\x81\x10\x15a\x03\xD6W\x85\x81\x81Q\x81\x10a\x02\xFCWa\x02\xFCa\x1D\xE2V[` \x02` \x01\x01Q\x15a\x03\xC4W`\0`\x03\x82\x81T\x81\x10a\x03\x1EWa\x03\x1Ea\x1D\xE2V[`\0\x91\x82R` \x80\x83 `@\x80Q`\x80\x81\x01\x82R`\x04\x90\x94\x02\x90\x91\x01\x80T\x80\x85R`\x01\x82\x01T\x85\x85\x01\x81\x90R`\x02\x83\x01T\x86\x85\x01\x81\x90R`\x03\x90\x93\x01T``\x80\x88\x01\x82\x90R\x8BQ\x96\x8C\x01Q\x95\x8C\x01Q\x90\x8C\x01Q\x97\x99P\x95\x97\x94\x96\x94\x92\x93\x91\x92\x80\x80\x80a\x03\x90\x8B\x8D\x8B\x8D\x8B\x8D\x8B\x8Da\x07\x15V[`@\x80Q`\x80\x81\x01\x82R\x93\x84R` \x84\x01\x94\x90\x94R\x92\x82\x01\x92\x90\x92R``\x81\x01\x91\x90\x91R\x9EPPPPPPPPPPPPPP[\x80a\x03\xCE\x81a\x1E\x0EV[\x91PPa\x02\xE1V[Pa\x03\xE2\x87\x87\x83a\x08bV[PPPPPPPV[\x82\x81\x14a\x04\x15W`@Qcc\x8D\xF5\xD1`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x01\x90V[a\x01\xF4\x83\x11\x15a\x04;W`@Qc\xE0\x82\x84\x0B`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x01\x90V[`\x01T`\0[\x84\x81\x10\x15a\x05\x12Wa\x04\x96`\x01T\x87\x87\x84\x81\x81\x10a\x04aWa\x04aa\x1D\xE2V[\x90P` \x02\x015\x86\x86\x85\x81\x81\x10a\x04zWa\x04za\x1D\xE2V[\x90P` \x02\x81\x01\x90a\x04\x8C\x91\x90a\x1E@V[`\x01\x94\x93PPPPV[a\x04\xB9W`\x01T`@Qcx\x18g\x19`\xE0\x1B\x81R`\x04\x01a\x01\x90\x91\x81R` \x01\x90V[\x85\x85\x82\x81\x81\x10a\x04\xCBWa\x04\xCBa\x1D\xE2V[\x90P` \x02\x015`\0\x80`\x01T\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x80`\0\x82\x82Ta\x04\xFC\x91\x90a\x1E'V[\x90\x91UPa\x05\x0B\x90P\x81a\x1E\x0EV[\x90Pa\x04AV[P`@\x80Q\x82\x81R` \x81\x01\x86\x90R\x7F\x82\x03\xA2\x1EO\x95\xF7.P\x81\xD5\xE0\x92\x9B\x1A\x8CR\x14\x1E\x12?\x9A\x14\xE1\xE7K\x02`\xFA_R\xF1\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[a\x05~`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0`\x03\x83\x81T\x81\x10a\x05\x93Wa\x05\x93a\x1D\xE2V[`\0\x91\x82R` \x80\x83 \x95\x83R`\x02\x80\x82R`@\x93\x84\x90 T\x84Q`\x80\x81\x01\x86R`\x04\x90\x94\x02\x90\x97\x01\x80T\x84R`\x01\x81\x01T\x92\x84\x01\x92\x90\x92R\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x90\x91\x01T``\x82\x01R\x93\x91PPV[`\x03\x80T`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x90U\x84T`\x01\x81\x01\x86U\x94\x90\x92R\x85Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[`\x04\x90\x95\x02\x94\x85\x01\x81\x90U\x86\x82\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8\\\x87\x01U\x87\x84\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8]\x88\x01U``\x80\x8A\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8^\x90\x99\x01\x98\x90\x98U\x85Q\x93\x84R\x91Q\x93\x83\x01\x93\x90\x93R\x91Q\x81\x84\x01R\x93Q\x90\x84\x01R`\x80\x83\x01\x84\x90R`\xA0\x83\x01\x82\x90RQ\x90\x91\x7F\xD7/\xE1\xACW\xD3\xE6\xD5\x1C\x92*\xE4\xD8\x11\xCCP\xAA:\xD7\x02b\x83\xAE\xA67IJ\x072RVZ\x91\x90\x81\x90\x03`\xC0\x01\x90\xA1PPPV[`\0\x80\x80\x80\x8B\x15\x80\x15a\x07&WP\x8A\x15[\x80\x15a\x070WP\x89\x15[\x80\x15a\x07:WP\x88\x15[\x15a\x07\x8BW\x87\x15\x80\x15a\x07KWP\x86\x15[\x80\x15a\x07UWP\x85\x15[\x80\x15a\x07_WP\x84\x15[a\x07{Wa\x07o\x88\x88\x88\x88a\t\x14V[a\x07{Wa\x07{a\x1E\x87V[P\x86\x92P\x85\x91P\x84\x90P\x83a\x08SV[\x87\x15\x80\x15a\x07\x97WP\x86\x15[\x80\x15a\x07\xA1WP\x85\x15[\x80\x15a\x07\xABWP\x84\x15[\x15a\x07\xD8Wa\x07\xBC\x8C\x8C\x8C\x8Ca\t\x14V[a\x07\xC8Wa\x07\xC8a\x1E\x87V[P\x8A\x92P\x89\x91P\x88\x90P\x87a\x08SV[a\x07\xE4\x8C\x8C\x8C\x8Ca\t\x14V[a\x07\xF0Wa\x07\xF0a\x1E\x87V[a\x07\xFC\x88\x88\x88\x88a\t\x14V[a\x08\x08Wa\x08\x08a\x1E\x87V[`\0a\x08\"\x8D\x8D\x8D\x8D`\x01`\0\x8F\x8F\x8F\x8F`\x01`\0a\t\xC9V[\x80Q` \x82\x01Q`@\x83\x01Q``\x84\x01Q`\x80\x85\x01Q`\xA0\x86\x01Q\x95\x96Pa\x08I\x95a\x0CRV[\x94P\x94P\x94P\x94PP[\x98P\x98P\x98P\x98\x94PPPPPV[a\x08k\x82a\x0C\x9CV[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a!\x08`$\x919\x90P`\0\x84\x82`@Q` \x01a\x08\x9D\x92\x91\x90a\x1E\xCDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80a\x08\xBA\x83a\r+V[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91Pa\x08\xEE\x81\x87a\x08\xE1\x8Aa\r\xFEV[a\x08\xE9a\x0EyV[a\x0FJV[a\t\nW`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0\x80`\0\x80`\0a\t(\x87\x87\x89\x89a\x10,V[\x90\x94P\x92Pa\t9\x89\x89\x81\x81a\x10,V[\x90\x92P\x90Pa\tJ\x82\x82\x8B\x8Ba\x10,V[\x90\x92P\x90Pa\t[\x84\x84\x84\x84a\x10\x9DV[\x90\x94P\x92Pa\t\xAB\x84\x84\x7F+\x14\x9D@\xCE\xB8\xAA\xAE\x81\xBE\x18\x99\x1B\xE0j\xC3\xB5\xB4\xC5\xE5Y\xDB\xEF\xA32g\xE6\xDC$\xA18\xE5~\x97\x13\xB0:\xF0\xFE\xD4\xCD,\xAF\xAD\xEE\xD8\xFD\xF4\xA7O\xA0\x84\xE5-\x18R\xE4\xA2\xBD\x06\x85\xC3\x15\xD2a\x10\x9DV[\x90\x94P\x92P\x83\x15\x80\x15a\t\xBCWP\x82\x15[\x99\x98PPPPPPPPPV[a\t\xD1a\x1A\x8DV[\x88\x15\x80\x15a\t\xDDWP\x87\x15[\x15a\n\x1FW\x86\x86\x86\x86\x86\x86\x86`\0[`\xA0\x89\x01\x92\x90\x92R`\x80\x88\x01\x92\x90\x92R``\x87\x01\x92\x90\x92R`@\x86\x01\x92\x90\x92R` \x85\x81\x01\x93\x90\x93R\x90\x91\x02\x01Ra\x0CBV[\x82\x15\x80\x15a\n+WP\x81\x15[\x15a\n>W\x8C\x8C\x8C\x8C\x8C\x8C\x86`\0a\t\xECV[a\nJ\x85\x85\x8B\x8Ba\x10,V[\x90\x95P\x93Pa\n[\x8B\x8B\x85\x85a\x10,V[``\x83\x01R`@\x82\x01Ra\nq\x87\x87\x8B\x8Ba\x10,V[\x90\x97P\x95Pa\n\x82\x8D\x8D\x85\x85a\x10,V[`\xA0\x83\x01R`\x80\x82\x01\x81\x90R\x87\x14\x80\x15a\n\x9FWP`\xA0\x81\x01Q\x86\x14[\x15a\n\xE4W`@\x81\x01Q\x85\x14\x80\x15a\n\xBAWP``\x81\x01Q\x84\x14[\x15a\n\xD5Wa\n\xCD\x8D\x8D\x8D\x8D\x8D\x8Da\x10\xDFV[\x86`\0a\t\xECV[`\x01`\0\x81\x81\x80\x80\x86\x81a\t\xECV[a\n\xF0\x89\x89\x85\x85a\x10,V[\x90\x93P\x91Pa\x0B\x10\x85\x85\x83`\x02` \x02\x01Q\x84`\x03[` \x02\x01Qa\x10\x9DV[\x90\x9DP\x9BPa\x0B*\x87\x87\x83`\x04` \x02\x01Q\x84`\x05a\x0B\x06V[\x90\x9BP\x99Pa\x0B;\x8B\x8B\x81\x81a\x10,V[\x90\x99P\x97Pa\x0B[\x89\x89\x83`\x04` \x02\x01Q\x84`\x05[` \x02\x01Qa\x10,V[\x90\x95P\x93Pa\x0Bl\x89\x89\x8D\x8Da\x10,V[\x90\x99P\x97Pa\x0B}\x89\x89\x85\x85a\x10,V[`\xA0\x83\x01R`\x80\x82\x01Ra\x0B\x93\x8D\x8D\x81\x81a\x10,V[\x90\x97P\x95Pa\x0B\xA4\x87\x87\x85\x85a\x10,V[\x90\x97P\x95Pa\x0B\xB5\x87\x87\x8B\x8Ba\x10\x9DV[\x90\x97P\x95Pa\x0B\xC6\x85\x85`\x02a\x12NV[\x90\x93P\x91Pa\x0B\xD7\x87\x87\x85\x85a\x10\x9DV[\x90\x97P\x95Pa\x0B\xE8\x8B\x8B\x89\x89a\x10,V[` \x83\x01R\x81Ra\x0B\xFB\x85\x85\x89\x89a\x10\x9DV[\x90\x9BP\x99Pa\x0C\x0C\x8D\x8D\x8D\x8Da\x10,V[\x90\x9BP\x99Pa\x0C&\x89\x89\x83`\x02` \x02\x01Q\x84`\x03a\x0BQV[\x90\x9DP\x9BPa\x0C7\x8B\x8B\x8F\x8Fa\x10\x9DV[``\x83\x01R`@\x82\x01R[\x9C\x9BPPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x80a\x0Ce\x88\x88a\x12\x81V[\x90\x92P\x90Pa\x0Cv\x8C\x8C\x84\x84a\x10,V[\x90\x96P\x94Pa\x0C\x87\x8A\x8A\x84\x84a\x10,V[\x96\x9D\x95\x9CP\x9AP\x94\x98P\x92\x96PPPPPPPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a!,\x839\x81Q\x91R\x91\x82`\x03\x81\x80\x85\x80\t\x85\t\x08\x83\x82\x83\t\x14\x81\x15\x83\x15\x17\x19\x84\x83\x10\x85\x85\x10\x16\x16\x16\x93PPP\x81a\r&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[PPPV[`\0\x80`\0a\r9\x84a\x13\rV[\x90P`\0\x80Q` a!,\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\rbWa\rba\x1E\xEAV[\x84\x82\t\x90P\x82\x80a\ruWa\rua\x1E\xEAV[\x82\x82\x08\x90P`\0\x80a\r\x86\x83a\x15@V[\x92P\x90P[\x80a\r\xEFW\x84\x80a\r\x9EWa\r\x9Ea\x1E\xEAV[`\x01\x87\x08\x95P\x84\x80a\r\xB2Wa\r\xB2a\x1E\xEAV[\x86\x87\t\x92P\x84\x80a\r\xC5Wa\r\xC5a\x1E\xEAV[\x86\x84\t\x92P\x84\x80a\r\xD8Wa\r\xD8a\x1E\xEAV[\x84\x84\x08\x92Pa\r\xE6\x83a\x15@V[\x92P\x90Pa\r\x8BV[P\x93\x97\x93\x96P\x92\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x0E&WP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a!,\x839\x81Q\x91R\x84` \x01Qa\x0EY\x91\x90a\x1F\0V[a\x0Eq\x90`\0\x80Q` a!,\x839\x81Q\x91Ra\x1F\"V[\x90R\x92\x91PPV[a\x0E\xA4`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R\x86Q`@\x82\x01R` \x87\x01Q``\x82\x01R`@\x87\x01Q`\x80\x82\x01R``\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R` \x85\x01Qa\x01 \x82\x01R`@\x85\x01Qa\x01@\x82\x01R``\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x10 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[P\x15\x15\x95\x94PPPPPV[`\0\x80a\x10j`\0\x80Q` a!,\x839\x81Q\x91R\x85\x88\t`\0\x80Q` a!,\x839\x81Q\x91R\x85\x88\t`\0\x80Q` a!,\x839\x81Q\x91Ra\x16;V[`\0\x80Q` a!,\x839\x81Q\x91R\x80\x86\x88\t`\0\x80Q` a!,\x839\x81Q\x91R\x86\x8A\t\x08\x91P\x91P\x94P\x94\x92PPPV[`\0\x80a\x10\xB9\x86\x85`\0\x80Q` a!,\x839\x81Q\x91Ra\x16;V[a\x10\xD2\x86\x85`\0\x80Q` a!,\x839\x81Q\x91Ra\x16;V[\x91P\x91P\x94P\x94\x92PPPV[`\0\x80`\0\x80`\0\x80a\x10\xF4\x8C\x8C`\x03a\x12NV[\x90\x96P\x94Pa\x11\x05\x86\x86\x8E\x8Ea\x10,V[\x90\x96P\x94Pa\x11\x16\x8A\x8A\x8A\x8Aa\x10,V[\x90\x98P\x96Pa\x11'\x8C\x8C\x8C\x8Ca\x10,V[\x90\x94P\x92Pa\x118\x84\x84\x8A\x8Aa\x10,V[\x90\x94P\x92Pa\x11I\x86\x86\x81\x81a\x10,V[\x90\x9CP\x9APa\x11Z\x84\x84`\x08a\x12NV[\x90\x92P\x90Pa\x11k\x8C\x8C\x84\x84a\x10\x9DV[\x90\x9CP\x9APa\x11|\x88\x88\x81\x81a\x10,V[\x90\x92P\x90Pa\x11\x8D\x84\x84`\x04a\x12NV[\x90\x94P\x92Pa\x11\x9E\x84\x84\x8E\x8Ea\x10\x9DV[\x90\x94P\x92Pa\x11\xAF\x84\x84\x88\x88a\x10,V[\x90\x94P\x92Pa\x11\xC0\x8A\x8A`\x08a\x12NV[\x90\x96P\x94Pa\x11\xD1\x86\x86\x8C\x8Ca\x10,V[\x90\x96P\x94Pa\x11\xE2\x86\x86\x84\x84a\x10,V[\x90\x96P\x94Pa\x11\xF3\x84\x84\x88\x88a\x10\x9DV[\x90\x94P\x92Pa\x12\x04\x8C\x8C`\x02a\x12NV[\x90\x96P\x94Pa\x12\x15\x86\x86\x8A\x8Aa\x10,V[\x90\x96P\x94Pa\x12&\x88\x88\x84\x84a\x10,V[\x90\x92P\x90Pa\x127\x82\x82`\x08a\x12NV[\x80\x92P\x81\x93PPP\x96P\x96P\x96P\x96P\x96P\x96\x90PV[`\0\x80`\0\x80Q` a!,\x839\x81Q\x91R\x83\x86\t`\0\x80Q` a!,\x839\x81Q\x91R\x84\x86\t\x91P\x91P\x93P\x93\x91PPV[`\0\x80\x80a\x12\xC2`\0\x80Q` a!,\x839\x81Q\x91R\x80\x87\x88\t`\0\x80Q` a!,\x839\x81Q\x91R\x87\x88\t\x08`\0\x80Q` a!,\x839\x81Q\x91Ra\x16_V[\x90P`\0\x80Q` a!,\x839\x81Q\x91R\x81\x86\t`\0\x80Q` a!,\x839\x81Q\x91R\x82\x86\ta\x13\0\x90`\0\x80Q` a!,\x839\x81Q\x91Ra\x1F\"V[\x92P\x92PP[\x92P\x92\x90PV[`\0\x80a\x13\x19\x83a\x16\xB0V[\x80Q\x90\x91P`0\x81\x14a\x13.Wa\x13.a\x1E\x87V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13IWa\x13Ia\x1A\xABV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x13sW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x13\xEEW\x83`\x01a\x13\x8E\x83\x86a\x1F\"V[a\x13\x98\x91\x90a\x1F\"V[\x81Q\x81\x10a\x13\xA8Wa\x13\xA8a\x1D\xE2V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x13\xC5Wa\x13\xC5a\x1D\xE2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x13\xE6\x81a\x1E\x0EV[\x91PPa\x13yV[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x14\x8AW\x83\x81a\x14,\x85\x88a\x1F\"V[a\x146\x91\x90a\x1E'V[\x81Q\x81\x10a\x14FWa\x14Fa\x1D\xE2V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x14fWa\x14fa\x1D\xE2V[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x14\x82\x81a\x1E\x0EV[\x91PPa\x14\x18V[P`\0a\x14\x96\x82a\x1A\"V[\x90Pa\x01\0`\0\x80Q` a!,\x839\x81Q\x91R`\0a\x14\xB6\x86\x89a\x1F\"V[\x90P`\0[\x81\x81\x10\x15a\x150W`\0\x88`\x01a\x14\xD2\x84\x86a\x1F\"V[a\x14\xDC\x91\x90a\x1F\"V[\x81Q\x81\x10a\x14\xECWa\x14\xECa\x1D\xE2V[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x15\x04Wa\x15\x04a\x1E\xEAV[\x85\x87\t\x95P\x83\x80a\x15\x17Wa\x15\x17a\x1E\xEAV[\x81\x87\x08\x95PP\x80\x80a\x15(\x90a\x1E\x0EV[\x91PPa\x14\xBBV[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a!,\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x86``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x93PP`\0Q\x93P\x82a\x16\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[\x80`\x01\x85\x90\x1B\x11\x15a\x16\x1BWa\x16\x18\x84\x82a\x1F\"V[\x93P[\x80\x80a\x16)Wa\x16)a\x1E\xEAV[\x84\x85\t\x91P\x85\x82\x14\x94PPPP\x91P\x91V[`\0\x81\x80a\x16KWa\x16Ka\x1E\xEAV[a\x16U\x84\x84a\x1F\"V[\x85\x08\x94\x93PPPPV[`\0\x80`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01R`\x02\x84\x03`\x80\x82\x01R\x83`\xA0\x82\x01R` \x81`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x90Q\x92P\x90P\x80a\x16\xA9W`\0\x80\xFD[P\x92\x91PPV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x16\xF1\x92\x91\x90a\x1E\xCDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x17\x18\x92\x91\x90a\x1F5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x17:\x91\x90a\x1FaV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x17d\x90\x83\x90\x83\x90` \x01a\x1F{V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\xD5Wa\x17\xD5a\x1A\xABV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x17\xFFW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x18\x17\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x18\x8CW\x81\x81\x81Q\x81\x10a\x18FWa\x18Fa\x1D\xE2V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x18cWa\x18ca\x1D\xE2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x18\x84\x81a\x1E\x0EV[\x91PPa\x18+V[P`\0\x84`@Q` \x01a\x18\xA2\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x19@W`\0\x83\x82\x81Q\x81\x10a\x18\xDDWa\x18\xDDa\x1D\xE2V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x18\xFAWa\x18\xFAa\x1D\xE2V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x19\x1B\x92\x91\x90a\x1F\xA0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x98PP\x80\x80a\x198\x90a\x1E\x0EV[\x91PPa\x18\xC1V[P\x86\x88\x87`@Q` \x01a\x19V\x93\x92\x91\x90a\x1F\xC5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x19\x84\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x19\xA5\x8A`\xFF\x8D\x16a\x1F\"V[\x81\x10\x15a\x1A\x11W\x82\x81\x81Q\x81\x10a\x19\xBEWa\x19\xBEa\x1D\xE2V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x19\xD8\x83\x8Da\x1E'V[\x81Q\x81\x10a\x19\xE8Wa\x19\xE8a\x1D\xE2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x1A\t\x81a\x1E\x0EV[\x91PPa\x19\x98V[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a\x16\xA9W\x83\x81\x81Q\x81\x10a\x1ABWa\x1ABa\x1D\xE2V[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x1AZ\x91\x90a\x1F\xF9V[a\x1Ae\x90`\x02a \xF4V[a\x1Ao\x91\x90a\x1F\xF9V[a\x1Ay\x90\x83a\x1E'V[\x91P\x80a\x1A\x85\x81a\x1E\x0EV[\x91PPa\x1A'V[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A\xEAWa\x1A\xEAa\x1A\xABV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x1B\x04W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1B'Wa\x1B'a\x1A\xABV[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1BRW`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1BnWa\x1Bna\x1A\xABV[\x81`\x05\x1Ba\x1B}\x82\x82\x01a\x1A\xC1V[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x1B\x97W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x1B\xC5W\x825\x80\x15\x15\x81\x14a\x1B\xB6W`\0\x80\x81\xFD[\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x1B\x9DV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\xA0\x85\x87\x03\x12\x15a\x1B\xE6W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\xFEW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x1C\x12W`\0\x80\xFD[\x815` \x82\x82\x11\x15a\x1C&Wa\x1C&a\x1A\xABV[a\x1C8`\x1F\x83\x01`\x1F\x19\x16\x82\x01a\x1A\xC1V[\x82\x81R\x8A\x82\x84\x87\x01\x01\x11\x15a\x1CLW`\0\x80\xFD[\x82\x82\x86\x01\x83\x83\x017`\0\x81\x84\x01\x83\x01R\x97Pa\x1Cj\x8A\x8A\x83\x01a\x1A\xF2V[\x96PPP``\x87\x015\x91P\x80\x82\x11\x15a\x1C\x82W`\0\x80\xFD[Pa\x1C\x8F\x87\x82\x88\x01a\x1BAV[\x94\x97\x93\x96P\x93\x94`\x80\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x1C\xB2W`\0\x80\xFD[P5\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x1C\xCBW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xE3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x13\x06W`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x1D\x14W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1D,W`\0\x80\xFD[a\x1D8\x88\x83\x89\x01a\x1C\xB9V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x1DQW`\0\x80\xFD[Pa\x1D^\x87\x82\x88\x01a\x1C\xB9V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a\x1D~W`\0\x80\xFD[`\x80\x81\x12\x15a\x1D\x8CW`\0\x80\xFD[P`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1D\xB0Wa\x1D\xB0a\x1A\xABV[`@\x90\x81R\x845\x82R` \x80\x86\x015\x90\x83\x01R\x84\x81\x015\x90\x82\x01R``\x80\x85\x015\x90\x82\x01R\x94`\x80\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1E Wa\x1E a\x1D\xF8V[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x1E:Wa\x1E:a\x1D\xF8V[\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1EWW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1ErW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x13\x06W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x81Q`\0[\x81\x81\x10\x15a\x1E\xBEW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x1E\xA4V[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x1E\xE2a\x1E\xDC\x83\x86a\x1E\x9DV[\x84a\x1E\x9DV[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1F\x1DWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1E:Wa\x1E:a\x1D\xF8V[`\0a\x1FA\x82\x85a\x1E\x9DV[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a\x1Fm\x82\x84a\x1E\x9DV[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a\x1F\x87\x82\x85a\x1E\x9DV[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a\x1F\xAC\x82\x85a\x1E\x9DV[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a\x1F\xD1\x82\x86a\x1E\x9DV[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1E:Wa\x1E:a\x1D\xF8V[`\x01\x81\x81[\x80\x85\x11\x15a KW\x81`\0\x19\x04\x82\x11\x15a 1Wa 1a\x1D\xF8V[\x80\x85\x16\x15a >W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a \x15V[P\x92P\x92\x90PV[`\0\x82a bWP`\x01a\x1E:V[\x81a oWP`\0a\x1E:V[\x81`\x01\x81\x14a \x85W`\x02\x81\x14a \x8FWa \xABV[`\x01\x91PPa\x1E:V[`\xFF\x84\x11\x15a \xA0Wa \xA0a\x1D\xF8V[PP`\x01\x82\x1Ba\x1E:V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a \xCEWP\x81\x81\na\x1E:V[a \xD8\x83\x83a \x10V[\x80`\0\x19\x04\x82\x11\x15a \xECWa \xECa\x1D\xF8V[\x02\x93\x92PPPV[`\0a!\0\x83\x83a SV[\x93\x92PPPV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \x03D\xFD\x08\x04\x03\x8D\xD0\xA7X\x9E-~\xF2\x1F \xA2\x19\x8EG \x1C<\xA8y\xD3\x8F\xBCH\xC7\xCD\x94dsolcC\0\x08\x14\x003\xA2dipfsX\"\x12 \xFFd\x0F\x16\x10&\x10?\xD2\x18\xC4\x13\x8C\xBAw?vJ\x17\t\xFE}rPD-\x8D\xAAvm\x95\x01dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static HOTSHOTTEST_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\x8CW\x80c\xB5P\x8A\xA9\x11a\0fW\x80c\xB5P\x8A\xA9\x14a\x01qW\x80c\xBAAO\xA6\x14a\x01yW\x80c\xE2\x0C\x9Fq\x14a\x01\x91W\x80c\xFAv&\xD4\x14a\x01\x99W`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x01LW\x80c\x91j\x17\xC6\x14a\x01aW\x80c\x99\x97\xAF\xDB\x14a\x01iW`\0\x80\xFD[\x80c\n\x92T\xE4\x14a\0\xD4W\x80c\x1E\xD7\x83\x1C\x14a\0\xDEW\x80c*\xDC\x8Bv\x14a\0\xFCW\x80c>^<#\x14a\x01'W\x80c?r\x86\xF4\x14a\x01/W\x80cf\xD9\xA9\xA0\x14a\x017W[`\0\x80\xFD[a\0\xDCa\x01\xA6V[\0[a\0\xE6a\x01\xF1V[`@Qa\0\xF3\x91\x90a\x0E\x1CV[`@Q\x80\x91\x03\x90\xF3[`\x1BTa\x01\x0F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF3V[a\0\xE6a\x02SV[a\0\xE6a\x02\xB3V[a\x01?a\x03\x13V[`@Qa\0\xF3\x91\x90a\x0EiV[a\x01Ta\x04\x02V[`@Qa\0\xF3\x91\x90a\x0FlV[a\x01?a\x04\xD2V[a\0\xDCa\x05\xB8V[a\x01Ta\t\x86V[a\x01\x81a\nVV[`@Q\x90\x15\x15\x81R` \x01a\0\xF3V[a\0\xE6a\x0B\x81V[`\0Ta\x01\x81\x90`\xFF\x16\x81V[`@Qa\x01\xB2\x90a\x0E\x0FV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x01\xCEW=`\0\x80>=`\0\xFD[P`\x1B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+WPPPPP\x90P\x90V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x03\xE1W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x03\xA3W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x037V[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x04E\x90a\x0F\xCEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04q\x90a\x0F\xCEV[\x80\x15a\x04\xBEW\x80`\x1F\x10a\x04\x93Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xBEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xA1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04&V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x05\xA0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x05bW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04\xF6V[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R`\0\x92` \x83\x01\x90\x806\x837PP`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x82\x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05\xF2W\x90PP\x90Pd\x868#p\x95\x82`\0\x81Q\x81\x10a\x06\"Wa\x06\"a\x10\x08V[` \x02` \x01\x01\x81\x81RPP`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e0x3333`\xD0\x1B\x81RP\x81`\0\x81Q\x81\x10a\x06`Wa\x06`a\x10\x08V[` \x02` \x01\x01\x81\x90RPd6\x8B\xD51\xFE\x82`\x01\x81Q\x81\x10a\x06\x84Wa\x06\x84a\x10\x08V[` \x02` \x01\x01\x81\x81RPP`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e\x0C\x1E\r\r\r\r`\xD2\x1B\x81RP\x81`\x01\x81Q\x81\x10a\x06\xC2Wa\x06\xC2a\x10\x08V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x1BT`@Qc\x81\xBA\xD6\xF3`\xE0\x1B\x81R`\0`\x04\x82\x01\x81\x90R`$\x82\x01\x81\x90R`D\x82\x01R`\x01`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x84\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x81\xBA\xD6\xF3\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07VW=`\0\x80>=`\0\xFD[PP`@\x80Q`\0\x81R`\x02` \x82\x01R\x7F\x82\x03\xA2\x1EO\x95\xF7.P\x81\xD5\xE0\x92\x9B\x1A\x8CR\x14\x1E\x12?\x9A\x14\xE1\xE7K\x02`\xFA_R\xF1\x93P\x01\x90P`@Q\x80\x91\x03\x90\xA1`\x1BT`@Qc\x06v\x925`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cgi#P\x90a\x07\xC7\x90\x85\x90\x85\x90`\x04\x01a\x10\x1EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xF5W=`\0\x80>=`\0\xFD[PP`\x1BT`@QcI\xCE\x89\x97`\xE0\x1B\x81R`\0`\x04\x82\x01Ra\x08\x8A\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91PcI\xCE\x89\x97\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08j\x91\x90a\x10\xB6V[\x83`\0\x81Q\x81\x10a\x08}Wa\x08}a\x10\x08V[` \x02` \x01\x01Qa\x0B\xE1V[`\x1BT`@QcI\xCE\x89\x97`\xE0\x1B\x81R`\x01`\x04\x82\x01Ra\t\x0C\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cI\xCE\x89\x97\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xF9\x91\x90a\x10\xB6V[\x83`\x01\x81Q\x81\x10a\x08}Wa\x08}a\x10\x08V[`\x1BT`@QcI\xCE\x89\x97`\xE0\x1B\x81R`\x02`\x04\x82\x01Ra\t\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cI\xCE\x89\x97\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tWW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t{\x91\x90a\x10\xB6V[`\0a\x0B\xE1V[PPV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W\x83\x82\x90`\0R` `\0 \x01\x80Ta\t\xC9\x90a\x0F\xCEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xF5\x90a\x0F\xCEV[\x80\x15a\nBW\x80`\x1F\x10a\n\x17Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\nBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n%W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\t\xAAV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\nvWP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0B|W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x0B\x04\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x10\xCFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0B\x1E\x91a\x11\0V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0B[W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B`V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x0Bx\x91\x90a\x11\x1CV[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+WPPPPP\x90P\x90V[\x80\x82\x14a\t\x82W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x0CR\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1a\t\x82sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\r\xFEW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\x9D\x92\x91` \x01a\x10\xCFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\xB7\x91a\x11\0V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\r\xF4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xF9V[``\x91P[PPPP[`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[a!\xA1\x80a\x11F\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0E]W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E8V[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\x0F\rW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\x0E\xF8W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a\x0E\xCEV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\x0E\x91V[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15a\x0F7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0F\x1FV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0FX\x81` \x86\x01` \x86\x01a\x0F\x1CV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x0F\xC1W`?\x19\x88\x86\x03\x01\x84Ra\x0F\xAF\x85\x83Qa\x0F@V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x0F\x93V[P\x92\x97\x96PPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0F\xE2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x10\x02WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R`\0\x90` \x90``\x84\x01\x90\x82\x87\x01\x84[\x82\x81\x10\x15a\x10WW\x81Q\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x10;V[PPP\x83\x81\x03\x82\x85\x01R\x84Q\x80\x82R\x82\x82\x01\x90`\x05\x81\x90\x1B\x83\x01\x84\x01\x87\x85\x01`\0[\x83\x81\x10\x15a\x10\xA7W`\x1F\x19\x86\x84\x03\x01\x85Ra\x10\x95\x83\x83Qa\x0F@V[\x94\x87\x01\x94\x92P\x90\x86\x01\x90`\x01\x01a\x10yV[P\x90\x99\x98PPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x10\xC8W`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a\x10\xF2\x81`\x04\x85\x01` \x87\x01a\x0F\x1CV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa\x11\x12\x81\x84` \x87\x01a\x0F\x1CV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11.W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x11>W`\0\x80\xFD[\x93\x92PPPV\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa!\x81\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cgi#P\x11a\0[W\x80cgi#P\x14a\0\xD3W\x80cg\xA2\x1Ep\x14a\0\xE6W\x80c\xF1\xF4]\x99\x14a\x010W\x80c\xF4O\xF7\x12\x14a\x01CW`\0\x80\xFD[\x80c\x03@\x96\x1E\x14a\0\x82W\x80c&\x83=\xCC\x14a\0\x97W\x80cI\xCE\x89\x97\x14a\0\xB3W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x1B\xD0V[a\x01LV[\0[a\0\xA0a\x01\xF4\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA0a\0\xC16`\x04a\x1C\xA0V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\0\x95a\0\xE16`\x04a\x1C\xFEV[a\x03\xEBV[a\0\xF9a\0\xF46`\x04a\x1C\xA0V[a\x05SV[`@\x80Q\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R\x83\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x92\x83\x01Q\x92\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x01a\0\xAAV[a\0\x95a\x01>6`\x04a\x1DjV[a\x05\xE7V[a\0\xA0`\x01T\x81V[`\x03T\x82Q\x11\x15a\x01\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqbitmap is too long`p\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x81Q\x81\x10a\x01\xAEWa\x01\xAEa\x1D\xE2V[` \x02` \x01\x01Q\x15\x80\x15a\x01\xC3WP\x82Q\x81\x10[\x15a\x01\xDAW\x80a\x01\xD2\x81a\x1E\x0EV[\x91PPa\x01\x9CV[\x82Q\x81\x10a\x01\xFBW`@QcKe\x82-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81[\x84Q\x81\x10\x15a\x02UW\x84\x81\x81Q\x81\x10a\x02\x1AWa\x02\x1Aa\x1D\xE2V[` \x02` \x01\x01Q\x15a\x02CW`\0\x81\x81R`\x02` R`@\x90 Ta\x02@\x90\x83a\x1E'V[\x91P[\x80a\x02M\x81a\x1E\x0EV[\x91PPa\x01\xFFV[P\x82\x81\x10\x15a\x02wW`@Qc<)\x0BS`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x03\x83\x81T\x81\x10a\x02\x8CWa\x02\x8Ca\x1D\xE2V[\x90`\0R` `\0 \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPP\x90P`\0\x83`\x01a\x02\xDE\x91\x90a\x1E'V[\x90P[\x85Q\x81\x10\x15a\x03\xD6W\x85\x81\x81Q\x81\x10a\x02\xFCWa\x02\xFCa\x1D\xE2V[` \x02` \x01\x01Q\x15a\x03\xC4W`\0`\x03\x82\x81T\x81\x10a\x03\x1EWa\x03\x1Ea\x1D\xE2V[`\0\x91\x82R` \x80\x83 `@\x80Q`\x80\x81\x01\x82R`\x04\x90\x94\x02\x90\x91\x01\x80T\x80\x85R`\x01\x82\x01T\x85\x85\x01\x81\x90R`\x02\x83\x01T\x86\x85\x01\x81\x90R`\x03\x90\x93\x01T``\x80\x88\x01\x82\x90R\x8BQ\x96\x8C\x01Q\x95\x8C\x01Q\x90\x8C\x01Q\x97\x99P\x95\x97\x94\x96\x94\x92\x93\x91\x92\x80\x80\x80a\x03\x90\x8B\x8D\x8B\x8D\x8B\x8D\x8B\x8Da\x07\x15V[`@\x80Q`\x80\x81\x01\x82R\x93\x84R` \x84\x01\x94\x90\x94R\x92\x82\x01\x92\x90\x92R``\x81\x01\x91\x90\x91R\x9EPPPPPPPPPPPPPP[\x80a\x03\xCE\x81a\x1E\x0EV[\x91PPa\x02\xE1V[Pa\x03\xE2\x87\x87\x83a\x08bV[PPPPPPPV[\x82\x81\x14a\x04\x15W`@Qcc\x8D\xF5\xD1`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x01\x90V[a\x01\xF4\x83\x11\x15a\x04;W`@Qc\xE0\x82\x84\x0B`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x01\x90V[`\x01T`\0[\x84\x81\x10\x15a\x05\x12Wa\x04\x96`\x01T\x87\x87\x84\x81\x81\x10a\x04aWa\x04aa\x1D\xE2V[\x90P` \x02\x015\x86\x86\x85\x81\x81\x10a\x04zWa\x04za\x1D\xE2V[\x90P` \x02\x81\x01\x90a\x04\x8C\x91\x90a\x1E@V[`\x01\x94\x93PPPPV[a\x04\xB9W`\x01T`@Qcx\x18g\x19`\xE0\x1B\x81R`\x04\x01a\x01\x90\x91\x81R` \x01\x90V[\x85\x85\x82\x81\x81\x10a\x04\xCBWa\x04\xCBa\x1D\xE2V[\x90P` \x02\x015`\0\x80`\x01T\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x80`\0\x82\x82Ta\x04\xFC\x91\x90a\x1E'V[\x90\x91UPa\x05\x0B\x90P\x81a\x1E\x0EV[\x90Pa\x04AV[P`@\x80Q\x82\x81R` \x81\x01\x86\x90R\x7F\x82\x03\xA2\x1EO\x95\xF7.P\x81\xD5\xE0\x92\x9B\x1A\x8CR\x14\x1E\x12?\x9A\x14\xE1\xE7K\x02`\xFA_R\xF1\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[a\x05~`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0`\x03\x83\x81T\x81\x10a\x05\x93Wa\x05\x93a\x1D\xE2V[`\0\x91\x82R` \x80\x83 \x95\x83R`\x02\x80\x82R`@\x93\x84\x90 T\x84Q`\x80\x81\x01\x86R`\x04\x90\x94\x02\x90\x97\x01\x80T\x84R`\x01\x81\x01T\x92\x84\x01\x92\x90\x92R\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x90\x91\x01T``\x82\x01R\x93\x91PPV[`\x03\x80T`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x90U\x84T`\x01\x81\x01\x86U\x94\x90\x92R\x85Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[`\x04\x90\x95\x02\x94\x85\x01\x81\x90U\x86\x82\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8\\\x87\x01U\x87\x84\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8]\x88\x01U``\x80\x8A\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8^\x90\x99\x01\x98\x90\x98U\x85Q\x93\x84R\x91Q\x93\x83\x01\x93\x90\x93R\x91Q\x81\x84\x01R\x93Q\x90\x84\x01R`\x80\x83\x01\x84\x90R`\xA0\x83\x01\x82\x90RQ\x90\x91\x7F\xD7/\xE1\xACW\xD3\xE6\xD5\x1C\x92*\xE4\xD8\x11\xCCP\xAA:\xD7\x02b\x83\xAE\xA67IJ\x072RVZ\x91\x90\x81\x90\x03`\xC0\x01\x90\xA1PPPV[`\0\x80\x80\x80\x8B\x15\x80\x15a\x07&WP\x8A\x15[\x80\x15a\x070WP\x89\x15[\x80\x15a\x07:WP\x88\x15[\x15a\x07\x8BW\x87\x15\x80\x15a\x07KWP\x86\x15[\x80\x15a\x07UWP\x85\x15[\x80\x15a\x07_WP\x84\x15[a\x07{Wa\x07o\x88\x88\x88\x88a\t\x14V[a\x07{Wa\x07{a\x1E\x87V[P\x86\x92P\x85\x91P\x84\x90P\x83a\x08SV[\x87\x15\x80\x15a\x07\x97WP\x86\x15[\x80\x15a\x07\xA1WP\x85\x15[\x80\x15a\x07\xABWP\x84\x15[\x15a\x07\xD8Wa\x07\xBC\x8C\x8C\x8C\x8Ca\t\x14V[a\x07\xC8Wa\x07\xC8a\x1E\x87V[P\x8A\x92P\x89\x91P\x88\x90P\x87a\x08SV[a\x07\xE4\x8C\x8C\x8C\x8Ca\t\x14V[a\x07\xF0Wa\x07\xF0a\x1E\x87V[a\x07\xFC\x88\x88\x88\x88a\t\x14V[a\x08\x08Wa\x08\x08a\x1E\x87V[`\0a\x08\"\x8D\x8D\x8D\x8D`\x01`\0\x8F\x8F\x8F\x8F`\x01`\0a\t\xC9V[\x80Q` \x82\x01Q`@\x83\x01Q``\x84\x01Q`\x80\x85\x01Q`\xA0\x86\x01Q\x95\x96Pa\x08I\x95a\x0CRV[\x94P\x94P\x94P\x94PP[\x98P\x98P\x98P\x98\x94PPPPPV[a\x08k\x82a\x0C\x9CV[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a!\x08`$\x919\x90P`\0\x84\x82`@Q` \x01a\x08\x9D\x92\x91\x90a\x1E\xCDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80a\x08\xBA\x83a\r+V[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91Pa\x08\xEE\x81\x87a\x08\xE1\x8Aa\r\xFEV[a\x08\xE9a\x0EyV[a\x0FJV[a\t\nW`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0\x80`\0\x80`\0a\t(\x87\x87\x89\x89a\x10,V[\x90\x94P\x92Pa\t9\x89\x89\x81\x81a\x10,V[\x90\x92P\x90Pa\tJ\x82\x82\x8B\x8Ba\x10,V[\x90\x92P\x90Pa\t[\x84\x84\x84\x84a\x10\x9DV[\x90\x94P\x92Pa\t\xAB\x84\x84\x7F+\x14\x9D@\xCE\xB8\xAA\xAE\x81\xBE\x18\x99\x1B\xE0j\xC3\xB5\xB4\xC5\xE5Y\xDB\xEF\xA32g\xE6\xDC$\xA18\xE5~\x97\x13\xB0:\xF0\xFE\xD4\xCD,\xAF\xAD\xEE\xD8\xFD\xF4\xA7O\xA0\x84\xE5-\x18R\xE4\xA2\xBD\x06\x85\xC3\x15\xD2a\x10\x9DV[\x90\x94P\x92P\x83\x15\x80\x15a\t\xBCWP\x82\x15[\x99\x98PPPPPPPPPV[a\t\xD1a\x1A\x8DV[\x88\x15\x80\x15a\t\xDDWP\x87\x15[\x15a\n\x1FW\x86\x86\x86\x86\x86\x86\x86`\0[`\xA0\x89\x01\x92\x90\x92R`\x80\x88\x01\x92\x90\x92R``\x87\x01\x92\x90\x92R`@\x86\x01\x92\x90\x92R` \x85\x81\x01\x93\x90\x93R\x90\x91\x02\x01Ra\x0CBV[\x82\x15\x80\x15a\n+WP\x81\x15[\x15a\n>W\x8C\x8C\x8C\x8C\x8C\x8C\x86`\0a\t\xECV[a\nJ\x85\x85\x8B\x8Ba\x10,V[\x90\x95P\x93Pa\n[\x8B\x8B\x85\x85a\x10,V[``\x83\x01R`@\x82\x01Ra\nq\x87\x87\x8B\x8Ba\x10,V[\x90\x97P\x95Pa\n\x82\x8D\x8D\x85\x85a\x10,V[`\xA0\x83\x01R`\x80\x82\x01\x81\x90R\x87\x14\x80\x15a\n\x9FWP`\xA0\x81\x01Q\x86\x14[\x15a\n\xE4W`@\x81\x01Q\x85\x14\x80\x15a\n\xBAWP``\x81\x01Q\x84\x14[\x15a\n\xD5Wa\n\xCD\x8D\x8D\x8D\x8D\x8D\x8Da\x10\xDFV[\x86`\0a\t\xECV[`\x01`\0\x81\x81\x80\x80\x86\x81a\t\xECV[a\n\xF0\x89\x89\x85\x85a\x10,V[\x90\x93P\x91Pa\x0B\x10\x85\x85\x83`\x02` \x02\x01Q\x84`\x03[` \x02\x01Qa\x10\x9DV[\x90\x9DP\x9BPa\x0B*\x87\x87\x83`\x04` \x02\x01Q\x84`\x05a\x0B\x06V[\x90\x9BP\x99Pa\x0B;\x8B\x8B\x81\x81a\x10,V[\x90\x99P\x97Pa\x0B[\x89\x89\x83`\x04` \x02\x01Q\x84`\x05[` \x02\x01Qa\x10,V[\x90\x95P\x93Pa\x0Bl\x89\x89\x8D\x8Da\x10,V[\x90\x99P\x97Pa\x0B}\x89\x89\x85\x85a\x10,V[`\xA0\x83\x01R`\x80\x82\x01Ra\x0B\x93\x8D\x8D\x81\x81a\x10,V[\x90\x97P\x95Pa\x0B\xA4\x87\x87\x85\x85a\x10,V[\x90\x97P\x95Pa\x0B\xB5\x87\x87\x8B\x8Ba\x10\x9DV[\x90\x97P\x95Pa\x0B\xC6\x85\x85`\x02a\x12NV[\x90\x93P\x91Pa\x0B\xD7\x87\x87\x85\x85a\x10\x9DV[\x90\x97P\x95Pa\x0B\xE8\x8B\x8B\x89\x89a\x10,V[` \x83\x01R\x81Ra\x0B\xFB\x85\x85\x89\x89a\x10\x9DV[\x90\x9BP\x99Pa\x0C\x0C\x8D\x8D\x8D\x8Da\x10,V[\x90\x9BP\x99Pa\x0C&\x89\x89\x83`\x02` \x02\x01Q\x84`\x03a\x0BQV[\x90\x9DP\x9BPa\x0C7\x8B\x8B\x8F\x8Fa\x10\x9DV[``\x83\x01R`@\x82\x01R[\x9C\x9BPPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x80a\x0Ce\x88\x88a\x12\x81V[\x90\x92P\x90Pa\x0Cv\x8C\x8C\x84\x84a\x10,V[\x90\x96P\x94Pa\x0C\x87\x8A\x8A\x84\x84a\x10,V[\x96\x9D\x95\x9CP\x9AP\x94\x98P\x92\x96PPPPPPPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a!,\x839\x81Q\x91R\x91\x82`\x03\x81\x80\x85\x80\t\x85\t\x08\x83\x82\x83\t\x14\x81\x15\x83\x15\x17\x19\x84\x83\x10\x85\x85\x10\x16\x16\x16\x93PPP\x81a\r&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[PPPV[`\0\x80`\0a\r9\x84a\x13\rV[\x90P`\0\x80Q` a!,\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\rbWa\rba\x1E\xEAV[\x84\x82\t\x90P\x82\x80a\ruWa\rua\x1E\xEAV[\x82\x82\x08\x90P`\0\x80a\r\x86\x83a\x15@V[\x92P\x90P[\x80a\r\xEFW\x84\x80a\r\x9EWa\r\x9Ea\x1E\xEAV[`\x01\x87\x08\x95P\x84\x80a\r\xB2Wa\r\xB2a\x1E\xEAV[\x86\x87\t\x92P\x84\x80a\r\xC5Wa\r\xC5a\x1E\xEAV[\x86\x84\t\x92P\x84\x80a\r\xD8Wa\r\xD8a\x1E\xEAV[\x84\x84\x08\x92Pa\r\xE6\x83a\x15@V[\x92P\x90Pa\r\x8BV[P\x93\x97\x93\x96P\x92\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x0E&WP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a!,\x839\x81Q\x91R\x84` \x01Qa\x0EY\x91\x90a\x1F\0V[a\x0Eq\x90`\0\x80Q` a!,\x839\x81Q\x91Ra\x1F\"V[\x90R\x92\x91PPV[a\x0E\xA4`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R\x86Q`@\x82\x01R` \x87\x01Q``\x82\x01R`@\x87\x01Q`\x80\x82\x01R``\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R` \x85\x01Qa\x01 \x82\x01R`@\x85\x01Qa\x01@\x82\x01R``\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x10 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[P\x15\x15\x95\x94PPPPPV[`\0\x80a\x10j`\0\x80Q` a!,\x839\x81Q\x91R\x85\x88\t`\0\x80Q` a!,\x839\x81Q\x91R\x85\x88\t`\0\x80Q` a!,\x839\x81Q\x91Ra\x16;V[`\0\x80Q` a!,\x839\x81Q\x91R\x80\x86\x88\t`\0\x80Q` a!,\x839\x81Q\x91R\x86\x8A\t\x08\x91P\x91P\x94P\x94\x92PPPV[`\0\x80a\x10\xB9\x86\x85`\0\x80Q` a!,\x839\x81Q\x91Ra\x16;V[a\x10\xD2\x86\x85`\0\x80Q` a!,\x839\x81Q\x91Ra\x16;V[\x91P\x91P\x94P\x94\x92PPPV[`\0\x80`\0\x80`\0\x80a\x10\xF4\x8C\x8C`\x03a\x12NV[\x90\x96P\x94Pa\x11\x05\x86\x86\x8E\x8Ea\x10,V[\x90\x96P\x94Pa\x11\x16\x8A\x8A\x8A\x8Aa\x10,V[\x90\x98P\x96Pa\x11'\x8C\x8C\x8C\x8Ca\x10,V[\x90\x94P\x92Pa\x118\x84\x84\x8A\x8Aa\x10,V[\x90\x94P\x92Pa\x11I\x86\x86\x81\x81a\x10,V[\x90\x9CP\x9APa\x11Z\x84\x84`\x08a\x12NV[\x90\x92P\x90Pa\x11k\x8C\x8C\x84\x84a\x10\x9DV[\x90\x9CP\x9APa\x11|\x88\x88\x81\x81a\x10,V[\x90\x92P\x90Pa\x11\x8D\x84\x84`\x04a\x12NV[\x90\x94P\x92Pa\x11\x9E\x84\x84\x8E\x8Ea\x10\x9DV[\x90\x94P\x92Pa\x11\xAF\x84\x84\x88\x88a\x10,V[\x90\x94P\x92Pa\x11\xC0\x8A\x8A`\x08a\x12NV[\x90\x96P\x94Pa\x11\xD1\x86\x86\x8C\x8Ca\x10,V[\x90\x96P\x94Pa\x11\xE2\x86\x86\x84\x84a\x10,V[\x90\x96P\x94Pa\x11\xF3\x84\x84\x88\x88a\x10\x9DV[\x90\x94P\x92Pa\x12\x04\x8C\x8C`\x02a\x12NV[\x90\x96P\x94Pa\x12\x15\x86\x86\x8A\x8Aa\x10,V[\x90\x96P\x94Pa\x12&\x88\x88\x84\x84a\x10,V[\x90\x92P\x90Pa\x127\x82\x82`\x08a\x12NV[\x80\x92P\x81\x93PPP\x96P\x96P\x96P\x96P\x96P\x96\x90PV[`\0\x80`\0\x80Q` a!,\x839\x81Q\x91R\x83\x86\t`\0\x80Q` a!,\x839\x81Q\x91R\x84\x86\t\x91P\x91P\x93P\x93\x91PPV[`\0\x80\x80a\x12\xC2`\0\x80Q` a!,\x839\x81Q\x91R\x80\x87\x88\t`\0\x80Q` a!,\x839\x81Q\x91R\x87\x88\t\x08`\0\x80Q` a!,\x839\x81Q\x91Ra\x16_V[\x90P`\0\x80Q` a!,\x839\x81Q\x91R\x81\x86\t`\0\x80Q` a!,\x839\x81Q\x91R\x82\x86\ta\x13\0\x90`\0\x80Q` a!,\x839\x81Q\x91Ra\x1F\"V[\x92P\x92PP[\x92P\x92\x90PV[`\0\x80a\x13\x19\x83a\x16\xB0V[\x80Q\x90\x91P`0\x81\x14a\x13.Wa\x13.a\x1E\x87V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13IWa\x13Ia\x1A\xABV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x13sW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x13\xEEW\x83`\x01a\x13\x8E\x83\x86a\x1F\"V[a\x13\x98\x91\x90a\x1F\"V[\x81Q\x81\x10a\x13\xA8Wa\x13\xA8a\x1D\xE2V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x13\xC5Wa\x13\xC5a\x1D\xE2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x13\xE6\x81a\x1E\x0EV[\x91PPa\x13yV[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x14\x8AW\x83\x81a\x14,\x85\x88a\x1F\"V[a\x146\x91\x90a\x1E'V[\x81Q\x81\x10a\x14FWa\x14Fa\x1D\xE2V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x14fWa\x14fa\x1D\xE2V[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x14\x82\x81a\x1E\x0EV[\x91PPa\x14\x18V[P`\0a\x14\x96\x82a\x1A\"V[\x90Pa\x01\0`\0\x80Q` a!,\x839\x81Q\x91R`\0a\x14\xB6\x86\x89a\x1F\"V[\x90P`\0[\x81\x81\x10\x15a\x150W`\0\x88`\x01a\x14\xD2\x84\x86a\x1F\"V[a\x14\xDC\x91\x90a\x1F\"V[\x81Q\x81\x10a\x14\xECWa\x14\xECa\x1D\xE2V[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x15\x04Wa\x15\x04a\x1E\xEAV[\x85\x87\t\x95P\x83\x80a\x15\x17Wa\x15\x17a\x1E\xEAV[\x81\x87\x08\x95PP\x80\x80a\x15(\x90a\x1E\x0EV[\x91PPa\x14\xBBV[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a!,\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x86``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x93PP`\0Q\x93P\x82a\x16\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[\x80`\x01\x85\x90\x1B\x11\x15a\x16\x1BWa\x16\x18\x84\x82a\x1F\"V[\x93P[\x80\x80a\x16)Wa\x16)a\x1E\xEAV[\x84\x85\t\x91P\x85\x82\x14\x94PPPP\x91P\x91V[`\0\x81\x80a\x16KWa\x16Ka\x1E\xEAV[a\x16U\x84\x84a\x1F\"V[\x85\x08\x94\x93PPPPV[`\0\x80`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01R`\x02\x84\x03`\x80\x82\x01R\x83`\xA0\x82\x01R` \x81`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x90Q\x92P\x90P\x80a\x16\xA9W`\0\x80\xFD[P\x92\x91PPV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x16\xF1\x92\x91\x90a\x1E\xCDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x17\x18\x92\x91\x90a\x1F5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x17:\x91\x90a\x1FaV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x17d\x90\x83\x90\x83\x90` \x01a\x1F{V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\xD5Wa\x17\xD5a\x1A\xABV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x17\xFFW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x18\x17\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x18\x8CW\x81\x81\x81Q\x81\x10a\x18FWa\x18Fa\x1D\xE2V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x18cWa\x18ca\x1D\xE2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x18\x84\x81a\x1E\x0EV[\x91PPa\x18+V[P`\0\x84`@Q` \x01a\x18\xA2\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x19@W`\0\x83\x82\x81Q\x81\x10a\x18\xDDWa\x18\xDDa\x1D\xE2V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x18\xFAWa\x18\xFAa\x1D\xE2V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x19\x1B\x92\x91\x90a\x1F\xA0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x98PP\x80\x80a\x198\x90a\x1E\x0EV[\x91PPa\x18\xC1V[P\x86\x88\x87`@Q` \x01a\x19V\x93\x92\x91\x90a\x1F\xC5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x19\x84\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x19\xA5\x8A`\xFF\x8D\x16a\x1F\"V[\x81\x10\x15a\x1A\x11W\x82\x81\x81Q\x81\x10a\x19\xBEWa\x19\xBEa\x1D\xE2V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x19\xD8\x83\x8Da\x1E'V[\x81Q\x81\x10a\x19\xE8Wa\x19\xE8a\x1D\xE2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x1A\t\x81a\x1E\x0EV[\x91PPa\x19\x98V[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a\x16\xA9W\x83\x81\x81Q\x81\x10a\x1ABWa\x1ABa\x1D\xE2V[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x1AZ\x91\x90a\x1F\xF9V[a\x1Ae\x90`\x02a \xF4V[a\x1Ao\x91\x90a\x1F\xF9V[a\x1Ay\x90\x83a\x1E'V[\x91P\x80a\x1A\x85\x81a\x1E\x0EV[\x91PPa\x1A'V[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A\xEAWa\x1A\xEAa\x1A\xABV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x1B\x04W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1B'Wa\x1B'a\x1A\xABV[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1BRW`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1BnWa\x1Bna\x1A\xABV[\x81`\x05\x1Ba\x1B}\x82\x82\x01a\x1A\xC1V[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x1B\x97W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x1B\xC5W\x825\x80\x15\x15\x81\x14a\x1B\xB6W`\0\x80\x81\xFD[\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x1B\x9DV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\xA0\x85\x87\x03\x12\x15a\x1B\xE6W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\xFEW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x1C\x12W`\0\x80\xFD[\x815` \x82\x82\x11\x15a\x1C&Wa\x1C&a\x1A\xABV[a\x1C8`\x1F\x83\x01`\x1F\x19\x16\x82\x01a\x1A\xC1V[\x82\x81R\x8A\x82\x84\x87\x01\x01\x11\x15a\x1CLW`\0\x80\xFD[\x82\x82\x86\x01\x83\x83\x017`\0\x81\x84\x01\x83\x01R\x97Pa\x1Cj\x8A\x8A\x83\x01a\x1A\xF2V[\x96PPP``\x87\x015\x91P\x80\x82\x11\x15a\x1C\x82W`\0\x80\xFD[Pa\x1C\x8F\x87\x82\x88\x01a\x1BAV[\x94\x97\x93\x96P\x93\x94`\x80\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x1C\xB2W`\0\x80\xFD[P5\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x1C\xCBW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xE3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x13\x06W`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x1D\x14W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1D,W`\0\x80\xFD[a\x1D8\x88\x83\x89\x01a\x1C\xB9V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x1DQW`\0\x80\xFD[Pa\x1D^\x87\x82\x88\x01a\x1C\xB9V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a\x1D~W`\0\x80\xFD[`\x80\x81\x12\x15a\x1D\x8CW`\0\x80\xFD[P`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1D\xB0Wa\x1D\xB0a\x1A\xABV[`@\x90\x81R\x845\x82R` \x80\x86\x015\x90\x83\x01R\x84\x81\x015\x90\x82\x01R``\x80\x85\x015\x90\x82\x01R\x94`\x80\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1E Wa\x1E a\x1D\xF8V[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x1E:Wa\x1E:a\x1D\xF8V[\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1EWW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1ErW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x13\x06W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x81Q`\0[\x81\x81\x10\x15a\x1E\xBEW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x1E\xA4V[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x1E\xE2a\x1E\xDC\x83\x86a\x1E\x9DV[\x84a\x1E\x9DV[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1F\x1DWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1E:Wa\x1E:a\x1D\xF8V[`\0a\x1FA\x82\x85a\x1E\x9DV[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a\x1Fm\x82\x84a\x1E\x9DV[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a\x1F\x87\x82\x85a\x1E\x9DV[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a\x1F\xAC\x82\x85a\x1E\x9DV[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a\x1F\xD1\x82\x86a\x1E\x9DV[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1E:Wa\x1E:a\x1D\xF8V[`\x01\x81\x81[\x80\x85\x11\x15a KW\x81`\0\x19\x04\x82\x11\x15a 1Wa 1a\x1D\xF8V[\x80\x85\x16\x15a >W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a \x15V[P\x92P\x92\x90PV[`\0\x82a bWP`\x01a\x1E:V[\x81a oWP`\0a\x1E:V[\x81`\x01\x81\x14a \x85W`\x02\x81\x14a \x8FWa \xABV[`\x01\x91PPa\x1E:V[`\xFF\x84\x11\x15a \xA0Wa \xA0a\x1D\xF8V[PP`\x01\x82\x1Ba\x1E:V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a \xCEWP\x81\x81\na\x1E:V[a \xD8\x83\x83a \x10V[\x80`\0\x19\x04\x82\x11\x15a \xECWa \xECa\x1D\xF8V[\x02\x93\x92PPPV[`\0a!\0\x83\x83a SV[\x93\x92PPPV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \x03D\xFD\x08\x04\x03\x8D\xD0\xA7X\x9E-~\xF2\x1F \xA2\x19\x8EG \x1C<\xA8y\xD3\x8F\xBCH\xC7\xCD\x94dsolcC\0\x08\x14\x003\xA2dipfsX\"\x12 \xFFd\x0F\x16\x10&\x10?\xD2\x18\xC4\x13\x8C\xBAw?vJ\x17\t\xFE}rPD-\x8D\xAAvm\x95\x01dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static HOTSHOTTEST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct HotShotTest<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for HotShotTest<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for HotShotTest<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for HotShotTest<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for HotShotTest<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(HotShotTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> HotShotTest<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                HOTSHOTTEST_ABI.clone(),
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
                HOTSHOTTEST_ABI.clone(),
                HOTSHOTTEST_BYTECODE.clone().into(),
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
        ///Calls the contract's `hotshot` (0x2adc8b76) function
        pub fn hotshot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([42, 220, 139, 118], ())
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
        ///Calls the contract's `testPublishCommitments` (0x9997afdb) function
        pub fn test_publish_commitments(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 151, 175, 219], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `NewBlocks` event
        pub fn new_blocks_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewBlocksFilter> {
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, HotShotTestEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for HotShotTest<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "NewBlocks", abi = "NewBlocks(uint256,uint256)")]
    pub struct NewBlocksFilter {
        pub first_block_number: ::ethers::core::types::U256,
        pub num_blocks: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ::ethers::core::types::Bytes);
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum HotShotTestEvents {
        NewBlocksFilter(NewBlocksFilter),
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
    impl ::ethers::contract::EthLogDecode for HotShotTestEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = NewBlocksFilter::decode_log(log) {
                return Ok(HotShotTestEvents::NewBlocksFilter(decoded));
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for HotShotTestEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NewBlocksFilter(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<NewBlocksFilter> for HotShotTestEvents {
        fn from(value: NewBlocksFilter) -> Self {
            Self::NewBlocksFilter(value)
        }
    }
    impl ::core::convert::From<LogFilter> for HotShotTestEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for HotShotTestEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogArray1Filter> for HotShotTestEvents {
        fn from(value: LogArray1Filter) -> Self {
            Self::LogArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogArray2Filter> for HotShotTestEvents {
        fn from(value: LogArray2Filter) -> Self {
            Self::LogArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogArray3Filter> for HotShotTestEvents {
        fn from(value: LogArray3Filter) -> Self {
            Self::LogArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for HotShotTestEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for HotShotTestEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for HotShotTestEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedAddressFilter> for HotShotTestEvents {
        fn from(value: LogNamedAddressFilter) -> Self {
            Self::LogNamedAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray1Filter> for HotShotTestEvents {
        fn from(value: LogNamedArray1Filter) -> Self {
            Self::LogNamedArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray2Filter> for HotShotTestEvents {
        fn from(value: LogNamedArray2Filter) -> Self {
            Self::LogNamedArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray3Filter> for HotShotTestEvents {
        fn from(value: LogNamedArray3Filter) -> Self {
            Self::LogNamedArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytesFilter> for HotShotTestEvents {
        fn from(value: LogNamedBytesFilter) -> Self {
            Self::LogNamedBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytes32Filter> for HotShotTestEvents {
        fn from(value: LogNamedBytes32Filter) -> Self {
            Self::LogNamedBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalIntFilter> for HotShotTestEvents {
        fn from(value: LogNamedDecimalIntFilter) -> Self {
            Self::LogNamedDecimalIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalUintFilter> for HotShotTestEvents {
        fn from(value: LogNamedDecimalUintFilter) -> Self {
            Self::LogNamedDecimalUintFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedIntFilter> for HotShotTestEvents {
        fn from(value: LogNamedIntFilter) -> Self {
            Self::LogNamedIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedStringFilter> for HotShotTestEvents {
        fn from(value: LogNamedStringFilter) -> Self {
            Self::LogNamedStringFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedUintFilter> for HotShotTestEvents {
        fn from(value: LogNamedUintFilter) -> Self {
            Self::LogNamedUintFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for HotShotTestEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for HotShotTestEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    impl ::core::convert::From<LogsFilter> for HotShotTestEvents {
        fn from(value: LogsFilter) -> Self {
            Self::LogsFilter(value)
        }
    }
    ///Container type for all input parameters for the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    ///Container type for all input parameters for the `hotshot` function with signature `hotshot()` and selector `0x2adc8b76`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "hotshot", abi = "hotshot()")]
    pub struct HotshotCall;
    ///Container type for all input parameters for the `setUp` function with signature `setUp()` and selector `0x0a9254e4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "targetSenders", abi = "targetSenders()")]
    pub struct TargetSendersCall;
    ///Container type for all input parameters for the `testPublishCommitments` function with signature `testPublishCommitments()` and selector `0x9997afdb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "testPublishCommitments", abi = "testPublishCommitments()")]
    pub struct TestPublishCommitmentsCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum HotShotTestCalls {
        IsTest(IsTestCall),
        ExcludeArtifacts(ExcludeArtifactsCall),
        ExcludeContracts(ExcludeContractsCall),
        ExcludeSenders(ExcludeSendersCall),
        Failed(FailedCall),
        Hotshot(HotshotCall),
        SetUp(SetUpCall),
        TargetArtifactSelectors(TargetArtifactSelectorsCall),
        TargetArtifacts(TargetArtifactsCall),
        TargetContracts(TargetContractsCall),
        TargetSelectors(TargetSelectorsCall),
        TargetSenders(TargetSendersCall),
        TestPublishCommitments(TestPublishCommitmentsCall),
    }
    impl ::ethers::core::abi::AbiDecode for HotShotTestCalls {
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
            if let Ok(decoded) = <HotshotCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Hotshot(decoded));
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
                <TestPublishCommitmentsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TestPublishCommitments(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HotShotTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsTest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeArtifacts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeContracts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeSenders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Failed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Hotshot(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetArtifactSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetArtifacts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetContracts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetSelectors(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetSenders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TestPublishCommitments(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for HotShotTestCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsTest(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::Failed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Hotshot(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifactSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestPublishCommitments(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsTestCall> for HotShotTestCalls {
        fn from(value: IsTestCall) -> Self {
            Self::IsTest(value)
        }
    }
    impl ::core::convert::From<ExcludeArtifactsCall> for HotShotTestCalls {
        fn from(value: ExcludeArtifactsCall) -> Self {
            Self::ExcludeArtifacts(value)
        }
    }
    impl ::core::convert::From<ExcludeContractsCall> for HotShotTestCalls {
        fn from(value: ExcludeContractsCall) -> Self {
            Self::ExcludeContracts(value)
        }
    }
    impl ::core::convert::From<ExcludeSendersCall> for HotShotTestCalls {
        fn from(value: ExcludeSendersCall) -> Self {
            Self::ExcludeSenders(value)
        }
    }
    impl ::core::convert::From<FailedCall> for HotShotTestCalls {
        fn from(value: FailedCall) -> Self {
            Self::Failed(value)
        }
    }
    impl ::core::convert::From<HotshotCall> for HotShotTestCalls {
        fn from(value: HotshotCall) -> Self {
            Self::Hotshot(value)
        }
    }
    impl ::core::convert::From<SetUpCall> for HotShotTestCalls {
        fn from(value: SetUpCall) -> Self {
            Self::SetUp(value)
        }
    }
    impl ::core::convert::From<TargetArtifactSelectorsCall> for HotShotTestCalls {
        fn from(value: TargetArtifactSelectorsCall) -> Self {
            Self::TargetArtifactSelectors(value)
        }
    }
    impl ::core::convert::From<TargetArtifactsCall> for HotShotTestCalls {
        fn from(value: TargetArtifactsCall) -> Self {
            Self::TargetArtifacts(value)
        }
    }
    impl ::core::convert::From<TargetContractsCall> for HotShotTestCalls {
        fn from(value: TargetContractsCall) -> Self {
            Self::TargetContracts(value)
        }
    }
    impl ::core::convert::From<TargetSelectorsCall> for HotShotTestCalls {
        fn from(value: TargetSelectorsCall) -> Self {
            Self::TargetSelectors(value)
        }
    }
    impl ::core::convert::From<TargetSendersCall> for HotShotTestCalls {
        fn from(value: TargetSendersCall) -> Self {
            Self::TargetSenders(value)
        }
    }
    impl ::core::convert::From<TestPublishCommitmentsCall> for HotShotTestCalls {
        fn from(value: TestPublishCommitmentsCall) -> Self {
            Self::TestPublishCommitments(value)
        }
    }
    ///Container type for all return fields from the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FailedReturn(pub bool);
    ///Container type for all return fields from the `hotshot` function with signature `hotshot()` and selector `0x2adc8b76`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HotshotReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
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
