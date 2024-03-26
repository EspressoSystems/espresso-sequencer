//! Sequencer-specific API endpoint handlers.

use super::{
    data_source::{
        SequencerDataSource, StateDataSource, StateSignatureDataSource, SubmitDataSource,
    },
    StorageState,
};
use crate::{
    block::payload::{parse_ns_payload, NamespaceProof},
    network,
    state::{BlockMerkleTree, FeeAccountProof, ValidatedState},
    NamespaceId, SeqTypes, Transaction,
};
use async_std::sync::{Arc, RwLock};
use commit::Committable;
use ethers::prelude::U256;
use futures::{try_join, FutureExt};
use hotshot_query_service::{
    availability::{self, AvailabilityDataSource, CustomSnafu, FetchBlockSnafu},
    merklized_state::{self, MerklizedState, MerklizedStateDataSource},
    node, Error,
};
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime};
use jf_primitives::merkle_tree::MerkleTreeScheme;
use serde::{Deserialize, Serialize};
use snafu::OptionExt;
use tagged_base64::TaggedBase64;
use tide_disco::{
    method::{ReadState, WriteState},
    Api, Error as _, StatusCode,
};

use versioned_binary_serialization::version::StaticVersionType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamespaceProofQueryData {
    pub proof: NamespaceProof,
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountQueryData {
    pub balance: U256,
    pub proof: FeeAccountProof,
}

impl From<(FeeAccountProof, U256)> for AccountQueryData {
    fn from((proof, balance): (FeeAccountProof, U256)) -> Self {
        Self { balance, proof }
    }
}

pub type BlocksFrontier = <BlockMerkleTree as MerkleTreeScheme>::MembershipProof;

pub(super) type AvailState<N, D, Ver> = Arc<RwLock<StorageState<N, D, Ver>>>;

pub(super) fn availability<N, D, Ver: StaticVersionType + 'static>(
    bind_version: Ver,
) -> anyhow::Result<Api<AvailState<N, D, Ver>, availability::Error, Ver>>
where
    N: network::Type,
    D: SequencerDataSource + Send + Sync + 'static,
{
    let mut options = availability::Options::default();
    let extension = toml::from_str(include_str!("../../api/availability.toml"))?;
    options.extensions.push(extension);
    let timeout = options.fetch_timeout;

    let mut api =
        availability::define_api::<AvailState<N, D, Ver>, SeqTypes, Ver>(&options, bind_version)?;

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

pub(super) fn node<N, D, Ver: StaticVersionType + 'static>(
    bind_version: Ver,
) -> anyhow::Result<Api<AvailState<N, D, Ver>, node::Error, Ver>>
where
    N: network::Type,
    D: SequencerDataSource + Send + Sync + 'static,
{
    let api = node::define_api::<AvailState<N, D, Ver>, SeqTypes, Ver>(
        &Default::default(),
        bind_version,
    )?;
    Ok(api)
}
pub(super) fn submit<N, S, Ver: StaticVersionType + 'static>() -> anyhow::Result<Api<S, Error, Ver>>
where
    N: network::Type,
    S: 'static + Send + Sync + WriteState,
    S::State: Send + Sync + SubmitDataSource<N>,
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
                .consensus()
                .submit_transaction(tx)
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
) -> anyhow::Result<Api<S, Error, Ver>>
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

pub(super) fn catchup<S, Ver: StaticVersionType + 'static>(
    _: Ver,
) -> anyhow::Result<Api<S, Error, Ver>>
where
    S: 'static + Send + Sync + ReadState,
    S::State: Send + Sync + StateDataSource,
{
    let toml = toml::from_str::<toml::Value>(include_str!("../../api/catchup.toml"))?;
    let mut api = Api::<S, Error, Ver>::new(toml)?;

    async fn get_state<S: StateDataSource>(
        req: &tide_disco::RequestParams,
        state: &S,
    ) -> Result<Arc<ValidatedState>, Error> {
        match req
            .opt_integer_param("view")
            .map_err(Error::from_request_error)?
        {
            Some(view) => state
                .get_undecided_state(ViewNumber::new(view))
                .await
                .ok_or(Error::catch_all(
                    StatusCode::NotFound,
                    format!("state not available for view {view}"),
                )),
            None => Ok(state.get_decided_state().await),
        }
    }

    api.get("account", |req, state| {
        async move {
            let state = get_state(&req, state).await?;
            let account = req
                .string_param("address")
                .map_err(Error::from_request_error)?;
            let account = account.parse().map_err(|err| {
                Error::catch_all(
                    StatusCode::BadRequest,
                    format!("malformed account {account}: {err}"),
                )
            })?;

            let (proof, balance) =
                FeeAccountProof::prove(&state.fee_merkle_tree, account).ok_or(Error::catch_all(
                    StatusCode::NotFound,
                    format!("account {account} is not in memory"),
                ))?;
            Ok(AccountQueryData { balance, proof })
        }
        .boxed()
    })?
    .get("blocks", |req, state| {
        async move {
            let state = get_state(&req, state).await?;

            // Get the frontier of the blocks Merkle tree, if we have it.
            let tree = &state.block_merkle_tree;
            let frontier: BlocksFrontier = tree
                .lookup(tree.num_leaves() - 1)
                .expect_ok()
                .map_err(|err| {
                    Error::catch_all(
                        StatusCode::NotFound,
                        format!("blocks frontier is not in memory: {err}"),
                    )
                })?
                .1;
            Ok(frontier)
        }
        .boxed()
    })?;

    Ok(api)
}

pub(super) fn merklized_state<N, D, S, Ver: StaticVersionType + 'static>(
    _: Ver,
) -> anyhow::Result<Api<AvailState<N, D, Ver>, merklized_state::Error, Ver>>
where
    N: network::Type,
    D: MerklizedStateDataSource<SeqTypes, S> + Send + Sync + 'static,
    S: MerklizedState<SeqTypes>,
    for<'a> <S::Commit as TryFrom<&'a TaggedBase64>>::Error: std::fmt::Display,
{
    let api = merklized_state::define_api::<AvailState<N, D, Ver>, SeqTypes, S, Ver>(
        &Default::default(),
    )?;
    Ok(api)
}
