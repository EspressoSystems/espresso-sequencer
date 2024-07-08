use super::client_id::ClientId;
use super::server_message::ServerMessage;
use futures::channel::mpsc::Sender;
use serde::{Deserialize, Serialize};

/// [ClientMessage] represents the messages that the client can send to the
/// server for a request.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ClientMessage {
    SubscribeLatestBlock,
    SubscribeNodeIdentity,
    SubscribeVoters,

    RequestBlocksSnapshot,
    RequestNodeIdentitySnapshot,
    RequestHistogramSnapshot,
    RequestVotersSnapshot,
}

/// InternalClientMessage represents the message requests that the client can
/// send to the server.  These messages are request that the client can send
/// in order for the server to send back responses that correspond to the
/// request.
#[derive(Debug)]
pub enum InternalClientMessage {
    Connected(Sender<ServerMessage>),
    Disconnected(ClientId),

    SubscribeLatestBlock(ClientId),
    SubscribeNodeIdentity(ClientId),
    SubscribeVoters(ClientId),

    RequestBlocksSnapshot(ClientId),
    RequestNodeIdentitySnapshot(ClientId),
    RequestHistogramSnapshot(ClientId),
    RequestVotersSnapshot(ClientId),
}

impl PartialEq for InternalClientMessage {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // We don't care about the [Sender] here, as it is unable to be
            // compared.
            (Self::Connected(_), Self::Connected(_)) => true,
            (Self::Disconnected(lhs), Self::Disconnected(rhs)) => lhs == rhs,
            (Self::SubscribeLatestBlock(lhs), Self::SubscribeLatestBlock(rhs)) => lhs == rhs,
            (Self::SubscribeNodeIdentity(lhs), Self::SubscribeNodeIdentity(rhs)) => lhs == rhs,
            (Self::SubscribeVoters(lhs), Self::SubscribeVoters(rhs)) => lhs == rhs,
            (Self::RequestBlocksSnapshot(lhs), Self::RequestBlocksSnapshot(rhs)) => lhs == rhs,
            (Self::RequestNodeIdentitySnapshot(lhs), Self::RequestNodeIdentitySnapshot(rhs)) => {
                lhs == rhs
            }
            (Self::RequestHistogramSnapshot(lhs), Self::RequestHistogramSnapshot(rhs)) => {
                lhs == rhs
            }
            (Self::RequestVotersSnapshot(lhs), Self::RequestVotersSnapshot(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl ClientMessage {
    /// [to_internal_with_client_id] converts the [ClientMessage] into an
    /// [InternalClientMessage] with the given [ClientId].
    pub fn to_internal_with_client_id(&self, client_id: ClientId) -> InternalClientMessage {
        match self {
            ClientMessage::SubscribeLatestBlock => {
                InternalClientMessage::SubscribeLatestBlock(client_id)
            }
            ClientMessage::SubscribeNodeIdentity => {
                InternalClientMessage::SubscribeNodeIdentity(client_id)
            }
            ClientMessage::SubscribeVoters => InternalClientMessage::SubscribeVoters(client_id),
            ClientMessage::RequestBlocksSnapshot => {
                InternalClientMessage::RequestBlocksSnapshot(client_id)
            }
            ClientMessage::RequestNodeIdentitySnapshot => {
                InternalClientMessage::RequestNodeIdentitySnapshot(client_id)
            }
            ClientMessage::RequestHistogramSnapshot => {
                InternalClientMessage::RequestHistogramSnapshot(client_id)
            }
            ClientMessage::RequestVotersSnapshot => {
                InternalClientMessage::RequestVotersSnapshot(client_id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InternalClientMessage;
    use super::*;
    use std::iter::zip;

    #[test]
    fn test_client_message_partial_eq() {
        let messages = [
            ClientMessage::SubscribeLatestBlock,
            ClientMessage::SubscribeNodeIdentity,
            ClientMessage::SubscribeVoters,
            ClientMessage::RequestBlocksSnapshot,
            ClientMessage::RequestNodeIdentitySnapshot,
            ClientMessage::RequestHistogramSnapshot,
        ];

        for (l, r) in zip(messages.iter(), messages.iter()) {
            assert_eq!(l, r);
        }

        for i in 1..messages.len() {
            for (l, r) in zip(
                messages.iter(),
                messages.iter().skip(i).chain(messages.iter().take(i)),
            ) {
                assert_ne!(l, r);
            }
        }
    }

    #[test]
    fn test_client_message_debug() {
        let messages = [
            ClientMessage::SubscribeLatestBlock,
            ClientMessage::SubscribeNodeIdentity,
            ClientMessage::SubscribeVoters,
            ClientMessage::RequestBlocksSnapshot,
            ClientMessage::RequestNodeIdentitySnapshot,
            ClientMessage::RequestHistogramSnapshot,
        ];

        for message in messages.iter() {
            assert_eq!(format!("{:?}", message), format!("{:?}", message));
        }
    }

    #[test]
    #[cfg(feature = "testing")]
    fn test_client_message_serialize() {
        use serde_json;

        let messages = [
            ClientMessage::SubscribeLatestBlock,
            ClientMessage::SubscribeNodeIdentity,
            ClientMessage::SubscribeVoters,
            ClientMessage::RequestBlocksSnapshot,
            ClientMessage::RequestNodeIdentitySnapshot,
            ClientMessage::RequestHistogramSnapshot,
        ];

        for message in messages.iter() {
            let serialized = serde_json::to_string(message).unwrap();
            let deserialized: ClientMessage = serde_json::from_str(&serialized).unwrap();
            assert_eq!(*message, deserialized);
        }
    }

    #[test]
    fn test_client_message_to_internal_with_client_id() {
        let messages = [
            ClientMessage::SubscribeLatestBlock,
            ClientMessage::SubscribeNodeIdentity,
            ClientMessage::SubscribeVoters,
            ClientMessage::RequestBlocksSnapshot,
            ClientMessage::RequestNodeIdentitySnapshot,
            ClientMessage::RequestHistogramSnapshot,
        ];

        for message in messages {
            for i in 0..10 {
                let client_id = ClientId::from_count(i);
                let internal_client_message = message.to_internal_with_client_id(client_id);
                match internal_client_message {
                    InternalClientMessage::SubscribeLatestBlock(id) => {
                        assert_eq!(id, client_id);
                    }
                    InternalClientMessage::SubscribeNodeIdentity(id) => {
                        assert_eq!(id, client_id);
                    }
                    InternalClientMessage::SubscribeVoters(id) => {
                        assert_eq!(id, client_id);
                    }
                    InternalClientMessage::RequestBlocksSnapshot(id) => {
                        assert_eq!(id, client_id);
                    }
                    InternalClientMessage::RequestNodeIdentitySnapshot(id) => {
                        assert_eq!(id, client_id);
                    }
                    InternalClientMessage::RequestHistogramSnapshot(id) => {
                        assert_eq!(id, client_id);
                    }
                    _ => panic!("Unexpected InternalClientMessage"),
                }
            }
        }
    }

    #[test]
    fn test_internal_client_message_partial_eq() {
        let (sender, _) = futures::channel::mpsc::channel(1);
        let messages = [
            InternalClientMessage::Connected(sender),
            InternalClientMessage::Disconnected(ClientId::from_count(1)),
            InternalClientMessage::SubscribeLatestBlock(ClientId::from_count(1)),
            InternalClientMessage::SubscribeNodeIdentity(ClientId::from_count(1)),
            InternalClientMessage::SubscribeVoters(ClientId::from_count(1)),
            InternalClientMessage::RequestBlocksSnapshot(ClientId::from_count(1)),
            InternalClientMessage::RequestNodeIdentitySnapshot(ClientId::from_count(1)),
            InternalClientMessage::RequestHistogramSnapshot(ClientId::from_count(1)),
        ];

        for (l, r) in zip(messages.iter(), messages.iter()) {
            assert_eq!(l, r);
        }

        for i in 1..messages.len() {
            for (l, r) in zip(
                messages.iter(),
                messages.iter().skip(i).chain(messages.iter().take(i)),
            ) {
                assert_ne!(l, r);
            }
        }

        for j in 2..12 {
            let iter_messages = [
                InternalClientMessage::Disconnected(ClientId::from_count(j)),
                InternalClientMessage::SubscribeLatestBlock(ClientId::from_count(j)),
                InternalClientMessage::SubscribeNodeIdentity(ClientId::from_count(j)),
                InternalClientMessage::SubscribeVoters(ClientId::from_count(j)),
                InternalClientMessage::RequestBlocksSnapshot(ClientId::from_count(j)),
                InternalClientMessage::RequestNodeIdentitySnapshot(ClientId::from_count(j)),
                InternalClientMessage::RequestHistogramSnapshot(ClientId::from_count(j)),
            ];

            // We skip the first message, as we don't want to include the
            // Connected message.
            for (l, r) in zip(messages.iter().skip(1), iter_messages.iter()) {
                assert_ne!(l, r);
            }
        }
    }
}
