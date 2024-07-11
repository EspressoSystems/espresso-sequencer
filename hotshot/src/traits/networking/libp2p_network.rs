//! Libp2p based/production networking implementation
//! This module provides a libp2p based networking implementation where each node in the
//! network forms a tcp or udp connection to a subset of other nodes in the network
#[cfg(feature = "hotshot-testing")]
use std::str::FromStr;
use std::{
    collections::{BTreeSet, HashSet},
    fmt::Debug,
    net::SocketAddr,
    num::NonZeroUsize,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use anyhow::anyhow;
use async_compatibility_layer::{
    art::{async_sleep, async_spawn},
    channel::{
        self, bounded, unbounded, Receiver as BoundedReceiver, Sender as BoundedSender,
        TrySendError, UnboundedReceiver, UnboundedSender,
    },
};
use async_lock::{Mutex, RwLock};
use async_trait::async_trait;
use bimap::BiHashMap;
use futures::{
    channel::mpsc::{self, channel, Receiver, Sender},
    future::{join_all, Either},
    FutureExt, StreamExt,
};
use hotshot_orchestrator::config::NetworkConfig;
#[cfg(feature = "hotshot-testing")]
use hotshot_types::traits::network::{
    AsyncGenerator, NetworkReliability, TestableNetworkingImplementation,
};
use hotshot_types::{
    boxed_sync,
    constants::LOOK_AHEAD,
    data::ViewNumber,
    message::{DataMessage::DataResponse, Message, MessageKind},
    traits::{
        election::Membership,
        metrics::{Counter, Gauge, Metrics, NoMetrics},
        network::{self, ConnectedNetwork, NetworkError, ResponseMessage},
        node_implementation::{ConsensusTime, NodeType},
        signature_key::SignatureKey,
    },
    BoxSyncFuture,
};
use libp2p_identity::{
    ed25519::{self, SecretKey},
    Keypair, PeerId,
};
use libp2p_networking::{
    network::{
        behaviours::request_response::{Request, Response},
        spawn_network_node, MeshParams,
        NetworkEvent::{self, DirectRequest, DirectResponse, GossipMsg},
        NetworkNodeConfig, NetworkNodeConfigBuilder, NetworkNodeHandle, NetworkNodeHandleError,
        NetworkNodeReceiver, NetworkNodeType,
    },
    reexport::{Multiaddr, ResponseChannel},
};
use rand::{rngs::StdRng, seq::IteratorRandom, SeedableRng};
use serde::Serialize;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::BroadcastDelay;

/// Libp2p-specific metrics
#[derive(Clone, Debug)]
pub struct Libp2pMetricsValue {
    /// The number of currently connected peers
    pub num_connected_peers: Box<dyn Gauge>,
    /// The number of failed messages
    pub num_failed_messages: Box<dyn Counter>,
}

impl Libp2pMetricsValue {
    /// Populate the metrics with Libp2p-specific metrics
    pub fn new(metrics: &dyn Metrics) -> Self {
        // Create a `libp2p subgroup
        let subgroup = metrics.subgroup("libp2p".into());

        // Create the metrics
        Self {
            num_connected_peers: subgroup.create_gauge("num_connected_peers".into(), None),
            num_failed_messages: subgroup.create_counter("num_failed_messages".into(), None),
        }
    }
}

impl Default for Libp2pMetricsValue {
    /// Initialize with empty metrics
    fn default() -> Self {
        Self::new(&*NoMetrics::boxed())
    }
}

/// convenience alias for the type for bootstrap addresses
/// concurrency primitives are needed for having tests
pub type BootstrapAddrs = Arc<RwLock<Vec<(PeerId, Multiaddr)>>>;

/// hardcoded topic of QC used
pub const QC_TOPIC: &str = "global";

/// Stubbed out Ack
///
/// Note: as part of versioning for upgradability,
/// all network messages must begin with a 4-byte version number.
///
/// Hence:
///   * `Empty` *must* be a struct (enums are serialized with a leading byte for the variant), and
///   * we must have an explicit version field.
#[derive(Serialize)]
pub struct Empty {
    /// This should not be required, but it is. Version automatically gets prepended.
    /// Perhaps this could be replaced with something zero-sized and serializable.
    byte: u8,
}

impl<K: SignatureKey + 'static> Debug for Libp2pNetwork<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Libp2p").field("inner", &"inner").finish()
    }
}

