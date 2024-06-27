//! Networking Implementation that has a primary and a fallback network.  If the primary
//! Errors we will use the backup to send or receive
use std::{
    collections::{hash_map::DefaultHasher, BTreeMap, BTreeSet, HashMap},
    future::Future,
    hash::{Hash, Hasher},
    num::NonZeroUsize,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use async_broadcast::{broadcast, InactiveReceiver, Sender};
use async_compatibility_layer::{
    art::{async_sleep, async_spawn},
    channel::UnboundedSendError,
};
use async_lock::RwLock;
use async_trait::async_trait;
use futures::{channel::mpsc, join, select, FutureExt};
#[cfg(feature = "hotshot-testing")]
use hotshot_types::traits::network::{
    AsyncGenerator, NetworkReliability, TestableNetworkingImplementation,
};
use hotshot_types::{
    boxed_sync,
    constants::{
        COMBINED_NETWORK_CACHE_SIZE, COMBINED_NETWORK_MIN_PRIMARY_FAILURES,
        COMBINED_NETWORK_PRIMARY_CHECK_INTERVAL,
    },
    data::ViewNumber,
    traits::{
        network::{BroadcastDelay, ConnectedNetwork, ResponseChannel},
        node_implementation::NodeType,
    },
    BoxSyncFuture,
};
use lru::LruCache;
use tracing::{debug, warn};

use super::{push_cdn_network::PushCdnNetwork, NetworkError};
use crate::traits::implementations::Libp2pNetwork;

/// Helper function to calculate a hash of a type that implements Hash
pub fn calculate_hash_of<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// Thread-safe ref counted lock to a map of channels to the delayed tasks
type DelayedTasksChannelsMap = Arc<RwLock<BTreeMap<u64, (Sender<()>, InactiveReceiver<()>)>>>;

/// A communication channel with 2 networks, where we can fall back to the slower network if the
/// primary fails
#[derive(Clone)]
pub struct CombinedNetworks<TYPES: NodeType> {
    /// The two networks we'll use for send/recv
    networks: Arc<UnderlyingCombinedNetworks<TYPES>>,

    /// Last n seen messages to prevent processing duplicates
    message_cache: Arc<RwLock<LruCache<u64, ()>>>,

    /// How many times primary failed to deliver
    primary_fail_counter: Arc<AtomicU64>,

    /// Whether primary is considered down
    primary_down: Arc<AtomicBool>,

    /// How long to delay
    delay_duration: Arc<RwLock<Duration>>,

    /// Channels to the delayed tasks
    delayed_tasks_channels: DelayedTasksChannelsMap,

    /// How many times messages were sent on secondary without delay because primary is down
    no_delay_counter: Arc<AtomicU64>,
}

impl<TYPES: NodeType> CombinedNetworks<TYPES> {
    /// Constructor
    ///
    /// # Panics
    ///
    /// Panics if `COMBINED_NETWORK_CACHE_SIZE` is 0
    #[must_use]
    pub fn new(
        primary_network: PushCdnNetwork<TYPES>,
        secondary_network: Libp2pNetwork<TYPES::SignatureKey>,
        delay_duration: Duration,
    ) -> Self {
        // Create networks from the ones passed in
        let networks = Arc::from(UnderlyingCombinedNetworks(
            primary_network,
            secondary_network,
        ));

        Self {
            networks,
            message_cache: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(COMBINED_NETWORK_CACHE_SIZE).unwrap(),
            ))),
            primary_fail_counter: Arc::new(AtomicU64::new(0)),
            primary_down: Arc::new(AtomicBool::new(false)),
            delay_duration: Arc::new(RwLock::new(delay_duration)),
            delayed_tasks_channels: Arc::default(),
            no_delay_counter: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Get a ref to the primary network
    #[must_use]
    pub fn primary(&self) -> &PushCdnNetwork<TYPES> {
        &self.networks.0
    }

    /// Get a ref to the backup network
    #[must_use]
    pub fn secondary(&self) -> &Libp2pNetwork<TYPES::SignatureKey> {
        &self.networks.1
    }

    /// a helper function to send messages through both networks (possibly delayed)
    async fn send_both_networks(
        &self,
        _message: Vec<u8>,
        primary_future: impl Future<Output = Result<(), NetworkError>> + Send + 'static,
        secondary_future: impl Future<Output = Result<(), NetworkError>> + Send + 'static,
        broadcast_delay: BroadcastDelay,
    ) -> Result<(), NetworkError> {
        // A local variable used to decide whether to delay this message or not
        let mut primary_failed = false;
        if self.primary_down.load(Ordering::Relaxed) {
            // If the primary is considered down, we don't want to delay
            primary_failed = true;
        } else if self.primary_fail_counter.load(Ordering::Relaxed)
            > COMBINED_NETWORK_MIN_PRIMARY_FAILURES
        {
            // If the primary failed more than `COMBINED_NETWORK_MIN_PRIMARY_FAILURES` times,
            // we don't want to delay this message, and from now on we consider the primary as down
            warn!(
                "Primary failed more than {} times and is considered down now",
                COMBINED_NETWORK_MIN_PRIMARY_FAILURES
            );
            self.primary_down.store(true, Ordering::Relaxed);
            primary_failed = true;
        }

        // Always send on the primary network
        if let Err(e) = primary_future.await {
            // If the primary failed right away, we don't want to delay this message
            warn!("Error on primary network: {}", e);
            self.primary_fail_counter.fetch_add(1, Ordering::Relaxed);
            primary_failed = true;
        };

        if let (BroadcastDelay::View(view), false) = (broadcast_delay, primary_failed) {
            // We are delaying this message
            let duration = *self.delay_duration.read().await;
            let primary_down = Arc::clone(&self.primary_down);
            let primary_fail_counter = Arc::clone(&self.primary_fail_counter);
            // Each delayed task gets its own receiver clone to get a signal cancelling all tasks
            // related to the given view.
            let mut receiver = self
                .delayed_tasks_channels
                .write()
                .await
                .entry(view)
                .or_insert_with(|| {
                    let (s, r) = broadcast(1);
                    (s, r.deactivate())
                })
                .1
                .activate_cloned();
            // Spawn a task that sleeps for `duration` and then sends the message if it wasn't cancelled
            async_spawn(async move {
                async_sleep(duration).await;
                if receiver.try_recv().is_ok() {
                    // The task has been cancelled because the view progressed, it means the primary is working fine
                    debug!(
                        "Not sending on secondary after delay, task was canceled in view update"
                    );
                    match primary_fail_counter.load(Ordering::Relaxed) {
                        0u64 => {
                            // The primary fail counter reached 0, the primary is now considered up
                            primary_down.store(false, Ordering::Relaxed);
                            debug!("primary_fail_counter reached zero, primary_down set to false");
                        }
                        c => {
                            // Decrement the primary fail counter
                            primary_fail_counter.store(c - 1, Ordering::Relaxed);
                            debug!("primary_fail_counter set to {:?}", c - 1);
                        }
                    }
                    return Ok(());
                }
                // The task hasn't been cancelled, the primary probably failed.
                // Increment the primary fail counter and send the message.
                debug!("Sending on secondary after delay, message possibly has not reached recipient on primary");
                secondary_future.await
            });
            Ok(())
        } else {
            // We will send without delay
            if self.primary_down.load(Ordering::Relaxed) {
                // If the primary is considered down, we want to periodically delay sending
                // on the secondary to check whether the primary is able to deliver.
                // This message will be sent without delay but the next might be delayed.
                match self.no_delay_counter.load(Ordering::Relaxed) {
                    c if c < COMBINED_NETWORK_PRIMARY_CHECK_INTERVAL => {
                        // Just increment the 'no delay counter'
                        self.no_delay_counter.store(c + 1, Ordering::Relaxed);
                    }
                    _ => {
                        // The 'no delay counter' reached the threshold
                        debug!(
                            "Sent on secondary without delay more than {} times,\
                            try delaying to check primary",
                            COMBINED_NETWORK_PRIMARY_CHECK_INTERVAL
                        );
                        // Reset the 'no delay counter'
                        self.no_delay_counter.store(0u64, Ordering::Relaxed);
                        // The primary is not considered down for the moment
                        self.primary_down.store(false, Ordering::Relaxed);
                        // The primary fail counter is set just below the threshold to delay the next message
                        self.primary_fail_counter
                            .store(COMBINED_NETWORK_MIN_PRIMARY_FAILURES, Ordering::Relaxed);
                    }
                }
            }
            // Send the message
            secondary_future.await
        }
    }
}

