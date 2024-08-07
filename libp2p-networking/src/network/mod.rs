// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

/// networking behaviours wrapping libp2p's behaviours
pub mod behaviours;
/// defines the swarm and network definition (internal)
mod def;
/// libp2p network errors
pub mod error;
/// functionality of a libp2p network node
mod node;

use std::{collections::HashSet, fmt::Debug, str::FromStr};

use futures::channel::oneshot::{self, Sender};
#[cfg(async_executor_impl = "async-std")]
use libp2p::dns::async_std::Transport as DnsTransport;
#[cfg(async_executor_impl = "tokio")]
use libp2p::dns::tokio::Transport as DnsTransport;
use libp2p::{
    build_multiaddr,
    core::{muxing::StreamMuxerBox, transport::Boxed},
    gossipsub::Event as GossipEvent,
    identify::Event as IdentifyEvent,
    identity::Keypair,
    quic,
    request_response::ResponseChannel,
    Multiaddr, Transport,
};
use libp2p_identity::PeerId;
#[cfg(async_executor_impl = "async-std")]
use quic::async_std::Transport as QuicTransport;
#[cfg(async_executor_impl = "tokio")]
use quic::tokio::Transport as QuicTransport;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use self::behaviours::request_response::{Request, Response};
pub use self::{
    def::NetworkDef,
    error::NetworkError,
    node::{
        network_node_handle_error, spawn_network_node, MeshParams, NetworkNode, NetworkNodeConfig,
        NetworkNodeConfigBuilder, NetworkNodeConfigBuilderError, NetworkNodeHandle,
        NetworkNodeHandleError, NetworkNodeReceiver, DEFAULT_REPLICATION_FACTOR,
    },
};
#[cfg(not(any(async_executor_impl = "async-std", async_executor_impl = "tokio")))]
compile_error! {"Either config option \"async-std\" or \"tokio\" must be enabled for this crate."}

/// this is mostly to estimate how many network connections
/// a node should allow
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum NetworkNodeType {
    /// bootstrap node accepts all connections
    Bootstrap,
    /// regular node has a limit to the
    /// number of connections to accept
    Regular,
    /// conductor node is never pruned
    Conductor,
}

impl FromStr for NetworkNodeType {
    type Err = String;

    fn from_str(input: &str) -> Result<NetworkNodeType, Self::Err> {
        match input {
            "Conductor" => Ok(NetworkNodeType::Conductor),
            "Regular" => Ok(NetworkNodeType::Regular),
            "Bootstrap" => Ok(NetworkNodeType::Bootstrap),
            _ => Err(
                "Couldn't parse node type. Must be one of Conductor, Bootstrap, Regular"
                    .to_string(),
            ),
        }
    }
}

impl Default for NetworkNodeType {
    fn default() -> Self {
        Self::Bootstrap
    }
}

/// Actions to send from the client to the swarm
#[derive(Debug)]
pub enum ClientRequest {
    /// Start the bootstrap process to kademlia
    BeginBootstrap,
    /// kill the swarm
    Shutdown,
    /// broadcast a serialized message
    GossipMsg(String, Vec<u8>),
    /// subscribe to a topic
    Subscribe(String, Option<Sender<()>>),
    /// unsubscribe from a topic
    Unsubscribe(String, Option<Sender<()>>),
    /// client request to send a direct serialized message
    DirectRequest {
        /// peer id
        pid: PeerId,
        /// msg contents
        contents: Vec<u8>,
        /// number of retries
        retry_count: u8,
    },
    /// client request to send a direct reply to a message
    DirectResponse(ResponseChannel<Vec<u8>>, Vec<u8>),
    /// request for data from another peer
    DataRequest {
        /// request sent on wire
        request: Request,
        /// Peer to try sending the request to
        peer: PeerId,
        /// Send back request ID to client
        chan: oneshot::Sender<Option<Response>>,
    },
    /// Respond with some data to another peer
    DataResponse {
        /// Data
        response: Response,
        /// Send back channel
        chan: ResponseChannel<Response>,
    },
    /// prune a peer
    Prune(PeerId),
    /// add vec of known peers or addresses
    AddKnownPeers(Vec<(PeerId, Multiaddr)>),
    /// Ignore peers. Only here for debugging purposes.
    /// Allows us to have nodes that are never pruned
    IgnorePeers(Vec<PeerId>),
    /// Put(Key, Value) into DHT
    /// relay success back on channel
    PutDHT {
        /// Key to publish under
        key: Vec<u8>,
        /// Value to publish under
        value: Vec<u8>,
        /// Channel to notify caller of result of publishing
        notify: Sender<()>,
    },
    /// Get(Key, Chan)
    GetDHT {
        /// Key to search for
        key: Vec<u8>,
        /// Channel to notify caller of value (or failure to find value)
        notify: Sender<Vec<u8>>,
        /// number of retries to make
        retry_count: u8,
    },
    /// Request the number of connected peers
    GetConnectedPeerNum(Sender<usize>),
    /// Request the set of connected peers
    GetConnectedPeers(Sender<HashSet<PeerId>>),
    /// Print the routing  table to stderr, debugging only
    GetRoutingTable(Sender<()>),
    /// Get address of peer
    LookupPeer(PeerId, Sender<()>),
}

