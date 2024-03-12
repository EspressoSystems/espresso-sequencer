use std::{hash::Hash, marker::PhantomData};

use hotshot_types::event::Event;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound = "")]
pub struct EventInfo<I: NodeType> {
    pub event: Event<I>,
    pub signature: <<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    pub sender: <I as NodeType>::SignatureKey,
    pub _phantom: PhantomData<I>,
}