use crate::ResolvableChainConfig;

use super::{block::NsTableValidationError, BlockSize, FeeAccount, FeeAmount};
use anyhow::{ensure, Context};
use ethers::types::{Address, U256};
use jf_merkle_tree::{
    prelude::{LightWeightSHA3MerkleTree, Sha3Digest, Sha3Node},
    universal_merkle_tree::UniversalMerkleTree,
    ForgetableMerkleTreeScheme, ForgetableUniversalMerkleTreeScheme, LookupResult,
    MerkleCommitment, MerkleTreeError, MerkleTreeScheme, UniversalMerkleTreeScheme,
};
use serde::{Deserialize, Serialize};

// The block merkle tree accumulates header commitments. However, since the underlying
// representation of the commitment type remains the same even while the header itself changes,
// using the underlying type `[u8; 32]` allows us to use the same state type across minor versions.
pub type BlockMerkleTree = LightWeightSHA3MerkleTree<[u8; 32]>;
pub type BlockMerkleCommitment = <BlockMerkleTree as MerkleTreeScheme>::Commitment;

pub type FeeMerkleTree = UniversalMerkleTree<FeeAmount, Sha3Digest, FeeAccount, 256, Sha3Node>;
pub type FeeMerkleCommitment = <FeeMerkleTree as MerkleTreeScheme>::Commitment;

use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, Read, SerializationError, Valid, Validate,
};
use async_std::stream::StreamExt;
use async_std::sync::RwLock;
use committable::{Commitment, Committable, RawCommitmentBuilder};
use contract_bindings::fee_contract::DepositFilter;
use core::fmt::Debug;
use derive_more::{Add, Display, From, Into, Mul, Sub};
use ethers::utils::{parse_units, ParseUnits};
use futures::future::Future;
use hotshot::traits::ValidatedState as HotShotState;
use hotshot_query_service::{
    availability::{AvailabilityDataSource, LeafQueryData},
    data_source::VersionedDataSource,
    explorer::MonetaryValue,
    merklized_state::{MerklizedState, MerklizedStateHeightPersistence, UpdateStateData},
    types::HeightIndexed,
};
use hotshot_types::{
    data::{BlockError, ViewNumber},
    traits::{
        block_contents::{BlockHeader, BuilderFee},
        node_implementation::ConsensusTime,
        signature_key::BuilderSignatureKey,
        states::StateDelta,
    },
    vid::{VidCommon, VidSchemeType},
};
use itertools::Itertools;
use jf_vid::VidScheme;
use num_traits::CheckedSub;
use sequencer_utils::{
    impl_serde_from_string_or_integer, impl_to_fixed_bytes, ser::FromStringOrInteger,
};
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashSet, ops::Add, str::FromStr};
use thiserror::Error;
use vbs::version::Version;

const BLOCK_MERKLE_TREE_HEIGHT: usize = 32;
const FEE_MERKLE_TREE_HEIGHT: usize = 20;

/// This enum is not used in code but functions as an index of
/// possible validation errors.
#[allow(dead_code)]
enum StateValidationError {
    ProposalValidation(ProposalValidationError),
    BuilderValidation(BuilderValidationError),
    Fee(FeeError),
}

#[derive(Hash, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ValidatedState {
    /// Frontier of Block Merkle Tree
    pub block_merkle_tree: BlockMerkleTree,
    /// Fee Merkle Tree
    pub fee_merkle_tree: FeeMerkleTree,
    pub chain_config: ResolvableChainConfig,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Delta {
    pub fees_delta: HashSet<FeeAccount>,
}

/// Possible proposal validation failures
#[derive(Error, Debug, Eq, PartialEq)]
pub enum ProposalValidationError {
    #[error("Invalid ChainConfig: expected={expected}, proposal={proposal}")]
    InvalidChainConfig { expected: String, proposal: String },

    #[error(
        "Invalid Payload Size: (max_block_size={max_block_size}, proposed_block_size={block_size})"
    )]
    MaxBlockSizeExceeded {
        max_block_size: BlockSize,
        block_size: BlockSize,
    },
    #[error("Insufficient Fee: block_size={max_block_size}, base_fee={base_fee}, proposed_fee={proposed_fee}")]
    InsufficientFee {
        max_block_size: BlockSize,
        base_fee: FeeAmount,
        proposed_fee: FeeAmount,
    },
    #[error("Invalid Height: parent_height={parent_height}, proposal_height={proposal_height}")]
    InvalidHeight {
        parent_height: u64,
        proposal_height: u64,
    },
    #[error("Invalid Block Root Error: expected={expected_root}, proposal={proposal_root}")]
    InvalidBlockRoot {
        expected_root: BlockMerkleCommitment,
        proposal_root: BlockMerkleCommitment,
    },
    #[error("Invalid Fee Root Error: expected={expected_root}, proposal={proposal_root}")]
    InvalidFeeRoot {
        expected_root: FeeMerkleCommitment,
        proposal_root: FeeMerkleCommitment,
    },
    #[error("Invalid namespace table: {err}")]
    InvalidNsTable { err: NsTableValidationError },
}

/// Possible charge fee failures
#[derive(Error, Debug, Eq, PartialEq)]
pub enum FeeError {
    #[error("Insuficcient Funds: have {balance:?}, required {amount:?}")]
    InsufficientFunds {
        balance: Option<FeeAmount>,
        amount: FeeAmount,
    },
    #[error("Merkle Tree Error: {0}")]
    MerkleTreeError(MerkleTreeError),
}

/// Possible builder validation failures
#[derive(Error, Debug, Eq, PartialEq)]
pub enum BuilderValidationError {
    #[error("Builder signature not found")]
    SignatureNotFound,
    #[error("Fee amount out of range: {0}")]
    FeeAmountOutOfRange(FeeAmount),
    #[error("Invalid Builder Signature")]
    InvalidBuilderSignature,
}
