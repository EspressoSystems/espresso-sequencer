//! Networking Implementation that has a primary and a fallback network.  If the primary
//! Errors we will use the backup to send or receive
use super::{push_cdn_network::PushCdnNetwork, NetworkError};
use crate::traits::implementations::Libp2pNetwork;
use async_lock::RwLock;
use hotshot_types::constants::{
    COMBINED_NETWORK_CACHE_SIZE, COMBINED_NETWORK_MIN_PRIMARY_FAILURES,
    COMBINED_NETWORK_PRIMARY_CHECK_INTERVAL,
};

use lru::LruCache;
use std::{
    collections::BTreeSet,
    hash::Hasher,
    num::NonZeroUsize,
    sync::atomic::{AtomicU64, Ordering},
};
use tracing::warn;

use async_trait::async_trait;

use futures::{channel::mpsc, join, select, FutureExt};

use async_compatibility_layer::channel::UnboundedSendError;
#[cfg(feature = "hotshot-testing")]
use hotshot_types::traits::network::{
    AsyncGenerator, NetworkReliability, TestableNetworkingImplementation,
};
use hotshot_types::{
    boxed_sync,
    data::ViewNumber,
    message::{Message, SequencingMessage},
    traits::{
        network::{ConnectedNetwork, ConsensusIntentEvent, ResponseChannel, ResponseMessage},
        node_implementation::NodeType,
    },
    BoxSyncFuture,
};
use std::collections::{BTreeMap, HashMap};
use std::future::Future;
use std::{collections::hash_map::DefaultHasher, sync::Arc};

use async_compatibility_layer::art::{async_sleep, async_spawn};
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use futures::future::join_all;
use hotshot_task_impls::helpers::cancel_task;
use hotshot_types::message::{GeneralConsensusMessage, MessageKind};
use hotshot_types::traits::network::ViewMessage;
use hotshot_types::traits::node_implementation::ConsensusTime;
use std::hash::Hash;
use std::time::Duration;
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use versioned_binary_serialization::version::StaticVersionType;

/// Helper function to calculate a hash of a type that implements Hash
pub fn calculate_hash_of<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// thread-safe ref counted lock to a map of delayed tasks
type DelayedTasksLockedMap = Arc<RwLock<BTreeMap<u64, Vec<JoinHandle<Result<(), NetworkError>>>>>>;

/// A communication channel with 2 networks, where we can fall back to the slower network if the
/// primary fails
#[derive(Clone)]
pub struct CombinedNetworks<TYPES: NodeType> {
    /// The two networks we'll use for send/recv
    networks: Arc<UnderlyingCombinedNetworks<TYPES>>,

    /// Last n seen messages to prevent processing duplicates
    message_cache: Arc<RwLock<LruCache<u64, ()>>>,

    /// If the primary network is down (0) or not, and for how many messages
    primary_down: Arc<AtomicU64>,

    /// delayed, cancelable tasks for secondary network
    delayed_tasks: DelayedTasksLockedMap,

