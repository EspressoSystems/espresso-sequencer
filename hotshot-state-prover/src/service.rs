//! A light client prover service

use crate::circuit::PublicInput;
use crate::snark::{generate_state_update_proof, Proof, ProvingKey};
use crate::state::{LightClientState, StateSignaturesBundle, StateVerKey};
use crate::CircuitField;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use async_std::task::{sleep, spawn};
use displaydoc::Display;
use ethers::types::U256;
use hotshot_stake_table::vec_based::StakeTable;
use hotshot_types::signature_key::BLSPubKey;
use hotshot_types::traits::stake_table::{SnapshotVersion, StakeTableError, StakeTableScheme as _};
use jf_plonk::errors::PlonkError;
use jf_primitives::pcs::prelude::UnivariateUniversalParams;
use jf_relation::Circuit as _;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use surf_disco::Client;
use tide_disco::error::ServerError;
use time::Instant;
use url::Url;

const STAKE_TABLE_CAPACITY: usize = 200;

pub async fn sync_stake_table() -> StakeTable<BLSPubKey, StateVerKey, CircuitField> {
    // TODO(Chengyu): initialize a stake table
    Default::default()
}

pub fn load_proving_key(path: PathBuf) -> ProvingKey {
    let f = File::open(path).unwrap_or_else(|err| panic!("{err}"));
    <ProvingKey as CanonicalDeserialize>::deserialize_compressed(f)
        .unwrap_or_else(|err| panic!("{err}"))
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
    _proof: Proof,
    _public_input: PublicInput<CircuitField>,
) -> Result<(), ProverError> {
    todo!()
}

pub async fn sync_state(
    st: &StakeTable<BLSPubKey, StateVerKey, CircuitField>,
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
    let proof_gen_start = time::Instant::now();
    let (proof, public_input) = generate_state_update_proof::<_, _, _, _, STAKE_TABLE_CAPACITY>(
        &mut ark_std::rand::thread_rng(),
        proving_key,
        &entries,
        signer_bit_vec,
        signatures,
        &bundle.state,
        &threshold,
    )?;
    let proof_gen_elapsed = proof_gen_start.elapsed();
    tracing::info!("Proof generation completed. Elapsed: {proof_gen_elapsed:.3}");

    submit_state_and_proof(proof, public_input).await?;

    tracing::info!("Successfully synced light client state.");
    Ok(())
}

pub fn key_gen(path: PathBuf) {
    let srs = {
        let num_gates = crate::circuit::build_for_preprocessing::<
            CircuitField,
            ark_ed_on_bn254::EdwardsConfig,
            STAKE_TABLE_CAPACITY,
        >()
        .unwrap()
        .0
        .num_gates();

        std::println!("Loading SRS from Aztec's ceremony...");
        let srs_timer = Instant::now();
        let srs = crs::aztec20::kzg10_setup(num_gates + 2).expect("Aztec SRS fail to load");
        let srs_elapsed = srs_timer.elapsed();
        std::println!("Done in {srs_elapsed:.3}");

        // convert to Jellyfish type
        // TODO: (alex) use constructor instead https://github.com/EspressoSystems/jellyfish/issues/440
        UnivariateUniversalParams {
            powers_of_g: srs.powers_of_g,
            h: srs.h,
            beta_h: srs.beta_h,
            powers_of_h: vec![srs.h, srs.beta_h],
        }
    };

    std::println!("Generating proving key and verification key.");
    let key_gen_timer = Instant::now();
    let (pk, vk) = crate::snark::preprocess::<STAKE_TABLE_CAPACITY>(&srs)
        .expect("Fail to preprocess state prover circuit");
    let key_gen_elapsed = key_gen_timer.elapsed();
    std::println!("Done in {key_gen_elapsed:.3}");

    let mut vk_path = path.clone();
    pk.serialize_compressed(File::create(&path).expect("Error creating file {path}."))
        .expect("Error serializing the proving key");
    vk_path.set_extension("pub");
    vk.serialize_compressed(File::create(&vk_path).expect("Error creating file {vk_path}."))
        .expect("Error serializing the verification key");
    std::println!(
        "Proving key: {}\nVerification key: {}",
        path.into_os_string().into_string().unwrap(),
        vk_path.into_os_string().into_string().unwrap()
    );
}

pub async fn run_prover_service(key_path: PathBuf, relay_server_url: Url, freq: Duration) {
    // TODO(#1022): maintain the following stake table
    let st = sync_stake_table().await;
    let proving_key = load_proving_key(key_path);
    let relay_server_client = Client::<ServerError>::new(relay_server_url);

    spawn(async move {
        loop {
            if let Err(err) = sync_state(&st, &proving_key, &relay_server_client).await {
                tracing::error!("Cannot sync the light client state: {}", err);
            }
            sleep(freq).await;
        }
    })
    .await;
}

/// Run light client state prover once
pub async fn run_prover_once(key_path: PathBuf, relay_server_url: Url) {
    let st = sync_stake_table().await;
    let proving_key = load_proving_key(key_path);
    let relay_server_client = Client::<ServerError>::new(relay_server_url);

    sync_state(&st, &proving_key, &relay_server_client)
        .await
        .expect("Error syncing the light client state.");
}

#[derive(Debug, Display)]
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
    /// Internal error
    Internal(String),
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

impl std::error::Error for ProverError {}
