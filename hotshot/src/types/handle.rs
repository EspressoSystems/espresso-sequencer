//! Provides an event-streaming handle for a [`SystemContext`] running in the background

use std::sync::Arc;

use async_broadcast::{InactiveReceiver, Receiver, Sender};
use async_lock::RwLock;
use futures::Stream;
use hotshot_task::task::TaskRegistry;
use hotshot_task_impls::events::HotShotEvent;
use hotshot_types::{
    boxed_sync,
    consensus::Consensus,
    data::Leaf,
    error::HotShotError,
    traits::{election::Membership, node_implementation::NodeType},
    BoxSyncFuture,
};

use crate::{traits::NodeImplementation, types::Event, SystemContext};

/// Event streaming handle for a [`SystemContext`] instance running in the background
///
/// This type provides the means to message and interact with a background [`SystemContext`] instance,
/// allowing the ability to receive [`Event`]s from it, send transactions to it, and interact with
/// the underlying storage.
#[derive(Clone)]
pub struct SystemContextHandle<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// The [sender](Sender) and an `InactiveReceiver` to keep the channel open.
    /// The Channel will output all the events.  Subscribers will get an activated
    /// clone of the `Receiver` when they get output stream.
    pub(crate) output_event_stream: (Sender<Event<TYPES>>, InactiveReceiver<Event<TYPES>>),

    /// access to the internal event stream, in case we need to, say, shut something down
    #[allow(clippy::type_complexity)]
    pub(crate) internal_event_stream: (
        Sender<Arc<HotShotEvent<TYPES>>>,
        InactiveReceiver<Arc<HotShotEvent<TYPES>>>,
    ),
    /// registry for controlling tasks
    pub(crate) registry: Arc<TaskRegistry>,

    /// Internal reference to the underlying [`SystemContext`]
    pub hotshot: Arc<SystemContext<TYPES, I>>,

    /// Reference to the internal storage for consensus datum.
    pub(crate) storage: Arc<RwLock<I::Storage>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES> + 'static> SystemContextHandle<TYPES, I> {
    /// obtains a stream to expose to the user
    pub fn get_event_stream(&self) -> impl Stream<Item = Event<TYPES>> {
        self.output_event_stream.1.activate_cloned()
    }

    /// HACK so we can know the types when running tests...
    /// there are two cleaner solutions:
    /// - make the stream generic and in nodetypes or nodeimpelmentation
    /// - type wrapper
    #[must_use]
    pub fn get_event_stream_known_impl(&self) -> Receiver<Event<TYPES>> {
        self.output_event_stream.1.activate_cloned()
    }

    /// HACK so we can know the types when running tests...
    /// there are two cleaner solutions:
    /// - make the stream generic and in nodetypes or nodeimpelmentation
    /// - type wrapper
    /// NOTE: this is only used for sanity checks in our tests
    #[must_use]
    pub fn get_internal_event_stream_known_impl(&self) -> Receiver<Arc<HotShotEvent<TYPES>>> {
        self.internal_event_stream.1.activate_cloned()
    }

    /// Get the last decided validated state of the [`SystemContext`] instance.
    ///
    /// # Panics
    /// If the internal consensus is in an inconsistent state.
    pub async fn get_decided_state(&self) -> Arc<TYPES::ValidatedState> {
        self.hotshot.get_decided_state().await
    }

    /// Get the validated state from a given `view`.
    ///
    /// Returns the requested state, if the [`SystemContext`] is tracking this view. Consensus
    /// tracks views that have not yet been decided but could be in the future. This function may
    /// return [`None`] if the requested view has already been decided (but see
    /// [`get_decided_state`](Self::get_decided_state)) or if there is no path for the requested
    /// view to ever be decided.
    pub async fn get_state(&self, view: TYPES::Time) -> Option<Arc<TYPES::ValidatedState>> {
        self.hotshot.get_state(view).await
    }

    /// Get the last decided leaf of the [`SystemContext`] instance.
    ///
    /// # Panics
    /// If the internal consensus is in an inconsistent state.
    pub async fn get_decided_leaf(&self) -> Leaf<TYPES> {
        self.hotshot.get_decided_leaf().await
    }

    /// Tries to get the most recent decided leaf, returning instantly
    /// if we can't acquire the lock.
    ///
    /// # Panics
    /// Panics if internal consensus is in an inconsistent state.
    #[must_use]
    pub fn try_get_decided_leaf(&self) -> Option<Leaf<TYPES>> {
        self.hotshot.try_get_decided_leaf()
    }

    /// Submits a transaction to the backing [`SystemContext`] instance.
    ///
    /// The current node broadcasts the transaction to all nodes on the network.
    ///
    /// # Errors
    ///
    /// Will return a [`HotShotError`] if some error occurs in the underlying
    /// [`SystemContext`] instance.
    pub async fn submit_transaction(
        &self,
        tx: TYPES::Transaction,
    ) -> Result<(), HotShotError<TYPES>> {
        self.hotshot.publish_transaction_async(tx).await
    }

    /// Get the underlying consensus state for this [`SystemContext`]
    #[must_use]
    pub fn get_consensus(&self) -> Arc<RwLock<Consensus<TYPES>>> {
        self.hotshot.get_consensus()
    }

    /// Block the underlying quorum (and committee) networking interfaces until node is
    /// successfully initialized into the networks.
    pub async fn wait_for_networks_ready(&self) {
        self.hotshot.networks.wait_for_networks_ready().await;
    }

    /// Shut down the the inner hotshot and wait until all background threads are closed.
    //     pub async fn shut_down(mut self) {
    //         self.registry.shutdown_all().await
    pub fn shut_down<'a, 'b>(&'a mut self) -> BoxSyncFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b,
    {
        boxed_sync(async move {
            self.hotshot.networks.shut_down_networks().await;
            self.registry.shutdown().await;
        })
    }

    /// return the timeout for a view of the underlying `SystemContext`
    #[must_use]
    pub fn get_next_view_timeout(&self) -> u64 {
        self.hotshot.get_next_view_timeout()
    }

    /// Wrapper for `HotShotConsensusApi`'s `get_leader` function
    #[allow(clippy::unused_async)] // async for API compatibility reasons
    pub async fn get_leader(&self, view_number: TYPES::Time) -> TYPES::SignatureKey {
        self.hotshot
            .memberships
            .quorum_membership
            .get_leader(view_number)
    }

    // Below is for testing only:
    /// Wrapper to get this node's public key
    #[cfg(feature = "hotshot-testing")]
    #[must_use]
    pub fn get_public_key(&self) -> TYPES::SignatureKey {
        self.hotshot.public_key.clone()
    }

    /// Wrapper to get the view number this node is on.
    pub async fn get_cur_view(&self) -> TYPES::Time {
        self.hotshot.consensus.read().await.cur_view
    }

    /// Provides a reference to the underlying storage for this [`SystemContext`], allowing access to
    /// historical data
    #[must_use]
    pub fn get_storage(&self) -> Arc<RwLock<I::Storage>> {
        self.storage.clone()
    }
}
