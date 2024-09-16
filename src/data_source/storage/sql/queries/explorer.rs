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

//! Explorer storage implementation for a database query engine.

use super::{
    super::transaction::{query, Transaction, TransactionMode},
    Database, Db, DecodeError, QueryBuilder, BLOCK_COLUMNS,
};
use crate::{
    availability::{BlockQueryData, QueryableHeader, QueryablePayload, TransactionIndex},
    data_source::storage::ExplorerStorage,
    explorer::{
        self, errors::NotFound, query_data::TransactionDetailResponse, traits::ExplorerHeader,
        BalanceAmount, BlockDetail, BlockIdentifier, BlockRange, BlockSummary, ExplorerHistograms,
        ExplorerSummary, GenesisOverview, GetBlockDetailError, GetBlockSummariesError,
        GetBlockSummariesRequest, GetExplorerSummaryError, GetSearchResultsError,
        GetTransactionDetailError, GetTransactionSummariesError, GetTransactionSummariesRequest,
        MonetaryValue, SearchResult, TransactionIdentifier, TransactionRange, TransactionSummary,
        TransactionSummaryFilter,
    },
    Header, Payload, QueryError, QueryResult,
};
use async_trait::async_trait;
use committable::Committable;
use futures::stream::{self, StreamExt, TryStreamExt};
use hotshot_types::traits::node_implementation::NodeType;
use itertools::Itertools;
use sqlx::{types::Json, FromRow, Row};
use std::num::NonZeroUsize;

impl From<sqlx::Error> for GetExplorerSummaryError {
    fn from(err: sqlx::Error) -> Self {
        Self::from(QueryError::from(err))
    }
}

impl From<sqlx::Error> for GetTransactionDetailError {
    fn from(err: sqlx::Error) -> Self {
        Self::from(QueryError::from(err))
    }
}

impl From<sqlx::Error> for GetTransactionSummariesError {
    fn from(err: sqlx::Error) -> Self {
        Self::from(QueryError::from(err))
    }
}

impl From<sqlx::Error> for GetBlockDetailError {
    fn from(err: sqlx::Error) -> Self {
        Self::from(QueryError::from(err))
    }
}

impl From<sqlx::Error> for GetBlockSummariesError {
    fn from(err: sqlx::Error) -> Self {
        Self::from(QueryError::from(err))
    }
}

impl From<sqlx::Error> for GetSearchResultsError {
    fn from(err: sqlx::Error) -> Self {
        Self::from(QueryError::from(err))
    }
}

impl<'r, Types> FromRow<'r, <Db as Database>::Row> for BlockSummary<Types>
where
    Types: NodeType,
    Header<Types>: QueryableHeader<Types> + ExplorerHeader<Types>,
    Payload<Types>: QueryablePayload<Types>,
{
    fn from_row(row: &'r <Db as Database>::Row) -> sqlx::Result<Self> {
        BlockQueryData::<Types>::from_row(row)?
            .try_into()
            .decode_error("malformed block summary")
    }
}

impl<'r, Types> FromRow<'r, <Db as Database>::Row> for BlockDetail<Types>
where
    Types: NodeType,
    Header<Types>: QueryableHeader<Types> + ExplorerHeader<Types>,
    Payload<Types>: QueryablePayload<Types>,
    BalanceAmount<Types>: Into<MonetaryValue>,
{
    fn from_row(row: &'r <Db as Database>::Row) -> sqlx::Result<Self> {
        BlockQueryData::<Types>::from_row(row)?
            .try_into()
            .decode_error("malformed block detail")
    }
}

