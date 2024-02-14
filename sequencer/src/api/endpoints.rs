//! Sequencer-specific API endpoint handlers.

use super::{
    data_source::{
        SequencerDataSource, StateDataSource, StateSignatureDataSource, SubmitDataSource,
    },
    AppState,
};
use crate::{
    network,
    state::{BlockMerkleTree, FeeAccountProof, ValidatedState},
    Header, SeqTypes, Transaction, VmId,
};
use ark_bls12_381::Bls12_381;
use async_std::sync::{Arc, RwLock};
use commit::Committable;
use ethers::prelude::U256;
use futures::FutureExt;
use hotshot_query_service::{
    availability::{self, AvailabilityDataSource, BlockHash, FetchBlockSnafu},
    Error,
};
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime};
use jf_primitives::{
    merkle_tree::MerkleTreeScheme,
    vid::{advz::Advz, VidScheme},
};
use serde::{Deserialize, Serialize};
use tide_disco::{
    method::{ReadState, WriteState},
    Api, Error as _, StatusCode,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamespaceProofQueryData {
    pub proof: crate::block::payload::NamespaceProof<Advz<Bls12_381, sha2::Sha256>>,
    pub header: Header,
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeWindowQueryData {
    pub window: Vec<Header>,
    pub prev: Option<Header>,
    pub next: Option<Header>,
}

impl TimeWindowQueryData {
    /// The block height of the block that starts the window.
    ///
    /// If the window is empty, this is the height of the block that ends the window.
    pub fn from(&self) -> Option<u64> {
        self.window
            .first()
            .or(self.next.as_ref())
            .map(|header| header.height)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountQueryData {
    pub balance: U256,
    pub proof: FeeAccountProof,
}

pub type BlocksFrontier = <BlockMerkleTree as MerkleTreeScheme>::MembershipProof;

pub(super) type AvailState<N, D> = Arc<RwLock<AppState<N, D>>>;

pub(super) fn availability<N, D>() -> anyhow::Result<Api<AvailState<N, D>, availability::Error>>
where
    N: network::Type,
    D: SequencerDataSource + Send + Sync + 'static,
{
    let mut options = availability::Options::default();
    let extension = toml::from_str(include_str!("../../api/availability.toml"))?;
    options.extensions.push(extension);
    let mut api = availability::define_api::<AvailState<N, D>, SeqTypes>(&options)?;

    api.get("getnamespaceproof", |req, state| {
        async move {
            let height: usize = req.integer_param("height")?;
            let ns_id = VmId(req.integer_param("namespace")?);
            let block = state.get_block(height).await.context(FetchBlockSnafu {
                resource: height.to_string(),
            })?;

            // TODO fake VidScheme construction
            // https://github.com/EspressoSystems/espresso-sequencer/issues/1047
            // TODO do not disperse here!
            // https://github.com/EspressoSystems/espresso-sequencer/issues/1093
            let vid = crate::block::payload::test_vid_factory();
            use hotshot::traits::BlockPayload;
            let disperse_data = vid
                .disperse(block.payload().encode().unwrap().collect::<Vec<u8>>())
                .unwrap();

            let proof = block
                .payload()
                .namespace_with_proof(
                    block.payload().get_ns_table(),
                    ns_id,
                    &vid,
                    disperse_data.common,
                )
                .unwrap();

            // use NamespaceProof::verify to get a Vec<Transactions> for this namespace
            let transactions = proof
                .verify(&vid, &disperse_data.commit, block.payload().get_ns_table())
                .unwrap()
                .0;

            Ok(NamespaceProofQueryData {
                transactions,
                proof,
                header: block.header().clone(),
            })
        }
        .boxed()
    })?
    .get("gettimestampwindow", |req, state| {
        async move {
            let end = req.integer_param("end")?;
            let res = if let Some(height) = req.opt_integer_param("height")? {
                state.inner().window_from::<usize>(height, end).await
            } else if let Some(hash) = req.opt_blob_param("hash")? {
                state
                    .inner()
                    .window_from::<BlockHash<SeqTypes>>(hash, end)
                    .await
            } else {
                let start: u64 = req.integer_param("start")?;
                state.inner().window(start, end).await
            };
            res.map_err(|err| availability::Error::Custom {
                message: err.to_string(),
                status: err.status(),
            })
        }
        .boxed()
    })?;

    Ok(api)
}

pub(super) fn submit<N, S>() -> anyhow::Result<Api<S, Error>>
where
    N: network::Type,
    S: 'static + Send + Sync + WriteState,
    S::State: Send + Sync + SubmitDataSource<N>,
{
    let toml = toml::from_str::<toml::Value>(include_str!("../../api/submit.toml"))?;
    let mut api = Api::<S, Error>::new(toml)?;

    api.post("submit", |req, state| {
        async move {
            let tx = req
                .body_auto::<Transaction>()
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

pub(super) fn state_signature<N, S>() -> anyhow::Result<Api<S, Error>>
where
    N: network::Type,
    S: 'static + Send + Sync + ReadState,
    S::State: Send + Sync + StateSignatureDataSource<N>,
{
    let toml = toml::from_str::<toml::Value>(include_str!("../../api/state_signature.toml"))?;
    let mut api = Api::<S, Error>::new(toml)?;

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

pub(super) fn state<S>() -> anyhow::Result<Api<S, Error>>
where
    S: 'static + Send + Sync + ReadState,
    S::State: Send + Sync + StateDataSource,
{
    let toml = toml::from_str::<toml::Value>(include_str!("../../api/state.toml"))?;
    let mut api = Api::<S, Error>::new(toml)?;

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
