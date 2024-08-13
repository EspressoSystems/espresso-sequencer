// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{collections::HashSet, fmt::Debug, time::Duration};

use async_compatibility_layer::{
    art::{async_sleep, async_timeout, future::to},
    channel::{Receiver, SendError, UnboundedReceiver, UnboundedRecvError, UnboundedSender},
};
use futures::channel::oneshot;
use hotshot_types::{
    request_response::{Request, Response},
    traits::{network::NetworkError as HotshotNetworkError, signature_key::SignatureKey},
};
use libp2p::{request_response::ResponseChannel, Multiaddr};
use libp2p_identity::PeerId;
use snafu::{ResultExt, Snafu};
use tracing::{debug, info, instrument};

use crate::network::{
    error::{CancelledRequestSnafu, DHTError},
    gen_multiaddr, ClientRequest, NetworkError, NetworkEvent, NetworkNode, NetworkNodeConfig,
    NetworkNodeConfigBuilderError,
};

/// A handle containing:
/// - A reference to the state
/// - Controls for the swarm
#[derive(Debug, Clone)]
pub struct NetworkNodeHandle<K: SignatureKey + 'static> {
    /// network configuration
    network_config: NetworkNodeConfig<K>,

    /// send an action to the networkbehaviour
    send_network: UnboundedSender<ClientRequest>,

    /// the local address we're listening on
    listen_addr: Multiaddr,

    /// the peer id of the networkbehaviour
    peer_id: PeerId,

    /// human readable id
    id: usize,
}

/// internal network node receiver
#[derive(Debug)]
pub struct NetworkNodeReceiver {
    /// the receiver
    receiver: UnboundedReceiver<NetworkEvent>,

    ///kill switch
    recv_kill: Option<Receiver<()>>,
}

impl NetworkNodeReceiver {
    /// recv a network event
    /// # Errors
    /// Errors if the receiver channel is closed
    pub async fn recv(&self) -> Result<NetworkEvent, NetworkNodeHandleError> {
        self.receiver.recv().await.context(ReceiverEndedSnafu)
    }
    /// Add a kill switch to the receiver
    pub fn set_kill_switch(&mut self, kill_switch: Receiver<()>) {
        self.recv_kill = Some(kill_switch);
    }

    /// Take the kill switch to allow killing the receiver task
    pub fn take_kill_switch(&mut self) -> Option<Receiver<()>> {
        self.recv_kill.take()
    }
}

/// Spawn a network node task task and return the handle and the receiver for it
/// # Errors
/// Errors if spawning the task fails
pub async fn spawn_network_node<K: SignatureKey + 'static>(
    config: NetworkNodeConfig<K>,
    id: usize,
) -> Result<(NetworkNodeReceiver, NetworkNodeHandle<K>), NetworkNodeHandleError> {
    let mut network = NetworkNode::new(config.clone())
        .await
        .context(NetworkSnafu)?;
    // randomly assigned port
    let listen_addr = config
        .bound_addr
        .clone()
        .unwrap_or_else(|| gen_multiaddr(0));
    let peer_id = network.peer_id();
    let listen_addr = network
        .start_listen(listen_addr)
        .await
        .context(NetworkSnafu)?;
    // pin here to force the future onto the heap since it can be large
    // in the case of flume
    let (send_chan, recv_chan) = Box::pin(network.spawn_listeners())
        .await
        .context(NetworkSnafu)?;
    let receiver = NetworkNodeReceiver {
        receiver: recv_chan,
        recv_kill: None,
    };

    let handle = NetworkNodeHandle {
        network_config: config,
        send_network: send_chan,
        listen_addr,
        peer_id,
        id,
    };
    Ok((receiver, handle))
}

