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

//! Queries for HotShot chain state.
//!
//! The availability API provides an objective view of the HotShot blockchain. It provides access
//! only to normative data: that is, data which is agreed upon by all honest consensus nodes and
//! which is immutable. This means access to core consensus data structures including leaves,
//! blocks, and headers, where each query is pure and idempotent. This also means that it is
//! possible for a client to verify all of the information provided by this API, by running a
//! HotShot light client and downloading the appropriate evidence with each query.
//!
//! This API does not provide any queries which represent only the _current_ state of the chain or
//! may change over time, and it does not provide information for which there is not (yet) agreement
//! of a supermajority of consensus nodes. For information about the current dynamic state of
//! consensus and uncommitted state, try the [status](crate::status) API. For information about the
//! chain which is tabulated by this specific node and not subject to full consensus agreement, try
//! the [node](crate::node) API.

use crate::{api::load_api, Payload};
use derive_more::From;
use futures::{FutureExt, StreamExt, TryFutureExt, TryStreamExt};
use hotshot_types::traits::node_implementation::NodeType;
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, Snafu};
use std::{fmt::Display, path::PathBuf, time::Duration};
use tide_disco::{api::ApiError, method::ReadState, Api, RequestError, StatusCode};
use vbs::version::StaticVersionType;

pub(crate) mod data_source;
mod fetch;
pub(crate) mod query_data;
pub use data_source::*;
pub use fetch::Fetch;
pub use query_data::*;

pub struct Options {
    pub api_path: Option<PathBuf>,

    /// Timeout for failing requests due to missing data.
    ///
    /// If data needed to respond to a request is missing, it can (in some cases) be fetched from an
    /// external provider. This parameter controls how long the request handler will wait for
    /// missing data to be fetched before giving up and failing the request.
    pub fetch_timeout: Duration,

    /// Additional API specification files to merge with `availability-api-path`.
    ///
    /// These optional files may contain route definitions for application-specific routes that have
    /// been added as extensions to the basic availability API.
    pub extensions: Vec<toml::Value>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            api_path: None,
            fetch_timeout: Duration::from_millis(500),
            extensions: vec![],
        }
    }
}

#[derive(Clone, Debug, From, Snafu, Deserialize, Serialize)]
#[snafu(visibility(pub))]
pub enum Error {
    Request {
        source: RequestError,
    },
    #[snafu(display("leaf {resource} missing or not available"))]
    #[from(ignore)]
    FetchLeaf {
        resource: String,
    },
    #[snafu(display("block {resource} missing or not available"))]
    #[from(ignore)]
    FetchBlock {
        resource: String,
    },
    #[snafu(display("transaction {resource} missing or not available"))]
    #[from(ignore)]
    FetchTransaction {
        resource: String,
    },
    #[snafu(display("transaction index {index} out of range for block {height}"))]
    #[from(ignore)]
    InvalidTransactionIndex {
        height: u64,
        index: u64,
    },
    Custom {
        message: String,
        status: StatusCode,
    },
}

