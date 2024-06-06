//! Sequencer-specific API endpoint handlers.

use serde::de::Error as _;
use std::{
    collections::{BTreeSet, HashMap},
    env,
};

use super::{
    data_source::{
        CatchupDataSource, HotShotConfigDataSource, SequencerDataSource, StateSignatureDataSource,
        SubmitDataSource,
    },
    StorageState,
};
use crate::{
    block::payload::{parse_ns_payload, NamespaceProof},
    network,
    persistence::SequencerPersistence,
    NamespaceId, SeqTypes, Transaction,
};
use anyhow::Result;
use async_std::sync::{Arc, RwLock};
use committable::Committable;
use futures::{try_join, FutureExt};
use hotshot_query_service::{
    availability::{self, AvailabilityDataSource, CustomSnafu, FetchBlockSnafu},
    data_source::storage::ExplorerStorage,
    explorer::{self},
    merklized_state::{
        self, MerklizedState, MerklizedStateDataSource, MerklizedStateHeightPersistence,
    },
    node, Error,
};
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime};
use serde::{Deserialize, Serialize};
use snafu::OptionExt;
use tagged_base64::TaggedBase64;
use tide_disco::{
    method::{ReadState, WriteState},
    Api, Error as _, StatusCode,
};

use vbs::version::StaticVersionType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamespaceProofQueryData {
    pub proof: NamespaceProof,
    pub transactions: Vec<Transaction>,
}

pub(super) type AvailState<N, P, D, Ver> = Arc<RwLock<StorageState<N, P, D, Ver>>>;

type AvailabilityApi<N, P, D, Ver> = Api<AvailState<N, P, D, Ver>, availability::Error, Ver>;

pub(super) fn availability<N, P, D, Ver: StaticVersionType + 'static>(
    bind_version: Ver,
) -> Result<AvailabilityApi<N, P, D, Ver>>
where
    N: network::Type,
    D: SequencerDataSource + Send + Sync + 'static,
    P: SequencerPersistence,
{
    let mut options = availability::Options::default();
    let extension = toml::from_str(include_str!("../../api/availability.toml"))?;
    options.extensions.push(extension);
    let timeout = options.fetch_timeout;

    let mut api = availability::define_api::<AvailState<N, P, D, Ver>, SeqTypes, Ver>(
        &options,
        bind_version,
    )?;

    api.get("getnamespaceproof", move |req, state| {
        async move {
            let height: usize = req.integer_param("height")?;
            let ns_id: u64 = req.integer_param("namespace")?;
            let ns_id = NamespaceId::from(ns_id);
            let (block, common) = try_join!(
                async move {
                    state
                        .get_block(height)
                        .await
                        .with_timeout(timeout)
                        .await
                        .context(FetchBlockSnafu {
                            resource: height.to_string(),
                        })
                },
                async move {
                    state
                        .get_vid_common(height)
                        .await
                        .with_timeout(timeout)
                        .await
                        .context(FetchBlockSnafu {
                            resource: height.to_string(),
                        })
                }
            )?;

            let proof = block
                .payload()
                .namespace_with_proof(
                    block.payload().get_ns_table(),
                    ns_id,
                    common.common().clone(),
                )
                .context(CustomSnafu {
                    message: format!("failed to make proof for namespace {ns_id}"),
                    status: StatusCode::NotFound,
                })?;

            let transactions = if let NamespaceProof::Existence {
                ref ns_payload_flat,
                ..
            } = proof
            {
                parse_ns_payload(ns_payload_flat, ns_id)
            } else {
                Vec::new()
            };

            Ok(NamespaceProofQueryData {
                transactions,
                proof,
            })
        }
        .boxed()
    })?;

    Ok(api)
}

type ExplorerApi<N, P, D, Ver> = Api<AvailState<N, P, D, Ver>, explorer::Error, Ver>;

pub(super) fn explorer<N, P, D, Ver: StaticVersionType + 'static>(
    bind_version: Ver,
) -> Result<ExplorerApi<N, P, D, Ver>>
where
    N: network::Type,
    D: ExplorerStorage<SeqTypes> + Send + Sync + 'static,
    P: SequencerPersistence,
{
    let api = explorer::define_api::<AvailState<N, P, D, Ver>, SeqTypes, Ver>(bind_version)?;
    Ok(api)
}

type NodeApi<N, P, D, Ver> = Api<AvailState<N, P, D, Ver>, node::Error, Ver>;

pub(super) fn node<N, P, D, Ver: StaticVersionType + 'static>(
    bind_version: Ver,
) -> Result<NodeApi<N, P, D, Ver>>
where
    N: network::Type,
    D: SequencerDataSource + Send + Sync + 'static,
    P: SequencerPersistence,
{
    let api = node::define_api::<AvailState<N, P, D, Ver>, SeqTypes, Ver>(
        &Default::default(),
        bind_version,
    )?;
    Ok(api)
}
pub(super) fn submit<N, P, S, Ver: StaticVersionType + 'static>() -> Result<Api<S, Error, Ver>>
where
    N: network::Type,
    S: 'static + Send + Sync + WriteState,
    P: SequencerPersistence,
    S::State: Send + Sync + SubmitDataSource<N, P>,
{
    let toml = toml::from_str::<toml::Value>(include_str!("../../api/submit.toml"))?;
    let mut api = Api::<S, Error, Ver>::new(toml)?;

    api.at("submit", |req, state| {
        async move {
            let tx = req
                .body_auto::<Transaction, Ver>(Ver::instance())
                .map_err(Error::from_request_error)?;

            // Transactions with namespaces that do not fit in the u32
            // cannot be included in the block.
            // TODO: This issue will be addressed in the next release.
            if tx.namespace() > NamespaceId::from(u32::MAX as u64) {
                return Err(Error::Custom {
                    message: "Transaction namespace > u32::MAX".to_string(),
                    status: StatusCode::BadRequest,
                });
            }

            let hash = tx.commit();
            state
                .read(|state| state.submit(tx).boxed())
                .await
                .map_err(|err| Error::internal(err.to_string()))?;
            Ok(hash)
        }
        .boxed()
    })?;

    Ok(api)
}

