use hotshot_types::{
    data::{EpochNumber, ViewNumber},
    traits::node_implementation::NodeType,
    vote::HasViewNumber,
};
use serde::{Deserialize, Serialize};

use crate::message::{
    CatchupEvidence, Certificate1,
    fetch::{Request, Response},
};

pub type CertFetchRequest = Request<CertRequestBody>;
pub type CertFetchResponse<T> = Response<CertResponseBody<T>>;

/// What a node asks a peer for. The request's view is the requester's current
/// view for `Newest` and the certificate's view for `Cert1`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
pub enum CertRequestBody {
    /// The newest certificate the peer holds.
    Newest,
    /// The `Certificate1` the peer holds at the request's view and `epoch`.
    Cert1 { epoch: EpochNumber },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
#[serde(bound(deserialize = ""))]
pub enum CertResponseBody<T: NodeType> {
    Newest(Option<CatchupEvidence<T>>),
    Cert1(Option<Certificate1<T>>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
#[serde(bound(deserialize = ""))]
pub enum CertFetchMessage<T: NodeType> {
    Req(CertFetchRequest),
    Res(CertFetchResponse<T>),
}

impl<T: NodeType> HasViewNumber for CertFetchMessage<T> {
    fn view_number(&self) -> ViewNumber {
        match self {
            Self::Req(r) => r.view_number(),
            Self::Res(r) => r.view_number(),
        }
    }
}
