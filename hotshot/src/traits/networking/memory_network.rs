//! In memory network simulator
//!
//! This module provides an in-memory only simulation of an actual network, useful for unit and
//! integration tests.

use super::{FailedToSerializeSnafu, NetworkError, NetworkReliability, NetworkingMetricsValue};
use async_compatibility_layer::{
    art::async_spawn,
    channel::{bounded, Receiver, SendError, Sender},
};
use async_lock::{Mutex, RwLock};
use async_trait::async_trait;
use bincode::Options;
use dashmap::DashMap;
use futures::StreamExt;
use hotshot_types::traits::network::MemoryNetworkError;
use hotshot_types::{
    boxed_sync,
    message::Message,
    traits::{
        network::{ConnectedNetwork, NetworkMsg, TestableNetworkingImplementation, TransmitType},
        node_implementation::NodeType,
        signature_key::SignatureKey,
    },
    BoxSyncFuture,
};
use hotshot_utils::bincode::bincode_opts;
use rand::Rng;
use snafu::ResultExt;
use std::{
    collections::BTreeSet,
    fmt::Debug,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tracing::{debug, error, info, info_span, instrument, trace, warn, Instrument};

/// Shared state for in-memory mock networking.
///
/// This type is responsible for keeping track of the channels to each [`MemoryNetwork`], and is
/// used to group the [`MemoryNetwork`] instances.
#[derive(custom_debug::Debug)]
pub struct MasterMap<M: NetworkMsg, K: SignatureKey> {
    /// The list of `MemoryNetwork`s
    #[debug(skip)]
    map: DashMap<K, MemoryNetwork<M, K>>,
    /// The id of this `MemoryNetwork` cluster
    id: u64,
}

impl<M: NetworkMsg, K: SignatureKey> MasterMap<M, K> {
    /// Create a new, empty, `MasterMap`
    #[must_use]
    pub fn new() -> Arc<MasterMap<M, K>> {
        Arc::new(MasterMap {
            map: DashMap::new(),
            id: rand::thread_rng().gen(),
        })
    }
}

/// Internal enum for combining streams
enum Combo<T> {
    /// Direct message
    Direct(T),
    /// Broadcast message
    Broadcast(T),
}

/// Internal state for a `MemoryNetwork` instance
#[derive(Debug)]
struct MemoryNetworkInner<M: NetworkMsg, K: SignatureKey> {
    /// Input for broadcast messages
    broadcast_input: RwLock<Option<Sender<Vec<u8>>>>,
    /// Input for direct messages
    direct_input: RwLock<Option<Sender<Vec<u8>>>>,
    /// Output for broadcast messages
    broadcast_output: Mutex<Receiver<M>>,
    /// Output for direct messages
    direct_output: Mutex<Receiver<M>>,
    /// The master map
    master_map: Arc<MasterMap<M, K>>,

    /// Count of messages that are in-flight (send but not processed yet)
    in_flight_message_count: AtomicUsize,

    /// The networking metrics we're keeping track of
    metrics: NetworkingMetricsValue,

    /// config to introduce unreliability to the network
    reliability_config: Option<Box<dyn NetworkReliability>>,
}

/// In memory only network simulator.
///
/// This provides an in memory simulation of a networking implementation, allowing nodes running on
/// the same machine to mock networking while testing other functionality.
///
/// Under the hood, this simply maintains mpmc channels to every other `MemoryNetwork` insane of the
/// same group.
#[derive(Clone)]
pub struct MemoryNetwork<M: NetworkMsg, K: SignatureKey> {
    /// The actual internal state
    inner: Arc<MemoryNetworkInner<M, K>>,
}

impl<M: NetworkMsg, K: SignatureKey> Debug for MemoryNetwork<M, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryNetwork")
            .field("inner", &"inner")
            .finish()
    }
}

