#[cfg(feature = "webui")]
pub mod web;

#[cfg(all(feature = "lossy_network", target_os = "linux"))]
pub mod lossy_network;

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
#[cfg(async_executor_impl = "async-std")]
use async_std::prelude::StreamExt;
#[cfg(async_executor_impl = "tokio")]
use tokio_stream::StreamExt;
#[cfg(not(any(async_executor_impl = "async-std", async_executor_impl = "tokio")))]
compile_error! {"Either config option \"async-std\" or \"tokio\" must be enabled for this crate."}

use async_compatibility_layer::{
    art::{async_sleep, async_spawn},
    channel::oneshot,
};
use clap::{Args, Parser};
use libp2p::{multiaddr, request_response::ResponseChannel, Multiaddr};
use libp2p_identity::PeerId;
use libp2p_networking::network::{
    deserialize_msg,
    network_node_handle_error::NodeConfigSnafu, spin_up_swarm, NetworkEvent,
    NetworkNodeConfigBuilder, NetworkNodeHandle, NetworkNodeHandleError, NetworkNodeType,
};
use rand::{
    distributions::Bernoulli, prelude::Distribution, seq::IteratorRandom, thread_rng, RngCore,
};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tracing::{debug, error, info, instrument, warn};

#[cfg(feature = "webui")]
use std::net::SocketAddr;

// number of success responses we need in order
// to increment the round number.
const SUCCESS_NUMBER: usize = 15;

/// probability numerator that recv-er node sends back timing stats
const SEND_NUMERATOR: u32 = 40;
/// probaiblity denominator that recv-er node sends back timing states
const SEND_DENOMINATOR: u32 = 100;

/// the timeout before ending rounding
const TIMEOUT: Duration = Duration::from_secs(500);

/// timeout before failing to broadcast
const BROADCAST_TIMEOUT: Duration = Duration::from_secs(10);

// we want message size of 32kb
// so we pad with a randomly generated number
// in order to do this use:
// 8 bytes per u64
// 32kb = 32000 bytes
// so 32000/8 usizes
const PADDING_SIZE: usize = 32000 / 8;

pub type CounterState = Epoch;
pub type Epoch = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpochType {
    BroadcastViaGossip,
    BroadcastViaDM,
    DMViaDM,
}

#[derive(Debug, Clone)]
pub struct ConductorState {
    ready_set: HashSet<PeerId>,
    current_epoch: EpochData,
    previous_epochs: HashMap<Epoch, EpochData>,
}

#[derive(Debug, Clone)]
pub struct EpochData {
    epoch_idx: Epoch,
    epoch_type: EpochType,
    node_states: HashMap<PeerId, CounterState>,
    message_durations: Vec<Duration>,
}

impl ConductorState {
    /// returns time per data
    pub fn aggregate_epochs(&self, num_nodes: usize) -> (Duration, usize) {
        let tmp_entry = NormalMessage {
            req: CounterRequest::StateRequest,
            relay_to_conductor: false,
            sent_ts: SystemTime::now(),
            epoch: 0,
            padding: vec![0; PADDING_SIZE],
        };
        let data_size = std::mem::size_of_val(&tmp_entry.req)
            + std::mem::size_of_val(&tmp_entry.relay_to_conductor)
            + std::mem::size_of_val(&tmp_entry.sent_ts)
            + std::mem::size_of_val(&tmp_entry.epoch)
            + PADDING_SIZE * 8;

        let mut total_time = Duration::ZERO;
        let mut total_data = 0;
        for epoch_data in self.previous_epochs.values() {
            if epoch_data.message_durations.iter().len() != num_nodes {
                error!(
                    "didn't match! expected {} got {} ",
                    num_nodes,
                    epoch_data.message_durations.iter().len()
                );
            }
            if let Some(max_prop_time) = epoch_data.message_durations.iter().max() {
                info!("data size is {}", data_size);
                total_time += *max_prop_time;
                total_data += data_size;
            } else {
                error!("No timing data available for this round!");
            }
        }
        (total_time, total_data)
    }
}

impl EpochData {
    pub fn increment_epoch(&mut self) {
        self.epoch_idx += 1;
    }
}

