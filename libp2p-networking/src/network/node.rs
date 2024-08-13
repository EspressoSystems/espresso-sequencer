// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

/// configuration for the libp2p network (e.g. how it should be built)
mod config;

/// libp2p network handle
/// allows for control over the libp2p network
mod handle;

use std::{
    collections::{HashMap, HashSet},
    iter,
    num::{NonZeroU32, NonZeroUsize},
    time::Duration,
};

use async_compatibility_layer::{
    art::async_spawn,
    channel::{unbounded, UnboundedReceiver, UnboundedRecvError, UnboundedSender},
};
use futures::{channel::mpsc, select, FutureExt, SinkExt, StreamExt};
use hotshot_types::{
    constants::KAD_DEFAULT_REPUB_INTERVAL_SEC,
    request_response::{Request, Response},
    traits::signature_key::SignatureKey,
};
use libp2p::{
    autonat,
    core::transport::ListenerId,
    gossipsub::{
        Behaviour as Gossipsub, ConfigBuilder as GossipsubConfigBuilder, Event as GossipEvent,
        Message as GossipsubMessage, MessageAuthenticity, MessageId, Topic, ValidationMode,
    },
    identify::{
        Behaviour as IdentifyBehaviour, Config as IdentifyConfig, Event as IdentifyEvent,
        Info as IdentifyInfo,
    },
    identity::Keypair,
    kad::{store::MemoryStore, Behaviour, Config, Mode, Record},
    request_response::{
        Behaviour as RequestResponse, Config as RequestResponseConfig, ProtocolSupport,
    },
    swarm::SwarmEvent,
    Multiaddr, StreamProtocol, Swarm, SwarmBuilder,
};
use libp2p_identity::PeerId;
use rand::{prelude::SliceRandom, thread_rng};
use snafu::ResultExt;
use tracing::{debug, error, info, info_span, instrument, warn, Instrument};

pub use self::{
    config::{
        MeshParams, NetworkNodeConfig, NetworkNodeConfigBuilder, NetworkNodeConfigBuilderError,
        DEFAULT_REPLICATION_FACTOR,
    },
    handle::{
        network_node_handle_error, spawn_network_node, NetworkNodeHandle, NetworkNodeHandleError,
        NetworkNodeReceiver,
    },
};
use super::{
    behaviours::dht::bootstrap::{self, DHTBootstrapTask, InputEvent},
    error::{GossipsubBuildSnafu, GossipsubConfigSnafu, NetworkError, TransportSnafu},
    gen_transport, BoxedTransport, ClientRequest, NetworkDef, NetworkEvent, NetworkEventInternal,
    NetworkNodeType,
};
use crate::network::behaviours::{
    dht::{DHTBehaviour, DHTProgress, KadPutQuery, NUM_REPLICATED_TO_TRUST},
    direct_message::{DMBehaviour, DMRequest},
    exponential_backoff::ExponentialBackoff,
    request_response::RequestResponseState,
};

/// Maximum size of a message
pub const MAX_GOSSIP_MSG_SIZE: usize = 2_000_000_000;

/// Wrapped num of connections
pub const ESTABLISHED_LIMIT: NonZeroU32 =
    unsafe { NonZeroU32::new_unchecked(ESTABLISHED_LIMIT_UNWR) };
/// Number of connections to a single peer before logging an error
pub const ESTABLISHED_LIMIT_UNWR: u32 = 10;

/// Network definition
#[derive(custom_debug::Debug)]
pub struct NetworkNode<K: SignatureKey + 'static> {
    /// pub/private key from with peer_id is derived
    identity: Keypair,
    /// peer id of network node
    peer_id: PeerId,
    /// the swarm of networkbehaviours
    #[debug(skip)]
    swarm: Swarm<NetworkDef>,
    /// the configuration parameters of the netework
    config: NetworkNodeConfig<K>,
    /// the listener id we are listening on, if it exists
    listener_id: Option<ListenerId>,
    /// Handler for requests and response behavior events.
    request_response_state: RequestResponseState,
    /// Handler for direct messages
    direct_message_state: DMBehaviour,
    /// Handler for DHT Events
    dht_handler: DHTBehaviour,
    /// Channel to resend requests, set to Some when we call `spawn_listeners`
    resend_tx: Option<UnboundedSender<ClientRequest>>,
    /// Send to the bootstrap task to tell it to start a bootstrap
    bootstrap_tx: Option<mpsc::Sender<bootstrap::InputEvent>>,
}

