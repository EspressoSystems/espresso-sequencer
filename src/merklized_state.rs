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

//! Api for querying merklized state
//!
//! The state API provides an interface for serving queries against arbitrarily old snapshots of the state.
//! This allows a full Merkle tree to be reconstructed from storage.
//! If any parent state is missing then the partial snapshot can not be queried.
use std::{fmt::Display, path::PathBuf};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use clap::Args;
use derive_more::From;
use futures::FutureExt;
use hotshot_types::traits::node_implementation::NodeType;
use jf_primitives::merkle_tree::{Index, MerkleTreeScheme, ToTraversalPath};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::Snafu;
use tagged_base64::TaggedBase64;
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
    <State as ReadState>::State: Send + Sync + MerklizedStateDataSource<Types, M>,
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
    M::Commitment: for<'a> TryFrom<&'a TaggedBase64> + Display,
    for<'a> <M::Commitment as TryFrom<&'a TaggedBase64>>::Error: Display,
{
    let mut api = load_api::<State, Error>(
        options.api_path.as_ref(),
        include_str!("../api/state.toml"),
        options.extensions.clone(),
    )?;

    // Determine the type of state for which the api is initialized
    let state_type = state.state_type();
    let tree_height = state.height();
    // Identify the specific field in the header storing the Merkle Tree root commitment
    // Useful for accessing the correct commitment among multiple state roots
    let header_state_commitment_field = state.header_state_commitment_field();

    api.with_version("0.0.1".parse().unwrap())
        .get("get_path", move |req, state| {
            async move {
                // Determine the snapshot type based on request parameters, either index or commit
                let snapshot = if let Some(height) = req.opt_integer_param("height")? {
                    Snapshot::Index(height)
                } else {
                    Snapshot::Commit(req.blob_param("commit")?)
                };

                let key = req.string_param("key")?;

                state
                    .get_path(
                        state_type,
                        tree_height,
                        header_state_commitment_field,
                        snapshot,
                        key.to_string(),
                    )
                    .await
                    .map_err(internal)
            }
            .boxed()
        })?;

    Ok(api)
}
