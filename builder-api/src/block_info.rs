use std::{hash::Hash, marker::PhantomData};

use hotshot_types::{
    traits::{node_implementation::NodeType, signature_key::BuilderSignatureKey, BlockPayload},
    utils::BuilderCommitment,
    vid::{VidCommitment, VidPrecomputeData},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound = "")]
pub struct AvailableBlockInfo<TYPES: NodeType> {
    pub block_hash: BuilderCommitment,
    pub block_size: u64,
    pub offered_fee: u64,
    pub signature:
        <<TYPES as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::BuilderSignature,
    pub sender: <TYPES as NodeType>::BuilderSignatureKey,
    pub _phantom: PhantomData<TYPES>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound = "")]
pub struct AvailableBlockData<TYPES: NodeType> {
    pub block_payload: TYPES::BlockPayload,
    pub metadata: <TYPES::BlockPayload as BlockPayload>::Metadata,
    pub signature:
        <<TYPES as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::BuilderSignature,
    pub sender: <TYPES as NodeType>::BuilderSignatureKey,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound = "")]
pub struct AvailableBlockHeaderInput<TYPES: NodeType> {
    pub vid_commitment: VidCommitment,
    pub vid_precompute_data: VidPrecomputeData,
    // signature over vid_commitment, BlockPayload::Metadata, and offered_fee
    pub fee_signature:
        <<TYPES as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::BuilderSignature,
    // signature over the current response
    pub message_signature:
        <<TYPES as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::BuilderSignature,
    pub sender: <TYPES as NodeType>::BuilderSignatureKey,
}
