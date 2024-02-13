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

//! A node's view of a HotShot chain
//!
//! The node API provides a subjective view of the HotShot blockchain, from the perspective of
//! one particular node. It provides access to information that the
//! [availability](crate::availability) API does not, because this information depends on the
//! perspective of the node observing it, and may be subject to eventual consistency. For example,
//! `/node/block-height` and `/node/proposals/:proposer_id/count` may both return smaller counts
//! than expected, if the node being queried is not fully synced with the entire history of the
//! chain. However, the node will _eventually_ sync and return the expected counts.

use crate::{api::load_api, QueryError, SignatureKey};
use clap::Args;
use derive_more::From;
use futures::FutureExt;
use hotshot_types::traits::node_implementation::NodeType;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::fmt::Display;
use std::path::PathBuf;
use tide_disco::{api::ApiError, method::ReadState, Api, RequestError, RequestParams, StatusCode};

pub(crate) mod data_source;
pub(crate) mod query_data;
pub use data_source::*;
pub use query_data::*;

#[derive(Args, Default)]
pub struct Options {
    #[arg(long = "node-api-path", env = "HOTSHOT_NODE_API_PATH")]
    pub api_path: Option<PathBuf>,

    /// Additional API specification files to merge with `node-api-path`.
    ///
    /// These optional files may contain route definitions for application-specific routes that have
    /// been added as extensions to the basic node API.
    #[arg(
        long = "node-extension",
        env = "HOTSHOT_NODE_EXTENSIONS",
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
    #[snafu(display("{source}"))]
    Query {
        source: QueryError,
    },
    #[snafu(display("error fetching proposals by {proposer}: {source}"))]
    #[from(ignore)]
    QueryProposals {
        source: QueryError,
        proposer: String,
    },
    #[snafu(display("malformed signature key"))]
    #[from(ignore)]
    InvalidSignatureKey,
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
            Self::Request { .. } | Self::InvalidSignatureKey => StatusCode::BadRequest,
            Self::Query { source, .. } | Self::QueryProposals { source, .. } => source.status(),
            Self::Custom { status, .. } => *status,
        }
    }
}

pub fn define_api<State, Types: NodeType>(options: &Options) -> Result<Api<State, Error>, ApiError>
where
    State: 'static + Send + Sync + ReadState,
    <State as ReadState>::State: Send + Sync + NodeDataSource<Types>,
{
    let mut api = load_api::<State, Error>(
        options.api_path.as_ref(),
        include_str!("../api/node.toml"),
        options.extensions.clone(),
    )?;
    api.with_version("0.0.1".parse().unwrap())
        .get("block_height", |_req, state| {
            async move { state.block_height().await.context(QuerySnafu) }.boxed()
        })?
        .get("count_proposals", |req, state| {
            async move {
                let proposer = proposer_param::<Types>(&req, "proposer_id")?;
                state
                    .count_proposals(&proposer)
                    .await
                    .context(QueryProposalsSnafu {
                        proposer: proposer.to_string(),
                    })
            }
            .boxed()
        })?
        .get("get_proposals", |req, state| {
            async move {
                let proposer = proposer_param::<Types>(&req, "proposer_id")?;
                let limit = req.opt_integer_param("count")?;
                state
                    .get_proposals(&proposer, limit)
                    .await
                    .context(QueryProposalsSnafu {
                        proposer: proposer.to_string(),
                    })
            }
            .boxed()
        })?
        .get("count_transactions", |_req, state| {
            async move { Ok(state.count_transactions().await?) }.boxed()
        })?
        .get("payload_size", |_req, state| {
            async move { Ok(state.payload_size().await?) }.boxed()
        })?
        .get("sync_status", |_req, state| {
            async move { Ok(state.sync_status().await?) }.boxed()
        })?;
    Ok(api)
}