    /// how long to delay
    delay_duration: Arc<RwLock<Duration>>,
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
        secondary_network: Libp2pNetwork<Message<TYPES>, TYPES::SignatureKey>,
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
            primary_down: Arc::new(AtomicU64::new(0)),
            delayed_tasks: Arc::default(),
            delay_duration: Arc::new(RwLock::new(delay_duration)),
        }
    }

    /// Get a ref to the primary network
    #[must_use]
    pub fn primary(&self) -> &PushCdnNetwork<TYPES> {
        &self.networks.0
    }

    /// Get a ref to the backup network
    #[must_use]
    pub fn secondary(&self) -> &Libp2pNetwork<Message<TYPES>, TYPES::SignatureKey> {
        &self.networks.1
    }

    /// a helper function returning a bool whether a given message is of delayable type
    fn should_delay(message: &Message<TYPES>) -> bool {
        match &message.kind {
            MessageKind::Consensus(consensus_message) => match &consensus_message {
                SequencingMessage::General(general_consensus_message) => {
                    matches!(general_consensus_message, GeneralConsensusMessage::Vote(_))
                }
                SequencingMessage::Committee(_) => true,
            },
            MessageKind::Data(_) => false,
        }
    }

    /// a helper function to send messages through both networks (possibly delayed)
    async fn send_both_networks(
        &self,
        message: Message<TYPES>,
        primary_future: impl Future<Output = Result<(), NetworkError>> + Send + 'static,
        secondary_future: impl Future<Output = Result<(), NetworkError>> + Send + 'static,
    ) -> Result<(), NetworkError> {
        // send optimistically on both networks, but if the primary network is down, skip it
        let primary_down = self.primary_down.load(Ordering::Relaxed);
        let mut primary_failed = false;
        if primary_down < COMBINED_NETWORK_MIN_PRIMARY_FAILURES
            || primary_down % COMBINED_NETWORK_PRIMARY_CHECK_INTERVAL == 0
        {
            // send on the primary network as it is not down, or we are checking if it is back up
            match primary_future.await {
                Ok(()) => {
                    self.primary_down.store(0, Ordering::Relaxed);
                }
                Err(e) => {
                    warn!("Error on primary network: {}", e);
                    self.primary_down.fetch_add(1, Ordering::Relaxed);
                    primary_failed = true;
                }
            };
        }

        if !primary_failed && Self::should_delay(&message) {
            let duration = *self.delay_duration.read().await;
            self.delayed_tasks
                .write()
                .await
                .entry(message.kind.get_view_number().get_u64())
                .or_default()
                .push(async_spawn(async move {
                    async_sleep(duration).await;
                    secondary_future.await
                }));
            Ok(())
        } else {
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
    pub Libp2pNetwork<Message<TYPES>, TYPES::SignatureKey>,
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
    ) -> AsyncGenerator<(Arc<Self>, Arc<Self>)> {
        let generators = (
            <PushCdnNetwork<TYPES> as TestableNetworkingImplementation<_>>::generator(
                expected_node_count,
                num_bootstrap,
                network_id,
                da_committee_size,
                is_da,
                None,
                Duration::default(),
            ),
            <Libp2pNetwork<Message<TYPES>, TYPES::SignatureKey> as TestableNetworkingImplementation<_>>::generator(
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
                let (cdn, _) = gen0.await;
                let cdn = Arc::<PushCdnNetwork<TYPES>>::into_inner(cdn).unwrap();

                let (quorum_p2p, da_p2p) = gen1.await;
                let da_networks = UnderlyingCombinedNetworks(
                    cdn.clone(),
                    Arc::<Libp2pNetwork<Message<TYPES>, TYPES::SignatureKey>>::unwrap_or_clone(
                        da_p2p,
                    ),
                );
                let quorum_networks = UnderlyingCombinedNetworks(
                    cdn,
                    Arc::<Libp2pNetwork<Message<TYPES>, TYPES::SignatureKey>>::unwrap_or_clone(
                        quorum_p2p,
                    ),
                );
                let quorum_net = Self {
                    networks: Arc::new(quorum_networks),
                    message_cache: Arc::new(RwLock::new(LruCache::new(
                        NonZeroUsize::new(COMBINED_NETWORK_CACHE_SIZE).unwrap(),
                    ))),
                    primary_down: Arc::new(AtomicU64::new(0)),
                    delayed_tasks: Arc::default(),
                    delay_duration: Arc::new(RwLock::new(secondary_network_delay)),
                };
                let da_net = Self {
                    networks: Arc::new(da_networks),
                    message_cache: Arc::new(RwLock::new(LruCache::new(
                        NonZeroUsize::new(COMBINED_NETWORK_CACHE_SIZE).unwrap(),
                    ))),
                    primary_down: Arc::new(AtomicU64::new(0)),
                    delayed_tasks: Arc::default(),
                    delay_duration: Arc::new(RwLock::new(secondary_network_delay)),
                };
                (quorum_net.into(), da_net.into())
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
impl<TYPES: NodeType> ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>
    for CombinedNetworks<TYPES>
{
    async fn request_data<T: NodeType, VER: 'static + StaticVersionType>(
        &self,
        request: Message<TYPES>,
        recipient: TYPES::SignatureKey,
        bind_version: VER,
    ) -> Result<ResponseMessage<T>, NetworkError> {
        self.secondary()
            .request_data(request, recipient, bind_version)
            .await
    }

    async fn spawn_request_receiver_task<VER: 'static + StaticVersionType>(
        &self,
        bind_version: VER,
    ) -> Option<mpsc::Receiver<(Message<TYPES>, ResponseChannel<Message<TYPES>>)>> {
        self.secondary()
            .spawn_request_receiver_task(bind_version)
            .await
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

    async fn broadcast_message<VER: StaticVersionType + 'static>(
        &self,
        message: Message<TYPES>,
        recipients: BTreeSet<TYPES::SignatureKey>,
        bind_version: VER,
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
                    .broadcast_message(primary_message, primary_recipients, bind_version)
                    .await
            },
            async move {
                secondary
                    .broadcast_message(secondary_message, recipients, bind_version)
                    .await
            },
        )
        .await
    }

    async fn da_broadcast_message<VER: StaticVersionType + 'static>(
        &self,
        message: Message<TYPES>,
        recipients: BTreeSet<TYPES::SignatureKey>,
        bind_version: VER,
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
                    .da_broadcast_message(primary_message, primary_recipients, bind_version)
                    .await
            },
            async move {
                secondary
                    .da_broadcast_message(secondary_message, recipients, bind_version)
                    .await
            },
        )
        .await
    }

    async fn direct_message<VER: StaticVersionType + 'static>(
        &self,
        message: Message<TYPES>,
        recipient: TYPES::SignatureKey,
        bind_version: VER,
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
                    .direct_message(primary_message, primary_recipient, bind_version)
                    .await
            },
            async move {
                secondary
                    .direct_message(secondary_message, recipient, bind_version)
                    .await
            },
        )
        .await
    }

    async fn vid_broadcast_message<VER: StaticVersionType + 'static>(
        &self,
        messages: HashMap<TYPES::SignatureKey, Message<TYPES>>,
        bind_version: VER,
    ) -> Result<(), NetworkError> {
        self.networks
            .0
            .vid_broadcast_message(messages, bind_version)
            .await
    }

    /// Receive one or many messages from the underlying network.
    ///
    /// # Errors
    /// Does not error
    async fn recv_msgs(&self) -> Result<Vec<Message<TYPES>>, NetworkError> {
        // recv on both networks because nodes may be accessible only on either. discard duplicates
        // TODO: improve this algorithm: https://github.com/EspressoSystems/HotShot/issues/2089
        let mut primary_fut = self.primary().recv_msgs().fuse();
        let mut secondary_fut = self.secondary().recv_msgs().fuse();

        let msgs = select! {
            p = primary_fut => p?,
            s = secondary_fut => s?,
        };

        let mut filtered_msgs = Vec::with_capacity(msgs.len());
        for msg in msgs {
            // see if we've already seen this message
            if !self
                .message_cache
                .read()
                .await
                .contains(&calculate_hash_of(&msg))
            {
                filtered_msgs.push(msg.clone());
                self.message_cache
                    .write()
                    .await
                    .put(calculate_hash_of(&msg), ());
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

    async fn inject_consensus_info(&self, event: ConsensusIntentEvent<TYPES::SignatureKey>) {
        <PushCdnNetwork<_> as ConnectedNetwork<
            Message<TYPES>,
            TYPES::SignatureKey,
        >>::inject_consensus_info(self.primary(), event.clone())
        .await;

        <Libp2pNetwork<_, _> as ConnectedNetwork<Message<TYPES>,TYPES::SignatureKey>>::
            inject_consensus_info(self.secondary(), event).await;
    }

    fn update_view(&self, view: u64) {
        let delayed_map = self.delayed_tasks.clone();
        async_spawn(async move {
            let mut cancel_tasks = Vec::new();
            {
                let mut map_lock = delayed_map.write().await;
                while let Some((first_view, _tasks)) = map_lock.first_key_value() {
                    if *first_view < view {
                        if let Some((_view, tasks)) = map_lock.pop_first() {
                            let mut ctasks = tasks.into_iter().map(cancel_task).collect();
                            cancel_tasks.append(&mut ctasks);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            join_all(cancel_tasks).await;
        });
    }
}
