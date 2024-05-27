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

#![cfg(feature = "no-storage")]

use super::AvailabilityStorage;
use crate::{
    availability::{
        BlockId, BlockQueryData, LeafId, LeafQueryData, PayloadQueryData, QueryablePayload,
        TransactionHash, TransactionQueryData, UpdateAvailabilityData, VidCommonQueryData,
    },
    data_source::{
        storage::pruning::{PruneStorage, PrunedHeightStorage, PrunerConfig},
        VersionedDataSource,
    },
    node::{NodeDataSource, SyncStatus, TimeWindowQueryData, WindowStart},
    Header, Payload, QueryError, QueryResult, VidShare,
};
use async_trait::async_trait;
use hotshot_types::traits::node_implementation::NodeType;
use std::{convert::Infallible, ops::RangeBounds};

/// Mock storage implementation which doesn't actually store anything.
///
/// This is useful for adversarial testing, as it can be used to test the behavior of the query
/// service where data is never available locally and must always be fetched on demand from a peer
/// query service.
#[derive(Clone, Debug, Default, Copy)]
pub struct NoStorage;

#[async_trait]
impl VersionedDataSource for NoStorage {
    type Error = Infallible;

    async fn commit(&mut self) -> Result<(), Infallible> {
        Ok(())
    }

    async fn revert(&mut self) {}
}
impl PrunerConfig for NoStorage {}
impl PruneStorage for NoStorage {}
impl PrunedHeightStorage for NoStorage {
    type Error = Infallible;
}

#[async_trait]
impl<Types: NodeType> AvailabilityStorage<Types> for NoStorage
where
    Payload<Types>: QueryablePayload<Types>,
{
    async fn get_leaf(&self, _id: LeafId<Types>) -> QueryResult<LeafQueryData<Types>> {
        Err(QueryError::Missing)
    }

    async fn get_block(&self, _id: BlockId<Types>) -> QueryResult<BlockQueryData<Types>> {
        Err(QueryError::Missing)
    }

    async fn get_header(&self, _id: BlockId<Types>) -> QueryResult<Header<Types>> {
        Err(QueryError::Missing)
    }

    async fn get_payload(&self, _id: BlockId<Types>) -> QueryResult<PayloadQueryData<Types>> {
        Err(QueryError::Missing)
    }

    async fn get_vid_common(&self, _id: BlockId<Types>) -> QueryResult<VidCommonQueryData<Types>> {
        Err(QueryError::Missing)
    }

    async fn get_leaf_range<R>(
        &self,
        _range: R,
    ) -> QueryResult<Vec<QueryResult<LeafQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        Err(QueryError::Missing)
    }

    async fn get_block_range<R>(
        &self,
        _range: R,
    ) -> QueryResult<Vec<QueryResult<BlockQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        Err(QueryError::Missing)
    }

    async fn get_payload_range<R>(
        &self,
        _range: R,
    ) -> QueryResult<Vec<QueryResult<PayloadQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        Err(QueryError::Missing)
    }

    async fn get_vid_common_range<R>(
        &self,
        _range: R,
    ) -> QueryResult<Vec<QueryResult<VidCommonQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        Err(QueryError::Missing)
    }

    async fn get_transaction(
        &self,
        _hash: TransactionHash<Types>,
    ) -> QueryResult<TransactionQueryData<Types>> {
        Err(QueryError::Missing)
    }
}

