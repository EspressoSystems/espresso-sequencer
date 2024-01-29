pub use std_error::*;
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
pub mod std_error {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("arithmeticError"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("arithmeticError"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assertionError"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assertionError"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("divisionError"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("divisionError"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeStorageError"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("encodeStorageError"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enumConversionError"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("enumConversionError",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("indexOOBError"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("indexOOBError"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("memOverflowError"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("memOverflowError"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("popError"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("popError"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("zeroVarError"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("zeroVarError"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
    pub static STDERROR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x02'a\0:`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14a\0-WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0\x9DW`\x005`\xE0\x1C\x80c\x98l_h\x11a\0pW\x80c\x98l_h\x14a\0\xD8W\x80c\xB2-\xC5M\x14a\0\xE0W\x80c\xB6v\x89\xDA\x14a\0\xE8W\x80c\xD1`\xE4\xDE\x14a\0\xF0W\x80c\xFAxJD\x14a\0\xF8W`\0\x80\xFD[\x80c\x05\xEE\x86\x12\x14a\0\xA2W\x80c\x103)w\x14a\0\xC0W\x80c\x1D\xE4U`\x14a\0\xC8W\x80c\x89\x95)\x0F\x14a\0\xD0W[`\0\x80\xFD[a\0\xAAa\x01\0V[`@Qa\0\xB7\x91\x90a\x01\xCBV[`@Q\x80\x91\x03\x90\xF3[a\0\xAAa\x01;V[a\0\xAAa\x01MV[a\0\xAAa\x01_V[a\0\xAAa\x01qV[a\0\xAAa\x01\x83V[a\0\xAAa\x01\x95V[a\0\xAAa\x01\xA7V[a\0\xAAa\x01\xB9V[`@Q`2`$\x82\x01R`D\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cNH{q`\xE0\x1B\x17\x90R\x81V[`@Q`\x01`$\x82\x01R`D\x01a\x01\x0EV[`@Q`!`$\x82\x01R`D\x01a\x01\x0EV[`@Q`\x11`$\x82\x01R`D\x01a\x01\x0EV[`@Q`A`$\x82\x01R`D\x01a\x01\x0EV[`@Q`1`$\x82\x01R`D\x01a\x01\x0EV[`@Q`Q`$\x82\x01R`D\x01a\x01\x0EV[`@Q`\"`$\x82\x01R`D\x01a\x01\x0EV[`@Q`\x12`$\x82\x01R`D\x01a\x01\x0EV[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x01\xF9W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x01\xDDV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static STDERROR_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0\x9DW`\x005`\xE0\x1C\x80c\x98l_h\x11a\0pW\x80c\x98l_h\x14a\0\xD8W\x80c\xB2-\xC5M\x14a\0\xE0W\x80c\xB6v\x89\xDA\x14a\0\xE8W\x80c\xD1`\xE4\xDE\x14a\0\xF0W\x80c\xFAxJD\x14a\0\xF8W`\0\x80\xFD[\x80c\x05\xEE\x86\x12\x14a\0\xA2W\x80c\x103)w\x14a\0\xC0W\x80c\x1D\xE4U`\x14a\0\xC8W\x80c\x89\x95)\x0F\x14a\0\xD0W[`\0\x80\xFD[a\0\xAAa\x01\0V[`@Qa\0\xB7\x91\x90a\x01\xCBV[`@Q\x80\x91\x03\x90\xF3[a\0\xAAa\x01;V[a\0\xAAa\x01MV[a\0\xAAa\x01_V[a\0\xAAa\x01qV[a\0\xAAa\x01\x83V[a\0\xAAa\x01\x95V[a\0\xAAa\x01\xA7V[a\0\xAAa\x01\xB9V[`@Q`2`$\x82\x01R`D\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cNH{q`\xE0\x1B\x17\x90R\x81V[`@Q`\x01`$\x82\x01R`D\x01a\x01\x0EV[`@Q`!`$\x82\x01R`D\x01a\x01\x0EV[`@Q`\x11`$\x82\x01R`D\x01a\x01\x0EV[`@Q`A`$\x82\x01R`D\x01a\x01\x0EV[`@Q`1`$\x82\x01R`D\x01a\x01\x0EV[`@Q`Q`$\x82\x01R`D\x01a\x01\x0EV[`@Q`\"`$\x82\x01R`D\x01a\x01\x0EV[`@Q`\x12`$\x82\x01R`D\x01a\x01\x0EV[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\x01\xF9W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x01\xDDV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static STDERROR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct stdError<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for stdError<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for stdError<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for stdError<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for stdError<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(stdError))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> stdError<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                STDERROR_ABI.clone(),
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
                STDERROR_ABI.clone(),
                STDERROR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `arithmeticError` (0x8995290f) function
        pub fn arithmetic_error(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([137, 149, 41, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assertionError` (0x10332977) function
        pub fn assertion_error(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([16, 51, 41, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `divisionError` (0xfa784a44) function
        pub fn division_error(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([250, 120, 74, 68], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeStorageError` (0xd160e4de) function
        pub fn encode_storage_error(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([209, 96, 228, 222], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enumConversionError` (0x1de45560) function
        pub fn enum_conversion_error(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([29, 228, 85, 96], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `indexOOBError` (0x05ee8612) function
        pub fn index_oob_error(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([5, 238, 134, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `memOverflowError` (0x986c5f68) function
        pub fn mem_overflow_error(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([152, 108, 95, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `popError` (0xb22dc54d) function
        pub fn pop_error(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([178, 45, 197, 77], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `zeroVarError` (0xb67689da) function
        pub fn zero_var_error(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([182, 118, 137, 218], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for stdError<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `arithmeticError` function with signature `arithmeticError()` and selector `0x8995290f`
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
    #[ethcall(name = "arithmeticError", abi = "arithmeticError()")]
    pub struct ArithmeticErrorCall;
    ///Container type for all input parameters for the `assertionError` function with signature `assertionError()` and selector `0x10332977`
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
    #[ethcall(name = "assertionError", abi = "assertionError()")]
    pub struct AssertionErrorCall;
    ///Container type for all input parameters for the `divisionError` function with signature `divisionError()` and selector `0xfa784a44`
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
    #[ethcall(name = "divisionError", abi = "divisionError()")]
    pub struct DivisionErrorCall;
    ///Container type for all input parameters for the `encodeStorageError` function with signature `encodeStorageError()` and selector `0xd160e4de`
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
    #[ethcall(name = "encodeStorageError", abi = "encodeStorageError()")]
    pub struct EncodeStorageErrorCall;
    ///Container type for all input parameters for the `enumConversionError` function with signature `enumConversionError()` and selector `0x1de45560`
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
    #[ethcall(name = "enumConversionError", abi = "enumConversionError()")]
    pub struct EnumConversionErrorCall;
    ///Container type for all input parameters for the `indexOOBError` function with signature `indexOOBError()` and selector `0x05ee8612`
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
    #[ethcall(name = "indexOOBError", abi = "indexOOBError()")]
    pub struct IndexOOBErrorCall;
    ///Container type for all input parameters for the `memOverflowError` function with signature `memOverflowError()` and selector `0x986c5f68`
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
    #[ethcall(name = "memOverflowError", abi = "memOverflowError()")]
    pub struct MemOverflowErrorCall;
    ///Container type for all input parameters for the `popError` function with signature `popError()` and selector `0xb22dc54d`
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
    #[ethcall(name = "popError", abi = "popError()")]
    pub struct PopErrorCall;
    ///Container type for all input parameters for the `zeroVarError` function with signature `zeroVarError()` and selector `0xb67689da`
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
    #[ethcall(name = "zeroVarError", abi = "zeroVarError()")]
    pub struct ZeroVarErrorCall;
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
    pub enum stdErrorCalls {
        ArithmeticError(ArithmeticErrorCall),
        AssertionError(AssertionErrorCall),
        DivisionError(DivisionErrorCall),
        EncodeStorageError(EncodeStorageErrorCall),
        EnumConversionError(EnumConversionErrorCall),
        IndexOOBError(IndexOOBErrorCall),
        MemOverflowError(MemOverflowErrorCall),
        PopError(PopErrorCall),
        ZeroVarError(ZeroVarErrorCall),
    }
    impl ::ethers::core::abi::AbiDecode for stdErrorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ArithmeticErrorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ArithmeticError(decoded));
            }
            if let Ok(decoded) =
                <AssertionErrorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AssertionError(decoded));
            }
            if let Ok(decoded) = <DivisionErrorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DivisionError(decoded));
            }
            if let Ok(decoded) =
                <EncodeStorageErrorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EncodeStorageError(decoded));
            }
            if let Ok(decoded) =
                <EnumConversionErrorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EnumConversionError(decoded));
            }
            if let Ok(decoded) = <IndexOOBErrorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IndexOOBError(decoded));
            }
            if let Ok(decoded) =
                <MemOverflowErrorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MemOverflowError(decoded));
            }
            if let Ok(decoded) = <PopErrorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PopError(decoded));
            }
            if let Ok(decoded) = <ZeroVarErrorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ZeroVarError(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for stdErrorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ArithmeticError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssertionError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DivisionError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EncodeStorageError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnumConversionError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IndexOOBError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MemOverflowError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PopError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroVarError(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for stdErrorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ArithmeticError(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssertionError(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivisionError(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeStorageError(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnumConversionError(element) => ::core::fmt::Display::fmt(element, f),
                Self::IndexOOBError(element) => ::core::fmt::Display::fmt(element, f),
                Self::MemOverflowError(element) => ::core::fmt::Display::fmt(element, f),
                Self::PopError(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroVarError(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ArithmeticErrorCall> for stdErrorCalls {
        fn from(value: ArithmeticErrorCall) -> Self {
            Self::ArithmeticError(value)
        }
    }
    impl ::core::convert::From<AssertionErrorCall> for stdErrorCalls {
        fn from(value: AssertionErrorCall) -> Self {
            Self::AssertionError(value)
        }
    }
    impl ::core::convert::From<DivisionErrorCall> for stdErrorCalls {
        fn from(value: DivisionErrorCall) -> Self {
            Self::DivisionError(value)
        }
    }
    impl ::core::convert::From<EncodeStorageErrorCall> for stdErrorCalls {
        fn from(value: EncodeStorageErrorCall) -> Self {
            Self::EncodeStorageError(value)
        }
    }
    impl ::core::convert::From<EnumConversionErrorCall> for stdErrorCalls {
        fn from(value: EnumConversionErrorCall) -> Self {
            Self::EnumConversionError(value)
        }
    }
    impl ::core::convert::From<IndexOOBErrorCall> for stdErrorCalls {
        fn from(value: IndexOOBErrorCall) -> Self {
            Self::IndexOOBError(value)
        }
    }
    impl ::core::convert::From<MemOverflowErrorCall> for stdErrorCalls {
        fn from(value: MemOverflowErrorCall) -> Self {
            Self::MemOverflowError(value)
        }
    }
    impl ::core::convert::From<PopErrorCall> for stdErrorCalls {
        fn from(value: PopErrorCall) -> Self {
            Self::PopError(value)
        }
    }
    impl ::core::convert::From<ZeroVarErrorCall> for stdErrorCalls {
        fn from(value: ZeroVarErrorCall) -> Self {
            Self::ZeroVarError(value)
        }
    }
    ///Container type for all return fields from the `arithmeticError` function with signature `arithmeticError()` and selector `0x8995290f`
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
    pub struct ArithmeticErrorReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `assertionError` function with signature `assertionError()` and selector `0x10332977`
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
    pub struct AssertionErrorReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `divisionError` function with signature `divisionError()` and selector `0xfa784a44`
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
    pub struct DivisionErrorReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `encodeStorageError` function with signature `encodeStorageError()` and selector `0xd160e4de`
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
    pub struct EncodeStorageErrorReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `enumConversionError` function with signature `enumConversionError()` and selector `0x1de45560`
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
    pub struct EnumConversionErrorReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `indexOOBError` function with signature `indexOOBError()` and selector `0x05ee8612`
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
    pub struct IndexOOBErrorReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `memOverflowError` function with signature `memOverflowError()` and selector `0x986c5f68`
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
    pub struct MemOverflowErrorReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `popError` function with signature `popError()` and selector `0xb22dc54d`
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
    pub struct PopErrorReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `zeroVarError` function with signature `zeroVarError()` and selector `0xb67689da`
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
    pub struct ZeroVarErrorReturn(pub ::ethers::core::types::Bytes);
}
