use crate::ResolvableChainConfig;

use super::{BlockSize, FeeAccount, FeeAmount, NsTableValidationError};

use jf_merkle_tree::{
    prelude::{LightWeightSHA3MerkleTree, Sha3Digest, Sha3Node},
    universal_merkle_tree::UniversalMerkleTree,
    MerkleTreeError, MerkleTreeScheme,
};
use serde::{Deserialize, Serialize};

// The block merkle tree accumulates header commitments. However, since the underlying
// representation of the commitment type remains the same even while the header itself changes,
// using the underlying type `[u8; 32]` allows us to use the same state type across minor versions.
pub type BlockMerkleTree = LightWeightSHA3MerkleTree<[u8; 32]>;
pub type BlockMerkleCommitment = <BlockMerkleTree as MerkleTreeScheme>::Commitment;

pub type FeeMerkleTree = UniversalMerkleTree<FeeAmount, Sha3Digest, FeeAccount, 256, Sha3Node>;
pub type FeeMerkleCommitment = <FeeMerkleTree as MerkleTreeScheme>::Commitment;

use core::fmt::Debug;

use std::collections::HashSet;
use thiserror::Error;

pub const BLOCK_MERKLE_TREE_HEIGHT: usize = 32;
pub const FEE_MERKLE_TREE_HEIGHT: usize = 20;

/// This enum is not used in code but functions as an index of
/// possible validation errors.
#[allow(dead_code)]
pub enum StateValidationError {
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
    #[error("Total fee amount out of range")]
    TotalFeeAmountOutOfRange,
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
