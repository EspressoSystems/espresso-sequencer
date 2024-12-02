
use crate::{FeeAmount, NamespaceId, PubKey, Update};
use hotshot_types::traits::signature_key::SignatureKey;
use serde::{Deserialize, Serialize};
use tide_disco::Url;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct RollupRegistration {
    pub body: RollupRegistrationBody,
    // signature over the above data (must be from a key in the 'signature_keys` list)
    pub signature:
    <PubKey as SignatureKey>::PureAssembledSignatureType,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct RollupRegistrationBody {
    pub namespace_id: NamespaceId,
    // URL of reserve builder for this rollup
    pub reserve_url: Option<Url>,
    // Denominated in Wei
    pub reserve_price: FeeAmount,
    // whether this registration is active in the marketplace
    pub active: bool,
    // a list of keys authorized to update the registration information
    pub signature_keys: Vec<PubKey>,
    // The signature key used to sign this registration body
    pub signature_key: PubKey,
    // Optional field for human readable information
    pub text: String,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct RollupUpdate {
    pub body: RollupUpdatebody,
    // signature over the above data (must be from a key in the 'signature_keys` list)
    pub signature:
        <PubKey as SignatureKey>::PureAssembledSignatureType,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct RollupUpdatebody {
    pub namespace_id: NamespaceId,
    // Denominated in Wei
    #[serde(default)]
    pub reserve_url: Update<Option<Url>>,
    #[serde(default)]
    pub reserve_price: Update<FeeAmount>,
    // whether this registration is active in the marketplace
    #[serde(default)]
    pub active: Update<bool>,
    // a list of keys authorized to update the registration information
    #[serde(default)]
    pub signature_keys: Update<Vec<PubKey>>,
    // The signature key used to sign this update body
    pub signature_key: PubKey,
    // Optional field for human readable information
    pub text: Update<String>,
}
