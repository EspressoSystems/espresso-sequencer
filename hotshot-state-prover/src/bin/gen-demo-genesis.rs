use clap::Parser;
use ethers::abi::AbiEncode;
use ethers::types::U256;
use hotshot::types::{BLSPrivKey, BLSPubKey, SignatureKey};
use hotshot_contract_adapter::jellyfish::u256_to_field;
use hotshot_contract_adapter::light_client::ParsedLightClientState;
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;
use hotshot_stake_table::vec_based::StakeTable;
use hotshot_types::light_client::{GenericPublicInput, StateSignKey, StateVerKey};
use hotshot_types::{
    light_client::CircuitField,
    traits::stake_table::{SnapshotVersion, StakeTableScheme as _},
};

type F = ark_ed_on_bn254::Fq;

#[derive(Parser)]
struct Args {
    // Sequencer Private Staking key list
    #[clap(
        long,
        env = "ESPRESSO_DEMO_SEQUENCER_STAKING_PRIVATE_KEY_LIST",
        value_delimiter = ','
    )]
    pub private_staking_keys: Vec<BLSPrivKey>,

    // Sequencer State signing key list
    #[clap(
        long,
        env = "ESPRESSO_DEMO_SEQUENCER_STATE_PRIVATE_KEY_LIST",
        value_delimiter = ','
    )]
    pub private_state_keys: Vec<StateSignKey>,
}

pub fn stake_table_commitment_for_demo(
    bls_priv_keys: &[BLSPrivKey],
    state_priv_keys: &[StateSignKey],
) -> ((CircuitField, CircuitField, CircuitField), U256) {
    // We now initialize a static stake table from the environment variables.
    // In the future we should get the stake table from the contract.
    let mut st = StakeTable::<BLSPubKey, StateVerKey, CircuitField>::new(STAKE_TABLE_CAPACITY);
    bls_priv_keys
        .iter()
        .zip(state_priv_keys)
        .for_each(|(bls_priv_key, state_priv_key)| {
            let bls_ver_key = BLSPubKey::from_private(bls_priv_key);
            let state_ver_key = StateVerKey::from(state_priv_key);
            st.register(bls_ver_key, U256::one(), state_ver_key)
                .expect("Stake table registration shouldn't fail.");
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
        stake_table_commitment_for_demo(&args.private_staking_keys, &args.private_state_keys);

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
