pub use polygon_zk_evm_bridge::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod polygon_zk_evm_bridge {
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
    #[doc = "PolygonZkEVMBridge was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"AlreadyClaimed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AmountDoesNotMatchMsgValue\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"DestinationNetworkInvalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"EtherTransferFailed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"GlobalExitRootInvalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidSmtProof\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MerkleTreeFull\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MessageFailed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MsgValueNotZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotValidAmount\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotValidOwner\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotValidSignature\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotValidSpender\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyEmergencyState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyNotEmergencyState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyPolygonZkEVM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"leafType\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"originAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"depositCount\",\"type\":\"uint32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BridgeEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"originAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ClaimEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"EmergencyStateActivated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"EmergencyStateDeactivated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"originTokenAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"wrappedTokenAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewWrappedToken\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"activateEmergencyState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"permitData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"bridgeAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"bridgeMessage\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32[32]\",\"name\":\"smtProof\",\"type\":\"bytes32[32]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"mainnetExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"rollupExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32[32]\",\"name\":\"smtProof\",\"type\":\"bytes32[32]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"mainnetExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"rollupExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimMessage\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"claimedBitMap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deactivateEmergencyState\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"depositCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDepositRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"leafType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"metadataHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getLeafValue\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originTokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTokenWrappedAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"globalExitRootManager\",\"outputs\":[{\"internalType\":\"contract IBasePolygonZkEVMGlobalExitRoot\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"_networkID\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"contract IBasePolygonZkEVMGlobalExitRoot\",\"name\":\"_globalExitRootManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_polygonZkEVMaddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isClaimed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isEmergencyState\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"networkID\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"polygonZkEVMaddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"precalculatedWrapperAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenInfoToWrappedToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"leafHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32[32]\",\"name\":\"smtProof\",\"type\":\"bytes32[32]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"verifyMerkleProof\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"wrappedTokenToTokenInfo\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originTokenAddress\",\"type\":\"address\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static POLYGONZKEVMBRIDGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct PolygonZkEVMBridge<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PolygonZkEVMBridge<M> {
        fn clone(&self) -> Self {
            PolygonZkEVMBridge(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PolygonZkEVMBridge<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PolygonZkEVMBridge<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PolygonZkEVMBridge))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PolygonZkEVMBridge<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), POLYGONZKEVMBRIDGE_ABI.clone(), client)
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
        #[doc = "Calls the contract's `claimAsset` (0x2cffd02e) function"]
        pub fn claim_asset(
            &self,
            smt_proof: [[u8; 32]; 32usize],
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
                    [44, 255, 208, 46],
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
        #[doc = "Calls the contract's `claimMessage` (0x2d2c9d94) function"]
        pub fn claim_message(
            &self,
            smt_proof: [[u8; 32]; 32usize],
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
                    [45, 44, 157, 148],
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
        #[doc = "Calls the contract's `networkID` (0xbab161bf) function"]
        pub fn network_id(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([186, 177, 97, 191], ())
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
        #[doc = "Calls the contract's `tokenInfoToWrappedToken` (0x81b1c174) function"]
        pub fn token_info_to_wrapped_token(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([129, 177, 193, 116], p0)
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
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PolygonZkEVMBridgeEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for PolygonZkEVMBridge<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `AlreadyClaimed` with signature `AlreadyClaimed()` and selector `[100, 108, 245, 88]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "AlreadyClaimed", abi = "AlreadyClaimed()")]
    pub struct AlreadyClaimed;
    #[doc = "Custom Error type `AmountDoesNotMatchMsgValue` with signature `AmountDoesNotMatchMsgValue()` and selector `[184, 146, 64, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "AmountDoesNotMatchMsgValue",
        abi = "AmountDoesNotMatchMsgValue()"
    )]
    pub struct AmountDoesNotMatchMsgValue;
    #[doc = "Custom Error type `DestinationNetworkInvalid` with signature `DestinationNetworkInvalid()` and selector `[5, 149, 234, 46]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "DestinationNetworkInvalid",
        abi = "DestinationNetworkInvalid()"
    )]
    pub struct DestinationNetworkInvalid;
    #[doc = "Custom Error type `EtherTransferFailed` with signature `EtherTransferFailed()` and selector `[103, 71, 162, 136]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "EtherTransferFailed", abi = "EtherTransferFailed()")]
    pub struct EtherTransferFailed;
    #[doc = "Custom Error type `GlobalExitRootInvalid` with signature `GlobalExitRootInvalid()` and selector `[0, 47, 111, 173]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "GlobalExitRootInvalid", abi = "GlobalExitRootInvalid()")]
    pub struct GlobalExitRootInvalid;
    #[doc = "Custom Error type `InvalidSmtProof` with signature `InvalidSmtProof()` and selector `[224, 65, 124, 236]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidSmtProof", abi = "InvalidSmtProof()")]
    pub struct InvalidSmtProof;
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
    #[doc = "Custom Error type `MessageFailed` with signature `MessageFailed()` and selector `[55, 227, 145, 195]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "MessageFailed", abi = "MessageFailed()")]
    pub struct MessageFailed;
    #[doc = "Custom Error type `MsgValueNotZero` with signature `MsgValueNotZero()` and selector `[121, 142, 230, 241]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "MsgValueNotZero", abi = "MsgValueNotZero()")]
    pub struct MsgValueNotZero;
    #[doc = "Custom Error type `NotValidAmount` with signature `NotValidAmount()` and selector `[3, 255, 252, 75]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NotValidAmount", abi = "NotValidAmount()")]
    pub struct NotValidAmount;
    #[doc = "Custom Error type `NotValidOwner` with signature `NotValidOwner()` and selector `[145, 46, 204, 231]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NotValidOwner", abi = "NotValidOwner()")]
    pub struct NotValidOwner;
    #[doc = "Custom Error type `NotValidSignature` with signature `NotValidSignature()` and selector `[226, 130, 192, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NotValidSignature", abi = "NotValidSignature()")]
    pub struct NotValidSignature;
    #[doc = "Custom Error type `NotValidSpender` with signature `NotValidSpender()` and selector `[117, 6, 67, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NotValidSpender", abi = "NotValidSpender()")]
    pub struct NotValidSpender;
    #[doc = "Custom Error type `OnlyEmergencyState` with signature `OnlyEmergencyState()` and selector `[83, 134, 105, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyEmergencyState", abi = "OnlyEmergencyState()")]
    pub struct OnlyEmergencyState;
    #[doc = "Custom Error type `OnlyNotEmergencyState` with signature `OnlyNotEmergencyState()` and selector `[47, 0, 71, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyNotEmergencyState", abi = "OnlyNotEmergencyState()")]
    pub struct OnlyNotEmergencyState;
    #[doc = "Custom Error type `OnlyPolygonZkEVM` with signature `OnlyPolygonZkEVM()` and selector `[226, 232, 16, 107]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyPolygonZkEVM", abi = "OnlyPolygonZkEVM()")]
    pub struct OnlyPolygonZkEVM;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PolygonZkEVMBridgeErrors {
        AlreadyClaimed(AlreadyClaimed),
        AmountDoesNotMatchMsgValue(AmountDoesNotMatchMsgValue),
        DestinationNetworkInvalid(DestinationNetworkInvalid),
        EtherTransferFailed(EtherTransferFailed),
        GlobalExitRootInvalid(GlobalExitRootInvalid),
        InvalidSmtProof(InvalidSmtProof),
        MerkleTreeFull(MerkleTreeFull),
        MessageFailed(MessageFailed),
        MsgValueNotZero(MsgValueNotZero),
        NotValidAmount(NotValidAmount),
        NotValidOwner(NotValidOwner),
        NotValidSignature(NotValidSignature),
        NotValidSpender(NotValidSpender),
        OnlyEmergencyState(OnlyEmergencyState),
        OnlyNotEmergencyState(OnlyNotEmergencyState),
        OnlyPolygonZkEVM(OnlyPolygonZkEVM),
    }
    impl ethers::core::abi::AbiDecode for PolygonZkEVMBridgeErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AlreadyClaimed as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::AlreadyClaimed(decoded));
            }
            if let Ok(decoded) =
                <AmountDoesNotMatchMsgValue as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::AmountDoesNotMatchMsgValue(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DestinationNetworkInvalid as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::DestinationNetworkInvalid(decoded));
            }
            if let Ok(decoded) =
                <EtherTransferFailed as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::EtherTransferFailed(decoded));
            }
            if let Ok(decoded) =
                <GlobalExitRootInvalid as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::GlobalExitRootInvalid(decoded));
            }
            if let Ok(decoded) =
                <InvalidSmtProof as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::InvalidSmtProof(decoded));
            }
            if let Ok(decoded) =
                <MerkleTreeFull as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::MerkleTreeFull(decoded));
            }
            if let Ok(decoded) =
                <MessageFailed as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::MessageFailed(decoded));
            }
            if let Ok(decoded) =
                <MsgValueNotZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::MsgValueNotZero(decoded));
            }
            if let Ok(decoded) =
                <NotValidAmount as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::NotValidAmount(decoded));
            }
            if let Ok(decoded) =
                <NotValidOwner as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::NotValidOwner(decoded));
            }
            if let Ok(decoded) =
                <NotValidSignature as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::NotValidSignature(decoded));
            }
            if let Ok(decoded) =
                <NotValidSpender as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::NotValidSpender(decoded));
            }
            if let Ok(decoded) =
                <OnlyEmergencyState as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::OnlyEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <OnlyNotEmergencyState as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::OnlyNotEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <OnlyPolygonZkEVM as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeErrors::OnlyPolygonZkEVM(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PolygonZkEVMBridgeErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                PolygonZkEVMBridgeErrors::AlreadyClaimed(element) => element.encode(),
                PolygonZkEVMBridgeErrors::AmountDoesNotMatchMsgValue(element) => element.encode(),
                PolygonZkEVMBridgeErrors::DestinationNetworkInvalid(element) => element.encode(),
                PolygonZkEVMBridgeErrors::EtherTransferFailed(element) => element.encode(),
                PolygonZkEVMBridgeErrors::GlobalExitRootInvalid(element) => element.encode(),
                PolygonZkEVMBridgeErrors::InvalidSmtProof(element) => element.encode(),
                PolygonZkEVMBridgeErrors::MerkleTreeFull(element) => element.encode(),
                PolygonZkEVMBridgeErrors::MessageFailed(element) => element.encode(),
                PolygonZkEVMBridgeErrors::MsgValueNotZero(element) => element.encode(),
                PolygonZkEVMBridgeErrors::NotValidAmount(element) => element.encode(),
                PolygonZkEVMBridgeErrors::NotValidOwner(element) => element.encode(),
                PolygonZkEVMBridgeErrors::NotValidSignature(element) => element.encode(),
                PolygonZkEVMBridgeErrors::NotValidSpender(element) => element.encode(),
                PolygonZkEVMBridgeErrors::OnlyEmergencyState(element) => element.encode(),
                PolygonZkEVMBridgeErrors::OnlyNotEmergencyState(element) => element.encode(),
                PolygonZkEVMBridgeErrors::OnlyPolygonZkEVM(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMBridgeErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMBridgeErrors::AlreadyClaimed(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::AmountDoesNotMatchMsgValue(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::DestinationNetworkInvalid(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::EtherTransferFailed(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::GlobalExitRootInvalid(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::InvalidSmtProof(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::MerkleTreeFull(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::MessageFailed(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::MsgValueNotZero(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::NotValidAmount(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::NotValidOwner(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::NotValidSignature(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::NotValidSpender(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::OnlyEmergencyState(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::OnlyNotEmergencyState(element) => element.fmt(f),
                PolygonZkEVMBridgeErrors::OnlyPolygonZkEVM(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AlreadyClaimed> for PolygonZkEVMBridgeErrors {
        fn from(var: AlreadyClaimed) -> Self {
            PolygonZkEVMBridgeErrors::AlreadyClaimed(var)
        }
    }
    impl ::std::convert::From<AmountDoesNotMatchMsgValue> for PolygonZkEVMBridgeErrors {
        fn from(var: AmountDoesNotMatchMsgValue) -> Self {
            PolygonZkEVMBridgeErrors::AmountDoesNotMatchMsgValue(var)
        }
    }
    impl ::std::convert::From<DestinationNetworkInvalid> for PolygonZkEVMBridgeErrors {
        fn from(var: DestinationNetworkInvalid) -> Self {
            PolygonZkEVMBridgeErrors::DestinationNetworkInvalid(var)
        }
    }
    impl ::std::convert::From<EtherTransferFailed> for PolygonZkEVMBridgeErrors {
        fn from(var: EtherTransferFailed) -> Self {
            PolygonZkEVMBridgeErrors::EtherTransferFailed(var)
        }
    }
    impl ::std::convert::From<GlobalExitRootInvalid> for PolygonZkEVMBridgeErrors {
        fn from(var: GlobalExitRootInvalid) -> Self {
            PolygonZkEVMBridgeErrors::GlobalExitRootInvalid(var)
        }
    }
    impl ::std::convert::From<InvalidSmtProof> for PolygonZkEVMBridgeErrors {
        fn from(var: InvalidSmtProof) -> Self {
            PolygonZkEVMBridgeErrors::InvalidSmtProof(var)
        }
    }
    impl ::std::convert::From<MerkleTreeFull> for PolygonZkEVMBridgeErrors {
        fn from(var: MerkleTreeFull) -> Self {
            PolygonZkEVMBridgeErrors::MerkleTreeFull(var)
        }
    }
    impl ::std::convert::From<MessageFailed> for PolygonZkEVMBridgeErrors {
        fn from(var: MessageFailed) -> Self {
            PolygonZkEVMBridgeErrors::MessageFailed(var)
        }
    }
    impl ::std::convert::From<MsgValueNotZero> for PolygonZkEVMBridgeErrors {
        fn from(var: MsgValueNotZero) -> Self {
            PolygonZkEVMBridgeErrors::MsgValueNotZero(var)
        }
    }
    impl ::std::convert::From<NotValidAmount> for PolygonZkEVMBridgeErrors {
        fn from(var: NotValidAmount) -> Self {
            PolygonZkEVMBridgeErrors::NotValidAmount(var)
        }
    }
    impl ::std::convert::From<NotValidOwner> for PolygonZkEVMBridgeErrors {
        fn from(var: NotValidOwner) -> Self {
            PolygonZkEVMBridgeErrors::NotValidOwner(var)
        }
    }
    impl ::std::convert::From<NotValidSignature> for PolygonZkEVMBridgeErrors {
        fn from(var: NotValidSignature) -> Self {
            PolygonZkEVMBridgeErrors::NotValidSignature(var)
        }
    }
    impl ::std::convert::From<NotValidSpender> for PolygonZkEVMBridgeErrors {
        fn from(var: NotValidSpender) -> Self {
            PolygonZkEVMBridgeErrors::NotValidSpender(var)
        }
    }
    impl ::std::convert::From<OnlyEmergencyState> for PolygonZkEVMBridgeErrors {
        fn from(var: OnlyEmergencyState) -> Self {
            PolygonZkEVMBridgeErrors::OnlyEmergencyState(var)
        }
    }
    impl ::std::convert::From<OnlyNotEmergencyState> for PolygonZkEVMBridgeErrors {
        fn from(var: OnlyNotEmergencyState) -> Self {
            PolygonZkEVMBridgeErrors::OnlyNotEmergencyState(var)
        }
    }
    impl ::std::convert::From<OnlyPolygonZkEVM> for PolygonZkEVMBridgeErrors {
        fn from(var: OnlyPolygonZkEVM) -> Self {
            PolygonZkEVMBridgeErrors::OnlyPolygonZkEVM(var)
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
        abi = "NewWrappedToken(uint32,address,address,bytes)"
    )]
    pub struct NewWrappedTokenFilter {
        pub origin_network: u32,
        pub origin_token_address: ethers::core::types::Address,
        pub wrapped_token_address: ethers::core::types::Address,
        pub metadata: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PolygonZkEVMBridgeEvents {
        BridgeEventFilter(BridgeEventFilter),
        ClaimEventFilter(ClaimEventFilter),
        EmergencyStateActivatedFilter(EmergencyStateActivatedFilter),
        EmergencyStateDeactivatedFilter(EmergencyStateDeactivatedFilter),
        InitializedFilter(InitializedFilter),
        NewWrappedTokenFilter(NewWrappedTokenFilter),
    }
    impl ethers::contract::EthLogDecode for PolygonZkEVMBridgeEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BridgeEventFilter::decode_log(log) {
                return Ok(PolygonZkEVMBridgeEvents::BridgeEventFilter(decoded));
            }
            if let Ok(decoded) = ClaimEventFilter::decode_log(log) {
                return Ok(PolygonZkEVMBridgeEvents::ClaimEventFilter(decoded));
            }
            if let Ok(decoded) = EmergencyStateActivatedFilter::decode_log(log) {
                return Ok(PolygonZkEVMBridgeEvents::EmergencyStateActivatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EmergencyStateDeactivatedFilter::decode_log(log) {
                return Ok(PolygonZkEVMBridgeEvents::EmergencyStateDeactivatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(PolygonZkEVMBridgeEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = NewWrappedTokenFilter::decode_log(log) {
                return Ok(PolygonZkEVMBridgeEvents::NewWrappedTokenFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMBridgeEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMBridgeEvents::BridgeEventFilter(element) => element.fmt(f),
                PolygonZkEVMBridgeEvents::ClaimEventFilter(element) => element.fmt(f),
                PolygonZkEVMBridgeEvents::EmergencyStateActivatedFilter(element) => element.fmt(f),
                PolygonZkEVMBridgeEvents::EmergencyStateDeactivatedFilter(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMBridgeEvents::InitializedFilter(element) => element.fmt(f),
                PolygonZkEVMBridgeEvents::NewWrappedTokenFilter(element) => element.fmt(f),
            }
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
    #[doc = "Container type for all input parameters for the `claimAsset` function with signature `claimAsset(bytes32[32],uint32,bytes32,bytes32,uint32,address,uint32,address,uint256,bytes)` and selector `[44, 255, 208, 46]`"]
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
        abi = "claimAsset(bytes32[32],uint32,bytes32,bytes32,uint32,address,uint32,address,uint256,bytes)"
    )]
    pub struct ClaimAssetCall {
        pub smt_proof: [[u8; 32]; 32usize],
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
    #[doc = "Container type for all input parameters for the `claimMessage` function with signature `claimMessage(bytes32[32],uint32,bytes32,bytes32,uint32,address,uint32,address,uint256,bytes)` and selector `[45, 44, 157, 148]`"]
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
        abi = "claimMessage(bytes32[32],uint32,bytes32,bytes32,uint32,address,uint32,address,uint256,bytes)"
    )]
    pub struct ClaimMessageCall {
        pub smt_proof: [[u8; 32]; 32usize],
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
    pub enum PolygonZkEVMBridgeCalls {
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
        NetworkID(NetworkIDCall),
        PolygonZkEVMaddress(PolygonZkEVMaddressCall),
        PrecalculatedWrapperAddress(PrecalculatedWrapperAddressCall),
        TokenInfoToWrappedToken(TokenInfoToWrappedTokenCall),
        VerifyMerkleProof(VerifyMerkleProofCall),
        WrappedTokenToTokenInfo(WrappedTokenToTokenInfoCall),
    }
    impl ethers::core::abi::AbiDecode for PolygonZkEVMBridgeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ActivateEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::ActivateEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <BridgeAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::BridgeAsset(decoded));
            }
            if let Ok(decoded) =
                <BridgeMessageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::BridgeMessage(decoded));
            }
            if let Ok(decoded) =
                <ClaimAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::ClaimAsset(decoded));
            }
            if let Ok(decoded) =
                <ClaimMessageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::ClaimMessage(decoded));
            }
            if let Ok(decoded) =
                <ClaimedBitMapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::ClaimedBitMap(decoded));
            }
            if let Ok(decoded) =
                <DeactivateEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMBridgeCalls::DeactivateEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <DepositCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::DepositCount(decoded));
            }
            if let Ok(decoded) =
                <GetDepositRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::GetDepositRoot(decoded));
            }
            if let Ok(decoded) =
                <GetLeafValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::GetLeafValue(decoded));
            }
            if let Ok(decoded) =
                <GetTokenWrappedAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::GetTokenWrappedAddress(decoded));
            }
            if let Ok(decoded) =
                <GlobalExitRootManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::GlobalExitRootManager(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsClaimedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::IsClaimed(decoded));
            }
            if let Ok(decoded) =
                <IsEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::IsEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <NetworkIDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::NetworkID(decoded));
            }
            if let Ok(decoded) =
                <PolygonZkEVMaddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::PolygonZkEVMaddress(decoded));
            }
            if let Ok(decoded) =
                <PrecalculatedWrapperAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMBridgeCalls::PrecalculatedWrapperAddress(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TokenInfoToWrappedTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::TokenInfoToWrappedToken(decoded));
            }
            if let Ok(decoded) =
                <VerifyMerkleProofCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::VerifyMerkleProof(decoded));
            }
            if let Ok(decoded) =
                <WrappedTokenToTokenInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMBridgeCalls::WrappedTokenToTokenInfo(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PolygonZkEVMBridgeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PolygonZkEVMBridgeCalls::ActivateEmergencyState(element) => element.encode(),
                PolygonZkEVMBridgeCalls::BridgeAsset(element) => element.encode(),
                PolygonZkEVMBridgeCalls::BridgeMessage(element) => element.encode(),
                PolygonZkEVMBridgeCalls::ClaimAsset(element) => element.encode(),
                PolygonZkEVMBridgeCalls::ClaimMessage(element) => element.encode(),
                PolygonZkEVMBridgeCalls::ClaimedBitMap(element) => element.encode(),
                PolygonZkEVMBridgeCalls::DeactivateEmergencyState(element) => element.encode(),
                PolygonZkEVMBridgeCalls::DepositCount(element) => element.encode(),
                PolygonZkEVMBridgeCalls::GetDepositRoot(element) => element.encode(),
                PolygonZkEVMBridgeCalls::GetLeafValue(element) => element.encode(),
                PolygonZkEVMBridgeCalls::GetTokenWrappedAddress(element) => element.encode(),
                PolygonZkEVMBridgeCalls::GlobalExitRootManager(element) => element.encode(),
                PolygonZkEVMBridgeCalls::Initialize(element) => element.encode(),
                PolygonZkEVMBridgeCalls::IsClaimed(element) => element.encode(),
                PolygonZkEVMBridgeCalls::IsEmergencyState(element) => element.encode(),
                PolygonZkEVMBridgeCalls::NetworkID(element) => element.encode(),
                PolygonZkEVMBridgeCalls::PolygonZkEVMaddress(element) => element.encode(),
                PolygonZkEVMBridgeCalls::PrecalculatedWrapperAddress(element) => element.encode(),
                PolygonZkEVMBridgeCalls::TokenInfoToWrappedToken(element) => element.encode(),
                PolygonZkEVMBridgeCalls::VerifyMerkleProof(element) => element.encode(),
                PolygonZkEVMBridgeCalls::WrappedTokenToTokenInfo(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMBridgeCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMBridgeCalls::ActivateEmergencyState(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::BridgeAsset(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::BridgeMessage(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::ClaimAsset(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::ClaimMessage(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::ClaimedBitMap(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::DeactivateEmergencyState(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::DepositCount(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::GetDepositRoot(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::GetLeafValue(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::GetTokenWrappedAddress(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::GlobalExitRootManager(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::Initialize(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::IsClaimed(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::IsEmergencyState(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::NetworkID(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::PolygonZkEVMaddress(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::PrecalculatedWrapperAddress(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::TokenInfoToWrappedToken(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::VerifyMerkleProof(element) => element.fmt(f),
                PolygonZkEVMBridgeCalls::WrappedTokenToTokenInfo(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ActivateEmergencyStateCall> for PolygonZkEVMBridgeCalls {
        fn from(var: ActivateEmergencyStateCall) -> Self {
            PolygonZkEVMBridgeCalls::ActivateEmergencyState(var)
        }
    }
    impl ::std::convert::From<BridgeAssetCall> for PolygonZkEVMBridgeCalls {
        fn from(var: BridgeAssetCall) -> Self {
            PolygonZkEVMBridgeCalls::BridgeAsset(var)
        }
    }
    impl ::std::convert::From<BridgeMessageCall> for PolygonZkEVMBridgeCalls {
        fn from(var: BridgeMessageCall) -> Self {
            PolygonZkEVMBridgeCalls::BridgeMessage(var)
        }
    }
    impl ::std::convert::From<ClaimAssetCall> for PolygonZkEVMBridgeCalls {
        fn from(var: ClaimAssetCall) -> Self {
            PolygonZkEVMBridgeCalls::ClaimAsset(var)
        }
    }
    impl ::std::convert::From<ClaimMessageCall> for PolygonZkEVMBridgeCalls {
        fn from(var: ClaimMessageCall) -> Self {
            PolygonZkEVMBridgeCalls::ClaimMessage(var)
        }
    }
    impl ::std::convert::From<ClaimedBitMapCall> for PolygonZkEVMBridgeCalls {
        fn from(var: ClaimedBitMapCall) -> Self {
            PolygonZkEVMBridgeCalls::ClaimedBitMap(var)
        }
    }
    impl ::std::convert::From<DeactivateEmergencyStateCall> for PolygonZkEVMBridgeCalls {
        fn from(var: DeactivateEmergencyStateCall) -> Self {
            PolygonZkEVMBridgeCalls::DeactivateEmergencyState(var)
        }
    }
    impl ::std::convert::From<DepositCountCall> for PolygonZkEVMBridgeCalls {
        fn from(var: DepositCountCall) -> Self {
            PolygonZkEVMBridgeCalls::DepositCount(var)
        }
    }
    impl ::std::convert::From<GetDepositRootCall> for PolygonZkEVMBridgeCalls {
        fn from(var: GetDepositRootCall) -> Self {
            PolygonZkEVMBridgeCalls::GetDepositRoot(var)
        }
    }
    impl ::std::convert::From<GetLeafValueCall> for PolygonZkEVMBridgeCalls {
        fn from(var: GetLeafValueCall) -> Self {
            PolygonZkEVMBridgeCalls::GetLeafValue(var)
        }
    }
    impl ::std::convert::From<GetTokenWrappedAddressCall> for PolygonZkEVMBridgeCalls {
        fn from(var: GetTokenWrappedAddressCall) -> Self {
            PolygonZkEVMBridgeCalls::GetTokenWrappedAddress(var)
        }
    }
    impl ::std::convert::From<GlobalExitRootManagerCall> for PolygonZkEVMBridgeCalls {
        fn from(var: GlobalExitRootManagerCall) -> Self {
            PolygonZkEVMBridgeCalls::GlobalExitRootManager(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for PolygonZkEVMBridgeCalls {
        fn from(var: InitializeCall) -> Self {
            PolygonZkEVMBridgeCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<IsClaimedCall> for PolygonZkEVMBridgeCalls {
        fn from(var: IsClaimedCall) -> Self {
            PolygonZkEVMBridgeCalls::IsClaimed(var)
        }
    }
    impl ::std::convert::From<IsEmergencyStateCall> for PolygonZkEVMBridgeCalls {
        fn from(var: IsEmergencyStateCall) -> Self {
            PolygonZkEVMBridgeCalls::IsEmergencyState(var)
        }
    }
    impl ::std::convert::From<NetworkIDCall> for PolygonZkEVMBridgeCalls {
        fn from(var: NetworkIDCall) -> Self {
            PolygonZkEVMBridgeCalls::NetworkID(var)
        }
    }
    impl ::std::convert::From<PolygonZkEVMaddressCall> for PolygonZkEVMBridgeCalls {
        fn from(var: PolygonZkEVMaddressCall) -> Self {
            PolygonZkEVMBridgeCalls::PolygonZkEVMaddress(var)
        }
    }
    impl ::std::convert::From<PrecalculatedWrapperAddressCall> for PolygonZkEVMBridgeCalls {
        fn from(var: PrecalculatedWrapperAddressCall) -> Self {
            PolygonZkEVMBridgeCalls::PrecalculatedWrapperAddress(var)
        }
    }
    impl ::std::convert::From<TokenInfoToWrappedTokenCall> for PolygonZkEVMBridgeCalls {
        fn from(var: TokenInfoToWrappedTokenCall) -> Self {
            PolygonZkEVMBridgeCalls::TokenInfoToWrappedToken(var)
        }
    }
    impl ::std::convert::From<VerifyMerkleProofCall> for PolygonZkEVMBridgeCalls {
        fn from(var: VerifyMerkleProofCall) -> Self {
            PolygonZkEVMBridgeCalls::VerifyMerkleProof(var)
        }
    }
    impl ::std::convert::From<WrappedTokenToTokenInfoCall> for PolygonZkEVMBridgeCalls {
        fn from(var: WrappedTokenToTokenInfoCall) -> Self {
            PolygonZkEVMBridgeCalls::WrappedTokenToTokenInfo(var)
        }
    }
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
