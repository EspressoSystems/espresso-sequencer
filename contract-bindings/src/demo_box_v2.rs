pub use demo_box_v2::*;
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
pub mod demo_box_v2 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("UPGRADE_INTERFACE_VERSION"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("UPGRADE_INTERFACE_VERSION",),
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
                    ::std::borrow::ToOwned::to_owned("addBox"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addBox"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_size"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_maxItems"),
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
                    ::std::borrow::ToOwned::to_owned("boxes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("boxes"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("boxOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("size"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("status"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("enum DemoBoxV2.BoxStatus"),
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
                                name: ::std::borrow::ToOwned::to_owned("maxItems"),
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
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("receiver"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBox"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getBox"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct DemoBoxV2.Box"),
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
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("proxiableUUID"),
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
                (
                    ::std::borrow::ToOwned::to_owned("updateBox"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateBox"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_newSize"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_maxItems"),
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
                    ::std::borrow::ToOwned::to_owned("updateBoxCapacity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateBoxCapacity"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_maxItems"),
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
                    ::std::borrow::ToOwned::to_owned("updateBoxStatus"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateBoxStatus"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_status"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("enum DemoBoxV2.BoxStatus"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("version"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("version"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawETH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawETH"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("Upgrade"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Upgrade"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("target"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BoxAlreadyExists"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BoxAlreadyExists"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BoxCapcityTooSmall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BoxCapcityTooSmall"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BoxSizeTooSmall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BoxSizeTooSmall"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1967InvalidImplementation"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1967InvalidImplementation",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedToSendEther"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FailedToSendEther"),
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
                    ::std::borrow::ToOwned::to_owned("NoBoxExists"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NoBoxExists"),
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
                    ::std::borrow::ToOwned::to_owned("UUPSUnauthorizedCallContext"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UUPSUnauthorizedCallContext",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UUPSUnsupportedProxiableUUID"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UUPSUnsupportedProxiableUUID",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("slot"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("YouMustDepositETH"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("YouMustDepositETH"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ZeroBalance"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DEMOBOXV2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R0`\x80R`\x02`\xA0R4\x80\x15a\0\x19W`\0\x80\xFD[Pa\0\"a\0'V[a\0\xD9V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\0wW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a\0\xD6W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80Q`\xA0Qa\x18\xF2a\x01\x0C`\09`\0a\x01b\x01R`\0\x81\x81a\r\xFD\x01R\x81\x81a\x0E&\x01Ra\x10\x93\x01Ra\x18\xF2`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\x0EW`\x005`\xE0\x1C\x80c\xAD<\xB1\xCC\x11a\0\xA5W\x80c\xE0\x86\xE5\xEC\x11a\0tW\x80c\xF2\xFD\xE3\x8B\x11a\0YW\x80c\xF2\xFD\xE3\x8B\x14a\x03mW\x80c\xF3@\xFA\x01\x14a\x03\x8DW\x80c\xF9`\x9F\x08\x14a\x03\xA0W`\0\x80\xFD[\x80c\xE0\x86\xE5\xEC\x14a\x03EW\x80c\xF1\xA0\xD1\xC8\x14a\x03MW`\0\x80\xFD[\x80c\xAD<\xB1\xCC\x14a\x02\x8FW\x80c\xB7'O\xBE\x14a\x02\xE5W\x80c\xC4\xD5\xE2:\x14a\x03\x05W\x80c\xC4\xEB{\xA4\x14a\x03%W`\0\x80\xFD[\x80cq\xD8\xCA\x0B\x11a\0\xE1W\x80cq\xD8\xCA\x0B\x14a\x01\xAEW\x80c\x81)\xFC\x1C\x14a\x01\xD0W\x80c\x8D\xA5\xCB[\x14a\x01\xE5W\x80c\x9A\xA3\xBBE\x14a\x029W`\0\x80\xFD[\x80cO\x1E\xF2\x86\x14a\x01\x13W\x80cR\xD1\x90-\x14a\x01(W\x80cT\xFDMP\x14a\x01PW\x80cqP\x18\xA6\x14a\x01\x99W[`\0\x80\xFD[a\x01&a\x01!6`\x04a\x15\xB1V[a\x03\xB3V[\0[4\x80\x15a\x014W`\0\x80\xFD[Pa\x01=a\x03\xD2V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\\W`\0\x80\xFD[Pa\x01\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01GV[4\x80\x15a\x01\xA5W`\0\x80\xFD[Pa\x01&a\x04\x01V[4\x80\x15a\x01\xBAW`\0\x80\xFD[Pa\x01\xC3a\x04\x15V[`@Qa\x01G\x91\x90a\x16\xFBV[4\x80\x15a\x01\xDCW`\0\x80\xFD[Pa\x01&a\x04\xABV[4\x80\x15a\x01\xF1W`\0\x80\xFD[P\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01GV[4\x80\x15a\x02EW`\0\x80\xFD[Pa\x02\x7Fa\x02T6`\x04a\x171V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x92`\xFF\x90\x91\x16\x91\x84V[`@Qa\x01G\x94\x93\x92\x91\x90a\x17LV[4\x80\x15a\x02\x9BW`\0\x80\xFD[Pa\x02\xD8`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x01G\x91\x90a\x17\x96V[4\x80\x15a\x02\xF1W`\0\x80\xFD[Pa\x01&a\x03\x006`\x04a\x17\xE7V[a\x062V[4\x80\x15a\x03\x11W`\0\x80\xFD[Pa\x01&a\x03 6`\x04a\x18\x08V[a\x06\xD1V[4\x80\x15a\x031W`\0\x80\xFD[Pa\x01&a\x03@6`\x04a\x18*V[a\x08}V[a\x01&a\t\x14V[4\x80\x15a\x03YW`\0\x80\xFD[Pa\x01&a\x03h6`\x04a\x18\x08V[a\n?V[4\x80\x15a\x03yW`\0\x80\xFD[Pa\x01&a\x03\x886`\x04a\x171V[a\x0B\x92V[a\x01&a\x03\x9B6`\x04a\x171V[a\x0B\xFBV[a\x01&a\x03\xAE6`\x04a\x18CV[a\x0C\xBDV[a\x03\xBBa\r\xE5V[a\x03\xC4\x82a\x0E\xE9V[a\x03\xCE\x82\x82a\x0F=V[PPV[`\0a\x03\xDCa\x10{V[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[a\x04\ta\x10\xEAV[a\x04\x13`\0a\x11xV[V[a\x04A`@\x80Q`\x80\x81\x01\x90\x91R`\0\x80\x82R` \x82\x01\x90\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[3`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x04}Wa\x04}a\x16\x91V[`\x02\x81\x11\x15a\x04\x8EWa\x04\x8Ea\x16\x91V[\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPP\x90P\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x81\x15\x80\x15a\x04\xF6WP\x82[\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x05\x13WP0;\x15[\x90P\x81\x15\x80\x15a\x05!WP\x80\x15[\x15a\x05XW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16`\x01\x17\x85U\x83\x15a\x05\xB9W\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0\x17\x85U[a\x05\xC23a\x12\x0EV[a\x05\xCAa\x12\x1FV[\x83\x15a\x06+W\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[3`\0\x90\x81R` \x81\x90R`@\x81 T\x90\x03a\x06zW`@Q\x7F\xF8\"\x86Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x90\x81\x01\x80T\x83\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x90\x91\x16\x90\x83`\x02\x81\x11\x15a\x06\xC9Wa\x06\xC9a\x16\x91V[\x02\x17\x90UPPV[\x81`\0\x03a\x07\x0BW`@Q\x7Fia\x07\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x03a\x07EW`@Q\x7F\xC0*\x884\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x81 T\x90\x03a\x07\x8DW`@Q\x7F\xF8\"\x86Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x91\x92\x90\x91\x90\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x07\xCBWa\x07\xCBa\x16\x91V[`\x02\x81\x11\x15a\x07\xDCWa\x07\xDCa\x16\x91V[\x81R`\x02\x82\x81\x01T` \x80\x84\x01\x91\x90\x91R`\x03\x90\x93\x01T`@\x92\x83\x01R\x86\x84R``\x84\x01\x86\x90R3`\0\x90\x81R\x80\x84R\x91\x90\x91 \x83Q\x81U\x91\x83\x01Q`\x01\x80\x84\x01\x80T\x95\x96P\x86\x95\x92\x93\x90\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x90\x84\x90\x81\x11\x15a\x08]Wa\x08]a\x16\x91V[\x02\x17\x90UP`@\x82\x01Q`\x02\x82\x01U``\x90\x91\x01Q`\x03\x90\x91\x01UPPPV[\x80`\0\x03a\x08\xB7W`@Q\x7F\xC0*\x884\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x81 T\x90\x03a\x08\xFFW`@Q\x7F\xF8\"\x86Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x90 `\x03\x01UV[3`\0\x90\x81R` \x81\x90R`@\x81 T\x90\x03a\t\\W`@Q\x7F\xF8\"\x86Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x81 `\x02\x01T\x90\x03a\t\xA7W`@Q\x7Ff\x95g\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x81\x81R` \x81\x90R`@\x80\x82 `\x02\x01\x80T\x90\x83\x90U\x90Q\x90\x92\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\t\xFFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\x04V[``\x91P[PP\x90P\x80a\x03\xCEW`@Q\x7F\xDC\xF3]\xB0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x90 T\x15a\n\x86W`@Q\x7F\xB55\xC2\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\n\xC0W`@Q\x7Fia\x07\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x03a\n\xFAW`@Q\x7F\xC0*\x884\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x80\x81\x01\x90\x91R\x82\x81R` \x81\x01`\0\x81R`\0` \x80\x83\x01\x82\x90R`@\x92\x83\x01\x85\x90R3\x82R\x81\x81R\x91\x90 \x82Q\x81U\x90\x82\x01Q`\x01\x80\x83\x01\x80T\x90\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x90\x91\x16\x90\x83`\x02\x81\x11\x15a\x0BsWa\x0Bsa\x16\x91V[\x02\x17\x90UP`@\x82\x01Q`\x02\x82\x01U``\x90\x91\x01Q`\x03\x90\x91\x01UPPV[a\x0B\x9Aa\x10\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x0B\xEFW`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x0B\xF8\x81a\x11xV[PV[4`\0\x03a\x0C5W`@Q\x7F\xA0C\x9C\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x81 T\x90\x03a\x0C}W`@Q\x7F\xF8\"\x86Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x81 `\x02\x01\x80T4\x92\x90a\x0C\xB5\x90\x84\x90a\x18vV[\x90\x91UPPPV[4`\0\x03a\x0C\xF7W`@Q\x7F\xA0C\x9C\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x15\x80\x15a\rMWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x15[\x15a\r\x84W`@Q\x7F\xF8\"\x86Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\r\xA3WP\x80[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x81 `\x02\x01\x80T4\x92\x90a\r\xDB\x90\x84\x90a\x18vV[\x90\x91UPPPPPV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x0E\xB2WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0E\x99\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x04\x13W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xF1a\x10\xEAV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81R\x7F\xF7\x87!\"n\xFE\x9A\x1B\xB6x\x18\x9A\x16\xD1UI(\xB9\xF2\x19.,\xB9>\xED\xA8;y\xFA@\0}\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x0F\xC2WP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\x0F\xBF\x91\x81\x01\x90a\x18\xB0V[`\x01[a\x10\x10W`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x01a\x0B\xE6V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a\x10lW`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0B\xE6V[a\x10v\x83\x83a\x12'V[PPPV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\x13W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\x11)\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x04\x13W`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x01a\x0B\xE6V[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPPV[a\x12\x16a\x12\x8AV[a\x0B\xF8\x81a\x12\xF1V[a\x04\x13a\x12\x8AV[a\x120\x82a\x12\xF9V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\x12\x82Wa\x10v\x82\x82a\x13\xC8V[a\x03\xCEa\x14MV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x04\x13W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x9Aa\x12\x8AV[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;`\0\x03a\x13bW`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\x0B\xE6V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa\x13\xF2\x91\x90a\x18\xC9V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x14-W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x142V[``\x91P[P\x91P\x91Pa\x14B\x85\x83\x83a\x14\x85V[\x92PPP[\x92\x91PPV[4\x15a\x04\x13W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x14\x9AWa\x14\x95\x82a\x15\x17V[a\x15\x10V[\x81Q\x15\x80\x15a\x14\xBEWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;\x15[\x15a\x15\rW`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`$\x01a\x0B\xE6V[P\x80[\x93\x92PPPV[\x80Q\x15a\x15'W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x15}W`\0\x80\xFD[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x15\xC4W`\0\x80\xFD[a\x15\xCD\x83a\x15YV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15\xEAW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x15\xFEW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x16\x10Wa\x16\x10a\x15\x82V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x16VWa\x16Va\x15\x82V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x16oW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a\x16\xF7W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`\x80\x83\x01\x91a\x17\x16\x90\x84\x01\x82a\x16\xC0V[P`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x17CW`\0\x80\xFD[a\x15\x10\x82a\x15YV[\x84\x81R`\x80\x81\x01a\x17`` \x83\x01\x86a\x16\xC0V[`@\x82\x01\x93\x90\x93R``\x01R\x92\x91PPV[`\0[\x83\x81\x10\x15a\x17\x8DW\x81\x81\x01Q\x83\x82\x01R` \x01a\x17uV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x17\xB5\x81`@\x85\x01` \x87\x01a\x17rV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x17\xF9W`\0\x80\xFD[\x815`\x03\x81\x10a\x15\x10W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x18\x1BW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x18<W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x18VW`\0\x80\xFD[a\x18_\x83a\x15YV[\x91Pa\x18m` \x84\x01a\x15YV[\x90P\x92P\x92\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x14GW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x18\xC2W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82Qa\x18\xDB\x81\x84` \x87\x01a\x17rV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static DEMOBOXV2_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\x0EW`\x005`\xE0\x1C\x80c\xAD<\xB1\xCC\x11a\0\xA5W\x80c\xE0\x86\xE5\xEC\x11a\0tW\x80c\xF2\xFD\xE3\x8B\x11a\0YW\x80c\xF2\xFD\xE3\x8B\x14a\x03mW\x80c\xF3@\xFA\x01\x14a\x03\x8DW\x80c\xF9`\x9F\x08\x14a\x03\xA0W`\0\x80\xFD[\x80c\xE0\x86\xE5\xEC\x14a\x03EW\x80c\xF1\xA0\xD1\xC8\x14a\x03MW`\0\x80\xFD[\x80c\xAD<\xB1\xCC\x14a\x02\x8FW\x80c\xB7'O\xBE\x14a\x02\xE5W\x80c\xC4\xD5\xE2:\x14a\x03\x05W\x80c\xC4\xEB{\xA4\x14a\x03%W`\0\x80\xFD[\x80cq\xD8\xCA\x0B\x11a\0\xE1W\x80cq\xD8\xCA\x0B\x14a\x01\xAEW\x80c\x81)\xFC\x1C\x14a\x01\xD0W\x80c\x8D\xA5\xCB[\x14a\x01\xE5W\x80c\x9A\xA3\xBBE\x14a\x029W`\0\x80\xFD[\x80cO\x1E\xF2\x86\x14a\x01\x13W\x80cR\xD1\x90-\x14a\x01(W\x80cT\xFDMP\x14a\x01PW\x80cqP\x18\xA6\x14a\x01\x99W[`\0\x80\xFD[a\x01&a\x01!6`\x04a\x15\xB1V[a\x03\xB3V[\0[4\x80\x15a\x014W`\0\x80\xFD[Pa\x01=a\x03\xD2V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\\W`\0\x80\xFD[Pa\x01\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01GV[4\x80\x15a\x01\xA5W`\0\x80\xFD[Pa\x01&a\x04\x01V[4\x80\x15a\x01\xBAW`\0\x80\xFD[Pa\x01\xC3a\x04\x15V[`@Qa\x01G\x91\x90a\x16\xFBV[4\x80\x15a\x01\xDCW`\0\x80\xFD[Pa\x01&a\x04\xABV[4\x80\x15a\x01\xF1W`\0\x80\xFD[P\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01GV[4\x80\x15a\x02EW`\0\x80\xFD[Pa\x02\x7Fa\x02T6`\x04a\x171V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T\x91\x92`\xFF\x90\x91\x16\x91\x84V[`@Qa\x01G\x94\x93\x92\x91\x90a\x17LV[4\x80\x15a\x02\x9BW`\0\x80\xFD[Pa\x02\xD8`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x01G\x91\x90a\x17\x96V[4\x80\x15a\x02\xF1W`\0\x80\xFD[Pa\x01&a\x03\x006`\x04a\x17\xE7V[a\x062V[4\x80\x15a\x03\x11W`\0\x80\xFD[Pa\x01&a\x03 6`\x04a\x18\x08V[a\x06\xD1V[4\x80\x15a\x031W`\0\x80\xFD[Pa\x01&a\x03@6`\x04a\x18*V[a\x08}V[a\x01&a\t\x14V[4\x80\x15a\x03YW`\0\x80\xFD[Pa\x01&a\x03h6`\x04a\x18\x08V[a\n?V[4\x80\x15a\x03yW`\0\x80\xFD[Pa\x01&a\x03\x886`\x04a\x171V[a\x0B\x92V[a\x01&a\x03\x9B6`\x04a\x171V[a\x0B\xFBV[a\x01&a\x03\xAE6`\x04a\x18CV[a\x0C\xBDV[a\x03\xBBa\r\xE5V[a\x03\xC4\x82a\x0E\xE9V[a\x03\xCE\x82\x82a\x0F=V[PPV[`\0a\x03\xDCa\x10{V[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[a\x04\ta\x10\xEAV[a\x04\x13`\0a\x11xV[V[a\x04A`@\x80Q`\x80\x81\x01\x90\x91R`\0\x80\x82R` \x82\x01\x90\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[3`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x04}Wa\x04}a\x16\x91V[`\x02\x81\x11\x15a\x04\x8EWa\x04\x8Ea\x16\x91V[\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPP\x90P\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x81\x15\x80\x15a\x04\xF6WP\x82[\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x05\x13WP0;\x15[\x90P\x81\x15\x80\x15a\x05!WP\x80\x15[\x15a\x05XW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16`\x01\x17\x85U\x83\x15a\x05\xB9W\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0\x17\x85U[a\x05\xC23a\x12\x0EV[a\x05\xCAa\x12\x1FV[\x83\x15a\x06+W\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[3`\0\x90\x81R` \x81\x90R`@\x81 T\x90\x03a\x06zW`@Q\x7F\xF8\"\x86Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x90\x81\x01\x80T\x83\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x90\x91\x16\x90\x83`\x02\x81\x11\x15a\x06\xC9Wa\x06\xC9a\x16\x91V[\x02\x17\x90UPPV[\x81`\0\x03a\x07\x0BW`@Q\x7Fia\x07\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x03a\x07EW`@Q\x7F\xC0*\x884\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x81 T\x90\x03a\x07\x8DW`@Q\x7F\xF8\"\x86Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x91\x92\x90\x91\x90\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x07\xCBWa\x07\xCBa\x16\x91V[`\x02\x81\x11\x15a\x07\xDCWa\x07\xDCa\x16\x91V[\x81R`\x02\x82\x81\x01T` \x80\x84\x01\x91\x90\x91R`\x03\x90\x93\x01T`@\x92\x83\x01R\x86\x84R``\x84\x01\x86\x90R3`\0\x90\x81R\x80\x84R\x91\x90\x91 \x83Q\x81U\x91\x83\x01Q`\x01\x80\x84\x01\x80T\x95\x96P\x86\x95\x92\x93\x90\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x90\x84\x90\x81\x11\x15a\x08]Wa\x08]a\x16\x91V[\x02\x17\x90UP`@\x82\x01Q`\x02\x82\x01U``\x90\x91\x01Q`\x03\x90\x91\x01UPPPV[\x80`\0\x03a\x08\xB7W`@Q\x7F\xC0*\x884\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x81 T\x90\x03a\x08\xFFW`@Q\x7F\xF8\"\x86Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x90 `\x03\x01UV[3`\0\x90\x81R` \x81\x90R`@\x81 T\x90\x03a\t\\W`@Q\x7F\xF8\"\x86Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x81 `\x02\x01T\x90\x03a\t\xA7W`@Q\x7Ff\x95g\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x81\x81R` \x81\x90R`@\x80\x82 `\x02\x01\x80T\x90\x83\x90U\x90Q\x90\x92\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\t\xFFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\x04V[``\x91P[PP\x90P\x80a\x03\xCEW`@Q\x7F\xDC\xF3]\xB0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x90 T\x15a\n\x86W`@Q\x7F\xB55\xC2\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\n\xC0W`@Q\x7Fia\x07\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0\x03a\n\xFAW`@Q\x7F\xC0*\x884\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x80\x81\x01\x90\x91R\x82\x81R` \x81\x01`\0\x81R`\0` \x80\x83\x01\x82\x90R`@\x92\x83\x01\x85\x90R3\x82R\x81\x81R\x91\x90 \x82Q\x81U\x90\x82\x01Q`\x01\x80\x83\x01\x80T\x90\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x90\x91\x16\x90\x83`\x02\x81\x11\x15a\x0BsWa\x0Bsa\x16\x91V[\x02\x17\x90UP`@\x82\x01Q`\x02\x82\x01U``\x90\x91\x01Q`\x03\x90\x91\x01UPPV[a\x0B\x9Aa\x10\xEAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x0B\xEFW`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x0B\xF8\x81a\x11xV[PV[4`\0\x03a\x0C5W`@Q\x7F\xA0C\x9C\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R` \x81\x90R`@\x81 T\x90\x03a\x0C}W`@Q\x7F\xF8\"\x86Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x81 `\x02\x01\x80T4\x92\x90a\x0C\xB5\x90\x84\x90a\x18vV[\x90\x91UPPPV[4`\0\x03a\x0C\xF7W`@Q\x7F\xA0C\x9C\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x15\x80\x15a\rMWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x15[\x15a\r\x84W`@Q\x7F\xF8\"\x86Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\r\xA3WP\x80[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x81 `\x02\x01\x80T4\x92\x90a\r\xDB\x90\x84\x90a\x18vV[\x90\x91UPPPPPV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x0E\xB2WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0E\x99\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x04\x13W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xF1a\x10\xEAV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81R\x7F\xF7\x87!\"n\xFE\x9A\x1B\xB6x\x18\x9A\x16\xD1UI(\xB9\xF2\x19.,\xB9>\xED\xA8;y\xFA@\0}\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x0F\xC2WP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra\x0F\xBF\x91\x81\x01\x90a\x18\xB0V[`\x01[a\x10\x10W`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x01a\x0B\xE6V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a\x10lW`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0B\xE6V[a\x10v\x83\x83a\x12'V[PPPV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\x13W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\x11)\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x04\x13W`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x01a\x0B\xE6V[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPPV[a\x12\x16a\x12\x8AV[a\x0B\xF8\x81a\x12\xF1V[a\x04\x13a\x12\x8AV[a\x120\x82a\x12\xF9V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a\x12\x82Wa\x10v\x82\x82a\x13\xC8V[a\x03\xCEa\x14MV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x04\x13W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x9Aa\x12\x8AV[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;`\0\x03a\x13bW`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\x0B\xE6V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa\x13\xF2\x91\x90a\x18\xC9V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x14-W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x142V[``\x91P[P\x91P\x91Pa\x14B\x85\x83\x83a\x14\x85V[\x92PPP[\x92\x91PPV[4\x15a\x04\x13W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x14\x9AWa\x14\x95\x82a\x15\x17V[a\x15\x10V[\x81Q\x15\x80\x15a\x14\xBEWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;\x15[\x15a\x15\rW`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`$\x01a\x0B\xE6V[P\x80[\x93\x92PPPV[\x80Q\x15a\x15'W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\x14%\xEAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x15}W`\0\x80\xFD[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x15\xC4W`\0\x80\xFD[a\x15\xCD\x83a\x15YV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15\xEAW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x15\xFEW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x16\x10Wa\x16\x10a\x15\x82V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x16VWa\x16Va\x15\x82V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x16oW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a\x16\xF7W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`\x80\x83\x01\x91a\x17\x16\x90\x84\x01\x82a\x16\xC0V[P`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x17CW`\0\x80\xFD[a\x15\x10\x82a\x15YV[\x84\x81R`\x80\x81\x01a\x17`` \x83\x01\x86a\x16\xC0V[`@\x82\x01\x93\x90\x93R``\x01R\x92\x91PPV[`\0[\x83\x81\x10\x15a\x17\x8DW\x81\x81\x01Q\x83\x82\x01R` \x01a\x17uV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x17\xB5\x81`@\x85\x01` \x87\x01a\x17rV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x17\xF9W`\0\x80\xFD[\x815`\x03\x81\x10a\x15\x10W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x18\x1BW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x18<W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x18VW`\0\x80\xFD[a\x18_\x83a\x15YV[\x91Pa\x18m` \x84\x01a\x15YV[\x90P\x92P\x92\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x14GW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x18\xC2W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82Qa\x18\xDB\x81\x84` \x87\x01a\x17rV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static DEMOBOXV2_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct DemoBoxV2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DemoBoxV2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DemoBoxV2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DemoBoxV2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DemoBoxV2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DemoBoxV2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DemoBoxV2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                DEMOBOXV2_ABI.clone(),
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
                DEMOBOXV2_ABI.clone(),
                DEMOBOXV2_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `UPGRADE_INTERFACE_VERSION` (0xad3cb1cc) function
        pub fn upgrade_interface_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([173, 60, 177, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addBox` (0xf1a0d1c8) function
        pub fn add_box(
            &self,
            size: ::ethers::core::types::U256,
            max_items: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 160, 209, 200], (size, max_items))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `boxes` (0x9aa3bb45) function
        pub fn boxes(
            &self,
            box_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                u8,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([154, 163, 187, 69], box_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xf340fa01) function
        pub fn deposit(
            &self,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 64, 250, 1], receiver)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xf9609f08) function
        pub fn deposit_with_receiver_2(
            &self,
            receiver: ::ethers::core::types::Address,
            receiver_2: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 96, 159, 8], (receiver, receiver_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBox` (0x71d8ca0b) function
        pub fn get_box(&self) -> ::ethers::contract::builders::ContractCall<M, Box> {
            self.0
                .method_hash([113, 216, 202, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
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
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
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
        ///Calls the contract's `updateBox` (0xc4d5e23a) function
        pub fn update_box(
            &self,
            new_size: ::ethers::core::types::U256,
            max_items: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 213, 226, 58], (new_size, max_items))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateBoxCapacity` (0xc4eb7ba4) function
        pub fn update_box_capacity(
            &self,
            max_items: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 235, 123, 164], max_items)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateBoxStatus` (0xb7274fbe) function
        pub fn update_box_status(
            &self,
            status: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 39, 79, 190], status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeToAndCall` (0x4f1ef286) function
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawETH` (0xe086e5ec) function
        pub fn withdraw_eth(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 134, 229, 236], ())
                .expect("method not found (this should never happen)")
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
        ///Gets the contract's `Upgrade` event
        pub fn upgrade_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradeFilter> {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DemoBoxV2Events> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for DemoBoxV2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
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
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `BoxAlreadyExists` with signature `BoxAlreadyExists()` and selector `0xb535c218`
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
    #[etherror(name = "BoxAlreadyExists", abi = "BoxAlreadyExists()")]
    pub struct BoxAlreadyExists;
    ///Custom Error type `BoxCapcityTooSmall` with signature `BoxCapcityTooSmall()` and selector `0xc02a8834`
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
    #[etherror(name = "BoxCapcityTooSmall", abi = "BoxCapcityTooSmall()")]
    pub struct BoxCapcityTooSmall;
    ///Custom Error type `BoxSizeTooSmall` with signature `BoxSizeTooSmall()` and selector `0x69610713`
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
    #[etherror(name = "BoxSizeTooSmall", abi = "BoxSizeTooSmall()")]
    pub struct BoxSizeTooSmall;
    ///Custom Error type `ERC1967InvalidImplementation` with signature `ERC1967InvalidImplementation(address)` and selector `0x4c9c8ce3`
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
        name = "ERC1967InvalidImplementation",
        abi = "ERC1967InvalidImplementation(address)"
    )]
    pub struct ERC1967InvalidImplementation {
        pub implementation: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1967NonPayable` with signature `ERC1967NonPayable()` and selector `0xb398979f`
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
    #[etherror(name = "ERC1967NonPayable", abi = "ERC1967NonPayable()")]
    pub struct ERC1967NonPayable;
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
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
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
    ///Custom Error type `FailedToSendEther` with signature `FailedToSendEther()` and selector `0xdcf35db0`
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
    #[etherror(name = "FailedToSendEther", abi = "FailedToSendEther()")]
    pub struct FailedToSendEther;
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
    ///Custom Error type `NoBoxExists` with signature `NoBoxExists()` and selector `0xf8228659`
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
    #[etherror(name = "NoBoxExists", abi = "NoBoxExists()")]
    pub struct NoBoxExists;
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
    ///Custom Error type `UUPSUnauthorizedCallContext` with signature `UUPSUnauthorizedCallContext()` and selector `0xe07c8dba`
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
        name = "UUPSUnauthorizedCallContext",
        abi = "UUPSUnauthorizedCallContext()"
    )]
    pub struct UUPSUnauthorizedCallContext;
    ///Custom Error type `UUPSUnsupportedProxiableUUID` with signature `UUPSUnsupportedProxiableUUID(bytes32)` and selector `0xaa1d49a4`
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
        name = "UUPSUnsupportedProxiableUUID",
        abi = "UUPSUnsupportedProxiableUUID(bytes32)"
    )]
    pub struct UUPSUnsupportedProxiableUUID {
        pub slot: [u8; 32],
    }
    ///Custom Error type `YouMustDepositETH` with signature `YouMustDepositETH()` and selector `0xa0439c1f`
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
    #[etherror(name = "YouMustDepositETH", abi = "YouMustDepositETH()")]
    pub struct YouMustDepositETH;
    ///Custom Error type `ZeroBalance` with signature `ZeroBalance()` and selector `0x669567ea`
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
    #[etherror(name = "ZeroBalance", abi = "ZeroBalance()")]
    pub struct ZeroBalance;
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
    pub enum DemoBoxV2Errors {
        AddressEmptyCode(AddressEmptyCode),
        BoxAlreadyExists(BoxAlreadyExists),
        BoxCapcityTooSmall(BoxCapcityTooSmall),
        BoxSizeTooSmall(BoxSizeTooSmall),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedInnerCall(FailedInnerCall),
        FailedToSendEther(FailedToSendEther),
        InvalidInitialization(InvalidInitialization),
        NoBoxExists(NoBoxExists),
        NotInitializing(NotInitializing),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        YouMustDepositETH(YouMustDepositETH),
        ZeroBalance(ZeroBalance),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DemoBoxV2Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) = <BoxAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BoxAlreadyExists(decoded));
            }
            if let Ok(decoded) =
                <BoxCapcityTooSmall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BoxCapcityTooSmall(decoded));
            }
            if let Ok(decoded) = <BoxSizeTooSmall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BoxSizeTooSmall(decoded));
            }
            if let Ok(decoded) =
                <ERC1967InvalidImplementation as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1967InvalidImplementation(decoded));
            }
            if let Ok(decoded) = <ERC1967NonPayable as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1967NonPayable(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <FailedToSendEther as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FailedToSendEther(decoded));
            }
            if let Ok(decoded) =
                <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) = <NoBoxExists as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoBoxExists(decoded));
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
            if let Ok(decoded) =
                <UUPSUnauthorizedCallContext as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UUPSUnauthorizedCallContext(decoded));
            }
            if let Ok(decoded) =
                <UUPSUnsupportedProxiableUUID as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UUPSUnsupportedProxiableUUID(decoded));
            }
            if let Ok(decoded) = <YouMustDepositETH as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::YouMustDepositETH(decoded));
            }
            if let Ok(decoded) = <ZeroBalance as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroBalance(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DemoBoxV2Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BoxAlreadyExists(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BoxCapcityTooSmall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BoxSizeTooSmall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ERC1967InvalidImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967NonPayable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedInnerCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedToSendEther(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoBoxExists(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotInitializing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UUPSUnauthorizedCallContext(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::YouMustDepositETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DemoBoxV2Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <BoxAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BoxCapcityTooSmall as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <BoxSizeTooSmall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967InvalidImplementation as ::ethers::contract::EthError>::selector(
                    ) =>
                {
                    true
                }
                _ if selector
                    == <ERC1967NonPayable as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedToSendEther as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <NoBoxExists as ::ethers::contract::EthError>::selector() => true,
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
                _ if selector
                    == <UUPSUnauthorizedCallContext as ::ethers::contract::EthError>::selector(
                    ) =>
                {
                    true
                }
                _ if selector
                    == <UUPSUnsupportedProxiableUUID as ::ethers::contract::EthError>::selector(
                    ) =>
                {
                    true
                }
                _ if selector
                    == <YouMustDepositETH as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <ZeroBalance as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DemoBoxV2Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::BoxAlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::BoxCapcityTooSmall(element) => ::core::fmt::Display::fmt(element, f),
                Self::BoxSizeTooSmall(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1967InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967NonPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedToSendEther(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInitialization(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoBoxExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotInitializing(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableInvalidOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableUnauthorizedAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::UUPSUnauthorizedCallContext(element) => ::core::fmt::Display::fmt(element, f),
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::YouMustDepositETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DemoBoxV2Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for DemoBoxV2Errors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<BoxAlreadyExists> for DemoBoxV2Errors {
        fn from(value: BoxAlreadyExists) -> Self {
            Self::BoxAlreadyExists(value)
        }
    }
    impl ::core::convert::From<BoxCapcityTooSmall> for DemoBoxV2Errors {
        fn from(value: BoxCapcityTooSmall) -> Self {
            Self::BoxCapcityTooSmall(value)
        }
    }
    impl ::core::convert::From<BoxSizeTooSmall> for DemoBoxV2Errors {
        fn from(value: BoxSizeTooSmall) -> Self {
            Self::BoxSizeTooSmall(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation> for DemoBoxV2Errors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for DemoBoxV2Errors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for DemoBoxV2Errors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<FailedToSendEther> for DemoBoxV2Errors {
        fn from(value: FailedToSendEther) -> Self {
            Self::FailedToSendEther(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for DemoBoxV2Errors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NoBoxExists> for DemoBoxV2Errors {
        fn from(value: NoBoxExists) -> Self {
            Self::NoBoxExists(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for DemoBoxV2Errors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for DemoBoxV2Errors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for DemoBoxV2Errors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext> for DemoBoxV2Errors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID> for DemoBoxV2Errors {
        fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
            Self::UUPSUnsupportedProxiableUUID(value)
        }
    }
    impl ::core::convert::From<YouMustDepositETH> for DemoBoxV2Errors {
        fn from(value: YouMustDepositETH) -> Self {
            Self::YouMustDepositETH(value)
        }
    }
    impl ::core::convert::From<ZeroBalance> for DemoBoxV2Errors {
        fn from(value: ZeroBalance) -> Self {
            Self::ZeroBalance(value)
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
    #[ethevent(name = "Upgrade", abi = "Upgrade(address)")]
    pub struct UpgradeFilter {
        pub implementation: ::ethers::core::types::Address,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
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
    pub enum DemoBoxV2Events {
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        UpgradeFilter(UpgradeFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for DemoBoxV2Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(DemoBoxV2Events::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(DemoBoxV2Events::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = UpgradeFilter::decode_log(log) {
                return Ok(DemoBoxV2Events::UpgradeFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(DemoBoxV2Events::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DemoBoxV2Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for DemoBoxV2Events {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for DemoBoxV2Events {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<UpgradeFilter> for DemoBoxV2Events {
        fn from(value: UpgradeFilter) -> Self {
            Self::UpgradeFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for DemoBoxV2Events {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `UPGRADE_INTERFACE_VERSION` function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`
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
        name = "UPGRADE_INTERFACE_VERSION",
        abi = "UPGRADE_INTERFACE_VERSION()"
    )]
    pub struct UpgradeInterfaceVersionCall;
    ///Container type for all input parameters for the `addBox` function with signature `addBox(uint256,uint256)` and selector `0xf1a0d1c8`
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
    #[ethcall(name = "addBox", abi = "addBox(uint256,uint256)")]
    pub struct AddBoxCall {
        pub size: ::ethers::core::types::U256,
        pub max_items: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `boxes` function with signature `boxes(address)` and selector `0x9aa3bb45`
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
    #[ethcall(name = "boxes", abi = "boxes(address)")]
    pub struct BoxesCall {
        pub box_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(address)` and selector `0xf340fa01`
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
    #[ethcall(name = "deposit", abi = "deposit(address)")]
    pub struct DepositCall {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(address,address)` and selector `0xf9609f08`
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
    #[ethcall(name = "deposit", abi = "deposit(address,address)")]
    pub struct DepositWithReceiver2Call {
        pub receiver: ::ethers::core::types::Address,
        pub receiver_2: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getBox` function with signature `getBox()` and selector `0x71d8ca0b`
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
    #[ethcall(name = "getBox", abi = "getBox()")]
    pub struct GetBoxCall;
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
    ///Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
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
    ///Container type for all input parameters for the `updateBox` function with signature `updateBox(uint256,uint256)` and selector `0xc4d5e23a`
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
    #[ethcall(name = "updateBox", abi = "updateBox(uint256,uint256)")]
    pub struct UpdateBoxCall {
        pub new_size: ::ethers::core::types::U256,
        pub max_items: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateBoxCapacity` function with signature `updateBoxCapacity(uint256)` and selector `0xc4eb7ba4`
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
    #[ethcall(name = "updateBoxCapacity", abi = "updateBoxCapacity(uint256)")]
    pub struct UpdateBoxCapacityCall {
        pub max_items: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateBoxStatus` function with signature `updateBoxStatus(uint8)` and selector `0xb7274fbe`
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
    #[ethcall(name = "updateBoxStatus", abi = "updateBoxStatus(uint8)")]
    pub struct UpdateBoxStatusCall {
        pub status: u8,
    }
    ///Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`
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
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
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
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `withdrawETH` function with signature `withdrawETH()` and selector `0xe086e5ec`
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
    #[ethcall(name = "withdrawETH", abi = "withdrawETH()")]
    pub struct WithdrawETHCall;
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
    pub enum DemoBoxV2Calls {
        UpgradeInterfaceVersion(UpgradeInterfaceVersionCall),
        AddBox(AddBoxCall),
        Boxes(BoxesCall),
        Deposit(DepositCall),
        DepositWithReceiver2(DepositWithReceiver2Call),
        GetBox(GetBoxCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        ProxiableUUID(ProxiableUUIDCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateBox(UpdateBoxCall),
        UpdateBoxCapacity(UpdateBoxCapacityCall),
        UpdateBoxStatus(UpdateBoxStatusCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        Version(VersionCall),
        WithdrawETH(WithdrawETHCall),
    }
    impl ::ethers::core::abi::AbiDecode for DemoBoxV2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <UpgradeInterfaceVersionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeInterfaceVersion(decoded));
            }
            if let Ok(decoded) = <AddBoxCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddBox(decoded));
            }
            if let Ok(decoded) = <BoxesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Boxes(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DepositWithReceiver2Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DepositWithReceiver2(decoded));
            }
            if let Ok(decoded) = <GetBoxCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBox(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProxiableUUID(decoded));
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
            if let Ok(decoded) = <UpdateBoxCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateBox(decoded));
            }
            if let Ok(decoded) =
                <UpdateBoxCapacityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateBoxCapacity(decoded));
            }
            if let Ok(decoded) =
                <UpdateBoxStatusCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateBoxStatus(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded) = <WithdrawETHCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawETH(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DemoBoxV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::UpgradeInterfaceVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddBox(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Boxes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositWithReceiver2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBox(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateBox(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateBoxCapacity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateBoxStatus(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeToAndCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DemoBoxV2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::UpgradeInterfaceVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddBox(element) => ::core::fmt::Display::fmt(element, f),
                Self::Boxes(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositWithReceiver2(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBox(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateBox(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateBoxCapacity(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateBoxStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawETH(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall> for DemoBoxV2Calls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<AddBoxCall> for DemoBoxV2Calls {
        fn from(value: AddBoxCall) -> Self {
            Self::AddBox(value)
        }
    }
    impl ::core::convert::From<BoxesCall> for DemoBoxV2Calls {
        fn from(value: BoxesCall) -> Self {
            Self::Boxes(value)
        }
    }
    impl ::core::convert::From<DepositCall> for DemoBoxV2Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositWithReceiver2Call> for DemoBoxV2Calls {
        fn from(value: DepositWithReceiver2Call) -> Self {
            Self::DepositWithReceiver2(value)
        }
    }
    impl ::core::convert::From<GetBoxCall> for DemoBoxV2Calls {
        fn from(value: GetBoxCall) -> Self {
            Self::GetBox(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for DemoBoxV2Calls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for DemoBoxV2Calls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for DemoBoxV2Calls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for DemoBoxV2Calls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for DemoBoxV2Calls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateBoxCall> for DemoBoxV2Calls {
        fn from(value: UpdateBoxCall) -> Self {
            Self::UpdateBox(value)
        }
    }
    impl ::core::convert::From<UpdateBoxCapacityCall> for DemoBoxV2Calls {
        fn from(value: UpdateBoxCapacityCall) -> Self {
            Self::UpdateBoxCapacity(value)
        }
    }
    impl ::core::convert::From<UpdateBoxStatusCall> for DemoBoxV2Calls {
        fn from(value: UpdateBoxStatusCall) -> Self {
            Self::UpdateBoxStatus(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for DemoBoxV2Calls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<VersionCall> for DemoBoxV2Calls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<WithdrawETHCall> for DemoBoxV2Calls {
        fn from(value: WithdrawETHCall) -> Self {
            Self::WithdrawETH(value)
        }
    }
    ///Container type for all return fields from the `UPGRADE_INTERFACE_VERSION` function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`
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
    pub struct UpgradeInterfaceVersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `boxes` function with signature `boxes(address)` and selector `0x9aa3bb45`
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
    pub struct BoxesReturn {
        pub size: ::ethers::core::types::U256,
        pub status: u8,
        pub balance: ::ethers::core::types::U256,
        pub max_items: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getBox` function with signature `getBox()` and selector `0x71d8ca0b`
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
    pub struct GetBoxReturn(pub Box);
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
    ///Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
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
    pub struct VersionReturn(pub u32);
    ///`Box(uint256,uint8,uint256,uint256)`
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
    pub struct Box {
        pub size: ::ethers::core::types::U256,
        pub status: u8,
        pub balance: ::ethers::core::types::U256,
        pub max_items: ::ethers::core::types::U256,
    }
}
