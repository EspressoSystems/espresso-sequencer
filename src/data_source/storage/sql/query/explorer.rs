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
    parse_block,
    postgres::types::{Json, ToSql},
    Transaction, BLOCK_COLUMNS,
};
use crate::{
    availability::{QueryableHeader, QueryablePayload, TransactionIndex},
    data_source::storage::ExplorerStorage,
    explorer::{
        self, errors::NotFound, query_data::TransactionDetailResponse, BalanceAmount, BlockDetail,
        BlockIdentifier, BlockRange, BlockSummary, ExplorerHistograms, ExplorerSummary,
        GenesisOverview, GetBlockDetailError, GetBlockSummariesError, GetBlockSummariesRequest,
        GetExplorerSummaryError, GetSearchResultsError, GetTransactionDetailError,
        GetTransactionSummariesError, GetTransactionSummariesRequest, SearchResult,
        TransactionIdentifier, TransactionRange, TransactionSummary, TransactionSummaryFilter,
    },
    Header, Payload, QueryError, QueryResult,
};
use async_trait::async_trait;
use committable::Committable;
use futures::stream::{self, StreamExt, TryStreamExt};
use hotshot_types::traits::node_implementation::NodeType;
use itertools::Itertools;
use std::num::NonZeroUsize;

#[async_trait]
impl<'a, Types: NodeType> ExplorerStorage<Types> for Transaction<'a>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    Header<Types>: QueryableHeader<Types> + explorer::traits::ExplorerHeader<Types>,
    crate::Transaction<Types>: explorer::traits::ExplorerTransaction,
    BalanceAmount<Types>: Into<explorer::monetary_value::MonetaryValue>,
{
    async fn get_block_summaries(
        &self,
        request: GetBlockSummariesRequest<Types>,
    ) -> Result<Vec<BlockSummary<Types>>, GetBlockSummariesError> {
        let request = &request.0;

        let (query, params): (String, Vec<Box<dyn ToSql + Send + Sync>>) = match request.target {
            BlockIdentifier::Latest => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        ORDER BY h.height DESC 
                        LIMIT $1"
                ),
                vec![Box::new(request.num_blocks.get() as i64)],
            ),
            BlockIdentifier::Height(height) => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        WHERE h.height <= $1
                        ORDER BY h.height DESC 
                        LIMIT $2"
                ),
                vec![
                    Box::new(height as i64),
                    Box::new(request.num_blocks.get() as i64),
                ],
            ),
            BlockIdentifier::Hash(hash) => (
                // We want to match the blocks starting with the given hash, and working backwards until we
                // have returned up to the number of requested blocks.  The hash for a block should be unique,
                // so we should just need to start with identifying the block height with the given hash, and
                // return all blocks with a height less than or equal to that height, up to the number of
                // requested blocks.
                format!(
                    "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        WHERE h.height <= (SELECT h1.height FROM header AS h1 WHERE h1.hash = $1)
                        ORDER BY h.height DESC 
                        LIMIT $2"
                ),
                vec![
                    Box::new(hash.to_string()),
                    Box::new(request.num_blocks.get() as i64),
                ],
            ),
        };

        let row_stream = self.query(&query, params).await?;
        let result = row_stream.map(|row| -> QueryResult<BlockSummary<Types>> {
            let block = parse_block::<Types>(row?)?;
            Ok(BlockSummary::try_from(block)?)
        });

        Ok(result.try_collect().await?)
    }

    async fn get_block_detail(
        &self,
        request: BlockIdentifier<Types>,
    ) -> Result<BlockDetail<Types>, GetBlockDetailError> {
        let (query, params): (String, Vec<Box<dyn ToSql + Sync + Send>>) = match request {
            BlockIdentifier::Latest => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        ORDER BY h.height DESC 
                        LIMIT 1"
                ),
                vec![],
            ),
            BlockIdentifier::Height(height) => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        WHERE h.height = $1
                        ORDER BY h.height DESC 
                        LIMIT 1"
                ),
                vec![Box::new(height as i64)],
            ),
            BlockIdentifier::Hash(hash) => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        WHERE h.hash = $1
                        ORDER BY h.height DESC 
                        LIMIT 1"
                ),
                vec![Box::new(hash.to_string())],
            ),
        };

        let query_result = self.query_one(&query, params).await?;
        let block = parse_block(query_result)?;

        Ok(BlockDetail::try_from(block).map_err(|e| QueryError::Error {
            message: e.to_string(),
        })?)
    }

    async fn get_transaction_summaries(
        &self,
        request: GetTransactionSummariesRequest<Types>,
    ) -> Result<Vec<TransactionSummary<Types>>, GetTransactionSummariesError> {
        let range = &request.range;
        let target = &range.target;
        let filter = &request.filter;

        // We need to figure out the transaction target we are going to start
        // returned results based on.
        let transaction_target = match target {
            TransactionIdentifier::Latest => self.query_one::<str, Vec<Box<dyn ToSql + Send + Sync>>>(
                "SELECT t.block_height AS height, t.index AS index FROM transaction AS t ORDER BY (t.block_height, t.index) DESC LIMIT 1",
                vec![],
            ).await,
            TransactionIdentifier::HeightAndOffset(height, _) => self.query_one(
                "SELECT t.block_height AS height, t.index AS index FROM transaction AS t WHERE t.block_height = $1 ORDER BY (t.block_height, t.index) DESC LIMIT 1",
                [*height as i64],
            ).await,
            TransactionIdentifier::Hash(hash) => self.query_one(
                "SELECT t.block_height AS height, t.index AS index FROM transaction AS t WHERE t.hash = $1 ORDER BY (t.block_height, t.index) DESC LIMIT 1",
                [hash.to_string()],
            ).await,
        };

        let transaction_target = match transaction_target {
            // If nothing is found, then we want to return an empty summary list
            // as it means there is either no transaction, or the targeting
            // criteria fails to identify any transaction
            Err(QueryError::NotFound) => return Ok(vec![]),
            _ => transaction_target,
        }?;

        let block_height = transaction_target.get::<_, i64>("height") as usize;
        let transaction_index = transaction_target.get::<_, Json<TransactionIndex<Types>>>("index");
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
        let block_stream = match filter {
            TransactionSummaryFilter::RollUp(_) => return Ok(vec![]),

            TransactionSummaryFilter::None => {
                self.query::<str, Vec<Box<dyn ToSql + Send + Sync>>>(
                    &format!(
                        "SELECT {BLOCK_COLUMNS}
                        FROM header AS h
                        JOIN payload AS p ON h.height = p.height
                        WHERE h.height IN (
                            SELECT t.block_height
                                FROM transaction AS t
                                WHERE (t.block_height, t.index) <= ($1, $2)
                                ORDER BY (t.block_height, t.index) DESC
                                LIMIT $3
                        )
                        ORDER BY h.height DESC"
                    ),
                    vec![
                        Box::new(block_height as i64),
                        Box::new(&transaction_index),
                        Box::new((range.num_transactions.get() + offset) as i64),
                    ],
                )
                .await
            }

            TransactionSummaryFilter::Block(block) => {
                self.query::<str, Vec<Box<dyn ToSql + Send + Sync>>>(
                    &format!(
                        "SELECT {BLOCK_COLUMNS}
                    FROM header AS h
                    JOIN payload AS p ON h.height = p.height
                    WHERE  h.height = $1
                    ORDER BY h.height DESC"
                    ),
                    vec![Box::new(*block as i64)],
                )
                .await
            }
        }?
        .map(|result| match result {
            Ok(row) => parse_block::<Types>(row),
            Err(err) => Err(err),
        });

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
            Err(err) => stream::iter(vec![Err(err)]),
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
        &self,
        request: TransactionIdentifier<Types>,
    ) -> Result<TransactionDetailResponse<Types>, GetTransactionDetailError> {
        let target = request;

        let (query, params): (String, Vec<Box<dyn ToSql + Send + Sync>>) = match target {
            TransactionIdentifier::Latest => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                            FROM header AS h
                            JOIN payload AS p ON h.height = p.height
                            WHERE h.height = (
                                SELECT MAX(t1.block_height)
                                    FROM transaction AS t1
                            )
                            ORDER BY h.height DESC"
                ),
                vec![],
            ),
            TransactionIdentifier::HeightAndOffset(height, offset) => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                            FROM header AS h
                            JOIN payload AS p ON h.height = p.height
                            WHERE h.height = (
                                SELECT t1.block_height
                                    FROM transaction AS t1
                                    WHERE t1.block_height = $1
                                    ORDER BY (t1.block_height, t1.index) DESC
                                    OFFSET $2
                                    LIMIT 1
                            )
                            ORDER BY h.height DESC"
                ),
                vec![Box::new(height as i64), Box::new(offset as i64)],
            ),
            TransactionIdentifier::Hash(hash) => (
                format!(
                    "SELECT {BLOCK_COLUMNS}
                            FROM header AS h
                            JOIN payload AS p ON h.height = p.height
                            WHERE h.height = (
                                SELECT t1.block_height
                                    FROM transaction AS t1
                                    WHERE t1.hash = $1
                                    ORDER BY (t1.block_height, t1.index) DESC
                                    LIMIT 1
                            )
                            ORDER BY h.height DESC"
                ),
                vec![Box::new(hash.to_string())],
            ),
        };

        let query_row = self.query_one(&query, params).await?;
        let block = parse_block::<Types>(query_row)?;

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
        &self,
    ) -> Result<ExplorerSummary<Types>, GetExplorerSummaryError> {
        let histograms= async {
            let historgram_query_result = self.query(
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
                Vec::<Box<dyn ToSql + Send + Sync>>::new(),
            ).await?;

            let histograms:Result<ExplorerHistograms, QueryError> = historgram_query_result
                .map(|row_stream| {
                    row_stream.map(|row| {
                        let height: i64 = row.try_get("height").map_err(|e| QueryError::Error {
                            message: format!("failed to get column height {e}"),
                        })?;
                        let timestamp: i64 =
                            row.try_get("timestamp").map_err(|e| QueryError::Error {
                                message: format!("failed to get column timestamp {e}"),
                            })?;
                        let time: Option<i64> = row.try_get("time").map_err(|e| QueryError::Error {
                            message: format!("failed to get column time {e}"),
                        })?;
                        let size: i32 = row.try_get("size").map_err(|e| QueryError::Error {
                            message: format!("failed to get column size {e}"),
                        })?;
                        let num_transactions: i64 =
                            row.try_get("transactions").map_err(|e| QueryError::Error {
                                message: format!("failed to get column transactions {e}"),
                            })?;

                        let height: u64 = height.try_into().map_err(|e| QueryError::Error {
                            message: format!("failed to convert height to u64 {e}"),
                        })?;
                        let timestamp: u64 =
                            timestamp.try_into().map_err(|e| QueryError::Error {
                                message: format!("failed to convert timestamp to u64 {e}"),
                            })?;
                        let time: u64 = time.unwrap_or(0).try_into().map_err(|e| QueryError::Error {
                            message: format!("failed to convert time to u64 {e}"),
                        })?;
                        let size: u64 = size.try_into().map_err(|e| QueryError::Error {
                            message: format!("failed to convert size to u64 {e}"),
                        })?;
                        let num_transactions: u64 =
                            num_transactions.try_into().map_err(|e| QueryError::Error {
                                message: format!("failed to convert num_transactions to u64 {e}"),
                            })?;

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
                     row: Result<(u64, u64, u64, u64, u64), QueryError>| async {
                        let (height, _timestamp, time, size, num_transactions) = row?;
                        histograms
                            .block_time
                            .push(time);
                        histograms.block_size.push(size);
                        histograms.block_transactions.push(num_transactions);
                        histograms.block_heights.push(height);
                        Ok(histograms)
                    },
                ).await;

            histograms
        }.await?;

        let genesis_overview = async {
            let row = self
                .query_one(
                    "SELECT
                    (SELECT MAX(height) + 1 FROM header) AS blocks,
                    (SELECT COUNT(*) FROM transaction) AS transactions",
                    Vec::<Box<dyn ToSql + Send + Sync>>::new(),
                )
                .await?;

            let blocks: i64 = row.try_get("blocks").map_err(|e| QueryError::Error {
                message: format!("failed to get column blocks {e}"),
            })?;
            let transactions: i64 = row.try_get("transactions").map_err(|e| QueryError::Error {
                message: format!("failed to get column transactions {e}"),
            })?;

            let blocks: u64 = blocks.try_into().map_err(|e| QueryError::Error {
                message: format!("failed to convert blocks to u64 {e}"),
            })?;
            let transactions: u64 = transactions.try_into().map_err(|e| QueryError::Error {
                message: format!("failed to convert transactions to u64 {e}"),
            })?;

            Ok::<_, QueryError>(GenesisOverview {
                rollups: 0,
                transactions,
                blocks,
            })
        }
        .await?;

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
        &self,
        query: String,
    ) -> Result<SearchResult<Types>, GetSearchResultsError> {
        let block_query = format!(
            "SELECT {BLOCK_COLUMNS}
                FROM header AS h
                JOIN payload AS p ON h.height = p.height
                WHERE h.hash LIKE $1 || '%'
                ORDER BY h.height DESC
                LIMIT 5"
        );
        let block_query_rows = self.query(&block_query, vec![&query]).await?;
        let block_query_result: Vec<BlockSummary<Types>> = block_query_rows
            .map(|row| -> Result<BlockSummary<Types>, QueryError> {
                let block = parse_block::<Types>(row?)?;
                Ok(BlockSummary::try_from(block)?)
            })
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
        let transactions_query_rows = self.query(&transactions_query, vec![&query]).await?;
        let transactions_query_result: Vec<TransactionSummary<Types>> = transactions_query_rows
            .map(|row| -> Result<Vec<TransactionSummary<Types>>, QueryError>{
                let block = parse_block::<Types>(row?)?;
                let transactions = block
                    .enumerate()
                    .enumerate()
                    .filter(|(_, (_, txn))| txn.commit().to_string().starts_with(&query))
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