impl Default for ConductorState {
    fn default() -> Self {
        Self {
            ready_set: Default::default(),
            current_epoch: EpochData {
                epoch_idx: 0,
                epoch_type: EpochType::BroadcastViaGossip,
                node_states: Default::default(),
                message_durations: Default::default(),
            },
            previous_epochs: Default::default(),
        }
    }
}

impl ConductorState {
    /// Increment conductor to the next epoch
    pub fn complete_round(&mut self, next_epoch_type: EpochType) {
        let current_epoch = self.current_epoch.clone();
        self.previous_epochs
            .insert(current_epoch.epoch_idx, current_epoch);
        self.current_epoch.epoch_type = next_epoch_type;
        self.current_epoch.message_durations = Default::default();
        self.current_epoch.increment_epoch();
    }
}

#[cfg(feature = "webui")]
impl web::WebInfo for ConductorState {
    type Serialized = serde_json::Value;

    fn get_serializable(&self) -> Self::Serialized {
        let mut map = serde_json::map::Map::new();
        for (peer, state) in self.current_epoch.node_states.iter() {
            map.insert(peer.to_base58(), (*state).into());
        }
        serde_json::Value::Object(map)
    }
}

#[cfg(feature = "webui")]
impl web::WebInfo for (CounterState, Option<PeerId>) {
    type Serialized = (u32, Option<PeerId>);
    fn get_serializable(&self) -> Self::Serialized {
        *self
    }
}

/// Normal message. Sent amongst [`NetworkNodeType::Regular`] and [`NetworkNodeType::Bootstrap`] nodes
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum CounterRequest {
    /// Request state
    StateRequest,
    /// Reply with state
    StateResponse(CounterState),
    /// kill node
    Kill,
}

/// Message sent between non-[`NetworkNodeType::Conductor`] nodes
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct NormalMessage {
    /// timestamp when message was sent
    sent_ts: SystemTime,
    /// whether or not message shuld be relayed to conductor
    relay_to_conductor: bool,
    /// the underlying request the recv-ing node should take
    req: CounterRequest,
    /// the epoch the message was sent on
    epoch: Epoch,
    /// arbitrary amount of padding to vary message length
    padding: Vec<u64>,
}

/// A message sent and recv-ed by a ['NetworkNodeType::Regular'] or ['NetworkNodeType::Bootstrap'] node
/// that is to be relayed back to a [`NetworkNodeType::Conductor`] node
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct RelayedMessage {
    /// peer
    from_peer: PeerId,
    /// time message took to propagate from sender to recv-er
    duration: Duration,
    /// the requeset being made
    req: CounterRequest,
    /// the epoch the request was made on
    epoch: Epoch,
}

/// A message sent and recv-ed by a ['NetworkNodeType::Regular'] or ['NetworkNodeType::Bootstrap'] node
/// that is to be relayed back to a [`NetworkNodeType::Conductor`] node
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ConductorMessage {
    /// the request the recv-ing node should make
    req: CounterRequest,
    state: Epoch,
    /// the type of broadcast (direct or broadcast)
    broadcast_type: ConductorMessageMethod,
}

/// overall message
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Message {
    /// message to end from a peer to a peer
    Normal(NormalMessage),
    /// messaged recved and relayed to conductor
    Relayed(RelayedMessage),
    /// conductor requests that message is sent to node
    /// that the node must send to other node(s)
    Conductor(ConductorMessage),
    // announce the conductor
    ConductorIdIs(PeerId),
    /// recv-ed the conductor id
    RecvdConductor,
    DummyRecv,
}

impl NormalMessage {
    /// convert a normal message into a message to relay to conductor
    pub fn normal_to_relayed(&self, peer_id: PeerId) -> RelayedMessage {
        let recv_ts = SystemTime::now();
        let elapsed_time = recv_ts
            .duration_since(self.sent_ts)
            .unwrap_or(Duration::MAX);
        RelayedMessage {
            from_peer: peer_id,
            duration: elapsed_time,
            req: self.req.clone(),
            epoch: self.epoch,
        }
    }
}

/// ways to send messages between nodes
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ConductorMessageMethod {
    /// broadcast message to all nodes
    Broadcast,
    /// direct message [`PeerId`]
    DirectMessage(PeerId),
}

