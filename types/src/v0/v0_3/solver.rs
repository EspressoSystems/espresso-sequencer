use crate::{FeeAmount, NamespaceId, SeqTypes};
use hotshot::types::SignatureKey;
use hotshot_types::traits::node_implementation::NodeType;
use serde::{Deserialize, Serialize};
use tide_disco::Url;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct RollupRegistration {
    pub body: RollupRegistrationBody,
    // signature over the above data (must be from a key in the 'signature_keys` list)
    pub signature:
        <<SeqTypes as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct RollupRegistrationBody {
    pub namespace_id: NamespaceId,
    pub reserve_url: Url,
    // Denominated in Wei
    pub reserve_price: FeeAmount,
    // whether this registration is active in the marketplace
    pub active: bool,
    // a list of keys authorized to update the registration information
    pub signature_keys: Vec<<SeqTypes as NodeType>::SignatureKey>,
    // The signature key used to sign this registration body
    pub signature_key: <SeqTypes as NodeType>::SignatureKey,
    // Optional field for human readable information
    pub text: String,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct RollupUpdate {
    pub body: RollupUpdatebody,
    // signature over the above data (must be from a key in the 'signature_keys` list)
    pub signature:
        <<SeqTypes as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct RollupUpdatebody {
    pub namespace_id: NamespaceId,
    // Denominated in Wei
    pub reserve_url: Option<Url>,
    pub reserve_price: Option<FeeAmount>,
    // whether this registration is active in the marketplace
    pub active: Option<bool>,
    // a list of keys authorized to update the registration information
    pub signature_keys: Option<Vec<<SeqTypes as NodeType>::SignatureKey>>,
    // The signature key used to sign this update body
    pub signature_key: <SeqTypes as NodeType>::SignatureKey,
    // Optional field for human readable information
    pub text: Option<String>,
}
