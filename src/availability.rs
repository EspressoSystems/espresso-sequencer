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

use crate::{api::load_api, Block};
use clap::Args;
use derive_more::From;
use futures::{FutureExt, StreamExt, TryFutureExt};
use hotshot_types::traits::node_implementation::{NodeImplementation, NodeType};
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, Snafu};
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
pub enum Error {
    Request {
        source: RequestError,
    },
    /// The requested leaf hash is not in the database.
    #[snafu(display("the requested leaf hash {} is not in the database", hash))]
    #[from(ignore)]
    UnknownLeafHash {
        hash: String,
    },
    /// The requested leaf height is out of range for the current ledger.
    #[snafu(display(
        "the requested leaf height {} is out of range for the current ledger",
        height
    ))]
    #[from(ignore)]
    InvalidLeafHeight {
        height: u64,
    },
    /// The requested leaf exists but this query service instance does not have its data.
    #[snafu(display(
        "the requested leaf {} exists but this query service instance does not have its data",
        height
    ))]
    #[from(ignore)]
    MissingLeaf {
        height: u64,
    },
    /// The requested block hash is not in the database.
    #[snafu(display("the requested block hash {} is not in the database", hash))]
    #[from(ignore)]
    UnknownBlockHash {
        hash: String,
    },
    /// The requested block height is out of range for the current ledger.
    #[snafu(display(
        "the requested block height {} is out of range for the current ledger",
        height
    ))]
    #[from(ignore)]
    InvalidBlockHeight {
        height: u64,
    },
    /// The requested block exists but this query service instance does not have its data.
    #[snafu(display(
        "the requested block {} exists but this query service instance does not have its data",
        height
    ))]
    #[from(ignore)]
    MissingBlock {
        height: u64,
    },
    /// The requested transaction hash is not in the database.
    #[snafu(display("the requested transaction hash {} is not in the database", hash))]
    #[from(ignore)]
    UnknownTransactionHash {
        hash: String,
    },
    /// The requested transaction index is out of range for its block.
    #[snafu(display(
        "the requested transaction index {} is out of range for its block {}",
        index,
        height
    ))]
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
            Self::UnknownLeafHash { .. }
            | Self::InvalidLeafHeight { .. }
            | Self::MissingLeaf { .. }
            | Self::UnknownBlockHash { .. }
            | Self::InvalidBlockHeight { .. }
            | Self::MissingBlock { .. }
            | Self::UnknownTransactionHash { .. }
            | Self::InvalidTransactionIndex { .. } => StatusCode::NotFound,
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
        &options.extensions,
    )?;
    api.with_version("0.0.1".parse().unwrap())
        .get("getleaf", |req, state| {
            async move {
                let height = match req.opt_integer_param("height")? {
                    Some(height) => height,
                    None => {
                        let hash = req.blob_param("hash")?;
                        state
                            .get_leaf_index_by_hash(hash)
                            .context(UnknownLeafHashSnafu {
                                hash: hash.to_string(),
                            })?
                    }
                };
                state
                    .get_nth_leaf_iter(height as usize)
                    .next()
                    .context(InvalidLeafHeightSnafu { height })?
                    .context(MissingLeafSnafu { height })
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
                                .map_err(|err| Error::LeafStream {
                                    height: height as u64,
                                    reason: err.to_string(),
                                })?
                                .map(|block| Ok(block.header())))
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
                let height = match req.opt_integer_param("height")? {
                    Some(height) => height,
                    None => {
                        let hash = req.blob_param("hash")?;
                        state
                            .get_block_index_by_hash(hash)
                            .context(UnknownBlockHashSnafu {
                                hash: hash.to_string(),
                            })?
                    }
                };
                get_block(state, height)
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
                    Some(hash) => {
                        let (height, index) = state.get_txn_index_by_hash(hash).context(
                            UnknownTransactionHashSnafu {
                                hash: hash.to_string(),
                            },
                        )?;
                        let block = get_block(state, height)?;
                        (block, index)
                    }
                    None => {
                        let height = req.integer_param("height")?;
                        let block = get_block(state, height)?;
                        let i = req.integer_param("index")?;
                        let index = block.block().nth(i).context(InvalidTransactionIndexSnafu {
                            height,
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
                Ok(state.get_block_ids_by_proposer_id(&proposer).len())
            }
            .boxed()
        })?
        .get("getproposals", |req, state| {
            async move {
                let proposer = req.blob_param("proposer_id")?;
                let all_ids = state.get_block_ids_by_proposer_id(&proposer);
                let start_from = match req.opt_integer_param("count")? {
                    Some(count) => all_ids.len().saturating_sub(count),
                    None => 0,
                };
                all_ids
                    .into_iter()
                    .skip(start_from)
                    .map(|height| {
                        state
                            .get_nth_leaf_iter(height as usize)
                            .next()
                            .context(InvalidLeafHeightSnafu { height })?
                            .context(MissingLeafSnafu { height })
                    })
                    .collect::<Result<Vec<_>, _>>()
            }
            .boxed()
        })?;
    Ok(api)
}

fn get_block<State, Types, I>(state: &State, height: u64) -> Result<BlockQueryData<Types>, Error>
where
    State: AvailabilityDataSource<Types, I>,
    Block<Types>: QueryableBlock,
    Types: NodeType,
    I: NodeImplementation<Types>,
{
    state
        .get_nth_block_iter(height as usize)
        .next()
        .context(InvalidBlockHeightSnafu { height })?
        .context(MissingBlockSnafu { height })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        data_source::QueryData,
        testing::{
            consensus::MockNetwork,
            mocks::{MockNodeImpl, MockTransaction, MockTypes},
            setup_test,
        },
        Error,
    };
    use async_std::{sync::RwLock, task::spawn};
    use commit::Committable;
    use futures::FutureExt;
    use hotshot::types::{ed25519::Ed25519Pub, SignatureKey};
    use portpicker::pick_unused_port;
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
                    source: super::Error::InvalidBlockHeight { height },
                }) if height == i => {
                    tracing::info!(
                        "found end of ledger at height {}, non-empty blocks are {:?}",
                        i,
                        blocks
                    );
                    return (i, blocks);
                }
                Err(err) => panic!("unexpected error {}", err),
            }
        }
        unreachable!()
    }

    async fn validate(client: &Client<Error>, height: u64) {
        // Check the consistency of every block/leaf pair.
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
            assert_eq!(
                block,
                client
                    .get(&format!("block/hash/{}", block.hash()))
                    .send()
                    .await
                    .unwrap()
            );

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
                assert_eq!(
                    txn,
                    client
                        .get(&format!("transaction/hash/{}", txn_from_block.commit()))
                        .send()
                        .await
                        .unwrap()
                );
            }
        }
    }

    #[async_std::test]
    async fn test_api() {
        setup_test();

        // Create the consensus network.
        let network = MockNetwork::init(()).await;
        let hotshot = network.handle();
        network.start().await;

        // Start the web server.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.query_data());
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
        let leaves = client.socket("stream/leaves/0").subscribe().await.unwrap();
        let blocks = client.socket("stream/blocks/0").subscribe().await.unwrap();
        let mut leaf_blocks = leaves.zip(blocks).enumerate();
        for nonce in 0..3 {
            let txn = MockTransaction { nonce };
            hotshot.submit_transaction(txn).await.unwrap();

            // Wait for the transaction to be finalized.
            let (i, leaf, block) = loop {
                tracing::info!("waiting for block with transaction {}", nonce);
                let (i, (leaf, block)) = leaf_blocks.next().await.unwrap();
                tracing::info!("got block {}\nLeaf: {:?}\nBlock: {:?}", i, leaf, block);
                let leaf: LeafQueryData<MockTypes, MockNodeImpl> = leaf.unwrap();
                let block: BlockQueryData<MockTypes> = block.unwrap();
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
        let query_data = QueryData::<MockTypes, MockNodeImpl, u64>::create(dir.path(), 0).unwrap();

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
            define_api::<RwLock<QueryData<MockTypes, MockNodeImpl, u64>>, MockTypes, MockNodeImpl>(
                &Options {
                    extensions: vec![extensions],
                    ..Default::default()
                },
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

        let mut app = App::<_, Error>::with_state(RwLock::new(query_data));
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
        let (key, _) = Ed25519Pub::generated_from_seed_indexed([0; 32], 0);
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
