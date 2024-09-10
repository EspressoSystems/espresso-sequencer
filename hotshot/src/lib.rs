// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! Provides a generic rust implementation of the `HotShot` BFT protocol
//!

// Documentation module
#[cfg(feature = "docs")]
pub mod documentation;

use futures::future::{select, Either};
use hotshot_types::{
    message::UpgradeLock,
    traits::{network::BroadcastDelay, node_implementation::Versions},
};
use rand::Rng;
use url::Url;

/// Contains traits consumed by [`SystemContext`]
pub mod traits;
/// Contains types used by the crate
pub mod types;

pub mod tasks;

use std::{
    collections::{BTreeMap, HashMap},
    num::NonZeroUsize,
    sync::Arc,
    time::Duration,
};

use async_broadcast::{broadcast, InactiveReceiver, Receiver, Sender};
use async_compatibility_layer::art::async_spawn;
use async_lock::RwLock;
use async_trait::async_trait;
use futures::join;
use hotshot_task::task::{ConsensusTaskRegistry, NetworkTaskRegistry};
use hotshot_task_impls::{events::HotShotEvent, helpers::broadcast_event};
// Internal
/// Reexport error type
pub use hotshot_types::error::HotShotError;
use hotshot_types::{
    consensus::{Consensus, ConsensusMetricsValue, OuterConsensus, View, ViewInner},
    constants::{EVENT_CHANNEL_SIZE, EXTERNAL_EVENT_CHANNEL_SIZE},
    data::{Leaf, QuorumProposal},
    event::{EventType, LeafInfo},
    message::{DataMessage, Message, MessageKind, Proposal},
    simple_certificate::QuorumCertificate,
    traits::{
        consensus_api::ConsensusApi,
        election::Membership,
        network::ConnectedNetwork,
        node_implementation::{ConsensusTime, NodeType},
        signature_key::SignatureKey,
        states::ValidatedState,
        EncodeBytes,
    },
    HotShotConfig,
};
// -- Rexports
// External
/// Reexport rand crate
pub use rand;
use tracing::{debug, instrument, trace};

use crate::{
    tasks::{add_consensus_tasks, add_network_tasks},
    traits::NodeImplementation,
    types::{Event, SystemContextHandle},
};

/// Length, in bytes, of a 512 bit hash
pub const H_512: usize = 64;
/// Length, in bytes, of a 256 bit hash
pub const H_256: usize = 32;

#[derive(Clone)]
/// Wrapper for all marketplace config that needs to be passed when creating a new instance of HotShot
pub struct MarketplaceConfig<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// auction results provider
    pub auction_results_provider: Arc<I::AuctionResultsProvider>,
    /// fallback builder
    pub fallback_builder_url: Url,
}

/// Bundle of all the memberships a consensus instance uses
#[derive(Clone)]
pub struct Memberships<TYPES: NodeType> {
    /// The entire quorum
    pub quorum_membership: TYPES::Membership,
    /// The DA nodes
    pub da_membership: TYPES::Membership,
}

/// Holds the state needed to participate in `HotShot` consensus
pub struct SystemContext<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> {
    /// The public key of this node
    public_key: TYPES::SignatureKey,

    /// The private key of this node
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,

    /// Configuration items for this hotshot instance
    pub config: HotShotConfig<TYPES::SignatureKey>,

    /// The underlying network
    pub network: Arc<I::Network>,

    /// Memberships used by consensus
    pub memberships: Arc<Memberships<TYPES>>,

    /// the metrics that the implementor is using.
    metrics: Arc<ConsensusMetricsValue>,

    /// The hotstuff implementation
    consensus: OuterConsensus<TYPES>,

    /// Immutable instance state
    instance_state: Arc<TYPES::InstanceState>,

    /// The view to enter when first starting consensus
    start_view: TYPES::Time,

    /// Access to the output event stream.
    output_event_stream: (Sender<Event<TYPES>>, InactiveReceiver<Event<TYPES>>),

    /// External event stream for communication with the application.
    pub(crate) external_event_stream: (Sender<Event<TYPES>>, InactiveReceiver<Event<TYPES>>),

    /// Anchored leaf provided by the initializer.
    anchored_leaf: Leaf<TYPES>,

    /// access to the internal event stream, in case we need to, say, shut something down
    #[allow(clippy::type_complexity)]
    internal_event_stream: (
        Sender<Arc<HotShotEvent<TYPES>>>,
        InactiveReceiver<Arc<HotShotEvent<TYPES>>>,
    ),

