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
//! # Storage Traits vs Data Source Traits
//!
//! Many of the traits defined in this module (e.g. [`NodeStorage`], [`ExplorerStorage`], and
//! others) are nearly identical to the corresponding data source traits (e.g.
//! [`NodeDataSource`](crate::node::NodeDataSource),
//! [`ExplorerDataSource`](crate::explorer::ExplorerDataSource), etc). They typically differ in
//! mutability: the storage traits are intended to be implemented on storage
//! [transactions](super::Transaction), and because even reading may update the internal
//! state of a transaction, such as a buffer or database cursor, these traits typically take `&mut
//! self`. This is not a barrier for concurrency since there may be many transactions open
//! simultaneously from a single data source. The data source traits, meanwhile, are implemented on
//! the data source itself. Internally, they usually open a fresh transaction and do all their work
//! on the transaction, not modifying the data source itself, so they take `&self`.
//!
//! For traits that differ _only_ in the mutability of the `self` parameter, it is almost possible
//! to combine them into a single trait whose methods take `self` by value, and implementing said
//! traits for the reference types `&SomeDataSource` and `&mut SomeDataSourceTransaction`. There are
//! two problems with this approach, which lead us to prefer the slight redundance of having
//! separate versions of the traits with mutable and immutable methods:
//! * The trait bounds quickly get out of hand, since we now have trait bounds not only on the type
//!   itself, but also on references to that type, and the reference also requires the introduction
//!   of an additional lifetime parameter.
//! * We run into a longstanding [`rustc` bug](https://github.com/rust-lang/rust/issues/85063) in
//!   which type inference diverges when given trait bounds on reference types, even when
//!   theoretically the types are uniquely inferrable. This issue can be worked around by [explicitly
//!   specifying type paramters at every call site](https://users.rust-lang.org/t/type-recursion-when-trait-bound-is-added-on-reference-type/74525/2),
//!   but this further exacerbates the ergonomic issues with this approach, past the point of
//!   viability.
//!
//! Occasionally, there may be further differences between the data source traits and corresponding
//! storage traits. For example, [`AvailabilityStorage`] also differs from
//! [`AvailabilityDataSource`](crate::availability::AvailabilityDataSource) in fallibility.
//!

use crate::{
    availability::{
        BlockId, BlockQueryData, LeafId, LeafQueryData, PayloadQueryData, QueryableHeader,
        QueryablePayload, TransactionHash, TransactionQueryData, VidCommonQueryData,
    },
    explorer::{
        query_data::{
            BlockDetail, BlockIdentifier, BlockSummary, ExplorerSummary, GetBlockDetailError,
            GetBlockSummariesError, GetBlockSummariesRequest, GetExplorerSummaryError,
            GetSearchResultsError, GetTransactionDetailError, GetTransactionSummariesError,
            GetTransactionSummariesRequest, SearchResult, TransactionDetailResponse,
            TransactionIdentifier, TransactionSummary,
        },
        traits::{ExplorerHeader, ExplorerTransaction},
    },
    merklized_state::{MerklizedState, Snapshot},
    node::{SyncStatus, TimeWindowQueryData, WindowStart},
    Header, Payload, QueryResult, Transaction, VidShare,
};
use async_trait::async_trait;
use hotshot_types::traits::node_implementation::NodeType;
use jf_merkle_tree::prelude::MerkleProof;
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
pub trait AvailabilityStorage<Types>: Send + Sync
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
{
    async fn get_leaf(&mut self, id: LeafId<Types>) -> QueryResult<LeafQueryData<Types>>;
    async fn get_block(&mut self, id: BlockId<Types>) -> QueryResult<BlockQueryData<Types>>;
    async fn get_header(&mut self, id: BlockId<Types>) -> QueryResult<Header<Types>>;
    async fn get_payload(&mut self, id: BlockId<Types>) -> QueryResult<PayloadQueryData<Types>>;
    async fn get_vid_common(
        &mut self,
        id: BlockId<Types>,
    ) -> QueryResult<VidCommonQueryData<Types>>;

    async fn get_leaf_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<LeafQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send + 'static;
    async fn get_block_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<BlockQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send + 'static;
    async fn get_payload_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<PayloadQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send + 'static;
    async fn get_vid_common_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<VidCommonQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send + 'static;

    async fn get_transaction(
        &mut self,
        hash: TransactionHash<Types>,
    ) -> QueryResult<TransactionQueryData<Types>>;
}