/// Locked Option of a receiver for moving the value out of the option
type TakeReceiver = Mutex<Option<Receiver<(Vec<u8>, ResponseChannel<Response>)>>>;

/// Type alias for a shared collection of peerid, multiaddrs
pub type PeerInfoVec = Arc<RwLock<Vec<(PeerId, Multiaddr)>>>;

/// The underlying state of the libp2p network
#[derive(Debug)]
struct Libp2pNetworkInner<K: SignatureKey + 'static> {
    /// this node's public key
    pk: K,
    /// handle to control the network
    handle: Arc<NetworkNodeHandle>,
    /// Message Receiver
    receiver: UnboundedReceiver<Vec<u8>>,
    /// Receiver for Requests for Data, includes the request and the response chan
    /// Lock should only be used once to take the channel and move it into the request
    /// handler task
    requests_rx: TakeReceiver,
    /// Sender for broadcast messages
    sender: UnboundedSender<Vec<u8>>,
    /// Sender for node lookup (relevant view number, key of node) (None for shutdown)
    node_lookup_send: BoundedSender<Option<(ViewNumber, K)>>,
    /// this is really cheating to enable local tests
    /// hashset of (bootstrap_addr, peer_id)
    bootstrap_addrs: PeerInfoVec,
    /// whether or not the network is ready to send
    is_ready: Arc<AtomicBool>,
    /// max time before dropping message due to DHT error
    dht_timeout: Duration,
    /// whether or not we've bootstrapped into the DHT yet
    is_bootstrapped: Arc<AtomicBool>,
    /// The Libp2p metrics we're managing
    metrics: Libp2pMetricsValue,
    /// topic map
    /// hash(hashset) -> topic
    /// btreemap ordered so is hashable
    topic_map: RwLock<BiHashMap<BTreeSet<K>, String>>,
    /// the latest view number (for node lookup purposes)
    /// NOTE: supposed to represent a ViewNumber but we
    /// haven't made that atomic yet and we prefer lock-free
    latest_seen_view: Arc<AtomicU64>,
    #[cfg(feature = "hotshot-testing")]
    /// reliability_config
    reliability_config: Option<Box<dyn NetworkReliability>>,
    /// if we're a member of the DA committee or not
    is_da: bool,
    /// Killswitch sender
    kill_switch: channel::Sender<()>,
}

/// Networking implementation that uses libp2p
/// generic over `M` which is the message type
#[derive(Clone)]
pub struct Libp2pNetwork<K: SignatureKey + 'static> {
    /// holds the state of the libp2p network
    inner: Arc<Libp2pNetworkInner<K>>,
}

#[cfg(feature = "hotshot-testing")]
impl<TYPES: NodeType> TestableNetworkingImplementation<TYPES>
    for Libp2pNetwork<TYPES::SignatureKey>
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
    #[allow(clippy::panic)]
    fn generator(
        expected_node_count: usize,
        num_bootstrap: usize,
        _network_id: usize,
        da_committee_size: usize,
        _is_da: bool,
        reliability_config: Option<Box<dyn NetworkReliability>>,
        _secondary_network_delay: Duration,
    ) -> AsyncGenerator<Arc<Self>> {
        assert!(
            da_committee_size <= expected_node_count,
            "DA committee size must be less than or equal to total # nodes"
        );
        let bootstrap_addrs: PeerInfoVec = Arc::default();
        // We assign known_nodes' public key and stake value rather than read from config file since it's a test
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
        Box::pin({
            move |node_id| {
                info!(
                    "GENERATOR: Node id {:?}, is bootstrap: {:?}",
                    node_id,
                    node_id < num_bootstrap as u64
                );

                // pick a free, unused UDP port for testing
                let port = portpicker::pick_unused_port().expect("Could not find an open port");

                let addr =
                    Multiaddr::from_str(&format!("/ip4/127.0.0.1/udp/{port}/quic-v1")).unwrap();

                // We assign node's public key and stake value rather than read from config file since it's a test
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
                        .server_mode(true)
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
                        .server_mode(true)
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
                let bootstrap_addrs_ref = Arc::clone(&bootstrap_addrs);
                let keys = all_keys.clone();
                let da = da_keys.clone();
                let reliability_config_dup = reliability_config.clone();
                Box::pin(async move {
                    Arc::new(
                        match Libp2pNetwork::new(
                            Libp2pMetricsValue::default(),
                            config,
                            pubkey.clone(),
                            bootstrap_addrs_ref,
                            usize::try_from(node_id).unwrap(),
                            keys,
                            #[cfg(feature = "hotshot-testing")]
                            reliability_config_dup,
                            da.clone(),
                            da.contains(&pubkey),
                        )
                        .await
                        {
                            Ok(network) => network,
                            Err(err) => {
                                panic!("Failed to create libp2p network: {err:?}");
                            }
                        },
                    )
                })
            }
        })
    }

    fn in_flight_message_count(&self) -> Option<usize> {
        None
    }
}

