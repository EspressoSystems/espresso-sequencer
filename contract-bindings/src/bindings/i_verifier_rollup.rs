pub use i_verifier_rollup::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_verifier_rollup {
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
    #[doc = "IVerifierRollup was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"proofA\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2][2]\",\"name\":\"proofB\",\"type\":\"uint256[2][2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofC\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[1]\",\"name\":\"input\",\"type\":\"uint256[1]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verifyProof\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IVERIFIERROLLUP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IVerifierRollup<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IVerifierRollup<M> {
        fn clone(&self) -> Self {
            IVerifierRollup(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IVerifierRollup<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IVerifierRollup<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IVerifierRollup))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IVerifierRollup<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IVERIFIERROLLUP_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `verifyProof` (0x43753b4d) function"]
        pub fn verify_proof(
            &self,
            proof_a: [ethers::core::types::U256; 2usize],
            proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
            proof_c: [ethers::core::types::U256; 2usize],
            input: [ethers::core::types::U256; 1usize],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([67, 117, 59, 77], (proof_a, proof_b, proof_c, input))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IVerifierRollup<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `verifyProof` function with signature `verifyProof(uint256[2],uint256[2][2],uint256[2],uint256[1])` and selector `[67, 117, 59, 77]`"]
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
        name = "verifyProof",
        abi = "verifyProof(uint256[2],uint256[2][2],uint256[2],uint256[1])"
    )]
    pub struct VerifyProofCall {
        pub proof_a: [ethers::core::types::U256; 2usize],
        pub proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
        pub proof_c: [ethers::core::types::U256; 2usize],
        pub input: [ethers::core::types::U256; 1usize],
    }
    #[doc = "Container type for all return fields from the `verifyProof` function with signature `verifyProof(uint256[2],uint256[2][2],uint256[2],uint256[1])` and selector `[67, 117, 59, 77]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VerifyProofReturn(pub bool);
}
