pub use simple_stake_table::*;
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
pub mod simple_stake_table {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("initialOwner"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
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
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BN254.G2Point[]"),
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
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BN254.G2Point[]"),
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
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
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
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
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
    pub static SIMPLESTAKETABLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x07\xA88\x03\x80a\x07\xA8\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\xBEV[\x80`\x01`\x01`\xA0\x1B\x03\x81\x16a\0^W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[a\0g\x81a\0nV[PPa\0\xEEV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0` \x82\x84\x03\x12\x15a\0\xD0W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xE7W`\0\x80\xFD[\x93\x92PPPV[a\x06\xAB\x80a\0\xFD`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xC7W\x80c\x9B0\xA5\xE6\x14a\0\xE2W\x80c\xCE\xD4~f\x14a\x01\x03W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x16W`\0\x80\xFD[\x80c=\x8D\x10@\x14a\0\x82W\x80cqP\x18\xA6\x14a\0\x97W\x80cu\xD7\x05\xE9\x14a\0\x9FW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x05\x1AV[a\x01)V[\0[a\0\x95a\x021V[a\0\xB2a\0\xAD6`\x04a\x05\xCDV[a\x02EV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xBEV[a\0\xF5a\0\xF06`\x04a\x05\xCDV[a\x02nV[`@Q\x90\x81R` \x01a\0\xBEV[a\0\x95a\x01\x116`\x04a\x05\x1AV[a\x02\xCAV[a\0\x95a\x01$6`\x04a\x05\xF0V[a\x03\xB2V[a\x011a\x03\xF0V[`\0[\x81Q\x81\x10\x15a\x01\xF6W`\0a\x01a\x83\x83\x81Q\x81\x10a\x01TWa\x01Ta\x06\x19V[` \x02` \x01\x01Qa\x02nV[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x01\xD5W\x82\x82\x81Q\x81\x10a\x01\x8DWa\x01\x8Da\x06\x19V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Qc\x1B\x06\xE1A`\xE1\x1B\x81R\x82Q`\x04\x82\x01R\x92\x82\x01Q`$\x84\x01R\x81\x01Q`D\x83\x01R``\x01Q`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x82\x17\x90U\x01a\x014V[P\x7Fz\xC3a=\xC3\xB4>q#Y\x94 \xBDXa\x0C\xD6\xFF;\x14\xB6\x0E?%T\xA1\x0E\xC2\xCCM\x1E)\x81`@Qa\x02&\x91\x90a\x06/V[`@Q\x80\x91\x03\x90\xA1PV[a\x029a\x03\xF0V[a\x02C`\0a\x04\x1DV[V[`\0`\x01`\0a\x02T\x84a\x02nV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x92\x91PPV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x02\xAD\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[a\x02\xD2a\x03\xF0V[`\0[\x81Q\x81\x10\x15a\x03\x82W`\0a\x02\xF5\x83\x83\x81Q\x81\x10a\x01TWa\x01Ta\x06\x19V[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91P`\xFF\x16a\x03cW\x82\x82\x81Q\x81\x10a\x03 Wa\x03 a\x06\x19V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Qc4\xA7V\x1F`\xE0\x1B\x81R\x82Q`\x04\x82\x01R\x92\x82\x01Q`$\x84\x01R\x81\x01Q`D\x83\x01R``\x01Q`d\x82\x01R`\x84\x01a\x01\xCCV[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90U\x01a\x02\xD5V[P\x7F[L\xFC\x86_\xA8H:\x99<\xD6)\x1F\x0Cg\x18T\xFC\xBFnD\x89\xD8\xD6\x912\\\xF4\xD8\x08\xA6S\x81`@Qa\x02&\x91\x90a\x06/V[a\x03\xBAa\x03\xF0V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xE4W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x01\xCCV[a\x03\xED\x81a\x04\x1DV[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02CW`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x01\xCCV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xACWa\x04\xACa\x04mV[`@R\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x04\xC6W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04\xE9Wa\x04\xE9a\x04mV[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15a\x05-W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05EW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x05YW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05kWa\x05ka\x04mV[a\x05y\x84\x82`\x05\x1B\x01a\x04\x83V[\x81\x81R\x84\x81\x01\x92P`\x07\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a\x05\x99W`\0\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a\x05\xC2Wa\x05\xB0\x88\x85a\x04\xB4V[\x83R\x84\x83\x01\x92P`\x80\x84\x01\x93Pa\x05\x9EV[\x97\x96PPPPPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x05\xDFW`\0\x80\xFD[a\x05\xE9\x83\x83a\x04\xB4V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\x02W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xE9W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x06\x92Wa\x06\x7F\x83\x85Q\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R``\x81\x01Q``\x83\x01RPPV[\x92\x84\x01\x92`\x80\x92\x90\x92\x01\x91`\x01\x01a\x06KV[P\x90\x96\x95PPPPPPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static SIMPLESTAKETABLE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xC7W\x80c\x9B0\xA5\xE6\x14a\0\xE2W\x80c\xCE\xD4~f\x14a\x01\x03W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x16W`\0\x80\xFD[\x80c=\x8D\x10@\x14a\0\x82W\x80cqP\x18\xA6\x14a\0\x97W\x80cu\xD7\x05\xE9\x14a\0\x9FW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x05\x1AV[a\x01)V[\0[a\0\x95a\x021V[a\0\xB2a\0\xAD6`\x04a\x05\xCDV[a\x02EV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xBEV[a\0\xF5a\0\xF06`\x04a\x05\xCDV[a\x02nV[`@Q\x90\x81R` \x01a\0\xBEV[a\0\x95a\x01\x116`\x04a\x05\x1AV[a\x02\xCAV[a\0\x95a\x01$6`\x04a\x05\xF0V[a\x03\xB2V[a\x011a\x03\xF0V[`\0[\x81Q\x81\x10\x15a\x01\xF6W`\0a\x01a\x83\x83\x81Q\x81\x10a\x01TWa\x01Ta\x06\x19V[` \x02` \x01\x01Qa\x02nV[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x01\xD5W\x82\x82\x81Q\x81\x10a\x01\x8DWa\x01\x8Da\x06\x19V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Qc\x1B\x06\xE1A`\xE1\x1B\x81R\x82Q`\x04\x82\x01R\x92\x82\x01Q`$\x84\x01R\x81\x01Q`D\x83\x01R``\x01Q`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x82\x17\x90U\x01a\x014V[P\x7Fz\xC3a=\xC3\xB4>q#Y\x94 \xBDXa\x0C\xD6\xFF;\x14\xB6\x0E?%T\xA1\x0E\xC2\xCCM\x1E)\x81`@Qa\x02&\x91\x90a\x06/V[`@Q\x80\x91\x03\x90\xA1PV[a\x029a\x03\xF0V[a\x02C`\0a\x04\x1DV[V[`\0`\x01`\0a\x02T\x84a\x02nV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x92\x91PPV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x02\xAD\x94\x93\x92\x91\x90\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[a\x02\xD2a\x03\xF0V[`\0[\x81Q\x81\x10\x15a\x03\x82W`\0a\x02\xF5\x83\x83\x81Q\x81\x10a\x01TWa\x01Ta\x06\x19V[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91P`\xFF\x16a\x03cW\x82\x82\x81Q\x81\x10a\x03 Wa\x03 a\x06\x19V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Qc4\xA7V\x1F`\xE0\x1B\x81R\x82Q`\x04\x82\x01R\x92\x82\x01Q`$\x84\x01R\x81\x01Q`D\x83\x01R``\x01Q`d\x82\x01R`\x84\x01a\x01\xCCV[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90U\x01a\x02\xD5V[P\x7F[L\xFC\x86_\xA8H:\x99<\xD6)\x1F\x0Cg\x18T\xFC\xBFnD\x89\xD8\xD6\x912\\\xF4\xD8\x08\xA6S\x81`@Qa\x02&\x91\x90a\x06/V[a\x03\xBAa\x03\xF0V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xE4W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x01\xCCV[a\x03\xED\x81a\x04\x1DV[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02CW`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x01\xCCV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xACWa\x04\xACa\x04mV[`@R\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x04\xC6W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04\xE9Wa\x04\xE9a\x04mV[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15a\x05-W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05EW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x05YW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05kWa\x05ka\x04mV[a\x05y\x84\x82`\x05\x1B\x01a\x04\x83V[\x81\x81R\x84\x81\x01\x92P`\x07\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a\x05\x99W`\0\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a\x05\xC2Wa\x05\xB0\x88\x85a\x04\xB4V[\x83R\x84\x83\x01\x92P`\x80\x84\x01\x93Pa\x05\x9EV[\x97\x96PPPPPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x05\xDFW`\0\x80\xFD[a\x05\xE9\x83\x83a\x04\xB4V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\x02W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xE9W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x06\x92Wa\x06\x7F\x83\x85Q\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R``\x81\x01Q``\x83\x01RPPV[\x92\x84\x01\x92`\x80\x92\x90\x92\x01\x91`\x01\x01a\x06KV[P\x90\x96\x95PPPPPPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static SIMPLESTAKETABLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct SimpleStakeTable<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SimpleStakeTable<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SimpleStakeTable<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SimpleStakeTable<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SimpleStakeTable<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SimpleStakeTable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SimpleStakeTable<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SIMPLESTAKETABLE_ABI.clone(),
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
                SIMPLESTAKETABLE_ABI.clone(),
                SIMPLESTAKETABLE_BYTECODE.clone().into(),
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
        ///Calls the contract's `insert` (0x3d8d1040) function
        pub fn insert(
            &self,
            new_stakers: ::std::vec::Vec<G2Point>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 141, 16, 64], new_stakers)
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
        ///Calls the contract's `remove` (0xced47e66) function
        pub fn remove(
            &self,
            stakers_to_remove: ::std::vec::Vec<G2Point>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 212, 126, 102], stakers_to_remove)
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SimpleStakeTableEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for SimpleStakeTable<M>
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
    pub enum SimpleStakeTableErrors {
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        StakerAlreadyExists(StakerAlreadyExists),
        StakerNotFound(StakerNotFound),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SimpleStakeTableErrors {
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
    impl ::ethers::core::abi::AbiEncode for SimpleStakeTableErrors {
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
    impl ::ethers::contract::ContractRevert for SimpleStakeTableErrors {
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
    impl ::core::fmt::Display for SimpleStakeTableErrors {
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
    impl ::core::convert::From<::std::string::String> for SimpleStakeTableErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for SimpleStakeTableErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for SimpleStakeTableErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<StakerAlreadyExists> for SimpleStakeTableErrors {
        fn from(value: StakerAlreadyExists) -> Self {
            Self::StakerAlreadyExists(value)
        }
    }
    impl ::core::convert::From<StakerNotFound> for SimpleStakeTableErrors {
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
    #[ethevent(name = "Added", abi = "Added((uint256,uint256,uint256,uint256)[])")]
    pub struct AddedFilter(pub ::std::vec::Vec<G2Point>);
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
    #[ethevent(name = "Removed", abi = "Removed((uint256,uint256,uint256,uint256)[])")]
    pub struct RemovedFilter(pub ::std::vec::Vec<G2Point>);
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
    pub enum SimpleStakeTableEvents {
        AddedFilter(AddedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RemovedFilter(RemovedFilter),
    }
    impl ::ethers::contract::EthLogDecode for SimpleStakeTableEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddedFilter::decode_log(log) {
                return Ok(SimpleStakeTableEvents::AddedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(SimpleStakeTableEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = RemovedFilter::decode_log(log) {
                return Ok(SimpleStakeTableEvents::RemovedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SimpleStakeTableEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddedFilter> for SimpleStakeTableEvents {
        fn from(value: AddedFilter) -> Self {
            Self::AddedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for SimpleStakeTableEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RemovedFilter> for SimpleStakeTableEvents {
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
    ///Container type for all input parameters for the `insert` function with signature `insert((uint256,uint256,uint256,uint256)[])` and selector `0x3d8d1040`
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
    #[ethcall(name = "insert", abi = "insert((uint256,uint256,uint256,uint256)[])")]
    pub struct InsertCall {
        pub new_stakers: ::std::vec::Vec<G2Point>,
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
    ///Container type for all input parameters for the `remove` function with signature `remove((uint256,uint256,uint256,uint256)[])` and selector `0xced47e66`
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
    #[ethcall(name = "remove", abi = "remove((uint256,uint256,uint256,uint256)[])")]
    pub struct RemoveCall {
        pub stakers_to_remove: ::std::vec::Vec<G2Point>,
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
    pub enum SimpleStakeTableCalls {
        HashBlsKey(HashBlsKeyCall),
        Insert(InsertCall),
        IsStaker(IsStakerCall),
        Owner(OwnerCall),
        Remove(RemoveCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for SimpleStakeTableCalls {
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
    impl ::ethers::core::abi::AbiEncode for SimpleStakeTableCalls {
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
    impl ::core::fmt::Display for SimpleStakeTableCalls {
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
    impl ::core::convert::From<HashBlsKeyCall> for SimpleStakeTableCalls {
        fn from(value: HashBlsKeyCall) -> Self {
            Self::HashBlsKey(value)
        }
    }
    impl ::core::convert::From<InsertCall> for SimpleStakeTableCalls {
        fn from(value: InsertCall) -> Self {
            Self::Insert(value)
        }
    }
    impl ::core::convert::From<IsStakerCall> for SimpleStakeTableCalls {
        fn from(value: IsStakerCall) -> Self {
            Self::IsStaker(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for SimpleStakeTableCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RemoveCall> for SimpleStakeTableCalls {
        fn from(value: RemoveCall) -> Self {
            Self::Remove(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for SimpleStakeTableCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for SimpleStakeTableCalls {
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
}