pub(super) fn state_signature<N, S, Ver: StaticVersionType + 'static>(
    _: Ver,
) -> Result<Api<S, Error, Ver>>
where
    N: network::Type,
    S: 'static + Send + Sync + ReadState,
    S::State: Send + Sync + StateSignatureDataSource<N>,
{
    let toml = toml::from_str::<toml::Value>(include_str!("../../api/state_signature.toml"))?;
    let mut api = Api::<S, Error, Ver>::new(toml)?;

    api.get("get_state_signature", |req, state| {
        async move {
            let height = req
                .integer_param("height")
                .map_err(Error::from_request_error)?;
            state
                .get_state_signature(height)
                .await
                .ok_or(tide_disco::Error::catch_all(
                    StatusCode::NotFound,
                    "Signature not found.".to_owned(),
                ))
        }
        .boxed()
    })?;

    Ok(api)
}

pub(super) fn catchup<S, Ver: StaticVersionType + 'static>(_: Ver) -> Result<Api<S, Error, Ver>>
where
    S: 'static + Send + Sync + ReadState,
    S::State: Send + Sync + CatchupDataSource,
{
    let toml = toml::from_str::<toml::Value>(include_str!("../../api/catchup.toml"))?;
    let mut api = Api::<S, Error, Ver>::new(toml)?;

    api.get("account", |req, state| {
        async move {
            let height = req
                .integer_param("height")
                .map_err(Error::from_request_error)?;
            let view = req
                .integer_param("view")
                .map_err(Error::from_request_error)?;
            let account = req
                .string_param("address")
                .map_err(Error::from_request_error)?;
            let account = account.parse().map_err(|err| {
                Error::catch_all(
                    StatusCode::BadRequest,
                    format!("malformed account {account}: {err}"),
                )
            })?;

            state
                .get_account(height, ViewNumber::new(view), account)
                .await
                .map_err(|err| Error::catch_all(StatusCode::NotFound, format!("{err:#}")))
        }
        .boxed()
    })?
    .get("blocks", |req, state| {
        async move {
            let height = req
                .integer_param("height")
                .map_err(Error::from_request_error)?;
            let view = req
                .integer_param("view")
                .map_err(Error::from_request_error)?;

            state
                .get_frontier(height, ViewNumber::new(view))
                .await
                .map_err(|err| Error::catch_all(StatusCode::NotFound, format!("{err:#}")))
        }
        .boxed()
    })?;

    Ok(api)
}

type MerklizedStateApi<N, P, D, Ver> = Api<AvailState<N, P, D, Ver>, merklized_state::Error, Ver>;
pub(super) fn merklized_state<N, P, D, S, Ver: StaticVersionType + 'static, const ARITY: usize>(
    _: Ver,
) -> Result<MerklizedStateApi<N, P, D, Ver>>
where
    N: network::Type,
    D: MerklizedStateDataSource<SeqTypes, S, ARITY>
        + Send
        + Sync
        + MerklizedStateHeightPersistence
        + 'static,
    S: MerklizedState<SeqTypes, ARITY>,
    P: SequencerPersistence,
    for<'a> <S::Commit as TryFrom<&'a TaggedBase64>>::Error: std::fmt::Display,
{
    let api = merklized_state::define_api::<AvailState<N, P, D, Ver>, SeqTypes, S, Ver, ARITY>(
        &Default::default(),
    )?;
    Ok(api)
}

pub(super) fn config<S, Ver: StaticVersionType + 'static>(_: Ver) -> Result<Api<S, Error, Ver>>
where
    S: 'static + Send + Sync + ReadState,
    S::State: Send + Sync + HotShotConfigDataSource,
{
    let toml = toml::from_str::<toml::Value>(include_str!("../../api/config.toml"))?;
    let mut api = Api::<S, Error, Ver>::new(toml)?;

    let env_variables = get_public_env_vars()
        .map_err(|err| Error::catch_all(StatusCode::InternalServerError, format!("{err:#}")))?;

    api.get("hotshot", |_, state| {
        async move { Ok(state.get_config().await) }.boxed()
    })?
    .get("env", move |_, _| {
        {
            let env_variables = env_variables.clone();
            async move { Ok(env_variables) }
        }
        .boxed()
    })?;

    Ok(api)
}

fn get_public_env_vars() -> Result<Vec<String>> {
    let toml: toml::Value = toml::from_str(include_str!("../../api/public-env-vars.toml"))?;

    let keys = toml
        .get("variables")
        .ok_or_else(|| toml::de::Error::custom("variables not found"))?
        .as_array()
        .ok_or_else(|| toml::de::Error::custom("variables is not an array"))?
        .clone()
        .into_iter()
        .map(|v| v.try_into())
        .collect::<Result<BTreeSet<String>, toml::de::Error>>()?;

    let hashmap: HashMap<String, String> = env::vars().collect();
    let mut public_env_vars: Vec<String> = Vec::new();
    for key in keys {
        let value = hashmap.get(&key).cloned().unwrap_or_default();
        public_env_vars.push(format!("{key}={value}"));
    }

    Ok(public_env_vars)
}
