use crate::{block::Block, chain_variables::ChainVariables, Error, Transaction};
use commit::{Commitment, Committable};
use hotshot::traits::State as HotShotState;
use hotshot_types::{
    data::ViewNumber,
    traits::{state::TestableState, Block as _},
};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

#[derive(Default, Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct State {
    chain_variables: ChainVariables,
}

impl HotShotState for State {
    type Error = Error;

    type BlockType = Block;

    type Time = ViewNumber;

    fn next_block(prev_state: Option<Self>) -> Self::BlockType {
        match prev_state {
            Some(_state) => unimplemented!("Using sequencing consensus, no previous state"),
            None => Block::new(),
        }
    }

    fn validate_block(&self, _block: &Self::BlockType, _view_number: &Self::Time) -> bool {
        unimplemented!("Using sequencing consensus, no validation")
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
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[cfg(any(test, feature = "testing"))]
impl TestableState for State {
    fn create_random_transaction(
        _state: Option<&Self>,
        rng: &mut dyn rand::RngCore,
        _padding: u64,
    ) -> <Self::BlockType as hotshot::traits::Block>::Transaction {
        Transaction::random(rng)
    }
}

impl Committable for State {
    fn commit(&self) -> Commitment<Self> {
        unimplemented!("Not used in sequencing consensus")
    }
}
