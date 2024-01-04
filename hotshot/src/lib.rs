#![warn(
    clippy::all,
    clippy::pedantic,
    rust_2018_idioms,
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::panic
)]
#![allow(clippy::module_name_repetitions)]
// Temporary
#![allow(clippy::cast_possible_truncation)]
// Temporary, should be disabled after the completion of the NodeImplementation refactor
#![allow(clippy::type_complexity)]
//! Provides a generic rust implementation of the `HotShot` BFT protocol
//!
//! See the [protocol documentation](https://github.com/EspressoSystems/hotshot-spec) for a protocol description.

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
        add_transaction_task, add_view_sync_task,
    },
    traits::{NodeImplementation, Storage},
    types::{Event, SystemContextHandle},
};
use async_compatibility_layer::{
    art::{async_spawn, async_spawn_local},
    async_primitives::broadcast::BroadcastSender,
    channel::UnboundedSender,
};
use async_lock::{RwLock, RwLockUpgradableReadGuard, RwLockWriteGuard};
use async_trait::async_trait;
use commit::Committable;
use custom_debug::Debug;
use hotshot_task::{
    event_stream::{ChannelStream, EventStream},
    task_launcher::TaskRunner,
};
use hotshot_task_impls::{events::HotShotEvent, network::NetworkTaskKind};

