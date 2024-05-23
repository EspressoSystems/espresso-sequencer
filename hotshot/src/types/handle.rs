//! Provides an event-streaming handle for a [`SystemContext`] running in the background

use std::sync::Arc;

use async_broadcast::{InactiveReceiver, Receiver, Sender};
use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use futures::Stream;
use hotshot_task::task::TaskRegistry;
use hotshot_task_impls::{events::HotShotEvent, helpers::broadcast_event};
use hotshot_types::{
    boxed_sync,
    consensus::Consensus,
    data::Leaf,
    error::HotShotError,
    traits::{election::Membership, node_implementation::NodeType},
    BoxSyncFuture,
};
use std::time::Duration;
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;

use crate::{traits::NodeImplementation, types::Event, SystemContext};

/// Event streaming handle for a [`SystemContext`] instance running in the background
///
/// This type provides the means to message and interact with a background [`SystemContext`] instance,
/// allowing the ability to receive [`Event`]s from it, send transactions to it, and interact with
/// the underlying storage.
#[derive(Clone)]
pub struct SystemContextHandle<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// The [sender](Sender) and [receiver](Receiver),
    /// to allow the application to communicate with HotShot.
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
    pub fn event_stream(&self) -> impl Stream<Item = Event<TYPES>> {
        self.output_event_stream.1.activate_cloned()
    }

    /// HACK so we can know the types when running tests...
    /// there are two cleaner solutions:
    /// - make the stream generic and in nodetypes or nodeimpelmentation
    /// - type wrapper
    #[must_use]
    pub fn event_stream_known_impl(&self) -> Receiver<Event<TYPES>> {
        self.output_event_stream.1.activate_cloned()
    }

    /// HACK so we can know the types when running tests...
    /// there are two cleaner solutions:
    /// - make the stream generic and in nodetypes or nodeimpelmentation
    /// - type wrapper
    /// NOTE: this is only used for sanity checks in our tests
    #[must_use]
    pub fn internal_event_stream_known_impl(&self) -> Receiver<Arc<HotShotEvent<TYPES>>> {
        self.internal_event_stream.1.activate_cloned()
    }

    /// Get the last decided validated state of the [`SystemContext`] instance.
    ///
    /// # Panics
    /// If the internal consensus is in an inconsistent state.
    pub async fn decided_state(&self) -> Arc<TYPES::ValidatedState> {
        self.hotshot.decided_state().await
    }

    /// Get the validated state from a given `view`.
    ///
    /// Returns the requested state, if the [`SystemContext`] is tracking this view. Consensus
    /// tracks views that have not yet been decided but could be in the future. This function may
    /// return [`None`] if the requested view has already been decided (but see
    /// [`decided_state`](Self::decided_state)) or if there is no path for the requested
    /// view to ever be decided.
    pub async fn state(&self, view: TYPES::Time) -> Option<Arc<TYPES::ValidatedState>> {
        self.hotshot.state(view).await
    }

    /// Get the last decided leaf of the [`SystemContext`] instance.
    ///
    /// # Panics
    /// If the internal consensus is in an inconsistent state.
    pub async fn decided_leaf(&self) -> Leaf<TYPES> {
        self.hotshot.decided_leaf().await
    }

    /// Tries to get the most recent decided leaf, returning instantly
    /// if we can't acquire the lock.
    ///
    /// # Panics
    /// Panics if internal consensus is in an inconsistent state.
    #[must_use]
    pub fn try_decided_leaf(&self) -> Option<Leaf<TYPES>> {
        self.hotshot.try_decided_leaf()
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
    pub fn consensus(&self) -> Arc<RwLock<Consensus<TYPES>>> {
        self.hotshot.consensus()
    }

    /// Block the underlying quorum (and DA) networking interfaces until node is
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
            // this is required because `SystemContextHandle` holds an inactive receiver and
            // `broadcast_direct` below can wait indefinitely
            self.internal_event_stream.0.set_await_active(false);
            let _ = self
                .internal_event_stream
                .0
                .broadcast_direct(Arc::new(HotShotEvent::Shutdown))
                .await;
            self.registry.shutdown().await;
        })
    }

    /// return the timeout for a view of the underlying `SystemContext`
    #[must_use]
    pub fn next_view_timeout(&self) -> u64 {
        self.hotshot.next_view_timeout()
    }

    /// Wrapper for `HotShotConsensusApi`'s `leader` function
    #[allow(clippy::unused_async)] // async for API compatibility reasons
    pub async fn leader(&self, view_number: TYPES::Time) -> TYPES::SignatureKey {
        self.hotshot
            .memberships
            .quorum_membership
            .leader(view_number)
    }

    // Below is for testing only:
    /// Wrapper to get this node's public key
    #[cfg(feature = "hotshot-testing")]
    #[must_use]
    pub fn public_key(&self) -> TYPES::SignatureKey {
        self.hotshot.public_key.clone()
    }

    /// Wrapper to get the view number this node is on.
    pub async fn cur_view(&self) -> TYPES::Time {
        self.hotshot.consensus.read().await.cur_view()
    }

    /// Provides a reference to the underlying storage for this [`SystemContext`], allowing access to
    /// historical data
    #[must_use]
    pub fn storage(&self) -> Arc<RwLock<I::Storage>> {
        Arc::clone(&self.storage)
    }

    /// A helper function to spawn the initial timeout task from a given `SystemContextHandle`.
    #[must_use]
    pub fn spawn_initial_timeout_task(&self) -> JoinHandle<()> {
        // Clone the event stream that we send the timeout event to
        let event_stream = self.internal_event_stream.0.clone();
        let next_view_timeout = self.hotshot.config.next_view_timeout;
        let start_view = self.hotshot.start_view;

        // Spawn a task that will sleep for the next view timeout and then send a timeout event
        // if not cancelled
        async_spawn({
            async move {
                async_sleep(Duration::from_millis(next_view_timeout)).await;
                broadcast_event(
                    Arc::new(HotShotEvent::Timeout(start_view + 1)),
                    &event_stream,
                )
                .await;
            }
        })
    }
}
