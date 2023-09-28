//! Sequencing consensus demo
//!
//! This module provides an implementation of the `HotShot` suite of traits that implements a
//! basic demonstration of sequencing consensus.
//!
//! These implementations are useful in examples and integration testing, but are not suitable for
//! production use.
use crate::traits::election::static_committee::{StaticElectionConfig, StaticVoteToken};
use commit::{Commitment, Committable};
use derivative::Derivative;
use either::Either;
use hotshot_signature_key::bn254::BLSPubKey;
use hotshot_types::{
    block_impl::{BlockPayloadError, VIDBlockPayload, VIDTransaction},
    certificate::{AssembledSignature, QuorumCertificate},
    data::{
        fake_commitment, genesis_proposer_id, random_commitment, LeafType, SequencingLeaf,
        ViewNumber,
    },
    traits::{
        election::Membership,
        node_implementation::NodeType,
        state::{ConsensusTime, TestableState},
        BlockPayload, State,
    },
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};

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

impl State for SDemoState {
    type Error = BlockPayloadError;

    type BlockType = VIDBlockPayload;

    type Time = ViewNumber;

    fn validate_block(&self, _block: &Self::BlockType, view_number: &Self::Time) -> bool {
        if view_number == &ViewNumber::genesis() {
            &self.view_number == view_number
        } else {
            self.view_number < *view_number
        }
    }

    fn append(
        &self,
        block: &Self::BlockType,
        view_number: &Self::Time,
    ) -> Result<Self, Self::Error> {
        if !self.validate_block(block, view_number) {
            return Err(BlockPayloadError::InvalidBlock);
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
        /// clippy appeasement for `RANDOM_TX_BASE_SIZE`
        const RANDOM_TX_BASE_SIZE: usize = 8;
        let mut bytes = vec![0; RANDOM_TX_BASE_SIZE + (padding as usize)];
        rng.fill_bytes(&mut bytes);
        VIDTransaction(bytes)
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
pub struct DemoTypes;

impl NodeType for DemoTypes {
    type Time = ViewNumber;
    type BlockType = VIDBlockPayload;
    type SignatureKey = BLSPubKey;
    type VoteTokenType = StaticVoteToken<Self::SignatureKey>;
    type Transaction = VIDTransaction;
    type ElectionConfigType = StaticElectionConfig;
    type StateType = SDemoState;
}

/// The node implementation for the sequencing demo
#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct SDemoNode<MEMBERSHIP>(PhantomData<MEMBERSHIP>)
where
    MEMBERSHIP: Membership<DemoTypes> + std::fmt::Debug;

impl<MEMBERSHIP> SDemoNode<MEMBERSHIP>
where
    MEMBERSHIP: Membership<DemoTypes> + std::fmt::Debug,
{
    /// Create a new `SDemoNode`
    #[must_use]
    pub fn new() -> Self {
        SDemoNode(PhantomData)
    }
}

impl<MEMBERSHIP> Debug for SDemoNode<MEMBERSHIP>
where
    MEMBERSHIP: Membership<DemoTypes> + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SDemoNode")
            .field("_phantom", &"phantom")
            .finish()
    }
}

impl<MEMBERSHIP> Default for SDemoNode<MEMBERSHIP>
where
    MEMBERSHIP: Membership<DemoTypes> + std::fmt::Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Provides a random [`QuorumCertificate`]
pub fn random_quorum_certificate<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>>(
    rng: &mut dyn rand::RngCore,
) -> QuorumCertificate<TYPES, Commitment<LEAF>> {
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
