//! Provides a generic rust implementation of the `HotShot` BFT protocol
//!

// Documentation module
#[cfg(feature = "docs")]
pub mod documentation;

/// Contains traits consumed by [`SystemContext`]
pub mod traits;
/// Contains types used by the crate
pub mod types;

pub mod tasks;

use crate::{
    tasks::{
        add_consensus_task, add_da_task, add_network_event_task, add_network_message_task,
        add_transaction_task, add_upgrade_task, add_view_sync_task,
    },
    traits::{NodeImplementation, Storage},
    types::{Event, SystemContextHandle},
};
use async_broadcast::{broadcast, InactiveReceiver, Receiver, Sender};
use async_compatibility_layer::art::async_spawn;
use async_lock::RwLock;
use async_trait::async_trait;
use commit::Committable;
use futures::join;
use hotshot_constants::{EVENT_CHANNEL_SIZE, VERSION_0_1};
use hotshot_task_impls::events::HotShotEvent;
use hotshot_task_impls::helpers::broadcast_event;
use hotshot_task_impls::network;

use hotshot_task::task::TaskRegistry;
use hotshot_types::{
    consensus::{Consensus, ConsensusMetricsValue, View, ViewInner},
    data::Leaf,
    error::StorageSnafu,
    event::EventType,
    message::{DataMessage, Message, MessageKind},
    simple_certificate::QuorumCertificate,
    traits::{
        consensus_api::ConsensusApi,
        election::Membership,
        network::ConnectedNetwork,
        node_implementation::{ConsensusTime, NodeType},
        signature_key::SignatureKey,
        states::ValidatedState,
        storage::StoredView,
        BlockPayload,
    },
    HotShotConfig,
};
use snafu::ResultExt;
use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
    num::NonZeroUsize,
    sync::Arc,
    time::Duration,
};
use tasks::add_vid_task;
use tracing::{debug, instrument, trace};

// -- Rexports
// External
/// Reexport rand crate
pub use rand;
// Internal
/// Reexport error type
pub use hotshot_types::error::HotShotError;

/// Length, in bytes, of a 512 bit hash
pub const H_512: usize = 64;
/// Length, in bytes, of a 256 bit hash
pub const H_256: usize = 32;

/// Bundle of the networks used in consensus
pub struct Networks<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Newtork for reaching all nodes
    pub quorum_network: Arc<I::QuorumNetwork>,

    /// Network for reaching the DA committee
    pub da_network: Arc<I::CommitteeNetwork>,

    /// Phantom for TYPES and I
    pub _pd: PhantomData<(TYPES, I)>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> Networks<TYPES, I> {
    /// wait for all networks to be ready
    pub async fn wait_for_networks_ready(&self) {
        self.quorum_network.wait_for_ready().await;
        self.da_network.wait_for_ready().await;
    }

    /// shut down all networks
    pub async fn shut_down_networks(&self) {
        self.quorum_network.shut_down().await;
        self.da_network.shut_down().await;
    }
}

/// Bundle of all the memberships a consensus instance uses
#[derive(Clone)]
pub struct Memberships<TYPES: NodeType> {
    /// Quorum Membership
    pub quorum_membership: TYPES::Membership,
    /// DA
    pub da_membership: TYPES::Membership,
    /// VID
    pub vid_membership: TYPES::Membership,
    /// View Sync
    pub view_sync_membership: TYPES::Membership,
}

