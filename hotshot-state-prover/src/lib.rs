//! SNARK-assisted light client state update verification in HotShot

/// State verifier circuit builder
pub mod circuit;
mod utils;

use ark_bn254::Bn254;
use ark_std::{
    borrow::Borrow,
    rand::{CryptoRng, RngCore},
};
use circuit::build_state_verifier_circuit;
use ethereum_types::U256;
use hotshot_stake_table::vec_based::StakeTable;
use hotshot_types::traits::{
    stake_table::{SnapshotVersion, StakeTableScheme},
    state::LightClientState,
};
use jf_plonk::{
    errors::PlonkError,
    proof_system::{PlonkKzgSnark, UniversalSNARK},
    transcript::StandardTranscript,
};
use jf_primitives::signatures::schnorr::Signature;
use jf_relation::PlonkCircuit;

/// BLS verification key, base field and Schnorr verification key
pub use hotshot_stake_table::vec_based::config::{
    BLSVerKey, FieldType as BaseField, SchnorrVerKey,
};
/// Proving key
pub type ProvingKey = jf_plonk::proof_system::structs::ProvingKey<Bn254>;
/// Verifying key
pub type VerifyingKey = jf_plonk::proof_system::structs::VerifyingKey<Bn254>;
/// Proof
pub type Proof = jf_plonk::proof_system::structs::Proof<Bn254>;
/// Universal SRS
pub type UniversalSrs = jf_plonk::proof_system::structs::UniversalSrs<Bn254>;
/// Curve config for Schnorr signatures
pub use ark_ed_on_bn254::EdwardsConfig;

/// Given a SRS, returns the proving key and verifying key for state update
pub fn preprocess(srs: &UniversalSrs) -> Result<(ProvingKey, VerifyingKey), PlonkError> {
    let (circuit, _) = build_dummy_circuit_for_preprocessing()?;
    PlonkKzgSnark::preprocess(srs, &circuit)
}

/// Given a proving key and
/// - a list of stake table entries (`Vec<(BLSVerKey, Amount, SchnorrVerKey)>`)
/// - a list of schnorr signatures of the updated states (`Vec<SchnorrSignature>`), default if the node doesn't sign the state
/// - updated light client state (`(view_number, block_height, block_comm, fee_ledger_comm, stake_table_comm)`)
/// - a bit vector indicates the signers
/// - a quorum threshold
/// Returns error or a pair (proof, public_inputs) asserting that
/// - the signer's accumulated weight exceeds the quorum threshold
/// - the stake table corresponds to the one committed in the light client state
/// - all signed schnorr signatures are valid
pub fn generate_state_update_proof<ST, R, BitIter, SigIter>(
    rng: &mut R,
    pk: &ProvingKey,
    stake_table: &ST,
    signer_bit_vec: BitIter,
    signatures: SigIter,
    lightclient_state: &LightClientState<BaseField>,
    threshold: &U256,
) -> Result<(Proof, Vec<BaseField>), PlonkError>
where
    ST: StakeTableScheme<Key = BLSVerKey, Amount = U256, Aux = SchnorrVerKey>,
    R: CryptoRng + RngCore,
    BitIter: IntoIterator,
    BitIter::Item: Borrow<bool>,
    BitIter::IntoIter: ExactSizeIterator,
    SigIter: IntoIterator,
    SigIter::Item: Borrow<Signature<EdwardsConfig>>,
    SigIter::IntoIter: ExactSizeIterator,
{
    let stake_table_entries = stake_table
        .try_iter(SnapshotVersion::LastEpochStart)
        .unwrap()
        .map(|(_, stake_amount, schnorr_key)| (schnorr_key, stake_amount))
        .collect::<Vec<_>>();
    let (circuit, public_inputs) = build_state_verifier_circuit(
        stake_table_entries,
        signer_bit_vec,
        signatures,
        lightclient_state,
        threshold,
    )?;
    let proof = PlonkKzgSnark::<Bn254>::prove::<_, _, StandardTranscript>(rng, &circuit, pk, None)?;
    Ok((proof, public_inputs))
}

/// Internal function for helping generate the proving/verifying key
fn build_dummy_circuit_for_preprocessing(
) -> Result<(PlonkCircuit<BaseField>, Vec<BaseField>), PlonkError> {
    let st = StakeTable::<BLSVerKey, SchnorrVerKey, BaseField>::new();
    let lightclient_state = LightClientState {
        view_number: 0,
        block_height: 0,
        block_comm: BaseField::default(),
        fee_ledger_comm: BaseField::default(),
        stake_table_comm: st.commitment(SnapshotVersion::LastEpochStart).unwrap(),
    };
    build_state_verifier_circuit::<BaseField, EdwardsConfig, _, _, _>(
        &[],
        &[],
        &[],
        &lightclient_state,
        &U256::zero(),
    )
}
