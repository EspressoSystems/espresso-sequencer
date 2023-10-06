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
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0-W`\0\x80\xFD[Pa3p\x80a\0=`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\x8CW\x80c\xB5P\x8A\xA9\x11a\0fW\x80c\xB5P\x8A\xA9\x14a\x01qW\x80c\xBAAO\xA6\x14a\x01yW\x80c\xE2\x0C\x9Fq\x14a\x01\x91W\x80c\xFAv&\xD4\x14a\x01\x99W`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x01LW\x80c\x91j\x17\xC6\x14a\x01aW\x80c\x99\x97\xAF\xDB\x14a\x01iW`\0\x80\xFD[\x80c\n\x92T\xE4\x14a\0\xD4W\x80c\x1E\xD7\x83\x1C\x14a\0\xDEW\x80c*\xDC\x8Bv\x14a\0\xFCW\x80c>^<#\x14a\x01'W\x80c?r\x86\xF4\x14a\x01/W\x80cf\xD9\xA9\xA0\x14a\x017W[`\0\x80\xFD[a\0\xDCa\x01\xA6V[\0[a\0\xE6a\x01\xF1V[`@Qa\0\xF3\x91\x90a\x0F\x0BV[`@Q\x80\x91\x03\x90\xF3[`\x1BTa\x01\x0F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF3V[a\0\xE6a\x02SV[a\0\xE6a\x02\xB3V[a\x01?a\x03\x13V[`@Qa\0\xF3\x91\x90a\x0FXV[a\x01Ta\x04\x02V[`@Qa\0\xF3\x91\x90a\x10[V[a\x01?a\x04\xD2V[a\0\xDCa\x05\xB8V[a\x01Ta\nlV[a\x01\x81a\x0B<V[`@Q\x90\x15\x15\x81R` \x01a\0\xF3V[a\0\xE6a\x0CgV[`\0Ta\x01\x81\x90`\xFF\x16\x81V[`@Qa\x01\xB2\x90a\x0E\xFEV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x01\xCEW=`\0\x80>=`\0\xFD[P`\x1B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+WPPPPP\x90P\x90V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x03\xE1W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x03\xA3W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x037V[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x04E\x90a\x10\xBDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04q\x90a\x10\xBDV[\x80\x15a\x04\xBEW\x80`\x1F\x10a\x04\x93Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xBEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xA1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04&V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x05\xA0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x05bW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04\xF6V[`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R`\0\x91\x81` \x01[a\x05\xFB`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05\xD0W\x90PP\x90Pd\x868#p\x95\x81`\0\x81Q\x81\x10a\x06)Wa\x06)a\x10\xF7V[` \x02` \x01\x01Q` \x01\x81\x81RPP`\0\x81`\0\x81Q\x81\x10a\x06NWa\x06Na\x10\xF7V[` \x02` \x01\x01Q`\0\x01\x81\x81RPPd6\x8B\xD51\xFE\x81`\x01\x81Q\x81\x10a\x06wWa\x06wa\x10\xF7V[` \x02` \x01\x01Q` \x01\x81\x81RPP`\x01\x81`\x01\x81Q\x81\x10a\x06\x9CWa\x06\x9Ca\x10\xF7V[` \x90\x81\x02\x91\x90\x91\x01\x01QR`\x1BT`@Qc\x81\xBA\xD6\xF3`\xE0\x1B\x81R`\0`\x04\x82\x01\x81\x90R`$\x82\x01\x81\x90R`D\x82\x01R`\x01`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x84\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x81\xBA\xD6\xF3\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x1DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x071W=`\0\x80>=`\0\xFD[PP`@\x80Q`\0\x81R`\x02` \x82\x01R\x7F\x82\x03\xA2\x1EO\x95\xF7.P\x81\xD5\xE0\x92\x9B\x1A\x8CR\x14\x1E\x12?\x9A\x14\xE1\xE7K\x02`\xFA_R\xF1\x93P\x01\x90P`@Q\x80\x91\x03\x90\xA1`\x1BT`@Qc\n2\x1C\xFF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n2\x1C\xFF\x90a\x07\xA0\x90\x84\x90`\x04\x01a\x11\rV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xBAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xCEW=`\0\x80>=`\0\xFD[PP`\x1BT`@QcI\xCE\x89\x97`\xE0\x1B\x81R`\0`\x04\x82\x01Ra\x08g\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91PcI\xCE\x89\x97\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08C\x91\x90a\x11qV[\x82`\0\x81Q\x81\x10a\x08VWa\x08Va\x10\xF7V[` \x02` \x01\x01Q` \x01Qa\x0C\xC7V[`\x1BT`@QcI\xCE\x89\x97`\xE0\x1B\x81R`\x01`\x04\x82\x01Ra\x08\xE9\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cI\xCE\x89\x97\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xB2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD6\x91\x90a\x11qV[\x82`\x01\x81Q\x81\x10a\x08VWa\x08Va\x10\xF7V[`\x1BT`@QcI\xCE\x89\x97`\xE0\x1B\x81R`\x02`\x04\x82\x01Ra\t_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cI\xCE\x89\x97\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tX\x91\x90a\x11qV[`\0a\x0C\xC7V[`@\x80Q`\0`$\x82\x01R`\x02`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c4\xE4#\xFF`\xE0\x1B\x17\x90R\x90Qc\xF2\x8D\xCE\xB3`\xE0\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\t\xD5\x91\x90`\x04\x01a\x11\x8AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xEFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\x03W=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\n2\x1C\xFF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\n2\x1C\xFF\x91Pa\n7\x90\x84\x90`\x04\x01a\x11\rV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\nQW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\neW=`\0\x80>=`\0\xFD[PPPPPV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W\x83\x82\x90`\0R` `\0 \x01\x80Ta\n\xAF\x90a\x10\xBDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xDB\x90a\x10\xBDV[\x80\x15a\x0B(W\x80`\x1F\x10a\n\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B(V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x0BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\n\x90V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x0B\\WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0CbW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x0B\xEA\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x11\xA4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0C\x04\x91a\x11\xD5V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0CAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0CFV[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x0C^\x91\x90a\x11\xF1V[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+WPPPPP\x90P\x90V[\x80\x82\x14a\r\xEEW\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\r8\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1a\r\xEEa\r\xF2V[PPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0E\xEDW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0E\x8C\x92\x91` \x01a\x11\xA4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0E\xA6\x91a\x11\xD5V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0E\xE3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0E\xE8V[``\x91P[PPPP[`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[a!'\x80a\x12\x14\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0FLW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0F'V[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\x0F\xFCW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\x0F\xE7W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a\x0F\xBDV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\x0F\x80V[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15a\x10&W\x81\x81\x01Q\x83\x82\x01R` \x01a\x10\x0EV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x10G\x81` \x86\x01` \x86\x01a\x10\x0BV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x10\xB0W`?\x19\x88\x86\x03\x01\x84Ra\x10\x9E\x85\x83Qa\x10/V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x10\x82V[P\x92\x97\x96PPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x10\xD1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x10\xF1WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x11dW\x81Q\x80Q\x85R\x86\x81\x01Q\x87\x86\x01R\x85\x81\x01Q\x86\x86\x01R``\x90\x81\x01Q\x90\x85\x01R`\x80\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a\x11*V[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x11\x83W`\0\x80\xFD[PQ\x91\x90PV[` \x81R`\0a\x11\x9D` \x83\x01\x84a\x10/V[\x93\x92PPPV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a\x11\xC7\x81`\x04\x85\x01` \x87\x01a\x10\x0BV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa\x11\xE7\x81\x84` \x87\x01a\x10\x0BV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12\x03W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x11\x9DW`\0\x80\xFD\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa!\x07\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cI\xCE\x89\x97\x11a\0[W\x80cI\xCE\x89\x97\x14a\0\xC6W\x80cg\xA2\x1Ep\x14a\0\xE6W\x80c\xF1\xF4]\x99\x14a\x010W\x80c\xF4O\xF7\x12\x14a\x01CW`\0\x80\xFD[\x80c\x03@\x96\x1E\x14a\0\x82W\x80c\n2\x1C\xFF\x14a\0\x97W\x80c&\x83=\xCC\x14a\0\xAAW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x1B\xD9V[a\x01LV[\0[a\0\x95a\0\xA56`\x04a\x1C\xA9V[a\x03\xEBV[a\0\xB3a\x01\xF4\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB3a\0\xD46`\x04a\x1D\x1EV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\0\xF9a\0\xF46`\x04a\x1D\x1EV[a\x05cV[`@\x80Q\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R\x83\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x92\x83\x01Q\x92\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x01a\0\xBDV[a\0\x95a\x01>6`\x04a\x1D7V[a\x05\xF7V[a\0\xB3`\x01T\x81V[`\x03T\x82Q\x11\x15a\x01\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqbitmap is too long`p\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x81Q\x81\x10a\x01\xAEWa\x01\xAEa\x1D\xAFV[` \x02` \x01\x01Q\x15\x80\x15a\x01\xC3WP\x82Q\x81\x10[\x15a\x01\xDAW\x80a\x01\xD2\x81a\x1D\xDBV[\x91PPa\x01\x9CV[\x82Q\x81\x10a\x01\xFBW`@QcKe\x82-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81[\x84Q\x81\x10\x15a\x02UW\x84\x81\x81Q\x81\x10a\x02\x1AWa\x02\x1Aa\x1D\xAFV[` \x02` \x01\x01Q\x15a\x02CW`\0\x81\x81R`\x02` R`@\x90 Ta\x02@\x90\x83a\x1D\xF4V[\x91P[\x80a\x02M\x81a\x1D\xDBV[\x91PPa\x01\xFFV[P\x82\x81\x10\x15a\x02wW`@Qc<)\x0BS`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x03\x83\x81T\x81\x10a\x02\x8CWa\x02\x8Ca\x1D\xAFV[\x90`\0R` `\0 \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPP\x90P`\0\x83`\x01a\x02\xDE\x91\x90a\x1D\xF4V[\x90P[\x85Q\x81\x10\x15a\x03\xD6W\x85\x81\x81Q\x81\x10a\x02\xFCWa\x02\xFCa\x1D\xAFV[` \x02` \x01\x01Q\x15a\x03\xC4W`\0`\x03\x82\x81T\x81\x10a\x03\x1EWa\x03\x1Ea\x1D\xAFV[`\0\x91\x82R` \x80\x83 `@\x80Q`\x80\x81\x01\x82R`\x04\x90\x94\x02\x90\x91\x01\x80T\x80\x85R`\x01\x82\x01T\x85\x85\x01\x81\x90R`\x02\x83\x01T\x86\x85\x01\x81\x90R`\x03\x90\x93\x01T``\x80\x88\x01\x82\x90R\x8BQ\x96\x8C\x01Q\x95\x8C\x01Q\x90\x8C\x01Q\x97\x99P\x95\x97\x94\x96\x94\x92\x93\x91\x92\x80\x80\x80a\x03\x90\x8B\x8D\x8B\x8D\x8B\x8D\x8B\x8Da\x07\x1FV[`@\x80Q`\x80\x81\x01\x82R\x93\x84R` \x84\x01\x94\x90\x94R\x92\x82\x01\x92\x90\x92R``\x81\x01\x91\x90\x91R\x9EPPPPPPPPPPPPPP[\x80a\x03\xCE\x81a\x1D\xDBV[\x91PPa\x02\xE1V[Pa\x03\xE2\x87\x87\x83a\x08lV[PPPPPPPV[a\x01\xF4\x81\x11\x15a\x04\x11W`@Qc\xE0\x82\x84\x0B`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x01\x90V[`\x01T`\0[\x82\x81\x10\x15a\x05#W`\x01T\x84\x84\x83\x81\x81\x10a\x044Wa\x044a\x1D\xAFV[\x90P`\x80\x02\x01`\0\x015\x14a\x04\x86W\x83\x83\x82\x81\x81\x10a\x04UWa\x04Ua\x1D\xAFV[\x90P`\x80\x02\x01`\0\x015`\x01T`@Qc4\xE4#\xFF`\xE0\x1B\x81R`\x04\x01a\x01\x90\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[a\x04\xA4\x84\x84\x83\x81\x81\x10a\x04\x9BWa\x04\x9Ba\x1D\xAFV[\x90PPP`\x01\x90V[a\x04\xC7W`\x01T`@Qcx\x18g\x19`\xE0\x1B\x81R`\x04\x01a\x01\x90\x91\x81R` \x01\x90V[\x83\x83\x82\x81\x81\x10a\x04\xD9Wa\x04\xD9a\x1D\xAFV[\x90P`\x80\x02\x01` \x015`\0\x80`\x01T\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x80`\0\x82\x82Ta\x05\r\x91\x90a\x1D\xF4V[\x90\x91UPa\x05\x1C\x90P\x81a\x1D\xDBV[\x90Pa\x04\x17V[P`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x82\x03\xA2\x1EO\x95\xF7.P\x81\xD5\xE0\x92\x9B\x1A\x8CR\x14\x1E\x12?\x9A\x14\xE1\xE7K\x02`\xFA_R\xF1\x91\x01[`@Q\x80\x91\x03\x90\xA1PPPV[a\x05\x8E`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0`\x03\x83\x81T\x81\x10a\x05\xA3Wa\x05\xA3a\x1D\xAFV[`\0\x91\x82R` \x80\x83 \x95\x83R`\x02\x80\x82R`@\x93\x84\x90 T\x84Q`\x80\x81\x01\x86R`\x04\x90\x94\x02\x90\x97\x01\x80T\x84R`\x01\x81\x01T\x92\x84\x01\x92\x90\x92R\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x90\x91\x01T``\x82\x01R\x93\x91PPV[`\x03\x80T`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x90U\x84T`\x01\x81\x01\x86U\x94\x90\x92R\x85Q`\x04\x90\x94\x02\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x81\x01\x85\x90U\x86\x82\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8\\\x83\x01U\x87\x84\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8]\x84\x01U``\x80\x8A\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8^\x90\x95\x01\x94\x90\x94U\x85Q\x97\x88R\x91Q\x93\x87\x01\x93\x90\x93R\x91Q\x92\x85\x01\x92\x90\x92R\x90Q\x90\x83\x01R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x81\x90R\x90\x7F\xD7/\xE1\xACW\xD3\xE6\xD5\x1C\x92*\xE4\xD8\x11\xCCP\xAA:\xD7\x02b\x83\xAE\xA67IJ\x072RVZ\x90`\xC0\x01a\x05VV[`\0\x80\x80\x80\x8B\x15\x80\x15a\x070WP\x8A\x15[\x80\x15a\x07:WP\x89\x15[\x80\x15a\x07DWP\x88\x15[\x15a\x07\x95W\x87\x15\x80\x15a\x07UWP\x86\x15[\x80\x15a\x07_WP\x85\x15[\x80\x15a\x07iWP\x84\x15[a\x07\x85Wa\x07y\x88\x88\x88\x88a\t\x1EV[a\x07\x85Wa\x07\x85a\x1E\rV[P\x86\x92P\x85\x91P\x84\x90P\x83a\x08]V[\x87\x15\x80\x15a\x07\xA1WP\x86\x15[\x80\x15a\x07\xABWP\x85\x15[\x80\x15a\x07\xB5WP\x84\x15[\x15a\x07\xE2Wa\x07\xC6\x8C\x8C\x8C\x8Ca\t\x1EV[a\x07\xD2Wa\x07\xD2a\x1E\rV[P\x8A\x92P\x89\x91P\x88\x90P\x87a\x08]V[a\x07\xEE\x8C\x8C\x8C\x8Ca\t\x1EV[a\x07\xFAWa\x07\xFAa\x1E\rV[a\x08\x06\x88\x88\x88\x88a\t\x1EV[a\x08\x12Wa\x08\x12a\x1E\rV[`\0a\x08,\x8D\x8D\x8D\x8D`\x01`\0\x8F\x8F\x8F\x8F`\x01`\0a\t\xD3V[\x80Q` \x82\x01Q`@\x83\x01Q``\x84\x01Q`\x80\x85\x01Q`\xA0\x86\x01Q\x95\x96Pa\x08S\x95a\x0C\\V[\x94P\x94P\x94P\x94PP[\x98P\x98P\x98P\x98\x94PPPPPV[a\x08u\x82a\x0C\xA6V[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a \x8E`$\x919\x90P`\0\x84\x82`@Q` \x01a\x08\xA7\x92\x91\x90a\x1ESV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80a\x08\xC4\x83a\r5V[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91Pa\x08\xF8\x81\x87a\x08\xEB\x8Aa\x0E\x08V[a\x08\xF3a\x0E\x83V[a\x0FTV[a\t\x14W`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0\x80`\0\x80`\0a\t2\x87\x87\x89\x89a\x106V[\x90\x94P\x92Pa\tC\x89\x89\x81\x81a\x106V[\x90\x92P\x90Pa\tT\x82\x82\x8B\x8Ba\x106V[\x90\x92P\x90Pa\te\x84\x84\x84\x84a\x10\xA7V[\x90\x94P\x92Pa\t\xB5\x84\x84\x7F+\x14\x9D@\xCE\xB8\xAA\xAE\x81\xBE\x18\x99\x1B\xE0j\xC3\xB5\xB4\xC5\xE5Y\xDB\xEF\xA32g\xE6\xDC$\xA18\xE5~\x97\x13\xB0:\xF0\xFE\xD4\xCD,\xAF\xAD\xEE\xD8\xFD\xF4\xA7O\xA0\x84\xE5-\x18R\xE4\xA2\xBD\x06\x85\xC3\x15\xD2a\x10\xA7V[\x90\x94P\x92P\x83\x15\x80\x15a\t\xC6WP\x82\x15[\x99\x98PPPPPPPPPV[a\t\xDBa\x1A\x96V[\x88\x15\x80\x15a\t\xE7WP\x87\x15[\x15a\n)W\x86\x86\x86\x86\x86\x86\x86`\0[`\xA0\x89\x01\x92\x90\x92R`\x80\x88\x01\x92\x90\x92R``\x87\x01\x92\x90\x92R`@\x86\x01\x92\x90\x92R` \x85\x81\x01\x93\x90\x93R\x90\x91\x02\x01Ra\x0CLV[\x82\x15\x80\x15a\n5WP\x81\x15[\x15a\nHW\x8C\x8C\x8C\x8C\x8C\x8C\x86`\0a\t\xF6V[a\nT\x85\x85\x8B\x8Ba\x106V[\x90\x95P\x93Pa\ne\x8B\x8B\x85\x85a\x106V[``\x83\x01R`@\x82\x01Ra\n{\x87\x87\x8B\x8Ba\x106V[\x90\x97P\x95Pa\n\x8C\x8D\x8D\x85\x85a\x106V[`\xA0\x83\x01R`\x80\x82\x01\x81\x90R\x87\x14\x80\x15a\n\xA9WP`\xA0\x81\x01Q\x86\x14[\x15a\n\xEEW`@\x81\x01Q\x85\x14\x80\x15a\n\xC4WP``\x81\x01Q\x84\x14[\x15a\n\xDFWa\n\xD7\x8D\x8D\x8D\x8D\x8D\x8Da\x10\xE9V[\x86`\0a\t\xF6V[`\x01`\0\x81\x81\x80\x80\x86\x81a\t\xF6V[a\n\xFA\x89\x89\x85\x85a\x106V[\x90\x93P\x91Pa\x0B\x1A\x85\x85\x83`\x02` \x02\x01Q\x84`\x03[` \x02\x01Qa\x10\xA7V[\x90\x9DP\x9BPa\x0B4\x87\x87\x83`\x04` \x02\x01Q\x84`\x05a\x0B\x10V[\x90\x9BP\x99Pa\x0BE\x8B\x8B\x81\x81a\x106V[\x90\x99P\x97Pa\x0Be\x89\x89\x83`\x04` \x02\x01Q\x84`\x05[` \x02\x01Qa\x106V[\x90\x95P\x93Pa\x0Bv\x89\x89\x8D\x8Da\x106V[\x90\x99P\x97Pa\x0B\x87\x89\x89\x85\x85a\x106V[`\xA0\x83\x01R`\x80\x82\x01Ra\x0B\x9D\x8D\x8D\x81\x81a\x106V[\x90\x97P\x95Pa\x0B\xAE\x87\x87\x85\x85a\x106V[\x90\x97P\x95Pa\x0B\xBF\x87\x87\x8B\x8Ba\x10\xA7V[\x90\x97P\x95Pa\x0B\xD0\x85\x85`\x02a\x12XV[\x90\x93P\x91Pa\x0B\xE1\x87\x87\x85\x85a\x10\xA7V[\x90\x97P\x95Pa\x0B\xF2\x8B\x8B\x89\x89a\x106V[` \x83\x01R\x81Ra\x0C\x05\x85\x85\x89\x89a\x10\xA7V[\x90\x9BP\x99Pa\x0C\x16\x8D\x8D\x8D\x8Da\x106V[\x90\x9BP\x99Pa\x0C0\x89\x89\x83`\x02` \x02\x01Q\x84`\x03a\x0B[V[\x90\x9DP\x9BPa\x0CA\x8B\x8B\x8F\x8Fa\x10\xA7V[``\x83\x01R`@\x82\x01R[\x9C\x9BPPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x80a\x0Co\x88\x88a\x12\x8BV[\x90\x92P\x90Pa\x0C\x80\x8C\x8C\x84\x84a\x106V[\x90\x96P\x94Pa\x0C\x91\x8A\x8A\x84\x84a\x106V[\x96\x9D\x95\x9CP\x9AP\x94\x98P\x92\x96PPPPPPPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a \xB2\x839\x81Q\x91R\x91\x82`\x03\x81\x80\x85\x80\t\x85\t\x08\x83\x82\x83\t\x14\x81\x15\x83\x15\x17\x19\x84\x83\x10\x85\x85\x10\x16\x16\x16\x93PPP\x81a\r0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[PPPV[`\0\x80`\0a\rC\x84a\x13\x16V[\x90P`\0\x80Q` a \xB2\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\rlWa\rla\x1EpV[\x84\x82\t\x90P\x82\x80a\r\x7FWa\r\x7Fa\x1EpV[\x82\x82\x08\x90P`\0\x80a\r\x90\x83a\x15IV[\x92P\x90P[\x80a\r\xF9W\x84\x80a\r\xA8Wa\r\xA8a\x1EpV[`\x01\x87\x08\x95P\x84\x80a\r\xBCWa\r\xBCa\x1EpV[\x86\x87\t\x92P\x84\x80a\r\xCFWa\r\xCFa\x1EpV[\x86\x84\t\x92P\x84\x80a\r\xE2Wa\r\xE2a\x1EpV[\x84\x84\x08\x92Pa\r\xF0\x83a\x15IV[\x92P\x90Pa\r\x95V[P\x93\x97\x93\x96P\x92\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x0E0WP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a \xB2\x839\x81Q\x91R\x84` \x01Qa\x0Ec\x91\x90a\x1E\x86V[a\x0E{\x90`\0\x80Q` a \xB2\x839\x81Q\x91Ra\x1E\xA8V[\x90R\x92\x91PPV[a\x0E\xAE`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R\x86Q`@\x82\x01R` \x87\x01Q``\x82\x01R`@\x87\x01Q`\x80\x82\x01R``\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R` \x85\x01Qa\x01 \x82\x01R`@\x85\x01Qa\x01@\x82\x01R``\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x10*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[P\x15\x15\x95\x94PPPPPV[`\0\x80a\x10t`\0\x80Q` a \xB2\x839\x81Q\x91R\x85\x88\t`\0\x80Q` a \xB2\x839\x81Q\x91R\x85\x88\t`\0\x80Q` a \xB2\x839\x81Q\x91Ra\x16DV[`\0\x80Q` a \xB2\x839\x81Q\x91R\x80\x86\x88\t`\0\x80Q` a \xB2\x839\x81Q\x91R\x86\x8A\t\x08\x91P\x91P\x94P\x94\x92PPPV[`\0\x80a\x10\xC3\x86\x85`\0\x80Q` a \xB2\x839\x81Q\x91Ra\x16DV[a\x10\xDC\x86\x85`\0\x80Q` a \xB2\x839\x81Q\x91Ra\x16DV[\x91P\x91P\x94P\x94\x92PPPV[`\0\x80`\0\x80`\0\x80a\x10\xFE\x8C\x8C`\x03a\x12XV[\x90\x96P\x94Pa\x11\x0F\x86\x86\x8E\x8Ea\x106V[\x90\x96P\x94Pa\x11 \x8A\x8A\x8A\x8Aa\x106V[\x90\x98P\x96Pa\x111\x8C\x8C\x8C\x8Ca\x106V[\x90\x94P\x92Pa\x11B\x84\x84\x8A\x8Aa\x106V[\x90\x94P\x92Pa\x11S\x86\x86\x81\x81a\x106V[\x90\x9CP\x9APa\x11d\x84\x84`\x08a\x12XV[\x90\x92P\x90Pa\x11u\x8C\x8C\x84\x84a\x10\xA7V[\x90\x9CP\x9APa\x11\x86\x88\x88\x81\x81a\x106V[\x90\x92P\x90Pa\x11\x97\x84\x84`\x04a\x12XV[\x90\x94P\x92Pa\x11\xA8\x84\x84\x8E\x8Ea\x10\xA7V[\x90\x94P\x92Pa\x11\xB9\x84\x84\x88\x88a\x106V[\x90\x94P\x92Pa\x11\xCA\x8A\x8A`\x08a\x12XV[\x90\x96P\x94Pa\x11\xDB\x86\x86\x8C\x8Ca\x106V[\x90\x96P\x94Pa\x11\xEC\x86\x86\x84\x84a\x106V[\x90\x96P\x94Pa\x11\xFD\x84\x84\x88\x88a\x10\xA7V[\x90\x94P\x92Pa\x12\x0E\x8C\x8C`\x02a\x12XV[\x90\x96P\x94Pa\x12\x1F\x86\x86\x8A\x8Aa\x106V[\x90\x96P\x94Pa\x120\x88\x88\x84\x84a\x106V[\x90\x92P\x90Pa\x12A\x82\x82`\x08a\x12XV[\x80\x92P\x81\x93PPP\x96P\x96P\x96P\x96P\x96P\x96\x90PV[`\0\x80`\0\x80Q` a \xB2\x839\x81Q\x91R\x83\x86\t`\0\x80Q` a \xB2\x839\x81Q\x91R\x84\x86\t\x91P\x91P\x93P\x93\x91PPV[`\0\x80\x80a\x12\xCC`\0\x80Q` a \xB2\x839\x81Q\x91R\x80\x87\x88\t`\0\x80Q` a \xB2\x839\x81Q\x91R\x87\x88\t\x08`\0\x80Q` a \xB2\x839\x81Q\x91Ra\x16hV[\x90P`\0\x80Q` a \xB2\x839\x81Q\x91R\x81\x86\t`\0\x80Q` a \xB2\x839\x81Q\x91R\x82\x86\ta\x13\n\x90`\0\x80Q` a \xB2\x839\x81Q\x91Ra\x1E\xA8V[\x92P\x92PP\x92P\x92\x90PV[`\0\x80a\x13\"\x83a\x16\xB9V[\x80Q\x90\x91P`0\x81\x14a\x137Wa\x137a\x1E\rV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13RWa\x13Ra\x1A\xB4V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x13|W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x13\xF7W\x83`\x01a\x13\x97\x83\x86a\x1E\xA8V[a\x13\xA1\x91\x90a\x1E\xA8V[\x81Q\x81\x10a\x13\xB1Wa\x13\xB1a\x1D\xAFV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x13\xCEWa\x13\xCEa\x1D\xAFV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x13\xEF\x81a\x1D\xDBV[\x91PPa\x13\x82V[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x14\x93W\x83\x81a\x145\x85\x88a\x1E\xA8V[a\x14?\x91\x90a\x1D\xF4V[\x81Q\x81\x10a\x14OWa\x14Oa\x1D\xAFV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x14oWa\x14oa\x1D\xAFV[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x14\x8B\x81a\x1D\xDBV[\x91PPa\x14!V[P`\0a\x14\x9F\x82a\x1A+V[\x90Pa\x01\0`\0\x80Q` a \xB2\x839\x81Q\x91R`\0a\x14\xBF\x86\x89a\x1E\xA8V[\x90P`\0[\x81\x81\x10\x15a\x159W`\0\x88`\x01a\x14\xDB\x84\x86a\x1E\xA8V[a\x14\xE5\x91\x90a\x1E\xA8V[\x81Q\x81\x10a\x14\xF5Wa\x14\xF5a\x1D\xAFV[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x15\rWa\x15\ra\x1EpV[\x85\x87\t\x95P\x83\x80a\x15 Wa\x15 a\x1EpV[\x81\x87\x08\x95PP\x80\x80a\x151\x90a\x1D\xDBV[\x91PPa\x14\xC4V[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a \xB2\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x86``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x93PP`\0Q\x93P\x82a\x16\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[\x80`\x01\x85\x90\x1B\x11\x15a\x16$Wa\x16!\x84\x82a\x1E\xA8V[\x93P[\x80\x80a\x162Wa\x162a\x1EpV[\x84\x85\t\x91P\x85\x82\x14\x94PPPP\x91P\x91V[`\0\x81\x80a\x16TWa\x16Ta\x1EpV[a\x16^\x84\x84a\x1E\xA8V[\x85\x08\x94\x93PPPPV[`\0\x80`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01R`\x02\x84\x03`\x80\x82\x01R\x83`\xA0\x82\x01R` \x81`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x90Q\x92P\x90P\x80a\x16\xB2W`\0\x80\xFD[P\x92\x91PPV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x16\xFA\x92\x91\x90a\x1ESV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x17!\x92\x91\x90a\x1E\xBBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x17C\x91\x90a\x1E\xE7V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x17m\x90\x83\x90\x83\x90` \x01a\x1F\x01V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\xDEWa\x17\xDEa\x1A\xB4V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x18\x08W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x18 \x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x18\x95W\x81\x81\x81Q\x81\x10a\x18OWa\x18Oa\x1D\xAFV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x18lWa\x18la\x1D\xAFV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x18\x8D\x81a\x1D\xDBV[\x91PPa\x184V[P`\0\x84`@Q` \x01a\x18\xAB\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x19IW`\0\x83\x82\x81Q\x81\x10a\x18\xE6Wa\x18\xE6a\x1D\xAFV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x19\x03Wa\x19\x03a\x1D\xAFV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x19$\x92\x91\x90a\x1F&V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x98PP\x80\x80a\x19A\x90a\x1D\xDBV[\x91PPa\x18\xCAV[P\x86\x88\x87`@Q` \x01a\x19_\x93\x92\x91\x90a\x1FKV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x19\x8D\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x19\xAE\x8A`\xFF\x8D\x16a\x1E\xA8V[\x81\x10\x15a\x1A\x1AW\x82\x81\x81Q\x81\x10a\x19\xC7Wa\x19\xC7a\x1D\xAFV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x19\xE1\x83\x8Da\x1D\xF4V[\x81Q\x81\x10a\x19\xF1Wa\x19\xF1a\x1D\xAFV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x1A\x12\x81a\x1D\xDBV[\x91PPa\x19\xA1V[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a\x16\xB2W\x83\x81\x81Q\x81\x10a\x1AKWa\x1AKa\x1D\xAFV[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x1Ac\x91\x90a\x1F\x7FV[a\x1An\x90`\x02a zV[a\x1Ax\x91\x90a\x1F\x7FV[a\x1A\x82\x90\x83a\x1D\xF4V[\x91P\x80a\x1A\x8E\x81a\x1D\xDBV[\x91PPa\x1A0V[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A\xF3Wa\x1A\xF3a\x1A\xB4V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x1B\rW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1B0Wa\x1B0a\x1A\xB4V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1B[W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1BwWa\x1Bwa\x1A\xB4V[\x81`\x05\x1Ba\x1B\x86\x82\x82\x01a\x1A\xCAV[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x1B\xA0W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x1B\xCEW\x825\x80\x15\x15\x81\x14a\x1B\xBFW`\0\x80\x81\xFD[\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x1B\xA6V[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\xA0\x85\x87\x03\x12\x15a\x1B\xEFW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1C\x07W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x1C\x1BW`\0\x80\xFD[\x815` \x82\x82\x11\x15a\x1C/Wa\x1C/a\x1A\xB4V[a\x1CA`\x1F\x83\x01`\x1F\x19\x16\x82\x01a\x1A\xCAV[\x82\x81R\x8A\x82\x84\x87\x01\x01\x11\x15a\x1CUW`\0\x80\xFD[\x82\x82\x86\x01\x83\x83\x017`\0\x81\x84\x01\x83\x01R\x97Pa\x1Cs\x8A\x8A\x83\x01a\x1A\xFBV[\x96PPP``\x87\x015\x91P\x80\x82\x11\x15a\x1C\x8BW`\0\x80\xFD[Pa\x1C\x98\x87\x82\x88\x01a\x1BJV[\x94\x97\x93\x96P\x93\x94`\x80\x015\x93PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x1C\xBCW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1C\xD4W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x1C\xE8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1C\xF7W`\0\x80\xFD[\x86` \x82`\x07\x1B\x85\x01\x01\x11\x15a\x1D\x0CW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x1D0W`\0\x80\xFD[P5\x91\x90PV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a\x1DKW`\0\x80\xFD[`\x80\x81\x12\x15a\x1DYW`\0\x80\xFD[P`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1D}Wa\x1D}a\x1A\xB4V[`@\x90\x81R\x845\x82R` \x80\x86\x015\x90\x83\x01R\x84\x81\x015\x90\x82\x01R``\x80\x85\x015\x90\x82\x01R\x94`\x80\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1D\xEDWa\x1D\xEDa\x1D\xC5V[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x1E\x07Wa\x1E\x07a\x1D\xC5V[\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x81Q`\0[\x81\x81\x10\x15a\x1EDW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x1E*V[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x1Eha\x1Eb\x83\x86a\x1E#V[\x84a\x1E#V[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1E\xA3WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1E\x07Wa\x1E\x07a\x1D\xC5V[`\0a\x1E\xC7\x82\x85a\x1E#V[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a\x1E\xF3\x82\x84a\x1E#V[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a\x1F\r\x82\x85a\x1E#V[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a\x1F2\x82\x85a\x1E#V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a\x1FW\x82\x86a\x1E#V[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1E\x07Wa\x1E\x07a\x1D\xC5V[`\x01\x81\x81[\x80\x85\x11\x15a\x1F\xD1W\x81`\0\x19\x04\x82\x11\x15a\x1F\xB7Wa\x1F\xB7a\x1D\xC5V[\x80\x85\x16\x15a\x1F\xC4W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x1F\x9BV[P\x92P\x92\x90PV[`\0\x82a\x1F\xE8WP`\x01a\x1E\x07V[\x81a\x1F\xF5WP`\0a\x1E\x07V[\x81`\x01\x81\x14a \x0BW`\x02\x81\x14a \x15Wa 1V[`\x01\x91PPa\x1E\x07V[`\xFF\x84\x11\x15a &Wa &a\x1D\xC5V[PP`\x01\x82\x1Ba\x1E\x07V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a TWP\x81\x81\na\x1E\x07V[a ^\x83\x83a\x1F\x96V[\x80`\0\x19\x04\x82\x11\x15a rWa ra\x1D\xC5V[\x02\x93\x92PPPV[`\0a \x86\x83\x83a\x1F\xD9V[\x93\x92PPPV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 Q\xB3\xED\xD2\xB17\x80&<\x87e\xE0Pbw\xCEZq4\xBDeQf9\xCE4\xE6\x9C'm\xF3\xBCdsolcC\0\x08\x14\x003\xA2dipfsX\"\x12 \xB2\x88\x98\xEC\x9BF\xB0b9)Z4\x8E\xBA\xEC~\xBE\xB8\xEC\0\xDA\xDD\xEAo\xB0\xFF\xD8\x9B5u\xF0\x8DdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static HOTSHOTTEST_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\x8CW\x80c\xB5P\x8A\xA9\x11a\0fW\x80c\xB5P\x8A\xA9\x14a\x01qW\x80c\xBAAO\xA6\x14a\x01yW\x80c\xE2\x0C\x9Fq\x14a\x01\x91W\x80c\xFAv&\xD4\x14a\x01\x99W`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x01LW\x80c\x91j\x17\xC6\x14a\x01aW\x80c\x99\x97\xAF\xDB\x14a\x01iW`\0\x80\xFD[\x80c\n\x92T\xE4\x14a\0\xD4W\x80c\x1E\xD7\x83\x1C\x14a\0\xDEW\x80c*\xDC\x8Bv\x14a\0\xFCW\x80c>^<#\x14a\x01'W\x80c?r\x86\xF4\x14a\x01/W\x80cf\xD9\xA9\xA0\x14a\x017W[`\0\x80\xFD[a\0\xDCa\x01\xA6V[\0[a\0\xE6a\x01\xF1V[`@Qa\0\xF3\x91\x90a\x0F\x0BV[`@Q\x80\x91\x03\x90\xF3[`\x1BTa\x01\x0F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF3V[a\0\xE6a\x02SV[a\0\xE6a\x02\xB3V[a\x01?a\x03\x13V[`@Qa\0\xF3\x91\x90a\x0FXV[a\x01Ta\x04\x02V[`@Qa\0\xF3\x91\x90a\x10[V[a\x01?a\x04\xD2V[a\0\xDCa\x05\xB8V[a\x01Ta\nlV[a\x01\x81a\x0B<V[`@Q\x90\x15\x15\x81R` \x01a\0\xF3V[a\0\xE6a\x0CgV[`\0Ta\x01\x81\x90`\xFF\x16\x81V[`@Qa\x01\xB2\x90a\x0E\xFEV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x01\xCEW=`\0\x80>=`\0\xFD[P`\x1B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+WPPPPP\x90P\x90V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x03\xE1W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x03\xA3W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x037V[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x04E\x90a\x10\xBDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04q\x90a\x10\xBDV[\x80\x15a\x04\xBEW\x80`\x1F\x10a\x04\x93Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xBEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xA1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04&V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x05\xA0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x05bW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04\xF6V[`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R`\0\x91\x81` \x01[a\x05\xFB`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05\xD0W\x90PP\x90Pd\x868#p\x95\x81`\0\x81Q\x81\x10a\x06)Wa\x06)a\x10\xF7V[` \x02` \x01\x01Q` \x01\x81\x81RPP`\0\x81`\0\x81Q\x81\x10a\x06NWa\x06Na\x10\xF7V[` \x02` \x01\x01Q`\0\x01\x81\x81RPPd6\x8B\xD51\xFE\x81`\x01\x81Q\x81\x10a\x06wWa\x06wa\x10\xF7V[` \x02` \x01\x01Q` \x01\x81\x81RPP`\x01\x81`\x01\x81Q\x81\x10a\x06\x9CWa\x06\x9Ca\x10\xF7V[` \x90\x81\x02\x91\x90\x91\x01\x01QR`\x1BT`@Qc\x81\xBA\xD6\xF3`\xE0\x1B\x81R`\0`\x04\x82\x01\x81\x90R`$\x82\x01\x81\x90R`D\x82\x01R`\x01`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x84\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x81\xBA\xD6\xF3\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x1DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x071W=`\0\x80>=`\0\xFD[PP`@\x80Q`\0\x81R`\x02` \x82\x01R\x7F\x82\x03\xA2\x1EO\x95\xF7.P\x81\xD5\xE0\x92\x9B\x1A\x8CR\x14\x1E\x12?\x9A\x14\xE1\xE7K\x02`\xFA_R\xF1\x93P\x01\x90P`@Q\x80\x91\x03\x90\xA1`\x1BT`@Qc\n2\x1C\xFF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\n2\x1C\xFF\x90a\x07\xA0\x90\x84\x90`\x04\x01a\x11\rV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xBAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xCEW=`\0\x80>=`\0\xFD[PP`\x1BT`@QcI\xCE\x89\x97`\xE0\x1B\x81R`\0`\x04\x82\x01Ra\x08g\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91PcI\xCE\x89\x97\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08C\x91\x90a\x11qV[\x82`\0\x81Q\x81\x10a\x08VWa\x08Va\x10\xF7V[` \x02` \x01\x01Q` \x01Qa\x0C\xC7V[`\x1BT`@QcI\xCE\x89\x97`\xE0\x1B\x81R`\x01`\x04\x82\x01Ra\x08\xE9\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cI\xCE\x89\x97\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xB2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD6\x91\x90a\x11qV[\x82`\x01\x81Q\x81\x10a\x08VWa\x08Va\x10\xF7V[`\x1BT`@QcI\xCE\x89\x97`\xE0\x1B\x81R`\x02`\x04\x82\x01Ra\t_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cI\xCE\x89\x97\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tX\x91\x90a\x11qV[`\0a\x0C\xC7V[`@\x80Q`\0`$\x82\x01R`\x02`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c4\xE4#\xFF`\xE0\x1B\x17\x90R\x90Qc\xF2\x8D\xCE\xB3`\xE0\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\t\xD5\x91\x90`\x04\x01a\x11\x8AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xEFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\x03W=`\0\x80>=`\0\xFD[PP`\x1BT`@Qc\n2\x1C\xFF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\n2\x1C\xFF\x91Pa\n7\x90\x84\x90`\x04\x01a\x11\rV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\nQW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\neW=`\0\x80>=`\0\xFD[PPPPPV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03\xF9W\x83\x82\x90`\0R` `\0 \x01\x80Ta\n\xAF\x90a\x10\xBDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xDB\x90a\x10\xBDV[\x80\x15a\x0B(W\x80`\x1F\x10a\n\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B(V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x0BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\n\x90V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x0B\\WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0CbW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x0B\xEA\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x11\xA4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0C\x04\x91a\x11\xD5V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0CAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0CFV[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x0C^\x91\x90a\x11\xF1V[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02IW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02+WPPPPP\x90P\x90V[\x80\x82\x14a\r\xEEW\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\r8\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1a\r\xEEa\r\xF2V[PPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0E\xEDW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0E\x8C\x92\x91` \x01a\x11\xA4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0E\xA6\x91a\x11\xD5V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0E\xE3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0E\xE8V[``\x91P[PPPP[`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[a!'\x80a\x12\x14\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0FLW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0F'V[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\x0F\xFCW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\x0F\xE7W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a\x0F\xBDV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\x0F\x80V[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15a\x10&W\x81\x81\x01Q\x83\x82\x01R` \x01a\x10\x0EV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x10G\x81` \x86\x01` \x86\x01a\x10\x0BV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x10\xB0W`?\x19\x88\x86\x03\x01\x84Ra\x10\x9E\x85\x83Qa\x10/V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x10\x82V[P\x92\x97\x96PPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x10\xD1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x10\xF1WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x11dW\x81Q\x80Q\x85R\x86\x81\x01Q\x87\x86\x01R\x85\x81\x01Q\x86\x86\x01R``\x90\x81\x01Q\x90\x85\x01R`\x80\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a\x11*V[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x11\x83W`\0\x80\xFD[PQ\x91\x90PV[` \x81R`\0a\x11\x9D` \x83\x01\x84a\x10/V[\x93\x92PPPV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a\x11\xC7\x81`\x04\x85\x01` \x87\x01a\x10\x0BV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa\x11\xE7\x81\x84` \x87\x01a\x10\x0BV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12\x03W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x11\x9DW`\0\x80\xFD\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa!\x07\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cI\xCE\x89\x97\x11a\0[W\x80cI\xCE\x89\x97\x14a\0\xC6W\x80cg\xA2\x1Ep\x14a\0\xE6W\x80c\xF1\xF4]\x99\x14a\x010W\x80c\xF4O\xF7\x12\x14a\x01CW`\0\x80\xFD[\x80c\x03@\x96\x1E\x14a\0\x82W\x80c\n2\x1C\xFF\x14a\0\x97W\x80c&\x83=\xCC\x14a\0\xAAW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x1B\xD9V[a\x01LV[\0[a\0\x95a\0\xA56`\x04a\x1C\xA9V[a\x03\xEBV[a\0\xB3a\x01\xF4\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB3a\0\xD46`\x04a\x1D\x1EV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\0\xF9a\0\xF46`\x04a\x1D\x1EV[a\x05cV[`@\x80Q\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R\x83\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x92\x83\x01Q\x92\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x01a\0\xBDV[a\0\x95a\x01>6`\x04a\x1D7V[a\x05\xF7V[a\0\xB3`\x01T\x81V[`\x03T\x82Q\x11\x15a\x01\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqbitmap is too long`p\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x81Q\x81\x10a\x01\xAEWa\x01\xAEa\x1D\xAFV[` \x02` \x01\x01Q\x15\x80\x15a\x01\xC3WP\x82Q\x81\x10[\x15a\x01\xDAW\x80a\x01\xD2\x81a\x1D\xDBV[\x91PPa\x01\x9CV[\x82Q\x81\x10a\x01\xFBW`@QcKe\x82-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81[\x84Q\x81\x10\x15a\x02UW\x84\x81\x81Q\x81\x10a\x02\x1AWa\x02\x1Aa\x1D\xAFV[` \x02` \x01\x01Q\x15a\x02CW`\0\x81\x81R`\x02` R`@\x90 Ta\x02@\x90\x83a\x1D\xF4V[\x91P[\x80a\x02M\x81a\x1D\xDBV[\x91PPa\x01\xFFV[P\x82\x81\x10\x15a\x02wW`@Qc<)\x0BS`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x03\x83\x81T\x81\x10a\x02\x8CWa\x02\x8Ca\x1D\xAFV[\x90`\0R` `\0 \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPP\x90P`\0\x83`\x01a\x02\xDE\x91\x90a\x1D\xF4V[\x90P[\x85Q\x81\x10\x15a\x03\xD6W\x85\x81\x81Q\x81\x10a\x02\xFCWa\x02\xFCa\x1D\xAFV[` \x02` \x01\x01Q\x15a\x03\xC4W`\0`\x03\x82\x81T\x81\x10a\x03\x1EWa\x03\x1Ea\x1D\xAFV[`\0\x91\x82R` \x80\x83 `@\x80Q`\x80\x81\x01\x82R`\x04\x90\x94\x02\x90\x91\x01\x80T\x80\x85R`\x01\x82\x01T\x85\x85\x01\x81\x90R`\x02\x83\x01T\x86\x85\x01\x81\x90R`\x03\x90\x93\x01T``\x80\x88\x01\x82\x90R\x8BQ\x96\x8C\x01Q\x95\x8C\x01Q\x90\x8C\x01Q\x97\x99P\x95\x97\x94\x96\x94\x92\x93\x91\x92\x80\x80\x80a\x03\x90\x8B\x8D\x8B\x8D\x8B\x8D\x8B\x8Da\x07\x1FV[`@\x80Q`\x80\x81\x01\x82R\x93\x84R` \x84\x01\x94\x90\x94R\x92\x82\x01\x92\x90\x92R``\x81\x01\x91\x90\x91R\x9EPPPPPPPPPPPPPP[\x80a\x03\xCE\x81a\x1D\xDBV[\x91PPa\x02\xE1V[Pa\x03\xE2\x87\x87\x83a\x08lV[PPPPPPPV[a\x01\xF4\x81\x11\x15a\x04\x11W`@Qc\xE0\x82\x84\x0B`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x01\x90V[`\x01T`\0[\x82\x81\x10\x15a\x05#W`\x01T\x84\x84\x83\x81\x81\x10a\x044Wa\x044a\x1D\xAFV[\x90P`\x80\x02\x01`\0\x015\x14a\x04\x86W\x83\x83\x82\x81\x81\x10a\x04UWa\x04Ua\x1D\xAFV[\x90P`\x80\x02\x01`\0\x015`\x01T`@Qc4\xE4#\xFF`\xE0\x1B\x81R`\x04\x01a\x01\x90\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[a\x04\xA4\x84\x84\x83\x81\x81\x10a\x04\x9BWa\x04\x9Ba\x1D\xAFV[\x90PPP`\x01\x90V[a\x04\xC7W`\x01T`@Qcx\x18g\x19`\xE0\x1B\x81R`\x04\x01a\x01\x90\x91\x81R` \x01\x90V[\x83\x83\x82\x81\x81\x10a\x04\xD9Wa\x04\xD9a\x1D\xAFV[\x90P`\x80\x02\x01` \x015`\0\x80`\x01T\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x80`\0\x82\x82Ta\x05\r\x91\x90a\x1D\xF4V[\x90\x91UPa\x05\x1C\x90P\x81a\x1D\xDBV[\x90Pa\x04\x17V[P`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x82\x03\xA2\x1EO\x95\xF7.P\x81\xD5\xE0\x92\x9B\x1A\x8CR\x14\x1E\x12?\x9A\x14\xE1\xE7K\x02`\xFA_R\xF1\x91\x01[`@Q\x80\x91\x03\x90\xA1PPPV[a\x05\x8E`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0`\x03\x83\x81T\x81\x10a\x05\xA3Wa\x05\xA3a\x1D\xAFV[`\0\x91\x82R` \x80\x83 \x95\x83R`\x02\x80\x82R`@\x93\x84\x90 T\x84Q`\x80\x81\x01\x86R`\x04\x90\x94\x02\x90\x97\x01\x80T\x84R`\x01\x81\x01T\x92\x84\x01\x92\x90\x92R\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x90\x91\x01T``\x82\x01R\x93\x91PPV[`\x03\x80T`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x90U\x84T`\x01\x81\x01\x86U\x94\x90\x92R\x85Q`\x04\x90\x94\x02\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x81\x01\x85\x90U\x86\x82\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8\\\x83\x01U\x87\x84\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8]\x84\x01U``\x80\x8A\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8^\x90\x95\x01\x94\x90\x94U\x85Q\x97\x88R\x91Q\x93\x87\x01\x93\x90\x93R\x91Q\x92\x85\x01\x92\x90\x92R\x90Q\x90\x83\x01R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x81\x90R\x90\x7F\xD7/\xE1\xACW\xD3\xE6\xD5\x1C\x92*\xE4\xD8\x11\xCCP\xAA:\xD7\x02b\x83\xAE\xA67IJ\x072RVZ\x90`\xC0\x01a\x05VV[`\0\x80\x80\x80\x8B\x15\x80\x15a\x070WP\x8A\x15[\x80\x15a\x07:WP\x89\x15[\x80\x15a\x07DWP\x88\x15[\x15a\x07\x95W\x87\x15\x80\x15a\x07UWP\x86\x15[\x80\x15a\x07_WP\x85\x15[\x80\x15a\x07iWP\x84\x15[a\x07\x85Wa\x07y\x88\x88\x88\x88a\t\x1EV[a\x07\x85Wa\x07\x85a\x1E\rV[P\x86\x92P\x85\x91P\x84\x90P\x83a\x08]V[\x87\x15\x80\x15a\x07\xA1WP\x86\x15[\x80\x15a\x07\xABWP\x85\x15[\x80\x15a\x07\xB5WP\x84\x15[\x15a\x07\xE2Wa\x07\xC6\x8C\x8C\x8C\x8Ca\t\x1EV[a\x07\xD2Wa\x07\xD2a\x1E\rV[P\x8A\x92P\x89\x91P\x88\x90P\x87a\x08]V[a\x07\xEE\x8C\x8C\x8C\x8Ca\t\x1EV[a\x07\xFAWa\x07\xFAa\x1E\rV[a\x08\x06\x88\x88\x88\x88a\t\x1EV[a\x08\x12Wa\x08\x12a\x1E\rV[`\0a\x08,\x8D\x8D\x8D\x8D`\x01`\0\x8F\x8F\x8F\x8F`\x01`\0a\t\xD3V[\x80Q` \x82\x01Q`@\x83\x01Q``\x84\x01Q`\x80\x85\x01Q`\xA0\x86\x01Q\x95\x96Pa\x08S\x95a\x0C\\V[\x94P\x94P\x94P\x94PP[\x98P\x98P\x98P\x98\x94PPPPPV[a\x08u\x82a\x0C\xA6V[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a \x8E`$\x919\x90P`\0\x84\x82`@Q` \x01a\x08\xA7\x92\x91\x90a\x1ESV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80a\x08\xC4\x83a\r5V[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91Pa\x08\xF8\x81\x87a\x08\xEB\x8Aa\x0E\x08V[a\x08\xF3a\x0E\x83V[a\x0FTV[a\t\x14W`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0\x80`\0\x80`\0a\t2\x87\x87\x89\x89a\x106V[\x90\x94P\x92Pa\tC\x89\x89\x81\x81a\x106V[\x90\x92P\x90Pa\tT\x82\x82\x8B\x8Ba\x106V[\x90\x92P\x90Pa\te\x84\x84\x84\x84a\x10\xA7V[\x90\x94P\x92Pa\t\xB5\x84\x84\x7F+\x14\x9D@\xCE\xB8\xAA\xAE\x81\xBE\x18\x99\x1B\xE0j\xC3\xB5\xB4\xC5\xE5Y\xDB\xEF\xA32g\xE6\xDC$\xA18\xE5~\x97\x13\xB0:\xF0\xFE\xD4\xCD,\xAF\xAD\xEE\xD8\xFD\xF4\xA7O\xA0\x84\xE5-\x18R\xE4\xA2\xBD\x06\x85\xC3\x15\xD2a\x10\xA7V[\x90\x94P\x92P\x83\x15\x80\x15a\t\xC6WP\x82\x15[\x99\x98PPPPPPPPPV[a\t\xDBa\x1A\x96V[\x88\x15\x80\x15a\t\xE7WP\x87\x15[\x15a\n)W\x86\x86\x86\x86\x86\x86\x86`\0[`\xA0\x89\x01\x92\x90\x92R`\x80\x88\x01\x92\x90\x92R``\x87\x01\x92\x90\x92R`@\x86\x01\x92\x90\x92R` \x85\x81\x01\x93\x90\x93R\x90\x91\x02\x01Ra\x0CLV[\x82\x15\x80\x15a\n5WP\x81\x15[\x15a\nHW\x8C\x8C\x8C\x8C\x8C\x8C\x86`\0a\t\xF6V[a\nT\x85\x85\x8B\x8Ba\x106V[\x90\x95P\x93Pa\ne\x8B\x8B\x85\x85a\x106V[``\x83\x01R`@\x82\x01Ra\n{\x87\x87\x8B\x8Ba\x106V[\x90\x97P\x95Pa\n\x8C\x8D\x8D\x85\x85a\x106V[`\xA0\x83\x01R`\x80\x82\x01\x81\x90R\x87\x14\x80\x15a\n\xA9WP`\xA0\x81\x01Q\x86\x14[\x15a\n\xEEW`@\x81\x01Q\x85\x14\x80\x15a\n\xC4WP``\x81\x01Q\x84\x14[\x15a\n\xDFWa\n\xD7\x8D\x8D\x8D\x8D\x8D\x8Da\x10\xE9V[\x86`\0a\t\xF6V[`\x01`\0\x81\x81\x80\x80\x86\x81a\t\xF6V[a\n\xFA\x89\x89\x85\x85a\x106V[\x90\x93P\x91Pa\x0B\x1A\x85\x85\x83`\x02` \x02\x01Q\x84`\x03[` \x02\x01Qa\x10\xA7V[\x90\x9DP\x9BPa\x0B4\x87\x87\x83`\x04` \x02\x01Q\x84`\x05a\x0B\x10V[\x90\x9BP\x99Pa\x0BE\x8B\x8B\x81\x81a\x106V[\x90\x99P\x97Pa\x0Be\x89\x89\x83`\x04` \x02\x01Q\x84`\x05[` \x02\x01Qa\x106V[\x90\x95P\x93Pa\x0Bv\x89\x89\x8D\x8Da\x106V[\x90\x99P\x97Pa\x0B\x87\x89\x89\x85\x85a\x106V[`\xA0\x83\x01R`\x80\x82\x01Ra\x0B\x9D\x8D\x8D\x81\x81a\x106V[\x90\x97P\x95Pa\x0B\xAE\x87\x87\x85\x85a\x106V[\x90\x97P\x95Pa\x0B\xBF\x87\x87\x8B\x8Ba\x10\xA7V[\x90\x97P\x95Pa\x0B\xD0\x85\x85`\x02a\x12XV[\x90\x93P\x91Pa\x0B\xE1\x87\x87\x85\x85a\x10\xA7V[\x90\x97P\x95Pa\x0B\xF2\x8B\x8B\x89\x89a\x106V[` \x83\x01R\x81Ra\x0C\x05\x85\x85\x89\x89a\x10\xA7V[\x90\x9BP\x99Pa\x0C\x16\x8D\x8D\x8D\x8Da\x106V[\x90\x9BP\x99Pa\x0C0\x89\x89\x83`\x02` \x02\x01Q\x84`\x03a\x0B[V[\x90\x9DP\x9BPa\x0CA\x8B\x8B\x8F\x8Fa\x10\xA7V[``\x83\x01R`@\x82\x01R[\x9C\x9BPPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x80a\x0Co\x88\x88a\x12\x8BV[\x90\x92P\x90Pa\x0C\x80\x8C\x8C\x84\x84a\x106V[\x90\x96P\x94Pa\x0C\x91\x8A\x8A\x84\x84a\x106V[\x96\x9D\x95\x9CP\x9AP\x94\x98P\x92\x96PPPPPPPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a \xB2\x839\x81Q\x91R\x91\x82`\x03\x81\x80\x85\x80\t\x85\t\x08\x83\x82\x83\t\x14\x81\x15\x83\x15\x17\x19\x84\x83\x10\x85\x85\x10\x16\x16\x16\x93PPP\x81a\r0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[PPPV[`\0\x80`\0a\rC\x84a\x13\x16V[\x90P`\0\x80Q` a \xB2\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\rlWa\rla\x1EpV[\x84\x82\t\x90P\x82\x80a\r\x7FWa\r\x7Fa\x1EpV[\x82\x82\x08\x90P`\0\x80a\r\x90\x83a\x15IV[\x92P\x90P[\x80a\r\xF9W\x84\x80a\r\xA8Wa\r\xA8a\x1EpV[`\x01\x87\x08\x95P\x84\x80a\r\xBCWa\r\xBCa\x1EpV[\x86\x87\t\x92P\x84\x80a\r\xCFWa\r\xCFa\x1EpV[\x86\x84\t\x92P\x84\x80a\r\xE2Wa\r\xE2a\x1EpV[\x84\x84\x08\x92Pa\r\xF0\x83a\x15IV[\x92P\x90Pa\r\x95V[P\x93\x97\x93\x96P\x92\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x0E0WP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a \xB2\x839\x81Q\x91R\x84` \x01Qa\x0Ec\x91\x90a\x1E\x86V[a\x0E{\x90`\0\x80Q` a \xB2\x839\x81Q\x91Ra\x1E\xA8V[\x90R\x92\x91PPV[a\x0E\xAE`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R\x86Q`@\x82\x01R` \x87\x01Q``\x82\x01R`@\x87\x01Q`\x80\x82\x01R``\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R` \x85\x01Qa\x01 \x82\x01R`@\x85\x01Qa\x01@\x82\x01R``\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x10*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[P\x15\x15\x95\x94PPPPPV[`\0\x80a\x10t`\0\x80Q` a \xB2\x839\x81Q\x91R\x85\x88\t`\0\x80Q` a \xB2\x839\x81Q\x91R\x85\x88\t`\0\x80Q` a \xB2\x839\x81Q\x91Ra\x16DV[`\0\x80Q` a \xB2\x839\x81Q\x91R\x80\x86\x88\t`\0\x80Q` a \xB2\x839\x81Q\x91R\x86\x8A\t\x08\x91P\x91P\x94P\x94\x92PPPV[`\0\x80a\x10\xC3\x86\x85`\0\x80Q` a \xB2\x839\x81Q\x91Ra\x16DV[a\x10\xDC\x86\x85`\0\x80Q` a \xB2\x839\x81Q\x91Ra\x16DV[\x91P\x91P\x94P\x94\x92PPPV[`\0\x80`\0\x80`\0\x80a\x10\xFE\x8C\x8C`\x03a\x12XV[\x90\x96P\x94Pa\x11\x0F\x86\x86\x8E\x8Ea\x106V[\x90\x96P\x94Pa\x11 \x8A\x8A\x8A\x8Aa\x106V[\x90\x98P\x96Pa\x111\x8C\x8C\x8C\x8Ca\x106V[\x90\x94P\x92Pa\x11B\x84\x84\x8A\x8Aa\x106V[\x90\x94P\x92Pa\x11S\x86\x86\x81\x81a\x106V[\x90\x9CP\x9APa\x11d\x84\x84`\x08a\x12XV[\x90\x92P\x90Pa\x11u\x8C\x8C\x84\x84a\x10\xA7V[\x90\x9CP\x9APa\x11\x86\x88\x88\x81\x81a\x106V[\x90\x92P\x90Pa\x11\x97\x84\x84`\x04a\x12XV[\x90\x94P\x92Pa\x11\xA8\x84\x84\x8E\x8Ea\x10\xA7V[\x90\x94P\x92Pa\x11\xB9\x84\x84\x88\x88a\x106V[\x90\x94P\x92Pa\x11\xCA\x8A\x8A`\x08a\x12XV[\x90\x96P\x94Pa\x11\xDB\x86\x86\x8C\x8Ca\x106V[\x90\x96P\x94Pa\x11\xEC\x86\x86\x84\x84a\x106V[\x90\x96P\x94Pa\x11\xFD\x84\x84\x88\x88a\x10\xA7V[\x90\x94P\x92Pa\x12\x0E\x8C\x8C`\x02a\x12XV[\x90\x96P\x94Pa\x12\x1F\x86\x86\x8A\x8Aa\x106V[\x90\x96P\x94Pa\x120\x88\x88\x84\x84a\x106V[\x90\x92P\x90Pa\x12A\x82\x82`\x08a\x12XV[\x80\x92P\x81\x93PPP\x96P\x96P\x96P\x96P\x96P\x96\x90PV[`\0\x80`\0\x80Q` a \xB2\x839\x81Q\x91R\x83\x86\t`\0\x80Q` a \xB2\x839\x81Q\x91R\x84\x86\t\x91P\x91P\x93P\x93\x91PPV[`\0\x80\x80a\x12\xCC`\0\x80Q` a \xB2\x839\x81Q\x91R\x80\x87\x88\t`\0\x80Q` a \xB2\x839\x81Q\x91R\x87\x88\t\x08`\0\x80Q` a \xB2\x839\x81Q\x91Ra\x16hV[\x90P`\0\x80Q` a \xB2\x839\x81Q\x91R\x81\x86\t`\0\x80Q` a \xB2\x839\x81Q\x91R\x82\x86\ta\x13\n\x90`\0\x80Q` a \xB2\x839\x81Q\x91Ra\x1E\xA8V[\x92P\x92PP\x92P\x92\x90PV[`\0\x80a\x13\"\x83a\x16\xB9V[\x80Q\x90\x91P`0\x81\x14a\x137Wa\x137a\x1E\rV[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13RWa\x13Ra\x1A\xB4V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x13|W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x13\xF7W\x83`\x01a\x13\x97\x83\x86a\x1E\xA8V[a\x13\xA1\x91\x90a\x1E\xA8V[\x81Q\x81\x10a\x13\xB1Wa\x13\xB1a\x1D\xAFV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x13\xCEWa\x13\xCEa\x1D\xAFV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x13\xEF\x81a\x1D\xDBV[\x91PPa\x13\x82V[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x14\x93W\x83\x81a\x145\x85\x88a\x1E\xA8V[a\x14?\x91\x90a\x1D\xF4V[\x81Q\x81\x10a\x14OWa\x14Oa\x1D\xAFV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x14oWa\x14oa\x1D\xAFV[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x14\x8B\x81a\x1D\xDBV[\x91PPa\x14!V[P`\0a\x14\x9F\x82a\x1A+V[\x90Pa\x01\0`\0\x80Q` a \xB2\x839\x81Q\x91R`\0a\x14\xBF\x86\x89a\x1E\xA8V[\x90P`\0[\x81\x81\x10\x15a\x159W`\0\x88`\x01a\x14\xDB\x84\x86a\x1E\xA8V[a\x14\xE5\x91\x90a\x1E\xA8V[\x81Q\x81\x10a\x14\xF5Wa\x14\xF5a\x1D\xAFV[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x15\rWa\x15\ra\x1EpV[\x85\x87\t\x95P\x83\x80a\x15 Wa\x15 a\x1EpV[\x81\x87\x08\x95PP\x80\x80a\x151\x90a\x1D\xDBV[\x91PPa\x14\xC4V[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a \xB2\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x86``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x93PP`\0Q\x93P\x82a\x16\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[\x80`\x01\x85\x90\x1B\x11\x15a\x16$Wa\x16!\x84\x82a\x1E\xA8V[\x93P[\x80\x80a\x162Wa\x162a\x1EpV[\x84\x85\t\x91P\x85\x82\x14\x94PPPP\x91P\x91V[`\0\x81\x80a\x16TWa\x16Ta\x1EpV[a\x16^\x84\x84a\x1E\xA8V[\x85\x08\x94\x93PPPPV[`\0\x80`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01R`\x02\x84\x03`\x80\x82\x01R\x83`\xA0\x82\x01R` \x81`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x90Q\x92P\x90P\x80a\x16\xB2W`\0\x80\xFD[P\x92\x91PPV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x16\xFA\x92\x91\x90a\x1ESV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x17!\x92\x91\x90a\x1E\xBBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x17C\x91\x90a\x1E\xE7V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x17m\x90\x83\x90\x83\x90` \x01a\x1F\x01V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\xDEWa\x17\xDEa\x1A\xB4V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x18\x08W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x18 \x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x18\x95W\x81\x81\x81Q\x81\x10a\x18OWa\x18Oa\x1D\xAFV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x18lWa\x18la\x1D\xAFV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x18\x8D\x81a\x1D\xDBV[\x91PPa\x184V[P`\0\x84`@Q` \x01a\x18\xAB\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x19IW`\0\x83\x82\x81Q\x81\x10a\x18\xE6Wa\x18\xE6a\x1D\xAFV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x19\x03Wa\x19\x03a\x1D\xAFV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x19$\x92\x91\x90a\x1F&V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x98PP\x80\x80a\x19A\x90a\x1D\xDBV[\x91PPa\x18\xCAV[P\x86\x88\x87`@Q` \x01a\x19_\x93\x92\x91\x90a\x1FKV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x19\x8D\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x19\xAE\x8A`\xFF\x8D\x16a\x1E\xA8V[\x81\x10\x15a\x1A\x1AW\x82\x81\x81Q\x81\x10a\x19\xC7Wa\x19\xC7a\x1D\xAFV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x19\xE1\x83\x8Da\x1D\xF4V[\x81Q\x81\x10a\x19\xF1Wa\x19\xF1a\x1D\xAFV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x80a\x1A\x12\x81a\x1D\xDBV[\x91PPa\x19\xA1V[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a\x16\xB2W\x83\x81\x81Q\x81\x10a\x1AKWa\x1AKa\x1D\xAFV[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x1Ac\x91\x90a\x1F\x7FV[a\x1An\x90`\x02a zV[a\x1Ax\x91\x90a\x1F\x7FV[a\x1A\x82\x90\x83a\x1D\xF4V[\x91P\x80a\x1A\x8E\x81a\x1D\xDBV[\x91PPa\x1A0V[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A\xF3Wa\x1A\xF3a\x1A\xB4V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x1B\rW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1B0Wa\x1B0a\x1A\xB4V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1B[W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1BwWa\x1Bwa\x1A\xB4V[\x81`\x05\x1Ba\x1B\x86\x82\x82\x01a\x1A\xCAV[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x1B\xA0W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x1B\xCEW\x825\x80\x15\x15\x81\x14a\x1B\xBFW`\0\x80\x81\xFD[\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x1B\xA6V[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\xA0\x85\x87\x03\x12\x15a\x1B\xEFW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1C\x07W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x1C\x1BW`\0\x80\xFD[\x815` \x82\x82\x11\x15a\x1C/Wa\x1C/a\x1A\xB4V[a\x1CA`\x1F\x83\x01`\x1F\x19\x16\x82\x01a\x1A\xCAV[\x82\x81R\x8A\x82\x84\x87\x01\x01\x11\x15a\x1CUW`\0\x80\xFD[\x82\x82\x86\x01\x83\x83\x017`\0\x81\x84\x01\x83\x01R\x97Pa\x1Cs\x8A\x8A\x83\x01a\x1A\xFBV[\x96PPP``\x87\x015\x91P\x80\x82\x11\x15a\x1C\x8BW`\0\x80\xFD[Pa\x1C\x98\x87\x82\x88\x01a\x1BJV[\x94\x97\x93\x96P\x93\x94`\x80\x015\x93PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x1C\xBCW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1C\xD4W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x1C\xE8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1C\xF7W`\0\x80\xFD[\x86` \x82`\x07\x1B\x85\x01\x01\x11\x15a\x1D\x0CW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x1D0W`\0\x80\xFD[P5\x91\x90PV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a\x1DKW`\0\x80\xFD[`\x80\x81\x12\x15a\x1DYW`\0\x80\xFD[P`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1D}Wa\x1D}a\x1A\xB4V[`@\x90\x81R\x845\x82R` \x80\x86\x015\x90\x83\x01R\x84\x81\x015\x90\x82\x01R``\x80\x85\x015\x90\x82\x01R\x94`\x80\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1D\xEDWa\x1D\xEDa\x1D\xC5V[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x1E\x07Wa\x1E\x07a\x1D\xC5V[\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x81Q`\0[\x81\x81\x10\x15a\x1EDW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x1E*V[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x1Eha\x1Eb\x83\x86a\x1E#V[\x84a\x1E#V[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1E\xA3WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1E\x07Wa\x1E\x07a\x1D\xC5V[`\0a\x1E\xC7\x82\x85a\x1E#V[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a\x1E\xF3\x82\x84a\x1E#V[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a\x1F\r\x82\x85a\x1E#V[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a\x1F2\x82\x85a\x1E#V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a\x1FW\x82\x86a\x1E#V[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1E\x07Wa\x1E\x07a\x1D\xC5V[`\x01\x81\x81[\x80\x85\x11\x15a\x1F\xD1W\x81`\0\x19\x04\x82\x11\x15a\x1F\xB7Wa\x1F\xB7a\x1D\xC5V[\x80\x85\x16\x15a\x1F\xC4W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x1F\x9BV[P\x92P\x92\x90PV[`\0\x82a\x1F\xE8WP`\x01a\x1E\x07V[\x81a\x1F\xF5WP`\0a\x1E\x07V[\x81`\x01\x81\x14a \x0BW`\x02\x81\x14a \x15Wa 1V[`\x01\x91PPa\x1E\x07V[`\xFF\x84\x11\x15a &Wa &a\x1D\xC5V[PP`\x01\x82\x1Ba\x1E\x07V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a TWP\x81\x81\na\x1E\x07V[a ^\x83\x83a\x1F\x96V[\x80`\0\x19\x04\x82\x11\x15a rWa ra\x1D\xC5V[\x02\x93\x92PPPV[`\0a \x86\x83\x83a\x1F\xD9V[\x93\x92PPPV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 Q\xB3\xED\xD2\xB17\x80&<\x87e\xE0Pbw\xCEZq4\xBDeQf9\xCE4\xE6\x9C'm\xF3\xBCdsolcC\0\x08\x14\x003\xA2dipfsX\"\x12 \xB2\x88\x98\xEC\x9BF\xB0b9)Z4\x8E\xBA\xEC~\xBE\xB8\xEC\0\xDA\xDD\xEAo\xB0\xFF\xD8\x9B5u\xF0\x8DdsolcC\0\x08\x14\x003";
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
    ///`FuzzSelector(address,bytes4[])`
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
    pub struct FuzzSelector {
        pub addr: ::ethers::core::types::Address,
        pub selectors: ::std::vec::Vec<[u8; 4]>,
    }
}
