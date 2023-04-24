pub use i_bridge_message_receiver::*;
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
pub mod i_bridge_message_receiver {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"originAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"onMessageReceived\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IBRIDGEMESSAGERECEIVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IBridgeMessageReceiver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IBridgeMessageReceiver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IBridgeMessageReceiver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IBridgeMessageReceiver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IBridgeMessageReceiver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IBridgeMessageReceiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IBridgeMessageReceiver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IBRIDGEMESSAGERECEIVER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `onMessageReceived` (0x1806b5f2) function
        pub fn on_message_received(
            &self,
            origin_address: ::ethers::core::types::Address,
            origin_network: u32,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 6, 181, 242], (origin_address, origin_network, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IBridgeMessageReceiver<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `onMessageReceived` function with signature `onMessageReceived(address,uint32,bytes)` and selector `0x1806b5f2`
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
    #[ethcall(
        name = "onMessageReceived",
        abi = "onMessageReceived(address,uint32,bytes)"
    )]
    pub struct OnMessageReceivedCall {
        pub origin_address: ::ethers::core::types::Address,
        pub origin_network: u32,
        pub data: ::ethers::core::types::Bytes,
    }
}
