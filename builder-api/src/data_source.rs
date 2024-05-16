use async_trait::async_trait;
use committable::Commitment;
use hotshot_types::{
    traits::{node_implementation::NodeType, signature_key::SignatureKey},
    utils::BuilderCommitment,
    vid::VidCommitment,
};

use crate::{
    block_info::{AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo},
    builder::BuildError,
};

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

    /// To claim a block header input
    async fn claim_block_header_input(
        &self,
        block_hash: &BuilderCommitment,
        view_number: u64,
        sender: TYPES::SignatureKey,
        signature: &<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockHeaderInput<TYPES>, BuildError>;

    /// To get the builder address
    async fn builder_address(&self) -> Result<TYPES::BuilderSignatureKey, BuildError>;
}

#[async_trait]
pub trait AcceptsTxnSubmits<I>
where
    I: NodeType,
{
    async fn submit_txns(
        &self,
        txns: Vec<<I as NodeType>::Transaction>,
    ) -> Result<Vec<Commitment<<I as NodeType>::Transaction>>, BuildError>;
}