    /// uid for instrumentation
    pub id: u64,

    /// Reference to the internal storage for consensus datum.
    pub storage: Arc<RwLock<I::Storage>>,

    /// shared lock for upgrade information
    pub upgrade_lock: UpgradeLock<TYPES, V>,

    /// Marketplace config for this instance of HotShot
    pub marketplace_config: MarketplaceConfig<TYPES, I>,
}
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> Clone
    for SystemContext<TYPES, I, V>
{
    #![allow(deprecated)]
    fn clone(&self) -> Self {
        Self {
            public_key: self.public_key.clone(),
            private_key: self.private_key.clone(),
            config: self.config.clone(),
            network: Arc::clone(&self.network),
            memberships: Arc::clone(&self.memberships),
            metrics: Arc::clone(&self.metrics),
            consensus: self.consensus.clone(),
            instance_state: Arc::clone(&self.instance_state),
            start_view: self.start_view,
            output_event_stream: self.output_event_stream.clone(),
            external_event_stream: self.external_event_stream.clone(),
            anchored_leaf: self.anchored_leaf.clone(),
            internal_event_stream: self.internal_event_stream.clone(),
            id: self.id,
            storage: Arc::clone(&self.storage),
            upgrade_lock: self.upgrade_lock.clone(),
            marketplace_config: self.marketplace_config.clone(),
        }
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> SystemContext<TYPES, I, V> {
    #![allow(deprecated)]
    /// Creates a new [`Arc<SystemContext>`] with the given configuration options.
    ///
    /// To do a full initialization, use `fn init` instead, which will set up background tasks as
    /// well.
    ///
    /// Use this instead of `init` if you want to start the tasks manually
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        public_key: TYPES::SignatureKey,
        private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
        nonce: u64,
        config: HotShotConfig<TYPES::SignatureKey>,
        memberships: Memberships<TYPES>,
        network: Arc<I::Network>,
        initializer: HotShotInitializer<TYPES>,
        metrics: ConsensusMetricsValue,
        storage: I::Storage,
        marketplace_config: MarketplaceConfig<TYPES, I>,
    ) -> Arc<Self> {
        let interal_chan = broadcast(EVENT_CHANNEL_SIZE);
        let external_chan = broadcast(EXTERNAL_EVENT_CHANNEL_SIZE);

        Self::new_from_channels(
            public_key,
            private_key,
            nonce,
            config,
            memberships,
            network,
            initializer,
            metrics,
            storage,
            marketplace_config,
            interal_chan,
            external_chan,
        )
        .await
    }

    /// Creates a new [`Arc<SystemContext>`] with the given configuration options.
    ///
    /// To do a full initialization, use `fn init` instead, which will set up background tasks as
    /// well.
    ///
    /// Use this function if you want to use some prexisting channels and to spin up the tasks
    /// and start consensus manually.  Mostly useful for tests
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub async fn new_from_channels(
        public_key: TYPES::SignatureKey,
        private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
        nonce: u64,
        config: HotShotConfig<TYPES::SignatureKey>,
        memberships: Memberships<TYPES>,
        network: Arc<I::Network>,
        initializer: HotShotInitializer<TYPES>,
        metrics: ConsensusMetricsValue,
        storage: I::Storage,
        marketplace_config: MarketplaceConfig<TYPES, I>,
        internal_channel: (
            Sender<Arc<HotShotEvent<TYPES>>>,
            Receiver<Arc<HotShotEvent<TYPES>>>,
        ),
        external_channel: (Sender<Event<TYPES>>, Receiver<Event<TYPES>>),
    ) -> Arc<Self> {
        debug!("Creating a new hotshot");

        let consensus_metrics = Arc::new(metrics);
        let anchored_leaf = initializer.inner;
        let instance_state = initializer.instance_state;

        let (internal_tx, internal_rx) = internal_channel;
        let (mut external_tx, mut external_rx) = external_channel;

        let upgrade_lock = UpgradeLock::<TYPES, V>::new();

        // Allow overflow on the channel, otherwise sending to it may block.
        external_rx.set_overflow(true);

        // Get the validated state from the initializer or construct an incomplete one from the
        // block header.
        let validated_state = match initializer.validated_state {
            Some(state) => state,
            None => Arc::new(TYPES::ValidatedState::from_header(
                anchored_leaf.block_header(),
            )),
        };

        // Insert the validated state to state map.
        let mut validated_state_map = BTreeMap::default();
        validated_state_map.insert(
            anchored_leaf.view_number(),
            View {
                view_inner: ViewInner::Leaf {
                    leaf: anchored_leaf.commit(&upgrade_lock).await,
                    state: Arc::clone(&validated_state),
                    delta: initializer.state_delta.clone(),
                },
            },
        );
        for (view_num, inner) in initializer.undecided_state {
            validated_state_map.insert(view_num, inner);
        }

        let mut saved_leaves = HashMap::new();
        let mut saved_payloads = BTreeMap::new();
        saved_leaves.insert(
            anchored_leaf.commit(&upgrade_lock).await,
            anchored_leaf.clone(),
        );

        for leaf in initializer.undecided_leafs {
            saved_leaves.insert(leaf.commit(&upgrade_lock).await, leaf.clone());
        }
        if let Some(payload) = anchored_leaf.block_payload() {
            let encoded_txns = payload.encode();

            saved_payloads.insert(anchored_leaf.view_number(), Arc::clone(&encoded_txns));
        }

        let consensus = Consensus::new(
            validated_state_map,
            anchored_leaf.view_number(),
            anchored_leaf.view_number(),
            anchored_leaf.view_number(),
            initializer.actioned_view,
            initializer.saved_proposals,
            saved_leaves,
            saved_payloads,
            initializer.high_qc,
            Arc::clone(&consensus_metrics),
        );

        let consensus = Arc::new(RwLock::new(consensus));

        // This makes it so we won't block on broadcasting if there is not a receiver
        // Our own copy of the receiver is inactive so it doesn't count.
        external_tx.set_await_active(false);

        let inner: Arc<SystemContext<TYPES, I, V>> = Arc::new(SystemContext {
            id: nonce,
            consensus: OuterConsensus::new(consensus),
            instance_state: Arc::new(instance_state),
            public_key,
            private_key,
            config,
            start_view: initializer.start_view,
            network,
            memberships: Arc::new(memberships),
            metrics: Arc::clone(&consensus_metrics),
            internal_event_stream: (internal_tx, internal_rx.deactivate()),
            output_event_stream: (external_tx.clone(), external_rx.clone().deactivate()),
            external_event_stream: (external_tx, external_rx.deactivate()),
            anchored_leaf: anchored_leaf.clone(),
            storage: Arc::new(RwLock::new(storage)),
            upgrade_lock,
            marketplace_config,
        });

        inner
    }

    /// "Starts" consensus by sending a `QcFormed`, `ViewChange`, and `ValidatedStateUpdated` events
    ///
    /// # Panics
    /// Panics if sending genesis fails
    #[instrument(skip_all, target = "SystemContext", fields(id = self.id))]
    pub async fn start_consensus(&self) {
        #[cfg(feature = "dependency-tasks")]
        tracing::error!("HotShot is running with the dependency tasks feature enabled!!");

        #[cfg(all(feature = "rewind", not(debug_assertions)))]
        compile_error!("Cannot run rewind in production builds!");

        debug!("Starting Consensus");
        let consensus = self.consensus.read().await;

        #[allow(clippy::panic)]
        self.internal_event_stream
            .0
            .broadcast_direct(Arc::new(HotShotEvent::ViewChange(self.start_view)))
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "Genesis Broadcast failed; event = ViewChange({:?})",
                    self.start_view
                )
            });
        #[cfg(feature = "dependency-tasks")]
        {
            if let Some(validated_state) = consensus.validated_state_map().get(&self.start_view) {
                #[allow(clippy::panic)]
                self.internal_event_stream
                    .0
                    .broadcast_direct(Arc::new(HotShotEvent::ValidatedStateUpdated(
                        TYPES::Time::new(*self.start_view),
                        validated_state.clone(),
                    )))
                    .await
                    .unwrap_or_else(|_| {
                        panic!(
                            "Genesis Broadcast failed; event = ValidatedStateUpdated({:?})",
                            self.start_view,
                        )
                    });
            }
        }
        #[allow(clippy::panic)]
        self.internal_event_stream
            .0
            .broadcast_direct(Arc::new(HotShotEvent::QcFormed(either::Left(
                consensus.high_qc().clone(),
            ))))
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "Genesis Broadcast failed; event = QcFormed(either::Left({:?}))",
                    consensus.high_qc()
                )
            });

        {
            // Some applications seem to expect a leaf decide event for the genesis leaf,
            // which contains only that leaf and nothing else.
            if self.anchored_leaf.view_number() == TYPES::Time::genesis() {
                let (validated_state, state_delta) =
                    TYPES::ValidatedState::genesis(&self.instance_state);

                let qc = Arc::new(
                    QuorumCertificate::genesis::<V>(&validated_state, self.instance_state.as_ref())
                        .await,
                );

                broadcast_event(
                    Event {
                        view_number: self.anchored_leaf.view_number(),
                        event: EventType::Decide {
                            leaf_chain: Arc::new(vec![LeafInfo::new(
                                self.anchored_leaf.clone(),
                                Arc::new(validated_state),
                                Some(Arc::new(state_delta)),
                                None,
                            )]),
                            qc,
                            block_size: None,
                        },
                    },
                    &self.external_event_stream.0,
                )
                .await;
            }
        }
    }

    /// Emit an external event
    // A copypasta of `ConsensusApi::send_event`
    // TODO: remove with https://github.com/EspressoSystems/HotShot/issues/2407
    async fn send_external_event(&self, event: Event<TYPES>) {
        debug!(?event, "send_external_event");
        broadcast_event(event, &self.external_event_stream.0).await;
    }

    /// Publishes a transaction asynchronously to the network.
    ///
    /// # Errors
    ///
    /// Always returns Ok; does not return an error if the transaction couldn't be published to the network
    #[instrument(skip(self), err, target = "SystemContext", fields(id = self.id))]
    pub async fn publish_transaction_async(
        &self,
        transaction: TYPES::Transaction,
    ) -> Result<(), HotShotError<TYPES>> {
        trace!("Adding transaction to our own queue");

        let api = self.clone();
        let view_number = api.consensus.read().await.cur_view();

        // Wrap up a message
        let message_kind: DataMessage<TYPES> =
            DataMessage::SubmitTransaction(transaction.clone(), view_number);
        let message = Message {
            sender: api.public_key.clone(),
            kind: MessageKind::from(message_kind),
        };

        let serialized_message = self
            .upgrade_lock
            .serialize(&message)
            .await
            .map_err(|_| HotShotError::FailedToSerialize)?;

        async_spawn(async move {
            let da_membership = &api.memberships.da_membership.clone();
            join! {
                // TODO We should have a function that can return a network error if there is one
                // but first we'd need to ensure our network implementations can support that
                // (and not hang instead)

                // version <0, 1> currently fixed; this is the same as VERSION_0_1,
                // and will be updated to be part of SystemContext. I wanted to use associated
                // constants in NodeType, but that seems to be unavailable in the current Rust.
                api
                    .network.broadcast_message(
                        serialized_message,
                        da_membership.committee_topic(),
                        BroadcastDelay::None,
                    ),
                api
                    .send_external_event(Event {
                        view_number,
                        event: EventType::Transactions {
                            transactions: vec![transaction],
                        },
                    }),
            }
        });
        Ok(())
    }

    /// Returns a copy of the consensus struct
    #[must_use]
    pub fn consensus(&self) -> Arc<RwLock<Consensus<TYPES>>> {
        Arc::clone(&self.consensus.inner_consensus)
    }

    /// Returns a copy of the instance state
    pub fn instance_state(&self) -> Arc<TYPES::InstanceState> {
        Arc::clone(&self.instance_state)
    }

    /// Returns a copy of the last decided leaf
    /// # Panics
    /// Panics if internal leaf for consensus is inconsistent
    #[instrument(skip_all, target = "SystemContext", fields(id = self.id))]
    pub async fn decided_leaf(&self) -> Leaf<TYPES> {
        self.consensus.read().await.decided_leaf()
    }

    /// [Non-blocking] instantly returns a copy of the last decided leaf if
    /// it is available to be read. If not, we return `None`.
    ///
    /// # Panics
    /// Panics if internal state for consensus is inconsistent
    #[must_use]
    #[instrument(skip_all, target = "SystemContext", fields(id = self.id))]
    pub fn try_decided_leaf(&self) -> Option<Leaf<TYPES>> {
        self.consensus.try_read().map(|guard| guard.decided_leaf())
    }

    /// Returns the last decided validated state.
    ///
    /// # Panics
    /// Panics if internal state for consensus is inconsistent
    #[instrument(skip_all, target = "SystemContext", fields(id = self.id))]
    pub async fn decided_state(&self) -> Arc<TYPES::ValidatedState> {
        Arc::clone(&self.consensus.read().await.decided_state())
    }

    /// Get the validated state from a given `view`.
    ///
    /// Returns the requested state, if the [`SystemContext`] is tracking this view. Consensus
    /// tracks views that have not yet been decided but could be in the future. This function may
    /// return [`None`] if the requested view has already been decided (but see
    /// [`decided_state`](Self::decided_state)) or if there is no path for the requested
    /// view to ever be decided.
    #[instrument(skip_all, target = "SystemContext", fields(id = self.id))]
    pub async fn state(&self, view: TYPES::Time) -> Option<Arc<TYPES::ValidatedState>> {
        self.consensus.read().await.state(view).cloned()
    }

    /// Initializes a new [`SystemContext`] and does the work of setting up all the background tasks
    ///
    /// Assumes networking implementation is already primed.
    ///
    /// Underlying `HotShot` instance starts out paused, and must be unpaused
    ///
    /// Upon encountering an unrecoverable error, such as a failure to send to a broadcast channel,
    /// the `HotShot` instance will log the error and shut down.
    ///
    /// To construct a [`SystemContext`] without setting up tasks, use `fn new` instead.
    /// # Errors
    ///
    /// Can throw an error if `Self::new` fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn init(
        public_key: TYPES::SignatureKey,
        private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
        node_id: u64,
        config: HotShotConfig<TYPES::SignatureKey>,
        memberships: Memberships<TYPES>,
        network: Arc<I::Network>,
        initializer: HotShotInitializer<TYPES>,
        metrics: ConsensusMetricsValue,
        storage: I::Storage,
        marketplace_config: MarketplaceConfig<TYPES, I>,
    ) -> Result<
        (
            SystemContextHandle<TYPES, I, V>,
            Sender<Arc<HotShotEvent<TYPES>>>,
            Receiver<Arc<HotShotEvent<TYPES>>>,
        ),
        HotShotError<TYPES>,
    > {
        let hotshot = Self::new(
            public_key,
            private_key,
            node_id,
            config,
            memberships,
            network,
            initializer,
            metrics,
            storage,
            marketplace_config,
        )
        .await;
        let handle = Arc::clone(&hotshot).run_tasks().await;
        let (tx, rx) = hotshot.internal_event_stream.clone();

        Ok((handle, tx, rx.activate()))
    }
    /// return the timeout for a view for `self`
    #[must_use]
    pub fn next_view_timeout(&self) -> u64 {
        self.config.next_view_timeout
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> SystemContext<TYPES, I, V> {
    /// Spawn all tasks that operate on [`SystemContextHandle`].
    ///
    /// For a list of which tasks are being spawned, see this module's documentation.
    pub async fn run_tasks(&self) -> SystemContextHandle<TYPES, I, V> {
        let consensus_registry = ConsensusTaskRegistry::new();
        let network_registry = NetworkTaskRegistry::new();

        let output_event_stream = self.external_event_stream.clone();
        let internal_event_stream = self.internal_event_stream.clone();

        let mut handle = SystemContextHandle {
            consensus_registry,
            network_registry,
            output_event_stream: output_event_stream.clone(),
            internal_event_stream: internal_event_stream.clone(),
            hotshot: self.clone().into(),
            storage: Arc::clone(&self.storage),
            network: Arc::clone(&self.network),
            memberships: Arc::clone(&self.memberships),
        };

        add_network_tasks::<TYPES, I, V>(&mut handle).await;
        add_consensus_tasks::<TYPES, I, V>(&mut handle).await;

        handle
    }
}

/// An async broadcast channel
type Channel<S> = (Sender<Arc<S>>, Receiver<Arc<S>>);

#[async_trait]
/// Trait for handling messages for a node with a twin copy of consensus
pub trait TwinsHandlerState<TYPES, I, V>
where
    Self: std::fmt::Debug + Send + Sync,
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    V: Versions,
{
    /// Handle a message sent to the twin from the network task, forwarding it to one of the two twins.
    async fn send_handler(
        &mut self,
        event: &HotShotEvent<TYPES>,
    ) -> Vec<Either<HotShotEvent<TYPES>, HotShotEvent<TYPES>>>;

    /// Handle a message from either twin, forwarding it to the network task
    async fn recv_handler(
        &mut self,
        event: &Either<HotShotEvent<TYPES>, HotShotEvent<TYPES>>,
    ) -> Vec<HotShotEvent<TYPES>>;

    /// Fuse two channels into a single channel
    ///
    /// Note: the channels are fused using two async loops, whose `JoinHandle`s are dropped.
    fn fuse_channels(
        &'static mut self,
        left: Channel<HotShotEvent<TYPES>>,
        right: Channel<HotShotEvent<TYPES>>,
    ) -> Channel<HotShotEvent<TYPES>> {
        let send_state = Arc::new(RwLock::new(self));
        let recv_state = Arc::clone(&send_state);

        let (left_sender, mut left_receiver) = (left.0, left.1);
        let (right_sender, mut right_receiver) = (right.0, right.1);

        // channel to the network task
        let (sender_to_network, network_task_receiver) = broadcast(EVENT_CHANNEL_SIZE);
        // channel from the network task
        let (network_task_sender, mut receiver_from_network): Channel<HotShotEvent<TYPES>> =
            broadcast(EVENT_CHANNEL_SIZE);

        let _recv_loop_handle = async_spawn(async move {
            loop {
                let msg = match select(left_receiver.recv(), right_receiver.recv()).await {
                    Either::Left(msg) => Either::Left(msg.0.unwrap().as_ref().clone()),
                    Either::Right(msg) => Either::Right(msg.0.unwrap().as_ref().clone()),
                };

                let mut state = recv_state.write().await;
                let mut result = state.recv_handler(&msg).await;

                while let Some(event) = result.pop() {
                    let _ = sender_to_network.broadcast(event.into()).await;
                }
            }
        });

        let _send_loop_handle = async_spawn(async move {
            loop {
                if let Ok(msg) = receiver_from_network.recv().await {
                    let mut state = send_state.write().await;

                    let mut result = state.send_handler(&msg).await;

                    while let Some(event) = result.pop() {
                        match event {
                            Either::Left(msg) => {
                                let _ = left_sender.broadcast(msg.into()).await;
                            }
                            Either::Right(msg) => {
                                let _ = right_sender.broadcast(msg.into()).await;
                            }
                        }
                    }
                }
            }
        });

        (network_task_sender, network_task_receiver)
    }

    #[allow(clippy::too_many_arguments)]
    /// Spawn all tasks that operate on [`SystemContextHandle`].
    ///
    /// For a list of which tasks are being spawned, see this module's documentation.
    async fn spawn_twin_handles(
        &'static mut self,
        public_key: TYPES::SignatureKey,
        private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
        nonce: u64,
        config: HotShotConfig<TYPES::SignatureKey>,
        memberships: Memberships<TYPES>,
        network: Arc<I::Network>,
        initializer: HotShotInitializer<TYPES>,
        metrics: ConsensusMetricsValue,
        storage: I::Storage,
        marketplace_config: MarketplaceConfig<TYPES, I>,
    ) -> (
        SystemContextHandle<TYPES, I, V>,
        SystemContextHandle<TYPES, I, V>,
    ) {
        let left_system_context = SystemContext::new(
            public_key.clone(),
            private_key.clone(),
            nonce,
            config.clone(),
            memberships.clone(),
            Arc::clone(&network),
            initializer.clone(),
            metrics.clone(),
            storage.clone(),
            marketplace_config.clone(),
        )
        .await;
        let right_system_context = SystemContext::new(
            public_key,
            private_key,
            nonce,
            config,
            memberships,
            network,
            initializer,
            metrics,
            storage,
            marketplace_config,
        )
        .await;

        // create registries for both handles
        let left_consensus_registry = ConsensusTaskRegistry::new();
        let left_network_registry = NetworkTaskRegistry::new();

        let right_consensus_registry = ConsensusTaskRegistry::new();
        let right_network_registry = NetworkTaskRegistry::new();

        // create external channels for both handles
        let (left_external_sender, left_external_receiver) = broadcast(EXTERNAL_EVENT_CHANNEL_SIZE);
        let left_external_event_stream =
            (left_external_sender, left_external_receiver.deactivate());

        let (right_external_sender, right_external_receiver) =
            broadcast(EXTERNAL_EVENT_CHANNEL_SIZE);
        let right_external_event_stream =
            (right_external_sender, right_external_receiver.deactivate());

        // create internal channels for both handles
        let (left_internal_sender, left_internal_receiver) = broadcast(EVENT_CHANNEL_SIZE);
        let left_internal_event_stream = (
            left_internal_sender.clone(),
            left_internal_receiver.clone().deactivate(),
        );

        let (right_internal_sender, right_internal_receiver) = broadcast(EVENT_CHANNEL_SIZE);
        let right_internal_event_stream = (
            right_internal_sender.clone(),
            right_internal_receiver.clone().deactivate(),
        );

        // create each handle
        let mut left_handle = SystemContextHandle {
            consensus_registry: left_consensus_registry,
            network_registry: left_network_registry,
            output_event_stream: left_external_event_stream.clone(),
            internal_event_stream: left_internal_event_stream.clone(),
            hotshot: Arc::clone(&left_system_context),
            storage: Arc::clone(&left_system_context.storage),
            network: Arc::clone(&left_system_context.network),
            memberships: Arc::clone(&left_system_context.memberships),
        };

        let mut right_handle = SystemContextHandle {
            consensus_registry: right_consensus_registry,
            network_registry: right_network_registry,
            output_event_stream: right_external_event_stream.clone(),
            internal_event_stream: right_internal_event_stream.clone(),
            hotshot: Arc::clone(&right_system_context),
            storage: Arc::clone(&right_system_context.storage),
            network: Arc::clone(&right_system_context.network),
            memberships: Arc::clone(&right_system_context.memberships),
        };

        // add consensus tasks to each handle, using their individual internal event streams
        add_consensus_tasks::<TYPES, I, V>(&mut left_handle).await;
        add_consensus_tasks::<TYPES, I, V>(&mut right_handle).await;

        // fuse the event streams from both handles before initializing the network tasks
        let fused_internal_event_stream = self.fuse_channels(
            (left_internal_sender, left_internal_receiver),
            (right_internal_sender, right_internal_receiver),
        );

        // swap out the event stream on the left handle
        left_handle.internal_event_stream = (
            fused_internal_event_stream.0,
            fused_internal_event_stream.1.deactivate(),
        );

        // add the network tasks to the left handle. note: because the left handle has the fused event stream, the network tasks on the left handle will handle messages from both handles.
        add_network_tasks::<TYPES, I, V>(&mut left_handle).await;

        // revert to the original event stream on the left handle, for any applications that want to listen to it
        left_handle.internal_event_stream = left_internal_event_stream.clone();

        (left_handle, right_handle)
    }
}

#[derive(Debug)]
/// A `TwinsHandlerState` that randomly forwards a message to either twin,
/// and returns messages from both.
pub struct RandomTwinsHandler;

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> TwinsHandlerState<TYPES, I, V>
    for RandomTwinsHandler
{
    async fn send_handler(
        &mut self,
        event: &HotShotEvent<TYPES>,
    ) -> Vec<Either<HotShotEvent<TYPES>, HotShotEvent<TYPES>>> {
        let random: bool = rand::thread_rng().gen();

        #[allow(clippy::match_bool)]
        match random {
            true => vec![Either::Left(event.clone())],
            false => vec![Either::Right(event.clone())],
        }
    }

    async fn recv_handler(
        &mut self,
        event: &Either<HotShotEvent<TYPES>, HotShotEvent<TYPES>>,
    ) -> Vec<HotShotEvent<TYPES>> {
        match event {
            Either::Left(msg) | Either::Right(msg) => vec![msg.clone()],
        }
    }
}

#[derive(Debug)]
/// A `TwinsHandlerState` that forwards each message to both twins,
/// and returns messages from each of them.
pub struct DoubleTwinsHandler;

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> TwinsHandlerState<TYPES, I, V>
    for DoubleTwinsHandler
{
    async fn send_handler(
        &mut self,
        event: &HotShotEvent<TYPES>,
    ) -> Vec<Either<HotShotEvent<TYPES>, HotShotEvent<TYPES>>> {
        vec![Either::Left(event.clone()), Either::Right(event.clone())]
    }

    async fn recv_handler(
        &mut self,
        event: &Either<HotShotEvent<TYPES>, HotShotEvent<TYPES>>,
    ) -> Vec<HotShotEvent<TYPES>> {
        match event {
            Either::Left(msg) | Either::Right(msg) => vec![msg.clone()],
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> ConsensusApi<TYPES, I>
    for SystemContextHandle<TYPES, I, V>
{
    fn total_nodes(&self) -> NonZeroUsize {
        self.hotshot.config.num_nodes_with_stake
    }

    fn builder_timeout(&self) -> Duration {
        self.hotshot.config.builder_timeout
    }

    async fn send_event(&self, event: Event<TYPES>) {
        debug!(?event, "send_event");
        broadcast_event(event, &self.hotshot.external_event_stream.0).await;
    }

    fn public_key(&self) -> &TYPES::SignatureKey {
        &self.hotshot.public_key
    }

    fn private_key(&self) -> &<TYPES::SignatureKey as SignatureKey>::PrivateKey {
        &self.hotshot.private_key
    }
}

#[derive(Clone)]
/// initializer struct for creating starting block
pub struct HotShotInitializer<TYPES: NodeType> {
    /// the leaf specified initialization
    inner: Leaf<TYPES>,

    /// Instance-level state.
    instance_state: TYPES::InstanceState,

    /// Optional validated state.
    ///
    /// If it's given, we'll use it to construct the `SystemContext`. Otherwise, we'll construct
    /// the state from the block header.
    validated_state: Option<Arc<TYPES::ValidatedState>>,

    /// Optional state delta.
    ///
    /// If it's given, we'll use it to construct the `SystemContext`.
    state_delta: Option<Arc<<TYPES::ValidatedState as ValidatedState<TYPES>>::Delta>>,

    /// Starting view number that should be equivelant to the view the node shut down with last.
    start_view: TYPES::Time,
    /// The view we last performed an action in.  An action is Proposing or voting for
    /// Either the quorum or DA.
    actioned_view: TYPES::Time,
    /// Highest QC that was seen, for genesis it's the genesis QC.  It should be for a view greater
    /// than `inner`s view number for the non genesis case because we must have seen higher QCs
    /// to decide on the leaf.
    high_qc: QuorumCertificate<TYPES>,
    /// Undecided leafs that were seen, but not yet decided on.  These allow a restarting node
    /// to vote and propose right away if they didn't miss anything while down.
    undecided_leafs: Vec<Leaf<TYPES>>,
    /// Not yet decided state
    undecided_state: BTreeMap<TYPES::Time, View<TYPES>>,
    /// Proposals we have sent out to provide to others for catchup
    saved_proposals: BTreeMap<TYPES::Time, Proposal<TYPES, QuorumProposal<TYPES>>>,
}

impl<TYPES: NodeType> HotShotInitializer<TYPES> {
    /// initialize from genesis
    /// # Errors
    /// If we are unable to apply the genesis block to the default state
    pub async fn from_genesis<V: Versions>(
        instance_state: TYPES::InstanceState,
    ) -> Result<Self, HotShotError<TYPES>> {
        let (validated_state, state_delta) = TYPES::ValidatedState::genesis(&instance_state);
        let high_qc = QuorumCertificate::genesis::<V>(&validated_state, &instance_state).await;

        Ok(Self {
            inner: Leaf::genesis(&validated_state, &instance_state).await,
            validated_state: Some(Arc::new(validated_state)),
            state_delta: Some(Arc::new(state_delta)),
            start_view: TYPES::Time::new(0),
            actioned_view: TYPES::Time::new(0),
            saved_proposals: BTreeMap::new(),
            high_qc,
            undecided_leafs: Vec::new(),
            undecided_state: BTreeMap::new(),
            instance_state,
        })
    }

    /// Reload previous state based on most recent leaf and the instance-level state.
    ///
    /// # Arguments
    /// *  `start_view` - The minimum view number that we are confident won't lead to a double vote
    /// after restart.
    /// * `validated_state` - Optional validated state that if given, will be used to construct the
    /// `SystemContext`.
    #[allow(clippy::too_many_arguments)]
    pub fn from_reload(
        anchor_leaf: Leaf<TYPES>,
        instance_state: TYPES::InstanceState,
        validated_state: Option<Arc<TYPES::ValidatedState>>,
        start_view: TYPES::Time,
        actioned_view: TYPES::Time,
        saved_proposals: BTreeMap<TYPES::Time, Proposal<TYPES, QuorumProposal<TYPES>>>,
        high_qc: QuorumCertificate<TYPES>,
        undecided_leafs: Vec<Leaf<TYPES>>,
        undecided_state: BTreeMap<TYPES::Time, View<TYPES>>,
    ) -> Self {
        Self {
            inner: anchor_leaf,
            instance_state,
            validated_state,
            state_delta: None,
            start_view,
            actioned_view,
            saved_proposals,
            high_qc,
            undecided_leafs,
            undecided_state,
        }
    }
}
