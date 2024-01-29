use crate::{
    block::{BlockMerkleTree, FeeAccount, FeeAmount, FeeMerkleTree, _validate_proposal, ValidatedState},
    Error, Header, Payload,
};
use commit::{Commitment, Committable};
use hotshot::traits::State as HotShotState;
use hotshot_types::data::{ViewNumber, BlockError};
use jf_primitives::merkle_tree::{MerkleTreeScheme, UniversalMerkleTreeScheme};

impl Default for ValidatedState {
    fn default() -> Self {
        let block_merkle_tree =
            BlockMerkleTree::from_elems(32, Vec::<Commitment<Header>>::new()).unwrap();
        let fee_merkle_tree =
            FeeMerkleTree::from_kv_set(20, Vec::<(FeeAccount, FeeAmount)>::new()).unwrap();
        Self {
            block_merkle_tree,
            fee_merkle_tree,
        }
    }
}

impl HotShotState for ValidatedState {
    type Error = BlockError;

    type BlockHeader = Header;
    type BlockPayload = Payload;

    type Time = ViewNumber;

    fn on_commit(&self) {}

    fn validate_and_apply_header(
        &self,
        proposed_header: &Self::BlockHeader,
        parent_header: &Self::BlockHeader,
        _view_number: &Self::Time,
    ) -> Result<Self, Self::Error> {

         // TODO validate fee state
         if let Err(_) = _validate_proposal(parent_header, proposed_header) {
             Err(BlockError::InvalidBlockHeader)
         } else {
             Ok(proposed_header.validated_state)
         }
    }
}

// Required for TestableState
#[cfg(any(test, feature = "testing"))]
impl std::fmt::Display for ValidatedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[cfg(any(test, feature = "testing"))]
impl hotshot_types::traits::state::TestableState for ValidatedState {
    fn create_random_transaction(
        _state: Option<&Self>,
        rng: &mut dyn rand::RngCore,
        _padding: u64,
    ) -> crate::Transaction {
        crate::Transaction::random(rng)
    }
}

impl Committable for ValidatedState {
    fn commit(&self) -> Commitment<Self> {
        let mut bmt_bytes = vec![];
        self.block_merkle_tree
            .serialize_with_mode(&mut bmt_bytes, ark_serialize::Compress::Yes)
            .unwrap();
        let mut fmt_bytes = vec![];
        self.fee_merkle_tree
            .serialize_with_mode(&mut fmt_bytes, ark_serialize::Compress::Yes)
            .unwrap();
        commit::RawCommitmentBuilder::new("Test State Commit")
            .var_size_field("block_merkle_tree", &bmt_bytes)
            .var_size_field("fee_merkle_tree", &fmt_bytes)
            .finalize()
    }
}
