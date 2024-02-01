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
        TransactionHash, TransactionIndex, UpdateAvailabilityData,
    },
    data_source::VersionedDataSource,
    node::{NodeDataSource, UpdateNodeData},
    Header, Payload, QueryError, QueryResult, SignatureKey,
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

#[async_trait]
impl<Types: NodeType> AvailabilityStorage<Types> for NoStorage
where
    Payload<Types>: QueryablePayload,
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

    async fn get_block_with_transaction(
        &self,
        _hash: TransactionHash<Types>,
    ) -> QueryResult<(BlockQueryData<Types>, TransactionIndex<Types>)> {
        Err(QueryError::Missing)
    }
}

#[async_trait]
impl<Types: NodeType> UpdateAvailabilityData<Types> for NoStorage
where
    Payload<Types>: QueryablePayload,
{
    type Error = Infallible;

    async fn insert_leaf(&mut self, _leaf: LeafQueryData<Types>) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn insert_block(&mut self, _block: BlockQueryData<Types>) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[async_trait]
impl<Types: NodeType> NodeDataSource<Types> for NoStorage
where
    Payload<Types>: QueryablePayload,
{
    async fn block_height(&self) -> QueryResult<usize> {
        Ok(0)
    }

    async fn get_proposals(
        &self,
        _id: &SignatureKey<Types>,
        _limit: Option<usize>,
    ) -> QueryResult<Vec<LeafQueryData<Types>>> {
        Err(QueryError::Missing)
    }

    async fn count_proposals(&self, _id: &SignatureKey<Types>) -> QueryResult<usize> {
        Err(QueryError::Missing)
    }
}

#[async_trait]
impl<Types: NodeType> UpdateNodeData<Types> for NoStorage
where
    Payload<Types>: QueryablePayload,
{
    type Error = Infallible;

    async fn insert_leaf(&mut self, _leaf: LeafQueryData<Types>) -> Result<(), Self::Error> {
        Ok(())
    }
}
