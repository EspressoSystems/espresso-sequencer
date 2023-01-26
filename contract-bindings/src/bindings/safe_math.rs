pub use safe_math::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod safe_math {
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
    #[doc = "SafeMath was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static SAFEMATH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct SafeMath<M>(ethers::contract::Contract<M>);
    impl<M> Clone for SafeMath<M> {
        fn clone(&self) -> Self {
            SafeMath(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for SafeMath<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for SafeMath<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(SafeMath))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> SafeMath<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), SAFEMATH_ABI.clone(), client).into()
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for SafeMath<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
}
