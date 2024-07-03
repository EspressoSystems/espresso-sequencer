use crate::ResolvableChainConfig;

use super::{FeeAccount, FeeAmount};

use jf_merkle_tree::{
    prelude::{LightWeightSHA3MerkleTree, Sha3Digest, Sha3Node},
    universal_merkle_tree::UniversalMerkleTree,
    MerkleTreeScheme,
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

pub const BLOCK_MERKLE_TREE_HEIGHT: usize = 32;
pub const FEE_MERKLE_TREE_HEIGHT: usize = 20;

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
