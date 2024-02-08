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

//! Persistent storage and sources of data consumed by APIs.
//!
//! The APIs provided by this query service are generic over the implementation which actually
//! retrieves data in answer to queries. We call this implementation a _data source_. This module
//! defines a data source and provides several pre-built implementations:
//! * [`FileSystemDataSource`]
//! * [`SqlDataSource`]
//! * [`FetchingDataSource`], a generalization of the above
//! * [`MetricsDataSource`]
//!
//! The user can choose which data source to use when initializing the query service.
//!
//! We also provide combinators for modularly adding functionality to existing data sources:
//! * [`ExtensibleDataSource`]
//!

mod extension;
pub mod fetching;
mod fs;
mod metrics;
mod notifier;
pub mod sql;
pub mod storage;
mod update;

pub use extension::ExtensibleDataSource;
pub use fetching::{AvailabilityProvider, FetchingDataSource};
#[cfg(feature = "file-system-data-source")]
pub use fs::FileSystemDataSource;
#[cfg(feature = "metrics-data-source")]
pub use metrics::MetricsDataSource;
#[cfg(feature = "sql-data-source")]
pub use sql::SqlDataSource;
pub use update::{UpdateDataSource, VersionedDataSource};

#[cfg(any(test, feature = "testing"))]
mod test_helpers {
    use crate::{
        availability::{BlockQueryData, Fetch, LeafQueryData},
        node::NodeDataSource,
        testing::{consensus::TestableDataSource, mocks::MockTypes},
    };
    use async_std::sync::RwLock;
    use futures::{
        future,
        stream::{BoxStream, StreamExt},
    };
    use std::ops::{Bound, RangeBounds};

    /// Apply an upper bound to a range based on the currently available block height.
    async fn bound_range<R, D>(ds: &D, range: R) -> impl RangeBounds<usize>
    where
        D: TestableDataSource,
        R: RangeBounds<usize>,
    {
        let start = range.start_bound().cloned();
        let mut end = range.end_bound().cloned();
        if end == Bound::Unbounded {
            end = Bound::Excluded(NodeDataSource::block_height(ds).await.unwrap());
        }
        (start, end)
    }

    /// Get a stream of blocks, implicitly terminating at the current block height.
    pub async fn block_range<R, D>(
        ds: &D,
        range: R,
    ) -> BoxStream<'static, BlockQueryData<MockTypes>>
    where
        D: TestableDataSource,
        R: RangeBounds<usize> + Send + 'static,
    {
        ds.get_block_range(bound_range(ds, range).await)
            .await
            .then(Fetch::resolve)
            .boxed()
    }

    /// Get a stream of leaves, implicitly terminating at the current block height.
    pub async fn leaf_range<R, D>(ds: &D, range: R) -> BoxStream<'static, LeafQueryData<MockTypes>>
    where
        D: TestableDataSource,
        R: RangeBounds<usize> + Send + 'static,
    {
        ds.get_leaf_range(bound_range(ds, range).await)
            .await
            .then(Fetch::resolve)
            .boxed()
    }

    pub async fn get_non_empty_blocks(
        ds: &RwLock<impl TestableDataSource>,
    ) -> Vec<(LeafQueryData<MockTypes>, BlockQueryData<MockTypes>)> {
        let ds = ds.read().await;
        // Ignore the genesis block (start from height 1).
        leaf_range(&*ds, 1..)
            .await
            .zip(block_range(&*ds, 1..).await)
            .filter(|(_, block)| future::ready(!block.is_empty()))
            .collect()
            .await
    }
}

/// Generic tests we can instantiate for all the availability data sources.
#[cfg(any(test, feature = "testing"))]
#[espresso_macros::generic_tests]
pub mod availability_tests {
    use super::test_helpers::*;
    use crate::{
        availability::{payload_size, BlockId},
        node::NodeDataSource,
        testing::{
            consensus::{MockNetwork, TestableDataSource},
            mocks::{mock_transaction, MockTypes},
            setup_test,
        },
    };
    use async_std::sync::RwLock;
    use commit::Committable;
    use futures::stream::StreamExt;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::ops::{Bound, RangeBounds};

