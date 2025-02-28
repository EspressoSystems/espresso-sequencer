pub use stake_table::*;
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
pub mod stake_table {
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
                        name: ::std::borrow::ToOwned::to_owned("_escrowPeriod"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_initialOwner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("_isEqualBlsKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("_isEqualBlsKey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("a"),
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
                                name: ::std::borrow::ToOwned::to_owned("b"),
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("admin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("delegate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delegate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("validator"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deregisterValidator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deregisterValidator",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("escrowPeriod"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("escrowPeriod"),
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
                    ::std::borrow::ToOwned::to_owned("exitEscrowPeriod"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("exitEscrowPeriod"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("node"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initializedAtBlock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initializedAtBlock"),
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
                    ::std::borrow::ToOwned::to_owned("lookupNode"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lookupNode"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
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
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("nodes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nodes"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
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
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("registerValidator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerValidator"),
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
                                name: ::std::borrow::ToOwned::to_owned("commission"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("totalStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalStake"),
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
                    ::std::borrow::ToOwned::to_owned("undelegate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("undelegate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("validator"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateConsensusKeys"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateConsensusKeys",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newBlsVK"),
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
                                name: ::std::borrow::ToOwned::to_owned("newSchnorrVK"),
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
                                name: ::std::borrow::ToOwned::to_owned("newBlsSig"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawFunds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawFunds"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ConsensusKeysUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ConsensusKeysUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("blsVK"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("schnorrVK"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Delegated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Delegated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("delegator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("validator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("Undelegated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Undelegated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("delegator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("validator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("ValidatorExit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ValidatorExit"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("validator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ValidatorRegistered"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ValidatorRegistered",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("blsVk"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("schnorrVk"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("commission"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Withdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Withdrawal"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("InsufficientAllowance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientAllowance",),
                        inputs: ::std::vec![
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientBalance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientStakeAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientStakeAmount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientStakeBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientStakeBalance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("InvalidBlsVK"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidBlsVK"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidHotShotBlocksPerEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidHotShotBlocksPerEpoch",),
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
                    ::std::borrow::ToOwned::to_owned("InvalidSchnorrVK"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidSchnorrVK"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidValue"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidValue"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoKeyChange"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NoKeyChange"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("NodeNotRegistered"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NodeNotRegistered"),
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
                (
                    ::std::borrow::ToOwned::to_owned("Unauthorized"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Unauthorized"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STAKETABLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x10W_\x80\xFD[P`@Qb\0&P8\x03\x80b\0&P\x839\x81\x01`@\x81\x90Rb\0\x003\x91b\0\x02-V[\x80`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\0bW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[b\0\0m\x81b\0\0\xABV[Pb\0\0xb\0\0\xFCV[P`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x19\x93\x84\x16\x17\x90U`\x07U`\x08\x80T\x90\x91\x163\x17\x90Ub\0\x02kV[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16_\x81\x15\x80\x15b\0\x01FWP\x82[\x90P_\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15b\0\x01bWP0;\x15[\x90P\x81\x15\x80\x15b\0\x01qWP\x80\x15[\x15b\0\x01\x90W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84T`\x01`\x01`@\x1B\x03\x19\x16`\x01\x17\x85U\x83\x15b\0\x01\xBFW\x84T`\xFF`@\x1B\x19\x16h\x01\0\0\0\0\0\0\0\0\x17\x85U[C`\x03U\x83\x15b\0\x02\nW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02(W_\x80\xFD[\x91\x90PV[_\x80_``\x84\x86\x03\x12\x15b\0\x02@W_\x80\xFD[b\0\x02K\x84b\0\x02\x11V[\x92P` \x84\x01Q\x91Pb\0\x02b`@\x85\x01b\0\x02\x11V[\x90P\x92P\x92P\x92V[a#\xD7\x80b\0\x02y_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01HW_5`\xE0\x1C\x80cqP\x18\xA6\x11a\0\xBFW\x80c\xC3\x15\xB6\xBD\x11a\0yW\x80c\xC3\x15\xB6\xBD\x14a\x03UW\x80c\xF0 e\xF8\x14a\x03hW\x80c\xF1kQ\xC1\x14a\x03\x88W\x80c\xF2\xF8\n\x18\x14a\x03\x91W\x80c\xF2\xFD\xE3\x8B\x14a\x03\xB4W\x80c\xF8Q\xA4@\x14a\x03\xC7W_\x80\xFD[\x80cqP\x18\xA6\x14a\x02\xF1W\x80c\x81)\xFC\x1C\x14a\x02\xF9W\x80c\x8B\x0E\x9F?\x14a\x03\x01W\x80c\x8D\xA5\xCB[\x14a\x03\nW\x80c\x9B0\xA5\xE6\x14a\x03/W\x80c\x9Dv\xEAX\x14a\x03BW_\x80\xFD[\x80c>\x9D\xF9\xB5\x11a\x01\x10W\x80c>\x9D\xF9\xB5\x14a\x02\x8DW\x80cC\x17\xD0\x0B\x14a\x02\x96W\x80cH\x8B\xDA\xBC\x14a\x02\x9FW\x80cM\x99\xDD\x16\x14a\x02\xC3W\x80cUD\xC2\xF1\x14a\x02\xD6W\x80cj\x91\x1C\xCF\x14a\x02\xE9W_\x80\xFD[\x80c\x02n@+\x14a\x01LW\x80c\x13\xB9\x05z\x14a\x01aW\x80c\x18\x9AZ\x17\x14a\x01tW\x80c$`\x0F\xC3\x14a\x02KW\x80c1\xC7\x1E\xBF\x14a\x02aW[_\x80\xFD[a\x01_a\x01Z6`\x04a\x1C\xA9V[a\x03\xDAV[\0[a\x01_a\x01o6`\x04a\x1D\xBBV[a\x052V[a\x01\xF0a\x01\x826`\x04a\x1E\x19V[`\x04` \x81\x81R_\x92\x83R`@\x92\x83\x90 \x80T`\x01\x82\x01T\x85Q\x80\x87\x01\x87R`\x02\x84\x01T\x81R`\x03\x84\x01T\x81\x86\x01R\x86Q`\x80\x81\x01\x88R\x95\x84\x01T\x86R`\x05\x84\x01T\x94\x86\x01\x94\x90\x94R`\x06\x83\x01T\x95\x85\x01\x95\x90\x95R`\x07\x90\x91\x01T``\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x84V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85R` \x80\x86\x01\x94\x90\x94R\x82Q\x85\x82\x01R\x91\x83\x01Q``\x80\x86\x01\x91\x90\x91R\x81Q`\x80\x86\x01R\x92\x81\x01Q`\xA0\x85\x01R\x90\x81\x01Q`\xC0\x84\x01R\x01Q`\xE0\x82\x01Ra\x01\0\x01[`@Q\x80\x91\x03\x90\xF3[a\x02Sa\x07EV[`@Q\x90\x81R` \x01a\x02BV[a\x02ta\x02o6`\x04a\x1E4V[a\x08\xC6V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02BV[a\x02S`\x03T\x81V[a\x02S`\x01T\x81V[_Ta\x02\xAE\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02BV[a\x01_a\x02\xD16`\x04a\x1C\xA9V[a\x08\xE4V[a\x01_a\x02\xE46`\x04a\x1E\x8FV[a\t1V[a\x01_a\x0B\x82V[a\x01_a\x0C\x84V[a\x01_a\x0C\x97V[a\x02S`\x05T\x81V[`\x02T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02BV[a\x02Sa\x03=6`\x04a\x1E\xD3V[a\r\x9FV[`\x06Ta\x03\x17\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02Sa\x03c6`\x04a\x1E\x19V[a\r\xF9V[a\x03{a\x03v6`\x04a\x1E\x19V[a\x0EoV[`@Qa\x02B\x91\x90a\x1E\xEDV[a\x02S`\x07T\x81V[a\x03\xA4a\x03\x9F6`\x04a\x1FVV[a\x0F\x05V[`@Q\x90\x15\x15\x81R` \x01a\x02BV[a\x01_a\x03\xC26`\x04a\x1E\x19V[a\x0FLV[`\x08Ta\x03\x17\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[3_\x90\x81R`\x04` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R`\x01\x84\x01T\x83\x86\x01R\x86Q\x80\x88\x01\x88R`\x02\x85\x01T\x81R`\x03\x85\x01T\x81\x87\x01R\x83\x88\x01R\x86Q\x91\x82\x01\x87R\x94\x83\x01T\x81R`\x05\x83\x01T\x93\x81\x01\x93\x90\x93R`\x06\x82\x01T\x94\x83\x01\x94\x90\x94R`\x07\x01T``\x82\x81\x01\x91\x90\x91R\x83\x01R\x81Q\x16a\x04|W`@Qb)\xEA\xAD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xA6W`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x04` R`@\x81 `\x01\x01\x80T\x84\x92\x90a\x04\xC7\x90\x84\x90a\x1F\x9EV[\x90\x91UPP`\x06Ta\x04\xE4\x90`\x01`\x01`\xA0\x1B\x03\x1630\x85a\x0F\x89V[`@\x80Q3\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16` \x82\x01R\x90\x81\x01\x83\x90R\x7F\xE5T\x1Aka\x03\xD4\xFA~\x02\x1E\xD5O\xAD9\xC6o'\xA7k\xD1=7L\xF6$\n\xE6\xBD\x0B\xB7+\x90``\x01`@Q\x80\x91\x03\x90\xA1PPPV[3_\x90\x81R`\x04` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R`\x01\x84\x01T\x83\x86\x01R\x86Q\x80\x88\x01\x88R`\x02\x85\x01T\x81R`\x03\x85\x01T\x81\x87\x01R\x83\x88\x01R\x86Q\x91\x82\x01\x87R\x94\x83\x01T\x81R`\x05\x83\x01T\x93\x81\x01\x93\x90\x93R`\x06\x82\x01T\x94\x83\x01\x94\x90\x94R`\x07\x01T``\x82\x81\x01\x91\x90\x91R\x83\x01R\x81Q\x16\x15a\x05\xD6W`@Qc\x0E\xB0\xD3\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\0\x85`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RPa\x0F\x05V[\x15a\x06\x1EW`@Qc>\xE8\xB0q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R_\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x06G\x81\x85\x88a\x10\x1AV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x06e\x90\x86\x90a\x10\xAFV[\x15a\x06\x83W`@Qc\x06\xCFC\x8F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3\x80\x83R``\x80\x84\x01\x88\x81R`@\x80\x86\x01\x89\x81R_\x85\x81R`\x04` \x81\x81R\x91\x84\x90 \x89Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x82\x8A\x01Q`\x01\x82\x01U\x92Q\x80Q`\x02\x85\x01U\x82\x01Q`\x03\x84\x01U\x93Q\x80Q\x94\x83\x01\x94\x90\x94U\x83\x01Q`\x05\x82\x01U\x82\x82\x01Q`\x06\x82\x01U\x91\x90\x92\x01Q`\x07\x90\x91\x01UQ\x7F\xF6\xE85\x9CWR\x0BF\x964sk\xFC;\xB7\xEC\\\xBD\x1A\x0B\xD2\x8B\x10\xA8'W\x93\xBBs\x0By\x7F\x91a\x075\x91\x89\x90\x89\x90\x88\x90a\x1F\xB1V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[3_\x90\x81R`\x04` \x81\x81R`@\x80\x84 \x81Q`\x80\x80\x82\x01\x84R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R`\x01\x84\x01T\x83\x87\x01R\x84Q\x80\x86\x01\x86R`\x02\x85\x01T\x81R`\x03\x85\x01T\x81\x88\x01R\x83\x86\x01R\x84Q\x91\x82\x01\x85R\x95\x83\x01T\x81R`\x05\x83\x01T\x94\x81\x01\x94\x90\x94R`\x06\x82\x01T\x92\x84\x01\x92\x90\x92R`\x07\x01T``\x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x80Q\x90\x91\x16a\x07\xEBW`@Qb)\xEA\xAD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\x15W`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q_\x81\x90\x03a\x08BW`@Qc\xBA\xA1tO`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[\x80`\x05_\x82\x82Ta\x08S\x91\x90a \x14V[\x90\x91UPP3_\x90\x81R`\x04` \x81\x90R`@\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x83\x90U`\x02\x81\x01\x83\x90U`\x03\x81\x01\x83\x90U\x90\x81\x01\x82\x90U`\x05\x81\x01\x82\x90U`\x06\x80\x82\x01\x83\x90U`\x07\x90\x91\x01\x91\x90\x91UT\x82Qa\x08\xC0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x83a\x10\xCBV[\x92\x91PPV[_`d\x82` \x01Q\x11\x15a\x08\xDCWP`\n\x91\x90PV[P`\x05\x91\x90PV[`@\x80Q3\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R\x90\x81\x01\x82\x90R\x7FM\x10\xBD\x04\x97u\xC7{\xD7\xF2U\x19Z\xFB\xA5\x08\x80(\xEC\xB3\xC7\xC2w\xD3\x93\xCC\xFFy4\xF2\xF9,\x90``\x01`@Q\x80\x91\x03\x90\xA1PPV[3_\x90\x81R`\x04` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R`\x01\x84\x01T\x83\x86\x01R\x86Q\x80\x88\x01\x88R`\x02\x85\x01T\x81R`\x03\x85\x01T\x81\x87\x01R\x83\x88\x01R\x86Q\x91\x82\x01\x87R\x94\x83\x01T\x81R`\x05\x83\x01T\x93\x81\x01\x93\x90\x93R`\x06\x82\x01T\x94\x83\x01\x94\x90\x94R`\x07\x01T``\x82\x81\x01\x91\x90\x91R\x83\x01R\x81Q\x16a\t\xD3W`@Qb)\xEA\xAD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xE1\x84\x82``\x01Qa\x0F\x05V[\x80\x15a\t\xF8WP`@\x81\x01Qa\t\xF8\x90\x84\x90a\x10\xAFV[\x15a\n\x16W`@Qc\xE0\x12.3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90a\nR\x86\x83a\x0F\x05V[\x15a\npW`@Qc>\xE8\xB0q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\nz\x85\x82a\x10\xAFV[\x15a\n\x98W`@Qc\x06\xCFC\x8F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R_\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\n\xC1\x81\x86\x89a\x10\x1AV[``\x84\x81\x01\x88\x81R`@\x80\x87\x01\x89\x81R3_\x81\x81R`\x04` \x81\x81R\x91\x85\x90 \x8BQ\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x82\x8C\x01Q`\x01\x82\x01U\x93Q\x80Q`\x02\x86\x01U\x80\x83\x01Q`\x03\x86\x01U\x95Q\x80Q\x91\x85\x01\x91\x90\x91U\x90\x81\x01Q`\x05\x84\x01U\x80\x84\x01Q`\x06\x84\x01U\x94\x85\x01Q`\x07\x90\x92\x01\x91\x90\x91U\x90Q\x7F\x80\xD8\xA4\xA1f3(\xA9\x98\xD4U[\xA2\x1D\x8B\xBAn\xF1Wj\x8C^\x9D'\xF9\xC5E\xF1\xA3\xD5+\x1D\x93a\x0Bq\x93\x90\x91a 'V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[3_\x90\x81R`\x04` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R`\x01\x84\x01T\x83\x86\x01R\x86Q\x80\x88\x01\x88R`\x02\x85\x01T\x81R`\x03\x85\x01T\x81\x87\x01R\x83\x88\x01R\x86Q\x91\x82\x01\x87R\x94\x83\x01T\x81R`\x05\x83\x01T\x93\x81\x01\x93\x90\x93R`\x06\x82\x01T\x94\x83\x01\x94\x90\x94R`\x07\x01T``\x82\x81\x01\x91\x90\x91R\x83\x01R\x81Q\x16a\x0C$W`@Qb)\xEA\xAD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0CNW`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q3\x81R\x7F\xFB$0ST\xC8wb\xD5WHz\xE4\xA5d\xE8\xD0>\xCB\xB9\xA9}\xD8\xAF\xFF\x8E\x1Fo\xCA\xF0\xDD\x16\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[a\x0C\x8Ca\x11NV[a\x0C\x95_a\x11{V[V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x81\x15\x80\x15a\x0C\xDCWP\x82[\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x0C\xF8WP0;\x15[\x90P\x81\x15\x80\x15a\r\x06WP\x80\x15[\x15a\r$W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\rNW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[C`\x03U\x83\x15a\r\x98W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\r\xDC\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@Qc\x1E\x04\x0C\xBF`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R_\x90\x81\x900\x90c\xF0 e\xF8\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E@W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ed\x91\x90a {V[` \x01Q\x93\x92PPPV[a\x0Ewa\x1C)V[P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x04` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T\x90\x96\x16\x81R`\x01\x82\x01T\x81\x84\x01R\x84Q\x80\x86\x01\x86R`\x02\x83\x01T\x81R`\x03\x83\x01T\x81\x85\x01R\x81\x86\x01R\x84Q\x95\x86\x01\x85R\x92\x81\x01T\x85R`\x05\x81\x01T\x91\x85\x01\x91\x90\x91R`\x06\x81\x01T\x92\x84\x01\x92\x90\x92R`\x07\x90\x91\x01T``\x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[\x80Q\x82Q_\x91\x14\x80\x15a\x0F\x1FWP\x81` \x01Q\x83` \x01Q\x14[\x80\x15a\x0F2WP\x81`@\x01Q\x83`@\x01Q\x14[\x80\x15a\x0FEWP\x81``\x01Q\x83``\x01Q\x14[\x93\x92PPPV[a\x0FTa\x11NV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0F}W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x089V[a\x0F\x86\x81a\x11{V[PV[_`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R\x82`D\x82\x01R` _`d\x83_\x8AZ\xF1=\x15`\x1F=\x11`\x01_Q\x14\x16\x17\x16\x91PP\x80a\r\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x089V[a\x10#\x82a\x11\xCCV[_`@Q\x80``\x01`@R\x80`$\x81R` \x01a#\x87`$\x919\x90P_\x84\x82`@Q` \x01a\x10S\x92\x91\x90a!XV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P_a\x10n\x82a\x12gV[\x90Pa\x10\x8B\x81\x85a\x10~\x88a\x13TV[a\x10\x86a\x13\xCBV[a\x14\x98V[a\x10\xA7W`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[\x80Q\x82Q_\x91\x14\x80\x15a\x0FEWPP` \x90\x81\x01Q\x91\x01Q\x14\x90V[_`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` _`D\x83_\x89Z\xF1=\x15`\x1F=\x11`\x01_Q\x14\x16\x17\x16\x91PP\x80a\x11HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x089V[PPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\x95W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x089V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[\x80Q` \x82\x01Q_\x91_\x80Q` a#\xAB\x839\x81Q\x91R\x91\x15\x90\x15\x16\x15a\x11\xF2WPPPV[\x82Q` \x84\x01Q\x82`\x03\x84\x85\x85\x86\t\x85\t\x08\x83\x82\x83\t\x14\x83\x82\x10\x84\x84\x10\x16\x16\x93PPP\x81a\x12bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x089V[PPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_a\x12\x84\x83a\x15xV[\x90P_\x80Q` a#\xAB\x839\x81Q\x91R`\x03_\x82\x84\x85\t\x90P\x82\x80a\x12\xABWa\x12\xABa!lV[\x84\x82\t\x90P\x82\x80a\x12\xBEWa\x12\xBEa!lV[\x82\x82\x08\x90P_\x80a\x12\xCE\x83a\x17\x81V[\x92P\x90P[\x80a\x137W\x84\x80a\x12\xE6Wa\x12\xE6a!lV[`\x01\x87\x08\x95P\x84\x80a\x12\xFAWa\x12\xFAa!lV[\x86\x87\t\x92P\x84\x80a\x13\rWa\x13\ra!lV[\x86\x84\t\x92P\x84\x80a\x13 Wa\x13 a!lV[\x84\x84\x08\x92Pa\x13.\x83a\x17\x81V[\x92P\x90Pa\x12\xD3V[P`@\x80Q\x80\x82\x01\x90\x91R\x94\x85R` \x85\x01RP\x91\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x13{WP\x90V[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01_\x80Q` a#\xAB\x839\x81Q\x91R\x84` \x01Qa\x13\xAC\x91\x90a!\x80V[a\x13\xC3\x90_\x80Q` a#\xAB\x839\x81Q\x91Ra \x14V[\x90R\x92\x91PPV[a\x13\xF2`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81RP\x90P\x90V[_\x80_`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R` \x87\x01Q`@\x82\x01R\x86Q``\x82\x01R``\x87\x01Q`\x80\x82\x01R`@\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R` \x85\x01Qa\x01\0\x82\x01R\x84Qa\x01 \x82\x01R``\x85\x01Qa\x01@\x82\x01R`@\x85\x01Qa\x01`\x82\x01R` _a\x01\x80\x83`\x08Z\xFA\x91PP_Q\x91P\x80a\x15jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x089V[P\x15\x15\x90P[\x94\x93PPPPV[_\x80a\x15\x83\x83a\x18xV[\x80Q\x90\x91P`0\x81\x14a\x15\x98Wa\x15\x98a!\x9FV[_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xB2Wa\x15\xB2a\x1C\xD3V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x15\xDCW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a\x16KW\x83`\x01a\x15\xF6\x83\x86a \x14V[a\x16\0\x91\x90a \x14V[\x81Q\x81\x10a\x16\x10Wa\x16\x10a!\xB3V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x16-Wa\x16-a!\xB3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x15\xE1V[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R_\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P_[\x82\x81\x10\x15a\x16\xDBW\x83\x81a\x16\x87\x85\x88a \x14V[a\x16\x91\x91\x90a\x1F\x9EV[\x81Q\x81\x10a\x16\xA1Wa\x16\xA1a!\xB3V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x16\xC1Wa\x16\xC1a!\xB3V[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x16sV[P_a\x16\xE6\x82a\x1B\xC2V[\x90Pa\x01\0_\x80Q` a#\xAB\x839\x81Q\x91R_a\x17\x04\x86\x89a \x14V[\x90P_[\x81\x81\x10\x15a\x17qW_\x88`\x01a\x17\x1E\x84\x86a \x14V[a\x17(\x91\x90a \x14V[\x81Q\x81\x10a\x178Wa\x178a!\xB3V[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x17PWa\x17Pa!lV[\x85\x87\t\x95P\x83\x80a\x17cWa\x17ca!lV[\x81\x87\x08\x95PP`\x01\x01a\x17\x08V[P\x92\x9A\x99PPPPPPPPPPV[_\x80_\x80_\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P__\x80Q` a#\xAB\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x87``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` _`\xC0\x83`\x05Z\xFA\x94PP_Q\x92P\x83a\x18>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x089V[\x80`\x01\x84\x90\x1B\x11\x15a\x18WWa\x18T\x83\x82a \x14V[\x92P[\x80\x80a\x18eWa\x18ea!lV[\x83\x84\t\x96\x90\x96\x14\x96\x91\x95P\x90\x93PPPPV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90_\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x18\xB8\x92\x91\x90a!XV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x18\xDF\x92\x91\x90a!\xC7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x19\x01\x91\x90a!\xF1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x19+\x90\x83\x90\x83\x90` \x01a\"\tV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90_`\xFF\x88\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\x9BWa\x19\x9Ba\x1C\xD3V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x19\xC5W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_\x82`@Q` \x01a\x19\xDC\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P_[\x81Q\x81\x10\x15a\x1AEW\x81\x81\x81Q\x81\x10a\x1A\nWa\x1A\na!\xB3V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x1A'Wa\x1A'a!\xB3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x19\xEFV[P_\x84`@Q` \x01a\x1AZ\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R_\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x1A\xECW_\x83\x82\x81Q\x81\x10a\x1A\x93Wa\x1A\x93a!\xB3V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x1A\xB0Wa\x1A\xB0a!\xB3V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x1A\xD1\x92\x91\x90a\"-V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x98PP`\x01\x01a\x1AxV[P\x86\x88\x87`@Q` \x01a\x1B\x02\x93\x92\x91\x90a\"QV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x1B0\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P_[a\x1BP\x8A`\xFF\x8D\x16a \x14V[\x81\x10\x15a\x1B\xB1W\x82\x81\x81Q\x81\x10a\x1BiWa\x1Bia!\xB3V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x1B\x83\x83\x8Da\x1F\x9EV[\x81Q\x81\x10a\x1B\x93Wa\x1B\x93a!\xB3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x1BCV[P\x91\x9B\x9APPPPPPPPPPPV[_\x80\x80[\x83Q\x81\x10\x15a\x1C\"W\x83\x81\x81Q\x81\x10a\x1B\xE1Wa\x1B\xE1a!\xB3V[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x1B\xF9\x91\x90a\"\x84V[a\x1C\x04\x90`\x02a#{V[a\x1C\x0E\x91\x90a\"\x84V[a\x1C\x18\x90\x83a\x1F\x9EV[\x91P`\x01\x01a\x1B\xC6V[P\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01a\x1Cd`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x1C\x90`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x90R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\x86W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a\x1C\xBAW_\x80\xFD[\x825a\x1C\xC5\x81a\x1C\x95V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\x16WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\x16WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_`\x80\x82\x84\x03\x12\x15a\x1D[W_\x80\xFD[a\x1Dca\x1C\xE7V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R\x92\x91PPV[_`@\x82\x84\x03\x12\x15a\x1D\x9DW_\x80\xFD[a\x1D\xA5a\x1D\x1CV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[_\x80_\x80a\x01 \x85\x87\x03\x12\x15a\x1D\xCFW_\x80\xFD[a\x1D\xD9\x86\x86a\x1DKV[\x93Pa\x1D\xE8\x86`\x80\x87\x01a\x1D\x8DV[\x92Pa\x1D\xF7\x86`\xC0\x87\x01a\x1D\x8DV[\x91Pa\x01\0\x85\x015a\xFF\xFF\x81\x16\x81\x14a\x1E\x0EW_\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[_` \x82\x84\x03\x12\x15a\x1E)W_\x80\xFD[\x815a\x0FE\x81a\x1C\x95V[_a\x01\0\x82\x84\x03\x12\x15a\x1EEW_\x80\xFD[a\x1EMa\x1C\xE7V[\x825a\x1EX\x81a\x1C\x95V[\x81R` \x83\x81\x015\x90\x82\x01Ra\x1Eq\x84`@\x85\x01a\x1D\x8DV[`@\x82\x01Ra\x1E\x83\x84`\x80\x85\x01a\x1DKV[``\x82\x01R\x93\x92PPPV[_\x80_a\x01\0\x84\x86\x03\x12\x15a\x1E\xA2W_\x80\xFD[a\x1E\xAC\x85\x85a\x1DKV[\x92Pa\x1E\xBB\x85`\x80\x86\x01a\x1D\x8DV[\x91Pa\x1E\xCA\x85`\xC0\x86\x01a\x1D\x8DV[\x90P\x92P\x92P\x92V[_`\x80\x82\x84\x03\x12\x15a\x1E\xE3W_\x80\xFD[a\x0FE\x83\x83a\x1DKV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Qa\x01\0\x83\x01\x91a\x1F&\x90\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P``\x83\x81\x01Q\x80Q`\x80\x85\x01R` \x81\x01Q`\xA0\x85\x01R`@\x81\x01Q`\xC0\x85\x01R\x90\x81\x01Q`\xE0\x84\x01Ra\x1C\"V[_\x80a\x01\0\x83\x85\x03\x12\x15a\x1FhW_\x80\xFD[a\x1Fr\x84\x84a\x1DKV[\x91Pa\x1F\x81\x84`\x80\x85\x01a\x1DKV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x08\xC0Wa\x08\xC0a\x1F\x8AV[`\x01`\x01`\xA0\x1B\x03\x85\x16\x81Ra\x01\0\x81\x01a\x1F\xF0` \x83\x01\x86\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R``\x81\x01Q``\x83\x01RPPV[\x83Q`\xA0\x83\x01R` \x84\x01Q`\xC0\x83\x01Ra\xFF\xFF\x83\x16`\xE0\x83\x01R\x95\x94PPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x08\xC0Wa\x08\xC0a\x1F\x8AV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\xE0\x81\x01a e` \x83\x01\x85\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R``\x81\x01Q``\x83\x01RPPV[\x82Q`\xA0\x83\x01R` \x83\x01Q`\xC0\x83\x01Ra\x15pV[_\x81\x83\x03a\x01\0\x81\x12\x15a \x8DW_\x80\xFD[a \x95a\x1C\xE7V[\x83Qa \xA0\x81a\x1C\x95V[\x81R` \x84\x81\x01Q\x90\x82\x01R`@`?\x19\x83\x01\x12\x15a \xBDW_\x80\xFD[a \xC5a\x1D\x1CV[`@\x85\x81\x01Q\x82R``\x86\x01Q` \x83\x01R\x82\x01R`\x80`\x7F\x19\x83\x01\x12\x15a \xEBW_\x80\xFD[a \xF3a\x1C\xE7V[\x91P`\x80\x84\x01Q\x82R`\xA0\x84\x01Q` \x83\x01R`\xC0\x84\x01Q`@\x83\x01R`\xE0\x84\x01Q``\x83\x01R\x81``\x82\x01R\x80\x92PPP\x92\x91PPV[_\x81Q_[\x81\x81\x10\x15a!JW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a!0V[P_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x15pa!f\x83\x86a!+V[\x84a!+V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a!\x9AWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_a!\xD2\x82\x85a!+V[_\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[_a!\xFC\x82\x84a!+V[_\x81R`\x01\x01\x93\x92PPPV[_a\"\x14\x82\x85a!+V[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[_a\"8\x82\x85a!+V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[_a\"\\\x82\x86a!+V[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x08\xC0Wa\x08\xC0a\x1F\x8AV[`\x01\x81\x81[\x80\x85\x11\x15a\"\xD5W\x81_\x19\x04\x82\x11\x15a\"\xBBWa\"\xBBa\x1F\x8AV[\x80\x85\x16\x15a\"\xC8W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\"\xA0V[P\x92P\x92\x90PV[_\x82a\"\xEBWP`\x01a\x08\xC0V[\x81a\"\xF7WP_a\x08\xC0V[\x81`\x01\x81\x14a#\rW`\x02\x81\x14a#\x17Wa#3V[`\x01\x91PPa\x08\xC0V[`\xFF\x84\x11\x15a#(Wa#(a\x1F\x8AV[PP`\x01\x82\x1Ba\x08\xC0V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a#VWP\x81\x81\na\x08\xC0V[a#`\x83\x83a\"\x9BV[\x80_\x19\x04\x82\x11\x15a#sWa#sa\x1F\x8AV[\x02\x93\x92PPPV[_a\x0FE\x83\x83a\"\xDDV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static STAKETABLE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01HW_5`\xE0\x1C\x80cqP\x18\xA6\x11a\0\xBFW\x80c\xC3\x15\xB6\xBD\x11a\0yW\x80c\xC3\x15\xB6\xBD\x14a\x03UW\x80c\xF0 e\xF8\x14a\x03hW\x80c\xF1kQ\xC1\x14a\x03\x88W\x80c\xF2\xF8\n\x18\x14a\x03\x91W\x80c\xF2\xFD\xE3\x8B\x14a\x03\xB4W\x80c\xF8Q\xA4@\x14a\x03\xC7W_\x80\xFD[\x80cqP\x18\xA6\x14a\x02\xF1W\x80c\x81)\xFC\x1C\x14a\x02\xF9W\x80c\x8B\x0E\x9F?\x14a\x03\x01W\x80c\x8D\xA5\xCB[\x14a\x03\nW\x80c\x9B0\xA5\xE6\x14a\x03/W\x80c\x9Dv\xEAX\x14a\x03BW_\x80\xFD[\x80c>\x9D\xF9\xB5\x11a\x01\x10W\x80c>\x9D\xF9\xB5\x14a\x02\x8DW\x80cC\x17\xD0\x0B\x14a\x02\x96W\x80cH\x8B\xDA\xBC\x14a\x02\x9FW\x80cM\x99\xDD\x16\x14a\x02\xC3W\x80cUD\xC2\xF1\x14a\x02\xD6W\x80cj\x91\x1C\xCF\x14a\x02\xE9W_\x80\xFD[\x80c\x02n@+\x14a\x01LW\x80c\x13\xB9\x05z\x14a\x01aW\x80c\x18\x9AZ\x17\x14a\x01tW\x80c$`\x0F\xC3\x14a\x02KW\x80c1\xC7\x1E\xBF\x14a\x02aW[_\x80\xFD[a\x01_a\x01Z6`\x04a\x1C\xA9V[a\x03\xDAV[\0[a\x01_a\x01o6`\x04a\x1D\xBBV[a\x052V[a\x01\xF0a\x01\x826`\x04a\x1E\x19V[`\x04` \x81\x81R_\x92\x83R`@\x92\x83\x90 \x80T`\x01\x82\x01T\x85Q\x80\x87\x01\x87R`\x02\x84\x01T\x81R`\x03\x84\x01T\x81\x86\x01R\x86Q`\x80\x81\x01\x88R\x95\x84\x01T\x86R`\x05\x84\x01T\x94\x86\x01\x94\x90\x94R`\x06\x83\x01T\x95\x85\x01\x95\x90\x95R`\x07\x90\x91\x01T``\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x84V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85R` \x80\x86\x01\x94\x90\x94R\x82Q\x85\x82\x01R\x91\x83\x01Q``\x80\x86\x01\x91\x90\x91R\x81Q`\x80\x86\x01R\x92\x81\x01Q`\xA0\x85\x01R\x90\x81\x01Q`\xC0\x84\x01R\x01Q`\xE0\x82\x01Ra\x01\0\x01[`@Q\x80\x91\x03\x90\xF3[a\x02Sa\x07EV[`@Q\x90\x81R` \x01a\x02BV[a\x02ta\x02o6`\x04a\x1E4V[a\x08\xC6V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02BV[a\x02S`\x03T\x81V[a\x02S`\x01T\x81V[_Ta\x02\xAE\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02BV[a\x01_a\x02\xD16`\x04a\x1C\xA9V[a\x08\xE4V[a\x01_a\x02\xE46`\x04a\x1E\x8FV[a\t1V[a\x01_a\x0B\x82V[a\x01_a\x0C\x84V[a\x01_a\x0C\x97V[a\x02S`\x05T\x81V[`\x02T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02BV[a\x02Sa\x03=6`\x04a\x1E\xD3V[a\r\x9FV[`\x06Ta\x03\x17\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02Sa\x03c6`\x04a\x1E\x19V[a\r\xF9V[a\x03{a\x03v6`\x04a\x1E\x19V[a\x0EoV[`@Qa\x02B\x91\x90a\x1E\xEDV[a\x02S`\x07T\x81V[a\x03\xA4a\x03\x9F6`\x04a\x1FVV[a\x0F\x05V[`@Q\x90\x15\x15\x81R` \x01a\x02BV[a\x01_a\x03\xC26`\x04a\x1E\x19V[a\x0FLV[`\x08Ta\x03\x17\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[3_\x90\x81R`\x04` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R`\x01\x84\x01T\x83\x86\x01R\x86Q\x80\x88\x01\x88R`\x02\x85\x01T\x81R`\x03\x85\x01T\x81\x87\x01R\x83\x88\x01R\x86Q\x91\x82\x01\x87R\x94\x83\x01T\x81R`\x05\x83\x01T\x93\x81\x01\x93\x90\x93R`\x06\x82\x01T\x94\x83\x01\x94\x90\x94R`\x07\x01T``\x82\x81\x01\x91\x90\x91R\x83\x01R\x81Q\x16a\x04|W`@Qb)\xEA\xAD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xA6W`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x04` R`@\x81 `\x01\x01\x80T\x84\x92\x90a\x04\xC7\x90\x84\x90a\x1F\x9EV[\x90\x91UPP`\x06Ta\x04\xE4\x90`\x01`\x01`\xA0\x1B\x03\x1630\x85a\x0F\x89V[`@\x80Q3\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16` \x82\x01R\x90\x81\x01\x83\x90R\x7F\xE5T\x1Aka\x03\xD4\xFA~\x02\x1E\xD5O\xAD9\xC6o'\xA7k\xD1=7L\xF6$\n\xE6\xBD\x0B\xB7+\x90``\x01`@Q\x80\x91\x03\x90\xA1PPPV[3_\x90\x81R`\x04` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R`\x01\x84\x01T\x83\x86\x01R\x86Q\x80\x88\x01\x88R`\x02\x85\x01T\x81R`\x03\x85\x01T\x81\x87\x01R\x83\x88\x01R\x86Q\x91\x82\x01\x87R\x94\x83\x01T\x81R`\x05\x83\x01T\x93\x81\x01\x93\x90\x93R`\x06\x82\x01T\x94\x83\x01\x94\x90\x94R`\x07\x01T``\x82\x81\x01\x91\x90\x91R\x83\x01R\x81Q\x16\x15a\x05\xD6W`@Qc\x0E\xB0\xD3\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\0\x85`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RPa\x0F\x05V[\x15a\x06\x1EW`@Qc>\xE8\xB0q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R_\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x06G\x81\x85\x88a\x10\x1AV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x06e\x90\x86\x90a\x10\xAFV[\x15a\x06\x83W`@Qc\x06\xCFC\x8F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3\x80\x83R``\x80\x84\x01\x88\x81R`@\x80\x86\x01\x89\x81R_\x85\x81R`\x04` \x81\x81R\x91\x84\x90 \x89Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x82\x8A\x01Q`\x01\x82\x01U\x92Q\x80Q`\x02\x85\x01U\x82\x01Q`\x03\x84\x01U\x93Q\x80Q\x94\x83\x01\x94\x90\x94U\x83\x01Q`\x05\x82\x01U\x82\x82\x01Q`\x06\x82\x01U\x91\x90\x92\x01Q`\x07\x90\x91\x01UQ\x7F\xF6\xE85\x9CWR\x0BF\x964sk\xFC;\xB7\xEC\\\xBD\x1A\x0B\xD2\x8B\x10\xA8'W\x93\xBBs\x0By\x7F\x91a\x075\x91\x89\x90\x89\x90\x88\x90a\x1F\xB1V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[3_\x90\x81R`\x04` \x81\x81R`@\x80\x84 \x81Q`\x80\x80\x82\x01\x84R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R`\x01\x84\x01T\x83\x87\x01R\x84Q\x80\x86\x01\x86R`\x02\x85\x01T\x81R`\x03\x85\x01T\x81\x88\x01R\x83\x86\x01R\x84Q\x91\x82\x01\x85R\x95\x83\x01T\x81R`\x05\x83\x01T\x94\x81\x01\x94\x90\x94R`\x06\x82\x01T\x92\x84\x01\x92\x90\x92R`\x07\x01T``\x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x80Q\x90\x91\x16a\x07\xEBW`@Qb)\xEA\xAD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\x15W`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q_\x81\x90\x03a\x08BW`@Qc\xBA\xA1tO`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[\x80`\x05_\x82\x82Ta\x08S\x91\x90a \x14V[\x90\x91UPP3_\x90\x81R`\x04` \x81\x90R`@\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x83\x90U`\x02\x81\x01\x83\x90U`\x03\x81\x01\x83\x90U\x90\x81\x01\x82\x90U`\x05\x81\x01\x82\x90U`\x06\x80\x82\x01\x83\x90U`\x07\x90\x91\x01\x91\x90\x91UT\x82Qa\x08\xC0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x83a\x10\xCBV[\x92\x91PPV[_`d\x82` \x01Q\x11\x15a\x08\xDCWP`\n\x91\x90PV[P`\x05\x91\x90PV[`@\x80Q3\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R\x90\x81\x01\x82\x90R\x7FM\x10\xBD\x04\x97u\xC7{\xD7\xF2U\x19Z\xFB\xA5\x08\x80(\xEC\xB3\xC7\xC2w\xD3\x93\xCC\xFFy4\xF2\xF9,\x90``\x01`@Q\x80\x91\x03\x90\xA1PPV[3_\x90\x81R`\x04` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R`\x01\x84\x01T\x83\x86\x01R\x86Q\x80\x88\x01\x88R`\x02\x85\x01T\x81R`\x03\x85\x01T\x81\x87\x01R\x83\x88\x01R\x86Q\x91\x82\x01\x87R\x94\x83\x01T\x81R`\x05\x83\x01T\x93\x81\x01\x93\x90\x93R`\x06\x82\x01T\x94\x83\x01\x94\x90\x94R`\x07\x01T``\x82\x81\x01\x91\x90\x91R\x83\x01R\x81Q\x16a\t\xD3W`@Qb)\xEA\xAD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xE1\x84\x82``\x01Qa\x0F\x05V[\x80\x15a\t\xF8WP`@\x81\x01Qa\t\xF8\x90\x84\x90a\x10\xAFV[\x15a\n\x16W`@Qc\xE0\x12.3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x83\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90a\nR\x86\x83a\x0F\x05V[\x15a\npW`@Qc>\xE8\xB0q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\nz\x85\x82a\x10\xAFV[\x15a\n\x98W`@Qc\x06\xCFC\x8F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q3` \x82\x01R_\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\n\xC1\x81\x86\x89a\x10\x1AV[``\x84\x81\x01\x88\x81R`@\x80\x87\x01\x89\x81R3_\x81\x81R`\x04` \x81\x81R\x91\x85\x90 \x8BQ\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x82\x8C\x01Q`\x01\x82\x01U\x93Q\x80Q`\x02\x86\x01U\x80\x83\x01Q`\x03\x86\x01U\x95Q\x80Q\x91\x85\x01\x91\x90\x91U\x90\x81\x01Q`\x05\x84\x01U\x80\x84\x01Q`\x06\x84\x01U\x94\x85\x01Q`\x07\x90\x92\x01\x91\x90\x91U\x90Q\x7F\x80\xD8\xA4\xA1f3(\xA9\x98\xD4U[\xA2\x1D\x8B\xBAn\xF1Wj\x8C^\x9D'\xF9\xC5E\xF1\xA3\xD5+\x1D\x93a\x0Bq\x93\x90\x91a 'V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[3_\x90\x81R`\x04` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R`\x01\x84\x01T\x83\x86\x01R\x86Q\x80\x88\x01\x88R`\x02\x85\x01T\x81R`\x03\x85\x01T\x81\x87\x01R\x83\x88\x01R\x86Q\x91\x82\x01\x87R\x94\x83\x01T\x81R`\x05\x83\x01T\x93\x81\x01\x93\x90\x93R`\x06\x82\x01T\x94\x83\x01\x94\x90\x94R`\x07\x01T``\x82\x81\x01\x91\x90\x91R\x83\x01R\x81Q\x16a\x0C$W`@Qb)\xEA\xAD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0CNW`@Qc\xC8u\x9C\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q3\x81R\x7F\xFB$0ST\xC8wb\xD5WHz\xE4\xA5d\xE8\xD0>\xCB\xB9\xA9}\xD8\xAF\xFF\x8E\x1Fo\xCA\xF0\xDD\x16\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[a\x0C\x8Ca\x11NV[a\x0C\x95_a\x11{V[V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x81\x15\x80\x15a\x0C\xDCWP\x82[\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x0C\xF8WP0;\x15[\x90P\x81\x15\x80\x15a\r\x06WP\x80\x15[\x15a\r$W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\rNW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[C`\x03U\x83\x15a\r\x98W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\r\xDC\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@Qc\x1E\x04\x0C\xBF`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R_\x90\x81\x900\x90c\xF0 e\xF8\x90`$\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E@W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ed\x91\x90a {V[` \x01Q\x93\x92PPPV[a\x0Ewa\x1C)V[P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x04` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T\x90\x96\x16\x81R`\x01\x82\x01T\x81\x84\x01R\x84Q\x80\x86\x01\x86R`\x02\x83\x01T\x81R`\x03\x83\x01T\x81\x85\x01R\x81\x86\x01R\x84Q\x95\x86\x01\x85R\x92\x81\x01T\x85R`\x05\x81\x01T\x91\x85\x01\x91\x90\x91R`\x06\x81\x01T\x92\x84\x01\x92\x90\x92R`\x07\x90\x91\x01T``\x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[\x80Q\x82Q_\x91\x14\x80\x15a\x0F\x1FWP\x81` \x01Q\x83` \x01Q\x14[\x80\x15a\x0F2WP\x81`@\x01Q\x83`@\x01Q\x14[\x80\x15a\x0FEWP\x81``\x01Q\x83``\x01Q\x14[\x93\x92PPPV[a\x0FTa\x11NV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0F}W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x089V[a\x0F\x86\x81a\x11{V[PV[_`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R\x82`D\x82\x01R` _`d\x83_\x8AZ\xF1=\x15`\x1F=\x11`\x01_Q\x14\x16\x17\x16\x91PP\x80a\r\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x089V[a\x10#\x82a\x11\xCCV[_`@Q\x80``\x01`@R\x80`$\x81R` \x01a#\x87`$\x919\x90P_\x84\x82`@Q` \x01a\x10S\x92\x91\x90a!XV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P_a\x10n\x82a\x12gV[\x90Pa\x10\x8B\x81\x85a\x10~\x88a\x13TV[a\x10\x86a\x13\xCBV[a\x14\x98V[a\x10\xA7W`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[\x80Q\x82Q_\x91\x14\x80\x15a\x0FEWPP` \x90\x81\x01Q\x91\x01Q\x14\x90V[_`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` _`D\x83_\x89Z\xF1=\x15`\x1F=\x11`\x01_Q\x14\x16\x17\x16\x91PP\x80a\x11HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x089V[PPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\x95W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x089V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[\x80Q` \x82\x01Q_\x91_\x80Q` a#\xAB\x839\x81Q\x91R\x91\x15\x90\x15\x16\x15a\x11\xF2WPPPV[\x82Q` \x84\x01Q\x82`\x03\x84\x85\x85\x86\t\x85\t\x08\x83\x82\x83\t\x14\x83\x82\x10\x84\x84\x10\x16\x16\x93PPP\x81a\x12bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x089V[PPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_a\x12\x84\x83a\x15xV[\x90P_\x80Q` a#\xAB\x839\x81Q\x91R`\x03_\x82\x84\x85\t\x90P\x82\x80a\x12\xABWa\x12\xABa!lV[\x84\x82\t\x90P\x82\x80a\x12\xBEWa\x12\xBEa!lV[\x82\x82\x08\x90P_\x80a\x12\xCE\x83a\x17\x81V[\x92P\x90P[\x80a\x137W\x84\x80a\x12\xE6Wa\x12\xE6a!lV[`\x01\x87\x08\x95P\x84\x80a\x12\xFAWa\x12\xFAa!lV[\x86\x87\t\x92P\x84\x80a\x13\rWa\x13\ra!lV[\x86\x84\t\x92P\x84\x80a\x13 Wa\x13 a!lV[\x84\x84\x08\x92Pa\x13.\x83a\x17\x81V[\x92P\x90Pa\x12\xD3V[P`@\x80Q\x80\x82\x01\x90\x91R\x94\x85R` \x85\x01RP\x91\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x13{WP\x90V[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01_\x80Q` a#\xAB\x839\x81Q\x91R\x84` \x01Qa\x13\xAC\x91\x90a!\x80V[a\x13\xC3\x90_\x80Q` a#\xAB\x839\x81Q\x91Ra \x14V[\x90R\x92\x91PPV[a\x13\xF2`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81RP\x90P\x90V[_\x80_`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R` \x87\x01Q`@\x82\x01R\x86Q``\x82\x01R``\x87\x01Q`\x80\x82\x01R`@\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R` \x85\x01Qa\x01\0\x82\x01R\x84Qa\x01 \x82\x01R``\x85\x01Qa\x01@\x82\x01R`@\x85\x01Qa\x01`\x82\x01R` _a\x01\x80\x83`\x08Z\xFA\x91PP_Q\x91P\x80a\x15jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x089V[P\x15\x15\x90P[\x94\x93PPPPV[_\x80a\x15\x83\x83a\x18xV[\x80Q\x90\x91P`0\x81\x14a\x15\x98Wa\x15\x98a!\x9FV[_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xB2Wa\x15\xB2a\x1C\xD3V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x15\xDCW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a\x16KW\x83`\x01a\x15\xF6\x83\x86a \x14V[a\x16\0\x91\x90a \x14V[\x81Q\x81\x10a\x16\x10Wa\x16\x10a!\xB3V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x16-Wa\x16-a!\xB3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x15\xE1V[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R_\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P_[\x82\x81\x10\x15a\x16\xDBW\x83\x81a\x16\x87\x85\x88a \x14V[a\x16\x91\x91\x90a\x1F\x9EV[\x81Q\x81\x10a\x16\xA1Wa\x16\xA1a!\xB3V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x16\xC1Wa\x16\xC1a!\xB3V[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x16sV[P_a\x16\xE6\x82a\x1B\xC2V[\x90Pa\x01\0_\x80Q` a#\xAB\x839\x81Q\x91R_a\x17\x04\x86\x89a \x14V[\x90P_[\x81\x81\x10\x15a\x17qW_\x88`\x01a\x17\x1E\x84\x86a \x14V[a\x17(\x91\x90a \x14V[\x81Q\x81\x10a\x178Wa\x178a!\xB3V[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x17PWa\x17Pa!lV[\x85\x87\t\x95P\x83\x80a\x17cWa\x17ca!lV[\x81\x87\x08\x95PP`\x01\x01a\x17\x08V[P\x92\x9A\x99PPPPPPPPPPV[_\x80_\x80_\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P__\x80Q` a#\xAB\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x87``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` _`\xC0\x83`\x05Z\xFA\x94PP_Q\x92P\x83a\x18>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x089V[\x80`\x01\x84\x90\x1B\x11\x15a\x18WWa\x18T\x83\x82a \x14V[\x92P[\x80\x80a\x18eWa\x18ea!lV[\x83\x84\t\x96\x90\x96\x14\x96\x91\x95P\x90\x93PPPPV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90_\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x18\xB8\x92\x91\x90a!XV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x18\xDF\x92\x91\x90a!\xC7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x19\x01\x91\x90a!\xF1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x19+\x90\x83\x90\x83\x90` \x01a\"\tV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90_`\xFF\x88\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\x9BWa\x19\x9Ba\x1C\xD3V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x19\xC5W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_\x82`@Q` \x01a\x19\xDC\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P_[\x81Q\x81\x10\x15a\x1AEW\x81\x81\x81Q\x81\x10a\x1A\nWa\x1A\na!\xB3V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x1A'Wa\x1A'a!\xB3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x19\xEFV[P_\x84`@Q` \x01a\x1AZ\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R_\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x1A\xECW_\x83\x82\x81Q\x81\x10a\x1A\x93Wa\x1A\x93a!\xB3V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x1A\xB0Wa\x1A\xB0a!\xB3V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x1A\xD1\x92\x91\x90a\"-V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x98PP`\x01\x01a\x1AxV[P\x86\x88\x87`@Q` \x01a\x1B\x02\x93\x92\x91\x90a\"QV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x1B0\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P_[a\x1BP\x8A`\xFF\x8D\x16a \x14V[\x81\x10\x15a\x1B\xB1W\x82\x81\x81Q\x81\x10a\x1BiWa\x1Bia!\xB3V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x1B\x83\x83\x8Da\x1F\x9EV[\x81Q\x81\x10a\x1B\x93Wa\x1B\x93a!\xB3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x1BCV[P\x91\x9B\x9APPPPPPPPPPPV[_\x80\x80[\x83Q\x81\x10\x15a\x1C\"W\x83\x81\x81Q\x81\x10a\x1B\xE1Wa\x1B\xE1a!\xB3V[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x1B\xF9\x91\x90a\"\x84V[a\x1C\x04\x90`\x02a#{V[a\x1C\x0E\x91\x90a\"\x84V[a\x1C\x18\x90\x83a\x1F\x9EV[\x91P`\x01\x01a\x1B\xC6V[P\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01a\x1Cd`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x1C\x90`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x90R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\x86W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a\x1C\xBAW_\x80\xFD[\x825a\x1C\xC5\x81a\x1C\x95V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\x16WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\x16WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_`\x80\x82\x84\x03\x12\x15a\x1D[W_\x80\xFD[a\x1Dca\x1C\xE7V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R\x92\x91PPV[_`@\x82\x84\x03\x12\x15a\x1D\x9DW_\x80\xFD[a\x1D\xA5a\x1D\x1CV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[_\x80_\x80a\x01 \x85\x87\x03\x12\x15a\x1D\xCFW_\x80\xFD[a\x1D\xD9\x86\x86a\x1DKV[\x93Pa\x1D\xE8\x86`\x80\x87\x01a\x1D\x8DV[\x92Pa\x1D\xF7\x86`\xC0\x87\x01a\x1D\x8DV[\x91Pa\x01\0\x85\x015a\xFF\xFF\x81\x16\x81\x14a\x1E\x0EW_\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[_` \x82\x84\x03\x12\x15a\x1E)W_\x80\xFD[\x815a\x0FE\x81a\x1C\x95V[_a\x01\0\x82\x84\x03\x12\x15a\x1EEW_\x80\xFD[a\x1EMa\x1C\xE7V[\x825a\x1EX\x81a\x1C\x95V[\x81R` \x83\x81\x015\x90\x82\x01Ra\x1Eq\x84`@\x85\x01a\x1D\x8DV[`@\x82\x01Ra\x1E\x83\x84`\x80\x85\x01a\x1DKV[``\x82\x01R\x93\x92PPPV[_\x80_a\x01\0\x84\x86\x03\x12\x15a\x1E\xA2W_\x80\xFD[a\x1E\xAC\x85\x85a\x1DKV[\x92Pa\x1E\xBB\x85`\x80\x86\x01a\x1D\x8DV[\x91Pa\x1E\xCA\x85`\xC0\x86\x01a\x1D\x8DV[\x90P\x92P\x92P\x92V[_`\x80\x82\x84\x03\x12\x15a\x1E\xE3W_\x80\xFD[a\x0FE\x83\x83a\x1DKV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Qa\x01\0\x83\x01\x91a\x1F&\x90\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P``\x83\x81\x01Q\x80Q`\x80\x85\x01R` \x81\x01Q`\xA0\x85\x01R`@\x81\x01Q`\xC0\x85\x01R\x90\x81\x01Q`\xE0\x84\x01Ra\x1C\"V[_\x80a\x01\0\x83\x85\x03\x12\x15a\x1FhW_\x80\xFD[a\x1Fr\x84\x84a\x1DKV[\x91Pa\x1F\x81\x84`\x80\x85\x01a\x1DKV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x08\xC0Wa\x08\xC0a\x1F\x8AV[`\x01`\x01`\xA0\x1B\x03\x85\x16\x81Ra\x01\0\x81\x01a\x1F\xF0` \x83\x01\x86\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R``\x81\x01Q``\x83\x01RPPV[\x83Q`\xA0\x83\x01R` \x84\x01Q`\xC0\x83\x01Ra\xFF\xFF\x83\x16`\xE0\x83\x01R\x95\x94PPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x08\xC0Wa\x08\xC0a\x1F\x8AV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\xE0\x81\x01a e` \x83\x01\x85\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R``\x81\x01Q``\x83\x01RPPV[\x82Q`\xA0\x83\x01R` \x83\x01Q`\xC0\x83\x01Ra\x15pV[_\x81\x83\x03a\x01\0\x81\x12\x15a \x8DW_\x80\xFD[a \x95a\x1C\xE7V[\x83Qa \xA0\x81a\x1C\x95V[\x81R` \x84\x81\x01Q\x90\x82\x01R`@`?\x19\x83\x01\x12\x15a \xBDW_\x80\xFD[a \xC5a\x1D\x1CV[`@\x85\x81\x01Q\x82R``\x86\x01Q` \x83\x01R\x82\x01R`\x80`\x7F\x19\x83\x01\x12\x15a \xEBW_\x80\xFD[a \xF3a\x1C\xE7V[\x91P`\x80\x84\x01Q\x82R`\xA0\x84\x01Q` \x83\x01R`\xC0\x84\x01Q`@\x83\x01R`\xE0\x84\x01Q``\x83\x01R\x81``\x82\x01R\x80\x92PPP\x92\x91PPV[_\x81Q_[\x81\x81\x10\x15a!JW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a!0V[P_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x15pa!f\x83\x86a!+V[\x84a!+V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a!\x9AWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_a!\xD2\x82\x85a!+V[_\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[_a!\xFC\x82\x84a!+V[_\x81R`\x01\x01\x93\x92PPPV[_a\"\x14\x82\x85a!+V[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[_a\"8\x82\x85a!+V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[_a\"\\\x82\x86a!+V[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x08\xC0Wa\x08\xC0a\x1F\x8AV[`\x01\x81\x81[\x80\x85\x11\x15a\"\xD5W\x81_\x19\x04\x82\x11\x15a\"\xBBWa\"\xBBa\x1F\x8AV[\x80\x85\x16\x15a\"\xC8W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\"\xA0V[P\x92P\x92\x90PV[_\x82a\"\xEBWP`\x01a\x08\xC0V[\x81a\"\xF7WP_a\x08\xC0V[\x81`\x01\x81\x14a#\rW`\x02\x81\x14a#\x17Wa#3V[`\x01\x91PPa\x08\xC0V[`\xFF\x84\x11\x15a#(Wa#(a\x1F\x8AV[PP`\x01\x82\x1Ba\x08\xC0V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a#VWP\x81\x81\na\x08\xC0V[a#`\x83\x83a\"\x9BV[\x80_\x19\x04\x82\x11\x15a#sWa#sa\x1F\x8AV[\x02\x93\x92PPPV[_a\x0FE\x83\x83a\"\xDDV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static STAKETABLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct StakeTable<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StakeTable<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StakeTable<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StakeTable<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StakeTable<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StakeTable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StakeTable<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                STAKETABLE_ABI.clone(),
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
                STAKETABLE_ABI.clone(),
                STAKETABLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `_isEqualBlsKey` (0xf2f80a18) function
        pub fn is_equal_bls_key(
            &self,
            a: G2Point,
            b: G2Point,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([242, 248, 10, 24], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `admin` (0xf851a440) function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegate` (0x026e402b) function
        pub fn delegate(
            &self,
            validator: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 110, 64, 43], (validator, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregisterValidator` (0x6a911ccf) function
        pub fn deregister_validator(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 145, 28, 207], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `escrowPeriod` (0xf16b51c1) function
        pub fn escrow_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([241, 107, 81, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exitEscrowPeriod` (0x31c71ebf) function
        pub fn exit_escrow_period(
            &self,
            node: Node,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([49, 199, 30, 191], (node,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initializedAtBlock` (0x3e9df9b5) function
        pub fn initialized_at_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([62, 157, 249, 181], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lookupNode` (0xf02065f8) function
        pub fn lookup_node(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Node> {
            self.0
                .method_hash([240, 32, 101, 248], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lookupStake` (0xc315b6bd) function
        pub fn lookup_stake(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([195, 21, 182, 189], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nodes` (0x189a5a17) function
        pub fn nodes(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                EdOnBN254Point,
                G2Point,
            ),
        > {
            self.0
                .method_hash([24, 154, 90, 23], account)
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
        ///Calls the contract's `registerValidator` (0x13b9057a) function
        pub fn register_validator(
            &self,
            bls_vk: G2Point,
            schnorr_vk: EdOnBN254Point,
            bls_sig: G1Point,
            commission: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 185, 5, 122], (bls_vk, schnorr_vk, bls_sig, commission))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        ///Calls the contract's `totalStake` (0x8b0e9f3f) function
        pub fn total_stake(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
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
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `undelegate` (0x4d99dd16) function
        pub fn undelegate(
            &self,
            validator: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 153, 221, 22], (validator, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateConsensusKeys` (0x5544c2f1) function
        pub fn update_consensus_keys(
            &self,
            new_bls_vk: G2Point,
            new_schnorr_vk: EdOnBN254Point,
            new_bls_sig: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [85, 68, 194, 241],
                    (new_bls_vk, new_schnorr_vk, new_bls_sig),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawFunds` (0x24600fc3) function
        pub fn withdraw_funds(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([36, 96, 15, 195], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ConsensusKeysUpdated` event
        pub fn consensus_keys_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ConsensusKeysUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Delegated` event
        pub fn delegated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DelegatedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Undelegated` event
        pub fn undelegated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UndelegatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ValidatorExit` event
        pub fn validator_exit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ValidatorExitFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ValidatorRegistered` event
        pub fn validator_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ValidatorRegisteredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Withdrawal` event
        pub fn withdrawal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawalFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeTableEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for StakeTable<M> {
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
    ///Custom Error type `InsufficientAllowance` with signature `InsufficientAllowance(uint256,uint256)` and selector `0x2a1b2dd8`
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
        name = "InsufficientAllowance",
        abi = "InsufficientAllowance(uint256,uint256)"
    )]
    pub struct InsufficientAllowance(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Custom Error type `InsufficientBalance` with signature `InsufficientBalance(uint256)` and selector `0x92665351`
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
    #[etherror(name = "InsufficientBalance", abi = "InsufficientBalance(uint256)")]
    pub struct InsufficientBalance(pub ::ethers::core::types::U256);
    ///Custom Error type `InsufficientStakeAmount` with signature `InsufficientStakeAmount(uint256)` and selector `0x1d820b17`
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
        name = "InsufficientStakeAmount",
        abi = "InsufficientStakeAmount(uint256)"
    )]
    pub struct InsufficientStakeAmount(pub ::ethers::core::types::U256);
    ///Custom Error type `InsufficientStakeBalance` with signature `InsufficientStakeBalance(uint256)` and selector `0xbaa1744f`
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
        name = "InsufficientStakeBalance",
        abi = "InsufficientStakeBalance(uint256)"
    )]
    pub struct InsufficientStakeBalance(pub ::ethers::core::types::U256);
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
    ///Custom Error type `InvalidBlsVK` with signature `InvalidBlsVK()` and selector `0x3ee8b071`
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
    #[etherror(name = "InvalidBlsVK", abi = "InvalidBlsVK()")]
    pub struct InvalidBlsVK;
    ///Custom Error type `InvalidHotShotBlocksPerEpoch` with signature `InvalidHotShotBlocksPerEpoch()` and selector `0x0bc93345`
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
        name = "InvalidHotShotBlocksPerEpoch",
        abi = "InvalidHotShotBlocksPerEpoch()"
    )]
    pub struct InvalidHotShotBlocksPerEpoch;
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
    ///Custom Error type `InvalidSchnorrVK` with signature `InvalidSchnorrVK()` and selector `0x06cf438f`
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
    #[etherror(name = "InvalidSchnorrVK", abi = "InvalidSchnorrVK()")]
    pub struct InvalidSchnorrVK;
    ///Custom Error type `InvalidValue` with signature `InvalidValue()` and selector `0xaa7feadc`
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
    #[etherror(name = "InvalidValue", abi = "InvalidValue()")]
    pub struct InvalidValue;
    ///Custom Error type `NoKeyChange` with signature `NoKeyChange()` and selector `0xe0122e33`
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
    #[etherror(name = "NoKeyChange", abi = "NoKeyChange()")]
    pub struct NoKeyChange;
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
    ///Custom Error type `NodeNotRegistered` with signature `NodeNotRegistered()` and selector `0x014f5568`
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
    #[etherror(name = "NodeNotRegistered", abi = "NodeNotRegistered()")]
    pub struct NodeNotRegistered;
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
    ///Custom Error type `Unauthorized` with signature `Unauthorized()` and selector `0x82b42900`
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
    #[etherror(name = "Unauthorized", abi = "Unauthorized()")]
    pub struct Unauthorized;
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
    pub enum StakeTableErrors {
        BLSSigVerificationFailed(BLSSigVerificationFailed),
        ExitRequestInProgress(ExitRequestInProgress),
        InsufficientAllowance(InsufficientAllowance),
        InsufficientBalance(InsufficientBalance),
        InsufficientStakeAmount(InsufficientStakeAmount),
        InsufficientStakeBalance(InsufficientStakeBalance),
        InvalidAddress(InvalidAddress),
        InvalidBlsVK(InvalidBlsVK),
        InvalidHotShotBlocksPerEpoch(InvalidHotShotBlocksPerEpoch),
        InvalidInitialization(InvalidInitialization),
        InvalidNextRegistrationEpoch(InvalidNextRegistrationEpoch),
        InvalidSchnorrVK(InvalidSchnorrVK),
        InvalidValue(InvalidValue),
        NoKeyChange(NoKeyChange),
        NodeAlreadyRegistered(NodeAlreadyRegistered),
        NodeNotRegistered(NodeNotRegistered),
        NotInitializing(NotInitializing),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        PrematureDeposit(PrematureDeposit),
        PrematureExit(PrematureExit),
        PrematureWithdrawal(PrematureWithdrawal),
        RestakingNotImplemented(RestakingNotImplemented),
        Unauthenticated(Unauthenticated),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for StakeTableErrors {
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
                <InsufficientAllowance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientAllowance(decoded));
            }
            if let Ok(decoded) =
                <InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientBalance(decoded));
            }
            if let Ok(decoded) =
                <InsufficientStakeAmount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientStakeAmount(decoded));
            }
            if let Ok(decoded) =
                <InsufficientStakeBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientStakeBalance(decoded));
            }
            if let Ok(decoded) = <InvalidAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidAddress(decoded));
            }
            if let Ok(decoded) = <InvalidBlsVK as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidBlsVK(decoded));
            }
            if let Ok(decoded) =
                <InvalidHotShotBlocksPerEpoch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidHotShotBlocksPerEpoch(decoded));
            }
            if let Ok(decoded) =
                <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) =
                <InvalidNextRegistrationEpoch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidNextRegistrationEpoch(decoded));
            }
            if let Ok(decoded) = <InvalidSchnorrVK as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidSchnorrVK(decoded));
            }
            if let Ok(decoded) = <InvalidValue as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidValue(decoded));
            }
            if let Ok(decoded) = <NoKeyChange as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoKeyChange(decoded));
            }
            if let Ok(decoded) =
                <NodeAlreadyRegistered as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NodeAlreadyRegistered(decoded));
            }
            if let Ok(decoded) = <NodeNotRegistered as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NodeNotRegistered(decoded));
            }
            if let Ok(decoded) = <NotInitializing as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotInitializing(decoded));
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
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakeTableErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BLSSigVerificationFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExitRequestInProgress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientStakeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientStakeBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidBlsVK(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidHotShotBlocksPerEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidNextRegistrationEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSchnorrVK(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidValue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NoKeyChange(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NodeAlreadyRegistered(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NodeNotRegistered(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotInitializing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
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
                Self::Unauthorized(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for StakeTableErrors {
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
                    == <InsufficientAllowance as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InsufficientBalance as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InsufficientStakeAmount as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InsufficientStakeBalance as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <InvalidBlsVK as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidHotShotBlocksPerEpoch as ::ethers::contract::EthError>::selector(
                    ) =>
                {
                    true
                }
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InvalidNextRegistrationEpoch as ::ethers::contract::EthError>::selector(
                    ) =>
                {
                    true
                }
                _ if selector == <InvalidSchnorrVK as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <InvalidValue as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NoKeyChange as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NodeAlreadyRegistered as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <NodeNotRegistered as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <NotInitializing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() =>
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
                _ if selector == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for StakeTableErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BLSSigVerificationFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExitRequestInProgress(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientStakeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientStakeBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidBlsVK(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidHotShotBlocksPerEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidInitialization(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidNextRegistrationEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidSchnorrVK(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoKeyChange(element) => ::core::fmt::Display::fmt(element, f),
                Self::NodeAlreadyRegistered(element) => ::core::fmt::Display::fmt(element, f),
                Self::NodeNotRegistered(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotInitializing(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableInvalidOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableUnauthorizedAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrematureDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrematureExit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrematureWithdrawal(element) => ::core::fmt::Display::fmt(element, f),
                Self::RestakingNotImplemented(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthenticated(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for StakeTableErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BLSSigVerificationFailed> for StakeTableErrors {
        fn from(value: BLSSigVerificationFailed) -> Self {
            Self::BLSSigVerificationFailed(value)
        }
    }
    impl ::core::convert::From<ExitRequestInProgress> for StakeTableErrors {
        fn from(value: ExitRequestInProgress) -> Self {
            Self::ExitRequestInProgress(value)
        }
    }
    impl ::core::convert::From<InsufficientAllowance> for StakeTableErrors {
        fn from(value: InsufficientAllowance) -> Self {
            Self::InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<InsufficientBalance> for StakeTableErrors {
        fn from(value: InsufficientBalance) -> Self {
            Self::InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<InsufficientStakeAmount> for StakeTableErrors {
        fn from(value: InsufficientStakeAmount) -> Self {
            Self::InsufficientStakeAmount(value)
        }
    }
    impl ::core::convert::From<InsufficientStakeBalance> for StakeTableErrors {
        fn from(value: InsufficientStakeBalance) -> Self {
            Self::InsufficientStakeBalance(value)
        }
    }
    impl ::core::convert::From<InvalidAddress> for StakeTableErrors {
        fn from(value: InvalidAddress) -> Self {
            Self::InvalidAddress(value)
        }
    }
    impl ::core::convert::From<InvalidBlsVK> for StakeTableErrors {
        fn from(value: InvalidBlsVK) -> Self {
            Self::InvalidBlsVK(value)
        }
    }
    impl ::core::convert::From<InvalidHotShotBlocksPerEpoch> for StakeTableErrors {
        fn from(value: InvalidHotShotBlocksPerEpoch) -> Self {
            Self::InvalidHotShotBlocksPerEpoch(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for StakeTableErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<InvalidNextRegistrationEpoch> for StakeTableErrors {
        fn from(value: InvalidNextRegistrationEpoch) -> Self {
            Self::InvalidNextRegistrationEpoch(value)
        }
    }
    impl ::core::convert::From<InvalidSchnorrVK> for StakeTableErrors {
        fn from(value: InvalidSchnorrVK) -> Self {
            Self::InvalidSchnorrVK(value)
        }
    }
    impl ::core::convert::From<InvalidValue> for StakeTableErrors {
        fn from(value: InvalidValue) -> Self {
            Self::InvalidValue(value)
        }
    }
    impl ::core::convert::From<NoKeyChange> for StakeTableErrors {
        fn from(value: NoKeyChange) -> Self {
            Self::NoKeyChange(value)
        }
    }
    impl ::core::convert::From<NodeAlreadyRegistered> for StakeTableErrors {
        fn from(value: NodeAlreadyRegistered) -> Self {
            Self::NodeAlreadyRegistered(value)
        }
    }
    impl ::core::convert::From<NodeNotRegistered> for StakeTableErrors {
        fn from(value: NodeNotRegistered) -> Self {
            Self::NodeNotRegistered(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for StakeTableErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for StakeTableErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for StakeTableErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<PrematureDeposit> for StakeTableErrors {
        fn from(value: PrematureDeposit) -> Self {
            Self::PrematureDeposit(value)
        }
    }
    impl ::core::convert::From<PrematureExit> for StakeTableErrors {
        fn from(value: PrematureExit) -> Self {
            Self::PrematureExit(value)
        }
    }
    impl ::core::convert::From<PrematureWithdrawal> for StakeTableErrors {
        fn from(value: PrematureWithdrawal) -> Self {
            Self::PrematureWithdrawal(value)
        }
    }
    impl ::core::convert::From<RestakingNotImplemented> for StakeTableErrors {
        fn from(value: RestakingNotImplemented) -> Self {
            Self::RestakingNotImplemented(value)
        }
    }
    impl ::core::convert::From<Unauthenticated> for StakeTableErrors {
        fn from(value: Unauthenticated) -> Self {
            Self::Unauthenticated(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for StakeTableErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
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
    #[ethevent(
        name = "ConsensusKeysUpdated",
        abi = "ConsensusKeysUpdated(address,(uint256,uint256,uint256,uint256),(uint256,uint256))"
    )]
    pub struct ConsensusKeysUpdatedFilter {
        pub account: ::ethers::core::types::Address,
        pub bls_vk: G2Point,
        pub schnorr_vk: EdOnBN254Point,
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
    #[ethevent(name = "Delegated", abi = "Delegated(address,address,uint256)")]
    pub struct DelegatedFilter {
        pub delegator: ::ethers::core::types::Address,
        pub validator: ::ethers::core::types::Address,
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
    #[ethevent(name = "Undelegated", abi = "Undelegated(address,address,uint256)")]
    pub struct UndelegatedFilter {
        pub delegator: ::ethers::core::types::Address,
        pub validator: ::ethers::core::types::Address,
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
    #[ethevent(name = "ValidatorExit", abi = "ValidatorExit(address)")]
    pub struct ValidatorExitFilter {
        pub validator: ::ethers::core::types::Address,
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
        name = "ValidatorRegistered",
        abi = "ValidatorRegistered(address,(uint256,uint256,uint256,uint256),(uint256,uint256),uint16)"
    )]
    pub struct ValidatorRegisteredFilter {
        pub account: ::ethers::core::types::Address,
        pub bls_vk: G2Point,
        pub schnorr_vk: EdOnBN254Point,
        pub commission: u16,
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
    #[ethevent(name = "Withdrawal", abi = "Withdrawal(address,uint256)")]
    pub struct WithdrawalFilter {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    pub enum StakeTableEvents {
        ConsensusKeysUpdatedFilter(ConsensusKeysUpdatedFilter),
        DelegatedFilter(DelegatedFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        UndelegatedFilter(UndelegatedFilter),
        ValidatorExitFilter(ValidatorExitFilter),
        ValidatorRegisteredFilter(ValidatorRegisteredFilter),
        WithdrawalFilter(WithdrawalFilter),
    }
    impl ::ethers::contract::EthLogDecode for StakeTableEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ConsensusKeysUpdatedFilter::decode_log(log) {
                return Ok(StakeTableEvents::ConsensusKeysUpdatedFilter(decoded));
            }
            if let Ok(decoded) = DelegatedFilter::decode_log(log) {
                return Ok(StakeTableEvents::DelegatedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(StakeTableEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(StakeTableEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = UndelegatedFilter::decode_log(log) {
                return Ok(StakeTableEvents::UndelegatedFilter(decoded));
            }
            if let Ok(decoded) = ValidatorExitFilter::decode_log(log) {
                return Ok(StakeTableEvents::ValidatorExitFilter(decoded));
            }
            if let Ok(decoded) = ValidatorRegisteredFilter::decode_log(log) {
                return Ok(StakeTableEvents::ValidatorRegisteredFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalFilter::decode_log(log) {
                return Ok(StakeTableEvents::WithdrawalFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StakeTableEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConsensusKeysUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UndelegatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidatorExitFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidatorRegisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ConsensusKeysUpdatedFilter> for StakeTableEvents {
        fn from(value: ConsensusKeysUpdatedFilter) -> Self {
            Self::ConsensusKeysUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DelegatedFilter> for StakeTableEvents {
        fn from(value: DelegatedFilter) -> Self {
            Self::DelegatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for StakeTableEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for StakeTableEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<UndelegatedFilter> for StakeTableEvents {
        fn from(value: UndelegatedFilter) -> Self {
            Self::UndelegatedFilter(value)
        }
    }
    impl ::core::convert::From<ValidatorExitFilter> for StakeTableEvents {
        fn from(value: ValidatorExitFilter) -> Self {
            Self::ValidatorExitFilter(value)
        }
    }
    impl ::core::convert::From<ValidatorRegisteredFilter> for StakeTableEvents {
        fn from(value: ValidatorRegisteredFilter) -> Self {
            Self::ValidatorRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalFilter> for StakeTableEvents {
        fn from(value: WithdrawalFilter) -> Self {
            Self::WithdrawalFilter(value)
        }
    }
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
    ///Container type for all input parameters for the `_isEqualBlsKey` function with signature `_isEqualBlsKey((uint256,uint256,uint256,uint256),(uint256,uint256,uint256,uint256))` and selector `0xf2f80a18`
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
        name = "_isEqualBlsKey",
        abi = "_isEqualBlsKey((uint256,uint256,uint256,uint256),(uint256,uint256,uint256,uint256))"
    )]
    pub struct IsEqualBlsKeyCall {
        pub a: G2Point,
        pub b: G2Point,
    }
    ///Container type for all input parameters for the `admin` function with signature `admin()` and selector `0xf851a440`
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
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    ///Container type for all input parameters for the `delegate` function with signature `delegate(address,uint256)` and selector `0x026e402b`
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
    #[ethcall(name = "delegate", abi = "delegate(address,uint256)")]
    pub struct DelegateCall {
        pub validator: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deregisterValidator` function with signature `deregisterValidator()` and selector `0x6a911ccf`
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
    #[ethcall(name = "deregisterValidator", abi = "deregisterValidator()")]
    pub struct DeregisterValidatorCall;
    ///Container type for all input parameters for the `escrowPeriod` function with signature `escrowPeriod()` and selector `0xf16b51c1`
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
    #[ethcall(name = "escrowPeriod", abi = "escrowPeriod()")]
    pub struct EscrowPeriodCall;
    ///Container type for all input parameters for the `exitEscrowPeriod` function with signature `exitEscrowPeriod((address,uint256,(uint256,uint256),(uint256,uint256,uint256,uint256)))` and selector `0x31c71ebf`
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
        abi = "exitEscrowPeriod((address,uint256,(uint256,uint256),(uint256,uint256,uint256,uint256)))"
    )]
    pub struct ExitEscrowPeriodCall {
        pub node: Node,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize()` and selector `0x8129fc1c`
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
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
    ///Container type for all input parameters for the `initializedAtBlock` function with signature `initializedAtBlock()` and selector `0x3e9df9b5`
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
    #[ethcall(name = "initializedAtBlock", abi = "initializedAtBlock()")]
    pub struct InitializedAtBlockCall;
    ///Container type for all input parameters for the `lookupNode` function with signature `lookupNode(address)` and selector `0xf02065f8`
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
    #[ethcall(name = "lookupNode", abi = "lookupNode(address)")]
    pub struct LookupNodeCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lookupStake` function with signature `lookupStake(address)` and selector `0xc315b6bd`
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
    #[ethcall(name = "lookupStake", abi = "lookupStake(address)")]
    pub struct LookupStakeCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `nodes` function with signature `nodes(address)` and selector `0x189a5a17`
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
    #[ethcall(name = "nodes", abi = "nodes(address)")]
    pub struct NodesCall {
        pub account: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `registerValidator` function with signature `registerValidator((uint256,uint256,uint256,uint256),(uint256,uint256),(uint256,uint256),uint16)` and selector `0x13b9057a`
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
        name = "registerValidator",
        abi = "registerValidator((uint256,uint256,uint256,uint256),(uint256,uint256),(uint256,uint256),uint16)"
    )]
    pub struct RegisterValidatorCall {
        pub bls_vk: G2Point,
        pub schnorr_vk: EdOnBN254Point,
        pub bls_sig: G1Point,
        pub commission: u16,
    }
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
    ///Container type for all input parameters for the `undelegate` function with signature `undelegate(address,uint256)` and selector `0x4d99dd16`
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
    #[ethcall(name = "undelegate", abi = "undelegate(address,uint256)")]
    pub struct UndelegateCall {
        pub validator: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateConsensusKeys` function with signature `updateConsensusKeys((uint256,uint256,uint256,uint256),(uint256,uint256),(uint256,uint256))` and selector `0x5544c2f1`
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
        name = "updateConsensusKeys",
        abi = "updateConsensusKeys((uint256,uint256,uint256,uint256),(uint256,uint256),(uint256,uint256))"
    )]
    pub struct UpdateConsensusKeysCall {
        pub new_bls_vk: G2Point,
        pub new_schnorr_vk: EdOnBN254Point,
        pub new_bls_sig: G1Point,
    }
    ///Container type for all input parameters for the `withdrawFunds` function with signature `withdrawFunds()` and selector `0x24600fc3`
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
    #[ethcall(name = "withdrawFunds", abi = "withdrawFunds()")]
    pub struct WithdrawFundsCall;
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
    pub enum StakeTableCalls {
        HashBlsKey(HashBlsKeyCall),
        IsEqualBlsKey(IsEqualBlsKeyCall),
        Admin(AdminCall),
        Delegate(DelegateCall),
        DeregisterValidator(DeregisterValidatorCall),
        EscrowPeriod(EscrowPeriodCall),
        ExitEscrowPeriod(ExitEscrowPeriodCall),
        Initialize(InitializeCall),
        InitializedAtBlock(InitializedAtBlockCall),
        LookupNode(LookupNodeCall),
        LookupStake(LookupStakeCall),
        Nodes(NodesCall),
        Owner(OwnerCall),
        RegisterValidator(RegisterValidatorCall),
        RenounceOwnership(RenounceOwnershipCall),
        TokenAddress(TokenAddressCall),
        TotalKeys(TotalKeysCall),
        TotalStake(TotalStakeCall),
        TotalVotingStake(TotalVotingStakeCall),
        TransferOwnership(TransferOwnershipCall),
        Undelegate(UndelegateCall),
        UpdateConsensusKeys(UpdateConsensusKeysCall),
        WithdrawFunds(WithdrawFundsCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakeTableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <HashBlsKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HashBlsKey(decoded));
            }
            if let Ok(decoded) = <IsEqualBlsKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsEqualBlsKey(decoded));
            }
            if let Ok(decoded) = <AdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded) = <DelegateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delegate(decoded));
            }
            if let Ok(decoded) =
                <DeregisterValidatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeregisterValidator(decoded));
            }
            if let Ok(decoded) = <EscrowPeriodCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EscrowPeriod(decoded));
            }
            if let Ok(decoded) =
                <ExitEscrowPeriodCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExitEscrowPeriod(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InitializedAtBlockCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InitializedAtBlock(decoded));
            }
            if let Ok(decoded) = <LookupNodeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LookupNode(decoded));
            }
            if let Ok(decoded) = <LookupStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LookupStake(decoded));
            }
            if let Ok(decoded) = <NodesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nodes(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <RegisterValidatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterValidator(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <TokenAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TokenAddress(decoded));
            }
            if let Ok(decoded) = <TotalKeysCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalKeys(decoded));
            }
            if let Ok(decoded) = <TotalStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalStake(decoded));
            }
            if let Ok(decoded) =
                <TotalVotingStakeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalVotingStake(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UndelegateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Undelegate(decoded));
            }
            if let Ok(decoded) =
                <UpdateConsensusKeysCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateConsensusKeys(decoded));
            }
            if let Ok(decoded) = <WithdrawFundsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawFunds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakeTableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::HashBlsKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsEqualBlsKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Delegate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeregisterValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EscrowPeriod(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExitEscrowPeriod(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitializedAtBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LookupNode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LookupStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nodes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterValidator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalKeys(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalVotingStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Undelegate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateConsensusKeys(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawFunds(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for StakeTableCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::HashBlsKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsEqualBlsKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegate(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterValidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::EscrowPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExitEscrowPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedAtBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::LookupNode(element) => ::core::fmt::Display::fmt(element, f),
                Self::LookupStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nodes(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterValidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalKeys(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalVotingStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Undelegate(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateConsensusKeys(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFunds(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<HashBlsKeyCall> for StakeTableCalls {
        fn from(value: HashBlsKeyCall) -> Self {
            Self::HashBlsKey(value)
        }
    }
    impl ::core::convert::From<IsEqualBlsKeyCall> for StakeTableCalls {
        fn from(value: IsEqualBlsKeyCall) -> Self {
            Self::IsEqualBlsKey(value)
        }
    }
    impl ::core::convert::From<AdminCall> for StakeTableCalls {
        fn from(value: AdminCall) -> Self {
            Self::Admin(value)
        }
    }
    impl ::core::convert::From<DelegateCall> for StakeTableCalls {
        fn from(value: DelegateCall) -> Self {
            Self::Delegate(value)
        }
    }
    impl ::core::convert::From<DeregisterValidatorCall> for StakeTableCalls {
        fn from(value: DeregisterValidatorCall) -> Self {
            Self::DeregisterValidator(value)
        }
    }
    impl ::core::convert::From<EscrowPeriodCall> for StakeTableCalls {
        fn from(value: EscrowPeriodCall) -> Self {
            Self::EscrowPeriod(value)
        }
    }
    impl ::core::convert::From<ExitEscrowPeriodCall> for StakeTableCalls {
        fn from(value: ExitEscrowPeriodCall) -> Self {
            Self::ExitEscrowPeriod(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for StakeTableCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InitializedAtBlockCall> for StakeTableCalls {
        fn from(value: InitializedAtBlockCall) -> Self {
            Self::InitializedAtBlock(value)
        }
    }
    impl ::core::convert::From<LookupNodeCall> for StakeTableCalls {
        fn from(value: LookupNodeCall) -> Self {
            Self::LookupNode(value)
        }
    }
    impl ::core::convert::From<LookupStakeCall> for StakeTableCalls {
        fn from(value: LookupStakeCall) -> Self {
            Self::LookupStake(value)
        }
    }
    impl ::core::convert::From<NodesCall> for StakeTableCalls {
        fn from(value: NodesCall) -> Self {
            Self::Nodes(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for StakeTableCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RegisterValidatorCall> for StakeTableCalls {
        fn from(value: RegisterValidatorCall) -> Self {
            Self::RegisterValidator(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for StakeTableCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TokenAddressCall> for StakeTableCalls {
        fn from(value: TokenAddressCall) -> Self {
            Self::TokenAddress(value)
        }
    }
    impl ::core::convert::From<TotalKeysCall> for StakeTableCalls {
        fn from(value: TotalKeysCall) -> Self {
            Self::TotalKeys(value)
        }
    }
    impl ::core::convert::From<TotalStakeCall> for StakeTableCalls {
        fn from(value: TotalStakeCall) -> Self {
            Self::TotalStake(value)
        }
    }
    impl ::core::convert::From<TotalVotingStakeCall> for StakeTableCalls {
        fn from(value: TotalVotingStakeCall) -> Self {
            Self::TotalVotingStake(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for StakeTableCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UndelegateCall> for StakeTableCalls {
        fn from(value: UndelegateCall) -> Self {
            Self::Undelegate(value)
        }
    }
    impl ::core::convert::From<UpdateConsensusKeysCall> for StakeTableCalls {
        fn from(value: UpdateConsensusKeysCall) -> Self {
            Self::UpdateConsensusKeys(value)
        }
    }
    impl ::core::convert::From<WithdrawFundsCall> for StakeTableCalls {
        fn from(value: WithdrawFundsCall) -> Self {
            Self::WithdrawFunds(value)
        }
    }
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
    ///Container type for all return fields from the `_isEqualBlsKey` function with signature `_isEqualBlsKey((uint256,uint256,uint256,uint256),(uint256,uint256,uint256,uint256))` and selector `0xf2f80a18`
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
    pub struct IsEqualBlsKeyReturn(pub bool);
    ///Container type for all return fields from the `admin` function with signature `admin()` and selector `0xf851a440`
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
    pub struct AdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `escrowPeriod` function with signature `escrowPeriod()` and selector `0xf16b51c1`
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
    pub struct EscrowPeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `exitEscrowPeriod` function with signature `exitEscrowPeriod((address,uint256,(uint256,uint256),(uint256,uint256,uint256,uint256)))` and selector `0x31c71ebf`
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
    ///Container type for all return fields from the `initializedAtBlock` function with signature `initializedAtBlock()` and selector `0x3e9df9b5`
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
    pub struct InitializedAtBlockReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lookupNode` function with signature `lookupNode(address)` and selector `0xf02065f8`
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
    ///Container type for all return fields from the `lookupStake` function with signature `lookupStake(address)` and selector `0xc315b6bd`
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
    pub struct LookupStakeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nodes` function with signature `nodes(address)` and selector `0x189a5a17`
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
        pub balance: ::ethers::core::types::U256,
        pub schnorr_vk: EdOnBN254Point,
        pub bls_vk: G2Point,
    }
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
    pub struct TotalStakeReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `withdrawFunds` function with signature `withdrawFunds()` and selector `0x24600fc3`
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
    pub struct WithdrawFundsReturn(pub ::ethers::core::types::U256);
    ///`Node(address,uint256,(uint256,uint256),(uint256,uint256,uint256,uint256))`
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
    pub struct Node {
        pub account: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
        pub schnorr_vk: EdOnBN254Point,
        pub bls_vk: G2Point,
    }
}