/// events generated by the swarm that we wish
/// to relay to the client
#[derive(Debug)]
pub enum NetworkEvent {
    /// Recv-ed a broadcast
    GossipMsg(Vec<u8>),
    /// Recv-ed a direct message from a node
    DirectRequest(Vec<u8>, PeerId, ResponseChannel<Vec<u8>>),
    /// Recv-ed a direct response from a node (that hopefully was initiated by this node)
    DirectResponse(Vec<u8>, PeerId),
    /// A peer is asking us for data
    ResponseRequested(Request, ResponseChannel<Response>),
    /// Report that kademlia has successfully bootstrapped into the network
    IsBootstrapped,
    /// The number of connected peers has possibly changed
    ConnectedPeersUpdate(usize),
}

#[derive(Debug)]
/// internal representation of the network events
/// only used for event processing before relaying to client
pub enum NetworkEventInternal {
    /// a DHT event
    DHTEvent(libp2p::kad::Event),
    /// a identify event. Is boxed because this event is much larger than the other ones so we want
    /// to store it on the heap.
    IdentifyEvent(Box<IdentifyEvent>),
    /// a gossip  event
    GossipEvent(Box<GossipEvent>),
    /// a direct message event
    DMEvent(libp2p::request_response::Event<Vec<u8>, Vec<u8>>),
    /// a request response event
    RequestResponseEvent(libp2p::request_response::Event<Request, Response>),
    /// a autonat event
    AutonatEvent(libp2p::autonat::Event),
}

/// Bind all interfaces on port `port`
/// NOTE we may want something more general in the fture.
#[must_use]
pub fn gen_multiaddr(port: u16) -> Multiaddr {
    build_multiaddr!(Ip4([0, 0, 0, 0]), Udp(port), QuicV1)
}

/// `BoxedTransport` is a type alias for a boxed tuple containing a `PeerId` and a `StreamMuxerBox`.
///
/// This type is used to represent a transport in the libp2p network framework. The `PeerId` is a unique identifier for each peer in the network, and the `StreamMuxerBox` is a type of multiplexer that can handle multiple substreams over a single connection.
type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;

/// Generate authenticated transport
/// # Errors
/// could not sign the quic key with `identity`
#[instrument(skip(identity))]
pub async fn gen_transport(identity: Keypair) -> Result<BoxedTransport, NetworkError> {
    let quic_transport = {
        let mut config = quic::Config::new(&identity);
        config.handshake_timeout = std::time::Duration::from_secs(20);
        QuicTransport::new(config)
    };

    let dns_quic = {
        #[cfg(async_executor_impl = "async-std")]
        {
            DnsTransport::system(quic_transport).await
        }

        #[cfg(async_executor_impl = "tokio")]
        {
            DnsTransport::system(quic_transport)
        }
    }
    .map_err(|e| NetworkError::TransportLaunch { source: e })?;

    Ok(dns_quic
        .map(|(peer_id, connection), _| (peer_id, StreamMuxerBox::new(connection)))
        .boxed())
}
