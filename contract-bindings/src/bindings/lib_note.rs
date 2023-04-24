pub use lib_note::*;
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
pub mod lib_note {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"sig\",\"type\":\"bytes4\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"usr\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"arg1\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"arg2\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LogNote\",\"outputs\":[],\"anonymous\":true}]";
    ///The parsed JSON ABI of the contract.
    pub static LIBNOTE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
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
        62,
        128,
        96,
        29,
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
        96,
        0,
        128,
        253,
        254,
        162,
        101,
        98,
        122,
        122,
        114,
        49,
        88,
        32,
        202,
        7,
        177,
        39,
        77,
        115,
        14,
        243,
        133,
        105,
        125,
        229,
        240,
        90,
        247,
        233,
        103,
        241,
        169,
        186,
        133,
        47,
        139,
        239,
        32,
        245,
        130,
        53,
        224,
        6,
        37,
        48,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        5,
        12,
        0,
        50,
    ];
    ///The bytecode of the contract.
    pub static LIBNOTE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
        253,
        254,
        162,
        101,
        98,
        122,
        122,
        114,
        49,
        88,
        32,
        202,
        7,
        177,
        39,
        77,
        115,
        14,
        243,
        133,
        105,
        125,
        229,
        240,
        90,
        247,
        233,
        103,
        241,
        169,
        186,
        133,
        47,
        139,
        239,
        32,
        245,
        130,
        53,
        224,
        6,
        37,
        48,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        5,
        12,
        0,
        50,
    ];
    ///The deployed bytecode of the contract.
    pub static LIBNOTE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LibNote<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LibNote<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LibNote<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LibNote<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LibNote<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(LibNote))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LibNote<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LIBNOTE_ABI.clone(),
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
                LIBNOTE_ABI.clone(),
                LIBNOTE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Gets the contract's `LogNote` event
        pub fn log_note_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNoteFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNoteFilter> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for LibNote<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "LogNote",
        abi = "LogNote(bytes4,address,bytes32,bytes32,bytes) anonymous"
    )]
    pub struct LogNoteFilter {
        #[ethevent(indexed)]
        pub sig: [u8; 4],
        #[ethevent(indexed)]
        pub usr: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub arg_1: [u8; 32],
        #[ethevent(indexed)]
        pub arg_2: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
    }
}
