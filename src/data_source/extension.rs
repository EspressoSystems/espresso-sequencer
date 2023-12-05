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

use super::VersionedDataSource;
use crate::{
    availability::{
        AvailabilityDataSource, BlockId, BlockQueryData, LeafId, LeafQueryData, QueryableBlock,
        TransactionHash, TransactionIndex, UpdateAvailabilityData,
    },
    metrics::PrometheusMetrics,
    status::StatusDataSource,
    Block, QueryResult,
};
use async_trait::async_trait;
use commit::Committable;
use hotshot_types::traits::{node_implementation::NodeType, signature_key::EncodedPublicKey};
use std::ops::RangeBounds;

/// Wrapper to add extensibility to an existing data source.
///
/// [`ExtensibleDataSource`] adds app-specific data to any existing data source. It implements all
/// the data source traits defined in this crate as long as the underlying data source does so,
/// which means it can be used as state for instantiating the APIs defined in this crate. At the
/// same time, it provides access to an application-defined state type, which means it can also be
/// used to implement application-specific endpoints.
///
/// [`ExtensibleDataSource`] implements `AsRef<U>` and `AsMut<U>` for some user-defined type `U`, so
/// your API extensions can always access application-specific state from [`ExtensibleDataSource`].
/// We can use this to complete the [UTXO example](crate#extension) by extending our data source
/// with an index to look up transactions by the UTXOs they contain:
///
/// ```
/// # use async_trait::async_trait;
/// # use hotshot_query_service::availability::{AvailabilityDataSource, TransactionIndex};
/// # use hotshot_query_service::data_source::ExtensibleDataSource;
/// # use hotshot_query_service::testing::{
/// #   mocks::{
/// #       MockNodeImpl as AppNodeImpl, MockTypes as AppTypes,
/// #   },
/// # };
/// # use std::collections::HashMap;
/// # #[async_trait]
/// # trait UtxoDataSource: AvailabilityDataSource<AppTypes, AppNodeImpl> {
/// #   async fn find_utxo(&self, utxo: u64) -> Option<(usize, TransactionIndex<AppTypes>, usize)>;
/// # }
/// type UtxoIndex = HashMap<u64, (usize, TransactionIndex<AppTypes>, usize)>;
///
/// #[async_trait]
/// impl<UnderlyingDataSource> UtxoDataSource for
///     ExtensibleDataSource<UnderlyingDataSource, UtxoIndex>
/// where
///     UnderlyingDataSource: AvailabilityDataSource<AppTypes, AppNodeImpl> + Send + Sync,
/// {
///     async fn find_utxo(&self, utxo: u64) -> Option<(usize, TransactionIndex<AppTypes>, usize)> {
///         self.as_ref().get(&utxo).cloned()
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ExtensibleDataSource<D, U> {
    data_source: D,
    user_data: U,
}

impl<D, U> ExtensibleDataSource<D, U> {
    pub fn new(data_source: D, user_data: U) -> Self {
        Self {
            data_source,
            user_data,
        }
    }

    /// Access the underlying data source.
    ///
    /// This functionality is provided as an inherent method rather than an implementation of the
    /// [`AsRef`] trait so that `self.as_ref()` unambiguously returns `&U`, helping with type
    /// inference.
    pub fn inner(&self) -> &D {
        &self.data_source
    }

    /// Mutably access the underlying data source.
    ///
    /// This functionality is provided as an inherent method rather than an implementation of the
    /// [`AsMut`] trait so that `self.as_mut()` unambiguously returns `&U`, helping with type
    /// inference.
    pub fn inner_mut(&mut self) -> &mut D {
        &mut self.data_source
    }
}

impl<D, U> AsRef<U> for ExtensibleDataSource<D, U> {
    fn as_ref(&self) -> &U {
        &self.user_data
    }
}

impl<D, U> AsMut<U> for ExtensibleDataSource<D, U> {
    fn as_mut(&mut self) -> &mut U {
        &mut self.user_data
    }
}

#[async_trait]
impl<D, U> VersionedDataSource for ExtensibleDataSource<D, U>
where
    D: VersionedDataSource + Send,
    U: Send,
{
    type Error = D::Error;

    async fn commit(&mut self) -> Result<(), Self::Error> {
        self.data_source.commit().await
    }

    async fn revert(&mut self) {
        self.data_source.revert().await
    }
}

