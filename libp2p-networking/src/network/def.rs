use std::num::NonZeroUsize;

use futures::channel::oneshot::Sender;
use libp2p::{
    autonat,
    gossipsub::{Behaviour as GossipBehaviour, Event as GossipEvent, IdentTopic},
    identify::{Behaviour as IdentifyBehaviour, Event as IdentifyEvent},
    request_response::{cbor, OutboundRequestId, ResponseChannel},
    Multiaddr,
};
use libp2p_identity::PeerId;
use libp2p_swarm_derive::NetworkBehaviour;
use tracing::{debug, error};

use super::{
    behaviours::{
        dht::{DHTBehaviour, DHTEvent, KadPutQuery},
        exponential_backoff::ExponentialBackoff,
        request_response::{Request, Response},
    },
    NetworkEventInternal,
};

/// Overarching network behaviour performing:
/// - network topology discovoery
/// - direct messaging
/// - p2p broadcast
/// - connection management
#[derive(NetworkBehaviour, custom_debug::Debug)]
#[behaviour(to_swarm = "NetworkEventInternal")]
pub struct NetworkDef {
    /// purpose: broadcasting messages to many peers
    /// NOTE gossipsub works ONLY for sharing messages right now
    /// in the future it may be able to do peer discovery and routing
    /// <https://github.com/libp2p/rust-libp2p/issues/2398>
    #[debug(skip)]
    gossipsub: GossipBehaviour,

    /// purpose: peer routing
    /// purpose: storing pub key <-> peer id bijection
    #[debug(skip)]
    pub dht: DHTBehaviour,

    /// purpose: identifying the addresses from an outside POV
    #[debug(skip)]
    identify: IdentifyBehaviour,

    /// purpose: directly messaging peer
    #[debug(skip)]
    pub direct_message: libp2p::request_response::cbor::Behaviour<Vec<u8>, Vec<u8>>,

    /// Behaviour for requesting and receiving data
    #[debug(skip)]
    pub request_response: libp2p::request_response::cbor::Behaviour<Request, Response>,

    /// Auto NAT behaviour to determine if we are publically reachable and
    /// by which address
    #[debug(skip)]
    pub autonat: libp2p::autonat::Behaviour,
}

impl NetworkDef {
    /// Create a new instance of a `NetworkDef`
    #[must_use]
    pub fn new(
        gossipsub: GossipBehaviour,
        dht: DHTBehaviour,
        identify: IdentifyBehaviour,
        direct_message: cbor::Behaviour<Vec<u8>, Vec<u8>>,
        request_response: cbor::Behaviour<Request, Response>,
        autonat: autonat::Behaviour,
    ) -> NetworkDef {
        Self {
            gossipsub,
            dht,
            identify,
            direct_message,
            request_response,
            autonat,
        }
    }
}

/// Address functions
impl NetworkDef {
    /// Add an address
    pub fn add_address(&mut self, peer_id: &PeerId, address: Multiaddr) {
        // NOTE to get this address to play nice with the other
        // behaviours using the DHT for ouring
        // we only need to add this address to the DHT since it
        // is always enabled. If it were not always enabled,
        // we would need to manually add the address to
        // the direct message behaviour
        self.dht.add_address(peer_id, address);
    }
}

/// Gossip functions
impl NetworkDef {
    /// Publish a given gossip
    pub fn publish_gossip(&mut self, topic: IdentTopic, contents: Vec<u8>) {
        if let Err(e) = self.gossipsub.publish(topic, contents) {
            tracing::warn!("Failed to publish gossip message. Error: {:?}", e);
        }
    }
    /// Subscribe to a given topic
    pub fn subscribe_gossip(&mut self, t: &str) {
        if let Err(e) = self.gossipsub.subscribe(&IdentTopic::new(t)) {
            error!("Failed to subscribe to topic {:?}. Error: {:?}", t, e);
        }
    }

    /// Unsubscribe from a given topic
    pub fn unsubscribe_gossip(&mut self, t: &str) {
        if let Err(e) = self.gossipsub.unsubscribe(&IdentTopic::new(t)) {
            error!("Failed to unsubscribe from topic {:?}. Error: {:?}", t, e);
        }
    }
}

/// DHT functions
impl NetworkDef {
    /// Publish a key/value to the kv store.
    /// Once replicated upon all nodes, the caller is notified over
    /// `chan`. If there is an error, a [`super::error::DHTError`] is
    /// sent instead.
    pub fn put_record(&mut self, query: KadPutQuery) {
        self.dht.put_record(query);
    }

    /// Retrieve a value for a key from the DHT.
    /// Value (serialized) is sent over `chan`, and if a value is not found,
    /// a [`super::error::DHTError`] is sent instead.
    pub fn get_record(
        &mut self,
        key: Vec<u8>,
        chan: Sender<Vec<u8>>,
        factor: NonZeroUsize,
        retry_count: u8,
    ) {
        self.dht.get_record(
            key,
            chan,
            factor,
            ExponentialBackoff::default(),
            retry_count,
        );
    }
}

/// Request/response functions
impl NetworkDef {
    /// Add a direct request for a given peer
    pub fn add_direct_request(&mut self, peer_id: PeerId, data: Vec<u8>) -> OutboundRequestId {
        self.direct_message.send_request(&peer_id, data)
    }

    /// Add a direct response for a channel
    pub fn add_direct_response(&mut self, chan: ResponseChannel<Vec<u8>>, msg: Vec<u8>) {
        let _ = self.direct_message.send_response(chan, msg);
    }
}

impl From<GossipEvent> for NetworkEventInternal {
    fn from(event: GossipEvent) -> Self {
        Self::GossipEvent(Box::new(event))
    }
}

impl From<DHTEvent> for NetworkEventInternal {
    fn from(event: DHTEvent) -> Self {
        Self::DHTEvent(event)
    }
}

impl From<IdentifyEvent> for NetworkEventInternal {
    fn from(event: IdentifyEvent) -> Self {
        Self::IdentifyEvent(Box::new(event))
    }
}
impl From<libp2p::request_response::Event<Vec<u8>, Vec<u8>>> for NetworkEventInternal {
    fn from(value: libp2p::request_response::Event<Vec<u8>, Vec<u8>>) -> Self {
        Self::DMEvent(value)
    }
}

impl From<libp2p::request_response::Event<Request, Response>> for NetworkEventInternal {
    fn from(event: libp2p::request_response::Event<Request, Response>) -> Self {
        Self::RequestResponseEvent(event)
    }
}

impl From<libp2p::autonat::Event> for NetworkEventInternal {
    fn from(event: libp2p::autonat::Event) -> Self {
        Self::AutonatEvent(event)
    }
}
