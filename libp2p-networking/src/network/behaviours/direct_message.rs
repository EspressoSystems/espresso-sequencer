use std::{
    collections::{HashMap, VecDeque},
    task::Poll,
};

use libp2p::{
    request_response::{Behaviour, Event, Message, RequestId, ResponseChannel},
    swarm::{NetworkBehaviour, THandlerInEvent, THandlerOutEvent, ToSwarm},
    Multiaddr,
};
use libp2p_identity::PeerId;
use tracing::{error, info};

use super::{
    direct_message_codec::{DirectMessageCodec, DirectMessageRequest, DirectMessageResponse},
    exponential_backoff::ExponentialBackoff,
};

/// Request to direct message a peert
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
pub struct DMBehaviour {
    /// The wrapped behaviour
    request_response: Behaviour<DirectMessageCodec>,
    /// In progress queries
    in_progress_rr: HashMap<RequestId, DMRequest>,
    /// Failed queries to be retried
    failed_rr: VecDeque<DMRequest>,
    /// lsit of out events for parent behaviour
    out_event_queue: Vec<DMEvent>,
}

/// Lilst of direct message output events
#[derive(Debug)]
pub enum DMEvent {
    /// We received as Direct Request
    DirectRequest(Vec<u8>, PeerId, ResponseChannel<DirectMessageResponse>),
    /// We received a Direct Response
    DirectResponse(Vec<u8>, PeerId),
}

impl DMBehaviour {
    fn handle_dm_event(&mut self, event: Event<DirectMessageRequest, DirectMessageResponse>) {
        match event {
            Event::InboundFailure {
                peer,
                request_id,
                error,
            } => {
                error!(
                    "inbound failure to send message to {:?} with error {:?}",
                    peer, error
                );
                if let Some(mut req) = self.in_progress_rr.remove(&request_id) {
                    req.backoff.start_next(false);
                    self.failed_rr.push_back(req);
                }
            }
            Event::OutboundFailure {
                peer,
                request_id,
                error,
            } => {
                error!(
                    "outbound failure to send message to {:?} with error {:?}",
                    peer, error
                );
                if let Some(mut req) = self.in_progress_rr.remove(&request_id) {
                    req.backoff.start_next(false);
                    self.failed_rr.push_back(req);
                }
            }
            Event::Message { message, peer, .. } => match message {
                Message::Request {
                    request: DirectMessageRequest(msg),
                    channel,
                    ..
                } => {
                    info!("recv-ed DIRECT REQUEST {:?}", msg);
                    // receiver, not initiator.
                    // don't track. If we are disconnected, sender will reinitiate
                    self.out_event_queue
                        .push(DMEvent::DirectRequest(msg, peer, channel));
                }
                Message::Response {
                    request_id,
                    response: DirectMessageResponse(msg),
                } => {
                    // success, finished.
                    if let Some(req) = self.in_progress_rr.remove(&request_id) {
                        info!("recv-ed DIRECT RESPONSE {:?}", msg);
                        self.out_event_queue
                            .push(DMEvent::DirectResponse(msg, req.peer_id));
                    } else {
                        error!("recv-ed a direct response, but is no longer tracking message!");
                    }
                }
            },
            e @ Event::ResponseSent { .. } => {
                info!(?e, " sending response");
            }
        }
    }
}

impl NetworkBehaviour for DMBehaviour {
    type ConnectionHandler = <Behaviour<DirectMessageCodec> as NetworkBehaviour>::ConnectionHandler;

    type ToSwarm = DMEvent;

    fn on_swarm_event(
        &mut self,
        event: libp2p::swarm::derive_prelude::FromSwarm<'_, Self::ConnectionHandler>,
    ) {
        self.request_response.on_swarm_event(event);
    }

    fn on_connection_handler_event(
        &mut self,
        peer_id: PeerId,
        connection_id: libp2p::swarm::derive_prelude::ConnectionId,
        event: THandlerOutEvent<Self>,
    ) {
        self.request_response
            .on_connection_handler_event(peer_id, connection_id, event);
    }

