pub use i_polygon_zk_evm_errors::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_polygon_zk_evm_errors {
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
    #[doc = "IPolygonZkEVMErrors was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"BatchAlreadyVerified\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BatchNotSequencedOrNotSequenceEnd\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ExceedMaxVerifyBatches\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FinalNumBatchBelowLastVerifiedBatch\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FinalNumBatchDoesNotMatchPendingState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FinalPendingStateNumInvalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ForceBatchTimeoutNotExpired\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ForceBatchesOverflow\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ForcedDataDoesNotMatch\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"GlobalExitRootNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"HaltTimeoutNotExpired\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InitNumBatchAboveLastVerifiedBatch\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InitNumBatchDoesNotMatchPendingState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidProof\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidRangeBatchTimeTarget\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidRangeMultiplierBatchFee\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NewAccInputHashDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NewPendingStateTimeoutMustBeLower\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NewTrustedAggregatorTimeoutMustBeLower\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotEnoughMaticAmount\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OldAccInputHashDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OldStateRootDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyAdmin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyPendingAdmin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyTrustedAggregator\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyTrustedSequencer\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PendingStateDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PendingStateInvalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PendingStateNotConsolidable\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PendingStateTimeoutExceedHaltAggregationTimeout\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SequenceZeroBatches\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SequencedTimestampBelowForcedTimestamp\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SequencedTimestampInvalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"StoredRootMustBeDifferentThanNewRoot\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransactionsLengthAboveMax\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TrustedAggregatorTimeoutExceedHaltAggregationTimeout\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TrustedAggregatorTimeoutNotExpired\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IPOLYGONZKEVMERRORS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IPolygonZkEVMErrors<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IPolygonZkEVMErrors<M> {
        fn clone(&self) -> Self {
            IPolygonZkEVMErrors(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IPolygonZkEVMErrors<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IPolygonZkEVMErrors<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPolygonZkEVMErrors))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IPolygonZkEVMErrors<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IPOLYGONZKEVMERRORS_ABI.clone(), client)
                .into()
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IPolygonZkEVMErrors<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `BatchAlreadyVerified` with signature `BatchAlreadyVerified()` and selector `[129, 42, 55, 45]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "BatchAlreadyVerified", abi = "BatchAlreadyVerified()")]
    pub struct BatchAlreadyVerified;
    #[doc = "Custom Error type `BatchNotSequencedOrNotSequenceEnd` with signature `BatchNotSequencedOrNotSequenceEnd()` and selector `[152, 197, 192, 20]`"]
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
        name = "BatchNotSequencedOrNotSequenceEnd",
        abi = "BatchNotSequencedOrNotSequenceEnd()"
    )]
    pub struct BatchNotSequencedOrNotSequenceEnd;
    #[doc = "Custom Error type `ExceedMaxVerifyBatches` with signature `ExceedMaxVerifyBatches()` and selector `[181, 159, 117, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "ExceedMaxVerifyBatches", abi = "ExceedMaxVerifyBatches()")]
    pub struct ExceedMaxVerifyBatches;
    #[doc = "Custom Error type `FinalNumBatchBelowLastVerifiedBatch` with signature `FinalNumBatchBelowLastVerifiedBatch()` and selector `[185, 177, 143, 87]`"]
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
        name = "FinalNumBatchBelowLastVerifiedBatch",
        abi = "FinalNumBatchBelowLastVerifiedBatch()"
    )]
    pub struct FinalNumBatchBelowLastVerifiedBatch;
    #[doc = "Custom Error type `FinalNumBatchDoesNotMatchPendingState` with signature `FinalNumBatchDoesNotMatchPendingState()` and selector `[50, 162, 167, 127]`"]
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
        name = "FinalNumBatchDoesNotMatchPendingState",
        abi = "FinalNumBatchDoesNotMatchPendingState()"
    )]
    pub struct FinalNumBatchDoesNotMatchPendingState;
    #[doc = "Custom Error type `FinalPendingStateNumInvalid` with signature `FinalPendingStateNumInvalid()` and selector `[191, 167, 7, 159]`"]
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
        name = "FinalPendingStateNumInvalid",
        abi = "FinalPendingStateNumInvalid()"
    )]
    pub struct FinalPendingStateNumInvalid;
    #[doc = "Custom Error type `ForceBatchTimeoutNotExpired` with signature `ForceBatchTimeoutNotExpired()` and selector `[196, 74, 8, 33]`"]
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
        name = "ForceBatchTimeoutNotExpired",
        abi = "ForceBatchTimeoutNotExpired()"
    )]
    pub struct ForceBatchTimeoutNotExpired;
    #[doc = "Custom Error type `ForceBatchesOverflow` with signature `ForceBatchesOverflow()` and selector `[198, 48, 160, 13]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "ForceBatchesOverflow", abi = "ForceBatchesOverflow()")]
    pub struct ForceBatchesOverflow;
    #[doc = "Custom Error type `ForcedDataDoesNotMatch` with signature `ForcedDataDoesNotMatch()` and selector `[206, 61, 117, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "ForcedDataDoesNotMatch", abi = "ForcedDataDoesNotMatch()")]
    pub struct ForcedDataDoesNotMatch;
    #[doc = "Custom Error type `GlobalExitRootNotExist` with signature `GlobalExitRootNotExist()` and selector `[115, 189, 102, 141]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "GlobalExitRootNotExist", abi = "GlobalExitRootNotExist()")]
    pub struct GlobalExitRootNotExist;
    #[doc = "Custom Error type `HaltTimeoutNotExpired` with signature `HaltTimeoutNotExpired()` and selector `[210, 87, 85, 90]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "HaltTimeoutNotExpired", abi = "HaltTimeoutNotExpired()")]
    pub struct HaltTimeoutNotExpired;
    #[doc = "Custom Error type `InitNumBatchAboveLastVerifiedBatch` with signature `InitNumBatchAboveLastVerifiedBatch()` and selector `[30, 86, 233, 226]`"]
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
        name = "InitNumBatchAboveLastVerifiedBatch",
        abi = "InitNumBatchAboveLastVerifiedBatch()"
    )]
    pub struct InitNumBatchAboveLastVerifiedBatch;
    #[doc = "Custom Error type `InitNumBatchDoesNotMatchPendingState` with signature `InitNumBatchDoesNotMatchPendingState()` and selector `[43, 210, 227, 231]`"]
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
        name = "InitNumBatchDoesNotMatchPendingState",
        abi = "InitNumBatchDoesNotMatchPendingState()"
    )]
    pub struct InitNumBatchDoesNotMatchPendingState;
    #[doc = "Custom Error type `InvalidProof` with signature `InvalidProof()` and selector `[9, 189, 227, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidProof", abi = "InvalidProof()")]
    pub struct InvalidProof;
    #[doc = "Custom Error type `InvalidRangeBatchTimeTarget` with signature `InvalidRangeBatchTimeTarget()` and selector `[224, 103, 223, 232]`"]
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
        name = "InvalidRangeBatchTimeTarget",
        abi = "InvalidRangeBatchTimeTarget()"
    )]
    pub struct InvalidRangeBatchTimeTarget;
    #[doc = "Custom Error type `InvalidRangeMultiplierBatchFee` with signature `InvalidRangeMultiplierBatchFee()` and selector `[76, 37, 51, 200]`"]
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
        name = "InvalidRangeMultiplierBatchFee",
        abi = "InvalidRangeMultiplierBatchFee()"
    )]
    pub struct InvalidRangeMultiplierBatchFee;
    #[doc = "Custom Error type `NewAccInputHashDoesNotExist` with signature `NewAccInputHashDoesNotExist()` and selector `[102, 56, 91, 81]`"]
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
        name = "NewAccInputHashDoesNotExist",
        abi = "NewAccInputHashDoesNotExist()"
    )]
    pub struct NewAccInputHashDoesNotExist;
    #[doc = "Custom Error type `NewPendingStateTimeoutMustBeLower` with signature `NewPendingStateTimeoutMustBeLower()` and selector `[72, 160, 90, 144]`"]
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
        name = "NewPendingStateTimeoutMustBeLower",
        abi = "NewPendingStateTimeoutMustBeLower()"
    )]
    pub struct NewPendingStateTimeoutMustBeLower;
    #[doc = "Custom Error type `NewTrustedAggregatorTimeoutMustBeLower` with signature `NewTrustedAggregatorTimeoutMustBeLower()` and selector `[64, 22, 54, 223]`"]
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
        name = "NewTrustedAggregatorTimeoutMustBeLower",
        abi = "NewTrustedAggregatorTimeoutMustBeLower()"
    )]
    pub struct NewTrustedAggregatorTimeoutMustBeLower;
    #[doc = "Custom Error type `NotEnoughMaticAmount` with signature `NotEnoughMaticAmount()` and selector `[71, 50, 253, 181]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NotEnoughMaticAmount", abi = "NotEnoughMaticAmount()")]
    pub struct NotEnoughMaticAmount;
    #[doc = "Custom Error type `OldAccInputHashDoesNotExist` with signature `OldAccInputHashDoesNotExist()` and selector `[104, 24, 194, 158]`"]
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
        name = "OldAccInputHashDoesNotExist",
        abi = "OldAccInputHashDoesNotExist()"
    )]
    pub struct OldAccInputHashDoesNotExist;
    #[doc = "Custom Error type `OldStateRootDoesNotExist` with signature `OldStateRootDoesNotExist()` and selector `[73, 151, 185, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OldStateRootDoesNotExist", abi = "OldStateRootDoesNotExist()")]
    pub struct OldStateRootDoesNotExist;
    #[doc = "Custom Error type `OnlyAdmin` with signature `OnlyAdmin()` and selector `[71, 85, 101, 121]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyAdmin", abi = "OnlyAdmin()")]
    pub struct OnlyAdmin;
    #[doc = "Custom Error type `OnlyPendingAdmin` with signature `OnlyPendingAdmin()` and selector `[209, 236, 75, 35]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyPendingAdmin", abi = "OnlyPendingAdmin()")]
    pub struct OnlyPendingAdmin;
    #[doc = "Custom Error type `OnlyTrustedAggregator` with signature `OnlyTrustedAggregator()` and selector `[187, 203, 188, 5]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyTrustedAggregator", abi = "OnlyTrustedAggregator()")]
    pub struct OnlyTrustedAggregator;
    #[doc = "Custom Error type `OnlyTrustedSequencer` with signature `OnlyTrustedSequencer()` and selector `[17, 231, 190, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OnlyTrustedSequencer", abi = "OnlyTrustedSequencer()")]
    pub struct OnlyTrustedSequencer;
    #[doc = "Custom Error type `PendingStateDoesNotExist` with signature `PendingStateDoesNotExist()` and selector `[187, 20, 194, 5]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "PendingStateDoesNotExist", abi = "PendingStateDoesNotExist()")]
    pub struct PendingStateDoesNotExist;
    #[doc = "Custom Error type `PendingStateInvalid` with signature `PendingStateInvalid()` and selector `[208, 134, 183, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "PendingStateInvalid", abi = "PendingStateInvalid()")]
    pub struct PendingStateInvalid;
    #[doc = "Custom Error type `PendingStateNotConsolidable` with signature `PendingStateNotConsolidable()` and selector `[12, 233, 228, 162]`"]
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
        name = "PendingStateNotConsolidable",
        abi = "PendingStateNotConsolidable()"
    )]
    pub struct PendingStateNotConsolidable;
    #[doc = "Custom Error type `PendingStateTimeoutExceedHaltAggregationTimeout` with signature `PendingStateTimeoutExceedHaltAggregationTimeout()` and selector `[204, 150, 80, 112]`"]
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
        name = "PendingStateTimeoutExceedHaltAggregationTimeout",
        abi = "PendingStateTimeoutExceedHaltAggregationTimeout()"
    )]
    pub struct PendingStateTimeoutExceedHaltAggregationTimeout;
    #[doc = "Custom Error type `SequenceZeroBatches` with signature `SequenceZeroBatches()` and selector `[203, 89, 26, 95]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "SequenceZeroBatches", abi = "SequenceZeroBatches()")]
    pub struct SequenceZeroBatches;
    #[doc = "Custom Error type `SequencedTimestampBelowForcedTimestamp` with signature `SequencedTimestampBelowForcedTimestamp()` and selector `[127, 122, 184, 114]`"]
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
        name = "SequencedTimestampBelowForcedTimestamp",
        abi = "SequencedTimestampBelowForcedTimestamp()"
    )]
    pub struct SequencedTimestampBelowForcedTimestamp;
    #[doc = "Custom Error type `SequencedTimestampInvalid` with signature `SequencedTimestampInvalid()` and selector `[234, 130, 121, 22]`"]
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
        name = "SequencedTimestampInvalid",
        abi = "SequencedTimestampInvalid()"
    )]
    pub struct SequencedTimestampInvalid;
    #[doc = "Custom Error type `StoredRootMustBeDifferentThanNewRoot` with signature `StoredRootMustBeDifferentThanNewRoot()` and selector `[164, 114, 118, 189]`"]
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
        name = "StoredRootMustBeDifferentThanNewRoot",
        abi = "StoredRootMustBeDifferentThanNewRoot()"
    )]
    pub struct StoredRootMustBeDifferentThanNewRoot;
    #[doc = "Custom Error type `TransactionsLengthAboveMax` with signature `TransactionsLengthAboveMax()` and selector `[162, 154, 108, 124]`"]
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
        name = "TransactionsLengthAboveMax",
        abi = "TransactionsLengthAboveMax()"
    )]
    pub struct TransactionsLengthAboveMax;
    #[doc = "Custom Error type `TrustedAggregatorTimeoutExceedHaltAggregationTimeout` with signature `TrustedAggregatorTimeoutExceedHaltAggregationTimeout()` and selector `[29, 6, 232, 121]`"]
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
        name = "TrustedAggregatorTimeoutExceedHaltAggregationTimeout",
        abi = "TrustedAggregatorTimeoutExceedHaltAggregationTimeout()"
    )]
    pub struct TrustedAggregatorTimeoutExceedHaltAggregationTimeout;
    #[doc = "Custom Error type `TrustedAggregatorTimeoutNotExpired` with signature `TrustedAggregatorTimeoutNotExpired()` and selector `[138, 7, 4, 211]`"]
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
        name = "TrustedAggregatorTimeoutNotExpired",
        abi = "TrustedAggregatorTimeoutNotExpired()"
    )]
    pub struct TrustedAggregatorTimeoutNotExpired;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
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
    }
    impl ethers::core::abi::AbiDecode for IPolygonZkEVMErrorsErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BatchAlreadyVerified as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::BatchAlreadyVerified(decoded));
            }
            if let Ok(decoded) =
                <BatchNotSequencedOrNotSequenceEnd as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPolygonZkEVMErrorsErrors::BatchNotSequencedOrNotSequenceEnd(decoded));
            }
            if let Ok(decoded) =
                <ExceedMaxVerifyBatches as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::ExceedMaxVerifyBatches(decoded));
            }
            if let Ok(decoded) =
                <FinalNumBatchBelowLastVerifiedBatch as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPolygonZkEVMErrorsErrors::FinalNumBatchBelowLastVerifiedBatch(decoded));
            }
            if let Ok(decoded) =
                <FinalNumBatchDoesNotMatchPendingState as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    IPolygonZkEVMErrorsErrors::FinalNumBatchDoesNotMatchPendingState(decoded),
                );
            }
            if let Ok(decoded) =
                <FinalPendingStateNumInvalid as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::FinalPendingStateNumInvalid(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ForceBatchTimeoutNotExpired as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::ForceBatchTimeoutNotExpired(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ForceBatchesOverflow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::ForceBatchesOverflow(decoded));
            }
            if let Ok(decoded) =
                <ForcedDataDoesNotMatch as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::ForcedDataDoesNotMatch(decoded));
            }
            if let Ok(decoded) =
                <GlobalExitRootNotExist as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::GlobalExitRootNotExist(decoded));
            }
            if let Ok(decoded) =
                <HaltTimeoutNotExpired as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::HaltTimeoutNotExpired(decoded));
            }
            if let Ok(decoded) =
                <InitNumBatchAboveLastVerifiedBatch as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPolygonZkEVMErrorsErrors::InitNumBatchAboveLastVerifiedBatch(decoded));
            }
            if let Ok(decoded) =
                <InitNumBatchDoesNotMatchPendingState as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    IPolygonZkEVMErrorsErrors::InitNumBatchDoesNotMatchPendingState(decoded),
                );
            }
            if let Ok(decoded) =
                <InvalidProof as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::InvalidProof(decoded));
            }
            if let Ok(decoded) =
                <InvalidRangeBatchTimeTarget as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::InvalidRangeBatchTimeTarget(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InvalidRangeMultiplierBatchFee as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPolygonZkEVMErrorsErrors::InvalidRangeMultiplierBatchFee(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <NewAccInputHashDoesNotExist as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::NewAccInputHashDoesNotExist(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <NewPendingStateTimeoutMustBeLower as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPolygonZkEVMErrorsErrors::NewPendingStateTimeoutMustBeLower(decoded));
            }
            if let Ok(decoded) =
                <NewTrustedAggregatorTimeoutMustBeLower as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    IPolygonZkEVMErrorsErrors::NewTrustedAggregatorTimeoutMustBeLower(decoded),
                );
            }
            if let Ok(decoded) =
                <NotEnoughMaticAmount as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::NotEnoughMaticAmount(decoded));
            }
            if let Ok(decoded) =
                <OldAccInputHashDoesNotExist as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::OldAccInputHashDoesNotExist(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <OldStateRootDoesNotExist as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::OldStateRootDoesNotExist(decoded));
            }
            if let Ok(decoded) = <OnlyAdmin as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::OnlyAdmin(decoded));
            }
            if let Ok(decoded) =
                <OnlyPendingAdmin as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::OnlyPendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <OnlyTrustedAggregator as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::OnlyTrustedAggregator(decoded));
            }
            if let Ok(decoded) =
                <OnlyTrustedSequencer as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::OnlyTrustedSequencer(decoded));
            }
            if let Ok(decoded) =
                <PendingStateDoesNotExist as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::PendingStateDoesNotExist(decoded));
            }
            if let Ok(decoded) =
                <PendingStateInvalid as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::PendingStateInvalid(decoded));
            }
            if let Ok(decoded) =
                <PendingStateNotConsolidable as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::PendingStateNotConsolidable(
                    decoded,
                ));
            }
            if let Ok (decoded) = < PendingStateTimeoutExceedHaltAggregationTimeout as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IPolygonZkEVMErrorsErrors :: PendingStateTimeoutExceedHaltAggregationTimeout (decoded)) }
            if let Ok(decoded) =
                <SequenceZeroBatches as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::SequenceZeroBatches(decoded));
            }
            if let Ok(decoded) =
                <SequencedTimestampBelowForcedTimestamp as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    IPolygonZkEVMErrorsErrors::SequencedTimestampBelowForcedTimestamp(decoded),
                );
            }
            if let Ok(decoded) =
                <SequencedTimestampInvalid as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::SequencedTimestampInvalid(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <StoredRootMustBeDifferentThanNewRoot as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    IPolygonZkEVMErrorsErrors::StoredRootMustBeDifferentThanNewRoot(decoded),
                );
            }
            if let Ok(decoded) =
                <TransactionsLengthAboveMax as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPolygonZkEVMErrorsErrors::TransactionsLengthAboveMax(
                    decoded,
                ));
            }
            if let Ok (decoded) = < TrustedAggregatorTimeoutExceedHaltAggregationTimeout as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IPolygonZkEVMErrorsErrors :: TrustedAggregatorTimeoutExceedHaltAggregationTimeout (decoded)) }
            if let Ok(decoded) =
                <TrustedAggregatorTimeoutNotExpired as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPolygonZkEVMErrorsErrors::TrustedAggregatorTimeoutNotExpired(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPolygonZkEVMErrorsErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                IPolygonZkEVMErrorsErrors::BatchAlreadyVerified(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::BatchNotSequencedOrNotSequenceEnd(element) => {
                    element.encode()
                }
                IPolygonZkEVMErrorsErrors::ExceedMaxVerifyBatches(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::FinalNumBatchBelowLastVerifiedBatch(element) => {
                    element.encode()
                }
                IPolygonZkEVMErrorsErrors::FinalNumBatchDoesNotMatchPendingState(element) => {
                    element.encode()
                }
                IPolygonZkEVMErrorsErrors::FinalPendingStateNumInvalid(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::ForceBatchTimeoutNotExpired(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::ForceBatchesOverflow(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::ForcedDataDoesNotMatch(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::GlobalExitRootNotExist(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::HaltTimeoutNotExpired(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::InitNumBatchAboveLastVerifiedBatch(element) => {
                    element.encode()
                }
                IPolygonZkEVMErrorsErrors::InitNumBatchDoesNotMatchPendingState(element) => {
                    element.encode()
                }
                IPolygonZkEVMErrorsErrors::InvalidProof(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::InvalidRangeBatchTimeTarget(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::InvalidRangeMultiplierBatchFee(element) => {
                    element.encode()
                }
                IPolygonZkEVMErrorsErrors::NewAccInputHashDoesNotExist(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::NewPendingStateTimeoutMustBeLower(element) => {
                    element.encode()
                }
                IPolygonZkEVMErrorsErrors::NewTrustedAggregatorTimeoutMustBeLower(element) => {
                    element.encode()
                }
                IPolygonZkEVMErrorsErrors::NotEnoughMaticAmount(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::OldAccInputHashDoesNotExist(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::OldStateRootDoesNotExist(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::OnlyAdmin(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::OnlyPendingAdmin(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::OnlyTrustedAggregator(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::OnlyTrustedSequencer(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::PendingStateDoesNotExist(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::PendingStateInvalid(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::PendingStateNotConsolidable(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::PendingStateTimeoutExceedHaltAggregationTimeout(
                    element,
                ) => element.encode(),
                IPolygonZkEVMErrorsErrors::SequenceZeroBatches(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::SequencedTimestampBelowForcedTimestamp(element) => {
                    element.encode()
                }
                IPolygonZkEVMErrorsErrors::SequencedTimestampInvalid(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::StoredRootMustBeDifferentThanNewRoot(element) => {
                    element.encode()
                }
                IPolygonZkEVMErrorsErrors::TransactionsLengthAboveMax(element) => element.encode(),
                IPolygonZkEVMErrorsErrors::TrustedAggregatorTimeoutExceedHaltAggregationTimeout(
                    element,
                ) => element.encode(),
                IPolygonZkEVMErrorsErrors::TrustedAggregatorTimeoutNotExpired(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for IPolygonZkEVMErrorsErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPolygonZkEVMErrorsErrors::BatchAlreadyVerified(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::BatchNotSequencedOrNotSequenceEnd(element) => {
                    element.fmt(f)
                }
                IPolygonZkEVMErrorsErrors::ExceedMaxVerifyBatches(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::FinalNumBatchBelowLastVerifiedBatch(element) => {
                    element.fmt(f)
                }
                IPolygonZkEVMErrorsErrors::FinalNumBatchDoesNotMatchPendingState(element) => {
                    element.fmt(f)
                }
                IPolygonZkEVMErrorsErrors::FinalPendingStateNumInvalid(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::ForceBatchTimeoutNotExpired(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::ForceBatchesOverflow(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::ForcedDataDoesNotMatch(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::GlobalExitRootNotExist(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::HaltTimeoutNotExpired(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::InitNumBatchAboveLastVerifiedBatch(element) => {
                    element.fmt(f)
                }
                IPolygonZkEVMErrorsErrors::InitNumBatchDoesNotMatchPendingState(element) => {
                    element.fmt(f)
                }
                IPolygonZkEVMErrorsErrors::InvalidProof(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::InvalidRangeBatchTimeTarget(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::InvalidRangeMultiplierBatchFee(element) => {
                    element.fmt(f)
                }
                IPolygonZkEVMErrorsErrors::NewAccInputHashDoesNotExist(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::NewPendingStateTimeoutMustBeLower(element) => {
                    element.fmt(f)
                }
                IPolygonZkEVMErrorsErrors::NewTrustedAggregatorTimeoutMustBeLower(element) => {
                    element.fmt(f)
                }
                IPolygonZkEVMErrorsErrors::NotEnoughMaticAmount(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::OldAccInputHashDoesNotExist(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::OldStateRootDoesNotExist(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::OnlyAdmin(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::OnlyPendingAdmin(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::OnlyTrustedAggregator(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::OnlyTrustedSequencer(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::PendingStateDoesNotExist(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::PendingStateInvalid(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::PendingStateNotConsolidable(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::PendingStateTimeoutExceedHaltAggregationTimeout(
                    element,
                ) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::SequenceZeroBatches(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::SequencedTimestampBelowForcedTimestamp(element) => {
                    element.fmt(f)
                }
                IPolygonZkEVMErrorsErrors::SequencedTimestampInvalid(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::StoredRootMustBeDifferentThanNewRoot(element) => {
                    element.fmt(f)
                }
                IPolygonZkEVMErrorsErrors::TransactionsLengthAboveMax(element) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::TrustedAggregatorTimeoutExceedHaltAggregationTimeout(
                    element,
                ) => element.fmt(f),
                IPolygonZkEVMErrorsErrors::TrustedAggregatorTimeoutNotExpired(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<BatchAlreadyVerified> for IPolygonZkEVMErrorsErrors {
        fn from(var: BatchAlreadyVerified) -> Self {
            IPolygonZkEVMErrorsErrors::BatchAlreadyVerified(var)
        }
    }
    impl ::std::convert::From<BatchNotSequencedOrNotSequenceEnd> for IPolygonZkEVMErrorsErrors {
        fn from(var: BatchNotSequencedOrNotSequenceEnd) -> Self {
            IPolygonZkEVMErrorsErrors::BatchNotSequencedOrNotSequenceEnd(var)
        }
    }
    impl ::std::convert::From<ExceedMaxVerifyBatches> for IPolygonZkEVMErrorsErrors {
        fn from(var: ExceedMaxVerifyBatches) -> Self {
            IPolygonZkEVMErrorsErrors::ExceedMaxVerifyBatches(var)
        }
    }
    impl ::std::convert::From<FinalNumBatchBelowLastVerifiedBatch> for IPolygonZkEVMErrorsErrors {
        fn from(var: FinalNumBatchBelowLastVerifiedBatch) -> Self {
            IPolygonZkEVMErrorsErrors::FinalNumBatchBelowLastVerifiedBatch(var)
        }
    }
    impl ::std::convert::From<FinalNumBatchDoesNotMatchPendingState> for IPolygonZkEVMErrorsErrors {
        fn from(var: FinalNumBatchDoesNotMatchPendingState) -> Self {
            IPolygonZkEVMErrorsErrors::FinalNumBatchDoesNotMatchPendingState(var)
        }
    }
    impl ::std::convert::From<FinalPendingStateNumInvalid> for IPolygonZkEVMErrorsErrors {
        fn from(var: FinalPendingStateNumInvalid) -> Self {
            IPolygonZkEVMErrorsErrors::FinalPendingStateNumInvalid(var)
        }
    }
    impl ::std::convert::From<ForceBatchTimeoutNotExpired> for IPolygonZkEVMErrorsErrors {
        fn from(var: ForceBatchTimeoutNotExpired) -> Self {
            IPolygonZkEVMErrorsErrors::ForceBatchTimeoutNotExpired(var)
        }
    }
    impl ::std::convert::From<ForceBatchesOverflow> for IPolygonZkEVMErrorsErrors {
        fn from(var: ForceBatchesOverflow) -> Self {
            IPolygonZkEVMErrorsErrors::ForceBatchesOverflow(var)
        }
    }
    impl ::std::convert::From<ForcedDataDoesNotMatch> for IPolygonZkEVMErrorsErrors {
        fn from(var: ForcedDataDoesNotMatch) -> Self {
            IPolygonZkEVMErrorsErrors::ForcedDataDoesNotMatch(var)
        }
    }
    impl ::std::convert::From<GlobalExitRootNotExist> for IPolygonZkEVMErrorsErrors {
        fn from(var: GlobalExitRootNotExist) -> Self {
            IPolygonZkEVMErrorsErrors::GlobalExitRootNotExist(var)
        }
    }
    impl ::std::convert::From<HaltTimeoutNotExpired> for IPolygonZkEVMErrorsErrors {
        fn from(var: HaltTimeoutNotExpired) -> Self {
            IPolygonZkEVMErrorsErrors::HaltTimeoutNotExpired(var)
        }
    }
    impl ::std::convert::From<InitNumBatchAboveLastVerifiedBatch> for IPolygonZkEVMErrorsErrors {
        fn from(var: InitNumBatchAboveLastVerifiedBatch) -> Self {
            IPolygonZkEVMErrorsErrors::InitNumBatchAboveLastVerifiedBatch(var)
        }
    }
    impl ::std::convert::From<InitNumBatchDoesNotMatchPendingState> for IPolygonZkEVMErrorsErrors {
        fn from(var: InitNumBatchDoesNotMatchPendingState) -> Self {
            IPolygonZkEVMErrorsErrors::InitNumBatchDoesNotMatchPendingState(var)
        }
    }
    impl ::std::convert::From<InvalidProof> for IPolygonZkEVMErrorsErrors {
        fn from(var: InvalidProof) -> Self {
            IPolygonZkEVMErrorsErrors::InvalidProof(var)
        }
    }
    impl ::std::convert::From<InvalidRangeBatchTimeTarget> for IPolygonZkEVMErrorsErrors {
        fn from(var: InvalidRangeBatchTimeTarget) -> Self {
            IPolygonZkEVMErrorsErrors::InvalidRangeBatchTimeTarget(var)
        }
    }
    impl ::std::convert::From<InvalidRangeMultiplierBatchFee> for IPolygonZkEVMErrorsErrors {
        fn from(var: InvalidRangeMultiplierBatchFee) -> Self {
            IPolygonZkEVMErrorsErrors::InvalidRangeMultiplierBatchFee(var)
        }
    }
    impl ::std::convert::From<NewAccInputHashDoesNotExist> for IPolygonZkEVMErrorsErrors {
        fn from(var: NewAccInputHashDoesNotExist) -> Self {
            IPolygonZkEVMErrorsErrors::NewAccInputHashDoesNotExist(var)
        }
    }
    impl ::std::convert::From<NewPendingStateTimeoutMustBeLower> for IPolygonZkEVMErrorsErrors {
        fn from(var: NewPendingStateTimeoutMustBeLower) -> Self {
            IPolygonZkEVMErrorsErrors::NewPendingStateTimeoutMustBeLower(var)
        }
    }
    impl ::std::convert::From<NewTrustedAggregatorTimeoutMustBeLower> for IPolygonZkEVMErrorsErrors {
        fn from(var: NewTrustedAggregatorTimeoutMustBeLower) -> Self {
            IPolygonZkEVMErrorsErrors::NewTrustedAggregatorTimeoutMustBeLower(var)
        }
    }
    impl ::std::convert::From<NotEnoughMaticAmount> for IPolygonZkEVMErrorsErrors {
        fn from(var: NotEnoughMaticAmount) -> Self {
            IPolygonZkEVMErrorsErrors::NotEnoughMaticAmount(var)
        }
    }
    impl ::std::convert::From<OldAccInputHashDoesNotExist> for IPolygonZkEVMErrorsErrors {
        fn from(var: OldAccInputHashDoesNotExist) -> Self {
            IPolygonZkEVMErrorsErrors::OldAccInputHashDoesNotExist(var)
        }
    }
    impl ::std::convert::From<OldStateRootDoesNotExist> for IPolygonZkEVMErrorsErrors {
        fn from(var: OldStateRootDoesNotExist) -> Self {
            IPolygonZkEVMErrorsErrors::OldStateRootDoesNotExist(var)
        }
    }
    impl ::std::convert::From<OnlyAdmin> for IPolygonZkEVMErrorsErrors {
        fn from(var: OnlyAdmin) -> Self {
            IPolygonZkEVMErrorsErrors::OnlyAdmin(var)
        }
    }
    impl ::std::convert::From<OnlyPendingAdmin> for IPolygonZkEVMErrorsErrors {
        fn from(var: OnlyPendingAdmin) -> Self {
            IPolygonZkEVMErrorsErrors::OnlyPendingAdmin(var)
        }
    }
    impl ::std::convert::From<OnlyTrustedAggregator> for IPolygonZkEVMErrorsErrors {
        fn from(var: OnlyTrustedAggregator) -> Self {
            IPolygonZkEVMErrorsErrors::OnlyTrustedAggregator(var)
        }
    }
    impl ::std::convert::From<OnlyTrustedSequencer> for IPolygonZkEVMErrorsErrors {
        fn from(var: OnlyTrustedSequencer) -> Self {
            IPolygonZkEVMErrorsErrors::OnlyTrustedSequencer(var)
        }
    }
    impl ::std::convert::From<PendingStateDoesNotExist> for IPolygonZkEVMErrorsErrors {
        fn from(var: PendingStateDoesNotExist) -> Self {
            IPolygonZkEVMErrorsErrors::PendingStateDoesNotExist(var)
        }
    }
    impl ::std::convert::From<PendingStateInvalid> for IPolygonZkEVMErrorsErrors {
        fn from(var: PendingStateInvalid) -> Self {
            IPolygonZkEVMErrorsErrors::PendingStateInvalid(var)
        }
    }
    impl ::std::convert::From<PendingStateNotConsolidable> for IPolygonZkEVMErrorsErrors {
        fn from(var: PendingStateNotConsolidable) -> Self {
            IPolygonZkEVMErrorsErrors::PendingStateNotConsolidable(var)
        }
    }
    impl ::std::convert::From<PendingStateTimeoutExceedHaltAggregationTimeout>
        for IPolygonZkEVMErrorsErrors
    {
        fn from(var: PendingStateTimeoutExceedHaltAggregationTimeout) -> Self {
            IPolygonZkEVMErrorsErrors::PendingStateTimeoutExceedHaltAggregationTimeout(var)
        }
    }
    impl ::std::convert::From<SequenceZeroBatches> for IPolygonZkEVMErrorsErrors {
        fn from(var: SequenceZeroBatches) -> Self {
            IPolygonZkEVMErrorsErrors::SequenceZeroBatches(var)
        }
    }
    impl ::std::convert::From<SequencedTimestampBelowForcedTimestamp> for IPolygonZkEVMErrorsErrors {
        fn from(var: SequencedTimestampBelowForcedTimestamp) -> Self {
            IPolygonZkEVMErrorsErrors::SequencedTimestampBelowForcedTimestamp(var)
        }
    }
    impl ::std::convert::From<SequencedTimestampInvalid> for IPolygonZkEVMErrorsErrors {
        fn from(var: SequencedTimestampInvalid) -> Self {
            IPolygonZkEVMErrorsErrors::SequencedTimestampInvalid(var)
        }
    }
    impl ::std::convert::From<StoredRootMustBeDifferentThanNewRoot> for IPolygonZkEVMErrorsErrors {
        fn from(var: StoredRootMustBeDifferentThanNewRoot) -> Self {
            IPolygonZkEVMErrorsErrors::StoredRootMustBeDifferentThanNewRoot(var)
        }
    }
    impl ::std::convert::From<TransactionsLengthAboveMax> for IPolygonZkEVMErrorsErrors {
        fn from(var: TransactionsLengthAboveMax) -> Self {
            IPolygonZkEVMErrorsErrors::TransactionsLengthAboveMax(var)
        }
    }
    impl ::std::convert::From<TrustedAggregatorTimeoutExceedHaltAggregationTimeout>
        for IPolygonZkEVMErrorsErrors
    {
        fn from(var: TrustedAggregatorTimeoutExceedHaltAggregationTimeout) -> Self {
            IPolygonZkEVMErrorsErrors::TrustedAggregatorTimeoutExceedHaltAggregationTimeout(var)
        }
    }
    impl ::std::convert::From<TrustedAggregatorTimeoutNotExpired> for IPolygonZkEVMErrorsErrors {
        fn from(var: TrustedAggregatorTimeoutNotExpired) -> Self {
            IPolygonZkEVMErrorsErrors::TrustedAggregatorTimeoutNotExpired(var)
        }
    }
}