    async fn validate(ds: &RwLock<impl TestableDataSource>) {
        let ds = ds.read().await;

        // Check the consistency of every block/leaf pair. Keep track of payloads and transactions
        // we've seen so we can detect duplicates.
        let mut seen_payloads = HashMap::new();
        let mut seen_transactions = HashMap::new();
        let mut leaves = leaf_range(&*ds, ..).await.enumerate();
        while let Some((i, leaf)) = leaves.next().await {
            assert_eq!(leaf.height(), i as u64);
            assert_eq!(leaf.hash(), leaf.leaf().commit());

            // Check indices.
            tracing::info!("looking up leaf {i} various ways");
            assert_eq!(leaf, ds.get_leaf(i).await.await);
            assert_eq!(leaf, ds.get_leaf(leaf.hash()).await.await);

            tracing::info!("looking up block {i} various ways");
            let block = ds.get_block(i).await.await;
            assert_eq!(leaf.block_hash(), block.hash());
            assert_eq!(block.height(), i as u64);
            assert_eq!(block.hash(), block.header().commit());
            assert_eq!(block.size(), payload_size::<MockTypes>(block.payload()));

            // Check indices.
            assert_eq!(block, ds.get_block(i).await.await);
            assert_eq!(ds.get_block(block.hash()).await.await.height(), i as u64);
            // We should be able to look up the block by payload hash unless its payload is a
            // duplicate. For duplicate payloads, this function returns the index of the first
            // duplicate.
            //
            // Note: this ordering is not a strict requirement. It should hold for payloads in local
            // storage, but we don't have a good way of enforcing it if the payload is missing, in
            // which case we will return the first matching payload we see, which could happen in
            // any order. We use `try_resolve` to skip this check if the object isn't available
            // locally.
            let ix = seen_payloads
                .entry(block.payload_hash())
                .or_insert(i as u64);
            if let Ok(block) = ds
                .get_block(BlockId::PayloadHash(block.payload_hash()))
                .await
                .try_resolve()
            {
                assert_eq!(block.height(), *ix);
            } else {
                tracing::warn!(
                    "skipping block by payload index check for missing payload {:?}",
                    block.header()
                );
                // At least check that _some_ block can be fetched.
                ds.get_block(BlockId::PayloadHash(block.payload_hash()))
                    .await
                    .await;
            }

            // Check payload lookup.
            let expected_payload = block.clone().into();
            assert_eq!(ds.get_payload(i).await.await, expected_payload);
            assert_eq!(ds.get_payload(block.hash()).await.await, expected_payload);
            // Similar to the above, we can't guarantee which index we will get when passively
            // fetching this payload, so only check the index if the payload is available locally.
            if let Ok(payload) = ds
                .get_payload(BlockId::PayloadHash(block.payload_hash()))
                .await
                .try_resolve()
            {
                if *ix == i as u64 {
                    assert_eq!(payload, expected_payload);
                }
            } else {
                tracing::warn!(
                    "skipping payload index check for missing payload {:?}",
                    block.header()
                );
                // At least check that _some_ payload can be fetched.
                ds.get_payload(BlockId::PayloadHash(block.payload_hash()))
                    .await
                    .await;
            }

            for (j, txn) in block.enumerate() {
                tracing::info!("looking up transaction {i},{j}");

                // We should be able to look up the transaction by hash unless it is a duplicate.
                // For duplicate transactions, this function returns the index of the first
                // duplicate.
                //
                // Similar to the above, we can't guarantee which index we will get when passively
                // fetching this transaction, so only check the index if the transaction is
                // available locally.
                let ix = seen_transactions
                    .entry(txn.commit())
                    .or_insert((i as u64, j));
                if let Ok((block, pos)) = ds
                    .get_block_with_transaction(txn.commit())
                    .await
                    .try_resolve()
                {
                    assert_eq!((block.height(), pos), *ix);
                } else {
                    tracing::warn!(
                        "skipping transaction index check for missing transaction {j} {txn:?}"
                    );
                    // At least check that _some_ transaction can be fetched.
                    ds.get_block_with_transaction(txn.commit()).await.await;
                }
            }
        }
    }

