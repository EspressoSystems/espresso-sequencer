// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::collections::HashMap;

use futures::channel::oneshot::Sender;
use hotshot_types::request_response::{Request, Response};
use libp2p::request_response::{Message, OutboundRequestId};

use crate::network::NetworkEvent;

#[derive(Default, Debug)]
/// Handler for request response messages
pub(crate) struct RequestResponseState {
    /// Map requests to the their response channels
    request_map: HashMap<OutboundRequestId, Sender<Option<Response>>>,
}

impl RequestResponseState {
    /// Handles messages from the `request_response` behaviour by sending them to the application
    pub fn handle_request_response(
        &mut self,
        event: libp2p::request_response::Event<Request, Response>,
    ) -> Option<NetworkEvent> {
        match event {
            libp2p::request_response::Event::Message { peer: _, message } => match message {
                Message::Request {
                    request_id: _,
                    request,
                    channel,
                } => Some(NetworkEvent::ResponseRequested(request, channel)),
                Message::Response {
                    request_id,
                    response,
                } => {
                    let chan = self.request_map.remove(&request_id)?;
                    if chan.send(Some(response)).is_err() {
                        tracing::warn!("Failed to send response to client, channel closed.");
                    }
                    None
                }
            },
            libp2p::request_response::Event::OutboundFailure {
                peer: _,
                request_id,
                error,
            } => {
                tracing::warn!("Error Sending Request {:?}", error);
                let chan = self.request_map.remove(&request_id)?;
                if chan.send(None).is_err() {
                    tracing::warn!("Failed to send response to client, channel closed.");
                }
                None
            }
            libp2p::request_response::Event::InboundFailure { .. }
            | libp2p::request_response::Event::ResponseSent { .. } => None,
        }
    }
    /// Add a requests return channel to the map of pending requests
    pub fn add_request(&mut self, id: OutboundRequestId, chan: Sender<Option<Response>>) {
        self.request_map.insert(id, chan);
    }
}
