//! Sequencer-specific API endpoint handlers.

use super::{
    data_source::{
        CatchupDataSource, SequencerDataSource, StateSignatureDataSource, SubmitDataSource,
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

    api.post("submit", |req, state| {
        async move {
            let tx = req
                .body_auto::<Transaction, Ver>(Ver::instance())
                .map_err(Error::from_request_error)?;
            let hash = tx.commit();
            state
                .submit(tx)
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
