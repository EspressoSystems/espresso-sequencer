use std::{
    collections::HashMap,
    num::NonZeroUsize,
    ops::Deref,
    sync::Arc,
    time::{Duration, Instant},
};

use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::RwLock;
use async_trait::async_trait;
use committable::{Commitment, Committable};
use futures::{future::BoxFuture, Stream, StreamExt};
use hotshot::{
    traits::BlockPayload,
    types::{Event, EventType, SignatureKey},
};
use hotshot_builder_api::{
    block_info::{AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo},
    builder::{BuildError, Error, Options},
    data_source::BuilderDataSource,
};
use hotshot_example_types::block_types::TestTransaction;
use hotshot_orchestrator::config::RandomBuilderConfig;
use hotshot_types::{
    constants::{Version01, STATIC_VER_0_1},
    traits::{
        block_contents::{precompute_vid_commitment, BlockHeader},
        node_implementation::NodeType,
        signature_key::BuilderSignatureKey,
    },
    utils::BuilderCommitment,
};
use lru::LruCache;
use rand::{rngs::SmallRng, Rng, RngCore, SeedableRng};
use tide_disco::{method::ReadState, App, Url};

#[async_trait]
pub trait TestBuilderImplementation<TYPES: NodeType> {
    type Config: Default;

    async fn start(
        num_storage_nodes: usize,
        options: Self::Config,
    ) -> (Option<Box<dyn BuilderTask<TYPES>>>, Url);
}

pub struct RandomBuilderImplementation;

#[async_trait]
impl<TYPES> TestBuilderImplementation<TYPES> for RandomBuilderImplementation
where
    TYPES: NodeType<Transaction = TestTransaction>,
{
    type Config = RandomBuilderConfig;

    async fn start(
        num_storage_nodes: usize,
        config: RandomBuilderConfig,
    ) -> (Option<Box<dyn BuilderTask<TYPES>>>, Url) {
        let port = portpicker::pick_unused_port().expect("No free ports");
        let url = Url::parse(&format!("http://localhost:{port}")).expect("Valid URL");
        run_random_builder::<TYPES>(url.clone(), num_storage_nodes, config);
        (None, url)
    }
}

pub struct SimpleBuilderImplementation;

#[async_trait]
impl<TYPES: NodeType> TestBuilderImplementation<TYPES> for SimpleBuilderImplementation {
    type Config = ();

    async fn start(
        num_storage_nodes: usize,
        _config: Self::Config,
    ) -> (Option<Box<dyn BuilderTask<TYPES>>>, Url) {
        let port = portpicker::pick_unused_port().expect("No free ports");
        let url = Url::parse(&format!("http://localhost:{port}")).expect("Valid URL");
        let (source, task) = make_simple_builder(num_storage_nodes).await;

        let builder_api = hotshot_builder_api::builder::define_api::<
            SimpleBuilderSource<TYPES>,
            TYPES,
            Version01,
        >(&Options::default())
        .expect("Failed to construct the builder API");
        let mut app: App<SimpleBuilderSource<TYPES>, hotshot_builder_api::builder::Error> =
            App::with_state(source);
        app.register_module("api", builder_api)
            .expect("Failed to register the builder API");

        async_spawn(app.serve(url.clone(), STATIC_VER_0_1));
        (Some(Box::new(task)), url)
    }
}

/// Entry for a built block
struct BlockEntry<TYPES: NodeType> {
    metadata: AvailableBlockInfo<TYPES>,
    payload: Option<AvailableBlockData<TYPES>>,
    header_input: Option<AvailableBlockHeaderInput<TYPES>>,
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
    priv_key: <TYPES::BuilderSignatureKey as BuilderSignatureKey>::BuilderPrivateKey,
}

