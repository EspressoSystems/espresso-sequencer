extern crate derive_more;
use commit::Commitment;
use derive_more::Into;
use hotshot_query_service::availability::BlockQueryData;
use sequencer::SeqTypes;

use crate::state::State;

/// A mock proof that state_commitment represents a valid state transition from
/// previous_state_commitment when the transactions in a given block are applied.
#[derive(Debug, Clone, Into)]
pub(crate) struct Proof(Vec<u8>);

impl Proof {
    pub fn generate(
        _block: &BlockQueryData<SeqTypes>,
        _state_commitment: Commitment<State>,
        _previous_state_commitment: Commitment<State>,
    ) -> Self {
        Self(vec![0; 32])
    }
}

/// A mock proof aggregating a batch of proofs for a range of blocks.
#[derive(Debug, Clone, Into)]
pub(crate) struct BatchProof(Vec<u8>);

impl BatchProof {
    pub fn generate(_proofs: impl IntoIterator<Item = Proof>) -> BatchProof {
        BatchProof(vec![0; 32])
    }
}