/// Derive a Libp2p keypair from a given private key
///
/// # Errors
/// If we are unable to derive a new `SecretKey` from the `blake3`-derived
/// bytes.
pub fn derive_libp2p_keypair<K: SignatureKey>(
    private_key: &K::PrivateKey,
) -> anyhow::Result<Keypair> {
    // Derive a secondary key from our primary private key
    let derived_key = blake3::derive_key("libp2p key", &(bincode::serialize(&private_key)?));
    let derived_key = SecretKey::try_from_bytes(derived_key)?;

    // Create an `ed25519` keypair from the derived key
    Ok(ed25519::Keypair::from(derived_key).into())
}

/// Derive a Libp2p Peer ID from a given private key
///
/// # Errors
/// If we are unable to derive a Libp2p keypair
pub fn derive_libp2p_peer_id<K: SignatureKey>(
    private_key: &K::PrivateKey,
) -> anyhow::Result<PeerId> {
    // Get the derived keypair
    let keypair = derive_libp2p_keypair::<K>(private_key)?;

    // Return the PeerID derived from the public key
    Ok(PeerId::from_public_key(&keypair.public()))
}

impl<K: SignatureKey + 'static> Libp2pNetwork<K> {
    /// Create and return a Libp2p network from a network config file
    /// and various other configuration-specific values.
    ///
    /// # Errors
    /// If we are unable to parse a Multiaddress
    ///
    /// # Panics
    /// If we are unable to calculate the replication factor
    pub async fn from_config<TYPES: NodeType>(
        mut config: NetworkConfig<K>,
        bind_address: SocketAddr,
        pub_key: &K,
        priv_key: &K::PrivateKey,
        metrics: Libp2pMetricsValue,
    ) -> anyhow::Result<Self> {
        // Try to take our Libp2p config from our broader network config
        let libp2p_config = config
            .libp2p_config
            .take()
            .ok_or(anyhow!("Libp2p config not supplied"))?;

        // Derive our Libp2p keypair from our supplied private key
        let keypair = derive_libp2p_keypair::<K>(priv_key)?;

        // Convert our bind address to a `Multiaddr`
        let bind_address: Multiaddr = format!(
            "/{}/{}/udp/{}/quic-v1",
            if bind_address.is_ipv4() { "ip4" } else { "ip6" },
            bind_address.ip(),
            bind_address.port()
        )
        .parse()?;

        // Build our libp2p configuration from our global, network configuration
        let mut config_builder = NetworkNodeConfigBuilder::default();

        config_builder
            .replication_factor(
                NonZeroUsize::new(config.config.num_nodes_with_stake.get() - 2)
                    .expect("failed to calculate replication factor"),
            )
            .server_mode(libp2p_config.server_mode)
            .identity(keypair)
            .bound_addr(Some(bind_address.clone()))
            .mesh_params(Some(MeshParams {
                mesh_n_high: libp2p_config.mesh_n_high,
                mesh_n_low: libp2p_config.mesh_n_low,
                mesh_outbound_min: libp2p_config.mesh_outbound_min,
                mesh_n: libp2p_config.mesh_n,
            }));

        // Choose `mesh_n` random nodes to connect to for bootstrap
        let bootstrap_nodes = libp2p_config
            .bootstrap_nodes
            .into_iter()
            .choose_multiple(&mut StdRng::from_entropy(), libp2p_config.mesh_n);
        config_builder.to_connect_addrs(HashSet::from_iter(bootstrap_nodes.clone()));

        // Build the node's configuration
        let node_config = config_builder.build()?;

        // Calculate all keys so we can keep track of direct message recipients
        let mut all_keys = BTreeSet::new();
        let mut da_keys = BTreeSet::new();

        // Make a node DA if it is under the staked committee size
        for node in config.config.known_da_nodes {
            da_keys.insert(K::public_key(&node.stake_table_entry));
        }

        // Insert all known nodes into the set of all keys
        for node in config.config.known_nodes_with_stake {
            all_keys.insert(K::public_key(&node.stake_table_entry));
        }

        Ok(Libp2pNetwork::new(
            metrics,
            node_config,
            pub_key.clone(),
            Arc::new(RwLock::new(bootstrap_nodes)),
            usize::try_from(config.node_index)?,
            // NOTE: this introduces an invariant that the keys are assigned using this indexed
            // function
            all_keys,
            #[cfg(feature = "hotshot-testing")]
            None,
            da_keys.clone(),
            da_keys.contains(pub_key),
        )
        .await?)
    }

    /// Returns when network is ready
    pub async fn wait_for_ready(&self) {
        loop {
            if self.inner.is_ready.load(Ordering::Relaxed) {
                break;
            }
            async_sleep(Duration::from_secs(1)).await;
        }
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
        metrics: Libp2pMetricsValue,
        config: NetworkNodeConfig,
        pk: K,
        bootstrap_addrs: BootstrapAddrs,
        id: usize,
        // HACK
        quorum_public_keys: BTreeSet<K>,
        #[cfg(feature = "hotshot-testing")] reliability_config: Option<Box<dyn NetworkReliability>>,
        da_public_keys: BTreeSet<K>,
        is_da: bool,
    ) -> Result<Libp2pNetwork<K>, NetworkError> {
        // Error if there were no bootstrap nodes specified
        #[cfg(not(feature = "hotshot-testing"))]
        if bootstrap_addrs.read().await.len() == 0 {
            return Err(NetworkError::NoBootstrapNodesSpecified);
        }
        let (mut rx, network_handle) = spawn_network_node(config.clone(), id)
            .await
            .map_err(Into::<NetworkError>::into)?;
        // Make bootstrap mappings known
        if matches!(
            network_handle.config().node_type,
            NetworkNodeType::Bootstrap
        ) {
            let addr = network_handle.listen_addr();
            let pid = network_handle.peer_id();
            let mut bs_cp = bootstrap_addrs.write().await;
            bs_cp.push((pid, addr));
            drop(bs_cp);
        }

        let mut pubkey_pid_map = BiHashMap::new();
        pubkey_pid_map.insert(pk.clone(), network_handle.peer_id());

        let mut topic_map = BiHashMap::new();
        topic_map.insert(quorum_public_keys, QC_TOPIC.to_string());
        topic_map.insert(da_public_keys, "DA".to_string());

        let topic_map = RwLock::new(topic_map);

        // unbounded channels may not be the best choice (spammed?)
        // if bounded figure out a way to log dropped msgs
        let (sender, receiver) = unbounded();
        let (requests_tx, requests_rx) = channel(100);
        let (node_lookup_send, node_lookup_recv) = bounded(10);
        let (kill_tx, kill_rx) = bounded(1);
        rx.set_kill_switch(kill_rx);

        let mut result = Libp2pNetwork {
            inner: Arc::new(Libp2pNetworkInner {
                handle: Arc::new(network_handle),
                receiver,
                requests_rx: Mutex::new(Some(requests_rx)),
                sender: sender.clone(),
                pk,
                bootstrap_addrs,
                is_ready: Arc::new(AtomicBool::new(false)),
                // This is optimal for 10-30 nodes. TODO: parameterize this for both tests and examples
                // https://github.com/EspressoSystems/HotShot/issues/2088
                dht_timeout: Duration::from_secs(120),
                is_bootstrapped: Arc::new(AtomicBool::new(false)),
                metrics,
                topic_map,
                node_lookup_send,
                // Start the latest view from 0. "Latest" refers to "most recent view we are polling for
                // proposals on". We need this because to have consensus info injected we need a working
                // network already. In the worst case, we send a few lookups we don't need.
                latest_seen_view: Arc::new(AtomicU64::new(0)),
                #[cfg(feature = "hotshot-testing")]
                reliability_config,
                is_da,
                kill_switch: kill_tx,
            }),
        };

        result.handle_event_generator(sender, requests_tx, rx);
        result.spawn_node_lookup(node_lookup_recv);
        result.spawn_connect(id);

        Ok(result)
    }

    /// Spawns task for looking up nodes pre-emptively
    #[allow(clippy::cast_sign_loss, clippy::cast_precision_loss)]
    fn spawn_node_lookup(&self, mut node_lookup_recv: BoundedReceiver<Option<(ViewNumber, K)>>) {
        let handle = Arc::clone(&self.inner.handle);
        let dht_timeout = self.inner.dht_timeout;
        let latest_seen_view = Arc::clone(&self.inner.latest_seen_view);

        // deals with handling lookup queue. should be infallible
        async_spawn(async move {
            // cancels on shutdown
            while let Ok(Some((view_number, pk))) = node_lookup_recv.recv().await {
                /// defines lookahead threshold based on the constant
                #[allow(clippy::cast_possible_truncation)]
                const THRESHOLD: u64 = (LOOK_AHEAD as f64 * 0.8) as u64;

                trace!("Performing lookup for peer {:?}", pk);

                // only run if we are not too close to the next view number
                if latest_seen_view.load(Ordering::Relaxed) + THRESHOLD <= *view_number {
                    let pk_bytes = match bincode::serialize(&pk) {
                        Ok(serialized) => serialized,
                        Err(e) => {
                            tracing::error!("Failed to serialize public key; this should never happen. Error: {e}");
                            return;
                        }
                    };
                    // look up
                    if let Err(err) = handle.lookup_node(&pk_bytes, dht_timeout).await {
                        error!("Failed to perform lookup for key {:?}: {}", pk, err);
                    };
                }
            }
        });
    }

    /// Initiates connection to the outside world
    fn spawn_connect(&mut self, id: usize) {
        let pk = self.inner.pk.clone();
        let bootstrap_ref = Arc::clone(&self.inner.bootstrap_addrs);
        let handle = Arc::clone(&self.inner.handle);
        let is_bootstrapped = Arc::clone(&self.inner.is_bootstrapped);
        let node_type = self.inner.handle.config().node_type;
        let is_da = self.inner.is_da;

        async_spawn({
            let is_ready = Arc::clone(&self.inner.is_ready);
            async move {
                let bs_addrs = bootstrap_ref.read().await.clone();

                debug!("Finished adding bootstrap addresses.");
                handle.add_known_peers(bs_addrs).await.unwrap();

                handle.begin_bootstrap().await?;

                while !is_bootstrapped.load(Ordering::Relaxed) {
                    async_sleep(Duration::from_secs(1)).await;
                    handle.begin_bootstrap().await?;
                }

                handle.subscribe(QC_TOPIC.to_string()).await.unwrap();

                // only subscribe to DA events if we are DA
                if is_da {
                    handle.subscribe("DA".to_string()).await.unwrap();
                }
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
                //
                // Note: this serialization should never fail,
                // and if it does the error is unrecoverable.
                while handle
                    .put_record(
                        &bincode::serialize(&pk).unwrap(),
                        &bincode::serialize(&handle.peer_id()).unwrap(),
                    )
                    .await
                    .is_err()
                {
                    async_sleep(Duration::from_secs(1)).await;
                }
                info!(
                    "Node {:?} is ready, type: {:?}",
                    handle.peer_id(),
                    node_type
                );

                while handle
                    .put_record(
                        &bincode::serialize(&handle.peer_id()).unwrap(),
                        &bincode::serialize(&pk).unwrap(),
                    )
                    .await
                    .is_err()
                {
                    async_sleep(Duration::from_secs(1)).await;
                }
                // perform connection
                info!("WAITING TO CONNECT ON NODE {:?}", id);

                // Wait for the network to connect to the required number of peers
                if let Err(e) = handle.wait_to_connect(4, id).await {
                    error!("Failed to connect to peers: {:?}", e);
                    return Err::<(), NetworkError>(e.into());
                }

                is_ready.store(true, Ordering::Relaxed);
                Ok::<(), NetworkError>(())
            }
        });
    }

    /// Handle events
    async fn handle_recvd_events(
        &self,
        msg: NetworkEvent,
        sender: &UnboundedSender<Vec<u8>>,
        mut request_tx: Sender<(Vec<u8>, ResponseChannel<Response>)>,
    ) -> Result<(), NetworkError> {
        match msg {
            GossipMsg(msg) => {
                sender
                    .send(msg)
                    .await
                    .map_err(|_| NetworkError::ChannelSend)?;
            }
            DirectRequest(msg, _pid, chan) => {
                sender
                    .send(msg)
                    .await
                    .map_err(|_| NetworkError::ChannelSend)?;
                if self
                    .inner
                    .handle
                    .direct_response(
                        chan,
                        &bincode::serialize(&Empty { byte: 0u8 })
                            .map_err(|e| NetworkError::Libp2p { source: e.into() })?,
                    )
                    .await
                    .is_err()
                {
                    error!("failed to ack!");
                };
            }
            DirectResponse(_msg, _) => {}
            NetworkEvent::IsBootstrapped => {
                error!("handle_recvd_events received `NetworkEvent::IsBootstrapped`, which should be impossible.");
            }
            NetworkEvent::ResponseRequested(Request(msg), chan) => {
                let res = request_tx.try_send((msg, chan));
                res.map_err(|_| NetworkError::ChannelSend)?;
            }
            NetworkEvent::ConnectedPeersUpdate(_) => {}
        }
        Ok::<(), NetworkError>(())
    }

    /// task to propagate messages to handlers
    /// terminates on shut down of network
    fn handle_event_generator(
        &self,
        sender: UnboundedSender<Vec<u8>>,
        request_tx: Sender<(Vec<u8>, ResponseChannel<Response>)>,
        mut network_rx: NetworkNodeReceiver,
    ) {
        let handle = self.clone();
        let is_bootstrapped = Arc::clone(&self.inner.is_bootstrapped);
        async_spawn(async move {
            let Some(mut kill_switch) = network_rx.take_kill_switch() else {
                tracing::error!(
                    "`spawn_handle` was called on a network handle that was already closed"
                );
                return;
            };
            let mut kill_switch = kill_switch.recv().boxed();
            let mut next_msg = network_rx.recv().boxed();

            loop {
                let msg_or_killed = futures::future::select(next_msg, kill_switch).await;
                match msg_or_killed {
                    Either::Left((Ok(message), other_stream)) => {
                        match &message {
                            NetworkEvent::IsBootstrapped => {
                                is_bootstrapped.store(true, Ordering::Relaxed);
                            }
                            GossipMsg(_)
                            | DirectRequest(_, _, _)
                            | DirectResponse(_, _)
                            | NetworkEvent::ResponseRequested(Request(_), _) => {
                                let _ = handle
                                    .handle_recvd_events(message, &sender, request_tx.clone())
                                    .await;
                            }
                            NetworkEvent::ConnectedPeersUpdate(num_peers) => {
                                handle.inner.metrics.num_connected_peers.set(*num_peers);
                            }
                        }
                        // re-set the `kill_switch` for the next loop
                        kill_switch = other_stream;
                        // re-set `receiver.recv()` for the next loop
                        next_msg = network_rx.recv().boxed();
                    }
                    Either::Left((Err(_), _)) => {
                        warn!("Network receiver shut down!");
                        return;
                    }
                    Either::Right(_) => {
                        warn!("Event Handler shutdown");
                        return;
                    }
                }
            }
        });
    }
}

