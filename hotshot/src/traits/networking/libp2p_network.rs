//! Libp2p based/production networking implementation
//! This module provides a libp2p based networking implementation where each node in the
//! network forms a tcp or udp connection to a subset of other nodes in the network

use super::NetworkingMetrics;
use crate::NodeImplementation;
use async_compatibility_layer::{
    art::{async_block_on, async_sleep, async_spawn},
    channel::{unbounded, UnboundedReceiver, UnboundedSender},
};
use async_lock::RwLock;
use async_trait::async_trait;
use bimap::BiHashMap;
use bincode::Options;
use hotshot_task::{boxed_sync, BoxSyncFuture};
use hotshot_types::{
    data::ProposalType,
    message::{Message, MessageKind},
    traits::{
        election::Membership,
        metrics::{Metrics, NoMetrics},
        network::{
            CommunicationChannel, ConnectedNetwork, ConsensusIntentEvent, FailedToSerializeSnafu,
            NetworkError, NetworkMsg, TestableChannelImplementation,
            TestableNetworkingImplementation, TransmitType, ViewMessage,
        },
        node_implementation::NodeType,
        signature_key::SignatureKey,
    },
    vote::VoteType,
};
use hotshot_utils::bincode::bincode_opts;
use libp2p_identity::PeerId;
use libp2p_networking::{
    network::{
        MeshParams,
        NetworkEvent::{self, DirectRequest, DirectResponse, GossipMsg},
        NetworkNodeConfig, NetworkNodeConfigBuilder, NetworkNodeHandle, NetworkNodeHandleError,
        NetworkNodeType,
    },
    reexport::Multiaddr,
};

use serde::Serialize;
use snafu::ResultExt;
use std::{
    collections::{BTreeSet, HashSet},
    fmt::Debug,
    marker::PhantomData,
    num::NonZeroUsize,
    str::FromStr,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};
use tracing::{error, info, instrument};

/// hardcoded topic of QC used
pub const QC_TOPIC: &str = "global";

/// Stubbed out Ack
#[derive(Serialize)]
pub enum Empty {
    /// Empty value
    Empty,
}

impl<M: NetworkMsg, K: SignatureKey + 'static> Debug for Libp2pNetwork<M, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Libp2p").field("inner", &"inner").finish()
    }
}

/// Type alias for a shared collection of peerid, multiaddrs
pub type PeerInfoVec = Arc<RwLock<Vec<(Option<PeerId>, Multiaddr)>>>;

/// The underlying state of the libp2p network
struct Libp2pNetworkInner<M: NetworkMsg, K: SignatureKey + 'static> {
    /// this node's public key
    pk: K,
    /// handle to control the network
    handle: Arc<NetworkNodeHandle<()>>,
    /// Bidirectional map from public key provided by espresso
    /// to public key provided by libp2p
    pubkey_pid_map: RwLock<BiHashMap<K, PeerId>>,
    /// map of known replica peer ids to public keys
    broadcast_recv: UnboundedReceiver<M>,
    /// Sender for broadcast messages
    broadcast_send: UnboundedSender<M>,
    /// Sender for direct messages (only used for sending messages back to oneself)
    direct_send: UnboundedSender<M>,
    /// Receiver for direct messages
    direct_recv: UnboundedReceiver<M>,
    /// this is really cheating to enable local tests
    /// hashset of (bootstrap_addr, peer_id)
    bootstrap_addrs: PeerInfoVec,
    /// bootstrap
    bootstrap_addrs_len: usize,
    /// whether or not the network is ready to send
    is_ready: Arc<AtomicBool>,
    /// max time before dropping message due to DHT error
    dht_timeout: Duration,
    /// whether or not we've bootstrapped into the DHT yet
    is_bootstrapped: Arc<AtomicBool>,
    /// The networking metrics we're keeping track of
    metrics: NetworkingMetrics,
    /// topic map
    /// hash(hashset) -> topic
    /// btreemap ordered so is hashable
    topic_map: RwLock<BiHashMap<BTreeSet<K>, String>>,
}

