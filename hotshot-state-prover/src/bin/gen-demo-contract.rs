use clap::Parser;
use derive_more::From;
use ethers::types::U256;
use ethers::utils::hex::{self, FromHexError};
use hotshot_stake_table::{config::STAKE_TABLE_CAPACITY, vec_based::StakeTable};
use hotshot_types::{
    light_client::{CircuitField, StateKeyPair, StateVerKey},
    signature_key::BLSPubKey,
    traits::{
        signature_key::SignatureKey as _,
        stake_table::{SnapshotVersion, StakeTableScheme as _},
    },
};
use snafu::Snafu;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

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

    #[clap(long, default_value = "contracts")]
    contracts_root_dir: PathBuf,
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

    // calculate the path to solidity file
    let contract_name = "LightClientDemo.s.sol";
    let mut path = args.contracts_root_dir;
    path.push("script");
    path.push(contract_name);
    path.set_extension("sol");

    println!(
        "Writing contract into file: {}",
        path.as_os_str().to_str().unwrap()
    );
    // overwrite the file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();

    let code = format!(
        "// SPDX-License-Identifier: UNLICENSED
        // This contract is auto-generated by gen-demo-contract. Direct edits are not recommended.
        pragma solidity ^0.8.0;
        
        import \"forge-std/Script.sol\";
        import {{ BN254 }} from \"bn254/BN254.sol\";
        import {{ LightClient as LC }} from \"../src/LightClient.sol\";
        
        contract DeployLightClientDemoScript is Script {{
            function run() external {{
                string memory seedPhrase = vm.envString(\"MNEMONIC\");
                uint256 privateKey = vm.deriveKey(seedPhrase, 0);
                vm.startBroadcast(privateKey);
        
                // For this version there will be only one epoch
                uint32 blocksPerEpoch = type(uint32).max;
        
                uint64 viewNum = 0;
                uint64 blockHeight = 0;
                BN254.ScalarField blockCommRoot = BN254.ScalarField.wrap(0);
                BN254.ScalarField feeLedgerComm = BN254.ScalarField.wrap(0);
                BN254.ScalarField stakeTableBlsKeyComm = BN254.ScalarField.wrap(
                    {}
                );
                BN254.ScalarField stakeTableSchnorrKeyComm = BN254.ScalarField.wrap(
                    {}
                );
                BN254.ScalarField stakeTableAmountComm = BN254.ScalarField.wrap(
                    {}
                );
                uint256 threshold = {};
        
                LC.LightClientState memory genesis = LC.LightClientState(
                    viewNum,
                    blockHeight,
                    blockCommRoot,
                    feeLedgerComm,
                    stakeTableBlsKeyComm,
                    stakeTableSchnorrKeyComm,
                    stakeTableAmountComm,
                    threshold
                );
                new LC();
                new LC.initialize(genesis, blocksPerEpoch);
        
                vm.stopBroadcast();
            }}
        }}
        ",
        bls_comm, schnorr_comm, stake_comm, threshold
    )
    .into_bytes();

    file.write_all(&code).unwrap();
}
