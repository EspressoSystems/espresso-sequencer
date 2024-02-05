//! A light client prover service

use crate::snark::{generate_state_update_proof, Proof, ProvingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use async_std::{
    sync::Arc,
    task::{sleep, spawn},
};
use contract_bindings::light_client::LightClient;
use displaydoc::Display;
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::Http,
    providers::Provider,
    signers::{LocalWallet, Wallet},
    types::{Address, U256},
};
use hotshot_contract_adapter::jellyfish::ParsedPlonkProof;
use hotshot_contract_adapter::light_client::ParsedLightClientState;
use hotshot_stake_table::vec_based::StakeTable;
use hotshot_types::signature_key::BLSPubKey;
use hotshot_types::traits::stake_table::{SnapshotVersion, StakeTableError, StakeTableScheme as _};
use hotshot_types::{
    light_client::{
        CircuitField, LightClientState, PublicInput, StateKeyPair, StateSignaturesBundle,
        StateVerKey,
    },
    traits::signature_key::SignatureKey,
};
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

/// A wallet with local signer and connected to network via http
pub type L1Wallet = SignerMiddleware<Provider<Http>, LocalWallet>;

/// Configuration/Parameters used for hotshot state prover
#[derive(Debug, Clone)]
pub struct StateProverConfig {
    /// Path to the proving key
    pub proving_key_path: PathBuf,
    /// Url of the state relay server (a CDN that sequencers push their Schnorr signatures to)
    pub relay_server: Url,
    /// Interval between light client state update
    pub update_interval: Duration,
    /// URL of layer 1 Ethereum JSON-RPC provider.
    pub l1_provider: Url,
    /// Address of LightClient contract on layer 1.
    pub light_client_address: Address,
    /// Transaction signing key for Ethereum
    pub eth_signing_key: SigningKey,
    /// Number of nodes
    pub num_nodes: usize,
    /// Seed to generate keys
    pub seed: [u8; 32],
}

pub async fn init_stake_table(
    config: &StateProverConfig,
) -> StakeTable<BLSPubKey, StateVerKey, CircuitField> {
    // We now initialize a static stake table as what hotshot orchestrator does.
    // In the future we should get the stake table from the contract.
    let mut st = StakeTable::<BLSPubKey, StateVerKey, CircuitField>::new(STAKE_TABLE_CAPACITY);
    (0..config.num_nodes).for_each(|id| {
        let bls_key = BLSPubKey::generated_from_seed_indexed(config.seed, id as u64).0;
        let state_ver_key =
            StateKeyPair::generate_from_seed_indexed(config.seed, id as u64).ver_key();
        st.register(bls_key, U256::from(1u64), state_ver_key)
            .expect("Key registration shouldn't fail.");
    });
    st.advance();
    st.advance();
    std::println!("Stake table initialized.");
    st
}

pub fn load_proving_key(path: PathBuf) -> ProvingKey {
    let f = File::open(path).unwrap_or_else(|err| panic!("{err}"));
    <ProvingKey as CanonicalDeserialize>::deserialize_compressed_unchecked(f)
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
    public_input: PublicInput,
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
        .map_err(ProverError::ContractError)?;

    Ok(())
}

pub async fn sync_state(
    st: &StakeTable<BLSPubKey, StateVerKey, CircuitField>,
    proving_key: &ProvingKey,
    relay_server_client: &Client<ServerError>,
    config: &StateProverConfig,
) -> Result<(), ProverError> {
    tracing::info!("Start syncing light client state.");

    let bundle = fetch_latest_state(relay_server_client).await?;
    let old_state = read_contract_state(config).await?;
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

pub async fn run_prover_service(config: StateProverConfig) {
    // TODO(#1022): maintain the following stake table
    let st = Arc::new(init_stake_table(&config).await);
    let proving_key = Arc::new(load_proving_key(config.proving_key_path.clone()));
    let relay_server_client = Arc::new(Client::<ServerError>::new(config.relay_server.clone()));
    let config = Arc::new(config);
    let update_interval = config.update_interval;

    loop {
        let st = st.clone();
        let proving_key = proving_key.clone();
        let relay_server_client = relay_server_client.clone();
        let config = config.clone();
        spawn(async move {
            if let Err(err) = sync_state(&st, &proving_key, &relay_server_client, &config).await {
                tracing::error!("Cannot sync the light client state: {}", err);
            }
        });
        sleep(update_interval).await;
    }
}

/// Run light client state prover once
pub async fn run_prover_once(config: StateProverConfig) {
    let st = init_stake_table(&config).await;
    let proving_key = load_proving_key(config.proving_key_path.clone());
    let relay_server_client = Client::<ServerError>::new(config.relay_server.clone());

    sync_state(&st, &proving_key, &relay_server_client, &config)
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

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use ethers::{providers::Middleware, utils::Anvil};
    use std::process::Command;

    #[async_std::test]
    async fn test_read_contract_state() -> Result<()> {
        setup_logging();
        setup_backtrace();

        let anvil = Anvil::new().spawn();
        let provider_url = Url::parse(&anvil.endpoint()).unwrap();
        let provider = Provider::<Http>::try_from(provider_url.to_string())?;
        let signer = Wallet::from(anvil.keys()[0].clone());
        let l1_wallet = Arc::new(L1Wallet::new(provider.clone(), signer));

        Command::new("just")
            .arg("sol-deploy-url")
            .arg(provider_url.to_string())
            .status()
            .expect("fail to deploy");

        let last_blk_num = provider.get_block_number().await?;
        let address = provider.get_block_receipts(last_blk_num).await?[1]
            .contract_address
            .expect("fail to get LightClient address from receipt");

        let contract = LightClient::new(address, l1_wallet);

        // now test if we can read from the contract
        assert_eq!(contract.blocks_per_epoch().call().await?, u32::MAX);
        let genesis: ParsedLightClientState = contract.genesis_state().await?.into();
        // NOTE: these values changes with `contracts/scripts/LightClient.s.sol`
        assert_eq!(genesis.view_num, 0);
        assert_eq!(genesis.block_height, 0);
        assert_eq!(genesis.block_comm_root, U256::from(42));
        assert_eq!(genesis.fee_ledger_comm, U256::from(0));
        assert_eq!(genesis.bls_key_comm, U256::from(42));
        assert_eq!(genesis.schnorr_key_comm, U256::from(42));
        assert_eq!(genesis.amount_comm, U256::from(42));
        assert_eq!(genesis.threshold, U256::from(10));
        Ok(())
    }
}