/// Networking implementation that uses libp2p
/// generic over `M` which is the message type
#[derive(Clone)]
pub struct Libp2pNetwork<M: NetworkMsg, K: SignatureKey + 'static> {
    /// holds the state of the libp2p network
    inner: Arc<Libp2pNetworkInner<M, K>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>>
    TestableNetworkingImplementation<TYPES, Message<TYPES, I>>
    for Libp2pNetwork<Message<TYPES, I>, TYPES::SignatureKey>
where
    MessageKind<TYPES, I>: ViewMessage<TYPES>,
{
    /// Returns a boxed function `f(node_id, public_key) -> Libp2pNetwork`
    /// with the purpose of generating libp2p networks.
    /// Generates `num_bootstrap` bootstrap nodes. The remainder of nodes are normal
    /// nodes with sane defaults.
    /// # Panics
    /// Returned function may panic either:
    /// - An invalid configuration
    ///   (probably an issue with the defaults of this function)
    /// - An inability to spin up the replica's network
    fn generator(
        expected_node_count: usize,
        num_bootstrap: usize,
        network_id: usize,
        da_committee_size: usize,
        _is_da: bool,
    ) -> Box<dyn Fn(u64) -> Self + 'static> {
        assert!(
            da_committee_size <= expected_node_count,
            "DA committee size must be less than or equal to total # nodes"
        );
        let bootstrap_addrs: PeerInfoVec = Arc::default();
        let mut all_keys = BTreeSet::new();
        let mut da_keys = BTreeSet::new();

        for i in 0u64..(expected_node_count as u64) {
            let privkey = TYPES::SignatureKey::generated_from_seed_indexed([0u8; 32], i).1;
            let pubkey = TYPES::SignatureKey::from_private(&privkey);
            if i < da_committee_size as u64 {
                da_keys.insert(pubkey.clone());
            }
            all_keys.insert(pubkey);
        }

        // NOTE uncomment this for easier debugging
        // let start_port = 5000;
        Box::new({
            move |node_id| {
                info!(
                    "GENERATOR: Node id {:?}, is bootstrap: {:?}",
                    node_id,
                    node_id < num_bootstrap as u64
                );
                let addr =
                    // Multiaddr::from_str(&format!("/ip4/127.0.0.1/udp/0/quic-v1")).unwrap();
                    Multiaddr::from_str(&format!("/ip4/127.0.0.1/udp/{}{}/quic-v1", 5000 + node_id, network_id)).unwrap();
                let privkey =
                    TYPES::SignatureKey::generated_from_seed_indexed([0u8; 32], node_id).1;
                let pubkey = TYPES::SignatureKey::from_private(&privkey);
                // we want the majority of peers to have this lying around.
                let replication_factor = NonZeroUsize::new(2 * expected_node_count / 3).unwrap();
                let config = if node_id < num_bootstrap as u64 {
                    NetworkNodeConfigBuilder::default()
                        // NOTICE the implicit assumption that bootstrap is less
                        // than half the network. This seems reasonable.
                        .mesh_params(Some(MeshParams {
                            mesh_n_high: expected_node_count,
                            mesh_n_low: 5,
                            mesh_outbound_min: 3,
                            // the worst case of 7/2+3 > 5
                            mesh_n: (expected_node_count / 2 + 3),
                        }))
                        .replication_factor(replication_factor)
                        .node_type(NetworkNodeType::Bootstrap)
                        .bound_addr(Some(addr))
                        .to_connect_addrs(HashSet::default())
                        // setting to sane defaults
                        .ttl(None)
                        .republication_interval(None)
                        .build()
                        .unwrap()
                } else {
                    NetworkNodeConfigBuilder::default()
                        // NOTE I'm hardcoding these because this is probably the MAX
                        // parameters. If there aren't this many nodes, gossip keeps looking
                        // for more. That is fine.
                        .mesh_params(Some(MeshParams {
                            mesh_n_high: 15,
                            mesh_n_low: 5,
                            mesh_outbound_min: 4,
                            mesh_n: 8,
                        }))
                        .replication_factor(replication_factor)
                        .node_type(NetworkNodeType::Regular)
                        .bound_addr(Some(addr))
                        .to_connect_addrs(HashSet::default())
                        // setting to sane defaults
                        .ttl(None)
                        .republication_interval(None)
                        .build()
                        .unwrap()
                };
                let bootstrap_addrs_ref = bootstrap_addrs.clone();
                let keys = all_keys.clone();
                let da = da_keys.clone();
                async_block_on(async move {
                    Libp2pNetwork::new(
                        NoMetrics::boxed(),
                        config,
                        pubkey,
                        bootstrap_addrs_ref,
                        num_bootstrap,
                        node_id as usize,
                        keys,
                        da,
                    )
                    .await
                    .unwrap()
                })
            }
        })
    }

    fn in_flight_message_count(&self) -> Option<usize> {
        None
    }
}

