// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the HotShot Query Service library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

use super::Provider;
use crate::{
    availability::{LeafQueryData, PayloadQueryData},
    fetching::request::{LeafRequest, PayloadRequest},
    Error, Payload,
};
use hotshot_types::traits::node_implementation::NodeType;
use surf_disco::{Client, Url};

/// Data availability provider backed by another instance of this query service.
///
/// This fetcher implements the [`Provider`] interface by querying the REST API provided by another
/// instance of this query service to try and retrieve missing objects.
#[derive(Clone, Debug)]
pub struct QueryServiceProvider {
    client: Client<Error>,
}

impl QueryServiceProvider {
    pub async fn new(url: Url) -> Self {
        let client = Client::new(url);
        client.connect(None).await;
        Self { client }
    }
}

impl<Types> Provider<Types, PayloadRequest> for QueryServiceProvider
where
    Types: NodeType,
{
    async fn fetch(&self, req: PayloadRequest) -> Option<Payload<Types>> {
        match self
            .client
            .get::<PayloadQueryData<Types>>(&format!("availability/payload/hash/{}", req.0))
            .send()
            .await
        {
            Ok(payload) => {
                // TODO Verify that the data we retrieved is consistent with the request we made.
                // https://github.com/EspressoSystems/hotshot-query-service/issues/355
                Some(payload.data)
            }
            Err(err) => {
                tracing::error!("failed to fetch payload {req:?}: {err}");
                None
            }
        }
    }
}

