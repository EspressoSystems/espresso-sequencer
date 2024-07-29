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

//! Node storage implementation for a database query engine.

use super::{header_where_clause, parse_header, Query, HEADER_COLUMNS};
use crate::{
    node::{BlockId, NodeDataSource, SyncStatus, TimeWindowQueryData, WindowStart},
    Header, MissingSnafu, NotFoundSnafu, QueryError, QueryResult, VidShare,
};
use async_trait::async_trait;
use futures::stream::{StreamExt, TryStreamExt};
use hotshot_types::traits::{block_contents::BlockHeader, node_implementation::NodeType};
use snafu::OptionExt;

#[async_trait]
impl<Types, T> NodeDataSource<Types> for T
where
    Types: NodeType,
    T: Query + Sync,
{
    async fn block_height(&self) -> QueryResult<usize> {
        let query = "SELECT max(height) FROM header";
        let row = self.query_one_static(query).await?;
        let height: Option<i64> = row.get(0);
        match height {
            Some(height) => {
                // The height of the block is the number of blocks below it, so the total number of
                // blocks is one more than the height of the highest block.
                Ok(height as usize + 1)
            }
            None => {
                // If there are no blocks yet, the height is 0.
                Ok(0)
            }
        }
    }

    async fn count_transactions(&self) -> QueryResult<usize> {
        let row = self
            .query_one_static("SELECT count(*) FROM transaction")
            .await?;
        let count: i64 = row.get(0);
        Ok(count as usize)
    }

    async fn payload_size(&self) -> QueryResult<usize> {
        let row = self
            .query_one_static("SELECT sum(size) FROM payload")
            .await?;
        let sum: Option<i64> = row.get(0);
        Ok(sum.unwrap_or(0) as usize)
    }

    async fn vid_share<ID>(&self, id: ID) -> QueryResult<VidShare>
    where
        ID: Into<BlockId<Types>> + Send + Sync,
    {
        let (where_clause, param) = header_where_clause(id.into());
        // ORDER BY h.height ASC ensures that if there are duplicate blocks (this can happen when
        // selecting by payload ID, as payloads are not unique), we return the first one.
        let query = format!(
            "SELECT v.share AS share FROM vid AS v
               JOIN header AS h ON v.height = h.height
              WHERE {where_clause}
              ORDER BY h.height ASC
              LIMIT 1"
        );
        let row = self.query_one(&query, [param]).await?;
        let share_data: Option<Vec<u8>> =
            row.try_get("share").map_err(|err| QueryError::Error {
                message: format!("error extracting share data from query results: {err}"),
            })?;
        let share_data = share_data.context(MissingSnafu)?;
        bincode::deserialize(&share_data).map_err(|err| QueryError::Error {
            message: format!("malformed VID share: {err}"),
        })
    }

    async fn sync_status(&self) -> QueryResult<SyncStatus> {
        // A leaf can only be missing if there is no row for it in the database (all its columns are
        // non-nullable). A block can be missing if its corresponding leaf is missing or if the
        // block's `data` field is `NULL`. We can find the number of missing leaves and blocks by
        // getting the number of fully missing leaf rows and the number of present but null-payload
        // block rows.
        //
        // Note that it should not be possible for a block's row to be missing (as opposed to
        // present but having a `NULL` payload) if the corresponding leaf is present. The schema
        // enforces this, since the payload table `REFERENCES` the corresponding leaf row. Thus,
        // missing block rows should be a subset of missing leaf rows and do not need to be counted
        // separately. This is very important: if we had to count the number of block rows that were
        // missing whose corresponding leaf row was present, this would require an expensive join
        // and table traversal.
        //
        // We can get the number of missing leaf rows very efficiently, by subtracting the total
        // number of leaf rows from the block height (since the block height by definition is the
        // height of the highest leaf we do have). We can also get the number of null payloads
        // directly using an `IS NULL` filter.
        //
        // For VID, common data can only be missing if the entire row is missing. Shares can be
        // missing in that case _or_ if the row is present but share data is NULL. Thus, we also
        // need to select the total number of VID rows and the number of present VID rows with a
        // NULL share.
        let query = "SELECT l.max_height, l.total_leaves, p.null_payloads, v.total_vid, vn.null_vid, pruned_height FROM
                (SELECT max(leaf.height) AS max_height, count(*) AS total_leaves FROM leaf) AS l,
                (SELECT count(*) AS null_payloads FROM payload WHERE data IS NULL) AS p,
                (SELECT count(*) AS total_vid FROM vid) AS v,
                (SELECT count(*) AS null_vid FROM vid WHERE share IS NULL) AS vn,
                coalesce((SELECT last_height FROM pruned_height ORDER BY id DESC LIMIT 1)) as pruned_height
            ";
        let row = self.query_opt_static(query).await?.context(NotFoundSnafu)?;

        let block_height = match row.get::<_, Option<i64>>("max_height") {
            Some(height) => {
                // The height of the block is the number of blocks below it, so the total number of
                // blocks is one more than the height of the highest block.
                height as usize + 1
            }
            None => {
                // If there are no blocks yet, the height is 0.
                0
            }
        };
        let total_leaves = row.get::<_, i64>("total_leaves") as usize;
        let null_payloads = row.get::<_, i64>("null_payloads") as usize;
        let total_vid = row.get::<_, i64>("total_vid") as usize;
        let null_vid = row.get::<_, i64>("null_vid") as usize;
        let pruned_height = row
            .get::<_, Option<i64>>("pruned_height")
            .map(|h| h as usize);

        let missing_leaves = block_height.saturating_sub(total_leaves);
        let missing_blocks = missing_leaves + null_payloads;
        let missing_vid_common = block_height.saturating_sub(total_vid);
        let missing_vid_shares = missing_vid_common + null_vid;

        Ok(SyncStatus {
            missing_leaves,
            missing_blocks,
            missing_vid_common,
            missing_vid_shares,
            pruned_height,
        })
    }

    async fn get_header_window(
        &self,
        start: impl Into<WindowStart<Types>> + Send + Sync,
        end: u64,
    ) -> QueryResult<TimeWindowQueryData<Header<Types>>> {
        // Find the specific block that starts the requested window.
        let first_block = match start.into() {
            WindowStart::Time(t) => {
                // If the request is not to start from a specific block, but from a timestamp, we
                // use a different method to find the window, as detecting whether we have
                // sufficient data to answer the query is not as simple as just trying `load_header`
                // for a specific block ID.
                return time_window::<Types, T>(self, t, end).await;
            }
            WindowStart::Height(h) => h,
            WindowStart::Hash(h) => self.load_header::<Types>(h).await?.block_number(),
        };

        // Find all blocks starting from `first_block` with timestamps less than `end`. Block
        // timestamps are monotonically increasing, so this query is guaranteed to return a
        // contiguous range of blocks ordered by increasing height.
        let query = format!(
            "SELECT {HEADER_COLUMNS}
               FROM header AS h
              WHERE h.height >= $1 AND h.timestamp < $2
              ORDER BY h.height"
        );
        let rows = self
            .query(&query, [&(first_block as i64), &(end as i64)])
            .await?;
        let window = rows
            .map(|row| {
                parse_header::<Types>(row.map_err(|err| QueryError::Error {
                    message: err.to_string(),
                })?)
            })
            .try_collect()
            .await?;

        // Find the block just before the window.
        let prev = if first_block > 0 {
            Some(self.load_header::<Types>(first_block as usize - 1).await?)
        } else {
            None
        };

        // Find the block just after the window. We order by timestamp _then_ height, because the
        // timestamp order allows the query planner to use the index on timestamp to also
        // efficiently solve the WHERE clause, but this process may turn up multiple results, due to
        // the 1-second resolution of block timestamps. The final sort by height guarantees us a
        // unique, deterministic result (the first block with a given timestamp). This sort may not
        // be able to use an index, but it shouldn't be too expensive, since there will never be
        // more than a handful of blocks with the same timestamp.
        let query = format!(
            "SELECT {HEADER_COLUMNS}
               FROM header AS h
              WHERE h.timestamp >= $1
              ORDER BY h.timestamp, h.height
              LIMIT 1"
        );
        let next = self
            .query_opt(&query, [&(end as i64)])
            .await?
            .map(parse_header::<Types>)
            .transpose()?;

        Ok(TimeWindowQueryData { window, prev, next })
    }
}

