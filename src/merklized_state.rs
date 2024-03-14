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

use std::{fmt::Display, path::PathBuf};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use clap::Args;
use derive_more::From;
use futures::FutureExt;
use hotshot_types::traits::node_implementation::NodeType;
use jf_primitives::merkle_tree::{Index, MerkleTreeScheme, ToTraversalPath};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::Snafu;
use tide_disco::{api::ApiError, method::ReadState, Api, RequestError, StatusCode};

use crate::api::load_api;

pub(crate) mod data_source;
pub use data_source::*;

#[derive(Args, Default)]
pub struct Options {
    #[arg(
        long = "merklized-state-api-path",
        env = "HOTSHOT_MERKLIZED_STATE_API_PATH"
    )]
    pub api_path: Option<PathBuf>,

    /// Additional API specification files to merge with `merklized-state-api-path`.
    ///
    /// These optional files may contain route definitions for application-specific routes that have
    /// been added as extensions to the basic status API.
    #[arg(
        long = "merklized-state-extension",
        env = "HOTSHOT_MERKLIZED_STATE_EXTENSIONS",
        value_delimiter = ','
    )]
    pub extensions: Vec<toml::Value>,
}

#[derive(Clone, Debug, From, Snafu, Deserialize, Serialize)]
pub enum Error {
    Request { source: RequestError },
    Internal { reason: String },
}

impl Error {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::Request { .. } => StatusCode::BadRequest,
            Self::Internal { .. } => StatusCode::InternalServerError,
        }
    }
}

fn internal<M: Display>(msg: M) -> Error {
    Error::Internal {
        reason: msg.to_string(),
    }
}

pub fn define_api<State, Types: NodeType, M: MerklizedState<Types>>(
    options: &Options,
    state: M,
) -> Result<Api<State, Error>, ApiError>
where
    State: 'static + Send + Sync + ReadState,
    <State as ReadState>::State: Send + Sync + MerklizedStateDataSource<Types>,
    Types: NodeType,
    <M as MerkleTreeScheme>::Element:
        Send + Sync + DeserializeOwned + CanonicalDeserialize + CanonicalSerialize,
    <M as MerkleTreeScheme>::Index: Index
        + Send
        + ToTraversalPath<<M as MerklizedState<Types>>::Arity>
        + DeserializeOwned
        + CanonicalDeserialize
        + CanonicalSerialize,
    <M as MerkleTreeScheme>::NodeValue: Send + Sync,
{
    let mut api = load_api::<State, Error>(
        options.api_path.as_ref(),
        include_str!("../api/state.toml"),
        options.extensions.clone(),
    )?;

    let state_type = state.state_type();
    let tree_height = state.height();
    let header_state_commitment_field = state.header_state_commitment_field();

    api.with_version("0.0.1".parse().unwrap())
        .get("get_path", move |req, state| {
            async move {
                let snapshot = if let Some(height) = req.opt_integer_param("height")? {
                    Snapshot::<Types>::Index(height)
                } else {
                    Snapshot::<Types>::Commit(req.blob_param("commit")?)
                };

                let key = req.integer_param::<_, usize>("key")?;
                tracing::warn!("req {:?}", key);
                //TODO: FIX THIS
                let key = serde_json::to_value(key).map_err(internal)?;

                state
                    .get_path::<M::Element, M::Index, M::Arity, M::NodeValue>(
                        state_type,
                        tree_height,
                        header_state_commitment_field,
                        snapshot,
                        key,
                    )
                    .await
                    .map_err(internal)
            }
            .boxed()
        })?;

    Ok(api)
}

// These tests run the `postgres` Docker image, which doesn't work on Windows.
#[cfg(all(test, not(target_os = "windows")))]
mod test {

    use std::time::Duration;

    use async_std::{stream::StreamExt, task::spawn};
    use hotshot::types::EventType;
    use jf_primitives::merkle_tree::{
        prelude::{MerklePath, Sha3Node},
        MerkleTreeScheme,
    };
    use portpicker::pick_unused_port;
    use surf_disco::Client;
    use tide_disco::App;

    use crate::testing::mocks::MockValidatedState;
    use crate::{
        merklized_state::define_api,
        testing::{
            consensus::{MockNetwork, MockSqlDataSource},
            setup_test,
        },
        Error,
    };

    #[async_std::test]
    async fn test_merklized_state_api() {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<MockSqlDataSource>::init().await;
        let mut events = network.handle().get_event_stream();

        network.start().await;

        // Start the web server.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.data_source());
        let validated_state = MockValidatedState::default();

        app.register_module(
            "state/test",
            define_api(&Default::default(), validated_state.tree).unwrap(),
        )
        .unwrap();

        spawn(app.serve(format!("0.0.0.0:{}", port)));

        let client = Client::<Error>::new(
            format!("http://localhost:{}/state/test", port)
                .parse()
                .unwrap(),
        );
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        'a: while let Some(event) = events.next().await {
            if let EventType::Decide { leaf_chain, .. } = event.event {
                for leaf in leaf_chain.iter() {
                    let state = leaf.state.clone();
                    let delta = leaf.delta.clone();

                    if let Some(delta) = delta {
                        for key in delta.0.iter() {
                            {
                                let (_, proof_from_state) =
                                    state.tree.lookup(key).expect_ok().unwrap();

                                let merkle_path_from_storage = client
                                    .get::<MerklePath<usize, usize, Sha3Node>>(&format!(
                                        "{:?}/{key}",
                                        leaf.leaf.get_height()
                                    ))
                                    .send()
                                    .await
                                    .unwrap();

                                assert_eq!(
                                    merkle_path_from_storage, proof_from_state.proof,
                                    "merkle paths don't match"
                                );

                                tracing::warn!("leaf get height {:?}", leaf.leaf.get_height());

                                if leaf.leaf.get_height() >= 20 {
                                    break 'a;
                                }
                            }
                        }
                    }
                }
            };
        }

        network.shut_down().await;
    }
}
