//! A payload the node appends itself within the local payload grace is not fetched from a peer.

use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use hotshot_example_types::node_types::TEST_VERSIONS;
use tokio::time::{sleep, timeout};

use crate::{
    availability::{
        AvailabilityDataSource, BlockId, BlockInfo, BlockQueryData, LeafQueryData,
        UpdateAvailabilityData,
    },
    data_source::{
        SqlDataSource, VersionedDataSource, sql::testing::TmpDb, storage::AvailabilityStorage,
    },
    fetching::{
        Provider,
        provider::{AnyProvider, TestProvider},
        request::PayloadRequest,
    },
    testing::mocks::{MockPayload, MockTypes, mock_transaction},
};

type Leaf = LeafQueryData<MockTypes>;
type Block = BlockQueryData<MockTypes>;
type Ds = SqlDataSource<MockTypes, AnyProvider<MockTypes>>;

const GRACE: Duration = Duration::from_secs(1);

/// Serves one payload and counts the requests.
#[derive(Debug)]
struct ServePayload {
    payload: MockPayload,
    requests: Arc<AtomicUsize>,
}

#[async_trait]
impl Provider<MockTypes, PayloadRequest> for ServePayload {
    async fn fetch(&self, _req: PayloadRequest) -> Option<MockPayload> {
        self.requests.fetch_add(1, Ordering::SeqCst);
        Some(self.payload.clone())
    }
}

struct Setup {
    // Dropping the database deletes it.
    _db: TmpDb,
    ds: Ds,
    provider: TestProvider<ServePayload>,
    requests: Arc<AtomicUsize>,
    leaf: Leaf,
    block: Block,
}

impl Setup {
    /// A data source with `grace`, a decided leaf at height 1 proposed at `timestamp_millis`, and
    /// a provider that serves its payload.
    async fn new(grace: Duration, timestamp_millis: u64) -> Self {
        let db = TmpDb::init().await.with_blobs();
        let (leaf, block) = block_at(1, timestamp_millis).await;
        let requests = Arc::new(AtomicUsize::new(0));
        let provider = TestProvider::new(ServePayload {
            payload: block.payload().clone(),
            requests: requests.clone(),
        });
        let ds = db
            .config()
            .builder(AnyProvider::default().with_block_provider(provider.clone()))
            .await
            .unwrap()
            .disable_proactive_fetching()
            // Its block stream would fetch the same payload again.
            .disable_aggregator()
            .with_local_payload_grace(grace)
            .build()
            .await
            .unwrap();
        Self {
            _db: db,
            ds,
            provider,
            requests,
            leaf,
            block,
        }
    }

    /// Decide the leaf without its payload, as a decide that beats reconstruction does.
    async fn decide_without_payload(&self) {
        self.ds
            .append(BlockInfo::new(self.leaf.clone(), None, None, None))
            .await
            .unwrap();
    }

    async fn append_local_payload(&self) {
        self.ds.append_payload(self.block.clone()).await.unwrap();
    }

    fn requests(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
    }

    async fn stored_block(&self) -> Option<Block> {
        let mut tx = self.ds.read().await.unwrap();
        tx.get_block(BlockId::Number(1)).await.ok()
    }

    async fn wait_stored(&self) -> Block {
        timeout(Duration::from_secs(10), async {
            loop {
                if let Some(block) = self.stored_block().await {
                    return block;
                }
                sleep(Duration::from_millis(50)).await;
            }
        })
        .await
        .expect("block was not stored")
    }

    fn payload_bytes(&self) -> u64 {
        self.ds.blob_store().unwrap().payload_bytes()
    }
}

async fn block_at(height: u64, timestamp_millis: u64) -> (Leaf, Block) {
    let mut leaf =
        Leaf::genesis(&Default::default(), &Default::default(), TEST_VERSIONS.test).await;
    let header = leaf.leaf.block_header_mut();
    header.block_number = height;
    header.metadata.num_transactions = 1;
    header.timestamp = timestamp_millis / 1000;
    header.timestamp_millis = timestamp_millis;
    let payload = MockPayload {
        transactions: vec![mock_transaction(vec![1, 2, 3])],
    };
    let block = Block::new(leaf.header().clone(), payload);
    (leaf, block)
}

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn local_payload_within_grace_skips_peer_fetch() {
    let setup = Setup::new(GRACE, now_millis()).await;
    setup.decide_without_payload().await;
    sleep(Duration::from_millis(100)).await;
    setup.append_local_payload().await;

    sleep(GRACE + Duration::from_millis(500)).await;
    assert_eq!(setup.requests(), 0);
    assert_eq!(setup.stored_block().await, Some(setup.block.clone()));
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn user_get_within_grace_waits_for_local_payload() {
    let setup = Setup::new(GRACE, now_millis()).await;
    setup.decide_without_payload().await;
    let fetch = setup.ds.get_block(1).await;
    sleep(Duration::from_millis(100)).await;
    setup.append_local_payload().await;

    let block = timeout(GRACE, fetch)
        .await
        .expect("GET did not resolve from the local payload");
    assert_eq!(block, setup.block);
    sleep(GRACE).await;
    assert_eq!(setup.requests(), 0);
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn missing_local_payload_fetches_after_grace() {
    let setup = Setup::new(GRACE, now_millis()).await;
    setup.decide_without_payload().await;

    sleep(GRACE / 2).await;
    assert_eq!(setup.requests(), 0, "fetched from a peer within the grace");
    assert_eq!(setup.wait_stored().await, setup.block);
    assert_eq!(setup.requests(), 1);
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn old_header_fetches_immediately() {
    let grace = Duration::from_secs(60);
    let setup = Setup::new(grace, now_millis() - 2 * grace.as_millis() as u64).await;
    setup.decide_without_payload().await;

    assert_eq!(setup.wait_stored().await, setup.block);
    assert_eq!(setup.requests(), 1);
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn fetched_payload_already_stored_not_rewritten() {
    let setup = Setup::new(Duration::ZERO, now_millis()).await;
    setup.provider.block().await;
    setup.decide_without_payload().await;
    // The peer fetch is now in flight, held in the provider.
    sleep(Duration::from_millis(200)).await;
    setup.append_local_payload().await;
    let bytes = setup.payload_bytes();
    assert!(bytes > 0);

    setup.provider.unblock().await;
    timeout(Duration::from_secs(10), async {
        while setup.requests() == 0 {
            sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("peer fetch did not complete");
    sleep(Duration::from_millis(500)).await;
    assert_eq!(
        setup.payload_bytes(),
        bytes,
        "fetched payload was written again"
    );
    assert_eq!(setup.stored_block().await, Some(setup.block.clone()));
}
