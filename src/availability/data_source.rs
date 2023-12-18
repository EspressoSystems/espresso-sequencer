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

use super::{
    fetch::Fetch,
    query_data::{
        BlockQueryData, LeafQueryData, QueryablePayload, TransactionHash, TransactionIndex,
    },
};
use crate::{Header, Leaf, Payload};
use async_trait::async_trait;
use commit::{Commitment, Committable};
use derivative::Derivative;
use derive_more::{Display, From};
use futures::stream::{BoxStream, Stream, StreamExt};
use hotshot_types::traits::node_implementation::NodeType;
use std::{cmp::Ordering, error::Error, fmt::Debug, ops::RangeBounds};

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

/// An interface for querying a HotShot blockchain.
///
/// This interface provides expressive queries over all the data which is made available by HotShot
/// consensus. The data exposed by this interface consists entirely of _normative_ data: data which
/// every correct HotShot node or light client will agree on, and which is guaranteed by consensus
/// to be immutable. This immutability has an interesting consequence: all of the methods exposed by
/// this trait are _pure_: given equivalent inputs, the same method will always return equivalent
/// outputs[^1].
///
/// This purity property has a further consequence: none of the methods defined here can fail! Even
/// if you query for a block at a position past the end of the current chain -- a block which does
/// not exist yet -- the query will not fail. It will return an in-progress [`Fetch`] which, when it
/// finally does resolve, resolves to the unique block at that position in the chain. All subsequent
/// queries for the same position will eventually resolve to the same block.
///
/// In other words, the abstraction is that of an infinitely long chain which exists statically, in
/// its entirety, at all times. In reality, of course, this chain is being produced incrementally
/// and has a finite length at any given time. But all this means is that some queries may take a
/// long time to resolve while others may resolve immediately.
///
/// [^1]: The data returned by these methods are wrapped in [`Fetch`], which does not implement
///       [`PartialEq]`. So to speak of equivalent outputs, we need to define an equivalence
///       relation on [`Fetch<T>`]. The relation we will use is defined when `T: PartialEq`, and
///       defines two fetches `f1` and `f2` as equivalent when `f1.await == f2.await`. That is,
///       depending on when you call a certain method, you may or may not get a response
///       immediately. But whenever you do get the data you requested, it is unique for that
///       combination of inputs.
#[async_trait]
pub trait AvailabilityDataSource<Types: NodeType>
where
    Payload<Types>: QueryablePayload,
{
    type LeafRange<R>: Stream<Item = Fetch<LeafQueryData<Types>>> + Unpin + Send + 'static
    where
        R: RangeBounds<usize> + Send;
    type BlockRange<R>: Stream<Item = Fetch<BlockQueryData<Types>>> + Unpin + Send + 'static
    where
        R: RangeBounds<usize> + Send;

    async fn get_leaf<ID>(&self, id: ID) -> Fetch<LeafQueryData<Types>>
    where
        ID: Into<LeafId<Types>> + Send + Sync;

    async fn get_block<ID>(&self, id: ID) -> Fetch<BlockQueryData<Types>>
    where
        ID: Into<BlockId<Types>> + Send + Sync;

    async fn get_leaf_range<R>(&self, range: R) -> Self::LeafRange<R>
    where
        R: RangeBounds<usize> + Send + 'static;

    async fn get_block_range<R>(&self, range: R) -> Self::BlockRange<R>
    where
        R: RangeBounds<usize> + Send + 'static;

    /// Returns the block containing a transaction with the given `hash` and the transaction's
    /// position in the block.
    async fn get_block_with_transaction(
        &self,
        hash: TransactionHash<Types>,
    ) -> Fetch<(BlockQueryData<Types>, TransactionIndex<Types>)>;

    async fn subscribe_blocks(&self, from: usize) -> BoxStream<'static, BlockQueryData<Types>> {
        self.get_block_range(from..)
            .await
            .then(Fetch::resolve)
            .boxed()
    }

    async fn subscribe_leaves(&self, from: usize) -> BoxStream<'static, LeafQueryData<Types>> {
        self.get_leaf_range(from..)
            .await
            .then(Fetch::resolve)
            .boxed()
    }
}

#[async_trait]
pub trait UpdateAvailabilityData<Types: NodeType> {
    type Error: Error + Debug + Send + Sync + 'static;
    async fn insert_leaf(&mut self, leaf: LeafQueryData<Types>) -> Result<(), Self::Error>;
    async fn insert_block(&mut self, block: BlockQueryData<Types>) -> Result<(), Self::Error>;
}
