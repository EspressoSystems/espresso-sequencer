use crate::api::node_validator::v0::create_node_validator_api::ExternalMessage;
use async_std::task::JoinHandle;
use espresso_types::{v0::SequencerVersions, PubKey, SeqTypes};
use futures::{channel::mpsc::SendError, Sink, SinkExt};
use hotshot::{
    traits::NetworkError,
    types::{Message, SignatureKey},
};
use hotshot_types::{
    message::{MessageKind, UpgradeLock},
    traits::{
        network::{BroadcastDelay, ConnectedNetwork, Topic},
        node_implementation::NodeType,
    },
};
use url::Url;

/// ConnectedNetworkConsumer represents a trait that splits up a portion of
/// the ConnectedNetwork trait, so that the consumer only needs to be aware of
/// the `wait_for_ready` and `recv_msgs` functions.
#[async_trait::async_trait]
pub trait ConnectedNetworkConsumer<K> {
    /// [wait_for_ready] will not return until the network is ready to be
    /// utilized.
    async fn wait_for_ready(&self);

    /// [recv_msgs] will return a list of messages that have been received from
    /// the network.
    ///
    /// ## Errors
    ///
    /// All errors are expected to be network related.
    async fn recv_msgs(&self) -> Result<Vec<Vec<u8>>, NetworkError>;
}

