use crate::{
    block::{BlockMerkleTree, FeeAccount, FeeAmount, FeeMerkleTree},
    Header, Payload,
};
use commit::{Commitment, Committable};
use hotshot::traits::State as HotShotState;
use hotshot_types::data::{BlockError, ViewNumber};
use jf_primitives::merkle_tree::{
    AppendableMerkleTreeScheme, MerkleTreeScheme, UniversalMerkleTreeScheme,
};
use serde::{Deserialize, Serialize};

#[derive(Hash, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ValidatedState {
    /// Frontier of Block Merkle Tree
    pub block_merkle_tree: BlockMerkleTree,
    /// Fee Merkle Tree
    pub fee_merkle_tree: FeeMerkleTree,
}

impl Default for ValidatedState {
    fn default() -> Self {
        let block_merkle_tree =
            BlockMerkleTree::from_elems(32, Vec::<Commitment<Header>>::new()).unwrap();

        // Words of wisdom from @mrain: "capacity = arity^height"
        // "For index space 2^160, arity 256 (2^8),
        // you should set the height as 160/8=20"
        let fee_merkle_tree =
            FeeMerkleTree::from_kv_set(20, Vec::<(FeeAccount, FeeAmount)>::new()).unwrap();
        Self {
            block_merkle_tree,
            fee_merkle_tree,
        }
    }
}

impl ValidatedState {
    fn validate_proposal(&self, parent: &Header, proposal: &Header) -> anyhow::Result<Self> {
        // validate height
        anyhow::ensure!(
            proposal.height == parent.height + 1,
            anyhow::anyhow!(
                "Invalid Height Error: {}, {}",
                parent.height,
                proposal.height
            )
        );

        // validate parent fee merkle tree root against state
        let fee_merkle_tree = self.fee_merkle_tree;
        let fee_merkle_tree_root = fee_merkle_tree.commitment();
        anyhow::ensure!(
            parent.fee_merkle_tree_root == fee_merkle_tree_root,
            anyhow::anyhow!(
                "Invalid Fee Merkle Tree Error: {}, {}",
                parent.fee_merkle_tree_root,
                fee_merkle_tree_root
            )
        );

        let mut block_merkle_tree = self.block_merkle_tree.clone();
        let block_merkle_tree_root = block_merkle_tree.commitment();

        // validate proposal is descendent of parent by appending to parent
        block_merkle_tree.push(parent.commit()).unwrap();
        let block_merkle_tree_root = block_merkle_tree.commitment();
        anyhow::ensure!(
            proposal.block_merkle_tree_root == block_merkle_tree_root,
            anyhow::anyhow!(
                "Invalid Block Merkle Tree Error: {}, {}",
                block_merkle_tree_root,
                proposal.block_merkle_tree_root
            )
        );

        Ok(ValidatedState {
            block_merkle_tree,
            fee_merkle_tree,
        })
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
        match self.validate_proposal(parent_header, proposed_header) {
            // Note that currently only block state is updated.
            Ok(validated_state) => Ok(validated_state),
            Err(e) => {
                tracing::warn!("Invalid Proposal: {}", e);
                Err(BlockError::InvalidBlockHeader)
            }
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
