pub use polygon_zk_evm_global_exit_root_l2_mock::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod polygon_zk_evm_global_exit_root_l2_mock {
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
    #[doc = "PolygonZkEVMGlobalExitRootL2Mock was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_bridgeAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyAllowedContracts\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bridgeAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"globalExitRootMap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastRollupExitRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"newRoot\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExitRoot\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"globalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLastGlobalExitRoot\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"newRoot\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateExitRoot\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static POLYGONZKEVMGLOBALEXITROOTL2MOCK_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
    });
    pub struct PolygonZkEVMGlobalExitRootL2Mock<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PolygonZkEVMGlobalExitRootL2Mock<M> {
        fn clone(&self) -> Self {
            PolygonZkEVMGlobalExitRootL2Mock(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PolygonZkEVMGlobalExitRootL2Mock<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PolygonZkEVMGlobalExitRootL2Mock<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PolygonZkEVMGlobalExitRootL2Mock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PolygonZkEVMGlobalExitRootL2Mock<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                POLYGONZKEVMGLOBALEXITROOTL2MOCK_ABI.clone(),
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
        #[doc = "Calls the contract's `setExitRoot` (0x116c40c3) function"]
        pub fn set_exit_root(
            &self,
            new_root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([17, 108, 64, 195], new_root)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLastGlobalExitRoot` (0x96e07459) function"]
        pub fn set_last_global_exit_root(
            &self,
            global_exit_root: [u8; 32],
            block_number: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 224, 116, 89], (global_exit_root, block_number))
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
        for PolygonZkEVMGlobalExitRootL2Mock<M>
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
    #[doc = "Container type for all input parameters for the `setExitRoot` function with signature `setExitRoot(bytes32)` and selector `[17, 108, 64, 195]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setExitRoot", abi = "setExitRoot(bytes32)")]
    pub struct SetExitRootCall {
        pub new_root: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setLastGlobalExitRoot` function with signature `setLastGlobalExitRoot(bytes32,uint256)` and selector `[150, 224, 116, 89]`"]
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
        name = "setLastGlobalExitRoot",
        abi = "setLastGlobalExitRoot(bytes32,uint256)"
    )]
    pub struct SetLastGlobalExitRootCall {
        pub global_exit_root: [u8; 32],
        pub block_number: ethers::core::types::U256,
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
    pub enum PolygonZkEVMGlobalExitRootL2MockCalls {
        BridgeAddress(BridgeAddressCall),
        GlobalExitRootMap(GlobalExitRootMapCall),
        LastRollupExitRoot(LastRollupExitRootCall),
        SetExitRoot(SetExitRootCall),
        SetLastGlobalExitRoot(SetLastGlobalExitRootCall),
        UpdateExitRoot(UpdateExitRootCall),
    }
    impl ethers::core::abi::AbiDecode for PolygonZkEVMGlobalExitRootL2MockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BridgeAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootL2MockCalls::BridgeAddress(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GlobalExitRootMapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootL2MockCalls::GlobalExitRootMap(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LastRollupExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootL2MockCalls::LastRollupExitRoot(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootL2MockCalls::SetExitRoot(decoded));
            }
            if let Ok(decoded) =
                <SetLastGlobalExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootL2MockCalls::SetLastGlobalExitRoot(decoded));
            }
            if let Ok(decoded) =
                <UpdateExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMGlobalExitRootL2MockCalls::UpdateExitRoot(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PolygonZkEVMGlobalExitRootL2MockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PolygonZkEVMGlobalExitRootL2MockCalls::BridgeAddress(element) => element.encode(),
                PolygonZkEVMGlobalExitRootL2MockCalls::GlobalExitRootMap(element) => {
                    element.encode()
                }
                PolygonZkEVMGlobalExitRootL2MockCalls::LastRollupExitRoot(element) => {
                    element.encode()
                }
                PolygonZkEVMGlobalExitRootL2MockCalls::SetExitRoot(element) => element.encode(),
                PolygonZkEVMGlobalExitRootL2MockCalls::SetLastGlobalExitRoot(element) => {
                    element.encode()
                }
                PolygonZkEVMGlobalExitRootL2MockCalls::UpdateExitRoot(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMGlobalExitRootL2MockCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMGlobalExitRootL2MockCalls::BridgeAddress(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootL2MockCalls::GlobalExitRootMap(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootL2MockCalls::LastRollupExitRoot(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMGlobalExitRootL2MockCalls::SetExitRoot(element) => element.fmt(f),
                PolygonZkEVMGlobalExitRootL2MockCalls::SetLastGlobalExitRoot(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMGlobalExitRootL2MockCalls::UpdateExitRoot(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BridgeAddressCall> for PolygonZkEVMGlobalExitRootL2MockCalls {
        fn from(var: BridgeAddressCall) -> Self {
            PolygonZkEVMGlobalExitRootL2MockCalls::BridgeAddress(var)
        }
    }
    impl ::std::convert::From<GlobalExitRootMapCall> for PolygonZkEVMGlobalExitRootL2MockCalls {
        fn from(var: GlobalExitRootMapCall) -> Self {
            PolygonZkEVMGlobalExitRootL2MockCalls::GlobalExitRootMap(var)
        }
    }
    impl ::std::convert::From<LastRollupExitRootCall> for PolygonZkEVMGlobalExitRootL2MockCalls {
        fn from(var: LastRollupExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootL2MockCalls::LastRollupExitRoot(var)
        }
    }
    impl ::std::convert::From<SetExitRootCall> for PolygonZkEVMGlobalExitRootL2MockCalls {
        fn from(var: SetExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootL2MockCalls::SetExitRoot(var)
        }
    }
    impl ::std::convert::From<SetLastGlobalExitRootCall> for PolygonZkEVMGlobalExitRootL2MockCalls {
        fn from(var: SetLastGlobalExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootL2MockCalls::SetLastGlobalExitRoot(var)
        }
    }
    impl ::std::convert::From<UpdateExitRootCall> for PolygonZkEVMGlobalExitRootL2MockCalls {
        fn from(var: UpdateExitRootCall) -> Self {
            PolygonZkEVMGlobalExitRootL2MockCalls::UpdateExitRoot(var)
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