async fn time_window<Types: NodeType, T: Query + Sync>(
    db: &T,
    start: u64,
    end: u64,
) -> QueryResult<TimeWindowQueryData<Header<Types>>> {
    // Find all blocks whose timestamps fall within the window [start, end). Block timestamps are
    // monotonically increasing, so this query is guaranteed to return a contiguous range of blocks
    // ordered by increasing height.
    //
    // We order by timestamp _then_ height, because the timestamp order allows the query planner to
    // use the index on timestamp to also efficiently solve the WHERE clause, but this process may
    // turn up multiple results, due to the 1-second resolution of block timestamps. The final sort
    // by height guarantees us a unique, deterministic result (the first block with a given
    // timestamp). This sort may not be able to use an index, but it shouldn't be too expensive,
    // since there will never be more than a handful of blocks with the same timestamp.
    let query = format!(
        "SELECT {HEADER_COLUMNS}
               FROM header AS h
              WHERE h.timestamp >= $1 AND h.timestamp < $2
              ORDER BY h.timestamp, h.height"
    );
    let rows = db.query(&query, [&(start as i64), &(end as i64)]).await?;
    let window: Vec<_> = rows
        .map(|row| {
            parse_header::<Types>(row.map_err(|err| QueryError::Error {
                message: err.to_string(),
            })?)
        })
        .try_collect()
        .await?;

    // Find the block just after the window.
    let query = format!(
        "SELECT {HEADER_COLUMNS}
               FROM header AS h
              WHERE h.timestamp >= $1
              ORDER BY h.timestamp, h.height
              LIMIT 1"
    );
    let next = db
        .query_opt(&query, [&(end as i64)])
        .await?
        .map(parse_header::<Types>)
        .transpose()?;

    // If the `next` block exists, _or_ if any block in the window exists, we know we have enough
    // information to definitively say at least where the window starts (we may or may not have
    // where it ends, depending on how many blocks have thus far been produced). However, if we have
    // neither a block in the window nor a block after it, we cannot say whether the next block
    // produced will have a timestamp before or after the window start. In this case, we don't know
    // what the `prev` field of the response should be, so we return an error: the caller must try
    // again after more blocks have been produced.
    if window.is_empty() && next.is_none() {
        return Err(QueryError::NotFound);
    }

    // Find the block just before the window.
    let query = format!(
        "SELECT {HEADER_COLUMNS}
               FROM header AS h
              WHERE h.timestamp < $1
              ORDER BY h.timestamp DESC, h.height DESC
              LIMIT 1"
    );
    let prev = db
        .query_opt(&query, [&(start as i64)])
        .await?
        .map(parse_header::<Types>)
        .transpose()?;

    Ok(TimeWindowQueryData { window, prev, next })
}
