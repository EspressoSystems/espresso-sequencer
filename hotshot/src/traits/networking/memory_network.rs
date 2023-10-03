//! In memory network simulator
//!
//! This module provides an in-memory only simulation of an actual network, useful for unit and
//! integration tests.

use super::{FailedToSerializeSnafu, NetworkError, NetworkReliability, NetworkingMetricsValue};
use crate::NodeImplementation;
use async_compatibility_layer::{
    art::{async_sleep, async_spawn},
    channel::{bounded, Receiver, SendError, Sender},
};
use async_lock::{Mutex, RwLock};
use async_trait::async_trait;
use bincode::Options;
use dashmap::DashMap;
use futures::StreamExt;
use hotshot_task::{boxed_sync, BoxSyncFuture};
use hotshot_types::{
    message::{Message, MessageKind},
    traits::{
        election::Membership,
        network::{
            CommunicationChannel, ConnectedNetwork, NetworkMsg, TestableChannelImplementation,
            TestableNetworkingImplementation, TransmitType, ViewMessage,
        },
        node_implementation::NodeType,
        signature_key::SignatureKey,
    },
};
use hotshot_utils::bincode::bincode_opts;
use rand::Rng;
use snafu::ResultExt;
use std::{
    collections::BTreeSet,
    fmt::Debug,
    marker::PhantomData,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tracing::{debug, error, info, info_span, instrument, trace, warn, Instrument};

#[derive(Debug, Clone, Copy)]
/// dummy implementation of network reliability
pub struct DummyReliability {}
impl NetworkReliability for DummyReliability {
    fn sample_keep(&self) -> bool {
        true
    }
    fn sample_delay(&self) -> std::time::Duration {
        std::time::Duration::ZERO
    }
}

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
        reliability_config: Option<Arc<dyn 'static + NetworkReliability>>,
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
                                    if let Some(r) = reliability_config.clone() {
                                        async_spawn(async move {
                                            if r.sample_keep() {
                                                let delay = r.sample_delay();
                                                if delay > std::time::Duration::ZERO {
                                                    async_sleep(delay).await;
                                                }
                                                let res = dts.send(x).await;
                                                if res.is_ok() {
                                                    trace!("Passed message to output queue");
                                                } else {
                                                    error!("Output queue receivers are shutdown");
                                                }
                                            } else {
                                                warn!("dropping packet!");
                                            }
                                        });
                                    } else {
                                        let res = dts.send(x).await;
                                        if res.is_ok() {
                                            trace!("Passed message to output queue");
                                        } else {
                                            error!("Output queue receivers are shutdown");
                                        }
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
                                    if let Some(r) = reliability_config.clone() {
                                        async_spawn(async move {
                                            if r.sample_keep() {
                                                let delay = r.sample_delay();
                                                if delay > std::time::Duration::ZERO {
                                                    async_sleep(delay).await;
                                                }
                                                let res = bts.send(x).await;
                                                if res.is_ok() {
                                                    trace!("Passed message to output queue");
                                                } else {
                                                    warn!("dropping packet!");
                                                }
                                            }
                                        });
                                    } else {
                                        let res = bts.send(x).await;
                                        if res.is_ok() {
                                            trace!("Passed message to output queue");
                                        } else {
                                            warn!("dropping packet!");
                                        }
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

impl<TYPES: NodeType, I: NodeImplementation<TYPES>>
    TestableNetworkingImplementation<TYPES, Message<TYPES, I>>
    for MemoryNetwork<Message<TYPES, I>, TYPES::SignatureKey>
{
    fn generator(
        _expected_node_count: usize,
        _num_bootstrap: usize,
        _network_id: usize,
        _da_committee_size: usize,
        _is_da: bool,
    ) -> Box<dyn Fn(u64) -> Self + 'static> {
        let master: Arc<_> = MasterMap::new();
        Box::new(move |node_id| {
            let privkey = TYPES::SignatureKey::generated_from_seed_indexed([0u8; 32], node_id).1;
            let pubkey = TYPES::SignatureKey::from_private(&privkey);
            MemoryNetwork::new(pubkey, NetworkingMetricsValue::new(), master.clone(), None)
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
        debug!(?message, "Broadcasting message");
        // Bincode the message
        let vec = bincode_opts()
            .serialize(&message)
            .context(FailedToSerializeSnafu)?;
        trace!("Message bincoded, sending");
        for node in &self.inner.master_map.map {
            let (key, node) = node.pair();
            if !recipients.contains(key) {
                continue;
            }
            trace!(?key, "Sending message to node");
            let res = node.broadcast_input(vec.clone()).await;
            match res {
                Ok(_) => {
                    self.inner.metrics.outgoing_broadcast_message_count.add(1);
                    trace!(?key, "Delivered message to remote");
                }
                Err(e) => {
                    self.inner.metrics.message_failed_to_send.add(1);
                    warn!(?e, ?key, "Error sending broadcast message to node");
                }
            }
        }
        Ok(())
    }

    #[instrument(name = "MemoryNetwork::direct_message")]
    async fn direct_message(&self, message: M, recipient: K) -> Result<(), NetworkError> {
        debug!(?message, ?recipient, "Sending direct message");
        // Bincode the message
        let vec = bincode_opts()
            .serialize(&message)
            .context(FailedToSerializeSnafu)?;
        trace!("Message bincoded, finding recipient");
        if let Some(node) = self.inner.master_map.map.get(&recipient) {
            let node = node.value();
            let res = node.direct_input(vec).await;
            match res {
                Ok(_) => {
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
            }
        };
        boxed_sync(closure)
    }
}

/// memory identity communication channel
#[derive(Clone, Debug)]
pub struct MemoryCommChannel<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    MEMBERSHIP: Membership<TYPES>,
>(
    Arc<MemoryNetwork<Message<TYPES, I>, TYPES::SignatureKey>>,
    PhantomData<(I, MEMBERSHIP)>,
);

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    MemoryCommChannel<TYPES, I, MEMBERSHIP>
{
    /// create new communication channel
    #[must_use]
    pub fn new(network: Arc<MemoryNetwork<Message<TYPES, I>, TYPES::SignatureKey>>) -> Self {
        Self(network, PhantomData)
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    TestableNetworkingImplementation<TYPES, Message<TYPES, I>>
    for MemoryCommChannel<TYPES, I, MEMBERSHIP>
where
    MessageKind<TYPES, I>: ViewMessage<TYPES>,
{
    fn generator(
        expected_node_count: usize,
        num_bootstrap: usize,
        network_id: usize,
        da_committee_size: usize,
        is_da: bool,
    ) -> Box<dyn Fn(u64) -> Self + 'static> {
        let generator = <MemoryNetwork<
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
        Some(self.0.inner.in_flight_message_count.load(Ordering::Relaxed))
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    CommunicationChannel<TYPES, Message<TYPES, I>, MEMBERSHIP>
    for MemoryCommChannel<TYPES, I, MEMBERSHIP>
where
    MessageKind<TYPES, I>: ViewMessage<TYPES>,
{
    type NETWORK = MemoryNetwork<Message<TYPES, I>, TYPES::SignatureKey>;

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
        election: &MEMBERSHIP,
    ) -> Result<(), NetworkError> {
        let recipients = <MEMBERSHIP as Membership<TYPES>>::get_committee(
            election,
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
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    TestableChannelImplementation<
        TYPES,
        Message<TYPES, I>,
        MEMBERSHIP,
        MemoryNetwork<Message<TYPES, I>, TYPES::SignatureKey>,
    > for MemoryCommChannel<TYPES, I, MEMBERSHIP>
{
    fn generate_network(
    ) -> Box<dyn Fn(Arc<MemoryNetwork<Message<TYPES, I>, TYPES::SignatureKey>>) -> Self + 'static>
    {
        Box::new(move |network| MemoryCommChannel::new(network))
    }
}

#[cfg(test)]
// panic in tests
#[allow(clippy::panic)]
mod tests {
    // use super::*;
    // use crate::{
    //     demos::vdemo::{Addition, Subtraction, VDemoBlock, VDemoState, VDemoTransaction},
    //     traits::election::static_committee::{
    //         GeneralStaticCommittee, StaticElectionConfig, StaticVoteToken,
    //     },
    // };
    //
    // use crate::traits::implementations::MemoryStorage;
    // use async_compatibility_layer::logging::setup_logging;
    // use hotshot_types::traits::election::QuorumExchange;
    // use hotshot_types::traits::node_implementation::{ChannelMaps, ValidatingExchanges};
    // use hotshot_types::{
    //     data::ViewNumber,
    //     message::{DataMessage, MessageKind, ValidatingMessage},
    //     traits::{
    //         signature_key::ed25519::{Ed25519Priv, Ed25519Pub},
    //         state::ConsensusTime,
    //     },
    //     vote::QuorumVote,
    // };
    // use hotshot_types::{
    //     data::{ValidatingLeaf, ValidatingProposal},
    //     traits::consensus_type::validating_consensus::ValidatingConsensus,
    // };
    // use serde::{Deserialize, Serialize};
    //
    // #[derive(
    //     Copy,
    //     Clone,
    //     Debug,
    //     Default,
    //     Hash,
    //     PartialEq,
    //     Eq,
    //     PartialOrd,
    //     Ord,
    //     serde::Serialize,
    //     serde::Deserialize,
    // )]
    // struct Test {}
    // #[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
    // struct TestImpl {}
    //
    // // impl NetworkMsg for Test {}
    //
    // impl NodeType for Test {
    //     // TODO (da) can this be SequencingConsensus?
    //     type ConsensusType = ValidatingConsensus;
    //     type Time = ViewNumber;
    //     type BlockType = VDemoBlock;
    //     type SignatureKey = Ed25519Pub;
    //     type VoteTokenType = StaticVoteToken<Ed25519Pub>;
    //     type Transaction = VDemoTransaction;
    //     type ElectionConfigType = StaticElectionConfig;
    //     type StateType = VDemoState;
    // }
    //
    // type TestMembership = GeneralStaticCommittee<Test, TestLeaf, Ed25519Pub>;
    // type TestNetwork = MemoryCommChannel<Test, TestImpl, TestProposal, TestVote, TestMembership>;
    //
    // impl NodeImplementation<Test> for TestImpl {
    //     type ConsensusMessage = ValidatingMessage<Test, Self>;
    //     type Exchanges = ValidatingExchanges<
    //         Test,
    //         Message<Test, Self>,
    //         QuorumExchange<
    //             Test,
    //             TestLeaf,
    //             TestProposal,
    //             TestMembership,
    //             TestNetwork,
    //             Message<Test, Self>,
    //         >,
    //         ViewSyncExchange<Test, TestProposal, TestMembership, TestNetwork, Message<Test, Self>>,
    //     >;
    //     type Leaf = TestLeaf;
    //     type Storage = MemoryStorage<Test, TestLeaf>;
    //
    //     fn new_channel_maps(
    //         start_view: ViewNumber,
    //     ) -> (ChannelMaps<Test, Self>, Option<ChannelMaps<Test, Self>>) {
    //         (ChannelMaps::new(start_view), None)
    //     }
    // }
    //
    // type TestLeaf = ValidatingLeaf<Test>;
    // type TestVote = QuorumVote<Test, TestLeaf>;
    // type TestProposal = ValidatingProposal<Test, TestLeaf>;
    //
    // /// fake Eq
    // /// we can't compare the votetokentype for equality, so we can't
    // /// derive EQ on `VoteType<TYPES>` and thereby message
    // /// we are only sending data messages, though so we compare key and
    // /// data message
    // fn fake_message_eq(message_1: Message<Test, TestImpl>, message_2: Message<Test, TestImpl>) {
    //     assert_eq!(message_1.sender, message_2.sender);
    //     if let MessageKind::Data(DataMessage::SubmitTransaction(d_1, _)) = message_1.kind {
    //         if let MessageKind::Data(DataMessage::SubmitTransaction(d_2, _)) = message_2.kind {
    //             assert_eq!(d_1, d_2);
    //         }
    //     } else {
    //         panic!("Got unexpected message type in memory test!");
    //     }
    // }
    //
    // #[instrument]
    // fn get_pubkey() -> Ed25519Pub {
    //     let priv_key = Ed25519Priv::generate();
    //     Ed25519Pub::from_private(&priv_key)
    // }
    //
    // /// create a message
    // fn gen_messages(num_messages: u64, seed: u64, pk: Ed25519Pub) -> Vec<Message<Test, TestImpl>> {
    //     let mut messages = Vec::new();
    //     for i in 0..num_messages {
    //         let message = Message {
    //             sender: pk,
    //             kind: MessageKind::Data(DataMessage::SubmitTransaction(
    //                 VDemoTransaction {
    //                     add: Addition {
    //                         account: "A".to_string(),
    //                         amount: 50 + i + seed,
    //                     },
    //                     sub: Subtraction {
    //                         account: "B".to_string(),
    //                         amount: 50 + i + seed,
    //                     },
    //                     nonce: seed + i,
    //                     padding: vec![50; 0],
    //                 },
    //                 <ViewNumber as ConsensusTime>::new(0),
    //             )),
    //             _phantom: PhantomData,
    //         };
    //         messages.push(message);
    //     }
    //     messages
    // }
    //
    // // Spawning a single MemoryNetwork should produce no errors
    // #[cfg_attr(
    //     feature = "tokio-executor",
    //     tokio::test(flavor = "multi_thread", worker_threads = 2)
    // )]
    // #[cfg_attr(feature = "async-std-executor", async_std::test)]
    // #[instrument]
    // async fn spawn_single() {
    //     setup_logging();
    //     let group: Arc<MasterMap<Message<Test, TestImpl>, <Test as NodeType>::SignatureKey>> =
    //         MasterMap::new();
    //     trace!(?group);
    //     let pub_key = get_pubkey();
    //     let _network = MemoryNetwork::new(pub_key, NoMetrics::boxed(), group, Option::None);
    // }
    //
    // // // Spawning a two MemoryNetworks and connecting them should produce no errors
    // #[cfg_attr(
    //     feature = "tokio-executor",
    //     tokio::test(flavor = "multi_thread", worker_threads = 2)
    // )]
    // #[cfg_attr(feature = "async-std-executor", async_std::test)]
    // #[instrument]
    // async fn spawn_double() {
    //     setup_logging();
    //     let group: Arc<MasterMap<Message<Test, TestImpl>, <Test as NodeType>::SignatureKey>> =
    //         MasterMap::new();
    //     trace!(?group);
    //     let pub_key_1 = get_pubkey();
    //     let _network_1 =
    //         MemoryNetwork::new(pub_key_1, NoMetrics::boxed(), group.clone(), Option::None);
    //     let pub_key_2 = get_pubkey();
    //     let _network_2 = MemoryNetwork::new(pub_key_2, NoMetrics::boxed(), group, Option::None);
    // }
    //
    // // Check to make sure direct queue works
    // #[cfg_attr(
    //     feature = "tokio-executor",
    //     tokio::test(flavor = "multi_thread", worker_threads = 2)
    // )]
    // #[cfg_attr(feature = "async-std-executor", async_std::test)]
    // #[allow(deprecated)]
    // #[instrument]
    // async fn direct_queue() {
    //     setup_logging();
    //     // Create some dummy messages
    //
    //     // Make and connect the networking instances
    //     let group: Arc<MasterMap<Message<Test, TestImpl>, <Test as NodeType>::SignatureKey>> =
    //         MasterMap::new();
    //     trace!(?group);
    //     let pub_key_1 = get_pubkey();
    //     let network1 =
    //         MemoryNetwork::new(pub_key_1, NoMetrics::boxed(), group.clone(), Option::None);
    //     let pub_key_2 = get_pubkey();
    //     let network2 = MemoryNetwork::new(pub_key_2, NoMetrics::boxed(), group, Option::None);
    //
    //     let first_messages: Vec<Message<Test, TestImpl>> = gen_messages(5, 100, pub_key_1);
    //
    //     // Test 1 -> 2
    //     // Send messages
    //     for sent_message in first_messages {
    //         network1
    //             .direct_message(sent_message.clone(), pub_key_2)
    //             .await
    //             .expect("Failed to message node");
    //         let mut recv_messages = network2
    //             .recv_msgs(TransmitType::Direct)
    //             .await
    //             .expect("Failed to receive message");
    //         let recv_message = recv_messages.pop().unwrap();
    //         assert!(recv_messages.is_empty());
    //         fake_message_eq(sent_message, recv_message);
    //     }
    //
    //     let second_messages: Vec<Message<Test, TestImpl>> = gen_messages(5, 200, pub_key_2);
    //
    //     // Test 2 -> 1
    //     // Send messages
    //     for sent_message in second_messages {
    //         network2
    //             .direct_message(sent_message.clone(), pub_key_1)
    //             .await
    //             .expect("Failed to message node");
    //         let mut recv_messages = network1
    //             .recv_msgs(TransmitType::Direct)
    //             .await
    //             .expect("Failed to receive message");
    //         let recv_message = recv_messages.pop().unwrap();
    //         assert!(recv_messages.is_empty());
    //         fake_message_eq(sent_message, recv_message);
    //     }
    // }
    //
    // // Check to make sure direct queue works
    // #[cfg_attr(
    //     feature = "tokio-executor",
    //     tokio::test(flavor = "multi_thread", worker_threads = 2)
    // )]
    // #[cfg_attr(feature = "async-std-executor", async_std::test)]
    // #[allow(deprecated)]
    // #[instrument]
    // async fn broadcast_queue() {
    //     setup_logging();
    //     // Make and connect the networking instances
    //     let group: Arc<MasterMap<Message<Test, TestImpl>, <Test as NodeType>::SignatureKey>> =
    //         MasterMap::new();
    //     trace!(?group);
    //     let pub_key_1 = get_pubkey();
    //     let network1 =
    //         MemoryNetwork::new(pub_key_1, NoMetrics::boxed(), group.clone(), Option::None);
    //     let pub_key_2 = get_pubkey();
    //     let network2 = MemoryNetwork::new(pub_key_2, NoMetrics::boxed(), group, Option::None);
    //
    //     let first_messages: Vec<Message<Test, TestImpl>> = gen_messages(5, 100, pub_key_1);
    //
    //     // Test 1 -> 2
    //     // Send messages
    //     for sent_message in first_messages {
    //         network1
    //             .broadcast_message(
    //                 sent_message.clone(),
    //                 vec![pub_key_2].into_iter().collect::<BTreeSet<_>>(),
    //             )
    //             .await
    //             .expect("Failed to message node");
    //         let mut recv_messages = network2
    //             .recv_msgs(TransmitType::Broadcast)
    //             .await
    //             .expect("Failed to receive message");
    //         let recv_message = recv_messages.pop().unwrap();
    //         assert!(recv_messages.is_empty());
    //         fake_message_eq(sent_message, recv_message);
    //     }
    //
    //     let second_messages: Vec<Message<Test, TestImpl>> = gen_messages(5, 200, pub_key_2);
    //
    //     // Test 2 -> 1
    //     // Send messages
    //     for sent_message in second_messages {
    //         network2
    //             .broadcast_message(
    //                 sent_message.clone(),
    //                 vec![pub_key_1].into_iter().collect::<BTreeSet<_>>(),
    //             )
    //             .await
    //             .expect("Failed to message node");
    //         let mut recv_messages = network1
    //             .recv_msgs(TransmitType::Broadcast)
    //             .await
    //             .expect("Failed to receive message");
    //         let recv_message = recv_messages.pop().unwrap();
    //         assert!(recv_messages.is_empty());
    //         fake_message_eq(sent_message, recv_message);
    //     }
    // }
    //
    // #[cfg_attr(
    //     feature = "tokio-executor",
    //     tokio::test(flavor = "multi_thread", worker_threads = 2)
    // )]
    // #[cfg_attr(feature = "async-std-executor", async_std::test)]
    // #[instrument]
    // #[allow(deprecated)]
    // async fn test_in_flight_message_count() {
    //     // setup_logging();
    //
    //     // let group: Arc<MasterMap<Message<Test, TestImpl>, <Test as NodeType>::SignatureKey>> =
    //     //     MasterMap::new();
    //     // trace!(?group);
    //     // let pub_key_1 = get_pubkey();
    //     // let network1 =
    //     //     MemoryNetwork::new(pub_key_1, NoMetrics::boxed(), group.clone(), Option::None);
    //     // let pub_key_2 = get_pubkey();
    //     // let network2 = MemoryNetwork::new(pub_key_2, NoMetrics::boxed(), group, Option::None);
    //
    //     // // Create some dummy messages
    //     // let messages: Vec<Message<Test, TestImpl>> = gen_messages(5, 100, pub_key_1);
    //
    //     // // assert_eq!(network1.in_flight_message_count(), Some(0));
    //     // // assert_eq!(network2.in_flight_message_count(), Some(0));
    //
    //     // for (_count, message) in messages.iter().enumerate() {
    //     //     network1
    //     //         .direct_message(message.clone(), pub_key_2)
    //     //         .await
    //     //         .unwrap();
    //     //     // network 2 has received `count` broadcast messages and `count + 1` direct messages
    //     //     // assert_eq!(network2.in_flight_message_count(), Some(count + count + 1));
    //
    //     //     // network2.broadcast_message(message.clone()).await.unwrap();
    //     //     // network 1 has received `count` broadcast messages
    //     //     // assert_eq!(network1.in_flight_message_count(), Some(count + 1));
    //
    //     //     // network 2 has received `count + 1` broadcast messages and `count + 1` direct messages
    //     //     // assert_eq!(network2.in_flight_message_count(), Some((count + 1) * 2));
    //     // }
    //
    //     // for _count in (0..messages.len()).rev() {
    //     //     network1.recv_msgs(TransmitType::Broadcast).await.unwrap();
    //     //     // assert_eq!(network1.in_flight_message_count(), Some(count));
    //
    //     //     network2.recv_msgs(TransmitType::Broadcast).await.unwrap();
    //     //     network2.recv_msgs(TransmitType::Direct).await.unwrap();
    //     //     // assert_eq!(network2.in_flight_message_count(), Some(count * 2));
    //     // }
    //
    //     // // assert_eq!(network1.in_flight_message_count(), Some(0));
    //     // // assert_eq!(network2.in_flight_message_count(), Some(0));
    // }
}