impl<K: SignatureKey + 'static> NetworkNode<K> {
    /// Returns number of peers this node is connected to
    pub fn num_connected(&self) -> usize {
        self.swarm.connected_peers().count()
    }

    /// return hashset of PIDs this node is connected to
    pub fn connected_pids(&self) -> HashSet<PeerId> {
        self.swarm.connected_peers().copied().collect()
    }

    /// starts the swarm listening on `listen_addr`
    /// and optionally dials into peer `known_peer`
    /// returns the address the swarm is listening upon
    #[instrument(skip(self))]
    pub async fn start_listen(
        &mut self,
        listen_addr: Multiaddr,
    ) -> Result<Multiaddr, NetworkError> {
        self.listener_id = Some(self.swarm.listen_on(listen_addr).context(TransportSnafu)?);
        let addr = loop {
            if let Some(SwarmEvent::NewListenAddr { address, .. }) = self.swarm.next().await {
                break address;
            }
        };
        info!("Libp2p listening on {:?}", addr);
        Ok(addr)
    }

    /// initialize the DHT with known peers
    /// add the peers to kademlia and then
    /// the `spawn_listeners` function
    /// will start connecting to peers
    #[instrument(skip(self))]
    pub fn add_known_peers(&mut self, known_peers: &[(PeerId, Multiaddr)]) {
        debug!("Adding {} known peers", known_peers.len());
        let behaviour = self.swarm.behaviour_mut();
        let mut bs_nodes = HashMap::<PeerId, HashSet<Multiaddr>>::new();
        let mut shuffled = known_peers.iter().collect::<Vec<_>>();
        shuffled.shuffle(&mut thread_rng());
        for (peer_id, addr) in shuffled {
            if *peer_id != self.peer_id {
                behaviour.dht.add_address(peer_id, addr.clone());
                behaviour.autonat.add_server(*peer_id, Some(addr.clone()));
                bs_nodes.insert(*peer_id, iter::once(addr.clone()).collect());
            }
        }
    }

    /// Creates a new `Network` with the given settings.
    ///
    /// Currently:
    ///   * Generates a random key pair and associated [`PeerId`]
    ///   * Launches a hopefully production ready transport:
    ///       QUIC v1 (RFC 9000) + DNS
    ///   * Generates a connection to the "broadcast" topic
    ///   * Creates a swarm to manage peers and events
    #[instrument]
    pub async fn new(config: NetworkNodeConfig<K>) -> Result<Self, NetworkError> {
        // Generate a random `KeyPair` if one is not specified
        let identity = if let Some(ref kp) = config.identity {
            kp.clone()
        } else {
            Keypair::generate_ed25519()
        };

        // Get the `PeerId` from the `KeyPair`
        let peer_id = PeerId::from(identity.public());

        // Generate the transport from the identity, stake table, and auth message
        let transport: BoxedTransport = gen_transport::<K>(
            identity.clone(),
            config.stake_table.clone(),
            config.auth_message.clone(),
        )
        .await?;

        // Generate the swarm
        let mut swarm: Swarm<NetworkDef> = {
            // Use the hash of the message's contents as the ID
            // Use blake3 for much paranoia at very high speeds
            let message_id_fn = |message: &GossipsubMessage| {
                let hash = blake3::hash(&message.data);
                MessageId::from(hash.as_bytes().to_vec())
            };

            let params = if let Some(ref params) = config.mesh_params {
                params.clone()
            } else {
                // NOTE this should most likely be a builder pattern
                // at some point in the future.
                match config.node_type {
                    NetworkNodeType::Bootstrap => MeshParams {
                        mesh_n_high: 1000, // make this super high in case we end up scaling to 1k
                        // nodes
                        mesh_n_low: 10,
                        mesh_outbound_min: 5,
                        mesh_n: 15,
                    },
                    NetworkNodeType::Regular => MeshParams {
                        mesh_n_high: 15,
                        mesh_n_low: 8,
                        mesh_outbound_min: 4,
                        mesh_n: 12,
                    },
                    NetworkNodeType::Conductor => MeshParams {
                        mesh_n_high: 21,
                        mesh_n_low: 8,
                        mesh_outbound_min: 4,
                        mesh_n: 12,
                    },
                }
            };

            // Create a custom gossipsub
            let gossipsub_config = GossipsubConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(1))
                // Force all messages to have valid signatures
                .validation_mode(ValidationMode::Strict)
                .history_gossip(10)
                .mesh_n_high(params.mesh_n_high)
                .mesh_n_low(params.mesh_n_low)
                .mesh_outbound_min(params.mesh_outbound_min)
                .mesh_n(params.mesh_n)
                .history_length(10)
                .max_transmit_size(MAX_GOSSIP_MSG_SIZE)
                // Use the (blake3) hash of a message as its ID
                .message_id_fn(message_id_fn)
                .build()
                .map_err(|s| {
                    GossipsubConfigSnafu {
                        message: s.to_string(),
                    }
                    .build()
                })?;

            // - Build a gossipsub network behavior
            let gossipsub: Gossipsub = Gossipsub::new(
                // TODO do we even need this?
                // <https://github.com/EspressoSystems/hotshot/issues/42>
                // if messages are signed at the the consensus level AND the network
                // level (noise), this feels redundant.
                MessageAuthenticity::Signed(identity.clone()),
                gossipsub_config,
            )
            .map_err(|s| GossipsubBuildSnafu { message: s }.build())?;

            //   Build a identify network behavior needed for own
            //   node connection information
            //   E.g. this will answer the question: how are other nodes
            //   seeing the peer from behind a NAT
            let identify_cfg =
                IdentifyConfig::new("HotShot/identify/1.0".to_string(), identity.public());
            let identify = IdentifyBehaviour::new(identify_cfg);

            // - Build DHT needed for peer discovery
            let mut kconfig = Config::default();
            // 8 hours by default
            let record_republication_interval = config
                .republication_interval
                .unwrap_or(Duration::from_secs(KAD_DEFAULT_REPUB_INTERVAL_SEC));
            let ttl = Some(config.ttl.unwrap_or(16 * record_republication_interval));
            kconfig
                .set_parallelism(NonZeroUsize::new(5).unwrap())
                .set_provider_publication_interval(Some(record_republication_interval))
                .set_publication_interval(Some(record_republication_interval))
                .set_record_ttl(ttl);

            // allowing panic here because something is very wrong if this fales
            #[allow(clippy::panic)]
            if let Some(factor) = config.replication_factor {
                kconfig.set_replication_factor(factor);
            } else {
                panic!("Replication factor not set");
            }

            let mut kadem = Behaviour::with_config(peer_id, MemoryStore::new(peer_id), kconfig);
            if config.server_mode {
                kadem.set_mode(Some(Mode::Server));
            }

            let rrconfig = RequestResponseConfig::default();

            let direct_message: libp2p::request_response::cbor::Behaviour<Vec<u8>, Vec<u8>> =
                RequestResponse::new(
                    [(
                        StreamProtocol::new("/HotShot/direct_message/1.0"),
                        ProtocolSupport::Full,
                    )]
                    .into_iter(),
                    rrconfig.clone(),
                );
            let request_response: libp2p::request_response::cbor::Behaviour<Request, Response> =
                RequestResponse::new(
                    [(
                        StreamProtocol::new("/HotShot/request_response/1.0"),
                        ProtocolSupport::Full,
                    )]
                    .into_iter(),
                    rrconfig.clone(),
                );

            let autonat_config = autonat::Config {
                only_global_ips: false,
                ..Default::default()
            };

            let network = NetworkDef::new(
                gossipsub,
                kadem,
                identify,
                direct_message,
                request_response,
                autonat::Behaviour::new(peer_id, autonat_config),
            );

            // build swarm
            let swarm = SwarmBuilder::with_existing_identity(identity.clone());
            #[cfg(async_executor_impl = "async-std")]
            let swarm = swarm.with_async_std();
            #[cfg(async_executor_impl = "tokio")]
            let swarm = swarm.with_tokio();

            swarm
                .with_other_transport(|_| transport)
                .unwrap()
                .with_behaviour(|_| network)
                .unwrap()
                .build()
        };
        for (peer, addr) in &config.to_connect_addrs {
            if peer != swarm.local_peer_id() {
                swarm.behaviour_mut().add_address(peer, addr.clone());
            }
        }

        Ok(Self {
            identity,
            peer_id,
            swarm,
            config: config.clone(),
            listener_id: None,
            request_response_state: RequestResponseState::default(),
            direct_message_state: DMBehaviour::default(),
            dht_handler: DHTBehaviour::new(
                peer_id,
                config
                    .replication_factor
                    .unwrap_or(NonZeroUsize::new(4).unwrap()),
            ),
            resend_tx: None,
            bootstrap_tx: None,
        })
    }

    /// Publish a key/value to the kv store.
    /// Once replicated upon all nodes, the caller is notified over
    /// `chan`. If there is an error, a [`super::error::DHTError`] is
    /// sent instead.
    ///
    /// # Panics
    /// If the default replication factor is `None`
    pub fn put_record(&mut self, mut query: KadPutQuery) {
        let record = Record::new(query.key.clone(), query.value.clone());
        match self.swarm.behaviour_mut().dht.put_record(
            record,
            libp2p::kad::Quorum::N(
                NonZeroUsize::try_from(self.dht_handler.replication_factor().get() / 2)
                    .expect("replication factor should be bigger than 0"),
            ),
        ) {
            Err(e) => {
                // failed try again later
                query.progress = DHTProgress::NotStarted;
                query.backoff.start_next(false);
                error!("Error publishing to DHT: {e:?} for peer {:?}", self.peer_id);
            }
            Ok(qid) => {
                debug!("Published record to DHT with qid {:?}", qid);
                let query = KadPutQuery {
                    progress: DHTProgress::InProgress(qid),
                    ..query
                };
                self.dht_handler.put_record(qid, query);
            }
        }
    }

    /// event handler for client events
    /// currectly supported actions include
    /// - shutting down the swarm
    /// - gossipping a message to known peers on the `global` topic
    /// - returning the id of the current peer
    /// - subscribing to a topic
    /// - unsubscribing from a toipc
    /// - direct messaging a peer
    #[instrument(skip(self))]
    async fn handle_client_requests(
        &mut self,
        msg: Result<ClientRequest, UnboundedRecvError>,
    ) -> Result<bool, NetworkError> {
        let behaviour = self.swarm.behaviour_mut();
        match msg {
            Ok(msg) => {
                match msg {
                    ClientRequest::BeginBootstrap => {
                        debug!("Beginning Libp2p bootstrap");
                        let _ = self.swarm.behaviour_mut().dht.bootstrap();
                    }
                    ClientRequest::LookupPeer(pid, chan) => {
                        let id = self.swarm.behaviour_mut().dht.get_closest_peers(pid);
                        self.dht_handler
                            .in_progress_get_closest_peers
                            .insert(id, chan);
                    }
                    ClientRequest::GetRoutingTable(chan) => {
                        self.dht_handler
                            .print_routing_table(&mut self.swarm.behaviour_mut().dht);
                        if chan.send(()).is_err() {
                            warn!("Tried to notify client but client not tracking anymore");
                        }
                    }
                    ClientRequest::PutDHT { key, value, notify } => {
                        let query = KadPutQuery {
                            progress: DHTProgress::NotStarted,
                            notify,
                            key,
                            value,
                            backoff: ExponentialBackoff::default(),
                        };
                        self.put_record(query);
                    }
                    ClientRequest::GetConnectedPeerNum(s) => {
                        if s.send(self.num_connected()).is_err() {
                            error!("error sending peer number to client");
                        }
                    }
                    ClientRequest::GetConnectedPeers(s) => {
                        if s.send(self.connected_pids()).is_err() {
                            error!("error sending peer set to client");
                        }
                    }
                    ClientRequest::GetDHT {
                        key,
                        notify,
                        retry_count,
                    } => {
                        self.dht_handler.record(
                            key,
                            notify,
                            NonZeroUsize::new(NUM_REPLICATED_TO_TRUST).unwrap(),
                            ExponentialBackoff::default(),
                            retry_count,
                            &mut self.swarm.behaviour_mut().dht,
                        );
                    }
                    ClientRequest::IgnorePeers(_peers) => {
                        // NOTE used by test with conductor only
                    }
                    ClientRequest::Shutdown => {
                        if let Some(listener_id) = self.listener_id {
                            self.swarm.remove_listener(listener_id);
                        }

                        return Ok(true);
                    }
                    ClientRequest::GossipMsg(topic, contents) => {
                        behaviour.publish_gossip(Topic::new(topic.clone()), contents.clone());
                    }
                    ClientRequest::Subscribe(t, chan) => {
                        behaviour.subscribe_gossip(&t);
                        if let Some(chan) = chan {
                            if chan.send(()).is_err() {
                                error!("finished subscribing but response channel dropped");
                            }
                        }
                    }
                    ClientRequest::Unsubscribe(t, chan) => {
                        behaviour.unsubscribe_gossip(&t);
                        if let Some(chan) = chan {
                            if chan.send(()).is_err() {
                                error!("finished unsubscribing but response channel dropped");
                            }
                        }
                    }
                    ClientRequest::DirectRequest {
                        pid,
                        contents,
                        retry_count,
                    } => {
                        debug!("Sending direct request to {:?}", pid);
                        let id = behaviour.add_direct_request(pid, contents.clone());
                        let req = DMRequest {
                            peer_id: pid,
                            data: contents,
                            backoff: ExponentialBackoff::default(),
                            retry_count,
                        };
                        self.direct_message_state.add_direct_request(req, id);
                    }
                    ClientRequest::DirectResponse(chan, msg) => {
                        behaviour.add_direct_response(chan, msg);
                    }
                    ClientRequest::DataRequest {
                        request,
                        peer,
                        chan,
                    } => {
                        let id = behaviour.request_response.send_request(&peer, request);
                        self.request_response_state.add_request(id, chan);
                    }
                    ClientRequest::DataResponse { response, chan } => {
                        if behaviour
                            .request_response
                            .send_response(chan, response)
                            .is_err()
                        {
                            debug!("Data response dropped because client is no longer connected");
                        }
                    }
                    ClientRequest::AddKnownPeers(peers) => {
                        self.add_known_peers(&peers);
                    }
                    ClientRequest::Prune(pid) => {
                        if self.swarm.disconnect_peer_id(pid).is_err() {
                            warn!("Could not disconnect from {:?}", pid);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error receiving msg in main behaviour loop: {:?}", e);
            }
        }
        Ok(false)
    }

    /// event handler for events emitted from the swarm
    #[allow(clippy::type_complexity)]
    #[instrument(skip(self))]
    async fn handle_swarm_events(
        &mut self,
        event: SwarmEvent<NetworkEventInternal>,
        send_to_client: &UnboundedSender<NetworkEvent>,
    ) -> Result<(), NetworkError> {
        // Make the match cleaner
        debug!("Swarm event observed {:?}", event);

        #[allow(deprecated)]
        match event {
            SwarmEvent::ConnectionEstablished {
                connection_id: _,
                peer_id,
                endpoint,
                num_established,
                concurrent_dial_errors,
                established_in: _established_in,
            } => {
                if num_established > ESTABLISHED_LIMIT {
                    error!(
                        "Num concurrent connections to a single peer exceeding {:?} at {:?}!",
                        ESTABLISHED_LIMIT, num_established
                    );
                } else {
                    debug!(
                        "Connection established with {:?} at {:?} with {:?} concurrent dial errors",
                        peer_id, endpoint, concurrent_dial_errors
                    );
                }

                // Send the number of connected peers to the client
                send_to_client
                    .send(NetworkEvent::ConnectedPeersUpdate(self.num_connected()))
                    .await
                    .map_err(|_e| NetworkError::StreamClosed)?;
            }
            SwarmEvent::ConnectionClosed {
                connection_id: _,
                peer_id,
                endpoint,
                num_established,
                cause,
            } => {
                if num_established > ESTABLISHED_LIMIT_UNWR {
                    error!(
                        "Num concurrent connections to a single peer exceeding {:?} at {:?}!",
                        ESTABLISHED_LIMIT, num_established
                    );
                } else {
                    debug!(
                        "Connection closed with {:?} at {:?} due to {:?}",
                        peer_id, endpoint, cause
                    );
                }

                // Send the number of connected peers to the client
                send_to_client
                    .send(NetworkEvent::ConnectedPeersUpdate(self.num_connected()))
                    .await
                    .map_err(|_e| NetworkError::StreamClosed)?;
            }
            SwarmEvent::Dialing {
                peer_id,
                connection_id: _,
            } => {
                debug!("Attempting to dial {:?}", peer_id);
            }
            SwarmEvent::ListenerClosed {
                listener_id: _,
                addresses: _,
                reason: _,
            }
            | SwarmEvent::NewListenAddr {
                listener_id: _,
                address: _,
            }
            | SwarmEvent::ExpiredListenAddr {
                listener_id: _,
                address: _,
            }
            | SwarmEvent::NewExternalAddrCandidate { .. }
            | SwarmEvent::ExternalAddrExpired { .. }
            | SwarmEvent::IncomingConnection {
                connection_id: _,
                local_addr: _,
                send_back_addr: _,
            } => {}
            SwarmEvent::Behaviour(b) => {
                let maybe_event = match b {
                    NetworkEventInternal::DHTEvent(e) => self
                        .dht_handler
                        .dht_handle_event(e, self.swarm.behaviour_mut().dht.store_mut()),
                    NetworkEventInternal::IdentifyEvent(e) => {
                        // NOTE feed identified peers into kademlia's routing table for peer discovery.
                        if let IdentifyEvent::Received {
                            peer_id,
                            info:
                                IdentifyInfo {
                                    listen_addrs,
                                    protocols: _,
                                    public_key: _,
                                    protocol_version: _,
                                    agent_version: _,
                                    observed_addr: _,
                                },
                        } = *e
                        {
                            let behaviour = self.swarm.behaviour_mut();

                            // into hashset to delete duplicates (I checked: there are duplicates)
                            for addr in listen_addrs.iter().collect::<HashSet<_>>() {
                                behaviour.dht.add_address(&peer_id, addr.clone());
                            }
                        }
                        None
                    }
                    NetworkEventInternal::GossipEvent(e) => match *e {
                        GossipEvent::Message {
                            propagation_source: _peer_id,
                            message_id: _id,
                            message,
                        } => Some(NetworkEvent::GossipMsg(message.data)),
                        GossipEvent::Subscribed { peer_id, topic } => {
                            debug!("Peer {:?} subscribed to topic {:?}", peer_id, topic);
                            None
                        }
                        GossipEvent::Unsubscribed { peer_id, topic } => {
                            debug!("Peer {:?} unsubscribed from topic {:?}", peer_id, topic);
                            None
                        }
                        GossipEvent::GossipsubNotSupported { peer_id } => {
                            warn!("Peer {:?} does not support gossipsub", peer_id);
                            None
                        }
                    },
                    NetworkEventInternal::DMEvent(e) => self
                        .direct_message_state
                        .handle_dm_event(e, self.resend_tx.clone()),
                    NetworkEventInternal::RequestResponseEvent(e) => {
                        self.request_response_state.handle_request_response(e)
                    }
                    NetworkEventInternal::AutonatEvent(e) => {
                        match e {
                            autonat::Event::InboundProbe(_) => {}
                            autonat::Event::OutboundProbe(e) => match e {
                                autonat::OutboundProbeEvent::Request { .. }
                                | autonat::OutboundProbeEvent::Response { .. } => {}
                                autonat::OutboundProbeEvent::Error {
                                    probe_id: _,
                                    peer,
                                    error,
                                } => {
                                    warn!(
                                        "AutoNAT Probe failed to peer {:?} with error: {:?}",
                                        peer, error
                                    );
                                }
                            },
                            autonat::Event::StatusChanged { old, new } => {
                                debug!("AutoNAT Status changed. Old: {:?}, New: {:?}", old, new);
                            }
                        };
                        None
                    }
                };

                if let Some(event) = maybe_event {
                    // forward messages directly to Client
                    send_to_client
                        .send(event)
                        .await
                        .map_err(|_e| NetworkError::StreamClosed)?;
                }
            }
            SwarmEvent::OutgoingConnectionError {
                connection_id: _,
                peer_id,
                error,
            } => {
                warn!("Outgoing connection error to {:?}: {:?}", peer_id, error);
            }
            SwarmEvent::IncomingConnectionError {
                connection_id: _,
                local_addr: _,
                send_back_addr: _,
                error,
            } => {
                warn!("Incoming connection error: {:?}", error);
            }
            SwarmEvent::ListenerError {
                listener_id: _,
                error,
            } => {
                warn!("Listener error: {:?}", error);
            }
            SwarmEvent::ExternalAddrConfirmed { address } => {
                let my_id = *self.swarm.local_peer_id();
                self.swarm
                    .behaviour_mut()
                    .dht
                    .add_address(&my_id, address.clone());
            }
            _ => {
                debug!("Unhandled swarm event {:?}", event);
            }
        }
        Ok(())
    }

    /// Spawn a task to listen for requests on the returned channel
    /// as well as any events produced by libp2p
    #[instrument]
    pub async fn spawn_listeners(
        mut self,
    ) -> Result<
        (
            UnboundedSender<ClientRequest>,
            UnboundedReceiver<NetworkEvent>,
        ),
        NetworkError,
    > {
        let (s_input, s_output) = unbounded::<ClientRequest>();
        let (r_input, r_output) = unbounded::<NetworkEvent>();
        let (mut bootstrap_tx, bootstrap_rx) = mpsc::channel(100);
        self.resend_tx = Some(s_input.clone());
        self.dht_handler.set_bootstrap_sender(bootstrap_tx.clone());

        DHTBootstrapTask::run(bootstrap_rx, s_input.clone());
        async_spawn(
            async move {
                let mut fuse = s_output.recv().boxed().fuse();
                loop {
                    select! {
                        event = self.swarm.next() => {
                            debug!("peerid {:?}\t\thandling maybe event {:?}", self.peer_id, event);
                            if let Some(event) = event {
                                debug!("peerid {:?}\t\thandling event {:?}", self.peer_id, event);
                                self.handle_swarm_events(event, &r_input).await?;
                            }
                        },
                        msg = fuse => {
                            debug!("peerid {:?}\t\thandling msg {:?}", self.peer_id, msg);
                            let shutdown = self.handle_client_requests(msg).await?;
                            if shutdown {
                                let _ = bootstrap_tx.send(InputEvent::ShutdownBootstrap).await;
                                break
                            }
                            fuse = s_output.recv().boxed().fuse();
                        }
                    }
                }
                Ok::<(), NetworkError>(())
            }
            .instrument(info_span!("Libp2p NetworkBehaviour Handler")),
        );
        Ok((s_input, r_output))
    }

    /// Get a reference to the network node's peer id.
    pub fn peer_id(&self) -> PeerId {
        self.peer_id
    }
}
