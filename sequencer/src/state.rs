use crate::{
    block::{
        BlockMerkleCommitment, BlockMerkleTree, FeeAccount, FeeAmount, FeeMerkleCommitment,
        FeeMerkleTree, ValidatedState, _validate_proposal,
    },
    Error, Header, Payload,
};
use commit::{Commitment, Committable};
use hotshot::traits::State as HotShotState;
use hotshot_types::data::{BlockError, ViewNumber};
use jf_primitives::merkle_tree::{
    AppendableMerkleTreeScheme, MerkleTreeScheme, UniversalMerkleTreeScheme,
};

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

impl ValidatedState {
    fn validate_header(&self, parent_header: &Header, proposed_header: &Header) -> Option<Header> {
        if let Err(_) = _validate_proposal(parent_header, proposed_header) {
            return None;
        };
        Some(proposed_header.clone())
    }
}

impl HotShotState for ValidatedState {
    type Error = BlockError;

    type BlockHeader = Header;
    type BlockPayload = Payload;

    type Time = ViewNumber;

    fn on_commit(&self) {}
    /// Validate parent against known values (from state) and validate
    /// proposal descends from parent. Returns updated `ValidatedState`.
    fn validate_and_apply_header(
        &self,
        proposed_header: &Self::BlockHeader,
        parent_header: &Self::BlockHeader,
        _view_number: &Self::Time,
    ) -> Result<Self, Self::Error> {
        // validate proposed fee merkle tree root against state
        let fee_merkle_tree = self.fee_merkle_tree;
        if parent_header.fee_merkle_tree_root != self.fee_merkle_tree.commitment() {
            return Err(BlockError::InvalidBlockHeader);
        };
        // validate proposed block merkle tree root against state
        if parent_header.fee_merkle_tree_root != self.fee_merkle_tree.commitment() {
            return Err(BlockError::InvalidBlockHeader);
        };
        // TODO update fee_merkle_tree
        // validate that proposed header is a descendent of parent and update
        if let Ok(block_merkle_tree) = _validate_proposal(parent_header, proposed_header) {
            Ok(ValidatedState {
                block_merkle_tree,
                fee_merkle_tree,
            })
        } else {
            Err(BlockError::InvalidBlockHeader)
        }
    }
}

// FIXME remove when `Commitable` is removed from the trait
impl Committable for ValidatedState {
    fn commit(&self) -> Commitment<Self> {
        unimplemented!("temporary implementation");
    }

    fn tag() -> String {
        "VALIDATED_STATE".to_string()
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
