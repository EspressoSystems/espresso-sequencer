pub use polygon_zk_evm_timelock::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod polygon_zk_evm_timelock {
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
    #[doc = "PolygonZkEVMTimelock was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"minDelay\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"proposers\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"executors\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract PolygonZkEVM\",\"name\":\"_polygonZkEVM\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CallExecuted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"predecessor\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"delay\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CallScheduled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Cancelled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldDuration\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newDuration\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MinDelayChange\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"previousAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"newAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleAdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleGranted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleRevoked\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CANCELLER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DEFAULT_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EXECUTOR_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"PROPOSER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"TIMELOCK_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancel\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"predecessor\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"execute\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"payloads\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"predecessor\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"executeBatch\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMinDelay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"duration\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoleAdmin\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"grantRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"predecessor\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hashOperation\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"payloads\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"predecessor\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hashOperationBatch\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOperation\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"registered\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOperationDone\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"done\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOperationPending\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"pending\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOperationReady\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ready\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC1155BatchReceived\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC1155Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC721Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"polygonZkEVM\",\"outputs\":[{\"internalType\":\"contract PolygonZkEVM\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"revokeRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"predecessor\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delay\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"schedule\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"payloads\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"predecessor\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delay\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"scheduleBatch\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newDelay\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateDelay\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static POLYGONZKEVMTIMELOCK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct PolygonZkEVMTimelock<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PolygonZkEVMTimelock<M> {
        fn clone(&self) -> Self {
            PolygonZkEVMTimelock(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PolygonZkEVMTimelock<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PolygonZkEVMTimelock<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PolygonZkEVMTimelock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PolygonZkEVMTimelock<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                POLYGONZKEVMTIMELOCK_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `CANCELLER_ROLE` (0xb08e51c0) function"]
        pub fn canceller_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([176, 142, 81, 192], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function"]
        pub fn default_admin_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `EXECUTOR_ROLE` (0x07bd0265) function"]
        pub fn executor_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([7, 189, 2, 101], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PROPOSER_ROLE` (0x8f61f4f5) function"]
        pub fn proposer_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([143, 97, 244, 245], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `TIMELOCK_ADMIN_ROLE` (0x0d3cf6fc) function"]
        pub fn timelock_admin_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([13, 60, 246, 252], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancel` (0xc4d252f5) function"]
        pub fn cancel(&self, id: [u8; 32]) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 210, 82, 245], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `execute` (0x134008d3) function"]
        pub fn execute(
            &self,
            target: ethers::core::types::Address,
            value: ethers::core::types::U256,
            payload: ethers::core::types::Bytes,
            predecessor: [u8; 32],
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [19, 64, 8, 211],
                    (target, value, payload, predecessor, salt),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeBatch` (0xe38335e5) function"]
        pub fn execute_batch(
            &self,
            targets: ::std::vec::Vec<ethers::core::types::Address>,
            values: ::std::vec::Vec<ethers::core::types::U256>,
            payloads: ::std::vec::Vec<ethers::core::types::Bytes>,
            predecessor: [u8; 32],
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [227, 131, 53, 229],
                    (targets, values, payloads, predecessor, salt),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinDelay` (0xf27a0c92) function"]
        pub fn get_min_delay(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([242, 122, 12, 146], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoleAdmin` (0x248a9ca3) function"]
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTimestamp` (0xd45c4435) function"]
        pub fn get_timestamp(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([212, 92, 68, 53], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `grantRole` (0x2f2ff15d) function"]
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasRole` (0x91d14854) function"]
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hashOperation` (0x8065657f) function"]
        pub fn hash_operation(
            &self,
            target: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
            predecessor: [u8; 32],
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [128, 101, 101, 127],
                    (target, value, data, predecessor, salt),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hashOperationBatch` (0xb1c5f427) function"]
        pub fn hash_operation_batch(
            &self,
            targets: ::std::vec::Vec<ethers::core::types::Address>,
            values: ::std::vec::Vec<ethers::core::types::U256>,
            payloads: ::std::vec::Vec<ethers::core::types::Bytes>,
            predecessor: [u8; 32],
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [177, 197, 244, 39],
                    (targets, values, payloads, predecessor, salt),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOperation` (0x31d50750) function"]
        pub fn is_operation(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([49, 213, 7, 80], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOperationDone` (0x2ab0f529) function"]
        pub fn is_operation_done(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([42, 176, 245, 41], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOperationPending` (0x584b153e) function"]
        pub fn is_operation_pending(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([88, 75, 21, 62], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOperationReady` (0x13bc9f20) function"]
        pub fn is_operation_ready(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([19, 188, 159, 32], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function"]
        pub fn on_erc1155_batch_received(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ::std::vec::Vec<ethers::core::types::U256>,
            p3: ::std::vec::Vec<ethers::core::types::U256>,
            p4: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `onERC1155Received` (0xf23a6e61) function"]
        pub fn on_erc1155_received(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::U256,
            p3: ethers::core::types::U256,
            p4: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `onERC721Received` (0x150b7a02) function"]
        pub fn on_erc721_received(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::U256,
            p3: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `polygonZkEVM` (0x3a6aae72) function"]
        pub fn polygon_zk_evm(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([58, 106, 174, 114], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceRole` (0x36568abe) function"]
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeRole` (0xd547741f) function"]
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `schedule` (0x01d5062a) function"]
        pub fn schedule(
            &self,
            target: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
            predecessor: [u8; 32],
            salt: [u8; 32],
            delay: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [1, 213, 6, 42],
                    (target, value, data, predecessor, salt, delay),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `scheduleBatch` (0x8f2a0bb0) function"]
        pub fn schedule_batch(
            &self,
            targets: ::std::vec::Vec<ethers::core::types::Address>,
            values: ::std::vec::Vec<ethers::core::types::U256>,
            payloads: ::std::vec::Vec<ethers::core::types::Bytes>,
            predecessor: [u8; 32],
            salt: [u8; 32],
            delay: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [143, 42, 11, 176],
                    (targets, values, payloads, predecessor, salt, delay),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateDelay` (0x64d62353) function"]
        pub fn update_delay(
            &self,
            new_delay: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 214, 35, 83], new_delay)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `CallExecuted` event"]
        pub fn call_executed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CallExecutedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CallScheduled` event"]
        pub fn call_scheduled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CallScheduledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Cancelled` event"]
        pub fn cancelled_filter(&self) -> ethers::contract::builders::Event<M, CancelledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MinDelayChange` event"]
        pub fn min_delay_change_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MinDelayChangeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleAdminChanged` event"]
        pub fn role_admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleAdminChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleGranted` event"]
        pub fn role_granted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleGrantedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleRevoked` event"]
        pub fn role_revoked_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleRevokedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PolygonZkEVMTimelockEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for PolygonZkEVMTimelock<M>
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
    #[ethevent(
        name = "CallExecuted",
        abi = "CallExecuted(bytes32,uint256,address,uint256,bytes)"
    )]
    pub struct CallExecutedFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub index: ethers::core::types::U256,
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
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
        name = "CallScheduled",
        abi = "CallScheduled(bytes32,uint256,address,uint256,bytes,bytes32,uint256)"
    )]
    pub struct CallScheduledFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub index: ethers::core::types::U256,
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
        pub predecessor: [u8; 32],
        pub delay: ethers::core::types::U256,
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
    #[ethevent(name = "Cancelled", abi = "Cancelled(bytes32)")]
    pub struct CancelledFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
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
    #[ethevent(name = "MinDelayChange", abi = "MinDelayChange(uint256,uint256)")]
    pub struct MinDelayChangeFilter {
        pub old_duration: ethers::core::types::U256,
        pub new_duration: ethers::core::types::U256,
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
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PolygonZkEVMTimelockEvents {
        CallExecutedFilter(CallExecutedFilter),
        CallScheduledFilter(CallScheduledFilter),
        CancelledFilter(CancelledFilter),
        MinDelayChangeFilter(MinDelayChangeFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ethers::contract::EthLogDecode for PolygonZkEVMTimelockEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CallExecutedFilter::decode_log(log) {
                return Ok(PolygonZkEVMTimelockEvents::CallExecutedFilter(decoded));
            }
            if let Ok(decoded) = CallScheduledFilter::decode_log(log) {
                return Ok(PolygonZkEVMTimelockEvents::CallScheduledFilter(decoded));
            }
            if let Ok(decoded) = CancelledFilter::decode_log(log) {
                return Ok(PolygonZkEVMTimelockEvents::CancelledFilter(decoded));
            }
            if let Ok(decoded) = MinDelayChangeFilter::decode_log(log) {
                return Ok(PolygonZkEVMTimelockEvents::MinDelayChangeFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(PolygonZkEVMTimelockEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(PolygonZkEVMTimelockEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(PolygonZkEVMTimelockEvents::RoleRevokedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMTimelockEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMTimelockEvents::CallExecutedFilter(element) => element.fmt(f),
                PolygonZkEVMTimelockEvents::CallScheduledFilter(element) => element.fmt(f),
                PolygonZkEVMTimelockEvents::CancelledFilter(element) => element.fmt(f),
                PolygonZkEVMTimelockEvents::MinDelayChangeFilter(element) => element.fmt(f),
                PolygonZkEVMTimelockEvents::RoleAdminChangedFilter(element) => element.fmt(f),
                PolygonZkEVMTimelockEvents::RoleGrantedFilter(element) => element.fmt(f),
                PolygonZkEVMTimelockEvents::RoleRevokedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `CANCELLER_ROLE` function with signature `CANCELLER_ROLE()` and selector `[176, 142, 81, 192]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "CANCELLER_ROLE", abi = "CANCELLER_ROLE()")]
    pub struct CancellerRoleCall;
    #[doc = "Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `[162, 23, 253, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    #[doc = "Container type for all input parameters for the `EXECUTOR_ROLE` function with signature `EXECUTOR_ROLE()` and selector `[7, 189, 2, 101]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "EXECUTOR_ROLE", abi = "EXECUTOR_ROLE()")]
    pub struct ExecutorRoleCall;
    #[doc = "Container type for all input parameters for the `PROPOSER_ROLE` function with signature `PROPOSER_ROLE()` and selector `[143, 97, 244, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "PROPOSER_ROLE", abi = "PROPOSER_ROLE()")]
    pub struct ProposerRoleCall;
    #[doc = "Container type for all input parameters for the `TIMELOCK_ADMIN_ROLE` function with signature `TIMELOCK_ADMIN_ROLE()` and selector `[13, 60, 246, 252]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "TIMELOCK_ADMIN_ROLE", abi = "TIMELOCK_ADMIN_ROLE()")]
    pub struct TimelockAdminRoleCall;
    #[doc = "Container type for all input parameters for the `cancel` function with signature `cancel(bytes32)` and selector `[196, 210, 82, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cancel", abi = "cancel(bytes32)")]
    pub struct CancelCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `execute` function with signature `execute(address,uint256,bytes,bytes32,bytes32)` and selector `[19, 64, 8, 211]`"]
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
        name = "execute",
        abi = "execute(address,uint256,bytes,bytes32,bytes32)"
    )]
    pub struct ExecuteCall {
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub payload: ethers::core::types::Bytes,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `executeBatch` function with signature `executeBatch(address[],uint256[],bytes[],bytes32,bytes32)` and selector `[227, 131, 53, 229]`"]
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
        name = "executeBatch",
        abi = "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)"
    )]
    pub struct ExecuteBatchCall {
        pub targets: ::std::vec::Vec<ethers::core::types::Address>,
        pub values: ::std::vec::Vec<ethers::core::types::U256>,
        pub payloads: ::std::vec::Vec<ethers::core::types::Bytes>,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getMinDelay` function with signature `getMinDelay()` and selector `[242, 122, 12, 146]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMinDelay", abi = "getMinDelay()")]
    pub struct GetMinDelayCall;
    #[doc = "Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `[36, 138, 156, 163]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getTimestamp` function with signature `getTimestamp(bytes32)` and selector `[212, 92, 68, 53]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getTimestamp", abi = "getTimestamp(bytes32)")]
    pub struct GetTimestampCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `[47, 47, 241, 93]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `[145, 209, 72, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hashOperation` function with signature `hashOperation(address,uint256,bytes,bytes32,bytes32)` and selector `[128, 101, 101, 127]`"]
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
        name = "hashOperation",
        abi = "hashOperation(address,uint256,bytes,bytes32,bytes32)"
    )]
    pub struct HashOperationCall {
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `hashOperationBatch` function with signature `hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)` and selector `[177, 197, 244, 39]`"]
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
        name = "hashOperationBatch",
        abi = "hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)"
    )]
    pub struct HashOperationBatchCall {
        pub targets: ::std::vec::Vec<ethers::core::types::Address>,
        pub values: ::std::vec::Vec<ethers::core::types::U256>,
        pub payloads: ::std::vec::Vec<ethers::core::types::Bytes>,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isOperation` function with signature `isOperation(bytes32)` and selector `[49, 213, 7, 80]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isOperation", abi = "isOperation(bytes32)")]
    pub struct IsOperationCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isOperationDone` function with signature `isOperationDone(bytes32)` and selector `[42, 176, 245, 41]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isOperationDone", abi = "isOperationDone(bytes32)")]
    pub struct IsOperationDoneCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isOperationPending` function with signature `isOperationPending(bytes32)` and selector `[88, 75, 21, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isOperationPending", abi = "isOperationPending(bytes32)")]
    pub struct IsOperationPendingCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isOperationReady` function with signature `isOperationReady(bytes32)` and selector `[19, 188, 159, 32]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isOperationReady", abi = "isOperationReady(bytes32)")]
    pub struct IsOperationReadyCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `[188, 25, 124, 129]`"]
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
        name = "onERC1155BatchReceived",
        abi = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct OnERC1155BatchReceivedCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ::std::vec::Vec<ethers::core::types::U256>,
        pub ::std::vec::Vec<ethers::core::types::U256>,
        pub ethers::core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `[242, 58, 110, 97]`"]
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
        name = "onERC1155Received",
        abi = "onERC1155Received(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnERC1155ReceivedCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `[21, 11, 122, 2]`"]
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
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `polygonZkEVM` function with signature `polygonZkEVM()` and selector `[58, 106, 174, 114]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "polygonZkEVM", abi = "polygonZkEVM()")]
    pub struct PolygonZkEVMCall;
    #[doc = "Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `[54, 86, 138, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `[213, 71, 116, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `schedule` function with signature `schedule(address,uint256,bytes,bytes32,bytes32,uint256)` and selector `[1, 213, 6, 42]`"]
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
        name = "schedule",
        abi = "schedule(address,uint256,bytes,bytes32,bytes32,uint256)"
    )]
    pub struct ScheduleCall {
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
        pub delay: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `scheduleBatch` function with signature `scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)` and selector `[143, 42, 11, 176]`"]
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
        name = "scheduleBatch",
        abi = "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)"
    )]
    pub struct ScheduleBatchCall {
        pub targets: ::std::vec::Vec<ethers::core::types::Address>,
        pub values: ::std::vec::Vec<ethers::core::types::U256>,
        pub payloads: ::std::vec::Vec<ethers::core::types::Bytes>,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
        pub delay: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `updateDelay` function with signature `updateDelay(uint256)` and selector `[100, 214, 35, 83]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "updateDelay", abi = "updateDelay(uint256)")]
    pub struct UpdateDelayCall {
        pub new_delay: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PolygonZkEVMTimelockCalls {
        CancellerRole(CancellerRoleCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        ExecutorRole(ExecutorRoleCall),
        ProposerRole(ProposerRoleCall),
        TimelockAdminRole(TimelockAdminRoleCall),
        Cancel(CancelCall),
        Execute(ExecuteCall),
        ExecuteBatch(ExecuteBatchCall),
        GetMinDelay(GetMinDelayCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetTimestamp(GetTimestampCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        HashOperation(HashOperationCall),
        HashOperationBatch(HashOperationBatchCall),
        IsOperation(IsOperationCall),
        IsOperationDone(IsOperationDoneCall),
        IsOperationPending(IsOperationPendingCall),
        IsOperationReady(IsOperationReadyCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        OnERC721Received(OnERC721ReceivedCall),
        PolygonZkEVM(PolygonZkEVMCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        Schedule(ScheduleCall),
        ScheduleBatch(ScheduleBatchCall),
        SupportsInterface(SupportsInterfaceCall),
        UpdateDelay(UpdateDelayCall),
    }
    impl ethers::core::abi::AbiDecode for PolygonZkEVMTimelockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CancellerRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::CancellerRole(decoded));
            }
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) =
                <ExecutorRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::ExecutorRole(decoded));
            }
            if let Ok(decoded) =
                <ProposerRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::ProposerRole(decoded));
            }
            if let Ok(decoded) =
                <TimelockAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::TimelockAdminRole(decoded));
            }
            if let Ok(decoded) = <CancelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::Cancel(decoded));
            }
            if let Ok(decoded) =
                <ExecuteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::Execute(decoded));
            }
            if let Ok(decoded) =
                <ExecuteBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::ExecuteBatch(decoded));
            }
            if let Ok(decoded) =
                <GetMinDelayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::GetMinDelay(decoded));
            }
            if let Ok(decoded) =
                <GetRoleAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) =
                <GetTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::GetTimestamp(decoded));
            }
            if let Ok(decoded) =
                <GrantRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::GrantRole(decoded));
            }
            if let Ok(decoded) =
                <HasRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::HasRole(decoded));
            }
            if let Ok(decoded) =
                <HashOperationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::HashOperation(decoded));
            }
            if let Ok(decoded) =
                <HashOperationBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::HashOperationBatch(decoded));
            }
            if let Ok(decoded) =
                <IsOperationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::IsOperation(decoded));
            }
            if let Ok(decoded) =
                <IsOperationDoneCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::IsOperationDone(decoded));
            }
            if let Ok(decoded) =
                <IsOperationPendingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::IsOperationPending(decoded));
            }
            if let Ok(decoded) =
                <IsOperationReadyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::IsOperationReady(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155BatchReceivedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155ReceivedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::OnERC1155Received(decoded));
            }
            if let Ok(decoded) =
                <OnERC721ReceivedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::OnERC721Received(decoded));
            }
            if let Ok(decoded) =
                <PolygonZkEVMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::PolygonZkEVM(decoded));
            }
            if let Ok(decoded) =
                <RenounceRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::RenounceRole(decoded));
            }
            if let Ok(decoded) =
                <RevokeRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <ScheduleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::Schedule(decoded));
            }
            if let Ok(decoded) =
                <ScheduleBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::ScheduleBatch(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <UpdateDelayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMTimelockCalls::UpdateDelay(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PolygonZkEVMTimelockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PolygonZkEVMTimelockCalls::CancellerRole(element) => element.encode(),
                PolygonZkEVMTimelockCalls::DefaultAdminRole(element) => element.encode(),
                PolygonZkEVMTimelockCalls::ExecutorRole(element) => element.encode(),
                PolygonZkEVMTimelockCalls::ProposerRole(element) => element.encode(),
                PolygonZkEVMTimelockCalls::TimelockAdminRole(element) => element.encode(),
                PolygonZkEVMTimelockCalls::Cancel(element) => element.encode(),
                PolygonZkEVMTimelockCalls::Execute(element) => element.encode(),
                PolygonZkEVMTimelockCalls::ExecuteBatch(element) => element.encode(),
                PolygonZkEVMTimelockCalls::GetMinDelay(element) => element.encode(),
                PolygonZkEVMTimelockCalls::GetRoleAdmin(element) => element.encode(),
                PolygonZkEVMTimelockCalls::GetTimestamp(element) => element.encode(),
                PolygonZkEVMTimelockCalls::GrantRole(element) => element.encode(),
                PolygonZkEVMTimelockCalls::HasRole(element) => element.encode(),
                PolygonZkEVMTimelockCalls::HashOperation(element) => element.encode(),
                PolygonZkEVMTimelockCalls::HashOperationBatch(element) => element.encode(),
                PolygonZkEVMTimelockCalls::IsOperation(element) => element.encode(),
                PolygonZkEVMTimelockCalls::IsOperationDone(element) => element.encode(),
                PolygonZkEVMTimelockCalls::IsOperationPending(element) => element.encode(),
                PolygonZkEVMTimelockCalls::IsOperationReady(element) => element.encode(),
                PolygonZkEVMTimelockCalls::OnERC1155BatchReceived(element) => element.encode(),
                PolygonZkEVMTimelockCalls::OnERC1155Received(element) => element.encode(),
                PolygonZkEVMTimelockCalls::OnERC721Received(element) => element.encode(),
                PolygonZkEVMTimelockCalls::PolygonZkEVM(element) => element.encode(),
                PolygonZkEVMTimelockCalls::RenounceRole(element) => element.encode(),
                PolygonZkEVMTimelockCalls::RevokeRole(element) => element.encode(),
                PolygonZkEVMTimelockCalls::Schedule(element) => element.encode(),
                PolygonZkEVMTimelockCalls::ScheduleBatch(element) => element.encode(),
                PolygonZkEVMTimelockCalls::SupportsInterface(element) => element.encode(),
                PolygonZkEVMTimelockCalls::UpdateDelay(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMTimelockCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMTimelockCalls::CancellerRole(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::DefaultAdminRole(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::ExecutorRole(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::ProposerRole(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::TimelockAdminRole(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::Cancel(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::Execute(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::ExecuteBatch(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::GetMinDelay(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::GetRoleAdmin(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::GetTimestamp(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::GrantRole(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::HasRole(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::HashOperation(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::HashOperationBatch(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::IsOperation(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::IsOperationDone(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::IsOperationPending(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::IsOperationReady(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::OnERC1155BatchReceived(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::OnERC1155Received(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::OnERC721Received(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::PolygonZkEVM(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::RenounceRole(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::RevokeRole(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::Schedule(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::ScheduleBatch(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::SupportsInterface(element) => element.fmt(f),
                PolygonZkEVMTimelockCalls::UpdateDelay(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CancellerRoleCall> for PolygonZkEVMTimelockCalls {
        fn from(var: CancellerRoleCall) -> Self {
            PolygonZkEVMTimelockCalls::CancellerRole(var)
        }
    }
    impl ::std::convert::From<DefaultAdminRoleCall> for PolygonZkEVMTimelockCalls {
        fn from(var: DefaultAdminRoleCall) -> Self {
            PolygonZkEVMTimelockCalls::DefaultAdminRole(var)
        }
    }
    impl ::std::convert::From<ExecutorRoleCall> for PolygonZkEVMTimelockCalls {
        fn from(var: ExecutorRoleCall) -> Self {
            PolygonZkEVMTimelockCalls::ExecutorRole(var)
        }
    }
    impl ::std::convert::From<ProposerRoleCall> for PolygonZkEVMTimelockCalls {
        fn from(var: ProposerRoleCall) -> Self {
            PolygonZkEVMTimelockCalls::ProposerRole(var)
        }
    }
    impl ::std::convert::From<TimelockAdminRoleCall> for PolygonZkEVMTimelockCalls {
        fn from(var: TimelockAdminRoleCall) -> Self {
            PolygonZkEVMTimelockCalls::TimelockAdminRole(var)
        }
    }
    impl ::std::convert::From<CancelCall> for PolygonZkEVMTimelockCalls {
        fn from(var: CancelCall) -> Self {
            PolygonZkEVMTimelockCalls::Cancel(var)
        }
    }
    impl ::std::convert::From<ExecuteCall> for PolygonZkEVMTimelockCalls {
        fn from(var: ExecuteCall) -> Self {
            PolygonZkEVMTimelockCalls::Execute(var)
        }
    }
    impl ::std::convert::From<ExecuteBatchCall> for PolygonZkEVMTimelockCalls {
        fn from(var: ExecuteBatchCall) -> Self {
            PolygonZkEVMTimelockCalls::ExecuteBatch(var)
        }
    }
    impl ::std::convert::From<GetMinDelayCall> for PolygonZkEVMTimelockCalls {
        fn from(var: GetMinDelayCall) -> Self {
            PolygonZkEVMTimelockCalls::GetMinDelay(var)
        }
    }
    impl ::std::convert::From<GetRoleAdminCall> for PolygonZkEVMTimelockCalls {
        fn from(var: GetRoleAdminCall) -> Self {
            PolygonZkEVMTimelockCalls::GetRoleAdmin(var)
        }
    }
    impl ::std::convert::From<GetTimestampCall> for PolygonZkEVMTimelockCalls {
        fn from(var: GetTimestampCall) -> Self {
            PolygonZkEVMTimelockCalls::GetTimestamp(var)
        }
    }
    impl ::std::convert::From<GrantRoleCall> for PolygonZkEVMTimelockCalls {
        fn from(var: GrantRoleCall) -> Self {
            PolygonZkEVMTimelockCalls::GrantRole(var)
        }
    }
    impl ::std::convert::From<HasRoleCall> for PolygonZkEVMTimelockCalls {
        fn from(var: HasRoleCall) -> Self {
            PolygonZkEVMTimelockCalls::HasRole(var)
        }
    }
    impl ::std::convert::From<HashOperationCall> for PolygonZkEVMTimelockCalls {
        fn from(var: HashOperationCall) -> Self {
            PolygonZkEVMTimelockCalls::HashOperation(var)
        }
    }
    impl ::std::convert::From<HashOperationBatchCall> for PolygonZkEVMTimelockCalls {
        fn from(var: HashOperationBatchCall) -> Self {
            PolygonZkEVMTimelockCalls::HashOperationBatch(var)
        }
    }
    impl ::std::convert::From<IsOperationCall> for PolygonZkEVMTimelockCalls {
        fn from(var: IsOperationCall) -> Self {
            PolygonZkEVMTimelockCalls::IsOperation(var)
        }
    }
    impl ::std::convert::From<IsOperationDoneCall> for PolygonZkEVMTimelockCalls {
        fn from(var: IsOperationDoneCall) -> Self {
            PolygonZkEVMTimelockCalls::IsOperationDone(var)
        }
    }
    impl ::std::convert::From<IsOperationPendingCall> for PolygonZkEVMTimelockCalls {
        fn from(var: IsOperationPendingCall) -> Self {
            PolygonZkEVMTimelockCalls::IsOperationPending(var)
        }
    }
    impl ::std::convert::From<IsOperationReadyCall> for PolygonZkEVMTimelockCalls {
        fn from(var: IsOperationReadyCall) -> Self {
            PolygonZkEVMTimelockCalls::IsOperationReady(var)
        }
    }
    impl ::std::convert::From<OnERC1155BatchReceivedCall> for PolygonZkEVMTimelockCalls {
        fn from(var: OnERC1155BatchReceivedCall) -> Self {
            PolygonZkEVMTimelockCalls::OnERC1155BatchReceived(var)
        }
    }
    impl ::std::convert::From<OnERC1155ReceivedCall> for PolygonZkEVMTimelockCalls {
        fn from(var: OnERC1155ReceivedCall) -> Self {
            PolygonZkEVMTimelockCalls::OnERC1155Received(var)
        }
    }
    impl ::std::convert::From<OnERC721ReceivedCall> for PolygonZkEVMTimelockCalls {
        fn from(var: OnERC721ReceivedCall) -> Self {
            PolygonZkEVMTimelockCalls::OnERC721Received(var)
        }
    }
    impl ::std::convert::From<PolygonZkEVMCall> for PolygonZkEVMTimelockCalls {
        fn from(var: PolygonZkEVMCall) -> Self {
            PolygonZkEVMTimelockCalls::PolygonZkEVM(var)
        }
    }
    impl ::std::convert::From<RenounceRoleCall> for PolygonZkEVMTimelockCalls {
        fn from(var: RenounceRoleCall) -> Self {
            PolygonZkEVMTimelockCalls::RenounceRole(var)
        }
    }
    impl ::std::convert::From<RevokeRoleCall> for PolygonZkEVMTimelockCalls {
        fn from(var: RevokeRoleCall) -> Self {
            PolygonZkEVMTimelockCalls::RevokeRole(var)
        }
    }
    impl ::std::convert::From<ScheduleCall> for PolygonZkEVMTimelockCalls {
        fn from(var: ScheduleCall) -> Self {
            PolygonZkEVMTimelockCalls::Schedule(var)
        }
    }
    impl ::std::convert::From<ScheduleBatchCall> for PolygonZkEVMTimelockCalls {
        fn from(var: ScheduleBatchCall) -> Self {
            PolygonZkEVMTimelockCalls::ScheduleBatch(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for PolygonZkEVMTimelockCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            PolygonZkEVMTimelockCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<UpdateDelayCall> for PolygonZkEVMTimelockCalls {
        fn from(var: UpdateDelayCall) -> Self {
            PolygonZkEVMTimelockCalls::UpdateDelay(var)
        }
    }
    #[doc = "Container type for all return fields from the `CANCELLER_ROLE` function with signature `CANCELLER_ROLE()` and selector `[176, 142, 81, 192]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CancellerRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `[162, 23, 253, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `EXECUTOR_ROLE` function with signature `EXECUTOR_ROLE()` and selector `[7, 189, 2, 101]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExecutorRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `PROPOSER_ROLE` function with signature `PROPOSER_ROLE()` and selector `[143, 97, 244, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ProposerRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `TIMELOCK_ADMIN_ROLE` function with signature `TIMELOCK_ADMIN_ROLE()` and selector `[13, 60, 246, 252]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TimelockAdminRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getMinDelay` function with signature `getMinDelay()` and selector `[242, 122, 12, 146]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMinDelayReturn {
        pub duration: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `[36, 138, 156, 163]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getTimestamp` function with signature `getTimestamp(bytes32)` and selector `[212, 92, 68, 53]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTimestampReturn {
        pub timestamp: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `[145, 209, 72, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HasRoleReturn(pub bool);
    #[doc = "Container type for all return fields from the `hashOperation` function with signature `hashOperation(address,uint256,bytes,bytes32,bytes32)` and selector `[128, 101, 101, 127]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HashOperationReturn {
        pub hash: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `hashOperationBatch` function with signature `hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)` and selector `[177, 197, 244, 39]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HashOperationBatchReturn {
        pub hash: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `isOperation` function with signature `isOperation(bytes32)` and selector `[49, 213, 7, 80]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsOperationReturn {
        pub registered: bool,
    }
    #[doc = "Container type for all return fields from the `isOperationDone` function with signature `isOperationDone(bytes32)` and selector `[42, 176, 245, 41]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsOperationDoneReturn {
        pub done: bool,
    }
    #[doc = "Container type for all return fields from the `isOperationPending` function with signature `isOperationPending(bytes32)` and selector `[88, 75, 21, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsOperationPendingReturn {
        pub pending: bool,
    }
    #[doc = "Container type for all return fields from the `isOperationReady` function with signature `isOperationReady(bytes32)` and selector `[19, 188, 159, 32]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsOperationReadyReturn {
        pub ready: bool,
    }
    #[doc = "Container type for all return fields from the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `[188, 25, 124, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OnERC1155BatchReceivedReturn(pub [u8; 4]);
    #[doc = "Container type for all return fields from the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `[242, 58, 110, 97]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OnERC1155ReceivedReturn(pub [u8; 4]);
    #[doc = "Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `[21, 11, 122, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    #[doc = "Container type for all return fields from the `polygonZkEVM` function with signature `polygonZkEVM()` and selector `[58, 106, 174, 114]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PolygonZkEVMReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
}
