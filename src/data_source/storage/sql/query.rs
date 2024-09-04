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

//! Immutable query functionality of a SQL database.

use super::{
    postgres::{self, types::ToSql, Row},
    Transaction,
};
use crate::{
    availability::{
        BlockId, BlockQueryData, LeafQueryData, PayloadQueryData, QueryablePayload,
        VidCommonQueryData,
    },
    Header, Leaf, Payload,
    QueryError::{self, Missing},
    QueryResult,
};
use hotshot_types::{
    simple_certificate::QuorumCertificate,
    traits::{
        block_contents::{BlockHeader, BlockPayload},
        node_implementation::NodeType,
    },
};
use std::ops::{Bound, RangeBounds};

pub(super) mod availability;
pub(super) mod explorer;
pub(super) mod node;
pub(super) mod state;

impl<'a> Transaction<'a> {
    /// Load a header from storage.
    ///
    /// This function is similar to `AvailabilityStorage::get_header`, but
    /// * does not require the `QueryablePayload<Types>` bound that that trait impl does
    /// * makes it easier to specify types since the type parameter is on the function and not on a
    ///   trait impl
    /// * allows type conversions for the `id` parameter
    ///
    /// This more ergonomic interface is useful as loading headers is important for many SQL storage
    /// functions, not just the `AvailabilityStorage` interface.
    async fn load_header<Types: NodeType>(
        &self,
        id: impl Into<BlockId<Types>> + Send,
    ) -> QueryResult<Header<Types>> {
        let (where_clause, param) = header_where_clause(id.into());
        // ORDER BY h.height ASC ensures that if there are duplicate blocks (this can happen when
        // selecting by payload ID, as payloads are not unique), we return the first one.
        let query = format!(
            "SELECT {HEADER_COLUMNS}
               FROM header AS h
              WHERE {where_clause}
              ORDER BY h.height ASC
              LIMIT 1"
        );
        let row = self.query_one(&query, [param]).await?;
        parse_header::<Types>(row)
    }
}

fn parse_leaf<Types>(row: Row) -> QueryResult<LeafQueryData<Types>>
where
    Types: NodeType,
{
    let leaf = row.try_get("leaf").map_err(|err| QueryError::Error {
        message: format!("error extracting leaf from query results: {err}"),
    })?;
    let leaf: Leaf<Types> = serde_json::from_value(leaf).map_err(|err| QueryError::Error {
        message: format!("malformed leaf: {err}"),
    })?;

    let qc = row.try_get("qc").map_err(|err| QueryError::Error {
        message: format!("error extracting QC from query results: {err}"),
    })?;
    let qc: QuorumCertificate<Types> =
        serde_json::from_value(qc).map_err(|err| QueryError::Error {
            message: format!("malformed QC: {err}"),
        })?;

    Ok(LeafQueryData { leaf, qc })
}

fn header_where_clause<Types: NodeType>(
    id: BlockId<Types>,
) -> (&'static str, Box<dyn ToSql + Send + Sync>) {
    match id {
        BlockId::Number(n) => ("h.height = $1", Box::new(n as i64)),
        BlockId::Hash(h) => ("h.hash = $1", Box::new(h.to_string())),
        BlockId::PayloadHash(h) => ("h.payload_hash = $1", Box::new(h.to_string())),
    }
}

const BLOCK_COLUMNS: &str =
    "h.hash AS hash, h.data AS header_data, p.size AS payload_size, p.data AS payload_data";

fn parse_block<Types>(row: Row) -> QueryResult<BlockQueryData<Types>>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
{
    // First, check if we have the payload for this block yet.
    let size: Option<i32> = row
        .try_get("payload_size")
        .map_err(|err| QueryError::Error {
            message: format!("error extracting payload size from query results: {err}"),
        })?;
    let payload_data: Option<Vec<u8>> =
        row.try_get("payload_data")
            .map_err(|err| QueryError::Error {
                message: format!("error extracting payload data from query results: {err}"),
            })?;
    let (size, payload_data) = size.zip(payload_data).ok_or(Missing)?;
    let size = size as u64;

    // Reconstruct the full header.
    let header_data = row
        .try_get("header_data")
        .map_err(|err| QueryError::Error {
            message: format!("error extracting header data from query results: {err}"),
        })?;
    let header: Header<Types> =
        serde_json::from_value(header_data).map_err(|err| QueryError::Error {
            message: format!("malformed header: {err}"),
        })?;

    // Reconstruct the full block payload.
    let payload = Payload::<Types>::from_bytes(&payload_data, header.metadata());

    // Reconstruct the query data by adding metadata.
    let hash: String = row.try_get("hash").map_err(|err| QueryError::Error {
        message: format!("error extracting block hash from query results: {err}"),
    })?;
    let hash = hash.parse().map_err(|err| QueryError::Error {
        message: format!("malformed block hash: {err}"),
    })?;

    Ok(BlockQueryData {
        num_transactions: payload.len(header.metadata()) as u64,
        header,
        payload,
        size,
        hash,
    })
}

