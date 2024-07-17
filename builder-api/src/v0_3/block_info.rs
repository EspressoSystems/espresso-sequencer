use hotshot_types::traits::{
    node_implementation::NodeType, signature_key::BuilderSignatureKey, BlockPayload,
};
use serde::{Deserialize, Serialize};

/// No changes to these types
pub use crate::v0_1::block_info::AvailableBlockInfo;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound = "")]
pub struct AvailableBlockData<TYPES: NodeType> {
    pub block_payload: TYPES::BlockPayload,
    pub metadata: <TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata,
    pub fee: u64,
    pub signature:
        <<TYPES as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::BuilderSignature,
    pub fee_signature:
        <<TYPES as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::BuilderSignature,
    pub sender: <TYPES as NodeType>::BuilderSignatureKey,
}

impl<TYPES: NodeType> AvailableBlockData<TYPES> {
    pub fn validate_signature(&self) -> bool {
        // verify the signature over the message, construct the builder commitment
        let builder_commitment = self.block_payload.builder_commitment(&self.metadata);
        self.sender
            .validate_builder_signature(&self.signature, builder_commitment.as_ref())
            && self
                .sender
                .validate_sequencing_fee_signature_marketplace(&self.fee_signature, self.fee)
    }
}