    #[async_std::test]
    pub async fn test_update<D: TestableDataSource>() {
        setup_test();

        let mut network = MockNetwork::<D>::init().await;
        let ds = network.data_source();

        network.start().await;
        assert_eq!(get_non_empty_blocks(&ds).await, vec![]);

        // Submit a few blocks and make sure each one gets reflected in the query service and
        // preserves the consistency of the data and indices.
        let mut blocks = { ds.read().await.subscribe_blocks(0).await.enumerate() };
        for nonce in 0..3 {
            let txn = mock_transaction(vec![nonce]);
            network.submit_transaction(txn).await;

            // Wait for the transaction to be finalized.
            let (i, block) = loop {
                tracing::info!("waiting for block {nonce}");
                let (i, block) = blocks.next().await.unwrap();
                if !block.is_empty() {
                    break (i, block);
                }
                tracing::info!("block {i} is empty");
            };

            assert_eq!(ds.read().await.get_block(i).await.await, block);
            validate(&ds).await;
        }

        // Check that all the updates have been committed to storage, not simply held in memory: we
        // should be able to read the same data if we connect an entirely new data source to the
        // underlying storage.
        {
            tracing::info!("checking persisted storage");

            // Lock the original data source to prevent concurrent updates.
            let ds = ds.read().await;
            let storage = D::connect(network.storage()).await;

            // Ensure we have the same data in both data sources (if data was missing from the
            // original it is of course allowed to be missing from persistent storage and thus from
            // the latter).
            let block_height = NodeDataSource::block_height(&*ds).await.unwrap();
            assert_eq!(
                ds.get_block_range(..block_height)
                    .await
                    .map(|fetch| fetch.try_resolve().ok())
                    .collect::<Vec<_>>()
                    .await,
                storage
                    .get_block_range(..block_height)
                    .await
                    .map(|fetch| fetch.try_resolve().ok())
                    .collect::<Vec<_>>()
                    .await
            );
            assert_eq!(
                ds.get_leaf_range(..block_height)
                    .await
                    .map(|fetch| fetch.try_resolve().ok())
                    .collect::<Vec<_>>()
                    .await,
                storage
                    .get_leaf_range(..block_height)
                    .await
                    .map(|fetch| fetch.try_resolve().ok())
                    .collect::<Vec<_>>()
                    .await
            );
        }
    }

    #[async_std::test]
    pub async fn test_range<D: TestableDataSource>() {
        setup_test();

        let mut network = MockNetwork::<D>::init().await;
        let ds = network.data_source();
        network.start().await;

        // Wait for there to be at least 3 blocks, then lock the state so the block height can't
        // change during the test.
        let (ds, block_height) = loop {
            let ds = ds.read().await;
            let block_height = NodeDataSource::<MockTypes>::block_height(&*ds)
                .await
                .unwrap();
            if block_height >= 3 {
                break (ds, block_height as u64);
            }
        };

        // Query for a variety of ranges testing all cases of included, excluded, and unbounded
        // starting and ending bounds
        do_range_test(&*ds, 1..=2, 1..3).await; // (inclusive, inclusive)
        do_range_test(&*ds, 1..3, 1..3).await; // (inclusive, exclusive)
        do_range_test(&*ds, 1.., 1..block_height).await; // (inclusive, unbounded)
        do_range_test(&*ds, ..=2, 0..3).await; // (unbounded, inclusive)
        do_range_test(&*ds, ..3, 0..3).await; // (unbounded, exclusive)
        do_range_test(&*ds, .., 0..block_height).await; // (unbounded, unbounded)
        do_range_test(&*ds, ExRange(0..=2), 1..3).await; // (exclusive, inclusive)
        do_range_test(&*ds, ExRange(0..3), 1..3).await; // (exclusive, exclusive)
        do_range_test(&*ds, ExRange(0..), 1..block_height).await; // (exclusive, unbounded)
    }