impl<M: NetworkMsg, K: SignatureKey> MemoryNetwork<M, K> {
    /// Creates a new `MemoryNetwork` and hooks it up to the group through the provided `MasterMap`
    #[instrument(skip(metrics))]
    pub fn new(
        pub_key: K,
        metrics: NetworkingMetricsValue,
        master_map: Arc<MasterMap<M, K>>,
        reliability_config: Option<Box<dyn NetworkReliability>>,
    ) -> MemoryNetwork<M, K> {
        info!("Attaching new MemoryNetwork");
        let (broadcast_input, broadcast_task_recv) = bounded(128);
        let (direct_input, direct_task_recv) = bounded(128);
        let (broadcast_task_send, broadcast_output) = bounded(128);
        let (direct_task_send, direct_output) = bounded(128);
        let in_flight_message_count = AtomicUsize::new(0);
        trace!("Channels open, spawning background task");

        async_spawn(
            async move {
                debug!("Starting background task");
                // direct input is right stream
                let direct = direct_task_recv.into_stream().map(Combo::<Vec<u8>>::Direct);
                // broadcast input is left stream
                let broadcast = broadcast_task_recv
                    .into_stream()
                    .map(Combo::<Vec<u8>>::Broadcast);
                // Combine the streams
                let mut combined = futures::stream::select(direct, broadcast);
                trace!("Entering processing loop");
                while let Some(message) = combined.next().await {
                    match message {
                        Combo::Direct(vec) => {
                            trace!(?vec, "Incoming direct message");
                            // Attempt to decode message
                            let x = bincode_opts().deserialize(&vec);
                            match x {
                                Ok(x) => {
                                    let dts = direct_task_send.clone();
                                    let res = dts.send(x).await;
                                    if res.is_ok() {
                                        trace!("Passed message to output queue");
                                    } else {
                                        error!("Output queue receivers are shutdown");
                                    }
                                }
                                Err(e) => {
                                    warn!(?e, "Failed to decode incoming message, skipping");
                                }
                            }
                        }
                        Combo::Broadcast(vec) => {
                            trace!(?vec, "Incoming broadcast message");
                            // Attempt to decode message
                            let x = bincode_opts().deserialize(&vec);
                            match x {
                                Ok(x) => {
                                    let bts = broadcast_task_send.clone();
                                    let res = bts.send(x).await;
                                    if res.is_ok() {
                                        trace!("Passed message to output queue");
                                    } else {
                                        warn!("dropping packet!");
                                    }
                                }
                                Err(e) => {
                                    warn!(?e, "Failed to decode incoming message, skipping");
                                }
                            }
                        }
                    }
                }
                warn!("Stream shutdown");
            }
            .instrument(info_span!("MemoryNetwork Background task", map = ?master_map)),
        );
        trace!("Notifying other networks of the new connected peer");
        trace!("Task spawned, creating MemoryNetwork");
        let mn = MemoryNetwork {
            inner: Arc::new(MemoryNetworkInner {
                broadcast_input: RwLock::new(Some(broadcast_input)),
                direct_input: RwLock::new(Some(direct_input)),
                broadcast_output: Mutex::new(broadcast_output),
                direct_output: Mutex::new(direct_output),
                master_map: master_map.clone(),
                in_flight_message_count,
                metrics,
                reliability_config,
            }),
        };
        master_map.map.insert(pub_key, mn.clone());
        trace!("Master map updated");

        mn
    }

    /// Send a [`Vec<u8>`] message to the inner `broadcast_input`
    async fn broadcast_input(&self, message: Vec<u8>) -> Result<(), SendError<Vec<u8>>> {
        self.inner
            .in_flight_message_count
            .fetch_add(1, Ordering::Relaxed);
        let input = self.inner.broadcast_input.read().await;
        if let Some(input) = &*input {
            self.inner.metrics.outgoing_broadcast_message_count.add(1);
            input.send(message).await
        } else {
            Err(SendError(message))
        }
    }

