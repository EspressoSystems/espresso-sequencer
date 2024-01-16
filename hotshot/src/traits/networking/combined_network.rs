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
    collections::HashSet,
    hash::Hasher,
    sync::atomic::{AtomicU64, Ordering},
};
use tracing::warn;

use async_trait::async_trait;

use futures::join;

use async_compatibility_layer::channel::UnboundedSendError;
use hotshot_task::{boxed_sync, BoxSyncFuture};
use hotshot_types::{
    data::ViewNumber,
    message::Message,
    traits::{
        election::Membership,
        network::{
            CommunicationChannel, ConnectedNetwork, ConsensusIntentEvent,
            TestableChannelImplementation, TestableNetworkingImplementation, TransmitType,
            ViewMessage,
        },
        node_implementation::NodeType,
    },
};
use std::{collections::hash_map::DefaultHasher, sync::Arc};

use std::hash::Hash;

/// A cache to keep track of the last n messages we've seen, avoids reprocessing duplicates
/// from multiple networks
#[derive(Clone, Debug)]
struct Cache {
    /// The maximum number of items to store in the cache
    capacity: usize,
    /// The cache itself
    #[allow(clippy::struct_field_names)]
    cache: HashSet<u64>,
    /// The hashes of the messages in the cache, in order of insertion
    hashes: Vec<u64>,
}

impl Cache {
    /// Create a new cache with the given capacity
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            cache: HashSet::with_capacity(capacity),
            hashes: Vec::with_capacity(capacity),
        }
    }

    /// Insert a hash into the cache
    fn insert(&mut self, hash: u64) {
        if self.cache.contains(&hash) {
            return;
        }

        // calculate how much we are over and remove that many elements from the cache. deal with overflow
        let over = (self.hashes.len() + 1).saturating_sub(self.capacity);
        if over > 0 {
            for _ in 0..over {
                let hash = self.hashes.remove(0);
                self.cache.remove(&hash);
            }
        }

        self.cache.insert(hash);
        self.hashes.push(hash);
    }

    /// Check if the cache contains a hash
    fn contains(&self, hash: u64) -> bool {
        self.cache.contains(&hash)
    }

    /// Get the number of items in the cache
    #[cfg(test)]
    fn len(&self) -> usize {
        self.cache.len()
    }
}

/// Helper function to calculate a hash of a type that implements Hash
fn calculate_hash_of<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// A communication channel with 2 networks, where we can fall back to the slower network if the
/// primary fails
#[derive(Clone, Debug)]
pub struct CombinedCommChannel<TYPES: NodeType> {
    /// The two networks we'll use for send/recv
    networks: Arc<CombinedNetworks<TYPES>>,

    /// Last n seen messages to prevent processing duplicates
    message_cache: Arc<RwLock<Cache>>,

    /// If the primary network is down (0) or not, and for how many messages
    primary_down: Arc<AtomicU64>,
}

