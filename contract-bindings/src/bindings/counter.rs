pub use counter::*;
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
pub mod counter {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"doNothing\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"doNothing3\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increment\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"number\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newNumber2\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setNumber\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static COUNTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        1,
        11,
        128,
        97,
        0,
        32,
        96,
        0,
        57,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        96,
        15,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        96,
        80,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        47,
        87,
        111,
        32,
        20,
        96,
        85,
        87,
        128,
        99,
        63,
        181,
        193,
        203,
        20,
        96,
        87,
        87,
        128,
        99,
        95,
        135,
        157,
        49,
        20,
        96,
        85,
        87,
        128,
        99,
        131,
        129,
        245,
        138,
        20,
        96,
        103,
        87,
        128,
        99,
        208,
        157,
        224,
        138,
        20,
        96,
        129,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        0,
        91,
        96,
        85,
        96,
        98,
        54,
        96,
        4,
        96,
        151,
        86,
        91,
        96,
        0,
        85,
        86,
        91,
        96,
        111,
        96,
        0,
        84,
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
        85,
        96,
        0,
        128,
        84,
        144,
        128,
        96,
        144,
        131,
        96,
        175,
        86,
        91,
        145,
        144,
        80,
        85,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        96,
        168,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        145,
        144,
        80,
        86,
        91,
        96,
        0,
        96,
        1,
        130,
        1,
        96,
        206,
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
        17,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        80,
        96,
        1,
        1,
        144,
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
        215,
        134,
        99,
        15,
        218,
        224,
        207,
        135,
        47,
        23,
        196,
        39,
        43,
        215,
        223,
        195,
        200,
        166,
        246,
        10,
        51,
        177,
        124,
        138,
        228,
        239,
        196,
        253,
        150,
        72,
        154,
        80,
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
    pub static COUNTER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        96,
        15,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        96,
        80,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        47,
        87,
        111,
        32,
        20,
        96,
        85,
        87,
        128,
        99,
        63,
        181,
        193,
        203,
        20,
        96,
        87,
        87,
        128,
        99,
        95,
        135,
        157,
        49,
        20,
        96,
        85,
        87,
        128,
        99,
        131,
        129,
        245,
        138,
        20,
        96,
        103,
        87,
        128,
        99,
        208,
        157,
        224,
        138,
        20,
        96,
        129,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        0,
        91,
        96,
        85,
        96,
        98,
        54,
        96,
        4,
        96,
        151,
        86,
        91,
        96,
        0,
        85,
        86,
        91,
        96,
        111,
        96,
        0,
        84,
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
        85,
        96,
        0,
        128,
        84,
        144,
        128,
        96,
        144,
        131,
        96,
        175,
        86,
        91,
        145,
        144,
        80,
        85,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        96,
        168,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        145,
        144,
        80,
        86,
        91,
        96,
        0,
        96,
        1,
        130,
        1,
        96,
        206,
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
        17,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        80,
        96,
        1,
        1,
        144,
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
        215,
        134,
        99,
        15,
        218,
        224,
        207,
        135,
        47,
        23,
        196,
        39,
        43,
        215,
        223,
        195,
        200,
        166,
        246,
        10,
        51,
        177,
        124,
        138,
        228,
        239,
        196,
        253,
        150,
        72,
        154,
        80,
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
    pub static COUNTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Counter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Counter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Counter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Counter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Counter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Counter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Counter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                COUNTER_ABI.clone(),
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
                COUNTER_ABI.clone(),
                COUNTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `doNothing` (0x2f576f20) function
        pub fn do_nothing(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 87, 111, 32], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `doNothing3` (0x5f879d31) function
        pub fn do_nothing_3(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 135, 157, 49], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increment` (0xd09de08a) function
        pub fn increment(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 157, 224, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `number` (0x8381f58a) function
        pub fn number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([131, 129, 245, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setNumber` (0x3fb5c1cb) function
        pub fn set_number(
            &self,
            new_number_2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 181, 193, 203], new_number_2)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Counter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `doNothing` function with signature `doNothing()` and selector `0x2f576f20`
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
    #[ethcall(name = "doNothing", abi = "doNothing()")]
    pub struct DoNothingCall;
    ///Container type for all input parameters for the `doNothing3` function with signature `doNothing3()` and selector `0x5f879d31`
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
    #[ethcall(name = "doNothing3", abi = "doNothing3()")]
    pub struct DoNothing3Call;
    ///Container type for all input parameters for the `increment` function with signature `increment()` and selector `0xd09de08a`
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
    #[ethcall(name = "increment", abi = "increment()")]
    pub struct IncrementCall;
    ///Container type for all input parameters for the `number` function with signature `number()` and selector `0x8381f58a`
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
    #[ethcall(name = "number", abi = "number()")]
    pub struct NumberCall;
    ///Container type for all input parameters for the `setNumber` function with signature `setNumber(uint256)` and selector `0x3fb5c1cb`
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
    #[ethcall(name = "setNumber", abi = "setNumber(uint256)")]
    pub struct SetNumberCall {
        pub new_number_2: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CounterCalls {
        DoNothing(DoNothingCall),
        DoNothing3(DoNothing3Call),
        Increment(IncrementCall),
        Number(NumberCall),
        SetNumber(SetNumberCall),
    }
    impl ::ethers::core::abi::AbiDecode for CounterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DoNothingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DoNothing(decoded));
            }
            if let Ok(decoded) = <DoNothing3Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DoNothing3(decoded));
            }
            if let Ok(decoded) = <IncrementCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Increment(decoded));
            }
            if let Ok(decoded) = <NumberCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Number(decoded));
            }
            if let Ok(decoded) = <SetNumberCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetNumber(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CounterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DoNothing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DoNothing3(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Increment(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Number(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetNumber(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for CounterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DoNothing(element) => ::core::fmt::Display::fmt(element, f),
                Self::DoNothing3(element) => ::core::fmt::Display::fmt(element, f),
                Self::Increment(element) => ::core::fmt::Display::fmt(element, f),
                Self::Number(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNumber(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DoNothingCall> for CounterCalls {
        fn from(value: DoNothingCall) -> Self {
            Self::DoNothing(value)
        }
    }
    impl ::core::convert::From<DoNothing3Call> for CounterCalls {
        fn from(value: DoNothing3Call) -> Self {
            Self::DoNothing3(value)
        }
    }
    impl ::core::convert::From<IncrementCall> for CounterCalls {
        fn from(value: IncrementCall) -> Self {
            Self::Increment(value)
        }
    }
    impl ::core::convert::From<NumberCall> for CounterCalls {
        fn from(value: NumberCall) -> Self {
            Self::Number(value)
        }
    }
    impl ::core::convert::From<SetNumberCall> for CounterCalls {
        fn from(value: SetNumberCall) -> Self {
            Self::SetNumber(value)
        }
    }
    ///Container type for all return fields from the `number` function with signature `number()` and selector `0x8381f58a`
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
    pub struct NumberReturn(pub ::ethers::core::types::U256);
}