impl Error {
    pub fn internal<M: Display>(message: M) -> Self {
        Self::Custom {
            message: message.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn status(&self) -> StatusCode {
        match self {
            Self::Request { .. } => StatusCode::BAD_REQUEST,
            Self::FetchLeaf { .. } | Self::FetchBlock { .. } | Self::FetchTransaction { .. } => {
                StatusCode::NOT_FOUND
            }
            Self::InvalidTransactionIndex { .. } => StatusCode::NOT_FOUND,
            Self::Custom { status, .. } => *status,
        }
    }
}

pub fn define_api<State, Types: NodeType, Ver: StaticVersionType + 'static>(
    options: &Options,
    _: Ver,
) -> Result<Api<State, Error, Ver>, ApiError>
where
    State: 'static + Send + Sync + ReadState,
    <State as ReadState>::State: Send + Sync + AvailabilityDataSource<Types>,
    Payload<Types>: QueryablePayload<Types>,
{
    let mut api = load_api::<State, Error, Ver>(
        options.api_path.as_ref(),
        include_str!("../api/availability.toml"),
        options.extensions.clone(),
    )?;
    let timeout = options.fetch_timeout;

    api.with_version("0.0.1".parse().unwrap())
        .at("get_leaf", move |req, state| {
            async move {
                let id = match req.opt_integer_param("height")? {
                    Some(height) => LeafId::Number(height),
                    None => LeafId::Hash(req.blob_param("hash")?),
                };
                let fetch = state.read(|state| state.get_leaf(id).boxed()).await;
                fetch.with_timeout(timeout).await.context(FetchLeafSnafu {
                    resource: id.to_string(),
                })
            }
            .boxed()
        })?
        .at("get_leaf_range", move |req, state| {
            async move {
                let from = req.integer_param::<_, usize>("from")?;
                let until = req.integer_param("until")?;

                let leaves = state
                    .read(|state| state.get_leaf_range(from..until).boxed())
                    .await;
                leaves
                    .enumerate()
                    .then(|(index, fetch)| async move {
                        fetch.with_timeout(timeout).await.context(FetchLeafSnafu {
                            resource: (index + from).to_string(),
                        })
                    })
                    .try_collect::<Vec<_>>()
                    .await
            }
            .boxed()
        })?
        .stream("stream_leaves", move |req, state| {
            async move {
                let height = req.integer_param("height")?;
                state
                    .read(|state| {
                        async move { Ok(state.subscribe_leaves(height).await.map(Ok)) }.boxed()
                    })
                    .await
            }
            .try_flatten_stream()
            .boxed()
        })?
        .at("get_header", move |req, state| {
            async move {
                let id = if let Some(height) = req.opt_integer_param("height")? {
                    BlockId::Number(height)
                } else if let Some(hash) = req.opt_blob_param("hash")? {
                    BlockId::Hash(hash)
                } else {
                    BlockId::PayloadHash(req.blob_param("payload-hash")?)
                };
                let fetch = state.read(|state| state.get_block(id).boxed()).await;
                Ok(fetch
                    .with_timeout(timeout)
                    .await
                    .context(FetchBlockSnafu {
                        resource: id.to_string(),
                    })?
                    .header()
                    .clone())
            }
            .boxed()
        })?
        .at("get_header_range", move |req, state| {
            async move {
                let from = req.integer_param::<_, usize>("from")?;
                let until = req.integer_param::<_, usize>("until")?;

                let headers = state
                    .read(|state| state.get_block_range(from..until).boxed())
                    .await;
                headers
                    .enumerate()
                    .then(|(index, fetch)| async move {
                        fetch.with_timeout(timeout).await.context(FetchBlockSnafu {
                            resource: (index + from).to_string(),
                        })
                    })
                    .map(|r| r.map(|block| block.header().clone()))
                    .try_collect::<Vec<_>>()
                    .await
            }
            .boxed()
        })?
        .stream("stream_headers", move |req, state| {
            async move {
                let height = req.integer_param("height")?;
                Ok(state
                    .read(|state| {
                        async move {
                            state
                                .subscribe_blocks(height)
                                .await
                                .map(|block| Ok(block.header))
                        }
                        .boxed()
                    })
                    .await)
            }
            .try_flatten_stream()
            .boxed()
        })?
        .at("get_block", move |req, state| {
            async move {
                let id = if let Some(height) = req.opt_integer_param("height")? {
                    BlockId::Number(height)
                } else if let Some(hash) = req.opt_blob_param("hash")? {
                    BlockId::Hash(hash)
                } else {
                    BlockId::PayloadHash(req.blob_param("payload-hash")?)
                };
                let fetch = state.read(|state| state.get_block(id).boxed()).await;
                fetch.with_timeout(timeout).await.context(FetchBlockSnafu {
                    resource: id.to_string(),
                })
            }
            .boxed()
        })?
        .at("get_block_range", move |req, state| {
            async move {
                let from = req.integer_param::<_, usize>("from")?;
                let until = req.integer_param("until")?;

                let blocks = state
                    .read(|state| state.get_block_range(from..until).boxed())
                    .await;
                blocks
                    .enumerate()
                    .then(|(index, fetch)| async move {
                        fetch.with_timeout(timeout).await.context(FetchBlockSnafu {
                            resource: (index + from).to_string(),
                        })
                    })
                    .try_collect::<Vec<_>>()
                    .await
            }
            .boxed()
        })?
        .stream("stream_blocks", move |req, state| {
            async move {
                let height = req.integer_param("height")?;
                Ok(state
                    .read(|state| {
                        async move { state.subscribe_blocks(height).await.map(Ok) }.boxed()
                    })
                    .await)
            }
            .try_flatten_stream()
            .boxed()
        })?
        .at("get_payload", move |req, state| {
            async move {
                let id = if let Some(height) = req.opt_integer_param("height")? {
                    BlockId::Number(height)
                } else if let Some(hash) = req.opt_blob_param("hash")? {
                    BlockId::PayloadHash(hash)
                } else {
                    BlockId::Hash(req.blob_param("block-hash")?)
                };
                let fetch = state.read(|state| state.get_payload(id).boxed()).await;
                fetch.with_timeout(timeout).await.context(FetchBlockSnafu {
                    resource: id.to_string(),
                })
            }
            .boxed()
        })?
        .at("get_payload_range", move |req, state| {
            async move {
                let from = req.integer_param::<_, usize>("from")?;
                let until = req.integer_param("until")?;

                let payloads = state
                    .read(|state| state.get_payload_range(from..until).boxed())
                    .await;
                payloads
                    .enumerate()
                    .then(|(index, fetch)| async move {
                        fetch.with_timeout(timeout).await.context(FetchBlockSnafu {
                            resource: (index + from).to_string(),
                        })
                    })
                    .try_collect::<Vec<_>>()
                    .await
            }
            .boxed()
        })?
        .stream("stream_payloads", move |req, state| {
            async move {
                let height = req.integer_param("height")?;
                Ok(state
                    .read(|state| {
                        async move { state.subscribe_payloads(height).await.map(Ok) }.boxed()
                    })
                    .await)
            }
            .try_flatten_stream()
            .boxed()
        })?
        .at("get_vid_common", move |req, state| {
            async move {
                let id = if let Some(height) = req.opt_integer_param("height")? {
                    BlockId::Number(height)
                } else if let Some(hash) = req.opt_blob_param("hash")? {
                    BlockId::Hash(hash)
                } else {
                    BlockId::PayloadHash(req.blob_param("payload-hash")?)
                };
                let fetch = state.read(|state| state.get_vid_common(id).boxed()).await;
                fetch.with_timeout(timeout).await.context(FetchBlockSnafu {
                    resource: id.to_string(),
                })
            }
            .boxed()
        })?
        .stream("stream_vid_common", move |req, state| {
            async move {
                let height = req.integer_param("height")?;
                Ok(state
                    .read(|state| {
                        async move { state.subscribe_vid_common(height).await.map(Ok) }.boxed()
                    })
                    .await)
            }
            .try_flatten_stream()
            .boxed()
        })?
        .at("get_transaction", move |req, state| {
            async move {
                match req.opt_blob_param("hash")? {
                    Some(hash) => {
                        let fetch = state
                            .read(|state| state.get_transaction(hash).boxed())
                            .await;
                        fetch
                            .with_timeout(timeout)
                            .await
                            .context(FetchTransactionSnafu {
                                resource: hash.to_string(),
                            })
                    }
                    None => {
                        let height: u64 = req.integer_param("height")?;
                        let fetch = state
                            .read(|state| state.get_block(height as usize).boxed())
                            .await;
                        let block = fetch.with_timeout(timeout).await.context(FetchBlockSnafu {
                            resource: height.to_string(),
                        })?;
                        let i: u64 = req.integer_param("index")?;
                        let index = block
                            .payload()
                            .nth(block.metadata(), i as usize)
                            .context(InvalidTransactionIndexSnafu { height, index: i })?;
                        TransactionQueryData::new(&block, index, i)
                            .context(InvalidTransactionIndexSnafu { height, index: i })
                    }
                }
            }
            .boxed()
        })?
        .at("get_block_summary", move |req, state| {
            async move {
                let id: usize = req.integer_param("height")?;

                let fetch = state.read(|state| state.get_block(id).boxed()).await;
                fetch
                    .with_timeout(timeout)
                    .await
                    .context(FetchBlockSnafu {
                        resource: id.to_string(),
                    })
                    .map(BlockSummaryQueryData::from)
            }
            .boxed()
        })?
        .at("get_block_summary_range", move |req, state| {
            async move {
                let from: usize = req.integer_param("from")?;
                let until: usize = req.integer_param("until")?;

                let blocks = state
                    .read(|state| state.get_block_range(from..until).boxed())
                    .await;
                let result: Vec<BlockSummaryQueryData<Types>> = blocks
                    .enumerate()
                    .then(|(index, fetch)| async move {
                        fetch.with_timeout(timeout).await.context(FetchBlockSnafu {
                            resource: (index + from).to_string(),
                        })
                    })
                    .map(|result| result.map(BlockSummaryQueryData::from))
                    .try_collect()
                    .await?;

                Ok(result)
            }
            .boxed()
        })?;
    Ok(api)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        data_source::{
            storage::no_storage, ExtensibleDataSource, Transaction, VersionedDataSource,
        },
        status::StatusDataSource,
        task::BackgroundTask,
        testing::{
            consensus::{MockDataSource, MockNetwork, TestableDataSource},
            mocks::{mock_transaction, MockBase, MockHeader, MockPayload, MockTypes},
            setup_test,
        },
        types::HeightIndexed,
        ApiState, Error, Header,
    };
    use async_std::sync::RwLock;
    use committable::Committable;
    use futures::future::FutureExt;
    use hotshot_types::{data::Leaf, simple_certificate::QuorumCertificate};
    use portpicker::pick_unused_port;
    use std::time::Duration;
    use surf_disco::Client;
    use tempfile::TempDir;
    use tide_disco::App;
    use toml::toml;