/// Wrapper for the tuple of `PushCdnNetwork` and `Libp2pNetwork`
/// We need this so we can impl `TestableNetworkingImplementation`
/// on the tuple
#[derive(Clone)]
pub struct UnderlyingCombinedNetworks<TYPES: NodeType>(
    pub PushCdnNetwork<TYPES>,
    pub Libp2pNetwork<TYPES::SignatureKey>,
);

#[cfg(feature = "hotshot-testing")]
impl<TYPES: NodeType> TestableNetworkingImplementation<TYPES> for CombinedNetworks<TYPES> {
    fn generator(
        expected_node_count: usize,
        num_bootstrap: usize,
        network_id: usize,
        da_committee_size: usize,
        is_da: bool,
        reliability_config: Option<Box<dyn NetworkReliability>>,
        secondary_network_delay: Duration,
    ) -> AsyncGenerator<Arc<Self>> {
        let generators = (
            <PushCdnNetwork<TYPES> as TestableNetworkingImplementation<TYPES>>::generator(
                expected_node_count,
                num_bootstrap,
                network_id,
                da_committee_size,
                is_da,
                None,
                Duration::default(),
            ),
            <Libp2pNetwork<TYPES::SignatureKey> as TestableNetworkingImplementation<TYPES>>::generator(
                expected_node_count,
                num_bootstrap,
                network_id,
                da_committee_size,
                is_da,
                reliability_config,
                Duration::default(),
            )
        );
        Box::pin(move |node_id| {
            let gen0 = generators.0(node_id);
            let gen1 = generators.1(node_id);

            Box::pin(async move {
                // Generate the CDN network
                let cdn = gen0.await;
                let cdn = Arc::<PushCdnNetwork<TYPES>>::into_inner(cdn).unwrap();

                // Generate the p2p network
                let p2p = gen1.await;

                // Combine the two
                let underlying_combined = UnderlyingCombinedNetworks(
                    cdn.clone(),
                    Arc::<Libp2pNetwork<TYPES::SignatureKey>>::unwrap_or_clone(p2p),
                );

                // We want to use the same message cache between the two networks
                let message_cache = Arc::new(RwLock::new(LruCache::new(
                    NonZeroUsize::new(COMBINED_NETWORK_CACHE_SIZE).unwrap(),
                )));

                // Combine the two networks with the same cache
                let combined_network = Self {
                    networks: Arc::new(underlying_combined),
                    primary_fail_counter: Arc::new(AtomicU64::new(0)),
                    primary_down: Arc::new(AtomicBool::new(false)),
                    message_cache: Arc::clone(&message_cache),
                    delay_duration: Arc::new(RwLock::new(secondary_network_delay)),
                    delayed_tasks_channels: Arc::default(),
                    no_delay_counter: Arc::new(AtomicU64::new(0)),
                };

                Arc::new(combined_network)
            })
        })
    }

