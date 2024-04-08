//! Implementations for examples and tests only
use committable::{Commitment, Committable};

use hotshot_types::{
    data::{fake_commitment, BlockError, Leaf, ViewNumber},
    traits::{
        block_contents::BlockHeader,
        node_implementation::NodeType,
        states::{InstanceState, StateDelta, TestableState, ValidatedState},
        BlockPayload,
    },
};

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::block_types::{TestBlockPayload, TestTransaction};
pub use crate::node_types::TestTypes;

/// Instance-level state implementation for testing purposes.
#[derive(Clone, Copy, Debug, Default)]
pub struct TestInstanceState {}

impl InstanceState for TestInstanceState {}

/// Application-specific state delta implementation for testing purposes.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct TestStateDelta {}

impl StateDelta for TestStateDelta {}

/// Validated state implementation for testing purposes.
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct TestValidatedState {
    /// the block height
    block_height: u64,
    /// the previous state commitment
    prev_state_commitment: Commitment<Self>,
}

impl Committable for TestValidatedState {
    fn commit(&self) -> Commitment<Self> {
        committable::RawCommitmentBuilder::new("Test State Commit")
            .u64_field("block_height", self.block_height)
            .field("prev_state_commitment", self.prev_state_commitment)
            .finalize()
    }

    fn tag() -> String {
        "TEST_STATE".to_string()
    }
}

impl Default for TestValidatedState {
    fn default() -> Self {
        Self {
            block_height: 0,
            prev_state_commitment: fake_commitment(),
        }
    }
}

impl<TYPES: NodeType> ValidatedState<TYPES> for TestValidatedState {
    type Error = BlockError;

    type Instance = TestInstanceState;

    type Delta = TestStateDelta;

    type Time = ViewNumber;

    async fn validate_and_apply_header(
        &self,
        _instance: &Self::Instance,
        _parent_leaf: &Leaf<TYPES>,
        _proposed_header: &TYPES::BlockHeader,
    ) -> Result<(Self, Self::Delta), Self::Error> {
        Ok((
            TestValidatedState {
                block_height: self.block_height + 1,
                prev_state_commitment: self.commit(),
            },
            TestStateDelta {},
        ))
    }

    fn from_header(block_header: &TYPES::BlockHeader) -> Self {
        Self {
            block_height: block_header.block_number(),
            ..Default::default()
        }
    }

    fn on_commit(&self) {}

    fn genesis(_instance: &Self::Instance) -> (Self, Self::Delta) {
        (Self::default(), TestStateDelta {})
    }
}

impl<TYPES: NodeType<BlockPayload = TestBlockPayload>> TestableState<TYPES> for TestValidatedState {
    fn create_random_transaction(
        _state: Option<&Self>,
        _rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <TYPES::BlockPayload as BlockPayload>::Transaction {
        /// clippy appeasement for `RANDOM_TX_BASE_SIZE`
        const RANDOM_TX_BASE_SIZE: usize = 8;
        TestTransaction(vec![
            0;
            RANDOM_TX_BASE_SIZE + usize::try_from(padding).unwrap()
        ])
    }
}
