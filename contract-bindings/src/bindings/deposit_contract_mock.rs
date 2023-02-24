pub use deposit_contract_mock::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod deposit_contract_mock {
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
    #[doc = "DepositContractMock was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MerkleTreeFull\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"leafType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"metadataHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"depositCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDepositRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"leafType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"metadataHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getLeafValue\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"leafHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32[32]\",\"name\":\"smtProof\",\"type\":\"bytes32[32]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"verifyMerkleProof\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static DEPOSITCONTRACTMOCK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct DepositContractMock<M>(ethers::contract::Contract<M>);
    impl<M> Clone for DepositContractMock<M> {
        fn clone(&self) -> Self {
            DepositContractMock(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for DepositContractMock<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for DepositContractMock<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DepositContractMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> DepositContractMock<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), DEPOSITCONTRACTMOCK_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `deposit` (0x82c01902) function"]
        pub fn deposit(
            &self,
            leaf_type: u8,
            origin_network: u32,
            origin_token_address: ethers::core::types::Address,
            destination_network: u32,
            destination_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            metadata_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [130, 192, 25, 2],
                    (
                        leaf_type,
                        origin_network,
                        origin_token_address,
                        destination_network,
                        destination_address,
                        amount,
                        metadata_hash,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositCount` (0x2dfdf0b5) function"]
        pub fn deposit_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([45, 253, 240, 181], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDepositRoot` (0x3ae05047) function"]
        pub fn get_deposit_root(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([58, 224, 80, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLeafValue` (0x3e197043) function"]
        pub fn get_leaf_value(
            &self,
            leaf_type: u8,
            origin_network: u32,
            origin_address: ethers::core::types::Address,
            destination_network: u32,
            destination_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            metadata_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [62, 25, 112, 67],
                    (
                        leaf_type,
                        origin_network,
                        origin_address,
                        destination_network,
                        destination_address,
                        amount,
                        metadata_hash,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x8129fc1c) function"]
        pub fn initialize(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyMerkleProof` (0xfb570834) function"]
        pub fn verify_merkle_proof(
            &self,
            leaf_hash: [u8; 32],
            smt_proof: [[u8; 32]; 32usize],
            index: u32,
            root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([251, 87, 8, 52], (leaf_hash, smt_proof, index, root))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for DepositContractMock<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `MerkleTreeFull` with signature `MerkleTreeFull()` and selector `[239, 92, 207, 102]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "MerkleTreeFull", abi = "MerkleTreeFull()")]
    pub struct MerkleTreeFull;
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[doc = "Container type for all input parameters for the `deposit` function with signature `deposit(uint8,uint32,address,uint32,address,uint256,bytes32)` and selector `[130, 192, 25, 2]`"]
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
        name = "deposit",
        abi = "deposit(uint8,uint32,address,uint32,address,uint256,bytes32)"
    )]
    pub struct DepositCall {
        pub leaf_type: u8,
        pub origin_network: u32,
        pub origin_token_address: ethers::core::types::Address,
        pub destination_network: u32,
        pub destination_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub metadata_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `depositCount` function with signature `depositCount()` and selector `[45, 253, 240, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "depositCount", abi = "depositCount()")]
    pub struct DepositCountCall;
    #[doc = "Container type for all input parameters for the `getDepositRoot` function with signature `getDepositRoot()` and selector `[58, 224, 80, 71]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getDepositRoot", abi = "getDepositRoot()")]
    pub struct GetDepositRootCall;
    #[doc = "Container type for all input parameters for the `getLeafValue` function with signature `getLeafValue(uint8,uint32,address,uint32,address,uint256,bytes32)` and selector `[62, 25, 112, 67]`"]
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
        name = "getLeafValue",
        abi = "getLeafValue(uint8,uint32,address,uint32,address,uint256,bytes32)"
    )]
    pub struct GetLeafValueCall {
        pub leaf_type: u8,
        pub origin_network: u32,
        pub origin_address: ethers::core::types::Address,
        pub destination_network: u32,
        pub destination_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub metadata_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize()` and selector `[129, 41, 252, 28]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
    #[doc = "Container type for all input parameters for the `verifyMerkleProof` function with signature `verifyMerkleProof(bytes32,bytes32[32],uint32,bytes32)` and selector `[251, 87, 8, 52]`"]
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
        name = "verifyMerkleProof",
        abi = "verifyMerkleProof(bytes32,bytes32[32],uint32,bytes32)"
    )]
    pub struct VerifyMerkleProofCall {
        pub leaf_hash: [u8; 32],
        pub smt_proof: [[u8; 32]; 32usize],
        pub index: u32,
        pub root: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum DepositContractMockCalls {
        Deposit(DepositCall),
        DepositCount(DepositCountCall),
        GetDepositRoot(GetDepositRootCall),
        GetLeafValue(GetLeafValueCall),
        Initialize(InitializeCall),
        VerifyMerkleProof(VerifyMerkleProofCall),
    }
    impl ethers::core::abi::AbiDecode for DepositContractMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DepositContractMockCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DepositCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DepositContractMockCalls::DepositCount(decoded));
            }
            if let Ok(decoded) =
                <GetDepositRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DepositContractMockCalls::GetDepositRoot(decoded));
            }
            if let Ok(decoded) =
                <GetLeafValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DepositContractMockCalls::GetLeafValue(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DepositContractMockCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <VerifyMerkleProofCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DepositContractMockCalls::VerifyMerkleProof(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for DepositContractMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                DepositContractMockCalls::Deposit(element) => element.encode(),
                DepositContractMockCalls::DepositCount(element) => element.encode(),
                DepositContractMockCalls::GetDepositRoot(element) => element.encode(),
                DepositContractMockCalls::GetLeafValue(element) => element.encode(),
                DepositContractMockCalls::Initialize(element) => element.encode(),
                DepositContractMockCalls::VerifyMerkleProof(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DepositContractMockCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DepositContractMockCalls::Deposit(element) => element.fmt(f),
                DepositContractMockCalls::DepositCount(element) => element.fmt(f),
                DepositContractMockCalls::GetDepositRoot(element) => element.fmt(f),
                DepositContractMockCalls::GetLeafValue(element) => element.fmt(f),
                DepositContractMockCalls::Initialize(element) => element.fmt(f),
                DepositContractMockCalls::VerifyMerkleProof(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DepositCall> for DepositContractMockCalls {
        fn from(var: DepositCall) -> Self {
            DepositContractMockCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<DepositCountCall> for DepositContractMockCalls {
        fn from(var: DepositCountCall) -> Self {
            DepositContractMockCalls::DepositCount(var)
        }
    }
    impl ::std::convert::From<GetDepositRootCall> for DepositContractMockCalls {
        fn from(var: GetDepositRootCall) -> Self {
            DepositContractMockCalls::GetDepositRoot(var)
        }
    }
    impl ::std::convert::From<GetLeafValueCall> for DepositContractMockCalls {
        fn from(var: GetLeafValueCall) -> Self {
            DepositContractMockCalls::GetLeafValue(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for DepositContractMockCalls {
        fn from(var: InitializeCall) -> Self {
            DepositContractMockCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<VerifyMerkleProofCall> for DepositContractMockCalls {
        fn from(var: VerifyMerkleProofCall) -> Self {
            DepositContractMockCalls::VerifyMerkleProof(var)
        }
    }
    #[doc = "Container type for all return fields from the `depositCount` function with signature `depositCount()` and selector `[45, 253, 240, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DepositCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getDepositRoot` function with signature `getDepositRoot()` and selector `[58, 224, 80, 71]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetDepositRootReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getLeafValue` function with signature `getLeafValue(uint8,uint32,address,uint32,address,uint256,bytes32)` and selector `[62, 25, 112, 67]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLeafValueReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `verifyMerkleProof` function with signature `verifyMerkleProof(bytes32,bytes32[32],uint32,bytes32)` and selector `[251, 87, 8, 52]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VerifyMerkleProofReturn(pub bool);
}
