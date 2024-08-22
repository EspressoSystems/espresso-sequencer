//! Sequencer-specific API endpoint handlers.

use std::{
    collections::{BTreeSet, HashMap},
    env,
};

use anyhow::Result;
use async_std::sync::{Arc, RwLock};
use committable::Committable;
use espresso_types::{NamespaceId, NsProof, PubKey, Transaction};
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
use hotshot_types::{
    data::ViewNumber,
    traits::{
        network::ConnectedNetwork,
        node_implementation::{ConsensusTime, Versions},
    },
};
use serde::{de::Error as _, Deserialize, Serialize};
use snafu::OptionExt;
use tagged_base64::TaggedBase64;
use tide_disco::{
    method::{ReadState, WriteState},
    Api, Error as _, StatusCode,
};
use vbs::version::StaticVersionType;

use super::{
    data_source::{
        CatchupDataSource, HotShotConfigDataSource, SequencerDataSource, StateSignatureDataSource,
        SubmitDataSource,
    },
    StorageState,
};
use crate::{SeqTypes, SequencerApiVersion, SequencerPersistence};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamespaceProofQueryData {
    pub proof: Option<NsProof>,
    pub transactions: Vec<Transaction>,
}

pub(super) type AvailState<N, P, D, ApiVer> = Arc<RwLock<StorageState<N, P, D, ApiVer>>>;

type AvailabilityApi<N, P, D, V, ApiVer> = Api<AvailState<N, P, D, V>, availability::Error, ApiVer>;

// TODO (abdul): replace snafu with `this_error` in  hotshot query service
// Snafu has been replaced by `this_error` everywhere.
// However, the query service still uses snafu
pub(super) fn availability<N, P, D, V: Versions>(
) -> Result<AvailabilityApi<N, P, D, V, SequencerApiVersion>>
where
    N: ConnectedNetwork<PubKey>,
    D: SequencerDataSource + Send + Sync + 'static,
    P: SequencerPersistence,
{
    let mut options = availability::Options::default();
    let extension = toml::from_str(include_str!("../../api/availability.toml"))?;
    options.extensions.push(extension);
    let timeout = options.fetch_timeout;

    let mut api = availability::define_api::<AvailState<N, P, D, _>, SeqTypes, _>(
        &options,
        SequencerApiVersion::instance(),
    )?;

    api.get("getnamespaceproof", move |req, state| {
        async move {
            let height: usize = req.integer_param("height")?;
            let ns_id = NamespaceId::from(req.integer_param::<_, u32>("namespace")?);
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

            if let Some(ns_index) = block.payload().ns_table().find_ns_id(&ns_id) {
                let proof = NsProof::new(block.payload(), &ns_index, common.common()).context(
                    CustomSnafu {
                        message: format!("failed to make proof for namespace {ns_id}"),
                        status: StatusCode::NOT_FOUND,
                    },
                )?;

                Ok(NamespaceProofQueryData {
                    transactions: proof.export_all_txs(&ns_id),
                    proof: Some(proof),
                })
            } else {
                // ns_id not found in ns_table
                Ok(NamespaceProofQueryData {
                    proof: None,
                    transactions: Vec::new(),
                })
            }
        }
        .boxed()
    })?;

    Ok(api)
}

type ExplorerApi<N, P, D, V, ApiVer> = Api<AvailState<N, P, D, V>, explorer::Error, ApiVer>;

pub(super) fn explorer<N, P, D, V: Versions>(
) -> Result<ExplorerApi<N, P, D, V, SequencerApiVersion>>
where
    N: ConnectedNetwork<PubKey>,
    D: ExplorerStorage<SeqTypes> + Send + Sync + 'static,
    P: SequencerPersistence,
{
    let api = explorer::define_api::<AvailState<N, P, D, V>, SeqTypes, _>(
        SequencerApiVersion::instance(),
    )?;
    Ok(api)
}

type NodeApi<N, P, D, V, ApiVer> = Api<AvailState<N, P, D, V>, node::Error, ApiVer>;