#[async_trait]
impl<D, U, Types> AvailabilityDataSource<Types> for ExtensibleDataSource<D, U>
where
    D: AvailabilityDataSource<Types> + Send + Sync,
    U: Send + Sync,
    Types: NodeType,
    Block<Types>: QueryableBlock,
    <Types as NodeType>::BlockPayload: Committable,
{
    type LeafStream = D::LeafStream;
    type BlockStream = D::BlockStream;

    type LeafRange<'a, R> = D::LeafRange<'a, R>
    where
        Self: 'a,
        R: RangeBounds<usize> + Send;
    type BlockRange<'a, R> = D::BlockRange<'a, R>
    where
        Self: 'a,
        R: RangeBounds<usize> + Send;

    async fn get_leaf<ID>(&self, id: ID) -> QueryResult<LeafQueryData<Types>>
    where
        ID: Into<LeafId<Types>> + Send + Sync,
    {
        self.data_source.get_leaf(id).await
    }
    async fn get_block<ID>(&self, id: ID) -> QueryResult<BlockQueryData<Types>>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        self.data_source.get_block(id).await
    }
    async fn get_leaf_range<R>(&self, range: R) -> QueryResult<Self::LeafRange<'_, R>>
    where
        R: RangeBounds<usize> + Send,
    {
        self.data_source.get_leaf_range(range).await
    }
    async fn get_block_range<R>(&self, range: R) -> QueryResult<Self::BlockRange<'_, R>>
    where
        R: RangeBounds<usize> + Send,
    {
        self.data_source.get_block_range(range).await
    }
    async fn get_block_with_transaction(
        &self,
        hash: TransactionHash<Types>,
    ) -> QueryResult<(BlockQueryData<Types>, TransactionIndex<Types>)> {
        self.data_source.get_block_with_transaction(hash).await
    }
    async fn get_proposals(
        &self,
        proposer: &EncodedPublicKey,
        limit: Option<usize>,
    ) -> QueryResult<Vec<LeafQueryData<Types>>> {
        self.data_source.get_proposals(proposer, limit).await
    }
    async fn count_proposals(&self, proposer: &EncodedPublicKey) -> QueryResult<usize> {
        self.data_source.count_proposals(proposer).await
    }
    async fn subscribe_leaves(&self, height: usize) -> QueryResult<Self::LeafStream> {
        self.data_source.subscribe_leaves(height).await
    }
    async fn subscribe_blocks(&self, height: usize) -> QueryResult<Self::BlockStream> {
        self.data_source.subscribe_blocks(height).await
    }
}

#[async_trait]
impl<D, U, Types> UpdateAvailabilityData<Types> for ExtensibleDataSource<D, U>
where
    D: UpdateAvailabilityData<Types> + Send + Sync,
    U: Send + Sync,
    Types: NodeType,
    Block<Types>: QueryableBlock,
    <Types as NodeType>::BlockPayload: Committable,
{
    type Error = D::Error;

    async fn insert_leaf(&mut self, leaf: LeafQueryData<Types>) -> Result<(), Self::Error> {
        self.data_source.insert_leaf(leaf).await
    }

    async fn insert_block(&mut self, block: BlockQueryData<Types>) -> Result<(), Self::Error> {
        self.data_source.insert_block(block).await
    }
}

#[async_trait]
impl<D, U> StatusDataSource for ExtensibleDataSource<D, U>
where
    D: StatusDataSource + Send + Sync,
    U: Send + Sync,
{
    async fn block_height(&self) -> QueryResult<usize> {
        self.data_source.block_height().await
    }

    fn metrics(&self) -> &PrometheusMetrics {
        self.data_source.metrics()
    }
}

#[cfg(any(test, feature = "testing"))]
mod impl_testable_data_source {
    use super::*;
    use crate::testing::mocks::TestableDataSource;

    #[async_trait]
    impl<D, U> TestableDataSource for ExtensibleDataSource<D, U>
    where
        D: TestableDataSource,
        U: Default + Send + Sync + 'static,
    {
        type Storage = D::Storage;

        async fn create(node_id: usize) -> Self::Storage {
            D::create(node_id).await
        }

        async fn connect(storage: &Self::Storage) -> Self {
            Self::new(D::connect(storage).await, Default::default())
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::{data_source_tests, FileSystemDataSource};
    use super::ExtensibleDataSource;
    use crate::testing::mocks::{MockNodeImpl, MockTypes};

    // For some reason this is the only way to import the macro defined in another module of this
    // crate.
    use crate::*;

    instantiate_data_source_tests!(
        ExtensibleDataSource<FileSystemDataSource<MockTypes, MockNodeImpl>, ()>
    );
}
