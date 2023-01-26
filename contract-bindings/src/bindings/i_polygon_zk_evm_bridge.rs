pub use i_polygon_zk_evm_bridge::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_polygon_zk_evm_bridge {
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
    #[doc = "IPolygonZkEVMBridge was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"activateEmergencyState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"permitData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"bridgeAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"bridgeMessage\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"smtProof\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"mainnetExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"rollupExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"smtProof\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"mainnetExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"rollupExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimMessage\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deactivateEmergencyState\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IPOLYGONZKEVMBRIDGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IPolygonZkEVMBridge<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IPolygonZkEVMBridge<M> {
        fn clone(&self) -> Self {
            IPolygonZkEVMBridge(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IPolygonZkEVMBridge<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IPolygonZkEVMBridge<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPolygonZkEVMBridge))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IPolygonZkEVMBridge<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IPOLYGONZKEVMBRIDGE_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `activateEmergencyState` (0x2072f6c5) function"]
        pub fn activate_emergency_state(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 114, 246, 197], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bridgeAsset` (0x0871e971) function"]
        pub fn bridge_asset(
            &self,
            token: ethers::core::types::Address,
            destination_network: u32,
            destination_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            permit_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [8, 113, 233, 113],
                    (
                        token,
                        destination_network,
                        destination_address,
                        amount,
                        permit_data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bridgeMessage` (0xd96a15f7) function"]
        pub fn bridge_message(
            &self,
            destination_network: u32,
            destination_address: ethers::core::types::Address,
            metadata: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [217, 106, 21, 247],
                    (destination_network, destination_address, metadata),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimAsset` (0x7b6323c1) function"]
        pub fn claim_asset(
            &self,
            smt_proof: ::std::vec::Vec<[u8; 32]>,
            index: u32,
            mainnet_exit_root: [u8; 32],
            rollup_exit_root: [u8; 32],
            origin_network: u32,
            origin_token_address: ethers::core::types::Address,
            destination_network: u32,
            destination_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            metadata: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [123, 99, 35, 193],
                    (
                        smt_proof,
                        index,
                        mainnet_exit_root,
                        rollup_exit_root,
                        origin_network,
                        origin_token_address,
                        destination_network,
                        destination_address,
                        amount,
                        metadata,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimMessage` (0x46385549) function"]
        pub fn claim_message(
            &self,
            smt_proof: ::std::vec::Vec<[u8; 32]>,
            index: u32,
            mainnet_exit_root: [u8; 32],
            rollup_exit_root: [u8; 32],
            origin_network: u32,
            origin_address: ethers::core::types::Address,
            destination_network: u32,
            destination_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            metadata: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [70, 56, 85, 73],
                    (
                        smt_proof,
                        index,
                        mainnet_exit_root,
                        rollup_exit_root,
                        origin_network,
                        origin_address,
                        destination_network,
                        destination_address,
                        amount,
                        metadata,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deactivateEmergencyState` (0xdbc16976) function"]
        pub fn deactivate_emergency_state(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 193, 105, 118], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IPolygonZkEVMBridge<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `activateEmergencyState` function with signature `activateEmergencyState()` and selector `[32, 114, 246, 197]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "activateEmergencyState", abi = "activateEmergencyState()")]
    pub struct ActivateEmergencyStateCall;
    #[doc = "Container type for all input parameters for the `bridgeAsset` function with signature `bridgeAsset(address,uint32,address,uint256,bytes)` and selector `[8, 113, 233, 113]`"]
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
        name = "bridgeAsset",
        abi = "bridgeAsset(address,uint32,address,uint256,bytes)"
    )]
    pub struct BridgeAssetCall {
        pub token: ethers::core::types::Address,
        pub destination_network: u32,
        pub destination_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub permit_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `bridgeMessage` function with signature `bridgeMessage(uint32,address,bytes)` and selector `[217, 106, 21, 247]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "bridgeMessage", abi = "bridgeMessage(uint32,address,bytes)")]
    pub struct BridgeMessageCall {
        pub destination_network: u32,
        pub destination_address: ethers::core::types::Address,
        pub metadata: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `claimAsset` function with signature `claimAsset(bytes32[],uint32,bytes32,bytes32,uint32,address,uint32,address,uint256,bytes)` and selector `[123, 99, 35, 193]`"]
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
        name = "claimAsset",
        abi = "claimAsset(bytes32[],uint32,bytes32,bytes32,uint32,address,uint32,address,uint256,bytes)"
    )]
    pub struct ClaimAssetCall {
        pub smt_proof: ::std::vec::Vec<[u8; 32]>,
        pub index: u32,
        pub mainnet_exit_root: [u8; 32],
        pub rollup_exit_root: [u8; 32],
        pub origin_network: u32,
        pub origin_token_address: ethers::core::types::Address,
        pub destination_network: u32,
        pub destination_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub metadata: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `claimMessage` function with signature `claimMessage(bytes32[],uint32,bytes32,bytes32,uint32,address,uint32,address,uint256,bytes)` and selector `[70, 56, 85, 73]`"]
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
        name = "claimMessage",
        abi = "claimMessage(bytes32[],uint32,bytes32,bytes32,uint32,address,uint32,address,uint256,bytes)"
    )]
    pub struct ClaimMessageCall {
        pub smt_proof: ::std::vec::Vec<[u8; 32]>,
        pub index: u32,
        pub mainnet_exit_root: [u8; 32],
        pub rollup_exit_root: [u8; 32],
        pub origin_network: u32,
        pub origin_address: ethers::core::types::Address,
        pub destination_network: u32,
        pub destination_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub metadata: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `deactivateEmergencyState` function with signature `deactivateEmergencyState()` and selector `[219, 193, 105, 118]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deactivateEmergencyState", abi = "deactivateEmergencyState()")]
    pub struct DeactivateEmergencyStateCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPolygonZkEVMBridgeCalls {
        ActivateEmergencyState(ActivateEmergencyStateCall),
        BridgeAsset(BridgeAssetCall),
        BridgeMessage(BridgeMessageCall),
        ClaimAsset(ClaimAssetCall),
        ClaimMessage(ClaimMessageCall),
        DeactivateEmergencyState(DeactivateEmergencyStateCall),
    }
    impl ethers::core::abi::AbiDecode for IPolygonZkEVMBridgeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ActivateEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMBridgeCalls::ActivateEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <BridgeAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMBridgeCalls::BridgeAsset(decoded));
            }
            if let Ok(decoded) =
                <BridgeMessageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMBridgeCalls::BridgeMessage(decoded));
            }
            if let Ok(decoded) =
                <ClaimAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMBridgeCalls::ClaimAsset(decoded));
            }
            if let Ok(decoded) =
                <ClaimMessageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMBridgeCalls::ClaimMessage(decoded));
            }
            if let Ok(decoded) =
                <DeactivateEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPolygonZkEVMBridgeCalls::DeactivateEmergencyState(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPolygonZkEVMBridgeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPolygonZkEVMBridgeCalls::ActivateEmergencyState(element) => element.encode(),
                IPolygonZkEVMBridgeCalls::BridgeAsset(element) => element.encode(),
                IPolygonZkEVMBridgeCalls::BridgeMessage(element) => element.encode(),
                IPolygonZkEVMBridgeCalls::ClaimAsset(element) => element.encode(),
                IPolygonZkEVMBridgeCalls::ClaimMessage(element) => element.encode(),
                IPolygonZkEVMBridgeCalls::DeactivateEmergencyState(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPolygonZkEVMBridgeCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPolygonZkEVMBridgeCalls::ActivateEmergencyState(element) => element.fmt(f),
                IPolygonZkEVMBridgeCalls::BridgeAsset(element) => element.fmt(f),
                IPolygonZkEVMBridgeCalls::BridgeMessage(element) => element.fmt(f),
                IPolygonZkEVMBridgeCalls::ClaimAsset(element) => element.fmt(f),
                IPolygonZkEVMBridgeCalls::ClaimMessage(element) => element.fmt(f),
                IPolygonZkEVMBridgeCalls::DeactivateEmergencyState(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ActivateEmergencyStateCall> for IPolygonZkEVMBridgeCalls {
        fn from(var: ActivateEmergencyStateCall) -> Self {
            IPolygonZkEVMBridgeCalls::ActivateEmergencyState(var)
        }
    }
    impl ::std::convert::From<BridgeAssetCall> for IPolygonZkEVMBridgeCalls {
        fn from(var: BridgeAssetCall) -> Self {
            IPolygonZkEVMBridgeCalls::BridgeAsset(var)
        }
    }
    impl ::std::convert::From<BridgeMessageCall> for IPolygonZkEVMBridgeCalls {
        fn from(var: BridgeMessageCall) -> Self {
            IPolygonZkEVMBridgeCalls::BridgeMessage(var)
        }
    }
    impl ::std::convert::From<ClaimAssetCall> for IPolygonZkEVMBridgeCalls {
        fn from(var: ClaimAssetCall) -> Self {
            IPolygonZkEVMBridgeCalls::ClaimAsset(var)
        }
    }
    impl ::std::convert::From<ClaimMessageCall> for IPolygonZkEVMBridgeCalls {
        fn from(var: ClaimMessageCall) -> Self {
            IPolygonZkEVMBridgeCalls::ClaimMessage(var)
        }
    }
    impl ::std::convert::From<DeactivateEmergencyStateCall> for IPolygonZkEVMBridgeCalls {
        fn from(var: DeactivateEmergencyStateCall) -> Self {
            IPolygonZkEVMBridgeCalls::DeactivateEmergencyState(var)
        }
    }
}