use hotshot_types::{
    consensus::{Consensus, ConsensusMetricsValue, PayloadStore, View, ViewInner, ViewQueue},
    data::Leaf,
    error::StorageSnafu,
    message::{
        DataMessage, InternalTrigger, Message, MessageKind, ProcessedGeneralConsensusMessage,
    },
    simple_certificate::QuorumCertificate,
    traits::{
        consensus_api::ConsensusApi,
        network::{CommunicationChannel, NetworkError},
        node_implementation::{ChannelMaps, NodeType, SendToTasks},
        signature_key::SignatureKey,
        state::ConsensusTime,
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
use tracing::{debug, error, info, instrument, trace, warn};
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
    pub quorum_network: I::QuorumNetwork,

    /// Network for reaching the DA committee
    pub da_network: I::CommitteeNetwork,

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
pub struct SystemContextInner<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// The public key of this node
    public_key: TYPES::SignatureKey,

    /// The private key of this node
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,

    /// Configuration items for this hotshot instance
    config: HotShotConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,

    /// This `HotShot` instance's storage backend
    storage: I::Storage,

    /// Networks used by the instance of hotshot
    pub networks: Arc<Networks<TYPES, I>>,

    /// Memberships used by consensus
    pub memberships: Arc<Memberships<TYPES>>,

    // pub quorum_network: Arc<I::QuorumNetwork>;
    // pub committee_network: Arc<I::CommitteeNetwork>;
    /// Sender for [`Event`]s
    event_sender: RwLock<Option<BroadcastSender<Event<TYPES>>>>,

    /// the metrics that the implementor is using.
    _metrics: Arc<ConsensusMetricsValue>,

    /// The hotstuff implementation
    consensus: Arc<RwLock<Consensus<TYPES>>>,

    /// Channels for sending/recv-ing proposals and votes for quorum and committee exchanges, the
    /// latter of which is only applicable for sequencing consensus.
    channel_maps: (ChannelMaps<TYPES>, Option<ChannelMaps<TYPES>>),

    // global_registry: GlobalRegistry,
    /// Access to the output event stream.
    output_event_stream: ChannelStream<Event<TYPES>>,

    /// access to the internal event stream, in case we need to, say, shut something down
    internal_event_stream: ChannelStream<HotShotEvent<TYPES>>,

    /// uid for instrumentation
    id: u64,
}

/// Thread safe, shared view of a `HotShot`
// TODO Perhaps we can delete SystemContext since we only consume it in run_tasks()
#[derive(Clone)]
pub struct SystemContext<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Handle to internal hotshot implementation
    pub inner: Arc<SystemContextInner<TYPES, I>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> SystemContext<TYPES, I> {
    /// Creates a new hotshot with the given configuration options and sets it up with the given
    /// genesis block
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
    ) -> Result<Self, HotShotError<TYPES>> {
        debug!("Creating a new hotshot");

        let consensus_metrics = Arc::new(metrics);
        let anchored_leaf = initializer.inner;

        // insert to storage
        storage
            .append(vec![anchored_leaf.clone().into()])
            .await
            .context(StorageSnafu)?;

        // insert genesis (or latest block) to state map
        let mut state_map = BTreeMap::default();
        state_map.insert(
            anchored_leaf.get_view_number(),
            View {
                view_inner: ViewInner::Leaf {
                    leaf: anchored_leaf.commit(),
                },
            },
        );

        let mut saved_leaves = HashMap::new();
        let mut saved_payloads = PayloadStore::default();
        saved_leaves.insert(anchored_leaf.commit(), anchored_leaf.clone());
        let payload_commitment = anchored_leaf.get_payload_commitment();
        if let Some(payload) = anchored_leaf.get_block_payload() {
            let encoded_txns = match payload.encode() {
                // TODO (Keyao) [VALIDATED_STATE] - Avoid collect/copy on the encoded transaction bytes.
                // <https://github.com/EspressoSystems/HotShot/issues/2115>
                Ok(encoded) => encoded.into_iter().collect(),
                Err(e) => {
                    return Err(HotShotError::BlockError { source: e });
                }
            };
            saved_payloads.insert(payload_commitment, encoded_txns);
        }

        let start_view = anchored_leaf.get_view_number();

        let consensus = Consensus {
            state_map,
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

        let inner: Arc<SystemContextInner<TYPES, I>> = Arc::new(SystemContextInner {
            id: nonce,
            channel_maps: I::new_channel_maps(start_view),
            consensus,
            public_key,
            private_key,
            config,
            storage,
            networks: Arc::new(networks),
            memberships: Arc::new(memberships),
            event_sender: RwLock::default(),
            _metrics: consensus_metrics.clone(),
            internal_event_stream: ChannelStream::new(),
            output_event_stream: ChannelStream::new(),
        });

        Ok(Self { inner })
    }

    /// "Starts" consensus by sending a `QCFormed` event
    pub async fn start_consensus(&self) {
        self.inner
            .internal_event_stream
            .publish(HotShotEvent::QCFormed(either::Left(
                QuorumCertificate::genesis(),
            )))
            .await;
    }

    /// Marks a given view number as timed out. This should be called a fixed period after a round is started.
    ///
    /// If the round has already ended then this function will essentially be a no-op. Otherwise `run_round` will return shortly after this function is called.
    /// # Panics
    /// Panics if the current view is not in the channel map
    #[instrument(
        skip_all,
        fields(id = self.inner.id, view = *current_view),
        name = "Timeout consensus tasks",
        level = "warn"
    )]
    pub async fn timeout_view(
        &self,
        current_view: TYPES::Time,
        send_replica: UnboundedSender<ProcessedGeneralConsensusMessage<TYPES>>,
        send_next_leader: Option<UnboundedSender<ProcessedGeneralConsensusMessage<TYPES>>>,
    ) {
        let msg = ProcessedGeneralConsensusMessage::<TYPES>::InternalTrigger(
            InternalTrigger::Timeout(current_view),
        );
        if let Some(chan) = send_next_leader {
            if chan.send(msg.clone()).await.is_err() {
                debug!("Error timing out next leader task");
            }
        };
        // NOTE this should always exist
        if send_replica.send(msg).await.is_err() {
            debug!("Error timing out replica task");
        };
    }

    /// Publishes a transaction asynchronously to the network
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
        // Wrap up a message
        // TODO place a view number here that makes sense
        // we haven't worked out how this will work yet
        let message = DataMessage::SubmitTransaction(transaction, TYPES::Time::new(0));
        let api = self.clone();
        // TODO We should have a function that can return a network error if there is one
        // but first we'd need to ensure our network implementations can support that
        // (and not hang instead)
        async_spawn(async move {
            let _result = api
                .inner
                .networks
                .da_network
                .broadcast_message(
                    Message {
                        sender: api.inner.public_key.clone(),
                        kind: MessageKind::from(message),
                    },
                    &api.inner.memberships.da_membership.clone(),
                )
                .await;
        });
        Ok(())
    }

    /// Returns a copy of the state
    ///
    /// # Panics
    ///
    /// Panics if internal state for consensus is inconsistent
    pub async fn get_state(&self) {
        self.inner
            .consensus
            .read()
            .await
            .get_decided_leaf()
            .get_state();
    }

    /// Returns a copy of the consensus struct
    #[must_use]
    pub fn get_consensus(&self) -> Arc<RwLock<Consensus<TYPES>>> {
        self.inner.consensus.clone()
    }

    /// Returns a copy of the last decided leaf
    /// # Panics
    /// Panics if internal state for consensus is inconsistent
    pub async fn get_decided_leaf(&self) -> Leaf<TYPES> {
        self.inner.consensus.read().await.get_decided_leaf()
    }

    /// Initializes a new hotshot and does the work of setting up all the background tasks
    ///
    /// Assumes networking implementation is already primed.
    ///
    /// Underlying `HotShot` instance starts out paused, and must be unpaused
    ///
    /// Upon encountering an unrecoverable error, such as a failure to send to a broadcast channel,
    /// the `HotShot` instance will log the error and shut down.
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
            ChannelStream<HotShotEvent<TYPES>>,
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
        let internal_event_stream = hotshot.inner.internal_event_stream.clone();

        Ok((handle, internal_event_stream))
    }

    /// Send a broadcast message.
    ///
    /// This is an alias for `hotshot.inner.networking.broadcast_message(msg.into())`.
    ///
    /// # Errors
    ///
    /// Will return any errors that the underlying `broadcast_message` can return.
    // this clippy lint is silly. This is async by requirement of the trait.
    #[allow(clippy::unused_async)]
    pub async fn send_broadcast_message(
        &self,
        kind: impl Into<MessageKind<TYPES>>,
    ) -> std::result::Result<(), NetworkError> {
        let inner = self.inner.clone();
        let pk = self.inner.public_key.clone();
        let kind = kind.into();

        async_spawn_local(async move {
            if inner
                .networks
                .quorum_network
                .broadcast_message(
                    Message { sender: pk, kind },
                    // TODO this is morally wrong
                    &inner.memberships.quorum_membership.clone(),
                )
                .await
                .is_err()
            {
                warn!("Failed to broadcast message");
            };
        });
        Ok(())
    }

    /// Send a direct message to a given recipient.
    ///
    /// This is an alias for `hotshot.inner.networking.message_node(msg.into(), recipient)`.
    ///
    /// # Errors
    ///
    /// Will return any errors that the underlying `message_node` can return.
    pub async fn send_direct_message(
        &self,
        kind: impl Into<MessageKind<TYPES>>,
        recipient: TYPES::SignatureKey,
    ) -> std::result::Result<(), NetworkError> {
        self.inner
            .networks
            .quorum_network
            .direct_message(
                Message {
                    sender: self.inner.public_key.clone(),
                    kind: kind.into(),
                },
                recipient,
            )
            .await?;
        Ok(())
    }

    /// return the timeout for a view for `self`
    #[must_use]
    pub fn get_next_view_timeout(&self) -> u64 {
        self.inner.config.next_view_timeout
    }

    /// given a view number and a upgradable read lock on a channel map, inserts entry into map if it
    /// doesn't exist, or creates entry. Then returns a clone of the entry
    pub async fn create_or_obtain_chan_from_read(
        view_num: TYPES::Time,
        channel_map: RwLockUpgradableReadGuard<'_, SendToTasks<TYPES>>,
    ) -> ViewQueue<TYPES> {
        // check if we have the entry
        // if we don't, insert
        if let Some(vq) = channel_map.channel_map.get(&view_num) {
            vq.clone()
        } else {
            let mut channel_map =
                RwLockUpgradableReadGuard::<'_, SendToTasks<TYPES>>::upgrade(channel_map).await;
            let new_view_queue = ViewQueue::default();
            let vq = new_view_queue.clone();
            // NOTE: the read lock is held until all other read locks are DROPPED and
            // the read lock may be turned into a write lock.
            // This means that the `channel_map` will not change. So we don't need
            // to check again to see if a channel was added

            channel_map.channel_map.insert(view_num, new_view_queue);
            vq
        }
    }

    /// given a view number and a write lock on a channel map, inserts entry into map if it
    /// doesn't exist, or creates entry. Then returns a clone of the entry
    #[allow(clippy::unused_async)] // async for API compatibility reasons
    pub async fn create_or_obtain_chan_from_write(
        view_num: TYPES::Time,
        mut channel_map: RwLockWriteGuard<'_, SendToTasks<TYPES>>,
    ) -> ViewQueue<TYPES> {
        channel_map.channel_map.entry(view_num).or_default().clone()
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> SystemContext<TYPES, I> {
    /// Get access to [`Consensus`]
    #[must_use]
    pub fn consensus(&self) -> &Arc<RwLock<Consensus<TYPES>>> {
        &self.inner.consensus
    }

    /// Spawn all tasks that operate on [`SystemContextHandle`].
    ///
    /// For a list of which tasks are being spawned, see this module's documentation.
    #[allow(clippy::too_many_lines)]
    pub async fn run_tasks(self) -> SystemContextHandle<TYPES, I> {
        // ED Need to set first first number to 1, or properly trigger the change upon start
        let task_runner = TaskRunner::new();
        let registry = task_runner.registry.clone();

        let output_event_stream = self.inner.output_event_stream.clone();
        let internal_event_stream = self.inner.internal_event_stream.clone();

        let quorum_network = self.inner.networks.quorum_network.clone();
        let da_network = self.inner.networks.da_network.clone();
        let quorum_membership = self.inner.memberships.quorum_membership.clone();
        let da_membership = self.inner.memberships.da_membership.clone();
        let vid_membership = self.inner.memberships.vid_membership.clone();
        let view_sync_membership = self.inner.memberships.view_sync_membership.clone();

        let handle = SystemContextHandle {
            registry,
            output_event_stream: output_event_stream.clone(),
            internal_event_stream: internal_event_stream.clone(),
            hotshot: self.clone(),
            storage: self.inner.storage.clone(),
        };

        let task_runner = add_network_message_task(
            task_runner,
            internal_event_stream.clone(),
            quorum_network.clone(),
        )
        .await;
        let task_runner = add_network_message_task(
            task_runner,
            internal_event_stream.clone(),
            da_network.clone(),
        )
        .await;

        let task_runner = add_network_event_task(
            task_runner,
            internal_event_stream.clone(),
            quorum_network.clone(),
            quorum_membership,
            NetworkTaskKind::Quorum,
        )
        .await;
        let task_runner = add_network_event_task(
            task_runner,
            internal_event_stream.clone(),
            da_network.clone(),
            da_membership,
            NetworkTaskKind::Committee,
        )
        .await;
        let task_runner = add_network_event_task(
            task_runner,
            internal_event_stream.clone(),
            quorum_network.clone(),
            view_sync_membership,
            NetworkTaskKind::ViewSync,
        )
        .await;
        let task_runner = add_network_event_task(
            task_runner,
            internal_event_stream.clone(),
            quorum_network.clone(),
            vid_membership,
            NetworkTaskKind::VID,
        )
        .await;
        let task_runner = add_consensus_task(
            task_runner,
            internal_event_stream.clone(),
            output_event_stream.clone(),
            handle.clone(),
        )
        .await;
        let task_runner =
            add_da_task(task_runner, internal_event_stream.clone(), handle.clone()).await;
        let task_runner =
            add_vid_task(task_runner, internal_event_stream.clone(), handle.clone()).await;
        let task_runner =
            add_transaction_task(task_runner, internal_event_stream.clone(), handle.clone()).await;
        let task_runner =
            add_view_sync_task(task_runner, internal_event_stream.clone(), handle.clone()).await;
        async_spawn(async move {
            task_runner.launch().await;
            info!("Task runner exited!");
        });
        handle
    }
}

/// A handle that exposes the interface that hotstuff needs to interact with a [`SystemContextInner`]
#[derive(Clone, Debug)]
pub struct HotShotConsensusApi<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Reference to the [`SystemContextInner`]
    pub inner: Arc<SystemContextInner<TYPES, I>>,
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> ConsensusApi<TYPES, I>
    for HotShotConsensusApi<TYPES, I>
{
    fn total_nodes(&self) -> NonZeroUsize {
        self.inner.config.total_nodes
    }

    fn propose_min_round_time(&self) -> Duration {
        self.inner.config.propose_min_round_time
    }

    fn propose_max_round_time(&self) -> Duration {
        self.inner.config.propose_max_round_time
    }

    fn max_transactions(&self) -> NonZeroUsize {
        self.inner.config.max_transactions
    }

    fn min_transactions(&self) -> usize {
        self.inner.config.min_transactions
    }

    async fn send_event(&self, event: Event<TYPES>) {
        debug!(?event, "send_event");
        let mut event_sender = self.inner.event_sender.write().await;
        if let Some(sender) = &*event_sender {
            if let Err(e) = sender.send_async(event).await {
                error!(?e, "Could not send event to event_sender");
                *event_sender = None;
            }
        }
    }

    fn public_key(&self) -> &TYPES::SignatureKey {
        &self.inner.public_key
    }

    fn private_key(&self) -> &<TYPES::SignatureKey as SignatureKey>::PrivateKey {
        &self.inner.private_key
    }

    async fn store_leaf(
        &self,
        old_anchor_view: TYPES::Time,
        leaf: Leaf<TYPES>,
    ) -> std::result::Result<(), hotshot_types::traits::storage::StorageError> {
        let view_to_insert = StoredView::from(leaf);
        let storage = &self.inner.storage;
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
}

impl<TYPES: NodeType> HotShotInitializer<TYPES> {
    /// initialize from genesis
    /// # Errors
    /// If we are unable to apply the genesis block to the default state
    pub fn from_genesis() -> Result<Self, HotShotError<TYPES>> {
        Ok(Self {
            inner: Leaf::genesis(),
        })
    }

    /// reload previous state based on most recent leaf
    pub fn from_reload(anchor_leaf: Leaf<TYPES>) -> Self {
        Self { inner: anchor_leaf }
    }
}