    /// Send a [`Vec<u8>`] message to the inner `direct_input`
    async fn direct_input(&self, message: Vec<u8>) -> Result<(), SendError<Vec<u8>>> {
        self.inner
            .in_flight_message_count
            .fetch_add(1, Ordering::Relaxed);
        let input = self.inner.direct_input.read().await;
        if let Some(input) = &*input {
            self.inner.metrics.outgoing_direct_message_count.add(1);
            input.send(message).await
        } else {
            Err(SendError(message))
        }
    }
}

impl<TYPES: NodeType> TestableNetworkingImplementation<TYPES>
    for MemoryNetwork<Message<TYPES>, TYPES::SignatureKey>
{
    fn generator(
        _expected_node_count: usize,
        _num_bootstrap: usize,
        _network_id: usize,
        _da_committee_size: usize,
        _is_da: bool,
        reliability_config: Option<Box<dyn NetworkReliability>>,
    ) -> Box<dyn Fn(u64) -> (Arc<Self>, Arc<Self>) + 'static> {
        let master: Arc<_> = MasterMap::new();
        // We assign known_nodes' public key and stake value rather than read from config file since it's a test
        Box::new(move |node_id| {
            let privkey = TYPES::SignatureKey::generated_from_seed_indexed([0u8; 32], node_id).1;
            let pubkey = TYPES::SignatureKey::from_private(&privkey);
            let net = MemoryNetwork::new(
                pubkey,
                NetworkingMetricsValue::default(),
                master.clone(),
                reliability_config.clone(),
            );
            (net.clone().into(), net.into())
        })
    }

    fn in_flight_message_count(&self) -> Option<usize> {
        Some(self.inner.in_flight_message_count.load(Ordering::Relaxed))
    }
}