impl<TYPES: NodeType> CombinedCommChannel<TYPES> {
    /// Constructor
    #[must_use]
    pub fn new(networks: Arc<CombinedNetworks<TYPES>>) -> Self {
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
pub struct CombinedNetworks<TYPES: NodeType>(
    pub WebServerNetwork<TYPES>,
    pub Libp2pNetwork<Message<TYPES>, TYPES::SignatureKey>,
);

impl<TYPES: NodeType> TestableNetworkingImplementation<TYPES> for CombinedNetworks<TYPES> {
    fn generator(
        expected_node_count: usize,
        num_bootstrap: usize,
        network_id: usize,
        da_committee_size: usize,
        is_da: bool,
    ) -> Box<dyn Fn(u64) -> Self + 'static> {
        let generators = (
            <WebServerNetwork<
                TYPES,
            > as TestableNetworkingImplementation<_>>::generator(
                expected_node_count,
                num_bootstrap,
                network_id,
                da_committee_size,
                is_da
            ),
            <Libp2pNetwork<Message<TYPES>, TYPES::SignatureKey> as TestableNetworkingImplementation<_>>::generator(
                expected_node_count,
                num_bootstrap,
                network_id,
                da_committee_size,
                is_da
            )
        );
        Box::new(move |node_id| CombinedNetworks(generators.0(node_id), generators.1(node_id)))
    }

    /// Get the number of messages in-flight.
    ///
    /// Some implementations will not be able to tell how many messages there are in-flight. These implementations should return `None`.
    fn in_flight_message_count(&self) -> Option<usize> {
        None
    }
}

impl<TYPES: NodeType> TestableNetworkingImplementation<TYPES> for CombinedCommChannel<TYPES> {
    fn generator(
        expected_node_count: usize,
        num_bootstrap: usize,
        network_id: usize,
        da_committee_size: usize,
        is_da: bool,
    ) -> Box<dyn Fn(u64) -> Self + 'static> {
        let generator = <CombinedNetworks<TYPES> as TestableNetworkingImplementation<_>>::generator(
            expected_node_count,
            num_bootstrap,
            network_id,
            da_committee_size,
            is_da,
        );
        Box::new(move |node_id| Self {
            networks: generator(node_id).into(),
            message_cache: Arc::new(RwLock::new(Cache::new(COMBINED_NETWORK_CACHE_SIZE))),
            primary_down: Arc::new(AtomicU64::new(0)),
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
impl<TYPES: NodeType> CommunicationChannel<TYPES> for CombinedCommChannel<TYPES> {
    type NETWORK = CombinedNetworks<TYPES>;

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
        election: &TYPES::Membership,
    ) -> Result<(), NetworkError> {
        let recipients =
            <TYPES as NodeType>::Membership::get_committee(election, message.get_view_number());

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

    fn recv_msgs<'a, 'b>(
        &'a self,
        transmit_type: TransmitType,
    ) -> BoxSyncFuture<'b, Result<Vec<Message<TYPES>>, NetworkError>>
    where
        'a: 'b,
        Self: 'b,
    {
        // recv on both networks because nodes may be accessible only on either. discard duplicates
        // TODO: improve this algorithm: https://github.com/EspressoSystems/HotShot/issues/2089
        let closure = async move {
            let mut primary_msgs = self.primary().recv_msgs(transmit_type).await?;
            let mut secondary_msgs = self.secondary().recv_msgs(transmit_type).await?;

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

impl<TYPES: NodeType> TestableChannelImplementation<TYPES> for CombinedCommChannel<TYPES> {
    fn generate_network() -> Box<dyn Fn(Arc<Self::NETWORK>) -> Self + 'static> {
        Box::new(move |network| CombinedCommChannel::new(network))
    }
}

#[cfg(test)]
mod test {
    use hotshot_testing::block_types::TestTransaction;

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
        assert_eq!(cache.cache.len(), 3);
        assert_eq!(cache.hashes.len(), 3);
        assert!(!cache.cache.contains(&1));
        assert!(cache.cache.contains(&2));
        assert!(cache.cache.contains(&3));
        assert!(cache.cache.contains(&4));
        assert!(!cache.hashes.contains(&1));
        assert!(cache.hashes.contains(&2));
        assert!(cache.hashes.contains(&3));
        assert!(cache.hashes.contains(&4));
    }

    #[cfg_attr(
        async_executor_impl = "tokio",
        tokio::test(flavor = "multi_thread", worker_threads = 2)
    )]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    #[instrument]
    async fn test_hash_calculation() {
        let message1 = TestTransaction(vec![0; 32]);
        let message2 = TestTransaction(vec![1; 32]);

        assert_eq!(calculate_hash_of(&message1), calculate_hash_of(&message1));
        assert_ne!(calculate_hash_of(&message1), calculate_hash_of(&message2));
    }

    #[cfg_attr(
        async_executor_impl = "tokio",
        tokio::test(flavor = "multi_thread", worker_threads = 2)
    )]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    #[instrument]
    async fn test_cache_integrity() {
        let message1 = TestTransaction(vec![0; 32]);
        let message2 = TestTransaction(vec![1; 32]);

        let mut cache = Cache::new(3);

        // test insertion integrity
        cache.insert(calculate_hash_of(&message1));
        cache.insert(calculate_hash_of(&message2));

        assert!(cache.contains(calculate_hash_of(&message1)));
        assert!(cache.contains(calculate_hash_of(&message2)));

        // check that the cache is not modified on duplicate entries
        cache.insert(calculate_hash_of(&message1));
        assert!(cache.contains(calculate_hash_of(&message1)));
        assert!(cache.contains(calculate_hash_of(&message2)));
        assert_eq!(cache.len(), 2);
    }
}