#[async_trait::async_trait]
impl<K, N> ConnectedNetworkConsumer<K> for N
where
    K: SignatureKey + Send + Sync + 'static,
    N: ConnectedNetwork<K> + 'static,
{
    async fn wait_for_ready(&self) {
        <N as ConnectedNetwork<K>>::wait_for_ready(self).await
    }

    async fn recv_msgs(&self) -> Result<Vec<Vec<u8>>, NetworkError> {
        let cloned_self = self.clone();
        <N as ConnectedNetwork<K>>::recv_msgs(&cloned_self).await
    }
}

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
    pub fn new<N, K>(network: N, url_sender: K) -> Self
    where
        N: ConnectedNetworkConsumer<<SeqTypes as NodeType>::SignatureKey> + Send + 'static,
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
    async fn process_cdn_messages<N, K>(network: N, url_sender: K)
    where
        N: ConnectedNetworkConsumer<<SeqTypes as NodeType>::SignatureKey> + Send + 'static,
        K: Sink<Url, Error = SendError> + Clone + Send + Unpin + 'static,
    {
        network.wait_for_ready().await;
        let mut url_sender = url_sender;
        let upgrade_lock = UpgradeLock::<SeqTypes, SequencerVersions>::new();

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
                let message_deserialize_result = upgrade_lock
                    .deserialize::<Message<SeqTypes>>(&message)
                    .await;

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
                            return;
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

/// ConnectedNetworkPublisher represents a trait that splits up a portion of
/// the ConnectedNetwork trait, so that the consumer only needs to be aware of
/// the `wait_for_ready` and `broadcast_message` functions.
#[async_trait::async_trait]
pub trait ConnectedNetworkPublisher<K> {
    /// [wait_for_ready] will not return until the network is ready to be
    /// utilized.
    async fn wait_for_ready(&self);

    /// [broadcast_message] will broadcast the given message to some subset of
    /// nodes in the network based on the given topic.
    ///
    /// This is a blocking operation.
    async fn broadcast_message(
        &self,
        message: Vec<u8>,
        topic: Topic,
        broadcast_delay: BroadcastDelay,
    ) -> Result<(), NetworkError>;
}

#[async_trait::async_trait]
impl<K, N> ConnectedNetworkPublisher<K> for N
where
    K: SignatureKey + Send + Sync + 'static,
    N: ConnectedNetwork<K> + 'static,
{
    async fn wait_for_ready(&self) {
        <N as ConnectedNetwork<K>>::wait_for_ready(self).await
    }

    async fn broadcast_message(
        &self,
        message: Vec<u8>,
        topic: Topic,
        broadcast_delay: BroadcastDelay,
    ) -> Result<(), NetworkError> {
        <N as ConnectedNetwork<K>>::broadcast_message(self, message, topic, broadcast_delay).await
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
    pub fn new<N>(network: N, public_key: PubKey) -> Self
    where
        N: ConnectedNetworkPublisher<<SeqTypes as NodeType>::SignatureKey> + Send + 'static,
    {
        let task_handle = async_std::task::spawn(Self::broadcast_roll_call(network, public_key));
        Self {
            task_handle: Some(task_handle),
        }
    }

    /// [broadcast_roll_call] is the function that will broadcast a
    /// RollCallRequest to the CDN network in order to request responses from
    /// the rest of the network participants, so we can collect the public API
    /// URLs in the message consuming task.
    async fn broadcast_roll_call<N>(network: N, public_key: PubKey)
    where
        N: ConnectedNetworkPublisher<<SeqTypes as NodeType>::SignatureKey> + Send + 'static,
    {
        network.wait_for_ready().await;
        let upgrade_lock = UpgradeLock::<SeqTypes, SequencerVersions>::new();

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

        let hotshot_message_serialized = match upgrade_lock.serialize(&hotshot_message).await {
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

#[cfg(test)]
mod test {
    use super::{BroadcastRollCallTask, ConnectedNetworkConsumer, ConnectedNetworkPublisher};
    use crate::api::node_validator::v0::create_node_validator_api::ExternalMessage;
    use crate::api::node_validator::v0::{
        cdn::CdnReceiveMessagesTask, create_node_validator_api::RollCallInfo,
    };
    use async_std::future::TimeoutError;
    use async_std::prelude::FutureExt;
    use core::panic;
    use espresso_types::{v0::SequencerVersions, SeqTypes};
    use futures::channel::mpsc::Sender;
    use futures::SinkExt;
    use futures::{
        channel::mpsc::{self},
        StreamExt,
    };
    use hotshot::types::SignatureKey;
    use hotshot::{
        traits::NetworkError,
        types::{BLSPubKey, Message},
    };
    use hotshot_types::message::{DataMessage, MessageKind, UpgradeLock};
    use hotshot_types::traits::network::{BroadcastDelay, ResponseMessage};
    use std::time::Duration;
    use url::Url;

    /// [TestConnectedNetworkConsumer] is a test implementation of the
    /// [ConnectedNetworkConsumer] trait that allows for the simulation of
    /// network messages being received.
    struct TestConnectedNetworkConsumer(Result<Vec<Vec<u8>>, NetworkError>);

    /// [clone_result] is a helper function that clones the result of a
    /// network message receive operation.  This is used to ensure that the
    /// original result is not consumed by the task.
    fn clone_result(
        result: &Result<Vec<Vec<u8>>, NetworkError>,
    ) -> Result<Vec<Vec<u8>>, NetworkError> {
        match result {
            Ok(messages) => Ok(messages.clone()),
            Err(err) => match err {
                NetworkError::ChannelSend => Err(NetworkError::ChannelSend),
                _ => panic!("unexpected network error"),
            },
        }
    }

    #[async_trait::async_trait]
    impl ConnectedNetworkConsumer<BLSPubKey> for TestConnectedNetworkConsumer {
        async fn wait_for_ready(&self) {}

        async fn recv_msgs(&self) -> Result<Vec<Vec<u8>>, NetworkError> {
            async_std::task::sleep(Duration::from_millis(5)).await;
            clone_result(&self.0)
        }
    }

    /// [test_cdn_receive_messages_task] is a test that verifies that the
    /// an expected External Message can be encoded, decoded, and sent to the
    /// url_sender appropriately.
    #[async_std::test]
    async fn test_cdn_receive_messages_task() {
        let upgrade_lock = UpgradeLock::<SeqTypes, SequencerVersions>::new();

        let test_hotshot_message_serialized = {
            let test_url = Url::parse("http://localhost:8080/").unwrap();

            let test_external_message = ExternalMessage::RollCallResponse(RollCallInfo {
                public_api_url: test_url.clone(),
            });

            let external_message_encoded = bincode::serialize(&test_external_message).unwrap();

            let test_message = Message::<SeqTypes> {
                sender: BLSPubKey::generated_from_seed_indexed([0; 32], 0).0,
                kind: MessageKind::External(external_message_encoded),
            };

            upgrade_lock.serialize(&test_message).await.unwrap()
        };

        let (url_sender, url_receiver) = mpsc::channel(1);
        let task = CdnReceiveMessagesTask::new(
            TestConnectedNetworkConsumer(Ok(vec![test_hotshot_message_serialized])),
            url_sender,
        );

        let mut url_receiver = url_receiver;
        let next_message = url_receiver
            .next()
            .timeout(Duration::from_millis(50))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(next_message, Url::parse("http://localhost:8080/").unwrap());

        drop(task);
    }

    /// [test_cdn_receive_messages_task_fails_receiving_message] is a test that
    /// verifies that the task does not close, nor send a url, when it
    /// encounters an error from the recv_msgs function.
    #[async_std::test]
    async fn test_cdn_receive_messages_task_fails_receiving_message() {
        let (url_sender, url_receiver) = mpsc::channel(1);
        let task = CdnReceiveMessagesTask::new(
            TestConnectedNetworkConsumer(Err(NetworkError::ChannelSend)),
            url_sender,
        );

        let mut url_receiver = url_receiver;
        // The task should not panic when it fails to receive a message.
        let receive_result = url_receiver.next().timeout(Duration::from_millis(50)).await;

        if let Err(TimeoutError { .. }) = receive_result {
            // This is expected
        } else {
            panic!("receive did not timeout");
        }

        drop(task);
    }

    /// [test_cdn_receive_messages_task_fails_decoding_hotshot_message] is a
    /// test that verifies that the task does not close, nor send a url, when it
    /// encounters an error from the deserialization of the hotshot message.
    #[async_std::test]
    async fn test_cdn_receive_messages_task_fails_decoding_hotshot_message() {
        let (url_sender, url_receiver) = mpsc::channel(1);
        let task = CdnReceiveMessagesTask::new(
            TestConnectedNetworkConsumer(Ok(vec![vec![0]])),
            url_sender,
        );

        let mut url_receiver = url_receiver;
        // The task should not panic when it fails to receive a message.
        let receive_result = url_receiver.next().timeout(Duration::from_millis(50)).await;

        if let Err(TimeoutError { .. }) = receive_result {
            // This is expected
        } else {
            panic!("receive did not timeout");
        }

        drop(task);
    }

    /// [test_cdn_receive_messages_task_fails_unexpected_hotshot_message_variant]
    /// is a test that verifies that the task does not close, nor send a url, when
    /// it encounters a hotshot message that was not an External message.
    ///
    /// This really shouldn't happen in practice.
    #[async_std::test]
    async fn test_cdn_receive_messages_task_fails_unexpected_hotshot_message_variant() {
        let upgrade_lock = UpgradeLock::<SeqTypes, SequencerVersions>::new();
        let (url_sender, url_receiver) = mpsc::channel(1);
        let bytes = upgrade_lock
            .serialize(&Message::<SeqTypes> {
                sender: BLSPubKey::generated_from_seed_indexed([0; 32], 0).0,
                kind: MessageKind::Data(DataMessage::DataResponse(ResponseMessage::NotFound)),
            })
            .await
            .unwrap();

        let task =
            CdnReceiveMessagesTask::new(TestConnectedNetworkConsumer(Ok(vec![bytes])), url_sender);

        let mut url_receiver = url_receiver;
        // The task should not panic when it fails to receive a message.
        let receive_result = url_receiver.next().timeout(Duration::from_millis(50)).await;

        if let Err(TimeoutError { .. }) = receive_result {
            // This is expected
        } else {
            panic!("receive did not timeout");
        }

        drop(task);
    }

    /// [test_cdn_receive_messages_task_fails_decoding_external_message] is a
    /// test that verifies that the task does not close, nor send a url, when
    /// it encounters an error from the deserialization of the external message.
    #[async_std::test]
    async fn test_cdn_receive_messages_task_fails_decoding_external_message() {
        let upgrade_lock = UpgradeLock::<SeqTypes, SequencerVersions>::new();
        let (url_sender, url_receiver) = mpsc::channel(1);
        let bytes = upgrade_lock
            .serialize(&Message::<SeqTypes> {
                sender: BLSPubKey::generated_from_seed_indexed([0; 32], 0).0,
                kind: MessageKind::External(vec![0]),
            })
            .await
            .unwrap();

        let task =
            CdnReceiveMessagesTask::new(TestConnectedNetworkConsumer(Ok(vec![bytes])), url_sender);

        let mut url_receiver = url_receiver;
        // The task should not panic when it fails to receive a message.
        let receive_result = url_receiver.next().timeout(Duration::from_millis(50)).await;

        if let Err(TimeoutError { .. }) = receive_result {
            // This is expected
        } else {
            panic!("receive did not timeout");
        }

        drop(task);
    }

    /// [test_cdn_receive_messages_tasks_exits_when_url_receiver_closed] is a
    /// test that verifies that the task exits when the url receiver is closed.
    ///
    /// Without being able to send urls to the url_sender, the task doesn't
    /// really have a point in existing.
    #[async_std::test]
    async fn test_cdn_receive_messages_tasks_exits_when_url_receiver_closed() {
        let upgrade_lock = UpgradeLock::<SeqTypes, SequencerVersions>::new();
        let (url_sender, url_receiver) = mpsc::channel(1);

        let test_hotshot_message_serialized = {
            let test_url = Url::parse("http://localhost:8080/").unwrap();

            let test_external_message = ExternalMessage::RollCallResponse(RollCallInfo {
                public_api_url: test_url.clone(),
            });

            let external_message_encoded = bincode::serialize(&test_external_message).unwrap();

            let test_message = Message::<SeqTypes> {
                sender: BLSPubKey::generated_from_seed_indexed([0; 32], 0).0,
                kind: MessageKind::External(external_message_encoded),
            };

            upgrade_lock.serialize(&test_message).await.unwrap()
        };
        drop(url_receiver);

        let mut task = CdnReceiveMessagesTask::new(
            TestConnectedNetworkConsumer(Ok(vec![test_hotshot_message_serialized])),
            url_sender.clone(),
        );

        let task_handle = task.task_handle.take();

        if let Some(task_handle) = task_handle {
            assert_eq!(task_handle.timeout(Duration::from_millis(50)).await, Ok(()));
        }
    }

    /// [TestConnectedNetworkPublisher] is a test implementation of the
    /// [ConnectedNetworkPublisher] trait that allows for the simulation of
    /// network messages being sent.
    struct TestConnectedNetworkPublisher(Sender<Vec<u8>>);

    #[async_trait::async_trait]
    impl ConnectedNetworkPublisher<BLSPubKey> for TestConnectedNetworkPublisher {
        async fn wait_for_ready(&self) {}

        async fn broadcast_message(
            &self,
            message: Vec<u8>,
            _topic: hotshot_types::traits::network::Topic,
            _broadcast_delay: BroadcastDelay,
        ) -> Result<(), NetworkError> {
            let mut sender = self.0.clone();
            let send_result = sender.send(message).await;
            send_result.map_err(|_| NetworkError::ChannelSend)
        }
    }

    /// [test_cdn_broadcast_roll_call_task] is a test that verifies that the
    /// task broadcasts a RollCallRequest message to the network.  It also
    /// verifies that the task is short-lived, as it does not need to persist
    /// beyond it's initial request.
    #[async_std::test]
    async fn test_cdn_broadcast_roll_call_task() {
        let upgrade_lock = UpgradeLock::<SeqTypes, SequencerVersions>::new();
        let (message_sender, message_receiver) = mpsc::channel(1);

        let task = BroadcastRollCallTask::new(
            TestConnectedNetworkPublisher(message_sender),
            BLSPubKey::generated_from_seed_indexed([0; 32], 0).0,
        );

        let mut message_receiver = message_receiver;
        let next_message = message_receiver.next().await.unwrap();
        let next_message = upgrade_lock
            .deserialize::<Message<SeqTypes>>(&next_message)
            .await
            .unwrap();

        let external_message = match next_message.kind {
            MessageKind::External(external_message) => external_message,
            _ => panic!("unexpected message kind"),
        };

        let external_message = bincode::deserialize::<ExternalMessage>(&external_message).unwrap();

        match external_message {
            ExternalMessage::RollCallRequest(public_key) => {
                assert_eq!(
                    public_key,
                    BLSPubKey::generated_from_seed_indexed([0; 32], 0).0
                );
            }
            _ => panic!("unexpected external message"),
        }

        let mut task = task;
        let task_handle = task.task_handle.take();

        if let Some(task_handle) = task_handle {
            assert_eq!(task_handle.timeout(Duration::from_millis(50)).await, Ok(()));
        }
    }
}