// TODO instrument these functions
#[async_trait]
impl<M: NetworkMsg, K: SignatureKey + 'static> ConnectedNetwork<M, K> for MemoryNetwork<M, K> {
    #[instrument(name = "MemoryNetwork::ready_blocking")]
    async fn wait_for_ready(&self) {}

    fn pause(&self) {
        unimplemented!("Pausing not implemented for the Memory network");
    }

    fn resume(&self) {
        unimplemented!("Resuming not implemented for the Memory network");
    }

    #[instrument(name = "MemoryNetwork::ready_nonblocking")]
    async fn is_ready(&self) -> bool {
        true
    }

    #[instrument(name = "MemoryNetwork::shut_down")]
    fn shut_down<'a, 'b>(&'a self) -> BoxSyncFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            *self.inner.broadcast_input.write().await = None;
            *self.inner.direct_input.write().await = None;
        };
        boxed_sync(closure)
    }

    #[instrument(name = "MemoryNetwork::broadcast_message")]
    async fn broadcast_message(
        &self,
        message: M,
        recipients: BTreeSet<K>,
    ) -> Result<(), NetworkError> {
        trace!(?message, "Broadcasting message");
        // Bincode the message
        let vec = bincode_opts()
            .serialize(&message)
            .context(FailedToSerializeSnafu)?;
        trace!("Message bincoded, sending");
        for node in &self.inner.master_map.map {
            // TODO delay/drop etc here
            let (key, node) = node.pair();
            if !recipients.contains(key) {
                continue;
            }
            trace!(?key, "Sending message to node");
            if let Some(ref config) = &self.inner.reliability_config {
                {
                    let node2 = node.clone();
                    let fut = config.chaos_send_msg(
                        vec.clone(),
                        Arc::new(move |msg: Vec<u8>| {
                            let node3 = (node2).clone();
                            boxed_sync(async move {
                                let _res = node3.broadcast_input(msg).await;
                                // NOTE we're dropping metrics here but this is only for testing
                                // purposes. I think that should be okay
                            })
                        }),
                    );
                    async_spawn(fut);
                }
            } else {
                let res = node.broadcast_input(vec.clone()).await;
                match res {
                    Ok(()) => {
                        self.inner.metrics.outgoing_broadcast_message_count.add(1);
                        trace!(?key, "Delivered message to remote");
                    }
                    Err(e) => {
                        self.inner.metrics.message_failed_to_send.add(1);
                        warn!(?e, ?key, "Error sending broadcast message to node");
                    }
                }
            }
        }
        Ok(())
    }

    #[instrument(name = "MemoryNetwork::da_broadcast_message")]
    async fn da_broadcast_message(
        &self,
        message: M,
        recipients: BTreeSet<K>,
    ) -> Result<(), NetworkError> {
        self.broadcast_message(message, recipients).await
    }

    #[instrument(name = "MemoryNetwork::direct_message")]
    async fn direct_message(&self, message: M, recipient: K) -> Result<(), NetworkError> {
        // debug!(?message, ?recipient, "Sending direct message");
        // Bincode the message
        let vec = bincode_opts()
            .serialize(&message)
            .context(FailedToSerializeSnafu)?;
        trace!("Message bincoded, finding recipient");
        if let Some(node) = self.inner.master_map.map.get(&recipient) {
            let node = node.value().clone();
            if let Some(ref config) = &self.inner.reliability_config {
                {
                    let fut = config.chaos_send_msg(
                        vec.clone(),
                        Arc::new(move |msg: Vec<u8>| {
                            let node2 = node.clone();
                            boxed_sync(async move {
                                let _res = node2.direct_input(msg).await;
                                // NOTE we're dropping metrics here but this is only for testing
                                // purposes. I think that should be okay
                            })
                        }),
                    );
                    async_spawn(fut);
                }
                Ok(())
            } else {
                let res = node.direct_input(vec).await;
                match res {
                    Ok(()) => {
                        self.inner.metrics.outgoing_direct_message_count.add(1);
                        trace!(?recipient, "Delivered message to remote");
                        Ok(())
                    }
                    Err(e) => {
                        self.inner.metrics.message_failed_to_send.add(1);
                        warn!(?e, ?recipient, "Error delivering direct message");
                        Err(NetworkError::CouldNotDeliver)
                    }
                }
            }
        } else {
            self.inner.metrics.message_failed_to_send.add(1);
            warn!(
                "{:#?} {:#?} {:#?}",
                recipient, self.inner.master_map.map, "Node does not exist in map"
            );
            Err(NetworkError::NoSuchNode)
        }
    }

    #[instrument(name = "MemoryNetwork::recv_msgs", skip_all)]
    fn recv_msgs<'a, 'b>(
        &'a self,
        transmit_type: TransmitType,
    ) -> BoxSyncFuture<'b, Result<Vec<M>, NetworkError>>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            match transmit_type {
                TransmitType::Direct => {
                    let ret = self
                        .inner
                        .direct_output
                        .lock()
                        .await
                        .drain_at_least_one()
                        .await
                        .map_err(|_x| NetworkError::ShutDown)?;
                    self.inner
                        .in_flight_message_count
                        .fetch_sub(ret.len(), Ordering::Relaxed);
                    self.inner
                        .metrics
                        .incoming_direct_message_count
                        .add(ret.len());
                    Ok(ret)
                }
                TransmitType::Broadcast => {
                    let ret = self
                        .inner
                        .broadcast_output
                        .lock()
                        .await
                        .drain_at_least_one()
                        .await
                        .map_err(|_x| NetworkError::ShutDown)?;
                    self.inner
                        .in_flight_message_count
                        .fetch_sub(ret.len(), Ordering::Relaxed);
                    self.inner
                        .metrics
                        .incoming_broadcast_message_count
                        .add(ret.len());
                    Ok(ret)
                }
                TransmitType::DACommitteeBroadcast => {
                    error!("Received DACommitteeBroadcast, it should have not happened.");
                    Err(NetworkError::MemoryNetwork {
                        source: MemoryNetworkError::Stub,
                    })
                }
            }
        };
        boxed_sync(closure)
    }
}
