//! Should probably rename this to "external" or something

use std::sync::Arc;

use anyhow::{Context, Result};
use espresso_types::{PubKey, SeqTypes};
use hotshot::types::{BLSPubKey, Event, EventType};
use hotshot_types::traits::network::ConnectedNetwork;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum ExternalMessage {
    /// A request to roll call
    /// Has the public key of whatever is asking for the roll call (so it can get a response)
    RollCallRequest(BLSPubKey),

    /// A response to a roll call
    /// Has the identifier of the node that is responding
    RollCallResponse(String),
}

pub struct RollCall<N: ConnectedNetwork<PubKey>> {
    // The network to respond over
    pub network: Arc<N>,

    // My node's public identifier
    pub identifier: String,
}

impl<N: ConnectedNetwork<PubKey>> RollCall<N> {
    pub async fn handle_event(&self, event: &Event<SeqTypes>) -> Result<()> {
        // Check if the event is an external message
        if let EventType::ExternalMessageReceived(external_message_bytes) = &event.event {
            // Deserialize the external message
            let external_message = bincode::deserialize(external_message_bytes)
                .with_context(|| "Failed to deserialize external message")?;

            // Match the type
            match external_message {
                ExternalMessage::RollCallRequest(pub_key) => {
                    // If it's a roll call request, send our identifier
                    let response = ExternalMessage::RollCallResponse(self.identifier.clone());

                    // Serialize the response
                    let response_bytes = bincode::serialize(&response)
                        .with_context(|| "Failed to serialize roll call response")?;

                    // Send the response
                    self.network.direct_message(response_bytes, pub_key).await?;
                }

                _ => {
                    return Err(anyhow::anyhow!("Unknown external message type"));
                }
            }
        };

        Ok(())
    }
}
