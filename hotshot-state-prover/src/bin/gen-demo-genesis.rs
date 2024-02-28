use clap::Parser;
use derive_more::From;
use ethers::abi::AbiEncode;
use ethers::types::U256;
use ethers::utils::hex::{self, FromHexError};
use hotshot_contract_adapter::jellyfish::u256_to_field;
use hotshot_contract_adapter::light_client::ParsedLightClientState;
use hotshot_stake_table::{config::STAKE_TABLE_CAPACITY, vec_based::StakeTable};
use hotshot_types::light_client::GenericPublicInput;
use hotshot_types::{
    light_client::{CircuitField, StateKeyPair, StateVerKey},
    signature_key::BLSPubKey,
    traits::{
        signature_key::SignatureKey as _,
        stake_table::{SnapshotVersion, StakeTableScheme as _},
    },
};
use snafu::Snafu;

type F = ark_ed_on_bn254::Fq;

#[derive(Parser)]
struct Args {
    /// Number of nodes in the stake table.
    /// WARNING: This is used temporarily to initialize a static stake table.
    ///          In the future we should get the stake table from the contract.
    #[clap(
        short,
        long,
        env = "ESPRESSO_ORCHESTRATOR_NUM_NODES",
        default_value = "5"
    )]
    num_nodes: usize,

    /// Seed to use for generating node keys.
    /// WARNING: This is used temporarily to initialize a static stake table.
    ///          In the future we should get the stake table from the contract.
    #[arg(long, env = "ESPRESSO_ORCHESTRATOR_KEYGEN_SEED", default_value = "0x0000000000000000000000000000000000000000000000000000000000000000", value_parser = parse_seed)]
    keygen_seed: [u8; 32],
}

#[derive(Debug, Snafu, From)]
enum ParseSeedError {
    #[snafu(display("seed must be valid hex: {source}"))]
    Hex { source: FromHexError },

    #[snafu(display("wrong length for seed {length} (expected 32)"))]
    WrongLength { length: usize },
}

fn parse_seed(s: &str) -> Result<[u8; 32], ParseSeedError> {
    <[u8; 32]>::try_from(hex::decode(s)?)
        .map_err(|vec| ParseSeedError::WrongLength { length: vec.len() })
}

pub fn stake_table_commitment_for_demo(
    num_nodes: usize,
    keygen_seed: [u8; 32],
) -> ((CircuitField, CircuitField, CircuitField), U256) {
    // We now initialize a static stake table as what hotshot orchestrator does.
    // In the future we should get the stake table from the contract.
    let mut st = StakeTable::<BLSPubKey, StateVerKey, CircuitField>::new(STAKE_TABLE_CAPACITY);
    (0..num_nodes).for_each(|id| {
        let bls_key = BLSPubKey::generated_from_seed_indexed(keygen_seed, id as u64).0;
        let state_ver_key =
            StateKeyPair::generate_from_seed_indexed(keygen_seed, id as u64).ver_key();
        st.register(bls_key, U256::from(1u64), state_ver_key)
            .expect("Key registration shouldn't fail.");
    });
    st.advance();
    st.advance();
    (
        st.commitment(SnapshotVersion::LastEpochStart)
            .expect("Commitment computation shouldn't fail."),
        st.total_stake(SnapshotVersion::LastEpochStart).unwrap() * 2 / 3,
    )
}

fn main() {
    let args = Args::parse();

    let ((bls_comm, schnorr_comm, stake_comm), threshold) =
        stake_table_commitment_for_demo(args.num_nodes, args.keygen_seed);

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
