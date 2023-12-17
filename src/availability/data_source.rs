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
    BlockQueryData, LeafQueryData, QueryablePayload, TransactionHash, TransactionIndex,
};
use crate::{Header, Leaf, Payload, QueryResult};
use async_trait::async_trait;
use commit::{Commitment, Committable};
use derivative::Derivative;
use derive_more::{Display, From};
use futures::stream::Stream;
use hotshot_types::traits::node_implementation::NodeType;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Debug;
use std::ops::RangeBounds;

#[derive(Derivative, From, Display)]
#[derivative(Ord = "feature_allow_slow_enum")]
#[derivative(
    Copy(bound = ""),
    Debug(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Ord(bound = ""),
    Hash(bound = "")
)]
pub enum ResourceId<T: Committable> {
    #[display(fmt = "{_0}")]
    Number(usize),
    #[display(fmt = "{_0}")]
    Hash(Commitment<T>),
}

impl<T: Committable> Clone for ResourceId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Committable> PartialOrd for ResourceId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub type BlockId<Types> = ResourceId<Header<Types>>;
pub type LeafId<Types> = ResourceId<Leaf<Types>>;

#[async_trait]
pub trait AvailabilityDataSource<Types: NodeType>
where
    Payload<Types>: QueryablePayload,
{
    type LeafStream: Stream<Item = QueryResult<LeafQueryData<Types>>> + Unpin + Send;
    type BlockStream: Stream<Item = QueryResult<BlockQueryData<Types>>> + Unpin + Send;

    type LeafRange<'a, R>: 'a + Stream<Item = QueryResult<LeafQueryData<Types>>> + Unpin
    where
        Self: 'a,
        R: RangeBounds<usize> + Send;
    type BlockRange<'a, R>: 'a + Stream<Item = QueryResult<BlockQueryData<Types>>> + Unpin
    where
        Self: 'a,
        R: RangeBounds<usize> + Send;

    async fn get_leaf<ID>(&self, id: ID) -> QueryResult<LeafQueryData<Types>>
    where
        ID: Into<LeafId<Types>> + Send + Sync;

    async fn get_block<ID>(&self, id: ID) -> QueryResult<BlockQueryData<Types>>
    where
        ID: Into<BlockId<Types>> + Send + Sync;

    async fn get_leaf_range<R>(&self, range: R) -> QueryResult<Self::LeafRange<'_, R>>
    where
        R: RangeBounds<usize> + Send;

    async fn get_block_range<R>(&self, range: R) -> QueryResult<Self::BlockRange<'_, R>>
    where
        R: RangeBounds<usize> + Send;

    /// Returns the block containing a transaction with the given `hash` and the transaction's
    /// position in the block.
    async fn get_block_with_transaction(
        &self,
        hash: TransactionHash<Types>,
    ) -> QueryResult<(BlockQueryData<Types>, TransactionIndex<Types>)>;

    async fn subscribe_leaves(&self, height: usize) -> QueryResult<Self::LeafStream>;
    async fn subscribe_blocks(&self, height: usize) -> QueryResult<Self::BlockStream>;
}

#[async_trait]
pub trait UpdateAvailabilityData<Types: NodeType> {
    type Error: Error + Debug + Send + Sync + 'static;
    async fn insert_leaf(&mut self, leaf: LeafQueryData<Types>) -> Result<(), Self::Error>;
    async fn insert_block(&mut self, block: BlockQueryData<Types>) -> Result<(), Self::Error>;
}