pub(super) fn node<N, P, D, V: Versions>() -> Result<NodeApi<N, P, D, V, SequencerApiVersion>>
where
    N: ConnectedNetwork<PubKey>,
    D: SequencerDataSource + Send + Sync + 'static,
    P: SequencerPersistence,
{
    let api = node::define_api::<AvailState<N, P, D, V>, SeqTypes, _>(
        &Default::default(),
        SequencerApiVersion::instance(),
    )?;
    Ok(api)
}
pub(super) fn submit<N, P, S, ApiVer: StaticVersionType + 'static>() -> Result<Api<S, Error, ApiVer>>
where
    N: ConnectedNetwork<PubKey>,
    S: 'static + Send + Sync + WriteState,
    P: SequencerPersistence,
    S::State: Send + Sync + SubmitDataSource<N, P>,
{
    let toml = toml::from_str::<toml::Value>(include_str!("../../api/submit.toml"))?;
    let mut api = Api::<S, Error, ApiVer>::new(toml)?;

    api.at("submit", |req, state| {
        async move {
            let tx = req
                .body_auto::<Transaction, ApiVer>(ApiVer::instance())
                .map_err(Error::from_request_error)?;

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

pub(super) fn state_signature<N, S, ApiVer: StaticVersionType + 'static>(
    _: ApiVer,
) -> Result<Api<S, Error, ApiVer>>
where
    N: ConnectedNetwork<PubKey>,
    S: 'static + Send + Sync + ReadState,
    S::State: Send + Sync + StateSignatureDataSource<N>,
{
    let toml = toml::from_str::<toml::Value>(include_str!("../../api/state_signature.toml"))?;
    let mut api = Api::<S, Error, ApiVer>::new(toml)?;

    api.get("get_state_signature", |req, state| {
        async move {
            let height = req
                .integer_param("height")
                .map_err(Error::from_request_error)?;
            state
                .get_state_signature(height)
                .await
                .ok_or(tide_disco::Error::catch_all(
                    StatusCode::NOT_FOUND,
                    "Signature not found.".to_owned(),
                ))
        }
        .boxed()
    })?;

    Ok(api)
}

pub(super) fn catchup<S, ApiVer: StaticVersionType + 'static>(
    _: ApiVer,
) -> Result<Api<S, Error, ApiVer>>
where
    S: 'static + Send + Sync + ReadState,
    S::State: Send + Sync + CatchupDataSource,
{
    let toml = toml::from_str::<toml::Value>(include_str!("../../api/catchup.toml"))?;
    let mut api = Api::<S, Error, ApiVer>::new(toml)?;

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
                    StatusCode::BAD_REQUEST,
                    format!("malformed account {account}: {err}"),
                )
            })?;

            state
                .get_account(height, ViewNumber::new(view), account)
                .await
                .map_err(|err| Error::catch_all(StatusCode::NOT_FOUND, format!("{err:#}")))
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
                .map_err(|err| Error::catch_all(StatusCode::NOT_FOUND, format!("{err:#}")))
        }
        .boxed()
    })?
    .get("chainconfig", |req, state| {
        async move {
            let commitment = req
                .blob_param("commitment")
                .map_err(Error::from_request_error)?;

            state
                .get_chain_config(commitment)
                .await
                .map_err(|err| Error::catch_all(StatusCode::NOT_FOUND, format!("{err:#}")))
        }
        .boxed()
    })?;

    Ok(api)
}

type MerklizedStateApi<N, P, D, V, ApiVer> =
    Api<AvailState<N, P, D, V>, merklized_state::Error, ApiVer>;
pub(super) fn merklized_state<N, P, D, S, V: Versions, const ARITY: usize>(
) -> Result<MerklizedStateApi<N, P, D, V, SequencerApiVersion>>
where
    N: ConnectedNetwork<PubKey>,
    D: MerklizedStateDataSource<SeqTypes, S, ARITY>
        + Send
        + Sync
        + MerklizedStateHeightPersistence
        + 'static,
    S: MerklizedState<SeqTypes, ARITY>,
    P: SequencerPersistence,
    for<'a> <S::Commit as TryFrom<&'a TaggedBase64>>::Error: std::fmt::Display,
{
    let api = merklized_state::define_api::<
        AvailState<N, P, D, V>,
        SeqTypes,
        S,
        SequencerApiVersion,
        ARITY,
    >(&Default::default())?;
    Ok(api)
}

pub(super) fn config<S, ApiVer: StaticVersionType + 'static>(
    _: ApiVer,
) -> Result<Api<S, Error, ApiVer>>
where
    S: 'static + Send + Sync + ReadState,
    S::State: Send + Sync + HotShotConfigDataSource,
{
    let toml = toml::from_str::<toml::Value>(include_str!("../../api/config.toml"))?;
    let mut api = Api::<S, Error, ApiVer>::new(toml)?;

    let env_variables = get_public_env_vars()
        .map_err(|err| Error::catch_all(StatusCode::INTERNAL_SERVER_ERROR, format!("{err:#}")))?;

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
