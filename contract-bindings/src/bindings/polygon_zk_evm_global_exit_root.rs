pub use polygon_zk_evm_global_exit_root::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod polygon_zk_evm_global_exit_root {
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
    #[doc = "PolygonZkEVMGlobalExitRoot was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"mainnetExitRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"rollupExitRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UpdateGlobalExitRoot\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bridgeAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastGlobalExitRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"globalExitRootMap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_rollupAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_bridgeAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastMainnetExitRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastRollupExitRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rollupAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"newRoot\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateExitRoot\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static POLYGONZKEVMGLOBALEXITROOT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct PolygonZkEVMGlobalExitRoot<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PolygonZkEVMGlobalExitRoot<M> {
        fn clone(&self) -> Self {
            PolygonZkEVMGlobalExitRoot(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PolygonZkEVMGlobalExitRoot<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PolygonZkEVMGlobalExitRoot<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PolygonZkEVMGlobalExitRoot))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PolygonZkEVMGlobalExitRoot<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                POLYGONZKEVMGLOBALEXITROOT_ABI.clone(),
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
        #[doc = "Calls the contract's `getLastGlobalExitRoot` (0x3ed691ef) function"]
        pub fn get_last_global_exit_root(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([62, 214, 145, 239], ())
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
        #[doc = "Calls the contract's `initialize` (0x485cc955) function"]
        pub fn initialize(
            &self,
            rollup_address: ethers::core::types::Address,
            bridge_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (rollup_address, bridge_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastMainnetExitRoot` (0x319cf735) function"]
        pub fn last_mainnet_exit_root(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([49, 156, 247, 53], ())
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
        #[doc = "Calls the contract's `rollupAddress` (0x5ec6a8df) function"]
        pub fn rollup_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([94, 198, 168, 223], ())
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
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdateGlobalExitRoot` event"]
        pub fn update_global_exit_root_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpdateGlobalExitRootFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, PolygonZkEVMGlobalExitRootEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for PolygonZkEVMGlobalExitRoot<M>
    {
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
        name = "UpdateGlobalExitRoot",
        abi = "UpdateGlobalExitRoot(bytes32,bytes32)"
    )]
    pub struct UpdateGlobalExitRootFilter {
        #[ethevent(indexed)]
        pub mainnet_exit_root: [u8; 32],
        #[ethevent(indexed)]
        pub rollup_exit_root: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PolygonZkEVMGlobalExitRootEvents {
        InitializedFilter(InitializedFilter),
        UpdateGlobalExitRootFilter(UpdateGlobalExitRootFilter),
    }
    impl ethers::contract::EthLogDecode for PolygonZkEVMGlobalExitRootEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(PolygonZkEVMGlobalExitRootEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = UpdateGlobalExitRootFilter::decode_log(log) {
                return Ok(PolygonZkEVMGlobalExitRootEvents::UpdateGlobalExitRootFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMGlobalExitRootEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMGlobalExitRootEvents::InitializedFilter(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootEvents::UpdateGlobalExitRootFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
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
    #[doc = "Container type for all input parameters for the `getLastGlobalExitRoot` function with signature `getLastGlobalExitRoot()` and selector `[62, 214, 145, 239]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getLastGlobalExitRoot", abi = "getLastGlobalExitRoot()")]
    pub struct GetLastGlobalExitRootCall;
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
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `[72, 92, 201, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub rollup_address: ethers::core::types::Address,
        pub bridge_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `lastMainnetExitRoot` function with signature `lastMainnetExitRoot()` and selector `[49, 156, 247, 53]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastMainnetExitRoot", abi = "lastMainnetExitRoot()")]
    pub struct LastMainnetExitRootCall;
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
    #[doc = "Container type for all input parameters for the `rollupAddress` function with signature `rollupAddress()` and selector `[94, 198, 168, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "rollupAddress", abi = "rollupAddress()")]
    pub struct RollupAddressCall;
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
    pub enum PolygonZkEVMGlobalExitRootCalls {
        BridgeAddress(BridgeAddressCall),
        GetLastGlobalExitRoot(GetLastGlobalExitRootCall),
        GlobalExitRootMap(GlobalExitRootMapCall),
        Initialize(InitializeCall),
        LastMainnetExitRoot(LastMainnetExitRootCall),
        LastRollupExitRoot(LastRollupExitRootCall),
        RollupAddress(RollupAddressCall),
        UpdateExitRoot(UpdateExitRootCall),
    }
    impl ethers::core::abi::AbiDecode for PolygonZkEVMGlobalExitRootCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BridgeAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootCalls::BridgeAddress(decoded));
            }
            if let Ok(decoded) =
                <GetLastGlobalExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootCalls::GetLastGlobalExitRoot(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GlobalExitRootMapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootCalls::GlobalExitRootMap(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LastMainnetExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootCalls::LastMainnetExitRoot(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LastRollupExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootCalls::LastRollupExitRoot(decoded));
            }
            if let Ok(decoded) =
                <RollupAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootCalls::RollupAddress(decoded));
            }
            if let Ok(decoded) =
                <UpdateExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootCalls::UpdateExitRoot(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PolygonZkEVMGlobalExitRootCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PolygonZkEVMGlobalExitRootCalls::BridgeAddress(element) => element.encode(),
                PolygonZkEVMGlobalExitRootCalls::GetLastGlobalExitRoot(element) => element.encode(),
                PolygonZkEVMGlobalExitRootCalls::GlobalExitRootMap(element) => element.encode(),
                PolygonZkEVMGlobalExitRootCalls::Initialize(element) => element.encode(),
                PolygonZkEVMGlobalExitRootCalls::LastMainnetExitRoot(element) => element.encode(),
                PolygonZkEVMGlobalExitRootCalls::LastRollupExitRoot(element) => element.encode(),
                PolygonZkEVMGlobalExitRootCalls::RollupAddress(element) => element.encode(),
                PolygonZkEVMGlobalExitRootCalls::UpdateExitRoot(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMGlobalExitRootCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMGlobalExitRootCalls::BridgeAddress(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootCalls::GetLastGlobalExitRoot(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootCalls::GlobalExitRootMap(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootCalls::Initialize(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootCalls::LastMainnetExitRoot(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootCalls::LastRollupExitRoot(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootCalls::RollupAddress(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootCalls::UpdateExitRoot(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BridgeAddressCall> for PolygonZkEVMGlobalExitRootCalls {
        fn from(var: BridgeAddressCall) -> Self {
            PolygonZkEVMGlobalExitRootCalls::BridgeAddress(var)
        }
    }
    impl ::std::convert::From<GetLastGlobalExitRootCall> for PolygonZkEVMGlobalExitRootCalls {
        fn from(var: GetLastGlobalExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootCalls::GetLastGlobalExitRoot(var)
        }
    }
    impl ::std::convert::From<GlobalExitRootMapCall> for PolygonZkEVMGlobalExitRootCalls {
        fn from(var: GlobalExitRootMapCall) -> Self {
            PolygonZkEVMGlobalExitRootCalls::GlobalExitRootMap(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for PolygonZkEVMGlobalExitRootCalls {
        fn from(var: InitializeCall) -> Self {
            PolygonZkEVMGlobalExitRootCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<LastMainnetExitRootCall> for PolygonZkEVMGlobalExitRootCalls {
        fn from(var: LastMainnetExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootCalls::LastMainnetExitRoot(var)
        }
    }
    impl ::std::convert::From<LastRollupExitRootCall> for PolygonZkEVMGlobalExitRootCalls {
        fn from(var: LastRollupExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootCalls::LastRollupExitRoot(var)
        }
    }
    impl ::std::convert::From<RollupAddressCall> for PolygonZkEVMGlobalExitRootCalls {
        fn from(var: RollupAddressCall) -> Self {
            PolygonZkEVMGlobalExitRootCalls::RollupAddress(var)
        }
    }
    impl ::std::convert::From<UpdateExitRootCall> for PolygonZkEVMGlobalExitRootCalls {
        fn from(var: UpdateExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootCalls::UpdateExitRoot(var)
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
    #[doc = "Container type for all return fields from the `getLastGlobalExitRoot` function with signature `getLastGlobalExitRoot()` and selector `[62, 214, 145, 239]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLastGlobalExitRootReturn(pub [u8; 32]);
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
    #[doc = "Container type for all return fields from the `lastMainnetExitRoot` function with signature `lastMainnetExitRoot()` and selector `[49, 156, 247, 53]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastMainnetExitRootReturn(pub [u8; 32]);
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
    #[doc = "Container type for all return fields from the `rollupAddress` function with signature `rollupAddress()` and selector `[94, 198, 168, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RollupAddressReturn(pub ethers::core::types::Address);
}