/// handler for non-conductor nodes for normal messages
pub async fn handle_normal_msg(
    handle: Arc<NetworkNodeHandle<(CounterState, Option<PeerId>)>>,
    msg: NormalMessage,
    // in case we need to reply to direct message
    chan: Option<ResponseChannel<Vec<u8>>>,
) -> Result<(), NetworkNodeHandleError> {
    debug!("node={} handling normal msg {:?}", handle.id(), msg);
    // send reply logic
    match msg.req {
        // direct message only
        CounterRequest::StateResponse(c) => {
            handle
                .modify_state(|s| {
                    debug!(
                        "node={} performing modify_state with c={c}, s={:?}",
                        handle.id(),
                        s
                    );
                    if c >= s.0 {
                        s.0 = c
                    }
                })
                .await;
            if let Some(chan) = chan {
                handle.direct_response(chan, &Message::DummyRecv).await?;
            }
        }
        // only as a response
        CounterRequest::StateRequest => {
            if let Some(chan) = chan {
                let state = handle.state().await;
                let data = {
                    let mut rng = thread_rng();
                    vec![rng.next_u64(); PADDING_SIZE]
                };
                let response = Message::Normal(NormalMessage {
                    sent_ts: SystemTime::now(),
                    relay_to_conductor: true,
                    req: CounterRequest::StateResponse(state.0),
                    epoch: 0,
                    padding: data,
                });
                handle.direct_response(chan, &response).await?;
            } else {
                error!("Error deserializing, channel closed!");
            }
        }
        CounterRequest::Kill => {
            handle.shutdown().await?;
        }
    }
    // relay the message to conductor
    if msg.relay_to_conductor {
        info!("Recv-ed message. Deciding if should relay to conductor.");
        if let Some(conductor_id) = handle.state().await.1 {
            // do a dice roll here to decide if we want to keep the thing
            if Bernoulli::from_ratio(SEND_NUMERATOR, SEND_DENOMINATOR)
                .unwrap()
                .sample(&mut rand::thread_rng())
            {
                info!("Deciding to relay to conductor");
                let relayed_msg = Message::Relayed(msg.normal_to_relayed(handle.peer_id()));
                handle.direct_request(conductor_id, &relayed_msg).await?;
            }
        } else {
            error!("We have a message to send to the conductor, but we do not know who the conductor is!");
        }
    }
    Ok(())
}

/// event handler for events from the swarm
/// - updates state based on events received
/// - replies to direct messages
#[instrument]
pub async fn regular_handle_network_event(
    event: NetworkEvent,
    handle: Arc<NetworkNodeHandle<(CounterState, Option<PeerId>)>>,
) -> Result<(), NetworkNodeHandleError> {
    debug!("node={} handling event {:?}", handle.id(), event);

    use NetworkEvent::*;
    match event {
        IsBootstrapped => {}
        GossipMsg(m, _) | DirectResponse(m, _) => {
            if let Ok(msg) = deserialize_msg::<Message>(&m) {
                info!("regular msg recved: {:?}", msg.clone());
                match msg {
                    Message::DummyRecv => { },
                    Message::ConductorIdIs(peerid) => {
                        handle.modify_state(|s| {
                            s.1 = Some(peerid);
                        }).await;
                    }
                    Message::Normal(msg) => {
                        handle_normal_msg(handle.clone(), msg, None).await?;
                    }
                    // do nothing. We only expect to be reached out to by the conductor via
                    // direct message
                    Message::Conductor(..) /* only the conductor expects to receive a relayed message */ | Message::Relayed(..) => { }
                    // only sent to conductor node
                    Message::RecvdConductor => {
                        unreachable!();
                    }
                }
            } else {
                info!("FAILED TO PARSE GOSSIP OR DIRECT RESPONSE MESSAGE");
            }
        }
        DirectRequest(msg, _peer_id, chan) => {
            if let Ok(msg) = deserialize_msg::<Message>(&msg) {
                info!("from pid {:?} msg recved: {:?}", msg.clone(), _peer_id);
                match msg {
                    Message::DummyRecv => {
                        handle.direct_response(chan, &Message::DummyRecv).await?;
                    }
                    // this is only done via broadcast
                    Message::ConductorIdIs(_)
                        // these are only sent to the conductor
                        | Message::Relayed(_) | Message::RecvdConductor =>
                    {
                        handle.direct_response(chan, &Message::DummyRecv).await?;
                    }
                    Message::Normal(msg) => {
                        handle_normal_msg(handle.clone(), msg, Some(chan)).await?;
                    }
                    Message::Conductor(msg) => {
                        let data = {
                            let mut rng = thread_rng();
                            vec![rng.next_u64(); PADDING_SIZE]
                        };
                        let response =
                            Message::Normal(NormalMessage {
                                sent_ts: SystemTime::now(),
                                relay_to_conductor: true,
                                req: msg.req,
                                epoch: msg.state,
                                padding: data,
                        });
                        match msg.broadcast_type {
                            // if the conductor says to broadcast
                            // perform broadcast with gossip protocol
                            ConductorMessageMethod::Broadcast => {
                                handle.gossip("global".to_string(), &response).await?;
                            }
                            ConductorMessageMethod::DirectMessage(pid) => {
                                handle.direct_request(
                                    pid,
                                    &response
                                ).await?;
                            }
                        }
                        handle.direct_response(chan, &Message::DummyRecv).await?;
                    }
                }
            } else {
            }
        }
    }
    Ok(())
}