impl<Types> Provider<Types, LeafRequest> for QueryServiceProvider
where
    Types: NodeType,
{
    async fn fetch(&self, req: LeafRequest) -> Option<LeafQueryData<Types>> {
        match self
            .client
            .get(&format!("availability/leaf/{}", usize::from(req)))
            .send()
            .await
        {
            Ok(leaf) => {
                // TODO we should also download a chain of QCs justifying the inclusion of `leaf` in
                // the chain at the requested height. However, HotShot currently lacks a good light
                // client API to verify this chain, so for now we just trust the other server.
                // https://github.com/EspressoSystems/HotShot/issues/2137
                // https://github.com/EspressoSystems/hotshot-query-service/issues/354
                Some(leaf)
            }
            Err(err) => {
                tracing::error!("failed to fetch leaf {req:?}: {err}");
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        availability::{define_api, AvailabilityDataSource, UpdateAvailabilityData},
        data_source::{storage::sql::testing::TmpDb, VersionedDataSource},
        fetching::provider::{NoFetching, TestProvider},
        testing::{
            consensus::{MockDataSource, MockNetwork},
            mocks::mock_transaction,
            setup_test, sleep,
        },
    };
    use async_std::task::spawn;
    use commit::Committable;
    use futures::{future::join, stream::StreamExt};
    use portpicker::pick_unused_port;
    use std::{future::IntoFuture, time::Duration};
    use tide_disco::App;

    type Provider = TestProvider<QueryServiceProvider>;

    fn ignore<T>(_: T) {}

    #[async_std::test]
    async fn test_fetch_on_request() {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<MockDataSource>::init().await;

        // Start a web server that the non-consensus node can use to fetch blocks.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.data_source());
        app.register_module("availability", define_api(&Default::default()).unwrap())
            .unwrap();
        spawn(app.serve(format!("0.0.0.0:{port}")));

        // Start a data source which is not receiving events from consensus, only from a peer.
        let db = TmpDb::init().await;
        let provider = Provider::new(
            QueryServiceProvider::new(format!("http://localhost:{port}").parse().unwrap()).await,
        );
        let mut data_source = db.config().connect(provider.clone()).await.unwrap();

        // Start consensus.
        network.start().await;

        // Wait until the block height reaches 5. This gives us the genesis block, one additional
        // block at the end, and then one block to play around with fetching each type of resource:
        // * Leaf
        // * Block
        // * Payload
        let leaves = { network.data_source().read().await.subscribe_leaves(1).await };
        let leaves = leaves.take(4).collect::<Vec<_>>().await;
        let test_leaf = &leaves[0];
        let test_block = &leaves[1];
        let test_payload = &leaves[2];

        // Make requests for missing data that should _not_ trigger an active fetch:
        tracing::info!("requesting unfetchable resources");
        let mut fetches = vec![];
        // * An unknown leaf hash.
        fetches.push(data_source.get_leaf(test_leaf.hash()).await.map(ignore));
        // * An unknown leaf height.
        fetches.push(
            data_source
                .get_leaf(test_leaf.height() as usize)
                .await
                .map(ignore),
        );
        // * An unknown block hash.
        fetches.push(
            data_source
                .get_block(test_block.block_hash())
                .await
                .map(ignore),
        );
        fetches.push(
            data_source
                .get_payload(test_payload.block_hash())
                .await
                .map(ignore),
        );
        // * An unknown block height.
        fetches.push(
            data_source
                .get_block(test_block.height() as usize)
                .await
                .map(ignore),
        );
        fetches.push(
            data_source
                .get_payload(test_payload.height() as usize)
                .await
                .map(ignore),
        );
        // * An unknown transaction.
        fetches.push(
            data_source
                .get_block_with_transaction(mock_transaction(vec![]).commit())
                .await
                .map(ignore),
        );

        // Even if we give data extra time to propagate, these requests will not resolve, since we
        // didn't trigger any active fetches.
        sleep(Duration::from_secs(1)).await;
        for (i, fetch) in fetches.into_iter().enumerate() {
            tracing::info!("checking fetch {i} is unresolved");
            fetch.try_resolve().unwrap_err();
        }

        // Now we will actually fetch the missing data. First, since our node is not really
        // connected to consensus, we need to give it a leaf after the range of interest so it
        // learns about the correct block height.
        data_source
            .insert_leaf(leaves.last().cloned().unwrap())
            .await
            .unwrap();
        data_source.commit().await.unwrap();

        // Block requests to the provider so that we can verify that without the provider, the node
        // does _not_ get the data.
        provider.block().await;

        tracing::info!("requesting fetchable resources");
        let req_leaf = data_source.get_leaf(test_leaf.height() as usize).await;
        let req_block = data_source.get_block(test_block.height() as usize).await;
        let req_payload = data_source
            .get_payload(test_payload.height() as usize)
            .await;

        // Give the requests some extra time to complete, and check that they still haven't
        // resolved, since the provider is blocked. This just ensures the integrity of the test by
        // checking the node didn't mysteriously get the block from somewhere else, so that when we
        // unblock the provider and the node finally gets the block, we know it came from the
        // provider.
        sleep(Duration::from_secs(1)).await;
        req_leaf.try_resolve().unwrap_err();
        req_block.try_resolve().unwrap_err();
        req_payload.try_resolve().unwrap_err();

        // Unblock the request and see that we eventually receive the data.
        provider.unblock().await;
        let leaf = data_source
            .get_leaf(test_leaf.height() as usize)
            .await
            .await;
        let block = data_source
            .get_block(test_block.height() as usize)
            .await
            .await;
        let payload = data_source
            .get_payload(test_payload.height() as usize)
            .await
            .await;
        {
            // Verify the data.
            let truth = network.data_source();
            let truth = truth.read().await;
            assert_eq!(
                leaf,
                truth.get_leaf(test_leaf.height() as usize).await.await
            );
            assert_eq!(
                block,
                truth.get_block(test_block.height() as usize).await.await
            );
            assert_eq!(
                payload,
                truth
                    .get_payload(test_payload.height() as usize)
                    .await
                    .await
            );
        }

        // Fetching the block and payload should have also fetched the corresponding leaves, since
        // we have an invariant that we should not store a block in the database without its
        // corresponding leaf and header. Thus we should be able to get the leaves even if the
        // provider is blocked.
        provider.block().await;
        for leaf in [test_block, test_payload] {
            tracing::info!("fetching existing leaf {}", leaf.height());
            let fetched_leaf = data_source.get_leaf(leaf.height() as usize).await.await;
            assert_eq!(*leaf, fetched_leaf);
        }

        // On the other hand, fetching the block corresponding to `leaf` _will_ trigger a fetch,
        // since fetching a leaf does not necessarily fetch the corresponding block. We can fetch by
        // hash now, since the presence of the corresponding leaf allows us to confirm that a block
        // with this hash exists, and trigger a fetch for it.
        tracing::info!("fetching block by hash");
        provider.unblock().await;
        {
            let block = data_source.get_block(test_leaf.block_hash()).await.await;
            assert_eq!(block.hash(), leaf.block_hash());
        }

        // Test a similar scenario, but with payload instead of block: we are aware of
        // `leaves.last()` but not the corresponding payload, but we can fetch that payload by block
        // hash.
        tracing::info!("fetching payload by hash");
        {
            let leaf = leaves.last().unwrap();
            let payload = data_source.get_payload(leaf.block_hash()).await.await;
            assert_eq!(payload.height(), leaf.height());
            assert_eq!(payload.block_hash(), leaf.block_hash());
            assert_eq!(payload.hash(), leaf.payload_hash());
        }
    }

    #[async_std::test]
    async fn test_fetch_block_and_leaf_concurrently() {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<MockDataSource>::init().await;

        // Start a web server that the non-consensus node can use to fetch blocks.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.data_source());
        app.register_module("availability", define_api(&Default::default()).unwrap())
            .unwrap();
        spawn(app.serve(format!("0.0.0.0:{port}")));

        // Start a data source which is not receiving events from consensus, only from a peer.
        let db = TmpDb::init().await;
        let provider = Provider::new(
            QueryServiceProvider::new(format!("http://localhost:{port}").parse().unwrap()).await,
        );
        let mut data_source = db.config().connect(provider.clone()).await.unwrap();

        // Start consensus.
        network.start().await;

        // Wait until the block height reaches 3. This gives us the genesis block, one additional
        // block at the end, and then one block that we can use to test fetching.
        let leaves = { network.data_source().read().await.subscribe_leaves(1).await };
        let leaves = leaves.take(2).collect::<Vec<_>>().await;
        let test_leaf = &leaves[0];

        // Tell the node about a leaf after the one of interest so it learns about the block height.
        data_source.insert_leaf(leaves[1].clone()).await.unwrap();
        data_source.commit().await.unwrap();

        // Fetch a leaf and the corresponding block at the same time. This will result in two tasks
        // trying to fetch the same leaf, but one should win and notify the other, which ultimately
        // ends up not fetching anything.
        let (leaf, block) = join(
            data_source
                .get_leaf(test_leaf.height() as usize)
                .await
                .into_future(),
            data_source
                .get_block(test_leaf.height() as usize)
                .await
                .into_future(),
        )
        .await;
        assert_eq!(leaf, *test_leaf);
        assert_eq!(leaf.header(), block.header());
    }

    #[async_std::test]
    async fn test_fetch_different_blocks_same_payload() {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<MockDataSource>::init().await;

        // Start a web server that the non-consensus node can use to fetch blocks.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.data_source());
        app.register_module("availability", define_api(&Default::default()).unwrap())
            .unwrap();
        spawn(app.serve(format!("0.0.0.0:{port}")));

        // Start a data source which is not receiving events from consensus, only from a peer.
        let db = TmpDb::init().await;
        let provider = Provider::new(
            QueryServiceProvider::new(format!("http://localhost:{port}").parse().unwrap()).await,
        );
        let mut data_source = db.config().connect(provider.clone()).await.unwrap();

        // Start consensus.
        network.start().await;

        // Wait until the block height reaches 4. This gives us the genesis block, one additional
        // block at the end, and then two blocks that we can use to test fetching.
        let leaves = { network.data_source().read().await.subscribe_leaves(1).await };
        let leaves = leaves.take(4).collect::<Vec<_>>().await;

        // Tell the node about a leaf after the range of interest so it learns about the block
        // height.
        data_source
            .insert_leaf(leaves.last().cloned().unwrap())
            .await
            .unwrap();
        data_source.commit().await.unwrap();

        // All the blocks here are empty, so they have the same payload:
        assert_eq!(leaves[0].payload_hash(), leaves[1].payload_hash());
        // If we fetch both blocks at the same time, we can check that we haven't broken anything
        // with whatever optimizations we add to deduplicate payload fetching.
        let (block1, block2) = join(
            data_source
                .get_block(leaves[0].height() as usize)
                .await
                .into_future(),
            data_source
                .get_block(leaves[1].height() as usize)
                .await
                .into_future(),
        )
        .await;
        assert_eq!(block1.header(), leaves[0].header());
        assert_eq!(block2.header(), leaves[1].header());
    }

    #[async_std::test]
    async fn test_fetch_stream() {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<MockDataSource>::init().await;

        // Start a web server that the non-consensus node can use to fetch blocks.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.data_source());
        app.register_module("availability", define_api(&Default::default()).unwrap())
            .unwrap();
        spawn(app.serve(format!("0.0.0.0:{port}")));

        // Start a data source which is not receiving events from consensus, only from a peer.
        let db = TmpDb::init().await;
        let provider = Provider::new(
            QueryServiceProvider::new(format!("http://localhost:{port}").parse().unwrap()).await,
        );
        let mut data_source = db.config().connect(provider.clone()).await.unwrap();

        // Start consensus.
        network.start().await;

        // Subscribe to blocks and leaves from the future.
        let blocks = data_source.subscribe_blocks(0).await;
        let leaves = data_source.subscribe_leaves(0).await;

        // Wait for a few blocks to be finalized.
        let finalized_leaves = { network.data_source().read().await.subscribe_leaves(0).await };
        let finalized_leaves = finalized_leaves.take(5).collect::<Vec<_>>().await;

        // Tell the node about a leaf after the range of interest so it learns about the block
        // height.
        data_source
            .insert_leaf(finalized_leaves.last().cloned().unwrap())
            .await
            .unwrap();
        data_source.commit().await.unwrap();

        // Check the subscriptions.
        let blocks = blocks.take(5).collect::<Vec<_>>().await;
        let leaves = leaves.take(5).collect::<Vec<_>>().await;
        for i in 0..5 {
            tracing::info!("checking block {i}");
            assert_eq!(leaves[i], finalized_leaves[i]);
            assert_eq!(blocks[i].header(), finalized_leaves[i].header());
        }
    }

    #[async_std::test]
    async fn fetch_transaction() {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<MockDataSource>::init().await;

        // Start a web server that the non-consensus node can use to fetch blocks.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.data_source());
        app.register_module("availability", define_api(&Default::default()).unwrap())
            .unwrap();
        spawn(app.serve(format!("0.0.0.0:{port}")));

        // Start a data source which is not receiving events from consensus.
        let db = TmpDb::init().await;
        let mut data_source = db.config().connect(NoFetching).await.unwrap();

        // Subscribe to blocks.
        let mut leaves = { network.data_source().read().await.subscribe_leaves(1).await };
        let mut blocks = { network.data_source().read().await.subscribe_blocks(1).await };

        // Start consensus.
        network.start().await;

        // Subscribe to a transaction which hasn't been sequenced yet. This is completely passive
        // and works without a fetcher; we don't trigger fetches for transactions that we don't know
        // exist.
        let tx = mock_transaction(vec![1, 2, 3]);
        let fut = data_source.get_block_with_transaction(tx.commit()).await;

        // Sequence the transaction.
        network.submit_transaction(tx.clone()).await;

        // Send blocks to the query service, the future will resolve as soon as it sees a block
        // containing the transaction.
        let block = loop {
            let leaf = leaves.next().await.unwrap();
            let block = blocks.next().await.unwrap();

            data_source.insert_leaf(leaf).await.unwrap();
            data_source.insert_block(block.clone()).await.unwrap();
            data_source.commit().await.unwrap();

            if block.transaction_by_hash(tx.commit()).is_some() {
                break block;
            }
        };
        tracing::info!("transaction included in block {}", block.height());

        let (fetched_block, tx_ix) = fut.await;
        assert_eq!(block, fetched_block);
        assert_eq!(
            tx,
            *fetched_block.transaction(&tx_ix).unwrap().transaction()
        );

        // Future queries for this transaction resolve immediately.
        assert_eq!(
            (fetched_block, tx_ix),
            data_source
                .get_block_with_transaction(tx.commit())
                .await
                .await
        );
    }
}
