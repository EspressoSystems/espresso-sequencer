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

use crate::{api::load_api, Block, QueryError};
use clap::Args;
use derive_more::From;
use futures::{FutureExt, StreamExt, TryFutureExt};
use hotshot_types::traits::{
    node_implementation::{NodeImplementation, NodeType},
    signature_key::EncodedPublicKey,
};
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt, Snafu};
use std::fmt::Display;
use std::path::PathBuf;
use tide_disco::{api::ApiError, method::ReadState, Api, RequestError, StatusCode};

pub(crate) mod data_source;
pub(crate) mod query_data;
pub use data_source::*;
pub use query_data::*;

#[derive(Args, Default)]
pub struct Options {
    #[arg(long = "availability-api-path", env = "HOTSHOT_AVAILABILITY_API_PATH")]
    pub api_path: Option<PathBuf>,

    /// Additional API specification files to merge with `availability-api-path`.
    ///
    /// These optional files may contain route definitions for application-specific routes that have
    /// been added as extensions to the basic availability API.
    #[arg(
        long = "availability-extension",
        env = "HOTSHOT_AVAILABILITY_EXTENSIONS",
        value_delimiter = ','
    )]
    pub extensions: Vec<toml::Value>,
}

#[derive(Clone, Debug, From, Snafu, Deserialize, Serialize)]
#[snafu(visibility(pub))]
pub enum Error {
    Request {
        source: RequestError,
    },
    #[snafu(display("error fetching leaf {resource}: {source}"))]
    #[from(ignore)]
    QueryLeaf {
        source: QueryError,
        resource: String,
    },
    #[snafu(display("error streaming leaves: {source}"))]
    #[from(ignore)]
    StreamLeaf {
        source: QueryError,
    },
    #[snafu(display("error fetching block {resource}: {source}"))]
    #[from(ignore)]
    QueryBlock {
        source: QueryError,
        resource: String,
    },
    #[snafu(display("error streaming blocks: {source}"))]
    #[from(ignore)]
    StreamBlock {
        source: QueryError,
    },
    #[snafu(display("error fetching transaction {resource}: {source}"))]
    #[from(ignore)]
    QueryTransaction {
        source: QueryError,
        resource: String,
    },
    #[snafu(display("error fetching proposals by {proposer}: {source}"))]
    #[from(ignore)]
    QueryProposals {
        source: QueryError,
        proposer: EncodedPublicKey,
    },
    #[snafu(display("transaction index {index} out of range for block {height}"))]
    #[from(ignore)]
    InvalidTransactionIndex {
        height: u64,
        index: u64,
    },
    #[snafu(display("unable to open leaf stream at {}: {}", height, reason))]
    #[from(ignore)]
    LeafStream {
        height: u64,
        reason: String,
    },
    #[snafu(display("unable to open block stream at {}: {}", height, reason))]
    #[from(ignore)]
    BlockStream {
        height: u64,
        reason: String,
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
            status: StatusCode::InternalServerError,
        }
    }

    pub fn status(&self) -> StatusCode {
        match self {
            Self::Request { .. } => StatusCode::BadRequest,
            Self::QueryLeaf { source, .. }
            | Self::StreamLeaf { source, .. }
            | Self::QueryBlock { source, .. }
            | Self::StreamBlock { source, .. }
            | Self::QueryTransaction { source, .. }
            | Self::QueryProposals { source, .. } => source.status(),
            Self::InvalidTransactionIndex { .. } => StatusCode::NotFound,
            Self::LeafStream { .. } | Self::BlockStream { .. } => StatusCode::InternalServerError,
            Self::Custom { status, .. } => *status,
        }
    }
}