impl<M: NetworkMsg, K: SignatureKey + 'static> Libp2pNetwork<M, K> {
    /// Returns when network is ready
    pub async fn wait_for_ready(&self) {
        loop {
            if self
                .inner
                .is_ready
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                break;
            }
            async_sleep(Duration::from_secs(1)).await;
        }
        info!("LIBP2P: IS READY GOT TRIGGERED!!");
    }

    /// Constructs new network for a node. Note that this network is unconnected.
    /// One must call `connect` in order to connect.
    /// * `config`: the configuration of the node
    /// * `pk`: public key associated with the node
    /// * `bootstrap_addrs`: rwlock containing the bootstrap addrs
    /// # Errors
    /// Returns error in the event that the underlying libp2p network
    /// is unable to create a network.
    ///
    /// # Panics
    ///
    /// This will panic if there are less than 5 bootstrap nodes
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        metrics: Box<dyn Metrics>,
        config: NetworkNodeConfig,
        pk: K,
        bootstrap_addrs: Arc<RwLock<Vec<(Option<PeerId>, Multiaddr)>>>,
        bootstrap_addrs_len: usize,
        id: usize,
        // HACK
        committee_pks: BTreeSet<K>,
        da_pks: BTreeSet<K>,
    ) -> Result<Libp2pNetwork<M, K>, NetworkError> {
        assert!(bootstrap_addrs_len > 4, "Need at least 5 bootstrap nodes");
        let network_handle = Arc::new(
            NetworkNodeHandle::<()>::new(config, id)
                .await
                .map_err(Into::<NetworkError>::into)?,
        );

        // Make bootstrap mappings known
        if matches!(
            network_handle.config().node_type,
            NetworkNodeType::Bootstrap
        ) {
            let addr = network_handle.listen_addr();
            let pid = network_handle.peer_id();
            let mut bs_cp = bootstrap_addrs.write().await;
            bs_cp.push((Some(pid), addr));
            drop(bs_cp);
        }

        let mut pubkey_pid_map = BiHashMap::new();

        pubkey_pid_map.insert(pk.clone(), network_handle.peer_id());

        let mut topic_map = BiHashMap::new();
        topic_map.insert(committee_pks, QC_TOPIC.to_string());
        topic_map.insert(da_pks, "DA".to_string());

        let topic_map = RwLock::new(topic_map);

        // unbounded channels may not be the best choice (spammed?)
        // if bounded figure out a way to log dropped msgs
        let (direct_send, direct_recv) = unbounded();
        let (broadcast_send, broadcast_recv) = unbounded();

        let result = Libp2pNetwork {
            inner: Arc::new(Libp2pNetworkInner {
                handle: network_handle,
                pubkey_pid_map: RwLock::new(pubkey_pid_map),
                broadcast_recv,
                direct_send: direct_send.clone(),
                direct_recv,
                pk,
                broadcast_send: broadcast_send.clone(),
                bootstrap_addrs_len,
                bootstrap_addrs,
                is_ready: Arc::new(AtomicBool::new(false)),
                dht_timeout: Duration::from_secs(30),
                is_bootstrapped: Arc::new(AtomicBool::new(false)),
                metrics: NetworkingMetrics::new(&*metrics),
                topic_map,
            }),
        };

        result.spawn_event_generator(direct_send, broadcast_send);

        result.spawn_connect(id);

        Ok(result)
    }

    /// Initiates connection to the outside world
    fn spawn_connect(&self, id: usize) {
        let pk = self.inner.pk.clone();
        let bootstrap_ref = self.inner.bootstrap_addrs.clone();
        let num_bootstrap = self.inner.bootstrap_addrs_len;
        let handle = self.inner.handle.clone();
        let is_bootstrapped = self.inner.is_bootstrapped.clone();
        let node_type = self.inner.handle.config().node_type;
        async_spawn({
            let is_ready = self.inner.is_ready.clone();
            async move {
                let bs_addrs = loop {
                    let bss = bootstrap_ref.read().await;
                    let bs_addrs = bss.clone();
                    drop(bss);
                    if bs_addrs.len() >= num_bootstrap {
                        break bs_addrs;
                    }
                    info!(
                        "NODE {:?} bs addr len {:?}, number of bootstrap expected {:?}",
                        id,
                        bs_addrs.len(),
                        num_bootstrap
                    );
                };
                handle.add_known_peers(bs_addrs).await.unwrap();

                // 10 minute timeout
                let timeout_duration = Duration::from_secs(600);
                // perform connection
                info!("WAITING TO CONNECT ON NODE {:?}", id);
                handle
                    .wait_to_connect(4, id, timeout_duration)
                    .await
                    .unwrap();

                while !is_bootstrapped.load(std::sync::atomic::Ordering::Relaxed) {
                    async_sleep(Duration::from_secs(1)).await;
                }

                handle.subscribe(QC_TOPIC.to_string()).await.unwrap();
                handle.subscribe("DA".to_string()).await.unwrap();
                // TODO figure out some way of passing in ALL keypairs. That way we can add the
                // global topic to the topic map
                // NOTE this wont' work without this change

                info!(
                    "peer {:?} waiting for publishing, type: {:?}",
                    handle.peer_id(),
                    node_type
                );

                // we want our records published before
                // we begin participating in consensus
                while handle.put_record(&pk, &handle.peer_id()).await.is_err() {
                    async_sleep(Duration::from_secs(1)).await;
                }

                info!(
                    "Node {:?} is ready, type: {:?}",
                    handle.peer_id(),
                    node_type
                );

                while handle.put_record(&handle.peer_id(), &pk).await.is_err() {
                    async_sleep(Duration::from_secs(1)).await;
                }

                info!(
                    "node {:?} is barring bootstrap, type: {:?}",
                    handle.peer_id(),
                    node_type
                );

                is_ready.store(true, std::sync::atomic::Ordering::Relaxed);
                info!("STARTING CONSENSUS ON {:?}", handle.peer_id());
                Ok::<(), NetworkError>(())
            }
        });
    }

    /// make network aware of known peers
    async fn _add_known_peers(
        &self,
        known_peers: Vec<(Option<PeerId>, Multiaddr)>,
    ) -> Result<(), NetworkError> {
        self.inner
            .handle
            .add_known_peers(known_peers)
            .await
            .map_err(Into::<NetworkError>::into)
    }

    /// task to propagate messages to handlers
    /// terminates on shut down of network
    fn spawn_event_generator(
        &self,
        direct_send: UnboundedSender<M>,
        broadcast_send: UnboundedSender<M>,
    ) {
        let handle = self.clone();
        let is_bootstrapped = self.inner.is_bootstrapped.clone();
        async_spawn(async move {
            while let Ok(msg) = handle.inner.handle.receiver().recv().await {
                match msg {
                    GossipMsg(msg, _topic) => {
                        let result: Result<M, _> = bincode_opts().deserialize(&msg);
                        if let Ok(result) = result {
                            broadcast_send
                                .send(result)
                                .await
                                .map_err(|_| NetworkError::ChannelSend)?;
                        }
                    }
                    DirectRequest(msg, _pid, chan) => {
                        let result: Result<M, _> = bincode_opts()
                            .deserialize(&msg)
                            .context(FailedToSerializeSnafu);
                        if let Ok(result) = result {
                            direct_send
                                .send(result)
                                .await
                                .map_err(|_| NetworkError::ChannelSend)?;
                        }
                        if handle
                            .inner
                            .handle
                            .direct_response(chan, &Empty::Empty)
                            .await
                            .is_err()
                        {
                            error!("failed to ack!");
                        };
                    }
                    DirectResponse(msg, _) => {
                        let _result: Result<M, _> = bincode_opts()
                            .deserialize(&msg)
                            .context(FailedToSerializeSnafu);
                    }
                    NetworkEvent::IsBootstrapped => {
                        is_bootstrapped.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                }
            }
            error!("Network receiever shut down!");
            Ok::<(), NetworkError>(())
        });
    }
}

