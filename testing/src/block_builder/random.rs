use std::{
    collections::HashMap,
    num::NonZeroUsize,
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use async_broadcast::{broadcast, Sender};
use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::RwLock;
use async_trait::async_trait;
use futures::{future::BoxFuture, Stream, StreamExt};
use hotshot::types::{Event, EventType, SignatureKey};
use hotshot_builder_api::{
    block_info::{AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo},
    builder::BuildError,
    data_source::BuilderDataSource,
};
use hotshot_example_types::block_types::TestTransaction;
use hotshot_orchestrator::config::RandomBuilderConfig;
use hotshot_types::{
    traits::{node_implementation::NodeType, signature_key::BuilderSignatureKey},
    utils::BuilderCommitment,
    vid::VidCommitment,
};
use lru::LruCache;
use rand::{rngs::SmallRng, Rng, RngCore, SeedableRng};
use tide_disco::{method::ReadState, Url};

use super::{build_block, run_builder_source, BlockEntry, BuilderTask, TestBuilderImplementation};
use crate::test_builder::BuilderChange;

pub struct RandomBuilderImplementation;

impl RandomBuilderImplementation {
    pub async fn create<TYPES: NodeType<Transaction = TestTransaction>>(
        num_storage_nodes: usize,
        config: RandomBuilderConfig,
        changes: HashMap<u64, BuilderChange>,
        change_sender: Sender<BuilderChange>,
    ) -> (RandomBuilderTask<TYPES>, RandomBuilderSource<TYPES>)
    where
        <TYPES as NodeType>::InstanceState: Default,
    {
        let (pub_key, priv_key) =
            TYPES::BuilderSignatureKey::generated_from_seed_indexed([1; 32], 0);
        let blocks = Arc::new(RwLock::new(LruCache::new(NonZeroUsize::new(256).unwrap())));
        let source = RandomBuilderSource {
            blocks: Arc::clone(&blocks),
            pub_key: pub_key.clone(),
            should_fail_claims: Arc::new(AtomicBool::new(false)),
        };
        let task = RandomBuilderTask {
            blocks,
            config,
            num_storage_nodes,
            changes,
            change_sender,
            pub_key,
            priv_key,
        };
        (task, source)
    }
}

#[async_trait]
impl<TYPES> TestBuilderImplementation<TYPES> for RandomBuilderImplementation
where
    TYPES: NodeType<Transaction = TestTransaction>,
    <TYPES as NodeType>::InstanceState: Default,
{
    type Config = RandomBuilderConfig;

    async fn start(
        num_storage_nodes: usize,
        url: Url,
        config: RandomBuilderConfig,
        changes: HashMap<u64, BuilderChange>,
    ) -> Box<dyn BuilderTask<TYPES>> {
        let (change_sender, change_receiver) = broadcast(128);

        let (task, source) = Self::create(num_storage_nodes, config, changes, change_sender).await;
        run_builder_source(url, change_receiver, source);
        Box::new(task)
    }
}

pub struct RandomBuilderTask<TYPES: NodeType<Transaction = TestTransaction>> {
    num_storage_nodes: usize,
    config: RandomBuilderConfig,
    changes: HashMap<u64, BuilderChange>,
    change_sender: Sender<BuilderChange>,
    pub_key: TYPES::BuilderSignatureKey,
    priv_key: <TYPES::BuilderSignatureKey as BuilderSignatureKey>::BuilderPrivateKey,
    blocks: Arc<RwLock<LruCache<BuilderCommitment, BlockEntry<TYPES>>>>,
}

impl<TYPES: NodeType<Transaction = TestTransaction>> RandomBuilderTask<TYPES> {
    async fn build_blocks(
        options: RandomBuilderConfig,
        num_storage_nodes: usize,
        pub_key: <TYPES as NodeType>::BuilderSignatureKey,
        priv_key: <<TYPES as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::BuilderPrivateKey,
        blocks: Arc<RwLock<LruCache<BuilderCommitment, BlockEntry<TYPES>>>>,
    ) where
        <TYPES as NodeType>::InstanceState: Default,
    {
        let mut rng = SmallRng::from_entropy();
        let time_per_block = Duration::from_secs(1) / options.blocks_per_second;
        loop {
            let start = std::time::Instant::now();
            let transactions: Vec<TestTransaction> = (0..options.txn_in_block)
                .map(|_| {
                    let mut bytes = vec![
                        0;
                        rng.gen_range(options.txn_size.clone())
                            .try_into()
                            .expect("We are NOT running on a 16-bit platform")
                    ];
                    rng.fill_bytes(&mut bytes);
                    TestTransaction::new(bytes)
                })
                .collect();

            let block = build_block(
                transactions,
                num_storage_nodes,
                pub_key.clone(),
                priv_key.clone(),
            )
            .await;

            if let Some((hash, _)) = blocks
                .write()
                .await
                .push(block.metadata.block_hash.clone(), block)
            {
                tracing::warn!("Block {} evicted", hash);
            };
            if time_per_block < start.elapsed() {
                tracing::warn!(
                    "Can't keep up: last block built in {}ms, target time per block: {}",
                    start.elapsed().as_millis(),
                    time_per_block.as_millis(),
                );
            }
            async_sleep(time_per_block.saturating_sub(start.elapsed())).await;
        }
    }
}

impl<TYPES: NodeType<Transaction = TestTransaction>> BuilderTask<TYPES> for RandomBuilderTask<TYPES>
where
    <TYPES as NodeType>::InstanceState: Default,
{
    fn start(
        mut self: Box<Self>,
        mut stream: Box<dyn Stream<Item = Event<TYPES>> + std::marker::Unpin + Send + 'static>,
    ) {
        let mut task = Some(async_spawn(Self::build_blocks(
            self.config.clone(),
            self.num_storage_nodes,
            self.pub_key.clone(),
            self.priv_key.clone(),
            self.blocks.clone(),
        )));

        async_spawn(async move {
            loop {
                match stream.next().await {
                    None => {
                        break;
                    }
                    Some(evt) => {
                        if let EventType::ViewFinished { view_number } = evt.event {
                            if let Some(change) = self.changes.remove(&view_number) {
                                match change {
                                    BuilderChange::Up => {
                                        if task.is_none() {
                                            task = Some(async_spawn(Self::build_blocks(
                                                self.config.clone(),
                                                self.num_storage_nodes,
                                                self.pub_key.clone(),
                                                self.priv_key.clone(),
                                                self.blocks.clone(),
                                            )))
                                        }
                                    }
                                    BuilderChange::Down => {
                                        if let Some(handle) = task.take() {
                                            #[cfg(async_executor_impl = "tokio")]
                                            handle.abort();
                                            #[cfg(async_executor_impl = "async-std")]
                                            handle.cancel().await;
                                        }
                                    }
                                    BuilderChange::FailClaims(_) => {}
                                }
                                let _ = self.change_sender.broadcast(change).await;
                            }
                        }
                    }
                }
            }
        });
    }
}

/// A mock implementation of the builder data source.
/// Builds random blocks, doesn't track HotShot state at all.
/// Evicts old available blocks if HotShot doesn't keep up.
#[derive(Clone, Debug)]
pub struct RandomBuilderSource<TYPES: NodeType> {
    /// Built blocks
    blocks: Arc<
        RwLock<
            // Isn't strictly speaking used as a cache,
            // just as a HashMap that evicts old blocks
            LruCache<BuilderCommitment, BlockEntry<TYPES>>,
        >,
    >,
    pub_key: TYPES::BuilderSignatureKey,
    should_fail_claims: Arc<AtomicBool>,
}

impl<TYPES> RandomBuilderSource<TYPES>
where
    TYPES: NodeType<Transaction = TestTransaction>,
    <TYPES as NodeType>::InstanceState: Default,
{
    /// Create new [`RandomBuilderSource`]
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // ony panics if 256 == 0
    pub fn new(pub_key: TYPES::BuilderSignatureKey) -> Self {
        Self {
            blocks: Arc::new(RwLock::new(LruCache::new(NonZeroUsize::new(256).unwrap()))),
            pub_key,
            should_fail_claims: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[async_trait]
impl<TYPES: NodeType> ReadState for RandomBuilderSource<TYPES> {
    type State = Self;

    async fn read<T>(
        &self,
        op: impl Send + for<'a> FnOnce(&'a Self::State) -> BoxFuture<'a, T> + 'async_trait,
    ) -> T {
        op(self).await
    }
}

#[async_trait]
impl<TYPES: NodeType> BuilderDataSource<TYPES> for RandomBuilderSource<TYPES> {
    async fn available_blocks(
        &self,
        _for_parent: &VidCommitment,
        _view_number: u64,
        _sender: TYPES::SignatureKey,
        _signature: &<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<Vec<AvailableBlockInfo<TYPES>>, BuildError> {
        Ok(self
            .blocks
            .deref()
            .read()
            .await
            .iter()
            .map(|(_, BlockEntry { metadata, .. })| metadata.clone())
            .collect())
    }

    async fn claim_block(
        &self,
        block_hash: &BuilderCommitment,
        _view_number: u64,
        _sender: TYPES::SignatureKey,
        _signature: &<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockData<TYPES>, BuildError> {
        if self.should_fail_claims.load(Ordering::Relaxed) {
            return Err(BuildError::Missing);
        }

        let mut blocks = self.blocks.write().await;
        let entry = blocks.get_mut(block_hash).ok_or(BuildError::NotFound)?;
        let payload = entry.payload.take().ok_or(BuildError::Missing)?;
        // Check if header input is claimed as well, if yes, then evict block
        if entry.header_input.is_none() {
            blocks.pop(block_hash);
        };
        Ok(payload)
    }

    async fn claim_block_header_input(
        &self,
        block_hash: &BuilderCommitment,
        _view_number: u64,
        _sender: TYPES::SignatureKey,
        _signature: &<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockHeaderInput<TYPES>, BuildError> {
        if self.should_fail_claims.load(Ordering::Relaxed) {
            return Err(BuildError::Missing);
        }

        let mut blocks = self.blocks.write().await;
        let entry = blocks.get_mut(block_hash).ok_or(BuildError::NotFound)?;
        let header_input = entry.header_input.take().ok_or(BuildError::Missing)?;
        // Check if payload is claimed as well, if yes, then evict block
        if entry.payload.is_none() {
            blocks.pop(block_hash);
        };
        Ok(header_input)
    }

    async fn builder_address(&self) -> Result<TYPES::BuilderSignatureKey, BuildError> {
        Ok(self.pub_key.clone())
    }
}
