use crate::network::{
    behaviours::direct_message_codec::DirectMessageResponse, error::DHTError, gen_multiaddr,
    ClientRequest, NetworkError, NetworkEvent, NetworkNode, NetworkNodeConfig,
    NetworkNodeConfigBuilderError,
};
use async_compatibility_layer::{
    art::{async_sleep, async_spawn, async_timeout, future::to, stream},
    async_primitives::subscribable_mutex::SubscribableMutex,
    channel::{
        bounded, oneshot, OneShotReceiver, OneShotSender, Receiver, SendError, Sender,
        UnboundedReceiver, UnboundedRecvError, UnboundedSender,
    },
};
use async_lock::Mutex;
use bincode::Options;
use futures::{stream::FuturesOrdered, Future, FutureExt};
use hotshot_utils::bincode::bincode_opts;
use libp2p::{request_response::ResponseChannel, Multiaddr};
use libp2p_identity::PeerId;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::{
    collections::HashSet,
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tracing::{debug, info, instrument};

/// A handle containing:
/// - A reference to the state
/// - Controls for the swarm
#[derive(Debug)]
pub struct NetworkNodeHandle<S> {
    /// network configuration
    network_config: NetworkNodeConfig,
    /// the state of the replica
    state: Arc<SubscribableMutex<S>>,
    /// send an action to the networkbehaviour
    send_network: UnboundedSender<ClientRequest>,

    /// the local address we're listening on
    listen_addr: Multiaddr,
    /// the peer id of the networkbehaviour
    peer_id: PeerId,
    /// human readable id
    id: usize,

    /// A list of webui listeners that are listening for changes on this node
    webui_listeners: Arc<Mutex<Vec<Sender<()>>>>,

    receiver: NetworkNodeReceiver,
}

/// internal network node receiver
#[derive(Debug)]
pub struct NetworkNodeReceiver {
    /// whether or not the receiver is started
    receiver_spawned: AtomicBool,

    /// whether or not the handle has been killed
    killed: AtomicBool,

    /// the receiver
    receiver: Mutex<UnboundedReceiver<NetworkEvent>>,

    ///kill switch
    recv_kill: Mutex<Option<OneShotReceiver<()>>>,

    /// kill the event handler for events from the swarm
    kill_switch: Mutex<Option<OneShotSender<()>>>,
}

impl NetworkNodeReceiver {
    pub async fn recv(&self) -> Result<NetworkEvent, NetworkNodeHandleError> {
        if self.killed.load(Ordering::Relaxed) {
            return Err(NetworkNodeHandleError::Killed);
        }
        let lock = self.receiver.lock().await;
        lock.recv().await.context(ReceiverEndedSnafu)
    }
}

impl<S: Default + Debug> NetworkNodeHandle<S> {
    /// constructs a new node listening on `known_addr`
    #[instrument]
    pub async fn new(config: NetworkNodeConfig, id: usize) -> Result<Self, NetworkNodeHandleError> {
        //`randomly assigned port
        let listen_addr = config
            .bound_addr
            .clone()
            .unwrap_or_else(|| gen_multiaddr(0));
        let mut network = NetworkNode::new(config.clone())
            .await
            .context(NetworkSnafu)?;

        let peer_id = network.peer_id();
        let listen_addr = network
            .start_listen(listen_addr)
            .await
            .context(NetworkSnafu)?;
        info!("LISTEN ADDRESS IS {:?}", listen_addr);
        // pin here to force the future onto the heap since it can be large
        // in the case of flume
        let (send_chan, recv_chan) = Box::pin(network.spawn_listeners())
            .await
            .context(NetworkSnafu)?;
        let (kill_switch, recv_kill) = oneshot();

        let kill_switch = Mutex::new(Some(kill_switch));
        let recv_kill = Mutex::new(Some(recv_kill));

        Ok(NetworkNodeHandle {
            network_config: config,
            state: std::sync::Arc::default(),
            send_network: send_chan,
            listen_addr,
            peer_id,
            id,
            webui_listeners: Arc::default(),
            receiver: NetworkNodeReceiver {
                kill_switch,
                killed: AtomicBool::new(false),
                receiver: Mutex::new(recv_chan),
                recv_kill,
                receiver_spawned: AtomicBool::new(false),
            },
        })
    }

    /// Spawn a handler `F` that will be notified every time a new [`NetworkEvent`] arrives.
    ///
    /// # Panics
    ///
    /// Will panic if a handler is already spawned
    pub async fn spawn_handler<F, RET>(self: &Arc<Self>, cb: F) -> impl Future<Output = ()>
    where
        F: Fn(NetworkEvent, Arc<NetworkNodeHandle<S>>) -> RET + Sync + Send + 'static,
        RET: Future<Output = Result<(), NetworkNodeHandleError>> + Send + 'static,
        S: Send + 'static,
    {
        assert!(
            !self.receiver.receiver_spawned.swap(true, Ordering::Relaxed),
            "Handler is already spawned, this is a bug"
        );

        let handle = Arc::clone(self);
        async_spawn(async move {
            let receiver = handle.receiver.receiver.lock().await;
             let Some(kill_switch) = handle.receiver.recv_kill.lock().await.take() else {
                     tracing::error!(
                         "`spawn_handle` was called on a network handle that was already closed"
                     );
                     return;
                 };
            let mut next_msg = receiver.recv().boxed();
            let mut kill_switch = kill_switch.recv().boxed();
            loop {
                match futures::future::select(next_msg, kill_switch).await {
                    futures::future::Either::Left((incoming_message, other_stream)) => {
                        let incoming_message = match incoming_message {
                            Ok(msg) => msg,
                            Err(e) => {
                                tracing::warn!(?e, "NetworkNodeHandle::spawn_handle was unable to receive more messages");
                                return;
                            }
                        };
                        if let Err(e) = cb(incoming_message, handle.clone()).await {
                            tracing::error!(
                                ?e,
                                "NetworkNodeHandle::spawn_handle returned an error"
                            );
                            return;
                        }

                        // re-set the `kill_switch` for the next loop
                        kill_switch = other_stream;
                        // re-set `receiver.recv()` for the next loop
                        next_msg = receiver.recv().boxed();
                    }
                    futures::future::Either::Right(_) => {
                        // killed
                        handle.receiver.killed.store(true, Ordering::Relaxed);
                        return;
                    }
                }
            }
        }).map(|_| ())
    }

    /// Wait until at least `num_peers` have connected, or until `timeout` time has passed.
    ///
    /// # Errors
    ///
    /// Will return any networking error encountered, or `ConnectTimeout` if the `timeout` has elapsed.
    pub async fn wait_to_connect(
        &self,
        num_peers: usize,
        node_id: usize,
        timeout: Duration,
    ) -> Result<(), NetworkNodeHandleError>
    where
        S: Default + Debug,
    {
        let start = Instant::now();
        self.begin_bootstrap().await?;
        let mut connected_ok = false;
        while !connected_ok {
            if start.elapsed() >= timeout {
                return Err(NetworkNodeHandleError::ConnectTimeout);
            }
            async_sleep(Duration::from_secs(1)).await;
            let num_connected = self.num_connected().await?;
            info!(
                "WAITING TO CONNECT, connected to {} / {} peers ON NODE {}",
                num_connected, num_peers, node_id
            );
            connected_ok = num_connected >= num_peers;
        }
        Ok(())
    }

    /// Receives a reference of the internal `NetworkNodeReceiver`, which can be used to query for incoming messages.
    pub fn receiver(&self) -> &NetworkNodeReceiver {
        &self.receiver
    }

    /// Cleanly shuts down a swarm node
    /// This is done by sending a message to
    /// the swarm event handler to stop handling events
    /// and a message to the swarm itself to spin down
    #[instrument]
    pub async fn shutdown(&self) -> Result<(), NetworkNodeHandleError> {
        self.send_request(ClientRequest::Shutdown).await?;
        // if this fails, the thread has already been killed.
        if let Some(kill_switch) = self.receiver.kill_switch.lock().await.take() {
            kill_switch.send(());
        } else {
            tracing::warn!("The network node handle is shutting down, but the kill switch was already consumed");
        }
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
    pub fn listen_addr(&self) -> Multiaddr {
        self.listen_addr.clone()
    }
}

impl<S> NetworkNodeHandle<S> {
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

    /// Insert a record into the kademlia DHT
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::DHTError`] when encountering an error putting to DHT
    /// - Will return [`NetworkNodeHandleError::SerializationError`] when unable to serialize the key or value
    pub async fn put_record(
        &self,
        key: &impl Serialize,
        value: &impl Serialize,
    ) -> Result<(), NetworkNodeHandleError> {
        use crate::network::error::CancelledRequestSnafu;

        let (s, r) = futures::channel::oneshot::channel();
        let req = ClientRequest::PutDHT {
            key: bincode_opts().serialize(key).context(SerializationSnafu)?,
            value: bincode_opts()
                .serialize(value)
                .context(SerializationSnafu)?,
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
    pub async fn get_record<V: for<'a> Deserialize<'a>>(
        &self,
        key: &impl Serialize,
        retry_count: u8,
    ) -> Result<V, NetworkNodeHandleError> {
        use crate::network::error::CancelledRequestSnafu;

        let (s, r) = futures::channel::oneshot::channel();
        let req = ClientRequest::GetDHT {
            key: bincode_opts().serialize(key).context(SerializationSnafu)?,
            notify: s,
            retry_count,
        };
        self.send_request(req).await?;

        match r.await.context(CancelledRequestSnafu) {
            Ok(result) => bincode_opts()
                .deserialize(&result)
                .context(DeserializationSnafu),
            Err(e) => Err(e).context(DHTSnafu),
        }
    }

    /// Get a record from the kademlia DHT with a timeout
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::DHTError`] when encountering an error putting to DHT
    /// - Will return [`NetworkNodeHandleError::TimeoutError`] when times out
    /// - Will return [`NetworkNodeHandleError::SerializationError`] when unable to serialize the key
    /// - Will return [`NetworkNodeHandleError::DeserializationError`] when unable to deserialize the returned value
    pub async fn get_record_timeout<V: for<'a> Deserialize<'a>>(
        &self,
        key: &impl Serialize,
        timeout: Duration,
    ) -> Result<V, NetworkNodeHandleError> {
        let result = async_timeout(timeout, self.get_record(key, 1)).await;
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
        key: &impl Serialize,
        value: &impl Serialize,
        timeout: Duration,
    ) -> Result<(), NetworkNodeHandleError> {
        let result = async_timeout(timeout, self.put_record(key, value)).await;
        match result {
            Err(e) => Err(e).context(TimeoutSnafu),
            Ok(r) => r,
        }
    }

    /// Notify the webui that either the `state` or `connection_state` has changed.
    ///
    /// If the webui is not started, this will do nothing.
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    pub async fn notify_webui(&self) {
        let mut lock = self.webui_listeners.lock().await;
        // Keep a list of indexes that are unable to send the update
        let mut indexes_to_remove = Vec::new();
        for (idx, sender) in lock.iter().enumerate() {
            if sender.send(()).await.is_err() {
                indexes_to_remove.push(idx);
            }
        }
        // Make sure to remove the indexes in reverse other, else removing an index will invalidate the following indexes.
        for idx in indexes_to_remove.into_iter().rev() {
            lock.remove(idx);
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
        msg: &impl Serialize,
    ) -> Result<(), NetworkNodeHandleError> {
        let serialized_msg = bincode_opts().serialize(msg).context(SerializationSnafu)?;
        let req = ClientRequest::DirectRequest {
            pid,
            contents: serialized_msg,
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
        chan: ResponseChannel<DirectMessageResponse>,
        msg: &impl Serialize,
    ) -> Result<(), NetworkNodeHandleError> {
        let serialized_msg = bincode_opts().serialize(msg).context(SerializationSnafu)?;
        let req = ClientRequest::DirectResponse(chan, serialized_msg);
        self.send_request(req).await
    }

    /// Forcefully disconnet from a peer
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
    pub async fn gossip(
        &self,
        topic: String,
        msg: &impl Serialize,
    ) -> Result<(), NetworkNodeHandleError> {
        let serialized_msg = bincode_opts().serialize(msg).context(SerializationSnafu)?;
        let req = ClientRequest::GossipMsg(topic, serialized_msg);
        self.send_request(req).await
    }

    /// Tell libp2p about known network nodes
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    pub async fn add_known_peers(
        &self,
        known_peers: Vec<(Option<PeerId>, Multiaddr)>,
    ) -> Result<(), NetworkNodeHandleError> {
        info!("ADDING KNOWN PEERS TO {:?}", self.peer_id);
        let req = ClientRequest::AddKnownPeers(known_peers);
        self.send_request(req).await
    }

    /// Send a client request to the network
    ///
    /// # Errors
    /// - Will return [`NetworkNodeHandleError::SendError`] when underlying `NetworkNode` has been killed
    async fn send_request(&self, req: ClientRequest) -> Result<(), NetworkNodeHandleError> {
        debug!("peerid {:?}\t\tsending message {:?}", self.peer_id, req);
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
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get a reference to the network node handle's peer id.
    pub fn peer_id(&self) -> PeerId {
        self.peer_id
    }

    /// Return a reference to the network config
    pub fn config(&self) -> &NetworkNodeConfig {
        &self.network_config
    }

    /// Modify the state. This will automatically call `state_changed` and `notify_webui`
    pub async fn modify_state<F>(&self, cb: F)
    where
        F: FnMut(&mut S),
    {
        self.state.modify(cb).await;
    }

    /// Returns `true` if the network state is killed
    pub fn is_killed(&self) -> bool {
        self.receiver.killed.load(Ordering::Relaxed)
    }

    /// Register a webui listener
    pub async fn register_webui_listener(&self) -> Receiver<()> {
        let (sender, receiver) = bounded(100);
        let mut lock = self.webui_listeners.lock().await;
        lock.push(sender);
        receiver
    }

    /// Call `wait_timeout_until` on the state's [`SubscribableMutex`]
    /// # Errors
    /// Will throw a [`NetworkNodeHandleError::TimeoutError`] error upon timeout
    pub async fn state_wait_timeout_until<F>(
        &self,
        timeout: Duration,
        f: F,
    ) -> Result<(), NetworkNodeHandleError>
    where
        F: FnMut(&S) -> bool,
    {
        self.state
            .wait_timeout_until(timeout, f)
            .await
            .context(TimeoutSnafu)
    }

    /// Call `wait_timeout_until_with_trigger` on the state's [`SubscribableMutex`]
    pub fn state_wait_timeout_until_with_trigger<'a, F>(
        &'a self,
        timeout: Duration,
        f: F,
    ) -> stream::to::Timeout<FuturesOrdered<impl Future<Output = ()> + 'a>>
    where
        F: FnMut(&S) -> bool + 'a,
    {
        self.state.wait_timeout_until_with_trigger(timeout, f)
    }

    /// Call `wait_until` on the state's [`SubscribableMutex`]
    /// # Errors
    /// Will throw a [`NetworkNodeHandleError::TimeoutError`] error upon timeout
    pub async fn state_wait_until<F>(&self, f: F) -> Result<(), NetworkNodeHandleError>
    where
        F: FnMut(&S) -> bool,
    {
        self.state.wait_until(f).await;
        Ok(())
    }
}

impl<S: Clone> NetworkNodeHandle<S> {
    /// Get a clone of the internal state
    pub async fn state(&self) -> S {
        self.state.cloned().await
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
        source: Box<bincode::ErrorKind>,
    },
    /// Failure to deserialize a message
    DeserializationError {
        /// source of error
        source: Box<bincode::ErrorKind>,
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

/// Re-exports of the snafu errors that [`NetworkNodeHandleError`] can throw
pub mod network_node_handle_error {
    pub use super::{
        NetworkSnafu, NodeConfigSnafu, RecvSnafu, SendSnafu, SerializationSnafu, TimeoutSnafu,
    };
}
