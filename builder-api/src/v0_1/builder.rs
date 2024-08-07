// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::path::PathBuf;

use clap::Args;
use committable::Committable;
use derive_more::From;
use futures::FutureExt;
use hotshot_types::{traits::node_implementation::NodeType, utils::BuilderCommitment};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use tagged_base64::TaggedBase64;
use tide_disco::{api::ApiError, method::ReadState, Api, RequestError, RequestParams, StatusCode};
use vbs::version::StaticVersionType;

use super::{
    data_source::{AcceptsTxnSubmits, BuilderDataSource},
    Version,
};
use crate::api::load_api;

#[derive(Args, Default)]
pub struct Options {
    #[arg(long = "builder-api-path", env = "HOTSHOT_BUILDER_API_PATH")]
    pub api_path: Option<PathBuf>,

    /// Additional API specification files to merge with `builder-api-path`.
    ///
    /// These optional files may contain route definitions for application-specific routes that have
    /// been added as extensions to the basic builder API.
    #[arg(
        long = "builder-extension",
        env = "HOTSHOT_BUILDER_EXTENSIONS",
        value_delimiter = ','
    )]
    pub extensions: Vec<toml::Value>,
}

#[derive(Clone, Debug, Snafu, Deserialize, Serialize)]
#[snafu(visibility(pub))]
pub enum BuildError {
    /// The requested resource does not exist or is not known to this builder service.
    NotFound,
    /// The requested resource exists but is not currently available.
    Missing,
    /// There was an error while trying to fetch the requested resource.
    #[snafu(display("Failed to fetch requested resource: {message}"))]
    Error { message: String },
}

#[derive(Clone, Debug, From, Snafu, Deserialize, Serialize)]
#[snafu(visibility(pub))]
pub enum Error {
    Request {
        source: RequestError,
    },
    #[snafu(display("error building block from {resource}: {source}"))]
    #[from(ignore)]
    BlockAvailable {
        source: BuildError,
        resource: String,
    },
    #[snafu(display("error claiming block {resource}: {source}"))]
    #[from(ignore)]
    BlockClaim {
        source: BuildError,
        resource: String,
    },
    #[snafu(display("error unpacking transaction: {source}"))]
    #[from(ignore)]
    TxnUnpack {
        source: RequestError,
    },
    #[snafu(display("error submitting transaction: {source}"))]
    #[from(ignore)]
    TxnSubmit {
        source: BuildError,
    },
    #[snafu(display("error getting builder address: {source}"))]
    #[from(ignore)]
    BuilderAddress {
        source: BuildError,
    },
    Custom {
        message: String,
        status: StatusCode,
    },
}

impl tide_disco::error::Error for Error {
    fn catch_all(status: StatusCode, msg: String) -> Self {
        Error::Custom {
            message: msg,
            status,
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            Error::Request { .. } => StatusCode::BAD_REQUEST,
            Error::BlockAvailable { source, .. } | Error::BlockClaim { source, .. } => match source
            {
                BuildError::NotFound => StatusCode::NOT_FOUND,
                BuildError::Missing => StatusCode::NOT_FOUND,
                BuildError::Error { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Error::TxnUnpack { .. } => StatusCode::BAD_REQUEST,
            Error::TxnSubmit { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Custom { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BuilderAddress { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub(crate) fn try_extract_param<T: for<'a> TryFrom<&'a TaggedBase64>>(
    params: &RequestParams,
    param_name: &str,
) -> Result<T, Error> {
    params
        .param(param_name)?
        .as_tagged_base64()?
        .try_into()
        .map_err(|_| Error::Custom {
            message: format!("Invalid {param_name}"),
            status: StatusCode::UNPROCESSABLE_ENTITY,
        })
}

pub fn define_api<State, Types: NodeType>(
    options: &Options,
) -> Result<Api<State, Error, Version>, ApiError>
where
    State: 'static + Send + Sync + ReadState,
    <State as ReadState>::State: Send + Sync + BuilderDataSource<Types>,
{
    let mut api = load_api::<State, Error, Version>(
        options.api_path.as_ref(),
        include_str!("../../api/v0_1/builder.toml"),
        options.extensions.clone(),
    )?;
    api.with_version("0.1.0".parse().unwrap())
        .get("available_blocks", |req, state| {
            async move {
                let hash = req.blob_param("parent_hash")?;
                let view_number = req.integer_param("view_number")?;
                let signature = try_extract_param(&req, "signature")?;
                let sender = try_extract_param(&req, "sender")?;
                state
                    .available_blocks(&hash, view_number, sender, &signature)
                    .await
                    .context(BlockAvailableSnafu {
                        resource: hash.to_string(),
                    })
            }
            .boxed()
        })?
        .get("claim_block", |req, state| {
            async move {
                let block_hash: BuilderCommitment = req.blob_param("block_hash")?;
                let view_number = req.integer_param("view_number")?;
                let signature = try_extract_param(&req, "signature")?;
                let sender = try_extract_param(&req, "sender")?;
                state
                    .claim_block(&block_hash, view_number, sender, &signature)
                    .await
                    .context(BlockClaimSnafu {
                        resource: block_hash.to_string(),
                    })
            }
            .boxed()
        })?
        .get("claim_header_input", |req, state| {
            async move {
                let block_hash: BuilderCommitment = req.blob_param("block_hash")?;
                let view_number = req.integer_param("view_number")?;
                let signature = try_extract_param(&req, "signature")?;
                let sender = try_extract_param(&req, "sender")?;
                state
                    .claim_block_header_input(&block_hash, view_number, sender, &signature)
                    .await
                    .context(BlockClaimSnafu {
                        resource: block_hash.to_string(),
                    })
            }
            .boxed()
        })?
        .get("builder_address", |_req, state| {
            async move { state.builder_address().await.context(BuilderAddressSnafu) }.boxed()
        })?;
    Ok(api)
}

pub fn submit_api<State, Types: NodeType, Ver: StaticVersionType + 'static>(
    options: &Options,
) -> Result<Api<State, Error, Ver>, ApiError>
where
    State: 'static + Send + Sync + AcceptsTxnSubmits<Types>,
{
    let mut api = load_api::<State, Error, Ver>(
        options.api_path.as_ref(),
        include_str!("../../api/v0_1/submit.toml"),
        options.extensions.clone(),
    )?;
    api.with_version("0.0.1".parse().unwrap())
        .at("submit_txn", |req: RequestParams, state| {
            async move {
                let tx = req
                    .body_auto::<<Types as NodeType>::Transaction, Ver>(Ver::instance())
                    .context(TxnUnpackSnafu)?;
                let hash = tx.commit();
                state.submit_txns(vec![tx]).await.context(TxnSubmitSnafu)?;
                Ok(hash)
            }
            .boxed()
        })?
        .at("submit_batch", |req: RequestParams, state| {
            async move {
                let txns = req
                    .body_auto::<Vec<<Types as NodeType>::Transaction>, Ver>(Ver::instance())
                    .context(TxnUnpackSnafu)?;
                let hashes = txns.iter().map(|tx| tx.commit()).collect::<Vec<_>>();
                state.submit_txns(txns).await.context(TxnSubmitSnafu)?;
                Ok(hashes)
            }
            .boxed()
        })?;
    Ok(api)
}
