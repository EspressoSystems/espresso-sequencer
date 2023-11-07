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

use super::query_data::{
    BlockHash, BlockQueryData, LeafHash, LeafQueryData, QueryableBlock, TransactionHash,
    TransactionIndex,
};
use crate::{Block, Deltas, Resolvable};
use futures::stream::Stream;
use hotshot_types::traits::{
    node_implementation::{NodeImplementation, NodeType},
    signature_key::EncodedPublicKey,
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::RangeBounds;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResourceId<H> {
    Number(usize),
    Hash(H),
}

impl<H: Display> Display for ResourceId<H> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Hash(h) => write!(f, "{h}"),
        }
    }
}

pub type BlockId<Types> = ResourceId<BlockHash<Types>>;
pub type LeafId<Types, I> = ResourceId<LeafHash<Types, I>>;

#[derive(Clone, Debug, Snafu, Deserialize, Serialize)]
#[snafu(visibility(pub))]
pub enum QueryError {
    /// The requested resource does not exist or is not known to this query service.
    NotFound,
    /// The requested resource exists but is not currently available.
    ///
    /// In most cases a missing resource can be recovered from DA.
    Missing,
    /// There was an error while trying to fetch the requested resource.
    #[snafu(display("Failed to fetch requested resource: {message}"))]
    Error { message: String },
}

pub type QueryResult<T> = Result<T, QueryError>;

pub trait AvailabilityDataSource<Types: NodeType, I: NodeImplementation<Types>>
where
    Block<Types>: QueryableBlock,
{
    type LeafStreamType: Stream<Item = LeafQueryData<Types, I>> + Send;
    type BlockStreamType: Stream<Item = BlockQueryData<Types>> + Send;

    type LeafRange<'a, R>: 'a + Iterator<Item = QueryResult<LeafQueryData<Types, I>>>
    where
        Self: 'a,
        R: RangeBounds<usize>;
    type BlockRange<'a, R>: 'a + Iterator<Item = QueryResult<BlockQueryData<Types>>>
    where
        Self: 'a,
        R: RangeBounds<usize>;

    fn get_leaf(&self, id: LeafId<Types, I>) -> QueryResult<LeafQueryData<Types, I>>;
    fn get_block(&self, id: BlockId<Types>) -> QueryResult<BlockQueryData<Types>>;

    fn get_leaf_range<R>(&self, range: R) -> QueryResult<Self::LeafRange<'_, R>>
    where
        R: RangeBounds<usize>;
    fn get_block_range<R>(&self, range: R) -> QueryResult<Self::BlockRange<'_, R>>
    where
        R: RangeBounds<usize>;

    /// Returns the block containing a transaction with the given `hash` and the transaction's
    /// position in the block.
    fn get_block_with_transaction(
        &self,
        hash: TransactionHash<Types>,
    ) -> QueryResult<(BlockQueryData<Types>, TransactionIndex<Types>)>;

    fn get_proposals(
        &self,
        proposer: &EncodedPublicKey,
        limit: Option<usize>,
    ) -> QueryResult<Vec<LeafQueryData<Types, I>>>;
    fn count_proposals(&self, proposer: &EncodedPublicKey) -> QueryResult<usize>;

    fn subscribe_leaves(&self, height: usize) -> QueryResult<Self::LeafStreamType>;
    fn subscribe_blocks(&self, height: usize) -> QueryResult<Self::BlockStreamType>;
}

pub trait UpdateAvailabilityData<Types: NodeType, I: NodeImplementation<Types>>
where
    Block<Types>: QueryableBlock,
{
    type Error: Error + Debug;
    fn insert_leaf(&mut self, leaf: LeafQueryData<Types, I>) -> Result<(), Self::Error>
    where
        Deltas<Types, I>: Resolvable<Block<Types>>;
    fn insert_block(&mut self, block: BlockQueryData<Types>) -> Result<(), Self::Error>;
}