#[async_trait]
impl<M: NetworkMsg, K: SignatureKey + 'static> ConnectedNetwork<M, K> for Libp2pNetwork<M, K> {
    #[instrument(name = "Libp2pNetwork::ready_blocking", skip_all)]
    async fn wait_for_ready(&self) {
        self.wait_for_ready().await;
    }

    #[instrument(name = "Libp2pNetwork::ready_nonblocking", skip_all)]
    async fn is_ready(&self) -> bool {
        self.inner
            .is_ready
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    #[instrument(name = "Libp2pNetwork::shut_down", skip_all)]
    fn shut_down<'a, 'b>(&'a self) -> BoxSyncFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            if self.inner.handle.is_killed() {
                error!("Called shut down when already shut down! Noop.");
            } else {
                self.inner.handle.shutdown().await.unwrap();
            }
        };
        boxed_sync(closure)
    }

    #[instrument(name = "Libp2pNetwork::broadcast_message", skip_all)]
    async fn broadcast_message(
        &self,
        message: M,
        recipients: BTreeSet<K>,
    ) -> Result<(), NetworkError> {
        if self.inner.handle.is_killed() {
            return Err(NetworkError::ShutDown);
        }

        self.wait_for_ready().await;
        info!(
            "broadcasting msg: {:?} with nodes: {:?} connected",
            message,
            self.inner.handle.connected_pids().await
        );

        let topic_map = self.inner.topic_map.read().await;
        let topic = topic_map
            .get_by_left(&recipients)
            .ok_or(NetworkError::Libp2p {
                source: NetworkNodeHandleError::NoSuchTopic,
            })?
            .clone();
        error!("Broadcasting to topic {}", topic);

        // gossip doesn't broadcast from itself, so special case
        if recipients.contains(&self.inner.pk) {
            // send to self
            self.inner
                .broadcast_send
                .send(message.clone())
                .await
                .map_err(|_| NetworkError::ShutDown)?;
        }

        match self.inner.handle.gossip(topic, &message).await {
            Ok(()) => {
                self.inner.metrics.outgoing_message_count.add(1);
                Ok(())
            }
            Err(e) => {
                self.inner.metrics.message_failed_to_send.add(1);
                Err(e.into())
            }
        }
    }

    #[instrument(name = "Libp2pNetwork::direct_message", skip_all)]
    async fn direct_message(&self, message: M, recipient: K) -> Result<(), NetworkError> {
        if self.inner.handle.is_killed() {
            return Err(NetworkError::ShutDown);
        }

        // short circuit if we're dming ourselves
        if recipient == self.inner.pk {
            // panic if we already shut down?
            self.inner
                .direct_send
                .send(message)
                .await
                .map_err(|_x| NetworkError::ShutDown)?;
            return Ok(());
        }

        self.wait_for_ready().await;
        // check local cache. if that fails, initiate search
        // if search fails, just error out
        // NOTE: relay may be a good way to fix this in the future .
        let pid: PeerId = if let Some(pid) = self
            .inner
            .pubkey_pid_map
            .read()
            .await
            .get_by_left(&recipient)
        {
            *pid
        } else {
            match self
                .inner
                .handle
                .get_record_timeout(&recipient, self.inner.dht_timeout)
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    self.inner.metrics.message_failed_to_send.add(1);
                    error!("Failed to message {:?} because could not find recipient peer id for pk {:?}", message, recipient);
                    return Err(NetworkError::Libp2p { source: e });
                }
            }
        };

        if let Err(e) = self.inner.handle.lookup_pid(pid).await {
            self.inner.metrics.message_failed_to_send.add(1);
            return Err(e.into());
        }
        match self.inner.handle.direct_request(pid, &message).await {
            Ok(()) => {
                self.inner.metrics.outgoing_message_count.add(1);
                Ok(())
            }
            Err(e) => {
                self.inner.metrics.message_failed_to_send.add(1);
                Err(e.into())
            }
        }
    }

    #[instrument(name = "Libp2pNetwork::recv_msgs", skip_all)]
    fn recv_msgs<'a, 'b>(
        &'a self,
        transmit_type: TransmitType,
    ) -> BoxSyncFuture<'b, Result<Vec<M>, NetworkError>>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            if self.inner.handle.is_killed() {
                Err(NetworkError::ShutDown)
            } else {
                match transmit_type {
                    TransmitType::Direct => {
                        let result = self
                            .inner
                            .direct_recv
                            .drain_at_least_one()
                            .await
                            .map_err(|_x| NetworkError::ShutDown)?;
                        self.inner.metrics.incoming_message_count.add(result.len());
                        Ok(result)
                    }
                    TransmitType::Broadcast => {
                        let result = self
                            .inner
                            .broadcast_recv
                            .drain_at_least_one()
                            .await
                            .map_err(|_x| NetworkError::ShutDown)?;
                        self.inner.metrics.incoming_message_count.add(result.len());
                        Ok(result)
                    }
                }
            }
        };
        boxed_sync(closure)
    }

    #[instrument(name = "Libp2pNetwork::lookup_node", skip_all)]
    async fn lookup_node(&self, pk: K) -> Result<(), NetworkError> {
        self.wait_for_ready().await;

        if self.inner.handle.is_killed() {
            return Err(NetworkError::ShutDown);
        }

        let maybe_pid = self
            .inner
            .handle
            .get_record_timeout(&pk, self.inner.dht_timeout)
            .await
            .map_err(Into::<NetworkError>::into);

        if let Ok(pid) = maybe_pid {
            if self.inner.handle.lookup_pid(pid).await.is_err() {
                error!("Failed to look up pid");
                return Err(NetworkError::Libp2p {
                    source: NetworkNodeHandleError::DHTError {
                        source: libp2p_networking::network::error::DHTError::NotFound,
                    },
                });
            };
        } else {
            error!("Unable to look up pubkey {:?}", pk);
            return Err(NetworkError::Libp2p {
                source: NetworkNodeHandleError::DHTError {
                    source: libp2p_networking::network::error::DHTError::NotFound,
                },
            });
        }

        Ok(())
    }

    async fn inject_consensus_info(&self, _event: ConsensusIntentEvent) {
        // Not required
    }
}

