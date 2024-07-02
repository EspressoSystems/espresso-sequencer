use crate::NsTable;

use super::{
    BlockMerkleCommitment, BuilderSignature, FeeInfo, FeeMerkleCommitment, L1BlockInfo,
    ResolvableChainConfig,
};
use hotshot_types::{utils::BuilderCommitment, vid::VidCommitment};
use serde::{
    de::{self, MapAccess, SeqAccess},
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

    pub fn deserialize_with_chain_config_map<'de, M>(
        chain_config: ResolvableChainConfig,
        mut map: M,
    ) -> Result<Header, M::Error>
    where
        M: MapAccess<'de>,
    {
        // defining an enum of all the fields that are expected in the struct
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum ChainConfigFields {
            Height,
            Timestamp,
            L1Head,
            L1Finalized,
            PayloadCommitment,
            BuilderCommitment,
            NsTable,
            BlockMerkleTreeRoot,
            FeeMerkleTreeRoot,
            FeeInfo,
            BuilderSignature,
        }

        let mut height = None;
        let mut timestamp = None;
        let mut l1_head = None;
        let mut l1_finalized = None;
        let mut payload_commitment = None;
        let mut builder_commitment = None;
        let mut ns_table = None;
        let mut block_merkle_tree_root = None;
        let mut fee_merkle_tree_root = None;
        let mut fee_info = None;
        let mut builder_signature = None;

        // Iterate over the key-value pairs in the map. The order of fields in the JSON
        // may not match the order in the struct, so we handle each field based on its key.
        while let Some(key) = map.next_key()? {
            match key {
                ChainConfigFields::Height => height = Some(map.next_value()?),
                ChainConfigFields::Timestamp => timestamp = Some(map.next_value()?),
                ChainConfigFields::L1Head => l1_head = Some(map.next_value()?),
                ChainConfigFields::L1Finalized => l1_finalized = map.next_value()?,
                ChainConfigFields::PayloadCommitment => {
                    payload_commitment = Some(map.next_value()?)
                }
                ChainConfigFields::BuilderCommitment => {
                    builder_commitment = Some(map.next_value()?)
                }
                ChainConfigFields::NsTable => ns_table = Some(map.next_value()?),
                ChainConfigFields::BlockMerkleTreeRoot => {
                    block_merkle_tree_root = Some(map.next_value()?)
                }
                ChainConfigFields::FeeMerkleTreeRoot => {
                    fee_merkle_tree_root = Some(map.next_value()?)
                }
                ChainConfigFields::FeeInfo => fee_info = Some(map.next_value()?),
                ChainConfigFields::BuilderSignature => builder_signature = map.next_value()?,
            }
        }

        // If any field is missing, we return an error indicating which field is missing.

        Ok(Self {
            chain_config,
            height: height.ok_or_else(|| de::Error::missing_field("height"))?,
            timestamp: timestamp.ok_or_else(|| de::Error::missing_field("timestamp"))?,
            l1_head: l1_head.ok_or_else(|| de::Error::missing_field("l1_head"))?,
            l1_finalized,
            payload_commitment: payload_commitment
                .ok_or_else(|| de::Error::missing_field("payload_commitment"))?,
            builder_commitment: builder_commitment
                .ok_or_else(|| de::Error::missing_field("builder_commitment"))?,
            ns_table: ns_table.ok_or_else(|| de::Error::missing_field("ns_table"))?,
            block_merkle_tree_root: block_merkle_tree_root
                .ok_or_else(|| de::Error::missing_field("block_merkle_tree_root"))?,
            fee_merkle_tree_root: fee_merkle_tree_root
                .ok_or_else(|| de::Error::missing_field("fee_merkle_tree_root"))?,
            fee_info: fee_info.ok_or_else(|| de::Error::missing_field("fee_info"))?,
            builder_signature,
        })
    }
}
