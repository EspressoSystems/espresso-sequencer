use std::{
    collections::{HashMap, HashSet, VecDeque},
    num::NonZeroUsize,
    task::Poll,
    time::Duration,
};

use futures::channel::oneshot::Sender;
use libp2p::{
    kad::{
        /* handler::KademliaHandlerIn, */ store::MemoryStore, BootstrapError, BootstrapOk,
        GetClosestPeersOk, GetRecordOk, GetRecordResult, Kademlia, KademliaEvent, Mode,
        ProgressStep, PutRecordResult, QueryId, QueryResult, Quorum, Record,
    },
    swarm::{NetworkBehaviour, THandlerInEvent, THandlerOutEvent, ToSwarm},
    Multiaddr,
};
use libp2p_identity::PeerId;
use tracing::{error, info, warn};

pub(crate) const NUM_REPLICATED_TO_TRUST: usize = 2;
const MAX_DHT_QUERY_SIZE: usize = 5;

use super::exponential_backoff::ExponentialBackoff;

/// Behaviour wrapping libp2p's kademlia
/// included:
/// - publishing API
/// - Request API
/// - bootstrapping into the network
/// - peer discovery
pub struct DHTBehaviour {
    /// client approval to begin bootstrap
    pub begin_bootstrap: bool,
    /// in progress queries for nearby peers
    pub in_progress_get_closest_peers: HashMap<QueryId, Sender<()>>,
    /// bootstrap nodes
    pub bootstrap_nodes: HashMap<PeerId, HashSet<Multiaddr>>,
    /// List of kademlia events
    pub event_queue: Vec<DHTEvent>,
    /// List of in-progress get requests
    in_progress_get_record_queries: HashMap<QueryId, KadGetQuery>,
    /// List of in-progress put requests
    in_progress_put_record_queries: HashMap<QueryId, KadPutQuery>,
    /// List of previously failled get requests
    queued_get_record_queries: VecDeque<KadGetQuery>,
    /// List of previously failled put requests
    queued_put_record_queries: VecDeque<KadPutQuery>,
    /// Kademlia behaviour
    pub kadem: Kademlia<MemoryStore>,
    /// State of bootstrapping
    pub bootstrap_state: Bootstrap,
    /// State of last random walk
    pub random_walk: RandomWalk,
    /// the peer id (useful only for debugging right now)
    pub peer_id: PeerId,
    /// replication factor
    pub replication_factor: NonZeroUsize,
}

/// State of bootstrapping
#[derive(Debug, Clone)]
pub struct Bootstrap {
    /// State of bootstrap
    pub state: State,
    /// Retry timeout
    pub backoff: ExponentialBackoff,
}

/// State of the periodic random walk
pub struct RandomWalk {
    /// State of random walk
    state: State,
    /// Retry timeout
    backoff: ExponentialBackoff,
}

/// State used for random walk and bootstrapping
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    /// Not in progress
    NotStarted,
    /// In progress
    Started,
    /// Sucessfully completed
    Finished,
}

/// DHT event enum
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DHTEvent {
    /// Only event tracked currently is when we successfully bootstrap into the network
    IsBootstrapped,
}

impl DHTBehaviour {
    /// Begin the bootstrap process
    pub fn begin_bootstrap(&mut self) {
        self.begin_bootstrap = true;
    }

    /// Start a query for the closest peers
    pub fn query_closest_peers(&mut self, random_peer: PeerId) {
        self.kadem.get_closest_peers(random_peer);
    }

    /// Create a new DHT behaviour
    #[must_use]
    pub fn new(
        mut kadem: Kademlia<MemoryStore>,
        pid: PeerId,
        replication_factor: NonZeroUsize,
    ) -> Self {
        // needed because otherwise we stay in client mode when testing locally
        // and don't publish keys stuff
        // e.g. dht just doesn't work. We'd need to add mdns and that doesn't seem worth it since
        // we won't have a local network
        // <https://github.com/libp2p/rust-libp2p/issues/4194>
        kadem.set_mode(Some(Mode::Server));
        Self {
            begin_bootstrap: false,
            bootstrap_nodes: HashMap::default(),
            peer_id: pid,
            event_queue: Vec::default(),
            in_progress_get_record_queries: HashMap::default(),
            in_progress_put_record_queries: HashMap::default(),
            queued_get_record_queries: VecDeque::default(),
            queued_put_record_queries: VecDeque::default(),
            kadem,
            bootstrap_state: Bootstrap {
                state: State::NotStarted,
                backoff: ExponentialBackoff::new(2, Duration::from_secs(1)),
            },
            random_walk: RandomWalk {
                state: State::NotStarted,
                // TODO jr this may be way too frequent
                backoff: ExponentialBackoff::new(2, Duration::from_secs(1)),
            },
            in_progress_get_closest_peers: HashMap::default(),
            replication_factor,
        }
    }