#[async_trait]
impl<Mode, Types> ExplorerStorage<Types> for Transaction<Mode>
where
    Mode: TransactionMode,
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    Header<Types>: QueryableHeader<Types> + ExplorerHeader<Types>,
    crate::Transaction<Types>: explorer::traits::ExplorerTransaction,
    BalanceAmount<Types>: Into<explorer::monetary_value::MonetaryValue>,
{
    async fn get_block_summaries(
        &mut self,
        request: GetBlockSummariesRequest<Types>,
    ) -> Result<Vec<BlockSummary<Types>>, GetBlockSummariesError> {
        let request = &request.0;

        let mut query = QueryBuilder::default();
        let sql = match request.target {
            BlockIdentifier::Latest => format!(
                "SELECT {BLOCK_COLUMNS}
                    FROM header AS h
                    JOIN payload AS p ON h.height = p.height
                    ORDER BY h.height DESC 
                    LIMIT {}",
                query.bind(request.num_blocks.get() as i64)?,
            ),
            BlockIdentifier::Height(height) => format!(
                "SELECT {BLOCK_COLUMNS}
                    FROM header AS h
                    JOIN payload AS p ON h.height = p.height
                    WHERE h.height <= {}
                    ORDER BY h.height DESC 
                    LIMIT {}",
                query.bind(height as i64)?,
                query.bind(request.num_blocks.get() as i64)?,
            ),
            BlockIdentifier::Hash(hash) => {
                // We want to match the blocks starting with the given hash, and working backwards
                // until we have returned up to the number of requested blocks.  The hash for a
                // block should be unique, so we should just need to start with identifying the
                // block height with the given hash, and return all blocks with a height less than
                // or equal to that height, up to the number of requested blocks.
                format!(
                    "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        WHERE h.height <= (SELECT h1.height FROM header AS h1 WHERE h1.hash = {})
                        ORDER BY h.height DESC 
                        LIMIT {}",
                    query.bind(hash.to_string())?,
                    query.bind(request.num_blocks.get() as i64)?,
                )
            }
        };

        let row_stream = query.query(&sql).fetch(self.as_mut());
        let result = row_stream.map(|row| BlockSummary::from_row(&row?));

        Ok(result.try_collect().await?)
    }

    async fn get_block_detail(
        &mut self,
        request: BlockIdentifier<Types>,
    ) -> Result<BlockDetail<Types>, GetBlockDetailError> {
        let mut query = QueryBuilder::default();
        let sql = match request {
            BlockIdentifier::Latest => format!(
                "SELECT {BLOCK_COLUMNS}
                    FROM header AS h
                    JOIN payload AS p ON h.height = p.height
                    ORDER BY h.height DESC 
                    LIMIT 1"
            ),
            BlockIdentifier::Height(height) => format!(
                "SELECT {BLOCK_COLUMNS}
                    FROM header AS h
                    JOIN payload AS p ON h.height = p.height
                    WHERE h.height = {}
                    ORDER BY h.height DESC 
                    LIMIT 1",
                query.bind(height as i64)?,
            ),
            BlockIdentifier::Hash(hash) => format!(
                "SELECT {BLOCK_COLUMNS}
                    FROM header AS h
                    JOIN payload AS p ON h.height = p.height
                    WHERE h.hash = {}
                    ORDER BY h.height DESC 
                    LIMIT 1",
                query.bind(hash.to_string())?,
            ),
        };

        let query_result = query.query(&sql).fetch_one(self.as_mut()).await?;
        let block = BlockDetail::from_row(&query_result)?;

        Ok(block)
    }

    async fn get_transaction_summaries(
        &mut self,
        request: GetTransactionSummariesRequest<Types>,
    ) -> Result<Vec<TransactionSummary<Types>>, GetTransactionSummariesError> {
        let range = &request.range;
        let target = &range.target;
        let filter = &request.filter;

        // We need to figure out the transaction target we are going to start
        // returned results based on.
        let transaction_target_query = match target {
            TransactionIdentifier::Latest => query(
                "SELECT t.block_height AS height, t.index AS index FROM transaction AS t ORDER BY (t.block_height, t.index) DESC LIMIT 1",
            ),
            TransactionIdentifier::HeightAndOffset(height, _) => query(
                "SELECT t.block_height AS height, t.index AS index FROM transaction AS t WHERE t.block_height = $1 ORDER BY (t.block_height, t.index) DESC LIMIT 1",
            )
            .bind(*height as i64),
            TransactionIdentifier::Hash(hash) => query(
                "SELECT t.block_height AS height, t.index AS index FROM transaction AS t WHERE t.hash = $1 ORDER BY (t.block_height, t.index) DESC LIMIT 1",
            )
            .bind(hash.to_string()),
        };
        let Some(transaction_target) = transaction_target_query
            .fetch_optional(self.as_mut())
            .await?
        else {
            // If nothing is found, then we want to return an empty summary list as it means there
            // is either no transaction, or the targeting criteria fails to identify any transaction
            return Ok(vec![]);
        };

        let block_height = transaction_target.get::<i64, _>("height") as usize;
        let transaction_index = transaction_target.get::<Json<TransactionIndex<Types>>, _>("index");
        let offset = if let TransactionIdentifier::HeightAndOffset(_, offset) = target {
            *offset
        } else {
            0
        };

        // Our block_stream is more-or-less always the same, the only difference
        // is a an additional filter on the identified transactions being found
        // In general, we use our `transaction_target` to identify the starting
        // `block_height` and `transaction_index`, and we grab up to `limit`
        // transactions from that point.  We then grab only the blocks for those
        // identified transactions, as only those blocks are needed to pull all
        // of the relevant transactions.
        let mut query = QueryBuilder::default();
        let sql = match filter {
            TransactionSummaryFilter::RollUp(_) => return Ok(vec![]),

            TransactionSummaryFilter::None => format!(
                "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        WHERE h.height IN (
                            SELECT t.block_height
                                FROM transaction AS t
                                WHERE (t.block_height, t.index) <= ({}, {})
                                ORDER BY (t.block_height, t.index) DESC
                                LIMIT {}
                        )
                        ORDER BY h.height DESC",
                query.bind(block_height as i64)?,
                query.bind(transaction_index)?,
                query.bind((range.num_transactions.get() + offset) as i64)?,
            ),

            TransactionSummaryFilter::Block(block) => format!(
                "SELECT {BLOCK_COLUMNS}
                    FROM header AS h
                    JOIN payload AS p ON h.height = p.height
                    WHERE  h.height = {}
                    ORDER BY h.height DESC",
                query.bind(*block as i64)?,
            ),
        };
        let block_stream = query
            .query(&sql)
            .fetch(self.as_mut())
            .map(|row| BlockQueryData::from_row(&row?));

        let transaction_summary_stream = block_stream.flat_map(|row| match row {
            Ok(block) => stream::iter(
                block
                    .enumerate()
                    .enumerate()
                    .map(|(index, (_, txn))| {
                        TransactionSummary::try_from((&block, index, txn)).map_err(|err| {
                            QueryError::Error {
                                message: err.to_string(),
                            }
                        })
                    })
                    .collect::<Vec<QueryResult<TransactionSummary<Types>>>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<QueryResult<TransactionSummary<Types>>>>(),
            ),
            Err(err) => stream::iter(vec![Err(err.into())]),
        });

        let transaction_summary_vec = transaction_summary_stream
            .try_collect::<Vec<TransactionSummary<Types>>>()
            .await?;

        Ok(transaction_summary_vec
            .into_iter()
            .skip(offset)
            .skip_while(|txn| {
                if let TransactionIdentifier::Hash(hash) = target {
                    txn.hash != *hash
                } else {
                    false
                }
            })
            .collect::<Vec<TransactionSummary<Types>>>())
    }

    async fn get_transaction_detail(
        &mut self,
        request: TransactionIdentifier<Types>,
    ) -> Result<TransactionDetailResponse<Types>, GetTransactionDetailError> {
        let target = request;

        let mut query = QueryBuilder::default();
        let sql = match target {
            TransactionIdentifier::Latest => format!(
                "SELECT {BLOCK_COLUMNS}
                    FROM header AS h
                    JOIN payload AS p ON h.height = p.height
                    WHERE h.height = (
                        SELECT MAX(t1.block_height)
                            FROM transaction AS t1
                    )
                    ORDER BY h.height DESC"
            ),
            TransactionIdentifier::HeightAndOffset(height, offset) => format!(
                "SELECT {BLOCK_COLUMNS}
                    FROM header AS h
                    JOIN payload AS p ON h.height = p.height
                    WHERE h.height = (
                        SELECT t1.block_height
                            FROM transaction AS t1
                            WHERE t1.block_height = {}
                            ORDER BY (t1.block_height, t1.index) DESC
                            OFFSET {}
                            LIMIT 1
                    )
                    ORDER BY h.height DESC",
                query.bind(height as i64)?,
                query.bind(offset as i64)?,
            ),
            TransactionIdentifier::Hash(hash) => format!(
                "SELECT {BLOCK_COLUMNS}
                    FROM header AS h
                    JOIN payload AS p ON h.height = p.height
                    WHERE h.height = (
                        SELECT t1.block_height
                            FROM transaction AS t1
                            WHERE t1.hash = {}
                            ORDER BY (t1.block_height, t1.index) DESC
                            LIMIT 1
                    )
                    ORDER BY h.height DESC",
                query.bind(hash.to_string())?,
            ),
        };

        let query_row = query.query(&sql).fetch_one(self.as_mut()).await?;
        let block = BlockQueryData::<Types>::from_row(&query_row)?;

        let txns = block.enumerate().map(|(_, txn)| txn).collect::<Vec<_>>();

        let (offset, txn) = match target {
            TransactionIdentifier::Latest => txns.into_iter().enumerate().last().ok_or(
                GetTransactionDetailError::TransactionNotFound(NotFound {
                    key: "Latest".to_string(),
                }),
            ),
            TransactionIdentifier::HeightAndOffset(height, offset) => {
                txns.into_iter().enumerate().rev().nth(offset).ok_or(
                    GetTransactionDetailError::TransactionNotFound(NotFound {
                        key: format!("at {height} and {offset}"),
                    }),
                )
            }
            TransactionIdentifier::Hash(hash) => txns
                .into_iter()
                .enumerate()
                .find(|(_, txn)| txn.commit() == hash)
                .ok_or(GetTransactionDetailError::TransactionNotFound(NotFound {
                    key: format!("hash {hash}"),
                })),
        }?;

        Ok(TransactionDetailResponse::try_from((&block, offset, txn))?)
    }

    async fn get_explorer_summary(
        &mut self,
    ) -> Result<ExplorerSummary<Types>, GetExplorerSummaryError> {
        let histograms = {
            let historgram_query_result = query(
                "SELECT
                    h.height AS height,
                    h.timestamp AS timestamp,
                    h.timestamp - lead(timestamp) OVER (ORDER BY h.height DESC) AS time,
                    p.size AS size,
                    (SELECT COUNT(*) AS transactions FROM transaction AS t WHERE t.block_height = h.height) as transactions
                FROM header AS h
                JOIN payload AS p ON
                    p.height = h.height
                WHERE
                    h.height IN (SELECT height FROM header ORDER BY height DESC LIMIT 50)
                ORDER BY h.height ASC 
                ",
            ).fetch(self.as_mut());

            let histograms: Result<ExplorerHistograms, sqlx::Error> = historgram_query_result
                .map(|row_stream| {
                    row_stream.map(|row| {
                        let height: i64 = row.try_get("height")?;
                        let timestamp: i64 = row.try_get("timestamp")?;
                        let time: Option<i64> = row.try_get("time")?;
                        let size: Option<i32> = row.try_get("size")?;
                        let num_transactions: i64 = row.try_get("transactions")?;

                        Ok((height, timestamp, time, size, num_transactions))
                    })
                })
                .try_fold(
                    ExplorerHistograms {
                        block_time: Vec::with_capacity(50),
                        block_size: Vec::with_capacity(50),
                        block_transactions: Vec::with_capacity(50),
                        block_heights: Vec::with_capacity(50),
                    },
                    |mut histograms: ExplorerHistograms,
                     row: sqlx::Result<(i64, i64, Option<i64>, Option<i32>, i64)>| async {
                        let (height, _timestamp, time, size, num_transactions) = row?;
                        histograms.block_time.push(time.map(|i| i as u64));
                        histograms.block_size.push(size.map(|i| i as u64));
                        histograms.block_transactions.push(num_transactions as u64);
                        histograms.block_heights.push(height as u64);
                        Ok(histograms)
                    },
                )
                .await;

            histograms?
        };

        let genesis_overview = {
            let row = query(
                "SELECT
                    (SELECT MAX(height) + 1 FROM header) AS blocks,
                    (SELECT COUNT(*) FROM transaction) AS transactions",
            )
            .fetch_one(self.as_mut())
            .await?;

            let blocks: i64 = row.try_get("blocks")?;
            let transactions: i64 = row.try_get("transactions")?;

            let blocks: u64 = blocks
                .try_into()
                .decode_error("failed to convert blocks to u64 {e}")?;
            let transactions: u64 = transactions
                .try_into()
                .decode_error("failed to convert transactions to u64 {e}")?;

            GenesisOverview {
                rollups: 0,
                transactions,
                blocks,
            }
        };

        let latest_block: BlockDetail<Types> =
            self.get_block_detail(BlockIdentifier::Latest).await?;
        let latest_blocks: Vec<BlockSummary<Types>> = self
            .get_block_summaries(GetBlockSummariesRequest(BlockRange {
                target: BlockIdentifier::Latest,
                num_blocks: NonZeroUsize::new(10).unwrap(),
            }))
            .await?;
        let latest_transactions: Vec<TransactionSummary<Types>> = self
            .get_transaction_summaries(GetTransactionSummariesRequest {
                range: TransactionRange {
                    target: TransactionIdentifier::Latest,
                    num_transactions: NonZeroUsize::new(10).unwrap(),
                },
                filter: TransactionSummaryFilter::None,
            })
            .await?;

        Ok(ExplorerSummary {
            genesis_overview,
            latest_block,
            latest_transactions,
            latest_blocks,
            histograms,
        })
    }

    async fn get_search_results(
        &mut self,
        search_query: String,
    ) -> Result<SearchResult<Types>, GetSearchResultsError> {
        let block_query = format!(
            "SELECT {BLOCK_COLUMNS}
                FROM header AS h
                JOIN payload AS p ON h.height = p.height
                WHERE h.hash LIKE $1 || '%'
                ORDER BY h.height DESC
                LIMIT 5"
        );
        let block_query_rows = query(block_query.as_str())
            .bind(&search_query)
            .fetch(self.as_mut());
        let block_query_result: Vec<BlockSummary<Types>> = block_query_rows
            .map(|row| BlockSummary::from_row(&row?))
            .try_collect()
            .await?;

        let transactions_query = format!(
            "SELECT {BLOCK_COLUMNS}
                FROM header AS h
                JOIN payload AS p ON h.height = p.height
                JOIN transaction AS t ON h.height = t.block_height
                WHERE t.hash LIKE $1 || '%'
                ORDER BY h.height DESC
                LIMIT 5"
        );
        let transactions_query_rows = query(transactions_query.as_str())
            .bind(&search_query)
            .fetch(self.as_mut());
        let transactions_query_result: Vec<TransactionSummary<Types>> = transactions_query_rows
            .map(|row| -> Result<Vec<TransactionSummary<Types>>, QueryError>{
                let block = BlockQueryData::<Types>::from_row(&row?)?;
                let transactions = block
                    .enumerate()
                    .enumerate()
                    .filter(|(_, (_, txn))| txn.commit().to_string().starts_with(&search_query))
                    .map(|(offset, (_, txn))| {
                        Ok(TransactionSummary::try_from((
                            &block, offset, txn,
                        ))?)
                    })
                    .try_collect::<TransactionSummary<Types>, Vec<TransactionSummary<Types>>, QueryError>()?;

                Ok(transactions)
            })
            .try_collect::<Vec<Vec<TransactionSummary<Types>>>>()
            .await?
            .into_iter()
            .flatten()
            .collect();

        Ok(SearchResult {
            blocks: block_query_result,
            transactions: transactions_query_result,
        })
    }
}
