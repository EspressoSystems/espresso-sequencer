use crate::{
    chain_variables::ChainVariables,
    l1_client::{L1Client, L1ClientOptions, L1Snapshot},
    Error, Header, Payload,
};
use async_std::task::{block_on, sleep};
use commit::{Commitment, Committable};
use ethers::prelude::BlockNumber;
use hotshot::traits::State as HotShotState;
use hotshot_types::data::ViewNumber;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Debug;
use std::time::Duration;

#[derive(Default, Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct State {
    chain_variables: ChainVariables,
    ethereum_block_tag: BlockNumber,
}

impl HotShotState for State {
    type Error = Error;

    type BlockHeader = Header;
    type BlockPayload = Payload;

    type Time = ViewNumber;

    fn validate_block(&self, _block: &Self::BlockType, _view_number: &Self::Time) -> bool {
        unimplemented!("Using sequencing consensus, no validation")
    }

    fn initialize() -> Self {
        Default::default()
    }

    // This function is called exactly once, with the first block.
    fn append(
        &self,
        _block: &Self::BlockType,
        _view_number: &Self::Time,
    ) -> Result<Self, Self::Error> {
        Ok(self.clone())
    }

    fn on_commit(&self) {}
}

// Required for TestableState
#[cfg(any(test, feature = "testing"))]
impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[cfg(any(test, feature = "testing"))]
impl hotshot_types::traits::state::TestableState for State {
    fn create_random_transaction(
        _state: Option<&Self>,
        rng: &mut dyn rand::RngCore,
        _padding: u64,
    ) -> <Self::BlockType as hotshot::traits::Block>::Transaction {
        crate::Transaction::random(rng)
    }
}

impl Committable for State {
    fn commit(&self) -> Commitment<Self> {
        unimplemented!("Not used in sequencing consensus")
    }
}
