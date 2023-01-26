pub use dai::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod dai {
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
    #[doc = "Dai was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainId_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"guy\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"wad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"sig\",\"type\":\"bytes4\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"usr\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"arg1\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"arg2\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LogNote\",\"outputs\":[],\"anonymous\":true},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"wad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"PERMIT_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"usr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"usr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"guy\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deny\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"usr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"move\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"holder\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"allowed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"permit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"usr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pull\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"usr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"push\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"guy\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rely\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"wards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static DAI_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Dai<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Dai<M> {
        fn clone(&self) -> Self {
            Dai(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Dai<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Dai<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Dai))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Dai<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), DAI_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
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
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            usr: ethers::core::types::Address,
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (usr, wad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x9dc29fac) function"]
        pub fn burn(
            &self,
            usr: ethers::core::types::Address,
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 194, 159, 172], (usr, wad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deny` (0x9c52a7f1) function"]
        pub fn deny(
            &self,
            guy: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 82, 167, 241], guy)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x40c10f19) function"]
        pub fn mint(
            &self,
            usr: ethers::core::types::Address,
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (usr, wad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `move` (0xbb35783b) function"]
        pub fn move_(
            &self,
            src: ethers::core::types::Address,
            dst: ethers::core::types::Address,
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 53, 120, 59], (src, dst, wad))
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
        #[doc = "Calls the contract's `permit` (0x8fcbaf0c) function"]
        pub fn permit(
            &self,
            holder: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
            expiry: ethers::core::types::U256,
            allowed: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [143, 203, 175, 12],
                    (holder, spender, nonce, expiry, allowed, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pull` (0xf2d5d56b) function"]
        pub fn pull(
            &self,
            usr: ethers::core::types::Address,
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 213, 213, 107], (usr, wad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `push` (0xb753a98c) function"]
        pub fn push(
            &self,
            usr: ethers::core::types::Address,
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 83, 169, 140], (usr, wad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rely` (0x65fae35e) function"]
        pub fn rely(
            &self,
            guy: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 250, 227, 94], guy)
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
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (dst, wad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            src: ethers::core::types::Address,
            dst: ethers::core::types::Address,
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (src, dst, wad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wards` (0xbf353dbb) function"]
        pub fn wards(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([191, 53, 61, 187], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogNote` event"]
        pub fn log_note_filter(&self) -> ethers::contract::builders::Event<M, LogNoteFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, DaiEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Dai<M> {
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
        pub src: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub guy: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
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
        name = "LogNote",
        abi = "LogNote(bytes4,address,bytes32,bytes32,bytes) anonymous"
    )]
    pub struct LogNoteFilter {
        #[ethevent(indexed)]
        pub sig: [u8; 4],
        #[ethevent(indexed)]
        pub usr: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub arg_1: [u8; 32],
        #[ethevent(indexed)]
        pub arg_2: [u8; 32],
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub src: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub dst: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum DaiEvents {
        ApprovalFilter(ApprovalFilter),
        LogNoteFilter(LogNoteFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for DaiEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(DaiEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = LogNoteFilter::decode_log(log) {
                return Ok(DaiEvents::LogNoteFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(DaiEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for DaiEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DaiEvents::ApprovalFilter(element) => element.fmt(f),
                DaiEvents::LogNoteFilter(element) => element.fmt(f),
                DaiEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
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
    pub struct AllowanceCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
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
        pub usr: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
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
    pub struct BalanceOfCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(address,uint256)` and selector `[157, 194, 159, 172]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burn", abi = "burn(address,uint256)")]
    pub struct BurnCall {
        pub usr: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `deny` function with signature `deny(address)` and selector `[156, 82, 167, 241]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deny", abi = "deny(address)")]
    pub struct DenyCall {
        pub guy: ethers::core::types::Address,
    }
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
        pub usr: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `move` function with signature `move(address,address,uint256)` and selector `[187, 53, 120, 59]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "move", abi = "move(address,address,uint256)")]
    pub struct MoveCall {
        pub src: ethers::core::types::Address,
        pub dst: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,bool,uint8,bytes32,bytes32)` and selector `[143, 203, 175, 12]`"]
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
        abi = "permit(address,address,uint256,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub holder: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
        pub expiry: ethers::core::types::U256,
        pub allowed: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `pull` function with signature `pull(address,uint256)` and selector `[242, 213, 213, 107]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "pull", abi = "pull(address,uint256)")]
    pub struct PullCall {
        pub usr: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `push` function with signature `push(address,uint256)` and selector `[183, 83, 169, 140]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "push", abi = "push(address,uint256)")]
    pub struct PushCall {
        pub usr: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `rely` function with signature `rely(address)` and selector `[101, 250, 227, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "rely", abi = "rely(address)")]
    pub struct RelyCall {
        pub guy: ethers::core::types::Address,
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
        pub wad: ethers::core::types::U256,
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
        pub wad: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `version` function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    #[doc = "Container type for all input parameters for the `wards` function with signature `wards(address)` and selector `[191, 53, 61, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "wards", abi = "wards(address)")]
    pub struct WardsCall(pub ethers::core::types::Address);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum DaiCalls {
        DomainSeparator(DomainSeparatorCall),
        PermitTypehash(PermitTypehashCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        Deny(DenyCall),
        Mint(MintCall),
        Move(MoveCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        Pull(PullCall),
        Push(PushCall),
        Rely(RelyCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Version(VersionCall),
        Wards(WardsCall),
    }
    impl ethers::core::abi::AbiDecode for DaiCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <PermitTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::PermitTypehash(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DaiCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::Decimals(decoded));
            }
            if let Ok(decoded) = <DenyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DaiCalls::Deny(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DaiCalls::Mint(decoded));
            }
            if let Ok(decoded) = <MoveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DaiCalls::Move(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DaiCalls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::Nonces(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::Permit(decoded));
            }
            if let Ok(decoded) = <PullCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DaiCalls::Pull(decoded));
            }
            if let Ok(decoded) = <PushCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DaiCalls::Push(decoded));
            }
            if let Ok(decoded) = <RelyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DaiCalls::Rely(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::Version(decoded));
            }
            if let Ok(decoded) = <WardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DaiCalls::Wards(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for DaiCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                DaiCalls::DomainSeparator(element) => element.encode(),
                DaiCalls::PermitTypehash(element) => element.encode(),
                DaiCalls::Allowance(element) => element.encode(),
                DaiCalls::Approve(element) => element.encode(),
                DaiCalls::BalanceOf(element) => element.encode(),
                DaiCalls::Burn(element) => element.encode(),
                DaiCalls::Decimals(element) => element.encode(),
                DaiCalls::Deny(element) => element.encode(),
                DaiCalls::Mint(element) => element.encode(),
                DaiCalls::Move(element) => element.encode(),
                DaiCalls::Name(element) => element.encode(),
                DaiCalls::Nonces(element) => element.encode(),
                DaiCalls::Permit(element) => element.encode(),
                DaiCalls::Pull(element) => element.encode(),
                DaiCalls::Push(element) => element.encode(),
                DaiCalls::Rely(element) => element.encode(),
                DaiCalls::Symbol(element) => element.encode(),
                DaiCalls::TotalSupply(element) => element.encode(),
                DaiCalls::Transfer(element) => element.encode(),
                DaiCalls::TransferFrom(element) => element.encode(),
                DaiCalls::Version(element) => element.encode(),
                DaiCalls::Wards(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DaiCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DaiCalls::DomainSeparator(element) => element.fmt(f),
                DaiCalls::PermitTypehash(element) => element.fmt(f),
                DaiCalls::Allowance(element) => element.fmt(f),
                DaiCalls::Approve(element) => element.fmt(f),
                DaiCalls::BalanceOf(element) => element.fmt(f),
                DaiCalls::Burn(element) => element.fmt(f),
                DaiCalls::Decimals(element) => element.fmt(f),
                DaiCalls::Deny(element) => element.fmt(f),
                DaiCalls::Mint(element) => element.fmt(f),
                DaiCalls::Move(element) => element.fmt(f),
                DaiCalls::Name(element) => element.fmt(f),
                DaiCalls::Nonces(element) => element.fmt(f),
                DaiCalls::Permit(element) => element.fmt(f),
                DaiCalls::Pull(element) => element.fmt(f),
                DaiCalls::Push(element) => element.fmt(f),
                DaiCalls::Rely(element) => element.fmt(f),
                DaiCalls::Symbol(element) => element.fmt(f),
                DaiCalls::TotalSupply(element) => element.fmt(f),
                DaiCalls::Transfer(element) => element.fmt(f),
                DaiCalls::TransferFrom(element) => element.fmt(f),
                DaiCalls::Version(element) => element.fmt(f),
                DaiCalls::Wards(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for DaiCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            DaiCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<PermitTypehashCall> for DaiCalls {
        fn from(var: PermitTypehashCall) -> Self {
            DaiCalls::PermitTypehash(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for DaiCalls {
        fn from(var: AllowanceCall) -> Self {
            DaiCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for DaiCalls {
        fn from(var: ApproveCall) -> Self {
            DaiCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for DaiCalls {
        fn from(var: BalanceOfCall) -> Self {
            DaiCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for DaiCalls {
        fn from(var: BurnCall) -> Self {
            DaiCalls::Burn(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for DaiCalls {
        fn from(var: DecimalsCall) -> Self {
            DaiCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DenyCall> for DaiCalls {
        fn from(var: DenyCall) -> Self {
            DaiCalls::Deny(var)
        }
    }
    impl ::std::convert::From<MintCall> for DaiCalls {
        fn from(var: MintCall) -> Self {
            DaiCalls::Mint(var)
        }
    }
    impl ::std::convert::From<MoveCall> for DaiCalls {
        fn from(var: MoveCall) -> Self {
            DaiCalls::Move(var)
        }
    }
    impl ::std::convert::From<NameCall> for DaiCalls {
        fn from(var: NameCall) -> Self {
            DaiCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for DaiCalls {
        fn from(var: NoncesCall) -> Self {
            DaiCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<PermitCall> for DaiCalls {
        fn from(var: PermitCall) -> Self {
            DaiCalls::Permit(var)
        }
    }
    impl ::std::convert::From<PullCall> for DaiCalls {
        fn from(var: PullCall) -> Self {
            DaiCalls::Pull(var)
        }
    }
    impl ::std::convert::From<PushCall> for DaiCalls {
        fn from(var: PushCall) -> Self {
            DaiCalls::Push(var)
        }
    }
    impl ::std::convert::From<RelyCall> for DaiCalls {
        fn from(var: RelyCall) -> Self {
            DaiCalls::Rely(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for DaiCalls {
        fn from(var: SymbolCall) -> Self {
            DaiCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for DaiCalls {
        fn from(var: TotalSupplyCall) -> Self {
            DaiCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for DaiCalls {
        fn from(var: TransferCall) -> Self {
            DaiCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for DaiCalls {
        fn from(var: TransferFromCall) -> Self {
            DaiCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<VersionCall> for DaiCalls {
        fn from(var: VersionCall) -> Self {
            DaiCalls::Version(var)
        }
    }
    impl ::std::convert::From<WardsCall> for DaiCalls {
        fn from(var: WardsCall) -> Self {
            DaiCalls::Wards(var)
        }
    }
    #[doc = "Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
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
    #[doc = "Container type for all return fields from the `version` function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VersionReturn(pub String);
    #[doc = "Container type for all return fields from the `wards` function with signature `wards(address)` and selector `[191, 53, 61, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WardsReturn(pub ethers::core::types::U256);
}
