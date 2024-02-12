//! Implementations for examples and tests only
use commit::{Commitment, Committable};

use hotshot_types::{
    data::{fake_commitment, BlockError, ViewNumber},
    traits::{
        states::{InstanceState, TestableState, ValidatedState},
        BlockPayload,
    },
};

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::block_types::TestTransaction;
use crate::block_types::{TestBlockHeader, TestBlockPayload};
pub use crate::node_types::TestTypes;

/// Instance-level state implementation for testing purposes.
#[derive(Clone, Debug)]
pub struct TestInstanceState {}

impl InstanceState for TestInstanceState {}

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
        commit::RawCommitmentBuilder::new("Test State Commit")
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

impl ValidatedState for TestValidatedState {
    type Error = BlockError;

    type Instance = TestInstanceState;

    type BlockHeader = TestBlockHeader;

    type BlockPayload = TestBlockPayload;

    type Time = ViewNumber;

    fn validate_and_apply_header(
        &self,
        _instance: &Self::Instance,
        _parent_header: &Self::BlockHeader,
        _proposed_header: &Self::BlockHeader,
    ) -> Result<Self, Self::Error> {
        Ok(TestValidatedState {
            block_height: self.block_height + 1,
            prev_state_commitment: self.commit(),
        })
    }

    fn from_header(block_header: &Self::BlockHeader) -> Self {
        Self {
            block_height: block_header.block_number,
            ..Default::default()
        }
    }

    fn on_commit(&self) {}
}

impl TestableState for TestValidatedState {
    fn create_random_transaction(
        _state: Option<&Self>,
        _rng: &mut dyn rand::RngCore,
        padding: u64,
    ) -> <Self::BlockPayload as BlockPayload>::Transaction {
        /// clippy appeasement for `RANDOM_TX_BASE_SIZE`
        const RANDOM_TX_BASE_SIZE: usize = 8;
        TestTransaction(vec![
            0;
            RANDOM_TX_BASE_SIZE + usize::try_from(padding).unwrap()
        ])
    }
}