/// Holds the state needed to participate in `HotShot` consensus
#[derive(Clone)]
pub struct SystemContext<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// The public key of this node
    public_key: TYPES::SignatureKey,

    /// The private key of this node
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,

    /// Configuration items for this hotshot instance
    pub config: HotShotConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,

    /// This `HotShot` instance's storage backend
    storage: I::Storage,

    /// Networks used by the instance of hotshot
    pub networks: Arc<Networks<TYPES, I>>,

    /// Memberships used by consensus
    pub memberships: Arc<Memberships<TYPES>>,

    /// the metrics that the implementor is using.
    _metrics: Arc<ConsensusMetricsValue>,

    /// The hotstuff implementation
    consensus: Arc<RwLock<Consensus<TYPES>>>,

    // global_registry: GlobalRegistry,
    /// Access to the output event stream.
    pub output_event_stream: (Sender<Event<TYPES>>, InactiveReceiver<Event<TYPES>>),

    /// access to the internal event stream, in case we need to, say, shut something down
    internal_event_stream: (
        Sender<HotShotEvent<TYPES>>,
        InactiveReceiver<HotShotEvent<TYPES>>,
    ),

    /// uid for instrumentation
    pub id: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> SystemContext<TYPES, I> {
    /// Creates a new [`Arc<SystemContext>`] with the given configuration options.
    ///
    /// To do a full initialization, use `fn init` instead, which will set up background tasks as
    /// well.
    #[allow(clippy::too_many_arguments)]
    #[instrument(skip(private_key, storage, memberships, networks, initializer, metrics))]
    pub async fn new(
        public_key: TYPES::SignatureKey,
        private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
        nonce: u64,
        config: HotShotConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
        storage: I::Storage,
        memberships: Memberships<TYPES>,
        networks: Networks<TYPES, I>,
        initializer: HotShotInitializer<TYPES>,
        metrics: ConsensusMetricsValue,
    ) -> Result<Arc<Self>, HotShotError<TYPES>> {
        debug!("Creating a new hotshot");

        let consensus_metrics = Arc::new(metrics);
        let anchored_leaf = initializer.inner;
        let instance_state = initializer.instance_state;

        // insert to storage
        storage
            .append(vec![anchored_leaf.clone().into()])
            .await
            .context(StorageSnafu)?;

        // insert genesis (or latest block) to state map
        let mut validated_state_map = BTreeMap::default();
        let validated_state = Arc::new(TYPES::ValidatedState::from_header(
            &anchored_leaf.block_header,
        ));
        validated_state_map.insert(
            anchored_leaf.get_view_number(),
            View {
                view_inner: ViewInner::Leaf {
                    leaf: anchored_leaf.commit(),
                    state: validated_state,
                },
            },
        );

        let mut saved_leaves = HashMap::new();
        let mut saved_payloads = BTreeMap::new();
        saved_leaves.insert(anchored_leaf.commit(), anchored_leaf.clone());
        if let Some(payload) = anchored_leaf.get_block_payload() {
            let encoded_txns: Vec<u8> = match payload.encode() {
                // TODO (Keyao) [VALIDATED_STATE] - Avoid collect/copy on the encoded transaction bytes.
                // <https://github.com/EspressoSystems/HotShot/issues/2115>
                Ok(encoded) => encoded.into_iter().collect(),
                Err(e) => {
                    return Err(HotShotError::BlockError { source: e });
                }
            };
            saved_payloads.insert(anchored_leaf.get_view_number(), encoded_txns.clone());
            // View 1 doesn't have DA which is responsible for saving the payloads, so we store the
            // payload for view 1 manually during the intialization.
            saved_payloads.insert(TYPES::Time::new(1), encoded_txns);
        }

        let start_view = initializer.start_view;

        let consensus = Consensus {
            instance_state,
            validated_state_map,
            cur_view: start_view,
            last_decided_view: anchored_leaf.get_view_number(),
            saved_leaves,
            saved_payloads,
            saved_da_certs: HashMap::new(),
            // TODO this is incorrect
            // https://github.com/EspressoSystems/HotShot/issues/560
            locked_view: anchored_leaf.get_view_number(),
            high_qc: anchored_leaf.get_justify_qc(),
            metrics: consensus_metrics.clone(),
        };
        let consensus = Arc::new(RwLock::new(consensus));

        let (internal_tx, internal_rx) = broadcast(EVENT_CHANNEL_SIZE);
        let (mut external_tx, external_rx) = broadcast(EVENT_CHANNEL_SIZE);

        // This makes it so we won't block on broadcasting if there is not a receiver
        // Our own copy of the receiver is inactive so it doesn't count.
        external_tx.set_await_active(false);

        let inner: Arc<SystemContext<TYPES, I>> = Arc::new(SystemContext {
            id: nonce,
            consensus,
            public_key,
            private_key,
            config,
            storage,
            networks: Arc::new(networks),
            memberships: Arc::new(memberships),
            _metrics: consensus_metrics.clone(),
            internal_event_stream: (internal_tx, internal_rx.deactivate()),
            output_event_stream: (external_tx, external_rx.deactivate()),
        });

        Ok(inner)
    }

    /// "Starts" consensus by sending a `QCFormed` event
    ///
    /// # Panics
    /// Panics if sending genesis fails
    pub async fn start_consensus(&self) {
        debug!("Starting Consensus");
        self.internal_event_stream
            .0
            .broadcast_direct(HotShotEvent::QCFormed(either::Left(
                QuorumCertificate::genesis(),
            )))
            .await
            .expect("Genesis Broadcast failed");
    }

    /// Emit an external event
    // A copypasta of `ConsensusApi::send_event`
    // TODO: remove with https://github.com/EspressoSystems/HotShot/issues/2407
    async fn send_external_event(&self, event: Event<TYPES>) {
        debug!(?event, "send_external_event");
        broadcast_event(event, &self.output_event_stream.0).await;
    }

    /// Publishes a transaction asynchronously to the network.
    ///
    /// # Errors
    ///
    /// Always returns Ok; does not return an error if the transaction couldn't be published to the network
    #[instrument(skip(self), err)]
    pub async fn publish_transaction_async(
        &self,
        transaction: TYPES::Transaction,
    ) -> Result<(), HotShotError<TYPES>> {
        trace!("Adding transaction to our own queue");

        let api = self.clone();
        let view_number = api.consensus.read().await.cur_view;

        // Wrap up a message
        let message = DataMessage::SubmitTransaction(transaction.clone(), view_number);

        async_spawn(async move {
            let da_membership = &api.memberships.da_membership.clone();
            join! {
                // TODO We should have a function that can return a network error if there is one
                // but first we'd need to ensure our network implementations can support that
                // (and not hang instead)
                //
                api
                    .networks
                    .da_network
                    .broadcast_message(
                        Message {
                            version: VERSION_0_1,
                            sender: api.public_key.clone(),
                            kind: MessageKind::from(message),
                        },
                        da_membership.get_committee(view_number),
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
    pub fn get_consensus(&self) -> Arc<RwLock<Consensus<TYPES>>> {
        self.consensus.clone()
    }

    /// Returns a copy of the last decided leaf
    /// # Panics
    /// Panics if internal leaf for consensus is inconsistent
    pub async fn get_decided_leaf(&self) -> Leaf<TYPES> {
        self.consensus.read().await.get_decided_leaf()
    }

    /// [Non-blocking] instantly returns a copy of the last decided leaf if
    /// it is available to be read. If not, we return `None`.
    ///
    /// # Panics
    /// Panics if internal state for consensus is inconsistent
    #[must_use]
    pub fn try_get_decided_leaf(&self) -> Option<Leaf<TYPES>> {
        self.consensus
            .try_read()
            .map(|guard| guard.get_decided_leaf())
    }

    /// Returns the last decided validated state.
    ///
    /// # Panics
    /// Panics if internal state for consensus is inconsistent
    pub async fn get_decided_state(&self) -> Arc<TYPES::ValidatedState> {
        self.consensus.read().await.get_decided_state().clone()
    }

    /// Get the validated state from a given `view`.
    ///
    /// Returns the requested state, if the [`SystemContext`] is tracking this view. Consensus
    /// tracks views that have not yet been decided but could be in the future. This function may
    /// return [`None`] if the requested view has already been decided (but see
    /// [`get_decided_state`](Self::get_decided_state)) or if there is no path for the requested
    /// view to ever be decided.
    pub async fn get_state(&self, view: TYPES::Time) -> Option<Arc<TYPES::ValidatedState>> {
        self.consensus.read().await.get_state(view).cloned()
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
    ///
    /// # Errors
    ///
    /// Will return an error when the storage failed to insert the first `QuorumCertificate`
    #[allow(clippy::too_many_arguments)]
    pub async fn init(
        public_key: TYPES::SignatureKey,
        private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
        node_id: u64,
        config: HotShotConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
        storage: I::Storage,
        memberships: Memberships<TYPES>,
        networks: Networks<TYPES, I>,
        initializer: HotShotInitializer<TYPES>,
        metrics: ConsensusMetricsValue,
    ) -> Result<
        (
            SystemContextHandle<TYPES, I>,
            Sender<HotShotEvent<TYPES>>,
            Receiver<HotShotEvent<TYPES>>,
        ),
        HotShotError<TYPES>,
    > {
        // Save a clone of the storage for the handle
        let hotshot = Self::new(
            public_key,
            private_key,
            node_id,
            config,
            storage,
            memberships,
            networks,
            initializer,
            metrics,
        )
        .await?;
        let handle = hotshot.clone().run_tasks().await;
        let (tx, rx) = hotshot.internal_event_stream.clone();

        Ok((handle, tx, rx.activate()))
    }
    /// return the timeout for a view for `self`
    #[must_use]
    pub fn get_next_view_timeout(&self) -> u64 {
        self.config.next_view_timeout
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> SystemContext<TYPES, I> {
    /// Get access to [`Consensus`]
    #[must_use]
    pub fn consensus(&self) -> &Arc<RwLock<Consensus<TYPES>>> {
        &self.consensus
    }

    /// Spawn all tasks that operate on [`SystemContextHandle`].
    ///
    /// For a list of which tasks are being spawned, see this module's documentation.
    #[allow(clippy::too_many_lines)]
    pub async fn run_tasks(&self) -> SystemContextHandle<TYPES, I> {
        // ED Need to set first first number to 1, or properly trigger the change upon start
        let registry = Arc::new(TaskRegistry::default());

        let output_event_stream = self.output_event_stream.clone();
        let internal_event_stream = self.internal_event_stream.clone();

        let quorum_network = self.networks.quorum_network.clone();
        let da_network = self.networks.da_network.clone();
        let quorum_membership = self.memberships.quorum_membership.clone();
        let da_membership = self.memberships.da_membership.clone();
        let vid_membership = self.memberships.vid_membership.clone();
        let view_sync_membership = self.memberships.view_sync_membership.clone();

        let (event_tx, event_rx) = internal_event_stream.clone();

        let handle = SystemContextHandle {
            registry: registry.clone(),
            output_event_stream: output_event_stream.clone(),
            internal_event_stream: internal_event_stream.clone(),
            hotshot: self.clone().into(),
            storage: self.storage.clone(),
        };

        add_network_message_task(registry.clone(), event_tx.clone(), quorum_network.clone()).await;
        add_network_message_task(registry.clone(), event_tx.clone(), da_network.clone()).await;

        add_network_event_task(
            registry.clone(),
            event_tx.clone(),
            event_rx.activate_cloned(),
            quorum_network.clone(),
            quorum_membership,
            network::quorum_filter,
        )
        .await;
        add_network_event_task(
            registry.clone(),
            event_tx.clone(),
            event_rx.activate_cloned(),
            da_network.clone(),
            da_membership,
            network::committee_filter,
        )
        .await;
        add_network_event_task(
            registry.clone(),
            event_tx.clone(),
            event_rx.activate_cloned(),
            quorum_network.clone(),
            view_sync_membership,
            network::view_sync_filter,
        )
        .await;
        add_network_event_task(
            registry.clone(),
            event_tx.clone(),
            event_rx.activate_cloned(),
            quorum_network.clone(),
            vid_membership,
            network::vid_filter,
        )
        .await;
        add_consensus_task(
            registry.clone(),
            event_tx.clone(),
            event_rx.activate_cloned(),
            &handle,
        )
        .await;
        add_da_task(
            registry.clone(),
            event_tx.clone(),
            event_rx.activate_cloned(),
            &handle,
        )
        .await;
        add_vid_task(
            registry.clone(),
            event_tx.clone(),
            event_rx.activate_cloned(),
            &handle,
        )
        .await;
        add_transaction_task(
            registry.clone(),
            event_tx.clone(),
            event_rx.activate_cloned(),
            &handle,
        )
        .await;
        add_view_sync_task(
            registry.clone(),
            event_tx.clone(),
            event_rx.activate_cloned(),
            &handle,
        )
        .await;
        add_upgrade_task(
            registry.clone(),
            event_tx.clone(),
            event_rx.activate_cloned(),
            &handle,
        )
        .await;
        handle
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> ConsensusApi<TYPES, I>
    for SystemContextHandle<TYPES, I>
{
    fn total_nodes(&self) -> NonZeroUsize {
        self.hotshot.config.total_nodes
    }

    fn propose_min_round_time(&self) -> Duration {
        self.hotshot.config.propose_min_round_time
    }

    fn propose_max_round_time(&self) -> Duration {
        self.hotshot.config.propose_max_round_time
    }

    fn max_transactions(&self) -> NonZeroUsize {
        self.hotshot.config.max_transactions
    }

    fn min_transactions(&self) -> usize {
        self.hotshot.config.min_transactions
    }

    async fn send_event(&self, event: Event<TYPES>) {
        debug!(?event, "send_event");
        broadcast_event(event, &self.hotshot.output_event_stream.0).await;
    }

    fn public_key(&self) -> &TYPES::SignatureKey {
        &self.hotshot.public_key
    }

    fn private_key(&self) -> &<TYPES::SignatureKey as SignatureKey>::PrivateKey {
        &self.hotshot.private_key
    }

    async fn store_leaf(
        &self,
        old_anchor_view: TYPES::Time,
        leaf: Leaf<TYPES>,
    ) -> std::result::Result<(), hotshot_types::traits::storage::StorageError> {
        let view_to_insert = StoredView::from(leaf);
        let storage = &self.hotshot.storage;
        storage.append_single_view(view_to_insert).await?;
        storage.cleanup_storage_up_to_view(old_anchor_view).await?;
        storage.commit().await?;
        Ok(())
    }
}

/// initializer struct for creating starting block
pub struct HotShotInitializer<TYPES: NodeType> {
    /// the leaf specified initialization
    inner: Leaf<TYPES>,

    /// Instance-level state.
    instance_state: TYPES::InstanceState,

    /// Starting view number that we are confident won't lead to a double vote after restart.
    start_view: TYPES::Time,
}

impl<TYPES: NodeType> HotShotInitializer<TYPES> {
    /// initialize from genesis
    /// # Errors
    /// If we are unable to apply the genesis block to the default state
    pub fn from_genesis(instance_state: TYPES::InstanceState) -> Result<Self, HotShotError<TYPES>> {
        Ok(Self {
            inner: Leaf::genesis(&instance_state),
            instance_state,
            start_view: TYPES::Time::new(0),
        })
    }

    /// Reload previous state based on most recent leaf and the instance-level state.
    ///
    /// # Arguments
    /// *  `start_view` - The minimum view number that we are confident won't lead to a double vote
    /// after restart.
    pub fn from_reload(
        anchor_leaf: Leaf<TYPES>,
        instance_state: TYPES::InstanceState,
        start_view: TYPES::Time,
    ) -> Self {
        Self {
            inner: anchor_leaf,
            instance_state,
            start_view,
        }
    }
}