#[async_trait]
impl<Types: NodeType> UpdateAvailabilityData<Types> for NoStorage
where
    Payload<Types>: QueryablePayload<Types>,
{
    type Error = Infallible;

    async fn insert_leaf(&mut self, _leaf: LeafQueryData<Types>) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn insert_block(&mut self, _block: BlockQueryData<Types>) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn insert_vid(
        &mut self,
        _common: VidCommonQueryData<Types>,
        _share: Option<VidShare>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[async_trait]
impl<Types: NodeType> NodeDataSource<Types> for NoStorage
where
    Payload<Types>: QueryablePayload<Types>,
{
    async fn block_height(&self) -> QueryResult<usize> {
        Ok(0)
    }

    async fn count_transactions(&self) -> QueryResult<usize> {
        Err(QueryError::Missing)
    }

    async fn payload_size(&self) -> QueryResult<usize> {
        Err(QueryError::Missing)
    }

    async fn vid_share<ID>(&self, _id: ID) -> QueryResult<VidShare>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        Err(QueryError::Missing)
    }

    async fn sync_status(&self) -> QueryResult<SyncStatus> {
        Err(QueryError::Missing)
    }

    async fn get_header_window(
        &self,
        _start: impl Into<WindowStart<Types>> + Send + Sync,
        _end: u64,
    ) -> QueryResult<TimeWindowQueryData<Header<Types>>> {
        Err(QueryError::Missing)
    }
}

// These tests run the `postgres` Docker image, which doesn't work on Windows.
#[cfg(all(any(test, feature = "testing"), not(target_os = "windows")))]
pub mod testing {
    use super::*;
    use crate::QueryError;
    use crate::{
        availability::{define_api, AvailabilityDataSource, Fetch},
        data_source::{
            storage::sql::testing::TmpDb, FetchingDataSource, SqlDataSource, UpdateDataSource,
        },
        fetching::provider::{NoFetching, QueryServiceProvider},
        metrics::PrometheusMetrics,
        node::NodeDataSource,
        status::StatusDataSource,
        testing::{
            consensus::{DataSourceLifeCycle, MockNetwork},
            mocks::MockTypes,
        },
        Error,
    };
    use futures::stream::{BoxStream, StreamExt};
    use hotshot::types::Event;
    use hotshot_types::constants::{Version01, STATIC_VER_0_1};
    use portpicker::pick_unused_port;
    use std::{fmt::Display, time::Duration};
    use tide_disco::App;

    /// Either Postgres or no storage.
    ///
    /// In order to instantiate [`NoStorage`] for the generic tests, we need some node running a
    /// real database that our [`NoStorage`] node can fetch from. We use this [`Storage`] enum to
    /// represent a network where the nodes are either using [`NoStorage`] or SQL storage. We will
    /// set node 0, the node under test, to always use [`NoStorage`].
    ///
    /// This gives us a strongly adversarial test of fetching, where the node under test never gets
    /// anything from local storage, but the tests still pass.
    pub enum Storage {
        Sql(TmpDb),
        NoStorage { fetch_from_port: u16 },
    }

    pub enum DataSource {
        Sql(SqlDataSource<MockTypes, NoFetching>),
        NoStorage(FetchingDataSource<MockTypes, NoStorage, QueryServiceProvider<Version01>>),
    }

    #[async_trait]
    impl DataSourceLifeCycle for DataSource {
        type Storage = Storage;

        async fn create(node_id: usize) -> Self::Storage {
            if node_id == 0 {
                Storage::NoStorage {
                    fetch_from_port: pick_unused_port().unwrap(),
                }
            } else {
                Storage::Sql(TmpDb::init().await)
            }
        }

        async fn connect(db: &Self::Storage) -> Self {
            match db {
                Storage::Sql(db) => {
                    Self::Sql(db.config().connect(Default::default()).await.unwrap())
                }
                Storage::NoStorage { fetch_from_port } => {
                    tracing::info!("creating NoStorage node, fetching missing data from port {fetch_from_port}");
                    let provider = QueryServiceProvider::new(
                        format!("http://localhost:{fetch_from_port}")
                            .parse()
                            .unwrap(),
                        STATIC_VER_0_1,
                    );
                    Self::NoStorage(
                        FetchingDataSource::builder(NoStorage, provider)
                            // The default minor scan interval is suitable for real scenarios, where
                            // missing blocks are quite rare. But in these tests, we rely entirely
                            // on proactive scanning to recover some objects, so we want to do this
                            // quite frequently.
                            .with_minor_scan_interval(Duration::from_millis(100))
                            // Similarly, we even need to do major scans frequently (every 2 minor
                            // scans, or 0.2s), since we are constantly losing old objects (since we
                            // don't have storage) and the test frequently goes back and looks up
                            // old objects.
                            .with_major_scan_interval(2)
                            .build()
                            .await
                            .unwrap(),
                    )
                }
            }
        }

        async fn reset(db: &Self::Storage) -> Self {
            match db {
                Storage::Sql(db) => Self::Sql(
                    db.config()
                        .reset_schema()
                        .connect(Default::default())
                        .await
                        .unwrap(),
                ),
                db => Self::connect(db).await,
            }
        }

        async fn setup(network: &mut MockNetwork<Self>) {
            // Spawn the web server on node 1 that node 0 will use to fetch missing data.
            let Storage::NoStorage { fetch_from_port } = network.storage() else {
                panic!("node 0 should always be NoStorage node");
            };
            tracing::info!("spawning server for missing data on port {fetch_from_port}");
            let api_data_source = network.data_source_index(1);
            let mut app = App::<_, Error>::with_state(api_data_source);
            app.register_module(
                "availability",
                define_api(&Default::default(), STATIC_VER_0_1).unwrap(),
            )
            .unwrap();
            network.spawn(
                "server",
                app.serve(format!("0.0.0.0:{fetch_from_port}"), STATIC_VER_0_1),
            );
        }

        async fn handle_event(&mut self, event: &Event<MockTypes>) {
            self.update(event).await.unwrap();
            self.commit().await.unwrap();
        }
    }

    // Now a lot of boilerplate to implement all teh traits for [`DataSource`], by dispatching each
    // method to either variant.
    #[async_trait]
    impl VersionedDataSource for DataSource {
        type Error = QueryError;

        async fn commit(&mut self) -> Result<(), Self::Error> {
            match self {
                Self::Sql(data_source) => data_source.commit().await.map_err(err_msg),
                Self::NoStorage(data_source) => data_source.commit().await.map_err(err_msg),
            }
        }

        async fn revert(&mut self) {
            match self {
                Self::Sql(data_source) => data_source.revert().await,
                Self::NoStorage(data_source) => data_source.revert().await,
            }
        }
    }

    #[async_trait]
    impl AvailabilityDataSource<MockTypes> for DataSource {
        type LeafRange<R> = BoxStream<'static, Fetch<LeafQueryData<MockTypes>>>
        where
            R: RangeBounds<usize> + Send;
        type BlockRange<R> = BoxStream<'static, Fetch<BlockQueryData<MockTypes>>>
        where
            R: RangeBounds<usize> + Send;
        type PayloadRange<R> = BoxStream<'static, Fetch<PayloadQueryData<MockTypes>>>
        where
            R: RangeBounds<usize> + Send;
        type VidCommonRange<R> = BoxStream<'static, Fetch<VidCommonQueryData<MockTypes>>>
        where
            R: RangeBounds<usize> + Send;

        async fn get_leaf<ID>(&self, id: ID) -> Fetch<LeafQueryData<MockTypes>>
        where
            ID: Into<LeafId<MockTypes>> + Send + Sync,
        {
            match self {
                Self::Sql(data_source) => data_source.get_leaf(id).await,
                Self::NoStorage(data_source) => data_source.get_leaf(id).await,
            }
        }

        async fn get_block<ID>(&self, id: ID) -> Fetch<BlockQueryData<MockTypes>>
        where
            ID: Into<BlockId<MockTypes>> + Send + Sync,
        {
            match self {
                Self::Sql(data_source) => data_source.get_block(id).await,
                Self::NoStorage(data_source) => data_source.get_block(id).await,
            }
        }

        async fn get_payload<ID>(&self, id: ID) -> Fetch<PayloadQueryData<MockTypes>>
        where
            ID: Into<BlockId<MockTypes>> + Send + Sync,
        {
            match self {
                Self::Sql(data_source) => data_source.get_payload(id).await,
                Self::NoStorage(data_source) => data_source.get_payload(id).await,
            }
        }

        async fn get_vid_common<ID>(&self, id: ID) -> Fetch<VidCommonQueryData<MockTypes>>
        where
            ID: Into<BlockId<MockTypes>> + Send + Sync,
        {
            match self {
                Self::Sql(data_source) => data_source.get_vid_common(id).await,
                Self::NoStorage(data_source) => data_source.get_vid_common(id).await,
            }
        }

        async fn get_leaf_range<R>(&self, range: R) -> Self::LeafRange<R>
        where
            R: RangeBounds<usize> + Send + 'static,
        {
            match self {
                Self::Sql(data_source) => data_source.get_leaf_range(range).await.boxed(),
                Self::NoStorage(data_source) => data_source.get_leaf_range(range).await.boxed(),
            }
        }

        async fn get_block_range<R>(&self, range: R) -> Self::BlockRange<R>
        where
            R: RangeBounds<usize> + Send + 'static,
        {
            match self {
                Self::Sql(data_source) => data_source.get_block_range(range).await.boxed(),
                Self::NoStorage(data_source) => data_source.get_block_range(range).await.boxed(),
            }
        }

        async fn get_payload_range<R>(&self, range: R) -> Self::PayloadRange<R>
        where
            R: RangeBounds<usize> + Send + 'static,
        {
            match self {
                Self::Sql(data_source) => data_source.get_payload_range(range).await.boxed(),
                Self::NoStorage(data_source) => data_source.get_payload_range(range).await.boxed(),
            }
        }

        async fn get_vid_common_range<R>(&self, range: R) -> Self::VidCommonRange<R>
        where
            R: RangeBounds<usize> + Send + 'static,
        {
            match self {
                Self::Sql(data_source) => data_source.get_vid_common_range(range).await.boxed(),
                Self::NoStorage(data_source) => {
                    data_source.get_vid_common_range(range).await.boxed()
                }
            }
        }

        async fn get_transaction(
            &self,
            hash: TransactionHash<MockTypes>,
        ) -> Fetch<TransactionQueryData<MockTypes>> {
            match self {
                Self::Sql(data_source) => data_source.get_transaction(hash).await,
                Self::NoStorage(data_source) => data_source.get_transaction(hash).await,
            }
        }
    }

    #[async_trait]
    impl UpdateAvailabilityData<MockTypes> for DataSource {
        type Error = QueryError;

        async fn insert_leaf(&mut self, leaf: LeafQueryData<MockTypes>) -> Result<(), Self::Error> {
            match self {
                Self::Sql(data_source) => data_source.insert_leaf(leaf).await.map_err(err_msg),
                Self::NoStorage(data_source) => {
                    data_source.insert_leaf(leaf).await.map_err(err_msg)
                }
            }
        }

        async fn insert_block(
            &mut self,
            block: BlockQueryData<MockTypes>,
        ) -> Result<(), Self::Error> {
            match self {
                Self::Sql(data_source) => data_source.insert_block(block).await.map_err(err_msg),
                Self::NoStorage(data_source) => {
                    data_source.insert_block(block).await.map_err(err_msg)
                }
            }
        }

        async fn insert_vid(
            &mut self,
            common: VidCommonQueryData<MockTypes>,
            share: Option<VidShare>,
        ) -> Result<(), Self::Error> {
            match self {
                Self::Sql(data_source) => {
                    data_source.insert_vid(common, share).await.map_err(err_msg)
                }
                Self::NoStorage(data_source) => {
                    data_source.insert_vid(common, share).await.map_err(err_msg)
                }
            }
        }
    }

    #[async_trait]
    impl NodeDataSource<MockTypes> for DataSource {
        async fn block_height(&self) -> QueryResult<usize> {
            match self {
                Self::Sql(data_source) => NodeDataSource::block_height(data_source).await,
                Self::NoStorage(data_source) => NodeDataSource::block_height(data_source).await,
            }
        }

        async fn count_transactions(&self) -> QueryResult<usize> {
            match self {
                Self::Sql(data_source) => data_source.count_transactions().await,
                Self::NoStorage(data_source) => data_source.count_transactions().await,
            }
        }

        async fn payload_size(&self) -> QueryResult<usize> {
            match self {
                Self::Sql(data_source) => data_source.payload_size().await,
                Self::NoStorage(data_source) => data_source.payload_size().await,
            }
        }

        async fn vid_share<ID>(&self, id: ID) -> QueryResult<VidShare>
        where
            ID: Into<BlockId<MockTypes>> + Send + Sync,
        {
            match self {
                Self::Sql(data_source) => data_source.vid_share(id).await,
                Self::NoStorage(data_source) => data_source.vid_share(id).await,
            }
        }

        async fn sync_status(&self) -> QueryResult<SyncStatus> {
            match self {
                Self::Sql(data_source) => data_source.sync_status().await,
                Self::NoStorage(data_source) => data_source.sync_status().await,
            }
        }

        async fn get_header_window(
            &self,
            start: impl Into<WindowStart<MockTypes>> + Send + Sync,
            end: u64,
        ) -> QueryResult<TimeWindowQueryData<Header<MockTypes>>> {
            match self {
                Self::Sql(data_source) => data_source.get_header_window(start, end).await,
                Self::NoStorage(data_source) => data_source.get_header_window(start, end).await,
            }
        }
    }

    #[async_trait]
    impl StatusDataSource for DataSource {
        async fn block_height(&self) -> QueryResult<usize> {
            match self {
                Self::Sql(data_source) => StatusDataSource::block_height(data_source).await,
                Self::NoStorage(data_source) => StatusDataSource::block_height(data_source).await,
            }
        }

        fn metrics(&self) -> &PrometheusMetrics {
            match self {
                Self::Sql(data_source) => data_source.metrics(),
                Self::NoStorage(data_source) => data_source.metrics(),
            }
        }
    }

    fn err_msg<E: Display>(err: E) -> QueryError {
        QueryError::Error {
            message: err.to_string(),
        }
    }
}

// These tests run the `postgres` Docker image, which doesn't work on Windows.
#[cfg(all(test, not(target_os = "windows")))]
mod test {
    use super::testing::DataSource;
    use crate::data_source::{availability_tests, status_tests};

    // For some reason this is the only way to import the macro defined in another module of this
    // crate.
    use crate::*;

    instantiate_availability_tests!(DataSource);
    instantiate_status_tests!(DataSource);
}
