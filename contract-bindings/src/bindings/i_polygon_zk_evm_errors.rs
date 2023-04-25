pub use i_polygon_zk_evm_errors::*;
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
pub mod i_polygon_zk_evm_errors {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"BatchAlreadyVerified\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BatchNotSequencedOrNotSequenceEnd\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ExceedMaxVerifyBatches\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FinalNumBatchBelowLastVerifiedBatch\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FinalNumBatchDoesNotMatchPendingState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FinalPendingStateNumInvalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ForceBatchTimeoutNotExpired\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ForceBatchesOverflow\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ForcedDataDoesNotMatch\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"GlobalExitRootNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"HaltTimeoutNotExpired\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InitNumBatchAboveLastVerifiedBatch\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InitNumBatchDoesNotMatchPendingState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidProof\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidRangeBatchTimeTarget\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidRangeMultiplierBatchFee\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NewAccInputHashDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NewPendingStateTimeoutMustBeLower\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NewTrustedAggregatorTimeoutMustBeLower\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotEnoughMaticAmount\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OldAccInputHashDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OldStateRootDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyAdmin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyPendingAdmin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyTrustedAggregator\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyTrustedSequencer\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PendingStateDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PendingStateInvalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PendingStateNotConsolidable\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PendingStateTimeoutExceedHaltAggregationTimeout\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SequenceZeroBatches\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SequencedTimestampBelowForcedTimestamp\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SequencedTimestampInvalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"StoredRootMustBeDifferentThanNewRoot\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransactionsLengthAboveMax\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TrustedAggregatorTimeoutExceedHaltAggregationTimeout\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TrustedAggregatorTimeoutNotExpired\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IPOLYGONZKEVMERRORS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IPolygonZkEVMErrors<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IPolygonZkEVMErrors<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IPolygonZkEVMErrors<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IPolygonZkEVMErrors<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IPolygonZkEVMErrors<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IPolygonZkEVMErrors))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IPolygonZkEVMErrors<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IPOLYGONZKEVMERRORS_ABI.clone(),
                client,
            ))
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IPolygonZkEVMErrors<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BatchAlreadyVerified` with signature `BatchAlreadyVerified()` and selector `0x812a372d`
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
    #[etherror(name = "BatchAlreadyVerified", abi = "BatchAlreadyVerified()")]
    pub struct BatchAlreadyVerified;
    ///Custom Error type `BatchNotSequencedOrNotSequenceEnd` with signature `BatchNotSequencedOrNotSequenceEnd()` and selector `0x98c5c014`
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
        name = "BatchNotSequencedOrNotSequenceEnd",
        abi = "BatchNotSequencedOrNotSequenceEnd()"
    )]
    pub struct BatchNotSequencedOrNotSequenceEnd;
    ///Custom Error type `ExceedMaxVerifyBatches` with signature `ExceedMaxVerifyBatches()` and selector `0xb59f753a`
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
    #[etherror(name = "ExceedMaxVerifyBatches", abi = "ExceedMaxVerifyBatches()")]
    pub struct ExceedMaxVerifyBatches;
    ///Custom Error type `FinalNumBatchBelowLastVerifiedBatch` with signature `FinalNumBatchBelowLastVerifiedBatch()` and selector `0xb9b18f57`
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
        name = "FinalNumBatchBelowLastVerifiedBatch",
        abi = "FinalNumBatchBelowLastVerifiedBatch()"
    )]
    pub struct FinalNumBatchBelowLastVerifiedBatch;
    ///Custom Error type `FinalNumBatchDoesNotMatchPendingState` with signature `FinalNumBatchDoesNotMatchPendingState()` and selector `0x32a2a77f`
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
        name = "FinalNumBatchDoesNotMatchPendingState",
        abi = "FinalNumBatchDoesNotMatchPendingState()"
    )]
    pub struct FinalNumBatchDoesNotMatchPendingState;
    ///Custom Error type `FinalPendingStateNumInvalid` with signature `FinalPendingStateNumInvalid()` and selector `0xbfa7079f`
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
        name = "FinalPendingStateNumInvalid",
        abi = "FinalPendingStateNumInvalid()"
    )]
    pub struct FinalPendingStateNumInvalid;
    ///Custom Error type `ForceBatchTimeoutNotExpired` with signature `ForceBatchTimeoutNotExpired()` and selector `0xc44a0821`
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
        name = "ForceBatchTimeoutNotExpired",
        abi = "ForceBatchTimeoutNotExpired()"
    )]
    pub struct ForceBatchTimeoutNotExpired;
    ///Custom Error type `ForceBatchesOverflow` with signature `ForceBatchesOverflow()` and selector `0xc630a00d`
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
    #[etherror(name = "ForceBatchesOverflow", abi = "ForceBatchesOverflow()")]
    pub struct ForceBatchesOverflow;
    ///Custom Error type `ForcedDataDoesNotMatch` with signature `ForcedDataDoesNotMatch()` and selector `0xce3d755e`
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
    #[etherror(name = "ForcedDataDoesNotMatch", abi = "ForcedDataDoesNotMatch()")]
    pub struct ForcedDataDoesNotMatch;
    ///Custom Error type `GlobalExitRootNotExist` with signature `GlobalExitRootNotExist()` and selector `0x73bd668d`
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
    #[etherror(name = "GlobalExitRootNotExist", abi = "GlobalExitRootNotExist()")]
    pub struct GlobalExitRootNotExist;
    ///Custom Error type `HaltTimeoutNotExpired` with signature `HaltTimeoutNotExpired()` and selector `0xd257555a`
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
    #[etherror(name = "HaltTimeoutNotExpired", abi = "HaltTimeoutNotExpired()")]
    pub struct HaltTimeoutNotExpired;
    ///Custom Error type `InitNumBatchAboveLastVerifiedBatch` with signature `InitNumBatchAboveLastVerifiedBatch()` and selector `0x1e56e9e2`
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
        name = "InitNumBatchAboveLastVerifiedBatch",
        abi = "InitNumBatchAboveLastVerifiedBatch()"
    )]
    pub struct InitNumBatchAboveLastVerifiedBatch;
    ///Custom Error type `InitNumBatchDoesNotMatchPendingState` with signature `InitNumBatchDoesNotMatchPendingState()` and selector `0x2bd2e3e7`
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
        name = "InitNumBatchDoesNotMatchPendingState",
        abi = "InitNumBatchDoesNotMatchPendingState()"
    )]
    pub struct InitNumBatchDoesNotMatchPendingState;
    ///Custom Error type `InvalidProof` with signature `InvalidProof()` and selector `0x09bde339`
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
    #[etherror(name = "InvalidProof", abi = "InvalidProof()")]
    pub struct InvalidProof;
    ///Custom Error type `InvalidRangeBatchTimeTarget` with signature `InvalidRangeBatchTimeTarget()` and selector `0xe067dfe8`
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
        name = "InvalidRangeBatchTimeTarget",
        abi = "InvalidRangeBatchTimeTarget()"
    )]
    pub struct InvalidRangeBatchTimeTarget;
    ///Custom Error type `InvalidRangeMultiplierBatchFee` with signature `InvalidRangeMultiplierBatchFee()` and selector `0x4c2533c8`
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
        name = "InvalidRangeMultiplierBatchFee",
        abi = "InvalidRangeMultiplierBatchFee()"
    )]
    pub struct InvalidRangeMultiplierBatchFee;
    ///Custom Error type `NewAccInputHashDoesNotExist` with signature `NewAccInputHashDoesNotExist()` and selector `0x66385b51`
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
        name = "NewAccInputHashDoesNotExist",
        abi = "NewAccInputHashDoesNotExist()"
    )]
    pub struct NewAccInputHashDoesNotExist;
    ///Custom Error type `NewPendingStateTimeoutMustBeLower` with signature `NewPendingStateTimeoutMustBeLower()` and selector `0x48a05a90`
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
        name = "NewPendingStateTimeoutMustBeLower",
        abi = "NewPendingStateTimeoutMustBeLower()"
    )]
    pub struct NewPendingStateTimeoutMustBeLower;
    ///Custom Error type `NewTrustedAggregatorTimeoutMustBeLower` with signature `NewTrustedAggregatorTimeoutMustBeLower()` and selector `0x401636df`
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
        name = "NewTrustedAggregatorTimeoutMustBeLower",
        abi = "NewTrustedAggregatorTimeoutMustBeLower()"
    )]
    pub struct NewTrustedAggregatorTimeoutMustBeLower;
    ///Custom Error type `NotEnoughMaticAmount` with signature `NotEnoughMaticAmount()` and selector `0x4732fdb5`
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
    #[etherror(name = "NotEnoughMaticAmount", abi = "NotEnoughMaticAmount()")]
    pub struct NotEnoughMaticAmount;
    ///Custom Error type `OldAccInputHashDoesNotExist` with signature `OldAccInputHashDoesNotExist()` and selector `0x6818c29e`
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
        name = "OldAccInputHashDoesNotExist",
        abi = "OldAccInputHashDoesNotExist()"
    )]
    pub struct OldAccInputHashDoesNotExist;
    ///Custom Error type `OldStateRootDoesNotExist` with signature `OldStateRootDoesNotExist()` and selector `0x4997b986`
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
    #[etherror(name = "OldStateRootDoesNotExist", abi = "OldStateRootDoesNotExist()")]
    pub struct OldStateRootDoesNotExist;
    ///Custom Error type `OnlyAdmin` with signature `OnlyAdmin()` and selector `0x47556579`
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
    #[etherror(name = "OnlyAdmin", abi = "OnlyAdmin()")]
    pub struct OnlyAdmin;
    ///Custom Error type `OnlyPendingAdmin` with signature `OnlyPendingAdmin()` and selector `0xd1ec4b23`
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
    #[etherror(name = "OnlyPendingAdmin", abi = "OnlyPendingAdmin()")]
    pub struct OnlyPendingAdmin;
    ///Custom Error type `OnlyTrustedAggregator` with signature `OnlyTrustedAggregator()` and selector `0xbbcbbc05`
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
    #[etherror(name = "OnlyTrustedAggregator", abi = "OnlyTrustedAggregator()")]
    pub struct OnlyTrustedAggregator;
    ///Custom Error type `OnlyTrustedSequencer` with signature `OnlyTrustedSequencer()` and selector `0x11e7be15`
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
    #[etherror(name = "OnlyTrustedSequencer", abi = "OnlyTrustedSequencer()")]
    pub struct OnlyTrustedSequencer;
    ///Custom Error type `PendingStateDoesNotExist` with signature `PendingStateDoesNotExist()` and selector `0xbb14c205`
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
    #[etherror(name = "PendingStateDoesNotExist", abi = "PendingStateDoesNotExist()")]
    pub struct PendingStateDoesNotExist;
    ///Custom Error type `PendingStateInvalid` with signature `PendingStateInvalid()` and selector `0xd086b70b`
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
    #[etherror(name = "PendingStateInvalid", abi = "PendingStateInvalid()")]
    pub struct PendingStateInvalid;
    ///Custom Error type `PendingStateNotConsolidable` with signature `PendingStateNotConsolidable()` and selector `0x0ce9e4a2`
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
        name = "PendingStateNotConsolidable",
        abi = "PendingStateNotConsolidable()"
    )]
    pub struct PendingStateNotConsolidable;
    ///Custom Error type `PendingStateTimeoutExceedHaltAggregationTimeout` with signature `PendingStateTimeoutExceedHaltAggregationTimeout()` and selector `0xcc965070`
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
        name = "PendingStateTimeoutExceedHaltAggregationTimeout",
        abi = "PendingStateTimeoutExceedHaltAggregationTimeout()"
    )]
    pub struct PendingStateTimeoutExceedHaltAggregationTimeout;
    ///Custom Error type `SequenceZeroBatches` with signature `SequenceZeroBatches()` and selector `0xcb591a5f`
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
    #[etherror(name = "SequenceZeroBatches", abi = "SequenceZeroBatches()")]
    pub struct SequenceZeroBatches;
    ///Custom Error type `SequencedTimestampBelowForcedTimestamp` with signature `SequencedTimestampBelowForcedTimestamp()` and selector `0x7f7ab872`
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
        name = "SequencedTimestampBelowForcedTimestamp",
        abi = "SequencedTimestampBelowForcedTimestamp()"
    )]
    pub struct SequencedTimestampBelowForcedTimestamp;
    ///Custom Error type `SequencedTimestampInvalid` with signature `SequencedTimestampInvalid()` and selector `0xea827916`
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
        name = "SequencedTimestampInvalid",
        abi = "SequencedTimestampInvalid()"
    )]
    pub struct SequencedTimestampInvalid;
    ///Custom Error type `StoredRootMustBeDifferentThanNewRoot` with signature `StoredRootMustBeDifferentThanNewRoot()` and selector `0xa47276bd`
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
        name = "StoredRootMustBeDifferentThanNewRoot",
        abi = "StoredRootMustBeDifferentThanNewRoot()"
    )]
    pub struct StoredRootMustBeDifferentThanNewRoot;
    ///Custom Error type `TransactionsLengthAboveMax` with signature `TransactionsLengthAboveMax()` and selector `0xa29a6c7c`
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
        name = "TransactionsLengthAboveMax",
        abi = "TransactionsLengthAboveMax()"
    )]
    pub struct TransactionsLengthAboveMax;
    ///Custom Error type `TrustedAggregatorTimeoutExceedHaltAggregationTimeout` with signature `TrustedAggregatorTimeoutExceedHaltAggregationTimeout()` and selector `0x1d06e879`
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
        name = "TrustedAggregatorTimeoutExceedHaltAggregationTimeout",
        abi = "TrustedAggregatorTimeoutExceedHaltAggregationTimeout()"
    )]
    pub struct TrustedAggregatorTimeoutExceedHaltAggregationTimeout;
    ///Custom Error type `TrustedAggregatorTimeoutNotExpired` with signature `TrustedAggregatorTimeoutNotExpired()` and selector `0x8a0704d3`
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
        name = "TrustedAggregatorTimeoutNotExpired",
        abi = "TrustedAggregatorTimeoutNotExpired()"
    )]
    pub struct TrustedAggregatorTimeoutNotExpired;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPolygonZkEVMErrorsErrors {
        BatchAlreadyVerified(BatchAlreadyVerified),
        BatchNotSequencedOrNotSequenceEnd(BatchNotSequencedOrNotSequenceEnd),
        ExceedMaxVerifyBatches(ExceedMaxVerifyBatches),
        FinalNumBatchBelowLastVerifiedBatch(FinalNumBatchBelowLastVerifiedBatch),
        FinalNumBatchDoesNotMatchPendingState(FinalNumBatchDoesNotMatchPendingState),
        FinalPendingStateNumInvalid(FinalPendingStateNumInvalid),
        ForceBatchTimeoutNotExpired(ForceBatchTimeoutNotExpired),
        ForceBatchesOverflow(ForceBatchesOverflow),
        ForcedDataDoesNotMatch(ForcedDataDoesNotMatch),
        GlobalExitRootNotExist(GlobalExitRootNotExist),
        HaltTimeoutNotExpired(HaltTimeoutNotExpired),
        InitNumBatchAboveLastVerifiedBatch(InitNumBatchAboveLastVerifiedBatch),
        InitNumBatchDoesNotMatchPendingState(InitNumBatchDoesNotMatchPendingState),
        InvalidProof(InvalidProof),
        InvalidRangeBatchTimeTarget(InvalidRangeBatchTimeTarget),
        InvalidRangeMultiplierBatchFee(InvalidRangeMultiplierBatchFee),
        NewAccInputHashDoesNotExist(NewAccInputHashDoesNotExist),
        NewPendingStateTimeoutMustBeLower(NewPendingStateTimeoutMustBeLower),
        NewTrustedAggregatorTimeoutMustBeLower(NewTrustedAggregatorTimeoutMustBeLower),
        NotEnoughMaticAmount(NotEnoughMaticAmount),
        OldAccInputHashDoesNotExist(OldAccInputHashDoesNotExist),
        OldStateRootDoesNotExist(OldStateRootDoesNotExist),
        OnlyAdmin(OnlyAdmin),
        OnlyPendingAdmin(OnlyPendingAdmin),
        OnlyTrustedAggregator(OnlyTrustedAggregator),
        OnlyTrustedSequencer(OnlyTrustedSequencer),
        PendingStateDoesNotExist(PendingStateDoesNotExist),
        PendingStateInvalid(PendingStateInvalid),
        PendingStateNotConsolidable(PendingStateNotConsolidable),
        PendingStateTimeoutExceedHaltAggregationTimeout(
            PendingStateTimeoutExceedHaltAggregationTimeout,
        ),
        SequenceZeroBatches(SequenceZeroBatches),
        SequencedTimestampBelowForcedTimestamp(SequencedTimestampBelowForcedTimestamp),
        SequencedTimestampInvalid(SequencedTimestampInvalid),
        StoredRootMustBeDifferentThanNewRoot(StoredRootMustBeDifferentThanNewRoot),
        TransactionsLengthAboveMax(TransactionsLengthAboveMax),
        TrustedAggregatorTimeoutExceedHaltAggregationTimeout(
            TrustedAggregatorTimeoutExceedHaltAggregationTimeout,
        ),
        TrustedAggregatorTimeoutNotExpired(TrustedAggregatorTimeoutNotExpired),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IPolygonZkEVMErrorsErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <BatchAlreadyVerified as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BatchAlreadyVerified(decoded));
            }
            if let Ok(decoded) =
                <BatchNotSequencedOrNotSequenceEnd as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BatchNotSequencedOrNotSequenceEnd(decoded));
            }
            if let Ok(decoded) =
                <ExceedMaxVerifyBatches as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExceedMaxVerifyBatches(decoded));
            }
            if let Ok(decoded) =
                <FinalNumBatchBelowLastVerifiedBatch as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::FinalNumBatchBelowLastVerifiedBatch(decoded));
            }
            if let Ok(decoded) =
                <FinalNumBatchDoesNotMatchPendingState as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::FinalNumBatchDoesNotMatchPendingState(decoded));
            }
            if let Ok(decoded) =
                <FinalPendingStateNumInvalid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FinalPendingStateNumInvalid(decoded));
            }
            if let Ok(decoded) =
                <ForceBatchTimeoutNotExpired as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ForceBatchTimeoutNotExpired(decoded));
            }
            if let Ok(decoded) =
                <ForceBatchesOverflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ForceBatchesOverflow(decoded));
            }
            if let Ok(decoded) =
                <ForcedDataDoesNotMatch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ForcedDataDoesNotMatch(decoded));
            }
            if let Ok(decoded) =
                <GlobalExitRootNotExist as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GlobalExitRootNotExist(decoded));
            }
            if let Ok(decoded) =
                <HaltTimeoutNotExpired as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HaltTimeoutNotExpired(decoded));
            }
            if let Ok(decoded) =
                <InitNumBatchAboveLastVerifiedBatch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InitNumBatchAboveLastVerifiedBatch(decoded));
            }
            if let Ok(decoded) =
                <InitNumBatchDoesNotMatchPendingState as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::InitNumBatchDoesNotMatchPendingState(decoded));
            }
            if let Ok(decoded) = <InvalidProof as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidProof(decoded));
            }
            if let Ok(decoded) =
                <InvalidRangeBatchTimeTarget as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidRangeBatchTimeTarget(decoded));
            }
            if let Ok(decoded) =
                <InvalidRangeMultiplierBatchFee as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidRangeMultiplierBatchFee(decoded));
            }
            if let Ok(decoded) =
                <NewAccInputHashDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NewAccInputHashDoesNotExist(decoded));
            }
            if let Ok(decoded) =
                <NewPendingStateTimeoutMustBeLower as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NewPendingStateTimeoutMustBeLower(decoded));
            }
            if let Ok(decoded) =
                <NewTrustedAggregatorTimeoutMustBeLower as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::NewTrustedAggregatorTimeoutMustBeLower(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughMaticAmount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NotEnoughMaticAmount(decoded));
            }
            if let Ok(decoded) =
                <OldAccInputHashDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OldAccInputHashDoesNotExist(decoded));
            }
            if let Ok(decoded) =
                <OldStateRootDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OldStateRootDoesNotExist(decoded));
            }
            if let Ok(decoded) = <OnlyAdmin as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyAdmin(decoded));
            }
            if let Ok(decoded) = <OnlyPendingAdmin as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyPendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <OnlyTrustedAggregator as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyTrustedAggregator(decoded));
            }
            if let Ok(decoded) =
                <OnlyTrustedSequencer as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyTrustedSequencer(decoded));
            }
            if let Ok(decoded) =
                <PendingStateDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingStateDoesNotExist(decoded));
            }
            if let Ok(decoded) =
                <PendingStateInvalid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingStateInvalid(decoded));
            }
            if let Ok(decoded) =
                <PendingStateNotConsolidable as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingStateNotConsolidable(decoded));
            }
            if let Ok(decoded)
                = <PendingStateTimeoutExceedHaltAggregationTimeout as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::PendingStateTimeoutExceedHaltAggregationTimeout(decoded),
                );
            }
            if let Ok(decoded) =
                <SequenceZeroBatches as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SequenceZeroBatches(decoded));
            }
            if let Ok(decoded) =
                <SequencedTimestampBelowForcedTimestamp as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SequencedTimestampBelowForcedTimestamp(decoded));
            }
            if let Ok(decoded) =
                <SequencedTimestampInvalid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SequencedTimestampInvalid(decoded));
            }
            if let Ok(decoded) =
                <StoredRootMustBeDifferentThanNewRoot as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::StoredRootMustBeDifferentThanNewRoot(decoded));
            }
            if let Ok(decoded) =
                <TransactionsLengthAboveMax as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransactionsLengthAboveMax(decoded));
            }
            if let Ok(decoded)
                = <TrustedAggregatorTimeoutExceedHaltAggregationTimeout as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::TrustedAggregatorTimeoutExceedHaltAggregationTimeout(decoded),
                );
            }
            if let Ok(decoded) =
                <TrustedAggregatorTimeoutNotExpired as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TrustedAggregatorTimeoutNotExpired(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPolygonZkEVMErrorsErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BatchAlreadyVerified(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BatchNotSequencedOrNotSequenceEnd(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExceedMaxVerifyBatches(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FinalNumBatchBelowLastVerifiedBatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FinalNumBatchDoesNotMatchPendingState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FinalPendingStateNumInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ForceBatchTimeoutNotExpired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ForceBatchesOverflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ForcedDataDoesNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GlobalExitRootNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HaltTimeoutNotExpired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitNumBatchAboveLastVerifiedBatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitNumBatchDoesNotMatchPendingState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidProof(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidRangeBatchTimeTarget(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidRangeMultiplierBatchFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NewAccInputHashDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NewPendingStateTimeoutMustBeLower(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NewTrustedAggregatorTimeoutMustBeLower(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughMaticAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OldAccInputHashDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OldStateRootDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyPendingAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyTrustedAggregator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyTrustedSequencer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingStateDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingStateInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingStateNotConsolidable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingStateTimeoutExceedHaltAggregationTimeout(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SequenceZeroBatches(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SequencedTimestampBelowForcedTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SequencedTimestampInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StoredRootMustBeDifferentThanNewRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransactionsLengthAboveMax(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TrustedAggregatorTimeoutExceedHaltAggregationTimeout(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TrustedAggregatorTimeoutNotExpired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IPolygonZkEVMErrorsErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BatchAlreadyVerified as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BatchNotSequencedOrNotSequenceEnd as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ExceedMaxVerifyBatches as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FinalNumBatchBelowLastVerifiedBatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FinalNumBatchDoesNotMatchPendingState as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FinalPendingStateNumInvalid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ForceBatchTimeoutNotExpired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ForceBatchesOverflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ForcedDataDoesNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <GlobalExitRootNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <HaltTimeoutNotExpired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InitNumBatchAboveLastVerifiedBatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InitNumBatchDoesNotMatchPendingState as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidProof as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidRangeBatchTimeTarget as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidRangeMultiplierBatchFee as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NewAccInputHashDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NewPendingStateTimeoutMustBeLower as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NewTrustedAggregatorTimeoutMustBeLower as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotEnoughMaticAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OldAccInputHashDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OldStateRootDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyAdmin as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <OnlyPendingAdmin as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyTrustedAggregator as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyTrustedSequencer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PendingStateDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PendingStateInvalid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PendingStateNotConsolidable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PendingStateTimeoutExceedHaltAggregationTimeout as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SequenceZeroBatches as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SequencedTimestampBelowForcedTimestamp as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SequencedTimestampInvalid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StoredRootMustBeDifferentThanNewRoot as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransactionsLengthAboveMax as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TrustedAggregatorTimeoutExceedHaltAggregationTimeout as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TrustedAggregatorTimeoutNotExpired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IPolygonZkEVMErrorsErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BatchAlreadyVerified(element) => ::core::fmt::Display::fmt(element, f),
                Self::BatchNotSequencedOrNotSequenceEnd(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExceedMaxVerifyBatches(element) => ::core::fmt::Display::fmt(element, f),
                Self::FinalNumBatchBelowLastVerifiedBatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FinalNumBatchDoesNotMatchPendingState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FinalPendingStateNumInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForceBatchTimeoutNotExpired(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForceBatchesOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForcedDataDoesNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::GlobalExitRootNotExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::HaltTimeoutNotExpired(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitNumBatchAboveLastVerifiedBatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitNumBatchDoesNotMatchPendingState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidRangeBatchTimeTarget(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidRangeMultiplierBatchFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewAccInputHashDoesNotExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewPendingStateTimeoutMustBeLower(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewTrustedAggregatorTimeoutMustBeLower(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotEnoughMaticAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::OldAccInputHashDoesNotExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::OldStateRootDoesNotExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyPendingAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyTrustedAggregator(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyTrustedSequencer(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingStateDoesNotExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingStateInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingStateNotConsolidable(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingStateTimeoutExceedHaltAggregationTimeout(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SequenceZeroBatches(element) => ::core::fmt::Display::fmt(element, f),
                Self::SequencedTimestampBelowForcedTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SequencedTimestampInvalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::StoredRootMustBeDifferentThanNewRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransactionsLengthAboveMax(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrustedAggregatorTimeoutExceedHaltAggregationTimeout(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TrustedAggregatorTimeoutNotExpired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IPolygonZkEVMErrorsErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BatchAlreadyVerified> for IPolygonZkEVMErrorsErrors {
        fn from(value: BatchAlreadyVerified) -> Self {
            Self::BatchAlreadyVerified(value)
        }
    }
    impl ::core::convert::From<BatchNotSequencedOrNotSequenceEnd> for IPolygonZkEVMErrorsErrors {
        fn from(value: BatchNotSequencedOrNotSequenceEnd) -> Self {
            Self::BatchNotSequencedOrNotSequenceEnd(value)
        }
    }
    impl ::core::convert::From<ExceedMaxVerifyBatches> for IPolygonZkEVMErrorsErrors {
        fn from(value: ExceedMaxVerifyBatches) -> Self {
            Self::ExceedMaxVerifyBatches(value)
        }
    }
    impl ::core::convert::From<FinalNumBatchBelowLastVerifiedBatch> for IPolygonZkEVMErrorsErrors {
        fn from(value: FinalNumBatchBelowLastVerifiedBatch) -> Self {
            Self::FinalNumBatchBelowLastVerifiedBatch(value)
        }
    }
    impl ::core::convert::From<FinalNumBatchDoesNotMatchPendingState> for IPolygonZkEVMErrorsErrors {
        fn from(value: FinalNumBatchDoesNotMatchPendingState) -> Self {
            Self::FinalNumBatchDoesNotMatchPendingState(value)
        }
    }
    impl ::core::convert::From<FinalPendingStateNumInvalid> for IPolygonZkEVMErrorsErrors {
        fn from(value: FinalPendingStateNumInvalid) -> Self {
            Self::FinalPendingStateNumInvalid(value)
        }
    }
    impl ::core::convert::From<ForceBatchTimeoutNotExpired> for IPolygonZkEVMErrorsErrors {
        fn from(value: ForceBatchTimeoutNotExpired) -> Self {
            Self::ForceBatchTimeoutNotExpired(value)
        }
    }
    impl ::core::convert::From<ForceBatchesOverflow> for IPolygonZkEVMErrorsErrors {
        fn from(value: ForceBatchesOverflow) -> Self {
            Self::ForceBatchesOverflow(value)
        }
    }
    impl ::core::convert::From<ForcedDataDoesNotMatch> for IPolygonZkEVMErrorsErrors {
        fn from(value: ForcedDataDoesNotMatch) -> Self {
            Self::ForcedDataDoesNotMatch(value)
        }
    }
    impl ::core::convert::From<GlobalExitRootNotExist> for IPolygonZkEVMErrorsErrors {
        fn from(value: GlobalExitRootNotExist) -> Self {
            Self::GlobalExitRootNotExist(value)
        }
    }
    impl ::core::convert::From<HaltTimeoutNotExpired> for IPolygonZkEVMErrorsErrors {
        fn from(value: HaltTimeoutNotExpired) -> Self {
            Self::HaltTimeoutNotExpired(value)
        }
    }
    impl ::core::convert::From<InitNumBatchAboveLastVerifiedBatch> for IPolygonZkEVMErrorsErrors {
        fn from(value: InitNumBatchAboveLastVerifiedBatch) -> Self {
            Self::InitNumBatchAboveLastVerifiedBatch(value)
        }
    }
    impl ::core::convert::From<InitNumBatchDoesNotMatchPendingState> for IPolygonZkEVMErrorsErrors {
        fn from(value: InitNumBatchDoesNotMatchPendingState) -> Self {
            Self::InitNumBatchDoesNotMatchPendingState(value)
        }
    }
    impl ::core::convert::From<InvalidProof> for IPolygonZkEVMErrorsErrors {
        fn from(value: InvalidProof) -> Self {
            Self::InvalidProof(value)
        }
    }
    impl ::core::convert::From<InvalidRangeBatchTimeTarget> for IPolygonZkEVMErrorsErrors {
        fn from(value: InvalidRangeBatchTimeTarget) -> Self {
            Self::InvalidRangeBatchTimeTarget(value)
        }
    }
    impl ::core::convert::From<InvalidRangeMultiplierBatchFee> for IPolygonZkEVMErrorsErrors {
        fn from(value: InvalidRangeMultiplierBatchFee) -> Self {
            Self::InvalidRangeMultiplierBatchFee(value)
        }
    }
    impl ::core::convert::From<NewAccInputHashDoesNotExist> for IPolygonZkEVMErrorsErrors {
        fn from(value: NewAccInputHashDoesNotExist) -> Self {
            Self::NewAccInputHashDoesNotExist(value)
        }
    }
    impl ::core::convert::From<NewPendingStateTimeoutMustBeLower> for IPolygonZkEVMErrorsErrors {
        fn from(value: NewPendingStateTimeoutMustBeLower) -> Self {
            Self::NewPendingStateTimeoutMustBeLower(value)
        }
    }
    impl ::core::convert::From<NewTrustedAggregatorTimeoutMustBeLower> for IPolygonZkEVMErrorsErrors {
        fn from(value: NewTrustedAggregatorTimeoutMustBeLower) -> Self {
            Self::NewTrustedAggregatorTimeoutMustBeLower(value)
        }
    }
    impl ::core::convert::From<NotEnoughMaticAmount> for IPolygonZkEVMErrorsErrors {
        fn from(value: NotEnoughMaticAmount) -> Self {
            Self::NotEnoughMaticAmount(value)
        }
    }
    impl ::core::convert::From<OldAccInputHashDoesNotExist> for IPolygonZkEVMErrorsErrors {
        fn from(value: OldAccInputHashDoesNotExist) -> Self {
            Self::OldAccInputHashDoesNotExist(value)
        }
    }
    impl ::core::convert::From<OldStateRootDoesNotExist> for IPolygonZkEVMErrorsErrors {
        fn from(value: OldStateRootDoesNotExist) -> Self {
            Self::OldStateRootDoesNotExist(value)
        }
    }
    impl ::core::convert::From<OnlyAdmin> for IPolygonZkEVMErrorsErrors {
        fn from(value: OnlyAdmin) -> Self {
            Self::OnlyAdmin(value)
        }
    }
    impl ::core::convert::From<OnlyPendingAdmin> for IPolygonZkEVMErrorsErrors {
        fn from(value: OnlyPendingAdmin) -> Self {
            Self::OnlyPendingAdmin(value)
        }
    }
    impl ::core::convert::From<OnlyTrustedAggregator> for IPolygonZkEVMErrorsErrors {
        fn from(value: OnlyTrustedAggregator) -> Self {
            Self::OnlyTrustedAggregator(value)
        }
    }
    impl ::core::convert::From<OnlyTrustedSequencer> for IPolygonZkEVMErrorsErrors {
        fn from(value: OnlyTrustedSequencer) -> Self {
            Self::OnlyTrustedSequencer(value)
        }
    }
    impl ::core::convert::From<PendingStateDoesNotExist> for IPolygonZkEVMErrorsErrors {
        fn from(value: PendingStateDoesNotExist) -> Self {
            Self::PendingStateDoesNotExist(value)
        }
    }
    impl ::core::convert::From<PendingStateInvalid> for IPolygonZkEVMErrorsErrors {
        fn from(value: PendingStateInvalid) -> Self {
            Self::PendingStateInvalid(value)
        }
    }
    impl ::core::convert::From<PendingStateNotConsolidable> for IPolygonZkEVMErrorsErrors {
        fn from(value: PendingStateNotConsolidable) -> Self {
            Self::PendingStateNotConsolidable(value)
        }
    }
    impl ::core::convert::From<PendingStateTimeoutExceedHaltAggregationTimeout>
        for IPolygonZkEVMErrorsErrors
    {
        fn from(value: PendingStateTimeoutExceedHaltAggregationTimeout) -> Self {
            Self::PendingStateTimeoutExceedHaltAggregationTimeout(value)
        }
    }
    impl ::core::convert::From<SequenceZeroBatches> for IPolygonZkEVMErrorsErrors {
        fn from(value: SequenceZeroBatches) -> Self {
            Self::SequenceZeroBatches(value)
        }
    }
    impl ::core::convert::From<SequencedTimestampBelowForcedTimestamp> for IPolygonZkEVMErrorsErrors {
        fn from(value: SequencedTimestampBelowForcedTimestamp) -> Self {
            Self::SequencedTimestampBelowForcedTimestamp(value)
        }
    }
    impl ::core::convert::From<SequencedTimestampInvalid> for IPolygonZkEVMErrorsErrors {
        fn from(value: SequencedTimestampInvalid) -> Self {
            Self::SequencedTimestampInvalid(value)
        }
    }
    impl ::core::convert::From<StoredRootMustBeDifferentThanNewRoot> for IPolygonZkEVMErrorsErrors {
        fn from(value: StoredRootMustBeDifferentThanNewRoot) -> Self {
            Self::StoredRootMustBeDifferentThanNewRoot(value)
        }
    }
    impl ::core::convert::From<TransactionsLengthAboveMax> for IPolygonZkEVMErrorsErrors {
        fn from(value: TransactionsLengthAboveMax) -> Self {
            Self::TransactionsLengthAboveMax(value)
        }
    }
    impl ::core::convert::From<TrustedAggregatorTimeoutExceedHaltAggregationTimeout>
        for IPolygonZkEVMErrorsErrors
    {
        fn from(value: TrustedAggregatorTimeoutExceedHaltAggregationTimeout) -> Self {
            Self::TrustedAggregatorTimeoutExceedHaltAggregationTimeout(value)
        }
    }
    impl ::core::convert::From<TrustedAggregatorTimeoutNotExpired> for IPolygonZkEVMErrorsErrors {
        fn from(value: TrustedAggregatorTimeoutNotExpired) -> Self {
            Self::TrustedAggregatorTimeoutNotExpired(value)
        }
    }
}