impl<K: SignatureKey + 'static> NetworkNodeHandle<K> {
    /// Cleanly shuts down a swarm node
    /// This is done by sending a message to
    /// the swarm itself to spin down
    #[instrument]
    pub async fn shutdown(&self) -> Result<(), NetworkNodeHandleError> {
        self.send_request(ClientRequest::Shutdown).await?;
        Ok(())
    }
    /// Notify the network to begin the bootstrap process
    /// # Errors
    /// If unable to send via `send_network`. This should only happen
    /// if the network is shut down.
    pub async fn begin_bootstrap(&self) -> Result<(), NetworkNodeHandleError> {
        let req = ClientRequest::BeginBootstrap;
        self.send_request(req).await
    }

    /// Get a reference to the network node handle's listen addr.
    #[must_use]
    pub fn listen_addr(&self) -> Multiaddr {
        self.listen_addr.clone()
    }

    /// Print out the routing table used by kademlia
    /// NOTE: only for debugging purposes currently
    /// # Errors
    /// if the client has stopped listening for a response
    pub async fn print_routing_table(&self) -> Result<(), NetworkNodeHandleError> {
        let (s, r) = futures::channel::oneshot::channel();
        let req = ClientRequest::GetRoutingTable(s);
        self.send_request(req).await?;
        r.await.map_err(|_| NetworkNodeHandleError::RecvError)
    }
    /// Wait until at least `num_peers` have connected
    ///
    /// # Errors
    /// If the channel closes before the result can be sent back
    pub async fn wait_to_connect(
        &self,
        num_required_peers: usize,
        node_id: usize,
    ) -> Result<(), NetworkNodeHandleError> {
        // Wait for the required number of peers to connect
        loop {
            // Get the number of currently connected peers
            let num_connected = self.num_connected().await?;
            if num_connected >= num_required_peers {
                break;
            }

            // Log the number of connected peers
            info!(
                "Node {} connected to {}/{} peers",
                node_id, num_connected, num_required_peers
            );

            // Sleep for a second before checking again
            async_sleep(Duration::from_secs(1)).await;
        }

        Ok(())
    }

    /// Request another peer for some data we want.  Returns the id of the request
    ///
    /// # Errors
    ///
    /// Will return a networking error if the channel closes before the result
    /// can be sent back
    pub async fn request_data(
        &self,
        request: &[u8],
        peer: PeerId,
    ) -> Result<Option<Response>, NetworkNodeHandleError> {
        let (tx, rx) = oneshot::channel();
        let req = ClientRequest::DataRequest {
            request: Request(request.to_vec()),
            peer,
            chan: tx,
        };

        self.send_request(req).await?;

        rx.await.map_err(|_| NetworkNodeHandleError::RecvError)
    }

    /// Send a response to a request with the response channel
    /// # Errors
    /// Will error if the client request channel is closed, or serialization fails.
    pub async fn respond_data(
        &self,
        response: Vec<u8>,
        chan: ResponseChannel<Response>,
    ) -> Result<(), NetworkNodeHandleError> {
        let req = ClientRequest::DataResponse {
            response: Response(response),
            chan,
        };
        self.send_request(req).await
    }

    /// Look up a peer's addresses in kademlia
    /// NOTE: this should always be called before any `request_response` is initiated
    /// # Errors
    /// if the client has stopped listening for a response
    pub async fn lookup_pid(&self, peer_id: PeerId) -> Result<(), NetworkNodeHandleError> {
        let (s, r) = futures::channel::oneshot::channel();
        let req = ClientRequest::LookupPeer(peer_id, s);
        self.send_request(req).await?;
        r.await.map_err(|_| NetworkNodeHandleError::RecvError)
    }

    /// Looks up a node's `PeerId` and attempts to validate routing
    /// # Errors
    /// if the peer was unable to be looked up (did not provide a response, DNE)
    pub async fn lookup_node(
        &self,
        key: &[u8],
        dht_timeout: Duration,
    ) -> Result<PeerId, NetworkNodeHandleError> {
        // get record (from DHT)
        let pid = self.record_timeout(key, dht_timeout).await?;

        // pid lookup for routing
        // self.lookup_pid(pid).await?;

        bincode::deserialize(&pid)
            .map_err(|e| NetworkNodeHandleError::DeserializationError { source: e.into() })
    }

    /// Insert a record into the kademlia DHT
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::DHTError`] when encountering an error putting to DHT
    /// - Will return [`NetworkNodeHandleError::SerializationError`] when unable to serialize the key or value
    pub async fn put_record(&self, key: &[u8], value: &[u8]) -> Result<(), NetworkNodeHandleError> {
        let (s, r) = futures::channel::oneshot::channel();
        let req = ClientRequest::PutDHT {
            key: key.to_vec(),
            value: value.to_vec(),
            notify: s,
        };

        self.send_request(req).await?;

        r.await.context(CancelledRequestSnafu).context(DHTSnafu)
    }

    /// Receive a record from the kademlia DHT if it exists.
    /// Must be replicated on at least 2 nodes
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::DHTError`] when encountering an error putting to DHT
    /// - Will return [`NetworkNodeHandleError::SerializationError`] when unable to serialize the key
    /// - Will return [`NetworkNodeHandleError::DeserializationError`] when unable to deserialize the returned value
    pub async fn record(
        &self,
        key: &[u8],
        retry_count: u8,
    ) -> Result<Vec<u8>, NetworkNodeHandleError> {
        let (s, r) = futures::channel::oneshot::channel();
        let req = ClientRequest::GetDHT {
            key: key.to_vec(),
            notify: s,
            retry_count,
        };
        self.send_request(req).await?;

        match r.await.context(CancelledRequestSnafu) {
            Ok(result) => Ok(result),
            Err(e) => Err(e).context(DHTSnafu),
        }
    }

    /// Get a record from the kademlia DHT with a timeout
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::DHTError`] when encountering an error putting to DHT
    /// - Will return [`NetworkNodeHandleError::TimeoutError`] when times out
    /// - Will return [`NetworkNodeHandleError::SerializationError`] when unable to serialize the key
    /// - Will return [`NetworkNodeHandleError::DeserializationError`] when unable to deserialize the returned value
    pub async fn record_timeout(
        &self,
        key: &[u8],
        timeout: Duration,
    ) -> Result<Vec<u8>, NetworkNodeHandleError> {
        let result = async_timeout(timeout, self.record(key, 3)).await;
        match result {
            Err(e) => Err(e).context(TimeoutSnafu),
            Ok(r) => r,
        }
    }

    /// Insert a record into the kademlia DHT with a timeout
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::DHTError`] when encountering an error putting to DHT
    /// - Will return [`NetworkNodeHandleError::TimeoutError`] when times out
    /// - Will return [`NetworkNodeHandleError::SerializationError`] when unable to serialize the key or value
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    pub async fn put_record_timeout(
        &self,
        key: &[u8],
        value: &[u8],
        timeout: Duration,
    ) -> Result<(), NetworkNodeHandleError> {
        let result = async_timeout(timeout, self.put_record(key, value)).await;
        match result {
            Err(e) => Err(e).context(TimeoutSnafu),
            Ok(r) => r,
        }
    }

    /// Subscribe to a topic
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    pub async fn subscribe(&self, topic: String) -> Result<(), NetworkNodeHandleError> {
        let (s, r) = futures::channel::oneshot::channel();
        let req = ClientRequest::Subscribe(topic, Some(s));
        self.send_request(req).await?;
        r.await.map_err(|_| NetworkNodeHandleError::RecvError)
    }

    /// Unsubscribe from a topic
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    pub async fn unsubscribe(&self, topic: String) -> Result<(), NetworkNodeHandleError> {
        let (s, r) = futures::channel::oneshot::channel();
        let req = ClientRequest::Unsubscribe(topic, Some(s));
        self.send_request(req).await?;
        r.await.map_err(|_| NetworkNodeHandleError::RecvError)
    }

    /// Ignore `peers` when pruning
    /// e.g. maintain their connection
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    pub async fn ignore_peers(&self, peers: Vec<PeerId>) -> Result<(), NetworkNodeHandleError> {
        let req = ClientRequest::IgnorePeers(peers);
        self.send_request(req).await
    }

    /// Make a direct request to `peer_id` containing `msg`
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    /// - Will return [`NetworkNodeHandleError::SerializationError`] when unable to serialize `msg`
    pub async fn direct_request(
        &self,
        pid: PeerId,
        msg: &[u8],
    ) -> Result<(), NetworkNodeHandleError> {
        self.direct_request_no_serialize(pid, msg.to_vec()).await
    }

    /// Make a direct request to `peer_id` containing `msg` without serializing
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    /// - Will return [`NetworkNodeHandleError::SerializationError`] when unable to serialize `msg`
    pub async fn direct_request_no_serialize(
        &self,
        pid: PeerId,
        contents: Vec<u8>,
    ) -> Result<(), NetworkNodeHandleError> {
        let req = ClientRequest::DirectRequest {
            pid,
            contents,
            retry_count: 1,
        };
        self.send_request(req).await
    }

    /// Reply with `msg` to a request over `chan`
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    /// - Will return [`NetworkNodeHandleError::SerializationError`] when unable to serialize `msg`
    pub async fn direct_response(
        &self,
        chan: ResponseChannel<Vec<u8>>,
        msg: &[u8],
    ) -> Result<(), NetworkNodeHandleError> {
        let req = ClientRequest::DirectResponse(chan, msg.to_vec());
        self.send_request(req).await
    }

    /// Forcefully disconnect from a peer
    /// # Errors
    /// If the channel is closed somehow
    /// Shouldnt' happen.
    /// # Panics
    /// If channel errors out
    /// shouldn't happen.
    pub async fn prune_peer(&self, pid: PeerId) -> Result<(), NetworkNodeHandleError> {
        let req = ClientRequest::Prune(pid);
        self.send_request(req).await
    }

    /// Gossip a message to peers
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    /// - Will return [`NetworkNodeHandleError::SerializationError`] when unable to serialize `msg`
    pub async fn gossip(&self, topic: String, msg: &[u8]) -> Result<(), NetworkNodeHandleError> {
        self.gossip_no_serialize(topic, msg.to_vec()).await
    }

    /// Gossip a message to peers without serializing
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    /// - Will return [`NetworkNodeHandleError::SerializationError`] when unable to serialize `msg`
    pub async fn gossip_no_serialize(
        &self,
        topic: String,
        msg: Vec<u8>,
    ) -> Result<(), NetworkNodeHandleError> {
        let req = ClientRequest::GossipMsg(topic, msg);
        self.send_request(req).await
    }

    /// Tell libp2p about known network nodes
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    pub async fn add_known_peers(
        &self,
        known_peers: Vec<(PeerId, Multiaddr)>,
    ) -> Result<(), NetworkNodeHandleError> {
        debug!("Adding {} known peers", known_peers.len());
        let req = ClientRequest::AddKnownPeers(known_peers);
        self.send_request(req).await
    }

    /// Send a client request to the network
    ///
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    async fn send_request(&self, req: ClientRequest) -> Result<(), NetworkNodeHandleError> {
        self.send_network
            .send(req)
            .await
            .map_err(|_| NetworkNodeHandleError::SendError)?;
        Ok(())
    }

    /// Returns number of peers this node is connected to
    /// # Errors
    /// If the channel is closed somehow
    /// Shouldnt' happen.
    /// # Panics
    /// If channel errors out
    /// shouldn't happen.
    pub async fn num_connected(&self) -> Result<usize, NetworkNodeHandleError> {
        let (s, r) = futures::channel::oneshot::channel();
        let req = ClientRequest::GetConnectedPeerNum(s);
        self.send_request(req).await?;
        Ok(r.await.unwrap())
    }

    /// return hashset of PIDs this node is connected to
    /// # Errors
    /// If the channel is closed somehow
    /// Shouldnt' happen.
    /// # Panics
    /// If channel errors out
    /// shouldn't happen.
    pub async fn connected_pids(&self) -> Result<HashSet<PeerId>, NetworkNodeHandleError> {
        let (s, r) = futures::channel::oneshot::channel();
        let req = ClientRequest::GetConnectedPeers(s);
        self.send_request(req).await?;
        Ok(r.await.unwrap())
    }

    /// Get a reference to the network node handle's id.
    #[must_use]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get a reference to the network node handle's peer id.
    #[must_use]
    pub fn peer_id(&self) -> PeerId {
        self.peer_id
    }

    /// Return a reference to the network config
    #[must_use]
    pub fn config(&self) -> &NetworkNodeConfig<K> {
        &self.network_config
    }
}

