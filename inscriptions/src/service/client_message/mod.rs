use super::client_id::ClientId;

/// InternalClientMessage represents the message requests that the client can
/// send to the server.  These messages are request that the client can send
/// in order for the server to send back responses that correspond to the
/// request.
#[derive(Debug)]
pub enum InternalClientMessage<K> {
    Connected(K),
    Disconnected(ClientId),
}

#[cfg(test)]
mod tests {
    use super::InternalClientMessage;
    use super::*;
    use crate::service::server_message::ServerMessage;
    use std::iter::zip;

    impl<K> PartialEq for InternalClientMessage<K> {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                // We don't care about the [Sender] here, as it is unable to be
                // compared.
                (Self::Connected(_), Self::Connected(_)) => true,
                (Self::Disconnected(lhs), Self::Disconnected(rhs)) => lhs == rhs,
                _ => false,
            }
        }
    }

    #[test]
    fn test_internal_client_message_partial_eq() {
        let (sender, _) = futures::channel::mpsc::channel::<ServerMessage>(1);
        let messages = [
            InternalClientMessage::Connected(sender),
            InternalClientMessage::Disconnected(ClientId::from_count(1)),
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
            let iter_messages = [InternalClientMessage::Disconnected(ClientId::from_count(j))];

            // We skip the first message, as we don't want to include the
            // Connected message.
            for (l, r) in zip(messages.iter().skip(1), iter_messages.iter()) {
                assert_ne!(l, r);
            }
        }
    }
}
