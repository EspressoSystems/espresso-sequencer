use super::{
    BlockMerkleCommitment, BuilderSignature, FeeInfo, FeeMerkleCommitment, L1BlockInfo,
    NameSpaceTable, ResolvableChainConfig, TxTableEntryWord,
};
use hotshot_types::{utils::BuilderCommitment, vid::VidCommitment};
use serde::{Deserialize, Serialize};

/// A header is like a block with the body replaced by a digest.
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Header {
    pub(super) chain_config: ResolvableChainConfig,
    pub(super) height: u64,
    pub(super) timestamp: u64,
    pub(super) l1_head: u64,
    pub(super) l1_finalized: Option<L1BlockInfo>,
    pub(super) payload_commitment: VidCommitment,
    pub(super) builder_commitment: BuilderCommitment,
    pub(super) ns_table: NameSpaceTable<TxTableEntryWord>,
    pub(super) block_merkle_tree_root: BlockMerkleCommitment,
    pub(super) fee_merkle_tree_root: FeeMerkleCommitment,
    pub(super) fee_info: FeeInfo,
    pub(super) builder_signature: Option<BuilderSignature>,
}