    async fn do_range_test<D, R, I>(ds: &D, range: R, expected_indices: I)
    where
        D: TestableDataSource,
        R: RangeBounds<usize> + Clone + Debug + Send + 'static,
        I: IntoIterator<Item = u64>,
    {
        tracing::info!("testing range {range:?}");

        let mut leaves = ds.get_leaf_range(range.clone()).await;
        let mut blocks = ds.get_block_range(range.clone()).await;

        for i in expected_indices {
            let leaf = leaves.next().await.unwrap().await;
            let block = blocks.next().await.unwrap().await;
            assert_eq!(leaf.height(), i);
            assert_eq!(block.height(), i);
        }

        if range.end_bound() == Bound::Unbounded {
            // If the range is unbounded, the stream should continue, yielding pending futures for
            // the objects which are not currently available.
            let fetch_leaf = leaves.next().await.unwrap();
            let fetch_block = blocks.next().await.unwrap();
            fetch_leaf.try_resolve().unwrap_err();
            fetch_block.try_resolve().unwrap_err();
        } else {
            // If the range is bounded, it should end where expected.
            assert!(leaves.next().await.is_none());
            assert!(blocks.next().await.is_none());
        }
    }

    // A wrapper around a range that turns the lower bound from inclusive to exclusive.
    #[derive(Clone, Copy, Debug)]
    struct ExRange<R>(R);

    impl<R: RangeBounds<usize>> RangeBounds<usize> for ExRange<R> {
        fn start_bound(&self) -> Bound<&usize> {
            match self.0.start_bound() {
                Bound::Included(x) => Bound::Excluded(x),
                Bound::Excluded(x) => Bound::Excluded(x),
                Bound::Unbounded => Bound::Excluded(&0),
            }
        }

        fn end_bound(&self) -> Bound<&usize> {
            self.0.end_bound()
        }
    }
}

/// Generic tests we can instantiate for any data source with reliable, versioned persistent storage.
#[cfg(any(test, feature = "testing"))]
#[espresso_macros::generic_tests]
pub mod persistence_tests {
    use crate::{
        availability::{BlockQueryData, LeafQueryData, UpdateAvailabilityData},
        node::NodeDataSource,
        testing::{
            consensus::TestableDataSource,
            mocks::{MockPayload, MockTypes},
            setup_test,
        },
        Leaf,
    };
    use commit::Committable;
    use hotshot_testing::state_types::TestInstanceState;
    use hotshot_types::simple_certificate::QuorumCertificate;

    #[async_std::test]
    pub async fn test_revert<D: TestableDataSource>() {
        setup_test();

        let storage = D::create(0).await;
        let mut ds = D::connect(&storage).await;

        // Mock up some consensus data.
        let mut qc = QuorumCertificate::<MockTypes>::genesis();
        let mut leaf = Leaf::<MockTypes>::genesis(&TestInstanceState {});
        // Increment the block number, to distinguish this block from the genesis block, which
        // already exists.
        leaf.block_header.block_number += 1;
        qc.data.leaf_commit = leaf.commit();

        let block = BlockQueryData::new(leaf.block_header.clone(), MockPayload::genesis());
        let leaf = LeafQueryData::new(leaf, qc).unwrap();

        // Insert, but do not commit, some data and check that we can read it back.
        UpdateAvailabilityData::<MockTypes>::insert_leaf(&mut ds, leaf.clone())
            .await
            .unwrap();
        ds.insert_block(block.clone()).await.unwrap();

        assert_eq!(
            NodeDataSource::<MockTypes>::block_height(&ds)
                .await
                .unwrap(),
            2
        );
        assert_eq!(leaf, ds.get_leaf(1).await.await);
        assert_eq!(block, ds.get_block(1).await.await);

        // Revert the changes.
        ds.revert().await;
        assert_eq!(
            NodeDataSource::<MockTypes>::block_height(&ds)
                .await
                .unwrap(),
            0
        );
        ds.get_leaf(1).await.try_resolve().unwrap_err();
        ds.get_block(1).await.try_resolve().unwrap_err();
    }

