use std::{
    num::NonZeroUsize,
    ops::{Deref, Range},
    sync::Arc,
    time::Duration,
};

use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::RwLock;
use async_trait::async_trait;
use futures::future::BoxFuture;
use hotshot::{traits::BlockPayload, types::SignatureKey};
use hotshot_builder_api::{
    block_info::{AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo},
    builder::{BuildError, Options},
    data_source::BuilderDataSource,
};
use hotshot_example_types::{
    block_types::{TestBlockPayload, TestTransaction},
    node_types::TestTypes,
};
use hotshot_types::{
    constants::{Version01, STATIC_VER_0_1},
    traits::{block_contents::vid_commitment, node_implementation::NodeType},
    utils::BuilderCommitment,
    vid::VidCommitment,
};
use lru::LruCache;
use rand::{rngs::SmallRng, Rng, RngCore, SeedableRng};
use tide_disco::{method::ReadState, App, Url};

/// Entry for a built block
struct BlockEntry {
    metadata: AvailableBlockInfo<TestTypes>,
    payload: Option<AvailableBlockData<TestTypes>>,
    header_input: Option<AvailableBlockHeaderInput<TestTypes>>,
}

/// Options controlling how the random builder generates blocks
#[derive(Clone, Debug)]
pub struct RandomBuilderOptions {
    /// How many transactions to include in a block
    pub txn_in_block: u64,
    /// How many blocks to generate per second
    pub blocks_per_second: u64,
    /// Range of how big a transaction can be (in bytes)
    pub txn_size: Range<u32>,
    /// Number of storage nodes for VID commitment
    pub num_storage_nodes: usize,
}

impl Default for RandomBuilderOptions {
    fn default() -> Self {
        Self {
            txn_in_block: 100,
            blocks_per_second: 1,
            txn_size: 20..100,
            num_storage_nodes: 1,
        }
    }
}

/// A mock implementation of the builder data source.
/// Builds random blocks, doesn't track HotShot state at all.
/// Evicts old available blocks if HotShot doesn't keep up.
#[derive(Clone, Debug)]
pub struct RandomBuilderSource {
    /// Built blocks
    blocks: Arc<
        RwLock<
            // Isn't strictly speaking used as a cache,
            // just as a HashMap that evicts old blocks
            LruCache<BuilderCommitment, BlockEntry>,
        >,
    >,
    pub_key: <TestTypes as NodeType>::SignatureKey,
    priv_key: <<TestTypes as NodeType>::SignatureKey as SignatureKey>::PrivateKey,
}