/// libp2p identity communication channel
#[derive(Clone, Debug)]
pub struct Libp2pCommChannel<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    MEMBERSHIP: Membership<TYPES>,
>(
    Arc<Libp2pNetwork<Message<TYPES, I>, TYPES::SignatureKey>>,
    PhantomData<(TYPES, I, MEMBERSHIP)>,
);

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    Libp2pCommChannel<TYPES, I, MEMBERSHIP>
{
    /// create a new libp2p communication channel
    #[must_use]
    pub fn new(network: Arc<Libp2pNetwork<Message<TYPES, I>, TYPES::SignatureKey>>) -> Self {
        Self(network, PhantomData)
    }
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES>,
      
        MEMBERSHIP: Membership<TYPES>,
    > TestableNetworkingImplementation<TYPES, Message<TYPES, I>>
    for Libp2pCommChannel<TYPES, I,  MEMBERSHIP>
where
    MessageKind<TYPES, I>: ViewMessage<TYPES>,
{
    /// Returns a boxed function `f(node_id, public_key) -> Libp2pNetwork`
    /// with the purpose of generating libp2p networks.
    /// Generates `num_bootstrap` bootstrap nodes. The remainder of nodes are normal
    /// nodes with sane defaults.
    /// # Panics
    /// Returned function may panic either:
    /// - An invalid configuration
    ///   (probably an issue with the defaults of this function)
    /// - An inability to spin up the replica's network
    fn generator(
        expected_node_count: usize,
        num_bootstrap: usize,
        network_id: usize,
        da_committee_size: usize,
        is_da: bool,
    ) -> Box<dyn Fn(u64) -> Self + 'static> {
        let generator = <Libp2pNetwork<
            Message<TYPES, I>,
            TYPES::SignatureKey,
        > as TestableNetworkingImplementation<_, _>>::generator(
            expected_node_count,
            num_bootstrap,
            network_id,
            da_committee_size,
            is_da
        );
        Box::new(move |node_id| Self(generator(node_id).into(), PhantomData))
    }

    fn in_flight_message_count(&self) -> Option<usize> {
        None
    }
}

