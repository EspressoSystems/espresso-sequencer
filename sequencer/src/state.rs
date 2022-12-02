use crate::{Block, ChainVariables, Error, SequencerTransaction};
use commit::{Commitment, Committable};
use hotshot::traits::State as HotShotState;
use hotshot_types::{data::ViewNumber, traits::state::ConsensusTime};
#[allow(deprecated)]
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, ops::Deref};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct State {
    chain_variables: ChainVariables,
    block_height: u64,
    view_number: ViewNumber,
    prev_state_commitment: Option<Commitment<Self>>,
}

impl Default for State {
    fn default() -> Self {
        State {
            chain_variables: Default::default(),
            block_height: Default::default(),
            view_number: ViewNumber::genesis(),
            prev_state_commitment: Default::default(),
        }
    }
}

impl State {
    // TODO: Is there a better name for this?
    fn validate_block_helper(
        &self,
        block: &<State as HotShotState>::BlockType,
        view_number: &<State as HotShotState>::Time,
    ) -> Result<(), Error> {
        // Parent state commitment of block must match current state commitment
        if block.parent_state != self.commit() {
            return Err(Error::IncorrectParent);
        }

        // New view number must be greater than current
        if *view_number <= self.view_number {
            return Err(Error::IncorrectView);
        }

        if self.block_height == 0 {
            // Must contain only a genesis transaction
            if block.transactions.len() != 1 {
                return Err(Error::GenesisWrongSize);
            }
            if !matches!(block.transactions[0], SequencerTransaction::Genesis(_)) {
                return Err(Error::MissingGenesis);
            }
        } else {
            // If any txn is Genesis, fail
            if block
                .transactions
                .iter()
                .any(|txn| matches!(txn, SequencerTransaction::Genesis(_)))
            {
                return Err(Error::UnexpectedGenesis);
            }
        }
        Ok(())
    }
}

impl HotShotState for State {
    type Error = Error;

    type BlockType = Block;

    type Time = ViewNumber;

    fn next_block(&self) -> Self::BlockType {
        Block::new(self.commit())
    }

    fn validate_block(&self, block: &Self::BlockType, view_number: &Self::Time) -> bool {
        self.validate_block_helper(block, view_number).is_ok()
    }

    fn append(
        &self,
        block: &Self::BlockType,
        view_number: &Self::Time,
    ) -> Result<Self, Self::Error> {
        // Have to save state commitment here if any changes are made

        // If there's a validation error, return it here
        self.validate_block_helper(block, view_number)?;

        Ok(State {
            chain_variables: self.chain_variables.clone(),
            block_height: self.block_height + 1,
            view_number: *view_number,
            prev_state_commitment: Some(self.commit()),
        })
    }

    fn on_commit(&self) {}
}

impl Committable for State {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("State")
            .field("chain_variables", self.chain_variables.commit())
            .u64_field("block_height", self.block_height)
            .u64_field("view_number", *self.view_number.deref())
            .array_field(
                "prev_state_commitment",
                &self
                    .prev_state_commitment
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>(),
            )
            .finalize()
    }
}