#[async_trait]
pub trait NodeStorage<Types: NodeType> {
    async fn block_height(&mut self) -> QueryResult<usize>;
    async fn count_transactions(&mut self) -> QueryResult<usize>;
    async fn payload_size(&mut self) -> QueryResult<usize>;
    async fn vid_share<ID>(&mut self, id: ID) -> QueryResult<VidShare>
    where
        ID: Into<BlockId<Types>> + Send + Sync;
    async fn get_header_window(
        &mut self,
        start: impl Into<WindowStart<Types>> + Send + Sync,
        end: u64,
    ) -> QueryResult<TimeWindowQueryData<Header<Types>>>;

    /// Search the database for missing objects and generate a report.
    async fn sync_status(&mut self) -> QueryResult<SyncStatus>;
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
    Header<Types>: ExplorerHeader<Types> + QueryableHeader<Types>,
    Transaction<Types>: ExplorerTransaction,
    Payload<Types>: QueryablePayload<Types>,
{
    /// `get_block_detail` is a method that retrieves the details of a specific
    /// block from the blockchain.  The block is identified by the given
    /// [BlockIdentifier].
    async fn get_block_detail(
        &mut self,
        request: BlockIdentifier<Types>,
    ) -> Result<BlockDetail<Types>, GetBlockDetailError>;

    /// `get_block_summaries` is a method that retrieves a list of block
    /// summaries from the blockchain.  The list is generated from the given
    /// [GetBlockSummariesRequest].
    async fn get_block_summaries(
        &mut self,
        request: GetBlockSummariesRequest<Types>,
    ) -> Result<Vec<BlockSummary<Types>>, GetBlockSummariesError>;

    /// `get_transaction_detail` is a method that retrieves the details of a
    /// specific transaction from the blockchain.  The transaction is identified
    /// by the given [TransactionIdentifier].
    async fn get_transaction_detail(
        &mut self,
        request: TransactionIdentifier<Types>,
    ) -> Result<TransactionDetailResponse<Types>, GetTransactionDetailError>;

    /// `get_transaction_summaries` is a method that retrieves a list of
    /// transaction summaries from the blockchain.  The list is generated from
    /// the given [GetTransactionSummariesRequest].
    async fn get_transaction_summaries(
        &mut self,
        request: GetTransactionSummariesRequest<Types>,
    ) -> Result<Vec<TransactionSummary<Types>>, GetTransactionSummariesError>;

    /// `get_explorer_summary` is a method that retrieves a summary overview of
    /// the blockchain.  This is useful for displaying information that
    /// indicates the overall status of the block chain.
    async fn get_explorer_summary(
        &mut self,
    ) -> Result<ExplorerSummary<Types>, GetExplorerSummaryError>;

    /// `get_search_results` is a method that retrieves the results of a search
    /// query against the blockchain.  The results are generated from the given
    /// query string.
    async fn get_search_results(
        &mut self,
        query: String,
    ) -> Result<SearchResult<Types>, GetSearchResultsError>;
}

/// This trait defines methods that a data source should implement
/// It enables retrieval of the membership path for a leaf node, which can be used to reconstruct the Merkle tree state.
#[async_trait]
pub trait MerklizedStateStorage<Types, State, const ARITY: usize>
where
    Types: NodeType,
    State: MerklizedState<Types, ARITY>,
{
    async fn get_path(
        &mut self,
        snapshot: Snapshot<Types, State, ARITY>,
        key: State::Key,
    ) -> QueryResult<MerkleProof<State::Entry, State::Key, State::T, ARITY>>;
}

#[async_trait]
pub trait MerklizedStateHeightStorage {
    async fn get_last_state_height(&mut self) -> QueryResult<usize>;
}
