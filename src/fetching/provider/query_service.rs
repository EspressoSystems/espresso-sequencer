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
    availability::{LeafQueryData, PayloadQueryData, VidCommonQueryData},
    fetching::request::{LeafRequest, PayloadRequest, VidCommonRequest},
    Error, Payload, VidCommon,
};
use async_trait::async_trait;
use futures::try_join;
use hotshot_types::{
    traits::{node_implementation::NodeType, EncodeBytes},
    vid::{vid_scheme, VidSchemeType},
};
use jf_vid::VidScheme;
use surf_disco::{Client, Url};
use vbs::version::StaticVersionType;

/// Data availability provider backed by another instance of this query service.
///
/// This fetcher implements the [`Provider`] interface by querying the REST API provided by another
/// instance of this query service to try and retrieve missing objects.
#[derive(Clone, Debug)]
pub struct QueryServiceProvider<Ver: StaticVersionType> {
    client: Client<Error, Ver>,
}

impl<Ver: StaticVersionType> QueryServiceProvider<Ver> {
    pub fn new(url: Url, _: Ver) -> Self {
        Self {
            client: Client::new(url),
        }
    }
}

#[async_trait]
impl<Types, Ver: StaticVersionType> Provider<Types, PayloadRequest> for QueryServiceProvider<Ver>
where
    Types: NodeType,
{
    async fn fetch(&self, req: PayloadRequest) -> Option<Payload<Types>> {
        // Fetch the payload and the VID common data. We need the common data to recompute the VID
        // commitment, to ensure the payload we received is consistent with the commitment we
        // requested.
        let res = try_join!(
            self.client
                .get::<PayloadQueryData<Types>>(&format!("availability/payload/hash/{}", req.0))
                .send(),
            self.client
                .get::<VidCommonQueryData<Types>>(&format!(
                    "availability/vid/common/payload-hash/{}",
                    req.0
                ))
                .send()
        );
        match res {
            Ok((payload, common)) => {
                // Verify that the data we retrieved is consistent with the request we made.
                let num_storage_nodes =
                    VidSchemeType::get_num_storage_nodes(common.common()) as usize;
                let bytes = payload.data().encode();
                let commit = match vid_scheme(num_storage_nodes).commit_only(bytes) {
                    Ok(commit) => commit,
                    Err(err) => {
                        tracing::error!(%err, "unable to compute VID commitment");
                        return None;
                    }
                };
                if commit != req.0 {
                    tracing::error!(?req, ?commit, "received inconsistent payload");
                    return None;
                }

                Some(payload.data)
            }
            Err(err) => {
                tracing::error!("failed to fetch payload {req:?}: {err}");
                None
            }
        }
    }
}

#[async_trait]
impl<Types, Ver: StaticVersionType> Provider<Types, LeafRequest> for QueryServiceProvider<Ver>
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

#[async_trait]
impl<Types, Ver: StaticVersionType> Provider<Types, VidCommonRequest> for QueryServiceProvider<Ver>
where
    Types: NodeType,
{
    async fn fetch(&self, req: VidCommonRequest) -> Option<VidCommon> {
        match self
            .client
            .get::<VidCommonQueryData<Types>>(&format!(
                "availability/vid/common/payload-hash/{}",
                req.0
            ))
            .send()
            .await
        {
            Ok(res) if VidSchemeType::is_consistent(&req.0, &res.common).is_ok() => {
                Some(res.common)
            }
            Ok(res) => {
                tracing::error!(?req, ?res, "fetched inconsistent VID common data");
                None
            }
            Err(err) => {
                tracing::error!("failed to fetch VID common {req:?}: {err}");
                None
            }
        }
    }
}

// These tests run the `postgres` Docker image, which doesn't work on Windows.
#[cfg(all(test, not(target_os = "windows")))]
mod test {
    use super::*;

    use crate::{
        api::load_api,
        availability::{
            define_api, AvailabilityDataSource, Fetch, TransactionQueryData, UpdateAvailabilityData,
        },
        data_source::{
            sql::{self, SqlDataSource},
            storage::{
                pruning::{PrunedHeightStorage, PrunerCfg},
                sql::testing::TmpDb,
            },
            AvailabilityProvider, VersionedDataSource,
        },
        fetching::provider::{NoFetching, Provider as ProviderTrait, TestProvider},
        node::{data_source::NodeDataSource, SyncStatus},
        task::BackgroundTask,
        testing::{
            consensus::{MockDataSource, MockNetwork},
            mocks::{mock_transaction, MockTypes},
            setup_test, sleep,
        },
        types::HeightIndexed,
        VidCommitment,
    };
    use committable::Committable;
    use futures::{
        future::{join, FutureExt},
        stream::StreamExt,
    };
    use generic_array::GenericArray;
    use hotshot_types::constants::{Version01, STATIC_VER_0_1};
    use portpicker::pick_unused_port;
    use rand::RngCore;
    use std::{future::IntoFuture, time::Duration};
    use tide_disco::{error::ServerError, App};

