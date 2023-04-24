pub use i_polygon_zk_evm_bridge::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod i_polygon_zk_evm_bridge {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"AlreadyClaimed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AmountDoesNotMatchMsgValue\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"DestinationNetworkInvalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"EtherTransferFailed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"GlobalExitRootInvalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidSmtProof\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MessageFailed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MsgValueNotZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotValidAmount\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotValidOwner\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotValidSignature\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotValidSpender\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyPolygonZkEVM\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"activateEmergencyState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"permitData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"bridgeAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"bridgeMessage\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32[32]\",\"name\":\"smtProof\",\"type\":\"bytes32[32]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"mainnetExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"rollupExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32[32]\",\"name\":\"smtProof\",\"type\":\"bytes32[32]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"mainnetExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"rollupExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"originNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"originAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"destinationNetwork\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destinationAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"metadata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimMessage\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deactivateEmergencyState\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IPOLYGONZKEVMBRIDGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IPolygonZkEVMBridge<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IPolygonZkEVMBridge<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IPolygonZkEVMBridge<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IPolygonZkEVMBridge<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IPolygonZkEVMBridge<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IPolygonZkEVMBridge))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IPolygonZkEVMBridge<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IPOLYGONZKEVMBRIDGE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `activateEmergencyState` (0x2072f6c5) function
        pub fn activate_emergency_state(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 114, 246, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridgeAsset` (0x0871e971) function
        pub fn bridge_asset(
            &self,
            token: ::ethers::core::types::Address,
            destination_network: u32,
            destination_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            permit_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
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
        ///Calls the contract's `bridgeMessage` (0xd96a15f7) function
        pub fn bridge_message(
            &self,
            destination_network: u32,
            destination_address: ::ethers::core::types::Address,
            metadata: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [217, 106, 21, 247],
                    (destination_network, destination_address, metadata),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimAsset` (0x2cffd02e) function
        pub fn claim_asset(
            &self,
            smt_proof: [[u8; 32]; 32],
            index: u32,
            mainnet_exit_root: [u8; 32],
            rollup_exit_root: [u8; 32],
            origin_network: u32,
            origin_token_address: ::ethers::core::types::Address,
            destination_network: u32,
            destination_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            metadata: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
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
        ///Calls the contract's `claimMessage` (0x2d2c9d94) function
        pub fn claim_message(
            &self,
            smt_proof: [[u8; 32]; 32],
            index: u32,
            mainnet_exit_root: [u8; 32],
            rollup_exit_root: [u8; 32],
            origin_network: u32,
            origin_address: ::ethers::core::types::Address,
            destination_network: u32,
            destination_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            metadata: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
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
        ///Calls the contract's `deactivateEmergencyState` (0xdbc16976) function
        pub fn deactivate_emergency_state(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 193, 105, 118], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IPolygonZkEVMBridge<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AlreadyClaimed` with signature `AlreadyClaimed()` and selector `0x646cf558`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "AlreadyClaimed", abi = "AlreadyClaimed()")]
    pub struct AlreadyClaimed;
    ///Custom Error type `AmountDoesNotMatchMsgValue` with signature `AmountDoesNotMatchMsgValue()` and selector `0xb89240f5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "AmountDoesNotMatchMsgValue",
        abi = "AmountDoesNotMatchMsgValue()"
    )]
    pub struct AmountDoesNotMatchMsgValue;
    ///Custom Error type `DestinationNetworkInvalid` with signature `DestinationNetworkInvalid()` and selector `0x0595ea2e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "DestinationNetworkInvalid",
        abi = "DestinationNetworkInvalid()"
    )]
    pub struct DestinationNetworkInvalid;
    ///Custom Error type `EtherTransferFailed` with signature `EtherTransferFailed()` and selector `0x6747a288`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "EtherTransferFailed", abi = "EtherTransferFailed()")]
    pub struct EtherTransferFailed;
    ///Custom Error type `GlobalExitRootInvalid` with signature `GlobalExitRootInvalid()` and selector `0x002f6fad`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "GlobalExitRootInvalid", abi = "GlobalExitRootInvalid()")]
    pub struct GlobalExitRootInvalid;
    ///Custom Error type `InvalidSmtProof` with signature `InvalidSmtProof()` and selector `0xe0417cec`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidSmtProof", abi = "InvalidSmtProof()")]
    pub struct InvalidSmtProof;
    ///Custom Error type `MessageFailed` with signature `MessageFailed()` and selector `0x37e391c3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "MessageFailed", abi = "MessageFailed()")]
    pub struct MessageFailed;
    ///Custom Error type `MsgValueNotZero` with signature `MsgValueNotZero()` and selector `0x798ee6f1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "MsgValueNotZero", abi = "MsgValueNotZero()")]
    pub struct MsgValueNotZero;
    ///Custom Error type `NotValidAmount` with signature `NotValidAmount()` and selector `0x03fffc4b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotValidAmount", abi = "NotValidAmount()")]
    pub struct NotValidAmount;
    ///Custom Error type `NotValidOwner` with signature `NotValidOwner()` and selector `0x912ecce7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotValidOwner", abi = "NotValidOwner()")]
    pub struct NotValidOwner;
    ///Custom Error type `NotValidSignature` with signature `NotValidSignature()` and selector `0xe282c0ba`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotValidSignature", abi = "NotValidSignature()")]
    pub struct NotValidSignature;
    ///Custom Error type `NotValidSpender` with signature `NotValidSpender()` and selector `0x750643af`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotValidSpender", abi = "NotValidSpender()")]
    pub struct NotValidSpender;
    ///Custom Error type `OnlyPolygonZkEVM` with signature `OnlyPolygonZkEVM()` and selector `0xe2e8106b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "OnlyPolygonZkEVM", abi = "OnlyPolygonZkEVM()")]
    pub struct OnlyPolygonZkEVM;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPolygonZkEVMBridgeErrors {
        AlreadyClaimed(AlreadyClaimed),
        AmountDoesNotMatchMsgValue(AmountDoesNotMatchMsgValue),
        DestinationNetworkInvalid(DestinationNetworkInvalid),
        EtherTransferFailed(EtherTransferFailed),
        GlobalExitRootInvalid(GlobalExitRootInvalid),
        InvalidSmtProof(InvalidSmtProof),
        MessageFailed(MessageFailed),
        MsgValueNotZero(MsgValueNotZero),
        NotValidAmount(NotValidAmount),
        NotValidOwner(NotValidOwner),
        NotValidSignature(NotValidSignature),
        NotValidSpender(NotValidSpender),
        OnlyPolygonZkEVM(OnlyPolygonZkEVM),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IPolygonZkEVMBridgeErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AlreadyClaimed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AlreadyClaimed(decoded));
            }
            if let Ok(decoded) =
                <AmountDoesNotMatchMsgValue as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AmountDoesNotMatchMsgValue(decoded));
            }
            if let Ok(decoded) =
                <DestinationNetworkInvalid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DestinationNetworkInvalid(decoded));
            }
            if let Ok(decoded) =
                <EtherTransferFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EtherTransferFailed(decoded));
            }
            if let Ok(decoded) =
                <GlobalExitRootInvalid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GlobalExitRootInvalid(decoded));
            }
            if let Ok(decoded) = <InvalidSmtProof as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSmtProof(decoded));
            }
            if let Ok(decoded) = <MessageFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MessageFailed(decoded));
            }
            if let Ok(decoded) = <MsgValueNotZero as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MsgValueNotZero(decoded));
            }
            if let Ok(decoded) = <NotValidAmount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotValidAmount(decoded));
            }
            if let Ok(decoded) = <NotValidOwner as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotValidOwner(decoded));
            }
            if let Ok(decoded) = <NotValidSignature as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NotValidSignature(decoded));
            }
            if let Ok(decoded) = <NotValidSpender as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotValidSpender(decoded));
            }
            if let Ok(decoded) = <OnlyPolygonZkEVM as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyPolygonZkEVM(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPolygonZkEVMBridgeErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AlreadyClaimed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AmountDoesNotMatchMsgValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DestinationNetworkInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EtherTransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GlobalExitRootInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSmtProof(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MessageFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MsgValueNotZero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotValidAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotValidOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotValidSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotValidSpender(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyPolygonZkEVM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IPolygonZkEVMBridgeErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AlreadyClaimed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AmountDoesNotMatchMsgValue as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <DestinationNetworkInvalid as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <EtherTransferFailed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <GlobalExitRootInvalid as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidSmtProof as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <MessageFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <MsgValueNotZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NotValidAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NotValidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotValidSignature as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <NotValidSpender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <OnlyPolygonZkEVM as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IPolygonZkEVMBridgeErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AlreadyClaimed(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountDoesNotMatchMsgValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::DestinationNetworkInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::EtherTransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::GlobalExitRootInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSmtProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::MessageFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::MsgValueNotZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotValidAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotValidOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotValidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotValidSpender(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyPolygonZkEVM(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IPolygonZkEVMBridgeErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AlreadyClaimed> for IPolygonZkEVMBridgeErrors {
        fn from(value: AlreadyClaimed) -> Self {
            Self::AlreadyClaimed(value)
        }
    }
    impl ::core::convert::From<AmountDoesNotMatchMsgValue> for IPolygonZkEVMBridgeErrors {
        fn from(value: AmountDoesNotMatchMsgValue) -> Self {
            Self::AmountDoesNotMatchMsgValue(value)
        }
    }
    impl ::core::convert::From<DestinationNetworkInvalid> for IPolygonZkEVMBridgeErrors {
        fn from(value: DestinationNetworkInvalid) -> Self {
            Self::DestinationNetworkInvalid(value)
        }
    }
    impl ::core::convert::From<EtherTransferFailed> for IPolygonZkEVMBridgeErrors {
        fn from(value: EtherTransferFailed) -> Self {
            Self::EtherTransferFailed(value)
        }
    }
    impl ::core::convert::From<GlobalExitRootInvalid> for IPolygonZkEVMBridgeErrors {
        fn from(value: GlobalExitRootInvalid) -> Self {
            Self::GlobalExitRootInvalid(value)
        }
    }
    impl ::core::convert::From<InvalidSmtProof> for IPolygonZkEVMBridgeErrors {
        fn from(value: InvalidSmtProof) -> Self {
            Self::InvalidSmtProof(value)
        }
    }
    impl ::core::convert::From<MessageFailed> for IPolygonZkEVMBridgeErrors {
        fn from(value: MessageFailed) -> Self {
            Self::MessageFailed(value)
        }
    }
    impl ::core::convert::From<MsgValueNotZero> for IPolygonZkEVMBridgeErrors {
        fn from(value: MsgValueNotZero) -> Self {
            Self::MsgValueNotZero(value)
        }
    }
    impl ::core::convert::From<NotValidAmount> for IPolygonZkEVMBridgeErrors {
        fn from(value: NotValidAmount) -> Self {
            Self::NotValidAmount(value)
        }
    }
    impl ::core::convert::From<NotValidOwner> for IPolygonZkEVMBridgeErrors {
        fn from(value: NotValidOwner) -> Self {
            Self::NotValidOwner(value)
        }
    }
    impl ::core::convert::From<NotValidSignature> for IPolygonZkEVMBridgeErrors {
        fn from(value: NotValidSignature) -> Self {
            Self::NotValidSignature(value)
        }
    }
    impl ::core::convert::From<NotValidSpender> for IPolygonZkEVMBridgeErrors {
        fn from(value: NotValidSpender) -> Self {
            Self::NotValidSpender(value)
        }
    }
    impl ::core::convert::From<OnlyPolygonZkEVM> for IPolygonZkEVMBridgeErrors {
        fn from(value: OnlyPolygonZkEVM) -> Self {
            Self::OnlyPolygonZkEVM(value)
        }
    }
    ///Container type for all input parameters for the `activateEmergencyState` function with signature `activateEmergencyState()` and selector `0x2072f6c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "activateEmergencyState", abi = "activateEmergencyState()")]
    pub struct ActivateEmergencyStateCall;
    ///Container type for all input parameters for the `bridgeAsset` function with signature `bridgeAsset(address,uint32,address,uint256,bytes)` and selector `0x0871e971`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "bridgeAsset",
        abi = "bridgeAsset(address,uint32,address,uint256,bytes)"
    )]
    pub struct BridgeAssetCall {
        pub token: ::ethers::core::types::Address,
        pub destination_network: u32,
        pub destination_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub permit_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `bridgeMessage` function with signature `bridgeMessage(uint32,address,bytes)` and selector `0xd96a15f7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "bridgeMessage", abi = "bridgeMessage(uint32,address,bytes)")]
    pub struct BridgeMessageCall {
        pub destination_network: u32,
        pub destination_address: ::ethers::core::types::Address,
        pub metadata: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `claimAsset` function with signature `claimAsset(bytes32[32],uint32,bytes32,bytes32,uint32,address,uint32,address,uint256,bytes)` and selector `0x2cffd02e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "claimAsset",
        abi = "claimAsset(bytes32[32],uint32,bytes32,bytes32,uint32,address,uint32,address,uint256,bytes)"
    )]
    pub struct ClaimAssetCall {
        pub smt_proof: [[u8; 32]; 32],
        pub index: u32,
        pub mainnet_exit_root: [u8; 32],
        pub rollup_exit_root: [u8; 32],
        pub origin_network: u32,
        pub origin_token_address: ::ethers::core::types::Address,
        pub destination_network: u32,
        pub destination_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub metadata: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `claimMessage` function with signature `claimMessage(bytes32[32],uint32,bytes32,bytes32,uint32,address,uint32,address,uint256,bytes)` and selector `0x2d2c9d94`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "claimMessage",
        abi = "claimMessage(bytes32[32],uint32,bytes32,bytes32,uint32,address,uint32,address,uint256,bytes)"
    )]
    pub struct ClaimMessageCall {
        pub smt_proof: [[u8; 32]; 32],
        pub index: u32,
        pub mainnet_exit_root: [u8; 32],
        pub rollup_exit_root: [u8; 32],
        pub origin_network: u32,
        pub origin_address: ::ethers::core::types::Address,
        pub destination_network: u32,
        pub destination_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub metadata: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deactivateEmergencyState` function with signature `deactivateEmergencyState()` and selector `0xdbc16976`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "deactivateEmergencyState", abi = "deactivateEmergencyState()")]
    pub struct DeactivateEmergencyStateCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPolygonZkEVMBridgeCalls {
        ActivateEmergencyState(ActivateEmergencyStateCall),
        BridgeAsset(BridgeAssetCall),
        BridgeMessage(BridgeMessageCall),
        ClaimAsset(ClaimAssetCall),
        ClaimMessage(ClaimMessageCall),
        DeactivateEmergencyState(DeactivateEmergencyStateCall),
    }
    impl ::ethers::core::abi::AbiDecode for IPolygonZkEVMBridgeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ActivateEmergencyStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ActivateEmergencyState(decoded));
            }
            if let Ok(decoded) = <BridgeAssetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BridgeAsset(decoded));
            }
            if let Ok(decoded) = <BridgeMessageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BridgeMessage(decoded));
            }
            if let Ok(decoded) = <ClaimAssetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClaimAsset(decoded));
            }
            if let Ok(decoded) = <ClaimMessageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ClaimMessage(decoded));
            }
            if let Ok(decoded) =
                <DeactivateEmergencyStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeactivateEmergencyState(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPolygonZkEVMBridgeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ActivateEmergencyState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BridgeAsset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BridgeMessage(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClaimAsset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClaimMessage(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeactivateEmergencyState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IPolygonZkEVMBridgeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActivateEmergencyState(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeactivateEmergencyState(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ActivateEmergencyStateCall> for IPolygonZkEVMBridgeCalls {
        fn from(value: ActivateEmergencyStateCall) -> Self {
            Self::ActivateEmergencyState(value)
        }
    }
    impl ::core::convert::From<BridgeAssetCall> for IPolygonZkEVMBridgeCalls {
        fn from(value: BridgeAssetCall) -> Self {
            Self::BridgeAsset(value)
        }
    }
    impl ::core::convert::From<BridgeMessageCall> for IPolygonZkEVMBridgeCalls {
        fn from(value: BridgeMessageCall) -> Self {
            Self::BridgeMessage(value)
        }
    }
    impl ::core::convert::From<ClaimAssetCall> for IPolygonZkEVMBridgeCalls {
        fn from(value: ClaimAssetCall) -> Self {
            Self::ClaimAsset(value)
        }
    }
    impl ::core::convert::From<ClaimMessageCall> for IPolygonZkEVMBridgeCalls {
        fn from(value: ClaimMessageCall) -> Self {
            Self::ClaimMessage(value)
        }
    }
    impl ::core::convert::From<DeactivateEmergencyStateCall> for IPolygonZkEVMBridgeCalls {
        fn from(value: DeactivateEmergencyStateCall) -> Self {
            Self::DeactivateEmergencyState(value)
        }
    }
}