#[async_trait]
impl<K: SignatureKey + 'static> ConnectedNetwork<K> for Libp2pNetwork<K> {
    async fn request_data<TYPES: NodeType>(
        &self,
        request: Vec<u8>,
        recipient: &K,
    ) -> Result<Vec<u8>, NetworkError> {
        self.wait_for_ready().await;

        let pid = match self
            .inner
            .handle
            .lookup_node(
                &bincode::serialize(&recipient)
                    .map_err(|e| NetworkError::Libp2p { source: e.into() })?,
                self.inner.dht_timeout,
            )
            .await
        {
            Ok(pid) => pid,
            Err(err) => {
                self.inner.metrics.num_failed_messages.add(1);
                error!(
                    "Failed to message {:?} because could not find recipient peer id for pk {:?}",
                    request, recipient
                );
                return Err(NetworkError::Libp2p {
                    source: Box::new(err),
                });
            }
        };
        let result = match self.inner.handle.request_data(&request, pid).await {
            Ok(response) => match response {
                Some(msg) => {
                    if msg.0.len() < 8 {
                        return Err(NetworkError::FailedToDeserialize {
                            source: anyhow!("insufficient bytes"),
                        });
                    }
                    let res: Message<TYPES> = bincode::deserialize(&msg.0)
                        .map_err(|e| NetworkError::FailedToDeserialize { source: e.into() })?;

                    match res.kind {
                        MessageKind::Data(DataResponse(data)) => data,
                        _ => ResponseMessage::NotFound,
                    }
                }
                None => ResponseMessage::NotFound,
            },
            Err(e) => {
                self.inner.metrics.num_failed_messages.add(1);
                return Err(e.into());
            }
        };

        Ok(bincode::serialize(&result).map_err(|e| NetworkError::Libp2p { source: e.into() })?)
    }

    async fn spawn_request_receiver_task(
        &self,
    ) -> Option<mpsc::Receiver<(Vec<u8>, network::ResponseChannel<Vec<u8>>)>> {
        let mut internal_rx = self.inner.requests_rx.lock().await.take()?;
        let handle = Arc::clone(&self.inner.handle);
        let (mut tx, rx) = mpsc::channel(100);
        async_spawn(async move {
            while let Some((request, chan)) = internal_rx.next().await {
                let (response_tx, response_rx) = futures::channel::oneshot::channel();
                if tx
                    .try_send((
                        request,
                        network::ResponseChannel {
                            sender: response_tx,
                        },
                    ))
                    .is_err()
                {
                    continue;
                }
                let Ok(response) = response_rx.await else {
                    continue;
                };

                let _ = handle.respond_data(response, chan).await;
            }
        });

        Some(rx)
    }
    #[instrument(name = "Libp2pNetwork::ready_blocking", skip_all)]
    async fn wait_for_ready(&self) {
        self.wait_for_ready().await;
    }

    fn pause(&self) {
        unimplemented!("Pausing not implemented for the Libp2p network");
    }

    fn resume(&self) {
        unimplemented!("Resuming not implemented for the Libp2p network");
    }

    #[instrument(name = "Libp2pNetwork::shut_down", skip_all)]
    fn shut_down<'a, 'b>(&'a self) -> BoxSyncFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            let _ = self.inner.handle.shutdown().await;
            let _ = self.inner.node_lookup_send.send(None).await;
            let _ = self.inner.kill_switch.send(()).await;
        };
        boxed_sync(closure)
    }

    #[instrument(name = "Libp2pNetwork::broadcast_message", skip_all)]
    async fn broadcast_message(
        &self,
        message: Vec<u8>,
        recipients: BTreeSet<K>,
        _broadcast_delay: BroadcastDelay,
    ) -> Result<(), NetworkError> {
        self.wait_for_ready().await;
        trace!(
            "broadcasting msg: {:?} with nodes: {:?} connected",
            message,
            self.inner.handle.connected_pids().await
        );

        let topic_map = self.inner.topic_map.read().await;
        let topic = topic_map
            .get_by_left(&recipients)
            .ok_or(NetworkError::Libp2p {
                source: Box::new(NetworkNodeHandleError::NoSuchTopic),
            })?
            .clone();
        trace!("broadcasting to topic: {}", topic);

        // gossip doesn't broadcast from itself, so special case
        if recipients.contains(&self.inner.pk) {
            // send to self
            self.inner
                .sender
                .send(message.clone())
                .await
                .map_err(|_| NetworkError::ShutDown)?;
        }

        // NOTE: metrics is threadsafe, so clone is fine (and lightweight)
        #[cfg(feature = "hotshot-testing")]
        {
            let metrics = self.inner.metrics.clone();
            if let Some(ref config) = &self.inner.reliability_config {
                let handle = Arc::clone(&self.inner.handle);

                let fut = config.clone().chaos_send_msg(
                    message,
                    Arc::new(move |msg: Vec<u8>| {
                        let topic_2 = topic.clone();
                        let handle_2 = Arc::clone(&handle);
                        let metrics_2 = metrics.clone();
                        boxed_sync(async move {
                            if let Err(e) = handle_2.gossip_no_serialize(topic_2, msg).await {
                                metrics_2.num_failed_messages.add(1);
                                warn!("Failed to broadcast to libp2p: {:?}", e);
                            }
                        })
                    }),
                );
                async_spawn(fut);
                return Ok(());
            }
        }

        if let Err(e) = self.inner.handle.gossip(topic, &message).await {
            self.inner.metrics.num_failed_messages.add(1);
            return Err(e.into());
        }

        Ok(())
    }

    #[instrument(name = "Libp2pNetwork::da_broadcast_message", skip_all)]
    async fn da_broadcast_message(
        &self,
        message: Vec<u8>,
        recipients: BTreeSet<K>,
        _broadcast_delay: BroadcastDelay,
    ) -> Result<(), NetworkError> {
        let future_results = recipients
            .into_iter()
            .map(|r| self.direct_message(message.clone(), r));
        let results = join_all(future_results).await;

        let errors: Vec<_> = results
            .into_iter()
            .filter_map(|r| match r {
                Err(NetworkError::Libp2p { source }) => Some(source),
                _ => None,
            })
            .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(NetworkError::Libp2pMulti { sources: errors })
        }
    }

    #[instrument(name = "Libp2pNetwork::direct_message", skip_all)]
    async fn direct_message(&self, message: Vec<u8>, recipient: K) -> Result<(), NetworkError> {
        // short circuit if we're dming ourselves
        if recipient == self.inner.pk {
            // panic if we already shut down?
            self.inner
                .sender
                .send(message)
                .await
                .map_err(|_x| NetworkError::ShutDown)?;
            return Ok(());
        }

        self.wait_for_ready().await;

        let pid = match self
            .inner
            .handle
            .lookup_node(
                &bincode::serialize(&recipient)
                    .map_err(|e| NetworkError::Libp2p { source: e.into() })?,
                self.inner.dht_timeout,
            )
            .await
        {
            Ok(pid) => pid,
            Err(err) => {
                self.inner.metrics.num_failed_messages.add(1);
                error!(
                    "Failed to message {:?} because could not find recipient peer id for pk {:?}",
                    message, recipient
                );
                return Err(NetworkError::Libp2p {
                    source: Box::new(err),
                });
            }
        };

        #[cfg(feature = "hotshot-testing")]
        {
            let metrics = self.inner.metrics.clone();
            if let Some(ref config) = &self.inner.reliability_config {
                let handle = Arc::clone(&self.inner.handle);

                let fut = config.clone().chaos_send_msg(
                    message,
                    Arc::new(move |msg: Vec<u8>| {
                        let handle_2 = Arc::clone(&handle);
                        let metrics_2 = metrics.clone();
                        boxed_sync(async move {
                            if let Err(e) = handle_2.direct_request_no_serialize(pid, msg).await {
                                metrics_2.num_failed_messages.add(1);
                                warn!("Failed to broadcast to libp2p: {:?}", e);
                            }
                        })
                    }),
                );
                async_spawn(fut);
                return Ok(());
            }
        }

        match self.inner.handle.direct_request(pid, &message).await {
            Ok(()) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    /// Receive one or many messages from the underlying network.
    ///
    /// # Errors
    /// If there is a network-related failure.
    #[instrument(name = "Libp2pNetwork::recv_msgs", skip_all)]
    async fn recv_msgs(&self) -> Result<Vec<Vec<u8>>, NetworkError> {
        let result = self
            .inner
            .receiver
            .drain_at_least_one()
            .await
            .map_err(|_x| NetworkError::ShutDown)?;

        Ok(result)
    }

    #[instrument(name = "Libp2pNetwork::queue_node_lookup", skip_all)]
    fn queue_node_lookup(
        &self,
        view_number: ViewNumber,
        pk: K,
    ) -> Result<(), TrySendError<Option<(ViewNumber, K)>>> {
        self.inner
            .node_lookup_send
            .try_send(Some((view_number, pk)))
    }

    /// handles view update
    async fn update_view<'a, TYPES>(&'a self, view: u64, membership: &TYPES::Membership)
    where
        TYPES: NodeType<SignatureKey = K> + 'a,
    {
        let future_view = <TYPES as NodeType>::Time::new(view) + LOOK_AHEAD;
        let future_leader = membership.leader(future_view);

        let _ = self
            .queue_node_lookup(ViewNumber::new(*future_view), future_leader)
            .map_err(|err| tracing::warn!("failed to process node lookup request: {err}"));
    }
}