    /// Get the current ledger height and a list of non-empty leaf/block pairs.
    async fn get_non_empty_blocks(
        client: &Client<Error, MockBase>,
    ) -> (
        u64,
        Vec<(LeafQueryData<MockTypes>, BlockQueryData<MockTypes>)>,
    ) {
        let mut blocks = vec![];
        // Ignore the genesis block (start from height 1).
        for i in 1.. {
            match client
                .get::<BlockQueryData<MockTypes>>(&format!("block/{}", i))
                .send()
                .await
            {
                Ok(block) => {
                    if !block.is_empty() {
                        let leaf = client.get(&format!("leaf/{}", i)).send().await.unwrap();
                        blocks.push((leaf, block));
                    }
                }
                Err(Error::Availability {
                    source: super::Error::FetchBlock { .. },
                }) => {
                    tracing::info!(
                        "found end of ledger at height {i}, non-empty blocks are {blocks:?}",
                    );
                    return (i, blocks);
                }
                Err(err) => panic!("unexpected error {}", err),
            }
        }
        unreachable!()
    }

    async fn validate(client: &Client<Error, MockBase>, height: u64) {
        // Check the consistency of every block/leaf pair.
        for i in 0..height {
            // Limit the number of blocks we validate in order to
            // speeed up the tests.
            if ![0, 1, height / 2, height - 1].contains(&i) {
                continue;
            }
            tracing::info!("validate block {i}/{height}");

            // Check that looking up the leaf various ways returns the correct leaf.
            let leaf: LeafQueryData<MockTypes> =
                client.get(&format!("leaf/{}", i)).send().await.unwrap();
            assert_eq!(leaf.height(), i);
            assert_eq!(
                leaf,
                client
                    .get(&format!("leaf/hash/{}", leaf.hash()))
                    .send()
                    .await
                    .unwrap()
            );

            // Check that looking up the block various ways returns the correct block.
            let block: BlockQueryData<MockTypes> =
                client.get(&format!("block/{}", i)).send().await.unwrap();
            let expected_payload = PayloadQueryData::from(block.clone());
            assert_eq!(leaf.block_hash(), block.hash());
            assert_eq!(block.height(), i);
            assert_eq!(
                block,
                client
                    .get(&format!("block/hash/{}", block.hash()))
                    .send()
                    .await
                    .unwrap()
            );
            assert_eq!(
                *block.header(),
                client.get(&format!("header/{i}")).send().await.unwrap()
            );
            assert_eq!(
                *block.header(),
                client
                    .get(&format!("header/hash/{}", block.hash()))
                    .send()
                    .await
                    .unwrap()
            );
            assert_eq!(
                expected_payload,
                client.get(&format!("payload/{i}")).send().await.unwrap(),
            );
            assert_eq!(
                expected_payload,
                client
                    .get(&format!("payload/block-hash/{}", block.hash()))
                    .send()
                    .await
                    .unwrap(),
            );
            // Look up the common VID data.
            let common: VidCommonQueryData<MockTypes> = client
                .get(&format!("vid/common/{}", block.height()))
                .send()
                .await
                .unwrap();
            assert_eq!(common.height(), block.height());
            assert_eq!(common.block_hash(), block.hash());
            assert_eq!(common.payload_hash(), block.payload_hash());
            assert_eq!(
                common,
                client
                    .get(&format!("vid/common/hash/{}", block.hash()))
                    .send()
                    .await
                    .unwrap()
            );

            let block_summary = client
                .get(&format!("block/summary/{}", i))
                .send()
                .await
                .unwrap();
            assert_eq!(
                BlockSummaryQueryData::<MockTypes>::from(block.clone()),
                block_summary,
            );
            assert_eq!(block_summary.header(), block.header());
            assert_eq!(block_summary.hash(), block.hash());
            assert_eq!(block_summary.size(), block.size());
            assert_eq!(block_summary.num_transactions(), block.num_transactions());

            let block_summaries: Vec<BlockSummaryQueryData<MockTypes>> = client
                .get(&format!("block/summaries/{}/{}", 0, i))
                .send()
                .await
                .unwrap();
            assert_eq!(block_summaries.len() as u64, i);

            // We should be able to look up the block by payload hash. Note that for duplicate
            // payloads, these endpoints may return a different block with the same payload, which
            // is acceptable. Therefore, we don't check equivalence of the entire `BlockQueryData`
            // response, only its payload.
            assert_eq!(
                block.payload(),
                client
                    .get::<BlockQueryData<MockTypes>>(&format!(
                        "block/payload-hash/{}",
                        block.payload_hash()
                    ))
                    .send()
                    .await
                    .unwrap()
                    .payload()
            );
            assert_eq!(
                block.payload_hash(),
                client
                    .get::<Header<MockTypes>>(&format!(
                        "header/payload-hash/{}",
                        block.payload_hash()
                    ))
                    .send()
                    .await
                    .unwrap()
                    .payload_commitment
            );
            assert_eq!(
                block.payload(),
                client
                    .get::<PayloadQueryData<MockTypes>>(&format!(
                        "payload/hash/{}",
                        block.payload_hash()
                    ))
                    .send()
                    .await
                    .unwrap()
                    .data(),
            );
            assert_eq!(
                common.common(),
                client
                    .get::<VidCommonQueryData<MockTypes>>(&format!(
                        "vid/common/payload-hash/{}",
                        block.payload_hash()
                    ))
                    .send()
                    .await
                    .unwrap()
                    .common()
            );

            // Check that looking up each transaction in the block various ways returns the correct
            // transaction.
            for (j, txn_from_block) in block.enumerate() {
                let txn: TransactionQueryData<MockTypes> = client
                    .get(&format!("transaction/{}/{}", i, j))
                    .send()
                    .await
                    .unwrap();
                assert_eq!(txn.block_height(), i);
                assert_eq!(txn.block_hash(), block.hash());
                assert_eq!(txn.index(), j as u64);
                assert_eq!(txn.hash(), txn_from_block.commit());
                assert_eq!(txn.transaction(), &txn_from_block);
                // We should be able to look up the transaction by hash. Note that for duplicate
                // transactions, this endpoint may return a different transaction with the same
                // hash, which is acceptable. Therefore, we don't check equivalence of the entire
                // `TransactionQueryData` response, only its commitment.
                assert_eq!(
                    txn.hash(),
                    client
                        .get::<TransactionQueryData<MockTypes>>(&format!(
                            "transaction/hash/{}",
                            txn.hash()
                        ))
                        .send()
                        .await
                        .unwrap()
                        .hash()
                );
            }

            let block_range: Vec<BlockQueryData<MockTypes>> = client
                .get(&format!("block/{}/{}", 0, i))
                .send()
                .await
                .unwrap();

            assert_eq!(block_range.len() as u64, i);

            let leaf_range: Vec<LeafQueryData<MockTypes>> = client
                .get(&format!("leaf/{}/{}", 0, i))
                .send()
                .await
                .unwrap();

            assert_eq!(leaf_range.len() as u64, i);

            let payload_range: Vec<PayloadQueryData<MockTypes>> = client
                .get(&format!("payload/{}/{}", 0, i))
                .send()
                .await
                .unwrap();

            assert_eq!(payload_range.len() as u64, i);

            let header_range: Vec<Header<MockTypes>> = client
                .get(&format!("header/{}/{}", 0, i))
                .send()
                .await
                .unwrap();

            assert_eq!(header_range.len() as u64, i);
        }
    }

