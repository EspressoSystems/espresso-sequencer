use serde::{Deserialize, Serialize};

use super::client_id::ClientId;
use super::server_message::ServerMessage;
use std::sync::mpsc::Sender;

/// InternalClientMessage represents the message requests that the client can
/// send to the server.  These messages are request that the client can send
/// in order for the server to send back responses that correspond to the
/// request.
pub enum InternalClientMessage {
    Connected(Sender<ServerMessage>),
    Disconnected(ClientId),

    SubscribeLatestBlock(ClientId),
    SubscribeNodeIdentity(ClientId),

    RequestBlocksSnapshot(ClientId),
    RequestNodeIdentitySnapshot(ClientId),
    RequestHistogramSnapshot(ClientId),
}

/// [ClientMessage] represents the messages that the client can send to the
/// server for a request.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ClientMessage {
    SubscribeLatestBlock,
    SubscribeNodeIdentity,

    RequestBlocksSnapshot,
    RequestNodeIdentitySnapshot,
    RequestHistogramSnapshot,
}

impl ClientMessage {
    /// [to_internal_with_client_id] converts the [ClientMessage] into an
    /// [InternalClientMessage] with the given [ClientId].
    pub fn to_internal_with_client_id(self, client_id: ClientId) -> InternalClientMessage {
        match self {
            ClientMessage::SubscribeLatestBlock => {
                InternalClientMessage::SubscribeLatestBlock(client_id)
            }
            ClientMessage::SubscribeNodeIdentity => {
                InternalClientMessage::SubscribeNodeIdentity(client_id)
            }
            ClientMessage::RequestBlocksSnapshot => {
                InternalClientMessage::RequestBlocksSnapshot(client_id)
            }
            ClientMessage::RequestNodeIdentitySnapshot => {
                InternalClientMessage::RequestNodeIdentitySnapshot(client_id)
            }
            ClientMessage::RequestHistogramSnapshot => {
                InternalClientMessage::RequestHistogramSnapshot(client_id)
            }
        }
    }
}
