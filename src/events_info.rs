use hotshot_types::event::Event;
use hotshot_types::traits::{node_implementation::NodeType, signature_key::SignatureKey};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = ""))]
pub struct EventInfo<Types: NodeType> {
    pub event: Event<Types>,
    pub signature: <<Types as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    pub sender: <Types as NodeType>::SignatureKey,
    pub _phantom: PhantomData<Types>,
}
