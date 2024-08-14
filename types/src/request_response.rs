// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! Types for the request/response implementations. This module incorporates all
//! of the shared types for all of the network backends.

use async_lock::Mutex;
use futures::channel::{mpsc::Receiver, oneshot};
use libp2p::request_response::ResponseChannel;
use serde::{Deserialize, Serialize};

use crate::traits::network::NetworkMsg;

/// Request for Consenus data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request(#[serde(with = "serde_bytes")] pub Vec<u8>);

/// Response for some VID data that we already collected
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response(
    /// Data
    #[serde(with = "serde_bytes")]
    pub Vec<u8>,
);

/// Wraps a oneshot channel for responding to requests. This is a
/// specialized version of the libp2p request-response `ResponseChannel`
/// which accepts any generic response.
pub struct NetworkMsgResponseChannel<M: NetworkMsg> {
    /// underlying sender for this channel
    pub sender: oneshot::Sender<M>,
}

/// Type alias for the channel that we receive requests from the network on.
pub type RequestReceiver = Receiver<(Vec<u8>, NetworkMsgResponseChannel<Vec<u8>>)>;

/// Locked Option of a receiver for moving the value out of the option. This
/// type takes any `Response` type depending on the underlying network impl.
pub type TakeReceiver = Mutex<Option<Receiver<(Vec<u8>, ResponseChannel<Response>)>>>;