    fn poll(
        &mut self,
        cx: &mut std::task::Context<'_>,
        params: &mut impl libp2p::swarm::PollParameters,
    ) -> Poll<ToSwarm<DMEvent, THandlerInEvent<Self>>> {
        while let Some(req) = self.failed_rr.pop_front() {
            if req.backoff.is_expired() {
                self.add_direct_request(req);
            } else {
                self.failed_rr.push_back(req);
            }
        }
        while let Poll::Ready(ready) =
            NetworkBehaviour::poll(&mut self.request_response, cx, params)
        {
            match ready {
                // NOTE: this generates request
                ToSwarm::GenerateEvent(e) => {
                    self.handle_dm_event(e);
                }
                ToSwarm::Dial { opts } => {
                    return Poll::Ready(ToSwarm::Dial { opts });
                }
                ToSwarm::NotifyHandler {
                    peer_id,
                    handler,
                    event,
                } => {
                    return Poll::Ready(ToSwarm::NotifyHandler {
                        peer_id,
                        handler,
                        event,
                    });
                }
                ToSwarm::CloseConnection {
                    peer_id,
                    connection,
                } => {
                    return Poll::Ready(ToSwarm::CloseConnection {
                        peer_id,
                        connection,
                    });
                }
                ToSwarm::ListenOn { opts } => {
                    return Poll::Ready(ToSwarm::ListenOn { opts });
                }
                ToSwarm::RemoveListener { id } => {
                    return Poll::Ready(ToSwarm::RemoveListener { id });
                }
                ToSwarm::NewExternalAddrCandidate(c) => {
                    return Poll::Ready(ToSwarm::NewExternalAddrCandidate(c));
                }
                ToSwarm::ExternalAddrConfirmed(c) => {
                    return Poll::Ready(ToSwarm::ExternalAddrConfirmed(c));
                }
                ToSwarm::ExternalAddrExpired(c) => {
                    return Poll::Ready(ToSwarm::ExternalAddrExpired(c));
                }
            }
        }
        if !self.out_event_queue.is_empty() {
            return Poll::Ready(ToSwarm::GenerateEvent(self.out_event_queue.remove(0)));
        }
        Poll::Pending
    }

    fn handle_pending_inbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> Result<(), libp2p::swarm::ConnectionDenied> {
        self.request_response.handle_pending_inbound_connection(
            connection_id,
            local_addr,
            remote_addr,
        )
    }

    fn handle_established_inbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        self.request_response.handle_established_inbound_connection(
            connection_id,
            peer,
            local_addr,
            remote_addr,
        )
    }

    fn handle_pending_outbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        maybe_peer: Option<PeerId>,
        addresses: &[Multiaddr],
        effective_role: libp2p::core::Endpoint,
    ) -> Result<Vec<Multiaddr>, libp2p::swarm::ConnectionDenied> {
        self.request_response.handle_pending_outbound_connection(
            connection_id,
            maybe_peer,
            addresses,
            effective_role,
        )
    }

    fn handle_established_outbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        addr: &Multiaddr,
        role_override: libp2p::core::Endpoint,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        self.request_response
            .handle_established_outbound_connection(connection_id, peer, addr, role_override)
    }
}

impl DMBehaviour {
    /// Create new behaviour based on request response
    #[must_use]
    pub fn new(request_response: Behaviour<DirectMessageCodec>) -> Self {
        Self {
            request_response,
            in_progress_rr: HashMap::default(),
            failed_rr: VecDeque::default(),
            out_event_queue: Vec::default(),
        }
    }

    /// Add address to request response behaviour
    pub fn add_address(&mut self, peer_id: &PeerId, address: Multiaddr) {
        self.request_response.add_address(peer_id, address);
    }

    /// Remove address from request response behaviour
    pub fn remove_address(&mut self, peer_id: &PeerId, address: &Multiaddr) {
        self.request_response.remove_address(peer_id, address);
    }

    /// Add a direct request for a given peer
    pub fn add_direct_request(&mut self, mut req: DMRequest) {
        if req.retry_count == 0 {
            return;
        }

        req.retry_count -= 1;

        let request_id = self
            .request_response
            .send_request(&req.peer_id, DirectMessageRequest(req.data.clone()));
        info!("direct message request with id {:?}", request_id);

        self.in_progress_rr.insert(request_id, req);
    }

    /// Add a direct response for a channel
    pub fn add_direct_response(
        &mut self,
        chan: ResponseChannel<DirectMessageResponse>,
        msg: Vec<u8>,
    ) {
        let res = self
            .request_response
            .send_response(chan, DirectMessageResponse(msg));
        if let Err(e) = res {
            error!("Error replying to direct message. {:?}", e);
        }
    }
}
