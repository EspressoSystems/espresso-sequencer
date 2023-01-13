pub use proof_of_efficiency::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod proof_of_efficiency {
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
    #[doc = "ProofOfEfficiency was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"numBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"stateRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"pendingStateNum\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"ConsolidatePendingState\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [],\n\t\t\"name\": \"EmergencyStateActivated\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [],\n\t\t\"name\": \"EmergencyStateDeactivated\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"forceBatchNum\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"lastGlobalExitRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"sequencer\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"bytes\",\n\t\t\t\t\"name\": \"transactions\",\n\t\t\t\t\"type\": \"bytes\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"ForceBatch\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"uint8\",\n\t\t\t\t\"name\": \"version\",\n\t\t\t\t\"type\": \"uint8\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"Initialized\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"numBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"stateRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"aggregator\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"OverridePendingState\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"previousOwner\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"newOwner\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"OwnershipTransferred\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"storedStateRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"provedStateRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"ProveNonDeterministicPendingState\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"numBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"SequenceBatches\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"numBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"SequenceForceBatches\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"newAdmin\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"SetAdmin\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"bool\",\n\t\t\t\t\"name\": \"newForceBatchAllowed\",\n\t\t\t\t\"type\": \"bool\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"SetForceBatchAllowed\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"newPendingStateTimeout\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"SetPendingStateTimeout\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"newTrustedAggregator\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"SetTrustedAggregator\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"newTrustedAggregatorTimeout\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"SetTrustedAggregatorTimeout\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"newTrustedSequencer\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"SetTrustedSequencer\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"string\",\n\t\t\t\t\"name\": \"newTrustedSequencerURL\",\n\t\t\t\t\"type\": \"string\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"SetTrustedSequencerURL\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"numBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"stateRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"aggregator\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"TrustedVerifyBatches\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"anonymous\": false,\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"numBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": false,\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"stateRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"indexed\": true,\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"aggregator\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"VerifyBatches\",\n\t\t\"type\": \"event\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"FORCE_BATCH_TIMEOUT\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"HALT_AGGREGATION_TIMEOUT\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"MAX_TRANSACTIONS_BYTE_LENGTH\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"MAX_VERIFY_BATCHES\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"MULTIPLIER_BATCH_FEE\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"VERIFY_BATCH_TIME_TARGET\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"sequencedBatchNum\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"activateEmergencyState\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"admin\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"batchFee\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"batchNumToStateRoot\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"bridgeAddress\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"contract IBridge\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"calculateRewardPerBatch\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"chainID\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"pendingStateNum\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"consolidatePendingState\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"deactivateEmergencyState\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes\",\n\t\t\t\t\"name\": \"transactions\",\n\t\t\t\t\"type\": \"bytes\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256\",\n\t\t\t\t\"name\": \"maticAmount\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"forceBatch\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"forceBatchAllowed\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"bool\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"bool\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"forcedBatches\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"getCurrentBatchFee\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"initNumBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"finalNewBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"newLocalExitRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"oldStateRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"newStateRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"getInputSnarkBytes\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"bytes\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"getLastVerifiedBatch\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"globalExitRootManager\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"contract IGlobalExitRootManager\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"contract IGlobalExitRootManager\",\n\t\t\t\t\"name\": \"_globalExitRootManager\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"contract IERC20Upgradeable\",\n\t\t\t\t\"name\": \"_matic\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"contract IVerifierRollup\",\n\t\t\t\t\"name\": \"_rollupVerifier\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"contract IBridge\",\n\t\t\t\t\"name\": \"_bridgeAddress\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"components\": [\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\t\t\"name\": \"admin\",\n\t\t\t\t\t\t\"type\": \"address\"\n\t\t\t\t\t},\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\t\t\"name\": \"chainID\",\n\t\t\t\t\t\t\"type\": \"uint64\"\n\t\t\t\t\t},\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\t\t\"name\": \"trustedSequencer\",\n\t\t\t\t\t\t\"type\": \"address\"\n\t\t\t\t\t},\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\t\t\"name\": \"pendingStateTimeout\",\n\t\t\t\t\t\t\"type\": \"uint64\"\n\t\t\t\t\t},\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"bool\",\n\t\t\t\t\t\t\"name\": \"forceBatchAllowed\",\n\t\t\t\t\t\t\"type\": \"bool\"\n\t\t\t\t\t},\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\t\t\"name\": \"trustedAggregator\",\n\t\t\t\t\t\t\"type\": \"address\"\n\t\t\t\t\t},\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\t\t\"name\": \"trustedAggregatorTimeout\",\n\t\t\t\t\t\t\"type\": \"uint64\"\n\t\t\t\t\t}\n\t\t\t\t],\n\t\t\t\t\"internalType\": \"struct ProofOfEfficiency.InitializePackedParameters\",\n\t\t\t\t\"name\": \"initializePackedParameters\",\n\t\t\t\t\"type\": \"tuple\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"genesisRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"string\",\n\t\t\t\t\"name\": \"_trustedSequencerURL\",\n\t\t\t\t\"type\": \"string\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"string\",\n\t\t\t\t\"name\": \"_networkName\",\n\t\t\t\t\"type\": \"string\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"initialize\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"isEmergencyState\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"bool\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"bool\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"pendingStateNum\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"isPendingStateConsolidable\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"bool\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"bool\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"lastBatchSequenced\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"lastForceBatch\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"lastForceBatchSequenced\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"lastPendingState\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"lastPendingStateConsolidated\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"lastTimestamp\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"lastVerifiedBatch\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"matic\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"contract IERC20Upgradeable\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"networkName\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"string\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"string\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"initPendingStateNum\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"finalPendingStateNum\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"initNumBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"finalNewBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"newLocalExitRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"newStateRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256[2]\",\n\t\t\t\t\"name\": \"proofA\",\n\t\t\t\t\"type\": \"uint256[2]\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256[2][2]\",\n\t\t\t\t\"name\": \"proofB\",\n\t\t\t\t\"type\": \"uint256[2][2]\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256[2]\",\n\t\t\t\t\"name\": \"proofC\",\n\t\t\t\t\"type\": \"uint256[2]\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"overridePendingState\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"owner\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"pendingStateTimeout\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint256\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"pendingStateTransitions\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"timestamp\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"lastVerifiedBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"exitRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"stateRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"initPendingStateNum\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"finalPendingStateNum\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"initNumBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"finalNewBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"newLocalExitRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"newStateRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256[2]\",\n\t\t\t\t\"name\": \"proofA\",\n\t\t\t\t\"type\": \"uint256[2]\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256[2][2]\",\n\t\t\t\t\"name\": \"proofB\",\n\t\t\t\t\"type\": \"uint256[2][2]\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256[2]\",\n\t\t\t\t\"name\": \"proofC\",\n\t\t\t\t\"type\": \"uint256[2]\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"proveNonDeterministicPendingState\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"renounceOwnership\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"rollupVerifier\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"contract IVerifierRollup\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"components\": [\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"bytes\",\n\t\t\t\t\t\t\"name\": \"transactions\",\n\t\t\t\t\t\t\"type\": \"bytes\"\n\t\t\t\t\t},\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\t\t\"name\": \"globalExitRoot\",\n\t\t\t\t\t\t\"type\": \"bytes32\"\n\t\t\t\t\t},\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\t\t\"name\": \"timestamp\",\n\t\t\t\t\t\t\"type\": \"uint64\"\n\t\t\t\t\t},\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\t\t\"name\": \"minForcedTimestamp\",\n\t\t\t\t\t\t\"type\": \"uint64\"\n\t\t\t\t\t}\n\t\t\t\t],\n\t\t\t\t\"internalType\": \"struct ProofOfEfficiency.BatchData[]\",\n\t\t\t\t\"name\": \"batches\",\n\t\t\t\t\"type\": \"tuple[]\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"sequenceBatches\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"components\": [\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"bytes\",\n\t\t\t\t\t\t\"name\": \"transactions\",\n\t\t\t\t\t\t\"type\": \"bytes\"\n\t\t\t\t\t},\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\t\t\"name\": \"globalExitRoot\",\n\t\t\t\t\t\t\"type\": \"bytes32\"\n\t\t\t\t\t},\n\t\t\t\t\t{\n\t\t\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\t\t\"name\": \"minForcedTimestamp\",\n\t\t\t\t\t\t\"type\": \"uint64\"\n\t\t\t\t\t}\n\t\t\t\t],\n\t\t\t\t\"internalType\": \"struct ProofOfEfficiency.ForcedBatchData[]\",\n\t\t\t\t\"name\": \"batches\",\n\t\t\t\t\"type\": \"tuple[]\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"sequenceForceBatches\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"sequencedBatches\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"accInputHash\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"sequencedTimestamp\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"previousLastBatchSequenced\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"newAdmin\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"setAdmin\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"bool\",\n\t\t\t\t\"name\": \"newForceBatchAllowed\",\n\t\t\t\t\"type\": \"bool\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"setForceBatchAllowed\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"newPendingStateTimeout\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"setPendingStateTimeout\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"newTrustedAggregator\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"setTrustedAggregator\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"newTrustedAggregatorTimeout\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"setTrustedAggregatorTimeout\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"newTrustedSequencer\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"setTrustedSequencer\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"string\",\n\t\t\t\t\"name\": \"newTrustedSequencerURL\",\n\t\t\t\t\"type\": \"string\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"setTrustedSequencerURL\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"newOwner\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"transferOwnership\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"trustedAggregator\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"trustedAggregatorTimeout\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"trustedSequencer\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"address\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"address\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [],\n\t\t\"name\": \"trustedSequencerURL\",\n\t\t\"outputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"string\",\n\t\t\t\t\"name\": \"\",\n\t\t\t\t\"type\": \"string\"\n\t\t\t}\n\t\t],\n\t\t\"stateMutability\": \"view\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"pendingStateNum\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"initNumBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"finalNewBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"newLocalExitRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"newStateRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256[2]\",\n\t\t\t\t\"name\": \"proofA\",\n\t\t\t\t\"type\": \"uint256[2]\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256[2][2]\",\n\t\t\t\t\"name\": \"proofB\",\n\t\t\t\t\"type\": \"uint256[2][2]\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256[2]\",\n\t\t\t\t\"name\": \"proofC\",\n\t\t\t\t\"type\": \"uint256[2]\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"trustedVerifyBatches\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t},\n\t{\n\t\t\"inputs\": [\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"pendingStateNum\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"initNumBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint64\",\n\t\t\t\t\"name\": \"finalNewBatch\",\n\t\t\t\t\"type\": \"uint64\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"newLocalExitRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"bytes32\",\n\t\t\t\t\"name\": \"newStateRoot\",\n\t\t\t\t\"type\": \"bytes32\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256[2]\",\n\t\t\t\t\"name\": \"proofA\",\n\t\t\t\t\"type\": \"uint256[2]\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256[2][2]\",\n\t\t\t\t\"name\": \"proofB\",\n\t\t\t\t\"type\": \"uint256[2][2]\"\n\t\t\t},\n\t\t\t{\n\t\t\t\t\"internalType\": \"uint256[2]\",\n\t\t\t\t\"name\": \"proofC\",\n\t\t\t\t\"type\": \"uint256[2]\"\n\t\t\t}\n\t\t],\n\t\t\"name\": \"verifyBatches\",\n\t\t\"outputs\": [],\n\t\t\"stateMutability\": \"nonpayable\",\n\t\t\"type\": \"function\"\n\t}\n]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static PROOFOFEFFICIENCY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct ProofOfEfficiency<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ProofOfEfficiency<M> {
        fn clone(&self) -> Self {
            ProofOfEfficiency(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ProofOfEfficiency<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ProofOfEfficiency<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ProofOfEfficiency))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ProofOfEfficiency<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PROOFOFEFFICIENCY_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `FORCE_BATCH_TIMEOUT` (0xab9fc5ef) function"]
        pub fn force_batch_timeout(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([171, 159, 197, 239], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `HALT_AGGREGATION_TIMEOUT` (0x8b48931e) function"]
        pub fn halt_aggregation_timeout(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([139, 72, 147, 30], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_TRANSACTIONS_BYTE_LENGTH` (0x2d0889d3) function"]
        pub fn max_transactions_byte_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([45, 8, 137, 211], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_VERIFY_BATCHES` (0xe217cfd6) function"]
        pub fn max_verify_batches(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([226, 23, 207, 214], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MULTIPLIER_BATCH_FEE` (0xf1d7b21c) function"]
        pub fn multiplier_batch_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([241, 215, 178, 28], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `VERIFY_BATCH_TIME_TARGET` (0x137f1edf) function"]
        pub fn verify_batch_time_target(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([19, 127, 30, 223], ())
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
        #[doc = "Calls the contract's `forceBatchAllowed` (0xd8f54db0) function"]
        pub fn force_batch_allowed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([216, 245, 77, 176], ())
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
        #[doc = "Calls the contract's `globalExitRootManager` (0xd02103ca) function"]
        pub fn global_exit_root_manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([208, 33, 3, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x60943d6a) function"]
        pub fn initialize(
            &self,
            global_exit_root_manager: ethers::core::types::Address,
            matic: ethers::core::types::Address,
            rollup_verifier: ethers::core::types::Address,
            bridge_address: ethers::core::types::Address,
            initialize_packed_parameters: InitializePackedParameters,
            genesis_root: [u8; 32],
            trusted_sequencer_url: String,
            network_name: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [96, 148, 61, 106],
                    (
                        global_exit_root_manager,
                        matic,
                        rollup_verifier,
                        bridge_address,
                        initialize_packed_parameters,
                        genesis_root,
                        trusted_sequencer_url,
                        network_name,
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
        #[doc = "Calls the contract's `sequenceBatches` (0x3c158267) function"]
        pub fn sequence_batches(
            &self,
            batches: ::std::vec::Vec<BatchData>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 21, 130, 103], batches)
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
        #[doc = "Calls the contract's `setAdmin` (0x704b6c02) function"]
        pub fn set_admin(
            &self,
            new_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 75, 108, 2], new_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setForceBatchAllowed` (0x8c4a0af7) function"]
        pub fn set_force_batch_allowed(
            &self,
            new_force_batch_allowed: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 74, 10, 247], new_force_batch_allowed)
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
        #[doc = "Calls the contract's `trustedVerifyBatches` (0xedc41121) function"]
        pub fn trusted_verify_batches(
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
                    [237, 196, 17, 33],
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
        #[doc = "Gets the contract's `SetAdmin` event"]
        pub fn set_admin_filter(&self) -> ethers::contract::builders::Event<M, SetAdminFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetForceBatchAllowed` event"]
        pub fn set_force_batch_allowed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetForceBatchAllowedFilter> {
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
        #[doc = "Gets the contract's `TrustedVerifyBatches` event"]
        pub fn trusted_verify_batches_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TrustedVerifyBatchesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VerifyBatches` event"]
        pub fn verify_batches_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VerifyBatchesFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ProofOfEfficiencyEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ProofOfEfficiency<M>
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
    #[ethevent(name = "SetAdmin", abi = "SetAdmin(address)")]
    pub struct SetAdminFilter {
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
    #[ethevent(name = "SetForceBatchAllowed", abi = "SetForceBatchAllowed(bool)")]
    pub struct SetForceBatchAllowedFilter {
        pub new_force_batch_allowed: bool,
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
        name = "TrustedVerifyBatches",
        abi = "TrustedVerifyBatches(uint64,bytes32,address)"
    )]
    pub struct TrustedVerifyBatchesFilter {
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
    #[ethevent(name = "VerifyBatches", abi = "VerifyBatches(uint64,bytes32,address)")]
    pub struct VerifyBatchesFilter {
        #[ethevent(indexed)]
        pub num_batch: u64,
        pub state_root: [u8; 32],
        #[ethevent(indexed)]
        pub aggregator: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ProofOfEfficiencyEvents {
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
        SetAdminFilter(SetAdminFilter),
        SetForceBatchAllowedFilter(SetForceBatchAllowedFilter),
        SetPendingStateTimeoutFilter(SetPendingStateTimeoutFilter),
        SetTrustedAggregatorFilter(SetTrustedAggregatorFilter),
        SetTrustedAggregatorTimeoutFilter(SetTrustedAggregatorTimeoutFilter),
        SetTrustedSequencerFilter(SetTrustedSequencerFilter),
        SetTrustedSequencerURLFilter(SetTrustedSequencerURLFilter),
        TrustedVerifyBatchesFilter(TrustedVerifyBatchesFilter),
        VerifyBatchesFilter(VerifyBatchesFilter),
    }
    impl ethers::contract::EthLogDecode for ProofOfEfficiencyEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ConsolidatePendingStateFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::ConsolidatePendingStateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EmergencyStateActivatedFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::EmergencyStateActivatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EmergencyStateDeactivatedFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::EmergencyStateDeactivatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ForceBatchFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::ForceBatchFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OverridePendingStateFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::OverridePendingStateFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProveNonDeterministicPendingStateFilter::decode_log(log) {
                return Ok(
                    ProofOfEfficiencyEvents::ProveNonDeterministicPendingStateFilter(decoded),
                );
            }
            if let Ok(decoded) = SequenceBatchesFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::SequenceBatchesFilter(decoded));
            }
            if let Ok(decoded) = SequenceForceBatchesFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::SequenceForceBatchesFilter(decoded));
            }
            if let Ok(decoded) = SetAdminFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::SetAdminFilter(decoded));
            }
            if let Ok(decoded) = SetForceBatchAllowedFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::SetForceBatchAllowedFilter(decoded));
            }
            if let Ok(decoded) = SetPendingStateTimeoutFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::SetPendingStateTimeoutFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SetTrustedAggregatorFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::SetTrustedAggregatorFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedAggregatorTimeoutFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::SetTrustedAggregatorTimeoutFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SetTrustedSequencerFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::SetTrustedSequencerFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedSequencerURLFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::SetTrustedSequencerURLFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TrustedVerifyBatchesFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::TrustedVerifyBatchesFilter(decoded));
            }
            if let Ok(decoded) = VerifyBatchesFilter::decode_log(log) {
                return Ok(ProofOfEfficiencyEvents::VerifyBatchesFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ProofOfEfficiencyEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ProofOfEfficiencyEvents::ConsolidatePendingStateFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::EmergencyStateActivatedFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::EmergencyStateDeactivatedFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::ForceBatchFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::InitializedFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::OverridePendingStateFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::ProveNonDeterministicPendingStateFilter(element) => {
                    element.fmt(f)
                }
                ProofOfEfficiencyEvents::SequenceBatchesFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::SequenceForceBatchesFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::SetAdminFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::SetForceBatchAllowedFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::SetPendingStateTimeoutFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::SetTrustedAggregatorFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::SetTrustedAggregatorTimeoutFilter(element) => {
                    element.fmt(f)
                }
                ProofOfEfficiencyEvents::SetTrustedSequencerFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::SetTrustedSequencerURLFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::TrustedVerifyBatchesFilter(element) => element.fmt(f),
                ProofOfEfficiencyEvents::VerifyBatchesFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `FORCE_BATCH_TIMEOUT` function with signature `FORCE_BATCH_TIMEOUT()` and selector `[171, 159, 197, 239]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "FORCE_BATCH_TIMEOUT", abi = "FORCE_BATCH_TIMEOUT()")]
    pub struct ForceBatchTimeoutCall;
    #[doc = "Container type for all input parameters for the `HALT_AGGREGATION_TIMEOUT` function with signature `HALT_AGGREGATION_TIMEOUT()` and selector `[139, 72, 147, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "HALT_AGGREGATION_TIMEOUT", abi = "HALT_AGGREGATION_TIMEOUT()")]
    pub struct HaltAggregationTimeoutCall;
    #[doc = "Container type for all input parameters for the `MAX_TRANSACTIONS_BYTE_LENGTH` function with signature `MAX_TRANSACTIONS_BYTE_LENGTH()` and selector `[45, 8, 137, 211]`"]
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
        name = "MAX_TRANSACTIONS_BYTE_LENGTH",
        abi = "MAX_TRANSACTIONS_BYTE_LENGTH()"
    )]
    pub struct MaxTransactionsByteLengthCall;
    #[doc = "Container type for all input parameters for the `MAX_VERIFY_BATCHES` function with signature `MAX_VERIFY_BATCHES()` and selector `[226, 23, 207, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "MAX_VERIFY_BATCHES", abi = "MAX_VERIFY_BATCHES()")]
    pub struct MaxVerifyBatchesCall;
    #[doc = "Container type for all input parameters for the `MULTIPLIER_BATCH_FEE` function with signature `MULTIPLIER_BATCH_FEE()` and selector `[241, 215, 178, 28]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "MULTIPLIER_BATCH_FEE", abi = "MULTIPLIER_BATCH_FEE()")]
    pub struct MultiplierBatchFeeCall;
    #[doc = "Container type for all input parameters for the `VERIFY_BATCH_TIME_TARGET` function with signature `VERIFY_BATCH_TIME_TARGET()` and selector `[19, 127, 30, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "VERIFY_BATCH_TIME_TARGET", abi = "VERIFY_BATCH_TIME_TARGET()")]
    pub struct VerifyBatchTimeTargetCall;
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
    #[doc = "Container type for all input parameters for the `forceBatchAllowed` function with signature `forceBatchAllowed()` and selector `[216, 245, 77, 176]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "forceBatchAllowed", abi = "forceBatchAllowed()")]
    pub struct ForceBatchAllowedCall;
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
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,(address,uint64,address,uint64,bool,address,uint64),bytes32,string,string)` and selector `[96, 148, 61, 106]`"]
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
        abi = "initialize(address,address,address,address,(address,uint64,address,uint64,bool,address,uint64),bytes32,string,string)"
    )]
    pub struct InitializeCall {
        pub global_exit_root_manager: ethers::core::types::Address,
        pub matic: ethers::core::types::Address,
        pub rollup_verifier: ethers::core::types::Address,
        pub bridge_address: ethers::core::types::Address,
        pub initialize_packed_parameters: InitializePackedParameters,
        pub genesis_root: [u8; 32],
        pub trusted_sequencer_url: String,
        pub network_name: String,
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
    #[doc = "Container type for all input parameters for the `sequenceBatches` function with signature `sequenceBatches((bytes,bytes32,uint64,uint64)[])` and selector `[60, 21, 130, 103]`"]
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
        abi = "sequenceBatches((bytes,bytes32,uint64,uint64)[])"
    )]
    pub struct SequenceBatchesCall {
        pub batches: ::std::vec::Vec<BatchData>,
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
    #[doc = "Container type for all input parameters for the `setAdmin` function with signature `setAdmin(address)` and selector `[112, 75, 108, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setAdmin", abi = "setAdmin(address)")]
    pub struct SetAdminCall {
        pub new_admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setForceBatchAllowed` function with signature `setForceBatchAllowed(bool)` and selector `[140, 74, 10, 247]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setForceBatchAllowed", abi = "setForceBatchAllowed(bool)")]
    pub struct SetForceBatchAllowedCall {
        pub new_force_batch_allowed: bool,
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
    #[doc = "Container type for all input parameters for the `trustedVerifyBatches` function with signature `trustedVerifyBatches(uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])` and selector `[237, 196, 17, 33]`"]
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
        name = "trustedVerifyBatches",
        abi = "trustedVerifyBatches(uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])"
    )]
    pub struct TrustedVerifyBatchesCall {
        pub pending_state_num: u64,
        pub init_num_batch: u64,
        pub final_new_batch: u64,
        pub new_local_exit_root: [u8; 32],
        pub new_state_root: [u8; 32],
        pub proof_a: [ethers::core::types::U256; 2usize],
        pub proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
        pub proof_c: [ethers::core::types::U256; 2usize],
    }
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ProofOfEfficiencyCalls {
        ForceBatchTimeout(ForceBatchTimeoutCall),
        HaltAggregationTimeout(HaltAggregationTimeoutCall),
        MaxTransactionsByteLength(MaxTransactionsByteLengthCall),
        MaxVerifyBatches(MaxVerifyBatchesCall),
        MultiplierBatchFee(MultiplierBatchFeeCall),
        VerifyBatchTimeTarget(VerifyBatchTimeTargetCall),
        ActivateEmergencyState(ActivateEmergencyStateCall),
        Admin(AdminCall),
        BatchFee(BatchFeeCall),
        BatchNumToStateRoot(BatchNumToStateRootCall),
        BridgeAddress(BridgeAddressCall),
        CalculateRewardPerBatch(CalculateRewardPerBatchCall),
        ChainID(ChainIDCall),
        ConsolidatePendingState(ConsolidatePendingStateCall),
        DeactivateEmergencyState(DeactivateEmergencyStateCall),
        ForceBatch(ForceBatchCall),
        ForceBatchAllowed(ForceBatchAllowedCall),
        ForcedBatches(ForcedBatchesCall),
        GetCurrentBatchFee(GetCurrentBatchFeeCall),
        GetInputSnarkBytes(GetInputSnarkBytesCall),
        GetLastVerifiedBatch(GetLastVerifiedBatchCall),
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
        NetworkName(NetworkNameCall),
        OverridePendingState(OverridePendingStateCall),
        Owner(OwnerCall),
        PendingStateTimeout(PendingStateTimeoutCall),
        PendingStateTransitions(PendingStateTransitionsCall),
        ProveNonDeterministicPendingState(ProveNonDeterministicPendingStateCall),
        RenounceOwnership(RenounceOwnershipCall),
        RollupVerifier(RollupVerifierCall),
        SequenceBatches(SequenceBatchesCall),
        SequenceForceBatches(SequenceForceBatchesCall),
        SequencedBatches(SequencedBatchesCall),
        SetAdmin(SetAdminCall),
        SetForceBatchAllowed(SetForceBatchAllowedCall),
        SetPendingStateTimeout(SetPendingStateTimeoutCall),
        SetTrustedAggregator(SetTrustedAggregatorCall),
        SetTrustedAggregatorTimeout(SetTrustedAggregatorTimeoutCall),
        SetTrustedSequencer(SetTrustedSequencerCall),
        SetTrustedSequencerURL(SetTrustedSequencerURLCall),
        TransferOwnership(TransferOwnershipCall),
        TrustedAggregator(TrustedAggregatorCall),
        TrustedAggregatorTimeout(TrustedAggregatorTimeoutCall),
        TrustedSequencer(TrustedSequencerCall),
        TrustedSequencerURL(TrustedSequencerURLCall),
        TrustedVerifyBatches(TrustedVerifyBatchesCall),
        VerifyBatches(VerifyBatchesCall),
    }
    impl ethers::core::abi::AbiDecode for ProofOfEfficiencyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ForceBatchTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::ForceBatchTimeout(decoded));
            }
            if let Ok(decoded) =
                <HaltAggregationTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::HaltAggregationTimeout(decoded));
            }
            if let Ok(decoded) =
                <MaxTransactionsByteLengthCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ProofOfEfficiencyCalls::MaxTransactionsByteLength(decoded));
            }
            if let Ok(decoded) =
                <MaxVerifyBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::MaxVerifyBatches(decoded));
            }
            if let Ok(decoded) =
                <MultiplierBatchFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::MultiplierBatchFee(decoded));
            }
            if let Ok(decoded) =
                <VerifyBatchTimeTargetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::VerifyBatchTimeTarget(decoded));
            }
            if let Ok(decoded) =
                <ActivateEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::ActivateEmergencyState(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <BatchFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::BatchFee(decoded));
            }
            if let Ok(decoded) =
                <BatchNumToStateRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::BatchNumToStateRoot(decoded));
            }
            if let Ok(decoded) =
                <BridgeAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::BridgeAddress(decoded));
            }
            if let Ok(decoded) =
                <CalculateRewardPerBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::CalculateRewardPerBatch(decoded));
            }
            if let Ok(decoded) =
                <ChainIDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::ChainID(decoded));
            }
            if let Ok(decoded) =
                <ConsolidatePendingStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::ConsolidatePendingState(decoded));
            }
            if let Ok(decoded) =
                <DeactivateEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ProofOfEfficiencyCalls::DeactivateEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <ForceBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::ForceBatch(decoded));
            }
            if let Ok(decoded) =
                <ForceBatchAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::ForceBatchAllowed(decoded));
            }
            if let Ok(decoded) =
                <ForcedBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::ForcedBatches(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentBatchFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::GetCurrentBatchFee(decoded));
            }
            if let Ok(decoded) =
                <GetInputSnarkBytesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::GetInputSnarkBytes(decoded));
            }
            if let Ok(decoded) =
                <GetLastVerifiedBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::GetLastVerifiedBatch(decoded));
            }
            if let Ok(decoded) =
                <GlobalExitRootManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::GlobalExitRootManager(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::IsEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <IsPendingStateConsolidableCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ProofOfEfficiencyCalls::IsPendingStateConsolidable(decoded));
            }
            if let Ok(decoded) =
                <LastBatchSequencedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::LastBatchSequenced(decoded));
            }
            if let Ok(decoded) =
                <LastForceBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::LastForceBatch(decoded));
            }
            if let Ok(decoded) =
                <LastForceBatchSequencedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::LastForceBatchSequenced(decoded));
            }
            if let Ok(decoded) =
                <LastPendingStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::LastPendingState(decoded));
            }
            if let Ok(decoded) =
                <LastPendingStateConsolidatedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ProofOfEfficiencyCalls::LastPendingStateConsolidated(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LastTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::LastTimestamp(decoded));
            }
            if let Ok(decoded) =
                <LastVerifiedBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::LastVerifiedBatch(decoded));
            }
            if let Ok(decoded) = <MaticCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::Matic(decoded));
            }
            if let Ok(decoded) =
                <NetworkNameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::NetworkName(decoded));
            }
            if let Ok(decoded) =
                <OverridePendingStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::OverridePendingState(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PendingStateTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::PendingStateTimeout(decoded));
            }
            if let Ok(decoded) =
                <PendingStateTransitionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::PendingStateTransitions(decoded));
            }
            if let Ok(decoded) =
                <ProveNonDeterministicPendingStateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ProofOfEfficiencyCalls::ProveNonDeterministicPendingState(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RollupVerifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::RollupVerifier(decoded));
            }
            if let Ok(decoded) =
                <SequenceBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::SequenceBatches(decoded));
            }
            if let Ok(decoded) =
                <SequenceForceBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::SequenceForceBatches(decoded));
            }
            if let Ok(decoded) =
                <SequencedBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::SequencedBatches(decoded));
            }
            if let Ok(decoded) =
                <SetAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::SetAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetForceBatchAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::SetForceBatchAllowed(decoded));
            }
            if let Ok(decoded) =
                <SetPendingStateTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::SetPendingStateTimeout(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedAggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::SetTrustedAggregator(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedAggregatorTimeoutCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ProofOfEfficiencyCalls::SetTrustedAggregatorTimeout(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedSequencerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::SetTrustedSequencer(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedSequencerURLCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::SetTrustedSequencerURL(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TrustedAggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::TrustedAggregator(decoded));
            }
            if let Ok(decoded) =
                <TrustedAggregatorTimeoutCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ProofOfEfficiencyCalls::TrustedAggregatorTimeout(decoded));
            }
            if let Ok(decoded) =
                <TrustedSequencerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::TrustedSequencer(decoded));
            }
            if let Ok(decoded) =
                <TrustedSequencerURLCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::TrustedSequencerURL(decoded));
            }
            if let Ok(decoded) =
                <TrustedVerifyBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::TrustedVerifyBatches(decoded));
            }
            if let Ok(decoded) =
                <VerifyBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProofOfEfficiencyCalls::VerifyBatches(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ProofOfEfficiencyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ProofOfEfficiencyCalls::ForceBatchTimeout(element) => element.encode(),
                ProofOfEfficiencyCalls::HaltAggregationTimeout(element) => element.encode(),
                ProofOfEfficiencyCalls::MaxTransactionsByteLength(element) => element.encode(),
                ProofOfEfficiencyCalls::MaxVerifyBatches(element) => element.encode(),
                ProofOfEfficiencyCalls::MultiplierBatchFee(element) => element.encode(),
                ProofOfEfficiencyCalls::VerifyBatchTimeTarget(element) => element.encode(),
                ProofOfEfficiencyCalls::ActivateEmergencyState(element) => element.encode(),
                ProofOfEfficiencyCalls::Admin(element) => element.encode(),
                ProofOfEfficiencyCalls::BatchFee(element) => element.encode(),
                ProofOfEfficiencyCalls::BatchNumToStateRoot(element) => element.encode(),
                ProofOfEfficiencyCalls::BridgeAddress(element) => element.encode(),
                ProofOfEfficiencyCalls::CalculateRewardPerBatch(element) => element.encode(),
                ProofOfEfficiencyCalls::ChainID(element) => element.encode(),
                ProofOfEfficiencyCalls::ConsolidatePendingState(element) => element.encode(),
                ProofOfEfficiencyCalls::DeactivateEmergencyState(element) => element.encode(),
                ProofOfEfficiencyCalls::ForceBatch(element) => element.encode(),
                ProofOfEfficiencyCalls::ForceBatchAllowed(element) => element.encode(),
                ProofOfEfficiencyCalls::ForcedBatches(element) => element.encode(),
                ProofOfEfficiencyCalls::GetCurrentBatchFee(element) => element.encode(),
                ProofOfEfficiencyCalls::GetInputSnarkBytes(element) => element.encode(),
                ProofOfEfficiencyCalls::GetLastVerifiedBatch(element) => element.encode(),
                ProofOfEfficiencyCalls::GlobalExitRootManager(element) => element.encode(),
                ProofOfEfficiencyCalls::Initialize(element) => element.encode(),
                ProofOfEfficiencyCalls::IsEmergencyState(element) => element.encode(),
                ProofOfEfficiencyCalls::IsPendingStateConsolidable(element) => element.encode(),
                ProofOfEfficiencyCalls::LastBatchSequenced(element) => element.encode(),
                ProofOfEfficiencyCalls::LastForceBatch(element) => element.encode(),
                ProofOfEfficiencyCalls::LastForceBatchSequenced(element) => element.encode(),
                ProofOfEfficiencyCalls::LastPendingState(element) => element.encode(),
                ProofOfEfficiencyCalls::LastPendingStateConsolidated(element) => element.encode(),
                ProofOfEfficiencyCalls::LastTimestamp(element) => element.encode(),
                ProofOfEfficiencyCalls::LastVerifiedBatch(element) => element.encode(),
                ProofOfEfficiencyCalls::Matic(element) => element.encode(),
                ProofOfEfficiencyCalls::NetworkName(element) => element.encode(),
                ProofOfEfficiencyCalls::OverridePendingState(element) => element.encode(),
                ProofOfEfficiencyCalls::Owner(element) => element.encode(),
                ProofOfEfficiencyCalls::PendingStateTimeout(element) => element.encode(),
                ProofOfEfficiencyCalls::PendingStateTransitions(element) => element.encode(),
                ProofOfEfficiencyCalls::ProveNonDeterministicPendingState(element) => {
                    element.encode()
                }
                ProofOfEfficiencyCalls::RenounceOwnership(element) => element.encode(),
                ProofOfEfficiencyCalls::RollupVerifier(element) => element.encode(),
                ProofOfEfficiencyCalls::SequenceBatches(element) => element.encode(),
                ProofOfEfficiencyCalls::SequenceForceBatches(element) => element.encode(),
                ProofOfEfficiencyCalls::SequencedBatches(element) => element.encode(),
                ProofOfEfficiencyCalls::SetAdmin(element) => element.encode(),
                ProofOfEfficiencyCalls::SetForceBatchAllowed(element) => element.encode(),
                ProofOfEfficiencyCalls::SetPendingStateTimeout(element) => element.encode(),
                ProofOfEfficiencyCalls::SetTrustedAggregator(element) => element.encode(),
                ProofOfEfficiencyCalls::SetTrustedAggregatorTimeout(element) => element.encode(),
                ProofOfEfficiencyCalls::SetTrustedSequencer(element) => element.encode(),
                ProofOfEfficiencyCalls::SetTrustedSequencerURL(element) => element.encode(),
                ProofOfEfficiencyCalls::TransferOwnership(element) => element.encode(),
                ProofOfEfficiencyCalls::TrustedAggregator(element) => element.encode(),
                ProofOfEfficiencyCalls::TrustedAggregatorTimeout(element) => element.encode(),
                ProofOfEfficiencyCalls::TrustedSequencer(element) => element.encode(),
                ProofOfEfficiencyCalls::TrustedSequencerURL(element) => element.encode(),
                ProofOfEfficiencyCalls::TrustedVerifyBatches(element) => element.encode(),
                ProofOfEfficiencyCalls::VerifyBatches(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ProofOfEfficiencyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ProofOfEfficiencyCalls::ForceBatchTimeout(element) => element.fmt(f),
                ProofOfEfficiencyCalls::HaltAggregationTimeout(element) => element.fmt(f),
                ProofOfEfficiencyCalls::MaxTransactionsByteLength(element) => element.fmt(f),
                ProofOfEfficiencyCalls::MaxVerifyBatches(element) => element.fmt(f),
                ProofOfEfficiencyCalls::MultiplierBatchFee(element) => element.fmt(f),
                ProofOfEfficiencyCalls::VerifyBatchTimeTarget(element) => element.fmt(f),
                ProofOfEfficiencyCalls::ActivateEmergencyState(element) => element.fmt(f),
                ProofOfEfficiencyCalls::Admin(element) => element.fmt(f),
                ProofOfEfficiencyCalls::BatchFee(element) => element.fmt(f),
                ProofOfEfficiencyCalls::BatchNumToStateRoot(element) => element.fmt(f),
                ProofOfEfficiencyCalls::BridgeAddress(element) => element.fmt(f),
                ProofOfEfficiencyCalls::CalculateRewardPerBatch(element) => element.fmt(f),
                ProofOfEfficiencyCalls::ChainID(element) => element.fmt(f),
                ProofOfEfficiencyCalls::ConsolidatePendingState(element) => element.fmt(f),
                ProofOfEfficiencyCalls::DeactivateEmergencyState(element) => element.fmt(f),
                ProofOfEfficiencyCalls::ForceBatch(element) => element.fmt(f),
                ProofOfEfficiencyCalls::ForceBatchAllowed(element) => element.fmt(f),
                ProofOfEfficiencyCalls::ForcedBatches(element) => element.fmt(f),
                ProofOfEfficiencyCalls::GetCurrentBatchFee(element) => element.fmt(f),
                ProofOfEfficiencyCalls::GetInputSnarkBytes(element) => element.fmt(f),
                ProofOfEfficiencyCalls::GetLastVerifiedBatch(element) => element.fmt(f),
                ProofOfEfficiencyCalls::GlobalExitRootManager(element) => element.fmt(f),
                ProofOfEfficiencyCalls::Initialize(element) => element.fmt(f),
                ProofOfEfficiencyCalls::IsEmergencyState(element) => element.fmt(f),
                ProofOfEfficiencyCalls::IsPendingStateConsolidable(element) => element.fmt(f),
                ProofOfEfficiencyCalls::LastBatchSequenced(element) => element.fmt(f),
                ProofOfEfficiencyCalls::LastForceBatch(element) => element.fmt(f),
                ProofOfEfficiencyCalls::LastForceBatchSequenced(element) => element.fmt(f),
                ProofOfEfficiencyCalls::LastPendingState(element) => element.fmt(f),
                ProofOfEfficiencyCalls::LastPendingStateConsolidated(element) => element.fmt(f),
                ProofOfEfficiencyCalls::LastTimestamp(element) => element.fmt(f),
                ProofOfEfficiencyCalls::LastVerifiedBatch(element) => element.fmt(f),
                ProofOfEfficiencyCalls::Matic(element) => element.fmt(f),
                ProofOfEfficiencyCalls::NetworkName(element) => element.fmt(f),
                ProofOfEfficiencyCalls::OverridePendingState(element) => element.fmt(f),
                ProofOfEfficiencyCalls::Owner(element) => element.fmt(f),
                ProofOfEfficiencyCalls::PendingStateTimeout(element) => element.fmt(f),
                ProofOfEfficiencyCalls::PendingStateTransitions(element) => element.fmt(f),
                ProofOfEfficiencyCalls::ProveNonDeterministicPendingState(element) => {
                    element.fmt(f)
                }
                ProofOfEfficiencyCalls::RenounceOwnership(element) => element.fmt(f),
                ProofOfEfficiencyCalls::RollupVerifier(element) => element.fmt(f),
                ProofOfEfficiencyCalls::SequenceBatches(element) => element.fmt(f),
                ProofOfEfficiencyCalls::SequenceForceBatches(element) => element.fmt(f),
                ProofOfEfficiencyCalls::SequencedBatches(element) => element.fmt(f),
                ProofOfEfficiencyCalls::SetAdmin(element) => element.fmt(f),
                ProofOfEfficiencyCalls::SetForceBatchAllowed(element) => element.fmt(f),
                ProofOfEfficiencyCalls::SetPendingStateTimeout(element) => element.fmt(f),
                ProofOfEfficiencyCalls::SetTrustedAggregator(element) => element.fmt(f),
                ProofOfEfficiencyCalls::SetTrustedAggregatorTimeout(element) => element.fmt(f),
                ProofOfEfficiencyCalls::SetTrustedSequencer(element) => element.fmt(f),
                ProofOfEfficiencyCalls::SetTrustedSequencerURL(element) => element.fmt(f),
                ProofOfEfficiencyCalls::TransferOwnership(element) => element.fmt(f),
                ProofOfEfficiencyCalls::TrustedAggregator(element) => element.fmt(f),
                ProofOfEfficiencyCalls::TrustedAggregatorTimeout(element) => element.fmt(f),
                ProofOfEfficiencyCalls::TrustedSequencer(element) => element.fmt(f),
                ProofOfEfficiencyCalls::TrustedSequencerURL(element) => element.fmt(f),
                ProofOfEfficiencyCalls::TrustedVerifyBatches(element) => element.fmt(f),
                ProofOfEfficiencyCalls::VerifyBatches(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ForceBatchTimeoutCall> for ProofOfEfficiencyCalls {
        fn from(var: ForceBatchTimeoutCall) -> Self {
            ProofOfEfficiencyCalls::ForceBatchTimeout(var)
        }
    }
    impl ::std::convert::From<HaltAggregationTimeoutCall> for ProofOfEfficiencyCalls {
        fn from(var: HaltAggregationTimeoutCall) -> Self {
            ProofOfEfficiencyCalls::HaltAggregationTimeout(var)
        }
    }
    impl ::std::convert::From<MaxTransactionsByteLengthCall> for ProofOfEfficiencyCalls {
        fn from(var: MaxTransactionsByteLengthCall) -> Self {
            ProofOfEfficiencyCalls::MaxTransactionsByteLength(var)
        }
    }
    impl ::std::convert::From<MaxVerifyBatchesCall> for ProofOfEfficiencyCalls {
        fn from(var: MaxVerifyBatchesCall) -> Self {
            ProofOfEfficiencyCalls::MaxVerifyBatches(var)
        }
    }
    impl ::std::convert::From<MultiplierBatchFeeCall> for ProofOfEfficiencyCalls {
        fn from(var: MultiplierBatchFeeCall) -> Self {
            ProofOfEfficiencyCalls::MultiplierBatchFee(var)
        }
    }
    impl ::std::convert::From<VerifyBatchTimeTargetCall> for ProofOfEfficiencyCalls {
        fn from(var: VerifyBatchTimeTargetCall) -> Self {
            ProofOfEfficiencyCalls::VerifyBatchTimeTarget(var)
        }
    }
    impl ::std::convert::From<ActivateEmergencyStateCall> for ProofOfEfficiencyCalls {
        fn from(var: ActivateEmergencyStateCall) -> Self {
            ProofOfEfficiencyCalls::ActivateEmergencyState(var)
        }
    }
    impl ::std::convert::From<AdminCall> for ProofOfEfficiencyCalls {
        fn from(var: AdminCall) -> Self {
            ProofOfEfficiencyCalls::Admin(var)
        }
    }
    impl ::std::convert::From<BatchFeeCall> for ProofOfEfficiencyCalls {
        fn from(var: BatchFeeCall) -> Self {
            ProofOfEfficiencyCalls::BatchFee(var)
        }
    }
    impl ::std::convert::From<BatchNumToStateRootCall> for ProofOfEfficiencyCalls {
        fn from(var: BatchNumToStateRootCall) -> Self {
            ProofOfEfficiencyCalls::BatchNumToStateRoot(var)
        }
    }
    impl ::std::convert::From<BridgeAddressCall> for ProofOfEfficiencyCalls {
        fn from(var: BridgeAddressCall) -> Self {
            ProofOfEfficiencyCalls::BridgeAddress(var)
        }
    }
    impl ::std::convert::From<CalculateRewardPerBatchCall> for ProofOfEfficiencyCalls {
        fn from(var: CalculateRewardPerBatchCall) -> Self {
            ProofOfEfficiencyCalls::CalculateRewardPerBatch(var)
        }
    }
    impl ::std::convert::From<ChainIDCall> for ProofOfEfficiencyCalls {
        fn from(var: ChainIDCall) -> Self {
            ProofOfEfficiencyCalls::ChainID(var)
        }
    }
    impl ::std::convert::From<ConsolidatePendingStateCall> for ProofOfEfficiencyCalls {
        fn from(var: ConsolidatePendingStateCall) -> Self {
            ProofOfEfficiencyCalls::ConsolidatePendingState(var)
        }
    }
    impl ::std::convert::From<DeactivateEmergencyStateCall> for ProofOfEfficiencyCalls {
        fn from(var: DeactivateEmergencyStateCall) -> Self {
            ProofOfEfficiencyCalls::DeactivateEmergencyState(var)
        }
    }
    impl ::std::convert::From<ForceBatchCall> for ProofOfEfficiencyCalls {
        fn from(var: ForceBatchCall) -> Self {
            ProofOfEfficiencyCalls::ForceBatch(var)
        }
    }
    impl ::std::convert::From<ForceBatchAllowedCall> for ProofOfEfficiencyCalls {
        fn from(var: ForceBatchAllowedCall) -> Self {
            ProofOfEfficiencyCalls::ForceBatchAllowed(var)
        }
    }
    impl ::std::convert::From<ForcedBatchesCall> for ProofOfEfficiencyCalls {
        fn from(var: ForcedBatchesCall) -> Self {
            ProofOfEfficiencyCalls::ForcedBatches(var)
        }
    }
    impl ::std::convert::From<GetCurrentBatchFeeCall> for ProofOfEfficiencyCalls {
        fn from(var: GetCurrentBatchFeeCall) -> Self {
            ProofOfEfficiencyCalls::GetCurrentBatchFee(var)
        }
    }
    impl ::std::convert::From<GetInputSnarkBytesCall> for ProofOfEfficiencyCalls {
        fn from(var: GetInputSnarkBytesCall) -> Self {
            ProofOfEfficiencyCalls::GetInputSnarkBytes(var)
        }
    }
    impl ::std::convert::From<GetLastVerifiedBatchCall> for ProofOfEfficiencyCalls {
        fn from(var: GetLastVerifiedBatchCall) -> Self {
            ProofOfEfficiencyCalls::GetLastVerifiedBatch(var)
        }
    }
    impl ::std::convert::From<GlobalExitRootManagerCall> for ProofOfEfficiencyCalls {
        fn from(var: GlobalExitRootManagerCall) -> Self {
            ProofOfEfficiencyCalls::GlobalExitRootManager(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for ProofOfEfficiencyCalls {
        fn from(var: InitializeCall) -> Self {
            ProofOfEfficiencyCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<IsEmergencyStateCall> for ProofOfEfficiencyCalls {
        fn from(var: IsEmergencyStateCall) -> Self {
            ProofOfEfficiencyCalls::IsEmergencyState(var)
        }
    }
    impl ::std::convert::From<IsPendingStateConsolidableCall> for ProofOfEfficiencyCalls {
        fn from(var: IsPendingStateConsolidableCall) -> Self {
            ProofOfEfficiencyCalls::IsPendingStateConsolidable(var)
        }
    }
    impl ::std::convert::From<LastBatchSequencedCall> for ProofOfEfficiencyCalls {
        fn from(var: LastBatchSequencedCall) -> Self {
            ProofOfEfficiencyCalls::LastBatchSequenced(var)
        }
    }
    impl ::std::convert::From<LastForceBatchCall> for ProofOfEfficiencyCalls {
        fn from(var: LastForceBatchCall) -> Self {
            ProofOfEfficiencyCalls::LastForceBatch(var)
        }
    }
    impl ::std::convert::From<LastForceBatchSequencedCall> for ProofOfEfficiencyCalls {
        fn from(var: LastForceBatchSequencedCall) -> Self {
            ProofOfEfficiencyCalls::LastForceBatchSequenced(var)
        }
    }
    impl ::std::convert::From<LastPendingStateCall> for ProofOfEfficiencyCalls {
        fn from(var: LastPendingStateCall) -> Self {
            ProofOfEfficiencyCalls::LastPendingState(var)
        }
    }
    impl ::std::convert::From<LastPendingStateConsolidatedCall> for ProofOfEfficiencyCalls {
        fn from(var: LastPendingStateConsolidatedCall) -> Self {
            ProofOfEfficiencyCalls::LastPendingStateConsolidated(var)
        }
    }
    impl ::std::convert::From<LastTimestampCall> for ProofOfEfficiencyCalls {
        fn from(var: LastTimestampCall) -> Self {
            ProofOfEfficiencyCalls::LastTimestamp(var)
        }
    }
    impl ::std::convert::From<LastVerifiedBatchCall> for ProofOfEfficiencyCalls {
        fn from(var: LastVerifiedBatchCall) -> Self {
            ProofOfEfficiencyCalls::LastVerifiedBatch(var)
        }
    }
    impl ::std::convert::From<MaticCall> for ProofOfEfficiencyCalls {
        fn from(var: MaticCall) -> Self {
            ProofOfEfficiencyCalls::Matic(var)
        }
    }
    impl ::std::convert::From<NetworkNameCall> for ProofOfEfficiencyCalls {
        fn from(var: NetworkNameCall) -> Self {
            ProofOfEfficiencyCalls::NetworkName(var)
        }
    }
    impl ::std::convert::From<OverridePendingStateCall> for ProofOfEfficiencyCalls {
        fn from(var: OverridePendingStateCall) -> Self {
            ProofOfEfficiencyCalls::OverridePendingState(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ProofOfEfficiencyCalls {
        fn from(var: OwnerCall) -> Self {
            ProofOfEfficiencyCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PendingStateTimeoutCall> for ProofOfEfficiencyCalls {
        fn from(var: PendingStateTimeoutCall) -> Self {
            ProofOfEfficiencyCalls::PendingStateTimeout(var)
        }
    }
    impl ::std::convert::From<PendingStateTransitionsCall> for ProofOfEfficiencyCalls {
        fn from(var: PendingStateTransitionsCall) -> Self {
            ProofOfEfficiencyCalls::PendingStateTransitions(var)
        }
    }
    impl ::std::convert::From<ProveNonDeterministicPendingStateCall> for ProofOfEfficiencyCalls {
        fn from(var: ProveNonDeterministicPendingStateCall) -> Self {
            ProofOfEfficiencyCalls::ProveNonDeterministicPendingState(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for ProofOfEfficiencyCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            ProofOfEfficiencyCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RollupVerifierCall> for ProofOfEfficiencyCalls {
        fn from(var: RollupVerifierCall) -> Self {
            ProofOfEfficiencyCalls::RollupVerifier(var)
        }
    }
    impl ::std::convert::From<SequenceBatchesCall> for ProofOfEfficiencyCalls {
        fn from(var: SequenceBatchesCall) -> Self {
            ProofOfEfficiencyCalls::SequenceBatches(var)
        }
    }
    impl ::std::convert::From<SequenceForceBatchesCall> for ProofOfEfficiencyCalls {
        fn from(var: SequenceForceBatchesCall) -> Self {
            ProofOfEfficiencyCalls::SequenceForceBatches(var)
        }
    }
    impl ::std::convert::From<SequencedBatchesCall> for ProofOfEfficiencyCalls {
        fn from(var: SequencedBatchesCall) -> Self {
            ProofOfEfficiencyCalls::SequencedBatches(var)
        }
    }
    impl ::std::convert::From<SetAdminCall> for ProofOfEfficiencyCalls {
        fn from(var: SetAdminCall) -> Self {
            ProofOfEfficiencyCalls::SetAdmin(var)
        }
    }
    impl ::std::convert::From<SetForceBatchAllowedCall> for ProofOfEfficiencyCalls {
        fn from(var: SetForceBatchAllowedCall) -> Self {
            ProofOfEfficiencyCalls::SetForceBatchAllowed(var)
        }
    }
    impl ::std::convert::From<SetPendingStateTimeoutCall> for ProofOfEfficiencyCalls {
        fn from(var: SetPendingStateTimeoutCall) -> Self {
            ProofOfEfficiencyCalls::SetPendingStateTimeout(var)
        }
    }
    impl ::std::convert::From<SetTrustedAggregatorCall> for ProofOfEfficiencyCalls {
        fn from(var: SetTrustedAggregatorCall) -> Self {
            ProofOfEfficiencyCalls::SetTrustedAggregator(var)
        }
    }
    impl ::std::convert::From<SetTrustedAggregatorTimeoutCall> for ProofOfEfficiencyCalls {
        fn from(var: SetTrustedAggregatorTimeoutCall) -> Self {
            ProofOfEfficiencyCalls::SetTrustedAggregatorTimeout(var)
        }
    }
    impl ::std::convert::From<SetTrustedSequencerCall> for ProofOfEfficiencyCalls {
        fn from(var: SetTrustedSequencerCall) -> Self {
            ProofOfEfficiencyCalls::SetTrustedSequencer(var)
        }
    }
    impl ::std::convert::From<SetTrustedSequencerURLCall> for ProofOfEfficiencyCalls {
        fn from(var: SetTrustedSequencerURLCall) -> Self {
            ProofOfEfficiencyCalls::SetTrustedSequencerURL(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ProofOfEfficiencyCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            ProofOfEfficiencyCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<TrustedAggregatorCall> for ProofOfEfficiencyCalls {
        fn from(var: TrustedAggregatorCall) -> Self {
            ProofOfEfficiencyCalls::TrustedAggregator(var)
        }
    }
    impl ::std::convert::From<TrustedAggregatorTimeoutCall> for ProofOfEfficiencyCalls {
        fn from(var: TrustedAggregatorTimeoutCall) -> Self {
            ProofOfEfficiencyCalls::TrustedAggregatorTimeout(var)
        }
    }
    impl ::std::convert::From<TrustedSequencerCall> for ProofOfEfficiencyCalls {
        fn from(var: TrustedSequencerCall) -> Self {
            ProofOfEfficiencyCalls::TrustedSequencer(var)
        }
    }
    impl ::std::convert::From<TrustedSequencerURLCall> for ProofOfEfficiencyCalls {
        fn from(var: TrustedSequencerURLCall) -> Self {
            ProofOfEfficiencyCalls::TrustedSequencerURL(var)
        }
    }
    impl ::std::convert::From<TrustedVerifyBatchesCall> for ProofOfEfficiencyCalls {
        fn from(var: TrustedVerifyBatchesCall) -> Self {
            ProofOfEfficiencyCalls::TrustedVerifyBatches(var)
        }
    }
    impl ::std::convert::From<VerifyBatchesCall> for ProofOfEfficiencyCalls {
        fn from(var: VerifyBatchesCall) -> Self {
            ProofOfEfficiencyCalls::VerifyBatches(var)
        }
    }
    #[doc = "Container type for all return fields from the `FORCE_BATCH_TIMEOUT` function with signature `FORCE_BATCH_TIMEOUT()` and selector `[171, 159, 197, 239]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ForceBatchTimeoutReturn(pub u64);
    #[doc = "Container type for all return fields from the `HALT_AGGREGATION_TIMEOUT` function with signature `HALT_AGGREGATION_TIMEOUT()` and selector `[139, 72, 147, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HaltAggregationTimeoutReturn(pub u64);
    #[doc = "Container type for all return fields from the `MAX_TRANSACTIONS_BYTE_LENGTH` function with signature `MAX_TRANSACTIONS_BYTE_LENGTH()` and selector `[45, 8, 137, 211]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxTransactionsByteLengthReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `MAX_VERIFY_BATCHES` function with signature `MAX_VERIFY_BATCHES()` and selector `[226, 23, 207, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxVerifyBatchesReturn(pub u64);
    #[doc = "Container type for all return fields from the `MULTIPLIER_BATCH_FEE` function with signature `MULTIPLIER_BATCH_FEE()` and selector `[241, 215, 178, 28]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MultiplierBatchFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `VERIFY_BATCH_TIME_TARGET` function with signature `VERIFY_BATCH_TIME_TARGET()` and selector `[19, 127, 30, 223]`"]
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
    #[doc = "Container type for all return fields from the `forceBatchAllowed` function with signature `forceBatchAllowed()` and selector `[216, 245, 77, 176]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ForceBatchAllowedReturn(pub bool);
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
    #[doc = "`BatchData(bytes,bytes32,uint64,uint64)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BatchData {
        pub transactions: ethers::core::types::Bytes,
        pub global_exit_root: [u8; 32],
        pub timestamp: u64,
        pub min_forced_timestamp: u64,
    }
    #[doc = "`ForcedBatchData(bytes,bytes32,uint64)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ForcedBatchData {
        pub transactions: ethers::core::types::Bytes,
        pub global_exit_root: [u8; 32],
        pub min_forced_timestamp: u64,
    }
    #[doc = "`InitializePackedParameters(address,uint64,address,uint64,bool,address,uint64)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct InitializePackedParameters {
        pub admin: ethers::core::types::Address,
        pub chain_id: u64,
        pub trusted_sequencer: ethers::core::types::Address,
        pub pending_state_timeout: u64,
        pub force_batch_allowed: bool,
        pub trusted_aggregator: ethers::core::types::Address,
        pub trusted_aggregator_timeout: u64,
    }
}
