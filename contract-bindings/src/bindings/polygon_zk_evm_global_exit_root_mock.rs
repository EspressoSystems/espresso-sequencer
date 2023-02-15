pub use polygon_zk_evm_global_exit_root_mock::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod polygon_zk_evm_global_exit_root_mock {
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
    #[doc = "PolygonZkEVMGlobalExitRootMock was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_rollupAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_bridgeAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyAllowedContracts\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"mainnetExitRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"rollupExitRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UpdateGlobalExitRoot\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bridgeAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastGlobalExitRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"globalExitRootMap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastMainnetExitRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastRollupExitRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rollupAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"globalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGlobalExitRoot\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLastGlobalExitRoot\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"newRoot\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateExitRoot\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static POLYGONZKEVMGLOBALEXITROOTMOCK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct PolygonZkEVMGlobalExitRootMock<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PolygonZkEVMGlobalExitRootMock<M> {
        fn clone(&self) -> Self {
            PolygonZkEVMGlobalExitRootMock(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PolygonZkEVMGlobalExitRootMock<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PolygonZkEVMGlobalExitRootMock<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PolygonZkEVMGlobalExitRootMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PolygonZkEVMGlobalExitRootMock<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                POLYGONZKEVMGLOBALEXITROOTMOCK_ABI.clone(),
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
        #[doc = "Calls the contract's `setGlobalExitRoot` (0x5bcef673) function"]
        pub fn set_global_exit_root(
            &self,
            global_exit_root: [u8; 32],
            timestamp: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 206, 246, 115], (global_exit_root, timestamp))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLastGlobalExitRoot` (0x051a9e28) function"]
        pub fn set_last_global_exit_root(
            &self,
            timestamp: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 26, 158, 40], timestamp)
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
        #[doc = "Gets the contract's `UpdateGlobalExitRoot` event"]
        pub fn update_global_exit_root_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpdateGlobalExitRootFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, UpdateGlobalExitRootFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for PolygonZkEVMGlobalExitRootMock<M>
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
    #[doc = "Container type for all input parameters for the `setGlobalExitRoot` function with signature `setGlobalExitRoot(bytes32,uint256)` and selector `[91, 206, 246, 115]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setGlobalExitRoot", abi = "setGlobalExitRoot(bytes32,uint256)")]
    pub struct SetGlobalExitRootCall {
        pub global_exit_root: [u8; 32],
        pub timestamp: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setLastGlobalExitRoot` function with signature `setLastGlobalExitRoot(uint256)` and selector `[5, 26, 158, 40]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setLastGlobalExitRoot", abi = "setLastGlobalExitRoot(uint256)")]
    pub struct SetLastGlobalExitRootCall {
        pub timestamp: ethers::core::types::U256,
    }
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
    pub enum PolygonZkEVMGlobalExitRootMockCalls {
        BridgeAddress(BridgeAddressCall),
        GetLastGlobalExitRoot(GetLastGlobalExitRootCall),
        GlobalExitRootMap(GlobalExitRootMapCall),
        LastMainnetExitRoot(LastMainnetExitRootCall),
        LastRollupExitRoot(LastRollupExitRootCall),
        RollupAddress(RollupAddressCall),
        SetGlobalExitRoot(SetGlobalExitRootCall),
        SetLastGlobalExitRoot(SetLastGlobalExitRootCall),
        UpdateExitRoot(UpdateExitRootCall),
    }
    impl ethers::core::abi::AbiDecode for PolygonZkEVMGlobalExitRootMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BridgeAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootMockCalls::BridgeAddress(decoded));
            }
            if let Ok(decoded) =
                <GetLastGlobalExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootMockCalls::GetLastGlobalExitRoot(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GlobalExitRootMapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootMockCalls::GlobalExitRootMap(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LastMainnetExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootMockCalls::LastMainnetExitRoot(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LastRollupExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootMockCalls::LastRollupExitRoot(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RollupAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootMockCalls::RollupAddress(decoded));
            }
            if let Ok(decoded) =
                <SetGlobalExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootMockCalls::SetGlobalExitRoot(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetLastGlobalExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootMockCalls::SetLastGlobalExitRoot(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UpdateExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootMockCalls::UpdateExitRoot(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PolygonZkEVMGlobalExitRootMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PolygonZkEVMGlobalExitRootMockCalls::BridgeAddress(element) => element.encode(),
                PolygonZkEVMGlobalExitRootMockCalls::GetLastGlobalExitRoot(element) => {
                    element.encode()
                }
                PolygonZkEVMGlobalExitRootMockCalls::GlobalExitRootMap(element) => element.encode(),
                PolygonZkEVMGlobalExitRootMockCalls::LastMainnetExitRoot(element) => {
                    element.encode()
                }
                PolygonZkEVMGlobalExitRootMockCalls::LastRollupExitRoot(element) => {
                    element.encode()
                }
                PolygonZkEVMGlobalExitRootMockCalls::RollupAddress(element) => element.encode(),
                PolygonZkEVMGlobalExitRootMockCalls::SetGlobalExitRoot(element) => element.encode(),
                PolygonZkEVMGlobalExitRootMockCalls::SetLastGlobalExitRoot(element) => {
                    element.encode()
                }
                PolygonZkEVMGlobalExitRootMockCalls::UpdateExitRoot(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMGlobalExitRootMockCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMGlobalExitRootMockCalls::BridgeAddress(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootMockCalls::GetLastGlobalExitRoot(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMGlobalExitRootMockCalls::GlobalExitRootMap(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootMockCalls::LastMainnetExitRoot(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootMockCalls::LastRollupExitRoot(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootMockCalls::RollupAddress(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootMockCalls::SetGlobalExitRoot(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootMockCalls::SetLastGlobalExitRoot(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMGlobalExitRootMockCalls::UpdateExitRoot(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BridgeAddressCall> for PolygonZkEVMGlobalExitRootMockCalls {
        fn from(var: BridgeAddressCall) -> Self {
            PolygonZkEVMGlobalExitRootMockCalls::BridgeAddress(var)
        }
    }
    impl ::std::convert::From<GetLastGlobalExitRootCall> for PolygonZkEVMGlobalExitRootMockCalls {
        fn from(var: GetLastGlobalExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootMockCalls::GetLastGlobalExitRoot(var)
        }
    }
    impl ::std::convert::From<GlobalExitRootMapCall> for PolygonZkEVMGlobalExitRootMockCalls {
        fn from(var: GlobalExitRootMapCall) -> Self {
            PolygonZkEVMGlobalExitRootMockCalls::GlobalExitRootMap(var)
        }
    }
    impl ::std::convert::From<LastMainnetExitRootCall> for PolygonZkEVMGlobalExitRootMockCalls {
        fn from(var: LastMainnetExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootMockCalls::LastMainnetExitRoot(var)
        }
    }
    impl ::std::convert::From<LastRollupExitRootCall> for PolygonZkEVMGlobalExitRootMockCalls {
        fn from(var: LastRollupExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootMockCalls::LastRollupExitRoot(var)
        }
    }
    impl ::std::convert::From<RollupAddressCall> for PolygonZkEVMGlobalExitRootMockCalls {
        fn from(var: RollupAddressCall) -> Self {
            PolygonZkEVMGlobalExitRootMockCalls::RollupAddress(var)
        }
    }
    impl ::std::convert::From<SetGlobalExitRootCall> for PolygonZkEVMGlobalExitRootMockCalls {
        fn from(var: SetGlobalExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootMockCalls::SetGlobalExitRoot(var)
        }
    }
    impl ::std::convert::From<SetLastGlobalExitRootCall> for PolygonZkEVMGlobalExitRootMockCalls {
        fn from(var: SetLastGlobalExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootMockCalls::SetLastGlobalExitRoot(var)
        }
    }
    impl ::std::convert::From<UpdateExitRootCall> for PolygonZkEVMGlobalExitRootMockCalls {
        fn from(var: UpdateExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootMockCalls::UpdateExitRoot(var)
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
