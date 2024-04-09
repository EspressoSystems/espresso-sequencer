use std::{hash::Hash, marker::PhantomData};

use hotshot_types::{
    traits::{node_implementation::NodeType, signature_key::SignatureKey, BlockPayload},
    utils::BuilderCommitment,
    vid::VidCommitment,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound = "")]
pub struct AvailableBlockInfo<I: NodeType> {
    pub block_hash: BuilderCommitment,
    pub block_size: u64,
    pub offered_fee: u64,
    pub signature: <<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    pub sender: <I as NodeType>::SignatureKey,
    pub _phantom: PhantomData<I>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound = "")]
pub struct AvailableBlockData<TYPES: NodeType> {
    pub block_payload: TYPES::BlockPayload,
    pub metadata: <TYPES::BlockPayload as BlockPayload>::Metadata,
    pub signature: <TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    pub sender: TYPES::SignatureKey,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound = "")]
pub struct AvailableBlockHeaderInput<I: NodeType> {
    pub vid_commitment: VidCommitment,
    pub signature: <<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    pub sender: <I as NodeType>::SignatureKey,
    pub _phantom: PhantomData<I>,
}
