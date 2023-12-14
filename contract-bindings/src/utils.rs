pub use utils::*;
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
pub mod utils {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("convertScalarFieldArraytoU256Array"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "convertScalarFieldArraytoU256Array",
                        ),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("a"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("BN254.ScalarField[]"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("convertU256ArrayToScalarFieldArray"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "convertU256ArrayToScalarFieldArray",
                        ),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("a"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("BN254.ScalarField[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("convertU256ArrayToScalarFieldArray2D"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "convertU256ArrayToScalarFieldArray2D",
                        ),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("a"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[][]"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("BN254.ScalarField[][]"),
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
    pub static UTILS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x06\xA9a\0:`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14a\0-WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0KW`\x005`\xE0\x1C\x80cH\xDF\x14]\x14a\0PW\x80cO!o\x88\x14a\0yW\x80c\xE0\x80\x9D`\x14a\0\x99W[`\0\x80\xFD[a\0ca\0^6`\x04a\x049V[a\0\xACV[`@Qa\0p\x91\x90a\x04\xEAV[`@Q\x80\x91\x03\x90\xF3[a\0\x8Ca\0\x876`\x04a\x05tV[a\x02.V[`@Qa\0p\x91\x90a\x05\xB1V[a\0\x8Ca\0\xA76`\x04a\x05\xF5V[a\x02\xCAV[\x80Q``\x90`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xCCWa\0\xCCa\x03_V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xFFW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\xEAW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x02&W`\0[\x85\x82\x81Q\x81\x10a\x01\"Wa\x01\"a\x06\x86V[` \x02` \x01\x01QQ\x81\x10\x15a\x02\x1DW\x85\x82\x81Q\x81\x10a\x01DWa\x01Da\x06\x86V[` \x02` \x01\x01QQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01eWa\x01ea\x03_V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x8EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83\x83\x81Q\x81\x10a\x01\xA1Wa\x01\xA1a\x06\x86V[` \x02` \x01\x01\x81\x90RP\x85\x82\x81Q\x81\x10a\x01\xBEWa\x01\xBEa\x06\x86V[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x01\xD7Wa\x01\xD7a\x06\x86V[` \x02` \x01\x01Q\x83\x83\x81Q\x81\x10a\x01\xF1Wa\x01\xF1a\x06\x86V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x02\nWa\x02\na\x06\x86V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x01\x10V[P`\x01\x01a\x01\x05V[P\x93\x92PPPV[```\0\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02LWa\x02La\x03_V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02uW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x02\xC3W\x83\x81\x81Q\x81\x10a\x02\x96Wa\x02\x96a\x06\x86V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x02\xB0Wa\x02\xB0a\x06\x86V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x02{V[P\x92\x91PPV[```\0\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xE8Wa\x02\xE8a\x03_V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\x11W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x02\xC3W\x83\x81\x81Q\x81\x10a\x032Wa\x032a\x06\x86V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x03LWa\x03La\x06\x86V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x03\x17V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\x9EWa\x03\x9Ea\x03_V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\xC0Wa\x03\xC0a\x03_V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x03\xDBW`\0\x80\xFD[\x815` a\x03\xF0a\x03\xEB\x83a\x03\xA6V[a\x03uV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x04\x12W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x04.W\x805\x83R\x91\x83\x01\x91\x83\x01a\x04\x17V[P\x96\x95PPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x04LW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04dW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04xW`\0\x80\xFD[\x815a\x04\x86a\x03\xEB\x82a\x03\xA6V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a\x04\xA5W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a\x04\xDDW\x805\x85\x81\x11\x15a\x04\xC1W`\0\x80\x81\xFD[a\x04\xCF\x8B\x89\x83\x8A\x01\x01a\x03\xCAV[\x84RP\x91\x86\x01\x91\x86\x01a\x04\xA9V[P\x98\x97PPPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0\x80[\x83\x81\x10\x15a\x05fW\x88\x86\x03`?\x19\x01\x85R\x82Q\x80Q\x80\x88R\x90\x88\x01\x90\x88\x88\x01\x90\x84[\x81\x81\x10\x15a\x05PW\x83Q\x83R\x92\x8A\x01\x92\x91\x8A\x01\x91`\x01\x01a\x054V[P\x90\x97PPP\x93\x86\x01\x93\x91\x86\x01\x91`\x01\x01a\x05\x12V[P\x93\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x05\x86W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x9DW`\0\x80\xFD[a\x05\xA9\x84\x82\x85\x01a\x03\xCAV[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x05\xE9W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x05\xCDV[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x06\x08W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x1FW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x060W`\0\x80\xFD[\x805a\x06>a\x03\xEB\x82a\x03\xA6V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x06]W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x06{W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x06bV[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The bytecode of the contract.
    pub static UTILS_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0KW`\x005`\xE0\x1C\x80cH\xDF\x14]\x14a\0PW\x80cO!o\x88\x14a\0yW\x80c\xE0\x80\x9D`\x14a\0\x99W[`\0\x80\xFD[a\0ca\0^6`\x04a\x049V[a\0\xACV[`@Qa\0p\x91\x90a\x04\xEAV[`@Q\x80\x91\x03\x90\xF3[a\0\x8Ca\0\x876`\x04a\x05tV[a\x02.V[`@Qa\0p\x91\x90a\x05\xB1V[a\0\x8Ca\0\xA76`\x04a\x05\xF5V[a\x02\xCAV[\x80Q``\x90`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xCCWa\0\xCCa\x03_V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xFFW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\xEAW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x02&W`\0[\x85\x82\x81Q\x81\x10a\x01\"Wa\x01\"a\x06\x86V[` \x02` \x01\x01QQ\x81\x10\x15a\x02\x1DW\x85\x82\x81Q\x81\x10a\x01DWa\x01Da\x06\x86V[` \x02` \x01\x01QQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01eWa\x01ea\x03_V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x8EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83\x83\x81Q\x81\x10a\x01\xA1Wa\x01\xA1a\x06\x86V[` \x02` \x01\x01\x81\x90RP\x85\x82\x81Q\x81\x10a\x01\xBEWa\x01\xBEa\x06\x86V[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x01\xD7Wa\x01\xD7a\x06\x86V[` \x02` \x01\x01Q\x83\x83\x81Q\x81\x10a\x01\xF1Wa\x01\xF1a\x06\x86V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x02\nWa\x02\na\x06\x86V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x01\x10V[P`\x01\x01a\x01\x05V[P\x93\x92PPPV[```\0\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02LWa\x02La\x03_V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02uW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x02\xC3W\x83\x81\x81Q\x81\x10a\x02\x96Wa\x02\x96a\x06\x86V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x02\xB0Wa\x02\xB0a\x06\x86V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x02{V[P\x92\x91PPV[```\0\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xE8Wa\x02\xE8a\x03_V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\x11W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x02\xC3W\x83\x81\x81Q\x81\x10a\x032Wa\x032a\x06\x86V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x03LWa\x03La\x06\x86V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x03\x17V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\x9EWa\x03\x9Ea\x03_V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\xC0Wa\x03\xC0a\x03_V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x03\xDBW`\0\x80\xFD[\x815` a\x03\xF0a\x03\xEB\x83a\x03\xA6V[a\x03uV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x04\x12W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x04.W\x805\x83R\x91\x83\x01\x91\x83\x01a\x04\x17V[P\x96\x95PPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x04LW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04dW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04xW`\0\x80\xFD[\x815a\x04\x86a\x03\xEB\x82a\x03\xA6V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a\x04\xA5W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a\x04\xDDW\x805\x85\x81\x11\x15a\x04\xC1W`\0\x80\x81\xFD[a\x04\xCF\x8B\x89\x83\x8A\x01\x01a\x03\xCAV[\x84RP\x91\x86\x01\x91\x86\x01a\x04\xA9V[P\x98\x97PPPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0\x80[\x83\x81\x10\x15a\x05fW\x88\x86\x03`?\x19\x01\x85R\x82Q\x80Q\x80\x88R\x90\x88\x01\x90\x88\x88\x01\x90\x84[\x81\x81\x10\x15a\x05PW\x83Q\x83R\x92\x8A\x01\x92\x91\x8A\x01\x91`\x01\x01a\x054V[P\x90\x97PPP\x93\x86\x01\x93\x91\x86\x01\x91`\x01\x01a\x05\x12V[P\x93\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x05\x86W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x9DW`\0\x80\xFD[a\x05\xA9\x84\x82\x85\x01a\x03\xCAV[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x05\xE9W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x05\xCDV[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x06\x08W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x1FW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x060W`\0\x80\xFD[\x805a\x06>a\x03\xEB\x82a\x03\xA6V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x06]W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x06{W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x06bV[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x17\0\n";
    /// The deployed bytecode of the contract.
    pub static UTILS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct utils<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for utils<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for utils<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for utils<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for utils<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(utils))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> utils<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                UTILS_ABI.clone(),
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
                UTILS_ABI.clone(),
                UTILS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `convertScalarFieldArraytoU256Array` (0xe0809d60) function
        pub fn convert_scalar_field_arrayto_u256_array(
            &self,
            a: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([224, 128, 157, 96], a)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertU256ArrayToScalarFieldArray` (0x4f216f88) function
        pub fn convert_u256_array_to_scalar_field_array(
            &self,
            a: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([79, 33, 111, 136], a)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertU256ArrayToScalarFieldArray2D` (0x48df145d) function
        pub fn convert_u256_array_to_scalar_field_array_2d(
            &self,
            a: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::U256>>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::U256>>,
        > {
            self.0
                .method_hash([72, 223, 20, 93], a)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for utils<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `convertScalarFieldArraytoU256Array` function with signature `convertScalarFieldArraytoU256Array(uint256[])` and selector `0xe0809d60`
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
        name = "convertScalarFieldArraytoU256Array",
        abi = "convertScalarFieldArraytoU256Array(uint256[])"
    )]
    pub struct ConvertScalarFieldArraytoU256ArrayCall {
        pub a: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `convertU256ArrayToScalarFieldArray` function with signature `convertU256ArrayToScalarFieldArray(uint256[])` and selector `0x4f216f88`
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
        name = "convertU256ArrayToScalarFieldArray",
        abi = "convertU256ArrayToScalarFieldArray(uint256[])"
    )]
    pub struct ConvertU256ArrayToScalarFieldArrayCall {
        pub a: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `convertU256ArrayToScalarFieldArray2D` function with signature `convertU256ArrayToScalarFieldArray2D(uint256[][])` and selector `0x48df145d`
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
        name = "convertU256ArrayToScalarFieldArray2D",
        abi = "convertU256ArrayToScalarFieldArray2D(uint256[][])"
    )]
    pub struct ConvertU256ArrayToScalarFieldArray2DCall {
        pub a: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::U256>>,
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
    pub enum utilsCalls {
        ConvertScalarFieldArraytoU256Array(ConvertScalarFieldArraytoU256ArrayCall),
        ConvertU256ArrayToScalarFieldArray(ConvertU256ArrayToScalarFieldArrayCall),
        ConvertU256ArrayToScalarFieldArray2D(ConvertU256ArrayToScalarFieldArray2DCall),
    }
    impl ::ethers::core::abi::AbiDecode for utilsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ConvertScalarFieldArraytoU256ArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ConvertScalarFieldArraytoU256Array(decoded));
            }
            if let Ok(decoded) =
                <ConvertU256ArrayToScalarFieldArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ConvertU256ArrayToScalarFieldArray(decoded));
            }
            if let Ok(decoded) =
                <ConvertU256ArrayToScalarFieldArray2DCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ConvertU256ArrayToScalarFieldArray2D(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for utilsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ConvertScalarFieldArraytoU256Array(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertU256ArrayToScalarFieldArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertU256ArrayToScalarFieldArray2D(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for utilsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConvertScalarFieldArraytoU256Array(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConvertU256ArrayToScalarFieldArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConvertU256ArrayToScalarFieldArray2D(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ConvertScalarFieldArraytoU256ArrayCall> for utilsCalls {
        fn from(value: ConvertScalarFieldArraytoU256ArrayCall) -> Self {
            Self::ConvertScalarFieldArraytoU256Array(value)
        }
    }
    impl ::core::convert::From<ConvertU256ArrayToScalarFieldArrayCall> for utilsCalls {
        fn from(value: ConvertU256ArrayToScalarFieldArrayCall) -> Self {
            Self::ConvertU256ArrayToScalarFieldArray(value)
        }
    }
    impl ::core::convert::From<ConvertU256ArrayToScalarFieldArray2DCall> for utilsCalls {
        fn from(value: ConvertU256ArrayToScalarFieldArray2DCall) -> Self {
            Self::ConvertU256ArrayToScalarFieldArray2D(value)
        }
    }
    ///Container type for all return fields from the `convertScalarFieldArraytoU256Array` function with signature `convertScalarFieldArraytoU256Array(uint256[])` and selector `0xe0809d60`
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
    pub struct ConvertScalarFieldArraytoU256ArrayReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `convertU256ArrayToScalarFieldArray` function with signature `convertU256ArrayToScalarFieldArray(uint256[])` and selector `0x4f216f88`
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
    pub struct ConvertU256ArrayToScalarFieldArrayReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `convertU256ArrayToScalarFieldArray2D` function with signature `convertU256ArrayToScalarFieldArray2D(uint256[][])` and selector `0x48df145d`
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
    pub struct ConvertU256ArrayToScalarFieldArray2DReturn(
        pub ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::U256>>,
    );
}
