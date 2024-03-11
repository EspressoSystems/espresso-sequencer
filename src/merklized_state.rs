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

use crate::api::load_api;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use clap::Args;
use derive_more::From;
use futures::FutureExt;
use hotshot_types::traits::node_implementation::NodeType;
use jf_primitives::merkle_tree::{Index, MerkleTreeScheme, ToTraversalPath};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::Snafu;
use std::fmt::Display;
use std::path::PathBuf;
use tide_disco::{api::ApiError, method::ReadState, Api, RequestError, StatusCode};
use typenum::Unsigned;

pub(crate) mod data_source;
pub use data_source::*;

#[derive(Args)]
pub struct Options {
    #[arg(
        long = "merklized-state-api-path",
        env = "HOTSHOT_MERKLIZED_STATE_API_PATH"
    )]
    pub api_path: Option<PathBuf>,

    /// Additional API specification files to merge with `status-api-path`.
    ///
    /// These optional files may contain route definitions for application-specific routes that have
    /// been added as extensions to the basic status API.
    #[arg(
        long = "status-extension",
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

// pub struct MerklizedStateType(&'static str);

// impl Display for MerklizedStateType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

pub trait MerklizedState: MerkleTreeScheme {
    type Arity: Unsigned;

    fn state_type(&self) -> &'static str;
    fn header_state_commitment_field(&self) -> &'static str;
}

pub fn define_api<State, Types: NodeType, M: MerklizedState>(
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
        + ToTraversalPath<<M as MerklizedState>::Arity>
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

                // let key =
                //     serde_json::value::from_value::<T::Index>(req.string_param("key")?.into())
                //         .map_err(internal)?;

                let key = serde_json::to_value(req.string_param("key")?).map_err(internal)?;

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