/// Error wrapper type for interacting with swarm handle
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum NetworkNodeHandleError {
    /// Error generating network
    NetworkError {
        /// source of error
        source: NetworkError,
    },
    /// Failure to serialize a message
    SerializationError {
        /// source of error
        source: anyhow::Error,
    },
    /// Failure to deserialize a message
    DeserializationError {
        /// source of error
        source: anyhow::Error,
    },
    /// Error sending request to network
    SendError,
    /// Error receiving message from network
    RecvError,
    /// Error building Node config
    NodeConfigError {
        /// source of error
        source: NetworkNodeConfigBuilderError,
    },
    /// Error waiting for connections
    TimeoutError {
        /// source of error
        source: to::TimeoutError,
    },
    /// Could not connect to the network in time
    ConnectTimeout,
    /// Error in the kademlia DHT
    DHTError {
        /// source of error
        source: DHTError,
    },
    /// The inner [`NetworkNode`] has already been killed
    CantKillTwice {
        /// dummy source
        source: SendError<()>,
    },
    /// The network node has been killed
    Killed,
    /// The receiver was unable to receive a new message
    ReceiverEnded {
        /// source of error
        source: UnboundedRecvError,
    },
    /// no known topic matches the hashset of keys
    NoSuchTopic,
}

impl From<NetworkNodeHandleError> for HotshotNetworkError {
    fn from(error: NetworkNodeHandleError) -> Self {
        match error {
            NetworkNodeHandleError::SerializationError { source } => {
                HotshotNetworkError::FailedToSerialize { source }
            }
            NetworkNodeHandleError::DeserializationError { source } => {
                HotshotNetworkError::FailedToDeserialize { source }
            }
            NetworkNodeHandleError::TimeoutError { source } => {
                HotshotNetworkError::Timeout { source }
            }
            NetworkNodeHandleError::Killed => HotshotNetworkError::ShutDown,
            source => HotshotNetworkError::Libp2p {
                source: Box::new(source),
            },
        }
    }
}

/// Re-exports of the snafu errors that [`NetworkNodeHandleError`] can throw
pub mod network_node_handle_error {
    pub use super::{
        NetworkSnafu, NodeConfigSnafu, RecvSnafu, SendSnafu, SerializationSnafu, TimeoutSnafu,
    };
}