    /// query a peer (e.g. obtain its address if it exists)
    pub fn lookup_peer(&mut self, peer_id: PeerId, chan: Sender<()>) {
        let qid = self.kadem.get_closest_peers(peer_id);
        self.in_progress_get_closest_peers.insert(qid, chan);
    }

    /// print out the routing table to stderr
    pub fn print_routing_table(&mut self) {
        let mut err = format!("KBUCKETS: PID: {:?}, ", self.peer_id);
        let v = self.kadem.kbuckets().collect::<Vec<_>>();
        for i in v {
            for j in i.iter() {
                let s = format!(
                    "node: key: {:?}, val {:?}, status: {:?}",
                    j.node.key, j.node.value, j.status
                );
                err.push_str(&s);
            }
        }
        error!("{:?}", err);
    }

    /// Passthru to kademlia
    /// Associate address with kademlia peer
    pub fn add_address(&mut self, peer_id: &PeerId, addr: Multiaddr) {
        // add address to kademlia
        self.kadem.add_address(peer_id, addr);
    }

    /// Save in case kademlia forgets about bootstrap nodes
    pub fn add_bootstrap_nodes(&mut self, nodes: HashMap<PeerId, HashSet<Multiaddr>>) {
        for (k, v) in nodes {
            self.bootstrap_nodes.insert(k, v);
        }
    }

    /// Publish a key/value to the kv store.
    /// Once replicated upon all nodes, the caller is notified over
    /// `chan`. If there is an error, a [`crate::network::error::DHTError`] is
    /// sent instead.
    pub fn put_record(&mut self, mut query: KadPutQuery) {
        let record = Record::new(query.key.clone(), query.value.clone());

        match self
            .kadem
            .put_record(record, Quorum::N(self.replication_factor))
        {
            Err(e) => {
                // failed try again later
                query.progress = DHTProgress::NotStarted;
                query.backoff.start_next(false);
                error!("Error publishing to DHT: {e:?} for peer {:?}", self.peer_id);
                self.queued_put_record_queries.push_back(query);
            }
            Ok(qid) => {
                info!("Success publishing {:?} to DHT", qid);
                let query = KadPutQuery {
                    progress: DHTProgress::InProgress(qid),
                    ..query
                };
                self.in_progress_put_record_queries.insert(qid, query);
            }
        }
    }

    /// Retrieve a value for a key from the DHT.
    /// Value (serialized) is sent over `chan`, and if a value is not found,
    /// a [`crate::network::error::DHTError`] is sent instead.
    /// NOTE: noop if `retry_count` is 0
    pub fn get_record(
        &mut self,
        key: Vec<u8>,
        chan: Sender<Vec<u8>>,
        factor: NonZeroUsize,
        backoff: ExponentialBackoff,
        retry_count: u8,
    ) {
        // noop
        if retry_count == 0 {
            return;
        }

        let qid = self.kadem.get_record(key.clone().into());
        let query = KadGetQuery {
            backoff,
            progress: DHTProgress::InProgress(qid),
            notify: chan,
            num_replicas: factor,
            key,
            retry_count: retry_count - 1,
            records: HashMap::default(),
        };
        self.in_progress_get_record_queries.insert(qid, query);
    }

