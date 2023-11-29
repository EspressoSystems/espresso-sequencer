pub use hot_shot::*;
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
pub mod hot_shot {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MAX_BLOCKS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MAX_BLOCKS"),
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
                    ::std::borrow::ToOwned::to_owned("addNewStakingKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addNewStakingKey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakingKey"),
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
                    ::std::borrow::ToOwned::to_owned("blockHeight"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("blockHeight"),
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
                    ::std::borrow::ToOwned::to_owned("commitments"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("commitments"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("blockHeight"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("commitment"),
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
                    ::std::borrow::ToOwned::to_owned("getStakingKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStakingKey"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("newBlocks"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("newBlocks"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("qcs"),
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
                                ::std::borrow::ToOwned::to_owned("struct HotShot.QC[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifyAggSig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verifyAggSig"),
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
                                name: ::std::borrow::ToOwned::to_owned("bitmap"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("minStakeThreshold"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("NewStakingKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewStakingKey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("stakingKey"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("IncorrectBlockNumber"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("IncorrectBlockNumber",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expectedBlockNumber",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidQC"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidQC"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoKeySelected"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NoKeySelected"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotEnoughStake"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotEnoughStake"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TooManyBlocks"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TooManyBlocks"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("numBlocks"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
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
    pub static HOTSHOT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa \xA3\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cI\xCE\x89\x97\x11a\0[W\x80cI\xCE\x89\x97\x14a\0\xC6W\x80cg\xA2\x1Ep\x14a\0\xE6W\x80c\xF1\xF4]\x99\x14a\x010W\x80c\xF4O\xF7\x12\x14a\x01CW`\0\x80\xFD[\x80c\x03@\x96\x1E\x14a\0\x82W\x80c\n2\x1C\xFF\x14a\0\x97W\x80c&\x83=\xCC\x14a\0\xAAW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x1BuV[a\x01LV[\0[a\0\x95a\0\xA56`\x04a\x1CEV[a\x03\xD7V[a\0\xB3a\x01\xF4\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB3a\0\xD46`\x04a\x1C\xBAV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\0\xF9a\0\xF46`\x04a\x1C\xBAV[a\x05FV[`@\x80Q\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R\x83\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x92\x83\x01Q\x92\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x01a\0\xBDV[a\0\x95a\x01>6`\x04a\x1C\xD3V[a\x05\xDAV[a\0\xB3`\x01T\x81V[`\x03T\x82Q\x11\x15a\x01\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqbitmap is too long`p\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x81Q\x81\x10a\x01\xAEWa\x01\xAEa\x1DKV[` \x02` \x01\x01Q\x15\x80\x15a\x01\xC3WP\x82Q\x81\x10[\x15a\x01\xDAW\x80a\x01\xD2\x81a\x1DwV[\x91PPa\x01\x9CV[\x82Q\x81\x10a\x01\xFBW`@QcKe\x82-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81[\x84Q\x81\x10\x15a\x02KW\x84\x81\x81Q\x81\x10a\x02\x1AWa\x02\x1Aa\x1DKV[` \x02` \x01\x01Q\x15a\x02CW`\0\x81\x81R`\x02` R`@\x90 Ta\x02@\x90\x83a\x1D\x90V[\x91P[`\x01\x01a\x01\xFFV[P\x82\x81\x10\x15a\x02mW`@Qc<)\x0BS`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x03\x83\x81T\x81\x10a\x02\x82Wa\x02\x82a\x1DKV[\x90`\0R` `\0 \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPP\x90P`\0\x83`\x01a\x02\xD4\x91\x90a\x1D\x90V[\x90P[\x85Q\x81\x10\x15a\x03\xC2W\x85\x81\x81Q\x81\x10a\x02\xF2Wa\x02\xF2a\x1DKV[` \x02` \x01\x01Q\x15a\x03\xBAW`\0`\x03\x82\x81T\x81\x10a\x03\x14Wa\x03\x14a\x1DKV[`\0\x91\x82R` \x80\x83 `@\x80Q`\x80\x81\x01\x82R`\x04\x90\x94\x02\x90\x91\x01\x80T\x80\x85R`\x01\x82\x01T\x85\x85\x01\x81\x90R`\x02\x83\x01T\x86\x85\x01\x81\x90R`\x03\x90\x93\x01T``\x80\x88\x01\x82\x90R\x8BQ\x96\x8C\x01Q\x95\x8C\x01Q\x90\x8C\x01Q\x97\x99P\x95\x97\x94\x96\x94\x92\x93\x91\x92\x80\x80\x80a\x03\x86\x8B\x8D\x8B\x8D\x8B\x8D\x8B\x8Da\x07\x02V[`@\x80Q`\x80\x81\x01\x82R\x93\x84R` \x84\x01\x94\x90\x94R\x92\x82\x01\x92\x90\x92R``\x81\x01\x91\x90\x91R\x9EPPPPPPPPPPPPPP[`\x01\x01a\x02\xD7V[Pa\x03\xCE\x87\x87\x83a\x08OV[PPPPPPPV[a\x01\xF4\x81\x11\x15a\x03\xFDW`@Qc\xE0\x82\x84\x0B`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x01\x90V[`\x01T`\0[\x82\x81\x10\x15a\x05\x06W`\x01T\x84\x84\x83\x81\x81\x10a\x04 Wa\x04 a\x1DKV[\x90P`\x80\x02\x01`\0\x015\x14a\x04rW\x83\x83\x82\x81\x81\x10a\x04AWa\x04Aa\x1DKV[\x90P`\x80\x02\x01`\0\x015`\x01T`@Qc4\xE4#\xFF`\xE0\x1B\x81R`\x04\x01a\x01\x90\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[a\x04\x90\x84\x84\x83\x81\x81\x10a\x04\x87Wa\x04\x87a\x1DKV[\x90PPP`\x01\x90V[a\x04\xB3W`\x01T`@Qcx\x18g\x19`\xE0\x1B\x81R`\x04\x01a\x01\x90\x91\x81R` \x01\x90V[\x83\x83\x82\x81\x81\x10a\x04\xC5Wa\x04\xC5a\x1DKV[\x90P`\x80\x02\x01` \x015`\0\x80`\x01T\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x80`\0\x82\x82Ta\x04\xF9\x91\x90a\x1D\x90V[\x90\x91UPP`\x01\x01a\x04\x03V[P`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x82\x03\xA2\x1EO\x95\xF7.P\x81\xD5\xE0\x92\x9B\x1A\x8CR\x14\x1E\x12?\x9A\x14\xE1\xE7K\x02`\xFA_R\xF1\x91\x01[`@Q\x80\x91\x03\x90\xA1PPPV[a\x05q`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0`\x03\x83\x81T\x81\x10a\x05\x86Wa\x05\x86a\x1DKV[`\0\x91\x82R` \x80\x83 \x95\x83R`\x02\x80\x82R`@\x93\x84\x90 T\x84Q`\x80\x81\x01\x86R`\x04\x90\x94\x02\x90\x97\x01\x80T\x84R`\x01\x81\x01T\x92\x84\x01\x92\x90\x92R\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x90\x91\x01T``\x82\x01R\x93\x91PPV[`\x03\x80T`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x90U\x84T`\x01\x81\x01\x86U\x94\x90\x92R\x85Q`\x04\x90\x94\x02\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x81\x01\x85\x90U\x86\x82\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8\\\x83\x01U\x87\x84\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8]\x84\x01U``\x80\x8A\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8^\x90\x95\x01\x94\x90\x94U\x85Q\x97\x88R\x91Q\x93\x87\x01\x93\x90\x93R\x91Q\x92\x85\x01\x92\x90\x92R\x90Q\x90\x83\x01R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x81\x90R\x90\x7F\xD7/\xE1\xACW\xD3\xE6\xD5\x1C\x92*\xE4\xD8\x11\xCCP\xAA:\xD7\x02b\x83\xAE\xA67IJ\x072RVZ\x90`\xC0\x01a\x059V[`\0\x80\x80\x80\x8B\x15\x80\x15a\x07\x13WP\x8A\x15[\x80\x15a\x07\x1DWP\x89\x15[\x80\x15a\x07'WP\x88\x15[\x15a\x07xW\x87\x15\x80\x15a\x078WP\x86\x15[\x80\x15a\x07BWP\x85\x15[\x80\x15a\x07LWP\x84\x15[a\x07hWa\x07\\\x88\x88\x88\x88a\t\x01V[a\x07hWa\x07ha\x1D\xA9V[P\x86\x92P\x85\x91P\x84\x90P\x83a\x08@V[\x87\x15\x80\x15a\x07\x84WP\x86\x15[\x80\x15a\x07\x8EWP\x85\x15[\x80\x15a\x07\x98WP\x84\x15[\x15a\x07\xC5Wa\x07\xA9\x8C\x8C\x8C\x8Ca\t\x01V[a\x07\xB5Wa\x07\xB5a\x1D\xA9V[P\x8A\x92P\x89\x91P\x88\x90P\x87a\x08@V[a\x07\xD1\x8C\x8C\x8C\x8Ca\t\x01V[a\x07\xDDWa\x07\xDDa\x1D\xA9V[a\x07\xE9\x88\x88\x88\x88a\t\x01V[a\x07\xF5Wa\x07\xF5a\x1D\xA9V[`\0a\x08\x0F\x8D\x8D\x8D\x8D`\x01`\0\x8F\x8F\x8F\x8F`\x01`\0a\t\xB6V[\x80Q` \x82\x01Q`@\x83\x01Q``\x84\x01Q`\x80\x85\x01Q`\xA0\x86\x01Q\x95\x96Pa\x086\x95a\x0C?V[\x94P\x94P\x94P\x94PP[\x98P\x98P\x98P\x98\x94PPPPPV[a\x08X\x82a\x0C\x89V[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a *`$\x919\x90P`\0\x84\x82`@Q` \x01a\x08\x8A\x92\x91\x90a\x1D\xEFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80a\x08\xA7\x83a\r\x18V[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91Pa\x08\xDB\x81\x87a\x08\xCE\x8Aa\r\xEBV[a\x08\xD6a\x0EfV[a\x0F7V[a\x08\xF7W`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0\x80`\0\x80`\0a\t\x15\x87\x87\x89\x89a\x10\x19V[\x90\x94P\x92Pa\t&\x89\x89\x81\x81a\x10\x19V[\x90\x92P\x90Pa\t7\x82\x82\x8B\x8Ba\x10\x19V[\x90\x92P\x90Pa\tH\x84\x84\x84\x84a\x10\x8AV[\x90\x94P\x92Pa\t\x98\x84\x84\x7F+\x14\x9D@\xCE\xB8\xAA\xAE\x81\xBE\x18\x99\x1B\xE0j\xC3\xB5\xB4\xC5\xE5Y\xDB\xEF\xA32g\xE6\xDC$\xA18\xE5~\x97\x13\xB0:\xF0\xFE\xD4\xCD,\xAF\xAD\xEE\xD8\xFD\xF4\xA7O\xA0\x84\xE5-\x18R\xE4\xA2\xBD\x06\x85\xC3\x15\xD2a\x10\x8AV[\x90\x94P\x92P\x83\x15\x80\x15a\t\xA9WP\x82\x15[\x99\x98PPPPPPPPPV[a\t\xBEa\x1A2V[\x88\x15\x80\x15a\t\xCAWP\x87\x15[\x15a\n\x0CW\x86\x86\x86\x86\x86\x86\x86`\0[`\xA0\x89\x01\x92\x90\x92R`\x80\x88\x01\x92\x90\x92R``\x87\x01\x92\x90\x92R`@\x86\x01\x92\x90\x92R` \x85\x81\x01\x93\x90\x93R\x90\x91\x02\x01Ra\x0C/V[\x82\x15\x80\x15a\n\x18WP\x81\x15[\x15a\n+W\x8C\x8C\x8C\x8C\x8C\x8C\x86`\0a\t\xD9V[a\n7\x85\x85\x8B\x8Ba\x10\x19V[\x90\x95P\x93Pa\nH\x8B\x8B\x85\x85a\x10\x19V[``\x83\x01R`@\x82\x01Ra\n^\x87\x87\x8B\x8Ba\x10\x19V[\x90\x97P\x95Pa\no\x8D\x8D\x85\x85a\x10\x19V[`\xA0\x83\x01R`\x80\x82\x01\x81\x90R\x87\x14\x80\x15a\n\x8CWP`\xA0\x81\x01Q\x86\x14[\x15a\n\xD1W`@\x81\x01Q\x85\x14\x80\x15a\n\xA7WP``\x81\x01Q\x84\x14[\x15a\n\xC2Wa\n\xBA\x8D\x8D\x8D\x8D\x8D\x8Da\x10\xCCV[\x86`\0a\t\xD9V[`\x01`\0\x81\x81\x80\x80\x86\x81a\t\xD9V[a\n\xDD\x89\x89\x85\x85a\x10\x19V[\x90\x93P\x91Pa\n\xFD\x85\x85\x83`\x02` \x02\x01Q\x84`\x03[` \x02\x01Qa\x10\x8AV[\x90\x9DP\x9BPa\x0B\x17\x87\x87\x83`\x04` \x02\x01Q\x84`\x05a\n\xF3V[\x90\x9BP\x99Pa\x0B(\x8B\x8B\x81\x81a\x10\x19V[\x90\x99P\x97Pa\x0BH\x89\x89\x83`\x04` \x02\x01Q\x84`\x05[` \x02\x01Qa\x10\x19V[\x90\x95P\x93Pa\x0BY\x89\x89\x8D\x8Da\x10\x19V[\x90\x99P\x97Pa\x0Bj\x89\x89\x85\x85a\x10\x19V[`\xA0\x83\x01R`\x80\x82\x01Ra\x0B\x80\x8D\x8D\x81\x81a\x10\x19V[\x90\x97P\x95Pa\x0B\x91\x87\x87\x85\x85a\x10\x19V[\x90\x97P\x95Pa\x0B\xA2\x87\x87\x8B\x8Ba\x10\x8AV[\x90\x97P\x95Pa\x0B\xB3\x85\x85`\x02a\x12;V[\x90\x93P\x91Pa\x0B\xC4\x87\x87\x85\x85a\x10\x8AV[\x90\x97P\x95Pa\x0B\xD5\x8B\x8B\x89\x89a\x10\x19V[` \x83\x01R\x81Ra\x0B\xE8\x85\x85\x89\x89a\x10\x8AV[\x90\x9BP\x99Pa\x0B\xF9\x8D\x8D\x8D\x8Da\x10\x19V[\x90\x9BP\x99Pa\x0C\x13\x89\x89\x83`\x02` \x02\x01Q\x84`\x03a\x0B>V[\x90\x9DP\x9BPa\x0C$\x8B\x8B\x8F\x8Fa\x10\x8AV[``\x83\x01R`@\x82\x01R[\x9C\x9BPPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x80a\x0CR\x88\x88a\x12nV[\x90\x92P\x90Pa\x0Cc\x8C\x8C\x84\x84a\x10\x19V[\x90\x96P\x94Pa\x0Ct\x8A\x8A\x84\x84a\x10\x19V[\x96\x9D\x95\x9CP\x9AP\x94\x98P\x92\x96PPPPPPPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a N\x839\x81Q\x91R\x91\x82`\x03\x81\x80\x85\x80\t\x85\t\x08\x83\x82\x83\t\x14\x81\x15\x83\x15\x17\x19\x84\x83\x10\x85\x85\x10\x16\x16\x16\x93PPP\x81a\r\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[PPPV[`\0\x80`\0a\r&\x84a\x12\xF9V[\x90P`\0\x80Q` a N\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\rOWa\rOa\x1E\x0CV[\x84\x82\t\x90P\x82\x80a\rbWa\rba\x1E\x0CV[\x82\x82\x08\x90P`\0\x80a\rs\x83a\x15\rV[\x92P\x90P[\x80a\r\xDCW\x84\x80a\r\x8BWa\r\x8Ba\x1E\x0CV[`\x01\x87\x08\x95P\x84\x80a\r\x9FWa\r\x9Fa\x1E\x0CV[\x86\x87\t\x92P\x84\x80a\r\xB2Wa\r\xB2a\x1E\x0CV[\x86\x84\t\x92P\x84\x80a\r\xC5Wa\r\xC5a\x1E\x0CV[\x84\x84\x08\x92Pa\r\xD3\x83a\x15\rV[\x92P\x90Pa\rxV[P\x93\x97\x93\x96P\x92\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x0E\x13WP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a N\x839\x81Q\x91R\x84` \x01Qa\x0EF\x91\x90a\x1E\"V[a\x0E^\x90`\0\x80Q` a N\x839\x81Q\x91Ra\x1EDV[\x90R\x92\x91PPV[a\x0E\x91`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R\x86Q`@\x82\x01R` \x87\x01Q``\x82\x01R`@\x87\x01Q`\x80\x82\x01R``\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R` \x85\x01Qa\x01 \x82\x01R`@\x85\x01Qa\x01@\x82\x01R``\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x10\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[P\x15\x15\x95\x94PPPPPV[`\0\x80a\x10W`\0\x80Q` a N\x839\x81Q\x91R\x85\x88\t`\0\x80Q` a N\x839\x81Q\x91R\x85\x88\t`\0\x80Q` a N\x839\x81Q\x91Ra\x16\x08V[`\0\x80Q` a N\x839\x81Q\x91R\x80\x86\x88\t`\0\x80Q` a N\x839\x81Q\x91R\x86\x8A\t\x08\x91P\x91P\x94P\x94\x92PPPV[`\0\x80a\x10\xA6\x86\x85`\0\x80Q` a N\x839\x81Q\x91Ra\x16\x08V[a\x10\xBF\x86\x85`\0\x80Q` a N\x839\x81Q\x91Ra\x16\x08V[\x91P\x91P\x94P\x94\x92PPPV[`\0\x80`\0\x80`\0\x80a\x10\xE1\x8C\x8C`\x03a\x12;V[\x90\x96P\x94Pa\x10\xF2\x86\x86\x8E\x8Ea\x10\x19V[\x90\x96P\x94Pa\x11\x03\x8A\x8A\x8A\x8Aa\x10\x19V[\x90\x98P\x96Pa\x11\x14\x8C\x8C\x8C\x8Ca\x10\x19V[\x90\x94P\x92Pa\x11%\x84\x84\x8A\x8Aa\x10\x19V[\x90\x94P\x92Pa\x116\x86\x86\x81\x81a\x10\x19V[\x90\x9CP\x9APa\x11G\x84\x84`\x08a\x12;V[\x90\x92P\x90Pa\x11X\x8C\x8C\x84\x84a\x10\x8AV[\x90\x9CP\x9APa\x11i\x88\x88\x81\x81a\x10\x19V[\x90\x92P\x90Pa\x11z\x84\x84`\x04a\x12;V[\x90\x94P\x92Pa\x11\x8B\x84\x84\x8E\x8Ea\x10\x8AV[\x90\x94P\x92Pa\x11\x9C\x84\x84\x88\x88a\x10\x19V[\x90\x94P\x92Pa\x11\xAD\x8A\x8A`\x08a\x12;V[\x90\x96P\x94Pa\x11\xBE\x86\x86\x8C\x8Ca\x10\x19V[\x90\x96P\x94Pa\x11\xCF\x86\x86\x84\x84a\x10\x19V[\x90\x96P\x94Pa\x11\xE0\x84\x84\x88\x88a\x10\x8AV[\x90\x94P\x92Pa\x11\xF1\x8C\x8C`\x02a\x12;V[\x90\x96P\x94Pa\x12\x02\x86\x86\x8A\x8Aa\x10\x19V[\x90\x96P\x94Pa\x12\x13\x88\x88\x84\x84a\x10\x19V[\x90\x92P\x90Pa\x12$\x82\x82`\x08a\x12;V[\x80\x92P\x81\x93PPP\x96P\x96P\x96P\x96P\x96P\x96\x90PV[`\0\x80`\0\x80Q` a N\x839\x81Q\x91R\x83\x86\t`\0\x80Q` a N\x839\x81Q\x91R\x84\x86\t\x91P\x91P\x93P\x93\x91PPV[`\0\x80\x80a\x12\xAF`\0\x80Q` a N\x839\x81Q\x91R\x80\x87\x88\t`\0\x80Q` a N\x839\x81Q\x91R\x87\x88\t\x08`\0\x80Q` a N\x839\x81Q\x91Ra\x16,V[\x90P`\0\x80Q` a N\x839\x81Q\x91R\x81\x86\t`\0\x80Q` a N\x839\x81Q\x91R\x82\x86\ta\x12\xED\x90`\0\x80Q` a N\x839\x81Q\x91Ra\x1EDV[\x92P\x92PP\x92P\x92\x90PV[`\0\x80a\x13\x05\x83a\x16}V[\x80Q\x90\x91P`0\x81\x14a\x13\x1AWa\x13\x1Aa\x1D\xA9V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x135Wa\x135a\x1APV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x13_W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x13\xD0W\x83`\x01a\x13z\x83\x86a\x1EDV[a\x13\x84\x91\x90a\x1EDV[\x81Q\x81\x10a\x13\x94Wa\x13\x94a\x1DKV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x13\xB1Wa\x13\xB1a\x1DKV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x13eV[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x14bW\x83\x81a\x14\x0E\x85\x88a\x1EDV[a\x14\x18\x91\x90a\x1D\x90V[\x81Q\x81\x10a\x14(Wa\x14(a\x1DKV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x14HWa\x14Ha\x1DKV[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x13\xFAV[P`\0a\x14n\x82a\x19\xD1V[\x90Pa\x01\0`\0\x80Q` a N\x839\x81Q\x91R`\0a\x14\x8E\x86\x89a\x1EDV[\x90P`\0[\x81\x81\x10\x15a\x14\xFDW`\0\x88`\x01a\x14\xAA\x84\x86a\x1EDV[a\x14\xB4\x91\x90a\x1EDV[\x81Q\x81\x10a\x14\xC4Wa\x14\xC4a\x1DKV[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x14\xDCWa\x14\xDCa\x1E\x0CV[\x85\x87\t\x95P\x83\x80a\x14\xEFWa\x14\xEFa\x1E\x0CV[\x81\x87\x08\x95PP`\x01\x01a\x14\x93V[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a N\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x86``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x93PP`\0Q\x93P\x82a\x15\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[\x80`\x01\x85\x90\x1B\x11\x15a\x15\xE8Wa\x15\xE5\x84\x82a\x1EDV[\x93P[\x80\x80a\x15\xF6Wa\x15\xF6a\x1E\x0CV[\x84\x85\t\x91P\x85\x82\x14\x94PPPP\x91P\x91V[`\0\x81\x80a\x16\x18Wa\x16\x18a\x1E\x0CV[a\x16\"\x84\x84a\x1EDV[\x85\x08\x94\x93PPPPV[`\0\x80`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01R`\x02\x84\x03`\x80\x82\x01R\x83`\xA0\x82\x01R` \x81`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x90Q\x92P\x90P\x80a\x16vW`\0\x80\xFD[P\x92\x91PPV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x16\xBE\x92\x91\x90a\x1D\xEFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x16\xE5\x92\x91\x90a\x1EWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x17\x07\x91\x90a\x1E\x83V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x171\x90\x83\x90\x83\x90` \x01a\x1E\x9DV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\xA2Wa\x17\xA2a\x1APV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x17\xCCW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x17\xE4\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x18OW\x81\x81\x81Q\x81\x10a\x18\x13Wa\x18\x13a\x1DKV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x180Wa\x180a\x1DKV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x17\xF8V[P`\0\x84`@Q` \x01a\x18e\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x18\xF9W`\0\x83\x82\x81Q\x81\x10a\x18\xA0Wa\x18\xA0a\x1DKV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x18\xBDWa\x18\xBDa\x1DKV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x18\xDE\x92\x91\x90a\x1E\xC2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x98PP`\x01\x01a\x18\x84V[P\x86\x88\x87`@Q` \x01a\x19\x0F\x93\x92\x91\x90a\x1E\xE7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x19=\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x19^\x8A`\xFF\x8D\x16a\x1EDV[\x81\x10\x15a\x19\xC0W\x82\x81\x81Q\x81\x10a\x19wWa\x19wa\x1DKV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x19\x91\x83\x8Da\x1D\x90V[\x81Q\x81\x10a\x19\xA1Wa\x19\xA1a\x1DKV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x19QV[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a\x16vW\x83\x81\x81Q\x81\x10a\x19\xF1Wa\x19\xF1a\x1DKV[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x1A\t\x91\x90a\x1F\x1BV[a\x1A\x14\x90`\x02a \x16V[a\x1A\x1E\x91\x90a\x1F\x1BV[a\x1A(\x90\x83a\x1D\x90V[\x91P`\x01\x01a\x19\xD6V[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A\x8FWa\x1A\x8Fa\x1APV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x1A\xA9W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1A\xCCWa\x1A\xCCa\x1APV[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1A\xF7W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1B\x13Wa\x1B\x13a\x1APV[\x81`\x05\x1Ba\x1B\"\x82\x82\x01a\x1AfV[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x1B<W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x1BjW\x825\x80\x15\x15\x81\x14a\x1B[W`\0\x80\x81\xFD[\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x1BBV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\xA0\x85\x87\x03\x12\x15a\x1B\x8BW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\xA3W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x1B\xB7W`\0\x80\xFD[\x815` \x82\x82\x11\x15a\x1B\xCBWa\x1B\xCBa\x1APV[a\x1B\xDD`\x1F\x83\x01`\x1F\x19\x16\x82\x01a\x1AfV[\x82\x81R\x8A\x82\x84\x87\x01\x01\x11\x15a\x1B\xF1W`\0\x80\xFD[\x82\x82\x86\x01\x83\x83\x017`\0\x81\x84\x01\x83\x01R\x97Pa\x1C\x0F\x8A\x8A\x83\x01a\x1A\x97V[\x96PPP``\x87\x015\x91P\x80\x82\x11\x15a\x1C'W`\0\x80\xFD[Pa\x1C4\x87\x82\x88\x01a\x1A\xE6V[\x94\x97\x93\x96P\x93\x94`\x80\x015\x93PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x1CXW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1CpW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x1C\x84W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1C\x93W`\0\x80\xFD[\x86` \x82`\x07\x1B\x85\x01\x01\x11\x15a\x1C\xA8W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x1C\xCCW`\0\x80\xFD[P5\x91\x90PV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a\x1C\xE7W`\0\x80\xFD[`\x80\x81\x12\x15a\x1C\xF5W`\0\x80\xFD[P`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1D\x19Wa\x1D\x19a\x1APV[`@\x90\x81R\x845\x82R` \x80\x86\x015\x90\x83\x01R\x84\x81\x015\x90\x82\x01R``\x80\x85\x015\x90\x82\x01R\x94`\x80\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1D\x89Wa\x1D\x89a\x1DaV[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x1D\xA3Wa\x1D\xA3a\x1DaV[\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x81Q`\0[\x81\x81\x10\x15a\x1D\xE0W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x1D\xC6V[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x1E\x04a\x1D\xFE\x83\x86a\x1D\xBFV[\x84a\x1D\xBFV[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1E?WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1D\xA3Wa\x1D\xA3a\x1DaV[`\0a\x1Ec\x82\x85a\x1D\xBFV[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a\x1E\x8F\x82\x84a\x1D\xBFV[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a\x1E\xA9\x82\x85a\x1D\xBFV[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a\x1E\xCE\x82\x85a\x1D\xBFV[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a\x1E\xF3\x82\x86a\x1D\xBFV[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1D\xA3Wa\x1D\xA3a\x1DaV[`\x01\x81\x81[\x80\x85\x11\x15a\x1FmW\x81`\0\x19\x04\x82\x11\x15a\x1FSWa\x1FSa\x1DaV[\x80\x85\x16\x15a\x1F`W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x1F7V[P\x92P\x92\x90PV[`\0\x82a\x1F\x84WP`\x01a\x1D\xA3V[\x81a\x1F\x91WP`\0a\x1D\xA3V[\x81`\x01\x81\x14a\x1F\xA7W`\x02\x81\x14a\x1F\xB1Wa\x1F\xCDV[`\x01\x91PPa\x1D\xA3V[`\xFF\x84\x11\x15a\x1F\xC2Wa\x1F\xC2a\x1DaV[PP`\x01\x82\x1Ba\x1D\xA3V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1F\xF0WP\x81\x81\na\x1D\xA3V[a\x1F\xFA\x83\x83a\x1F2V[\x80`\0\x19\x04\x82\x11\x15a \x0EWa \x0Ea\x1DaV[\x02\x93\x92PPPV[`\0a \"\x83\x83a\x1FuV[\x93\x92PPPV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 A\xD8\x01\x7F4\xAF\x9E5\xC3tP!D5.:H\x1C\x01\xC4\x1E\x82\xABM\xDCQ\x95\xB9\xBF\xFA\xF0\x1DdsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static HOTSHOT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cI\xCE\x89\x97\x11a\0[W\x80cI\xCE\x89\x97\x14a\0\xC6W\x80cg\xA2\x1Ep\x14a\0\xE6W\x80c\xF1\xF4]\x99\x14a\x010W\x80c\xF4O\xF7\x12\x14a\x01CW`\0\x80\xFD[\x80c\x03@\x96\x1E\x14a\0\x82W\x80c\n2\x1C\xFF\x14a\0\x97W\x80c&\x83=\xCC\x14a\0\xAAW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x1BuV[a\x01LV[\0[a\0\x95a\0\xA56`\x04a\x1CEV[a\x03\xD7V[a\0\xB3a\x01\xF4\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB3a\0\xD46`\x04a\x1C\xBAV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\0\xF9a\0\xF46`\x04a\x1C\xBAV[a\x05FV[`@\x80Q\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R\x83\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x92\x83\x01Q\x92\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x01a\0\xBDV[a\0\x95a\x01>6`\x04a\x1C\xD3V[a\x05\xDAV[a\0\xB3`\x01T\x81V[`\x03T\x82Q\x11\x15a\x01\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqbitmap is too long`p\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x81Q\x81\x10a\x01\xAEWa\x01\xAEa\x1DKV[` \x02` \x01\x01Q\x15\x80\x15a\x01\xC3WP\x82Q\x81\x10[\x15a\x01\xDAW\x80a\x01\xD2\x81a\x1DwV[\x91PPa\x01\x9CV[\x82Q\x81\x10a\x01\xFBW`@QcKe\x82-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81[\x84Q\x81\x10\x15a\x02KW\x84\x81\x81Q\x81\x10a\x02\x1AWa\x02\x1Aa\x1DKV[` \x02` \x01\x01Q\x15a\x02CW`\0\x81\x81R`\x02` R`@\x90 Ta\x02@\x90\x83a\x1D\x90V[\x91P[`\x01\x01a\x01\xFFV[P\x82\x81\x10\x15a\x02mW`@Qc<)\x0BS`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x03\x83\x81T\x81\x10a\x02\x82Wa\x02\x82a\x1DKV[\x90`\0R` `\0 \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPP\x90P`\0\x83`\x01a\x02\xD4\x91\x90a\x1D\x90V[\x90P[\x85Q\x81\x10\x15a\x03\xC2W\x85\x81\x81Q\x81\x10a\x02\xF2Wa\x02\xF2a\x1DKV[` \x02` \x01\x01Q\x15a\x03\xBAW`\0`\x03\x82\x81T\x81\x10a\x03\x14Wa\x03\x14a\x1DKV[`\0\x91\x82R` \x80\x83 `@\x80Q`\x80\x81\x01\x82R`\x04\x90\x94\x02\x90\x91\x01\x80T\x80\x85R`\x01\x82\x01T\x85\x85\x01\x81\x90R`\x02\x83\x01T\x86\x85\x01\x81\x90R`\x03\x90\x93\x01T``\x80\x88\x01\x82\x90R\x8BQ\x96\x8C\x01Q\x95\x8C\x01Q\x90\x8C\x01Q\x97\x99P\x95\x97\x94\x96\x94\x92\x93\x91\x92\x80\x80\x80a\x03\x86\x8B\x8D\x8B\x8D\x8B\x8D\x8B\x8Da\x07\x02V[`@\x80Q`\x80\x81\x01\x82R\x93\x84R` \x84\x01\x94\x90\x94R\x92\x82\x01\x92\x90\x92R``\x81\x01\x91\x90\x91R\x9EPPPPPPPPPPPPPP[`\x01\x01a\x02\xD7V[Pa\x03\xCE\x87\x87\x83a\x08OV[PPPPPPPV[a\x01\xF4\x81\x11\x15a\x03\xFDW`@Qc\xE0\x82\x84\x0B`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x01\x90V[`\x01T`\0[\x82\x81\x10\x15a\x05\x06W`\x01T\x84\x84\x83\x81\x81\x10a\x04 Wa\x04 a\x1DKV[\x90P`\x80\x02\x01`\0\x015\x14a\x04rW\x83\x83\x82\x81\x81\x10a\x04AWa\x04Aa\x1DKV[\x90P`\x80\x02\x01`\0\x015`\x01T`@Qc4\xE4#\xFF`\xE0\x1B\x81R`\x04\x01a\x01\x90\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[a\x04\x90\x84\x84\x83\x81\x81\x10a\x04\x87Wa\x04\x87a\x1DKV[\x90PPP`\x01\x90V[a\x04\xB3W`\x01T`@Qcx\x18g\x19`\xE0\x1B\x81R`\x04\x01a\x01\x90\x91\x81R` \x01\x90V[\x83\x83\x82\x81\x81\x10a\x04\xC5Wa\x04\xC5a\x1DKV[\x90P`\x80\x02\x01` \x015`\0\x80`\x01T\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x80`\0\x82\x82Ta\x04\xF9\x91\x90a\x1D\x90V[\x90\x91UPP`\x01\x01a\x04\x03V[P`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x82\x03\xA2\x1EO\x95\xF7.P\x81\xD5\xE0\x92\x9B\x1A\x8CR\x14\x1E\x12?\x9A\x14\xE1\xE7K\x02`\xFA_R\xF1\x91\x01[`@Q\x80\x91\x03\x90\xA1PPPV[a\x05q`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0`\x03\x83\x81T\x81\x10a\x05\x86Wa\x05\x86a\x1DKV[`\0\x91\x82R` \x80\x83 \x95\x83R`\x02\x80\x82R`@\x93\x84\x90 T\x84Q`\x80\x81\x01\x86R`\x04\x90\x94\x02\x90\x97\x01\x80T\x84R`\x01\x81\x01T\x92\x84\x01\x92\x90\x92R\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x90\x91\x01T``\x82\x01R\x93\x91PPV[`\x03\x80T`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x90U\x84T`\x01\x81\x01\x86U\x94\x90\x92R\x85Q`\x04\x90\x94\x02\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x81\x01\x85\x90U\x86\x82\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8\\\x83\x01U\x87\x84\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8]\x84\x01U``\x80\x8A\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8^\x90\x95\x01\x94\x90\x94U\x85Q\x97\x88R\x91Q\x93\x87\x01\x93\x90\x93R\x91Q\x92\x85\x01\x92\x90\x92R\x90Q\x90\x83\x01R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x81\x90R\x90\x7F\xD7/\xE1\xACW\xD3\xE6\xD5\x1C\x92*\xE4\xD8\x11\xCCP\xAA:\xD7\x02b\x83\xAE\xA67IJ\x072RVZ\x90`\xC0\x01a\x059V[`\0\x80\x80\x80\x8B\x15\x80\x15a\x07\x13WP\x8A\x15[\x80\x15a\x07\x1DWP\x89\x15[\x80\x15a\x07'WP\x88\x15[\x15a\x07xW\x87\x15\x80\x15a\x078WP\x86\x15[\x80\x15a\x07BWP\x85\x15[\x80\x15a\x07LWP\x84\x15[a\x07hWa\x07\\\x88\x88\x88\x88a\t\x01V[a\x07hWa\x07ha\x1D\xA9V[P\x86\x92P\x85\x91P\x84\x90P\x83a\x08@V[\x87\x15\x80\x15a\x07\x84WP\x86\x15[\x80\x15a\x07\x8EWP\x85\x15[\x80\x15a\x07\x98WP\x84\x15[\x15a\x07\xC5Wa\x07\xA9\x8C\x8C\x8C\x8Ca\t\x01V[a\x07\xB5Wa\x07\xB5a\x1D\xA9V[P\x8A\x92P\x89\x91P\x88\x90P\x87a\x08@V[a\x07\xD1\x8C\x8C\x8C\x8Ca\t\x01V[a\x07\xDDWa\x07\xDDa\x1D\xA9V[a\x07\xE9\x88\x88\x88\x88a\t\x01V[a\x07\xF5Wa\x07\xF5a\x1D\xA9V[`\0a\x08\x0F\x8D\x8D\x8D\x8D`\x01`\0\x8F\x8F\x8F\x8F`\x01`\0a\t\xB6V[\x80Q` \x82\x01Q`@\x83\x01Q``\x84\x01Q`\x80\x85\x01Q`\xA0\x86\x01Q\x95\x96Pa\x086\x95a\x0C?V[\x94P\x94P\x94P\x94PP[\x98P\x98P\x98P\x98\x94PPPPPV[a\x08X\x82a\x0C\x89V[`\0`@Q\x80``\x01`@R\x80`$\x81R` \x01a *`$\x919\x90P`\0\x84\x82`@Q` \x01a\x08\x8A\x92\x91\x90a\x1D\xEFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80a\x08\xA7\x83a\r\x18V[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91Pa\x08\xDB\x81\x87a\x08\xCE\x8Aa\r\xEBV[a\x08\xD6a\x0EfV[a\x0F7V[a\x08\xF7W`@Qb\xCE\xD3\xE5`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0\x80`\0\x80`\0a\t\x15\x87\x87\x89\x89a\x10\x19V[\x90\x94P\x92Pa\t&\x89\x89\x81\x81a\x10\x19V[\x90\x92P\x90Pa\t7\x82\x82\x8B\x8Ba\x10\x19V[\x90\x92P\x90Pa\tH\x84\x84\x84\x84a\x10\x8AV[\x90\x94P\x92Pa\t\x98\x84\x84\x7F+\x14\x9D@\xCE\xB8\xAA\xAE\x81\xBE\x18\x99\x1B\xE0j\xC3\xB5\xB4\xC5\xE5Y\xDB\xEF\xA32g\xE6\xDC$\xA18\xE5~\x97\x13\xB0:\xF0\xFE\xD4\xCD,\xAF\xAD\xEE\xD8\xFD\xF4\xA7O\xA0\x84\xE5-\x18R\xE4\xA2\xBD\x06\x85\xC3\x15\xD2a\x10\x8AV[\x90\x94P\x92P\x83\x15\x80\x15a\t\xA9WP\x82\x15[\x99\x98PPPPPPPPPV[a\t\xBEa\x1A2V[\x88\x15\x80\x15a\t\xCAWP\x87\x15[\x15a\n\x0CW\x86\x86\x86\x86\x86\x86\x86`\0[`\xA0\x89\x01\x92\x90\x92R`\x80\x88\x01\x92\x90\x92R``\x87\x01\x92\x90\x92R`@\x86\x01\x92\x90\x92R` \x85\x81\x01\x93\x90\x93R\x90\x91\x02\x01Ra\x0C/V[\x82\x15\x80\x15a\n\x18WP\x81\x15[\x15a\n+W\x8C\x8C\x8C\x8C\x8C\x8C\x86`\0a\t\xD9V[a\n7\x85\x85\x8B\x8Ba\x10\x19V[\x90\x95P\x93Pa\nH\x8B\x8B\x85\x85a\x10\x19V[``\x83\x01R`@\x82\x01Ra\n^\x87\x87\x8B\x8Ba\x10\x19V[\x90\x97P\x95Pa\no\x8D\x8D\x85\x85a\x10\x19V[`\xA0\x83\x01R`\x80\x82\x01\x81\x90R\x87\x14\x80\x15a\n\x8CWP`\xA0\x81\x01Q\x86\x14[\x15a\n\xD1W`@\x81\x01Q\x85\x14\x80\x15a\n\xA7WP``\x81\x01Q\x84\x14[\x15a\n\xC2Wa\n\xBA\x8D\x8D\x8D\x8D\x8D\x8Da\x10\xCCV[\x86`\0a\t\xD9V[`\x01`\0\x81\x81\x80\x80\x86\x81a\t\xD9V[a\n\xDD\x89\x89\x85\x85a\x10\x19V[\x90\x93P\x91Pa\n\xFD\x85\x85\x83`\x02` \x02\x01Q\x84`\x03[` \x02\x01Qa\x10\x8AV[\x90\x9DP\x9BPa\x0B\x17\x87\x87\x83`\x04` \x02\x01Q\x84`\x05a\n\xF3V[\x90\x9BP\x99Pa\x0B(\x8B\x8B\x81\x81a\x10\x19V[\x90\x99P\x97Pa\x0BH\x89\x89\x83`\x04` \x02\x01Q\x84`\x05[` \x02\x01Qa\x10\x19V[\x90\x95P\x93Pa\x0BY\x89\x89\x8D\x8Da\x10\x19V[\x90\x99P\x97Pa\x0Bj\x89\x89\x85\x85a\x10\x19V[`\xA0\x83\x01R`\x80\x82\x01Ra\x0B\x80\x8D\x8D\x81\x81a\x10\x19V[\x90\x97P\x95Pa\x0B\x91\x87\x87\x85\x85a\x10\x19V[\x90\x97P\x95Pa\x0B\xA2\x87\x87\x8B\x8Ba\x10\x8AV[\x90\x97P\x95Pa\x0B\xB3\x85\x85`\x02a\x12;V[\x90\x93P\x91Pa\x0B\xC4\x87\x87\x85\x85a\x10\x8AV[\x90\x97P\x95Pa\x0B\xD5\x8B\x8B\x89\x89a\x10\x19V[` \x83\x01R\x81Ra\x0B\xE8\x85\x85\x89\x89a\x10\x8AV[\x90\x9BP\x99Pa\x0B\xF9\x8D\x8D\x8D\x8Da\x10\x19V[\x90\x9BP\x99Pa\x0C\x13\x89\x89\x83`\x02` \x02\x01Q\x84`\x03a\x0B>V[\x90\x9DP\x9BPa\x0C$\x8B\x8B\x8F\x8Fa\x10\x8AV[``\x83\x01R`@\x82\x01R[\x9C\x9BPPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x80a\x0CR\x88\x88a\x12nV[\x90\x92P\x90Pa\x0Cc\x8C\x8C\x84\x84a\x10\x19V[\x90\x96P\x94Pa\x0Ct\x8A\x8A\x84\x84a\x10\x19V[\x96\x9D\x95\x9CP\x9AP\x94\x98P\x92\x96PPPPPPPV[\x80Q` \x82\x01Q`\0\x91`\0\x80Q` a N\x839\x81Q\x91R\x91\x82`\x03\x81\x80\x85\x80\t\x85\t\x08\x83\x82\x83\t\x14\x81\x15\x83\x15\x17\x19\x84\x83\x10\x85\x85\x10\x16\x16\x16\x93PPP\x81a\r\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBn254: invalid G1 point\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[PPPV[`\0\x80`\0a\r&\x84a\x12\xF9V[\x90P`\0\x80Q` a N\x839\x81Q\x91R`\x03`\0\x82\x84\x85\t\x90P\x82\x80a\rOWa\rOa\x1E\x0CV[\x84\x82\t\x90P\x82\x80a\rbWa\rba\x1E\x0CV[\x82\x82\x08\x90P`\0\x80a\rs\x83a\x15\rV[\x92P\x90P[\x80a\r\xDCW\x84\x80a\r\x8BWa\r\x8Ba\x1E\x0CV[`\x01\x87\x08\x95P\x84\x80a\r\x9FWa\r\x9Fa\x1E\x0CV[\x86\x87\t\x92P\x84\x80a\r\xB2Wa\r\xB2a\x1E\x0CV[\x86\x84\t\x92P\x84\x80a\r\xC5Wa\r\xC5a\x1E\x0CV[\x84\x84\x08\x92Pa\r\xD3\x83a\x15\rV[\x92P\x90Pa\rxV[P\x93\x97\x93\x96P\x92\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q` \x83\x01Q\x15\x90\x15\x16\x15a\x0E\x13WP\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a N\x839\x81Q\x91R\x84` \x01Qa\x0EF\x91\x90a\x1E\"V[a\x0E^\x90`\0\x80Q` a N\x839\x81Q\x91Ra\x1EDV[\x90R\x92\x91PPV[a\x0E\x91`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R` \x01\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED\x81R` \x01\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[\x81R` \x01\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAA\x81RP\x90P\x90V[`\0\x80`\0`@Q\x87Q\x81R` \x88\x01Q` \x82\x01R\x86Q`@\x82\x01R` \x87\x01Q``\x82\x01R`@\x87\x01Q`\x80\x82\x01R``\x87\x01Q`\xA0\x82\x01R\x85Q`\xC0\x82\x01R` \x86\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R` \x85\x01Qa\x01 \x82\x01R`@\x85\x01Qa\x01@\x82\x01R``\x85\x01Qa\x01`\x82\x01R` `\0a\x01\x80\x83`\x08Z\xFA\x91PP`\0Q\x91P\x80a\x10\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FBn254: Pairing check failed!\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[P\x15\x15\x95\x94PPPPPV[`\0\x80a\x10W`\0\x80Q` a N\x839\x81Q\x91R\x85\x88\t`\0\x80Q` a N\x839\x81Q\x91R\x85\x88\t`\0\x80Q` a N\x839\x81Q\x91Ra\x16\x08V[`\0\x80Q` a N\x839\x81Q\x91R\x80\x86\x88\t`\0\x80Q` a N\x839\x81Q\x91R\x86\x8A\t\x08\x91P\x91P\x94P\x94\x92PPPV[`\0\x80a\x10\xA6\x86\x85`\0\x80Q` a N\x839\x81Q\x91Ra\x16\x08V[a\x10\xBF\x86\x85`\0\x80Q` a N\x839\x81Q\x91Ra\x16\x08V[\x91P\x91P\x94P\x94\x92PPPV[`\0\x80`\0\x80`\0\x80a\x10\xE1\x8C\x8C`\x03a\x12;V[\x90\x96P\x94Pa\x10\xF2\x86\x86\x8E\x8Ea\x10\x19V[\x90\x96P\x94Pa\x11\x03\x8A\x8A\x8A\x8Aa\x10\x19V[\x90\x98P\x96Pa\x11\x14\x8C\x8C\x8C\x8Ca\x10\x19V[\x90\x94P\x92Pa\x11%\x84\x84\x8A\x8Aa\x10\x19V[\x90\x94P\x92Pa\x116\x86\x86\x81\x81a\x10\x19V[\x90\x9CP\x9APa\x11G\x84\x84`\x08a\x12;V[\x90\x92P\x90Pa\x11X\x8C\x8C\x84\x84a\x10\x8AV[\x90\x9CP\x9APa\x11i\x88\x88\x81\x81a\x10\x19V[\x90\x92P\x90Pa\x11z\x84\x84`\x04a\x12;V[\x90\x94P\x92Pa\x11\x8B\x84\x84\x8E\x8Ea\x10\x8AV[\x90\x94P\x92Pa\x11\x9C\x84\x84\x88\x88a\x10\x19V[\x90\x94P\x92Pa\x11\xAD\x8A\x8A`\x08a\x12;V[\x90\x96P\x94Pa\x11\xBE\x86\x86\x8C\x8Ca\x10\x19V[\x90\x96P\x94Pa\x11\xCF\x86\x86\x84\x84a\x10\x19V[\x90\x96P\x94Pa\x11\xE0\x84\x84\x88\x88a\x10\x8AV[\x90\x94P\x92Pa\x11\xF1\x8C\x8C`\x02a\x12;V[\x90\x96P\x94Pa\x12\x02\x86\x86\x8A\x8Aa\x10\x19V[\x90\x96P\x94Pa\x12\x13\x88\x88\x84\x84a\x10\x19V[\x90\x92P\x90Pa\x12$\x82\x82`\x08a\x12;V[\x80\x92P\x81\x93PPP\x96P\x96P\x96P\x96P\x96P\x96\x90PV[`\0\x80`\0\x80Q` a N\x839\x81Q\x91R\x83\x86\t`\0\x80Q` a N\x839\x81Q\x91R\x84\x86\t\x91P\x91P\x93P\x93\x91PPV[`\0\x80\x80a\x12\xAF`\0\x80Q` a N\x839\x81Q\x91R\x80\x87\x88\t`\0\x80Q` a N\x839\x81Q\x91R\x87\x88\t\x08`\0\x80Q` a N\x839\x81Q\x91Ra\x16,V[\x90P`\0\x80Q` a N\x839\x81Q\x91R\x81\x86\t`\0\x80Q` a N\x839\x81Q\x91R\x82\x86\ta\x12\xED\x90`\0\x80Q` a N\x839\x81Q\x91Ra\x1EDV[\x92P\x92PP\x92P\x92\x90PV[`\0\x80a\x13\x05\x83a\x16}V[\x80Q\x90\x91P`0\x81\x14a\x13\x1AWa\x13\x1Aa\x1D\xA9V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x135Wa\x135a\x1APV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x13_W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x13\xD0W\x83`\x01a\x13z\x83\x86a\x1EDV[a\x13\x84\x91\x90a\x1EDV[\x81Q\x81\x10a\x13\x94Wa\x13\x94a\x1DKV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x13\xB1Wa\x13\xB1a\x1DKV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x13eV[P`@\x80Q`\x1F\x80\x82Ra\x04\0\x82\x01\x90\x92R`\0\x90\x82` \x82\x01a\x03\xE0\x806\x837\x01\x90PP\x90P`\0[\x82\x81\x10\x15a\x14bW\x83\x81a\x14\x0E\x85\x88a\x1EDV[a\x14\x18\x91\x90a\x1D\x90V[\x81Q\x81\x10a\x14(Wa\x14(a\x1DKV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x82\x82\x81Q\x81\x10a\x14HWa\x14Ha\x1DKV[`\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x13\xFAV[P`\0a\x14n\x82a\x19\xD1V[\x90Pa\x01\0`\0\x80Q` a N\x839\x81Q\x91R`\0a\x14\x8E\x86\x89a\x1EDV[\x90P`\0[\x81\x81\x10\x15a\x14\xFDW`\0\x88`\x01a\x14\xAA\x84\x86a\x1EDV[a\x14\xB4\x91\x90a\x1EDV[\x81Q\x81\x10a\x14\xC4Wa\x14\xC4a\x1DKV[\x01` \x01Q`\xF8\x1C\x90P\x83\x80a\x14\xDCWa\x14\xDCa\x1E\x0CV[\x85\x87\t\x95P\x83\x80a\x14\xEFWa\x14\xEFa\x1E\x0CV[\x81\x87\x08\x95PP`\x01\x01a\x14\x93V[P\x92\x9A\x99PPPPPPPPPPV[`\0\x80`\0\x80\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R\x90P`\0`\0\x80Q` a N\x839\x81Q\x91R\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x86``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` `\0`\xC0\x83`\x05Z\xFA\x93PP`\0Q\x93P\x82a\x15\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fpow precompile call failed!\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x90V[\x80`\x01\x85\x90\x1B\x11\x15a\x15\xE8Wa\x15\xE5\x84\x82a\x1EDV[\x93P[\x80\x80a\x15\xF6Wa\x15\xF6a\x1E\x0CV[\x84\x85\t\x91P\x85\x82\x14\x94PPPP\x91P\x91V[`\0\x81\x80a\x16\x18Wa\x16\x18a\x1E\x0CV[a\x16\"\x84\x84a\x1EDV[\x85\x08\x94\x93PPPPV[`\0\x80`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01R`\x02\x84\x03`\x80\x82\x01R\x83`\xA0\x82\x01R` \x81`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x90Q\x92P\x90P\x80a\x16vW`\0\x80\xFD[P\x92\x91PPV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R\x90` \x90`\x01`\xF9\x1B\x90`\0\x90\x84` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x80\x86`@Q` \x01a\x16\xBE\x92\x91\x90a\x1D\xEFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80\x84`\xF8\x1B`@Q` \x01a\x16\xE5\x92\x91\x90a\x1EWV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80`@Q` \x01a\x17\x07\x91\x90a\x1E\x83V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x91Pa\x01\x01`\xF0\x1B\x90a\x171\x90\x83\x90\x83\x90` \x01a\x1E\x9DV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x81\x84\x01\x81\x90R`\x01`\xF8\x1B\x84\x84\x01R`\x01`\x01`\xF0\x1B\x03\x19\x85\x16`A\x85\x01R\x82Q`#\x81\x86\x03\x01\x81R`C\x90\x94\x01\x90\x92R\x82Q\x90\x83\x01 \x91\x93P\x90`\0`\xFF\x88\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\xA2Wa\x17\xA2a\x1APV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x17\xCCW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x82`@Q` \x01a\x17\xE4\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0[\x81Q\x81\x10\x15a\x18OW\x81\x81\x81Q\x81\x10a\x18\x13Wa\x18\x13a\x1DKV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\x180Wa\x180a\x1DKV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x17\xF8V[P`\0\x84`@Q` \x01a\x18e\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` \x83\x01\x90\x91R`\0\x80\x83R\x91\x98P\x91P[\x89\x81\x10\x15a\x18\xF9W`\0\x83\x82\x81Q\x81\x10a\x18\xA0Wa\x18\xA0a\x1DKV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x18\xBDWa\x18\xBDa\x1DKV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x18\x90P\x88\x81`@Q` \x01a\x18\xDE\x92\x91\x90a\x1E\xC2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x98PP`\x01\x01a\x18\x84V[P\x86\x88\x87`@Q` \x01a\x19\x0F\x93\x92\x91\x90a\x1E\xE7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P\x86\x80Q\x90` \x01 \x93P\x83`@Q` \x01a\x19=\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\0[a\x19^\x8A`\xFF\x8D\x16a\x1EDV[\x81\x10\x15a\x19\xC0W\x82\x81\x81Q\x81\x10a\x19wWa\x19wa\x1DKV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x84a\x19\x91\x83\x8Da\x1D\x90V[\x81Q\x81\x10a\x19\xA1Wa\x19\xA1a\x1DKV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x19QV[P\x91\x9B\x9APPPPPPPPPPPV[`\0\x80\x80[\x83Q\x81\x10\x15a\x16vW\x83\x81\x81Q\x81\x10a\x19\xF1Wa\x19\xF1a\x1DKV[` \x02` \x01\x01Q`\xFF\x16\x81`\x08a\x1A\t\x91\x90a\x1F\x1BV[a\x1A\x14\x90`\x02a \x16V[a\x1A\x1E\x91\x90a\x1F\x1BV[a\x1A(\x90\x83a\x1D\x90V[\x91P`\x01\x01a\x19\xD6V[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A\x8FWa\x1A\x8Fa\x1APV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x1A\xA9W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1A\xCCWa\x1A\xCCa\x1APV[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1A\xF7W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1B\x13Wa\x1B\x13a\x1APV[\x81`\x05\x1Ba\x1B\"\x82\x82\x01a\x1AfV[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x1B<W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x1BjW\x825\x80\x15\x15\x81\x14a\x1B[W`\0\x80\x81\xFD[\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x1BBV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\xA0\x85\x87\x03\x12\x15a\x1B\x8BW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\xA3W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x1B\xB7W`\0\x80\xFD[\x815` \x82\x82\x11\x15a\x1B\xCBWa\x1B\xCBa\x1APV[a\x1B\xDD`\x1F\x83\x01`\x1F\x19\x16\x82\x01a\x1AfV[\x82\x81R\x8A\x82\x84\x87\x01\x01\x11\x15a\x1B\xF1W`\0\x80\xFD[\x82\x82\x86\x01\x83\x83\x017`\0\x81\x84\x01\x83\x01R\x97Pa\x1C\x0F\x8A\x8A\x83\x01a\x1A\x97V[\x96PPP``\x87\x015\x91P\x80\x82\x11\x15a\x1C'W`\0\x80\xFD[Pa\x1C4\x87\x82\x88\x01a\x1A\xE6V[\x94\x97\x93\x96P\x93\x94`\x80\x015\x93PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x1CXW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1CpW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x1C\x84W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1C\x93W`\0\x80\xFD[\x86` \x82`\x07\x1B\x85\x01\x01\x11\x15a\x1C\xA8W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x1C\xCCW`\0\x80\xFD[P5\x91\x90PV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a\x1C\xE7W`\0\x80\xFD[`\x80\x81\x12\x15a\x1C\xF5W`\0\x80\xFD[P`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1D\x19Wa\x1D\x19a\x1APV[`@\x90\x81R\x845\x82R` \x80\x86\x015\x90\x83\x01R\x84\x81\x015\x90\x82\x01R``\x80\x85\x015\x90\x82\x01R\x94`\x80\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1D\x89Wa\x1D\x89a\x1DaV[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x1D\xA3Wa\x1D\xA3a\x1DaV[\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x81Q`\0[\x81\x81\x10\x15a\x1D\xE0W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x1D\xC6V[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x1E\x04a\x1D\xFE\x83\x86a\x1D\xBFV[\x84a\x1D\xBFV[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1E?WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1D\xA3Wa\x1D\xA3a\x1DaV[`\0a\x1Ec\x82\x85a\x1D\xBFV[`\0\x81R`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16`\x01\x84\x01RPP`\x02\x01\x91\x90PV[`\0a\x1E\x8F\x82\x84a\x1D\xBFV[`\0\x81R`\x01\x01\x93\x92PPPV[`\0a\x1E\xA9\x82\x85a\x1D\xBFV[`\x01`\x01`\xF0\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x02\x01\x91\x90PV[`\0a\x1E\xCE\x82\x85a\x1D\xBFV[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x01\x01\x91\x90PV[`\0a\x1E\xF3\x82\x86a\x1D\xBFV[`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84RPP`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01\x82\x01R`\x03\x01\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1D\xA3Wa\x1D\xA3a\x1DaV[`\x01\x81\x81[\x80\x85\x11\x15a\x1FmW\x81`\0\x19\x04\x82\x11\x15a\x1FSWa\x1FSa\x1DaV[\x80\x85\x16\x15a\x1F`W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x1F7V[P\x92P\x92\x90PV[`\0\x82a\x1F\x84WP`\x01a\x1D\xA3V[\x81a\x1F\x91WP`\0a\x1D\xA3V[\x81`\x01\x81\x14a\x1F\xA7W`\x02\x81\x14a\x1F\xB1Wa\x1F\xCDV[`\x01\x91PPa\x1D\xA3V[`\xFF\x84\x11\x15a\x1F\xC2Wa\x1F\xC2a\x1DaV[PP`\x01\x82\x1Ba\x1D\xA3V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1F\xF0WP\x81\x81\na\x1D\xA3V[a\x1F\xFA\x83\x83a\x1F2V[\x80`\0\x19\x04\x82\x11\x15a \x0EWa \x0Ea\x1DaV[\x02\x93\x92PPPV[`\0a \"\x83\x83a\x1FuV[\x93\x92PPPV\xFEBLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 A\xD8\x01\x7F4\xAF\x9E5\xC3tP!D5.:H\x1C\x01\xC4\x1E\x82\xABM\xDCQ\x95\xB9\xBF\xFA\xF0\x1DdsolcC\0\x08\x17\x003";
    /// The deployed bytecode of the contract.
    pub static HOTSHOT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct HotShot<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for HotShot<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for HotShot<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for HotShot<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for HotShot<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(HotShot))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> HotShot<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                HOTSHOT_ABI.clone(),
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
                HOTSHOT_ABI.clone(),
                HOTSHOT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MAX_BLOCKS` (0x26833dcc) function
        pub fn max_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([38, 131, 61, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addNewStakingKey` (0xf1f45d99) function
        pub fn add_new_staking_key(
            &self,
            staking_key: G2Point,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 244, 93, 153], (staking_key, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blockHeight` (0xf44ff712) function
        pub fn block_height(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([244, 79, 247, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commitments` (0x49ce8997) function
        pub fn commitments(
            &self,
            block_height: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([73, 206, 137, 151], block_height)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakingKey` (0x67a21e70) function
        pub fn get_staking_key(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (G2Point, ::ethers::core::types::U256)>
        {
            self.0
                .method_hash([103, 162, 30, 112], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `newBlocks` (0x0a321cff) function
        pub fn new_blocks(
            &self,
            qcs: ::std::vec::Vec<Qc>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 50, 28, 255], qcs)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyAggSig` (0x0340961e) function
        pub fn verify_agg_sig(
            &self,
            message: ::ethers::core::types::Bytes,
            sig: G1Point,
            bitmap: ::std::vec::Vec<bool>,
            min_stake_threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [3, 64, 150, 30],
                    (message, sig, bitmap, min_stake_threshold),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `NewBlocks` event
        pub fn new_blocks_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewBlocksFilter> {
            self.0.event()
        }
        ///Gets the contract's `NewStakingKey` event
        pub fn new_staking_key_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewStakingKeyFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, HotShotEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for HotShot<M> {
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
    ///Custom Error type `IncorrectBlockNumber` with signature `IncorrectBlockNumber(uint256,uint256)` and selector `0x34e423ff`
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
        name = "IncorrectBlockNumber",
        abi = "IncorrectBlockNumber(uint256,uint256)"
    )]
    pub struct IncorrectBlockNumber {
        pub block_number: ::ethers::core::types::U256,
        pub expected_block_number: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidQC` with signature `InvalidQC(uint256)` and selector `0x78186719`
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
    #[etherror(name = "InvalidQC", abi = "InvalidQC(uint256)")]
    pub struct InvalidQC {
        pub block_number: ::ethers::core::types::U256,
    }
    ///Custom Error type `NoKeySelected` with signature `NoKeySelected()` and selector `0x96cb045a`
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
    #[etherror(name = "NoKeySelected", abi = "NoKeySelected()")]
    pub struct NoKeySelected;
    ///Custom Error type `NotEnoughStake` with signature `NotEnoughStake()` and selector `0xf0a42d4c`
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
    #[etherror(name = "NotEnoughStake", abi = "NotEnoughStake()")]
    pub struct NotEnoughStake;
    ///Custom Error type `TooManyBlocks` with signature `TooManyBlocks(uint256)` and selector `0xe082840b`
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
    #[etherror(name = "TooManyBlocks", abi = "TooManyBlocks(uint256)")]
    pub struct TooManyBlocks {
        pub num_blocks: ::ethers::core::types::U256,
    }
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
    pub enum HotShotErrors {
        BLSSigVerificationFailed(BLSSigVerificationFailed),
        IncorrectBlockNumber(IncorrectBlockNumber),
        InvalidQC(InvalidQC),
        NoKeySelected(NoKeySelected),
        NotEnoughStake(NotEnoughStake),
        TooManyBlocks(TooManyBlocks),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for HotShotErrors {
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
                <IncorrectBlockNumber as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncorrectBlockNumber(decoded));
            }
            if let Ok(decoded) = <InvalidQC as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidQC(decoded));
            }
            if let Ok(decoded) = <NoKeySelected as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoKeySelected(decoded));
            }
            if let Ok(decoded) = <NotEnoughStake as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotEnoughStake(decoded));
            }
            if let Ok(decoded) = <TooManyBlocks as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TooManyBlocks(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HotShotErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BLSSigVerificationFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncorrectBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidQC(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NoKeySelected(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotEnoughStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TooManyBlocks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for HotShotErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BLSSigVerificationFailed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <IncorrectBlockNumber as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidQC as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NoKeySelected as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NotEnoughStake as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <TooManyBlocks as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for HotShotErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BLSSigVerificationFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncorrectBlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidQC(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoKeySelected(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotEnoughStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::TooManyBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for HotShotErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BLSSigVerificationFailed> for HotShotErrors {
        fn from(value: BLSSigVerificationFailed) -> Self {
            Self::BLSSigVerificationFailed(value)
        }
    }
    impl ::core::convert::From<IncorrectBlockNumber> for HotShotErrors {
        fn from(value: IncorrectBlockNumber) -> Self {
            Self::IncorrectBlockNumber(value)
        }
    }
    impl ::core::convert::From<InvalidQC> for HotShotErrors {
        fn from(value: InvalidQC) -> Self {
            Self::InvalidQC(value)
        }
    }
    impl ::core::convert::From<NoKeySelected> for HotShotErrors {
        fn from(value: NoKeySelected) -> Self {
            Self::NoKeySelected(value)
        }
    }
    impl ::core::convert::From<NotEnoughStake> for HotShotErrors {
        fn from(value: NotEnoughStake) -> Self {
            Self::NotEnoughStake(value)
        }
    }
    impl ::core::convert::From<TooManyBlocks> for HotShotErrors {
        fn from(value: TooManyBlocks) -> Self {
            Self::TooManyBlocks(value)
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
    #[ethevent(name = "NewBlocks", abi = "NewBlocks(uint256,uint256)")]
    pub struct NewBlocksFilter {
        pub first_block_number: ::ethers::core::types::U256,
        pub num_blocks: ::ethers::core::types::U256,
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
        name = "NewStakingKey",
        abi = "NewStakingKey((uint256,uint256,uint256,uint256),uint256,uint256)"
    )]
    pub struct NewStakingKeyFilter {
        pub staking_key: G2Point,
        pub amount: ::ethers::core::types::U256,
        pub index: ::ethers::core::types::U256,
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
    pub enum HotShotEvents {
        NewBlocksFilter(NewBlocksFilter),
        NewStakingKeyFilter(NewStakingKeyFilter),
    }
    impl ::ethers::contract::EthLogDecode for HotShotEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = NewBlocksFilter::decode_log(log) {
                return Ok(HotShotEvents::NewBlocksFilter(decoded));
            }
            if let Ok(decoded) = NewStakingKeyFilter::decode_log(log) {
                return Ok(HotShotEvents::NewStakingKeyFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for HotShotEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NewBlocksFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewStakingKeyFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<NewBlocksFilter> for HotShotEvents {
        fn from(value: NewBlocksFilter) -> Self {
            Self::NewBlocksFilter(value)
        }
    }
    impl ::core::convert::From<NewStakingKeyFilter> for HotShotEvents {
        fn from(value: NewStakingKeyFilter) -> Self {
            Self::NewStakingKeyFilter(value)
        }
    }
    ///Container type for all input parameters for the `MAX_BLOCKS` function with signature `MAX_BLOCKS()` and selector `0x26833dcc`
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
    #[ethcall(name = "MAX_BLOCKS", abi = "MAX_BLOCKS()")]
    pub struct MaxBlocksCall;
    ///Container type for all input parameters for the `addNewStakingKey` function with signature `addNewStakingKey((uint256,uint256,uint256,uint256),uint256)` and selector `0xf1f45d99`
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
        name = "addNewStakingKey",
        abi = "addNewStakingKey((uint256,uint256,uint256,uint256),uint256)"
    )]
    pub struct AddNewStakingKeyCall {
        pub staking_key: G2Point,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `blockHeight` function with signature `blockHeight()` and selector `0xf44ff712`
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
    #[ethcall(name = "blockHeight", abi = "blockHeight()")]
    pub struct BlockHeightCall;
    ///Container type for all input parameters for the `commitments` function with signature `commitments(uint256)` and selector `0x49ce8997`
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
    #[ethcall(name = "commitments", abi = "commitments(uint256)")]
    pub struct CommitmentsCall {
        pub block_height: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getStakingKey` function with signature `getStakingKey(uint256)` and selector `0x67a21e70`
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
    #[ethcall(name = "getStakingKey", abi = "getStakingKey(uint256)")]
    pub struct GetStakingKeyCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `newBlocks` function with signature `newBlocks((uint256,uint256,uint256,uint256)[])` and selector `0x0a321cff`
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
        name = "newBlocks",
        abi = "newBlocks((uint256,uint256,uint256,uint256)[])"
    )]
    pub struct NewBlocksCall {
        pub qcs: ::std::vec::Vec<Qc>,
    }
    ///Container type for all input parameters for the `verifyAggSig` function with signature `verifyAggSig(bytes,(uint256,uint256),bool[],uint256)` and selector `0x0340961e`
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
        name = "verifyAggSig",
        abi = "verifyAggSig(bytes,(uint256,uint256),bool[],uint256)"
    )]
    pub struct VerifyAggSigCall {
        pub message: ::ethers::core::types::Bytes,
        pub sig: G1Point,
        pub bitmap: ::std::vec::Vec<bool>,
        pub min_stake_threshold: ::ethers::core::types::U256,
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
    pub enum HotShotCalls {
        MaxBlocks(MaxBlocksCall),
        AddNewStakingKey(AddNewStakingKeyCall),
        BlockHeight(BlockHeightCall),
        Commitments(CommitmentsCall),
        GetStakingKey(GetStakingKeyCall),
        NewBlocks(NewBlocksCall),
        VerifyAggSig(VerifyAggSigCall),
    }
    impl ::ethers::core::abi::AbiDecode for HotShotCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MaxBlocksCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxBlocks(decoded));
            }
            if let Ok(decoded) =
                <AddNewStakingKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddNewStakingKey(decoded));
            }
            if let Ok(decoded) = <BlockHeightCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BlockHeight(decoded));
            }
            if let Ok(decoded) = <CommitmentsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Commitments(decoded));
            }
            if let Ok(decoded) = <GetStakingKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStakingKey(decoded));
            }
            if let Ok(decoded) = <NewBlocksCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NewBlocks(decoded));
            }
            if let Ok(decoded) = <VerifyAggSigCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyAggSig(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HotShotCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxBlocks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddNewStakingKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BlockHeight(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Commitments(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetStakingKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NewBlocks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyAggSig(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for HotShotCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddNewStakingKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlockHeight(element) => ::core::fmt::Display::fmt(element, f),
                Self::Commitments(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakingKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyAggSig(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxBlocksCall> for HotShotCalls {
        fn from(value: MaxBlocksCall) -> Self {
            Self::MaxBlocks(value)
        }
    }
    impl ::core::convert::From<AddNewStakingKeyCall> for HotShotCalls {
        fn from(value: AddNewStakingKeyCall) -> Self {
            Self::AddNewStakingKey(value)
        }
    }
    impl ::core::convert::From<BlockHeightCall> for HotShotCalls {
        fn from(value: BlockHeightCall) -> Self {
            Self::BlockHeight(value)
        }
    }
    impl ::core::convert::From<CommitmentsCall> for HotShotCalls {
        fn from(value: CommitmentsCall) -> Self {
            Self::Commitments(value)
        }
    }
    impl ::core::convert::From<GetStakingKeyCall> for HotShotCalls {
        fn from(value: GetStakingKeyCall) -> Self {
            Self::GetStakingKey(value)
        }
    }
    impl ::core::convert::From<NewBlocksCall> for HotShotCalls {
        fn from(value: NewBlocksCall) -> Self {
            Self::NewBlocks(value)
        }
    }
    impl ::core::convert::From<VerifyAggSigCall> for HotShotCalls {
        fn from(value: VerifyAggSigCall) -> Self {
            Self::VerifyAggSig(value)
        }
    }
    ///Container type for all return fields from the `MAX_BLOCKS` function with signature `MAX_BLOCKS()` and selector `0x26833dcc`
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
    pub struct MaxBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `blockHeight` function with signature `blockHeight()` and selector `0xf44ff712`
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
    pub struct BlockHeightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `commitments` function with signature `commitments(uint256)` and selector `0x49ce8997`
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
    pub struct CommitmentsReturn {
        pub commitment: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getStakingKey` function with signature `getStakingKey(uint256)` and selector `0x67a21e70`
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
    pub struct GetStakingKeyReturn(pub G2Point, pub ::ethers::core::types::U256);
    ///`Qc(uint256,uint256,uint256,uint256)`
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
    pub struct Qc {
        pub height: ::ethers::core::types::U256,
        pub block_commitment: ::ethers::core::types::U256,
        pub pad_1: ::ethers::core::types::U256,
        pub pad_2: ::ethers::core::types::U256,
    }
}
