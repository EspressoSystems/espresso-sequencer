//! In memory network simulator
//!
//! This module provides an in-memory only simulation of an actual network, useful for unit and
//! integration tests.

use core::time::Duration;
use std::{
    collections::BTreeSet,
    fmt::Debug,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use async_compatibility_layer::{
    art::async_spawn,
    channel::{bounded, BoundedStream, Receiver, SendError, Sender},
};
use async_lock::{Mutex, RwLock};
use async_trait::async_trait;
use dashmap::DashMap;
use futures::StreamExt;
use hotshot_types::{
    boxed_sync,
    traits::{
        network::{
            AsyncGenerator, BroadcastDelay, ConnectedNetwork, TestableNetworkingImplementation,
        },
        node_implementation::NodeType,
        signature_key::SignatureKey,
    },
    BoxSyncFuture,
};
use rand::Rng;
use tracing::{debug, error, info, info_span, instrument, trace, warn, Instrument};

use super::{NetworkError, NetworkReliability};

/// Shared state for in-memory mock networking.
///
/// This type is responsible for keeping track of the channels to each [`MemoryNetwork`], and is
/// used to group the [`MemoryNetwork`] instances.
#[derive(custom_debug::Debug)]
pub struct MasterMap<K: SignatureKey> {
    /// The list of `MemoryNetwork`s
    #[debug(skip)]
    map: DashMap<K, MemoryNetwork<K>>,
    /// The id of this `MemoryNetwork` cluster
    id: u64,
}

impl<K: SignatureKey> MasterMap<K> {
    /// Create a new, empty, `MasterMap`
    #[must_use]
    pub fn new() -> Arc<MasterMap<K>> {
        Arc::new(MasterMap {
            map: DashMap::new(),
            id: rand::thread_rng().gen(),
        })
    }
}

/// Internal state for a `MemoryNetwork` instance
#[derive(Debug)]
struct MemoryNetworkInner<K: SignatureKey> {
    /// Input for messages
    input: RwLock<Option<Sender<Vec<u8>>>>,
    /// Output for messages
    output: Mutex<Receiver<Vec<u8>>>,
    /// The master map
    master_map: Arc<MasterMap<K>>,

    /// Count of messages that are in-flight (send but not processed yet)
    in_flight_message_count: AtomicUsize,

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
pub struct MemoryNetwork<K: SignatureKey> {
    /// The actual internal state
    inner: Arc<MemoryNetworkInner<K>>,
}

impl<K: SignatureKey> Debug for MemoryNetwork<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryNetwork")
            .field("inner", &"inner")
            .finish()
    }
}

impl<K: SignatureKey> MemoryNetwork<K> {
    /// Creates a new `MemoryNetwork` and hooks it up to the group through the provided `MasterMap`
    pub fn new(
        pub_key: K,
        master_map: &Arc<MasterMap<K>>,
        reliability_config: Option<Box<dyn NetworkReliability>>,
    ) -> MemoryNetwork<K> {
        info!("Attaching new MemoryNetwork");
        let (input, task_recv) = bounded(128);
        let (task_send, output) = bounded(128);
        let in_flight_message_count = AtomicUsize::new(0);
        trace!("Channels open, spawning background task");

        async_spawn(
            async move {
                debug!("Starting background task");
                let mut task_stream: BoundedStream<Vec<u8>> = task_recv.into_stream();
                trace!("Entering processing loop");
                while let Some(vec) = task_stream.next().await {
                    trace!(?vec, "Incoming message");
                    // Attempt to decode message
                    let ts = task_send.clone();
                    let res = ts.send(vec).await;
                    if res.is_ok() {
                        trace!("Passed message to output queue");
                    } else {
                        error!("Output queue receivers are shutdown");
                    }
                }
            }
            .instrument(info_span!("MemoryNetwork Background task", map = ?master_map)),
        );
        trace!("Notifying other networks of the new connected peer");
        trace!("Task spawned, creating MemoryNetwork");
        let mn = MemoryNetwork {
            inner: Arc::new(MemoryNetworkInner {
                input: RwLock::new(Some(input)),
                output: Mutex::new(output),
                master_map: Arc::clone(master_map),
                in_flight_message_count,
                reliability_config,
            }),
        };
        master_map.map.insert(pub_key, mn.clone());
        trace!("Master map updated");

        mn
    }

    /// Send a [`Vec<u8>`] message to the inner `input`
    async fn input(&self, message: Vec<u8>) -> Result<(), SendError<Vec<u8>>> {
        self.inner
            .in_flight_message_count
            .fetch_add(1, Ordering::Relaxed);
        let input = self.inner.input.read().await;
        if let Some(input) = &*input {
            input.send(message).await
        } else {
            Err(SendError(message))
        }
    }
}

