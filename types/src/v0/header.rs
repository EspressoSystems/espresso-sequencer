use hotshot_query_service::VidCommitment;
use hotshot_types::utils::BuilderCommitment;
use serde::{Deserialize, Serialize};
use vbs::version::Version;

use crate::{
    v0_1, v0_2, v0_3, BlockMerkleCommitment, BuilderSignature, FeeInfo, FeeMerkleCommitment,
    L1BlockInfo, NsTable, ResolvableChainConfig,
};

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum Header {
    V1(v0_1::Header),
    V2(v0_2::Header),
    V3(v0_3::Header),
}

impl Header {
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        chain_config: ResolvableChainConfig,
        height: u64,
        timestamp: u64,
        l1_head: u64,
        l1_finalized: Option<L1BlockInfo>,
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        ns_table: NsTable,
        fee_merkle_tree_root: FeeMerkleCommitment,
        block_merkle_tree_root: BlockMerkleCommitment,
        fee_info: FeeInfo,
        builder_signature: Option<BuilderSignature>,
        version: Version,
    ) -> Self {
        let Version { major, minor } = version;

        assert!(major == 0, "Invalid major version {major}");

        match minor {
            1 => Self::V1(v0_1::Header {
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
            }),
            2 => Self::V2(v0_2::Header {
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
            }),
            3 => Self::V3(v0_3::Header {
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
            }),
            _ => panic!("invalid version: {version}"),
        }
    }
}
