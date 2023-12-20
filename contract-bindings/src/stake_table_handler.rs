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
                    ::std::borrow::ToOwned::to_owned("nextExitEpochBefore"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nextExitEpochBefore",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nextRegistrationEpochBefore"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nextRegistrationEpochBefore",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("pendingExitsBefore"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingExitsBefore"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingRegistrationsBefore"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingRegistrationsBefore",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("registrationSuccessful"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registrationSuccessful",),
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
                    ::std::borrow::ToOwned::to_owned("requestExitSuccessful"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestExitSuccessful",),
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
                    ::std::borrow::ToOwned::to_owned("stakeTableFirstAvailableExitEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeTableFirstAvailableExitEpoch",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stakeTableFirstAvailableRegistrationEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "stakeTableFirstAvailableRegistrationEpoch",
                        ),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stakeTableNumPendingExits"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeTableNumPendingExits",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stakeTableNumPendingRegistrations"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeTableNumPendingRegistrations",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x19\xAD8\x03\x80b\0\x19\xAD\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01hV[`\x10\x80T`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x12\x80T\x86\x84\x16\x90\x83\x16\x17\x90U`\x11\x80T\x87\x84\x16\x90\x83\x16\x17\x90U`\x15\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x80Qb\0\0\x93\x90`\x16\x90` \x84\x01\x90b\0\0\xA6V[PP`\x16T`\x17UPb\0\x02\x94\x92PPPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15b\0\0\xFEW\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\0\xFEW\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90b\0\0\xC7V[Pb\0\x01\x0C\x92\x91Pb\0\x01\x10V[P\x90V[[\x80\x82\x11\x15b\0\x01\x0CW`\0\x81U`\x01\x01b\0\x01\x11V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01=W`\0\x80\xFD[PV[\x80Qb\0\x01M\x81b\0\x01'V[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01\x81W`\0\x80\xFD[\x85Qb\0\x01\x8E\x81b\0\x01'V[\x80\x95PP` \x80\x87\x01Qb\0\x01\xA3\x81b\0\x01'V[`@\x88\x01Q\x90\x95Pb\0\x01\xB6\x81b\0\x01'V[``\x88\x01Q\x90\x94Pb\0\x01\xC9\x81b\0\x01'V[`\x80\x88\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x01\xE7W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12b\0\x01\xFCW`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x02\x11Wb\0\x02\x11b\0\x01RV[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15b\0\x029Wb\0\x029b\0\x01RV[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x8C\x83\x11\x15b\0\x02XW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15b\0\x02\x81Wb\0\x02q\x85b\0\x01@V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92b\0\x02]V[\x80\x96PPPPPPP\x92\x95P\x92\x95\x90\x93PV[a\x17\t\x80b\0\x02\xA4`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80c\x92H\xDDD\x11a\0\xB8W\x80c\xC2;\xBA\xC5\x11a\0|W\x80c\xC2;\xBA\xC5\x14a\x02\xD9W\x80c\xDB\x84%,\x14a\x02\xF3W\x80c\xE3Y%R\x14a\x03\x06W\x80c\xF5\xA0n\x9D\x14a\x03[W\x80c\xFA\x18/\xA1\x14a\x03hW\x80c\xFC\x0CTj\x14a\x03{W`\0\x80\xFD[\x80c\x92H\xDDD\x14a\x02hW\x80c\x9601h\x14a\x02\x82W\x80c\xB5p\x0Eh\x14a\x02\x9CW\x80c\xBBG\x10\xC5\x14a\x02\xAFW\x80c\xBC\xC4\xF0\xDD\x14a\x02\xC6W`\0\x80\xFD[\x80c<\xF8\x0El\x11a\x01\nW\x80c<\xF8\x0El\x14a\x01\xFCW\x80cRu/\xCE\x14a\x02\x04W\x80c\\\x05\x03G\x14a\x02\x1CW\x80cr\x1Ce\x13\x14a\x02/W\x80c\x7F\xAE\xB4\xEF\x14a\x02BW\x80c\x88M\xA7}\x14a\x02UW`\0\x80\xFD[\x80c\x01v\xA3\xE4\x14a\x01GW\x80c\x15]\xD5\xEE\x14a\x01~W\x80c%\xA2\xC5\x9B\x14a\x01\x93W\x80c*\x1B\xF7d\x14a\x01\xADW\x80c6[\x98\xB2\x14a\x01\xD1W[`\0\x80\xFD[`\x18Ta\x01a\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x91a\x01\x8C6`\x04a\x13\x16V[a\x03\x8EV[\0[`\x19Ta\x01a\x90`\x01`\x88\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x1ATa\x01\xC1\x90`\x01`@\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01uV[a\x01\xE4a\x01\xDF6`\x04a\x13\x16V[a\x05\xD8V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01uV[a\x01\x91a\x06\x02V[`\x19Ta\x01a\x90a\x01\0\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x10Ta\x01\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x91a\x02=6`\x04a\x13\x16V[a\x06\xEAV[a\x01\x91a\x02P6`\x04a\x13GV[a\n0V[`\x1ATa\x01a\x90`\x01`\x01`@\x1B\x03\x16\x81V[`\x19Ta\x01a\x90`\x01`H\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x18Ta\x01a\x90`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x15Ta\x01\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xB8`\x17T\x81V[`@Q\x90\x81R` \x01a\x01uV[`\x18Ta\x01a\x90`\x01`\x01`@\x1B\x03\x16\x81V[`\x18Ta\x01a\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x11Ta\x01\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03;a\x03\x146`\x04a\x13\x16V[`\x13` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x92\x90\x91\x84V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x01uV[`\x19Ta\x01\xC1\x90`\xFF\x16\x81V[a\x03;a\x03v6`\x04a\x13\x16V[a\x0CLV[`\x12Ta\x01\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0a\x03\xAA\x82`\0`\x01`\x17Ta\x03\xA5\x91\x90a\x13\x9CV[a\x0C\x86V[`\0\x81\x81R`\x13` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x81\x85\x01R`\x02\x82\x01T\x81\x84\x01R`\x03\x90\x91\x01T``\x82\x01R`\x15T\x82Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x92Q\x95\x96P\x90\x94`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92cvg\x18\x08\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04P\x91\x90a\x13\xB5V[\x90P`d`\0a\x04`\x82\x84a\x13\xD2V[`\x15T`@Qc9I\xD1\xE9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c9I\xD1\xE9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xAFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xC3W=`\0\x80>=`\0\xFD[PP`\x16\x80Tsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x93Pc\xCAf\x9F\xA7\x92P\x88\x90\x81\x10a\x04\xF7Wa\x04\xF7a\x13\xF9V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05XW=`\0\x80>=`\0\xFD[PP`\x10T`@Qc\x01\x84\x95\xE3`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\x0C$\xAF\x18\x91Pa\x05\x8C\x90\x87\x90`\x04\x01a\x14\x0FV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xCF\x91\x90a\x13\xB5V[PPPPPPPV[`\x16\x81\x81T\x81\x10a\x05\xE8W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\x15T`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cvg\x18\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x06LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06p\x91\x90a\x13\xB5V[\x90P`\0a\x06\x7F\x82`\x01a\x13\xD2V[`\x15T`@Qc9I\xD1\xE9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c9I\xD1\xE9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xCEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xE2W=`\0\x80>=`\0\xFD[PPPPPPV[`\0a\x07\x01\x82`\0`\x01`\x17Ta\x03\xA5\x91\x90a\x13\x9CV[`\x10T`@\x80Qc;\t\xC2g`\xE0\x1B\x81R\x81Q\x93\x94P`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c;\t\xC2g\x92`\x04\x80\x82\x01\x93\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x07JW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07n\x91\x90a\x14:V[`\x19\x80Tp\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16`\x01`H\x1B`\x01`\x01`@\x1B\x03\x93\x84\x16\x02h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16\x17a\x01\0\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U`\x16\x80Tsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xCAf\x9F\xA7\x91\x84\x90\x81\x10a\x07\xE2Wa\x07\xE2a\x13\xF9V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08/W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08CW=`\0\x80>=`\0\xFD[PP`\x10T`\0\x84\x81R`\x13` R`@\x80\x82 \x90QcJ\xA7\xC2\x7F`\xE0\x1B\x81R\x81T`\x04\x82\x01R`\x01\x82\x01T`$\x82\x01R`\x02\x82\x01T`D\x82\x01R`\x03\x90\x91\x01T`d\x82\x01R\x90\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91PcJ\xA7\xC2\x7F\x90`\x84\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xE5\x91\x90a\x14iV[\x90P`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xE43=\xB5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t^\x91\x90a\x13\xB5V[`\x19`\x11a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xD6{l\xA5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xFB\x91\x90a\x13\xB5V[`\x1A\x80T\x92\x15\x15`\x01`@\x1B\x02h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90UPPV[`\x10T`@\x80Qc\x0B\x14\xC1a`\xE2\x1B\x81R\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c,S\x05\x84\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\nwW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9B\x91\x90a\x14:V[`\x18\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`@\x1B`\x01`\x01`@\x1B\x03\x93\x84\x16\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x92\x82\x16\x92\x90\x92\x17\x90\x91U`\x11T`\0\x91a\n\xF9\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\xFF\x86\x16\x90\x85\x16a\x0C\xCAV[\x90P`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xA6\xE2\xE3\xDC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BNW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Br\x91\x90a\x13\xB5V[`\x18`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x16\xFE\xFE\xD7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x0F\x91\x90a\x13\xB5V[`\x18\x80T`\x01`\x01`\xC0\x1B\x03\x16`\x01`\xC0\x1B`\x01`\x01`@\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90U`\x19\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UPPV[`\x14\x81\x81T\x81\x10a\x0C\\W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x93P\x91\x90\x84V[`\0a\x0C\x93\x84\x84\x84a\x10\xB4V[\x90Pa\x0C\xC3`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x14\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82a\x12|V[\x93\x92PPPV[`\0a\x0C\xE1\x83`\0`\x01`\x17Ta\x03\xA5\x91\x90a\x13\x9CV[\x92P`\0\x83\x90P`\0`\x16\x85\x81T\x81\x10a\x0C\xFDWa\x0C\xFDa\x13\xF9V[`\0\x91\x82R` \x82 \x01T`\x0FT`@Qc5\xD6\x9C\x89`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`\xFF\x87\x16`$\x83\x01R\x92\x83\x16\x94P\x83\x92\x83\x92\x16\x90ck\xAD9\x12\x90`D\x01a\x01\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\reW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x89\x91\x90a\x15!V[\x92P\x92P\x92P`\0a\r\x9E\x88`\0`da\x0C\x86V[`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R\x90\x91Pa\x03\xE8\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xFAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x0EW=`\0\x80>=`\0\xFD[PP`\x12T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R`\x01`\x01`@\x1B\x03\x87\x16`$\x83\x01R\x90\x91\x16\x92Pc\xA9\x05\x9C\xBB\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0EmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x91\x91\x90a\x14iV[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xFBW=`\0\x80>=`\0\xFD[PP`\x12T`\x10T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`\x01`\x01`@\x1B\x03\x87\x16`$\x82\x01R\x91\x16\x92Pc\t^\xA7\xB3\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0F\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x80\x91\x90a\x14iV[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xD6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xEAW=`\0\x80>=`\0\xFD[PP`\x10T`@Qc\xC7,\xC7\x17`\xE0\x1B\x81R`\0\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xC7,\xC7\x17\x90a\x10+\x90\x89\x90\x89\x90\x88\x90\x87\x90\x8B\x90\x8A\x90`\x04\x01a\x15\xB8V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x10JW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10n\x91\x90a\x14iV[`\0\x9B\x8CR`\x13` \x90\x81R`@\x9C\x8D\x90 \x88Q\x81U\x90\x88\x01Q`\x01\x82\x01U\x9B\x87\x01Q`\x02\x8D\x01U``\x90\x96\x01Q`\x03\x90\x9B\x01\x9A\x90\x9AUP\x92\x99\x98PPPPPPPPPV[`\0\x81\x83\x11\x15a\x110W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x11@WP\x81\x84\x11\x15[\x15a\x11LWP\x82a\x0C\xC3V[`\0a\x11X\x84\x84a\x13\x9CV[a\x11c\x90`\x01a\x16LV[\x90P`\x03\x85\x11\x15\x80\x15a\x11uWP\x84\x81\x11[\x15a\x11\x8CWa\x11\x84\x85\x85a\x16LV[\x91PPa\x0C\xC3V[a\x11\x99`\x03`\0\x19a\x13\x9CV[\x85\x10\x15\x80\x15a\x11\xB2WPa\x11\xAF\x85`\0\x19a\x13\x9CV[\x81\x11[\x15a\x11\xCDWa\x11\xC3\x85`\0\x19a\x13\x9CV[a\x11\x84\x90\x84a\x13\x9CV[\x82\x85\x11\x15a\x12#W`\0a\x11\xE1\x84\x87a\x13\x9CV[\x90P`\0a\x11\xEF\x83\x83a\x16_V[\x90P\x80`\0\x03a\x12\x04W\x84\x93PPPPa\x0C\xC3V[`\x01a\x12\x10\x82\x88a\x16LV[a\x12\x1A\x91\x90a\x13\x9CV[\x93PPPa\x12tV[\x83\x85\x10\x15a\x12tW`\0a\x127\x86\x86a\x13\x9CV[\x90P`\0a\x12E\x83\x83a\x16_V[\x90P\x80`\0\x03a\x12ZW\x85\x93PPPPa\x0C\xC3V[a\x12d\x81\x86a\x13\x9CV[a\x12o\x90`\x01a\x16LV[\x93PPP[P\x93\x92PPPV[`\0jconsole.log`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Q`$\x01a\x12\xA6\x92\x91\x90a\x16\xA5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90RQa\x12\xDB\x91\x90a\x16\xE0V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x06\xE2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\xE2V[`\0` \x82\x84\x03\x12\x15a\x13(W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x13DW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x13ZW`\0\x80\xFD[\x825`\xFF\x81\x16\x81\x14a\x13kW`\0\x80\xFD[\x91P` \x83\x015a\x13{\x81a\x13/V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x13\xAFWa\x13\xAFa\x13\x86V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x13\xC7W`\0\x80\xFD[\x81Qa\x0C\xC3\x81a\x13/V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x13\xF2Wa\x13\xF2a\x13\x86V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x81\x01a\x13\xAFV[`\0\x80`@\x83\x85\x03\x12\x15a\x14MW`\0\x80\xFD[\x82Qa\x14X\x81a\x13/V[` \x84\x01Q\x90\x92Pa\x13{\x81a\x13/V[`\0` \x82\x84\x03\x12\x15a\x14{W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0C\xC3W`\0\x80\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xBBWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xBBWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a\x15\x03W`\0\x80\xFD[a\x15\x0Ba\x14\x8BV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R\x92\x91PPV[`\0\x80`\0\x83\x85\x03a\x01\0\x81\x12\x15a\x158W`\0\x80\xFD[`\x80\x81\x12\x15a\x15FW`\0\x80\xFD[a\x15Na\x14\xC1V[\x85Q\x81R` \x86\x01Q` \x82\x01R`@\x86\x01Q`@\x82\x01R``\x86\x01Q``\x82\x01R\x80\x94PP`@`\x7F\x19\x82\x01\x12\x15a\x15\x86W`\0\x80\xFD[Pa\x15\x8Fa\x14\x8BV[`\x80\x85\x01Q\x81R`\xA0\x85\x01Q` \x82\x01R\x91Pa\x15\xAF\x85`\xC0\x86\x01a\x14\xF1V[\x90P\x92P\x92P\x92V[\x86Q\x81R` \x80\x88\x01Q\x90\x82\x01R`@\x80\x88\x01Q\x90\x82\x01R``\x80\x88\x01Q\x90\x82\x01Ra\x01`\x81\x01\x86Q`\x80\x83\x01R` \x87\x01Q`\xA0\x83\x01R`\x01`\x01`@\x1B\x03\x80\x87\x16`\xC0\x84\x01R`\x02\x86\x10a\x16\x1EWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x85`\xE0\x84\x01R\x84Qa\x01\0\x84\x01R` \x85\x01Qa\x01 \x84\x01R\x80\x84\x16a\x01@\x84\x01RP\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x13\xAFWa\x13\xAFa\x13\x86V[`\0\x82a\x16|WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0[\x83\x81\x10\x15a\x16\x9CW\x81\x81\x01Q\x83\x82\x01R` \x01a\x16\x84V[PP`\0\x91\x01RV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x16\xC4\x81``\x85\x01` \x88\x01a\x16\x81V[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01`\x1F\x19\x16\x01``\x01\x91\x90PV[`\0\x82Qa\x16\xF2\x81\x84` \x87\x01a\x16\x81V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static STAKETABLEHANDLER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80c\x92H\xDDD\x11a\0\xB8W\x80c\xC2;\xBA\xC5\x11a\0|W\x80c\xC2;\xBA\xC5\x14a\x02\xD9W\x80c\xDB\x84%,\x14a\x02\xF3W\x80c\xE3Y%R\x14a\x03\x06W\x80c\xF5\xA0n\x9D\x14a\x03[W\x80c\xFA\x18/\xA1\x14a\x03hW\x80c\xFC\x0CTj\x14a\x03{W`\0\x80\xFD[\x80c\x92H\xDDD\x14a\x02hW\x80c\x9601h\x14a\x02\x82W\x80c\xB5p\x0Eh\x14a\x02\x9CW\x80c\xBBG\x10\xC5\x14a\x02\xAFW\x80c\xBC\xC4\xF0\xDD\x14a\x02\xC6W`\0\x80\xFD[\x80c<\xF8\x0El\x11a\x01\nW\x80c<\xF8\x0El\x14a\x01\xFCW\x80cRu/\xCE\x14a\x02\x04W\x80c\\\x05\x03G\x14a\x02\x1CW\x80cr\x1Ce\x13\x14a\x02/W\x80c\x7F\xAE\xB4\xEF\x14a\x02BW\x80c\x88M\xA7}\x14a\x02UW`\0\x80\xFD[\x80c\x01v\xA3\xE4\x14a\x01GW\x80c\x15]\xD5\xEE\x14a\x01~W\x80c%\xA2\xC5\x9B\x14a\x01\x93W\x80c*\x1B\xF7d\x14a\x01\xADW\x80c6[\x98\xB2\x14a\x01\xD1W[`\0\x80\xFD[`\x18Ta\x01a\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x91a\x01\x8C6`\x04a\x13\x16V[a\x03\x8EV[\0[`\x19Ta\x01a\x90`\x01`\x88\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x1ATa\x01\xC1\x90`\x01`@\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01uV[a\x01\xE4a\x01\xDF6`\x04a\x13\x16V[a\x05\xD8V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01uV[a\x01\x91a\x06\x02V[`\x19Ta\x01a\x90a\x01\0\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x10Ta\x01\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x91a\x02=6`\x04a\x13\x16V[a\x06\xEAV[a\x01\x91a\x02P6`\x04a\x13GV[a\n0V[`\x1ATa\x01a\x90`\x01`\x01`@\x1B\x03\x16\x81V[`\x19Ta\x01a\x90`\x01`H\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x18Ta\x01a\x90`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x15Ta\x01\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xB8`\x17T\x81V[`@Q\x90\x81R` \x01a\x01uV[`\x18Ta\x01a\x90`\x01`\x01`@\x1B\x03\x16\x81V[`\x18Ta\x01a\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x11Ta\x01\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03;a\x03\x146`\x04a\x13\x16V[`\x13` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x92\x90\x91\x84V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x01uV[`\x19Ta\x01\xC1\x90`\xFF\x16\x81V[a\x03;a\x03v6`\x04a\x13\x16V[a\x0CLV[`\x12Ta\x01\xE4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0a\x03\xAA\x82`\0`\x01`\x17Ta\x03\xA5\x91\x90a\x13\x9CV[a\x0C\x86V[`\0\x81\x81R`\x13` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x81\x85\x01R`\x02\x82\x01T\x81\x84\x01R`\x03\x90\x91\x01T``\x82\x01R`\x15T\x82Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x92Q\x95\x96P\x90\x94`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92cvg\x18\x08\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04P\x91\x90a\x13\xB5V[\x90P`d`\0a\x04`\x82\x84a\x13\xD2V[`\x15T`@Qc9I\xD1\xE9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c9I\xD1\xE9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xAFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xC3W=`\0\x80>=`\0\xFD[PP`\x16\x80Tsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x93Pc\xCAf\x9F\xA7\x92P\x88\x90\x81\x10a\x04\xF7Wa\x04\xF7a\x13\xF9V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05XW=`\0\x80>=`\0\xFD[PP`\x10T`@Qc\x01\x84\x95\xE3`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\x0C$\xAF\x18\x91Pa\x05\x8C\x90\x87\x90`\x04\x01a\x14\x0FV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xCF\x91\x90a\x13\xB5V[PPPPPPPV[`\x16\x81\x81T\x81\x10a\x05\xE8W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\x15T`@\x80Qc\x0E\xCC\xE3\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cvg\x18\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x06LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06p\x91\x90a\x13\xB5V[\x90P`\0a\x06\x7F\x82`\x01a\x13\xD2V[`\x15T`@Qc9I\xD1\xE9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c9I\xD1\xE9\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xCEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xE2W=`\0\x80>=`\0\xFD[PPPPPPV[`\0a\x07\x01\x82`\0`\x01`\x17Ta\x03\xA5\x91\x90a\x13\x9CV[`\x10T`@\x80Qc;\t\xC2g`\xE0\x1B\x81R\x81Q\x93\x94P`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c;\t\xC2g\x92`\x04\x80\x82\x01\x93\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x07JW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07n\x91\x90a\x14:V[`\x19\x80Tp\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16`\x01`H\x1B`\x01`\x01`@\x1B\x03\x93\x84\x16\x02h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16\x17a\x01\0\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U`\x16\x80Tsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xCAf\x9F\xA7\x91\x84\x90\x81\x10a\x07\xE2Wa\x07\xE2a\x13\xF9V[`\0\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08/W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08CW=`\0\x80>=`\0\xFD[PP`\x10T`\0\x84\x81R`\x13` R`@\x80\x82 \x90QcJ\xA7\xC2\x7F`\xE0\x1B\x81R\x81T`\x04\x82\x01R`\x01\x82\x01T`$\x82\x01R`\x02\x82\x01T`D\x82\x01R`\x03\x90\x91\x01T`d\x82\x01R\x90\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91PcJ\xA7\xC2\x7F\x90`\x84\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xE5\x91\x90a\x14iV[\x90P`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xE43=\xB5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t^\x91\x90a\x13\xB5V[`\x19`\x11a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xD6{l\xA5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xFB\x91\x90a\x13\xB5V[`\x1A\x80T\x92\x15\x15`\x01`@\x1B\x02h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90UPPV[`\x10T`@\x80Qc\x0B\x14\xC1a`\xE2\x1B\x81R\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c,S\x05\x84\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\nwW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9B\x91\x90a\x14:V[`\x18\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`@\x1B`\x01`\x01`@\x1B\x03\x93\x84\x16\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x92\x82\x16\x92\x90\x92\x17\x90\x91U`\x11T`\0\x91a\n\xF9\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\xFF\x86\x16\x90\x85\x16a\x0C\xCAV[\x90P`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xA6\xE2\xE3\xDC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BNW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Br\x91\x90a\x13\xB5V[`\x18`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\x10`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x16\xFE\xFE\xD7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x0F\x91\x90a\x13\xB5V[`\x18\x80T`\x01`\x01`\xC0\x1B\x03\x16`\x01`\xC0\x1B`\x01`\x01`@\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90U`\x19\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UPPV[`\x14\x81\x81T\x81\x10a\x0C\\W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x93P\x91\x90\x84V[`\0a\x0C\x93\x84\x84\x84a\x10\xB4V[\x90Pa\x0C\xC3`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x14\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82a\x12|V[\x93\x92PPPV[`\0a\x0C\xE1\x83`\0`\x01`\x17Ta\x03\xA5\x91\x90a\x13\x9CV[\x92P`\0\x83\x90P`\0`\x16\x85\x81T\x81\x10a\x0C\xFDWa\x0C\xFDa\x13\xF9V[`\0\x91\x82R` \x82 \x01T`\x0FT`@Qc5\xD6\x9C\x89`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`\xFF\x87\x16`$\x83\x01R\x92\x83\x16\x94P\x83\x92\x83\x92\x16\x90ck\xAD9\x12\x90`D\x01a\x01\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\reW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x89\x91\x90a\x15!V[\x92P\x92P\x92P`\0a\r\x9E\x88`\0`da\x0C\x86V[`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R\x90\x91Pa\x03\xE8\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xFAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x0EW=`\0\x80>=`\0\xFD[PP`\x12T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R`\x01`\x01`@\x1B\x03\x87\x16`$\x83\x01R\x90\x91\x16\x92Pc\xA9\x05\x9C\xBB\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0EmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x91\x91\x90a\x14iV[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xFBW=`\0\x80>=`\0\xFD[PP`\x12T`\x10T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`\x01`\x01`@\x1B\x03\x87\x16`$\x82\x01R\x91\x16\x92Pc\t^\xA7\xB3\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0F\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x80\x91\x90a\x14iV[P`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xD6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xEAW=`\0\x80>=`\0\xFD[PP`\x10T`@Qc\xC7,\xC7\x17`\xE0\x1B\x81R`\0\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xC7,\xC7\x17\x90a\x10+\x90\x89\x90\x89\x90\x88\x90\x87\x90\x8B\x90\x8A\x90`\x04\x01a\x15\xB8V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x10JW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10n\x91\x90a\x14iV[`\0\x9B\x8CR`\x13` \x90\x81R`@\x9C\x8D\x90 \x88Q\x81U\x90\x88\x01Q`\x01\x82\x01U\x9B\x87\x01Q`\x02\x8D\x01U``\x90\x96\x01Q`\x03\x90\x9B\x01\x9A\x90\x9AUP\x92\x99\x98PPPPPPPPPV[`\0\x81\x83\x11\x15a\x110W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a\x11@WP\x81\x84\x11\x15[\x15a\x11LWP\x82a\x0C\xC3V[`\0a\x11X\x84\x84a\x13\x9CV[a\x11c\x90`\x01a\x16LV[\x90P`\x03\x85\x11\x15\x80\x15a\x11uWP\x84\x81\x11[\x15a\x11\x8CWa\x11\x84\x85\x85a\x16LV[\x91PPa\x0C\xC3V[a\x11\x99`\x03`\0\x19a\x13\x9CV[\x85\x10\x15\x80\x15a\x11\xB2WPa\x11\xAF\x85`\0\x19a\x13\x9CV[\x81\x11[\x15a\x11\xCDWa\x11\xC3\x85`\0\x19a\x13\x9CV[a\x11\x84\x90\x84a\x13\x9CV[\x82\x85\x11\x15a\x12#W`\0a\x11\xE1\x84\x87a\x13\x9CV[\x90P`\0a\x11\xEF\x83\x83a\x16_V[\x90P\x80`\0\x03a\x12\x04W\x84\x93PPPPa\x0C\xC3V[`\x01a\x12\x10\x82\x88a\x16LV[a\x12\x1A\x91\x90a\x13\x9CV[\x93PPPa\x12tV[\x83\x85\x10\x15a\x12tW`\0a\x127\x86\x86a\x13\x9CV[\x90P`\0a\x12E\x83\x83a\x16_V[\x90P\x80`\0\x03a\x12ZW\x85\x93PPPPa\x0C\xC3V[a\x12d\x81\x86a\x13\x9CV[a\x12o\x90`\x01a\x16LV[\x93PPP[P\x93\x92PPPV[`\0jconsole.log`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Q`$\x01a\x12\xA6\x92\x91\x90a\x16\xA5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90RQa\x12\xDB\x91\x90a\x16\xE0V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x06\xE2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\xE2V[`\0` \x82\x84\x03\x12\x15a\x13(W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x13DW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x13ZW`\0\x80\xFD[\x825`\xFF\x81\x16\x81\x14a\x13kW`\0\x80\xFD[\x91P` \x83\x015a\x13{\x81a\x13/V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x13\xAFWa\x13\xAFa\x13\x86V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x13\xC7W`\0\x80\xFD[\x81Qa\x0C\xC3\x81a\x13/V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x13\xF2Wa\x13\xF2a\x13\x86V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x81\x01a\x13\xAFV[`\0\x80`@\x83\x85\x03\x12\x15a\x14MW`\0\x80\xFD[\x82Qa\x14X\x81a\x13/V[` \x84\x01Q\x90\x92Pa\x13{\x81a\x13/V[`\0` \x82\x84\x03\x12\x15a\x14{W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0C\xC3W`\0\x80\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xBBWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xBBWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a\x15\x03W`\0\x80\xFD[a\x15\x0Ba\x14\x8BV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R\x92\x91PPV[`\0\x80`\0\x83\x85\x03a\x01\0\x81\x12\x15a\x158W`\0\x80\xFD[`\x80\x81\x12\x15a\x15FW`\0\x80\xFD[a\x15Na\x14\xC1V[\x85Q\x81R` \x86\x01Q` \x82\x01R`@\x86\x01Q`@\x82\x01R``\x86\x01Q``\x82\x01R\x80\x94PP`@`\x7F\x19\x82\x01\x12\x15a\x15\x86W`\0\x80\xFD[Pa\x15\x8Fa\x14\x8BV[`\x80\x85\x01Q\x81R`\xA0\x85\x01Q` \x82\x01R\x91Pa\x15\xAF\x85`\xC0\x86\x01a\x14\xF1V[\x90P\x92P\x92P\x92V[\x86Q\x81R` \x80\x88\x01Q\x90\x82\x01R`@\x80\x88\x01Q\x90\x82\x01R``\x80\x88\x01Q\x90\x82\x01Ra\x01`\x81\x01\x86Q`\x80\x83\x01R` \x87\x01Q`\xA0\x83\x01R`\x01`\x01`@\x1B\x03\x80\x87\x16`\xC0\x84\x01R`\x02\x86\x10a\x16\x1EWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x85`\xE0\x84\x01R\x84Qa\x01\0\x84\x01R` \x85\x01Qa\x01 \x84\x01R\x80\x84\x16a\x01@\x84\x01RP\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x13\xAFWa\x13\xAFa\x13\x86V[`\0\x82a\x16|WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0[\x83\x81\x10\x15a\x16\x9CW\x81\x81\x01Q\x83\x82\x01R` \x01a\x16\x84V[PP`\0\x91\x01RV[`@\x81R`\0\x83Q\x80`@\x84\x01Ra\x16\xC4\x81``\x85\x01` \x88\x01a\x16\x81V[` \x83\x01\x93\x90\x93RP`\x1F\x91\x90\x91\x01`\x1F\x19\x16\x01``\x01\x91\x90PV[`\0\x82Qa\x16\xF2\x81\x84` \x87\x01a\x16\x81V[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x17\0\n";
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
        ///Calls the contract's `nextExitEpochBefore` (0x52752fce) function
        pub fn next_exit_epoch_before(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([82, 117, 47, 206], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nextRegistrationEpochBefore` (0xbcc4f0dd) function
        pub fn next_registration_epoch_before(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([188, 196, 240, 221], ())
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
        ///Calls the contract's `pendingExitsBefore` (0x9248dd44) function
        pub fn pending_exits_before(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([146, 72, 221, 68], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingRegistrationsBefore` (0xc23bbac5) function
        pub fn pending_registrations_before(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([194, 59, 186, 197], ())
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
        ///Calls the contract's `registrationSuccessful` (0xf5a06e9d) function
        pub fn registration_successful(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([245, 160, 110, 157], ())
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
        ///Calls the contract's `requestExitSuccessful` (0x2a1bf764) function
        pub fn request_exit_successful(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([42, 27, 247, 100], ())
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
        ///Calls the contract's `stakeTableFirstAvailableExitEpoch` (0x25a2c59b) function
        pub fn stake_table_first_available_exit_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([37, 162, 197, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeTableFirstAvailableRegistrationEpoch` (0x0176a3e4) function
        pub fn stake_table_first_available_registration_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([1, 118, 163, 228], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeTableNumPendingExits` (0x884da77d) function
        pub fn stake_table_num_pending_exits(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([136, 77, 167, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeTableNumPendingRegistrations` (0x96303168) function
        pub fn stake_table_num_pending_registrations(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([150, 48, 49, 104], ())
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
    ///Container type for all input parameters for the `nextExitEpochBefore` function with signature `nextExitEpochBefore()` and selector `0x52752fce`
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
    #[ethcall(name = "nextExitEpochBefore", abi = "nextExitEpochBefore()")]
    pub struct NextExitEpochBeforeCall;
    ///Container type for all input parameters for the `nextRegistrationEpochBefore` function with signature `nextRegistrationEpochBefore()` and selector `0xbcc4f0dd`
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
        name = "nextRegistrationEpochBefore",
        abi = "nextRegistrationEpochBefore()"
    )]
    pub struct NextRegistrationEpochBeforeCall;
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
    ///Container type for all input parameters for the `pendingExitsBefore` function with signature `pendingExitsBefore()` and selector `0x9248dd44`
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
    #[ethcall(name = "pendingExitsBefore", abi = "pendingExitsBefore()")]
    pub struct PendingExitsBeforeCall;
    ///Container type for all input parameters for the `pendingRegistrationsBefore` function with signature `pendingRegistrationsBefore()` and selector `0xc23bbac5`
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
        name = "pendingRegistrationsBefore",
        abi = "pendingRegistrationsBefore()"
    )]
    pub struct PendingRegistrationsBeforeCall;
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
    ///Container type for all input parameters for the `registrationSuccessful` function with signature `registrationSuccessful()` and selector `0xf5a06e9d`
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
    #[ethcall(name = "registrationSuccessful", abi = "registrationSuccessful()")]
    pub struct RegistrationSuccessfulCall;
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
    ///Container type for all input parameters for the `requestExitSuccessful` function with signature `requestExitSuccessful()` and selector `0x2a1bf764`
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
    #[ethcall(name = "requestExitSuccessful", abi = "requestExitSuccessful()")]
    pub struct RequestExitSuccessfulCall;
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
    ///Container type for all input parameters for the `stakeTableFirstAvailableExitEpoch` function with signature `stakeTableFirstAvailableExitEpoch()` and selector `0x25a2c59b`
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
        name = "stakeTableFirstAvailableExitEpoch",
        abi = "stakeTableFirstAvailableExitEpoch()"
    )]
    pub struct StakeTableFirstAvailableExitEpochCall;
    ///Container type for all input parameters for the `stakeTableFirstAvailableRegistrationEpoch` function with signature `stakeTableFirstAvailableRegistrationEpoch()` and selector `0x0176a3e4`
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
        name = "stakeTableFirstAvailableRegistrationEpoch",
        abi = "stakeTableFirstAvailableRegistrationEpoch()"
    )]
    pub struct StakeTableFirstAvailableRegistrationEpochCall;
    ///Container type for all input parameters for the `stakeTableNumPendingExits` function with signature `stakeTableNumPendingExits()` and selector `0x884da77d`
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
        name = "stakeTableNumPendingExits",
        abi = "stakeTableNumPendingExits()"
    )]
    pub struct StakeTableNumPendingExitsCall;
    ///Container type for all input parameters for the `stakeTableNumPendingRegistrations` function with signature `stakeTableNumPendingRegistrations()` and selector `0x96303168`
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
        name = "stakeTableNumPendingRegistrations",
        abi = "stakeTableNumPendingRegistrations()"
    )]
    pub struct StakeTableNumPendingRegistrationsCall;
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
        NextExitEpochBefore(NextExitEpochBeforeCall),
        NextRegistrationEpochBefore(NextRegistrationEpochBeforeCall),
        NumberUsers(NumberUsersCall),
        PendingExitsBefore(PendingExitsBeforeCall),
        PendingRegistrationsBefore(PendingRegistrationsBeforeCall),
        Register(RegisterCall),
        RegistrationSuccessful(RegistrationSuccessfulCall),
        RequestExit(RequestExitCall),
        RequestExitSuccessful(RequestExitSuccessfulCall),
        StakeTable(StakeTableCall),
        StakeTableFirstAvailableExitEpoch(StakeTableFirstAvailableExitEpochCall),
        StakeTableFirstAvailableRegistrationEpoch(StakeTableFirstAvailableRegistrationEpochCall),
        StakeTableNumPendingExits(StakeTableNumPendingExitsCall),
        StakeTableNumPendingRegistrations(StakeTableNumPendingRegistrationsCall),
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
            if let Ok(decoded) =
                <NextExitEpochBeforeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NextExitEpochBefore(decoded));
            }
            if let Ok(decoded) =
                <NextRegistrationEpochBeforeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NextRegistrationEpochBefore(decoded));
            }
            if let Ok(decoded) = <NumberUsersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NumberUsers(decoded));
            }
            if let Ok(decoded) =
                <PendingExitsBeforeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingExitsBefore(decoded));
            }
            if let Ok(decoded) =
                <PendingRegistrationsBeforeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingRegistrationsBefore(decoded));
            }
            if let Ok(decoded) = <RegisterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Register(decoded));
            }
            if let Ok(decoded) =
                <RegistrationSuccessfulCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegistrationSuccessful(decoded));
            }
            if let Ok(decoded) = <RequestExitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RequestExit(decoded));
            }
            if let Ok(decoded) =
                <RequestExitSuccessfulCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestExitSuccessful(decoded));
            }
            if let Ok(decoded) = <StakeTableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StakeTable(decoded));
            }
            if let Ok(decoded) =
                <StakeTableFirstAvailableExitEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::StakeTableFirstAvailableExitEpoch(decoded));
            }
            if let Ok(decoded) = <StakeTableFirstAvailableRegistrationEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakeTableFirstAvailableRegistrationEpoch(decoded));
            }
            if let Ok(decoded) =
                <StakeTableNumPendingExitsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeTableNumPendingExits(decoded));
            }
            if let Ok(decoded) =
                <StakeTableNumPendingRegistrationsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::StakeTableNumPendingRegistrations(decoded));
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
                Self::NextExitEpochBefore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NextRegistrationEpochBefore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NumberUsers(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingExitsBefore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingRegistrationsBefore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Register(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegistrationSuccessful(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestExit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestExitSuccessful(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeTable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeTableFirstAvailableExitEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeTableFirstAvailableRegistrationEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeTableNumPendingExits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeTableNumPendingRegistrations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::NextExitEpochBefore(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextRegistrationEpochBefore(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumberUsers(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingExitsBefore(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingRegistrationsBefore(element) => ::core::fmt::Display::fmt(element, f),
                Self::Register(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistrationSuccessful(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestExit(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestExitSuccessful(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTable(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTableFirstAvailableExitEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeTableFirstAvailableRegistrationEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeTableNumPendingExits(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTableNumPendingRegistrations(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<NextExitEpochBeforeCall> for StakeTableHandlerCalls {
        fn from(value: NextExitEpochBeforeCall) -> Self {
            Self::NextExitEpochBefore(value)
        }
    }
    impl ::core::convert::From<NextRegistrationEpochBeforeCall> for StakeTableHandlerCalls {
        fn from(value: NextRegistrationEpochBeforeCall) -> Self {
            Self::NextRegistrationEpochBefore(value)
        }
    }
    impl ::core::convert::From<NumberUsersCall> for StakeTableHandlerCalls {
        fn from(value: NumberUsersCall) -> Self {
            Self::NumberUsers(value)
        }
    }
    impl ::core::convert::From<PendingExitsBeforeCall> for StakeTableHandlerCalls {
        fn from(value: PendingExitsBeforeCall) -> Self {
            Self::PendingExitsBefore(value)
        }
    }
    impl ::core::convert::From<PendingRegistrationsBeforeCall> for StakeTableHandlerCalls {
        fn from(value: PendingRegistrationsBeforeCall) -> Self {
            Self::PendingRegistrationsBefore(value)
        }
    }
    impl ::core::convert::From<RegisterCall> for StakeTableHandlerCalls {
        fn from(value: RegisterCall) -> Self {
            Self::Register(value)
        }
    }
    impl ::core::convert::From<RegistrationSuccessfulCall> for StakeTableHandlerCalls {
        fn from(value: RegistrationSuccessfulCall) -> Self {
            Self::RegistrationSuccessful(value)
        }
    }
    impl ::core::convert::From<RequestExitCall> for StakeTableHandlerCalls {
        fn from(value: RequestExitCall) -> Self {
            Self::RequestExit(value)
        }
    }
    impl ::core::convert::From<RequestExitSuccessfulCall> for StakeTableHandlerCalls {
        fn from(value: RequestExitSuccessfulCall) -> Self {
            Self::RequestExitSuccessful(value)
        }
    }
    impl ::core::convert::From<StakeTableCall> for StakeTableHandlerCalls {
        fn from(value: StakeTableCall) -> Self {
            Self::StakeTable(value)
        }
    }
    impl ::core::convert::From<StakeTableFirstAvailableExitEpochCall> for StakeTableHandlerCalls {
        fn from(value: StakeTableFirstAvailableExitEpochCall) -> Self {
            Self::StakeTableFirstAvailableExitEpoch(value)
        }
    }
    impl ::core::convert::From<StakeTableFirstAvailableRegistrationEpochCall>
        for StakeTableHandlerCalls
    {
        fn from(value: StakeTableFirstAvailableRegistrationEpochCall) -> Self {
            Self::StakeTableFirstAvailableRegistrationEpoch(value)
        }
    }
    impl ::core::convert::From<StakeTableNumPendingExitsCall> for StakeTableHandlerCalls {
        fn from(value: StakeTableNumPendingExitsCall) -> Self {
            Self::StakeTableNumPendingExits(value)
        }
    }
    impl ::core::convert::From<StakeTableNumPendingRegistrationsCall> for StakeTableHandlerCalls {
        fn from(value: StakeTableNumPendingRegistrationsCall) -> Self {
            Self::StakeTableNumPendingRegistrations(value)
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
    ///Container type for all return fields from the `nextExitEpochBefore` function with signature `nextExitEpochBefore()` and selector `0x52752fce`
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
    pub struct NextExitEpochBeforeReturn(pub u64);
    ///Container type for all return fields from the `nextRegistrationEpochBefore` function with signature `nextRegistrationEpochBefore()` and selector `0xbcc4f0dd`
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
    pub struct NextRegistrationEpochBeforeReturn(pub u64);
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
    ///Container type for all return fields from the `pendingExitsBefore` function with signature `pendingExitsBefore()` and selector `0x9248dd44`
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
    pub struct PendingExitsBeforeReturn(pub u64);
    ///Container type for all return fields from the `pendingRegistrationsBefore` function with signature `pendingRegistrationsBefore()` and selector `0xc23bbac5`
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
    pub struct PendingRegistrationsBeforeReturn(pub u64);
    ///Container type for all return fields from the `registrationSuccessful` function with signature `registrationSuccessful()` and selector `0xf5a06e9d`
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
    pub struct RegistrationSuccessfulReturn(pub bool);
    ///Container type for all return fields from the `requestExitSuccessful` function with signature `requestExitSuccessful()` and selector `0x2a1bf764`
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
    pub struct RequestExitSuccessfulReturn(pub bool);
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
    ///Container type for all return fields from the `stakeTableFirstAvailableExitEpoch` function with signature `stakeTableFirstAvailableExitEpoch()` and selector `0x25a2c59b`
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
    pub struct StakeTableFirstAvailableExitEpochReturn(pub u64);
    ///Container type for all return fields from the `stakeTableFirstAvailableRegistrationEpoch` function with signature `stakeTableFirstAvailableRegistrationEpoch()` and selector `0x0176a3e4`
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
    pub struct StakeTableFirstAvailableRegistrationEpochReturn(pub u64);
    ///Container type for all return fields from the `stakeTableNumPendingExits` function with signature `stakeTableNumPendingExits()` and selector `0x884da77d`
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
    pub struct StakeTableNumPendingExitsReturn(pub u64);
    ///Container type for all return fields from the `stakeTableNumPendingRegistrations` function with signature `stakeTableNumPendingRegistrations()` and selector `0x96303168`
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
    pub struct StakeTableNumPendingRegistrationsReturn(pub u64);
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
