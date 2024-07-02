use crate::NsTable;

use super::{
    BlockMerkleCommitment, BuilderSignature, FeeInfo, FeeMerkleCommitment, L1BlockInfo,
    ResolvableChainConfig,
};
use hotshot_types::{utils::BuilderCommitment, vid::VidCommitment};
use serde::{
    de::{self, SeqAccess},
    Deserialize, Serialize,
};
/// A header is like a [`Block`] with the body replaced by a digest.
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Header {
    /// A commitment to a ChainConfig or a full ChainConfig.
    pub(crate) chain_config: ResolvableChainConfig,
    pub(crate) height: u64,
    pub(crate) timestamp: u64,
    pub(crate) l1_head: u64,
    pub(crate) l1_finalized: Option<L1BlockInfo>,
    pub(crate) payload_commitment: VidCommitment,
    pub(crate) builder_commitment: BuilderCommitment,
    pub(crate) ns_table: NsTable,
    pub(crate) block_merkle_tree_root: BlockMerkleCommitment,
    pub(crate) fee_merkle_tree_root: FeeMerkleCommitment,
    pub(crate) fee_info: FeeInfo,
    pub(crate) builder_signature: Option<BuilderSignature>,
}

macro_rules! element {
    ($seq:expr, $field:ident) => {
        $seq.next_element()?
            .ok_or_else(|| de::Error::missing_field(stringify!($field)))?
    };
}

impl Header {
    pub fn deserialize_with_chain_config<'de, A>(
        chain_config: ResolvableChainConfig,
        mut seq: A,
    ) -> Result<Header, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let height = element!(seq, height);
        let timestamp = element!(seq, timestamp);
        let l1_head = element!(seq, l1_head);
        let l1_finalized = element!(seq, l1_finalized);
        let payload_commitment = element!(seq, payload_commitment);
        let builder_commitment = element!(seq, builder_commitment);
        let ns_table = element!(seq, ns_table);
        let block_merkle_tree_root = element!(seq, block_merkle_tree_root);
        let fee_merkle_tree_root = element!(seq, fee_merkle_tree_root);
        let fee_info = element!(seq, fee_info);
        let builder_signature = element!(seq, builder_signature);

        Ok(Self {
            chain_config,
            height,
            timestamp,
            l1_head,
            l1_finalized,
            payload_commitment,
            builder_commitment,
            ns_table,
            block_merkle_tree_root,
            fee_merkle_tree_root,
            fee_info,
            builder_signature,
        })
    }
}
