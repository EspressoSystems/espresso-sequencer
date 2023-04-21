use commit::Commitment;
use hotshot_query_service::availability::BlockHash;
use sequencer::SeqTypes;

use crate::state::State;

/// A mock proof that state_commitment represents a valid state transition from
/// previous_state_commitment when the transactions committed to by block_commitment are applied.
#[derive(Debug, Clone)]
pub(crate) struct Proof(pub Vec<u8>);

impl Proof {
    pub fn generate(
        _block_commitment: BlockHash<SeqTypes>,
        _state_commitment: Commitment<State>,
        _previous_state_commitment: Commitment<State>,
    ) -> Self {
        Self(vec![0; 32])
    }
}