// FIXME maybe we should macro this...? It's repeated at verbatum EXCEPT for impl generics at the
// top
// we don't really want to make this the default implementation because that forces it to require ConnectedNetwork to be implemented. The struct we implement over might use multiple ConnectedNetworks
#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    CommunicationChannel<TYPES, Message<TYPES, I>, MEMBERSHIP>
    for Libp2pCommChannel<TYPES, I, MEMBERSHIP>
where
    MessageKind<TYPES, I>: ViewMessage<TYPES>,
{
    type NETWORK = Libp2pNetwork<Message<TYPES, I>, TYPES::SignatureKey>;

    async fn wait_for_ready(&self) {
        self.0.wait_for_ready().await;
    }

    async fn is_ready(&self) -> bool {
        self.0.is_ready().await
    }

    fn shut_down<'a, 'b>(&'a self) -> BoxSyncFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            self.0.shut_down().await;
        };
        boxed_sync(closure)
    }

    async fn broadcast_message(
        &self,
        message: Message<TYPES, I>,
        membership: &MEMBERSHIP,
    ) -> Result<(), NetworkError> {
        let recipients = <MEMBERSHIP as Membership<TYPES>>::get_committee(
            membership,
            message.kind.get_view_number(),
        );
        self.0.broadcast_message(message, recipients).await
    }

    async fn direct_message(
        &self,
        message: Message<TYPES, I>,
        recipient: TYPES::SignatureKey,
    ) -> Result<(), NetworkError> {
        self.0.direct_message(message, recipient).await
    }

    fn recv_msgs<'a, 'b>(
        &'a self,
        transmit_type: TransmitType,
    ) -> BoxSyncFuture<'b, Result<Vec<Message<TYPES, I>>, NetworkError>>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move { self.0.recv_msgs(transmit_type).await };
        boxed_sync(closure)
    }

    async fn lookup_node(&self, pk: TYPES::SignatureKey) -> Result<(), NetworkError> {
        self.0.lookup_node(pk).await
    }

    async fn inject_consensus_info(&self, _event: ConsensusIntentEvent) {
        // Not required
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    TestableChannelImplementation<
        TYPES,
        Message<TYPES, I>,
        MEMBERSHIP,
        Libp2pNetwork<Message<TYPES, I>, TYPES::SignatureKey>,
    > for Libp2pCommChannel<TYPES, I, MEMBERSHIP>
{
    fn generate_network(
    ) -> Box<dyn Fn(Arc<Libp2pNetwork<Message<TYPES, I>, TYPES::SignatureKey>>) -> Self + 'static>
    {
        Box::new(move |network| Libp2pCommChannel::new(network))
    }
}
