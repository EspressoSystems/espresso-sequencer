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

#[derive(Clone, Debug, Snafu, Deserialize, Serialize)]
#[snafu(visibility(pub))]
pub enum EventError {
    /// The requested resource does not exist or is not known to this hotshot node.
    NotFound,
    /// The requested resource exists but is not currently available.
    Missing,
    /// There was an error while trying to fetch the requested resource.
    #[snafu(display("Failed to fetch requested resource: {message}"))]
    Error { message: String },
}
