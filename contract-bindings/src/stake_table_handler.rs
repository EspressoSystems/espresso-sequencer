pub use stake_table_handler::*;
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
pub mod stake_table_handler {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_stakeTable"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract StakeTable"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_tokenCreator"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_token"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ExampleToken"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_lightClient"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract LightClientTest"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_users"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Address,),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address[]"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("advanceEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("advanceEpoch"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lightClient"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lightClient"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract LightClientTest"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("numberUsers"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("numberUsers"),
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
                    ::std::borrow::ToOwned::to_owned("register"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("register"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("seed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestExit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestExit"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("rand"),
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
                    ::std::borrow::ToOwned::to_owned("stakeTable"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeTable"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract StakeTable"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("token"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("token"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract ExampleToken"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenCreator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenCreator"),
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
                    ::std::borrow::ToOwned::to_owned("users"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("users"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("vks"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("vks"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("vksWithdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("vksWithdraw"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("BN254.BaseField"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawFunds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawFunds"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("rand"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STAKETABLEHANDLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x14\x118\x03\x80b\0\x14\x11\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01hV[`\x10\x80T`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x12\x80T\x86\x84\x16\x90\x83\x16\x17\x90U`\x11\x80T\x87\x84\x16\x90\x83\x16\x17\x90U`\x15\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x80Qb\0\0\x93\x90`\x16\x90` \x84\x01\x90b\0\0\xA6V[PP`\x16T`\x17UPb\0\x02\x94\x92PPPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15b\0\0\xFEW\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\0\xFEW\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90b\0\0\xC7V[Pb\0\x01\x0C\x92\x91Pb\0\x01\x10V[P\x90V[[\x80\x82\x11\x15b\0\x01\x0CW`\0\x81U`\x01\x01b\0\x01\x11V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01=W`\0\x80\xFD[PV[\x80Qb\0\x01M\x81b\0\x01'V[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01\x81W`\0\x80\xFD[\x85Qb\0\x01\x8E\x81b\0\x01'V[\x80\x95PP` \x80\x87\x01Qb\0\x01\xA3\x81b\0\x01'V[`@\x88\x01Q\x90\x95Pb\0\x01\xB6\x81b\0\x01'V[``\x88\x01Q\x90\x94Pb\0\x01\xC9\x81b\0\x01'V[`\x80\x88\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x01\xE7W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12b\0\x01\xFCW`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x02\x11Wb\0\x02\x11b\0\x01RV[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15b\0\x029Wb\0\x029b\0\x01RV[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x8C\x83\x11\x15b\0\x02XW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15b\0\x02\x81Wb\0\x02q\x85b\0\x01@V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92b\0\x02]V[\x80\x96PPPPPPP\x92\x95P\x92\x95\x90\x93PV[a\x11m\x80b\0\x02\xA4`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80c\xB5p\x0Eh\x11a\0qW\x80c\xB5p\x0Eh\x14a\x01?W\x80c\xBBG\x10\xC5\x14a\x01RW\x80c\xDB\x84%,\x14a\x01iW\x80c\xE3Y%R\x14a\x01|W\x80c\xFA\x18/\xA1\x14a\x01\xD1W\x80c\xFC\x0CTj\x14a\x01\xE4W`\0\x80\xFD[\x80c\x15]\xD5\xEE\x14a\0\xB9W\x80c6[\x98\xB2\x14a\0\xCEW\x80c<\xF8\x0El\x14a\0\xFEW\x80c\\\x05\x03G\x14a\x01\x06W\x80cr\x1Ce\x13\x14a\x01\x19W\x80c\x7F\xAE\xB4\xEF\x14a\x01,W[`\0\x80\xFD[a\0\xCCa\0\xC76`\x04a\r\xA4V[a\x01\xF7V[\0[a\0\xE1a\0\xDC6`\x04a\r\xA4V[a\x04BV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCCa\x04lV[`\x10Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xCCa\x01'6`\x04a\r\xA4V[a\x05UV[a\0\xCCa\x01:6`\x04a\r\xD6V[a\x06\xBAV[`\x15Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01[`\x17T\x81V[`@Q\x90\x81R` \x01a\0\xF5V[`\x11Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xB1a\x01\x8A6`\x04a\r\xA4V[`\x13` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x92\x90\x91\x84V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\0\xF5V[a\x01\xB1a\x01\xDF6`\x04a\r\xA4V[a\x06\xE2V[`\x12Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0a\x02\x13\x82`\0`\x01`\x17Ta\x02\x0E\x91\x90a\x0E+V[a\x07\x1CV[`\0\x81\x81R`\x13` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x81\x85\x01R`\x02\x82\x01T\x81\x84\x01R`\x03\x90\x91\x01T``\x82\x01R`\x15T\x82Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x92Q\x95\x96P\x90\x94`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92cvg\x18\x08\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x02\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xB9\x91\x90a\x0EDV[\x90P`d`\0a\x02\xC9\x82\x84a\x0EaV[`\x15T`@Qc9I\xD1\xE9`\xE0\x1B\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c9I\xD1\xE9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x19W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03-W=`\0\x80>=`\0\xFD[PP`\x16\x80Tsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x93Pc\xCAf\x9F\xA7\x92P\x88\x90\x81\x10a\x03aWa\x03aa\x0E\x89V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xAEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xC2W=`\0\x80>=`\0\xFD[PP`\x10T`@Qc\x01\x84\x95\xE3`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\x0C$\xAF\x18\x91Pa\x03\xF6\x90\x87\x90`\x04\x01a\x0E\x9FV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x049\x91\x90a\x0EDV[PPPPPPPV[`\x16\x81\x81T\x81\x10a\x04RW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\x15T`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cvg\x18\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xDA\x91\x90a\x0EDV[\x90P`\0a\x04\xE9\x82`\x01a\x0EaV[`\x15T`@Qc9I\xD1\xE9`\xE0\x1B\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c9I\xD1\xE9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x059W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05MW=`\0\x80>=`\0\xFD[PPPPPPV[`\0a\x05l\x82`\0`\x01`\x17Ta\x02\x0E\x91\x90a\x0E+V[\x90P\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xCAf\x9F\xA7`\x16\x83\x81T\x81\x10a\x05\xB3Wa\x05\xB3a\x0E\x89V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x14W=`\0\x80>=`\0\xFD[PP`\x10T`\0\x84\x81R`\x13` R`@\x90\x81\x90 \x90QcJ\xA7\xC2\x7F`\xE0\x1B\x81R\x81T`\x04\x82\x01R`\x01\x82\x01T`$\x82\x01R`\x02\x82\x01T`D\x82\x01R`\x03\x90\x91\x01T`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92PcJ\xA7\xC2\x7F\x91P`\x84\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB5\x91\x90a\x0E\xCAV[PPPV[`\x11Ta\x06\xDE\x90`\x01`\x01`\xA0\x1B\x03\x16`\xFF\x84\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x07`V[PPV[`\x14\x81\x81T\x81\x10a\x06\xF2W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x93P\x91\x90\x84V[`\0a\x07)\x84\x84\x84a\x0BBV[\x90Pa\x07Y`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x14\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82a\r\nV[\x93\x92PPPV[a\x07u\x82`\0`\x01`\x17Ta\x02\x0E\x91\x90a\x0E+V[\x91P`\0\x82\x90P`\0`\x16\x84\x81T\x81\x10a\x07\x91Wa\x07\x91a\x0E\x89V[`\0\x91\x82R` \x82 \x01T`\x0FT`@Qc5\xD6\x9C\x89`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R`\xFF\x87\x16`$\x83\x01R\x92\x83\x16\x94P\x83\x92\x83\x92\x16\x90ck\xAD9\x12\x90`D\x01a\x01\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1D\x91\x90a\x0F\x84V[\x92P\x92P\x92P`\0a\x082\x87`\0`da\x07\x1CV[`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R\x90\x91Pa\x03\xE8\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x8EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xA2W=`\0\x80>=`\0\xFD[PP`\x12T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16`\x04\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`$\x83\x01R\x90\x91\x16\x92Pc\xA9\x05\x9C\xBB\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t&\x91\x90a\x0E\xCAV[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t|W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\x90W=`\0\x80>=`\0\xFD[PP`\x12T`\x10T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`$\x82\x01R\x91\x16\x92Pc\t^\xA7\xB3\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x16\x91\x90a\x0E\xCAV[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\nlW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\x80W=`\0\x80>=`\0\xFD[PP`\x10T`@Qc\xC7,\xC7\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\xC7,\xC7\x17\x91Pa\n\xBF\x90\x88\x90\x88\x90\x87\x90`\0\x90\x8A\x90\x89\x90`\x04\x01a\x10\x1BV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x02\x91\x90a\x0E\xCAV[PPP`\0\x96\x87RPP`\x13` \x90\x81R`@\x95\x86\x90 \x82Q\x81U\x90\x82\x01Q`\x01\x82\x01U\x94\x81\x01Q`\x02\x86\x01U``\x01Q`\x03\x90\x94\x01\x93\x90\x93UPPPPV[`\0\x81\x83\x11\x15a\x0B\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x0B\xCEWP\x81\x84\x11\x15[\x15a\x0B\xDAWP\x82a\x07YV[`\0a\x0B\xE6\x84\x84a\x0E+V[a\x0B\xF1\x90`\x01a\x10\xB0V[\x90P`\x03\x85\x11\x15\x80\x15a\x0C\x03WP\x84\x81\x11[\x15a\x0C\x1AWa\x0C\x12\x85\x85a\x10\xB0V[\x91PPa\x07YV[a\x0C'`\x03`\0\x19a\x0E+V[\x85\x10\x15\x80\x15a\x0C@WPa\x0C=\x85`\0\x19a\x0E+V[\x81\x11[\x15a\x0C[Wa\x0CQ\x85`\0\x19a\x0E+V[a\x0C\x12\x90\x84a\x0E+V[\x82\x85\x11\x15a\x0C\xB1W`\0a\x0Co\x84\x87a\x0E+V[\x90P`\0a\x0C}\x83\x83a\x10\xC3V[\x90P\x80`\0\x03a\x0C\x92W\x84\x93PPPPa\x07YV[`\x01a\x0C\x9E\x82\x88a\x10\xB0V[a\x0C\xA8\x91\x90a\x0E+V[\x93PPPa\r\x02V[\x83\x85\x10\x15a\r\x02W`\0a\x0C\xC5\x86\x86a\x0E+V[\x90P`\0a\x0C\xD3\x83\x83a\x10\xC3V[\x90P\x80`\0\x03a\x0C\xE8W\x85\x93PPPPa\x07YV[a\x0C\xF2\x81\x86a\x0E+V[a\x0C\xFD\x90`\x01a\x10\xB0V[\x93PPP[P\x93\x92PPPV[`\0jconsole.log`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Q`$\x01a\r4\x92\x91\x90a\x11\tV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90RQa\ri\x91\x90a\x11DV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x05MW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05MV[`\0` \x82\x84\x03\x12\x15a\r\xB6W`\0\x80\xFD[P5\x91\x90PV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\r\xD3W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xE9W`\0\x80\xFD[\x825`\xFF\x81\x16\x81\x14a\r\xFAW`\0\x80\xFD[\x91P` \x83\x015a\x0E\n\x81a\r\xBDV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E>Wa\x0E>a\x0E\x15V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0EVW`\0\x80\xFD[\x81Qa\x07Y\x81a\r\xBDV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x0E\x82Wa\x0E\x82a\x0E\x15V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x81\x01a\x0E>V[`\0` \x82\x84\x03\x12\x15a\x0E\xDCW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07YW`\0\x80\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\x1DWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\x1DWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a\x0FfW`\0\x80\xFD[a\x0Fna\x0E\xECV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R\x92\x91PPV[`\0\x80`\0\x83\x85\x03a\x01\0\x81\x12\x15a\x0F\x9BW`\0\x80\xFD[`\x80\x81\x12\x15a\x0F\xA9W`\0\x80\xFD[a\x0F\xB1a\x0F#V[\x85Q\x81R` \x86\x01Q` \x82\x01R`@\x86\x01Q`@\x82\x01R``\x86\x01Q``\x82\x01R\x80\x94PP`@`\x7F\x19\x82\x01\x12\x15a\x0F\xE9W`\0\x80\xFD[Pa\x0F\xF2a\x0E\xECV[`\x80\x85\x01Q\x81R`\xA0\x85\x01Q` \x82\x01R\x91Pa\x10\x12\x85`\xC0\x86\x01a\x0FTV[\x90P\x92P\x92P\x92V[\x86Q\x81R` \x80\x88\x01Q\x90\x82\x01R`@\x80\x88\x01Q\x90\x82\x01R``\x80\x88\x01Q\x90\x82\x01Ra\x01`\x81\x01\x86Q`\x80\x83\x01R` \x87\x01Q`\xA0\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16`\xC0\x84\x01R`\x02\x86\x10a\x10\x82WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x85`\xE0\x84\x01R\x84Qa\x01\0\x84\x01R` \x85\x01Qa\x01 \x84\x01R\x80\x84\x16a\x01@\x84\x01RP\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x0E>Wa\x0E>a\x0E\x15V[`\0\x82a\x10\xE0WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0[\x83\x81\x10\x15a\x11\0W\x81\x81\x01Q\x83\x82\x01R` \x01a\x10\xE8V[PP`\0\x91\x01RV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x11(\x81``\x85\x01` \x88\x01a\x10\xE5V[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01`\x1F\x19\x16\x01``\x01\x91\x90PV[`\0\x82Qa\x11V\x81\x84` \x87\x01a\x10\xE5V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static STAKETABLEHANDLER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80c\xB5p\x0Eh\x11a\0qW\x80c\xB5p\x0Eh\x14a\x01?W\x80c\xBBG\x10\xC5\x14a\x01RW\x80c\xDB\x84%,\x14a\x01iW\x80c\xE3Y%R\x14a\x01|W\x80c\xFA\x18/\xA1\x14a\x01\xD1W\x80c\xFC\x0CTj\x14a\x01\xE4W`\0\x80\xFD[\x80c\x15]\xD5\xEE\x14a\0\xB9W\x80c6[\x98\xB2\x14a\0\xCEW\x80c<\xF8\x0El\x14a\0\xFEW\x80c\\\x05\x03G\x14a\x01\x06W\x80cr\x1Ce\x13\x14a\x01\x19W\x80c\x7F\xAE\xB4\xEF\x14a\x01,W[`\0\x80\xFD[a\0\xCCa\0\xC76`\x04a\r\xA4V[a\x01\xF7V[\0[a\0\xE1a\0\xDC6`\x04a\r\xA4V[a\x04BV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCCa\x04lV[`\x10Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xCCa\x01'6`\x04a\r\xA4V[a\x05UV[a\0\xCCa\x01:6`\x04a\r\xD6V[a\x06\xBAV[`\x15Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01[`\x17T\x81V[`@Q\x90\x81R` \x01a\0\xF5V[`\x11Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xB1a\x01\x8A6`\x04a\r\xA4V[`\x13` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x92\x90\x91\x84V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\0\xF5V[a\x01\xB1a\x01\xDF6`\x04a\r\xA4V[a\x06\xE2V[`\x12Ta\0\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0a\x02\x13\x82`\0`\x01`\x17Ta\x02\x0E\x91\x90a\x0E+V[a\x07\x1CV[`\0\x81\x81R`\x13` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x81\x85\x01R`\x02\x82\x01T\x81\x84\x01R`\x03\x90\x91\x01T``\x82\x01R`\x15T\x82Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x92Q\x95\x96P\x90\x94`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92cvg\x18\x08\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x02\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xB9\x91\x90a\x0EDV[\x90P`d`\0a\x02\xC9\x82\x84a\x0EaV[`\x15T`@Qc9I\xD1\xE9`\xE0\x1B\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c9I\xD1\xE9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x19W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03-W=`\0\x80>=`\0\xFD[PP`\x16\x80Tsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x93Pc\xCAf\x9F\xA7\x92P\x88\x90\x81\x10a\x03aWa\x03aa\x0E\x89V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xAEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xC2W=`\0\x80>=`\0\xFD[PP`\x10T`@Qc\x01\x84\x95\xE3`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\x0C$\xAF\x18\x91Pa\x03\xF6\x90\x87\x90`\x04\x01a\x0E\x9FV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x049\x91\x90a\x0EDV[PPPPPPPV[`\x16\x81\x81T\x81\x10a\x04RW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\x15T`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cvg\x18\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xDA\x91\x90a\x0EDV[\x90P`\0a\x04\xE9\x82`\x01a\x0EaV[`\x15T`@Qc9I\xD1\xE9`\xE0\x1B\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c9I\xD1\xE9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x059W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05MW=`\0\x80>=`\0\xFD[PPPPPPV[`\0a\x05l\x82`\0`\x01`\x17Ta\x02\x0E\x91\x90a\x0E+V[\x90P\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xCAf\x9F\xA7`\x16\x83\x81T\x81\x10a\x05\xB3Wa\x05\xB3a\x0E\x89V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x14W=`\0\x80>=`\0\xFD[PP`\x10T`\0\x84\x81R`\x13` R`@\x90\x81\x90 \x90QcJ\xA7\xC2\x7F`\xE0\x1B\x81R\x81T`\x04\x82\x01R`\x01\x82\x01T`$\x82\x01R`\x02\x82\x01T`D\x82\x01R`\x03\x90\x91\x01T`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92PcJ\xA7\xC2\x7F\x91P`\x84\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB5\x91\x90a\x0E\xCAV[PPPV[`\x11Ta\x06\xDE\x90`\x01`\x01`\xA0\x1B\x03\x16`\xFF\x84\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x07`V[PPV[`\x14\x81\x81T\x81\x10a\x06\xF2W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x93P\x91\x90\x84V[`\0a\x07)\x84\x84\x84a\x0BBV[\x90Pa\x07Y`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x14\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82a\r\nV[\x93\x92PPPV[a\x07u\x82`\0`\x01`\x17Ta\x02\x0E\x91\x90a\x0E+V[\x91P`\0\x82\x90P`\0`\x16\x84\x81T\x81\x10a\x07\x91Wa\x07\x91a\x0E\x89V[`\0\x91\x82R` \x82 \x01T`\x0FT`@Qc5\xD6\x9C\x89`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R`\xFF\x87\x16`$\x83\x01R\x92\x83\x16\x94P\x83\x92\x83\x92\x16\x90ck\xAD9\x12\x90`D\x01a\x01\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1D\x91\x90a\x0F\x84V[\x92P\x92P\x92P`\0a\x082\x87`\0`da\x07\x1CV[`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R\x90\x91Pa\x03\xE8\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x8EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xA2W=`\0\x80>=`\0\xFD[PP`\x12T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16`\x04\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`$\x83\x01R\x90\x91\x16\x92Pc\xA9\x05\x9C\xBB\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t&\x91\x90a\x0E\xCAV[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t|W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\x90W=`\0\x80>=`\0\xFD[PP`\x12T`\x10T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`$\x82\x01R\x91\x16\x92Pc\t^\xA7\xB3\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x16\x91\x90a\x0E\xCAV[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\nlW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\x80W=`\0\x80>=`\0\xFD[PP`\x10T`@Qc\xC7,\xC7\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\xC7,\xC7\x17\x91Pa\n\xBF\x90\x88\x90\x88\x90\x87\x90`\0\x90\x8A\x90\x89\x90`\x04\x01a\x10\x1BV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x02\x91\x90a\x0E\xCAV[PPP`\0\x96\x87RPP`\x13` \x90\x81R`@\x95\x86\x90 \x82Q\x81U\x90\x82\x01Q`\x01\x82\x01U\x94\x81\x01Q`\x02\x86\x01U``\x01Q`\x03\x90\x94\x01\x93\x90\x93UPPPPV[`\0\x81\x83\x11\x15a\x0B\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x0B\xCEWP\x81\x84\x11\x15[\x15a\x0B\xDAWP\x82a\x07YV[`\0a\x0B\xE6\x84\x84a\x0E+V[a\x0B\xF1\x90`\x01a\x10\xB0V[\x90P`\x03\x85\x11\x15\x80\x15a\x0C\x03WP\x84\x81\x11[\x15a\x0C\x1AWa\x0C\x12\x85\x85a\x10\xB0V[\x91PPa\x07YV[a\x0C'`\x03`\0\x19a\x0E+V[\x85\x10\x15\x80\x15a\x0C@WPa\x0C=\x85`\0\x19a\x0E+V[\x81\x11[\x15a\x0C[Wa\x0CQ\x85`\0\x19a\x0E+V[a\x0C\x12\x90\x84a\x0E+V[\x82\x85\x11\x15a\x0C\xB1W`\0a\x0Co\x84\x87a\x0E+V[\x90P`\0a\x0C}\x83\x83a\x10\xC3V[\x90P\x80`\0\x03a\x0C\x92W\x84\x93PPPPa\x07YV[`\x01a\x0C\x9E\x82\x88a\x10\xB0V[a\x0C\xA8\x91\x90a\x0E+V[\x93PPPa\r\x02V[\x83\x85\x10\x15a\r\x02W`\0a\x0C\xC5\x86\x86a\x0E+V[\x90P`\0a\x0C\xD3\x83\x83a\x10\xC3V[\x90P\x80`\0\x03a\x0C\xE8W\x85\x93PPPPa\x07YV[a\x0C\xF2\x81\x86a\x0E+V[a\x0C\xFD\x90`\x01a\x10\xB0V[\x93PPP[P\x93\x92PPPV[`\0jconsole.log`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Q`$\x01a\r4\x92\x91\x90a\x11\tV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90RQa\ri\x91\x90a\x11DV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x05MW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05MV[`\0` \x82\x84\x03\x12\x15a\r\xB6W`\0\x80\xFD[P5\x91\x90PV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\r\xD3W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xE9W`\0\x80\xFD[\x825`\xFF\x81\x16\x81\x14a\r\xFAW`\0\x80\xFD[\x91P` \x83\x015a\x0E\n\x81a\r\xBDV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E>Wa\x0E>a\x0E\x15V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0EVW`\0\x80\xFD[\x81Qa\x07Y\x81a\r\xBDV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x0E\x82Wa\x0E\x82a\x0E\x15V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x81\x01a\x0E>V[`\0` \x82\x84\x03\x12\x15a\x0E\xDCW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07YW`\0\x80\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\x1DWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\x1DWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a\x0FfW`\0\x80\xFD[a\x0Fna\x0E\xECV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R\x92\x91PPV[`\0\x80`\0\x83\x85\x03a\x01\0\x81\x12\x15a\x0F\x9BW`\0\x80\xFD[`\x80\x81\x12\x15a\x0F\xA9W`\0\x80\xFD[a\x0F\xB1a\x0F#V[\x85Q\x81R` \x86\x01Q` \x82\x01R`@\x86\x01Q`@\x82\x01R``\x86\x01Q``\x82\x01R\x80\x94PP`@`\x7F\x19\x82\x01\x12\x15a\x0F\xE9W`\0\x80\xFD[Pa\x0F\xF2a\x0E\xECV[`\x80\x85\x01Q\x81R`\xA0\x85\x01Q` \x82\x01R\x91Pa\x10\x12\x85`\xC0\x86\x01a\x0FTV[\x90P\x92P\x92P\x92V[\x86Q\x81R` \x80\x88\x01Q\x90\x82\x01R`@\x80\x88\x01Q\x90\x82\x01R``\x80\x88\x01Q\x90\x82\x01Ra\x01`\x81\x01\x86Q`\x80\x83\x01R` \x87\x01Q`\xA0\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16`\xC0\x84\x01R`\x02\x86\x10a\x10\x82WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x85`\xE0\x84\x01R\x84Qa\x01\0\x84\x01R` \x85\x01Qa\x01 \x84\x01R\x80\x84\x16a\x01@\x84\x01RP\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x0E>Wa\x0E>a\x0E\x15V[`\0\x82a\x10\xE0WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0[\x83\x81\x10\x15a\x11\0W\x81\x81\x01Q\x83\x82\x01R` \x01a\x10\xE8V[PP`\0\x91\x01RV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x11(\x81``\x85\x01` \x88\x01a\x10\xE5V[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01`\x1F\x19\x16\x01``\x01\x91\x90PV[`\0\x82Qa\x11V\x81\x84` \x87\x01a\x10\xE5V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static STAKETABLEHANDLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct StakeTableHandler<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StakeTableHandler<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StakeTableHandler<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StakeTableHandler<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StakeTableHandler<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StakeTableHandler))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StakeTableHandler<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                STAKETABLEHANDLER_ABI.clone(),
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
                STAKETABLEHANDLER_ABI.clone(),
                STAKETABLEHANDLER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `advanceEpoch` (0x3cf80e6c) function
        pub fn advance_epoch(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 248, 14, 108], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lightClient` (0xb5700e68) function
        pub fn light_client(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([181, 112, 14, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numberUsers` (0xbb4710c5) function
        pub fn number_users(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([187, 71, 16, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `register` (0x7faeb4ef) function
        pub fn register(
            &self,
            seed: u8,
            amount: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 174, 180, 239], (seed, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestExit` (0x721c6513) function
        pub fn request_exit(
            &self,
            rand: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 28, 101, 19], rand)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeTable` (0x5c050347) function
        pub fn stake_table(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([92, 5, 3, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token` (0xfc0c546a) function
        pub fn token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenCreator` (0xdb84252c) function
        pub fn token_creator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([219, 132, 37, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `users` (0x365b98b2) function
        pub fn users(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([54, 91, 152, 178], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vks` (0xe3592552) function
        pub fn vks(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([227, 89, 37, 82], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vksWithdraw` (0xfa182fa1) function
        pub fn vks_withdraw(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([250, 24, 47, 161], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawFunds` (0x155dd5ee) function
        pub fn withdraw_funds(
            &self,
            rand: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 93, 213, 238], rand)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for StakeTableHandler<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `advanceEpoch` function with signature `advanceEpoch()` and selector `0x3cf80e6c`
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
    #[ethcall(name = "advanceEpoch", abi = "advanceEpoch()")]
    pub struct AdvanceEpochCall;
    ///Container type for all input parameters for the `lightClient` function with signature `lightClient()` and selector `0xb5700e68`
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
    #[ethcall(name = "lightClient", abi = "lightClient()")]
    pub struct LightClientCall;
    ///Container type for all input parameters for the `numberUsers` function with signature `numberUsers()` and selector `0xbb4710c5`
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
    #[ethcall(name = "numberUsers", abi = "numberUsers()")]
    pub struct NumberUsersCall;
    ///Container type for all input parameters for the `register` function with signature `register(uint8,uint64)` and selector `0x7faeb4ef`
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
    #[ethcall(name = "register", abi = "register(uint8,uint64)")]
    pub struct RegisterCall {
        pub seed: u8,
        pub amount: u64,
    }
    ///Container type for all input parameters for the `requestExit` function with signature `requestExit(uint256)` and selector `0x721c6513`
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
    #[ethcall(name = "requestExit", abi = "requestExit(uint256)")]
    pub struct RequestExitCall {
        pub rand: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `stakeTable` function with signature `stakeTable()` and selector `0x5c050347`
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
    #[ethcall(name = "stakeTable", abi = "stakeTable()")]
    pub struct StakeTableCall;
    ///Container type for all input parameters for the `token` function with signature `token()` and selector `0xfc0c546a`
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
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    ///Container type for all input parameters for the `tokenCreator` function with signature `tokenCreator()` and selector `0xdb84252c`
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
    #[ethcall(name = "tokenCreator", abi = "tokenCreator()")]
    pub struct TokenCreatorCall;
    ///Container type for all input parameters for the `users` function with signature `users(uint256)` and selector `0x365b98b2`
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
    #[ethcall(name = "users", abi = "users(uint256)")]
    pub struct UsersCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `vks` function with signature `vks(uint256)` and selector `0xe3592552`
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
    #[ethcall(name = "vks", abi = "vks(uint256)")]
    pub struct VksCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `vksWithdraw` function with signature `vksWithdraw(uint256)` and selector `0xfa182fa1`
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
    #[ethcall(name = "vksWithdraw", abi = "vksWithdraw(uint256)")]
    pub struct VksWithdrawCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `withdrawFunds` function with signature `withdrawFunds(uint256)` and selector `0x155dd5ee`
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
    #[ethcall(name = "withdrawFunds", abi = "withdrawFunds(uint256)")]
    pub struct WithdrawFundsCall {
        pub rand: ::ethers::core::types::U256,
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
    pub enum StakeTableHandlerCalls {
        AdvanceEpoch(AdvanceEpochCall),
        LightClient(LightClientCall),
        NumberUsers(NumberUsersCall),
        Register(RegisterCall),
        RequestExit(RequestExitCall),
        StakeTable(StakeTableCall),
        Token(TokenCall),
        TokenCreator(TokenCreatorCall),
        Users(UsersCall),
        Vks(VksCall),
        VksWithdraw(VksWithdrawCall),
        WithdrawFunds(WithdrawFundsCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakeTableHandlerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdvanceEpochCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AdvanceEpoch(decoded));
            }
            if let Ok(decoded) = <LightClientCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LightClient(decoded));
            }
            if let Ok(decoded) = <NumberUsersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NumberUsers(decoded));
            }
            if let Ok(decoded) = <RegisterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Register(decoded));
            }
            if let Ok(decoded) = <RequestExitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RequestExit(decoded));
            }
            if let Ok(decoded) = <StakeTableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StakeTable(decoded));
            }
            if let Ok(decoded) = <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token(decoded));
            }
            if let Ok(decoded) = <TokenCreatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TokenCreator(decoded));
            }
            if let Ok(decoded) = <UsersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Users(decoded));
            }
            if let Ok(decoded) = <VksCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Vks(decoded));
            }
            if let Ok(decoded) = <VksWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VksWithdraw(decoded));
            }
            if let Ok(decoded) = <WithdrawFundsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawFunds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakeTableHandlerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdvanceEpoch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LightClient(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NumberUsers(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Register(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestExit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeTable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenCreator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Users(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Vks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VksWithdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawFunds(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for StakeTableHandlerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdvanceEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::LightClient(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumberUsers(element) => ::core::fmt::Display::fmt(element, f),
                Self::Register(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestExit(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTable(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenCreator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Users(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vks(element) => ::core::fmt::Display::fmt(element, f),
                Self::VksWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFunds(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdvanceEpochCall> for StakeTableHandlerCalls {
        fn from(value: AdvanceEpochCall) -> Self {
            Self::AdvanceEpoch(value)
        }
    }
    impl ::core::convert::From<LightClientCall> for StakeTableHandlerCalls {
        fn from(value: LightClientCall) -> Self {
            Self::LightClient(value)
        }
    }
    impl ::core::convert::From<NumberUsersCall> for StakeTableHandlerCalls {
        fn from(value: NumberUsersCall) -> Self {
            Self::NumberUsers(value)
        }
    }
    impl ::core::convert::From<RegisterCall> for StakeTableHandlerCalls {
        fn from(value: RegisterCall) -> Self {
            Self::Register(value)
        }
    }
    impl ::core::convert::From<RequestExitCall> for StakeTableHandlerCalls {
        fn from(value: RequestExitCall) -> Self {
            Self::RequestExit(value)
        }
    }
    impl ::core::convert::From<StakeTableCall> for StakeTableHandlerCalls {
        fn from(value: StakeTableCall) -> Self {
            Self::StakeTable(value)
        }
    }
    impl ::core::convert::From<TokenCall> for StakeTableHandlerCalls {
        fn from(value: TokenCall) -> Self {
            Self::Token(value)
        }
    }
    impl ::core::convert::From<TokenCreatorCall> for StakeTableHandlerCalls {
        fn from(value: TokenCreatorCall) -> Self {
            Self::TokenCreator(value)
        }
    }
    impl ::core::convert::From<UsersCall> for StakeTableHandlerCalls {
        fn from(value: UsersCall) -> Self {
            Self::Users(value)
        }
    }
    impl ::core::convert::From<VksCall> for StakeTableHandlerCalls {
        fn from(value: VksCall) -> Self {
            Self::Vks(value)
        }
    }
    impl ::core::convert::From<VksWithdrawCall> for StakeTableHandlerCalls {
        fn from(value: VksWithdrawCall) -> Self {
            Self::VksWithdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawFundsCall> for StakeTableHandlerCalls {
        fn from(value: WithdrawFundsCall) -> Self {
            Self::WithdrawFunds(value)
        }
    }
    ///Container type for all return fields from the `lightClient` function with signature `lightClient()` and selector `0xb5700e68`
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
    pub struct LightClientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `numberUsers` function with signature `numberUsers()` and selector `0xbb4710c5`
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
    pub struct NumberUsersReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stakeTable` function with signature `stakeTable()` and selector `0x5c050347`
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
    pub struct StakeTableReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `token` function with signature `token()` and selector `0xfc0c546a`
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
    pub struct TokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokenCreator` function with signature `tokenCreator()` and selector `0xdb84252c`
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
    pub struct TokenCreatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `users` function with signature `users(uint256)` and selector `0x365b98b2`
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
    pub struct UsersReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `vks` function with signature `vks(uint256)` and selector `0xe3592552`
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
    pub struct VksReturn {
        pub x_0: ::ethers::core::types::U256,
        pub x_1: ::ethers::core::types::U256,
        pub y_0: ::ethers::core::types::U256,
        pub y_1: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `vksWithdraw` function with signature `vksWithdraw(uint256)` and selector `0xfa182fa1`
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
    pub struct VksWithdrawReturn {
        pub x_0: ::ethers::core::types::U256,
        pub x_1: ::ethers::core::types::U256,
        pub y_0: ::ethers::core::types::U256,
        pub y_1: ::ethers::core::types::U256,
    }
}