/// convert node string into multi addr
pub fn parse_node(s: &str) -> Result<Multiaddr, multiaddr::Error> {
    let mut i = s.split(':');
    let ip = i.next().ok_or(multiaddr::Error::InvalidMultiaddr)?;
    let port = i.next().ok_or(multiaddr::Error::InvalidMultiaddr)?;
    Multiaddr::from_str(&format!("/ip4/{ip}/udp/{port}/quic-v1"))
}

#[cfg(feature = "webui")]
/// This will be flattened into CliOpt
#[derive(Args, Debug)]
pub struct WebUi {
    /// Doc comment
    #[arg(long = "webui")]
    pub webui_addr: Option<SocketAddr>,
}

#[cfg(not(feature = "webui"))]
/// This will be flattened into CliOpt
#[derive(Args, Debug)]
pub struct WebUi {}

#[cfg(all(feature = "lossy_network", target_os = "linux"))]
/// This will be flattened into CliOpt
#[derive(Args, Debug)]
pub struct EnvType {
    /// Doc comment
    #[arg(long = "env")]
    pub env_type: ExecutionEnvironment,
}
#[cfg(not(all(feature = "lossy_network", target_os = "linux")))]
/// This will be flattened into CliOpt
#[derive(Args, Debug)]
pub struct EnvType {}

#[derive(Parser, Debug)]
pub struct CliOpt {
    /// list of bootstrap node addrs
    #[arg(long, value_parser = parse_node)]
    pub to_connect_addrs: Vec<Multiaddr>,
    /// total number of nodes
    #[arg(long)]
    pub num_nodes: usize,
    /// the role this node plays
    #[arg(long)]
    pub node_type: NetworkNodeType,
    /// internal interface to bind to
    #[arg(long, value_parser = parse_node)]
    pub bound_addr: Multiaddr,
    /// If this value is set, a webserver will be spawned on this address with debug info
    #[arg(long, value_parser = parse_node)]
    pub conductor_addr: Multiaddr,

    #[command(flatten)]
    pub webui_delegate: WebUi,

    #[command(flatten)]
    pub env_type_delegate: EnvType,

    /// number of rounds of gossip
    #[arg(long)]
    pub num_gossip: u32,
}

/// The execution environemnt type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg(all(feature = "lossy_network", target_os = "linux"))]
pub enum ExecutionEnvironment {
    /// execution environment is within docker
    Docker,
    /// execution environment is on metal
    Metal,
}

#[cfg(all(feature = "lossy_network", target_os = "linux"))]
impl FromStr for ExecutionEnvironment {
    type Err = String;

    fn from_str(input: &str) -> Result<ExecutionEnvironment, Self::Err> {
        match input {
            "Docker" => Ok(ExecutionEnvironment::Docker),
            "Metal" => Ok(ExecutionEnvironment::Metal),
            _ => Err(
                "Couldn't parse execution environment. Must be one of Metal, Docker".to_string(),
            ),
        }
    }
}