const PAYLOAD_COLUMNS: &str = BLOCK_COLUMNS;

fn parse_payload<Types>(row: Row) -> QueryResult<PayloadQueryData<Types>>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
{
    parse_block(row).map(PayloadQueryData::from)
}

const VID_COMMON_COLUMNS: &str = "h.height AS height, h.hash AS block_hash, h.payload_hash AS payload_hash, v.common AS common_data";

fn parse_vid_common<Types>(row: Row) -> QueryResult<VidCommonQueryData<Types>>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
{
    let height = row
        .try_get::<_, i64>("height")
        .map_err(|err| QueryError::Error {
            message: format!("error extracting height from query results: {err}"),
        })? as u64;
    let block_hash: String = row.try_get("block_hash").map_err(|err| QueryError::Error {
        message: format!("error extracting block_hash from query results: {err}"),
    })?;
    let block_hash = block_hash.parse().map_err(|err| QueryError::Error {
        message: format!("malformed block hash: {err}"),
    })?;
    let payload_hash: String = row
        .try_get("payload_hash")
        .map_err(|err| QueryError::Error {
            message: format!("error extracting payload_hash from query results: {err}"),
        })?;
    let payload_hash = payload_hash.parse().map_err(|err| QueryError::Error {
        message: format!("malformed payload hash: {err}"),
    })?;
    let common_data: Vec<u8> = row
        .try_get("common_data")
        .map_err(|err| QueryError::Error {
            message: format!("error extracting common_data from query results: {err}"),
        })?;
    let common = bincode::deserialize(&common_data).map_err(|err| QueryError::Error {
        message: format!("malformed VID common data: {err}"),
    })?;
    Ok(VidCommonQueryData {
        height,
        block_hash,
        payload_hash,
        common,
    })
}

const HEADER_COLUMNS: &str = "h.data AS data";

fn parse_header<Types>(row: Row) -> QueryResult<Header<Types>>
where
    Types: NodeType,
{
    // Reconstruct the full header.
    let data = row.try_get("data").map_err(|err| QueryError::Error {
        message: format!("error extracting header data from query results: {err}"),
    })?;
    serde_json::from_value(data).map_err(|err| QueryError::Error {
        message: format!("malformed header: {err}"),
    })
}

/// Convert range bounds to a SQL where clause constraining a given column.
///
/// Returns the where clause as a string and a list of query parameters. We assume that there are no
/// other parameters in the query; that is, parameters in the where clause will start from $1.
fn bounds_to_where_clause<R>(range: R, column: &str) -> (String, Vec<i64>)
where
    R: RangeBounds<usize>,
{
    let mut bounds = vec![];
    let mut params = vec![];

    match range.start_bound() {
        Bound::Included(n) => {
            params.push(*n as i64);
            bounds.push(format!("{column} >= ${}", params.len()));
        }
        Bound::Excluded(n) => {
            params.push(*n as i64);
            bounds.push(format!("{column} > ${}", params.len()));
        }
        Bound::Unbounded => {}
    }
    match range.end_bound() {
        Bound::Included(n) => {
            params.push(*n as i64);
            bounds.push(format!("{column} <= ${}", params.len()));
        }
        Bound::Excluded(n) => {
            params.push(*n as i64);
            bounds.push(format!("{column} < ${}", params.len()));
        }
        Bound::Unbounded => {}
    }

    let mut where_clause = bounds.join(" AND ");
    if !where_clause.is_empty() {
        where_clause = format!(" WHERE {where_clause}");
    }

    (where_clause, params)
}

pub(super) fn sql_param<T: ToSql + Sync>(param: &T) -> &(dyn ToSql + Sync) {
    param
}
