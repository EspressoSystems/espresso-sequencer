pub use polygon_zk_evm_global_exit_root_l2::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod polygon_zk_evm_global_exit_root_l2 {
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
    #[doc = "PolygonZkEVMGlobalExitRootL2 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_bridgeAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyAllowedContracts\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bridgeAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"globalExitRootMap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastRollupExitRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"newRoot\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateExitRoot\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static POLYGONZKEVMGLOBALEXITROOTL2_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct PolygonZkEVMGlobalExitRootL2<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PolygonZkEVMGlobalExitRootL2<M> {
        fn clone(&self) -> Self {
            PolygonZkEVMGlobalExitRootL2(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PolygonZkEVMGlobalExitRootL2<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PolygonZkEVMGlobalExitRootL2<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PolygonZkEVMGlobalExitRootL2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PolygonZkEVMGlobalExitRootL2<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                POLYGONZKEVMGLOBALEXITROOTL2_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `bridgeAddress` (0xa3c573eb) function"]
        pub fn bridge_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([163, 197, 115, 235], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `globalExitRootMap` (0x257b3632) function"]
        pub fn global_exit_root_map(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([37, 123, 54, 50], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastRollupExitRoot` (0x01fd9044) function"]
        pub fn last_rollup_exit_root(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([1, 253, 144, 68], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateExitRoot` (0x33d6247d) function"]
        pub fn update_exit_root(
            &self,
            new_root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 214, 36, 125], new_root)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for PolygonZkEVMGlobalExitRootL2<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `OnlyAllowedContracts` with signature `OnlyAllowedContracts()` and selector `[180, 147, 101, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyAllowedContracts", abi = "OnlyAllowedContracts()")]
    pub struct OnlyAllowedContracts;
    #[doc = "Container type for all input parameters for the `bridgeAddress` function with signature `bridgeAddress()` and selector `[163, 197, 115, 235]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "bridgeAddress", abi = "bridgeAddress()")]
    pub struct BridgeAddressCall;
    #[doc = "Container type for all input parameters for the `globalExitRootMap` function with signature `globalExitRootMap(bytes32)` and selector `[37, 123, 54, 50]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "globalExitRootMap", abi = "globalExitRootMap(bytes32)")]
    pub struct GlobalExitRootMapCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `lastRollupExitRoot` function with signature `lastRollupExitRoot()` and selector `[1, 253, 144, 68]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastRollupExitRoot", abi = "lastRollupExitRoot()")]
    pub struct LastRollupExitRootCall;
    #[doc = "Container type for all input parameters for the `updateExitRoot` function with signature `updateExitRoot(bytes32)` and selector `[51, 214, 36, 125]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "updateExitRoot", abi = "updateExitRoot(bytes32)")]
    pub struct UpdateExitRootCall {
        pub new_root: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PolygonZkEVMGlobalExitRootL2Calls {
        BridgeAddress(BridgeAddressCall),
        GlobalExitRootMap(GlobalExitRootMapCall),
        LastRollupExitRoot(LastRollupExitRootCall),
        UpdateExitRoot(UpdateExitRootCall),
    }
    impl ethers::core::abi::AbiDecode for PolygonZkEVMGlobalExitRootL2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BridgeAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootL2Calls::BridgeAddress(decoded));
            }
            if let Ok(decoded) =
                <GlobalExitRootMapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootL2Calls::GlobalExitRootMap(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LastRollupExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootL2Calls::LastRollupExitRoot(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UpdateExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootL2Calls::UpdateExitRoot(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PolygonZkEVMGlobalExitRootL2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                PolygonZkEVMGlobalExitRootL2Calls::BridgeAddress(element) => element.encode(),
                PolygonZkEVMGlobalExitRootL2Calls::GlobalExitRootMap(element) => element.encode(),
                PolygonZkEVMGlobalExitRootL2Calls::LastRollupExitRoot(element) => element.encode(),
                PolygonZkEVMGlobalExitRootL2Calls::UpdateExitRoot(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMGlobalExitRootL2Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMGlobalExitRootL2Calls::BridgeAddress(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootL2Calls::GlobalExitRootMap(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootL2Calls::LastRollupExitRoot(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootL2Calls::UpdateExitRoot(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BridgeAddressCall> for PolygonZkEVMGlobalExitRootL2Calls {
        fn from(var: BridgeAddressCall) -> Self {
            PolygonZkEVMGlobalExitRootL2Calls::BridgeAddress(var)
        }
    }
    impl ::std::convert::From<GlobalExitRootMapCall> for PolygonZkEVMGlobalExitRootL2Calls {
        fn from(var: GlobalExitRootMapCall) -> Self {
            PolygonZkEVMGlobalExitRootL2Calls::GlobalExitRootMap(var)
        }
    }
    impl ::std::convert::From<LastRollupExitRootCall> for PolygonZkEVMGlobalExitRootL2Calls {
        fn from(var: LastRollupExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootL2Calls::LastRollupExitRoot(var)
        }
    }
    impl ::std::convert::From<UpdateExitRootCall> for PolygonZkEVMGlobalExitRootL2Calls {
        fn from(var: UpdateExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootL2Calls::UpdateExitRoot(var)
        }
    }
    #[doc = "Container type for all return fields from the `bridgeAddress` function with signature `bridgeAddress()` and selector `[163, 197, 115, 235]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BridgeAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `globalExitRootMap` function with signature `globalExitRootMap(bytes32)` and selector `[37, 123, 54, 50]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GlobalExitRootMapReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `lastRollupExitRoot` function with signature `lastRollupExitRoot()` and selector `[1, 253, 144, 68]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastRollupExitRootReturn(pub [u8; 32]);
}