/// ['bootstrap_addrs`] list of bootstrap multiaddrs. Needed to bootstrap into network
/// [`num_nodes`] total number of nodes. Needed to create pruning rules
/// [`node_type`] the type of this node
/// ['bound_addr`] the address to bind to
pub async fn start_main(opts: CliOpt) -> Result<(), CounterError> {
    setup_logging();
    setup_backtrace();
    let bootstrap_nodes = opts
        .to_connect_addrs
        .iter()
        .cloned()
        .map(|a| (None, a))
        .collect::<Vec<_>>();

    match opts.node_type {
        NetworkNodeType::Conductor => {
            let config = NetworkNodeConfigBuilder::default()
                .bound_addr(Some(opts.bound_addr))
                .node_type(NetworkNodeType::Conductor)
                .build()
                .context(NodeConfigSnafu)
                .context(HandleSnafu)?;
            let handle = Arc::new(
                NetworkNodeHandle::<ConductorState>::new(config.clone(), 0)
                    .await
                    .context(HandleSnafu)?,
            );

            #[cfg(feature = "webui")]
            if let Some(addr) = opts.webui_delegate.webui_addr {
                web::spawn_server(Arc::clone(&handle), addr);
            }

            spin_up_swarm(TIMEOUT, bootstrap_nodes, config, 0, &handle)
                .await
                .context(HandleSnafu)?;
            info!("spun up!");

            let handler_fut = handle.spawn_handler(conductor_handle_network_event).await;
            info!("spawned handler");

            handle.notify_webui().await;

            let conductor_peerid = handle.peer_id();

            let (s, _r) = oneshot::<bool>();

            async_spawn({
                let handle = handle.clone();
                // the "conductor id"
                // periodically say "ignore me!"
                async move {
                    loop {
                        // must wait for the listener to start
                        let msg = Message::ConductorIdIs(conductor_peerid);
                        if let Err(e) = handle
                            .gossip("global".to_string(), &msg)
                            .await
                            .context(HandleSnafu)
                        {
                            error!("Error {:?} gossiping the conductor ID to cluster.", e);
                        }
                        async_sleep(Duration::from_secs(1)).await;
                    }
                }
            });

            // For now, just do a sleep waiting for nodes to spin up. It's easier.
            async_sleep(Duration::from_secs(10)).await;

            // kill conductor id broadcast thread
            s.send(true);

            for i in 0..opts.num_gossip {
                info!("iteration i: {}", i);
                handle
                    .modify_state(|s| s.current_epoch.epoch_type = EpochType::BroadcastViaGossip)
                    .await;
                conductor_broadcast(BROADCAST_TIMEOUT, handle.clone())
                    .await
                    .context(HandleSnafu)?;
                handle
                    .modify_state(|s| s.complete_round(EpochType::BroadcastViaGossip))
                    .await;
            }
            handler_fut.await;

            #[cfg(feature = "benchmark-output")]
            {
                trace!("result raw: {:?}", handle.state().await);
                trace!(
                    "result: {:?}",
                    handle.state().await.aggregate_epochs(opts.num_nodes)
                );
            }
        }
        // regular and bootstrap nodes
        NetworkNodeType::Regular | NetworkNodeType::Bootstrap => {
            let config = NetworkNodeConfigBuilder::default()
                .bound_addr(Some(opts.bound_addr))
                .node_type(opts.node_type)
                .build()
                .context(NodeConfigSnafu)
                .context(HandleSnafu)?;

            let node = NetworkNodeHandle::<(CounterState, Option<PeerId>)>::new(config.clone(), 0)
                .await
                .context(HandleSnafu)?;

            let handle = Arc::new(node);
            #[cfg(feature = "webui")]
            if let Some(addr) = opts.webui_delegate.webui_addr {
                web::spawn_server(Arc::clone(&handle), addr);
            }

            spin_up_swarm(TIMEOUT, bootstrap_nodes, config, 0, &handle)
                .await
                .context(HandleSnafu)?;
            let handler_fut = handle.spawn_handler(regular_handle_network_event).await;
            handler_fut.await;
            // while !handle.is_killed().await {
            //     async_sleep(Duration::from_millis(100)).await;
            // }
        }
    }

    Ok(())
}