    #[async_std::test]
    pub async fn test_reset<D: TestableDataSource>() {
        setup_test();

        let storage = D::create(0).await;
        let mut ds = D::connect(&storage).await;

        // Mock up some consensus data.
        let mut qc = QuorumCertificate::<MockTypes>::genesis();
        let mut leaf = Leaf::<MockTypes>::genesis(&TestInstanceState {});
        // Increment the block number, to distinguish this block from the genesis block, which
        // already exists.
        leaf.block_header.block_number += 1;
        qc.data.leaf_commit = leaf.commit();

        let block = BlockQueryData::new(leaf.block_header.clone(), MockPayload::genesis());
        let leaf = LeafQueryData::new(leaf, qc).unwrap();

        // Insert some data and check that we can read it back.
        UpdateAvailabilityData::<MockTypes>::insert_leaf(&mut ds, leaf.clone())
            .await
            .unwrap();
        ds.insert_block(block.clone()).await.unwrap();
        ds.commit().await.unwrap();

        assert_eq!(
            NodeDataSource::<MockTypes>::block_height(&ds)
                .await
                .unwrap(),
            2
        );
        assert_eq!(leaf, ds.get_leaf(1).await.await);
        assert_eq!(block, ds.get_block(1).await.await);

        drop(ds);

        // Reset and check that the changes are gone.
        let ds = D::reset(&storage).await;
        assert_eq!(
            NodeDataSource::<MockTypes>::block_height(&ds)
                .await
                .unwrap(),
            0
        );
        ds.get_leaf(1).await.try_resolve().unwrap_err();
        ds.get_block(1).await.try_resolve().unwrap_err();
    }
}

/// Generic tests we can instantiate for all the node data sources.
#[cfg(any(test, feature = "testing"))]
#[espresso_macros::generic_tests]
pub mod node_tests {
    use super::test_helpers::*;
    use crate::testing::{
        consensus::{MockNetwork, TestableDataSource},
        setup_test,
    };
    use futures::stream::StreamExt;
    use std::collections::HashSet;

    async fn validate(ds: &impl TestableDataSource) {
        let mut leaves = leaf_range(ds, ..).await;

        // Check the consistency of our indexes by proposer for every leaf.
        while let Some(leaf) = leaves.next().await {
            assert!(ds
                .get_proposals(&leaf.proposer(), None)
                .await
                .unwrap()
                .contains(&leaf));
        }

        // Validate the list of proposals for every distinct proposer ID in the chain.
        for proposer in leaf_range(ds, ..)
            .await
            .map(|leaf| leaf.proposer())
            .collect::<HashSet<_>>()
            .await
        {
            let proposals = ds.get_proposals(&proposer, None).await.unwrap();
            // We found `proposer` by getting the proposer ID of a leaf, so there must be at least
            // one proposal from this proposer.
            assert!(!proposals.is_empty());
            // If we select with a limit, we should get the most recent `limit` proposals in
            // chronological order.
            let suffix = ds
                .get_proposals(&proposer, Some(proposals.len() / 2))
                .await
                .unwrap();
            assert_eq!(suffix.len(), proposals.len() / 2);
            assert!(proposals.ends_with(&suffix));

            // Check that the proposer ID of every leaf indexed by `proposer` is `proposer`, and
            // that the list of proposals is in chronological order.
            let mut prev_height = None;
            for leaf in proposals {
                assert_eq!(proposer, leaf.proposer());
                if let Some(prev_height) = prev_height {
                    assert!(prev_height < leaf.height());
                }
                prev_height = Some(leaf.height());
            }
        }
    }

    #[async_std::test]
    pub async fn test_proposer_queries<D: TestableDataSource>() {
        setup_test();

        let mut network = MockNetwork::<D>::init().await;
        let ds = network.data_source();

        network.start().await;

        // Wait for a few blocks to be produced, then validate the chain. We will wait for more
        // blocks than there are nodes in the network, so we have some blocks with the same proposer.
        let mut leaves = {
            ds.read()
                .await
                .subscribe_leaves(0)
                .await
                .take(2 * network.num_nodes())
        };
        while let Some(leaf) = leaves.next().await {
            tracing::info!("got leaf {}", leaf.height());
        }
        {
            validate(&*ds.read().await).await;
        }

        // Check that all the updates have been committed to storage, not simply held in memory: a
        // new data source created from the same underlying storage should have valid proposer
        // indexes.
        tracing::info!("checking persisted storage");

        // Lock the original data source to prevent concurrent updates.
        let _ = ds.read().await;
        let storage = D::connect(network.storage()).await;
        validate(&storage).await;
    }
}

