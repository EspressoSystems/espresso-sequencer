pub use i_bridge_message_receiver::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_bridge_message_receiver {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IBridgeMessageReceiver was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"originAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"onMessageReceived\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IBRIDGEMESSAGERECEIVER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IBridgeMessageReceiver<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IBridgeMessageReceiver<M> {
        fn clone(&self) -> Self {
            IBridgeMessageReceiver(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IBridgeMessageReceiver<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IBridgeMessageReceiver<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IBridgeMessageReceiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IBridgeMessageReceiver<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IBRIDGEMESSAGERECEIVER_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `onMessageReceived` (0x1806b5f2) function"]
        pub fn on_message_received(
            &self,
            origin_address: ethers::core::types::Address,
            origin_network: u32,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 6, 181, 242], (origin_address, origin_network, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IBridgeMessageReceiver<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `onMessageReceived` function with signature `onMessageReceived(address,uint32,bytes)` and selector `[24, 6, 181, 242]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "onMessageReceived",
        abi = "onMessageReceived(address,uint32,bytes)"
    )]
    pub struct OnMessageReceivedCall {
        pub origin_address: ethers::core::types::Address,
        pub origin_network: u32,
        pub data: ethers::core::types::Bytes,
    }
}
