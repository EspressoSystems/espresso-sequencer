pub use ierc1155_errors::*;
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
pub mod ierc1155_errors {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InsufficientBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InsufficientBalance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
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
                                name: ::std::borrow::ToOwned::to_owned("needed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidApprover"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidApprover",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("approver"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidArrayLength"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidArrayLength",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("idsLength"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("valuesLength"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidOperator",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidReceiver"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidReceiver",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("receiver"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidSender"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidSender",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("sender"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155MissingApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155MissingApprovalForAll",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IERC1155ERRORS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IERC1155Errors<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC1155Errors<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC1155Errors<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC1155Errors<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC1155Errors<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IERC1155Errors))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IERC1155Errors<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IERC1155ERRORS_ABI.clone(),
                client,
            ))
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IERC1155Errors<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ERC1155InsufficientBalance` with signature `ERC1155InsufficientBalance(address,uint256,uint256,uint256)` and selector `0x03dee4c5`
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
        name = "ERC1155InsufficientBalance",
        abi = "ERC1155InsufficientBalance(address,uint256,uint256,uint256)"
    )]
    pub struct ERC1155InsufficientBalance {
        pub sender: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC1155InvalidApprover` with signature `ERC1155InvalidApprover(address)` and selector `0x3e31884e`
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
        name = "ERC1155InvalidApprover",
        abi = "ERC1155InvalidApprover(address)"
    )]
    pub struct ERC1155InvalidApprover {
        pub approver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1155InvalidArrayLength` with signature `ERC1155InvalidArrayLength(uint256,uint256)` and selector `0x5b059991`
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
        name = "ERC1155InvalidArrayLength",
        abi = "ERC1155InvalidArrayLength(uint256,uint256)"
    )]
    pub struct ERC1155InvalidArrayLength {
        pub ids_length: ::ethers::core::types::U256,
        pub values_length: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC1155InvalidOperator` with signature `ERC1155InvalidOperator(address)` and selector `0xced3e100`
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
        name = "ERC1155InvalidOperator",
        abi = "ERC1155InvalidOperator(address)"
    )]
    pub struct ERC1155InvalidOperator {
        pub operator: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1155InvalidReceiver` with signature `ERC1155InvalidReceiver(address)` and selector `0x57f447ce`
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
        name = "ERC1155InvalidReceiver",
        abi = "ERC1155InvalidReceiver(address)"
    )]
    pub struct ERC1155InvalidReceiver {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1155InvalidSender` with signature `ERC1155InvalidSender(address)` and selector `0x01a83514`
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
    #[etherror(name = "ERC1155InvalidSender", abi = "ERC1155InvalidSender(address)")]
    pub struct ERC1155InvalidSender {
        pub sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1155MissingApprovalForAll` with signature `ERC1155MissingApprovalForAll(address,address)` and selector `0xe237d922`
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
        name = "ERC1155MissingApprovalForAll",
        abi = "ERC1155MissingApprovalForAll(address,address)"
    )]
    pub struct ERC1155MissingApprovalForAll {
        pub operator: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
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
    pub enum IERC1155ErrorsErrors {
        ERC1155InsufficientBalance(ERC1155InsufficientBalance),
        ERC1155InvalidApprover(ERC1155InvalidApprover),
        ERC1155InvalidArrayLength(ERC1155InvalidArrayLength),
        ERC1155InvalidOperator(ERC1155InvalidOperator),
        ERC1155InvalidReceiver(ERC1155InvalidReceiver),
        ERC1155InvalidSender(ERC1155InvalidSender),
        ERC1155MissingApprovalForAll(ERC1155MissingApprovalForAll),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IERC1155ErrorsErrors {
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
                <ERC1155InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InsufficientBalance(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidApprover as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidApprover(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidArrayLength as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidArrayLength(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidOperator as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidOperator(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidReceiver as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidReceiver(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidSender as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidSender(decoded));
            }
            if let Ok(decoded) =
                <ERC1155MissingApprovalForAll as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155MissingApprovalForAll(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IERC1155ErrorsErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ERC1155InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidArrayLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155MissingApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IERC1155ErrorsErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC1155InsufficientBalance as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidApprover as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidArrayLength as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidOperator as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidReceiver as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidSender as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155MissingApprovalForAll as ::ethers::contract::EthError>::selector(
                    ) =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IERC1155ErrorsErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC1155InsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidApprover(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidArrayLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidReceiver(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155MissingApprovalForAll(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IERC1155ErrorsErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC1155InsufficientBalance> for IERC1155ErrorsErrors {
        fn from(value: ERC1155InsufficientBalance) -> Self {
            Self::ERC1155InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidApprover> for IERC1155ErrorsErrors {
        fn from(value: ERC1155InvalidApprover) -> Self {
            Self::ERC1155InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidArrayLength> for IERC1155ErrorsErrors {
        fn from(value: ERC1155InvalidArrayLength) -> Self {
            Self::ERC1155InvalidArrayLength(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidOperator> for IERC1155ErrorsErrors {
        fn from(value: ERC1155InvalidOperator) -> Self {
            Self::ERC1155InvalidOperator(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidReceiver> for IERC1155ErrorsErrors {
        fn from(value: ERC1155InvalidReceiver) -> Self {
            Self::ERC1155InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidSender> for IERC1155ErrorsErrors {
        fn from(value: ERC1155InvalidSender) -> Self {
            Self::ERC1155InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC1155MissingApprovalForAll> for IERC1155ErrorsErrors {
        fn from(value: ERC1155MissingApprovalForAll) -> Self {
            Self::ERC1155MissingApprovalForAll(value)
        }
    }
}