impl<TYPES> RandomBuilderSource<TYPES>
where
    TYPES: NodeType<Transaction = TestTransaction>,
{
    /// Create new [`RandomBuilderSource`]
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // ony panics if 256 == 0
    pub fn new(
        pub_key: TYPES::BuilderSignatureKey,
        priv_key: <TYPES::BuilderSignatureKey as BuilderSignatureKey>::BuilderPrivateKey,
    ) -> Self {
        Self {
            blocks: Arc::new(RwLock::new(LruCache::new(NonZeroUsize::new(256).unwrap()))),
            priv_key,
            pub_key,
        }
    }

    /// Spawn a task building blocks, configured with given options
    #[allow(clippy::missing_panics_doc)] // ony panics on 16-bit platforms
    pub fn run(&self, num_storage_nodes: usize, options: RandomBuilderConfig) {
        let blocks = self.blocks.clone();
        let (priv_key, pub_key) = (self.priv_key.clone(), self.pub_key.clone());
        async_spawn(async move {
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
                        TestTransaction(bytes)
                    })
                    .collect();

                let (metadata, payload, header_input) = build_block(
                    transactions,
                    num_storage_nodes,
                    pub_key.clone(),
                    priv_key.clone(),
                );

                if let Some((hash, _)) = blocks.write().await.push(
                    metadata.block_hash.clone(),
                    BlockEntry {
                        metadata,
                        payload: Some(payload),
                        header_input: Some(header_input),
                    },
                ) {
                    tracing::warn!("Block {} evicted", hash);
                };
                async_sleep(time_per_block.saturating_sub(start.elapsed())).await;
            }
        });
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
    async fn get_available_blocks(
        &self,
        _for_parent: &BuilderCommitment,
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
        _sender: TYPES::SignatureKey,
        _signature: &<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockData<TYPES>, BuildError> {
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
        _sender: TYPES::SignatureKey,
        _signature: &<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockHeaderInput<TYPES>, BuildError> {
        let mut blocks = self.blocks.write().await;
        let entry = blocks.get_mut(block_hash).ok_or(BuildError::NotFound)?;
        let header_input = entry.header_input.take().ok_or(BuildError::Missing)?;
        // Check if payload is claimed as well, if yes, then evict block
        if entry.payload.is_none() {
            blocks.pop(block_hash);
        };
        Ok(header_input)
    }

    async fn get_builder_address(&self) -> Result<TYPES::BuilderSignatureKey, BuildError> {
        Ok(self.pub_key.clone())
    }
}

/// Construct a tide disco app that mocks the builder API.
///
/// # Panics
/// If constructing and launching the builder fails for any reason
pub fn run_random_builder<TYPES>(url: Url, num_storage_nodes: usize, options: RandomBuilderConfig)
where
    TYPES: NodeType<Transaction = TestTransaction>,
{
    let (pub_key, priv_key) = TYPES::BuilderSignatureKey::generated_from_seed_indexed([1; 32], 0);
    let source = RandomBuilderSource::new(pub_key, priv_key);
    source.run(num_storage_nodes, options);

    let builder_api =
        hotshot_builder_api::builder::define_api::<RandomBuilderSource<TYPES>, TYPES, Version01>(
            &Options::default(),
        )
        .expect("Failed to construct the builder API");
    let mut app: App<RandomBuilderSource<TYPES>, Error> = App::with_state(source);
    app.register_module::<Error, Version01>("api", builder_api)
        .expect("Failed to register the builder API");

    async_spawn(app.serve(url, STATIC_VER_0_1));
}

#[derive(Debug, Clone)]
struct SubmittedTransaction<TYPES: NodeType> {
    claimed: Option<Instant>,
    transaction: TYPES::Transaction,
}

pub struct SimpleBuilderSource<TYPES: NodeType> {
    pub_key: TYPES::BuilderSignatureKey,
    priv_key: <TYPES::BuilderSignatureKey as BuilderSignatureKey>::BuilderPrivateKey,
    num_storage_nodes: usize,
    #[allow(clippy::type_complexity)]
    transactions: Arc<RwLock<HashMap<Commitment<TYPES::Transaction>, SubmittedTransaction<TYPES>>>>,
    blocks: Arc<RwLock<HashMap<BuilderCommitment, BlockEntry<TYPES>>>>,
}

#[async_trait]
impl<TYPES: NodeType> ReadState for SimpleBuilderSource<TYPES> {
    type State = Self;

    async fn read<T>(
        &self,
        op: impl Send + for<'a> FnOnce(&'a Self::State) -> BoxFuture<'a, T> + 'async_trait,
    ) -> T {
        op(self).await
    }
}