    /// update state based on recv-ed get query
    fn handle_get_query(&mut self, record_results: GetRecordResult, id: QueryId, last: bool) {
        if let Some(query) = self.in_progress_get_record_queries.get_mut(&id) {
            if let Ok(GetRecordOk::FoundRecord(record)) = record_results {
                match query.records.entry(record.record.value) {
                    std::collections::hash_map::Entry::Occupied(mut o) => {
                        let num_entries = o.get_mut();
                        *num_entries += 1;
                    }
                    std::collections::hash_map::Entry::Vacant(v) => {
                        v.insert(1);
                    }
                }
            }
        } else {
            // inactive entry
            return;
        }

        if last {
            if let Some(KadGetQuery {
                backoff,
                progress,
                notify,
                num_replicas,
                key,
                retry_count,
                records,
            }) = self.in_progress_get_record_queries.remove(&id)
            {
                // if channel has been dropped, cancel request
                if notify.is_canceled() {
                    return;
                }

                let records_len = records.iter().fold(0, |acc, (_k, v)| acc + v);

                // NOTE case where multiple nodes agree on different
                // values is not handles
                if let Some((r, _)) = records
                    .into_iter()
                    .find(|(_, v)| *v >= NUM_REPLICATED_TO_TRUST)
                {
                    if notify.send(r).is_err() {
                        warn!("Get DHT: channel closed before get record request result could be sent");
                    }
                }
                // lack of replication => error
                else if records_len < NUM_REPLICATED_TO_TRUST {
                    warn!("Get DHT: Record not replicated enough for {:?}! requerying with more nodes", progress);
                    self.get_record(key, notify, num_replicas, backoff, retry_count);
                }
                // many records that don't match => disagreement
                else if records_len > MAX_DHT_QUERY_SIZE {
                    warn!(
                        "Get DHT: Record disagreed upon; {:?}! requerying with more nodes",
                        progress
                    );
                    self.get_record(key, notify, num_replicas, backoff, retry_count);
                }
                // disagreement => query more nodes
                else {
                    // there is some internal disagreement.
                    // Initiate new query that hits more replicas
                    let new_factor =
                        NonZeroUsize::new(num_replicas.get() + 1).unwrap_or(num_replicas);

                    self.get_record(key, notify, new_factor, backoff, retry_count);
                    warn!("Get DHT: Internal disagreement for get dht request {:?}! requerying with more nodes", progress);
                }
            }
        }
    }

    /// Update state based on put query
    fn handle_put_query(&mut self, record_results: PutRecordResult, id: QueryId) {
        if let Some(mut query) = self.in_progress_put_record_queries.remove(&id) {
            // dropped so we handle further
            if query.notify.is_canceled() {
                return;
            }

            match record_results {
                Ok(_) => {
                    if query.notify.send(()).is_err() {
                        warn!("Put DHT: client channel closed before put record request could be sent");
                    }
                }
                Err(e) => {
                    query.progress = DHTProgress::NotStarted;
                    query.backoff.start_next(false);

                    warn!(
                        "Put DHT: error performing put: {:?}. Retrying on pid {:?}.",
                        e, self.peer_id
                    );
                    // push back onto the queue
                    self.queued_put_record_queries.push_back(query);
                }
            }
        } else {
            warn!("Put DHT: completed DHT query that is no longer tracked.");
        }
    }
}

