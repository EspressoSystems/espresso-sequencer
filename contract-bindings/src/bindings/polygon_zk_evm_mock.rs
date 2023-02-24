pub use polygon_zk_evm_mock::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod polygon_zk_evm_mock {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    pub use super::super::shared_types::*;
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "PolygonZkEVMMock was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"contract IPolygonZkEVMGlobalExitRoot\",\"name\":\"_globalExitRootManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20Upgradeable\",\"name\":\"_matic\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IVerifierRollup\",\"name\":\"_rollupVerifier\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IPolygonZkEVMBridge\",\"name\":\"_bridgeAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_chainID\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_forkID\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BatchAlreadyVerified\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BatchNotSequencedOrNotSequenceEnd\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ExceedMaxVerifyBatches\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FinalNumBatchBelowLastVerifiedBatch\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FinalNumBatchDoesNotMatchPendingState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FinalPendingStateNumInvalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ForceBatchTimeoutNotExpired\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ForceBatchesOverflow\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ForcedDataDoesNotMatch\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"GlobalExitRootNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"HaltTimeoutNotExpired\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InitNumBatchAboveLastVerifiedBatch\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InitNumBatchDoesNotMatchPendingState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidProof\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidRangeBatchTimeTarget\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidRangeMultiplierBatchFee\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NewAccInputHashDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NewPendingStateTimeoutMustBeLower\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NewTrustedAggregatorTimeoutMustBeLower\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotEnoughMaticAmount\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OldAccInputHashDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OldStateRootDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyAdmin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyEmergencyState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyNotEmergencyState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyPendingAdmin\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyTrustedAggregator\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OnlyTrustedSequencer\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PendingStateDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PendingStateInvalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PendingStateNotConsolidable\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PendingStateTimeoutExceedHaltAggregationTimeout\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SequenceZeroBatches\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SequencedTimestampBelowForcedTimestamp\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SequencedTimestampInvalid\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"StoredRootMustBeDifferentThanNewRoot\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransactionsLengthAboveMax\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TrustedAggregatorTimeoutExceedHaltAggregationTimeout\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TrustedAggregatorTimeoutNotExpired\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AcceptAdminRole\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"numBatch\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"stateRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint64\",\"name\":\"pendingStateNum\",\"type\":\"uint64\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ConsolidatePendingState\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"EmergencyStateActivated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"EmergencyStateDeactivated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"forceBatchNum\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"lastGlobalExitRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"sequencer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"transactions\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ForceBatch\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"numBatch\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"stateRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"aggregator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OverridePendingState\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"storedStateRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"provedStateRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ProveNonDeterministicPendingState\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"numBatch\",\"type\":\"uint64\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"SequenceBatches\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"numBatch\",\"type\":\"uint64\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"SequenceForceBatches\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"newMultiplierBatchFee\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetMultiplierBatchFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"newPendingStateTimeout\",\"type\":\"uint64\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetPendingStateTimeout\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newTrustedAggregator\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetTrustedAggregator\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"newTrustedAggregatorTimeout\",\"type\":\"uint64\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetTrustedAggregatorTimeout\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newTrustedSequencer\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetTrustedSequencer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"newTrustedSequencerURL\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetTrustedSequencerURL\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"newVerifyBatchTimeTarget\",\"type\":\"uint64\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetVerifyBatchTimeTarget\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferAdminRole\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"numBatch\",\"type\":\"uint64\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint64\",\"name\":\"forkID\",\"type\":\"uint64\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"version\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UpdateZkEVMVersion\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"numBatch\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"stateRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"aggregator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"VerifyBatches\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"numBatch\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"stateRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"aggregator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"VerifyBatchesTrustedAggregator\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptAdminRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"sequencedBatchNum\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"activateEmergencyState\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"batchFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"batchNumToStateRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bridgeAddress\",\"outputs\":[{\"internalType\":\"contract IPolygonZkEVMBridge\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"currentAccInputHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"transactions\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"globalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"timestamp\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"sequencerAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"calculateAccInputHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calculateRewardPerBatch\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"chainID\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"pendingStateNum\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"consolidatePendingState\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deactivateEmergencyState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"transactions\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maticAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"forceBatch\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"forcedBatches\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"forkID\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentBatchFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"initNumBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalNewBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newLocalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"oldStateRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newStateRoot\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getInputSnarkBytes\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastVerifiedBatch\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"pendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"initNumBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalNewBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newLocalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newStateRoot\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNextSnarkInput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"globalExitRootManager\",\"outputs\":[{\"internalType\":\"contract IPolygonZkEVMGlobalExitRoot\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct PolygonZkEVM.InitializePackedParameters\",\"name\":\"initializePackedParameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trustedSequencer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"pendingStateTimeout\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trustedAggregator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"trustedAggregatorTimeout\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"genesisRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_trustedSequencerURL\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_networkName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_version\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isEmergencyState\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"pendingStateNum\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPendingStateConsolidable\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastBatchSequenced\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastForceBatch\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastForceBatchSequenced\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastPendingState\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastPendingStateConsolidated\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastTimestamp\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastVerifiedBatch\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"matic\",\"outputs\":[{\"internalType\":\"contract IERC20Upgradeable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"multiplierBatchFee\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"networkName\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"initPendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalPendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"initNumBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalNewBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newLocalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newStateRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofA\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2][2]\",\"name\":\"proofB\",\"type\":\"uint256[2][2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofC\",\"type\":\"uint256[2]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"overridePendingState\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingStateTimeout\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingStateTransitions\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"timestamp\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"lastVerifiedBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"exitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"stateRoot\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"initPendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalPendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"initNumBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalNewBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newLocalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newStateRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofA\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2][2]\",\"name\":\"proofB\",\"type\":\"uint256[2][2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofC\",\"type\":\"uint256[2]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proveNonDeterministicPendingState\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rollupVerifier\",\"outputs\":[{\"internalType\":\"contract IVerifierRollup\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct PolygonZkEVM.BatchData[]\",\"name\":\"batches\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"transactions\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"globalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"timestamp\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"minForcedTimestamp\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"address\",\"name\":\"feeRecipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sequenceBatches\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct PolygonZkEVM.ForcedBatchData[]\",\"name\":\"batches\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"transactions\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"globalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"minForcedTimestamp\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sequenceForceBatches\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sequencedBatches\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"accInputHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"sequencedTimestamp\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"previousLastBatchSequenced\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"newMultiplierBatchFee\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMultiplierBatchFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_networkName\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setNetworkName\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"newPendingStateTimeout\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPendingStateTimeout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"_numBatch\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSequencedBatch\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"batchNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"accInputData\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"timestamp\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"lastPendingStateConsolidated\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSequencedBatches\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"newStateRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"batchNum\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setStateRoot\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newTrustedAggregator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTrustedAggregator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"newTrustedAggregatorTimeout\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTrustedAggregatorTimeout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newTrustedSequencer\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTrustedSequencer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"newTrustedSequencerURL\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTrustedSequencerURL\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"_numBatch\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setVerifiedBatch\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"newVerifyBatchTimeTarget\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setVerifyBatchTimeTarget\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferAdminRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"trustedAggregator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"trustedAggregatorTimeout\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"trustedSequencer\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"trustedSequencerURL\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"pendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"initNumBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalNewBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newLocalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newStateRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofA\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2][2]\",\"name\":\"proofB\",\"type\":\"uint256[2][2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofC\",\"type\":\"uint256[2]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"trustedVerifyBatchesMock\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"newLastVerifiedBatch\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateBatchFee\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verifyBatchTimeTarget\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"pendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"initNumBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalNewBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newLocalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newStateRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofA\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2][2]\",\"name\":\"proofB\",\"type\":\"uint256[2][2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofC\",\"type\":\"uint256[2]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"verifyBatches\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"pendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"initNumBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalNewBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newLocalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newStateRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofA\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2][2]\",\"name\":\"proofB\",\"type\":\"uint256[2][2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofC\",\"type\":\"uint256[2]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"verifyBatchesTrustedAggregator\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static POLYGONZKEVMMOCK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct PolygonZkEVMMock<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PolygonZkEVMMock<M> {
        fn clone(&self) -> Self {
            PolygonZkEVMMock(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PolygonZkEVMMock<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PolygonZkEVMMock<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PolygonZkEVMMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PolygonZkEVMMock<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), POLYGONZKEVMMOCK_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `acceptAdminRole` (0x8c3d7301) function"]
        pub fn accept_admin_role(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 61, 115, 1], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `activateEmergencyState` (0x7215541a) function"]
        pub fn activate_emergency_state(
            &self,
            sequenced_batch_num: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 21, 84, 26], sequenced_batch_num)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `batchFee` (0xf8b823e4) function"]
        pub fn batch_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([248, 184, 35, 228], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `batchNumToStateRoot` (0x5392c5e0) function"]
        pub fn batch_num_to_state_root(
            &self,
            p0: u64,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([83, 146, 197, 224], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bridgeAddress` (0xa3c573eb) function"]
        pub fn bridge_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([163, 197, 115, 235], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calculateAccInputHash` (0x574f649e) function"]
        pub fn calculate_acc_input_hash(
            &self,
            current_acc_input_hash: [u8; 32],
            transactions: ethers::core::types::Bytes,
            global_exit_root: [u8; 32],
            timestamp: u64,
            sequencer_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [87, 79, 100, 158],
                    (
                        current_acc_input_hash,
                        transactions,
                        global_exit_root,
                        timestamp,
                        sequencer_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calculateRewardPerBatch` (0x99f5634e) function"]
        pub fn calculate_reward_per_batch(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([153, 245, 99, 78], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `chainID` (0xadc879e9) function"]
        pub fn chain_id(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([173, 200, 121, 233], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `consolidatePendingState` (0x4a910e6a) function"]
        pub fn consolidate_pending_state(
            &self,
            pending_state_num: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 145, 14, 106], pending_state_num)
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
        #[doc = "Calls the contract's `forceBatch` (0xeaeb077b) function"]
        pub fn force_batch(
            &self,
            transactions: ethers::core::types::Bytes,
            matic_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 235, 7, 123], (transactions, matic_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forcedBatches` (0x6b8616ce) function"]
        pub fn forced_batches(
            &self,
            p0: u64,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([107, 134, 22, 206], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forkID` (0x831c7ead) function"]
        pub fn fork_id(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([131, 28, 126, 173], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBatchFee` (0x9f0d039d) function"]
        pub fn get_current_batch_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([159, 13, 3, 157], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getInputSnarkBytes` (0x220d7899) function"]
        pub fn get_input_snark_bytes(
            &self,
            init_num_batch: u64,
            final_new_batch: u64,
            new_local_exit_root: [u8; 32],
            old_state_root: [u8; 32],
            new_state_root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash(
                    [34, 13, 120, 153],
                    (
                        init_num_batch,
                        final_new_batch,
                        new_local_exit_root,
                        old_state_root,
                        new_state_root,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastVerifiedBatch` (0xc0ed84e0) function"]
        pub fn get_last_verified_batch(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([192, 237, 132, 224], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNextSnarkInput` (0x0eaa86ea) function"]
        pub fn get_next_snark_input(
            &self,
            pending_state_num: u64,
            init_num_batch: u64,
            final_new_batch: u64,
            new_local_exit_root: [u8; 32],
            new_state_root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [14, 170, 134, 234],
                    (
                        pending_state_num,
                        init_num_batch,
                        final_new_batch,
                        new_local_exit_root,
                        new_state_root,
                    ),
                )
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
        #[doc = "Calls the contract's `initialize` (0xd2e129f9) function"]
        pub fn initialize(
            &self,
            initialize_packed_parameters: InitializePackedParameters,
            genesis_root: [u8; 32],
            trusted_sequencer_url: String,
            network_name: String,
            version: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [210, 225, 41, 249],
                    (
                        initialize_packed_parameters,
                        genesis_root,
                        trusted_sequencer_url,
                        network_name,
                        version,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isEmergencyState` (0x15064c96) function"]
        pub fn is_emergency_state(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([21, 6, 76, 150], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPendingStateConsolidable` (0x383b3be8) function"]
        pub fn is_pending_state_consolidable(
            &self,
            pending_state_num: u64,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([56, 59, 59, 232], pending_state_num)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastBatchSequenced` (0x423fa856) function"]
        pub fn last_batch_sequenced(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([66, 63, 168, 86], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastForceBatch` (0xe7a7ed02) function"]
        pub fn last_force_batch(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([231, 167, 237, 2], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastForceBatchSequenced` (0x45605267) function"]
        pub fn last_force_batch_sequenced(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([69, 96, 82, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastPendingState` (0x458c0477) function"]
        pub fn last_pending_state(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([69, 140, 4, 119], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastPendingStateConsolidated` (0x4a1a89a7) function"]
        pub fn last_pending_state_consolidated(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([74, 26, 137, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastTimestamp` (0x19d8ac61) function"]
        pub fn last_timestamp(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([25, 216, 172, 97], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastVerifiedBatch` (0x7fcb3653) function"]
        pub fn last_verified_batch(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([127, 203, 54, 83], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `matic` (0xb6b0b097) function"]
        pub fn matic(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([182, 176, 176, 151], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `multiplierBatchFee` (0xafd23cbe) function"]
        pub fn multiplier_batch_fee(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([175, 210, 60, 190], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `networkName` (0x107bf28c) function"]
        pub fn network_name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([16, 123, 242, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `overridePendingState` (0xe11f3f18) function"]
        pub fn override_pending_state(
            &self,
            init_pending_state_num: u64,
            final_pending_state_num: u64,
            init_num_batch: u64,
            final_new_batch: u64,
            new_local_exit_root: [u8; 32],
            new_state_root: [u8; 32],
            proof_a: [ethers::core::types::U256; 2usize],
            proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
            proof_c: [ethers::core::types::U256; 2usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [225, 31, 63, 24],
                    (
                        init_pending_state_num,
                        final_pending_state_num,
                        init_num_batch,
                        final_new_batch,
                        new_local_exit_root,
                        new_state_root,
                        proof_a,
                        proof_b,
                        proof_c,
                    ),
                )
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
        #[doc = "Calls the contract's `pendingAdmin` (0x26782247) function"]
        pub fn pending_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([38, 120, 34, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingStateTimeout` (0xd939b315) function"]
        pub fn pending_state_timeout(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([217, 57, 179, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingStateTransitions` (0x837a4738) function"]
        pub fn pending_state_transitions(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, (u64, u64, [u8; 32], [u8; 32])> {
            self.0
                .method_hash([131, 122, 71, 56], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proveNonDeterministicPendingState` (0x75c508b3) function"]
        pub fn prove_non_deterministic_pending_state(
            &self,
            init_pending_state_num: u64,
            final_pending_state_num: u64,
            init_num_batch: u64,
            final_new_batch: u64,
            new_local_exit_root: [u8; 32],
            new_state_root: [u8; 32],
            proof_a: [ethers::core::types::U256; 2usize],
            proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
            proof_c: [ethers::core::types::U256; 2usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [117, 197, 8, 179],
                    (
                        init_pending_state_num,
                        final_pending_state_num,
                        init_num_batch,
                        final_new_batch,
                        new_local_exit_root,
                        new_state_root,
                        proof_a,
                        proof_b,
                        proof_c,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rollupVerifier` (0xe8bf92ed) function"]
        pub fn rollup_verifier(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([232, 191, 146, 237], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sequenceBatches` (0x5e9145c9) function"]
        pub fn sequence_batches(
            &self,
            batches: ::std::vec::Vec<BatchData>,
            fee_recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 145, 69, 201], (batches, fee_recipient))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sequenceForceBatches` (0xd8d1091b) function"]
        pub fn sequence_force_batches(
            &self,
            batches: ::std::vec::Vec<ForcedBatchData>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([216, 209, 9, 27], batches)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sequencedBatches` (0xb4d63f58) function"]
        pub fn sequenced_batches(
            &self,
            p0: u64,
        ) -> ethers::contract::builders::ContractCall<M, ([u8; 32], u64, u64)> {
            self.0
                .method_hash([180, 214, 63, 88], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMultiplierBatchFee` (0x1816b7e5) function"]
        pub fn set_multiplier_batch_fee(
            &self,
            new_multiplier_batch_fee: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 22, 183, 229], new_multiplier_batch_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setNetworkName` (0xc0cad302) function"]
        pub fn set_network_name(
            &self,
            network_name: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 202, 211, 2], network_name)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPendingStateTimeout` (0x9c9f3dfe) function"]
        pub fn set_pending_state_timeout(
            &self,
            new_pending_state_timeout: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 159, 61, 254], new_pending_state_timeout)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSequencedBatch` (0x9b796760) function"]
        pub fn set_sequenced_batch(
            &self,
            num_batch: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 121, 103, 96], num_batch)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSequencedBatches` (0xe0d17441) function"]
        pub fn set_sequenced_batches(
            &self,
            batch_num: u64,
            acc_input_data: [u8; 32],
            timestamp: u64,
            last_pending_state_consolidated: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [224, 209, 116, 65],
                    (
                        batch_num,
                        acc_input_data,
                        timestamp,
                        last_pending_state_consolidated,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStateRoot` (0xfe16564f) function"]
        pub fn set_state_root(
            &self,
            new_state_root: [u8; 32],
            batch_num: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 22, 86, 79], (new_state_root, batch_num))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTrustedAggregator` (0xf14916d6) function"]
        pub fn set_trusted_aggregator(
            &self,
            new_trusted_aggregator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 73, 22, 214], new_trusted_aggregator)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTrustedAggregatorTimeout` (0x394218e9) function"]
        pub fn set_trusted_aggregator_timeout(
            &self,
            new_trusted_aggregator_timeout: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 66, 24, 233], new_trusted_aggregator_timeout)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTrustedSequencer` (0x6ff512cc) function"]
        pub fn set_trusted_sequencer(
            &self,
            new_trusted_sequencer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 245, 18, 204], new_trusted_sequencer)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTrustedSequencerURL` (0xc89e42df) function"]
        pub fn set_trusted_sequencer_url(
            &self,
            new_trusted_sequencer_url: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 158, 66, 223], new_trusted_sequencer_url)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setVerifiedBatch` (0x96dc3d39) function"]
        pub fn set_verified_batch(
            &self,
            num_batch: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 220, 61, 57], num_batch)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setVerifyBatchTimeTarget` (0xa066215c) function"]
        pub fn set_verify_batch_time_target(
            &self,
            new_verify_batch_time_target: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 102, 33, 92], new_verify_batch_time_target)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferAdminRole` (0xada8f919) function"]
        pub fn transfer_admin_role(
            &self,
            new_pending_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 168, 249, 25], new_pending_admin)
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
        #[doc = "Calls the contract's `trustedAggregator` (0x29878983) function"]
        pub fn trusted_aggregator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([41, 135, 137, 131], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `trustedAggregatorTimeout` (0x841b24d7) function"]
        pub fn trusted_aggregator_timeout(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([132, 27, 36, 215], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `trustedSequencer` (0xcfa8ed47) function"]
        pub fn trusted_sequencer(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([207, 168, 237, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `trustedSequencerURL` (0x542028d5) function"]
        pub fn trusted_sequencer_url(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([84, 32, 40, 213], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `trustedVerifyBatchesMock` (0x10a01a72) function"]
        pub fn trusted_verify_batches_mock(
            &self,
            pending_state_num: u64,
            init_num_batch: u64,
            final_new_batch: u64,
            new_local_exit_root: [u8; 32],
            new_state_root: [u8; 32],
            proof_a: [ethers::core::types::U256; 2usize],
            proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
            proof_c: [ethers::core::types::U256; 2usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [16, 160, 26, 114],
                    (
                        pending_state_num,
                        init_num_batch,
                        final_new_batch,
                        new_local_exit_root,
                        new_state_root,
                        proof_a,
                        proof_b,
                        proof_c,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateBatchFee` (0xb4f77ea9) function"]
        pub fn update_batch_fee(
            &self,
            new_last_verified_batch: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 247, 126, 169], new_last_verified_batch)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyBatchTimeTarget` (0x0a0d9fbe) function"]
        pub fn verify_batch_time_target(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([10, 13, 159, 190], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyBatches` (0x4834a343) function"]
        pub fn verify_batches(
            &self,
            pending_state_num: u64,
            init_num_batch: u64,
            final_new_batch: u64,
            new_local_exit_root: [u8; 32],
            new_state_root: [u8; 32],
            proof_a: [ethers::core::types::U256; 2usize],
            proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
            proof_c: [ethers::core::types::U256; 2usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [72, 52, 163, 67],
                    (
                        pending_state_num,
                        init_num_batch,
                        final_new_batch,
                        new_local_exit_root,
                        new_state_root,
                        proof_a,
                        proof_b,
                        proof_c,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyBatchesTrustedAggregator` (0xf020c93e) function"]
        pub fn verify_batches_trusted_aggregator(
            &self,
            pending_state_num: u64,
            init_num_batch: u64,
            final_new_batch: u64,
            new_local_exit_root: [u8; 32],
            new_state_root: [u8; 32],
            proof_a: [ethers::core::types::U256; 2usize],
            proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
            proof_c: [ethers::core::types::U256; 2usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [240, 32, 201, 62],
                    (
                        pending_state_num,
                        init_num_batch,
                        final_new_batch,
                        new_local_exit_root,
                        new_state_root,
                        proof_a,
                        proof_b,
                        proof_c,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AcceptAdminRole` event"]
        pub fn accept_admin_role_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AcceptAdminRoleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ConsolidatePendingState` event"]
        pub fn consolidate_pending_state_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ConsolidatePendingStateFilter> {
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
        #[doc = "Gets the contract's `ForceBatch` event"]
        pub fn force_batch_filter(&self) -> ethers::contract::builders::Event<M, ForceBatchFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OverridePendingState` event"]
        pub fn override_pending_state_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OverridePendingStateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProveNonDeterministicPendingState` event"]
        pub fn prove_non_deterministic_pending_state_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProveNonDeterministicPendingStateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SequenceBatches` event"]
        pub fn sequence_batches_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SequenceBatchesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SequenceForceBatches` event"]
        pub fn sequence_force_batches_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SequenceForceBatchesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetMultiplierBatchFee` event"]
        pub fn set_multiplier_batch_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetMultiplierBatchFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetPendingStateTimeout` event"]
        pub fn set_pending_state_timeout_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetPendingStateTimeoutFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetTrustedAggregator` event"]
        pub fn set_trusted_aggregator_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetTrustedAggregatorFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetTrustedAggregatorTimeout` event"]
        pub fn set_trusted_aggregator_timeout_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetTrustedAggregatorTimeoutFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetTrustedSequencer` event"]
        pub fn set_trusted_sequencer_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetTrustedSequencerFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetTrustedSequencerURL` event"]
        pub fn set_trusted_sequencer_url_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetTrustedSequencerURLFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetVerifyBatchTimeTarget` event"]
        pub fn set_verify_batch_time_target_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetVerifyBatchTimeTargetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferAdminRole` event"]
        pub fn transfer_admin_role_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferAdminRoleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdateZkEVMVersion` event"]
        pub fn update_zk_evm_version_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpdateZkEVMVersionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VerifyBatches` event"]
        pub fn verify_batches_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VerifyBatchesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VerifyBatchesTrustedAggregator` event"]
        pub fn verify_batches_trusted_aggregator_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VerifyBatchesTrustedAggregatorFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PolygonZkEVMMockEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for PolygonZkEVMMock<M> {
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
    pub enum PolygonZkEVMMockErrors {
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
        OnlyEmergencyState(OnlyEmergencyState),
        OnlyNotEmergencyState(OnlyNotEmergencyState),
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
    impl ethers::core::abi::AbiDecode for PolygonZkEVMMockErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BatchAlreadyVerified as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::BatchAlreadyVerified(decoded));
            }
            if let Ok(decoded) =
                <BatchNotSequencedOrNotSequenceEnd as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockErrors::BatchNotSequencedOrNotSequenceEnd(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ExceedMaxVerifyBatches as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::ExceedMaxVerifyBatches(decoded));
            }
            if let Ok(decoded) =
                <FinalNumBatchBelowLastVerifiedBatch as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockErrors::FinalNumBatchBelowLastVerifiedBatch(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <FinalNumBatchDoesNotMatchPendingState as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockErrors::FinalNumBatchDoesNotMatchPendingState(decoded));
            }
            if let Ok(decoded) =
                <FinalPendingStateNumInvalid as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::FinalPendingStateNumInvalid(decoded));
            }
            if let Ok(decoded) =
                <ForceBatchTimeoutNotExpired as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::ForceBatchTimeoutNotExpired(decoded));
            }
            if let Ok(decoded) =
                <ForceBatchesOverflow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::ForceBatchesOverflow(decoded));
            }
            if let Ok(decoded) =
                <ForcedDataDoesNotMatch as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::ForcedDataDoesNotMatch(decoded));
            }
            if let Ok(decoded) =
                <GlobalExitRootNotExist as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::GlobalExitRootNotExist(decoded));
            }
            if let Ok(decoded) =
                <HaltTimeoutNotExpired as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::HaltTimeoutNotExpired(decoded));
            }
            if let Ok(decoded) =
                <InitNumBatchAboveLastVerifiedBatch as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockErrors::InitNumBatchAboveLastVerifiedBatch(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InitNumBatchDoesNotMatchPendingState as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockErrors::InitNumBatchDoesNotMatchPendingState(decoded));
            }
            if let Ok(decoded) =
                <InvalidProof as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::InvalidProof(decoded));
            }
            if let Ok(decoded) =
                <InvalidRangeBatchTimeTarget as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::InvalidRangeBatchTimeTarget(decoded));
            }
            if let Ok(decoded) =
                <InvalidRangeMultiplierBatchFee as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockErrors::InvalidRangeMultiplierBatchFee(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <NewAccInputHashDoesNotExist as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::NewAccInputHashDoesNotExist(decoded));
            }
            if let Ok(decoded) =
                <NewPendingStateTimeoutMustBeLower as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockErrors::NewPendingStateTimeoutMustBeLower(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <NewTrustedAggregatorTimeoutMustBeLower as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockErrors::NewTrustedAggregatorTimeoutMustBeLower(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughMaticAmount as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::NotEnoughMaticAmount(decoded));
            }
            if let Ok(decoded) =
                <OldAccInputHashDoesNotExist as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::OldAccInputHashDoesNotExist(decoded));
            }
            if let Ok(decoded) =
                <OldStateRootDoesNotExist as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::OldStateRootDoesNotExist(decoded));
            }
            if let Ok(decoded) = <OnlyAdmin as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::OnlyAdmin(decoded));
            }
            if let Ok(decoded) =
                <OnlyEmergencyState as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::OnlyEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <OnlyNotEmergencyState as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::OnlyNotEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <OnlyPendingAdmin as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::OnlyPendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <OnlyTrustedAggregator as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::OnlyTrustedAggregator(decoded));
            }
            if let Ok(decoded) =
                <OnlyTrustedSequencer as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::OnlyTrustedSequencer(decoded));
            }
            if let Ok(decoded) =
                <PendingStateDoesNotExist as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::PendingStateDoesNotExist(decoded));
            }
            if let Ok(decoded) =
                <PendingStateInvalid as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::PendingStateInvalid(decoded));
            }
            if let Ok(decoded) =
                <PendingStateNotConsolidable as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::PendingStateNotConsolidable(decoded));
            }
            if let Ok (decoded) = < PendingStateTimeoutExceedHaltAggregationTimeout as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (PolygonZkEVMMockErrors :: PendingStateTimeoutExceedHaltAggregationTimeout (decoded)) }
            if let Ok(decoded) =
                <SequenceZeroBatches as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::SequenceZeroBatches(decoded));
            }
            if let Ok(decoded) =
                <SequencedTimestampBelowForcedTimestamp as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockErrors::SequencedTimestampBelowForcedTimestamp(decoded));
            }
            if let Ok(decoded) =
                <SequencedTimestampInvalid as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::SequencedTimestampInvalid(decoded));
            }
            if let Ok(decoded) =
                <StoredRootMustBeDifferentThanNewRoot as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockErrors::StoredRootMustBeDifferentThanNewRoot(decoded));
            }
            if let Ok(decoded) =
                <TransactionsLengthAboveMax as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockErrors::TransactionsLengthAboveMax(decoded));
            }
            if let Ok (decoded) = < TrustedAggregatorTimeoutExceedHaltAggregationTimeout as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (PolygonZkEVMMockErrors :: TrustedAggregatorTimeoutExceedHaltAggregationTimeout (decoded)) }
            if let Ok(decoded) =
                <TrustedAggregatorTimeoutNotExpired as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockErrors::TrustedAggregatorTimeoutNotExpired(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PolygonZkEVMMockErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                PolygonZkEVMMockErrors::BatchAlreadyVerified(element) => element.encode(),
                PolygonZkEVMMockErrors::BatchNotSequencedOrNotSequenceEnd(element) => {
                    element.encode()
                }
                PolygonZkEVMMockErrors::ExceedMaxVerifyBatches(element) => element.encode(),
                PolygonZkEVMMockErrors::FinalNumBatchBelowLastVerifiedBatch(element) => {
                    element.encode()
                }
                PolygonZkEVMMockErrors::FinalNumBatchDoesNotMatchPendingState(element) => {
                    element.encode()
                }
                PolygonZkEVMMockErrors::FinalPendingStateNumInvalid(element) => element.encode(),
                PolygonZkEVMMockErrors::ForceBatchTimeoutNotExpired(element) => element.encode(),
                PolygonZkEVMMockErrors::ForceBatchesOverflow(element) => element.encode(),
                PolygonZkEVMMockErrors::ForcedDataDoesNotMatch(element) => element.encode(),
                PolygonZkEVMMockErrors::GlobalExitRootNotExist(element) => element.encode(),
                PolygonZkEVMMockErrors::HaltTimeoutNotExpired(element) => element.encode(),
                PolygonZkEVMMockErrors::InitNumBatchAboveLastVerifiedBatch(element) => {
                    element.encode()
                }
                PolygonZkEVMMockErrors::InitNumBatchDoesNotMatchPendingState(element) => {
                    element.encode()
                }
                PolygonZkEVMMockErrors::InvalidProof(element) => element.encode(),
                PolygonZkEVMMockErrors::InvalidRangeBatchTimeTarget(element) => element.encode(),
                PolygonZkEVMMockErrors::InvalidRangeMultiplierBatchFee(element) => element.encode(),
                PolygonZkEVMMockErrors::NewAccInputHashDoesNotExist(element) => element.encode(),
                PolygonZkEVMMockErrors::NewPendingStateTimeoutMustBeLower(element) => {
                    element.encode()
                }
                PolygonZkEVMMockErrors::NewTrustedAggregatorTimeoutMustBeLower(element) => {
                    element.encode()
                }
                PolygonZkEVMMockErrors::NotEnoughMaticAmount(element) => element.encode(),
                PolygonZkEVMMockErrors::OldAccInputHashDoesNotExist(element) => element.encode(),
                PolygonZkEVMMockErrors::OldStateRootDoesNotExist(element) => element.encode(),
                PolygonZkEVMMockErrors::OnlyAdmin(element) => element.encode(),
                PolygonZkEVMMockErrors::OnlyEmergencyState(element) => element.encode(),
                PolygonZkEVMMockErrors::OnlyNotEmergencyState(element) => element.encode(),
                PolygonZkEVMMockErrors::OnlyPendingAdmin(element) => element.encode(),
                PolygonZkEVMMockErrors::OnlyTrustedAggregator(element) => element.encode(),
                PolygonZkEVMMockErrors::OnlyTrustedSequencer(element) => element.encode(),
                PolygonZkEVMMockErrors::PendingStateDoesNotExist(element) => element.encode(),
                PolygonZkEVMMockErrors::PendingStateInvalid(element) => element.encode(),
                PolygonZkEVMMockErrors::PendingStateNotConsolidable(element) => element.encode(),
                PolygonZkEVMMockErrors::PendingStateTimeoutExceedHaltAggregationTimeout(
                    element,
                ) => element.encode(),
                PolygonZkEVMMockErrors::SequenceZeroBatches(element) => element.encode(),
                PolygonZkEVMMockErrors::SequencedTimestampBelowForcedTimestamp(element) => {
                    element.encode()
                }
                PolygonZkEVMMockErrors::SequencedTimestampInvalid(element) => element.encode(),
                PolygonZkEVMMockErrors::StoredRootMustBeDifferentThanNewRoot(element) => {
                    element.encode()
                }
                PolygonZkEVMMockErrors::TransactionsLengthAboveMax(element) => element.encode(),
                PolygonZkEVMMockErrors::TrustedAggregatorTimeoutExceedHaltAggregationTimeout(
                    element,
                ) => element.encode(),
                PolygonZkEVMMockErrors::TrustedAggregatorTimeoutNotExpired(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMMockErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMMockErrors::BatchAlreadyVerified(element) => element.fmt(f),
                PolygonZkEVMMockErrors::BatchNotSequencedOrNotSequenceEnd(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMMockErrors::ExceedMaxVerifyBatches(element) => element.fmt(f),
                PolygonZkEVMMockErrors::FinalNumBatchBelowLastVerifiedBatch(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMMockErrors::FinalNumBatchDoesNotMatchPendingState(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMMockErrors::FinalPendingStateNumInvalid(element) => element.fmt(f),
                PolygonZkEVMMockErrors::ForceBatchTimeoutNotExpired(element) => element.fmt(f),
                PolygonZkEVMMockErrors::ForceBatchesOverflow(element) => element.fmt(f),
                PolygonZkEVMMockErrors::ForcedDataDoesNotMatch(element) => element.fmt(f),
                PolygonZkEVMMockErrors::GlobalExitRootNotExist(element) => element.fmt(f),
                PolygonZkEVMMockErrors::HaltTimeoutNotExpired(element) => element.fmt(f),
                PolygonZkEVMMockErrors::InitNumBatchAboveLastVerifiedBatch(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMMockErrors::InitNumBatchDoesNotMatchPendingState(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMMockErrors::InvalidProof(element) => element.fmt(f),
                PolygonZkEVMMockErrors::InvalidRangeBatchTimeTarget(element) => element.fmt(f),
                PolygonZkEVMMockErrors::InvalidRangeMultiplierBatchFee(element) => element.fmt(f),
                PolygonZkEVMMockErrors::NewAccInputHashDoesNotExist(element) => element.fmt(f),
                PolygonZkEVMMockErrors::NewPendingStateTimeoutMustBeLower(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMMockErrors::NewTrustedAggregatorTimeoutMustBeLower(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMMockErrors::NotEnoughMaticAmount(element) => element.fmt(f),
                PolygonZkEVMMockErrors::OldAccInputHashDoesNotExist(element) => element.fmt(f),
                PolygonZkEVMMockErrors::OldStateRootDoesNotExist(element) => element.fmt(f),
                PolygonZkEVMMockErrors::OnlyAdmin(element) => element.fmt(f),
                PolygonZkEVMMockErrors::OnlyEmergencyState(element) => element.fmt(f),
                PolygonZkEVMMockErrors::OnlyNotEmergencyState(element) => element.fmt(f),
                PolygonZkEVMMockErrors::OnlyPendingAdmin(element) => element.fmt(f),
                PolygonZkEVMMockErrors::OnlyTrustedAggregator(element) => element.fmt(f),
                PolygonZkEVMMockErrors::OnlyTrustedSequencer(element) => element.fmt(f),
                PolygonZkEVMMockErrors::PendingStateDoesNotExist(element) => element.fmt(f),
                PolygonZkEVMMockErrors::PendingStateInvalid(element) => element.fmt(f),
                PolygonZkEVMMockErrors::PendingStateNotConsolidable(element) => element.fmt(f),
                PolygonZkEVMMockErrors::PendingStateTimeoutExceedHaltAggregationTimeout(
                    element,
                ) => element.fmt(f),
                PolygonZkEVMMockErrors::SequenceZeroBatches(element) => element.fmt(f),
                PolygonZkEVMMockErrors::SequencedTimestampBelowForcedTimestamp(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMMockErrors::SequencedTimestampInvalid(element) => element.fmt(f),
                PolygonZkEVMMockErrors::StoredRootMustBeDifferentThanNewRoot(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMMockErrors::TransactionsLengthAboveMax(element) => element.fmt(f),
                PolygonZkEVMMockErrors::TrustedAggregatorTimeoutExceedHaltAggregationTimeout(
                    element,
                ) => element.fmt(f),
                PolygonZkEVMMockErrors::TrustedAggregatorTimeoutNotExpired(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<BatchAlreadyVerified> for PolygonZkEVMMockErrors {
        fn from(var: BatchAlreadyVerified) -> Self {
            PolygonZkEVMMockErrors::BatchAlreadyVerified(var)
        }
    }
    impl ::std::convert::From<BatchNotSequencedOrNotSequenceEnd> for PolygonZkEVMMockErrors {
        fn from(var: BatchNotSequencedOrNotSequenceEnd) -> Self {
            PolygonZkEVMMockErrors::BatchNotSequencedOrNotSequenceEnd(var)
        }
    }
    impl ::std::convert::From<ExceedMaxVerifyBatches> for PolygonZkEVMMockErrors {
        fn from(var: ExceedMaxVerifyBatches) -> Self {
            PolygonZkEVMMockErrors::ExceedMaxVerifyBatches(var)
        }
    }
    impl ::std::convert::From<FinalNumBatchBelowLastVerifiedBatch> for PolygonZkEVMMockErrors {
        fn from(var: FinalNumBatchBelowLastVerifiedBatch) -> Self {
            PolygonZkEVMMockErrors::FinalNumBatchBelowLastVerifiedBatch(var)
        }
    }
    impl ::std::convert::From<FinalNumBatchDoesNotMatchPendingState> for PolygonZkEVMMockErrors {
        fn from(var: FinalNumBatchDoesNotMatchPendingState) -> Self {
            PolygonZkEVMMockErrors::FinalNumBatchDoesNotMatchPendingState(var)
        }
    }
    impl ::std::convert::From<FinalPendingStateNumInvalid> for PolygonZkEVMMockErrors {
        fn from(var: FinalPendingStateNumInvalid) -> Self {
            PolygonZkEVMMockErrors::FinalPendingStateNumInvalid(var)
        }
    }
    impl ::std::convert::From<ForceBatchTimeoutNotExpired> for PolygonZkEVMMockErrors {
        fn from(var: ForceBatchTimeoutNotExpired) -> Self {
            PolygonZkEVMMockErrors::ForceBatchTimeoutNotExpired(var)
        }
    }
    impl ::std::convert::From<ForceBatchesOverflow> for PolygonZkEVMMockErrors {
        fn from(var: ForceBatchesOverflow) -> Self {
            PolygonZkEVMMockErrors::ForceBatchesOverflow(var)
        }
    }
    impl ::std::convert::From<ForcedDataDoesNotMatch> for PolygonZkEVMMockErrors {
        fn from(var: ForcedDataDoesNotMatch) -> Self {
            PolygonZkEVMMockErrors::ForcedDataDoesNotMatch(var)
        }
    }
    impl ::std::convert::From<GlobalExitRootNotExist> for PolygonZkEVMMockErrors {
        fn from(var: GlobalExitRootNotExist) -> Self {
            PolygonZkEVMMockErrors::GlobalExitRootNotExist(var)
        }
    }
    impl ::std::convert::From<HaltTimeoutNotExpired> for PolygonZkEVMMockErrors {
        fn from(var: HaltTimeoutNotExpired) -> Self {
            PolygonZkEVMMockErrors::HaltTimeoutNotExpired(var)
        }
    }
    impl ::std::convert::From<InitNumBatchAboveLastVerifiedBatch> for PolygonZkEVMMockErrors {
        fn from(var: InitNumBatchAboveLastVerifiedBatch) -> Self {
            PolygonZkEVMMockErrors::InitNumBatchAboveLastVerifiedBatch(var)
        }
    }
    impl ::std::convert::From<InitNumBatchDoesNotMatchPendingState> for PolygonZkEVMMockErrors {
        fn from(var: InitNumBatchDoesNotMatchPendingState) -> Self {
            PolygonZkEVMMockErrors::InitNumBatchDoesNotMatchPendingState(var)
        }
    }
    impl ::std::convert::From<InvalidProof> for PolygonZkEVMMockErrors {
        fn from(var: InvalidProof) -> Self {
            PolygonZkEVMMockErrors::InvalidProof(var)
        }
    }
    impl ::std::convert::From<InvalidRangeBatchTimeTarget> for PolygonZkEVMMockErrors {
        fn from(var: InvalidRangeBatchTimeTarget) -> Self {
            PolygonZkEVMMockErrors::InvalidRangeBatchTimeTarget(var)
        }
    }
    impl ::std::convert::From<InvalidRangeMultiplierBatchFee> for PolygonZkEVMMockErrors {
        fn from(var: InvalidRangeMultiplierBatchFee) -> Self {
            PolygonZkEVMMockErrors::InvalidRangeMultiplierBatchFee(var)
        }
    }
    impl ::std::convert::From<NewAccInputHashDoesNotExist> for PolygonZkEVMMockErrors {
        fn from(var: NewAccInputHashDoesNotExist) -> Self {
            PolygonZkEVMMockErrors::NewAccInputHashDoesNotExist(var)
        }
    }
    impl ::std::convert::From<NewPendingStateTimeoutMustBeLower> for PolygonZkEVMMockErrors {
        fn from(var: NewPendingStateTimeoutMustBeLower) -> Self {
            PolygonZkEVMMockErrors::NewPendingStateTimeoutMustBeLower(var)
        }
    }
    impl ::std::convert::From<NewTrustedAggregatorTimeoutMustBeLower> for PolygonZkEVMMockErrors {
        fn from(var: NewTrustedAggregatorTimeoutMustBeLower) -> Self {
            PolygonZkEVMMockErrors::NewTrustedAggregatorTimeoutMustBeLower(var)
        }
    }
    impl ::std::convert::From<NotEnoughMaticAmount> for PolygonZkEVMMockErrors {
        fn from(var: NotEnoughMaticAmount) -> Self {
            PolygonZkEVMMockErrors::NotEnoughMaticAmount(var)
        }
    }
    impl ::std::convert::From<OldAccInputHashDoesNotExist> for PolygonZkEVMMockErrors {
        fn from(var: OldAccInputHashDoesNotExist) -> Self {
            PolygonZkEVMMockErrors::OldAccInputHashDoesNotExist(var)
        }
    }
    impl ::std::convert::From<OldStateRootDoesNotExist> for PolygonZkEVMMockErrors {
        fn from(var: OldStateRootDoesNotExist) -> Self {
            PolygonZkEVMMockErrors::OldStateRootDoesNotExist(var)
        }
    }
    impl ::std::convert::From<OnlyAdmin> for PolygonZkEVMMockErrors {
        fn from(var: OnlyAdmin) -> Self {
            PolygonZkEVMMockErrors::OnlyAdmin(var)
        }
    }
    impl ::std::convert::From<OnlyEmergencyState> for PolygonZkEVMMockErrors {
        fn from(var: OnlyEmergencyState) -> Self {
            PolygonZkEVMMockErrors::OnlyEmergencyState(var)
        }
    }
    impl ::std::convert::From<OnlyNotEmergencyState> for PolygonZkEVMMockErrors {
        fn from(var: OnlyNotEmergencyState) -> Self {
            PolygonZkEVMMockErrors::OnlyNotEmergencyState(var)
        }
    }
    impl ::std::convert::From<OnlyPendingAdmin> for PolygonZkEVMMockErrors {
        fn from(var: OnlyPendingAdmin) -> Self {
            PolygonZkEVMMockErrors::OnlyPendingAdmin(var)
        }
    }
    impl ::std::convert::From<OnlyTrustedAggregator> for PolygonZkEVMMockErrors {
        fn from(var: OnlyTrustedAggregator) -> Self {
            PolygonZkEVMMockErrors::OnlyTrustedAggregator(var)
        }
    }
    impl ::std::convert::From<OnlyTrustedSequencer> for PolygonZkEVMMockErrors {
        fn from(var: OnlyTrustedSequencer) -> Self {
            PolygonZkEVMMockErrors::OnlyTrustedSequencer(var)
        }
    }
    impl ::std::convert::From<PendingStateDoesNotExist> for PolygonZkEVMMockErrors {
        fn from(var: PendingStateDoesNotExist) -> Self {
            PolygonZkEVMMockErrors::PendingStateDoesNotExist(var)
        }
    }
    impl ::std::convert::From<PendingStateInvalid> for PolygonZkEVMMockErrors {
        fn from(var: PendingStateInvalid) -> Self {
            PolygonZkEVMMockErrors::PendingStateInvalid(var)
        }
    }
    impl ::std::convert::From<PendingStateNotConsolidable> for PolygonZkEVMMockErrors {
        fn from(var: PendingStateNotConsolidable) -> Self {
            PolygonZkEVMMockErrors::PendingStateNotConsolidable(var)
        }
    }
    impl ::std::convert::From<PendingStateTimeoutExceedHaltAggregationTimeout>
        for PolygonZkEVMMockErrors
    {
        fn from(var: PendingStateTimeoutExceedHaltAggregationTimeout) -> Self {
            PolygonZkEVMMockErrors::PendingStateTimeoutExceedHaltAggregationTimeout(var)
        }
    }
    impl ::std::convert::From<SequenceZeroBatches> for PolygonZkEVMMockErrors {
        fn from(var: SequenceZeroBatches) -> Self {
            PolygonZkEVMMockErrors::SequenceZeroBatches(var)
        }
    }
    impl ::std::convert::From<SequencedTimestampBelowForcedTimestamp> for PolygonZkEVMMockErrors {
        fn from(var: SequencedTimestampBelowForcedTimestamp) -> Self {
            PolygonZkEVMMockErrors::SequencedTimestampBelowForcedTimestamp(var)
        }
    }
    impl ::std::convert::From<SequencedTimestampInvalid> for PolygonZkEVMMockErrors {
        fn from(var: SequencedTimestampInvalid) -> Self {
            PolygonZkEVMMockErrors::SequencedTimestampInvalid(var)
        }
    }
    impl ::std::convert::From<StoredRootMustBeDifferentThanNewRoot> for PolygonZkEVMMockErrors {
        fn from(var: StoredRootMustBeDifferentThanNewRoot) -> Self {
            PolygonZkEVMMockErrors::StoredRootMustBeDifferentThanNewRoot(var)
        }
    }
    impl ::std::convert::From<TransactionsLengthAboveMax> for PolygonZkEVMMockErrors {
        fn from(var: TransactionsLengthAboveMax) -> Self {
            PolygonZkEVMMockErrors::TransactionsLengthAboveMax(var)
        }
    }
    impl ::std::convert::From<TrustedAggregatorTimeoutExceedHaltAggregationTimeout>
        for PolygonZkEVMMockErrors
    {
        fn from(var: TrustedAggregatorTimeoutExceedHaltAggregationTimeout) -> Self {
            PolygonZkEVMMockErrors::TrustedAggregatorTimeoutExceedHaltAggregationTimeout(var)
        }
    }
    impl ::std::convert::From<TrustedAggregatorTimeoutNotExpired> for PolygonZkEVMMockErrors {
        fn from(var: TrustedAggregatorTimeoutNotExpired) -> Self {
            PolygonZkEVMMockErrors::TrustedAggregatorTimeoutNotExpired(var)
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
    #[ethevent(name = "AcceptAdminRole", abi = "AcceptAdminRole(address)")]
    pub struct AcceptAdminRoleFilter {
        pub new_admin: ethers::core::types::Address,
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
        name = "ConsolidatePendingState",
        abi = "ConsolidatePendingState(uint64,bytes32,uint64)"
    )]
    pub struct ConsolidatePendingStateFilter {
        #[ethevent(indexed)]
        pub num_batch: u64,
        pub state_root: [u8; 32],
        #[ethevent(indexed)]
        pub pending_state_num: u64,
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
    #[ethevent(name = "ForceBatch", abi = "ForceBatch(uint64,bytes32,address,bytes)")]
    pub struct ForceBatchFilter {
        #[ethevent(indexed)]
        pub force_batch_num: u64,
        pub last_global_exit_root: [u8; 32],
        pub sequencer: ethers::core::types::Address,
        pub transactions: ethers::core::types::Bytes,
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
        name = "OverridePendingState",
        abi = "OverridePendingState(uint64,bytes32,address)"
    )]
    pub struct OverridePendingStateFilter {
        #[ethevent(indexed)]
        pub num_batch: u64,
        pub state_root: [u8; 32],
        #[ethevent(indexed)]
        pub aggregator: ethers::core::types::Address,
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
        name = "ProveNonDeterministicPendingState",
        abi = "ProveNonDeterministicPendingState(bytes32,bytes32)"
    )]
    pub struct ProveNonDeterministicPendingStateFilter {
        pub stored_state_root: [u8; 32],
        pub proved_state_root: [u8; 32],
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
    #[ethevent(name = "SequenceBatches", abi = "SequenceBatches(uint64)")]
    pub struct SequenceBatchesFilter {
        #[ethevent(indexed)]
        pub num_batch: u64,
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
    #[ethevent(name = "SequenceForceBatches", abi = "SequenceForceBatches(uint64)")]
    pub struct SequenceForceBatchesFilter {
        #[ethevent(indexed)]
        pub num_batch: u64,
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
    #[ethevent(name = "SetMultiplierBatchFee", abi = "SetMultiplierBatchFee(uint16)")]
    pub struct SetMultiplierBatchFeeFilter {
        pub new_multiplier_batch_fee: u16,
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
        name = "SetPendingStateTimeout",
        abi = "SetPendingStateTimeout(uint64)"
    )]
    pub struct SetPendingStateTimeoutFilter {
        pub new_pending_state_timeout: u64,
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
    #[ethevent(name = "SetTrustedAggregator", abi = "SetTrustedAggregator(address)")]
    pub struct SetTrustedAggregatorFilter {
        pub new_trusted_aggregator: ethers::core::types::Address,
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
        name = "SetTrustedAggregatorTimeout",
        abi = "SetTrustedAggregatorTimeout(uint64)"
    )]
    pub struct SetTrustedAggregatorTimeoutFilter {
        pub new_trusted_aggregator_timeout: u64,
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
    #[ethevent(name = "SetTrustedSequencer", abi = "SetTrustedSequencer(address)")]
    pub struct SetTrustedSequencerFilter {
        pub new_trusted_sequencer: ethers::core::types::Address,
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
        name = "SetTrustedSequencerURL",
        abi = "SetTrustedSequencerURL(string)"
    )]
    pub struct SetTrustedSequencerURLFilter {
        pub new_trusted_sequencer_url: String,
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
        name = "SetVerifyBatchTimeTarget",
        abi = "SetVerifyBatchTimeTarget(uint64)"
    )]
    pub struct SetVerifyBatchTimeTargetFilter {
        pub new_verify_batch_time_target: u64,
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
    #[ethevent(name = "TransferAdminRole", abi = "TransferAdminRole(address)")]
    pub struct TransferAdminRoleFilter {
        pub new_pending_admin: ethers::core::types::Address,
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
        name = "UpdateZkEVMVersion",
        abi = "UpdateZkEVMVersion(uint64,uint64,string)"
    )]
    pub struct UpdateZkEVMVersionFilter {
        pub num_batch: u64,
        pub fork_id: u64,
        pub version: String,
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
    #[ethevent(name = "VerifyBatches", abi = "VerifyBatches(uint64,bytes32,address)")]
    pub struct VerifyBatchesFilter {
        #[ethevent(indexed)]
        pub num_batch: u64,
        pub state_root: [u8; 32],
        #[ethevent(indexed)]
        pub aggregator: ethers::core::types::Address,
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
        name = "VerifyBatchesTrustedAggregator",
        abi = "VerifyBatchesTrustedAggregator(uint64,bytes32,address)"
    )]
    pub struct VerifyBatchesTrustedAggregatorFilter {
        #[ethevent(indexed)]
        pub num_batch: u64,
        pub state_root: [u8; 32],
        #[ethevent(indexed)]
        pub aggregator: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PolygonZkEVMMockEvents {
        AcceptAdminRoleFilter(AcceptAdminRoleFilter),
        ConsolidatePendingStateFilter(ConsolidatePendingStateFilter),
        EmergencyStateActivatedFilter(EmergencyStateActivatedFilter),
        EmergencyStateDeactivatedFilter(EmergencyStateDeactivatedFilter),
        ForceBatchFilter(ForceBatchFilter),
        InitializedFilter(InitializedFilter),
        OverridePendingStateFilter(OverridePendingStateFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProveNonDeterministicPendingStateFilter(ProveNonDeterministicPendingStateFilter),
        SequenceBatchesFilter(SequenceBatchesFilter),
        SequenceForceBatchesFilter(SequenceForceBatchesFilter),
        SetMultiplierBatchFeeFilter(SetMultiplierBatchFeeFilter),
        SetPendingStateTimeoutFilter(SetPendingStateTimeoutFilter),
        SetTrustedAggregatorFilter(SetTrustedAggregatorFilter),
        SetTrustedAggregatorTimeoutFilter(SetTrustedAggregatorTimeoutFilter),
        SetTrustedSequencerFilter(SetTrustedSequencerFilter),
        SetTrustedSequencerURLFilter(SetTrustedSequencerURLFilter),
        SetVerifyBatchTimeTargetFilter(SetVerifyBatchTimeTargetFilter),
        TransferAdminRoleFilter(TransferAdminRoleFilter),
        UpdateZkEVMVersionFilter(UpdateZkEVMVersionFilter),
        VerifyBatchesFilter(VerifyBatchesFilter),
        VerifyBatchesTrustedAggregatorFilter(VerifyBatchesTrustedAggregatorFilter),
    }
    impl ethers::contract::EthLogDecode for PolygonZkEVMMockEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AcceptAdminRoleFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::AcceptAdminRoleFilter(decoded));
            }
            if let Ok(decoded) = ConsolidatePendingStateFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::ConsolidatePendingStateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EmergencyStateActivatedFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::EmergencyStateActivatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EmergencyStateDeactivatedFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::EmergencyStateDeactivatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ForceBatchFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::ForceBatchFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OverridePendingStateFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::OverridePendingStateFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProveNonDeterministicPendingStateFilter::decode_log(log) {
                return Ok(
                    PolygonZkEVMMockEvents::ProveNonDeterministicPendingStateFilter(decoded),
                );
            }
            if let Ok(decoded) = SequenceBatchesFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::SequenceBatchesFilter(decoded));
            }
            if let Ok(decoded) = SequenceForceBatchesFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::SequenceForceBatchesFilter(decoded));
            }
            if let Ok(decoded) = SetMultiplierBatchFeeFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::SetMultiplierBatchFeeFilter(decoded));
            }
            if let Ok(decoded) = SetPendingStateTimeoutFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::SetPendingStateTimeoutFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SetTrustedAggregatorFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::SetTrustedAggregatorFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedAggregatorTimeoutFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::SetTrustedAggregatorTimeoutFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SetTrustedSequencerFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::SetTrustedSequencerFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedSequencerURLFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::SetTrustedSequencerURLFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SetVerifyBatchTimeTargetFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::SetVerifyBatchTimeTargetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TransferAdminRoleFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::TransferAdminRoleFilter(decoded));
            }
            if let Ok(decoded) = UpdateZkEVMVersionFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::UpdateZkEVMVersionFilter(decoded));
            }
            if let Ok(decoded) = VerifyBatchesFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::VerifyBatchesFilter(decoded));
            }
            if let Ok(decoded) = VerifyBatchesTrustedAggregatorFilter::decode_log(log) {
                return Ok(PolygonZkEVMMockEvents::VerifyBatchesTrustedAggregatorFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMMockEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMMockEvents::AcceptAdminRoleFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::ConsolidatePendingStateFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::EmergencyStateActivatedFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::EmergencyStateDeactivatedFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::ForceBatchFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::InitializedFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::OverridePendingStateFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::ProveNonDeterministicPendingStateFilter(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMMockEvents::SequenceBatchesFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::SequenceForceBatchesFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::SetMultiplierBatchFeeFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::SetPendingStateTimeoutFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::SetTrustedAggregatorFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::SetTrustedAggregatorTimeoutFilter(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMMockEvents::SetTrustedSequencerFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::SetTrustedSequencerURLFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::SetVerifyBatchTimeTargetFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::TransferAdminRoleFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::UpdateZkEVMVersionFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::VerifyBatchesFilter(element) => element.fmt(f),
                PolygonZkEVMMockEvents::VerifyBatchesTrustedAggregatorFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `acceptAdminRole` function with signature `acceptAdminRole()` and selector `[140, 61, 115, 1]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "acceptAdminRole", abi = "acceptAdminRole()")]
    pub struct AcceptAdminRoleCall;
    #[doc = "Container type for all input parameters for the `activateEmergencyState` function with signature `activateEmergencyState(uint64)` and selector `[114, 21, 84, 26]`"]
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
        name = "activateEmergencyState",
        abi = "activateEmergencyState(uint64)"
    )]
    pub struct ActivateEmergencyStateCall {
        pub sequenced_batch_num: u64,
    }
    #[doc = "Container type for all input parameters for the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    #[doc = "Container type for all input parameters for the `batchFee` function with signature `batchFee()` and selector `[248, 184, 35, 228]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "batchFee", abi = "batchFee()")]
    pub struct BatchFeeCall;
    #[doc = "Container type for all input parameters for the `batchNumToStateRoot` function with signature `batchNumToStateRoot(uint64)` and selector `[83, 146, 197, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "batchNumToStateRoot", abi = "batchNumToStateRoot(uint64)")]
    pub struct BatchNumToStateRootCall(pub u64);
    #[doc = "Container type for all input parameters for the `bridgeAddress` function with signature `bridgeAddress()` and selector `[163, 197, 115, 235]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "bridgeAddress", abi = "bridgeAddress()")]
    pub struct BridgeAddressCall;
    #[doc = "Container type for all input parameters for the `calculateAccInputHash` function with signature `calculateAccInputHash(bytes32,bytes,bytes32,uint64,address)` and selector `[87, 79, 100, 158]`"]
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
        name = "calculateAccInputHash",
        abi = "calculateAccInputHash(bytes32,bytes,bytes32,uint64,address)"
    )]
    pub struct CalculateAccInputHashCall {
        pub current_acc_input_hash: [u8; 32],
        pub transactions: ethers::core::types::Bytes,
        pub global_exit_root: [u8; 32],
        pub timestamp: u64,
        pub sequencer_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `calculateRewardPerBatch` function with signature `calculateRewardPerBatch()` and selector `[153, 245, 99, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "calculateRewardPerBatch", abi = "calculateRewardPerBatch()")]
    pub struct CalculateRewardPerBatchCall;
    #[doc = "Container type for all input parameters for the `chainID` function with signature `chainID()` and selector `[173, 200, 121, 233]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "chainID", abi = "chainID()")]
    pub struct ChainIDCall;
    #[doc = "Container type for all input parameters for the `consolidatePendingState` function with signature `consolidatePendingState(uint64)` and selector `[74, 145, 14, 106]`"]
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
        name = "consolidatePendingState",
        abi = "consolidatePendingState(uint64)"
    )]
    pub struct ConsolidatePendingStateCall {
        pub pending_state_num: u64,
    }
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
    #[doc = "Container type for all input parameters for the `forceBatch` function with signature `forceBatch(bytes,uint256)` and selector `[234, 235, 7, 123]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "forceBatch", abi = "forceBatch(bytes,uint256)")]
    pub struct ForceBatchCall {
        pub transactions: ethers::core::types::Bytes,
        pub matic_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `forcedBatches` function with signature `forcedBatches(uint64)` and selector `[107, 134, 22, 206]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "forcedBatches", abi = "forcedBatches(uint64)")]
    pub struct ForcedBatchesCall(pub u64);
    #[doc = "Container type for all input parameters for the `forkID` function with signature `forkID()` and selector `[131, 28, 126, 173]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "forkID", abi = "forkID()")]
    pub struct ForkIDCall;
    #[doc = "Container type for all input parameters for the `getCurrentBatchFee` function with signature `getCurrentBatchFee()` and selector `[159, 13, 3, 157]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCurrentBatchFee", abi = "getCurrentBatchFee()")]
    pub struct GetCurrentBatchFeeCall;
    #[doc = "Container type for all input parameters for the `getInputSnarkBytes` function with signature `getInputSnarkBytes(uint64,uint64,bytes32,bytes32,bytes32)` and selector `[34, 13, 120, 153]`"]
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
        name = "getInputSnarkBytes",
        abi = "getInputSnarkBytes(uint64,uint64,bytes32,bytes32,bytes32)"
    )]
    pub struct GetInputSnarkBytesCall {
        pub init_num_batch: u64,
        pub final_new_batch: u64,
        pub new_local_exit_root: [u8; 32],
        pub old_state_root: [u8; 32],
        pub new_state_root: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getLastVerifiedBatch` function with signature `getLastVerifiedBatch()` and selector `[192, 237, 132, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getLastVerifiedBatch", abi = "getLastVerifiedBatch()")]
    pub struct GetLastVerifiedBatchCall;
    #[doc = "Container type for all input parameters for the `getNextSnarkInput` function with signature `getNextSnarkInput(uint64,uint64,uint64,bytes32,bytes32)` and selector `[14, 170, 134, 234]`"]
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
        name = "getNextSnarkInput",
        abi = "getNextSnarkInput(uint64,uint64,uint64,bytes32,bytes32)"
    )]
    pub struct GetNextSnarkInputCall {
        pub pending_state_num: u64,
        pub init_num_batch: u64,
        pub final_new_batch: u64,
        pub new_local_exit_root: [u8; 32],
        pub new_state_root: [u8; 32],
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
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize((address,address,uint64,address,uint64),bytes32,string,string,string)` and selector `[210, 225, 41, 249]`"]
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
        name = "initialize",
        abi = "initialize((address,address,uint64,address,uint64),bytes32,string,string,string)"
    )]
    pub struct InitializeCall {
        pub initialize_packed_parameters: InitializePackedParameters,
        pub genesis_root: [u8; 32],
        pub trusted_sequencer_url: String,
        pub network_name: String,
        pub version: String,
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
    #[doc = "Container type for all input parameters for the `isPendingStateConsolidable` function with signature `isPendingStateConsolidable(uint64)` and selector `[56, 59, 59, 232]`"]
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
        name = "isPendingStateConsolidable",
        abi = "isPendingStateConsolidable(uint64)"
    )]
    pub struct IsPendingStateConsolidableCall {
        pub pending_state_num: u64,
    }
    #[doc = "Container type for all input parameters for the `lastBatchSequenced` function with signature `lastBatchSequenced()` and selector `[66, 63, 168, 86]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastBatchSequenced", abi = "lastBatchSequenced()")]
    pub struct LastBatchSequencedCall;
    #[doc = "Container type for all input parameters for the `lastForceBatch` function with signature `lastForceBatch()` and selector `[231, 167, 237, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastForceBatch", abi = "lastForceBatch()")]
    pub struct LastForceBatchCall;
    #[doc = "Container type for all input parameters for the `lastForceBatchSequenced` function with signature `lastForceBatchSequenced()` and selector `[69, 96, 82, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastForceBatchSequenced", abi = "lastForceBatchSequenced()")]
    pub struct LastForceBatchSequencedCall;
    #[doc = "Container type for all input parameters for the `lastPendingState` function with signature `lastPendingState()` and selector `[69, 140, 4, 119]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastPendingState", abi = "lastPendingState()")]
    pub struct LastPendingStateCall;
    #[doc = "Container type for all input parameters for the `lastPendingStateConsolidated` function with signature `lastPendingStateConsolidated()` and selector `[74, 26, 137, 167]`"]
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
        name = "lastPendingStateConsolidated",
        abi = "lastPendingStateConsolidated()"
    )]
    pub struct LastPendingStateConsolidatedCall;
    #[doc = "Container type for all input parameters for the `lastTimestamp` function with signature `lastTimestamp()` and selector `[25, 216, 172, 97]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastTimestamp", abi = "lastTimestamp()")]
    pub struct LastTimestampCall;
    #[doc = "Container type for all input parameters for the `lastVerifiedBatch` function with signature `lastVerifiedBatch()` and selector `[127, 203, 54, 83]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastVerifiedBatch", abi = "lastVerifiedBatch()")]
    pub struct LastVerifiedBatchCall;
    #[doc = "Container type for all input parameters for the `matic` function with signature `matic()` and selector `[182, 176, 176, 151]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "matic", abi = "matic()")]
    pub struct MaticCall;
    #[doc = "Container type for all input parameters for the `multiplierBatchFee` function with signature `multiplierBatchFee()` and selector `[175, 210, 60, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "multiplierBatchFee", abi = "multiplierBatchFee()")]
    pub struct MultiplierBatchFeeCall;
    #[doc = "Container type for all input parameters for the `networkName` function with signature `networkName()` and selector `[16, 123, 242, 140]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "networkName", abi = "networkName()")]
    pub struct NetworkNameCall;
    #[doc = "Container type for all input parameters for the `overridePendingState` function with signature `overridePendingState(uint64,uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])` and selector `[225, 31, 63, 24]`"]
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
        name = "overridePendingState",
        abi = "overridePendingState(uint64,uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])"
    )]
    pub struct OverridePendingStateCall {
        pub init_pending_state_num: u64,
        pub final_pending_state_num: u64,
        pub init_num_batch: u64,
        pub final_new_batch: u64,
        pub new_local_exit_root: [u8; 32],
        pub new_state_root: [u8; 32],
        pub proof_a: [ethers::core::types::U256; 2usize],
        pub proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
        pub proof_c: [ethers::core::types::U256; 2usize],
    }
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
    #[doc = "Container type for all input parameters for the `pendingAdmin` function with signature `pendingAdmin()` and selector `[38, 120, 34, 71]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "pendingAdmin", abi = "pendingAdmin()")]
    pub struct PendingAdminCall;
    #[doc = "Container type for all input parameters for the `pendingStateTimeout` function with signature `pendingStateTimeout()` and selector `[217, 57, 179, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "pendingStateTimeout", abi = "pendingStateTimeout()")]
    pub struct PendingStateTimeoutCall;
    #[doc = "Container type for all input parameters for the `pendingStateTransitions` function with signature `pendingStateTransitions(uint256)` and selector `[131, 122, 71, 56]`"]
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
        name = "pendingStateTransitions",
        abi = "pendingStateTransitions(uint256)"
    )]
    pub struct PendingStateTransitionsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `proveNonDeterministicPendingState` function with signature `proveNonDeterministicPendingState(uint64,uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])` and selector `[117, 197, 8, 179]`"]
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
        name = "proveNonDeterministicPendingState",
        abi = "proveNonDeterministicPendingState(uint64,uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])"
    )]
    pub struct ProveNonDeterministicPendingStateCall {
        pub init_pending_state_num: u64,
        pub final_pending_state_num: u64,
        pub init_num_batch: u64,
        pub final_new_batch: u64,
        pub new_local_exit_root: [u8; 32],
        pub new_state_root: [u8; 32],
        pub proof_a: [ethers::core::types::U256; 2usize],
        pub proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
        pub proof_c: [ethers::core::types::U256; 2usize],
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
    #[doc = "Container type for all input parameters for the `rollupVerifier` function with signature `rollupVerifier()` and selector `[232, 191, 146, 237]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "rollupVerifier", abi = "rollupVerifier()")]
    pub struct RollupVerifierCall;
    #[doc = "Container type for all input parameters for the `sequenceBatches` function with signature `sequenceBatches((bytes,bytes32,uint64,uint64)[],address)` and selector `[94, 145, 69, 201]`"]
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
        name = "sequenceBatches",
        abi = "sequenceBatches((bytes,bytes32,uint64,uint64)[],address)"
    )]
    pub struct SequenceBatchesCall {
        pub batches: ::std::vec::Vec<BatchData>,
        pub fee_recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `sequenceForceBatches` function with signature `sequenceForceBatches((bytes,bytes32,uint64)[])` and selector `[216, 209, 9, 27]`"]
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
        name = "sequenceForceBatches",
        abi = "sequenceForceBatches((bytes,bytes32,uint64)[])"
    )]
    pub struct SequenceForceBatchesCall {
        pub batches: ::std::vec::Vec<ForcedBatchData>,
    }
    #[doc = "Container type for all input parameters for the `sequencedBatches` function with signature `sequencedBatches(uint64)` and selector `[180, 214, 63, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sequencedBatches", abi = "sequencedBatches(uint64)")]
    pub struct SequencedBatchesCall(pub u64);
    #[doc = "Container type for all input parameters for the `setMultiplierBatchFee` function with signature `setMultiplierBatchFee(uint16)` and selector `[24, 22, 183, 229]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMultiplierBatchFee", abi = "setMultiplierBatchFee(uint16)")]
    pub struct SetMultiplierBatchFeeCall {
        pub new_multiplier_batch_fee: u16,
    }
    #[doc = "Container type for all input parameters for the `setNetworkName` function with signature `setNetworkName(string)` and selector `[192, 202, 211, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setNetworkName", abi = "setNetworkName(string)")]
    pub struct SetNetworkNameCall {
        pub network_name: String,
    }
    #[doc = "Container type for all input parameters for the `setPendingStateTimeout` function with signature `setPendingStateTimeout(uint64)` and selector `[156, 159, 61, 254]`"]
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
        name = "setPendingStateTimeout",
        abi = "setPendingStateTimeout(uint64)"
    )]
    pub struct SetPendingStateTimeoutCall {
        pub new_pending_state_timeout: u64,
    }
    #[doc = "Container type for all input parameters for the `setSequencedBatch` function with signature `setSequencedBatch(uint64)` and selector `[155, 121, 103, 96]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setSequencedBatch", abi = "setSequencedBatch(uint64)")]
    pub struct SetSequencedBatchCall {
        pub num_batch: u64,
    }
    #[doc = "Container type for all input parameters for the `setSequencedBatches` function with signature `setSequencedBatches(uint64,bytes32,uint64,uint64)` and selector `[224, 209, 116, 65]`"]
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
        name = "setSequencedBatches",
        abi = "setSequencedBatches(uint64,bytes32,uint64,uint64)"
    )]
    pub struct SetSequencedBatchesCall {
        pub batch_num: u64,
        pub acc_input_data: [u8; 32],
        pub timestamp: u64,
        pub last_pending_state_consolidated: u64,
    }
    #[doc = "Container type for all input parameters for the `setStateRoot` function with signature `setStateRoot(bytes32,uint64)` and selector `[254, 22, 86, 79]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setStateRoot", abi = "setStateRoot(bytes32,uint64)")]
    pub struct SetStateRootCall {
        pub new_state_root: [u8; 32],
        pub batch_num: u64,
    }
    #[doc = "Container type for all input parameters for the `setTrustedAggregator` function with signature `setTrustedAggregator(address)` and selector `[241, 73, 22, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setTrustedAggregator", abi = "setTrustedAggregator(address)")]
    pub struct SetTrustedAggregatorCall {
        pub new_trusted_aggregator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setTrustedAggregatorTimeout` function with signature `setTrustedAggregatorTimeout(uint64)` and selector `[57, 66, 24, 233]`"]
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
        name = "setTrustedAggregatorTimeout",
        abi = "setTrustedAggregatorTimeout(uint64)"
    )]
    pub struct SetTrustedAggregatorTimeoutCall {
        pub new_trusted_aggregator_timeout: u64,
    }
    #[doc = "Container type for all input parameters for the `setTrustedSequencer` function with signature `setTrustedSequencer(address)` and selector `[111, 245, 18, 204]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setTrustedSequencer", abi = "setTrustedSequencer(address)")]
    pub struct SetTrustedSequencerCall {
        pub new_trusted_sequencer: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setTrustedSequencerURL` function with signature `setTrustedSequencerURL(string)` and selector `[200, 158, 66, 223]`"]
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
        name = "setTrustedSequencerURL",
        abi = "setTrustedSequencerURL(string)"
    )]
    pub struct SetTrustedSequencerURLCall {
        pub new_trusted_sequencer_url: String,
    }
    #[doc = "Container type for all input parameters for the `setVerifiedBatch` function with signature `setVerifiedBatch(uint64)` and selector `[150, 220, 61, 57]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setVerifiedBatch", abi = "setVerifiedBatch(uint64)")]
    pub struct SetVerifiedBatchCall {
        pub num_batch: u64,
    }
    #[doc = "Container type for all input parameters for the `setVerifyBatchTimeTarget` function with signature `setVerifyBatchTimeTarget(uint64)` and selector `[160, 102, 33, 92]`"]
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
        name = "setVerifyBatchTimeTarget",
        abi = "setVerifyBatchTimeTarget(uint64)"
    )]
    pub struct SetVerifyBatchTimeTargetCall {
        pub new_verify_batch_time_target: u64,
    }
    #[doc = "Container type for all input parameters for the `transferAdminRole` function with signature `transferAdminRole(address)` and selector `[173, 168, 249, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferAdminRole", abi = "transferAdminRole(address)")]
    pub struct TransferAdminRoleCall {
        pub new_pending_admin: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `trustedAggregator` function with signature `trustedAggregator()` and selector `[41, 135, 137, 131]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "trustedAggregator", abi = "trustedAggregator()")]
    pub struct TrustedAggregatorCall;
    #[doc = "Container type for all input parameters for the `trustedAggregatorTimeout` function with signature `trustedAggregatorTimeout()` and selector `[132, 27, 36, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "trustedAggregatorTimeout", abi = "trustedAggregatorTimeout()")]
    pub struct TrustedAggregatorTimeoutCall;
    #[doc = "Container type for all input parameters for the `trustedSequencer` function with signature `trustedSequencer()` and selector `[207, 168, 237, 71]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "trustedSequencer", abi = "trustedSequencer()")]
    pub struct TrustedSequencerCall;
    #[doc = "Container type for all input parameters for the `trustedSequencerURL` function with signature `trustedSequencerURL()` and selector `[84, 32, 40, 213]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "trustedSequencerURL", abi = "trustedSequencerURL()")]
    pub struct TrustedSequencerURLCall;
    #[doc = "Container type for all input parameters for the `trustedVerifyBatchesMock` function with signature `trustedVerifyBatchesMock(uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])` and selector `[16, 160, 26, 114]`"]
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
        name = "trustedVerifyBatchesMock",
        abi = "trustedVerifyBatchesMock(uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])"
    )]
    pub struct TrustedVerifyBatchesMockCall {
        pub pending_state_num: u64,
        pub init_num_batch: u64,
        pub final_new_batch: u64,
        pub new_local_exit_root: [u8; 32],
        pub new_state_root: [u8; 32],
        pub proof_a: [ethers::core::types::U256; 2usize],
        pub proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
        pub proof_c: [ethers::core::types::U256; 2usize],
    }
    #[doc = "Container type for all input parameters for the `updateBatchFee` function with signature `updateBatchFee(uint64)` and selector `[180, 247, 126, 169]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "updateBatchFee", abi = "updateBatchFee(uint64)")]
    pub struct UpdateBatchFeeCall {
        pub new_last_verified_batch: u64,
    }
    #[doc = "Container type for all input parameters for the `verifyBatchTimeTarget` function with signature `verifyBatchTimeTarget()` and selector `[10, 13, 159, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "verifyBatchTimeTarget", abi = "verifyBatchTimeTarget()")]
    pub struct VerifyBatchTimeTargetCall;
    #[doc = "Container type for all input parameters for the `verifyBatches` function with signature `verifyBatches(uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])` and selector `[72, 52, 163, 67]`"]
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
        name = "verifyBatches",
        abi = "verifyBatches(uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])"
    )]
    pub struct VerifyBatchesCall {
        pub pending_state_num: u64,
        pub init_num_batch: u64,
        pub final_new_batch: u64,
        pub new_local_exit_root: [u8; 32],
        pub new_state_root: [u8; 32],
        pub proof_a: [ethers::core::types::U256; 2usize],
        pub proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
        pub proof_c: [ethers::core::types::U256; 2usize],
    }
    #[doc = "Container type for all input parameters for the `verifyBatchesTrustedAggregator` function with signature `verifyBatchesTrustedAggregator(uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])` and selector `[240, 32, 201, 62]`"]
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
        name = "verifyBatchesTrustedAggregator",
        abi = "verifyBatchesTrustedAggregator(uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])"
    )]
    pub struct VerifyBatchesTrustedAggregatorCall {
        pub pending_state_num: u64,
        pub init_num_batch: u64,
        pub final_new_batch: u64,
        pub new_local_exit_root: [u8; 32],
        pub new_state_root: [u8; 32],
        pub proof_a: [ethers::core::types::U256; 2usize],
        pub proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
        pub proof_c: [ethers::core::types::U256; 2usize],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PolygonZkEVMMockCalls {
        AcceptAdminRole(AcceptAdminRoleCall),
        ActivateEmergencyState(ActivateEmergencyStateCall),
        Admin(AdminCall),
        BatchFee(BatchFeeCall),
        BatchNumToStateRoot(BatchNumToStateRootCall),
        BridgeAddress(BridgeAddressCall),
        CalculateAccInputHash(CalculateAccInputHashCall),
        CalculateRewardPerBatch(CalculateRewardPerBatchCall),
        ChainID(ChainIDCall),
        ConsolidatePendingState(ConsolidatePendingStateCall),
        DeactivateEmergencyState(DeactivateEmergencyStateCall),
        ForceBatch(ForceBatchCall),
        ForcedBatches(ForcedBatchesCall),
        ForkID(ForkIDCall),
        GetCurrentBatchFee(GetCurrentBatchFeeCall),
        GetInputSnarkBytes(GetInputSnarkBytesCall),
        GetLastVerifiedBatch(GetLastVerifiedBatchCall),
        GetNextSnarkInput(GetNextSnarkInputCall),
        GlobalExitRootManager(GlobalExitRootManagerCall),
        Initialize(InitializeCall),
        IsEmergencyState(IsEmergencyStateCall),
        IsPendingStateConsolidable(IsPendingStateConsolidableCall),
        LastBatchSequenced(LastBatchSequencedCall),
        LastForceBatch(LastForceBatchCall),
        LastForceBatchSequenced(LastForceBatchSequencedCall),
        LastPendingState(LastPendingStateCall),
        LastPendingStateConsolidated(LastPendingStateConsolidatedCall),
        LastTimestamp(LastTimestampCall),
        LastVerifiedBatch(LastVerifiedBatchCall),
        Matic(MaticCall),
        MultiplierBatchFee(MultiplierBatchFeeCall),
        NetworkName(NetworkNameCall),
        OverridePendingState(OverridePendingStateCall),
        Owner(OwnerCall),
        PendingAdmin(PendingAdminCall),
        PendingStateTimeout(PendingStateTimeoutCall),
        PendingStateTransitions(PendingStateTransitionsCall),
        ProveNonDeterministicPendingState(ProveNonDeterministicPendingStateCall),
        RenounceOwnership(RenounceOwnershipCall),
        RollupVerifier(RollupVerifierCall),
        SequenceBatches(SequenceBatchesCall),
        SequenceForceBatches(SequenceForceBatchesCall),
        SequencedBatches(SequencedBatchesCall),
        SetMultiplierBatchFee(SetMultiplierBatchFeeCall),
        SetNetworkName(SetNetworkNameCall),
        SetPendingStateTimeout(SetPendingStateTimeoutCall),
        SetSequencedBatch(SetSequencedBatchCall),
        SetSequencedBatches(SetSequencedBatchesCall),
        SetStateRoot(SetStateRootCall),
        SetTrustedAggregator(SetTrustedAggregatorCall),
        SetTrustedAggregatorTimeout(SetTrustedAggregatorTimeoutCall),
        SetTrustedSequencer(SetTrustedSequencerCall),
        SetTrustedSequencerURL(SetTrustedSequencerURLCall),
        SetVerifiedBatch(SetVerifiedBatchCall),
        SetVerifyBatchTimeTarget(SetVerifyBatchTimeTargetCall),
        TransferAdminRole(TransferAdminRoleCall),
        TransferOwnership(TransferOwnershipCall),
        TrustedAggregator(TrustedAggregatorCall),
        TrustedAggregatorTimeout(TrustedAggregatorTimeoutCall),
        TrustedSequencer(TrustedSequencerCall),
        TrustedSequencerURL(TrustedSequencerURLCall),
        TrustedVerifyBatchesMock(TrustedVerifyBatchesMockCall),
        UpdateBatchFee(UpdateBatchFeeCall),
        VerifyBatchTimeTarget(VerifyBatchTimeTargetCall),
        VerifyBatches(VerifyBatchesCall),
        VerifyBatchesTrustedAggregator(VerifyBatchesTrustedAggregatorCall),
    }
    impl ethers::core::abi::AbiDecode for PolygonZkEVMMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::AcceptAdminRole(decoded));
            }
            if let Ok(decoded) =
                <ActivateEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::ActivateEmergencyState(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <BatchFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::BatchFee(decoded));
            }
            if let Ok(decoded) =
                <BatchNumToStateRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::BatchNumToStateRoot(decoded));
            }
            if let Ok(decoded) =
                <BridgeAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::BridgeAddress(decoded));
            }
            if let Ok(decoded) =
                <CalculateAccInputHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::CalculateAccInputHash(decoded));
            }
            if let Ok(decoded) =
                <CalculateRewardPerBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::CalculateRewardPerBatch(decoded));
            }
            if let Ok(decoded) =
                <ChainIDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::ChainID(decoded));
            }
            if let Ok(decoded) =
                <ConsolidatePendingStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::ConsolidatePendingState(decoded));
            }
            if let Ok(decoded) =
                <DeactivateEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockCalls::DeactivateEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <ForceBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::ForceBatch(decoded));
            }
            if let Ok(decoded) =
                <ForcedBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::ForcedBatches(decoded));
            }
            if let Ok(decoded) = <ForkIDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::ForkID(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentBatchFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::GetCurrentBatchFee(decoded));
            }
            if let Ok(decoded) =
                <GetInputSnarkBytesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::GetInputSnarkBytes(decoded));
            }
            if let Ok(decoded) =
                <GetLastVerifiedBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::GetLastVerifiedBatch(decoded));
            }
            if let Ok(decoded) =
                <GetNextSnarkInputCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::GetNextSnarkInput(decoded));
            }
            if let Ok(decoded) =
                <GlobalExitRootManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::GlobalExitRootManager(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::IsEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <IsPendingStateConsolidableCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockCalls::IsPendingStateConsolidable(decoded));
            }
            if let Ok(decoded) =
                <LastBatchSequencedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::LastBatchSequenced(decoded));
            }
            if let Ok(decoded) =
                <LastForceBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::LastForceBatch(decoded));
            }
            if let Ok(decoded) =
                <LastForceBatchSequencedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::LastForceBatchSequenced(decoded));
            }
            if let Ok(decoded) =
                <LastPendingStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::LastPendingState(decoded));
            }
            if let Ok(decoded) =
                <LastPendingStateConsolidatedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockCalls::LastPendingStateConsolidated(decoded));
            }
            if let Ok(decoded) =
                <LastTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::LastTimestamp(decoded));
            }
            if let Ok(decoded) =
                <LastVerifiedBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::LastVerifiedBatch(decoded));
            }
            if let Ok(decoded) = <MaticCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::Matic(decoded));
            }
            if let Ok(decoded) =
                <MultiplierBatchFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::MultiplierBatchFee(decoded));
            }
            if let Ok(decoded) =
                <NetworkNameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::NetworkName(decoded));
            }
            if let Ok(decoded) =
                <OverridePendingStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::OverridePendingState(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <PendingStateTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::PendingStateTimeout(decoded));
            }
            if let Ok(decoded) =
                <PendingStateTransitionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::PendingStateTransitions(decoded));
            }
            if let Ok(decoded) =
                <ProveNonDeterministicPendingStateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockCalls::ProveNonDeterministicPendingState(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RollupVerifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::RollupVerifier(decoded));
            }
            if let Ok(decoded) =
                <SequenceBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::SequenceBatches(decoded));
            }
            if let Ok(decoded) =
                <SequenceForceBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::SequenceForceBatches(decoded));
            }
            if let Ok(decoded) =
                <SequencedBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::SequencedBatches(decoded));
            }
            if let Ok(decoded) =
                <SetMultiplierBatchFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::SetMultiplierBatchFee(decoded));
            }
            if let Ok(decoded) =
                <SetNetworkNameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::SetNetworkName(decoded));
            }
            if let Ok(decoded) =
                <SetPendingStateTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::SetPendingStateTimeout(decoded));
            }
            if let Ok(decoded) =
                <SetSequencedBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::SetSequencedBatch(decoded));
            }
            if let Ok(decoded) =
                <SetSequencedBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::SetSequencedBatches(decoded));
            }
            if let Ok(decoded) =
                <SetStateRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::SetStateRoot(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedAggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::SetTrustedAggregator(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedAggregatorTimeoutCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockCalls::SetTrustedAggregatorTimeout(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedSequencerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::SetTrustedSequencer(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedSequencerURLCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::SetTrustedSequencerURL(decoded));
            }
            if let Ok(decoded) =
                <SetVerifiedBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::SetVerifiedBatch(decoded));
            }
            if let Ok(decoded) =
                <SetVerifyBatchTimeTargetCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockCalls::SetVerifyBatchTimeTarget(decoded));
            }
            if let Ok(decoded) =
                <TransferAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::TransferAdminRole(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TrustedAggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::TrustedAggregator(decoded));
            }
            if let Ok(decoded) =
                <TrustedAggregatorTimeoutCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockCalls::TrustedAggregatorTimeout(decoded));
            }
            if let Ok(decoded) =
                <TrustedSequencerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::TrustedSequencer(decoded));
            }
            if let Ok(decoded) =
                <TrustedSequencerURLCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::TrustedSequencerURL(decoded));
            }
            if let Ok(decoded) =
                <TrustedVerifyBatchesMockCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockCalls::TrustedVerifyBatchesMock(decoded));
            }
            if let Ok(decoded) =
                <UpdateBatchFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::UpdateBatchFee(decoded));
            }
            if let Ok(decoded) =
                <VerifyBatchTimeTargetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::VerifyBatchTimeTarget(decoded));
            }
            if let Ok(decoded) =
                <VerifyBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMMockCalls::VerifyBatches(decoded));
            }
            if let Ok(decoded) =
                <VerifyBatchesTrustedAggregatorCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMMockCalls::VerifyBatchesTrustedAggregator(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PolygonZkEVMMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PolygonZkEVMMockCalls::AcceptAdminRole(element) => element.encode(),
                PolygonZkEVMMockCalls::ActivateEmergencyState(element) => element.encode(),
                PolygonZkEVMMockCalls::Admin(element) => element.encode(),
                PolygonZkEVMMockCalls::BatchFee(element) => element.encode(),
                PolygonZkEVMMockCalls::BatchNumToStateRoot(element) => element.encode(),
                PolygonZkEVMMockCalls::BridgeAddress(element) => element.encode(),
                PolygonZkEVMMockCalls::CalculateAccInputHash(element) => element.encode(),
                PolygonZkEVMMockCalls::CalculateRewardPerBatch(element) => element.encode(),
                PolygonZkEVMMockCalls::ChainID(element) => element.encode(),
                PolygonZkEVMMockCalls::ConsolidatePendingState(element) => element.encode(),
                PolygonZkEVMMockCalls::DeactivateEmergencyState(element) => element.encode(),
                PolygonZkEVMMockCalls::ForceBatch(element) => element.encode(),
                PolygonZkEVMMockCalls::ForcedBatches(element) => element.encode(),
                PolygonZkEVMMockCalls::ForkID(element) => element.encode(),
                PolygonZkEVMMockCalls::GetCurrentBatchFee(element) => element.encode(),
                PolygonZkEVMMockCalls::GetInputSnarkBytes(element) => element.encode(),
                PolygonZkEVMMockCalls::GetLastVerifiedBatch(element) => element.encode(),
                PolygonZkEVMMockCalls::GetNextSnarkInput(element) => element.encode(),
                PolygonZkEVMMockCalls::GlobalExitRootManager(element) => element.encode(),
                PolygonZkEVMMockCalls::Initialize(element) => element.encode(),
                PolygonZkEVMMockCalls::IsEmergencyState(element) => element.encode(),
                PolygonZkEVMMockCalls::IsPendingStateConsolidable(element) => element.encode(),
                PolygonZkEVMMockCalls::LastBatchSequenced(element) => element.encode(),
                PolygonZkEVMMockCalls::LastForceBatch(element) => element.encode(),
                PolygonZkEVMMockCalls::LastForceBatchSequenced(element) => element.encode(),
                PolygonZkEVMMockCalls::LastPendingState(element) => element.encode(),
                PolygonZkEVMMockCalls::LastPendingStateConsolidated(element) => element.encode(),
                PolygonZkEVMMockCalls::LastTimestamp(element) => element.encode(),
                PolygonZkEVMMockCalls::LastVerifiedBatch(element) => element.encode(),
                PolygonZkEVMMockCalls::Matic(element) => element.encode(),
                PolygonZkEVMMockCalls::MultiplierBatchFee(element) => element.encode(),
                PolygonZkEVMMockCalls::NetworkName(element) => element.encode(),
                PolygonZkEVMMockCalls::OverridePendingState(element) => element.encode(),
                PolygonZkEVMMockCalls::Owner(element) => element.encode(),
                PolygonZkEVMMockCalls::PendingAdmin(element) => element.encode(),
                PolygonZkEVMMockCalls::PendingStateTimeout(element) => element.encode(),
                PolygonZkEVMMockCalls::PendingStateTransitions(element) => element.encode(),
                PolygonZkEVMMockCalls::ProveNonDeterministicPendingState(element) => {
                    element.encode()
                }
                PolygonZkEVMMockCalls::RenounceOwnership(element) => element.encode(),
                PolygonZkEVMMockCalls::RollupVerifier(element) => element.encode(),
                PolygonZkEVMMockCalls::SequenceBatches(element) => element.encode(),
                PolygonZkEVMMockCalls::SequenceForceBatches(element) => element.encode(),
                PolygonZkEVMMockCalls::SequencedBatches(element) => element.encode(),
                PolygonZkEVMMockCalls::SetMultiplierBatchFee(element) => element.encode(),
                PolygonZkEVMMockCalls::SetNetworkName(element) => element.encode(),
                PolygonZkEVMMockCalls::SetPendingStateTimeout(element) => element.encode(),
                PolygonZkEVMMockCalls::SetSequencedBatch(element) => element.encode(),
                PolygonZkEVMMockCalls::SetSequencedBatches(element) => element.encode(),
                PolygonZkEVMMockCalls::SetStateRoot(element) => element.encode(),
                PolygonZkEVMMockCalls::SetTrustedAggregator(element) => element.encode(),
                PolygonZkEVMMockCalls::SetTrustedAggregatorTimeout(element) => element.encode(),
                PolygonZkEVMMockCalls::SetTrustedSequencer(element) => element.encode(),
                PolygonZkEVMMockCalls::SetTrustedSequencerURL(element) => element.encode(),
                PolygonZkEVMMockCalls::SetVerifiedBatch(element) => element.encode(),
                PolygonZkEVMMockCalls::SetVerifyBatchTimeTarget(element) => element.encode(),
                PolygonZkEVMMockCalls::TransferAdminRole(element) => element.encode(),
                PolygonZkEVMMockCalls::TransferOwnership(element) => element.encode(),
                PolygonZkEVMMockCalls::TrustedAggregator(element) => element.encode(),
                PolygonZkEVMMockCalls::TrustedAggregatorTimeout(element) => element.encode(),
                PolygonZkEVMMockCalls::TrustedSequencer(element) => element.encode(),
                PolygonZkEVMMockCalls::TrustedSequencerURL(element) => element.encode(),
                PolygonZkEVMMockCalls::TrustedVerifyBatchesMock(element) => element.encode(),
                PolygonZkEVMMockCalls::UpdateBatchFee(element) => element.encode(),
                PolygonZkEVMMockCalls::VerifyBatchTimeTarget(element) => element.encode(),
                PolygonZkEVMMockCalls::VerifyBatches(element) => element.encode(),
                PolygonZkEVMMockCalls::VerifyBatchesTrustedAggregator(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMMockCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMMockCalls::AcceptAdminRole(element) => element.fmt(f),
                PolygonZkEVMMockCalls::ActivateEmergencyState(element) => element.fmt(f),
                PolygonZkEVMMockCalls::Admin(element) => element.fmt(f),
                PolygonZkEVMMockCalls::BatchFee(element) => element.fmt(f),
                PolygonZkEVMMockCalls::BatchNumToStateRoot(element) => element.fmt(f),
                PolygonZkEVMMockCalls::BridgeAddress(element) => element.fmt(f),
                PolygonZkEVMMockCalls::CalculateAccInputHash(element) => element.fmt(f),
                PolygonZkEVMMockCalls::CalculateRewardPerBatch(element) => element.fmt(f),
                PolygonZkEVMMockCalls::ChainID(element) => element.fmt(f),
                PolygonZkEVMMockCalls::ConsolidatePendingState(element) => element.fmt(f),
                PolygonZkEVMMockCalls::DeactivateEmergencyState(element) => element.fmt(f),
                PolygonZkEVMMockCalls::ForceBatch(element) => element.fmt(f),
                PolygonZkEVMMockCalls::ForcedBatches(element) => element.fmt(f),
                PolygonZkEVMMockCalls::ForkID(element) => element.fmt(f),
                PolygonZkEVMMockCalls::GetCurrentBatchFee(element) => element.fmt(f),
                PolygonZkEVMMockCalls::GetInputSnarkBytes(element) => element.fmt(f),
                PolygonZkEVMMockCalls::GetLastVerifiedBatch(element) => element.fmt(f),
                PolygonZkEVMMockCalls::GetNextSnarkInput(element) => element.fmt(f),
                PolygonZkEVMMockCalls::GlobalExitRootManager(element) => element.fmt(f),
                PolygonZkEVMMockCalls::Initialize(element) => element.fmt(f),
                PolygonZkEVMMockCalls::IsEmergencyState(element) => element.fmt(f),
                PolygonZkEVMMockCalls::IsPendingStateConsolidable(element) => element.fmt(f),
                PolygonZkEVMMockCalls::LastBatchSequenced(element) => element.fmt(f),
                PolygonZkEVMMockCalls::LastForceBatch(element) => element.fmt(f),
                PolygonZkEVMMockCalls::LastForceBatchSequenced(element) => element.fmt(f),
                PolygonZkEVMMockCalls::LastPendingState(element) => element.fmt(f),
                PolygonZkEVMMockCalls::LastPendingStateConsolidated(element) => element.fmt(f),
                PolygonZkEVMMockCalls::LastTimestamp(element) => element.fmt(f),
                PolygonZkEVMMockCalls::LastVerifiedBatch(element) => element.fmt(f),
                PolygonZkEVMMockCalls::Matic(element) => element.fmt(f),
                PolygonZkEVMMockCalls::MultiplierBatchFee(element) => element.fmt(f),
                PolygonZkEVMMockCalls::NetworkName(element) => element.fmt(f),
                PolygonZkEVMMockCalls::OverridePendingState(element) => element.fmt(f),
                PolygonZkEVMMockCalls::Owner(element) => element.fmt(f),
                PolygonZkEVMMockCalls::PendingAdmin(element) => element.fmt(f),
                PolygonZkEVMMockCalls::PendingStateTimeout(element) => element.fmt(f),
                PolygonZkEVMMockCalls::PendingStateTransitions(element) => element.fmt(f),
                PolygonZkEVMMockCalls::ProveNonDeterministicPendingState(element) => element.fmt(f),
                PolygonZkEVMMockCalls::RenounceOwnership(element) => element.fmt(f),
                PolygonZkEVMMockCalls::RollupVerifier(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SequenceBatches(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SequenceForceBatches(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SequencedBatches(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SetMultiplierBatchFee(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SetNetworkName(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SetPendingStateTimeout(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SetSequencedBatch(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SetSequencedBatches(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SetStateRoot(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SetTrustedAggregator(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SetTrustedAggregatorTimeout(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SetTrustedSequencer(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SetTrustedSequencerURL(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SetVerifiedBatch(element) => element.fmt(f),
                PolygonZkEVMMockCalls::SetVerifyBatchTimeTarget(element) => element.fmt(f),
                PolygonZkEVMMockCalls::TransferAdminRole(element) => element.fmt(f),
                PolygonZkEVMMockCalls::TransferOwnership(element) => element.fmt(f),
                PolygonZkEVMMockCalls::TrustedAggregator(element) => element.fmt(f),
                PolygonZkEVMMockCalls::TrustedAggregatorTimeout(element) => element.fmt(f),
                PolygonZkEVMMockCalls::TrustedSequencer(element) => element.fmt(f),
                PolygonZkEVMMockCalls::TrustedSequencerURL(element) => element.fmt(f),
                PolygonZkEVMMockCalls::TrustedVerifyBatchesMock(element) => element.fmt(f),
                PolygonZkEVMMockCalls::UpdateBatchFee(element) => element.fmt(f),
                PolygonZkEVMMockCalls::VerifyBatchTimeTarget(element) => element.fmt(f),
                PolygonZkEVMMockCalls::VerifyBatches(element) => element.fmt(f),
                PolygonZkEVMMockCalls::VerifyBatchesTrustedAggregator(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptAdminRoleCall> for PolygonZkEVMMockCalls {
        fn from(var: AcceptAdminRoleCall) -> Self {
            PolygonZkEVMMockCalls::AcceptAdminRole(var)
        }
    }
    impl ::std::convert::From<ActivateEmergencyStateCall> for PolygonZkEVMMockCalls {
        fn from(var: ActivateEmergencyStateCall) -> Self {
            PolygonZkEVMMockCalls::ActivateEmergencyState(var)
        }
    }
    impl ::std::convert::From<AdminCall> for PolygonZkEVMMockCalls {
        fn from(var: AdminCall) -> Self {
            PolygonZkEVMMockCalls::Admin(var)
        }
    }
    impl ::std::convert::From<BatchFeeCall> for PolygonZkEVMMockCalls {
        fn from(var: BatchFeeCall) -> Self {
            PolygonZkEVMMockCalls::BatchFee(var)
        }
    }
    impl ::std::convert::From<BatchNumToStateRootCall> for PolygonZkEVMMockCalls {
        fn from(var: BatchNumToStateRootCall) -> Self {
            PolygonZkEVMMockCalls::BatchNumToStateRoot(var)
        }
    }
    impl ::std::convert::From<BridgeAddressCall> for PolygonZkEVMMockCalls {
        fn from(var: BridgeAddressCall) -> Self {
            PolygonZkEVMMockCalls::BridgeAddress(var)
        }
    }
    impl ::std::convert::From<CalculateAccInputHashCall> for PolygonZkEVMMockCalls {
        fn from(var: CalculateAccInputHashCall) -> Self {
            PolygonZkEVMMockCalls::CalculateAccInputHash(var)
        }
    }
    impl ::std::convert::From<CalculateRewardPerBatchCall> for PolygonZkEVMMockCalls {
        fn from(var: CalculateRewardPerBatchCall) -> Self {
            PolygonZkEVMMockCalls::CalculateRewardPerBatch(var)
        }
    }
    impl ::std::convert::From<ChainIDCall> for PolygonZkEVMMockCalls {
        fn from(var: ChainIDCall) -> Self {
            PolygonZkEVMMockCalls::ChainID(var)
        }
    }
    impl ::std::convert::From<ConsolidatePendingStateCall> for PolygonZkEVMMockCalls {
        fn from(var: ConsolidatePendingStateCall) -> Self {
            PolygonZkEVMMockCalls::ConsolidatePendingState(var)
        }
    }
    impl ::std::convert::From<DeactivateEmergencyStateCall> for PolygonZkEVMMockCalls {
        fn from(var: DeactivateEmergencyStateCall) -> Self {
            PolygonZkEVMMockCalls::DeactivateEmergencyState(var)
        }
    }
    impl ::std::convert::From<ForceBatchCall> for PolygonZkEVMMockCalls {
        fn from(var: ForceBatchCall) -> Self {
            PolygonZkEVMMockCalls::ForceBatch(var)
        }
    }
    impl ::std::convert::From<ForcedBatchesCall> for PolygonZkEVMMockCalls {
        fn from(var: ForcedBatchesCall) -> Self {
            PolygonZkEVMMockCalls::ForcedBatches(var)
        }
    }
    impl ::std::convert::From<ForkIDCall> for PolygonZkEVMMockCalls {
        fn from(var: ForkIDCall) -> Self {
            PolygonZkEVMMockCalls::ForkID(var)
        }
    }
    impl ::std::convert::From<GetCurrentBatchFeeCall> for PolygonZkEVMMockCalls {
        fn from(var: GetCurrentBatchFeeCall) -> Self {
            PolygonZkEVMMockCalls::GetCurrentBatchFee(var)
        }
    }
    impl ::std::convert::From<GetInputSnarkBytesCall> for PolygonZkEVMMockCalls {
        fn from(var: GetInputSnarkBytesCall) -> Self {
            PolygonZkEVMMockCalls::GetInputSnarkBytes(var)
        }
    }
    impl ::std::convert::From<GetLastVerifiedBatchCall> for PolygonZkEVMMockCalls {
        fn from(var: GetLastVerifiedBatchCall) -> Self {
            PolygonZkEVMMockCalls::GetLastVerifiedBatch(var)
        }
    }
    impl ::std::convert::From<GetNextSnarkInputCall> for PolygonZkEVMMockCalls {
        fn from(var: GetNextSnarkInputCall) -> Self {
            PolygonZkEVMMockCalls::GetNextSnarkInput(var)
        }
    }
    impl ::std::convert::From<GlobalExitRootManagerCall> for PolygonZkEVMMockCalls {
        fn from(var: GlobalExitRootManagerCall) -> Self {
            PolygonZkEVMMockCalls::GlobalExitRootManager(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for PolygonZkEVMMockCalls {
        fn from(var: InitializeCall) -> Self {
            PolygonZkEVMMockCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<IsEmergencyStateCall> for PolygonZkEVMMockCalls {
        fn from(var: IsEmergencyStateCall) -> Self {
            PolygonZkEVMMockCalls::IsEmergencyState(var)
        }
    }
    impl ::std::convert::From<IsPendingStateConsolidableCall> for PolygonZkEVMMockCalls {
        fn from(var: IsPendingStateConsolidableCall) -> Self {
            PolygonZkEVMMockCalls::IsPendingStateConsolidable(var)
        }
    }
    impl ::std::convert::From<LastBatchSequencedCall> for PolygonZkEVMMockCalls {
        fn from(var: LastBatchSequencedCall) -> Self {
            PolygonZkEVMMockCalls::LastBatchSequenced(var)
        }
    }
    impl ::std::convert::From<LastForceBatchCall> for PolygonZkEVMMockCalls {
        fn from(var: LastForceBatchCall) -> Self {
            PolygonZkEVMMockCalls::LastForceBatch(var)
        }
    }
    impl ::std::convert::From<LastForceBatchSequencedCall> for PolygonZkEVMMockCalls {
        fn from(var: LastForceBatchSequencedCall) -> Self {
            PolygonZkEVMMockCalls::LastForceBatchSequenced(var)
        }
    }
    impl ::std::convert::From<LastPendingStateCall> for PolygonZkEVMMockCalls {
        fn from(var: LastPendingStateCall) -> Self {
            PolygonZkEVMMockCalls::LastPendingState(var)
        }
    }
    impl ::std::convert::From<LastPendingStateConsolidatedCall> for PolygonZkEVMMockCalls {
        fn from(var: LastPendingStateConsolidatedCall) -> Self {
            PolygonZkEVMMockCalls::LastPendingStateConsolidated(var)
        }
    }
    impl ::std::convert::From<LastTimestampCall> for PolygonZkEVMMockCalls {
        fn from(var: LastTimestampCall) -> Self {
            PolygonZkEVMMockCalls::LastTimestamp(var)
        }
    }
    impl ::std::convert::From<LastVerifiedBatchCall> for PolygonZkEVMMockCalls {
        fn from(var: LastVerifiedBatchCall) -> Self {
            PolygonZkEVMMockCalls::LastVerifiedBatch(var)
        }
    }
    impl ::std::convert::From<MaticCall> for PolygonZkEVMMockCalls {
        fn from(var: MaticCall) -> Self {
            PolygonZkEVMMockCalls::Matic(var)
        }
    }
    impl ::std::convert::From<MultiplierBatchFeeCall> for PolygonZkEVMMockCalls {
        fn from(var: MultiplierBatchFeeCall) -> Self {
            PolygonZkEVMMockCalls::MultiplierBatchFee(var)
        }
    }
    impl ::std::convert::From<NetworkNameCall> for PolygonZkEVMMockCalls {
        fn from(var: NetworkNameCall) -> Self {
            PolygonZkEVMMockCalls::NetworkName(var)
        }
    }
    impl ::std::convert::From<OverridePendingStateCall> for PolygonZkEVMMockCalls {
        fn from(var: OverridePendingStateCall) -> Self {
            PolygonZkEVMMockCalls::OverridePendingState(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for PolygonZkEVMMockCalls {
        fn from(var: OwnerCall) -> Self {
            PolygonZkEVMMockCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for PolygonZkEVMMockCalls {
        fn from(var: PendingAdminCall) -> Self {
            PolygonZkEVMMockCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<PendingStateTimeoutCall> for PolygonZkEVMMockCalls {
        fn from(var: PendingStateTimeoutCall) -> Self {
            PolygonZkEVMMockCalls::PendingStateTimeout(var)
        }
    }
    impl ::std::convert::From<PendingStateTransitionsCall> for PolygonZkEVMMockCalls {
        fn from(var: PendingStateTransitionsCall) -> Self {
            PolygonZkEVMMockCalls::PendingStateTransitions(var)
        }
    }
    impl ::std::convert::From<ProveNonDeterministicPendingStateCall> for PolygonZkEVMMockCalls {
        fn from(var: ProveNonDeterministicPendingStateCall) -> Self {
            PolygonZkEVMMockCalls::ProveNonDeterministicPendingState(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for PolygonZkEVMMockCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            PolygonZkEVMMockCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RollupVerifierCall> for PolygonZkEVMMockCalls {
        fn from(var: RollupVerifierCall) -> Self {
            PolygonZkEVMMockCalls::RollupVerifier(var)
        }
    }
    impl ::std::convert::From<SequenceBatchesCall> for PolygonZkEVMMockCalls {
        fn from(var: SequenceBatchesCall) -> Self {
            PolygonZkEVMMockCalls::SequenceBatches(var)
        }
    }
    impl ::std::convert::From<SequenceForceBatchesCall> for PolygonZkEVMMockCalls {
        fn from(var: SequenceForceBatchesCall) -> Self {
            PolygonZkEVMMockCalls::SequenceForceBatches(var)
        }
    }
    impl ::std::convert::From<SequencedBatchesCall> for PolygonZkEVMMockCalls {
        fn from(var: SequencedBatchesCall) -> Self {
            PolygonZkEVMMockCalls::SequencedBatches(var)
        }
    }
    impl ::std::convert::From<SetMultiplierBatchFeeCall> for PolygonZkEVMMockCalls {
        fn from(var: SetMultiplierBatchFeeCall) -> Self {
            PolygonZkEVMMockCalls::SetMultiplierBatchFee(var)
        }
    }
    impl ::std::convert::From<SetNetworkNameCall> for PolygonZkEVMMockCalls {
        fn from(var: SetNetworkNameCall) -> Self {
            PolygonZkEVMMockCalls::SetNetworkName(var)
        }
    }
    impl ::std::convert::From<SetPendingStateTimeoutCall> for PolygonZkEVMMockCalls {
        fn from(var: SetPendingStateTimeoutCall) -> Self {
            PolygonZkEVMMockCalls::SetPendingStateTimeout(var)
        }
    }
    impl ::std::convert::From<SetSequencedBatchCall> for PolygonZkEVMMockCalls {
        fn from(var: SetSequencedBatchCall) -> Self {
            PolygonZkEVMMockCalls::SetSequencedBatch(var)
        }
    }
    impl ::std::convert::From<SetSequencedBatchesCall> for PolygonZkEVMMockCalls {
        fn from(var: SetSequencedBatchesCall) -> Self {
            PolygonZkEVMMockCalls::SetSequencedBatches(var)
        }
    }
    impl ::std::convert::From<SetStateRootCall> for PolygonZkEVMMockCalls {
        fn from(var: SetStateRootCall) -> Self {
            PolygonZkEVMMockCalls::SetStateRoot(var)
        }
    }
    impl ::std::convert::From<SetTrustedAggregatorCall> for PolygonZkEVMMockCalls {
        fn from(var: SetTrustedAggregatorCall) -> Self {
            PolygonZkEVMMockCalls::SetTrustedAggregator(var)
        }
    }
    impl ::std::convert::From<SetTrustedAggregatorTimeoutCall> for PolygonZkEVMMockCalls {
        fn from(var: SetTrustedAggregatorTimeoutCall) -> Self {
            PolygonZkEVMMockCalls::SetTrustedAggregatorTimeout(var)
        }
    }
    impl ::std::convert::From<SetTrustedSequencerCall> for PolygonZkEVMMockCalls {
        fn from(var: SetTrustedSequencerCall) -> Self {
            PolygonZkEVMMockCalls::SetTrustedSequencer(var)
        }
    }
    impl ::std::convert::From<SetTrustedSequencerURLCall> for PolygonZkEVMMockCalls {
        fn from(var: SetTrustedSequencerURLCall) -> Self {
            PolygonZkEVMMockCalls::SetTrustedSequencerURL(var)
        }
    }
    impl ::std::convert::From<SetVerifiedBatchCall> for PolygonZkEVMMockCalls {
        fn from(var: SetVerifiedBatchCall) -> Self {
            PolygonZkEVMMockCalls::SetVerifiedBatch(var)
        }
    }
    impl ::std::convert::From<SetVerifyBatchTimeTargetCall> for PolygonZkEVMMockCalls {
        fn from(var: SetVerifyBatchTimeTargetCall) -> Self {
            PolygonZkEVMMockCalls::SetVerifyBatchTimeTarget(var)
        }
    }
    impl ::std::convert::From<TransferAdminRoleCall> for PolygonZkEVMMockCalls {
        fn from(var: TransferAdminRoleCall) -> Self {
            PolygonZkEVMMockCalls::TransferAdminRole(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for PolygonZkEVMMockCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            PolygonZkEVMMockCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<TrustedAggregatorCall> for PolygonZkEVMMockCalls {
        fn from(var: TrustedAggregatorCall) -> Self {
            PolygonZkEVMMockCalls::TrustedAggregator(var)
        }
    }
    impl ::std::convert::From<TrustedAggregatorTimeoutCall> for PolygonZkEVMMockCalls {
        fn from(var: TrustedAggregatorTimeoutCall) -> Self {
            PolygonZkEVMMockCalls::TrustedAggregatorTimeout(var)
        }
    }
    impl ::std::convert::From<TrustedSequencerCall> for PolygonZkEVMMockCalls {
        fn from(var: TrustedSequencerCall) -> Self {
            PolygonZkEVMMockCalls::TrustedSequencer(var)
        }
    }
    impl ::std::convert::From<TrustedSequencerURLCall> for PolygonZkEVMMockCalls {
        fn from(var: TrustedSequencerURLCall) -> Self {
            PolygonZkEVMMockCalls::TrustedSequencerURL(var)
        }
    }
    impl ::std::convert::From<TrustedVerifyBatchesMockCall> for PolygonZkEVMMockCalls {
        fn from(var: TrustedVerifyBatchesMockCall) -> Self {
            PolygonZkEVMMockCalls::TrustedVerifyBatchesMock(var)
        }
    }
    impl ::std::convert::From<UpdateBatchFeeCall> for PolygonZkEVMMockCalls {
        fn from(var: UpdateBatchFeeCall) -> Self {
            PolygonZkEVMMockCalls::UpdateBatchFee(var)
        }
    }
    impl ::std::convert::From<VerifyBatchTimeTargetCall> for PolygonZkEVMMockCalls {
        fn from(var: VerifyBatchTimeTargetCall) -> Self {
            PolygonZkEVMMockCalls::VerifyBatchTimeTarget(var)
        }
    }
    impl ::std::convert::From<VerifyBatchesCall> for PolygonZkEVMMockCalls {
        fn from(var: VerifyBatchesCall) -> Self {
            PolygonZkEVMMockCalls::VerifyBatches(var)
        }
    }
    impl ::std::convert::From<VerifyBatchesTrustedAggregatorCall> for PolygonZkEVMMockCalls {
        fn from(var: VerifyBatchesTrustedAggregatorCall) -> Self {
            PolygonZkEVMMockCalls::VerifyBatchesTrustedAggregator(var)
        }
    }
    #[doc = "Container type for all return fields from the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AdminReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `batchFee` function with signature `batchFee()` and selector `[248, 184, 35, 228]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BatchFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `batchNumToStateRoot` function with signature `batchNumToStateRoot(uint64)` and selector `[83, 146, 197, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BatchNumToStateRootReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `bridgeAddress` function with signature `bridgeAddress()` and selector `[163, 197, 115, 235]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BridgeAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `calculateAccInputHash` function with signature `calculateAccInputHash(bytes32,bytes,bytes32,uint64,address)` and selector `[87, 79, 100, 158]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CalculateAccInputHashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `calculateRewardPerBatch` function with signature `calculateRewardPerBatch()` and selector `[153, 245, 99, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CalculateRewardPerBatchReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `chainID` function with signature `chainID()` and selector `[173, 200, 121, 233]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ChainIDReturn(pub u64);
    #[doc = "Container type for all return fields from the `forcedBatches` function with signature `forcedBatches(uint64)` and selector `[107, 134, 22, 206]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ForcedBatchesReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `forkID` function with signature `forkID()` and selector `[131, 28, 126, 173]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ForkIDReturn(pub u64);
    #[doc = "Container type for all return fields from the `getCurrentBatchFee` function with signature `getCurrentBatchFee()` and selector `[159, 13, 3, 157]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCurrentBatchFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getInputSnarkBytes` function with signature `getInputSnarkBytes(uint64,uint64,bytes32,bytes32,bytes32)` and selector `[34, 13, 120, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetInputSnarkBytesReturn(pub ethers::core::types::Bytes);
    #[doc = "Container type for all return fields from the `getLastVerifiedBatch` function with signature `getLastVerifiedBatch()` and selector `[192, 237, 132, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLastVerifiedBatchReturn(pub u64);
    #[doc = "Container type for all return fields from the `getNextSnarkInput` function with signature `getNextSnarkInput(uint64,uint64,uint64,bytes32,bytes32)` and selector `[14, 170, 134, 234]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetNextSnarkInputReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `isPendingStateConsolidable` function with signature `isPendingStateConsolidable(uint64)` and selector `[56, 59, 59, 232]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsPendingStateConsolidableReturn(pub bool);
    #[doc = "Container type for all return fields from the `lastBatchSequenced` function with signature `lastBatchSequenced()` and selector `[66, 63, 168, 86]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastBatchSequencedReturn(pub u64);
    #[doc = "Container type for all return fields from the `lastForceBatch` function with signature `lastForceBatch()` and selector `[231, 167, 237, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastForceBatchReturn(pub u64);
    #[doc = "Container type for all return fields from the `lastForceBatchSequenced` function with signature `lastForceBatchSequenced()` and selector `[69, 96, 82, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastForceBatchSequencedReturn(pub u64);
    #[doc = "Container type for all return fields from the `lastPendingState` function with signature `lastPendingState()` and selector `[69, 140, 4, 119]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastPendingStateReturn(pub u64);
    #[doc = "Container type for all return fields from the `lastPendingStateConsolidated` function with signature `lastPendingStateConsolidated()` and selector `[74, 26, 137, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastPendingStateConsolidatedReturn(pub u64);
    #[doc = "Container type for all return fields from the `lastTimestamp` function with signature `lastTimestamp()` and selector `[25, 216, 172, 97]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastTimestampReturn(pub u64);
    #[doc = "Container type for all return fields from the `lastVerifiedBatch` function with signature `lastVerifiedBatch()` and selector `[127, 203, 54, 83]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastVerifiedBatchReturn(pub u64);
    #[doc = "Container type for all return fields from the `matic` function with signature `matic()` and selector `[182, 176, 176, 151]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaticReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `multiplierBatchFee` function with signature `multiplierBatchFee()` and selector `[175, 210, 60, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MultiplierBatchFeeReturn(pub u16);
    #[doc = "Container type for all return fields from the `networkName` function with signature `networkName()` and selector `[16, 123, 242, 140]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NetworkNameReturn(pub String);
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
    #[doc = "Container type for all return fields from the `pendingAdmin` function with signature `pendingAdmin()` and selector `[38, 120, 34, 71]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PendingAdminReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `pendingStateTimeout` function with signature `pendingStateTimeout()` and selector `[217, 57, 179, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PendingStateTimeoutReturn(pub u64);
    #[doc = "Container type for all return fields from the `pendingStateTransitions` function with signature `pendingStateTransitions(uint256)` and selector `[131, 122, 71, 56]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PendingStateTransitionsReturn {
        pub timestamp: u64,
        pub last_verified_batch: u64,
        pub exit_root: [u8; 32],
        pub state_root: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `rollupVerifier` function with signature `rollupVerifier()` and selector `[232, 191, 146, 237]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RollupVerifierReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `sequencedBatches` function with signature `sequencedBatches(uint64)` and selector `[180, 214, 63, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SequencedBatchesReturn {
        pub acc_input_hash: [u8; 32],
        pub sequenced_timestamp: u64,
        pub previous_last_batch_sequenced: u64,
    }
    #[doc = "Container type for all return fields from the `trustedAggregator` function with signature `trustedAggregator()` and selector `[41, 135, 137, 131]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TrustedAggregatorReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `trustedAggregatorTimeout` function with signature `trustedAggregatorTimeout()` and selector `[132, 27, 36, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TrustedAggregatorTimeoutReturn(pub u64);
    #[doc = "Container type for all return fields from the `trustedSequencer` function with signature `trustedSequencer()` and selector `[207, 168, 237, 71]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TrustedSequencerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `trustedSequencerURL` function with signature `trustedSequencerURL()` and selector `[84, 32, 40, 213]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TrustedSequencerURLReturn(pub String);
    #[doc = "Container type for all return fields from the `verifyBatchTimeTarget` function with signature `verifyBatchTimeTarget()` and selector `[10, 13, 159, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VerifyBatchTimeTargetReturn(pub u64);
}
