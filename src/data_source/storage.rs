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

//! Persistent storage for data sources.
//!
//! Naturally, an archival query service such as this is heavily dependent on a persistent storage
//! implementation. This module defines the interfaces required of this storage. Any storage layer
//! implementing the appropriate interfaces can be used as the storage layer when constructing a
//! [`FetchingDataSource`](super::FetchingDataSource), which can in turn be used to instantiate the
//! REST APIs provided by this crate.
//!
//! This module also comes with a few pre-built persistence implementations:
//! * [`SqlStorage`]
//! * [`FileSystemStorage`]
//! * [`NoStorage`]
//!

use crate::{
    availability::{
        BlockId, BlockQueryData, LeafId, LeafQueryData, PayloadQueryData, QueryableHeader,
        QueryablePayload, TransactionHash, TransactionIndex, UpdateAvailabilityData,
        VidCommonQueryData,
    },
    data_source::VersionedDataSource,
    explorer, Header, Payload, QueryResult,
};
use async_trait::async_trait;
use hotshot_types::traits::node_implementation::NodeType;
use std::ops::RangeBounds;

pub mod fs;
mod ledger_log;
pub mod no_storage;
pub mod pruning;
pub mod sql;

#[cfg(feature = "file-system-data-source")]
pub use fs::FileSystemStorage;
#[cfg(feature = "no-storage")]
pub use no_storage::NoStorage;
#[cfg(feature = "sql-data-source")]
pub use sql::SqlStorage;

use self::pruning::PruneStorage;

/// Persistent storage for a HotShot blockchain.
///
/// This trait defines the interface which must be provided by the storage layer in order to
/// implement an availability data source. It is very similar to
/// [`AvailabilityDataSource`](crate::availability::AvailabilityDataSource) with every occurrence of
/// [`Fetch`](crate::availability::Fetch) replaced by [`QueryResult`]. This is not a coincidence.
/// The purpose of the storage layer is to provide all of the functionality of the data source
/// layer, but independent of an external fetcher for missing data. Thus, when the storage layer
/// encounters missing, corrupt, or inaccessible data, it simply gives up and replaces the missing
/// data with [`Err`], rather than creating an asynchronous fetch request to retrieve the missing
/// data.
///
/// Rust gives us ways to abstract and deduplicate these two similar APIs, but they do not lead to a
/// better interface.
#[async_trait]
pub trait AvailabilityStorage<Types>:
    UpdateAvailabilityData<Types> + VersionedDataSource + PruneStorage + Send + Sync
where
    Types: NodeType,
    Payload<Types>: QueryablePayload,
{
    async fn get_leaf(&self, id: LeafId<Types>) -> QueryResult<LeafQueryData<Types>>;
    async fn get_block(&self, id: BlockId<Types>) -> QueryResult<BlockQueryData<Types>>;
    async fn get_header(&self, id: BlockId<Types>) -> QueryResult<Header<Types>>;
    async fn get_payload(&self, id: BlockId<Types>) -> QueryResult<PayloadQueryData<Types>>;
    async fn get_vid_common(&self, id: BlockId<Types>) -> QueryResult<VidCommonQueryData<Types>>;

    async fn get_leaf_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<LeafQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send + 'static;
    async fn get_block_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<BlockQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send + 'static;
    async fn get_payload_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<PayloadQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send + 'static;
    async fn get_vid_common_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<VidCommonQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send + 'static;

    async fn get_block_with_transaction(
        &self,
        hash: TransactionHash<Types>,
    ) -> QueryResult<(BlockQueryData<Types>, TransactionIndex<Types>)>;
}

/// An interface for querying Data and Statistics from the HotShot Blockchain.
///
/// This interface provides methods that allows the enabling of querying data
/// concerning the blockchain from the stored data for use with a
/// block explorer.  It does not provide the same guarantees as the
/// Availability data source with data fetching.  It is not concerned with
/// being up-to-date or having all of the data required, but rather it is
/// concerned with providing the requested data as quickly as possible, and in
/// a way that can be easily cached.
#[async_trait]
pub trait ExplorerStorage<Types>
where
    Types: NodeType,
    Header<Types>: explorer::traits::ExplorerHeader<Types> + QueryableHeader<Types>,
    Payload<Types>: QueryablePayload,
{
    /// `get_block_detail` is a method that retrieves the details of a specific
    /// block from the blockchain.  The block is identified by the given
    /// [BlockIdentifier].
    async fn get_block_detail(
        &self,
        request: explorer::data_source::BlockIdentifier<Types>,
    ) -> Result<explorer::data_source::BlockDetail<Types>, explorer::data_source::GetBlockDetailError>;

    /// `get_block_summaries` is a method that retrieves a list of block
    /// summaries from the blockchain.  The list is generated from the given
    /// [GetBlockSummariesRequest].
    async fn get_block_summaries(
        &self,
        request: explorer::data_source::GetBlockSummariesRequest<Types>,
    ) -> Result<
        Vec<explorer::data_source::BlockSummary<Types>>,
        explorer::data_source::GetBlockSummariesError,
    >;

    /// `get_transaction_detail` is a method that retrieves the details of a
    /// specific transaction from the blockchain.  The transaction is identified
    /// by the given [TransactionIdentifier].
    async fn get_transaction_detail(
        &self,
        request: explorer::data_source::TransactionIdentifier<Types>,
    ) -> Result<
        explorer::data_source::TransactionDetailResponse<Types>,
        explorer::data_source::GetTransactionDetailError,
    >;

    /// `get_transaction_summaries` is a method that retrieves a list of
    /// transaction summaries from the blockchain.  The list is generated from
    /// the given [GetTransactionSummariesRequest].
    async fn get_transaction_summaries(
        &self,
        request: explorer::data_source::GetTransactionSummariesRequest<Types>,
    ) -> Result<
        Vec<explorer::data_source::TransactionSummary<Types>>,
        explorer::data_source::GetTransactionSummariesError,
    >;

    /// `get_explorer_summary` is a method that retrieves a summary overview of
    /// the blockchain.  This is useful for displaying information that
    /// indicates the overall status of the block chain.
    async fn get_explorer_summary(
        &self,
    ) -> Result<
        explorer::data_source::ExplorerSummary<Types>,
        explorer::data_source::GetExplorerSummaryError,
    >;

    /// `get_search_results` is a method that retrieves the results of a search
    /// query against the blockchain.  The results are generated from the given
    /// query string.
    async fn get_search_results(
        &self,
        query: String,
    ) -> Result<
        explorer::data_source::SearchResult<Types>,
        explorer::data_source::GetSearchResultsError,
    >;
}
