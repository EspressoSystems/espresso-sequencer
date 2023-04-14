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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"hotshotAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockHeight\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"IncorrectBlockHeight\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockHeight\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"proof\",\"type\":\"bytes[]\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidProof\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockHeight\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"StateUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"blockHeight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nextStateCommitment\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nextBlockHeight\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"proof\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"newBlock\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"stateCommitment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static EXAMPLEROLLUP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static EXAMPLEROLLUP_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506040516103a23803806103a283398101604081905261002f9161005c565b600080546001600160a01b0319166001600160a01b0392909216919091178155600181905560025561008c565b60006020828403121561006e57600080fd5b81516001600160a01b038116811461008557600080fd5b9392505050565b6103078061009b6000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c806397d039e314610046578063d800741e1461005b578063f44ff71214610076575b600080fd5b610059610054366004610167565b61007f565b005b61006460015481565b60405190815260200160405180910390f35b61006460025481565b60025461008d9060016101ea565b83146100b357604051631b00d85560e31b81526004810184905260240160405180910390fd5b600080546040516349ce899760e01b8152600481018690526001600160a01b03909116906349ce899790602401602060405180830381865afa1580156100fd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101219190610211565b9050600284905560018590556040518481527f960805e7dfc5cc387e0db0b8f6b4a6a3fafbe87a9e0669d505558889762b00b39060200160405180910390a15050505050565b6000806000806060858703121561017d57600080fd5b8435935060208501359250604085013567ffffffffffffffff808211156101a357600080fd5b818701915087601f8301126101b757600080fd5b8135818111156101c657600080fd5b8860208260051b85010111156101db57600080fd5b95989497505060200194505050565b8082018082111561020b57634e487b7160e01b600052601160045260246000fd5b92915050565b60006020828403121561022357600080fd5b5051919050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b878110156102c357868503605f190183528135368a9003601e1901811261027957600080fd5b8901848101903567ffffffffffffffff81111561029557600080fd5b8036038213156102a457600080fd5b6102af87828461022a565b965050509183019190830190600101610253565b50929897505050505050505056fea26469706673582212206ada3ef129a16737ffeefff333dd9e55a0c0c6b76eacfad1703985d4a59d882464736f6c63430008120033" . parse () . expect ("invalid bytecode")
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
        #[doc = "Calls the contract's `blockHeight` (0xf44ff712) function"]
        pub fn block_height(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([244, 79, 247, 18], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `newBlock` (0x97d039e3) function"]
        pub fn new_block(
            &self,
            next_state_commitment: ethers::core::types::U256,
            next_block_height: ethers::core::types::U256,
            proof: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [151, 208, 57, 227],
                    (next_state_commitment, next_block_height, proof),
                )
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
    #[doc = "Custom Error type `IncorrectBlockHeight` with signature `IncorrectBlockHeight(uint256)` and selector `[216, 6, 194, 168]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "IncorrectBlockHeight", abi = "IncorrectBlockHeight(uint256)")]
    pub struct IncorrectBlockHeight {
        pub block_height: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `InvalidProof` with signature `InvalidProof(uint256,bytes[])` and selector `[145, 180, 189, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidProof", abi = "InvalidProof(uint256,bytes[])")]
    pub struct InvalidProof {
        pub block_height: ethers::core::types::U256,
        pub proof: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ExampleRollupErrors {
        IncorrectBlockHeight(IncorrectBlockHeight),
        InvalidProof(InvalidProof),
    }
    impl ethers::core::abi::AbiDecode for ExampleRollupErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IncorrectBlockHeight as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupErrors::IncorrectBlockHeight(decoded));
            }
            if let Ok(decoded) =
                <InvalidProof as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupErrors::InvalidProof(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ExampleRollupErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                ExampleRollupErrors::IncorrectBlockHeight(element) => element.encode(),
                ExampleRollupErrors::InvalidProof(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ExampleRollupErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ExampleRollupErrors::IncorrectBlockHeight(element) => element.fmt(f),
                ExampleRollupErrors::InvalidProof(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IncorrectBlockHeight> for ExampleRollupErrors {
        fn from(var: IncorrectBlockHeight) -> Self {
            ExampleRollupErrors::IncorrectBlockHeight(var)
        }
    }
    impl ::std::convert::From<InvalidProof> for ExampleRollupErrors {
        fn from(var: InvalidProof) -> Self {
            ExampleRollupErrors::InvalidProof(var)
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
    #[ethevent(name = "StateUpdate", abi = "StateUpdate(uint256)")]
    pub struct StateUpdateFilter {
        pub block_height: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `blockHeight` function with signature `blockHeight()` and selector `[244, 79, 247, 18]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "blockHeight", abi = "blockHeight()")]
    pub struct BlockHeightCall;
    #[doc = "Container type for all input parameters for the `newBlock` function with signature `newBlock(uint256,uint256,bytes[])` and selector `[151, 208, 57, 227]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "newBlock", abi = "newBlock(uint256,uint256,bytes[])")]
    pub struct NewBlockCall {
        pub next_state_commitment: ethers::core::types::U256,
        pub next_block_height: ethers::core::types::U256,
        pub proof: ::std::vec::Vec<ethers::core::types::Bytes>,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ExampleRollupCalls {
        BlockHeight(BlockHeightCall),
        NewBlock(NewBlockCall),
        StateCommitment(StateCommitmentCall),
    }
    impl ethers::core::abi::AbiDecode for ExampleRollupCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BlockHeightCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupCalls::BlockHeight(decoded));
            }
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
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ExampleRollupCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ExampleRollupCalls::BlockHeight(element) => element.encode(),
                ExampleRollupCalls::NewBlock(element) => element.encode(),
                ExampleRollupCalls::StateCommitment(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ExampleRollupCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ExampleRollupCalls::BlockHeight(element) => element.fmt(f),
                ExampleRollupCalls::NewBlock(element) => element.fmt(f),
                ExampleRollupCalls::StateCommitment(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BlockHeightCall> for ExampleRollupCalls {
        fn from(var: BlockHeightCall) -> Self {
            ExampleRollupCalls::BlockHeight(var)
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
    #[doc = "Container type for all return fields from the `blockHeight` function with signature `blockHeight()` and selector `[244, 79, 247, 18]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BlockHeightReturn(pub ethers::core::types::U256);
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
}
