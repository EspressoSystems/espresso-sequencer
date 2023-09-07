//! Sequencing consensus demo
//!
//! This module provides an implementation of the `HotShot` suite of traits that implements a
//! basic demonstration of sequencing consensus.
//!
//! These implementations are useful in examples and integration testing, but are not suitable for
//! production use.
use crate::traits::election::static_committee::{StaticElectionConfig, StaticVoteToken};
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::Deref,
};

use commit::{Commitment, Committable};
use derivative::Derivative;
use either::Either;
use hotshot_signature_key::bn254::BN254Pub;
use hotshot_types::{
    certificate::{AssembledSignature, QuorumCertificate},
    constants::genesis_proposer_id,
    data::{fake_commitment, random_commitment, LeafType, SequencingLeaf, ViewNumber},
    traits::{
        block_contents::Transaction,
        election::Membership,
        node_implementation::NodeType,
        state::{ConsensusTime, TestableBlock, TestableState},
        BlockPayload, State,
    },
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

/// The transaction for the sequencing demo
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct SDemoTransaction {
    /// identifier for the transaction
    pub id: u64,
    /// padding to add to txn (to make it larger and thereby more realistic)
    pub padding: Vec<u8>,
}

impl Deref for SDemoTransaction {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl Committable for SDemoTransaction {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("SDemo Txn Comm")
            .u64_field("id", self.id)
            .finalize()
    }

    fn tag() -> String {
        "SEQUENCING_DEMO_TXN".to_string()
    }
}

impl Transaction for SDemoTransaction {}

impl SDemoTransaction {
    /// create a new transaction
    #[must_use]
    pub fn new(id: u64) -> Self {
        Self {
            id,
            padding: vec![],
        }
    }
}

/// genesis block
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct SDemoGenesisBlock {}

/// Any block after genesis
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct SDemoNormalBlock {
    /// BlockPayload state commitment
    pub previous_state: (),
    /// Transaction vector
    pub transactions: Vec<SDemoTransaction>,
}

/// The block for the sequencing demo
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub enum SDemoBlock {
    /// genesis block
    Genesis(SDemoGenesisBlock),
    /// normal block
    Normal(SDemoNormalBlock),
}

impl Committable for SDemoBlock {
    fn commit(&self) -> Commitment<Self> {
        match &self {
            SDemoBlock::Genesis(_) => {
                commit::RawCommitmentBuilder::new("SDemo Genesis Comm").finalize()
            }
            SDemoBlock::Normal(block) => {
                let mut builder = commit::RawCommitmentBuilder::new("SDemo Normal Comm");
                for txn in &block.transactions {
                    builder = builder.u64_field("transaction", **txn);
                }
                builder.finalize()
            }
        }
    }

    fn tag() -> String {
        "SEQUENCING_DEMO_BLOCK".to_string()
    }
}

/// sequencing demo entry state
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct SDemoState {
    /// the block height
    block_height: u64,
    /// the view number
    view_number: ViewNumber,
    /// the previous state commitment
    prev_state_commitment: Commitment<Self>,
}

impl Committable for SDemoState {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("SDemo State Commit")
            .u64_field("block_height", self.block_height)
            .u64_field("view_number", *self.view_number)
            .field("prev_state_commitment", self.prev_state_commitment)
            .finalize()
    }

    fn tag() -> String {
        "SEQUENCING_DEMO_STATE".to_string()
    }
}

impl Default for SDemoState {
    fn default() -> Self {
        Self {
            block_height: 0,
            view_number: ViewNumber::genesis(),
            prev_state_commitment: fake_commitment(),
        }
    }
}

/// The error type for the sequencing demo
#[derive(Snafu, Debug)]
pub enum SDemoError {
    /// Previous state commitment does not match
    PreviousStateMismatch,
    /// Nonce was reused
    ReusedTxn,
    /// Genesis failure
    GenesisFailed,
    /// Genesis reencountered after initialization
    GenesisAfterStart,
    /// no transasctions added to genesis
    GenesisCantHaveTransactions,
    /// invalid block
    InvalidBlock,
}

impl Display for SDemoBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SDemoBlock::Genesis(_) => {
                write!(f, "SDemo Genesis BlockPayload")
            }
            SDemoBlock::Normal(block) => {
                write!(
                    f,
                    "SDemo Normal BlockPayload #txns={}",
                    block.transactions.len()
                )
            }
        }
    }
}

impl TestableBlock for SDemoBlock {
    fn genesis() -> Self {
        SDemoBlock::Genesis(SDemoGenesisBlock {})
    }

    fn txn_count(&self) -> u64 {
        match self {
            SDemoBlock::Genesis(_) => 0,
            SDemoBlock::Normal(n) => n.transactions.len() as u64,
        }
    }
}

impl BlockPayload for SDemoBlock {
    type Error = SDemoError;

    type Transaction = SDemoTransaction;

    fn new() -> Self {
        <Self as TestableBlock>::genesis()
    }

