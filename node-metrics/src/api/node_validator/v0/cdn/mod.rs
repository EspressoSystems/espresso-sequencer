use crate::api::node_validator::v0::create_node_validator_api::ExternalMessage;
use async_std::task::JoinHandle;
use espresso_types::{PubKey, SeqTypes};
use futures::{channel::mpsc::SendError, Sink, SinkExt};
use hotshot::{traits::implementations::PushCdnNetwork, types::Message};
use hotshot_types::{
    message::{MessageKind, VersionedMessage},
    traits::network::{BroadcastDelay, ConnectedNetwork, Topic},
};
use url::Url;

/// CdnReceiveMessagesTask represents a task that is responsible for receiving
/// messages from the CDN network and processing them.
/// This task is primarily concerned with recording responses to RollCall
/// requests, and forwarding any discovered public API URLs to the URL sender.
pub struct CdnReceiveMessagesTask {
    task_handle: Option<JoinHandle<()>>,
}

impl CdnReceiveMessagesTask {
    /// Creates a new `CdnReceiveMessagesTask` with the given network and
    /// URL sender. Calling this function will create an async task that
    /// will begin executing immediately.  The handle for the task will
    /// be in the returned structure.
    pub fn new<K>(network: PushCdnNetwork<SeqTypes>, url_sender: K) -> Self
    where
        K: Sink<Url, Error = SendError> + Clone + Send + Unpin + 'static,
    {
        let task_handle = async_std::task::spawn(Self::process_cdn_messages(network, url_sender));
        Self {
            task_handle: Some(task_handle),
        }
    }

    /// [process_cdn_messages] is the function that will begin consuming
    /// messages off of the CDN, and start handling them.
    ///
    /// At the moment, this only looks for and recognizes
    /// [MessageKind::External] messages, and attempts to decode
    /// [ExternalMessage] from those contained pieces of data.  Though, in the
    /// future this may be able to be expanded to other things.
    async fn process_cdn_messages<K>(network: PushCdnNetwork<SeqTypes>, url_sender: K)
    where
        K: Sink<Url, Error = SendError> + Clone + Send + Unpin + 'static,
    {
        network.wait_for_ready().await;
        let mut url_sender = url_sender;

        loop {
            let messages_result = network.recv_msgs().await;
            let messages = match messages_result {
                Ok(message) => message,
                Err(err) => {
                    tracing::error!("error receiving message: {:?}", err);
                    continue;
                }
            };

            for message in messages {
                // We want to try and decode this message.
                let message_deserialize_result = Message::<SeqTypes>::deserialize(&message, &None);

                let message = match message_deserialize_result {
                    Ok(message) => message,
                    Err(err) => {
                        tracing::error!("error deserializing message: {:?}", err);
                        continue;
                    }
                };

                let external_message_deserialize_result = match message.kind {
                    MessageKind::External(external_message) => {
                        bincode::deserialize::<ExternalMessage>(&external_message)
                    }
                    _ => {
                        tracing::error!("unexpected message kind: {:?}", message);
                        continue;
                    }
                };

                let external_message = match external_message_deserialize_result {
                    Ok(external_message) => external_message,
                    Err(err) => {
                        tracing::error!("error deserializing message: {:?}", err);
                        continue;
                    }
                };

                match external_message {
                    ExternalMessage::RollCallResponse(roll_call_info) => {
                        let public_api_url = roll_call_info.public_api_url;

                        // We have a public api url, so we can process this url.

                        if let Err(err) = url_sender.send(public_api_url).await {
                            tracing::error!("error sending public api url: {:?}", err);
                        }
                    }

                    _ => {
                        // We're not concerned about other message types
                    }
                }
            }
        }
    }
}

impl Drop for CdnReceiveMessagesTask {
    fn drop(&mut self) {
        if let Some(task_handle) = self.task_handle.take() {
            async_std::task::block_on(task_handle.cancel());
        }
    }
}

/// BroadcastRollCallTask represents a task that is responsible for broadcasting
/// a RollCallRequest to the CDN network.
pub struct BroadcastRollCallTask {
    task_handle: Option<JoinHandle<()>>,
}

impl BroadcastRollCallTask {
    /// Creates a new `BroadcastRollCallTask` with the given network and
    /// public key. Calling this function will create an async task that
    /// will begin executing immediately.  The handle for the task will
    /// be in the returned structure.
    ///
    /// This task only performs one action, and then returns.  It is not
    /// long-lived.
    pub fn new(network: PushCdnNetwork<SeqTypes>, public_key: PubKey) -> Self {
        let task_handle = async_std::task::spawn(Self::broadcast_roll_call(network, public_key));
        Self {
            task_handle: Some(task_handle),
        }
    }

    /// [broadcast_roll_call] is the function that will broadcast a
    /// RollCallRequest to the CDN network in order to request responses from
    /// the rest of the network participants, so we can collect the public API
    /// URLs in the message consuming task.
    async fn broadcast_roll_call(network: PushCdnNetwork<SeqTypes>, public_key: PubKey) {
        network.wait_for_ready().await;

        // We want to send the Roll Call Request
        let rollcall_request = ExternalMessage::RollCallRequest(public_key);
        let rollcall_request_serialized = match bincode::serialize(&rollcall_request) {
            Ok(rollcall_request_serialized) => rollcall_request_serialized,
            Err(err) => {
                tracing::error!("error serializing rollcall request: {:?}", err);
                return;
            }
        };

        let hotshot_message = Message::<SeqTypes> {
            sender: public_key,
            kind: MessageKind::External(rollcall_request_serialized),
        };

        let hotshot_message_serialized = match hotshot_message.serialize(&None) {
            Ok(hotshot_message_serialized) => hotshot_message_serialized,
            Err(err) => {
                tracing::error!("error serializing hotshot message: {:?}", err);
                return;
            }
        };

        let broadcast_result = network
            .broadcast_message(
                hotshot_message_serialized,
                Topic::Global,
                BroadcastDelay::None,
            )
            .await;
        if let Err(err) = broadcast_result {
            tracing::error!("error broadcasting rollcall request: {:?}", err);
        }

        tracing::info!("broadcast roll call request completed");
    }
}

impl Drop for BroadcastRollCallTask {
    fn drop(&mut self) {
        if let Some(task_handle) = self.task_handle.take() {
            async_std::task::block_on(task_handle.cancel());
        }
    }
}