    #[async_std::test]
    async fn test_api() {
        test_api_helper::<MockDataSource>(Duration::from_millis(500)).await;
    }

    // This test runs the `postgres` Docker image, which doesn't work on Windows.
    #[cfg(not(target_os = "windows"))]
    #[async_std::test]
    async fn test_api_no_storage() {
        // With a long enough fetch timeout, we can run the API without any local storage and it
        // still works: missing data is fetched on demand or proactively from a peer.
        //
        // We set the timeout very conservatively here at 5s, so the test passes deterministically
        // even in resource-constrained environments like CI builders. In practice, on a reasonably
        // powerful server with a fast connection to a powerful peer, this timeout can likely be
        // made much shorter, under 1s.
        test_api_helper::<no_storage::testing::DataSource>(Duration::from_secs(5)).await;
    }

    async fn test_api_helper<D: TestableDataSource>(fetch_timeout: Duration) {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<D>::init_with_config(|cfg| {
            // Make the rate of empty block production slower than the API fetching timeout.
            // Otherwise, we will produce new blocks faster than we can fetch them (particularly in
            // the no-storage case, where fetching is quite slow) and the test will never finish.
            cfg.builder_timeout = fetch_timeout + Duration::from_millis(500);
        })
        .await;
        network.start().await;

        // Start the web server.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(ApiState::from(network.data_source()));
        app.register_module(
            "availability",
            define_api(
                &Options {
                    fetch_timeout,
                    ..Default::default()
                },
                MockBase::instance(),
            )
            .unwrap(),
        )
        .unwrap();
        network.spawn(
            "server",
            app.serve(format!("0.0.0.0:{}", port), MockBase::instance()),
        );

        // Start a client.
        let client = Client::<Error, MockBase>::new(
            format!("http://localhost:{}/availability", port)
                .parse()
                .unwrap(),
        );
        assert!(client.connect(Some(Duration::from_secs(60))).await);
        assert_eq!(get_non_empty_blocks(&client).await.1, vec![]);

        // Submit a few blocks and make sure each one gets reflected in the query service and
        // preserves the consistency of the data and indices.
        let leaves = client
            .socket("stream/leaves/0")
            .subscribe::<LeafQueryData<MockTypes>>()
            .await
            .unwrap();
        let headers = client
            .socket("stream/headers/0")
            .subscribe::<Header<MockTypes>>()
            .await
            .unwrap();
        let blocks = client
            .socket("stream/blocks/0")
            .subscribe::<BlockQueryData<MockTypes>>()
            .await
            .unwrap();
        let vid_common = client
            .socket("stream/vid/common/0")
            .subscribe::<VidCommonQueryData<MockTypes>>()
            .await
            .unwrap();
        let mut chain = leaves.zip(headers.zip(blocks.zip(vid_common))).enumerate();
        for nonce in 0..3 {
            let txn = mock_transaction(vec![nonce]);
            network.submit_transaction(txn).await;

            // Wait for the transaction to be finalized.
            let (i, leaf, block, common) = loop {
                tracing::info!("waiting for block with transaction {}", nonce);
                let (i, (leaf, (header, (block, common)))) = chain.next().await.unwrap();
                tracing::info!(i, ?leaf, ?header, ?block, ?common);
                let leaf = leaf.unwrap();
                let header = header.unwrap();
                let block = block.unwrap();
                let common = common.unwrap();
                assert_eq!(leaf.height() as usize, i);
                assert_eq!(leaf.block_hash(), block.hash());
                assert_eq!(block.header(), &header);
                assert_eq!(common.height() as usize, i);
                if !block.is_empty() {
                    break (i, leaf, block, common);
                }
            };
            assert_eq!(
                leaf,
                client.get(&format!("leaf/{}", i)).send().await.unwrap()
            );
            assert_eq!(
                block,
                client.get(&format!("block/{}", i)).send().await.unwrap()
            );
            assert_eq!(
                common,
                client.get(&format!("vid/common/{i}")).send().await.unwrap()
            );

            validate(&client, (i + 1) as u64).await;
        }

        network.shut_down().await;
    }

