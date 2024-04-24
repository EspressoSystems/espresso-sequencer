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
        types::HeightIndexed,
    };
    use async_std::sync::RwLock;
    use committable::Committable;
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
            tracing::info!("looking up payload {i} various ways");
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

            // Look up the common VID data.
            tracing::info!("looking up VID common {i} various ways");
            let common = ds.get_vid_common(block.height() as usize).await.await;
            assert_eq!(common, ds.get_vid_common(block.hash()).await.await);
            // Similar to the above, we can't guarantee which index we will get when passively
            // fetching this data, so only check the index if the data is available locally.
            if let Ok(res) = ds
                .get_vid_common(BlockId::PayloadHash(block.payload_hash()))
                .await
                .try_resolve()
            {
                if *ix == i as u64 {
                    assert_eq!(res, common);
                }
            } else {
                tracing::warn!(
                    "skipping VID common index check for missing data {:?}",
                    block.header()
                );
                // At least check that _some_ data can be fetched.
                let res = ds
                    .get_vid_common(BlockId::PayloadHash(block.payload_hash()))
                    .await
                    .await;
                assert_eq!(res.payload_hash(), common.payload_hash());
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
        let mut vid_common = ds.get_vid_common_range(range.clone()).await;

        for i in expected_indices {
            tracing::info!(i, "check entries");
            let leaf = leaves.next().await.unwrap().await;
            let block = blocks.next().await.unwrap().await;
            let common = vid_common.next().await.unwrap().await;
            assert_eq!(leaf.height(), i);
            assert_eq!(block.height(), i);
            assert_eq!(common, ds.get_vid_common(i as usize).await.await);
        }

        if range.end_bound() == Bound::Unbounded {
            // If the range is unbounded, the stream should continue, yielding pending futures for
            // the objects which are not currently available.
            let fetch_leaf = leaves.next().await.unwrap();
            let fetch_block = blocks.next().await.unwrap();
            let fetch_common = vid_common.next().await.unwrap();
            fetch_leaf.try_resolve().unwrap_err();
            fetch_block.try_resolve().unwrap_err();
            fetch_common.try_resolve().unwrap_err();
        } else {
            // If the range is bounded, it should end where expected.
            assert!(leaves.next().await.is_none());
            assert!(blocks.next().await.is_none());
            assert!(vid_common.next().await.is_none());
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
        availability::{BlockQueryData, LeafQueryData},
        node::NodeDataSource,
        testing::{
            consensus::TestableDataSource,
            mocks::{MockPayload, MockTypes},
            setup_test,
        },
        Leaf,
    };
    use committable::Committable;
    use hotshot_example_types::state_types::TestInstanceState;
    use hotshot_types::simple_certificate::QuorumCertificate;

    #[async_std::test]
    pub async fn test_revert<D: TestableDataSource>() {
        setup_test();

        let storage = D::create(0).await;
        let mut ds = D::connect(&storage).await;

        // Mock up some consensus data.
        let mut qc = QuorumCertificate::<MockTypes>::genesis(&TestInstanceState {});
        let mut leaf = Leaf::<MockTypes>::genesis(&TestInstanceState {});
        // Increment the block number, to distinguish this block from the genesis block, which
        // already exists.
        leaf.get_block_header_mut().block_number += 1;
        qc.data.leaf_commit = leaf.commit();

        let block = BlockQueryData::new(leaf.get_block_header().clone(), MockPayload::genesis());
        let leaf = LeafQueryData::new(leaf, qc).unwrap();

        // Insert, but do not commit, some data and check that we can read it back.
        ds.insert_leaf(leaf.clone()).await.unwrap();
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
        let mut qc = QuorumCertificate::<MockTypes>::genesis(&TestInstanceState {});
        let mut leaf = Leaf::<MockTypes>::genesis(&TestInstanceState {});
        // Increment the block number, to distinguish this block from the genesis block, which
        // already exists.
        leaf.get_block_header_mut().block_number += 1;
        qc.data.leaf_commit = leaf.commit();

        let block = BlockQueryData::new(leaf.get_block_header().clone(), MockPayload::genesis());
        let leaf = LeafQueryData::new(leaf, qc).unwrap();

        // Insert some data and check that we can read it back.
        ds.insert_leaf(leaf.clone()).await.unwrap();
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
    use crate::{
        availability::{BlockQueryData, LeafQueryData, QueryableHeader, VidCommonQueryData},
        node::{BlockId, SyncStatus, TimeWindowQueryData, WindowStart},
        testing::{
            consensus::{MockNetwork, TestableDataSource},
            mocks::{mock_transaction, MockPayload, MockTypes},
            setup_test,
        },
        types::HeightIndexed,
        Header, VidShare,
    };
    use committable::Committable;
    use futures::{future::join_all, stream::StreamExt};
    use hotshot_example_types::{
        block_types::{TestBlockHeader, TestBlockPayload, TestMetadata},
        state_types::TestInstanceState,
    };
    use hotshot_types::{
        traits::block_contents::{vid_commitment, BlockPayload},
        vid::{vid_scheme, VidSchemeType},
    };
    use jf_primitives::vid::VidScheme;

    #[async_std::test]
    pub async fn test_sync_status<D: TestableDataSource>() {
        setup_test();

        let storage = D::create(0).await;
        let mut ds = D::connect(&storage).await;

        // Set up a mock VID scheme to use for generating test data.
        let mut vid = vid_scheme(2);

        // Generate some mock leaves and blocks to insert.
        let mut leaves = vec![LeafQueryData::<MockTypes>::genesis(&TestInstanceState {})];
        let mut blocks = vec![BlockQueryData::<MockTypes>::genesis(&TestInstanceState {})];
        for i in 0..2 {
            let mut leaf = leaves[i].clone();
            leaf.leaf.get_block_header_mut().block_number += 1;
            leaves.push(leaf);

            let mut block = blocks[i].clone();
            block.header.block_number += 1;
            blocks.push(block);
        }
        // Generate mock VID data. We reuse the same (empty) payload for each block, but with
        // different metadata.
        let disperse = vid.disperse([]).unwrap();
        let vid = leaves
            .iter()
            .map(|leaf| {
                (
                    VidCommonQueryData::new(leaf.header().clone(), disperse.common.clone()),
                    disperse.shares[0].clone(),
                )
            })
            .collect::<Vec<_>>();

        // At first, the node is fully synced.
        assert!(ds.sync_status().await.unwrap().is_fully_synced());

        // Insert a leaf without the corresponding block or VID info, make sure we detect that the
        // block and VID info are missing.
        ds.insert_leaf(leaves[0].clone()).await.unwrap();
        ds.commit().await.unwrap();
        assert_eq!(
            ds.sync_status().await.unwrap(),
            SyncStatus {
                missing_blocks: 1,
                missing_vid_common: 1,
                missing_vid_shares: 1,
                missing_leaves: 0,
                pruned_height: None,
            }
        );

        // Insert a leaf whose height is not the successor of the previous leaf. We should now
        // detect that the leaf in between is missing (along with all _three_ corresponding blocks).
        ds.insert_leaf(leaves[2].clone()).await.unwrap();
        ds.commit().await.unwrap();
        assert_eq!(
            ds.sync_status().await.unwrap(),
            SyncStatus {
                missing_blocks: 3,
                missing_vid_common: 3,
                missing_vid_shares: 3,
                missing_leaves: 1,
                pruned_height: None,
            }
        );

        // Insert VID common without a corresponding share.
        ds.insert_vid(vid[0].0.clone(), None).await.unwrap();
        ds.commit().await.unwrap();
        assert_eq!(
            ds.sync_status().await.unwrap(),
            SyncStatus {
                missing_blocks: 3,
                missing_vid_common: 2,
                missing_vid_shares: 3,
                missing_leaves: 1,
                pruned_height: None,
            }
        );

        // Rectify the missing data.
        ds.insert_block(blocks[0].clone()).await.unwrap();
        ds.insert_vid(vid[0].0.clone(), Some(vid[0].1.clone()))
            .await
            .unwrap();
        ds.insert_leaf(leaves[1].clone()).await.unwrap();
        ds.insert_block(blocks[1].clone()).await.unwrap();
        ds.insert_vid(vid[1].0.clone(), Some(vid[1].1.clone()))
            .await
            .unwrap();
        ds.insert_block(blocks[2].clone()).await.unwrap();
        ds.insert_vid(vid[2].0.clone(), Some(vid[2].1.clone()))
            .await
            .unwrap();
        ds.commit().await.unwrap();

        // Some data sources (e.g. file system) don't support out-of-order insertion of missing
        // data. These would have just ignored the insertion of `vid[0]` (the share) and
        // `leaves[1]`. Detect if this is the case; then we allow 1 missing leaf and 1 missing VID
        // share.
        let expected_missing = if ds.get_leaf(1).await.try_resolve().is_err() {
            tracing::warn!(
                "data source does not support out-of-order filling, allowing one missing leaf and VID share"
            );
            1
        } else {
            0
        };
        let expected_sync_status = SyncStatus {
            missing_blocks: 0,
            missing_leaves: expected_missing,
            missing_vid_common: 0,
            missing_vid_shares: expected_missing,
            pruned_height: None,
        };
        assert_eq!(ds.sync_status().await.unwrap(), expected_sync_status);

        // If we re-insert one of the VID entries without a share, it should not overwrite the share
        // that we already have; that is, `insert_vid` should be monotonic.
        ds.insert_vid(vid[0].0.clone(), None).await.unwrap();
        assert_eq!(ds.sync_status().await.unwrap(), expected_sync_status);
    }

    #[async_std::test]
    pub async fn test_counters<D: TestableDataSource>() {
        setup_test();

        let storage = D::create(0).await;
        let mut ds = D::connect(&storage).await;

        assert_eq!(ds.count_transactions().await.unwrap(), 0);
        assert_eq!(ds.payload_size().await.unwrap(), 0);

        // Insert some transactions. We insert the blocks out of order to check that the counters
        // account for missing blocks fetched later.
        let mut total_transactions = 0;
        let mut total_size = 0;
        for i in [0, 2, 1] {
            // Using `i % 2` as the transaction data ensures we insert a duplicate transaction
            // (since we insert more than 2 transactions total). The query service should still
            // count these as separate transactions and should include both duplicates when
            // computing the total size.
            let (payload, metadata) =
                TestBlockPayload::from_transactions([mock_transaction(vec![i as u8 % 2])]).unwrap();
            let encoded = payload.encode().unwrap();
            let payload_commitment = vid_commitment(&encoded, 1);
            let header = TestBlockHeader {
                block_number: i,
                payload_commitment,
                timestamp: i,
                builder_commitment: payload.builder_commitment(&metadata),
            };

            let mut leaf = LeafQueryData::<MockTypes>::genesis(&TestInstanceState {});
            *leaf.leaf.get_block_header_mut() = header.clone();
            let block = BlockQueryData::new(header, payload);
            ds.insert_leaf(leaf).await.unwrap();
            ds.insert_block(block).await.unwrap();
            ds.commit().await.unwrap();

            total_transactions += 1;
            total_size += encoded.len();

            assert_eq!(ds.count_transactions().await.unwrap(), total_transactions);
            assert_eq!(ds.payload_size().await.unwrap(), total_size);
        }
    }

    #[async_std::test]
    pub async fn test_vid_shares<D: TestableDataSource>() {
        setup_test();

        let mut network = MockNetwork::<D>::init().await;
        let ds = network.data_source();

        network.start().await;

        // Check VID shares for a few blocks.
        let mut leaves = { ds.read().await.subscribe_leaves(0).await.take(3) };
        while let Some(leaf) = leaves.next().await {
            tracing::info!("got leaf {}", leaf.height());
            let ds = ds.read().await;
            let share = ds.vid_share(leaf.height() as usize).await.unwrap();
            assert_eq!(share, ds.vid_share(leaf.block_hash()).await.unwrap());
            assert_eq!(
                share,
                ds.vid_share(BlockId::PayloadHash(leaf.payload_hash()))
                    .await
                    .unwrap()
            );
        }
    }

    #[async_std::test]
    pub async fn test_vid_monotonicity<D: TestableDataSource>() {
        setup_test();

        let storage = D::create(0).await;
        let mut ds = D::connect(&storage).await;

        // Generate some test VID data.
        let mut vid = vid_scheme(2);
        let disperse = vid.disperse([]).unwrap();

        // Insert test data with VID common and a share.
        let leaf = LeafQueryData::<MockTypes>::genesis(&TestInstanceState {});
        let common = VidCommonQueryData::new(leaf.header().clone(), disperse.common);
        ds.insert_leaf(leaf).await.unwrap();
        ds.insert_vid(common.clone(), Some(disperse.shares[0].clone()))
            .await
            .unwrap();
        ds.commit().await.unwrap();

        assert_eq!(ds.get_vid_common(0).await.await, common);
        assert_eq!(ds.vid_share(0).await.unwrap(), disperse.shares[0]);

        // Re-insert the common data, without a share. This should not overwrite the share we
        // already have.
        ds.insert_vid(common.clone(), None).await.unwrap();
        ds.commit().await.unwrap();
        assert_eq!(ds.get_vid_common(0).await.await, common);
        assert_eq!(ds.vid_share(0).await.unwrap(), disperse.shares[0]);
    }

    #[async_std::test]
    pub async fn test_vid_recovery<D: TestableDataSource>() {
        setup_test();

        let mut network = MockNetwork::<D>::init().await;
        let ds = network.data_source();

        network.start().await;

        // Submit a transaction so we can try to recover a non-empty block.
        let mut blocks = { ds.read().await.subscribe_blocks(0).await };
        let txn = mock_transaction(vec![1, 2, 3]);
        network.submit_transaction(txn.clone()).await;

        // Wait for the transaction to be finalized.
        let block = loop {
            tracing::info!("waiting for transaction");
            let block = blocks.next().await.unwrap();
            if !block.is_empty() {
                tracing::info!(height = block.height(), "transaction sequenced");
                break block;
            }
            tracing::info!(height = block.height(), "empty block");
        };
        let height = block.height() as usize;
        let commit = block.payload_hash();

        // Set up a test VID scheme.
        let vid = vid_scheme(network.num_nodes());

        // Get VID common data and verify it.
        tracing::info!("fetching common data");
        let common = { ds.read().await.get_vid_common(height).await.await };
        let common = common.common();
        VidSchemeType::is_consistent(&commit, common).unwrap();

        // Collect shares from each node.
        tracing::info!("fetching shares");
        let network = &network;
        let vid = &vid;
        let shares: Vec<VidShare> = join_all((0..network.num_nodes()).map(|i| async move {
            let ds = network.data_source_index(i);

            // Wait until the node has processed up to the desired block; since we have thus far
            // only interacted with node 0, it is possible other nodes are slightly behind.
            let mut leaves = { ds.read().await.subscribe_leaves(height).await };
            let leaf = leaves.next().await.unwrap();
            assert_eq!(leaf.height(), height as u64);
            assert_eq!(leaf.payload_hash(), commit);

            let share = { ds.read().await.vid_share(height).await.unwrap() };
            vid.verify_share(&share, common, &commit).unwrap().unwrap();
            share
        }))
        .await;

        // Recover payload.
        tracing::info!("recovering payload");
        let bytes = vid.recover_payload(&shares, common).unwrap();
        let recovered = MockPayload::from_bytes(bytes.as_slice(), &TestMetadata);
        assert_eq!(recovered, *block.payload());
        assert_eq!(recovered.transactions, vec![txn]);
    }

    #[async_std::test]
    pub async fn test_timestamp_window<D: TestableDataSource>() {
        setup_test();

        let mut network = MockNetwork::<D>::init().await;
        let ds = network.data_source();

        network.start().await;

        // Wait for blocks with at least three different timestamps to be sequenced. This lets us
        // test all the edge cases.
        let mut leaves = { ds.read().await.subscribe_leaves(0).await };
        // `test_blocks` is a list of lists of headers with the same timestamp. The flattened list
        // of headers is contiguous.
        let mut test_blocks: Vec<Vec<Header<MockTypes>>> = vec![];
        while test_blocks.len() < 3 {
            // Wait for the next block to be sequenced.
            let leaf = leaves.next().await.unwrap();
            let header = leaf.header().clone();
            if let Some(last_timestamp) = test_blocks.last_mut() {
                if last_timestamp[0].timestamp() == header.timestamp() {
                    last_timestamp.push(header);
                } else {
                    test_blocks.push(vec![header]);
                }
            } else {
                test_blocks.push(vec![header]);
            }
        }
        tracing::info!("blocks for testing: {test_blocks:#?}");

        // Define invariants that every response should satisfy.
        let check_invariants =
            |res: &TimeWindowQueryData<Header<MockTypes>>, start, end, check_prev| {
                let mut prev = res.prev.as_ref();
                if let Some(prev) = prev {
                    if check_prev {
                        assert!(prev.timestamp() < start);
                    }
                } else {
                    // `prev` can only be `None` if the first block in the window is the genesis
                    // block.
                    assert_eq!(res.from().unwrap(), 0);
                };
                for header in &res.window {
                    assert!(start <= header.timestamp());
                    assert!(header.timestamp() < end);
                    if let Some(prev) = prev {
                        assert!(prev.timestamp() <= header.timestamp());
                    }
                    prev = Some(header);
                }
                if let Some(next) = &res.next {
                    assert!(next.timestamp() >= end);
                    // If there is a `next`, there must be at least one previous block (either `prev`
                    // itself or the last block if the window is nonempty), so we can `unwrap` here.
                    assert!(next.timestamp() >= prev.unwrap().timestamp());
                }
            };

        let get_window = |start, end| {
            let ds = ds.clone();
            async move {
                let window = ds
                    .read()
                    .await
                    .get_header_window(WindowStart::Time(start), end)
                    .await
                    .unwrap();
                tracing::info!("window for timestamp range {start}-{end}: {window:#?}");
                check_invariants(&window, start, end, true);
                window
            }
        };

        // Case 0: happy path. All blocks are available, including prev and next.
        let start = test_blocks[1][0].timestamp();
        let end = start + 1;
        let res = get_window(start, end).await;
        assert_eq!(res.prev.unwrap(), *test_blocks[0].last().unwrap());
        assert_eq!(res.window, test_blocks[1]);
        assert_eq!(res.next.unwrap(), test_blocks[2][0]);

        // Case 1: no `prev`, start of window is before genesis.
        let start = 0;
        let end = test_blocks[0][0].timestamp() + 1;
        let res = get_window(start, end).await;
        assert_eq!(res.prev, None);
        assert_eq!(res.window, test_blocks[0]);
        assert_eq!(res.next.unwrap(), test_blocks[1][0]);

        // Case 2: no `next`, end of window is after the most recently sequenced block.
        let start = test_blocks[2][0].timestamp();
        let end = i64::MAX as u64;
        let res = get_window(start, end).await;
        assert_eq!(res.prev.unwrap(), *test_blocks[1].last().unwrap());
        // There may have been more blocks sequenced since we grabbed `test_blocks`, so just check
        // that the prefix of the window is correct.
        assert_eq!(res.window[..test_blocks[2].len()], test_blocks[2]);
        assert_eq!(res.next, None);
        // Fetch more blocks using the `from` form of the endpoint. Start from the last block we had
        // previously (ie fetch a slightly overlapping window) to ensure there is at least one block
        // in the new window.
        let from = test_blocks.iter().flatten().count() - 1;
        let more = {
            ds.read()
                .await
                .get_header_window(WindowStart::Height(from as u64), end)
                .await
                .unwrap()
        };
        check_invariants(&more, start, end, false);
        assert_eq!(
            more.prev.as_ref().unwrap(),
            test_blocks.iter().flatten().nth(from - 1).unwrap()
        );
        assert_eq!(
            more.window[..res.window.len() - test_blocks[2].len() + 1],
            res.window[test_blocks[2].len() - 1..]
        );
        assert_eq!(res.next, None);
        // We should get the same result whether we query by block height or hash.
        let more2 = {
            ds.read()
                .await
                .get_header_window(test_blocks[2].last().unwrap().commit(), end)
                .await
                .unwrap()
        };
        check_invariants(&more2, start, end, false);
        assert_eq!(more2.from().unwrap(), more.from().unwrap());
        assert_eq!(more2.prev, more.prev);
        assert_eq!(more2.next, more.next);
        assert_eq!(more2.window[..more.window.len()], more.window);

        // Case 3: the window is empty.
        let start = test_blocks[1][0].timestamp();
        let end = start;
        let res = get_window(start, end).await;
        assert_eq!(res.prev.unwrap(), *test_blocks[0].last().unwrap());
        assert_eq!(res.next.unwrap(), test_blocks[1][0]);
        assert_eq!(res.window, vec![]);

        // Case 5: no relevant blocks are available yet.
        {
            ds.read()
                .await
                .get_header_window(WindowStart::Time((i64::MAX - 1) as u64), i64::MAX as u64)
                .await
                .unwrap_err();
        }
    }
}

/// Generic tests we can instantiate for all the status data sources.
#[cfg(any(test, feature = "testing"))]
#[espresso_macros::generic_tests]
pub mod status_tests {
    use crate::{
        status::StatusDataSource,
        testing::{
            consensus::{DataSourceLifeCycle, MockNetwork},
            mocks::mock_transaction,
            setup_test, sleep,
        },
    };
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
            // Since there is no block produced, "last_decided_time" metric is 0.
            // Therefore, the elapsed time since the last block should be close to the time elapsed since the Unix epoch.
            assert!(
                (ds.elapsed_time_since_last_decide().await.unwrap() as i64
                    - chrono::Utc::now().timestamp())
                .abs()
                    <= 1,
                "time elapsed since last_decided_time is not within 1s"
            );
        }

        // Submit a transaction
        let txn = mock_transaction(vec![1, 2, 3]);
        network.submit_transaction(txn.clone()).await;

        // Start consensus and wait for the transaction to be finalized.
        network.start().await;

        // Now wait for at least one block to be finalized.
        let block_height = ds.read().await.block_height().await.unwrap();
        loop {
            let current_height = ds.read().await.block_height().await.unwrap();
            if current_height > block_height {
                break;
            }
            tracing::info!(
                current_height,
                block_height,
                "waiting for a block to be finalized"
            );
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
            // Shutting down the consensus to halt block production
            // Introducing a delay of 3 seconds to ensure that elapsed time since last block is atleast 3seconds
            network.shut_down().await;
            sleep(Duration::from_secs(3)).await;
            // Asserting that the elapsed time since the last block is at least 3 seconds
            assert!(
                ds.read()
                    .await
                    .elapsed_time_since_last_decide()
                    .await
                    .unwrap()
                    >= 3
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
