pub use permissioned_stake_table::*;
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
pub mod permissioned_stake_table {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialOwner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialStakers"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Tuple(
                                ::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                ],
                            ),),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "struct PermissionedStakeTable.NodeInfo[]",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("insert"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("insert"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newStakers"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct PermissionedStakeTable.NodeInfo[]",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isStaker"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isStaker"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("staker"),
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
                    ::std::borrow::ToOwned::to_owned("remove"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("remove"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("stakersToRemove"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct PermissionedStakeTable.NodeInfo[]",
                                ),
                            ),
                        },],
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Added"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Added"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ],),
                                ),
                            ),
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
                    ::std::borrow::ToOwned::to_owned("Removed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Removed"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ],),
                                ),
                            ),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("StakerAlreadyExists"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("StakerAlreadyExists",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
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
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakerNotFound"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("StakerNotFound"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
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
                        },],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PERMISSIONEDSTAKETABLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\r\x108\x03\x80b\0\r\x10\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x03\x81V[\x81`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\0eW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[b\0\0p\x81b\0\0\x84V[Pb\0\0|\x81b\0\0\xD4V[PPb\0\x05\x7FV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[b\0\0\xDEb\0\x01\xF2V[`\0[\x81Q\x81\x10\x15b\0\x01\xB5W`\0b\0\x01\x1E\x83\x83\x81Q\x81\x10b\0\x01\x06Wb\0\x01\x06b\0\x04\xD7V[` \x02` \x01\x01Q`\0\x01Qb\0\x02#` \x1B` \x1CV[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91P`\xFF\x16\x15b\0\x01\x93W\x82\x82\x81Q\x81\x10b\0\x01NWb\0\x01Nb\0\x04\xD7V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@\x80Qc\x1B\x06\xE1A`\xE1\x1B\x81R\x82Q`\x04\x82\x01R\x92\x82\x01Q`$\x84\x01R\x81\x01Q`D\x83\x01R``\x01Q`d\x82\x01R`\x84\x01b\0\0\\V[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x82\x17\x90U\x01b\0\0\xE1V[P\x7FN\x90\xAE!_1`\x99\xC2\xB6gD%.\x9F\x07\x17\xC7\x9E)\x8B\xA0^#&\x0F\xFAiW\x85T\xF5\x81`@Qb\0\x01\xE7\x91\x90b\0\x04\xEDV[`@Q\x80\x91\x03\x90\xA1PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x02!W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01b\0\0\\V[V[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01b\0\x02c\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x02\xBBWb\0\x02\xBBb\0\x02\x80V[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x02\xBBWb\0\x02\xBBb\0\x02\x80V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x03\x11Wb\0\x03\x11b\0\x02\x80V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15b\0\x03,W`\0\x80\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x03QWb\0\x03Qb\0\x02\x80V[`@R\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[\x80Q\x80\x15\x15\x81\x14b\0\x03|W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x80\x84\x86\x03\x12\x15b\0\x03\x96W`\0\x80\xFD[\x83Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x03\xAEW`\0\x80\xFD[` \x85\x81\x01Q\x91\x94P\x90`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x03\xCEW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12b\0\x03\xE3W`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x03\xF8Wb\0\x03\xF8b\0\x02\x80V[b\0\x04\x08\x84\x82`\x05\x1B\x01b\0\x02\xE6V[\x81\x81R\x84\x81\x01\x92P`\xE0\x91\x82\x02\x84\x01\x85\x01\x91\x8A\x83\x11\x15b\0\x04(W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15b\0\x04\xC6W\x84\x8B\x03\x81\x81\x12\x15b\0\x04HW`\0\x80\x81\xFD[b\0\x04Rb\0\x02\x96V[`\x80\x80\x83\x12\x15b\0\x04cW`\0\x80\x81\xFD[b\0\x04mb\0\x02\xC1V[\x88Q\x81R\x89\x89\x01Q\x8A\x82\x01R\x8A\x89\x01Q\x8B\x82\x01R``\x80\x8A\x01Q\x90\x82\x01R\x80\x83R\x92Pb\0\x04\x9E\x8E\x82\x8A\x01b\0\x03\x19V[\x89\x83\x01RPb\0\x04\xB1`\xC0\x88\x01b\0\x03kV[\x81\x8A\x01R\x85RP\x93\x84\x01\x93\x92\x85\x01\x92b\0\x04-V[P\x80\x96PPPPPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15b\0\x05rW\x81Qb\0\x05B\x85\x82Q\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R``\x81\x01Q``\x83\x01RPPV[\x80\x87\x01Q\x80Q`\x80\x87\x01R\x87\x01Q`\xA0\x86\x01R\x85\x01Q\x15\x15`\xC0\x85\x01R`\xE0\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01b\0\x05\nV[P\x91\x97\x96PPPPPPPV[a\x07\x81\x80b\0\x05\x8F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xC7W\x80c\x9B0\xA5\xE6\x14a\0\xE2W\x80c\xCD\x1D\xA5\xE5\x14a\x01\x03W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x16W`\0\x80\xFD[\x80cQ\x82`4\x14a\0\x82W\x80cqP\x18\xA6\x14a\0\x97W\x80cu\xD7\x05\xE9\x14a\0\x9FW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x05lV[a\x01)V[\0[a\0\x95a\x023V[a\0\xB2a\0\xAD6`\x04a\x06\x83V[a\x02GV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xBEV[a\0\xF5a\0\xF06`\x04a\x06\x83V[a\x02pV[`@Q\x90\x81R` \x01a\0\xBEV[a\0\x95a\x01\x116`\x04a\x05lV[a\x02\xCCV[a\0\x95a\x01$6`\x04a\x06\xA6V[a\x03\xB8V[a\x011a\x03\xF6V[`\0[\x81Q\x81\x10\x15a\x01\xF8W`\0a\x01e\x83\x83\x81Q\x81\x10a\x01TWa\x01Ta\x06\xCFV[` \x02` \x01\x01Q`\0\x01Qa\x02pV[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91P`\xFF\x16a\x01\xD9W\x82\x82\x81Q\x81\x10a\x01\x90Wa\x01\x90a\x06\xCFV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@\x80Qc4\xA7V\x1F`\xE0\x1B\x81R\x82Q`\x04\x82\x01R\x92\x82\x01Q`$\x84\x01R\x81\x01Q`D\x83\x01R``\x01Q`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90U\x01a\x014V[P\x7F\xD4\xBD\x18%\xB9a\xF5\xC0\xFF\xB5\xCB\xA8\x15\x04\x14\x0F\xA9\x9D\x99/V\xD08\xA0Y\xD0{qa\xFC\x16\xFD\x81`@Qa\x02(\x91\x90a\x06\xE5V[`@Q\x80\x91\x03\x90\xA1PV[a\x02;a\x03\xF6V[a\x02E`\0a\x04#V[V[`\0`\x01`\0a\x02V\x84a\x02pV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x92\x91PPV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x02\xAF\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[a\x02\xD4a\x03\xF6V[`\0[\x81Q\x81\x10\x15a\x03\x88W`\0a\x02\xF7\x83\x83\x81Q\x81\x10a\x01TWa\x01Ta\x06\xCFV[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x03gW\x82\x82\x81Q\x81\x10a\x03#Wa\x03#a\x06\xCFV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@\x80Qc\x1B\x06\xE1A`\xE1\x1B\x81R\x82Q`\x04\x82\x01R\x92\x82\x01Q`$\x84\x01R\x81\x01Q`D\x83\x01R``\x01Q`d\x82\x01R`\x84\x01a\x01\xD0V[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x82\x17\x90U\x01a\x02\xD7V[P\x7FN\x90\xAE!_1`\x99\xC2\xB6gD%.\x9F\x07\x17\xC7\x9E)\x8B\xA0^#&\x0F\xFAiW\x85T\xF5\x81`@Qa\x02(\x91\x90a\x06\xE5V[a\x03\xC0a\x03\xF6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xEAW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x01\xD0V[a\x03\xF3\x81a\x04#V[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02EW`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x01\xD0V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xACWa\x04\xACa\x04sV[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xACWa\x04\xACa\x04sV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xFEWa\x04\xFEa\x04sV[`@R\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x05\x18W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x05;Wa\x05;a\x04sV[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15a\x05\x7FW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\x97W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x05\xABW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\xBDWa\x05\xBDa\x04sV[a\x05\xCB\x84\x82`\x05\x1B\x01a\x04\xD5V[\x81\x81R\x84\x81\x01\x92P`\xE0\x91\x82\x02\x84\x01\x85\x01\x91\x88\x83\x11\x15a\x05\xEAW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x06wW\x84\x89\x03\x81\x81\x12\x15a\x06\x08W`\0\x80\x81\xFD[a\x06\x10a\x04\x89V[a\x06\x1A\x8B\x88a\x05\x06V[\x81R`@\x80`\x7F\x19\x84\x01\x12\x15a\x060W`\0\x80\x81\xFD[a\x068a\x04\xB2V[`\x80\x89\x015\x81R`\xA0\x89\x015\x8A\x82\x01R\x82\x8A\x01R`\xC0\x88\x015\x92P\x82\x15\x15\x83\x14a\x06bW`\0\x80\x81\xFD[\x81\x01\x91\x90\x91R\x84R\x93\x84\x01\x93\x92\x85\x01\x92a\x05\xEFV[P\x97\x96PPPPPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x06\x95W`\0\x80\xFD[a\x06\x9F\x83\x83a\x05\x06V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\xB8W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\x9FW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x07gW\x81Qa\x078\x85\x82Q\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R``\x81\x01Q``\x83\x01RPPV[\x80\x87\x01Q\x80Q`\x80\x87\x01R\x87\x01Q`\xA0\x86\x01R\x85\x01Q\x15\x15`\xC0\x85\x01R`\xE0\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a\x07\x02V[P\x91\x97\x96PPPPPPPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static PERMISSIONEDSTAKETABLE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xC7W\x80c\x9B0\xA5\xE6\x14a\0\xE2W\x80c\xCD\x1D\xA5\xE5\x14a\x01\x03W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x16W`\0\x80\xFD[\x80cQ\x82`4\x14a\0\x82W\x80cqP\x18\xA6\x14a\0\x97W\x80cu\xD7\x05\xE9\x14a\0\x9FW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x05lV[a\x01)V[\0[a\0\x95a\x023V[a\0\xB2a\0\xAD6`\x04a\x06\x83V[a\x02GV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xBEV[a\0\xF5a\0\xF06`\x04a\x06\x83V[a\x02pV[`@Q\x90\x81R` \x01a\0\xBEV[a\0\x95a\x01\x116`\x04a\x05lV[a\x02\xCCV[a\0\x95a\x01$6`\x04a\x06\xA6V[a\x03\xB8V[a\x011a\x03\xF6V[`\0[\x81Q\x81\x10\x15a\x01\xF8W`\0a\x01e\x83\x83\x81Q\x81\x10a\x01TWa\x01Ta\x06\xCFV[` \x02` \x01\x01Q`\0\x01Qa\x02pV[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91P`\xFF\x16a\x01\xD9W\x82\x82\x81Q\x81\x10a\x01\x90Wa\x01\x90a\x06\xCFV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@\x80Qc4\xA7V\x1F`\xE0\x1B\x81R\x82Q`\x04\x82\x01R\x92\x82\x01Q`$\x84\x01R\x81\x01Q`D\x83\x01R``\x01Q`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90U\x01a\x014V[P\x7F\xD4\xBD\x18%\xB9a\xF5\xC0\xFF\xB5\xCB\xA8\x15\x04\x14\x0F\xA9\x9D\x99/V\xD08\xA0Y\xD0{qa\xFC\x16\xFD\x81`@Qa\x02(\x91\x90a\x06\xE5V[`@Q\x80\x91\x03\x90\xA1PV[a\x02;a\x03\xF6V[a\x02E`\0a\x04#V[V[`\0`\x01`\0a\x02V\x84a\x02pV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x92\x91PPV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x02\xAF\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[a\x02\xD4a\x03\xF6V[`\0[\x81Q\x81\x10\x15a\x03\x88W`\0a\x02\xF7\x83\x83\x81Q\x81\x10a\x01TWa\x01Ta\x06\xCFV[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x03gW\x82\x82\x81Q\x81\x10a\x03#Wa\x03#a\x06\xCFV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@\x80Qc\x1B\x06\xE1A`\xE1\x1B\x81R\x82Q`\x04\x82\x01R\x92\x82\x01Q`$\x84\x01R\x81\x01Q`D\x83\x01R``\x01Q`d\x82\x01R`\x84\x01a\x01\xD0V[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x82\x17\x90U\x01a\x02\xD7V[P\x7FN\x90\xAE!_1`\x99\xC2\xB6gD%.\x9F\x07\x17\xC7\x9E)\x8B\xA0^#&\x0F\xFAiW\x85T\xF5\x81`@Qa\x02(\x91\x90a\x06\xE5V[a\x03\xC0a\x03\xF6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xEAW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x01\xD0V[a\x03\xF3\x81a\x04#V[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02EW`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x01\xD0V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xACWa\x04\xACa\x04sV[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xACWa\x04\xACa\x04sV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xFEWa\x04\xFEa\x04sV[`@R\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x05\x18W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x05;Wa\x05;a\x04sV[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15a\x05\x7FW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\x97W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x05\xABW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\xBDWa\x05\xBDa\x04sV[a\x05\xCB\x84\x82`\x05\x1B\x01a\x04\xD5V[\x81\x81R\x84\x81\x01\x92P`\xE0\x91\x82\x02\x84\x01\x85\x01\x91\x88\x83\x11\x15a\x05\xEAW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x06wW\x84\x89\x03\x81\x81\x12\x15a\x06\x08W`\0\x80\x81\xFD[a\x06\x10a\x04\x89V[a\x06\x1A\x8B\x88a\x05\x06V[\x81R`@\x80`\x7F\x19\x84\x01\x12\x15a\x060W`\0\x80\x81\xFD[a\x068a\x04\xB2V[`\x80\x89\x015\x81R`\xA0\x89\x015\x8A\x82\x01R\x82\x8A\x01R`\xC0\x88\x015\x92P\x82\x15\x15\x83\x14a\x06bW`\0\x80\x81\xFD[\x81\x01\x91\x90\x91R\x84R\x93\x84\x01\x93\x92\x85\x01\x92a\x05\xEFV[P\x97\x96PPPPPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x06\x95W`\0\x80\xFD[a\x06\x9F\x83\x83a\x05\x06V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\xB8W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\x9FW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x07gW\x81Qa\x078\x85\x82Q\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R``\x81\x01Q``\x83\x01RPPV[\x80\x87\x01Q\x80Q`\x80\x87\x01R\x87\x01Q`\xA0\x86\x01R\x85\x01Q\x15\x15`\xC0\x85\x01R`\xE0\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a\x07\x02V[P\x91\x97\x96PPPPPPPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static PERMISSIONEDSTAKETABLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct PermissionedStakeTable<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PermissionedStakeTable<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PermissionedStakeTable<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PermissionedStakeTable<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PermissionedStakeTable<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PermissionedStakeTable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PermissionedStakeTable<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PERMISSIONEDSTAKETABLE_ABI.clone(),
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
                PERMISSIONEDSTAKETABLE_ABI.clone(),
                PERMISSIONEDSTAKETABLE_BYTECODE.clone().into(),
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
        ///Calls the contract's `insert` (0xcd1da5e5) function
        pub fn insert(
            &self,
            new_stakers: ::std::vec::Vec<NodeInfo>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 29, 165, 229], new_stakers)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isStaker` (0x75d705e9) function
        pub fn is_staker(
            &self,
            staker: G2Point,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([117, 215, 5, 233], (staker,))
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
        ///Calls the contract's `remove` (0x51826034) function
        pub fn remove(
            &self,
            stakers_to_remove: ::std::vec::Vec<NodeInfo>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 130, 96, 52], stakers_to_remove)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        ///Gets the contract's `Added` event
        pub fn added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddedFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Removed` event
        pub fn removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RemovedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PermissionedStakeTableEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for PermissionedStakeTable<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Custom Error type `StakerAlreadyExists` with signature `StakerAlreadyExists((uint256,uint256,uint256,uint256))` and selector `0x360dc282`
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
        name = "StakerAlreadyExists",
        abi = "StakerAlreadyExists((uint256,uint256,uint256,uint256))"
    )]
    pub struct StakerAlreadyExists(pub G2Point);
    ///Custom Error type `StakerNotFound` with signature `StakerNotFound((uint256,uint256,uint256,uint256))` and selector `0x34a7561f`
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
        name = "StakerNotFound",
        abi = "StakerNotFound((uint256,uint256,uint256,uint256))"
    )]
    pub struct StakerNotFound(pub G2Point);
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
    pub enum PermissionedStakeTableErrors {
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        StakerAlreadyExists(StakerAlreadyExists),
        StakerNotFound(StakerNotFound),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PermissionedStakeTableErrors {
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
                <StakerAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakerAlreadyExists(decoded));
            }
            if let Ok(decoded) = <StakerNotFound as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StakerNotFound(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PermissionedStakeTableErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerNotFound(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PermissionedStakeTableErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
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
                _ if selector
                    == <StakerAlreadyExists as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <StakerNotFound as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PermissionedStakeTableErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnableInvalidOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableUnauthorizedAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerAlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PermissionedStakeTableErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for PermissionedStakeTableErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for PermissionedStakeTableErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<StakerAlreadyExists> for PermissionedStakeTableErrors {
        fn from(value: StakerAlreadyExists) -> Self {
            Self::StakerAlreadyExists(value)
        }
    }
    impl ::core::convert::From<StakerNotFound> for PermissionedStakeTableErrors {
        fn from(value: StakerNotFound) -> Self {
            Self::StakerNotFound(value)
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
        name = "Added",
        abi = "Added(((uint256,uint256,uint256,uint256),(uint256,uint256),bool)[])"
    )]
    pub struct AddedFilter(pub ::std::vec::Vec<NodeInfo>);
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
        name = "Removed",
        abi = "Removed(((uint256,uint256,uint256,uint256),(uint256,uint256),bool)[])"
    )]
    pub struct RemovedFilter(pub ::std::vec::Vec<NodeInfo>);
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
    pub enum PermissionedStakeTableEvents {
        AddedFilter(AddedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RemovedFilter(RemovedFilter),
    }
    impl ::ethers::contract::EthLogDecode for PermissionedStakeTableEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddedFilter::decode_log(log) {
                return Ok(PermissionedStakeTableEvents::AddedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(PermissionedStakeTableEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RemovedFilter::decode_log(log) {
                return Ok(PermissionedStakeTableEvents::RemovedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PermissionedStakeTableEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddedFilter> for PermissionedStakeTableEvents {
        fn from(value: AddedFilter) -> Self {
            Self::AddedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for PermissionedStakeTableEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RemovedFilter> for PermissionedStakeTableEvents {
        fn from(value: RemovedFilter) -> Self {
            Self::RemovedFilter(value)
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
    ///Container type for all input parameters for the `insert` function with signature `insert(((uint256,uint256,uint256,uint256),(uint256,uint256),bool)[])` and selector `0xcd1da5e5`
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
        name = "insert",
        abi = "insert(((uint256,uint256,uint256,uint256),(uint256,uint256),bool)[])"
    )]
    pub struct InsertCall {
        pub new_stakers: ::std::vec::Vec<NodeInfo>,
    }
    ///Container type for all input parameters for the `isStaker` function with signature `isStaker((uint256,uint256,uint256,uint256))` and selector `0x75d705e9`
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
    #[ethcall(name = "isStaker", abi = "isStaker((uint256,uint256,uint256,uint256))")]
    pub struct IsStakerCall {
        pub staker: G2Point,
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
    ///Container type for all input parameters for the `remove` function with signature `remove(((uint256,uint256,uint256,uint256),(uint256,uint256),bool)[])` and selector `0x51826034`
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
        name = "remove",
        abi = "remove(((uint256,uint256,uint256,uint256),(uint256,uint256),bool)[])"
    )]
    pub struct RemoveCall {
        pub stakers_to_remove: ::std::vec::Vec<NodeInfo>,
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
    pub enum PermissionedStakeTableCalls {
        HashBlsKey(HashBlsKeyCall),
        Insert(InsertCall),
        IsStaker(IsStakerCall),
        Owner(OwnerCall),
        Remove(RemoveCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for PermissionedStakeTableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <HashBlsKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HashBlsKey(decoded));
            }
            if let Ok(decoded) = <InsertCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Insert(decoded));
            }
            if let Ok(decoded) = <IsStakerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsStaker(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RemoveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Remove(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PermissionedStakeTableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::HashBlsKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Insert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsStaker(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Remove(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PermissionedStakeTableCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::HashBlsKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::Insert(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsStaker(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Remove(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<HashBlsKeyCall> for PermissionedStakeTableCalls {
        fn from(value: HashBlsKeyCall) -> Self {
            Self::HashBlsKey(value)
        }
    }
    impl ::core::convert::From<InsertCall> for PermissionedStakeTableCalls {
        fn from(value: InsertCall) -> Self {
            Self::Insert(value)
        }
    }
    impl ::core::convert::From<IsStakerCall> for PermissionedStakeTableCalls {
        fn from(value: IsStakerCall) -> Self {
            Self::IsStaker(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for PermissionedStakeTableCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RemoveCall> for PermissionedStakeTableCalls {
        fn from(value: RemoveCall) -> Self {
            Self::Remove(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for PermissionedStakeTableCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for PermissionedStakeTableCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
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
    ///Container type for all return fields from the `isStaker` function with signature `isStaker((uint256,uint256,uint256,uint256))` and selector `0x75d705e9`
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
    pub struct IsStakerReturn(pub bool);
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
    ///`G2Point(uint256,uint256,uint256,uint256)`
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
    pub struct G2Point {
        pub x_0: ::ethers::core::types::U256,
        pub x_1: ::ethers::core::types::U256,
        pub y_0: ::ethers::core::types::U256,
        pub y_1: ::ethers::core::types::U256,
    }
    ///`EdOnBN254Point(uint256,uint256)`
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
    pub struct EdOnBN254Point {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///`NodeInfo((uint256,uint256,uint256,uint256),(uint256,uint256),bool)`
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
    pub struct NodeInfo {
        pub bls_vk: G2Point,
        pub schnorr_vk: EdOnBN254Point,
        pub is_da: bool,
    }
}
