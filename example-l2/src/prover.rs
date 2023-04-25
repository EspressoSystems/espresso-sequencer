extern crate derive_more;
use commit::Commitment;
use derive_more::Into;
use hotshot_query_service::availability::BlockQueryData;
use sequencer::SeqTypes;

use crate::state::State;

/// A mock proof that state_commitment represents a valid state transition from
/// previous_state_commitment when the transactions committed to by block_commitment are applied.
#[derive(Debug, Clone, Into)]
pub(crate) struct Proof(Vec<u8>);

impl Proof {
    pub fn generate(
        _block: &BlockQueryData<SeqTypes>, // A real prover would use the block commitment and transaction set to construct the state transition proof
        _state_commitment: Commitment<State>,
        _previous_state_commitment: Commitment<State>,
    ) -> Self {
        Self(vec![0; 32])
    }
}
