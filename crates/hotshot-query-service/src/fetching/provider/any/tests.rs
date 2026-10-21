use futures::stream::StreamExt;

use super::{sql::testing::TmpDb, *};
use crate::{
    availability::{AvailabilityDataSource, UpdateAvailabilityData},
    fetching::provider::{NoFetching, TrustedQueryServiceProvider, test_fixtures},
    testing::{chain::ChainNode, consensus::MockDataSource, mocks::MockTypes},
    types::HeightIndexed,
};

type Provider = AnyProvider<MockTypes>;

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_fetch_first_provider_fails() {
    // Create a peer that has already decided some blocks.
    let mut network = ChainNode::<MockDataSource>::new().await;
    network.push_empty(3).await;

    // Start a web server that the non-consensus node can use to fetch blocks.
    let (port, _server) = test_fixtures::serve_availability(network.data_source()).await;

    // Start a data source which is not receiving events from consensus, only from a peer.
    let db = TmpDb::init().await;
    let provider = Provider::default().with_provider(NoFetching).with_provider(
        TrustedQueryServiceProvider::new(format!("http://localhost:{port}").parse().unwrap()),
    );
    let data_source = db.config().connect(provider.clone()).await.unwrap();

    // Wait until the block height reaches 4. This gives us the genesis block, one additional
    // block at the end, and then one block each for fetching a leaf and a payload.
    let leaves = network.data_source().subscribe_leaves(1).await;
    let leaves = leaves.take(3).collect::<Vec<_>>().await;
    let test_leaf = &leaves[0];
    let test_payload = &leaves[1];

    // Give the node a leaf after the range of interest so it learns about the correct block
    // height.
    data_source
        .append(leaves.last().cloned().unwrap().into())
        .await
        .unwrap();

    tracing::info!("requesting leaf from multiple providers");
    let leaf = data_source
        .get_leaf(test_leaf.height() as usize)
        .await
        .await;
    assert_eq!(leaf, *test_leaf);

    tracing::info!("requesting payload from multiple providers");
    let payload = data_source
        .get_payload(test_payload.height() as usize)
        .await
        .await;
    assert_eq!(payload.height(), test_payload.height());
    assert_eq!(payload.block_hash(), test_payload.block_hash());
    assert_eq!(payload.hash(), test_payload.payload_hash());
}
