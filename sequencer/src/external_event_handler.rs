use std::{collections::BTreeSet, sync::Arc};

use anyhow::{Context, Result};
use async_compatibility_layer::channel::{Receiver, Sender};
use async_std::task::{self, JoinHandle};
use espresso_types::PubKey;
use hotshot::types::BLSPubKey;
use hotshot_types::traits::network::{BroadcastDelay, ConnectedNetwork};
use serde::{Deserialize, Serialize};
use url::Url;

/// An external message that can be sent to or received from a node
#[derive(Serialize, Deserialize, Clone)]
pub enum ExternalMessage {
    /// A request for a node to respond with its identifier
    /// Contains the public key of the node that is requesting the roll call
    RollCallRequest(BLSPubKey),

    /// A response to a roll call request
    /// Contains the identifier of the node
    RollCallResponse(RollCallInfo),
}

/// Information about a node that is used in a roll call response
#[derive(Serialize, Deserialize, Clone)]
pub struct RollCallInfo {
    // The public API URL of the node
    pub public_api_url: Option<Url>,
}

/// The external event handler state
pub struct ExternalEventHandler {
    // The `RollCallInfo` of the node (used in the roll call response)
    pub roll_call_info: RollCallInfo,

    // The tasks that are running
    pub tasks: Vec<JoinHandle<()>>,

    // The outbound message queue
    pub outbound_message_sender: Sender<OutboundMessage>,
}

// The different types of outbound messages (broadcast or direct)
pub enum OutboundMessage {
    Direct(Vec<u8>, PubKey),
    Broadcast(Vec<u8>),
}

impl ExternalEventHandler {
    /// Creates a new `ExternalEventHandler` with the given network and roll call info
    pub fn new<N: ConnectedNetwork<PubKey>>(
        network: Arc<N>,
        roll_call_info: RollCallInfo,
    ) -> Result<Self> {
        // Create the outbound message queue
        let (outbound_message_sender, outbound_message_receiver) =
            async_compatibility_layer::channel::bounded(10);

        // Spawn the outbound message handling loop
        let outbound_message_loop = async_std::task::spawn(Self::outbound_message_loop(
            outbound_message_receiver,
            network,
        ));

        // We just started, so queue an outbound RollCall message (if we have a public API URL)
        if roll_call_info.public_api_url.is_some() {
            let roll_call_message = ExternalMessage::RollCallResponse(roll_call_info.clone());
            let roll_call_message_bytes = bincode::serialize(&roll_call_message)
                .with_context(|| "Failed to serialize roll call message for initial broadcast")?;
            outbound_message_sender
                .try_send(OutboundMessage::Broadcast(roll_call_message_bytes))
                .with_context(|| "External outbound message queue is somehow full")?;
        }

        Ok(Self {
            roll_call_info,
            tasks: vec![outbound_message_loop],
            outbound_message_sender,
        })
    }

    /// Handles an event
    ///
    /// # Errors
    /// If the message type is unknown or if there is an error serializing or deserializing the message
    pub fn handle_event(&self, external_message_bytes: &[u8]) -> Result<()> {
        // Deserialize the external message
        let external_message = bincode::deserialize(external_message_bytes)
            .with_context(|| "Failed to deserialize external message")?;

        // Match the type
        match external_message {
            ExternalMessage::RollCallRequest(pub_key) => {
                if self.roll_call_info.public_api_url.is_none() {
                    // We don't have a public API URL, so we can't respond to the roll call
                    return Ok(());
                }

                // If it's a roll call request, send our information (if we have a public API URL)
                let response = ExternalMessage::RollCallResponse(self.roll_call_info.clone());

                // Serialize the response
                let response_bytes = bincode::serialize(&response)
                    .with_context(|| "Failed to serialize roll call response")?;

                // Send the response
                self.outbound_message_sender
                    .try_send(OutboundMessage::Direct(response_bytes, pub_key))
                    .with_context(|| "External outbound message queue is full")?;
            }

            _ => {
                return Err(anyhow::anyhow!("Unknown external message type"));
            }
        }
        Ok(())
    }

