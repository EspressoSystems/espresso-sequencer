use hotshot_types::event::Event;
use hotshot_types::traits::{node_implementation::NodeType, signature_key::SignatureKey};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = ""))]
pub struct EventInfo<I: NodeType> {
    pub event: Event<I>,
    pub signature: <<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    pub sender: <I as NodeType>::SignatureKey,
    pub _phantom: PhantomData<I>,
}
