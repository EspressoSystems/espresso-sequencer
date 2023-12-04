//! Types and structs associated with light client state

use ark_ff::PrimeField;
use jf_primitives::signatures::schnorr;

/// A serialized light client state for proof generation
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LightClientState<F: PrimeField> {
    /// Current view number
    pub view_number: usize,
    /// Current block height
    pub block_height: usize,
    /// Root of the block commitment tree
    pub block_comm_root: F,
    /// Commitment for fee ledger
    pub fee_ledger_comm: F,
    /// Commitment for the stake table
    pub stake_table_comm: (F, F, F),
}

impl<F: PrimeField> From<LightClientState<F>> for [F; 7] {
    fn from(state: LightClientState<F>) -> Self {
        [
            F::from(state.view_number as u64),
            F::from(state.block_height as u64),
            state.block_comm_root,
            state.fee_ledger_comm,
            state.stake_table_comm.0,
            state.stake_table_comm.1,
            state.stake_table_comm.2,
        ]
    }
}
impl<F: PrimeField> From<&LightClientState<F>> for [F; 7] {
    fn from(state: &LightClientState<F>) -> Self {
        [
            F::from(state.view_number as u64),
            F::from(state.block_height as u64),
            state.block_comm_root,
            state.fee_ledger_comm,
            state.stake_table_comm.0,
            state.stake_table_comm.1,
            state.stake_table_comm.2,
        ]
    }
}

use ark_ed_on_bn254::EdwardsConfig as Config;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

/// Signatures
pub type StateSignature = schnorr::Signature<Config>;
/// Verification key for verifying state signatures
pub type StateVerKey = schnorr::VerKey<Config>;
/// Signing key for signing a light client state
pub type StateSignKey = schnorr::SignKey<ark_ed_on_bn254::Fr>;
/// Key pairs for signing/verifying a light client state
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct StateKeyPair(schnorr::KeyPair<Config>);

impl std::ops::Deref for StateKeyPair {
    type Target = schnorr::KeyPair<Config>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StateKeyPair {
    /// Generate key pairs from `thread_rng()`
    #[must_use]
    pub fn generate() -> StateKeyPair {
        schnorr::KeyPair::generate(&mut rand::thread_rng()).into()
    }

    /// Generate key pairs from seed
    #[must_use]
    pub fn generate_from_seed(seed: [u8; 32]) -> StateKeyPair {
        schnorr::KeyPair::generate(&mut ChaCha20Rng::from_seed(seed)).into()
    }

    /// Generate key pairs from an index and a seed
    #[must_use]
    pub fn generate_from_seed_indexed(seed: [u8; 32], index: u64) -> StateKeyPair {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&seed);
        hasher.update(&index.to_le_bytes());
        let new_seed = *hasher.finalize().as_bytes();
        Self::generate_from_seed(new_seed)
    }
}

impl From<schnorr::KeyPair<Config>> for StateKeyPair {
    fn from(value: schnorr::KeyPair<Config>) -> Self {
        StateKeyPair(value)
    }
}
