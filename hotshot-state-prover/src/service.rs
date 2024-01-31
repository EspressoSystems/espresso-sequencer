//! A light client prover service

use crate::circuit::PublicInput;
use crate::snark::{generate_state_update_proof, Proof, ProvingKey};
use crate::state::{LightClientState, StateSignaturesBundle, StateVerKey};
use crate::CircuitField;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use async_std::{
    sync::Arc,
    task::{sleep, spawn},
};
use displaydoc::Display;
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::Http,
    providers::Provider,
    signers::{LocalWallet, Wallet},
    types::{Address, U256},
};
use hotshot_contract::jf_helpers::ParsedPlonkProof;
use hotshot_contract::light_client::ParsedLightClientState;
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

/// prepare a contract interface ready to be read from or written to
fn prepare_contract(config: &StateProverConfig) -> Result<LightClient<L1Wallet>, ProverError> {
    let provider = Provider::try_from(config.l1_provider.to_string())
        .expect("unable to instantiate Provider, likely wrong URL");
    let signer = Wallet::from(config.eth_signing_key.clone());
    let l1_wallet = Arc::new(L1Wallet::new(provider, signer));

    let contract = LightClient::new(config.light_client_address, l1_wallet);
    Ok(contract)
}

/// get the `finalizedState` from the LightClient contract storage on L1
pub async fn read_contract_state(
    config: &StateProverConfig,
) -> Result<LightClientState, ProverError> {
    let contract = prepare_contract(config)?;
    let state: ParsedLightClientState = match contract.finalized_state().call().await {
        Ok(s) => s.into(),
        Err(e) => {
            tracing::error!("unable to read finalized_state from contract: {}", e);
            return Err(ProverError::ContractError(e.into()));
        }
    };
    let state: LightClientState = state.into();

    Ok(state)
}

/// submit the latest finalized state along with a proof to the L1 LightClient contract
pub async fn submit_state_and_proof(
    proof: Proof,
    public_input: PublicInput<CircuitField>,
    config: &StateProverConfig,
) -> Result<(), ProverError> {
    let contract = prepare_contract(config)?;

    // prepare the input the contract call and the tx itself
    let proof: ParsedPlonkProof = proof.into();
    let new_state: ParsedLightClientState = public_input.into();
    let tx = contract.new_finalized_state(new_state.into(), proof.into());

    // send the tx
    let (_receipt, _included_block) = sequencer_utils::contract_send(&tx)
        .await
        .map_err(|e| ProverError::ContractError(e))?;

    Ok(())
}

pub async fn sync_state(
    st: &StakeTable<BLSPubKey, StateVerKey, CircuitField>,
    proving_key: &ProvingKey,
    relay_server_client: &Client<ServerError>,
) -> Result<(), ProverError> {
    tracing::info!("Start syncing light client state.");

    let bundle = fetch_latest_state(relay_server_client).await?;
    let old_state = read_contract_state(&config).await?;
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

    submit_state_and_proof(proof, public_input, config).await?;

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
    let st = Arc::new(sync_stake_table().await);
    let proving_key = Arc::new(load_proving_key(key_path));
    let relay_server_client = Arc::new(Client::<ServerError>::new(relay_server_url));

    loop {
        let st = st.clone();
        let proving_key = proving_key.clone();
        let relay_server_client = relay_server_client.clone();
        spawn(async move {
            if let Err(err) = sync_state(&st, &proving_key, &relay_server_client).await {
                tracing::error!("Cannot sync the light client state: {}", err);
            }
        });
        sleep(freq).await;
    }
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
    ContractError(anyhow::Error),
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