impl DHTBehaviour {
    #![allow(clippy::too_many_lines)]
    fn dht_handle_event(&mut self, event: KademliaEvent) {
        match event {
            KademliaEvent::OutboundQueryProgressed {
                result: QueryResult::PutRecord(record_results),
                id,
                step: ProgressStep { last: true, .. },
                ..
            } => {
                self.handle_put_query(record_results, id);
            }
            KademliaEvent::OutboundQueryProgressed {
                result: QueryResult::GetClosestPeers(r),
                id: query_id,
                stats,
                step: ProgressStep { last: true, .. },
                ..
            } => match r {
                Ok(GetClosestPeersOk { key, peers }) => {
                    if let Some(chan) = self.in_progress_get_closest_peers.remove(&query_id) {
                        if chan.send(()).is_err() {
                            warn!("DHT: finished query but client no longer interested");
                        };
                    } else {
                        self.random_walk.state = State::NotStarted;
                        self.random_walk.backoff.start_next(true);
                    }
                    info!(
                        "peer {:?} successfully completed get closest peers for {:?} with peers {:?}",
                        self.peer_id, key, peers
                    );
                }
                Err(e) => {
                    if let Some(chan) = self.in_progress_get_closest_peers.remove(&query_id) {
                        let _: Result<_, _> = chan.send(());
                    } else {
                        self.random_walk.state = State::NotStarted;
                        self.random_walk.backoff.start_next(true);
                    }
                    warn!(
                        "peer {:?} failed to get closest peers with {:?} and stats {:?}",
                        self.peer_id, e, stats
                    );
                }
            },
            KademliaEvent::OutboundQueryProgressed {
                result: QueryResult::GetRecord(record_results),
                id,
                step: ProgressStep { last, .. },
                ..
            } => {
                self.handle_get_query(record_results, id, last);
            }
            KademliaEvent::OutboundQueryProgressed {
                result:
                    QueryResult::Bootstrap(Ok(BootstrapOk {
                        peer: _,
                        num_remaining,
                    })),
                step: ProgressStep { last: true, .. },
                ..
            } => {
                if num_remaining == 0 {
                    // if bootstrap is successful, restart.
                    info!("Finished bootstrap for peer {:?}", self.peer_id);
                    self.bootstrap_state.state = State::Finished;
                    self.event_queue.push(DHTEvent::IsBootstrapped);
                    self.begin_bootstrap = false;
                } else {
                    warn!(
                        "Bootstrap in progress: num remaining nodes to ping {:?}",
                        num_remaining
                    );
                }
            }
            KademliaEvent::OutboundQueryProgressed {
                result: QueryResult::Bootstrap(Err(e)),
                ..
            } => {
                warn!("DHT: Bootstrap attempt failed. Retrying shortly.");
                let BootstrapError::Timeout { num_remaining, .. } = e;
                if num_remaining.is_none() {
                    error!(
                        "Peer {:?} failed bootstrap with error {:?}. This should not happen and means all bootstrap nodes are down or were evicted from our local DHT. Readding bootstrap nodes {:?}",
                        self.peer_id, e, self.bootstrap_nodes
                    );
                    for (peer, addrs) in self.bootstrap_nodes.clone() {
                        for addr in addrs {
                            self.kadem.add_address(&peer, addr);
                        }
                    }
                }
                self.bootstrap_state.state = State::NotStarted;
                self.bootstrap_state.backoff.start_next(true);
            }
            KademliaEvent::RoutablePeer { peer, address: _ } => {
                info!("on peer {:?} found routable peer {:?}", self.peer_id, peer);
            }
            KademliaEvent::PendingRoutablePeer { peer, address: _ } => {
                info!(
                    "on peer {:?} have pending routable peer {:?}",
                    self.peer_id, peer
                );
            }
            KademliaEvent::UnroutablePeer { peer } => {
                info!("on peer {:?} have unroutable peer {:?}", self.peer_id, peer);
            }
            KademliaEvent::InboundRequest { request: _r } => {}
            KademliaEvent::RoutingUpdated {
                peer: _,
                is_new_peer: _,
                addresses: _,
                bucket_range: _,
                old_peer: _,
            } => {}
            e @ KademliaEvent::OutboundQueryProgressed { .. } => {
                info!("Not handling dht event {:?}", e);
            }
        }
    }
}

/// Metadata holder for get query
#[derive(Debug)]
pub(crate) struct KadGetQuery {
    /// Exponential retry backoff
    pub(crate) backoff: ExponentialBackoff,
    /// progress through DHT query
    pub(crate) progress: DHTProgress,
    /// notify client of result
    pub(crate) notify: Sender<Vec<u8>>,
    /// number of replicas required to replicate over
    pub(crate) num_replicas: NonZeroUsize,
    /// the key to look up
    pub(crate) key: Vec<u8>,
    /// the number of remaining retries before giving up
    pub(crate) retry_count: u8,
    /// already received records
    pub(crate) records: HashMap<Vec<u8>, usize>,
}

/// Metadata holder for get query
#[derive(Debug)]
pub struct KadPutQuery {
    /// Exponential retry backoff
    pub(crate) backoff: ExponentialBackoff,
    /// progress through DHT query
    pub(crate) progress: DHTProgress,
    /// notify client of result
    pub(crate) notify: Sender<()>,
    /// the key to put
    pub(crate) key: Vec<u8>,
    /// the value to put
    pub(crate) value: Vec<u8>,
}