    #[async_std::test]
    async fn test_extensions() {
        use hotshot_example_types::node_types::TestVersions;

        setup_test();

        let dir = TempDir::with_prefix("test_availability_extensions").unwrap();
        let data_source = ExtensibleDataSource::new(
            MockDataSource::create(dir.path(), Default::default())
                .await
                .unwrap(),
            0,
        );

        // mock up some consensus data.
        let leaf = Leaf::<MockTypes>::genesis(&Default::default(), &Default::default()).await;
        let qc =
            QuorumCertificate::genesis::<TestVersions>(&Default::default(), &Default::default())
                .await;
        let leaf = LeafQueryData::new(leaf, qc).unwrap();
        let block = BlockQueryData::new(leaf.header().clone(), MockPayload::genesis());

        let mut tx = data_source.write().await.unwrap();
        tx.insert_leaf(leaf).await.unwrap();
        tx.insert_block(block.clone()).await.unwrap();
        tx.commit().await.unwrap();

        // assert that the store has data before we move on to API requests
        assert_eq!(
            ExtensibleDataSource::<MockDataSource, u64>::block_height(&data_source)
                .await
                .unwrap(),
            1
        );
        assert_eq!(block, data_source.get_block(0).await.await);

        // Create the API extensions specification.
        let extensions = toml! {
            [route.post_ext]
            PATH = ["/ext/:val"]
            METHOD = "POST"
            ":val" = "Integer"

            [route.get_ext]
            PATH = ["/ext"]
            METHOD = "GET"
        };

        let mut api =
            define_api::<RwLock<ExtensibleDataSource<MockDataSource, u64>>, MockTypes, MockBase>(
                &Options {
                    extensions: vec![extensions.into()],
                    ..Default::default()
                },
                MockBase::instance(),
            )
            .unwrap();
        api.get("get_ext", |_, state| {
            async move { Ok(*state.as_ref()) }.boxed()
        })
        .unwrap()
        .post("post_ext", |req, state| {
            async move {
                *state.as_mut() = req.integer_param("val")?;
                Ok(())
            }
            .boxed()
        })
        .unwrap();

        let mut app = App::<_, Error>::with_state(RwLock::new(data_source));
        app.register_module("availability", api).unwrap();

        let port = pick_unused_port().unwrap();
        let _server = BackgroundTask::spawn(
            "server",
            app.serve(format!("0.0.0.0:{}", port), MockBase::instance()),
        );

        let client = Client::<Error, MockBase>::new(
            format!("http://localhost:{}/availability", port)
                .parse()
                .unwrap(),
        );
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        assert_eq!(client.get::<u64>("ext").send().await.unwrap(), 0);
        client.post::<()>("ext/42").send().await.unwrap();
        assert_eq!(client.get::<u64>("ext").send().await.unwrap(), 42);

        // Ensure we can still access the built-in functionality.
        assert_eq!(
            client
                .get::<MockHeader>("header/0")
                .send()
                .await
                .unwrap()
                .block_number,
            0
        );
    }
}