impl<TYPES: NodeType> TestableNetworkingImplementation<TYPES>
    for MemoryNetwork<TYPES::SignatureKey>
{
    fn generator(
        _expected_node_count: usize,
        _num_bootstrap: usize,
        _network_id: usize,
        _da_committee_size: usize,
        _is_da: bool,
        reliability_config: Option<Box<dyn NetworkReliability>>,
        _secondary_network_delay: Duration,
    ) -> AsyncGenerator<Arc<Self>> {
        let master: Arc<_> = MasterMap::new();
        // We assign known_nodes' public key and stake value rather than read from config file since it's a test
        Box::pin(move |node_id| {
            let privkey = TYPES::SignatureKey::generated_from_seed_indexed([0u8; 32], node_id).1;
            let pubkey = TYPES::SignatureKey::from_private(&privkey);
            let net = MemoryNetwork::new(pubkey, &master, reliability_config.clone());
            Box::pin(async move { net.into() })
        })
    }

    fn in_flight_message_count(&self) -> Option<usize> {
        Some(self.inner.in_flight_message_count.load(Ordering::Relaxed))
    }
}

// TODO instrument these functions
#[async_trait]
impl<K: SignatureKey + 'static> ConnectedNetwork<K> for MemoryNetwork<K> {
    #[instrument(name = "MemoryNetwork::ready_blocking")]
    async fn wait_for_ready(&self) {}

    fn pause(&self) {
        unimplemented!("Pausing not implemented for the Memory network");
    }

    fn resume(&self) {
        unimplemented!("Resuming not implemented for the Memory network");
    }

    #[instrument(name = "MemoryNetwork::shut_down")]
    fn shut_down<'a, 'b>(&'a self) -> BoxSyncFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            *self.inner.input.write().await = None;
        };
        boxed_sync(closure)
    }

    #[instrument(name = "MemoryNetwork::broadcast_message")]
    async fn broadcast_message(
        &self,
        message: Vec<u8>,
        recipients: BTreeSet<K>,
        _broadcast_delay: BroadcastDelay,
    ) -> Result<(), NetworkError> {
        trace!(?message, "Broadcasting message");
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
                        message.clone(),
                        Arc::new(move |msg: Vec<u8>| {
                            let node3 = (node2).clone();
                            boxed_sync(async move {
                                let _res = node3.input(msg).await;
                                // NOTE we're dropping metrics here but this is only for testing
                                // purposes. I think that should be okay
                            })
                        }),
                    );
                    async_spawn(fut);
                }
            } else {
                let res = node.input(message.clone()).await;
                match res {
                    Ok(()) => {
                        trace!(?key, "Delivered message to remote");
                    }
                    Err(e) => {
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
        message: Vec<u8>,
        recipients: BTreeSet<K>,
        broadcast_delay: BroadcastDelay,
    ) -> Result<(), NetworkError> {
        self.broadcast_message(message, recipients, broadcast_delay)
            .await
    }

    #[instrument(name = "MemoryNetwork::direct_message")]
    async fn direct_message(&self, message: Vec<u8>, recipient: K) -> Result<(), NetworkError> {
        // debug!(?message, ?recipient, "Sending direct message");
        // Bincode the message
        trace!("Message bincoded, finding recipient");
        if let Some(node) = self.inner.master_map.map.get(&recipient) {
            let node = node.value().clone();
            if let Some(ref config) = &self.inner.reliability_config {
                {
                    let fut = config.chaos_send_msg(
                        message.clone(),
                        Arc::new(move |msg: Vec<u8>| {
                            let node2 = node.clone();
                            boxed_sync(async move {
                                let _res = node2.input(msg).await;
                                // NOTE we're dropping metrics here but this is only for testing
                                // purposes. I think that should be okay
                            })
                        }),
                    );
                    async_spawn(fut);
                }
                Ok(())
            } else {
                let res = node.input(message).await;
                match res {
                    Ok(()) => {
                        trace!(?recipient, "Delivered message to remote");
                        Ok(())
                    }
                    Err(e) => {
                        warn!(?e, ?recipient, "Error delivering direct message");
                        Err(NetworkError::CouldNotDeliver)
                    }
                }
            }
        } else {
            warn!(
                "{:#?} {:#?} {:#?}",
                recipient, self.inner.master_map.map, "Node does not exist in map"
            );
            Err(NetworkError::NoSuchNode)
        }
    }

    /// Receive one or many messages from the underlying network.
    ///
    /// # Errors
    /// If the other side of the channel is closed
    #[instrument(name = "MemoryNetwork::recv_msgs", skip_all)]
    async fn recv_msgs(&self) -> Result<Vec<Vec<u8>>, NetworkError> {
        let ret = self
            .inner
            .output
            .lock()
            .await
            .drain_at_least_one()
            .await
            .map_err(|_x| NetworkError::ShutDown)?;
        self.inner
            .in_flight_message_count
            .fetch_sub(ret.len(), Ordering::Relaxed);
        Ok(ret)
    }
}
