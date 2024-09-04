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
    bounds_to_where_clause, header_where_clause, parse_block, parse_leaf, parse_payload,
    parse_vid_common, postgres::types::ToSql, Transaction, BLOCK_COLUMNS, PAYLOAD_COLUMNS,
    VID_COMMON_COLUMNS,
};
use crate::{
    availability::{
        BlockId, BlockQueryData, LeafId, LeafQueryData, PayloadQueryData, QueryableHeader,
        QueryablePayload, TransactionHash, TransactionQueryData, VidCommonQueryData,
    },
    data_source::storage::AvailabilityStorage,
    types::HeightIndexed,
    ErrorSnafu, Header, Payload, QueryResult,
};
use async_trait::async_trait;
use futures::stream::StreamExt;
use hotshot_types::traits::node_implementation::NodeType;
use snafu::OptionExt;
use std::ops::RangeBounds;

#[async_trait]
impl<'a, Types> AvailabilityStorage<Types> for Transaction<'a>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    Header<Types>: QueryableHeader<Types>,
{
    async fn get_leaf(&self, id: LeafId<Types>) -> QueryResult<LeafQueryData<Types>> {
        let (where_clause, param): (&str, Box<dyn ToSql + Send + Sync>) = match id {
            LeafId::Number(n) => ("height = $1", Box::new(n as i64)),
            LeafId::Hash(h) => ("hash = $1", Box::new(h.to_string())),
        };
        let query = format!("SELECT leaf, qc FROM leaf WHERE {where_clause}");
        let row = self.query_one(&query, [param]).await?;
        parse_leaf(row)
    }

    async fn get_block(&self, id: BlockId<Types>) -> QueryResult<BlockQueryData<Types>> {
        let (where_clause, param) = header_where_clause(id);
        // ORDER BY h.height ASC ensures that if there are duplicate blocks (this can happen when
        // selecting by payload ID, as payloads are not unique), we return the first one.
        let query = format!(
            "SELECT {BLOCK_COLUMNS}
              FROM header AS h
              JOIN payload AS p ON h.height = p.height
              WHERE {where_clause}
              ORDER BY h.height ASC
              LIMIT 1"
        );
        let row = self.query_one(&query, [param]).await?;
        parse_block(row)
    }

    async fn get_header(&self, id: BlockId<Types>) -> QueryResult<Header<Types>> {
        self.load_header(id).await
    }

    async fn get_payload(&self, id: BlockId<Types>) -> QueryResult<PayloadQueryData<Types>> {
        let (where_clause, param) = header_where_clause(id);
        // ORDER BY h.height ASC ensures that if there are duplicate blocks (this can happen when
        // selecting by payload ID, as payloads are not unique), we return the first one.
        let query = format!(
            "SELECT {PAYLOAD_COLUMNS}
               FROM header AS h
               JOIN payload AS p ON h.height = p.height
               WHERE {where_clause}
               ORDER BY h.height ASC
               LIMIT 1"
        );
        let row = self.query_one(&query, [param]).await?;
        parse_payload(row)
    }

    async fn get_vid_common(&self, id: BlockId<Types>) -> QueryResult<VidCommonQueryData<Types>> {
        let (where_clause, param) = header_where_clause(id);
        // ORDER BY h.height ASC ensures that if there are duplicate blocks (this can happen when
        // selecting by payload ID, as payloads are not unique), we return the first one.
        let query = format!(
            "SELECT {VID_COMMON_COLUMNS}
               FROM header AS h
               JOIN vid AS v ON h.height = v.height
               WHERE {where_clause}
               ORDER BY h.height ASC
               LIMIT 1"
        );
        let row = self.query_one(&query, [param]).await?;
        parse_vid_common(row)
    }

    async fn get_leaf_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<LeafQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        let (where_clause, params) = bounds_to_where_clause(range, "height");
        let query = format!("SELECT leaf, qc FROM leaf {where_clause} ORDER BY height ASC");
        let rows = self.query(&query, params).await?;

        Ok(rows.map(|res| parse_leaf(res?)).collect().await)
    }

    async fn get_block_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<BlockQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        let (where_clause, params) = bounds_to_where_clause(range, "h.height");
        let query = format!(
            "SELECT {BLOCK_COLUMNS}
              FROM header AS h
              JOIN payload AS p ON h.height = p.height
              {where_clause}
              ORDER BY h.height ASC"
        );
        let rows = self.query(&query, params).await?;

        Ok(rows.map(|res| parse_block(res?)).collect().await)
    }

    async fn get_payload_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<PayloadQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        let (where_clause, params) = bounds_to_where_clause(range, "h.height");
        let query = format!(
            "SELECT {PAYLOAD_COLUMNS}
              FROM header AS h
              JOIN payload AS p ON h.height = p.height
              {where_clause}
              ORDER BY h.height ASC"
        );
        let rows = self.query(&query, params).await?;

        Ok(rows.map(|res| parse_payload(res?)).collect().await)
    }

    async fn get_vid_common_range<R>(
        &self,
        range: R,
    ) -> QueryResult<Vec<QueryResult<VidCommonQueryData<Types>>>>
    where
        R: RangeBounds<usize> + Send,
    {
        let (where_clause, params) = bounds_to_where_clause(range, "h.height");
        let query = format!(
            "SELECT {VID_COMMON_COLUMNS}
              FROM header AS h
              JOIN vid AS v ON h.height = v.height
              {where_clause}
              ORDER BY h.height ASC"
        );
        let rows = self.query(&query, params).await?;

        Ok(rows.map(|res| parse_vid_common(res?)).collect().await)
    }

    async fn get_transaction(
        &self,
        hash: TransactionHash<Types>,
    ) -> QueryResult<TransactionQueryData<Types>> {
        // ORDER BY ASC ensures that if there are duplicate transactions, we return the first
        // one.
        let query = format!(
            "SELECT {BLOCK_COLUMNS}, t.index AS tx_index
                FROM header AS h
                JOIN payload AS p ON h.height = p.height
                JOIN transaction AS t ON t.block_height = h.height
                WHERE t.hash = $1
                ORDER BY (t.block_height, t.index) ASC
                LIMIT 1"
        );
        let row = self.query_one(&query, &[&hash.to_string()]).await?;

        // Extract the block.
        let block = parse_block(row)?;

        TransactionQueryData::with_hash(&block, hash).context(ErrorSnafu {
            message: format!(
                "transaction index inconsistent: block {} contains no transaction {hash}",
                block.height()
            ),
        })
    }
}