    /// Get the number of messages in-flight.
    ///
    /// Some implementations will not be able to tell how many messages there are in-flight. These implementations should return `None`.
    fn in_flight_message_count(&self) -> Option<usize> {
        None
    }
}

#[async_trait]
impl<TYPES: NodeType> ConnectedNetwork<TYPES::SignatureKey> for CombinedNetworks<TYPES> {
    async fn request_data<T: NodeType>(
        &self,
        request: Vec<u8>,
        recipient: &TYPES::SignatureKey,
    ) -> Result<Vec<u8>, NetworkError> {
        self.secondary()
            .request_data::<TYPES>(request, recipient)
            .await
    }

    async fn spawn_request_receiver_task(
        &self,
    ) -> Option<mpsc::Receiver<(Vec<u8>, ResponseChannel<Vec<u8>>)>> {
        self.secondary().spawn_request_receiver_task().await
    }

    fn pause(&self) {
        self.networks.0.pause();
    }

    fn resume(&self) {
        self.networks.0.resume();
    }

    async fn wait_for_ready(&self) {
        join!(
            self.primary().wait_for_ready(),
            self.secondary().wait_for_ready()
        );
    }

    fn shut_down<'a, 'b>(&'a self) -> BoxSyncFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            join!(self.primary().shut_down(), self.secondary().shut_down());
        };
        boxed_sync(closure)
    }

    async fn broadcast_message(
        &self,
        message: Vec<u8>,
        recipients: BTreeSet<TYPES::SignatureKey>,
        broadcast_delay: BroadcastDelay,
    ) -> Result<(), NetworkError> {
        let primary = self.primary().clone();
        let secondary = self.secondary().clone();
        let primary_message = message.clone();
        let secondary_message = message.clone();
        let primary_recipients = recipients.clone();
        self.send_both_networks(
            message,
            async move {
                primary
                    .broadcast_message(primary_message, primary_recipients, BroadcastDelay::None)
                    .await
            },
            async move {
                secondary
                    .broadcast_message(secondary_message, recipients, BroadcastDelay::None)
                    .await
            },
            broadcast_delay,
        )
        .await
    }

    async fn da_broadcast_message(
        &self,
        message: Vec<u8>,
        recipients: BTreeSet<TYPES::SignatureKey>,
        broadcast_delay: BroadcastDelay,
    ) -> Result<(), NetworkError> {
        let primary = self.primary().clone();
        let secondary = self.secondary().clone();
        let primary_message = message.clone();
        let secondary_message = message.clone();
        let primary_recipients = recipients.clone();
        self.send_both_networks(
            message,
            async move {
                primary
                    .da_broadcast_message(primary_message, primary_recipients, BroadcastDelay::None)
                    .await
            },
            async move {
                secondary
                    .da_broadcast_message(secondary_message, recipients, BroadcastDelay::None)
                    .await
            },
            broadcast_delay,
        )
        .await
    }

    async fn direct_message(
        &self,
        message: Vec<u8>,
        recipient: TYPES::SignatureKey,
    ) -> Result<(), NetworkError> {
        let primary = self.primary().clone();
        let secondary = self.secondary().clone();
        let primary_message = message.clone();
        let secondary_message = message.clone();
        let primary_recipient = recipient.clone();
        self.send_both_networks(
            message,
            async move {
                primary
                    .direct_message(primary_message, primary_recipient)
                    .await
            },
            async move { secondary.direct_message(secondary_message, recipient).await },
            BroadcastDelay::None,
        )
        .await
    }

    async fn vid_broadcast_message(
        &self,
        messages: HashMap<TYPES::SignatureKey, Vec<u8>>,
    ) -> Result<(), NetworkError> {
        self.networks.0.vid_broadcast_message(messages).await
    }

    /// Receive one or many messages from the underlying network.
    ///
    /// # Errors
    /// Does not error
    async fn recv_msgs(&self) -> Result<Vec<Vec<u8>>, NetworkError> {
        // recv on both networks because nodes may be accessible only on either. discard duplicates
        // TODO: improve this algorithm: https://github.com/EspressoSystems/HotShot/issues/2089
        let mut primary_fut = self.primary().recv_msgs().fuse();
        let mut secondary_fut = self.secondary().recv_msgs().fuse();

        let msgs = select! {
            p = primary_fut => p?,
            s = secondary_fut => s?,
        };

        let mut filtered_msgs = Vec::with_capacity(msgs.len());

        // For each message,
        for msg in msgs {
            // Calculate hash of the message
            let message_hash = calculate_hash_of(&msg);

            // Add the hash to the cache
            if !self.message_cache.read().await.contains(&message_hash) {
                // If the message is not in the cache, process it
                filtered_msgs.push(msg.clone());

                // Add it to the cache
                self.message_cache.write().await.put(message_hash, ());
            }
        }

        Ok(filtered_msgs)
    }

    async fn queue_node_lookup(
        &self,
        view_number: ViewNumber,
        pk: TYPES::SignatureKey,
    ) -> Result<(), UnboundedSendError<Option<(ViewNumber, TYPES::SignatureKey)>>> {
        self.primary()
            .queue_node_lookup(view_number, pk.clone())
            .await?;
        self.secondary().queue_node_lookup(view_number, pk).await
    }

    async fn update_view<'a, T>(&'a self, view: u64, membership: &T::Membership)
    where
        T: NodeType<SignatureKey = TYPES::SignatureKey> + 'a,
    {
        let delayed_tasks_channels = Arc::clone(&self.delayed_tasks_channels);
        async_spawn(async move {
            let mut map_lock = delayed_tasks_channels.write().await;
            while let Some((first_view, _)) = map_lock.first_key_value() {
                // Broadcast a cancelling signal to all the tasks related to each view older than the new one
                if *first_view < view {
                    if let Some((_, (sender, _))) = map_lock.pop_first() {
                        let _ = sender.try_broadcast(());
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        });
        // Run `update_view` logic for the libp2p network
        self.networks.1.update_view::<T>(view, membership).await;
    }

    fn is_primary_down(&self) -> bool {
        self.primary_down.load(Ordering::Relaxed)
    }
}