/// Generic tests we can instantiate for all the status data sources.
#[cfg(any(test, feature = "testing"))]
#[espresso_macros::generic_tests]
pub mod status_tests {
    use crate::{
        status::{MempoolQueryData, StatusDataSource},
        testing::{
            consensus::{DataSourceLifeCycle, MockNetwork},
            mocks::mock_transaction,
            setup_test, sleep,
        },
    };
    use bincode::Options;
    use hotshot_utils::bincode::bincode_opts;
    use std::time::Duration;

    #[async_std::test]
    pub async fn test_metrics<D: DataSourceLifeCycle + StatusDataSource>() {
        setup_test();

        let mut network = MockNetwork::<D>::init().await;
        let ds = network.data_source();

        {
            let ds = ds.read().await;
            // Check that block height is initially zero.
            assert_eq!(ds.block_height().await.unwrap(), 0);
            // With consensus paused, check that the success rate returns NAN (since the block
            // height, the numerator, is 0, and the view number, the denominator, is 0).
            assert!(ds.success_rate().await.unwrap().is_nan());
        }

        // Submit a transaction, and check that it is reflected in the mempool.
        let txn = mock_transaction(vec![1, 2, 3]);
        network.submit_transaction(txn.clone()).await;
        loop {
            let mempool = { ds.read().await.mempool_info().await.unwrap() };
            let expected = MempoolQueryData {
                transaction_count: 1,
                memory_footprint: bincode_opts().serialized_size(&txn).unwrap(),
            };
            if mempool == expected {
                break;
            }
            tracing::info!(
                "waiting for mempool to reflect transaction (currently {:?})",
                mempool
            );
            sleep(Duration::from_secs(1)).await;
        }
        {
            assert_eq!(
                ds.read().await.mempool_info().await.unwrap(),
                MempoolQueryData {
                    transaction_count: 1,
                    memory_footprint: bincode_opts().serialized_size(&txn).unwrap(),
                }
            );
        }

        // Submitting the same transaction should not affect the mempool.
        network.submit_transaction(txn.clone()).await;
        sleep(Duration::from_secs(3)).await;
        {
            assert_eq!(
                ds.read().await.mempool_info().await.unwrap(),
                MempoolQueryData {
                    transaction_count: 1,
                    memory_footprint: bincode_opts().serialized_size(&txn).unwrap(),
                }
            );
        }

        // Start consensus and wait for the transaction to be finalized.
        network.start().await;
        // First wait for the transaction to be taken out of the mempool.
        let block_height = loop {
            let ds = ds.read().await;
            if ds.mempool_info().await.unwrap().transaction_count == 0 {
                break ds.block_height().await.unwrap();
            }
            sleep(Duration::from_secs(1)).await;
        };
        // Now wait for at least one block to be finalized.
        loop {
            if ds.read().await.block_height().await.unwrap() > block_height {
                break;
            }
            sleep(Duration::from_secs(1)).await;
        }

        {
            // Check that the success rate has been updated. Note that we can only check if success
            // rate is positive. We don't know exactly what it is because we can't know how many
            // views have elapsed without race conditions.
            let success_rate = ds.read().await.success_rate().await.unwrap();
            assert!(success_rate.is_finite(), "{success_rate}");
            assert!(success_rate > 0.0, "{success_rate}");
        }

        {
            // Check that the transaction is no longer reflected in the mempool.
            assert_eq!(
                ds.read().await.mempool_info().await.unwrap(),
                MempoolQueryData {
                    transaction_count: 0,
                    memory_footprint: 0,
                }
            );
        }
    }
}

#[macro_export]
macro_rules! instantiate_data_source_tests {
    ($t:ty) => {
        use $crate::data_source::{
            availability_tests, node_tests, persistence_tests, status_tests,
        };

        instantiate_availability_tests!($t);
        instantiate_persistence_tests!($t);
        instantiate_node_tests!($t);
        instantiate_status_tests!($t);
    };
}
