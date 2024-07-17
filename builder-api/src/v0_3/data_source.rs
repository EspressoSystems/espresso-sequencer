use async_trait::async_trait;
use hotshot_types::{
    traits::{node_implementation::NodeType, signature_key::SignatureKey},
    utils::BuilderCommitment,
    vid::VidCommitment,
};

use super::{
    block_info::{AvailableBlockData, AvailableBlockInfo},
    builder::BuildError,
};
/// No changes to these types
pub use crate::v0_1::data_source::AcceptsTxnSubmits;

#[async_trait]
pub trait BuilderDataSource<TYPES: NodeType> {
    /// To get the list of available blocks
    async fn available_blocks(
        &self,
        for_parent: &VidCommitment,
        view_number: u64,
        sender: TYPES::SignatureKey,
        signature: &<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<Vec<AvailableBlockInfo<TYPES>>, BuildError>;

    /// to claim a block from the list of provided available blocks
    async fn claim_block(
        &self,
        block_hash: &BuilderCommitment,
        view_number: u64,
        sender: TYPES::SignatureKey,
        signature: &<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockData<TYPES>, BuildError>;

    /// To get the builder's address
    async fn builder_address(&self) -> Result<TYPES::BuilderSignatureKey, BuildError>;
}
