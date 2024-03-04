//! Networking Implementation that has a primary and a fallback newtork.  If the primary
//! Errors we will use the backup to send or receive
use super::NetworkError;
use crate::traits::implementations::{Libp2pNetwork, WebServerNetwork};
use async_lock::RwLock;
use hotshot_constants::{
    COMBINED_NETWORK_CACHE_SIZE, COMBINED_NETWORK_MIN_PRIMARY_FAILURES,
    COMBINED_NETWORK_PRIMARY_CHECK_INTERVAL,
};
use std::{
    collections::{BTreeSet, HashSet},
    hash::Hasher,
    sync::atomic::{AtomicU64, Ordering},
};
use tracing::warn;

use async_trait::async_trait;

use futures::join;

use async_compatibility_layer::channel::UnboundedSendError;
#[cfg(feature = "hotshot-testing")]
use hotshot_types::traits::network::{NetworkReliability, TestableNetworkingImplementation};
use hotshot_types::{
    boxed_sync,
    data::ViewNumber,
    message::Message,
    traits::{
        network::{ConnectedNetwork, ConsensusIntentEvent},
        node_implementation::NodeType,
    },
    BoxSyncFuture,
};
use std::{collections::hash_map::DefaultHasher, sync::Arc};

use std::hash::Hash;

/// A cache to keep track of the last n messages we've seen, avoids reprocessing duplicates
/// from multiple networks
#[derive(Clone, Debug)]
pub struct Cache {
    /// The maximum number of items to store in the cache
    capacity: usize,
    /// The cache itself
    inner: HashSet<u64>,
    /// The hashes of the messages in the cache, in order of insertion
    hashes: Vec<u64>,
}

impl Cache {
    /// Create a new cache with the given capacity
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            inner: HashSet::with_capacity(capacity),
            hashes: Vec::with_capacity(capacity),
        }
    }

    /// Insert a hash into the cache
    pub fn insert(&mut self, hash: u64) {
        if self.inner.contains(&hash) {
            return;
        }

        // calculate how much we are over and remove that many elements from the cache. deal with overflow
        let over = (self.hashes.len() + 1).saturating_sub(self.capacity);
        if over > 0 {
            for _ in 0..over {
                let hash = self.hashes.remove(0);
                self.inner.remove(&hash);
            }
        }

        self.inner.insert(hash);
        self.hashes.push(hash);
    }

    /// Check if the cache contains a hash
    #[must_use]
    pub fn contains(&self, hash: u64) -> bool {
        self.inner.contains(&hash)
    }

    /// Get the number of items in the cache
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// True if the cache is empty false otherwise
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

/// Helper function to calculate a hash of a type that implements Hash
pub fn calculate_hash_of<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// A communication channel with 2 networks, where we can fall back to the slower network if the
/// primary fails
#[derive(Clone, Debug)]
pub struct CombinedNetworks<TYPES: NodeType> {
    /// The two networks we'll use for send/recv
    networks: Arc<UnderlyingCombinedNetworks<TYPES>>,

    /// Last n seen messages to prevent processing duplicates
    message_cache: Arc<RwLock<Cache>>,

    /// If the primary network is down (0) or not, and for how many messages
    primary_down: Arc<AtomicU64>,
}

