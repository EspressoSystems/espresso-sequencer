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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x05\xAC\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\n2\x1C\xFF\x14a\0gW\x80c&\x83=\xCC\x14a\0|W\x80cI\xCE\x89\x97\x14a\0\x98W\x80cg\xA2\x1Ep\x14a\0\xB8W\x80c\xF1\xF4]\x99\x14a\x01\x02W\x80c\xF4O\xF7\x12\x14a\x01\x15W[`\0\x80\xFD[a\0za\0u6`\x04a\x04NV[a\x01\x1EV[\0[a\0\x85a\x01\xF4\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x85a\0\xA66`\x04a\x04\xC3V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\0\xCBa\0\xC66`\x04a\x04\xC3V[a\x02\x92V[`@\x80Q\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R\x83\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x92\x83\x01Q\x92\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x01a\0\x8FV[a\0za\x01\x106`\x04a\x04\xDCV[a\x03&V[a\0\x85`\x01T\x81V[a\x01\xF4\x81\x11\x15a\x01IW`@Qc\xE0\x82\x84\x0B`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\x01T`\0[\x82\x81\x10\x15a\x02RW`\x01T\x84\x84\x83\x81\x81\x10a\x01lWa\x01la\x05bV[\x90P`\x80\x02\x01`\0\x015\x14a\x01\xBEW\x83\x83\x82\x81\x81\x10a\x01\x8DWa\x01\x8Da\x05bV[\x90P`\x80\x02\x01`\0\x015`\x01T`@Qc4\xE4#\xFF`\xE0\x1B\x81R`\x04\x01a\x01@\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[a\x01\xDC\x84\x84\x83\x81\x81\x10a\x01\xD3Wa\x01\xD3a\x05bV[\x90PPP`\x01\x90V[a\x01\xFFW`\x01T`@Qcx\x18g\x19`\xE0\x1B\x81R`\x04\x01a\x01@\x91\x81R` \x01\x90V[\x83\x83\x82\x81\x81\x10a\x02\x11Wa\x02\x11a\x05bV[\x90P`\x80\x02\x01` \x015`\0\x80`\x01T\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x80`\0\x82\x82Ta\x02E\x91\x90a\x05xV[\x90\x91UPP`\x01\x01a\x01OV[P`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x82\x03\xA2\x1EO\x95\xF7.P\x81\xD5\xE0\x92\x9B\x1A\x8CR\x14\x1E\x12?\x9A\x14\xE1\xE7K\x02`\xFA_R\xF1\x91\x01[`@Q\x80\x91\x03\x90\xA1PPPV[a\x02\xBD`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0`\x03\x83\x81T\x81\x10a\x02\xD2Wa\x02\xD2a\x05bV[`\0\x91\x82R` \x80\x83 \x95\x83R`\x02\x80\x82R`@\x93\x84\x90 T\x84Q`\x80\x81\x01\x86R`\x04\x90\x94\x02\x90\x97\x01\x80T\x84R`\x01\x81\x01T\x92\x84\x01\x92\x90\x92R\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x90\x91\x01T``\x82\x01R\x93\x91PPV[`\x03\x80T`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x90U\x84T`\x01\x81\x01\x86U\x94\x90\x92R\x85Q`\x04\x90\x94\x02\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x81\x01\x85\x90U\x86\x82\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8\\\x83\x01U\x87\x84\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8]\x84\x01U``\x80\x8A\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8^\x90\x95\x01\x94\x90\x94U\x85Q\x97\x88R\x91Q\x93\x87\x01\x93\x90\x93R\x91Q\x92\x85\x01\x92\x90\x92R\x90Q\x90\x83\x01R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x81\x90R\x90\x7F\xD7/\xE1\xACW\xD3\xE6\xD5\x1C\x92*\xE4\xD8\x11\xCCP\xAA:\xD7\x02b\x83\xAE\xA67IJ\x072RVZ\x90`\xC0\x01a\x02\x85V[`\0\x80` \x83\x85\x03\x12\x15a\x04aW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04yW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04\x8DW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04\x9CW`\0\x80\xFD[\x86` \x82`\x07\x1B\x85\x01\x01\x11\x15a\x04\xB1W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x04\xD5W`\0\x80\xFD[P5\x91\x90PV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a\x04\xF0W`\0\x80\xFD[`\x80\x81\x12\x15a\x04\xFEW`\0\x80\xFD[P`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x050WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x90\x81R\x845\x82R` \x80\x86\x015\x90\x83\x01R\x84\x81\x015\x90\x82\x01R``\x80\x85\x015\x90\x82\x01R\x94`\x80\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x05\x99WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static HOTSHOT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\n2\x1C\xFF\x14a\0gW\x80c&\x83=\xCC\x14a\0|W\x80cI\xCE\x89\x97\x14a\0\x98W\x80cg\xA2\x1Ep\x14a\0\xB8W\x80c\xF1\xF4]\x99\x14a\x01\x02W\x80c\xF4O\xF7\x12\x14a\x01\x15W[`\0\x80\xFD[a\0za\0u6`\x04a\x04NV[a\x01\x1EV[\0[a\0\x85a\x01\xF4\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x85a\0\xA66`\x04a\x04\xC3V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\0\xCBa\0\xC66`\x04a\x04\xC3V[a\x02\x92V[`@\x80Q\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R\x83\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x92\x83\x01Q\x92\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x01a\0\x8FV[a\0za\x01\x106`\x04a\x04\xDCV[a\x03&V[a\0\x85`\x01T\x81V[a\x01\xF4\x81\x11\x15a\x01IW`@Qc\xE0\x82\x84\x0B`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\x01T`\0[\x82\x81\x10\x15a\x02RW`\x01T\x84\x84\x83\x81\x81\x10a\x01lWa\x01la\x05bV[\x90P`\x80\x02\x01`\0\x015\x14a\x01\xBEW\x83\x83\x82\x81\x81\x10a\x01\x8DWa\x01\x8Da\x05bV[\x90P`\x80\x02\x01`\0\x015`\x01T`@Qc4\xE4#\xFF`\xE0\x1B\x81R`\x04\x01a\x01@\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[a\x01\xDC\x84\x84\x83\x81\x81\x10a\x01\xD3Wa\x01\xD3a\x05bV[\x90PPP`\x01\x90V[a\x01\xFFW`\x01T`@Qcx\x18g\x19`\xE0\x1B\x81R`\x04\x01a\x01@\x91\x81R` \x01\x90V[\x83\x83\x82\x81\x81\x10a\x02\x11Wa\x02\x11a\x05bV[\x90P`\x80\x02\x01` \x015`\0\x80`\x01T\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x80`\0\x82\x82Ta\x02E\x91\x90a\x05xV[\x90\x91UPP`\x01\x01a\x01OV[P`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x82\x03\xA2\x1EO\x95\xF7.P\x81\xD5\xE0\x92\x9B\x1A\x8CR\x14\x1E\x12?\x9A\x14\xE1\xE7K\x02`\xFA_R\xF1\x91\x01[`@Q\x80\x91\x03\x90\xA1PPPV[a\x02\xBD`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0`\x03\x83\x81T\x81\x10a\x02\xD2Wa\x02\xD2a\x05bV[`\0\x91\x82R` \x80\x83 \x95\x83R`\x02\x80\x82R`@\x93\x84\x90 T\x84Q`\x80\x81\x01\x86R`\x04\x90\x94\x02\x90\x97\x01\x80T\x84R`\x01\x81\x01T\x92\x84\x01\x92\x90\x92R\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x90\x91\x01T``\x82\x01R\x93\x91PPV[`\x03\x80T`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x90U\x84T`\x01\x81\x01\x86U\x94\x90\x92R\x85Q`\x04\x90\x94\x02\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x81\x01\x85\x90U\x86\x82\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8\\\x83\x01U\x87\x84\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8]\x84\x01U``\x80\x8A\x01\x80Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8^\x90\x95\x01\x94\x90\x94U\x85Q\x97\x88R\x91Q\x93\x87\x01\x93\x90\x93R\x91Q\x92\x85\x01\x92\x90\x92R\x90Q\x90\x83\x01R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x81\x90R\x90\x7F\xD7/\xE1\xACW\xD3\xE6\xD5\x1C\x92*\xE4\xD8\x11\xCCP\xAA:\xD7\x02b\x83\xAE\xA67IJ\x072RVZ\x90`\xC0\x01a\x02\x85V[`\0\x80` \x83\x85\x03\x12\x15a\x04aW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04yW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04\x8DW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04\x9CW`\0\x80\xFD[\x86` \x82`\x07\x1B\x85\x01\x01\x11\x15a\x04\xB1W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x04\xD5W`\0\x80\xFD[P5\x91\x90PV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a\x04\xF0W`\0\x80\xFD[`\x80\x81\x12\x15a\x04\xFEW`\0\x80\xFD[P`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x050WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x90\x81R\x845\x82R` \x80\x86\x015\x90\x83\x01R\x84\x81\x015\x90\x82\x01R``\x80\x85\x015\x90\x82\x01R\x94`\x80\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x05\x99WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA1dsolcC\0\x08\x17\0\n";
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
