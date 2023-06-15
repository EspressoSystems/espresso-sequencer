pub use bn254::*;
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
pub mod bn254 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"P_MOD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"R_MOD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static BN254_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        214,
        97,
        0,
        57,
        96,
        11,
        130,
        130,
        130,
        57,
        128,
        81,
        96,
        0,
        26,
        96,
        115,
        20,
        97,
        0,
        44,
        87,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        0,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        48,
        96,
        0,
        82,
        96,
        115,
        129,
        83,
        130,
        129,
        243,
        254,
        115,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        96,
        61,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        29,
        113,
        46,
        39,
        20,
        96,
        66,
        87,
        128,
        99,
        223,
        110,
        108,
        180,
        20,
        96,
        122,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        104,
        127,
        48,
        100,
        78,
        114,
        225,
        49,
        160,
        41,
        184,
        80,
        69,
        182,
        129,
        129,
        88,
        93,
        151,
        129,
        106,
        145,
        104,
        113,
        202,
        141,
        60,
        32,
        140,
        22,
        216,
        124,
        253,
        71,
        129,
        86,
        91,
        96,
        64,
        81,
        144,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        104,
        127,
        48,
        100,
        78,
        114,
        225,
        49,
        160,
        41,
        184,
        80,
        69,
        182,
        129,
        129,
        88,
        93,
        40,
        51,
        232,
        72,
        121,
        185,
        112,
        145,
        67,
        225,
        245,
        147,
        240,
        0,
        0,
        1,
        129,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        231,
        200,
        193,
        91,
        170,
        124,
        37,
        211,
        167,
        56,
        47,
        167,
        89,
        93,
        206,
        33,
        197,
        9,
        200,
        208,
        144,
        71,
        4,
        77,
        87,
        46,
        205,
        193,
        47,
        52,
        15,
        179,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        18,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static BN254_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        115,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        96,
        61,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        29,
        113,
        46,
        39,
        20,
        96,
        66,
        87,
        128,
        99,
        223,
        110,
        108,
        180,
        20,
        96,
        122,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        104,
        127,
        48,
        100,
        78,
        114,
        225,
        49,
        160,
        41,
        184,
        80,
        69,
        182,
        129,
        129,
        88,
        93,
        151,
        129,
        106,
        145,
        104,
        113,
        202,
        141,
        60,
        32,
        140,
        22,
        216,
        124,
        253,
        71,
        129,
        86,
        91,
        96,
        64,
        81,
        144,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        104,
        127,
        48,
        100,
        78,
        114,
        225,
        49,
        160,
        41,
        184,
        80,
        69,
        182,
        129,
        129,
        88,
        93,
        40,
        51,
        232,
        72,
        121,
        185,
        112,
        145,
        67,
        225,
        245,
        147,
        240,
        0,
        0,
        1,
        129,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        231,
        200,
        193,
        91,
        170,
        124,
        37,
        211,
        167,
        56,
        47,
        167,
        89,
        93,
        206,
        33,
        197,
        9,
        200,
        208,
        144,
        71,
        4,
        77,
        87,
        46,
        205,
        193,
        47,
        52,
        15,
        179,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        18,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static BN254_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct BN254<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BN254<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BN254<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BN254<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BN254<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(BN254))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BN254<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                BN254_ABI.clone(),
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
                BN254_ABI.clone(),
                BN254_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `P_MOD` (0x1d712e27) function
        pub fn p_mod(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([29, 113, 46, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `R_MOD` (0xdf6e6cb4) function
        pub fn r_mod(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([223, 110, 108, 180], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for BN254<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `P_MOD` function with signature `P_MOD()` and selector `0x1d712e27`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "P_MOD", abi = "P_MOD()")]
    pub struct PModCall;
    ///Container type for all input parameters for the `R_MOD` function with signature `R_MOD()` and selector `0xdf6e6cb4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "R_MOD", abi = "R_MOD()")]
    pub struct RModCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BN254Calls {
        PMod(PModCall),
        RMod(RModCall),
    }
    impl ::ethers::core::abi::AbiDecode for BN254Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <PModCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PMod(decoded));
            }
            if let Ok(decoded) = <RModCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RMod(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BN254Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::PMod(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RMod(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for BN254Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PMod(element) => ::core::fmt::Display::fmt(element, f),
                Self::RMod(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PModCall> for BN254Calls {
        fn from(value: PModCall) -> Self {
            Self::PMod(value)
        }
    }
    impl ::core::convert::From<RModCall> for BN254Calls {
        fn from(value: RModCall) -> Self {
            Self::RMod(value)
        }
    }
    ///Container type for all return fields from the `P_MOD` function with signature `P_MOD()` and selector `0x1d712e27`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PModReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `R_MOD` function with signature `R_MOD()` and selector `0xdf6e6cb4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RModReturn(pub ::ethers::core::types::U256);
}
