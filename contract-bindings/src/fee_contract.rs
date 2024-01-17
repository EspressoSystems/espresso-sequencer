pub use fee_contract::*;
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
pub mod fee_contract {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MAX_DEPOSIT_AMOUNT"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MAX_DEPOSIT_AMOUNT"),
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
                    ::std::borrow::ToOwned::to_owned("balances"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balances"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deposit"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getBalance"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Deposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Log"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Log"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("func"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("gas"),
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
                    ::std::borrow::ToOwned::to_owned("DepositTooLarge"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DepositTooLarge"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FunctionDoesNotExist"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FunctionDoesNotExist",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidUserAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidUserAddress"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoFunctionCalled"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NoFunctionCalled"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NonZeroDepositAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NonZeroDepositAmount",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotEnoughDeposited"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotEnoughDeposited"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FEECONTRACT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@Rg\r\xE0\xB6\xB3\xA7d\0\0`\x80R4\x80\x15a\0\x1CW`\0\x80\xFD[P`\x80Qa\x02\xC5a\0>`\09`\0\x81\x81`\xCB\x01Ra\x01E\x01Ra\x02\xC5`\0\xF3\xFE`\x80`@R`\x046\x10a\0CW`\x005`\xE0\x1C\x80c'\xE25\xE3\x14a\0zW\x80cL4\xA9\x82\x14a\0\xB9W\x80c\xF3@\xFA\x01\x14a\0\xEDW\x80c\xF8\xB2\xCBO\x14a\x01\x02Wa\0aV[6a\0aW`@Qc\xBC\x8E\xCA\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x155\xAC_`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[4\x80\x15a\0\x86W`\0\x80\xFD[Pa\0\xA7a\0\x956`\x04a\x02aV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xC5W`\0\x80\xFD[Pa\0\xA7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\0a\0\xFB6`\x04a\x02aV[a\x01\"V[\0[4\x80\x15a\x01\x0EW`\0\x80\xFD[Pa\0\xA7a\x01\x1D6`\x04a\x02aV[a\x02\x1CV[4`\0\x03a\x01CW`@Qc\x9E\xD2 \xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x004\x11\x15a\x01\x84W`@Qc\xC5mF\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01\xABW`@Qc\x07\x02\xB3\xD9`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T4\x92\x90a\x01\xD3\x90\x84\x90a\x02\x91V[\x90\x91UPP`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R4` \x82\x01R\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C\x91\x01`@Q\x80\x91\x03\x90\xA1PV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02EW`@Qc\x07\x02\xB3\xD9`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[`\0` \x82\x84\x03\x12\x15a\x02sW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x8AW`\0\x80\xFD[\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x02\xB2WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static FEECONTRACT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0CW`\x005`\xE0\x1C\x80c'\xE25\xE3\x14a\0zW\x80cL4\xA9\x82\x14a\0\xB9W\x80c\xF3@\xFA\x01\x14a\0\xEDW\x80c\xF8\xB2\xCBO\x14a\x01\x02Wa\0aV[6a\0aW`@Qc\xBC\x8E\xCA\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x155\xAC_`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[4\x80\x15a\0\x86W`\0\x80\xFD[Pa\0\xA7a\0\x956`\x04a\x02aV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xC5W`\0\x80\xFD[Pa\0\xA7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\0a\0\xFB6`\x04a\x02aV[a\x01\"V[\0[4\x80\x15a\x01\x0EW`\0\x80\xFD[Pa\0\xA7a\x01\x1D6`\x04a\x02aV[a\x02\x1CV[4`\0\x03a\x01CW`@Qc\x9E\xD2 \xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x004\x11\x15a\x01\x84W`@Qc\xC5mF\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01\xABW`@Qc\x07\x02\xB3\xD9`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T4\x92\x90a\x01\xD3\x90\x84\x90a\x02\x91V[\x90\x91UPP`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R4` \x82\x01R\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C\x91\x01`@Q\x80\x91\x03\x90\xA1PV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02EW`@Qc\x07\x02\xB3\xD9`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[`\0` \x82\x84\x03\x12\x15a\x02sW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x8AW`\0\x80\xFD[\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x02\xB2WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static FEECONTRACT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct FeeContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FeeContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FeeContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FeeContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FeeContract<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FeeContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FeeContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                FEECONTRACT_ABI.clone(),
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
                FEECONTRACT_ABI.clone(),
                FEECONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MAX_DEPOSIT_AMOUNT` (0x4c34a982) function
        pub fn max_deposit_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([76, 52, 169, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balances` (0x27e235e3) function
        pub fn balances(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 226, 53, 227], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xf340fa01) function
        pub fn deposit(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 64, 250, 1], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBalance` (0xf8b2cb4f) function
        pub fn get_balance(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([248, 178, 203, 79], user)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `Log` event
        pub fn log_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeeContractEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for FeeContract<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DepositTooLarge` with signature `DepositTooLarge()` and selector `0xc56d46d3`
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
    #[etherror(name = "DepositTooLarge", abi = "DepositTooLarge()")]
    pub struct DepositTooLarge;
    ///Custom Error type `FunctionDoesNotExist` with signature `FunctionDoesNotExist()` and selector `0xa9ad62f8`
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
    #[etherror(name = "FunctionDoesNotExist", abi = "FunctionDoesNotExist()")]
    pub struct FunctionDoesNotExist;
    ///Custom Error type `InvalidUserAddress` with signature `InvalidUserAddress()` and selector `0x702b3d90`
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
    #[etherror(name = "InvalidUserAddress", abi = "InvalidUserAddress()")]
    pub struct InvalidUserAddress;
    ///Custom Error type `NoFunctionCalled` with signature `NoFunctionCalled()` and selector `0xbc8eca1b`
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
    #[etherror(name = "NoFunctionCalled", abi = "NoFunctionCalled()")]
    pub struct NoFunctionCalled;
    ///Custom Error type `NonZeroDepositAmount` with signature `NonZeroDepositAmount()` and selector `0x9ed220d3`
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
    #[etherror(name = "NonZeroDepositAmount", abi = "NonZeroDepositAmount()")]
    pub struct NonZeroDepositAmount;
    ///Custom Error type `NotEnoughDeposited` with signature `NotEnoughDeposited()` and selector `0x5fd4ac44`
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
    #[etherror(name = "NotEnoughDeposited", abi = "NotEnoughDeposited()")]
    pub struct NotEnoughDeposited;
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
    pub enum FeeContractErrors {
        DepositTooLarge(DepositTooLarge),
        FunctionDoesNotExist(FunctionDoesNotExist),
        InvalidUserAddress(InvalidUserAddress),
        NoFunctionCalled(NoFunctionCalled),
        NonZeroDepositAmount(NonZeroDepositAmount),
        NotEnoughDeposited(NotEnoughDeposited),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for FeeContractErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DepositTooLarge as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositTooLarge(decoded));
            }
            if let Ok(decoded) =
                <FunctionDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FunctionDoesNotExist(decoded));
            }
            if let Ok(decoded) =
                <InvalidUserAddress as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidUserAddress(decoded));
            }
            if let Ok(decoded) = <NoFunctionCalled as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NoFunctionCalled(decoded));
            }
            if let Ok(decoded) =
                <NonZeroDepositAmount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NonZeroDepositAmount(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughDeposited as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NotEnoughDeposited(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FeeContractErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DepositTooLarge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FunctionDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidUserAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoFunctionCalled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NonZeroDepositAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughDeposited(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for FeeContractErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <DepositTooLarge as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FunctionDoesNotExist as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InvalidUserAddress as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <NoFunctionCalled as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NonZeroDepositAmount as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <NotEnoughDeposited as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for FeeContractErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::FunctionDoesNotExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUserAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoFunctionCalled(element) => ::core::fmt::Display::fmt(element, f),
                Self::NonZeroDepositAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotEnoughDeposited(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for FeeContractErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DepositTooLarge> for FeeContractErrors {
        fn from(value: DepositTooLarge) -> Self {
            Self::DepositTooLarge(value)
        }
    }
    impl ::core::convert::From<FunctionDoesNotExist> for FeeContractErrors {
        fn from(value: FunctionDoesNotExist) -> Self {
            Self::FunctionDoesNotExist(value)
        }
    }
    impl ::core::convert::From<InvalidUserAddress> for FeeContractErrors {
        fn from(value: InvalidUserAddress) -> Self {
            Self::InvalidUserAddress(value)
        }
    }
    impl ::core::convert::From<NoFunctionCalled> for FeeContractErrors {
        fn from(value: NoFunctionCalled) -> Self {
            Self::NoFunctionCalled(value)
        }
    }
    impl ::core::convert::From<NonZeroDepositAmount> for FeeContractErrors {
        fn from(value: NonZeroDepositAmount) -> Self {
            Self::NonZeroDepositAmount(value)
        }
    }
    impl ::core::convert::From<NotEnoughDeposited> for FeeContractErrors {
        fn from(value: NotEnoughDeposited) -> Self {
            Self::NotEnoughDeposited(value)
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,uint256)")]
    pub struct DepositFilter(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
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
    #[ethevent(name = "Log", abi = "Log(string,uint256)")]
    pub struct LogFilter {
        pub func: ::std::string::String,
        pub gas: ::ethers::core::types::U256,
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
    pub enum FeeContractEvents {
        DepositFilter(DepositFilter),
        LogFilter(LogFilter),
    }
    impl ::ethers::contract::EthLogDecode for FeeContractEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(FeeContractEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(FeeContractEvents::LogFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for FeeContractEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositFilter> for FeeContractEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<LogFilter> for FeeContractEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    ///Container type for all input parameters for the `MAX_DEPOSIT_AMOUNT` function with signature `MAX_DEPOSIT_AMOUNT()` and selector `0x4c34a982`
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
    #[ethcall(name = "MAX_DEPOSIT_AMOUNT", abi = "MAX_DEPOSIT_AMOUNT()")]
    pub struct MaxDepositAmountCall;
    ///Container type for all input parameters for the `balances` function with signature `balances(address)` and selector `0x27e235e3`
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
    #[ethcall(name = "balances", abi = "balances(address)")]
    pub struct BalancesCall {
        pub user: ::ethers::core::types::Address,
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
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getBalance` function with signature `getBalance(address)` and selector `0xf8b2cb4f`
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
    #[ethcall(name = "getBalance", abi = "getBalance(address)")]
    pub struct GetBalanceCall {
        pub user: ::ethers::core::types::Address,
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
    pub enum FeeContractCalls {
        MaxDepositAmount(MaxDepositAmountCall),
        Balances(BalancesCall),
        Deposit(DepositCall),
        GetBalance(GetBalanceCall),
    }
    impl ::ethers::core::abi::AbiDecode for FeeContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <MaxDepositAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxDepositAmount(decoded));
            }
            if let Ok(decoded) = <BalancesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Balances(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <GetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBalance(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FeeContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxDepositAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Balances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for FeeContractCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxDepositAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Balances(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBalance(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxDepositAmountCall> for FeeContractCalls {
        fn from(value: MaxDepositAmountCall) -> Self {
            Self::MaxDepositAmount(value)
        }
    }
    impl ::core::convert::From<BalancesCall> for FeeContractCalls {
        fn from(value: BalancesCall) -> Self {
            Self::Balances(value)
        }
    }
    impl ::core::convert::From<DepositCall> for FeeContractCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<GetBalanceCall> for FeeContractCalls {
        fn from(value: GetBalanceCall) -> Self {
            Self::GetBalance(value)
        }
    }
    ///Container type for all return fields from the `MAX_DEPOSIT_AMOUNT` function with signature `MAX_DEPOSIT_AMOUNT()` and selector `0x4c34a982`
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
    pub struct MaxDepositAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balances` function with signature `balances(address)` and selector `0x27e235e3`
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
    pub struct BalancesReturn {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getBalance` function with signature `getBalance(address)` and selector `0xf8b2cb4f`
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
    pub struct GetBalanceReturn {
        pub amount: ::ethers::core::types::U256,
    }
}
