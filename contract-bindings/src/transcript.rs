pub use transcript::*;
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
pub mod transcript {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("computeHash0"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computeHash0"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("a"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("b"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("c"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("computeHash1"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computeHash1"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("a"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("b"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("c"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TRANSCRIPT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x02/a\0:`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14a\0-WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0@W`\x005`\xE0\x1C\x80c\x1B\x84#\xD2\x14a\0EW\x80c\xE6an\x0C\x14a\0jW[`\0\x80\xFD[a\0Xa\0S6`\x04a\x01@V[a\0}V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0Xa\0x6`\x04a\x01@V[a\0\xC7V[`\0\x80`@Q\x85\x81R\x84` \x82\x01R`@\x81\x01\x84Q`\0[\x81\x81\x10\x15a\0\xAFW` \x81\x88\x01\x81\x01Q\x84\x83\x01R\x01a\0\x95V[P`\x01\x91\x81\x01\x91\x90\x91R`A\x01\x90 \x95\x94PPPPPV[`\0\x80`@Q\x85\x81R\x84` \x82\x01R`@\x81\x01\x84Q`\0[\x81\x81\x10\x15a\0\xF9W` \x81\x88\x01\x81\x01Q\x84\x83\x01R\x01a\0\xDFV[P`\0\x91\x81\x01\x91\x90\x91R`A\x01\x90 \x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x01UW`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01{W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x01\x8FW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01\xA1Wa\x01\xA1a\x01\x11V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x01\xE7Wa\x01\xE7a\x01\x11V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x02\0W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static TRANSCRIPT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0@W`\x005`\xE0\x1C\x80c\x1B\x84#\xD2\x14a\0EW\x80c\xE6an\x0C\x14a\0jW[`\0\x80\xFD[a\0Xa\0S6`\x04a\x01@V[a\0}V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0Xa\0x6`\x04a\x01@V[a\0\xC7V[`\0\x80`@Q\x85\x81R\x84` \x82\x01R`@\x81\x01\x84Q`\0[\x81\x81\x10\x15a\0\xAFW` \x81\x88\x01\x81\x01Q\x84\x83\x01R\x01a\0\x95V[P`\x01\x91\x81\x01\x91\x90\x91R`A\x01\x90 \x95\x94PPPPPV[`\0\x80`@Q\x85\x81R\x84` \x82\x01R`@\x81\x01\x84Q`\0[\x81\x81\x10\x15a\0\xF9W` \x81\x88\x01\x81\x01Q\x84\x83\x01R\x01a\0\xDFV[P`\0\x91\x81\x01\x91\x90\x91R`A\x01\x90 \x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x01UW`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01{W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x01\x8FW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01\xA1Wa\x01\xA1a\x01\x11V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x01\xE7Wa\x01\xE7a\x01\x11V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x02\0W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static TRANSCRIPT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Transcript<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Transcript<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Transcript<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Transcript<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Transcript<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Transcript))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Transcript<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                TRANSCRIPT_ABI.clone(),
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
                TRANSCRIPT_ABI.clone(),
                TRANSCRIPT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `computeHash0` (0xe6616e0c) function
        pub fn compute_hash_0(
            &self,
            a: [u8; 32],
            b: [u8; 32],
            c: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([230, 97, 110, 12], (a, b, c))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeHash1` (0x1b8423d2) function
        pub fn compute_hash_1(
            &self,
            a: [u8; 32],
            b: [u8; 32],
            c: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([27, 132, 35, 210], (a, b, c))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Transcript<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `computeHash0` function with signature `computeHash0(bytes32,bytes32,bytes)` and selector `0xe6616e0c`
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
    #[ethcall(name = "computeHash0", abi = "computeHash0(bytes32,bytes32,bytes)")]
    pub struct ComputeHash0Call {
        pub a: [u8; 32],
        pub b: [u8; 32],
        pub c: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `computeHash1` function with signature `computeHash1(bytes32,bytes32,bytes)` and selector `0x1b8423d2`
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
    #[ethcall(name = "computeHash1", abi = "computeHash1(bytes32,bytes32,bytes)")]
    pub struct ComputeHash1Call {
        pub a: [u8; 32],
        pub b: [u8; 32],
        pub c: ::ethers::core::types::Bytes,
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
    pub enum TranscriptCalls {
        ComputeHash0(ComputeHash0Call),
        ComputeHash1(ComputeHash1Call),
    }
    impl ::ethers::core::abi::AbiDecode for TranscriptCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ComputeHash0Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeHash0(decoded));
            }
            if let Ok(decoded) = <ComputeHash1Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeHash1(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TranscriptCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ComputeHash0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComputeHash1(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for TranscriptCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComputeHash0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeHash1(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ComputeHash0Call> for TranscriptCalls {
        fn from(value: ComputeHash0Call) -> Self {
            Self::ComputeHash0(value)
        }
    }
    impl ::core::convert::From<ComputeHash1Call> for TranscriptCalls {
        fn from(value: ComputeHash1Call) -> Self {
            Self::ComputeHash1(value)
        }
    }
    ///Container type for all return fields from the `computeHash0` function with signature `computeHash0(bytes32,bytes32,bytes)` and selector `0xe6616e0c`
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
    pub struct ComputeHash0Return(pub [u8; 32]);
    ///Container type for all return fields from the `computeHash1` function with signature `computeHash1(bytes32,bytes32,bytes)` and selector `0x1b8423d2`
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
    pub struct ComputeHash1Return(pub [u8; 32]);
}