    type Provider = TestProvider<QueryServiceProvider<Version01>>;

    fn ignore<T>(_: T) {}

    /// Build a data source suitable for this suite of tests.
    async fn builder<P: AvailabilityProvider<MockTypes> + Clone>(
        db: &TmpDb,
        provider: &P,
    ) -> sql::Builder<MockTypes, P> {
        db.config()
            .builder((*provider).clone())
            .await
            .unwrap()
            // We disable proactive fetching for these tests, since we are intending to test on
            // demand fetching, and proactive fetching could lead to false successes.
            .disable_proactive_fetching()
    }

    /// A data source suitable for this suite of tests, with the default options.
    async fn data_source<P: AvailabilityProvider<MockTypes> + Clone>(
        db: &TmpDb,
        provider: &P,
    ) -> SqlDataSource<MockTypes, P> {
        builder(db, provider).await.build().await.unwrap()
    }

    #[async_std::test]
    async fn test_fetch_on_request() {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<MockDataSource>::init().await;

        // Start a web server that the non-consensus node can use to fetch blocks.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.data_source());
        app.register_module(
            "availability",
            define_api(&Default::default(), STATIC_VER_0_1).unwrap(),
        )
        .unwrap();
        network.spawn(
            "server",
            app.serve(format!("0.0.0.0:{port}"), STATIC_VER_0_1),
        );

        // Start a data source which is not receiving events from consensus, only from a peer.
        let db = TmpDb::init().await;
        let provider = Provider::new(QueryServiceProvider::new(
            format!("http://localhost:{port}").parse().unwrap(),
            STATIC_VER_0_1,
        ));
        let mut data_source = data_source(&db, &provider).await;

        // Start consensus.
        network.start().await;

        // Wait until the block height reaches 6. This gives us the genesis block, one additional
        // block at the end, and then one block to play around with fetching each type of resource:
        // * Leaf
        // * Block
        // * Payload
        // * VID common
        let leaves = { network.data_source().read().await.subscribe_leaves(1).await };
        let leaves = leaves.take(5).collect::<Vec<_>>().await;
        let test_leaf = &leaves[0];
        let test_block = &leaves[1];
        let test_payload = &leaves[2];
        let test_common = &leaves[3];

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
        fetches.push(
            data_source
                .get_vid_common(test_common.block_hash())
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
        fetches.push(
            data_source
                .get_vid_common(test_common.height() as usize)
                .await
                .map(ignore),
        );
        // * Genesis VID common (no VID for genesis)
        fetches.push(data_source.get_vid_common(0).await.map(ignore));
        // * An unknown transaction.
        fetches.push(
            data_source
                .get_transaction(mock_transaction(vec![]).commit())
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
        let req_common = data_source
            .get_vid_common(test_common.height() as usize)
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
        req_common.try_resolve().unwrap_err();

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
        let common = data_source
            .get_vid_common(test_common.height() as usize)
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
            assert_eq!(
                common,
                truth
                    .get_vid_common(test_common.height() as usize)
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
        app.register_module(
            "availability",
            define_api(&Default::default(), STATIC_VER_0_1).unwrap(),
        )
        .unwrap();
        network.spawn(
            "server",
            app.serve(format!("0.0.0.0:{port}"), STATIC_VER_0_1),
        );

        // Start a data source which is not receiving events from consensus, only from a peer.
        let db = TmpDb::init().await;
        let provider = Provider::new(QueryServiceProvider::new(
            format!("http://localhost:{port}").parse().unwrap(),
            STATIC_VER_0_1,
        ));
        let mut data_source = data_source(&db, &provider).await;

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
        app.register_module(
            "availability",
            define_api(&Default::default(), STATIC_VER_0_1).unwrap(),
        )
        .unwrap();
        network.spawn(
            "server",
            app.serve(format!("0.0.0.0:{port}"), STATIC_VER_0_1),
        );

        // Start a data source which is not receiving events from consensus, only from a peer.
        let db = TmpDb::init().await;
        let provider = Provider::new(QueryServiceProvider::new(
            format!("http://localhost:{port}").parse().unwrap(),
            STATIC_VER_0_1,
        ));
        let mut data_source = data_source(&db, &provider).await;

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
        app.register_module(
            "availability",
            define_api(&Default::default(), STATIC_VER_0_1).unwrap(),
        )
        .unwrap();
        network.spawn(
            "server",
            app.serve(format!("0.0.0.0:{port}"), STATIC_VER_0_1),
        );

        // Start a data source which is not receiving events from consensus, only from a peer.
        let db = TmpDb::init().await;
        let provider = Provider::new(QueryServiceProvider::new(
            format!("http://localhost:{port}").parse().unwrap(),
            STATIC_VER_0_1,
        ));
        let mut data_source = data_source(&db, &provider).await;

        // Start consensus.
        network.start().await;

        // Subscribe to objects from the future.
        let blocks = data_source.subscribe_blocks(0).await;
        let leaves = data_source.subscribe_leaves(0).await;
        let common = data_source.subscribe_vid_common(0).await;

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
        let common = common.take(5).collect::<Vec<_>>().await;
        for i in 0..5 {
            tracing::info!("checking block {i}");
            assert_eq!(leaves[i], finalized_leaves[i]);
            assert_eq!(blocks[i].header(), finalized_leaves[i].header());
            assert_eq!(common[i], data_source.get_vid_common(i).await.await);
        }
    }

    #[async_std::test]
    async fn test_fetch_range_start() {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<MockDataSource>::init().await;

        // Start a web server that the non-consensus node can use to fetch blocks.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.data_source());
        app.register_module(
            "availability",
            define_api(&Default::default(), STATIC_VER_0_1).unwrap(),
        )
        .unwrap();
        network.spawn(
            "server",
            app.serve(format!("0.0.0.0:{port}"), STATIC_VER_0_1),
        );

        // Start a data source which is not receiving events from consensus, only from a peer.
        let db = TmpDb::init().await;
        let provider = Provider::new(QueryServiceProvider::new(
            format!("http://localhost:{port}").parse().unwrap(),
            STATIC_VER_0_1,
        ));
        let mut data_source = data_source(&db, &provider).await;

        // Start consensus.
        network.start().await;

        // Wait for a few blocks to be finalized.
        let finalized_leaves = { network.data_source().read().await.subscribe_leaves(0).await };
        let finalized_leaves = finalized_leaves.take(5).collect::<Vec<_>>().await;

        // Tell the node about a leaf after the range of interest (so it learns about the block
        // height) and one in the middle of the range, to test what happens when data is missing
        // from the beginning of the range but other data is available.
        data_source
            .insert_leaf(finalized_leaves[2].clone())
            .await
            .unwrap();
        data_source
            .insert_leaf(finalized_leaves[4].clone())
            .await
            .unwrap();
        data_source.commit().await.unwrap();

        // Get the whole range of leaves.
        let leaves = data_source
            .get_leaf_range(..5)
            .await
            .then(Fetch::resolve)
            .collect::<Vec<_>>()
            .await;
        for i in 0..5 {
            tracing::info!("checking leaf {i}");
            assert_eq!(leaves[i], finalized_leaves[i]);
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
        app.register_module(
            "availability",
            define_api(&Default::default(), STATIC_VER_0_1).unwrap(),
        )
        .unwrap();
        network.spawn(
            "server",
            app.serve(format!("0.0.0.0:{port}"), STATIC_VER_0_1),
        );

        // Start a data source which is not receiving events from consensus. We don't give it a
        // fetcher since transactions are always fetched passively anyways.
        let db = TmpDb::init().await;
        let mut data_source = data_source(&db, &NoFetching).await;

        // Subscribe to blocks.
        let mut leaves = { network.data_source().read().await.subscribe_leaves(1).await };
        let mut blocks = { network.data_source().read().await.subscribe_blocks(1).await };

        // Start consensus.
        network.start().await;

        // Subscribe to a transaction which hasn't been sequenced yet. This is completely passive
        // and works without a fetcher; we don't trigger fetches for transactions that we don't know
        // exist.
        let tx = mock_transaction(vec![1, 2, 3]);
        let fut = data_source.get_transaction(tx.commit()).await;

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

        let fetched_tx = fut.await;
        assert_eq!(
            fetched_tx,
            TransactionQueryData::with_hash(&block, tx.commit()).unwrap()
        );

        // Future queries for this transaction resolve immediately.
        assert_eq!(
            fetched_tx,
            data_source.get_transaction(tx.commit()).await.await
        );
    }

    #[async_std::test]
    async fn test_retry() {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<MockDataSource>::init().await;

        // Start a web server that the non-consensus node can use to fetch blocks.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.data_source());
        app.register_module(
            "availability",
            define_api(&Default::default(), STATIC_VER_0_1).unwrap(),
        )
        .unwrap();
        network.spawn(
            "server",
            app.serve(format!("0.0.0.0:{port}"), STATIC_VER_0_1),
        );

        // Start a data source which is not receiving events from consensus.
        let db = TmpDb::init().await;
        let provider = Provider::new(QueryServiceProvider::new(
            format!("http://localhost:{port}").parse().unwrap(),
            STATIC_VER_0_1,
        ));
        let mut data_source = builder(&db, &provider)
            .await
            .with_retry_delay(Duration::from_secs(1))
            .build()
            .await
            .unwrap();

        // Start consensus.
        network.start().await;

        // Wait until the block height reaches 3. This gives us the genesis block, one additional
        // block at the end, and one block to try fetching.
        let leaves = { network.data_source().read().await.subscribe_leaves(1).await };
        let leaves = leaves.take(2).collect::<Vec<_>>().await;
        let test_leaf = &leaves[0];

        // Give the node a leaf after the range of interest so it learns about the correct block
        // height.
        data_source
            .insert_leaf(leaves.last().cloned().unwrap())
            .await
            .unwrap();
        data_source.commit().await.unwrap();

        // Cause requests to fail temporarily, so we can test retries.
        provider.fail();

        tracing::info!("requesting leaf from failing providers");
        let fut = data_source.get_leaf(test_leaf.height() as usize).await;

        // Wait a few retries and check that the request has not completed, since the provider is
        // failing.
        sleep(Duration::from_secs(5)).await;
        fut.try_resolve().unwrap_err();

        // As soon as the provider recovers, the request can complete.
        provider.unfail();
        assert_eq!(
            data_source
                .get_leaf(test_leaf.height() as usize)
                .await
                .await,
            *test_leaf
        );
    }

    fn random_vid_commit() -> VidCommitment {
        let mut bytes = [0; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        VidCommitment::from(GenericArray::from(bytes))
    }

    async fn malicious_server(port: u16) {
        let mut api = load_api::<(), ServerError, Version01>(
            None::<std::path::PathBuf>,
            include_str!("../../../api/availability.toml"),
            vec![],
        )
        .unwrap();

        api.get("get_payload", move |_, _| {
            async move {
                // No matter what data we are asked for, always respond with dummy data.
                Ok(
                    PayloadQueryData::<MockTypes>::genesis(
                        &Default::default(),
                        &Default::default(),
                    )
                    .await,
                )
            }
            .boxed()
        })
        .unwrap()
        .get("get_vid_common", move |_, _| {
            async move {
                // No matter what data we are asked for, always respond with dummy data.
                Ok(VidCommonQueryData::<MockTypes>::genesis(
                    &Default::default(),
                    &Default::default(),
                )
                .await)
            }
            .boxed()
        })
        .unwrap();

        let mut app = App::<(), ServerError>::with_state(());
        app.register_module("availability", api).unwrap();
        app.serve(format!("0.0.0.0:{port}"), STATIC_VER_0_1)
            .await
            .ok();
    }

    #[async_std::test]
    async fn test_fetch_from_malicious_server() {
        setup_test();

        let port = pick_unused_port().unwrap();
        let _server = BackgroundTask::spawn("malicious server", malicious_server(port));

        let provider = QueryServiceProvider::new(
            format!("http://localhost:{port}").parse().unwrap(),
            STATIC_VER_0_1,
        );
        provider.client.connect(None).await;

        // Query for a random payload, the server will respond with a different payload, and we
        // should detect the error.
        let res =
            ProviderTrait::<MockTypes, _>::fetch(&provider, PayloadRequest(random_vid_commit()))
                .await;
        assert_eq!(res, None);

        // Query for a random VID common, the server will respond with a different one, and we
        // should detect the error.
        let res =
            ProviderTrait::<MockTypes, _>::fetch(&provider, VidCommonRequest(random_vid_commit()))
                .await;
        assert_eq!(res, None);
    }

    #[async_std::test]
    async fn test_archive_recovery() {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<MockDataSource>::init().await;

        // Start a web server that the non-consensus node can use to fetch blocks.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.data_source());
        app.register_module(
            "availability",
            define_api(&Default::default(), STATIC_VER_0_1).unwrap(),
        )
        .unwrap();
        network.spawn(
            "server",
            app.serve(format!("0.0.0.0:{port}"), STATIC_VER_0_1),
        );

        // Start a data source which is not receiving events from consensus, only from a peer. The
        // data source is at first configured to aggressively prune data.
        let db = TmpDb::init().await;
        let provider = Provider::new(QueryServiceProvider::new(
            format!("http://localhost:{port}").parse().unwrap(),
            STATIC_VER_0_1,
        ));
        let mut data_source = db
            .config()
            .pruner_cfg(
                PrunerCfg::new()
                    .with_target_retention(Duration::from_secs(0))
                    .with_interval(Duration::from_secs(1)),
            )
            .unwrap()
            .connect(provider.clone())
            .await
            .unwrap();

        // Start consensus.
        network.start().await;

        // Wait until a few blocks are produced.
        let leaves = { network.data_source().read().await.subscribe_leaves(1).await };
        let leaves = leaves.take(5).collect::<Vec<_>>().await;

        // The disconnected data source has no data yet, so it hasn't done any pruning.
        let pruned_height = {
            data_source
                .storage()
                .await
                .load_pruned_height()
                .await
                .unwrap()
        };
        // Either None or 0 is acceptable, depending on whether or not the prover has run yet.
        assert!(matches!(pruned_height, None | Some(0)), "{pruned_height:?}");

        // Send the last leaf to the disconnected data source so it learns about the height and
        // fetches the missing data.
        let last_leaf = leaves.last().unwrap();
        data_source.insert_leaf(last_leaf.clone()).await.unwrap();
        data_source.commit().await.unwrap();

        // Trigger a fetch of each leaf so the database gets populated.
        for i in 1..=last_leaf.height() {
            tracing::info!(i, "fetching leaf");
            assert_eq!(
                data_source.get_leaf(i as usize).await.await,
                leaves[i as usize - 1]
            );
        }

        // After a bit of time, the pruner has run and deleted all the missing data we just fetched.
        loop {
            let pruned_height = data_source
                .storage()
                .await
                .load_pruned_height()
                .await
                .unwrap();
            if pruned_height == Some(last_leaf.height()) {
                break;
            }
            tracing::info!(
                ?pruned_height,
                target_height = last_leaf.height(),
                "waiting for pruner to run"
            );
            sleep(Duration::from_secs(1)).await;
        }

        // Now close the data source and restart it with archive recovery.
        data_source = db
            .config()
            .archive()
            .builder(provider.clone())
            .await
            .unwrap()
            .with_minor_scan_interval(Duration::from_secs(1))
            .with_major_scan_interval(1)
            .build()
            .await
            .unwrap();

        // Pruned height should be reset.
        let pruned_height = {
            data_source
                .storage()
                .await
                .load_pruned_height()
                .await
                .unwrap()
        };
        assert_eq!(pruned_height, None);

        // The node has pruned all of it's data including the latest block, so it's forgotten the
        // block height. We need to give it another leaf with some height so it will be willing to
        // fetch.
        data_source.insert_leaf(last_leaf.clone()).await.unwrap();
        data_source.commit().await.unwrap();

        // Wait for the data to be restored. It should be restored by the next major scan.
        loop {
            let sync_status = data_source.sync_status().await.await.unwrap();

            // VID shares are unique to a node and will never be fetched from a peer; this is
            // acceptable since there is redundancy built into the VID scheme. Ignore missing VID
            // shares in the `is_fully_synced` check.
            if (SyncStatus {
                missing_vid_shares: 0,
                ..sync_status
            })
            .is_fully_synced()
            {
                break;
            }
            tracing::info!(?sync_status, "waiting for node to sync");
        }

        // The node remains fully synced even after some time; no pruning.
        sleep(Duration::from_secs(3)).await;
        let sync_status = data_source.sync_status().await.await.unwrap();
        assert!(
            (SyncStatus {
                missing_vid_shares: 0,
                ..sync_status
            })
            .is_fully_synced(),
            "{sync_status:?}"
        );
    }
}
