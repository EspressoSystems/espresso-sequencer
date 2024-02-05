//! Provides an event-streaming handle for a [`SystemContext`] running in the background

use crate::{traits::NodeImplementation, types::Event, SystemContext};
use async_compatibility_layer::channel::UnboundedStream;
use async_lock::RwLock;
use futures::Stream;
use hotshot_task::{
    boxed_sync,
    event_stream::{ChannelStream, EventStream, StreamId},
    global_registry::GlobalRegistry,
    task::FilterEvent,
    BoxSyncFuture,
};
use hotshot_task_impls::events::HotShotEvent;
#[cfg(feature = "hotshot-testing")]
use hotshot_types::traits::election::Membership;

use hotshot_types::{
    consensus::Consensus, data::Leaf, error::HotShotError, traits::node_implementation::NodeType,
};
use std::sync::Arc;

/// Event streaming handle for a [`SystemContext`] instance running in the background
///
/// This type provides the means to message and interact with a background [`SystemContext`] instance,
/// allowing the ability to receive [`Event`]s from it, send transactions to it, and interact with
/// the underlying storage.
pub struct SystemContextHandle<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// The [sender](ChannelStream) for the output stream from the background process
    pub(crate) output_event_stream: ChannelStream<Event<TYPES>>,
    /// access to the internal ev ent stream, in case we need to, say, shut something down
    pub(crate) internal_event_stream: ChannelStream<HotShotEvent<TYPES>>,
    /// registry for controlling tasks
    pub(crate) registry: GlobalRegistry,

    /// Internal reference to the underlying [`SystemContext`]
    pub hotshot: SystemContext<TYPES, I>,

    /// Our copy of the `Storage` view for a hotshot
    pub(crate) storage: I::Storage,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES> + 'static> Clone
    for SystemContextHandle<TYPES, I>
{
    fn clone(&self) -> Self {
        Self {
            registry: self.registry.clone(),
            output_event_stream: self.output_event_stream.clone(),
            internal_event_stream: self.internal_event_stream.clone(),
            hotshot: self.hotshot.clone(),
            storage: self.storage.clone(),
        }
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES> + 'static> SystemContextHandle<TYPES, I> {
    /// obtains a stream to expose to the user
    pub async fn get_event_stream(
        &mut self,
        filter: FilterEvent<Event<TYPES>>,
    ) -> (impl Stream<Item = Event<TYPES>>, StreamId) {
        self.output_event_stream.subscribe(filter).await
    }

    /// HACK so we can know the types when running tests...
    /// there are two cleaner solutions:
    /// - make the stream generic and in nodetypes or nodeimpelmentation
    /// - type wrapper
    pub async fn get_event_stream_known_impl(
        &mut self,
        filter: FilterEvent<Event<TYPES>>,
    ) -> (UnboundedStream<Event<TYPES>>, StreamId) {
        self.output_event_stream.subscribe(filter).await
    }

    /// HACK so we can know the types when running tests...
    /// there are two cleaner solutions:
    /// - make the stream generic and in nodetypes or nodeimpelmentation
    /// - type wrapper
    /// NOTE: this is only used for sanity checks in our tests
    pub async fn get_internal_event_stream_known_impl(
        &mut self,
        filter: FilterEvent<HotShotEvent<TYPES>>,
    ) -> (UnboundedStream<HotShotEvent<TYPES>>, StreamId) {
        self.internal_event_stream.subscribe(filter).await
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

    /// Provides a reference to the underlying storage for this [`SystemContext`], allowing access to
    /// historical data
    pub fn storage(&self) -> &I::Storage {
        &self.storage
    }

    /// Get the underlying consensus state for this [`SystemContext`]
    pub fn get_consensus(&self) -> Arc<RwLock<Consensus<TYPES>>> {
        self.hotshot.get_consensus()
    }

    /// Block the underlying quorum (and committee) networking interfaces until node is
    /// successfully initialized into the networks.
    pub async fn wait_for_networks_ready(&self) {
        self.hotshot.inner.networks.wait_for_networks_ready().await;
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
            self.hotshot.inner.networks.shut_down_networks().await;
            self.registry.shutdown_all().await;
        })
    }

    /// return the timeout for a view of the underlying `SystemContext`
    pub fn get_next_view_timeout(&self) -> u64 {
        self.hotshot.get_next_view_timeout()
    }

    // Below is for testing only:

    /// Wrapper for `HotShotConsensusApi`'s `get_leader` function
    #[allow(clippy::unused_async)] // async for API compatibility reasons
    #[cfg(feature = "hotshot-testing")]
    pub async fn get_leader(&self, view_number: TYPES::Time) -> TYPES::SignatureKey {
        self.hotshot
            .inner
            .memberships
            .quorum_membership
            .get_leader(view_number)
    }

    /// Wrapper to get this node's public key
    #[cfg(feature = "hotshot-testing")]
    pub fn get_public_key(&self) -> TYPES::SignatureKey {
        self.hotshot.inner.public_key.clone()
    }

    /// Wrapper to get this node's current view
    #[cfg(feature = "hotshot-testing")]
    pub async fn get_current_view(&self) -> TYPES::Time {
        self.hotshot.inner.consensus.read().await.cur_view
    }
}