impl RandomBuilderSource {
    /// Create new [`RandomBuilderSource`]
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // ony panics if 256 == 0
    pub fn new(
        pub_key: <TestTypes as NodeType>::SignatureKey,
        priv_key: <<TestTypes as NodeType>::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Self {
        Self {
            blocks: Arc::new(RwLock::new(LruCache::new(NonZeroUsize::new(256).unwrap()))),
            priv_key,
            pub_key,
        }
    }

    /// Spawn a task building blocks, configured with given options
    #[allow(clippy::missing_panics_doc)] // ony panics on 16-bit platforms
    pub fn run(&self, options: RandomBuilderOptions) {
        let blocks = self.blocks.clone();
        let (priv_key, pub_key) = (self.priv_key.clone(), self.pub_key);
        async_spawn(async move {
            let mut rng = SmallRng::from_entropy();
            let time_per_block = Duration::from_millis(1000 / options.blocks_per_second);
            loop {
                let start = std::time::Instant::now();
                let transactions = (0..options.txn_in_block)
                    .map(|_| {
                        let mut bytes = vec![
                            0;
                            rng.gen_range(options.txn_size.clone())
                                .try_into()
                                .expect("We are NOT running on a 16-bit platform")
                        ];
                        rng.fill_bytes(&mut bytes);
                        TestTransaction(bytes)
                    })
                    .collect::<Vec<TestTransaction>>();
                let block_size = transactions.iter().map(|t| t.0.len() as u64).sum::<u64>();

                let block_payload = TestBlockPayload { transactions };
                let commitment = block_payload.builder_commitment(&());
                let vid_commitment = vid_commitment(
                    &block_payload.encode().unwrap().collect(),
                    options.num_storage_nodes,
                );

                let signature_over_block_info = {
                    let mut block_info: Vec<u8> = Vec::new();
                    block_info.extend_from_slice(block_size.to_be_bytes().as_ref());
                    block_info.extend_from_slice(123_u64.to_be_bytes().as_ref());
                    block_info.extend_from_slice(commitment.as_ref());
                    match <TestTypes as NodeType>::SignatureKey::sign(&priv_key, &block_info) {
                        Ok(sig) => sig,
                        Err(e) => {
                            tracing::error!(error = %e, "Failed to sign block");
                            continue;
                        }
                    }
                };

                let signature_over_builder_commitment =
                    match <TestTypes as NodeType>::SignatureKey::sign(
                        &priv_key,
                        commitment.as_ref(),
                    ) {
                        Ok(sig) => sig,
                        Err(e) => {
                            tracing::error!(error = %e, "Failed to sign block");
                            continue;
                        }
                    };

                let signature_over_vid_commitment =
                    match <TestTypes as NodeType>::SignatureKey::sign(
                        &priv_key,
                        vid_commitment.as_ref(),
                    ) {
                        Ok(sig) => sig,
                        Err(e) => {
                            tracing::error!(error = %e, "Failed to sign block");
                            continue;
                        }
                    };

                let block = AvailableBlockData {
                    block_payload,
                    metadata: (),
                    sender: pub_key,
                    signature: signature_over_block_info,
                    _phantom: std::marker::PhantomData,
                };
                let metadata = AvailableBlockInfo {
                    sender: pub_key,
                    signature: signature_over_builder_commitment,
                    block_hash: commitment,
                    block_size,
                    offered_fee: 123,
                    _phantom: std::marker::PhantomData,
                };
                let header_input = AvailableBlockHeaderInput {
                    vid_commitment,
                    signature: signature_over_vid_commitment,
                    sender: pub_key,
                    _phantom: std::marker::PhantomData,
                };
                if let Some((hash, _)) = blocks.write().await.push(
                    metadata.block_hash.clone(),
                    BlockEntry {
                        metadata,
                        payload: Some(block),
                        header_input: Some(header_input),
                    },
                ) {
                    tracing::warn!("Block {} evicted", hash);
                };
                async_sleep(time_per_block - start.elapsed()).await;
            }
        });
    }
}

#[async_trait]
impl ReadState for RandomBuilderSource {
    type State = Self;

    async fn read<T>(
        &self,
        op: impl Send + for<'a> FnOnce(&'a Self::State) -> BoxFuture<'a, T> + 'async_trait,
    ) -> T {
        op(self).await
    }
}

#[async_trait]
impl BuilderDataSource<TestTypes> for RandomBuilderSource {
    async fn get_available_blocks(
        &self,
        _for_parent: &VidCommitment,
    ) -> Result<Vec<AvailableBlockInfo<TestTypes>>, BuildError> {
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
        _signature: &<<TestTypes as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockData<TestTypes>, BuildError> {
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
        _signature: &<<TestTypes as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockHeaderInput<TestTypes>, BuildError> {
        let mut blocks = self.blocks.write().await;
        let entry = blocks.get_mut(block_hash).ok_or(BuildError::NotFound)?;
        let header_input = entry.header_input.take().ok_or(BuildError::Missing)?;
        // Check if payload is claimed as well, if yes, then evict block
        if entry.payload.is_none() {
            blocks.pop(block_hash);
        };
        Ok(header_input)
    }
}

/// Construct a tide disco app that mocks the builder API.
///
/// # Panics
/// If constructing and launching the builder fails for any reason
pub fn run_random_builder(url: Url) {
    let (pub_key, priv_key) =
        <TestTypes as NodeType>::SignatureKey::generated_from_seed_indexed([1; 32], 0);
    let source = RandomBuilderSource::new(pub_key, priv_key);
    source.run(RandomBuilderOptions::default());

    let builder_api =
        hotshot_builder_api::builder::define_api::<RandomBuilderSource, TestTypes, Version01>(
            &Options::default(),
        )
        .expect("Failed to construct the builder API");
    let mut app: App<RandomBuilderSource, hotshot_builder_api::builder::Error, Version01> =
        App::with_state(source);
    app.register_module("/", builder_api)
        .expect("Failed to register the builder API");

    async_spawn(app.serve(url, STATIC_VER_0_1));
}
