pub use i_polygon_zk_evm_global_exit_root::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_polygon_zk_evm_global_exit_root {
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
    #[doc = "IPolygonZkEVMGlobalExitRoot was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyAllowedContracts\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastGlobalExitRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"globalExitRootNum\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"globalExitRootMap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"newRollupExitRoot\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateExitRoot\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IPOLYGONZKEVMGLOBALEXITROOT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IPolygonZkEVMGlobalExitRoot<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IPolygonZkEVMGlobalExitRoot<M> {
        fn clone(&self) -> Self {
            IPolygonZkEVMGlobalExitRoot(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IPolygonZkEVMGlobalExitRoot<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IPolygonZkEVMGlobalExitRoot<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPolygonZkEVMGlobalExitRoot))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IPolygonZkEVMGlobalExitRoot<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IPOLYGONZKEVMGLOBALEXITROOT_ABI.clone(),
                client,
            )
            .into()
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
            global_exit_root_num: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([37, 123, 54, 50], global_exit_root_num)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateExitRoot` (0x33d6247d) function"]
        pub fn update_exit_root(
            &self,
            new_rollup_exit_root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 214, 36, 125], new_rollup_exit_root)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IPolygonZkEVMGlobalExitRoot<M>
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
    pub struct GlobalExitRootMapCall {
        pub global_exit_root_num: [u8; 32],
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
        pub new_rollup_exit_root: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPolygonZkEVMGlobalExitRootCalls {
        GetLastGlobalExitRoot(GetLastGlobalExitRootCall),
        GlobalExitRootMap(GlobalExitRootMapCall),
        UpdateExitRoot(UpdateExitRootCall),
    }
    impl ethers::core::abi::AbiDecode for IPolygonZkEVMGlobalExitRootCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetLastGlobalExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMGlobalExitRootCalls::GetLastGlobalExitRoot(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GlobalExitRootMapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMGlobalExitRootCalls::GlobalExitRootMap(decoded));
            }
            if let Ok(decoded) =
                <UpdateExitRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMGlobalExitRootCalls::UpdateExitRoot(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPolygonZkEVMGlobalExitRootCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPolygonZkEVMGlobalExitRootCalls::GetLastGlobalExitRoot(element) => {
                    element.encode()
                }
                IPolygonZkEVMGlobalExitRootCalls::GlobalExitRootMap(element) => element.encode(),
                IPolygonZkEVMGlobalExitRootCalls::UpdateExitRoot(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPolygonZkEVMGlobalExitRootCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPolygonZkEVMGlobalExitRootCalls::GetLastGlobalExitRoot(element) => element.fmt(f),
                IPolygonZkEVMGlobalExitRootCalls::GlobalExitRootMap(element) => element.fmt(f),
                IPolygonZkEVMGlobalExitRootCalls::UpdateExitRoot(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetLastGlobalExitRootCall> for IPolygonZkEVMGlobalExitRootCalls {
        fn from(var: GetLastGlobalExitRootCall) -> Self {
            IPolygonZkEVMGlobalExitRootCalls::GetLastGlobalExitRoot(var)
        }
    }
    impl ::std::convert::From<GlobalExitRootMapCall> for IPolygonZkEVMGlobalExitRootCalls {
        fn from(var: GlobalExitRootMapCall) -> Self {
            IPolygonZkEVMGlobalExitRootCalls::GlobalExitRootMap(var)
        }
    }
    impl ::std::convert::From<UpdateExitRootCall> for IPolygonZkEVMGlobalExitRootCalls {
        fn from(var: UpdateExitRootCall) -> Self {
            IPolygonZkEVMGlobalExitRootCalls::UpdateExitRoot(var)
        }
    }
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
}
