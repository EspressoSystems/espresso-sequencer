use core::fmt::Debug;

use jf_merkle_tree::MerkleTreeError;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use thiserror::Error;

use super::{BlockSize, FeeAmount, NsTableValidationError};
use crate::{BlockMerkleCommitment, FeeMerkleCommitment};

#[derive(Clone, Debug, Snafu, Deserialize, Serialize)]
pub enum Error {
    // TODO: Can we nest these errors in a `ValidationError` to group them?

    // Parent state commitment of block doesn't match current state commitment
    IncorrectParent,

    // New view number isn't strictly after current view
    IncorrectView,

    // Genesis block either has zero or more than one transaction
    GenesisWrongSize,

    // Genesis transaction not present in genesis block
    MissingGenesis,

    // Genesis transaction in non-genesis block
    UnexpectedGenesis,

    // Merkle tree error
    MerkleTreeError { error: String },

    BlockBuilding,
}

/// This enum is not used in code but functions as an index of
/// possible validation errors.
#[allow(dead_code)]
pub enum StateValidationError {
    ProposalValidation(ProposalValidationError),
    BuilderValidation(BuilderValidationError),
    Fee(FeeError),
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