    fn add_transaction_raw(
        &self,
        tx: &Self::Transaction,
    ) -> std::result::Result<Self, Self::Error> {
        match self {
            SDemoBlock::Genesis(_) => Err(SDemoError::GenesisCantHaveTransactions),
            SDemoBlock::Normal(n) => {
                let mut new = n.clone();
                new.transactions.push(tx.clone());
                Ok(SDemoBlock::Normal(new))
            }
        }
    }

    fn contained_transactions(&self) -> HashSet<Commitment<Self::Transaction>> {
        match self {
            SDemoBlock::Genesis(_) => HashSet::new(),
            SDemoBlock::Normal(n) => n
                .transactions
                .iter()
                .map(commit::Committable::commit)
                .collect(),
        }
    }

    fn commitment(&self) -> Commitment<Self> {
        // TODO: Get the payload commitment after VID is added.
        // https://github.com/EspressoSystems/HotShot/issues/1673
        unimplemented!();
    }
}

impl State for SDemoState {
    type Error = SDemoError;

    type BlockType = SDemoBlock;

    type Time = ViewNumber;

    fn next_block(_state: Option<Self>) -> Self::BlockType {
        SDemoBlock::Normal(SDemoNormalBlock {
            previous_state: (),
            transactions: Vec::new(),
        })
    }

    fn validate_block(&self, block: &Self::BlockType, view_number: &Self::Time) -> bool {
        match block {
            SDemoBlock::Genesis(_) => {
                view_number == &ViewNumber::genesis() && view_number == &self.view_number
            }
            SDemoBlock::Normal(_n) => self.view_number < *view_number,
        }
    }

    fn append(
        &self,
        block: &Self::BlockType,
        view_number: &Self::Time,
    ) -> Result<Self, Self::Error> {
        if !self.validate_block(block, view_number) {
            return Err(SDemoError::InvalidBlock);
        }

        Ok(SDemoState {
            block_height: self.block_height + 1,
            view_number: *view_number,
            prev_state_commitment: self.commit(),
        })
    }

    fn on_commit(&self) {}
}

impl TestableState for SDemoState {
    fn create_random_transaction(
        _state: Option<&Self>,
        rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <Self::BlockType as BlockPayload>::Transaction {
        SDemoTransaction {
            id: rng.gen_range(0..10),
            padding: vec![0; padding as usize],
        }
    }
}
/// Implementation of [`NodeType`] for [`VDemoNode`]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct SDemoTypes;

impl NodeType for SDemoTypes {
    type Time = ViewNumber;
    type BlockType = SDemoBlock;
    type SignatureKey = BN254Pub;
    type VoteTokenType = StaticVoteToken<Self::SignatureKey>;
    type Transaction = SDemoTransaction;
    type ElectionConfigType = StaticElectionConfig;
    type StateType = SDemoState;
}

/// The node implementation for the sequencing demo
#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct SDemoNode<MEMBERSHIP>(PhantomData<MEMBERSHIP>)
where
    MEMBERSHIP: Membership<SDemoTypes> + std::fmt::Debug;

impl<MEMBERSHIP> SDemoNode<MEMBERSHIP>
where
    MEMBERSHIP: Membership<SDemoTypes> + std::fmt::Debug,
{
    /// Create a new `SDemoNode`
    #[must_use]
    pub fn new() -> Self {
        SDemoNode(PhantomData)
    }
}

impl<MEMBERSHIP> Debug for SDemoNode<MEMBERSHIP>
where
    MEMBERSHIP: Membership<SDemoTypes> + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SDemoNode")
            .field("_phantom", &"phantom")
            .finish()
    }
}

impl<MEMBERSHIP> Default for SDemoNode<MEMBERSHIP>
where
    MEMBERSHIP: Membership<SDemoTypes> + std::fmt::Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Provides a random [`QuorumCertificate`]
pub fn random_quorum_certificate<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>>(
    rng: &mut dyn rand::RngCore,
) -> QuorumCertificate<TYPES, LEAF> {
    QuorumCertificate {
        // block_commitment: random_commitment(rng),
        leaf_commitment: random_commitment(rng),
        view_number: TYPES::Time::new(rng.gen()),
        signatures: AssembledSignature::Genesis(),
        is_genesis: rng.gen(),
    }
}

/// Provides a random [`SequencingLeaf`]
pub fn random_sequencing_leaf<TYPES: NodeType>(
    deltas: Either<TYPES::BlockType, Commitment<TYPES::BlockType>>,
    rng: &mut dyn rand::RngCore,
) -> SequencingLeaf<TYPES> {
    let justify_qc = random_quorum_certificate(rng);
    // let state = TYPES::StateType::default()
    //     .append(&deltas, &TYPES::Time::new(42))
    //     .unwrap_or_default();
    SequencingLeaf {
        view_number: justify_qc.view_number,
        height: rng.next_u64(),
        justify_qc,
        parent_commitment: random_commitment(rng),
        deltas,
        rejected: Vec::new(),
        timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
        proposer_id: genesis_proposer_id(),
    }
}