impl<TYPES: NodeType> CombinedNetworks<TYPES> {
    /// Constructor
    #[must_use]
    pub fn new(networks: Arc<UnderlyingCombinedNetworks<TYPES>>) -> Self {
        Self {
            networks,
            message_cache: Arc::new(RwLock::new(Cache::new(COMBINED_NETWORK_CACHE_SIZE))),
            primary_down: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Get a ref to the primary network
    #[must_use]
    pub fn primary(&self) -> &WebServerNetwork<TYPES> {
        &self.networks.0
    }

    /// Get a ref to the backup network
    #[must_use]
    pub fn secondary(&self) -> &Libp2pNetwork<Message<TYPES>, TYPES::SignatureKey> {
        &self.networks.1
    }
}

/// Wrapper for the tuple of `WebServerNetwork` and `Libp2pNetwork`
/// We need this so we can impl `TestableNetworkingImplementation`
/// on the tuple
#[derive(Debug, Clone)]
pub struct UnderlyingCombinedNetworks<TYPES: NodeType>(
    pub WebServerNetwork<TYPES>,
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
    ) -> Box<dyn Fn(u64) -> (Arc<Self>, Arc<Self>) + 'static> {
        let generators = (
            <WebServerNetwork<
                TYPES,
            > as TestableNetworkingImplementation<_>>::generator(
                expected_node_count,
                num_bootstrap,
                network_id,
                da_committee_size,
                is_da,
                None,
            ),
            <Libp2pNetwork<Message<TYPES>, TYPES::SignatureKey> as TestableNetworkingImplementation<_>>::generator(
                expected_node_count,
                num_bootstrap,
                network_id,
                da_committee_size,
                is_da,
                reliability_config,
            )
        );
        Box::new(move |node_id| {
            let (quorum_web, da_web) = generators.0(node_id);
            let (quorum_p2p, da_p2p) = generators.1(node_id);
            let da_networks = UnderlyingCombinedNetworks(
                Arc::<WebServerNetwork<TYPES>>::into_inner(da_web).unwrap(),
                Arc::<Libp2pNetwork<Message<TYPES>, TYPES::SignatureKey>>::unwrap_or_clone(da_p2p),
            );
            let quorum_networks = UnderlyingCombinedNetworks(
                Arc::<WebServerNetwork<TYPES>>::into_inner(quorum_web).unwrap(),
                Arc::<Libp2pNetwork<Message<TYPES>, TYPES::SignatureKey>>::unwrap_or_clone(
                    quorum_p2p,
                ),
            );
            let quorum_net = Self {
                networks: Arc::new(quorum_networks),
                message_cache: Arc::new(RwLock::new(Cache::new(COMBINED_NETWORK_CACHE_SIZE))),
                primary_down: Arc::new(AtomicU64::new(0)),
            };
            let da_net = Self {
                networks: Arc::new(da_networks),
                message_cache: Arc::new(RwLock::new(Cache::new(COMBINED_NETWORK_CACHE_SIZE))),
                primary_down: Arc::new(AtomicU64::new(0)),
            };
            (quorum_net.into(), da_net.into())
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

    async fn is_ready(&self) -> bool {
        self.primary().is_ready().await && self.secondary().is_ready().await
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
        message: Message<TYPES>,
        recipients: BTreeSet<TYPES::SignatureKey>,
    ) -> Result<(), NetworkError> {
        // broadcast optimistically on both networks, but if the primary network is down, skip it
        let primary_down = self.primary_down.load(Ordering::Relaxed);
        if primary_down < COMBINED_NETWORK_MIN_PRIMARY_FAILURES
            || primary_down % COMBINED_NETWORK_PRIMARY_CHECK_INTERVAL == 0
        {
            // broadcast on the primary network as it is not down, or we are checking if it is back up
            match self
                .primary()
                .broadcast_message(message.clone(), recipients.clone())
                .await
            {
                Ok(()) => {
                    self.primary_down.store(0, Ordering::Relaxed);
                }
                Err(e) => {
                    warn!("Error on primary network: {}", e);
                    self.primary_down.fetch_add(1, Ordering::Relaxed);
                }
            };
        }

        self.secondary()
            .broadcast_message(message, recipients)
            .await
    }

    async fn da_broadcast_message(
        &self,
        message: Message<TYPES>,
        recipients: BTreeSet<TYPES::SignatureKey>,
    ) -> Result<(), NetworkError> {
        if self
            .primary()
            .da_broadcast_message(message.clone(), recipients.clone())
            .await
            .is_err()
        {
            warn!("Failed broadcasting DA on primary");
        }
        self.secondary()
            .da_broadcast_message(message, recipients)
            .await
    }

    async fn direct_message(
        &self,
        message: Message<TYPES>,
        recipient: TYPES::SignatureKey,
    ) -> Result<(), NetworkError> {
        // DM optimistically on both networks, but if the primary network is down, skip it
        let primary_down = self.primary_down.load(Ordering::Relaxed);
        if primary_down < COMBINED_NETWORK_MIN_PRIMARY_FAILURES
            || primary_down % COMBINED_NETWORK_PRIMARY_CHECK_INTERVAL == 0
        {
            // message on the primary network as it is not down, or we are checking if it is back up
            match self
                .primary()
                .direct_message(message.clone(), recipient.clone())
                .await
            {
                Ok(()) => {
                    self.primary_down.store(0, Ordering::Relaxed);
                }
                Err(e) => {
                    warn!("Error on primary network: {}", e);
                    self.primary_down.fetch_add(1, Ordering::Relaxed);
                }
            };
        }

        self.secondary().direct_message(message, recipient).await
    }

    fn recv_msgs<'a, 'b>(&'a self) -> BoxSyncFuture<'b, Result<Vec<Message<TYPES>>, NetworkError>>
    where
        'a: 'b,
        Self: 'b,
    {
        // recv on both networks because nodes may be accessible only on either. discard duplicates
        // TODO: improve this algorithm: https://github.com/EspressoSystems/HotShot/issues/2089
        let closure = async move {
            let mut primary_msgs = self.primary().recv_msgs().await?;
            let mut secondary_msgs = self.secondary().recv_msgs().await?;

            primary_msgs.append(secondary_msgs.as_mut());

            let mut filtered_msgs = Vec::with_capacity(primary_msgs.len());
            for msg in primary_msgs {
                // see if we've already seen this message
                if !self
                    .message_cache
                    .read()
                    .await
                    .contains(calculate_hash_of(&msg))
                {
                    filtered_msgs.push(msg.clone());
                    self.message_cache
                        .write()
                        .await
                        .insert(calculate_hash_of(&msg));
                }
            }

            Ok(filtered_msgs)
        };

        boxed_sync(closure)
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
        <WebServerNetwork<_> as ConnectedNetwork<Message<TYPES>,TYPES::SignatureKey>>::
            inject_consensus_info(self.primary(), event.clone()).await;

        <Libp2pNetwork<_, _> as ConnectedNetwork<Message<TYPES>,TYPES::SignatureKey>>::
            inject_consensus_info(self.secondary(), event).await;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tracing::instrument;

    /// cache eviction test
    #[cfg_attr(
        async_executor_impl = "tokio",
        tokio::test(flavor = "multi_thread", worker_threads = 2)
    )]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    #[instrument]
    async fn test_cache_eviction() {
        let mut cache = Cache::new(3);
        cache.insert(1);
        cache.insert(2);
        cache.insert(3);
        cache.insert(4);
        assert_eq!(cache.inner.len(), 3);
        assert_eq!(cache.hashes.len(), 3);
        assert!(!cache.inner.contains(&1));
        assert!(cache.inner.contains(&2));
        assert!(cache.inner.contains(&3));
        assert!(cache.inner.contains(&4));
        assert!(!cache.hashes.contains(&1));
        assert!(cache.hashes.contains(&2));
        assert!(cache.hashes.contains(&3));
        assert!(cache.hashes.contains(&4));
    }
}