pub async fn conductor_broadcast(
    timeout: Duration,
    handle: Arc<NetworkNodeHandle<ConductorState>>,
) -> Result<(), NetworkNodeHandleError> {
    let new_state = handle.state().await.current_epoch.epoch_idx;
    // nOTE it's probably easier to pass in a hard coded list of PIDs
    // from test.py orchestration
    let mut connected_peers = handle.connected_pids().await.unwrap();
    connected_peers.remove(&handle.peer_id());

    let chosen_peer = *connected_peers.iter().choose(&mut thread_rng()).unwrap();

    let request = CounterRequest::StateResponse(new_state);

    // tell the "leader" to do a "broadcast" message using gosisp protocol
    let msg = Message::Conductor(ConductorMessage {
        state: new_state,
        req: request.clone(),
        broadcast_type: ConductorMessageMethod::Broadcast,
    });

    let mut res_fut = Box::pin(
        handle.state_wait_timeout_until_with_trigger(timeout, |state| {
            state
                .current_epoch
                .node_states
                .iter()
                .filter(|(_, &s)| s >= new_state)
                .count()
                >= SUCCESS_NUMBER
        }),
    );

    // wait for ready signal
    res_fut.next().await.unwrap().unwrap();

    // send direct message from conductor to leader to do broadcast
    handle
        .direct_request(chosen_peer, &msg)
        .await
        .context(HandleSnafu)
        .unwrap();

    if res_fut.next().await.unwrap().is_err() {
        error!(
            "TIMED OUT with {} msgs recv-ed",
            handle.state().await.current_epoch.message_durations.len()
        );
    }

    Ok(())
}

/// network event handler for conductor
#[instrument]
pub async fn conductor_handle_network_event(
    event: NetworkEvent,
    handle: Arc<NetworkNodeHandle<ConductorState>>,
) -> Result<(), NetworkNodeHandleError> {
    use NetworkEvent::*;
    match event {
        IsBootstrapped => {}
        GossipMsg(_m, _t) => {
            // this node isn't going to participate in gossip/dms to update state
            // it's only purpose is to recv relayed messages
        }
        DirectRequest(m, peer_id, chan) => {
            info!("recv: {:?}", m);
            async_spawn({
                let handle = handle.clone();
                async move {
                    handle.direct_response(chan, &Message::DummyRecv).await?;
                    Result::<(), NetworkNodeHandleError>::Ok(())
                }
            });
            info!("finished spawning now deserializing");
            if let Ok(msg) = deserialize_msg::<Message>(&m) {
                info!("desrialized MESSAGE IS {:?}", msg);
                match msg {
                    Message::Relayed(msg) => {
                        match handle.state().await.current_epoch.epoch_type {
                            EpochType::BroadcastViaGossip => {
                                if let CounterRequest::StateResponse(..) = msg.req {
                                    handle
                                        .modify_state(|s| {
                                            if msg.epoch >= s.current_epoch.epoch_idx {
                                                s.current_epoch.message_durations.push(msg.duration)
                                            }

                                            if msg.epoch > s.current_epoch.epoch_idx {
                                                warn!("listening on epcoh {:?} but recv message on epoch {:?}", s.current_epoch.epoch_idx, msg.epoch);
                                            }

                                        })
                                        .await;
                                    let _ = handle.prune_peer(msg.from_peer).await;
                                }
                            }
                            EpochType::DMViaDM => {
                                info!("modifying state DM VIA DM {:?}", msg);
                                // NOTE maybe should check epoch
                                if let CounterRequest::StateRequest = msg.req {
                                    handle
                                        .modify_state(|s| {
                                            s.current_epoch.message_durations.push(msg.duration);
                                        })
                                        .await;
                                }
                            }
                            EpochType::BroadcastViaDM => {
                                unimplemented!("BroadcastViaDM is currently unimplemented");
                            }
                        }
                        if let CounterRequest::StateResponse(state) = msg.req {
                            handle
                                .modify_state(|s| {
                                    s.current_epoch.node_states.insert(peer_id, state);
                                })
                                .await;
                        }
                    }
                    Message::RecvdConductor => {
                        handle
                            .modify_state(|s| {
                                s.ready_set.insert(peer_id);
                            })
                            .await;
                    }
                    msg => {
                        info!("Unexpected message {:?}", msg);

                        /* Do nothing. Conductor doesn't care about these messages. */
                    }
                }
            } else {
                error!("failed to deserialize msg");
            }
        }
        DirectResponse(_m, _peer_id) => { /* nothing to do here */ }
    }
    Ok(())
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum CounterError {
    Handle { source: NetworkNodeHandleError },
    FileRead { source: std::io::Error },
    MissingBootstrap,
}
