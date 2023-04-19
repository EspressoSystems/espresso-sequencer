pub use bn254::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod bn254 {
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
    #[doc = "BN254 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"P_MOD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"R_MOD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static BN254_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BN254_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60d6610039600b82828239805160001a60731461002c57634e487b7160e01b600052600060045260246000fd5b30600052607381538281f3fe7300000000000000000000000000000000000000003014608060405260043610603d5760003560e01c80631d712e27146042578063df6e6cb414607a575b600080fd5b60687f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4781565b60405190815260200160405180910390f35b60687f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f00000018156fea2646970667358221220fb4ca027b589ab58956bb0f0e4ad96570025a2cbbcfe90a268e09d3531561de264736f6c63430008120033" . parse () . expect ("invalid bytecode")
        });
    pub struct BN254<M>(ethers::contract::Contract<M>);
    impl<M> Clone for BN254<M> {
        fn clone(&self) -> Self {
            BN254(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for BN254<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for BN254<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BN254))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> BN254<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BN254_ABI.clone(), client).into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                BN254_ABI.clone(),
                BN254_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `P_MOD` (0x1d712e27) function"]
        pub fn p_mod(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([29, 113, 46, 39], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `R_MOD` (0xdf6e6cb4) function"]
        pub fn r_mod(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([223, 110, 108, 180], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for BN254<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `P_MOD` function with signature `P_MOD()` and selector `[29, 113, 46, 39]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "P_MOD", abi = "P_MOD()")]
    pub struct PModCall;
    #[doc = "Container type for all input parameters for the `R_MOD` function with signature `R_MOD()` and selector `[223, 110, 108, 180]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "R_MOD", abi = "R_MOD()")]
    pub struct RModCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BN254Calls {
        PMod(PModCall),
        RMod(RModCall),
    }
    impl ethers::core::abi::AbiDecode for BN254Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <PModCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(BN254Calls::PMod(decoded));
            }
            if let Ok(decoded) = <RModCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(BN254Calls::RMod(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BN254Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                BN254Calls::PMod(element) => element.encode(),
                BN254Calls::RMod(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BN254Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BN254Calls::PMod(element) => element.fmt(f),
                BN254Calls::RMod(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<PModCall> for BN254Calls {
        fn from(var: PModCall) -> Self {
            BN254Calls::PMod(var)
        }
    }
    impl ::std::convert::From<RModCall> for BN254Calls {
        fn from(var: RModCall) -> Self {
            BN254Calls::RMod(var)
        }
    }
    #[doc = "Container type for all return fields from the `P_MOD` function with signature `P_MOD()` and selector `[29, 113, 46, 39]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PModReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `R_MOD` function with signature `R_MOD()` and selector `[223, 110, 108, 180]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RModReturn(pub ethers::core::types::U256);
}
