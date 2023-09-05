use std::{
    collections::{HashSet, VecDeque},
    task::Poll,
};

use libp2p::{
    gossipsub::{Behaviour, Event, IdentTopic, TopicHash},
    swarm::{NetworkBehaviour, PollParameters, THandlerInEvent, THandlerOutEvent, ToSwarm},
    Multiaddr,
};
use libp2p_identity::PeerId;

use tracing::{error, info, warn};

use super::exponential_backoff::ExponentialBackoff;

/// wrapper metadata around libp2p's gossip protocol
pub struct GossipBehaviour {
    /// Timeout trackidng when to retry gossip
    backoff: ExponentialBackoff,
    /// The in progress gossip queries
    in_progress_gossip: VecDeque<(IdentTopic, Vec<u8>)>,
    /// The gossip behaviour
    gossipsub: Behaviour,
    /// Output events to parent behavioru
    out_event_queue: Vec<GossipEvent>,
    /// Set of topics we are subscribed to
    subscribed_topics: HashSet<String>,
}

/// Output event
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GossipEvent {
    /// We received a gossip
    GossipMsg(Vec<u8>, TopicHash),
}

impl GossipBehaviour {
    fn gossip_handle_event(&mut self, event: Event) {
        match event {
            Event::Message { message, .. } => {
                // if we get an event from the gossipsub behaviour, push it
                // onto the event queue (which will get popped during poll)
                // and propagated back to the overall behaviour
                self.out_event_queue
                    .push(GossipEvent::GossipMsg(message.data, message.topic));
            }
            Event::Subscribed { topic, .. } => {
                info!("subscribed to topic {}", topic);
            }
            Event::Unsubscribed { topic, .. } => {
                info!("unsubscribed to topic {}", topic);
            }
            Event::GossipsubNotSupported { peer_id } => {
                error!("gossipsub not supported on {}!", peer_id);
            }
        }
    }
}

impl NetworkBehaviour for GossipBehaviour {
    type ConnectionHandler = <Behaviour as NetworkBehaviour>::ConnectionHandler;

    type ToSwarm = GossipEvent;

    fn on_swarm_event(
        &mut self,
        event: libp2p::swarm::derive_prelude::FromSwarm<'_, Self::ConnectionHandler>,
    ) {
        self.gossipsub.on_swarm_event(event);
    }

    fn poll(
        &mut self,
        cx: &mut std::task::Context<'_>,
        params: &mut impl PollParameters,
    ) -> Poll<ToSwarm<GossipEvent, THandlerInEvent<Self>>> {
        // retry sending shit
        if self.backoff.is_expired() {
            let published = self.drain_publish_gossips();
            self.backoff.start_next(published);
        }
        if let Poll::Ready(ready) = NetworkBehaviour::poll(&mut self.gossipsub, cx, params) {
            match ready {
                ToSwarm::GenerateEvent(e) => {
                    // add event to event queue which will be subsequently popped off.
                    self.gossip_handle_event(e);
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

    fn on_connection_handler_event(
        &mut self,
        peer_id: PeerId,
        connection_id: libp2p::swarm::derive_prelude::ConnectionId,
        event: THandlerOutEvent<Self>,
    ) {
        self.gossipsub
            .on_connection_handler_event(peer_id, connection_id, event);
    }

    fn handle_pending_inbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> Result<(), libp2p::swarm::ConnectionDenied> {
        self.gossipsub
            .handle_pending_inbound_connection(connection_id, local_addr, remote_addr)
    }

    fn handle_established_inbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        self.gossipsub.handle_established_inbound_connection(
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
        self.gossipsub.handle_pending_outbound_connection(
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
        self.gossipsub.handle_established_outbound_connection(
            connection_id,
            peer,
            addr,
            role_override,
        )
    }
}

impl GossipBehaviour {
    /// Create new gossip behavioru based on gossipsub
    #[must_use]
    pub fn new(gossipsub: Behaviour) -> Self {
        Self {
            backoff: ExponentialBackoff::default(),
            in_progress_gossip: VecDeque::default(),
            gossipsub,
            out_event_queue: Vec::default(),
            subscribed_topics: HashSet::default(),
        }
    }

    /// Publish a given gossip
    pub fn publish_gossip(&mut self, topic: IdentTopic, contents: Vec<u8>) {
        let res = self.gossipsub.publish(topic.clone(), contents.clone());
        if res.is_err() {
            error!("error publishing gossip message {:?}", res);
            self.in_progress_gossip.push_back((topic, contents));
        }
    }

    /// Subscribe to a given topic
    pub fn subscribe_gossip(&mut self, t: &str) {
        if self.subscribed_topics.contains(t) {
            warn!(
                "tried to subscribe to already subscribed topic {:?}. Noop.",
                t
            );
        } else if self.gossipsub.subscribe(&IdentTopic::new(t)).is_err() {
            error!("error subscribing to topic {}", t);
        } else {
            info!("subscribed req to {:?}", t);
            self.subscribed_topics.insert(t.to_string());
        }
    }

    /// Unsubscribe from a given topic
    pub fn unsubscribe_gossip(&mut self, t: &str) {
        if self.subscribed_topics.contains(t) {
            if self.gossipsub.unsubscribe(&IdentTopic::new(t)).is_err() {
                error!("error unsubscribing to topic {}", t);
            } else {
                self.subscribed_topics.remove(t);
            }
        } else {
            warn!("tried to unsubscribe to untracked topic {:?}. Noop.", t);
        }
    }

    /// Attempt to drain the internal gossip list, publishing each gossip
    pub fn drain_publish_gossips(&mut self) -> bool {
        let mut r_val = true;

        while let Some((topic, contents)) = self.in_progress_gossip.pop_front() {
            let res = self.gossipsub.publish(topic.clone(), contents.clone());
            if res.is_err() {
                self.in_progress_gossip.push_back((topic, contents));
                r_val = false;
                break;
            }
        }
        r_val
    }
}
