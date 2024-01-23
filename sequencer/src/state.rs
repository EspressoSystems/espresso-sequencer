use crate::{chain_variables::ChainVariables, Error, Header, Payload};
use commit::{Commitment, Committable};
use ethers::prelude::BlockNumber;
use hotshot::traits::State as HotShotState;
use hotshot_types::data::ViewNumber;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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

    // fn validate_block(&self, _header: &Self::BlockHeader, _view_number: &Self::Time) -> bool {
    //     unimplemented!("Using sequencing consensus, no validation")
    // }

    // fn initialize() -> Self {
    //     Default::default()
    // }

    // // This function is called exactly once, with the first block.
    // fn append(
    //     &self,
    //     _header: &Self::BlockHeader,
    //     _view_number: &Self::Time,
    // ) -> Result<Self, Self::Error> {
    //     Ok(self.clone())
    // }

    fn on_commit(&self) {}

    fn validate_and_apply_header(
        &self,
        proposed_header: &Self::BlockHeader,
        parent_header: &Self::BlockHeader,
        view_number: &Self::Time,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
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
    ) -> crate::Transaction {
        crate::Transaction::random(rng)
    }
}

impl Committable for State {
    fn commit(&self) -> Commitment<Self> {
        unimplemented!("Not used in sequencing consensus")
    }
}