pub fn define_api<State, Types: NodeType, I: NodeImplementation<Types>>(
    options: &Options,
) -> Result<Api<State, Error>, ApiError>
where
    State: 'static + Send + Sync + ReadState,
    <State as ReadState>::State: Send + Sync + AvailabilityDataSource<Types, I>,
    Block<Types>: QueryableBlock,
{
    let mut api = load_api::<State, Error>(
        options.api_path.as_ref(),
        include_str!("../api/availability.toml"),
        options.extensions.clone(),
    )?;
    api.with_version("0.0.1".parse().unwrap())
        .get("getleaf", |req, state| {
            async move {
                let id = match req.opt_integer_param("height")? {
                    Some(height) => ResourceId::Number(height),
                    None => ResourceId::Hash(req.blob_param("hash")?),
                };
                state.get_leaf(id).await.context(QueryLeafSnafu {
                    resource: id.to_string(),
                })
            }
            .boxed()
        })?
        .stream("streamleaves", |req, state| {
            async move {
                let height = req.integer_param("height")?;
                state
                    .read(|state| {
                        async move {
                            Ok(state
                                .subscribe_leaves(height)
                                .await
                                .map_err(|err| Error::LeafStream {
                                    height: height as u64,
                                    reason: err.to_string(),
                                })?
                                .map(Ok))
                        }
                        .boxed()
                    })
                    .await
            }
            .try_flatten_stream()
            .boxed()
        })?
        .stream("streamblockheaders", |req, state| {
            async move {
                let height = req.integer_param("height")?;
                state
                    .read(|state| {
                        async move {
                            Ok(state
                                .subscribe_blocks(height)
                                .await
                                .map_err(|err| Error::LeafStream {
                                    height: height as u64,
                                    reason: err.to_string(),
                                })?
                                .map(|block| Ok(block.context(StreamBlockSnafu)?.header())))
                        }
                        .boxed()
                    })
                    .await
            }
            .try_flatten_stream()
            .boxed()
        })?
        .get("getblock", |req, state| {
            async move {
                let id = match req.opt_integer_param("height")? {
                    Some(height) => ResourceId::Number(height),
                    None => ResourceId::Hash(req.blob_param("hash")?),
                };
                state.get_block(id).await.context(QueryBlockSnafu {
                    resource: id.to_string(),
                })
            }
            .boxed()
        })?
        .stream("streamblocks", |req, state| {
            async move {
                let height = req.integer_param("height")?;
                state
                    .read(|state| {
                        async move {
                            Ok(state
                                .subscribe_blocks(height)
                                .await
                                .map_err(|err| Error::BlockStream {
                                    height: height as u64,
                                    reason: err.to_string(),
                                })?
                                .map(Ok))
                        }
                        .boxed()
                    })
                    .await
            }
            .try_flatten_stream()
            .boxed()
        })?
        .get("gettransaction", |req, state| {
            async move {
                let (block, index) = match req.opt_blob_param("hash")? {
                    Some(hash) => state.get_block_with_transaction(hash).await.context(
                        QueryTransactionSnafu {
                            resource: hash.to_string(),
                        },
                    )?,
                    None => {
                        let height = req.integer_param("height")?;
                        let id = ResourceId::Number(height);
                        let block = state.get_block(id).await.context(QueryBlockSnafu {
                            resource: id.to_string(),
                        })?;
                        let i = req.integer_param("index")?;
                        let index = block.block().nth(i).context(InvalidTransactionIndexSnafu {
                            height: height as u64,
                            index: i as u64,
                        })?;
                        (block, index)
                    }
                };
                Ok(block
                    .transaction(&index)
                    // The computation of `index` above should ensure that it is a valid index.
                    .unwrap())
            }
            .boxed()
        })?
        .get("countproposals", |req, state| {
            async move {
                let proposer = req.blob_param("proposer_id")?;
                state
                    .count_proposals(&proposer)
                    .await
                    .context(QueryProposalsSnafu { proposer })
            }
            .boxed()
        })?
        .get("getproposals", |req, state| {
            async move {
                let proposer = req.blob_param("proposer_id")?;
                let limit = req.opt_integer_param("count")?;
                state
                    .get_proposals(&proposer, limit)
                    .await
                    .context(QueryProposalsSnafu { proposer })
            }
            .boxed()
        })?;
    Ok(api)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        data_source::{ExtensibleDataSource, FileSystemDataSource},
        testing::{
            consensus::{MockDataSource, MockNetwork},
            mocks::{MockNodeImpl, MockTransaction, MockTypes},
            setup_test,
        },
        Error, QueryResult,
    };
    use async_std::{sync::RwLock, task::spawn};
    use commit::Committable;
    use futures::FutureExt;
    use hotshot::types::SignatureKey;
    use hotshot_signature_key::bn254::BN254Pub;
    use portpicker::pick_unused_port;
    use std::collections::HashSet;
    use std::time::Duration;
    use surf_disco::Client;
    use tempdir::TempDir;
    use tide_disco::App;
    use toml::toml;

    /// Get the current ledger height and a list of non-empty leaf/block pairs.
    async fn get_non_empty_blocks(
        client: &Client<Error>,
    ) -> (
        u64,
        Vec<(
            LeafQueryData<MockTypes, MockNodeImpl>,
            BlockQueryData<MockTypes>,
        )>,
    ) {
        let mut blocks = vec![];
        for i in 0.. {
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
                    source:
                        super::Error::QueryBlock {
                            source: QueryError::NotFound,
                            ..
                        },
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

    async fn validate(client: &Client<Error>, height: u64) {
        // Check the consistency of every block/leaf pair. Keep track of blocks and transactions we
        // have seen so we can detect duplicates.
        let mut seen_blocks = HashSet::new();
        let mut seen_txns = HashSet::new();
        for i in 0..height {
            // Check that looking up the leaf various ways returns the correct leaf.
            let leaf: LeafQueryData<MockTypes, MockNodeImpl> =
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
            assert_eq!(leaf.block_hash(), block.hash());
            assert_eq!(block.height(), i);
            // We should be able to look up the block by hash as long as it's not a duplicate. For
            // duplicate blocks, this endpoint only returns the first one.
            if !seen_blocks.contains(&block.hash()) {
                assert_eq!(
                    block,
                    client
                        .get(&format!("block/hash/{}", block.hash()))
                        .send()
                        .await
                        .unwrap()
                );
                seen_blocks.insert(block.hash());
            }

            // Check that this block is included as a proposal by the proposer listed in the leaf.
            let proposals: Vec<LeafQueryData<MockTypes, MockNodeImpl>> = client
                .get(&format!("proposals/{}", leaf.proposer()))
                .send()
                .await
                .unwrap();
            assert!(proposals.contains(&leaf));
            // Check the `proposals/limit` and `proposals/count` features.
            assert!(
                client
                    .get::<u64>(&format!("proposals/{}/count", leaf.proposer()))
                    .send()
                    .await
                    .unwrap()
                    >= proposals.len() as u64
            );
            // For the limit queries, we just check the count. We don't know exactly which blocks to
            // expect in the response, since it returns the most recent `count` blocks which may
            // include new empty blocks committed since we started checking.
            assert_eq!(
                client
                    .get::<Vec<LeafQueryData<MockTypes, MockNodeImpl>>>(&format!(
                        "proposals/{}/limit/1",
                        leaf.proposer()
                    ))
                    .send()
                    .await
                    .unwrap()
                    .len(),
                1
            );
            assert_eq!(
                client
                    .get::<Vec<LeafQueryData<MockTypes, MockNodeImpl>>>(&format!(
                        "proposals/{}/limit/0",
                        leaf.proposer()
                    ))
                    .send()
                    .await
                    .unwrap()
                    .len(),
                0
            );

            // Check that looking up each transaction in the block various ways returns the correct
            // transaction.
            for (j, txn_from_block) in block.block().iter().enumerate() {
                let txn: TransactionQueryData<MockTypes> = client
                    .get(&format!("transaction/{}/{}", i, j))
                    .send()
                    .await
                    .unwrap();
                assert_eq!(txn.height(), i);
                assert_eq!(txn.block_hash(), block.hash());
                assert_eq!(txn.hash(), txn_from_block.commit());
                assert_eq!(txn.transaction(), txn_from_block);
                // We should be able to look up the transaction by hash as long as it's not a
                // duplicate. For duplicate transactions, this endpoint only returns the first one.
                if !seen_txns.contains(&txn.hash()) {
                    assert_eq!(
                        txn,
                        client
                            .get(&format!("transaction/hash/{}", txn.hash()))
                            .send()
                            .await
                            .unwrap()
                    );
                    seen_txns.insert(txn.hash());
                }
            }
        }
    }

    #[async_std::test]
    async fn test_api() {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<MockDataSource>::init().await;
        network.start().await;

        // Start the web server.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.data_source());
        app.register_module("availability", define_api(&Default::default()).unwrap())
            .unwrap();
        spawn(app.serve(format!("0.0.0.0:{}", port)));

        // Start a client.
        let client = Client::<Error>::new(
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
            .subscribe::<QueryResult<LeafQueryData<MockTypes, MockNodeImpl>>>()
            .await
            .unwrap();
        let blocks = client
            .socket("stream/blocks/0")
            .subscribe::<QueryResult<BlockQueryData<MockTypes>>>()
            .await
            .unwrap();
        let mut leaf_blocks = leaves.zip(blocks).enumerate();
        for nonce in 0..3 {
            let txn = MockTransaction { nonce };
            network.submit_transaction(txn).await;

            // Wait for the transaction to be finalized.
            let (i, leaf, block) = loop {
                tracing::info!("waiting for block with transaction {}", nonce);
                let (i, (leaf, block)) = leaf_blocks.next().await.unwrap();
                tracing::info!("got block {}\nLeaf: {:?}\nBlock: {:?}", i, leaf, block);
                let leaf = leaf.unwrap().unwrap();
                let block = block.unwrap().unwrap();
                assert_eq!(leaf.height() as usize, i);
                assert_eq!(leaf.block_hash(), block.hash());
                if !block.is_empty() {
                    break (i, leaf, block);
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
            validate(&client, (i + 1) as u64).await;
        }

        network.shut_down().await;
    }

    #[async_std::test]
    async fn test_extensions() {
        setup_test();

        let dir = TempDir::new("test_availability_extensions").unwrap();
        let data_source = ExtensibleDataSource::new(
            FileSystemDataSource::<MockTypes, MockNodeImpl>::create(dir.path()).unwrap(),
            0,
        );

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

        let mut api = define_api::<
            RwLock<ExtensibleDataSource<FileSystemDataSource<MockTypes, MockNodeImpl>, u64>>,
            MockTypes,
            MockNodeImpl,
        >(&Options {
            extensions: vec![extensions.into()],
            ..Default::default()
        })
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
        spawn(app.serve(format!("0.0.0.0:{}", port)));

        let client = Client::<Error>::new(
            format!("http://localhost:{}/availability", port)
                .parse()
                .unwrap(),
        );
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        assert_eq!(client.get::<u64>("ext").send().await.unwrap(), 0);
        client.post::<()>("ext/42").send().await.unwrap();
        assert_eq!(client.get::<u64>("ext").send().await.unwrap(), 42);

        // Ensure we can still access the built-in functionality.
        let (key, _) = BN254Pub::generated_from_seed_indexed([0; 32], 0);
        assert_eq!(
            client
                .get::<u64>(&format!("proposals/{}/count", key.to_bytes()))
                .send()
                .await
                .unwrap(),
            0
        );
    }
}
