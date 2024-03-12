use hotshot_types::event::Event;
use hotshot_types::{
    traits::{node_implementation::NodeType, signature_key::SignatureKey},
    utils::BuilderCommitment,
    vid::VidCommitment,
};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::{hash::Hash, marker::PhantomData};
#[derive(Clone, Debug)]
pub struct EventInfo<I: NodeType> {
    pub event: Event<I>,
    pub signature: <<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    pub sender: <I as NodeType>::SignatureKey,
    pub _phantom: PhantomData<I>,
}
