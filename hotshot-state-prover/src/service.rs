//! A light client prover service

use crate::circuit::PublicInput;
use crate::proof::{generate_state_update_proof, Proof, ProvingKey};
use crate::state::{LightClientState, StateSignaturesBundle, StateVerKey};
use crate::BaseField;
use ethers::types::U256;
use hotshot_stake_table::vec_based::StakeTable;
use hotshot_types::signature_key::BLSPubKey;
use hotshot_types::traits::stake_table::{SnapshotVersion, StakeTableError, StakeTableScheme as _};
use jf_plonk::errors::PlonkError;
use surf_disco::Client;
use tide_disco::error::ServerError;
use url::Url;

const STAKE_TABLE_CAPACITY: usize = 200;

pub async fn sync_stake_table() -> StakeTable<BLSPubKey, StateVerKey, BaseField> {
    // TODO(Chengyu): initialize a stake table
    Default::default()
}

pub fn load_proving_key() -> ProvingKey {
    // TODO(Chengyu): get a proving key
    todo!()
}

pub async fn fetch_latest_state(
    client: &Client<ServerError>,
) -> Result<StateSignaturesBundle, ServerError> {
    tracing::info!("Fetching the latest state signatures bundle from relay server.");
    client
        .get::<StateSignaturesBundle>("/api/state")
        .send()
        .await
}

pub async fn read_contract_state() -> Result<LightClientState, ProverError> {
    todo!()
}

pub async fn submit_state_and_proof(
    _state: LightClientState,
    _proof: Proof,
    _public_input: PublicInput<BaseField>,
) -> Result<(), ProverError> {
    todo!()
}

pub async fn sync_state(
    st: &StakeTable<BLSPubKey, StateVerKey, BaseField>,
    proving_key: &ProvingKey,
    relay_server_client: &Client<ServerError>,
) -> Result<(), ProverError> {
    tracing::info!("Start syncing light client state.");

    let bundle = fetch_latest_state(relay_server_client).await?;
    let old_state = read_contract_state().await?;
    if old_state.block_height >= bundle.state.block_height {
        tracing::info!("No update needed.");
        return Ok(());
    }

    let threshold = st.total_stake(SnapshotVersion::LastEpochStart)? * 2 / 3;
    let entries = st
        .try_iter(SnapshotVersion::LastEpochStart)
        .unwrap()
        .map(|(_, stake_amount, state_key)| (state_key, stake_amount))
        .collect::<Vec<_>>();
    let mut signer_bit_vec = vec![false; entries.len()];
    let mut signatures = vec![Default::default(); entries.len()];
    let mut accumulated_weight = U256::zero();
    entries.iter().enumerate().for_each(|(i, (key, stake))| {
        if let Some(sig) = bundle.signatures.get(key) {
            signer_bit_vec[i] = true;
            signatures[i] = sig.clone();
            accumulated_weight += *stake;
        }
    });

    tracing::info!("Collected latest state and signatures. Start generating SNARK proof.");
    let now = time::Instant::now();
    let (proof, public_input) = generate_state_update_proof::<_, _, _, _, STAKE_TABLE_CAPACITY>(
        &mut ark_std::rand::thread_rng(),
        proving_key,
        &entries,
        signer_bit_vec,
        signatures,
        &bundle.state,
        &threshold,
    )?;
    let elapsed = now.elapsed();
    tracing::info!("Proof generation completed. Elapsed: {elapsed:.2}");

    submit_state_and_proof(bundle.state, proof, public_input).await
}

pub async fn run_prover_service(_relay_server_url: Url) {
    todo!()
}

/// Run light client state prover once
pub async fn run_prover_once(relay_server_url: Url) {
    let st = sync_stake_table().await;
    let proving_key = load_proving_key();
    let client = Client::<ServerError>::new(relay_server_url);

    sync_state(&st, &proving_key, &client)
        .await
        .expect("Error syncing the light client state.");
}

#[derive(Debug)]
pub enum ProverError {
    /// Invalid light client state or signatures
    InvalidState,
    /// Error when communicating with the smart contract
    ContractError,
    /// Error when communicating with the state relay server
    RelayServerError(ServerError),
    /// Internal error with the stake table
    StakeTableError(StakeTableError),
    /// Internal error when generating the SNARK proof
    PlonkError(PlonkError),
}

impl From<ServerError> for ProverError {
    fn from(err: ServerError) -> Self {
        Self::RelayServerError(err)
    }
}

impl From<PlonkError> for ProverError {
    fn from(err: PlonkError) -> Self {
        Self::PlonkError(err)
    }
}

impl From<StakeTableError> for ProverError {
    fn from(err: StakeTableError) -> Self {
        Self::StakeTableError(err)
    }
}
