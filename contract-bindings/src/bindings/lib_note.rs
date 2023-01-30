pub use lib_note::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod lib_note {
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
    #[doc = "LibNote was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"sig\",\"type\":\"bytes4\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"usr\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"arg1\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"arg2\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LogNote\",\"outputs\":[],\"anonymous\":true}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static LIBNOTE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct LibNote<M>(ethers::contract::Contract<M>);
    impl<M> Clone for LibNote<M> {
        fn clone(&self) -> Self {
            LibNote(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for LibNote<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for LibNote<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(LibNote))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> LibNote<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), LIBNOTE_ABI.clone(), client).into()
        }
        #[doc = "Gets the contract's `LogNote` event"]
        pub fn log_note_filter(&self) -> ethers::contract::builders::Event<M, LogNoteFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, LogNoteFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for LibNote<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "LogNote",
        abi = "LogNote(bytes4,address,bytes32,bytes32,bytes) anonymous"
    )]
    pub struct LogNoteFilter {
        #[ethevent(indexed)]
        pub sig: [u8; 4],
        #[ethevent(indexed)]
        pub usr: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub arg_1: [u8; 32],
        #[ethevent(indexed)]
        pub arg_2: [u8; 32],
        pub data: ethers::core::types::Bytes,
    }
}