#[async_trait]
impl<TYPES: NodeType> BuilderDataSource<TYPES> for SimpleBuilderSource<TYPES> {
    async fn get_available_blocks(
        &self,
        _for_parent: &BuilderCommitment,
        _sender: TYPES::SignatureKey,
        _signature: &<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<Vec<AvailableBlockInfo<TYPES>>, BuildError> {
        let transactions = self
            .transactions
            .read(|txns| {
                Box::pin(async {
                    txns.values()
                        .filter(|txn| {
                            // We want transactions that are either unclaimed, or claimed long ago
                            // and thus probably not included, or they would've been decided on
                            // already and removed from the queue
                            txn.claimed
                                .map(|claim_time| claim_time.elapsed() > Duration::from_secs(30))
                                .unwrap_or(true)
                        })
                        .cloned()
                        .map(|txn| txn.transaction)
                        .collect::<Vec<TYPES::Transaction>>()
                })
            })
            .await;
        let (metadata, payload, header_input) = build_block(
            transactions,
            self.num_storage_nodes,
            self.pub_key.clone(),
            self.priv_key.clone(),
        );

        self.blocks.write().await.insert(
            metadata.block_hash.clone(),
            BlockEntry {
                metadata: metadata.clone(),
                payload: Some(payload),
                header_input: Some(header_input),
            },
        );

        Ok(vec![metadata])
    }

    async fn claim_block(
        &self,
        block_hash: &BuilderCommitment,
        _sender: TYPES::SignatureKey,
        _signature: &<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockData<TYPES>, BuildError> {
        let payload = {
            let mut blocks = self.blocks.write().await;
            let entry = blocks.get_mut(block_hash).ok_or(BuildError::NotFound)?;
            entry.payload.take().ok_or(BuildError::Missing)?
        };

        let now = Instant::now();

        let claimed_transactions = payload
            .block_payload
            .transaction_commitments(&payload.metadata);

        let mut transactions = self.transactions.write().await;
        for txn_hash in claimed_transactions {
            if let Some(txn) = transactions.get_mut(&txn_hash) {
                txn.claimed = Some(now);
            }
        }

        Ok(payload)
    }

    async fn claim_block_header_input(
        &self,
        block_hash: &BuilderCommitment,
        _sender: TYPES::SignatureKey,
        _signature: &<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockHeaderInput<TYPES>, BuildError> {
        let mut blocks = self.blocks.write().await;
        let entry = blocks.get_mut(block_hash).ok_or(BuildError::NotFound)?;
        entry.header_input.take().ok_or(BuildError::Missing)
    }

    async fn get_builder_address(&self) -> Result<TYPES::BuilderSignatureKey, BuildError> {
        Ok(self.pub_key.clone())
    }
}

impl<TYPES: NodeType> SimpleBuilderSource<TYPES> {
    pub async fn run(self, url: Url) {
        let builder_api = hotshot_builder_api::builder::define_api::<
            SimpleBuilderSource<TYPES>,
            TYPES,
            Version01,
        >(&Options::default())
        .expect("Failed to construct the builder API");
        let mut app: App<SimpleBuilderSource<TYPES>, Error> = App::with_state(self);
        app.register_module::<Error, Version01>("api", builder_api)
            .expect("Failed to register the builder API");

        async_spawn(app.serve(url, STATIC_VER_0_1));
    }
}

#[derive(Clone)]
pub struct SimpleBuilderTask<TYPES: NodeType> {
    #[allow(clippy::type_complexity)]
    transactions: Arc<RwLock<HashMap<Commitment<TYPES::Transaction>, SubmittedTransaction<TYPES>>>>,
    blocks: Arc<RwLock<HashMap<BuilderCommitment, BlockEntry<TYPES>>>>,
    decided_transactions: LruCache<Commitment<TYPES::Transaction>, ()>,
}

pub trait BuilderTask<TYPES: NodeType>: Send + Sync {
    fn start(
        self: Box<Self>,
        stream: Box<dyn Stream<Item = Event<TYPES>> + std::marker::Unpin + Send + 'static>,
    );
}

impl<TYPES: NodeType> BuilderTask<TYPES> for SimpleBuilderTask<TYPES> {
    fn start(
        mut self: Box<Self>,
        mut stream: Box<dyn Stream<Item = Event<TYPES>> + std::marker::Unpin + Send + 'static>,
    ) {
        async_spawn(async move {
            loop {
                match stream.next().await {
                    None => {
                        break;
                    }
                    Some(evt) => match evt.event {
                        EventType::Decide { leaf_chain, .. } => {
                            let mut queue = self.transactions.write().await;
                            for leaf_info in leaf_chain.iter() {
                                if let Some(ref payload) = leaf_info.leaf.get_block_payload() {
                                    for txn in payload.transaction_commitments(
                                        leaf_info.leaf.get_block_header().metadata(),
                                    ) {
                                        self.decided_transactions.put(txn, ());
                                        queue.remove(&txn);
                                    }
                                }
                            }
                            self.blocks.write().await.clear();
                        }
                        EventType::DAProposal { proposal, .. } => {
                            let payload = TYPES::BlockPayload::from_bytes(
                                proposal.data.encoded_transactions.into_iter(),
                                &proposal.data.metadata,
                            );
                            let now = Instant::now();

                            let mut queue = self.transactions.write().await;
                            for commitment in
                                payload.transaction_commitments(&proposal.data.metadata)
                            {
                                if let Some(txn) = queue.get_mut(&commitment) {
                                    txn.claimed = Some(now);
                                }
                            }
                        }
                        EventType::Transactions { transactions } => {
                            let mut queue = self.transactions.write().await;
                            for transaction in transactions {
                                if !self.decided_transactions.contains(&transaction.commit()) {
                                    queue.insert(
                                        transaction.commit(),
                                        SubmittedTransaction {
                                            claimed: None,
                                            transaction: transaction.clone(),
                                        },
                                    );
                                }
                            }
                        }
                        _ => {}
                    },
                }
            }
        });
    }
}

pub async fn make_simple_builder<TYPES: NodeType>(
    num_storage_nodes: usize,
) -> (SimpleBuilderSource<TYPES>, SimpleBuilderTask<TYPES>) {
    let (pub_key, priv_key) = TYPES::BuilderSignatureKey::generated_from_seed_indexed([1; 32], 0);

    let transactions = Arc::new(RwLock::new(HashMap::new()));
    let blocks = Arc::new(RwLock::new(HashMap::new()));

    let source = SimpleBuilderSource {
        pub_key,
        priv_key,
        transactions: transactions.clone(),
        blocks: blocks.clone(),
        num_storage_nodes,
    };

    let task = SimpleBuilderTask {
        transactions,
        blocks,
        decided_transactions: LruCache::new(NonZeroUsize::new(u16::MAX.into()).expect("> 0")),
    };

    (source, task)
}

/// Helper function to construct all builder data structures from a list of transactions
fn build_block<TYPES: NodeType>(
    transactions: Vec<TYPES::Transaction>,
    num_storage_nodes: usize,
    pub_key: TYPES::BuilderSignatureKey,
    priv_key: <TYPES::BuilderSignatureKey as BuilderSignatureKey>::BuilderPrivateKey,
) -> (
    AvailableBlockInfo<TYPES>,
    AvailableBlockData<TYPES>,
    AvailableBlockHeaderInput<TYPES>,
) {
    let (block_payload, metadata) = TYPES::BlockPayload::from_transactions(transactions)
        .expect("failed to build block payload from transactions");

    let commitment = block_payload.builder_commitment(&metadata);

    let (vid_commitment, precompute_data) = precompute_vid_commitment(
        &block_payload.encode().unwrap().collect(),
        num_storage_nodes,
    );

    // Get block size from the encoded payload
    let block_size = block_payload
        .encode()
        .expect("failed to encode block")
        .collect::<Vec<u8>>()
        .len() as u64;

    let signature_over_block_info = {
        let mut block_info: Vec<u8> = Vec::new();
        block_info.extend_from_slice(block_size.to_be_bytes().as_ref());
        block_info.extend_from_slice(123_u64.to_be_bytes().as_ref());
        block_info.extend_from_slice(commitment.as_ref());

        match TYPES::BuilderSignatureKey::sign_builder_message(&priv_key, &block_info) {
            Ok(sig) => sig,
            Err(e) => {
                panic!("Failed to sign block: {}", e);
            }
        }
    };

    let signature_over_builder_commitment =
        match TYPES::BuilderSignatureKey::sign_builder_message(&priv_key, commitment.as_ref()) {
            Ok(sig) => sig,
            Err(e) => {
                panic!("Failed to sign block: {}", e);
            }
        };

    let signature_over_vid_commitment = match TYPES::BuilderSignatureKey::sign_builder_message(
        &priv_key,
        vid_commitment.as_ref(),
    ) {
        Ok(sig) => sig,
        Err(e) => {
            panic!("Failed to sign block: {}", e);
        }
    };

    let signature_over_fee_info = {
        let mut fee_info: Vec<u8> = Vec::new();
        fee_info.extend_from_slice(123_u64.to_be_bytes().as_ref());
        fee_info.extend_from_slice(commitment.as_ref());
        fee_info.extend_from_slice(vid_commitment.as_ref());
        match TYPES::BuilderSignatureKey::sign_builder_message(&priv_key, &fee_info) {
            Ok(sig) => sig,
            Err(e) => {
                panic!("Failed to sign block: {}", e);
            }
        }
    };

    let block = AvailableBlockData {
        block_payload,
        metadata,
        sender: pub_key.clone(),
        signature: signature_over_builder_commitment,
    };
    let metadata = AvailableBlockInfo {
        sender: pub_key.clone(),
        signature: signature_over_block_info,
        block_hash: commitment,
        block_size,
        offered_fee: 123,
        _phantom: std::marker::PhantomData,
    };
    let header_input = AvailableBlockHeaderInput {
        vid_commitment,
        vid_precompute_data: precompute_data,
        message_signature: signature_over_vid_commitment.clone(),
        fee_signature: signature_over_fee_info,
        sender: pub_key,
        _phantom: std::marker::PhantomData,
    };

    (metadata, block, header_input)
}
