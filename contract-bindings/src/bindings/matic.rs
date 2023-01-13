pub use matic::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod matic {
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
    #[doc = "Matic was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n\t{\n\t\t\"constant\": true,\n\t\t\"inputs\": [],\n\t\t\"name\": \"name\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"string\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"spender\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"name\": \"value\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"approve\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"bool\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": true,\n\t\t\"inputs\": [],\n\t\t\"name\": \"totalSupply\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"from\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"name\": \"to\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"name\": \"value\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"transferFrom\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"bool\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": true,\n\t\t\"inputs\": [],\n\t\t\"name\": \"decimals\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint8\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"spender\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"name\": \"addedValue\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"increaseAllowance\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"success\",\n\t\t\t\t\"type\": \"bool\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": false,\n\t\t\"inputs\": [],\n\t\t\"name\": \"unpause\",\n\t\t\"outputs\": [],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": true,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"account\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"isPauser\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"bool\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": true,\n\t\t\"inputs\": [],\n\t\t\"name\": \"paused\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"bool\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": false,\n\t\t\"inputs\": [],\n\t\t\"name\": \"renouncePauser\",\n\t\t\"outputs\": [],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": true,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"owner\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"balanceOf\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"account\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"addPauser\",\n\t\t\"outputs\": [],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": false,\n\t\t\"inputs\": [],\n\t\t\"name\": \"pause\",\n\t\t\"outputs\": [],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": true,\n\t\t\"inputs\": [],\n\t\t\"name\": \"symbol\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"string\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"spender\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"name\": \"subtractedValue\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"decreaseAllowance\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"success\",\n\t\t\t\t\"type\": \"bool\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"to\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"name\": \"value\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"transfer\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"bool\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"constant\": true,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"owner\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"name\": \"spender\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"allowance\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"name\": \"name\",\n\t\t\t\t\"type\": \"string\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"name\": \"symbol\",\n\t\t\t\t\"type\": \"string\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"name\": \"decimals\",\n\t\t\t\t\"type\": \"uint8\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"name\": \"totalSupply\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"payable\": false,\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"constructor\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"name\": \"account\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"Paused\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"name\": \"account\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"Unpaused\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"name\": \"account\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"PauserAdded\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"name\": \"account\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"PauserRemoved\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"name\": \"from\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"name\": \"to\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"name\": \"value\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"Transfer\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"name\": \"owner\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"name\": \"spender\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"name\": \"value\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"Approval\",\n\t\t\"type\": \"event\"\n\t}\n]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MATIC_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Matic<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Matic<M> {
        fn clone(&self) -> Self {
            Matic(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Matic<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Matic<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Matic))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Matic<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MATIC_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `addPauser` (0x82dc1ec4) function"]
        pub fn add_pauser(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 220, 30, 196], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            spender: ethers::core::types::Address,
            subtracted_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            spender: ethers::core::types::Address,
            added_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPauser` (0x46fbf68e) function"]
        pub fn is_pauser(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([70, 251, 246, 142], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pause` (0x8456cb59) function"]
        pub fn pause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renouncePauser` (0x6ef8d66d) function"]
        pub fn renounce_pauser(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 248, 214, 109], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpause` (0x3f4ba83a) function"]
        pub fn unpause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(&self) -> ethers::contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PauserAdded` event"]
        pub fn pauser_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PauserAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PauserRemoved` event"]
        pub fn pauser_removed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PauserRemovedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(&self) -> ethers::contract::builders::Event<M, UnpausedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MaticEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Matic<M> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "PauserAdded", abi = "PauserAdded(address)")]
    pub struct PauserAddedFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "PauserRemoved", abi = "PauserRemoved(address)")]
    pub struct PauserRemovedFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MaticEvents {
        ApprovalFilter(ApprovalFilter),
        PausedFilter(PausedFilter),
        PauserAddedFilter(PauserAddedFilter),
        PauserRemovedFilter(PauserRemovedFilter),
        TransferFilter(TransferFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::contract::EthLogDecode for MaticEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MaticEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(MaticEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserAddedFilter::decode_log(log) {
                return Ok(MaticEvents::PauserAddedFilter(decoded));
            }
            if let Ok(decoded) = PauserRemovedFilter::decode_log(log) {
                return Ok(MaticEvents::PauserRemovedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MaticEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(MaticEvents::UnpausedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MaticEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MaticEvents::ApprovalFilter(element) => element.fmt(f),
                MaticEvents::PausedFilter(element) => element.fmt(f),
                MaticEvents::PauserAddedFilter(element) => element.fmt(f),
                MaticEvents::PauserRemovedFilter(element) => element.fmt(f),
                MaticEvents::TransferFilter(element) => element.fmt(f),
                MaticEvents::UnpausedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addPauser` function with signature `addPauser(address)` and selector `[130, 220, 30, 196]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addPauser", abi = "addPauser(address)")]
    pub struct AddPauserCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub subtracted_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub added_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isPauser` function with signature `isPauser(address)` and selector `[70, 251, 246, 142]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isPauser", abi = "isPauser(address)")]
    pub struct IsPauserCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `pause` function with signature `pause()` and selector `[132, 86, 203, 89]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    #[doc = "Container type for all input parameters for the `paused` function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    #[doc = "Container type for all input parameters for the `renouncePauser` function with signature `renouncePauser()` and selector `[110, 248, 214, 109]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "renouncePauser", abi = "renouncePauser()")]
    pub struct RenouncePauserCall;
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `[63, 75, 168, 58]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MaticCalls {
        AddPauser(AddPauserCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        IsPauser(IsPauserCall),
        Name(NameCall),
        Pause(PauseCall),
        Paused(PausedCall),
        RenouncePauser(RenouncePauserCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Unpause(UnpauseCall),
    }
    impl ethers::core::abi::AbiDecode for MaticCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddPauserCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::AddPauser(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <IsPauserCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::IsPauser(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MaticCalls::Name(decoded));
            }
            if let Ok(decoded) = <PauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <RenouncePauserCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::RenouncePauser(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UnpauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaticCalls::Unpause(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MaticCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MaticCalls::AddPauser(element) => element.encode(),
                MaticCalls::Allowance(element) => element.encode(),
                MaticCalls::Approve(element) => element.encode(),
                MaticCalls::BalanceOf(element) => element.encode(),
                MaticCalls::Decimals(element) => element.encode(),
                MaticCalls::DecreaseAllowance(element) => element.encode(),
                MaticCalls::IncreaseAllowance(element) => element.encode(),
                MaticCalls::IsPauser(element) => element.encode(),
                MaticCalls::Name(element) => element.encode(),
                MaticCalls::Pause(element) => element.encode(),
                MaticCalls::Paused(element) => element.encode(),
                MaticCalls::RenouncePauser(element) => element.encode(),
                MaticCalls::Symbol(element) => element.encode(),
                MaticCalls::TotalSupply(element) => element.encode(),
                MaticCalls::Transfer(element) => element.encode(),
                MaticCalls::TransferFrom(element) => element.encode(),
                MaticCalls::Unpause(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MaticCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MaticCalls::AddPauser(element) => element.fmt(f),
                MaticCalls::Allowance(element) => element.fmt(f),
                MaticCalls::Approve(element) => element.fmt(f),
                MaticCalls::BalanceOf(element) => element.fmt(f),
                MaticCalls::Decimals(element) => element.fmt(f),
                MaticCalls::DecreaseAllowance(element) => element.fmt(f),
                MaticCalls::IncreaseAllowance(element) => element.fmt(f),
                MaticCalls::IsPauser(element) => element.fmt(f),
                MaticCalls::Name(element) => element.fmt(f),
                MaticCalls::Pause(element) => element.fmt(f),
                MaticCalls::Paused(element) => element.fmt(f),
                MaticCalls::RenouncePauser(element) => element.fmt(f),
                MaticCalls::Symbol(element) => element.fmt(f),
                MaticCalls::TotalSupply(element) => element.fmt(f),
                MaticCalls::Transfer(element) => element.fmt(f),
                MaticCalls::TransferFrom(element) => element.fmt(f),
                MaticCalls::Unpause(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddPauserCall> for MaticCalls {
        fn from(var: AddPauserCall) -> Self {
            MaticCalls::AddPauser(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for MaticCalls {
        fn from(var: AllowanceCall) -> Self {
            MaticCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for MaticCalls {
        fn from(var: ApproveCall) -> Self {
            MaticCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for MaticCalls {
        fn from(var: BalanceOfCall) -> Self {
            MaticCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for MaticCalls {
        fn from(var: DecimalsCall) -> Self {
            MaticCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for MaticCalls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            MaticCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for MaticCalls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            MaticCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<IsPauserCall> for MaticCalls {
        fn from(var: IsPauserCall) -> Self {
            MaticCalls::IsPauser(var)
        }
    }
    impl ::std::convert::From<NameCall> for MaticCalls {
        fn from(var: NameCall) -> Self {
            MaticCalls::Name(var)
        }
    }
    impl ::std::convert::From<PauseCall> for MaticCalls {
        fn from(var: PauseCall) -> Self {
            MaticCalls::Pause(var)
        }
    }
    impl ::std::convert::From<PausedCall> for MaticCalls {
        fn from(var: PausedCall) -> Self {
            MaticCalls::Paused(var)
        }
    }
    impl ::std::convert::From<RenouncePauserCall> for MaticCalls {
        fn from(var: RenouncePauserCall) -> Self {
            MaticCalls::RenouncePauser(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for MaticCalls {
        fn from(var: SymbolCall) -> Self {
            MaticCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for MaticCalls {
        fn from(var: TotalSupplyCall) -> Self {
            MaticCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for MaticCalls {
        fn from(var: TransferCall) -> Self {
            MaticCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for MaticCalls {
        fn from(var: TransferFromCall) -> Self {
            MaticCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for MaticCalls {
        fn from(var: UnpauseCall) -> Self {
            MaticCalls::Unpause(var)
        }
    }
    #[doc = "Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AllowanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ApproveReturn(pub bool);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecreaseAllowanceReturn {
        pub success: bool,
    }
    #[doc = "Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IncreaseAllowanceReturn {
        pub success: bool,
    }
    #[doc = "Container type for all return fields from the `isPauser` function with signature `isPauser(address)` and selector `[70, 251, 246, 142]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsPauserReturn(pub bool);
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NameReturn(pub String);
    #[doc = "Container type for all return fields from the `paused` function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SymbolReturn(pub String);
    #[doc = "Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TotalSupplyReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TransferReturn(pub bool);
    #[doc = "Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TransferFromReturn(pub bool);
}
