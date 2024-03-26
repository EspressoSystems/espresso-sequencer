pub use mock_erc20::*;
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
pub mod mock_erc20 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allowance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("approve"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("decimals"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("name_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("symbol_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("decimals_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("name"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nonces"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("permit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deadline"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("v"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("r"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("symbol"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transfer"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Approval"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Transfer"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
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
    pub static MOCKERC20_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0EO\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c6D\xE5\x15\x11a\0\x8CW\x80c\x95\xD8\x9BA\x11a\0fW\x80c\x95\xD8\x9BA\x14a\x01\xB5W\x80c\xA9\x05\x9C\xBB\x14a\x01\xBDW\x80c\xD5\x05\xAC\xCF\x14a\x01\xD0W\x80c\xDDb\xED>\x14a\x01\xE3W`\0\x80\xFD[\x80c6D\xE5\x15\x14a\x01dW\x80cp\xA0\x821\x14a\x01lW\x80c~\xCE\xBE\0\x14a\x01\x95W`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\0\xD4W\x80c\t^\xA7\xB3\x14a\0\xF2W\x80c\x16$\xF6\xC6\x14a\x01\x15W\x80c\x18\x16\r\xDD\x14a\x01*W\x80c#\xB8r\xDD\x14a\x01<W\x80c1<\xE5g\x14a\x01OW[`\0\x80\xFD[a\0\xDCa\x02\x1CV[`@Qa\0\xE9\x91\x90a\t{V[`@Q\x80\x91\x03\x90\xF3[a\x01\x05a\x01\x006`\x04a\t\xE6V[a\x02\xAEV[`@Q\x90\x15\x15\x81R` \x01a\0\xE9V[a\x01(a\x01#6`\x04a\n\xC4V[a\x03\x1BV[\0[`\x03T[`@Q\x90\x81R` \x01a\0\xE9V[a\x01\x05a\x01J6`\x04a\x0B8V[a\x03\xBAV[`\x02T`@Q`\xFF\x90\x91\x16\x81R` \x01a\0\xE9V[a\x01.a\x04\xCFV[a\x01.a\x01z6`\x04a\x0BtV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x04` R`@\x90 T\x90V[a\x01.a\x01\xA36`\x04a\x0BtV[`\x08` R`\0\x90\x81R`@\x90 T\x81V[a\0\xDCa\x04\xF5V[a\x01\x05a\x01\xCB6`\x04a\t\xE6V[a\x05\x04V[a\x01(a\x01\xDE6`\x04a\x0B\x8FV[a\x05\x9BV[a\x01.a\x01\xF16`\x04a\x0B\xF9V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[```\0\x80Ta\x02+\x90a\x0C,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02W\x90a\x0C,V[\x80\x15a\x02\xA4W\x80`\x1F\x10a\x02yWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xA4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x87W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[3`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x03\t\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\tT`\xFF\x16\x15a\x03iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10S\x14\x91PQ\x16W\xD2S\x92U\x12PS\x12V\x91Q`j\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x03u\x84\x82a\x0C\xB7V[P`\x01a\x03\x82\x83\x82a\x0C\xB7V[P`\x02\x80T`\xFF\x19\x16`\xFF\x83\x16\x17\x90Ua\x03\x9Aa\x07\xF9V[`\x06Ua\x03\xA5a\x08\x12V[`\x07UPP`\t\x80T`\xFF\x19\x16`\x01\x17\x90UPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a\x04\x16Wa\x03\xF1\x81\x84a\x08\xB5V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x04` R`@\x90 Ta\x049\x90\x84a\x08\xB5V[`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\0\x90\x81R`\x04` R`@\x80\x82 \x93\x90\x93U\x90\x86\x16\x81R Ta\x04h\x90\x84a\t\x18V[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x81\x81R`\x04` R`@\x90\x81\x90 \x93\x90\x93U\x91Q\x90\x87\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x04\xBC\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`\0`\x06Ta\x04\xDCa\x07\xF9V[\x14a\x04\xEEWa\x04\xE9a\x08\x12V[\x90P\x90V[P`\x07T\x90V[```\x01\x80Ta\x02+\x90a\x0C,V[3`\0\x90\x81R`\x04` R`@\x81 Ta\x05\x1E\x90\x83a\x08\xB5V[3`\0\x90\x81R`\x04` R`@\x80\x82 \x92\x90\x92U`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R Ta\x05J\x90\x83a\t\x18V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81R`\x04` R`@\x90\x81\x90 \x92\x90\x92U\x90Q3\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x03\t\x90\x86\x81R` \x01\x90V[B\x84\x10\x15a\x05\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03`V[`\0`\x01a\x05\xF7a\x04\xCFV[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`\x08` R`@\x81 \x80T\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x92\x8D\x92\x8D\x92\x8D\x92\x90\x91\x90a\x06E\x83a\r\x8DV[\x90\x91UP`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x88\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x06\xBE\x92\x91\x90a\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x07\x1CW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x07RWP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x07\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\x03`V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x8B\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x8A\x90UQ\x89\x81R\x91\x92\x8B\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPPV[`\0a\tw\x80a\x08\x0Bc\xFF\xFF\xFF\xFF\x82\x16V[\x92PPP\x90V[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa\x08D\x91\x90a\r\xA6V[`@Q\x80\x91\x03\x90 \x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6a\x08ua\x07\xF9V[`@\x80Q` \x81\x01\x95\x90\x95R\x84\x01\x92\x90\x92R``\x83\x01R`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x81\x83\x10\x15a\t\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC20: subtraction underflow\0\0\0\0`D\x82\x01R`d\x01a\x03`V[a\t\x11\x82\x84a\x0E\x1CV[\x93\x92PPPV[`\0\x80a\t%\x83\x85a\x0E/V[\x90P\x83\x81\x10\x15a\t\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FERC20: addition overflow\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03`V[F\x90V[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\t\xA9W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\t\x8DV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xE1W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\t\xF9W`\0\x80\xFD[a\n\x02\x83a\t\xCAV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\n7W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\nRWa\nRa\n\x10V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\nzWa\nza\n\x10V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\n\x93W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a\t\xE1W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\xD9W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xF1W`\0\x80\xFD[a\n\xFD\x87\x83\x88\x01a\n&V[\x94P` \x86\x015\x91P\x80\x82\x11\x15a\x0B\x13W`\0\x80\xFD[Pa\x0B \x86\x82\x87\x01a\n&V[\x92PPa\x0B/`@\x85\x01a\n\xB3V[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0BMW`\0\x80\xFD[a\x0BV\x84a\t\xCAV[\x92Pa\x0Bd` \x85\x01a\t\xCAV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0B\x86W`\0\x80\xFD[a\t\x11\x82a\t\xCAV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x0B\xAAW`\0\x80\xFD[a\x0B\xB3\x88a\t\xCAV[\x96Pa\x0B\xC1` \x89\x01a\t\xCAV[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa\x0B\xDD`\x80\x89\x01a\n\xB3V[\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x0CW`\0\x80\xFD[a\x0C\x15\x83a\t\xCAV[\x91Pa\x0C#` \x84\x01a\t\xCAV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C@W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C`WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x0C\xB2W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0C\x8FWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0C\xAEW\x82\x81U`\x01\x01a\x0C\x9BV[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xD1Wa\x0C\xD1a\n\x10V[a\x0C\xE5\x81a\x0C\xDF\x84Ta\x0C,V[\x84a\x0CfV[` \x80`\x1F\x83\x11`\x01\x81\x14a\r\x1AW`\0\x84\x15a\r\x02WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0C\xAEV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\rIW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\r*V[P\x85\x82\x10\x15a\rgW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\r\x9FWa\r\x9Fa\rwV[P`\x01\x01\x90V[`\0\x80\x83Ta\r\xB4\x81a\x0C,V[`\x01\x82\x81\x16\x80\x15a\r\xCCW`\x01\x81\x14a\r\xE1Wa\x0E\x10V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x0E\x10V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x0E\x07W\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\r\xEEV[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x03\x15Wa\x03\x15a\rwV[\x80\x82\x01\x80\x82\x11\x15a\x03\x15Wa\x03\x15a\rwV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static MOCKERC20_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c6D\xE5\x15\x11a\0\x8CW\x80c\x95\xD8\x9BA\x11a\0fW\x80c\x95\xD8\x9BA\x14a\x01\xB5W\x80c\xA9\x05\x9C\xBB\x14a\x01\xBDW\x80c\xD5\x05\xAC\xCF\x14a\x01\xD0W\x80c\xDDb\xED>\x14a\x01\xE3W`\0\x80\xFD[\x80c6D\xE5\x15\x14a\x01dW\x80cp\xA0\x821\x14a\x01lW\x80c~\xCE\xBE\0\x14a\x01\x95W`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\0\xD4W\x80c\t^\xA7\xB3\x14a\0\xF2W\x80c\x16$\xF6\xC6\x14a\x01\x15W\x80c\x18\x16\r\xDD\x14a\x01*W\x80c#\xB8r\xDD\x14a\x01<W\x80c1<\xE5g\x14a\x01OW[`\0\x80\xFD[a\0\xDCa\x02\x1CV[`@Qa\0\xE9\x91\x90a\t{V[`@Q\x80\x91\x03\x90\xF3[a\x01\x05a\x01\x006`\x04a\t\xE6V[a\x02\xAEV[`@Q\x90\x15\x15\x81R` \x01a\0\xE9V[a\x01(a\x01#6`\x04a\n\xC4V[a\x03\x1BV[\0[`\x03T[`@Q\x90\x81R` \x01a\0\xE9V[a\x01\x05a\x01J6`\x04a\x0B8V[a\x03\xBAV[`\x02T`@Q`\xFF\x90\x91\x16\x81R` \x01a\0\xE9V[a\x01.a\x04\xCFV[a\x01.a\x01z6`\x04a\x0BtV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x04` R`@\x90 T\x90V[a\x01.a\x01\xA36`\x04a\x0BtV[`\x08` R`\0\x90\x81R`@\x90 T\x81V[a\0\xDCa\x04\xF5V[a\x01\x05a\x01\xCB6`\x04a\t\xE6V[a\x05\x04V[a\x01(a\x01\xDE6`\x04a\x0B\x8FV[a\x05\x9BV[a\x01.a\x01\xF16`\x04a\x0B\xF9V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[```\0\x80Ta\x02+\x90a\x0C,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02W\x90a\x0C,V[\x80\x15a\x02\xA4W\x80`\x1F\x10a\x02yWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xA4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x87W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[3`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x03\t\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\tT`\xFF\x16\x15a\x03iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10S\x14\x91PQ\x16W\xD2S\x92U\x12PS\x12V\x91Q`j\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x03u\x84\x82a\x0C\xB7V[P`\x01a\x03\x82\x83\x82a\x0C\xB7V[P`\x02\x80T`\xFF\x19\x16`\xFF\x83\x16\x17\x90Ua\x03\x9Aa\x07\xF9V[`\x06Ua\x03\xA5a\x08\x12V[`\x07UPP`\t\x80T`\xFF\x19\x16`\x01\x17\x90UPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a\x04\x16Wa\x03\xF1\x81\x84a\x08\xB5V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x04` R`@\x90 Ta\x049\x90\x84a\x08\xB5V[`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\0\x90\x81R`\x04` R`@\x80\x82 \x93\x90\x93U\x90\x86\x16\x81R Ta\x04h\x90\x84a\t\x18V[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x81\x81R`\x04` R`@\x90\x81\x90 \x93\x90\x93U\x91Q\x90\x87\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x04\xBC\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`\0`\x06Ta\x04\xDCa\x07\xF9V[\x14a\x04\xEEWa\x04\xE9a\x08\x12V[\x90P\x90V[P`\x07T\x90V[```\x01\x80Ta\x02+\x90a\x0C,V[3`\0\x90\x81R`\x04` R`@\x81 Ta\x05\x1E\x90\x83a\x08\xB5V[3`\0\x90\x81R`\x04` R`@\x80\x82 \x92\x90\x92U`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R Ta\x05J\x90\x83a\t\x18V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81R`\x04` R`@\x90\x81\x90 \x92\x90\x92U\x90Q3\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x03\t\x90\x86\x81R` \x01\x90V[B\x84\x10\x15a\x05\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03`V[`\0`\x01a\x05\xF7a\x04\xCFV[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`\x08` R`@\x81 \x80T\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x92\x8D\x92\x8D\x92\x8D\x92\x90\x91\x90a\x06E\x83a\r\x8DV[\x90\x91UP`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x88\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x06\xBE\x92\x91\x90a\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x07\x1CW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x07RWP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x07\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\x03`V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x8B\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x8A\x90UQ\x89\x81R\x91\x92\x8B\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPPV[`\0a\tw\x80a\x08\x0Bc\xFF\xFF\xFF\xFF\x82\x16V[\x92PPP\x90V[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa\x08D\x91\x90a\r\xA6V[`@Q\x80\x91\x03\x90 \x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6a\x08ua\x07\xF9V[`@\x80Q` \x81\x01\x95\x90\x95R\x84\x01\x92\x90\x92R``\x83\x01R`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x81\x83\x10\x15a\t\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC20: subtraction underflow\0\0\0\0`D\x82\x01R`d\x01a\x03`V[a\t\x11\x82\x84a\x0E\x1CV[\x93\x92PPPV[`\0\x80a\t%\x83\x85a\x0E/V[\x90P\x83\x81\x10\x15a\t\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FERC20: addition overflow\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03`V[F\x90V[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\t\xA9W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\t\x8DV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xE1W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\t\xF9W`\0\x80\xFD[a\n\x02\x83a\t\xCAV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\n7W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\nRWa\nRa\n\x10V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\nzWa\nza\n\x10V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\n\x93W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a\t\xE1W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\xD9W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xF1W`\0\x80\xFD[a\n\xFD\x87\x83\x88\x01a\n&V[\x94P` \x86\x015\x91P\x80\x82\x11\x15a\x0B\x13W`\0\x80\xFD[Pa\x0B \x86\x82\x87\x01a\n&V[\x92PPa\x0B/`@\x85\x01a\n\xB3V[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0BMW`\0\x80\xFD[a\x0BV\x84a\t\xCAV[\x92Pa\x0Bd` \x85\x01a\t\xCAV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0B\x86W`\0\x80\xFD[a\t\x11\x82a\t\xCAV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x0B\xAAW`\0\x80\xFD[a\x0B\xB3\x88a\t\xCAV[\x96Pa\x0B\xC1` \x89\x01a\t\xCAV[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa\x0B\xDD`\x80\x89\x01a\n\xB3V[\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x0CW`\0\x80\xFD[a\x0C\x15\x83a\t\xCAV[\x91Pa\x0C#` \x84\x01a\t\xCAV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C@W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C`WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x0C\xB2W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0C\x8FWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0C\xAEW\x82\x81U`\x01\x01a\x0C\x9BV[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xD1Wa\x0C\xD1a\n\x10V[a\x0C\xE5\x81a\x0C\xDF\x84Ta\x0C,V[\x84a\x0CfV[` \x80`\x1F\x83\x11`\x01\x81\x14a\r\x1AW`\0\x84\x15a\r\x02WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0C\xAEV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\rIW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\r*V[P\x85\x82\x10\x15a\rgW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\r\x9FWa\r\x9Fa\rwV[P`\x01\x01\x90V[`\0\x80\x83Ta\r\xB4\x81a\x0C,V[`\x01\x82\x81\x16\x80\x15a\r\xCCW`\x01\x81\x14a\r\xE1Wa\x0E\x10V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x0E\x10V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x0E\x07W\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\r\xEEV[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[\x81\x81\x03\x81\x81\x11\x15a\x03\x15Wa\x03\x15a\rwV[\x80\x82\x01\x80\x82\x11\x15a\x03\x15Wa\x03\x15a\rwV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static MOCKERC20_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockERC20<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockERC20<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockERC20<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockERC20<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockERC20<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockERC20))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockERC20<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKERC20_ABI.clone(),
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
                MOCKERC20_ABI.clone(),
                MOCKERC20_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x1624f6c6) function
        pub fn initialize(
            &self,
            name: ::std::string::String,
            symbol: ::std::string::String,
            decimals: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 36, 246, 198], (name, symbol, decimals))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalFilter> {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MockERC20Events> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MockERC20<M> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    pub enum MockERC20Events {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockERC20Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockERC20Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockERC20Events::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockERC20Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for MockERC20Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for MockERC20Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(string,string,uint8)` and selector `0x1624f6c6`
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
    #[ethcall(name = "initialize", abi = "initialize(string,string,uint8)")]
    pub struct InitializeCall {
        pub name: ::std::string::String,
        pub symbol: ::std::string::String,
        pub decimals: u8,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    pub enum MockERC20Calls {
        DomainSeparator(DomainSeparatorCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        Initialize(InitializeCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockERC20Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded) = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allowance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Transfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockERC20Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for MockERC20Calls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for MockERC20Calls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for MockERC20Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockERC20Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for MockERC20Calls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for MockERC20Calls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<NameCall> for MockERC20Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for MockERC20Calls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PermitCall> for MockERC20Calls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for MockERC20Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for MockERC20Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for MockERC20Calls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for MockERC20Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
}