/// represents progress through DHT
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum DHTProgress {
    /// The query has been started
    InProgress(QueryId),
    /// The query has not been started
    NotStarted,
}

// Diagnostics:
// 1. use of deprecated associated function `libp2p::libp2p_swarm::NetworkBehaviour::inject_event`: Implement `NetworkBehaviour::on_connection_handler_event` instead. The default implementation of this `inject_*` method delegates to it.

impl NetworkBehaviour for DHTBehaviour {
    type ConnectionHandler = <Kademlia<MemoryStore> as NetworkBehaviour>::ConnectionHandler;

    type ToSwarm = DHTEvent;

    // fn new_handler(&mut self) -> Self::ConnectionHandler {
    //     self.kadem.new_handler()
    // }

    fn poll(
        &mut self,
        cx: &mut std::task::Context<'_>,
        params: &mut impl libp2p::swarm::PollParameters,
    ) -> Poll<ToSwarm<DHTEvent, THandlerInEvent<Self>>> {
        if matches!(self.bootstrap_state.state, State::NotStarted)
            && self.bootstrap_state.backoff.is_expired()
            && self.begin_bootstrap
        {
            match self.kadem.bootstrap() {
                Ok(_) => {
                    info!("started bootstrap for peer {:?}", self.peer_id);
                    self.bootstrap_state.state = State::Started;
                }
                Err(e) => {
                    error!(
                        "peer id {:?} FAILED TO START BOOTSTRAP {:?} adding peers {:?}",
                        self.peer_id, e, self.bootstrap_nodes
                    );
                    for (peer, addrs) in self.bootstrap_nodes.clone() {
                        for addr in addrs {
                            self.kadem.add_address(&peer, addr);
                        }
                    }
                }
            }
        }

        if matches!(self.random_walk.state, State::NotStarted)
            && self.random_walk.backoff.is_expired()
            && matches!(self.bootstrap_state.state, State::Finished)
        {
            self.kadem.get_closest_peers(PeerId::random());
            self.random_walk.state = State::Started;
        }

        // retry put/gets if they are ready
        while let Some(req) = self.queued_get_record_queries.pop_front() {
            if req.backoff.is_expired() {
                self.get_record(
                    req.key,
                    req.notify,
                    req.num_replicas,
                    req.backoff,
                    req.retry_count,
                );
            } else {
                self.queued_get_record_queries.push_back(req);
            }
        }
        while let Some(req) = self.queued_put_record_queries.pop_front() {
            if req.backoff.is_expired() {
                self.put_record(req);
            } else {
                self.queued_put_record_queries.push_back(req);
            }
        }

        // poll behaviour which is a passthrough and call inject event
        while let Poll::Ready(ready) = NetworkBehaviour::poll(&mut self.kadem, cx, params) {
            match ready {
                ToSwarm::GenerateEvent(e) => {
                    self.dht_handle_event(e);
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
        if !self.event_queue.is_empty() {
            return Poll::Ready(ToSwarm::GenerateEvent(self.event_queue.remove(0)));
        }
        Poll::Pending
    }

    fn on_swarm_event(
        &mut self,
        event: libp2p::swarm::derive_prelude::FromSwarm<'_, Self::ConnectionHandler>,
    ) {
        self.kadem.on_swarm_event(event);
    }

    fn on_connection_handler_event(
        &mut self,
        peer_id: PeerId,
        connection_id: libp2p::swarm::derive_prelude::ConnectionId,
        event: THandlerOutEvent<Self>,
    ) {
        self.kadem
            .on_connection_handler_event(peer_id, connection_id, event);
    }

    fn handle_pending_inbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> Result<(), libp2p::swarm::ConnectionDenied> {
        self.kadem
            .handle_pending_inbound_connection(connection_id, local_addr, remote_addr)
    }

    fn handle_established_inbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        self.kadem.handle_established_inbound_connection(
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
        self.kadem.handle_pending_outbound_connection(
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
        self.kadem
            .handle_established_outbound_connection(connection_id, peer, addr, role_override)
    }
}
