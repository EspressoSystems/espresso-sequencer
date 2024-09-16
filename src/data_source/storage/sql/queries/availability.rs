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

//! Availability storage implementation for a database query engine.

use super::{
    super::transaction::{Transaction, TransactionMode},
    QueryBuilder, BLOCK_COLUMNS, LEAF_COLUMNS, PAYLOAD_COLUMNS, VID_COMMON_COLUMNS,
};
use crate::{
    availability::{
        BlockId, BlockQueryData, LeafId, LeafQueryData, PayloadQueryData, QueryableHeader,
        QueryablePayload, TransactionHash, TransactionQueryData, VidCommonQueryData,
    },
    data_source::storage::AvailabilityStorage,
    types::HeightIndexed,
    ErrorSnafu, Header, Payload, QueryError, QueryResult,
};
use async_trait::async_trait;
use futures::stream::{StreamExt, TryStreamExt};
use hotshot_types::traits::node_implementation::NodeType;
use snafu::OptionExt;
use sqlx::FromRow;
use std::ops::RangeBounds;

#[async_trait]
impl<Mode, Types> AvailabilityStorage<Types> for Transaction<Mode>
where
    Types: NodeType,
    Mode: TransactionMode,
    Payload<Types>: QueryablePayload<Types>,
    Header<Types>: QueryableHeader<Types>,
{
    async fn get_leaf(&mut self, id: LeafId<Types>) -> QueryResult<LeafQueryData<Types>> {
        let mut query = QueryBuilder::default();
        let where_clause = match id {
            LeafId::Number(n) => format!("height = {}", query.bind(n as i64)?),
            LeafId::Hash(h) => format!("hash = {}", query.bind(h.to_string())?),
        };
        let row = query
            .query(&format!(
                "SELECT {LEAF_COLUMNS} FROM leaf WHERE {where_clause}"
            ))
            .fetch_one(self.as_mut())
            .await?;
        let leaf = LeafQueryData::from_row(&row)?;
        Ok(leaf)
    }

    async fn get_block(&mut self, id: BlockId<Types>) -> QueryResult<BlockQueryData<Types>> {
        let mut query = QueryBuilder::default();
        let where_clause = query.header_where_clause(id)?;
        // ORDER BY h.height ASC ensures that if there are duplicate blocks (this can happen when
        // selecting by payload ID, as payloads are not unique), we return the first one.
        let sql = format!(
            "SELECT {BLOCK_COLUMNS}
              FROM header AS h
              JOIN payload AS p ON h.height = p.height
              WHERE {where_clause}
              ORDER BY h.height ASC
              LIMIT 1"
        );
        let row = query.query(&sql).fetch_one(self.as_mut()).await?;
        let block = BlockQueryData::from_row(&row)?;
        Ok(block)
    }

    async fn get_header(&mut self, id: BlockId<Types>) -> QueryResult<Header<Types>> {
        self.load_header(id).await
    }

    async fn get_payload(&mut self, id: BlockId<Types>) -> QueryResult<PayloadQueryData<Types>> {
        let mut query = QueryBuilder::default();
        let where_clause = query.header_where_clause(id)?;
        // ORDER BY h.height ASC ensures that if there are duplicate blocks (this can happen when
        // selecting by payload ID, as payloads are not unique), we return the first one.
        let sql = format!(
            "SELECT {PAYLOAD_COLUMNS}
              FROM header AS h
              JOIN payload AS p ON h.height = p.height
              WHERE {where_clause}
              ORDER BY h.height ASC
              LIMIT 1"
        );
        let row = query.query(&sql).fetch_one(self.as_mut()).await?;
        let payload = PayloadQueryData::from_row(&row)?;
        Ok(payload)
    }

    async fn get_vid_common(
        &mut self,
        id: BlockId<Types>,
    ) -> QueryResult<VidCommonQueryData<Types>> {
        let mut query = QueryBuilder::default();
        let where_clause = query.header_where_clause(id)?;
        // ORDER BY h.height ASC ensures that if there are duplicate blocks (this can happen when
        // selecting by payload ID, as payloads are not unique), we return the first one.
        let sql = format!(
            "SELECT {VID_COMMON_COLUMNS}
              FROM header AS h
              JOIN vid AS v ON h.height = v.height
              WHERE {where_clause}
              ORDER BY h.height ASC
              LIMIT 1"
        );
        let row = query.query(&sql).fetch_one(self.as_mut()).await?;
        let common = VidCommonQueryData::from_row(&row)?;
        Ok(common)
    }

    async fn get_leaf_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<LeafQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        let mut query = QueryBuilder::default();
        let where_clause = query.bounds_to_where_clause(range, "height")?;
        let sql = format!("SELECT {LEAF_COLUMNS} FROM leaf {where_clause} ORDER BY height ASC");
        Ok(query
            .query(&sql)
            .fetch(self.as_mut())
            .map(|res| LeafQueryData::from_row(&res?))
            .map_err(QueryError::from)
            .collect()
            .await)
    }

    async fn get_block_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<BlockQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        let mut query = QueryBuilder::default();
        let where_clause = query.bounds_to_where_clause(range, "h.height")?;
        let sql = format!(
            "SELECT {BLOCK_COLUMNS}
              FROM header AS h
              JOIN payload AS p ON h.height = p.height
              {where_clause}
              ORDER BY h.height ASC"
        );
        Ok(query
            .query(&sql)
            .fetch(self.as_mut())
            .map(|res| BlockQueryData::from_row(&res?))
            .map_err(QueryError::from)
            .collect()
            .await)
    }

    async fn get_payload_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<PayloadQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        let mut query = QueryBuilder::default();
        let where_clause = query.bounds_to_where_clause(range, "h.height")?;
        let sql = format!(
            "SELECT {PAYLOAD_COLUMNS}
              FROM header AS h
              JOIN payload AS p ON h.height = p.height
              {where_clause}
              ORDER BY h.height ASC"
        );
        Ok(query
            .query(&sql)
            .fetch(self.as_mut())
            .map(|res| PayloadQueryData::from_row(&res?))
            .map_err(QueryError::from)
            .collect()
            .await)
    }

    async fn get_vid_common_range<R>(
        &mut self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<VidCommonQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        let mut query = QueryBuilder::default();
        let where_clause = query.bounds_to_where_clause(range, "h.height")?;
        let sql = format!(
            "SELECT {VID_COMMON_COLUMNS}
              FROM header AS h
              JOIN vid AS v ON h.height = v.height
              {where_clause}
              ORDER BY h.height ASC"
        );
        Ok(query
            .query(&sql)
            .fetch(self.as_mut())
            .map(|res| VidCommonQueryData::from_row(&res?))
            .map_err(QueryError::from)
            .collect()
            .await)
    }

    async fn get_transaction(
        &mut self,
        hash: TransactionHash<Types>,
    ) -> QueryResult<TransactionQueryData<Types>> {
        let mut query = QueryBuilder::default();
        let hash_param = query.bind(hash.to_string())?;

        // ORDER BY ASC ensures that if there are duplicate transactions, we return the first
        // one.
        let sql = format!(
            "SELECT {BLOCK_COLUMNS}, t.index AS tx_index
                FROM header AS h
                JOIN payload AS p ON h.height = p.height
                JOIN transaction AS t ON t.block_height = h.height
                WHERE t.hash = {hash_param}
                ORDER BY (t.block_height, t.index) ASC
                LIMIT 1"
        );
        let row = query.query(&sql).fetch_one(self.as_mut()).await?;

        // Extract the block.
        let block = BlockQueryData::from_row(&row)?;

        TransactionQueryData::with_hash(&block, hash).context(ErrorSnafu {
            message: format!(
                "transaction index inconsistent: block {} contains no transaction {hash}",
                block.height()
            ),
        })
    }
}
