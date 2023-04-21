pub use example_rollup::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod example_rollup {
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
    #[doc = "ExampleRollup was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"hotshotAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockHeight\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidProof\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockHeight\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"StateUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nextStateCommitment\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"newBlock\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"stateCommitment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verifiedBlocks\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static EXAMPLEROLLUP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static EXAMPLEROLLUP_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506040516102ea3803806102ea83398101604081905261002f9161005c565b600080546001600160a01b0319166001600160a01b0392909216919091178155600181905560025561008c565b60006020828403121561006e57600080fd5b81516001600160a01b038116811461008557600080fd5b9392505050565b61024f8061009b6000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c806378178c8a146100465780638498067e1461005b578063d800741e14610076575b600080fd5b61005961005436600461015d565b61007f565b005b61006460025481565b60405190815260200160405180910390f35b61006460015481565b600080546002546001600160a01b03909116906349ce8997906100a39060016101d9565b6040518263ffffffff1660e01b81526004016100c191815260200190565b602060405180830381865afa1580156100de573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101029190610200565b905060016002600082825461011791906101d9565b909155505060018490556002546040519081527f960805e7dfc5cc387e0db0b8f6b4a6a3fafbe87a9e0669d505558889762b00b39060200160405180910390a150505050565b60008060006040848603121561017257600080fd5b83359250602084013567ffffffffffffffff8082111561019157600080fd5b818601915086601f8301126101a557600080fd5b8135818111156101b457600080fd5b8760208285010111156101c657600080fd5b6020830194508093505050509250925092565b808201808211156101fa57634e487b7160e01b600052601160045260246000fd5b92915050565b60006020828403121561021257600080fd5b505191905056fea26469706673582212208388fa5056e8b26d5ee17da7283788e51ff9a7b511d288746e150964f797abf764736f6c63430008120033" . parse () . expect ("invalid bytecode")
        });
    pub struct ExampleRollup<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ExampleRollup<M> {
        fn clone(&self) -> Self {
            ExampleRollup(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ExampleRollup<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ExampleRollup<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ExampleRollup))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ExampleRollup<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), EXAMPLEROLLUP_ABI.clone(), client)
                .into()
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
                EXAMPLEROLLUP_ABI.clone(),
                EXAMPLEROLLUP_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `newBlock` (0x78178c8a) function"]
        pub fn new_block(
            &self,
            next_state_commitment: ethers::core::types::U256,
            proof: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 23, 140, 138], (next_state_commitment, proof))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stateCommitment` (0xd800741e) function"]
        pub fn state_commitment(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([216, 0, 116, 30], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifiedBlocks` (0x8498067e) function"]
        pub fn verified_blocks(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([132, 152, 6, 126], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `StateUpdate` event"]
        pub fn state_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StateUpdateFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, StateUpdateFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ExampleRollup<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `InvalidProof` with signature `InvalidProof(uint256,bytes)` and selector `[128, 107, 179, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidProof", abi = "InvalidProof(uint256,bytes)")]
    pub struct InvalidProof {
        pub block_height: ethers::core::types::U256,
        pub proof: ethers::core::types::Bytes,
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
    #[ethevent(name = "StateUpdate", abi = "StateUpdate(uint256)")]
    pub struct StateUpdateFilter {
        pub block_height: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `newBlock` function with signature `newBlock(uint256,bytes)` and selector `[120, 23, 140, 138]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "newBlock", abi = "newBlock(uint256,bytes)")]
    pub struct NewBlockCall {
        pub next_state_commitment: ethers::core::types::U256,
        pub proof: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `stateCommitment` function with signature `stateCommitment()` and selector `[216, 0, 116, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "stateCommitment", abi = "stateCommitment()")]
    pub struct StateCommitmentCall;
    #[doc = "Container type for all input parameters for the `verifiedBlocks` function with signature `verifiedBlocks()` and selector `[132, 152, 6, 126]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "verifiedBlocks", abi = "verifiedBlocks()")]
    pub struct VerifiedBlocksCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ExampleRollupCalls {
        NewBlock(NewBlockCall),
        StateCommitment(StateCommitmentCall),
        VerifiedBlocks(VerifiedBlocksCall),
    }
    impl ethers::core::abi::AbiDecode for ExampleRollupCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <NewBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupCalls::NewBlock(decoded));
            }
            if let Ok(decoded) =
                <StateCommitmentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupCalls::StateCommitment(decoded));
            }
            if let Ok(decoded) =
                <VerifiedBlocksCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupCalls::VerifiedBlocks(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ExampleRollupCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ExampleRollupCalls::NewBlock(element) => element.encode(),
                ExampleRollupCalls::StateCommitment(element) => element.encode(),
                ExampleRollupCalls::VerifiedBlocks(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ExampleRollupCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ExampleRollupCalls::NewBlock(element) => element.fmt(f),
                ExampleRollupCalls::StateCommitment(element) => element.fmt(f),
                ExampleRollupCalls::VerifiedBlocks(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<NewBlockCall> for ExampleRollupCalls {
        fn from(var: NewBlockCall) -> Self {
            ExampleRollupCalls::NewBlock(var)
        }
    }
    impl ::std::convert::From<StateCommitmentCall> for ExampleRollupCalls {
        fn from(var: StateCommitmentCall) -> Self {
            ExampleRollupCalls::StateCommitment(var)
        }
    }
    impl ::std::convert::From<VerifiedBlocksCall> for ExampleRollupCalls {
        fn from(var: VerifiedBlocksCall) -> Self {
            ExampleRollupCalls::VerifiedBlocks(var)
        }
    }
    #[doc = "Container type for all return fields from the `stateCommitment` function with signature `stateCommitment()` and selector `[216, 0, 116, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct StateCommitmentReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `verifiedBlocks` function with signature `verifiedBlocks()` and selector `[132, 152, 6, 126]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VerifiedBlocksReturn(pub ethers::core::types::U256);
}
