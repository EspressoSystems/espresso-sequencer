pub use polygon_zk_evm_bridge_mock::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod polygon_zk_evm_bridge_mock {
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
    #[doc = "PolygonZkEVMBridgeMock was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"leafType\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"originAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"depositCount\",\"type\":\"uint32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BridgeEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"originAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ClaimEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"EmergencyStateActivated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"EmergencyStateDeactivated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"originTokenAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"wrappedTokenAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewWrappedToken\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"LEAF_TYPE_ASSET\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"LEAF_TYPE_MESSAGE\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAINNET_NETWORK_ID\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"activateEmergencyState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"permitData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"bridgeAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"bridgeMessage\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"smtProof\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"mainnetExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"rollupExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"smtProof\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"mainnetExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"rollupExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimMessage\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"claimedBitMap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deactivateEmergencyState\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"depositCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDepositRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"leafType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"metadataHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getLeafValue\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originTokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTokenWrappedAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"globalExitRootManager\",\"outputs\":[{\"internalType\":\"contract IPolygonZkEVMGlobalExitRoot\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"_networkID\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"contract IPolygonZkEVMGlobalExitRoot\",\"name\":\"_globalExitRootManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_polygonZkEVMaddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isClaimed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isEmergencyState\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxEtherBridge\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"networkID\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"polygonZkEVMaddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"precalculatedWrapperAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_maxEtherBridge\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMaxEtherBridge\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"_networkID\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setNetworkID\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenInfoToWrappedToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"leafHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"smtProof\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"index\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"verifyMerkleProof\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"wrappedTokenToTokenInfo\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originTokenAddress\",\"type\":\"address\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static POLYGONZKEVMBRIDGEMOCK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct PolygonZkEVMBridgeMock<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PolygonZkEVMBridgeMock<M> {
        fn clone(&self) -> Self {
            PolygonZkEVMBridgeMock(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PolygonZkEVMBridgeMock<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PolygonZkEVMBridgeMock<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PolygonZkEVMBridgeMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PolygonZkEVMBridgeMock<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                POLYGONZKEVMBRIDGEMOCK_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `LEAF_TYPE_ASSET` (0xa08e8a08) function"]
        pub fn leaf_type_asset(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([160, 142, 138, 8], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `LEAF_TYPE_MESSAGE` (0xff634ed7) function"]
        pub fn leaf_type_message(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([255, 99, 78, 215], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAINNET_NETWORK_ID` (0xed6be5c9) function"]
        pub fn mainnet_network_id(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([237, 107, 229, 201], ())
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `claimedBitMap` (0xee25560b) function"]
        pub fn claimed_bit_map(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([238, 37, 86, 11], p0)
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
        #[doc = "Calls the contract's `getTokenWrappedAddress` (0x22e95f2c) function"]
        pub fn get_token_wrapped_address(
            &self,
            origin_network: u32,
            origin_token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([34, 233, 95, 44], (origin_network, origin_token_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `globalExitRootManager` (0xd02103ca) function"]
        pub fn global_exit_root_manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([208, 33, 3, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x647c576c) function"]
        pub fn initialize(
            &self,
            network_id: u32,
            global_exit_root_manager: ethers::core::types::Address,
            polygon_zk_ev_maddress: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [100, 124, 87, 108],
                    (network_id, global_exit_root_manager, polygon_zk_ev_maddress),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isClaimed` (0x9e34070f) function"]
        pub fn is_claimed(
            &self,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([158, 52, 7, 15], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isEmergencyState` (0x15064c96) function"]
        pub fn is_emergency_state(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([21, 6, 76, 150], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxEtherBridge` (0x91e57e2d) function"]
        pub fn max_ether_bridge(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([145, 229, 126, 45], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `networkID` (0xbab161bf) function"]
        pub fn network_id(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([186, 177, 97, 191], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `polygonZkEVMaddress` (0x34ac9cf2) function"]
        pub fn polygon_zk_ev_maddress(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([52, 172, 156, 242], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `precalculatedWrapperAddress` (0xaaa13cc2) function"]
        pub fn precalculated_wrapper_address(
            &self,
            origin_network: u32,
            origin_token_address: ethers::core::types::Address,
            name: String,
            symbol: String,
            decimals: u8,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash(
                    [170, 161, 60, 194],
                    (origin_network, origin_token_address, name, symbol, decimals),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMaxEtherBridge` (0x2b5e42e7) function"]
        pub fn set_max_ether_bridge(
            &self,
            max_ether_bridge: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 94, 66, 231], max_ether_bridge)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setNetworkID` (0x2c3f58cd) function"]
        pub fn set_network_id(
            &self,
            network_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 63, 88, 205], network_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenInfoToWrappedToken` (0x81b1c174) function"]
        pub fn token_info_to_wrapped_token(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([129, 177, 193, 116], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyMerkleProof` (0x3da81682) function"]
        pub fn verify_merkle_proof(
            &self,
            leaf_hash: [u8; 32],
            smt_proof: ::std::vec::Vec<[u8; 32]>,
            index: u64,
            root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([61, 168, 22, 130], (leaf_hash, smt_proof, index, root))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrappedTokenToTokenInfo` (0x318aee3d) function"]
        pub fn wrapped_token_to_token_info(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (u32, ethers::core::types::Address)>
        {
            self.0
                .method_hash([49, 138, 238, 61], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `BridgeEvent` event"]
        pub fn bridge_event_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BridgeEventFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ClaimEvent` event"]
        pub fn claim_event_filter(&self) -> ethers::contract::builders::Event<M, ClaimEventFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EmergencyStateActivated` event"]
        pub fn emergency_state_activated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EmergencyStateActivatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EmergencyStateDeactivated` event"]
        pub fn emergency_state_deactivated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EmergencyStateDeactivatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewWrappedToken` event"]
        pub fn new_wrapped_token_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewWrappedTokenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PolygonZkEVMBridgeMockEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for PolygonZkEVMBridgeMock<M>
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
        name = "BridgeEvent",
        abi = "BridgeEvent(uint8,uint32,address,uint32,address,uint256,bytes,uint32)"
    )]
    pub struct BridgeEventFilter {
        pub leaf_type: u8,
        pub origin_network: u32,
        pub origin_address: ethers::core::types::Address,
        pub destination_network: u32,
        pub destination_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub metadata: ethers::core::types::Bytes,
        pub deposit_count: u32,
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
        name = "ClaimEvent",
        abi = "ClaimEvent(uint32,uint32,address,address,uint256)"
    )]
    pub struct ClaimEventFilter {
        pub index: u32,
        pub origin_network: u32,
        pub origin_address: ethers::core::types::Address,
        pub destination_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "EmergencyStateActivated", abi = "EmergencyStateActivated()")]
    pub struct EmergencyStateActivatedFilter();
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
        name = "EmergencyStateDeactivated",
        abi = "EmergencyStateDeactivated()"
    )]
    pub struct EmergencyStateDeactivatedFilter();
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
        name = "NewWrappedToken",
        abi = "NewWrappedToken(uint32,address,address)"
    )]
    pub struct NewWrappedTokenFilter {
        pub origin_network: u32,
        pub origin_token_address: ethers::core::types::Address,
        pub wrapped_token_address: ethers::core::types::Address,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PolygonZkEVMBridgeMockEvents {
        BridgeEventFilter(BridgeEventFilter),
        ClaimEventFilter(ClaimEventFilter),
        EmergencyStateActivatedFilter(EmergencyStateActivatedFilter),
        EmergencyStateDeactivatedFilter(EmergencyStateDeactivatedFilter),
        InitializedFilter(InitializedFilter),
        NewWrappedTokenFilter(NewWrappedTokenFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for PolygonZkEVMBridgeMockEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BridgeEventFilter::decode_log(log) {
                return Ok(PolygonZkEVMBridgeMockEvents::BridgeEventFilter(decoded));
            }
            if let Ok(decoded) = ClaimEventFilter::decode_log(log) {
                return Ok(PolygonZkEVMBridgeMockEvents::ClaimEventFilter(decoded));
            }
            if let Ok(decoded) = EmergencyStateActivatedFilter::decode_log(log) {
                return Ok(PolygonZkEVMBridgeMockEvents::EmergencyStateActivatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EmergencyStateDeactivatedFilter::decode_log(log) {
                return Ok(PolygonZkEVMBridgeMockEvents::EmergencyStateDeactivatedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(PolygonZkEVMBridgeMockEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = NewWrappedTokenFilter::decode_log(log) {
                return Ok(PolygonZkEVMBridgeMockEvents::NewWrappedTokenFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(PolygonZkEVMBridgeMockEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMBridgeMockEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMBridgeMockEvents::BridgeEventFilter(element) => element.fmt(f),
                PolygonZkEVMBridgeMockEvents::ClaimEventFilter(element) => element.fmt(f),
                PolygonZkEVMBridgeMockEvents::EmergencyStateActivatedFilter(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMBridgeMockEvents::EmergencyStateDeactivatedFilter(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMBridgeMockEvents::InitializedFilter(element) => element.fmt(f),
                PolygonZkEVMBridgeMockEvents::NewWrappedTokenFilter(element) => element.fmt(f),
                PolygonZkEVMBridgeMockEvents::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `LEAF_TYPE_ASSET` function with signature `LEAF_TYPE_ASSET()` and selector `[160, 142, 138, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "LEAF_TYPE_ASSET", abi = "LEAF_TYPE_ASSET()")]
    pub struct LeafTypeAssetCall;
    #[doc = "Container type for all input parameters for the `LEAF_TYPE_MESSAGE` function with signature `LEAF_TYPE_MESSAGE()` and selector `[255, 99, 78, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "LEAF_TYPE_MESSAGE", abi = "LEAF_TYPE_MESSAGE()")]
    pub struct LeafTypeMessageCall;
    #[doc = "Container type for all input parameters for the `MAINNET_NETWORK_ID` function with signature `MAINNET_NETWORK_ID()` and selector `[237, 107, 229, 201]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "MAINNET_NETWORK_ID", abi = "MAINNET_NETWORK_ID()")]
    pub struct MainnetNetworkIdCall;
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
    #[doc = "Container type for all input parameters for the `claimedBitMap` function with signature `claimedBitMap(uint256)` and selector `[238, 37, 86, 11]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "claimedBitMap", abi = "claimedBitMap(uint256)")]
    pub struct ClaimedBitMapCall(pub ethers::core::types::U256);
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
    #[doc = "Container type for all input parameters for the `getTokenWrappedAddress` function with signature `getTokenWrappedAddress(uint32,address)` and selector `[34, 233, 95, 44]`"]
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
        name = "getTokenWrappedAddress",
        abi = "getTokenWrappedAddress(uint32,address)"
    )]
    pub struct GetTokenWrappedAddressCall {
        pub origin_network: u32,
        pub origin_token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `globalExitRootManager` function with signature `globalExitRootManager()` and selector `[208, 33, 3, 202]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "globalExitRootManager", abi = "globalExitRootManager()")]
    pub struct GlobalExitRootManagerCall;
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(uint32,address,address)` and selector `[100, 124, 87, 108]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint32,address,address)")]
    pub struct InitializeCall {
        pub network_id: u32,
        pub global_exit_root_manager: ethers::core::types::Address,
        pub polygon_zk_ev_maddress: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isClaimed` function with signature `isClaimed(uint256)` and selector `[158, 52, 7, 15]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isClaimed", abi = "isClaimed(uint256)")]
    pub struct IsClaimedCall {
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isEmergencyState` function with signature `isEmergencyState()` and selector `[21, 6, 76, 150]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isEmergencyState", abi = "isEmergencyState()")]
    pub struct IsEmergencyStateCall;
    #[doc = "Container type for all input parameters for the `maxEtherBridge` function with signature `maxEtherBridge()` and selector `[145, 229, 126, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "maxEtherBridge", abi = "maxEtherBridge()")]
    pub struct MaxEtherBridgeCall;
    #[doc = "Container type for all input parameters for the `networkID` function with signature `networkID()` and selector `[186, 177, 97, 191]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "networkID", abi = "networkID()")]
    pub struct NetworkIDCall;
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `polygonZkEVMaddress` function with signature `polygonZkEVMaddress()` and selector `[52, 172, 156, 242]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "polygonZkEVMaddress", abi = "polygonZkEVMaddress()")]
    pub struct PolygonZkEVMaddressCall;
    #[doc = "Container type for all input parameters for the `precalculatedWrapperAddress` function with signature `precalculatedWrapperAddress(uint32,address,string,string,uint8)` and selector `[170, 161, 60, 194]`"]
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
        name = "precalculatedWrapperAddress",
        abi = "precalculatedWrapperAddress(uint32,address,string,string,uint8)"
    )]
    pub struct PrecalculatedWrapperAddressCall {
        pub origin_network: u32,
        pub origin_token_address: ethers::core::types::Address,
        pub name: String,
        pub symbol: String,
        pub decimals: u8,
    }
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `setMaxEtherBridge` function with signature `setMaxEtherBridge(uint256)` and selector `[43, 94, 66, 231]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMaxEtherBridge", abi = "setMaxEtherBridge(uint256)")]
    pub struct SetMaxEtherBridgeCall {
        pub max_ether_bridge: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setNetworkID` function with signature `setNetworkID(uint32)` and selector `[44, 63, 88, 205]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setNetworkID", abi = "setNetworkID(uint32)")]
    pub struct SetNetworkIDCall {
        pub network_id: u32,
    }
    #[doc = "Container type for all input parameters for the `tokenInfoToWrappedToken` function with signature `tokenInfoToWrappedToken(bytes32)` and selector `[129, 177, 193, 116]`"]
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
        name = "tokenInfoToWrappedToken",
        abi = "tokenInfoToWrappedToken(bytes32)"
    )]
    pub struct TokenInfoToWrappedTokenCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `verifyMerkleProof` function with signature `verifyMerkleProof(bytes32,bytes32[],uint64,bytes32)` and selector `[61, 168, 22, 130]`"]
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
        abi = "verifyMerkleProof(bytes32,bytes32[],uint64,bytes32)"
    )]
    pub struct VerifyMerkleProofCall {
        pub leaf_hash: [u8; 32],
        pub smt_proof: ::std::vec::Vec<[u8; 32]>,
        pub index: u64,
        pub root: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `wrappedTokenToTokenInfo` function with signature `wrappedTokenToTokenInfo(address)` and selector `[49, 138, 238, 61]`"]
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
        name = "wrappedTokenToTokenInfo",
        abi = "wrappedTokenToTokenInfo(address)"
    )]
    pub struct WrappedTokenToTokenInfoCall(pub ethers::core::types::Address);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PolygonZkEVMBridgeMockCalls {
        LeafTypeAsset(LeafTypeAssetCall),
        LeafTypeMessage(LeafTypeMessageCall),
        MainnetNetworkId(MainnetNetworkIdCall),
        ActivateEmergencyState(ActivateEmergencyStateCall),
        BridgeAsset(BridgeAssetCall),
        BridgeMessage(BridgeMessageCall),
        ClaimAsset(ClaimAssetCall),
        ClaimMessage(ClaimMessageCall),
        ClaimedBitMap(ClaimedBitMapCall),
        DeactivateEmergencyState(DeactivateEmergencyStateCall),
        DepositCount(DepositCountCall),
        GetDepositRoot(GetDepositRootCall),
        GetLeafValue(GetLeafValueCall),
        GetTokenWrappedAddress(GetTokenWrappedAddressCall),
        GlobalExitRootManager(GlobalExitRootManagerCall),
        Initialize(InitializeCall),
        IsClaimed(IsClaimedCall),
        IsEmergencyState(IsEmergencyStateCall),
        MaxEtherBridge(MaxEtherBridgeCall),
        NetworkID(NetworkIDCall),
        Owner(OwnerCall),
        PolygonZkEVMaddress(PolygonZkEVMaddressCall),
        PrecalculatedWrapperAddress(PrecalculatedWrapperAddressCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetMaxEtherBridge(SetMaxEtherBridgeCall),
        SetNetworkID(SetNetworkIDCall),
        TokenInfoToWrappedToken(TokenInfoToWrappedTokenCall),
        TransferOwnership(TransferOwnershipCall),
        VerifyMerkleProof(VerifyMerkleProofCall),
        WrappedTokenToTokenInfo(WrappedTokenToTokenInfoCall),
    }
    impl ethers::core::abi::AbiDecode for PolygonZkEVMBridgeMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <LeafTypeAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::LeafTypeAsset(decoded));
            }
            if let Ok(decoded) =
                <LeafTypeMessageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::LeafTypeMessage(decoded));
            }
            if let Ok(decoded) =
                <MainnetNetworkIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::MainnetNetworkId(decoded));
            }
            if let Ok(decoded) =
                <ActivateEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::ActivateEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <BridgeAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::BridgeAsset(decoded));
            }
            if let Ok(decoded) =
                <BridgeMessageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::BridgeMessage(decoded));
            }
            if let Ok(decoded) =
                <ClaimAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::ClaimAsset(decoded));
            }
            if let Ok(decoded) =
                <ClaimMessageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::ClaimMessage(decoded));
            }
            if let Ok(decoded) =
                <ClaimedBitMapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::ClaimedBitMap(decoded));
            }
            if let Ok(decoded) =
                <DeactivateEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMBridgeMockCalls::DeactivateEmergencyState(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DepositCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::DepositCount(decoded));
            }
            if let Ok(decoded) =
                <GetDepositRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::GetDepositRoot(decoded));
            }
            if let Ok(decoded) =
                <GetLeafValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::GetLeafValue(decoded));
            }
            if let Ok(decoded) =
                <GetTokenWrappedAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::GetTokenWrappedAddress(decoded));
            }
            if let Ok(decoded) =
                <GlobalExitRootManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::GlobalExitRootManager(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsClaimedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::IsClaimed(decoded));
            }
            if let Ok(decoded) =
                <IsEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::IsEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <MaxEtherBridgeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::MaxEtherBridge(decoded));
            }
            if let Ok(decoded) =
                <NetworkIDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::NetworkID(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PolygonZkEVMaddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::PolygonZkEVMaddress(decoded));
            }
            if let Ok(decoded) =
                <PrecalculatedWrapperAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMBridgeMockCalls::PrecalculatedWrapperAddress(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetMaxEtherBridgeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::SetMaxEtherBridge(decoded));
            }
            if let Ok(decoded) =
                <SetNetworkIDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::SetNetworkID(decoded));
            }
            if let Ok(decoded) =
                <TokenInfoToWrappedTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::TokenInfoToWrappedToken(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <VerifyMerkleProofCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::VerifyMerkleProof(decoded));
            }
            if let Ok(decoded) =
                <WrappedTokenToTokenInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeMockCalls::WrappedTokenToTokenInfo(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PolygonZkEVMBridgeMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PolygonZkEVMBridgeMockCalls::LeafTypeAsset(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::LeafTypeMessage(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::MainnetNetworkId(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::ActivateEmergencyState(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::BridgeAsset(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::BridgeMessage(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::ClaimAsset(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::ClaimMessage(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::ClaimedBitMap(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::DeactivateEmergencyState(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::DepositCount(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::GetDepositRoot(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::GetLeafValue(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::GetTokenWrappedAddress(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::GlobalExitRootManager(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::Initialize(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::IsClaimed(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::IsEmergencyState(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::MaxEtherBridge(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::NetworkID(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::Owner(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::PolygonZkEVMaddress(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::PrecalculatedWrapperAddress(element) => {
                    element.encode()
                }
                PolygonZkEVMBridgeMockCalls::RenounceOwnership(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::SetMaxEtherBridge(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::SetNetworkID(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::TokenInfoToWrappedToken(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::TransferOwnership(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::VerifyMerkleProof(element) => element.encode(),
                PolygonZkEVMBridgeMockCalls::WrappedTokenToTokenInfo(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMBridgeMockCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMBridgeMockCalls::LeafTypeAsset(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::LeafTypeMessage(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::MainnetNetworkId(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::ActivateEmergencyState(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::BridgeAsset(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::BridgeMessage(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::ClaimAsset(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::ClaimMessage(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::ClaimedBitMap(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::DeactivateEmergencyState(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::DepositCount(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::GetDepositRoot(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::GetLeafValue(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::GetTokenWrappedAddress(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::GlobalExitRootManager(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::Initialize(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::IsClaimed(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::IsEmergencyState(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::MaxEtherBridge(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::NetworkID(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::Owner(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::PolygonZkEVMaddress(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::PrecalculatedWrapperAddress(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::RenounceOwnership(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::SetMaxEtherBridge(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::SetNetworkID(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::TokenInfoToWrappedToken(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::TransferOwnership(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::VerifyMerkleProof(element) => element.fmt(f),
                PolygonZkEVMBridgeMockCalls::WrappedTokenToTokenInfo(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<LeafTypeAssetCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: LeafTypeAssetCall) -> Self {
            PolygonZkEVMBridgeMockCalls::LeafTypeAsset(var)
        }
    }
    impl ::std::convert::From<LeafTypeMessageCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: LeafTypeMessageCall) -> Self {
            PolygonZkEVMBridgeMockCalls::LeafTypeMessage(var)
        }
    }
    impl ::std::convert::From<MainnetNetworkIdCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: MainnetNetworkIdCall) -> Self {
            PolygonZkEVMBridgeMockCalls::MainnetNetworkId(var)
        }
    }
    impl ::std::convert::From<ActivateEmergencyStateCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: ActivateEmergencyStateCall) -> Self {
            PolygonZkEVMBridgeMockCalls::ActivateEmergencyState(var)
        }
    }
    impl ::std::convert::From<BridgeAssetCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: BridgeAssetCall) -> Self {
            PolygonZkEVMBridgeMockCalls::BridgeAsset(var)
        }
    }
    impl ::std::convert::From<BridgeMessageCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: BridgeMessageCall) -> Self {
            PolygonZkEVMBridgeMockCalls::BridgeMessage(var)
        }
    }
    impl ::std::convert::From<ClaimAssetCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: ClaimAssetCall) -> Self {
            PolygonZkEVMBridgeMockCalls::ClaimAsset(var)
        }
    }
    impl ::std::convert::From<ClaimMessageCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: ClaimMessageCall) -> Self {
            PolygonZkEVMBridgeMockCalls::ClaimMessage(var)
        }
    }
    impl ::std::convert::From<ClaimedBitMapCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: ClaimedBitMapCall) -> Self {
            PolygonZkEVMBridgeMockCalls::ClaimedBitMap(var)
        }
    }
    impl ::std::convert::From<DeactivateEmergencyStateCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: DeactivateEmergencyStateCall) -> Self {
            PolygonZkEVMBridgeMockCalls::DeactivateEmergencyState(var)
        }
    }
    impl ::std::convert::From<DepositCountCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: DepositCountCall) -> Self {
            PolygonZkEVMBridgeMockCalls::DepositCount(var)
        }
    }
    impl ::std::convert::From<GetDepositRootCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: GetDepositRootCall) -> Self {
            PolygonZkEVMBridgeMockCalls::GetDepositRoot(var)
        }
    }
    impl ::std::convert::From<GetLeafValueCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: GetLeafValueCall) -> Self {
            PolygonZkEVMBridgeMockCalls::GetLeafValue(var)
        }
    }
    impl ::std::convert::From<GetTokenWrappedAddressCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: GetTokenWrappedAddressCall) -> Self {
            PolygonZkEVMBridgeMockCalls::GetTokenWrappedAddress(var)
        }
    }
    impl ::std::convert::From<GlobalExitRootManagerCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: GlobalExitRootManagerCall) -> Self {
            PolygonZkEVMBridgeMockCalls::GlobalExitRootManager(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: InitializeCall) -> Self {
            PolygonZkEVMBridgeMockCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<IsClaimedCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: IsClaimedCall) -> Self {
            PolygonZkEVMBridgeMockCalls::IsClaimed(var)
        }
    }
    impl ::std::convert::From<IsEmergencyStateCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: IsEmergencyStateCall) -> Self {
            PolygonZkEVMBridgeMockCalls::IsEmergencyState(var)
        }
    }
    impl ::std::convert::From<MaxEtherBridgeCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: MaxEtherBridgeCall) -> Self {
            PolygonZkEVMBridgeMockCalls::MaxEtherBridge(var)
        }
    }
    impl ::std::convert::From<NetworkIDCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: NetworkIDCall) -> Self {
            PolygonZkEVMBridgeMockCalls::NetworkID(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: OwnerCall) -> Self {
            PolygonZkEVMBridgeMockCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PolygonZkEVMaddressCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: PolygonZkEVMaddressCall) -> Self {
            PolygonZkEVMBridgeMockCalls::PolygonZkEVMaddress(var)
        }
    }
    impl ::std::convert::From<PrecalculatedWrapperAddressCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: PrecalculatedWrapperAddressCall) -> Self {
            PolygonZkEVMBridgeMockCalls::PrecalculatedWrapperAddress(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            PolygonZkEVMBridgeMockCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetMaxEtherBridgeCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: SetMaxEtherBridgeCall) -> Self {
            PolygonZkEVMBridgeMockCalls::SetMaxEtherBridge(var)
        }
    }
    impl ::std::convert::From<SetNetworkIDCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: SetNetworkIDCall) -> Self {
            PolygonZkEVMBridgeMockCalls::SetNetworkID(var)
        }
    }
    impl ::std::convert::From<TokenInfoToWrappedTokenCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: TokenInfoToWrappedTokenCall) -> Self {
            PolygonZkEVMBridgeMockCalls::TokenInfoToWrappedToken(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            PolygonZkEVMBridgeMockCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<VerifyMerkleProofCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: VerifyMerkleProofCall) -> Self {
            PolygonZkEVMBridgeMockCalls::VerifyMerkleProof(var)
        }
    }
    impl ::std::convert::From<WrappedTokenToTokenInfoCall> for PolygonZkEVMBridgeMockCalls {
        fn from(var: WrappedTokenToTokenInfoCall) -> Self {
            PolygonZkEVMBridgeMockCalls::WrappedTokenToTokenInfo(var)
        }
    }
    #[doc = "Container type for all return fields from the `LEAF_TYPE_ASSET` function with signature `LEAF_TYPE_ASSET()` and selector `[160, 142, 138, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LeafTypeAssetReturn(pub u8);
    #[doc = "Container type for all return fields from the `LEAF_TYPE_MESSAGE` function with signature `LEAF_TYPE_MESSAGE()` and selector `[255, 99, 78, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LeafTypeMessageReturn(pub u8);
    #[doc = "Container type for all return fields from the `MAINNET_NETWORK_ID` function with signature `MAINNET_NETWORK_ID()` and selector `[237, 107, 229, 201]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MainnetNetworkIdReturn(pub u32);
    #[doc = "Container type for all return fields from the `claimedBitMap` function with signature `claimedBitMap(uint256)` and selector `[238, 37, 86, 11]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ClaimedBitMapReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `getTokenWrappedAddress` function with signature `getTokenWrappedAddress(uint32,address)` and selector `[34, 233, 95, 44]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTokenWrappedAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `globalExitRootManager` function with signature `globalExitRootManager()` and selector `[208, 33, 3, 202]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GlobalExitRootManagerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `isClaimed` function with signature `isClaimed(uint256)` and selector `[158, 52, 7, 15]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsClaimedReturn(pub bool);
    #[doc = "Container type for all return fields from the `isEmergencyState` function with signature `isEmergencyState()` and selector `[21, 6, 76, 150]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsEmergencyStateReturn(pub bool);
    #[doc = "Container type for all return fields from the `maxEtherBridge` function with signature `maxEtherBridge()` and selector `[145, 229, 126, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxEtherBridgeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `networkID` function with signature `networkID()` and selector `[186, 177, 97, 191]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NetworkIDReturn(pub u32);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `polygonZkEVMaddress` function with signature `polygonZkEVMaddress()` and selector `[52, 172, 156, 242]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PolygonZkEVMaddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `precalculatedWrapperAddress` function with signature `precalculatedWrapperAddress(uint32,address,string,string,uint8)` and selector `[170, 161, 60, 194]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PrecalculatedWrapperAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `tokenInfoToWrappedToken` function with signature `tokenInfoToWrappedToken(bytes32)` and selector `[129, 177, 193, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TokenInfoToWrappedTokenReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `verifyMerkleProof` function with signature `verifyMerkleProof(bytes32,bytes32[],uint64,bytes32)` and selector `[61, 168, 22, 130]`"]
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
    #[doc = "Container type for all return fields from the `wrappedTokenToTokenInfo` function with signature `wrappedTokenToTokenInfo(address)` and selector `[49, 138, 238, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WrappedTokenToTokenInfoReturn {
        pub origin_network: u32,
        pub origin_token_address: ethers::core::types::Address,
    }
}