fn proposer_param<Types: NodeType>(
    req: &RequestParams,
    param: &str,
) -> Result<SignatureKey<Types>, Error> {
    let encoded = req.tagged_base64_param(param)?;
    encoded.try_into().map_err(|_| Error::InvalidSignatureKey)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        data_source::ExtensibleDataSource,
        testing::{
            consensus::{MockDataSource, MockNetwork},
            mocks::MockTypes,
            setup_test,
        },
        Error,
    };
    use async_std::{
        sync::RwLock,
        task::{sleep, spawn},
    };
    use futures::FutureExt;
    use hotshot::types::SignatureKey;
    use hotshot_types::signature_key::BLSPubKey;
    use portpicker::pick_unused_port;
    use std::time::Duration;
    use surf_disco::Client;
    use tempdir::TempDir;
    use tide_disco::App;
    use toml::toml;

    #[async_std::test]
    async fn test_api() {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<MockDataSource>::init().await;
        network.start().await;

        // Start the web server.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.data_source());
        app.register_module("node", define_api(&Default::default()).unwrap())
            .unwrap();
        spawn(app.serve(format!("0.0.0.0:{}", port)));

        // Start a client.
        let client =
            Client::<Error>::new(format!("http://localhost:{}/node", port).parse().unwrap());
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        // Wait for each node to propose a block.
        while client.get::<usize>("block-height").send().await.unwrap() < network.num_nodes() + 1 {
            sleep(Duration::from_secs(1)).await;
        }

        // Check proposals for node 0.
        let proposals: Vec<LeafQueryData<MockTypes>> = client
            .get(&format!("proposals/{}", network.proposer(0)))
            .send()
            .await
            .unwrap();
        assert!(!proposals.is_empty());
        for proposal in &proposals {
            assert_eq!(proposal.proposer(), network.proposer(0));
        }
        // Check the `proposals/limit` and `proposals/count` features.
        assert!(
            client
                .get::<u64>(&format!("proposals/{}/count", network.proposer(0)))
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
                .get::<Vec<LeafQueryData<MockTypes>>>(&format!(
                    "proposals/{}/limit/1",
                    network.proposer(0)
                ))
                .send()
                .await
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            client
                .get::<Vec<LeafQueryData<MockTypes>>>(&format!(
                    "proposals/{}/limit/0",
                    network.proposer(0)
                ))
                .send()
                .await
                .unwrap()
                .len(),
            0
        );

        // We test these counters with non-trivial values in `data_source.rs`, here we just want to
        // make sure the API handlers are working, so a response of 0 is fine.
        assert_eq!(
            client
                .get::<u64>("transactions/count")
                .send()
                .await
                .unwrap(),
            0
        );
        assert_eq!(
            client
                .get::<u64>("payloads/total-size")
                .send()
                .await
                .unwrap(),
            0
        );

        // In this simple test, the node should be fully synchronized.
        let sync_status = client
            .get::<SyncStatus>("sync-status")
            .send()
            .await
            .unwrap();
        assert_eq!(sync_status.missing_blocks, 0);
        assert_eq!(sync_status.missing_leaves, 0);

        network.shut_down().await;
    }

    #[async_std::test]
    async fn test_extensions() {
        setup_test();

        let dir = TempDir::new("test_node_extensions").unwrap();
        let data_source = ExtensibleDataSource::new(
            MockDataSource::create(dir.path(), Default::default())
                .await
                .unwrap(),
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

        let mut api =
            define_api::<RwLock<ExtensibleDataSource<MockDataSource, u64>>, MockTypes>(&Options {
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
        app.register_module("node", api).unwrap();

        let port = pick_unused_port().unwrap();
        spawn(app.serve(format!("0.0.0.0:{}", port)));

        let client =
            Client::<Error>::new(format!("http://localhost:{}/node", port).parse().unwrap());
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        assert_eq!(client.get::<u64>("ext").send().await.unwrap(), 0);
        client.post::<()>("ext/42").send().await.unwrap();
        assert_eq!(client.get::<u64>("ext").send().await.unwrap(), 42);

        // Ensure we can still access the built-in functionality.
        let (key, _) = BLSPubKey::generated_from_seed_indexed([0; 32], 0);
        assert_eq!(
            client
                .get::<u64>(&format!("proposals/{key}/count"))
                .send()
                .await
                .unwrap(),
            0
        );
    }
}