    /// The main loop for sending outbound messages.
    async fn outbound_message_loop<N: ConnectedNetwork<PubKey>>(
        mut receiver: Receiver<OutboundMessage>,
        network: Arc<N>,
    ) {
        while let Ok(message) = receiver.recv().await {
            // Match the message type
            match message {
                OutboundMessage::Direct(message, recipient) => {
                    // Send the message directly to the recipient
                    if let Err(err) = network.direct_message(message, recipient).await {
                        tracing::error!("Failed to send message: {:?}", err);
                    };
                }

                OutboundMessage::Broadcast(message) => {
                    // Broadcast the message to the global topic
                    if let Err(err) = network
                        .broadcast_message(message, BTreeSet::new(), BroadcastDelay::None)
                        .await
                    {
                        tracing::error!("Failed to broadcast message: {:?}", err);
                    };
                }
            }
        }

        tracing::error!("External event handler loop exited unexpectedly");
    }
}

impl Drop for ExternalEventHandler {
    fn drop(&mut self) {
        // Cancel all tasks
        for task in self.tasks.drain(..) {
            task::block_on(task.cancel());
        }
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use std::{collections::BTreeSet, sync::Arc};

    use async_std::{sync::Mutex, task::sleep};
    use espresso_types::PubKey;
    use hotshot::{traits::NetworkError, types::SignatureKey};
    use hotshot_types::{
        traits::{
            network::{BroadcastDelay, ConnectedNetwork},
            node_implementation::NodeType,
        },
        BoxSyncFuture,
    };
    use url::Url;

    use crate::external_event_handler::{ExternalEventHandler, RollCallInfo};

    type Queue<T> = Arc<Mutex<Vec<T>>>;

    /// A `TestNetwork` is a `ConnectedNetwork` that stores messages in an internal queue to
    /// be checked against later
    #[derive(Clone)]
    struct TestNetwork {
        received_broadcast_messages: Queue<Vec<u8>>,
        received_direct_messages: Queue<(PubKey, Vec<u8>)>,
    }

    #[async_trait]
    impl ConnectedNetwork<PubKey> for TestNetwork {
        /// Pushes to an internal queue we can check later
        async fn broadcast_message(
            &self,
            message: Vec<u8>,
            _recipients: BTreeSet<PubKey>,
            _broadcast_delay: BroadcastDelay,
        ) -> Result<(), NetworkError> {
            self.received_broadcast_messages.lock().await.push(message);
            Ok(())
        }

        /// Pushes the message and the recipient to an internal queue we can check later
        async fn direct_message(
            &self,
            message: Vec<u8>,
            recipient: PubKey,
        ) -> Result<(), NetworkError> {
            self.received_direct_messages
                .lock()
                .await
                .push((recipient, message));
            Ok(())
        }

        /// The following are no-ops
        fn pause(&self) {}
        fn resume(&self) {}
        async fn wait_for_ready(&self) {}
        fn shut_down<'a, 'b>(&'a self) -> BoxSyncFuture<'b, ()>
        where
            'a: 'b,
            Self: 'b,
        {
            unimplemented!()
        }
        async fn da_broadcast_message(
            &self,
            _message: Vec<u8>,
            _recipients: BTreeSet<PubKey>,
            _broadcast_delay: BroadcastDelay,
        ) -> Result<(), NetworkError> {
            unimplemented!()
        }
        async fn recv_msgs(&self) -> Result<Vec<Vec<u8>>, NetworkError> {
            unimplemented!()
        }
        async fn update_view<'a, TYPES>(&'a self, _view: u64, _membership: &TYPES::Membership)
        where
            TYPES: NodeType<SignatureKey = PubKey> + 'a,
        {
        }
    }

    // A helper function for creating a new test network and event handler
    async fn new_test_network_and_event_handler() -> (Arc<TestNetwork>, ExternalEventHandler) {
        // Create a new test network
        let network = Arc::new(TestNetwork {
            received_broadcast_messages: Arc::new(Mutex::new(Vec::new())),
            received_direct_messages: Arc::new(Mutex::new(Vec::new())),
        });

        // Create a new ExternalEventHandler
        let network_clone = network.clone();
        let external_event_handler = ExternalEventHandler::new(
            network_clone,
            RollCallInfo {
                public_api_url: Some(Url::parse("http://localhost:8080").unwrap()),
            },
        )
        .expect("Failed to create ExternalEventHandler");

        // Wait for the broadcast message to be received
        sleep(std::time::Duration::from_secs(1)).await;

        (network, external_event_handler)
    }

    /// This test is supposed to test that the `ExternalEventHandler` sends a proper broadcast message on startup
    #[async_std::test]
    async fn test_broadcast_on_start() {
        // Create a new test network and event handler
        let (network, _external_event_handler) = new_test_network_and_event_handler().await;

        // Make sure we've received the broadcast message
        let message = network
            .received_broadcast_messages
            .lock()
            .await
            .pop()
            .expect("No message was received");

        // Make sure we haven't received any direct messages
        assert!(network.received_direct_messages.lock().await.is_empty());

        // Deserialize the message
        let external_message: crate::external_event_handler::ExternalMessage =
            bincode::deserialize(&message).expect("Failed to deserialize message");

        // Make sure the message is a RollCallResponse
        match external_message {
            crate::external_event_handler::ExternalMessage::RollCallResponse(_) => {}
            _ => panic!("Expected RollCallResponse, got something else"),
        }
    }

    /// This test is supposed to test that the `ExternalEventHandler` sends a proper direct message in response to a roll call request
    #[async_std::test]
    async fn test_direct_on_request() {
        // Create a new test network and event handler
        let (network, external_event_handler) = new_test_network_and_event_handler().await;

        // Create a roll call request
        let authority_public_key = PubKey::generated_from_seed_indexed([1; 32], 32);
        let roll_call_request =
            crate::external_event_handler::ExternalMessage::RollCallRequest(authority_public_key.0);

        // Handle the roll call request
        external_event_handler
            .handle_event(
                &bincode::serialize(&roll_call_request)
                    .expect("Failed to serialize RollCallRequest"),
            )
            .expect("Failed to handle `RollCallRequest`");

        // Wait for the direct message to be received
        sleep(std::time::Duration::from_secs(1)).await;

        // Make sure we've received the direct message
        let (recipient, message) = network
            .received_direct_messages
            .lock()
            .await
            .pop()
            .expect("No message was received");

        // Make sure the recipient is the authority public key
        assert!(
            recipient == authority_public_key.0,
            "Recipient was not authority public key"
        );

        // Deserialize the message
        let external_message: crate::external_event_handler::ExternalMessage =
            bincode::deserialize(&message).expect("Failed to deserialize message");

        // Make sure the message is a RollCallResponse
        match external_message {
            crate::external_event_handler::ExternalMessage::RollCallResponse(_) => {}
            _ => panic!("Expected RollCallResponse, got something else"),
        }
    }

    /// This test is supposed to test that the `ExternalEventHandler` properly kills its task when dropped
    #[async_std::test]
    async fn test_drop_kills_task() {
        // Create a new test network and event handler
        let (network, external_event_handler) = new_test_network_and_event_handler().await;

        // Clone the sender handle
        let external_event_handler_clone = external_event_handler.outbound_message_sender.clone();

        // Drop the ExternalEventHandler. This should kill the task
        drop(external_event_handler);

        // Send a message to the message queue
        external_event_handler_clone
            .try_send(crate::external_event_handler::OutboundMessage::Broadcast(
                vec![],
            ))
            .expect_err("Message was sent to a dropped message queue");

        // Wait for the task to exit
        sleep(std::time::Duration::from_secs(1)).await;

        // Make sure we have not received any messages besides the initial broadcast
        assert!(network.received_broadcast_messages.lock().await.len() == 1);
        assert!(network.received_direct_messages.lock().await.is_empty());
    }
}
