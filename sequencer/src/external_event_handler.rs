//! Should probably rename this to "external" or something

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
