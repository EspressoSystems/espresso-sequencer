use std::collections::HashMap;

use async_compatibility_layer::{
    art::{async_sleep, async_spawn},
    channel::UnboundedSender,
};
use libp2p::request_response::{Event, Message, OutboundRequestId, ResponseChannel};
use libp2p_identity::PeerId;
use tracing::{debug, error, warn};

use super::exponential_backoff::ExponentialBackoff;
use crate::network::{ClientRequest, NetworkEvent};

/// Request to direct message a peert
#[derive(Debug)]
pub struct DMRequest {
    /// the recv-ers peer id
    pub peer_id: PeerId,
    /// the data
    pub data: Vec<u8>,
    /// backoff since last attempted request
    pub backoff: ExponentialBackoff,
    /// the number of remaining retries before giving up
    pub(crate) retry_count: u8,
}

/// Wrapper metadata around libp2p's request response
/// usage: direct message peer
#[derive(Debug, Default)]
pub struct DMBehaviour {
    /// In progress queries
    in_progress_rr: HashMap<OutboundRequestId, DMRequest>,
}

/// Lilst of direct message output events
#[derive(Debug)]
pub enum DMEvent {
    /// We received as Direct Request
    DirectRequest(Vec<u8>, PeerId, ResponseChannel<Vec<u8>>),
    /// We received a Direct Response
    DirectResponse(Vec<u8>, PeerId),
}

impl DMBehaviour {
    /// handle a direct message event
    pub(crate) fn handle_dm_event(
        &mut self,
        event: Event<Vec<u8>, Vec<u8>>,
        retry_tx: Option<UnboundedSender<ClientRequest>>,
    ) -> Option<NetworkEvent> {
        match event {
            Event::InboundFailure {
                peer,
                request_id: _,
                error,
            } => {
                error!("Inbound message failure from {:?}: {:?}", peer, error);
                None
            }
            Event::OutboundFailure {
                peer,
                request_id,
                error,
            } => {
                error!("Outbound message failure to {:?}: {:?}", peer, error);
                if let Some(mut req) = self.in_progress_rr.remove(&request_id) {
                    if req.retry_count == 0 {
                        return None;
                    }
                    req.retry_count -= 1;
                    if let Some(retry_tx) = retry_tx {
                        async_spawn(async move {
                            async_sleep(req.backoff.next_timeout(false)).await;
                            let _ = retry_tx
                                .send(ClientRequest::DirectRequest {
                                    pid: peer,
                                    contents: req.data,
                                    retry_count: req.retry_count,
                                })
                                .await;
                        });
                    }
                }
                None
            }
            Event::Message { message, peer, .. } => match message {
                Message::Request {
                    request: msg,
                    channel,
                    ..
                } => {
                    debug!("Received direct request {:?}", msg);
                    // receiver, not initiator.
                    // don't track. If we are disconnected, sender will reinitiate
                    Some(NetworkEvent::DirectRequest(msg, peer, channel))
                }
                Message::Response {
                    request_id,
                    response: msg,
                } => {
                    // success, finished.
                    if let Some(req) = self.in_progress_rr.remove(&request_id) {
                        debug!("Received direct response {:?}", msg);
                        Some(NetworkEvent::DirectResponse(msg, req.peer_id))
                    } else {
                        warn!("Received response for unknown request id {:?}", request_id);
                        None
                    }
                }
            },
            e @ Event::ResponseSent { .. } => {
                debug!("Response sent {:?}", e);
                None
            }
        }
    }
}

impl DMBehaviour {
    /// Add a direct request for a given peer
    pub fn add_direct_request(&mut self, mut req: DMRequest, request_id: OutboundRequestId) {
        if req.retry_count == 0 {
            return;
        }

        req.retry_count -= 1;

        debug!("Adding direct request {:?}", req);

        self.in_progress_rr.insert(request_id, req);
    }
}
