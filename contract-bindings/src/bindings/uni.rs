pub use uni::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod uni {
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
    #[doc = "Uni was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"minter_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"mintingAllowedAfter_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"fromDelegate\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"toDelegate\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"DelegateChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"previousBalance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newBalance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DelegateVotesChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newMinter\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MinterChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DELEGATION_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"PERMIT_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rawAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkpoints\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"fromBlock\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint96\",\"name\":\"votes\",\"type\":\"uint96\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"delegate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"delegateBySig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delegates\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentVotes\",\"outputs\":[{\"internalType\":\"uint96\",\"name\":\"\",\"type\":\"uint96\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPriorVotes\",\"outputs\":[{\"internalType\":\"uint96\",\"name\":\"\",\"type\":\"uint96\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minimumTimeBetweenMints\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rawAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mintCap\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minter\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mintingAllowedAfter\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numCheckpoints\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rawAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"permit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"minter_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMinter\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rawAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rawAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static UNI_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Uni<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Uni<M> {
        fn clone(&self) -> Self {
            Uni(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Uni<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Uni<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Uni))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Uni<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), UNI_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `DELEGATION_TYPEHASH` (0xe7a324dc) function"]
        pub fn delegation_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([231, 163, 36, 220], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DOMAIN_TYPEHASH` (0x20606b70) function"]
        pub fn domain_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 96, 107, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function"]
        pub fn permit_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            account: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (account, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            raw_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, raw_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkpoints` (0xf1127ed8) function"]
        pub fn checkpoints(
            &self,
            p0: ethers::core::types::Address,
            p1: u32,
        ) -> ethers::contract::builders::ContractCall<M, (u32, u128)> {
            self.0
                .method_hash([241, 18, 126, 216], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegate` (0x5c19a95c) function"]
        pub fn delegate(
            &self,
            delegatee: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 25, 169, 92], delegatee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegateBySig` (0xc3cda520) function"]
        pub fn delegate_by_sig(
            &self,
            delegatee: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
            expiry: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 205, 165, 32], (delegatee, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegates` (0x587cde1e) function"]
        pub fn delegates(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([88, 124, 222, 30], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentVotes` (0xb4b5ea57) function"]
        pub fn get_current_votes(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([180, 181, 234, 87], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPriorVotes` (0x782d6fe1) function"]
        pub fn get_prior_votes(
            &self,
            account: ethers::core::types::Address,
            block_number: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([120, 45, 111, 225], (account, block_number))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minimumTimeBetweenMints` (0x5c11d62f) function"]
        pub fn minimum_time_between_mints(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([92, 17, 214, 47], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x40c10f19) function"]
        pub fn mint(
            &self,
            dst: ethers::core::types::Address,
            raw_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (dst, raw_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintCap` (0x76c71ca1) function"]
        pub fn mint_cap(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([118, 199, 28, 161], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minter` (0x07546172) function"]
        pub fn minter(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([7, 84, 97, 114], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintingAllowedAfter` (0x30b36cef) function"]
        pub fn minting_allowed_after(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([48, 179, 108, 239], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `numCheckpoints` (0x6fcfff45) function"]
        pub fn num_checkpoints(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([111, 207, 255, 69], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit` (0xd505accf) function"]
        pub fn permit(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            raw_amount: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, raw_amount, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMinter` (0xfca3b5aa) function"]
        pub fn set_minter(
            &self,
            minter: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 163, 181, 170], minter)
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
            dst: ethers::core::types::Address,
            raw_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (dst, raw_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            src: ethers::core::types::Address,
            dst: ethers::core::types::Address,
            raw_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (src, dst, raw_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DelegateChanged` event"]
        pub fn delegate_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DelegateChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DelegateVotesChanged` event"]
        pub fn delegate_votes_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DelegateVotesChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MinterChanged` event"]
        pub fn minter_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MinterChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, UniEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Uni<M> {
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
    #[ethevent(
        name = "DelegateChanged",
        abi = "DelegateChanged(address,address,address)"
    )]
    pub struct DelegateChangedFilter {
        #[ethevent(indexed)]
        pub delegator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from_delegate: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to_delegate: ethers::core::types::Address,
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
        name = "DelegateVotesChanged",
        abi = "DelegateVotesChanged(address,uint256,uint256)"
    )]
    pub struct DelegateVotesChangedFilter {
        #[ethevent(indexed)]
        pub delegate: ethers::core::types::Address,
        pub previous_balance: ethers::core::types::U256,
        pub new_balance: ethers::core::types::U256,
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
    #[ethevent(name = "MinterChanged", abi = "MinterChanged(address,address)")]
    pub struct MinterChangedFilter {
        pub minter: ethers::core::types::Address,
        pub new_minter: ethers::core::types::Address,
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
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniEvents {
        ApprovalFilter(ApprovalFilter),
        DelegateChangedFilter(DelegateChangedFilter),
        DelegateVotesChangedFilter(DelegateVotesChangedFilter),
        MinterChangedFilter(MinterChangedFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for UniEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(UniEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DelegateChangedFilter::decode_log(log) {
                return Ok(UniEvents::DelegateChangedFilter(decoded));
            }
            if let Ok(decoded) = DelegateVotesChangedFilter::decode_log(log) {
                return Ok(UniEvents::DelegateVotesChangedFilter(decoded));
            }
            if let Ok(decoded) = MinterChangedFilter::decode_log(log) {
                return Ok(UniEvents::MinterChangedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(UniEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for UniEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniEvents::ApprovalFilter(element) => element.fmt(f),
                UniEvents::DelegateChangedFilter(element) => element.fmt(f),
                UniEvents::DelegateVotesChangedFilter(element) => element.fmt(f),
                UniEvents::MinterChangedFilter(element) => element.fmt(f),
                UniEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DELEGATION_TYPEHASH` function with signature `DELEGATION_TYPEHASH()` and selector `[231, 163, 36, 220]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "DELEGATION_TYPEHASH", abi = "DELEGATION_TYPEHASH()")]
    pub struct DelegationTypehashCall;
    #[doc = "Container type for all input parameters for the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `[32, 96, 107, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "DOMAIN_TYPEHASH", abi = "DOMAIN_TYPEHASH()")]
    pub struct DomainTypehashCall;
    #[doc = "Container type for all input parameters for the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `[48, 173, 248, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "PERMIT_TYPEHASH", abi = "PERMIT_TYPEHASH()")]
    pub struct PermitTypehashCall;
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
        pub account: ethers::core::types::Address,
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
        pub raw_amount: ethers::core::types::U256,
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
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `checkpoints` function with signature `checkpoints(address,uint32)` and selector `[241, 18, 126, 216]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "checkpoints", abi = "checkpoints(address,uint32)")]
    pub struct CheckpointsCall(pub ethers::core::types::Address, pub u32);
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
    #[doc = "Container type for all input parameters for the `delegate` function with signature `delegate(address)` and selector `[92, 25, 169, 92]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "delegate", abi = "delegate(address)")]
    pub struct DelegateCall {
        pub delegatee: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `delegateBySig` function with signature `delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[195, 205, 165, 32]`"]
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
        name = "delegateBySig",
        abi = "delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct DelegateBySigCall {
        pub delegatee: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
        pub expiry: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `delegates` function with signature `delegates(address)` and selector `[88, 124, 222, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "delegates", abi = "delegates(address)")]
    pub struct DelegatesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `getCurrentVotes` function with signature `getCurrentVotes(address)` and selector `[180, 181, 234, 87]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCurrentVotes", abi = "getCurrentVotes(address)")]
    pub struct GetCurrentVotesCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getPriorVotes` function with signature `getPriorVotes(address,uint256)` and selector `[120, 45, 111, 225]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPriorVotes", abi = "getPriorVotes(address,uint256)")]
    pub struct GetPriorVotesCall {
        pub account: ethers::core::types::Address,
        pub block_number: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `minimumTimeBetweenMints` function with signature `minimumTimeBetweenMints()` and selector `[92, 17, 214, 47]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "minimumTimeBetweenMints", abi = "minimumTimeBetweenMints()")]
    pub struct MinimumTimeBetweenMintsCall;
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `[64, 193, 15, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub dst: ethers::core::types::Address,
        pub raw_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mintCap` function with signature `mintCap()` and selector `[118, 199, 28, 161]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mintCap", abi = "mintCap()")]
    pub struct MintCapCall;
    #[doc = "Container type for all input parameters for the `minter` function with signature `minter()` and selector `[7, 84, 97, 114]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "minter", abi = "minter()")]
    pub struct MinterCall;
    #[doc = "Container type for all input parameters for the `mintingAllowedAfter` function with signature `mintingAllowedAfter()` and selector `[48, 179, 108, 239]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mintingAllowedAfter", abi = "mintingAllowedAfter()")]
    pub struct MintingAllowedAfterCall;
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
    #[doc = "Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `numCheckpoints` function with signature `numCheckpoints(address)` and selector `[111, 207, 255, 69]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "numCheckpoints", abi = "numCheckpoints(address)")]
    pub struct NumCheckpointsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[213, 5, 172, 207]`"]
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub raw_amount: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setMinter` function with signature `setMinter(address)` and selector `[252, 163, 181, 170]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMinter", abi = "setMinter(address)")]
    pub struct SetMinterCall {
        pub minter: ethers::core::types::Address,
    }
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
        pub dst: ethers::core::types::Address,
        pub raw_amount: ethers::core::types::U256,
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
        pub src: ethers::core::types::Address,
        pub dst: ethers::core::types::Address,
        pub raw_amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniCalls {
        DelegationTypehash(DelegationTypehashCall),
        DomainTypehash(DomainTypehashCall),
        PermitTypehash(PermitTypehashCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Checkpoints(CheckpointsCall),
        Decimals(DecimalsCall),
        Delegate(DelegateCall),
        DelegateBySig(DelegateBySigCall),
        Delegates(DelegatesCall),
        GetCurrentVotes(GetCurrentVotesCall),
        GetPriorVotes(GetPriorVotesCall),
        MinimumTimeBetweenMints(MinimumTimeBetweenMintsCall),
        Mint(MintCall),
        MintCap(MintCapCall),
        Minter(MinterCall),
        MintingAllowedAfter(MintingAllowedAfterCall),
        Name(NameCall),
        Nonces(NoncesCall),
        NumCheckpoints(NumCheckpointsCall),
        Permit(PermitCall),
        SetMinter(SetMinterCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for UniCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DelegationTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::DelegationTypehash(decoded));
            }
            if let Ok(decoded) =
                <DomainTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::DomainTypehash(decoded));
            }
            if let Ok(decoded) =
                <PermitTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::PermitTypehash(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <CheckpointsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::Checkpoints(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DelegateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::Delegate(decoded));
            }
            if let Ok(decoded) =
                <DelegateBySigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::DelegateBySig(decoded));
            }
            if let Ok(decoded) =
                <DelegatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::Delegates(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentVotesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::GetCurrentVotes(decoded));
            }
            if let Ok(decoded) =
                <GetPriorVotesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::GetPriorVotes(decoded));
            }
            if let Ok(decoded) =
                <MinimumTimeBetweenMintsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::MinimumTimeBetweenMints(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniCalls::Mint(decoded));
            }
            if let Ok(decoded) =
                <MintCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::MintCap(decoded));
            }
            if let Ok(decoded) = <MinterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::Minter(decoded));
            }
            if let Ok(decoded) =
                <MintingAllowedAfterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::MintingAllowedAfter(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniCalls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::Nonces(decoded));
            }
            if let Ok(decoded) =
                <NumCheckpointsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::NumCheckpoints(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::Permit(decoded));
            }
            if let Ok(decoded) =
                <SetMinterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::SetMinter(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniCalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UniCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniCalls::DelegationTypehash(element) => element.encode(),
                UniCalls::DomainTypehash(element) => element.encode(),
                UniCalls::PermitTypehash(element) => element.encode(),
                UniCalls::Allowance(element) => element.encode(),
                UniCalls::Approve(element) => element.encode(),
                UniCalls::BalanceOf(element) => element.encode(),
                UniCalls::Checkpoints(element) => element.encode(),
                UniCalls::Decimals(element) => element.encode(),
                UniCalls::Delegate(element) => element.encode(),
                UniCalls::DelegateBySig(element) => element.encode(),
                UniCalls::Delegates(element) => element.encode(),
                UniCalls::GetCurrentVotes(element) => element.encode(),
                UniCalls::GetPriorVotes(element) => element.encode(),
                UniCalls::MinimumTimeBetweenMints(element) => element.encode(),
                UniCalls::Mint(element) => element.encode(),
                UniCalls::MintCap(element) => element.encode(),
                UniCalls::Minter(element) => element.encode(),
                UniCalls::MintingAllowedAfter(element) => element.encode(),
                UniCalls::Name(element) => element.encode(),
                UniCalls::Nonces(element) => element.encode(),
                UniCalls::NumCheckpoints(element) => element.encode(),
                UniCalls::Permit(element) => element.encode(),
                UniCalls::SetMinter(element) => element.encode(),
                UniCalls::Symbol(element) => element.encode(),
                UniCalls::TotalSupply(element) => element.encode(),
                UniCalls::Transfer(element) => element.encode(),
                UniCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UniCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniCalls::DelegationTypehash(element) => element.fmt(f),
                UniCalls::DomainTypehash(element) => element.fmt(f),
                UniCalls::PermitTypehash(element) => element.fmt(f),
                UniCalls::Allowance(element) => element.fmt(f),
                UniCalls::Approve(element) => element.fmt(f),
                UniCalls::BalanceOf(element) => element.fmt(f),
                UniCalls::Checkpoints(element) => element.fmt(f),
                UniCalls::Decimals(element) => element.fmt(f),
                UniCalls::Delegate(element) => element.fmt(f),
                UniCalls::DelegateBySig(element) => element.fmt(f),
                UniCalls::Delegates(element) => element.fmt(f),
                UniCalls::GetCurrentVotes(element) => element.fmt(f),
                UniCalls::GetPriorVotes(element) => element.fmt(f),
                UniCalls::MinimumTimeBetweenMints(element) => element.fmt(f),
                UniCalls::Mint(element) => element.fmt(f),
                UniCalls::MintCap(element) => element.fmt(f),
                UniCalls::Minter(element) => element.fmt(f),
                UniCalls::MintingAllowedAfter(element) => element.fmt(f),
                UniCalls::Name(element) => element.fmt(f),
                UniCalls::Nonces(element) => element.fmt(f),
                UniCalls::NumCheckpoints(element) => element.fmt(f),
                UniCalls::Permit(element) => element.fmt(f),
                UniCalls::SetMinter(element) => element.fmt(f),
                UniCalls::Symbol(element) => element.fmt(f),
                UniCalls::TotalSupply(element) => element.fmt(f),
                UniCalls::Transfer(element) => element.fmt(f),
                UniCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DelegationTypehashCall> for UniCalls {
        fn from(var: DelegationTypehashCall) -> Self {
            UniCalls::DelegationTypehash(var)
        }
    }
    impl ::std::convert::From<DomainTypehashCall> for UniCalls {
        fn from(var: DomainTypehashCall) -> Self {
            UniCalls::DomainTypehash(var)
        }
    }
    impl ::std::convert::From<PermitTypehashCall> for UniCalls {
        fn from(var: PermitTypehashCall) -> Self {
            UniCalls::PermitTypehash(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for UniCalls {
        fn from(var: AllowanceCall) -> Self {
            UniCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for UniCalls {
        fn from(var: ApproveCall) -> Self {
            UniCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for UniCalls {
        fn from(var: BalanceOfCall) -> Self {
            UniCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<CheckpointsCall> for UniCalls {
        fn from(var: CheckpointsCall) -> Self {
            UniCalls::Checkpoints(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for UniCalls {
        fn from(var: DecimalsCall) -> Self {
            UniCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DelegateCall> for UniCalls {
        fn from(var: DelegateCall) -> Self {
            UniCalls::Delegate(var)
        }
    }
    impl ::std::convert::From<DelegateBySigCall> for UniCalls {
        fn from(var: DelegateBySigCall) -> Self {
            UniCalls::DelegateBySig(var)
        }
    }
    impl ::std::convert::From<DelegatesCall> for UniCalls {
        fn from(var: DelegatesCall) -> Self {
            UniCalls::Delegates(var)
        }
    }
    impl ::std::convert::From<GetCurrentVotesCall> for UniCalls {
        fn from(var: GetCurrentVotesCall) -> Self {
            UniCalls::GetCurrentVotes(var)
        }
    }
    impl ::std::convert::From<GetPriorVotesCall> for UniCalls {
        fn from(var: GetPriorVotesCall) -> Self {
            UniCalls::GetPriorVotes(var)
        }
    }
    impl ::std::convert::From<MinimumTimeBetweenMintsCall> for UniCalls {
        fn from(var: MinimumTimeBetweenMintsCall) -> Self {
            UniCalls::MinimumTimeBetweenMints(var)
        }
    }
    impl ::std::convert::From<MintCall> for UniCalls {
        fn from(var: MintCall) -> Self {
            UniCalls::Mint(var)
        }
    }
    impl ::std::convert::From<MintCapCall> for UniCalls {
        fn from(var: MintCapCall) -> Self {
            UniCalls::MintCap(var)
        }
    }
    impl ::std::convert::From<MinterCall> for UniCalls {
        fn from(var: MinterCall) -> Self {
            UniCalls::Minter(var)
        }
    }
    impl ::std::convert::From<MintingAllowedAfterCall> for UniCalls {
        fn from(var: MintingAllowedAfterCall) -> Self {
            UniCalls::MintingAllowedAfter(var)
        }
    }
    impl ::std::convert::From<NameCall> for UniCalls {
        fn from(var: NameCall) -> Self {
            UniCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for UniCalls {
        fn from(var: NoncesCall) -> Self {
            UniCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<NumCheckpointsCall> for UniCalls {
        fn from(var: NumCheckpointsCall) -> Self {
            UniCalls::NumCheckpoints(var)
        }
    }
    impl ::std::convert::From<PermitCall> for UniCalls {
        fn from(var: PermitCall) -> Self {
            UniCalls::Permit(var)
        }
    }
    impl ::std::convert::From<SetMinterCall> for UniCalls {
        fn from(var: SetMinterCall) -> Self {
            UniCalls::SetMinter(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for UniCalls {
        fn from(var: SymbolCall) -> Self {
            UniCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for UniCalls {
        fn from(var: TotalSupplyCall) -> Self {
            UniCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for UniCalls {
        fn from(var: TransferCall) -> Self {
            UniCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for UniCalls {
        fn from(var: TransferFromCall) -> Self {
            UniCalls::TransferFrom(var)
        }
    }
    #[doc = "Container type for all return fields from the `DELEGATION_TYPEHASH` function with signature `DELEGATION_TYPEHASH()` and selector `[231, 163, 36, 220]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DelegationTypehashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `[32, 96, 107, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DomainTypehashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `[48, 173, 248, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PermitTypehashReturn(pub [u8; 32]);
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
    #[doc = "Container type for all return fields from the `checkpoints` function with signature `checkpoints(address,uint32)` and selector `[241, 18, 126, 216]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CheckpointsReturn {
        pub from_block: u32,
        pub votes: u128,
    }
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
    #[doc = "Container type for all return fields from the `delegates` function with signature `delegates(address)` and selector `[88, 124, 222, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DelegatesReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getCurrentVotes` function with signature `getCurrentVotes(address)` and selector `[180, 181, 234, 87]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCurrentVotesReturn(pub u128);
    #[doc = "Container type for all return fields from the `getPriorVotes` function with signature `getPriorVotes(address,uint256)` and selector `[120, 45, 111, 225]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPriorVotesReturn(pub u128);
    #[doc = "Container type for all return fields from the `minimumTimeBetweenMints` function with signature `minimumTimeBetweenMints()` and selector `[92, 17, 214, 47]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MinimumTimeBetweenMintsReturn(pub u32);
    #[doc = "Container type for all return fields from the `mintCap` function with signature `mintCap()` and selector `[118, 199, 28, 161]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MintCapReturn(pub u8);
    #[doc = "Container type for all return fields from the `minter` function with signature `minter()` and selector `[7, 84, 97, 114]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MinterReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `mintingAllowedAfter` function with signature `mintingAllowedAfter()` and selector `[48, 179, 108, 239]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MintingAllowedAfterReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NoncesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `numCheckpoints` function with signature `numCheckpoints(address)` and selector `[111, 207, 255, 69]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NumCheckpointsReturn(pub u32);
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
