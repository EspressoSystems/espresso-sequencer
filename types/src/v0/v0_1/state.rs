use super::{FeeAccount, FeeAmount};
use crate::Header;
use committable::Commitment;
use derive_more::derive::{Add, Display, From, Into, Mul, Sub};
use ethers::types::{Address, U256};
use jf_merkle_tree::{
    prelude::{LightWeightSHA3MerkleTree, Sha3Digest, Sha3Node},
    universal_merkle_tree::UniversalMerkleTree,
    MerkleTreeScheme,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Delta {
    pub fees_delta: HashSet<FeeAccount>,
}

pub const BLOCK_MERKLE_TREE_HEIGHT: usize = 32;
pub const FEE_MERKLE_TREE_HEIGHT: usize = 20;
pub const REWARD_MERKLE_TREE_HEIGHT: usize = 20;
const FEE_MERKLE_TREE_ARITY: usize = 256;
const REWARD_MERKLE_TREE_ARITY: usize = 256;

// The block merkle tree accumulates header commitments. However, since the underlying
// representation of the commitment type remains the same even while the header itself changes,
// using the underlying type `[u8; 32]` allows us to use the same state type across minor versions.
pub type BlockMerkleTree = LightWeightSHA3MerkleTree<Commitment<Header>>;
pub type BlockMerkleCommitment = <BlockMerkleTree as MerkleTreeScheme>::Commitment;

pub type FeeMerkleTree =
    UniversalMerkleTree<FeeAmount, Sha3Digest, FeeAccount, FEE_MERKLE_TREE_ARITY, Sha3Node>;
pub type FeeMerkleCommitment = <FeeMerkleTree as MerkleTreeScheme>::Commitment;

pub type RewardMerkleTree = UniversalMerkleTree<
    RewardAmount,
    Sha3Digest,
    RewardAccount,
    REWARD_MERKLE_TREE_ARITY,
    Sha3Node,
>;
pub type RewardMerkleCommitment = <RewardMerkleTree as MerkleTreeScheme>::Commitment;

// New Type for `Address` in order to implement `CanonicalSerialize` and
// `CanonicalDeserialize`
#[derive(
    Default,
    Hash,
    Copy,
    Clone,
    Debug,
    Display,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    From,
    Into,
)]
#[display("{_0:x}")]
pub struct RewardAccount(pub Address);

// New Type for `U256` in order to implement `CanonicalSerialize` and
// `CanonicalDeserialize`
#[derive(
    Default,
    Hash,
    Copy,
    Clone,
    Debug,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Add,
    Sub,
    Mul,
    From,
    Into,
)]
#[display("{_0}")]
pub struct RewardAmount(pub U256);

fn block_reward(_block_height: u64) -> RewardAmount {
    // rewards per year
    let reward: u64 = ANNUAL_INFLATION / BLOCKS_PER_YEAR;
    U256::from(reward).into()
}

pub const SUPPLY: u64 = 1000000000;
pub const INFLATION: u64 = 300;
pub const ANNUAL_INFLATION: u64 = SUPPLY * INFLATION / 100;
pub const BLOCKS_PER_YEAR: u64 = 365 * 24 * 60 * 60 / 2;
