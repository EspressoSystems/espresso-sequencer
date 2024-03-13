use clap::Parser;
use ethers::abi::AbiEncode;
use hotshot_contract_adapter::jellyfish::u256_to_field;
use hotshot_contract_adapter::light_client::ParsedLightClientState;
use hotshot_state_prover::service::init_stake_table_from_orchestrator;
use hotshot_types::light_client::GenericPublicInput;
use hotshot_types::traits::stake_table::SnapshotVersion;
use hotshot_types::traits::stake_table::StakeTableScheme as _;
use url::Url;

type F = ark_ed_on_bn254::Fq;

#[derive(Parser)]
struct Args {
    /// URL of the HotShot orchestrator.
    #[clap(
        short,
        long,
        env = "ESPRESSO_SEQUENCER_ORCHESTRATOR_URL",
        default_value = "http://localhost:8080"
    )]
    pub orchestrator_url: Url,
}

#[async_std::main]
async fn main() {
    let args = Args::parse();

    let st = init_stake_table_from_orchestrator(&args.orchestrator_url).await;
    let (bls_comm, schnorr_comm, stake_comm) = st
        .commitment(SnapshotVersion::LastEpochStart)
        .expect("Commitment computation shouldn't fail.");
    let threshold = st.total_stake(SnapshotVersion::LastEpochStart).unwrap() * 2 / 3;

    let pi = vec![
        u256_to_field(threshold),
        F::from(0_u64), // Arbitrary value for view number
        F::from(0_u64), // Arbitrary value for block height
        F::from(0_u64), // Arbitrary value for state commitment
        F::from(0_u64), // Arbitrary value for fee ledger commitment
        bls_comm,
        schnorr_comm,
        stake_comm,
    ];
    let pi: GenericPublicInput<F> = pi.into();

    let pi: ParsedLightClientState = pi.into();

    println!("{}", pi.encode_hex());
}
