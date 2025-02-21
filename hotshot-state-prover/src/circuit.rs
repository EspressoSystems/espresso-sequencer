//! Circuit implementation for verifying light client state update

use ark_ec::twisted_edwards::TECurveConfig;
use ark_ff::PrimeField;
use ark_std::borrow::Borrow;
use ethers::types::U256;
use hotshot_contract_adapter::light_client::{ParsedLightClientState, ParsedStakeTableState};
use hotshot_types::light_client::{
    GenericLightClientState, GenericPublicInput, GenericStakeTableState,
};
use jf_plonk::PlonkError;
use jf_relation::{BoolVar, Circuit, CircuitError, PlonkCircuit, Variable};
use jf_rescue::{RescueParameter, gadgets::RescueNativeGadget};
use jf_signature::{
    gadgets::schnorr::{SignatureGadget, VerKeyVar},
    schnorr::{Signature, VerKey as SchnorrVerKey},
};

/// Lossy conversion of a U256 into a field element.
pub(crate) fn u256_to_field<F: PrimeField>(v: &U256) -> F {
    let mut bytes = vec![0u8; 32];
    v.to_little_endian(&mut bytes);
    F::from_le_bytes_mod_order(&bytes)
}

/// Variable for stake table entry
#[derive(Clone, Debug)]
pub struct StakeTableEntryVar {
    /// state verification keys
    pub state_ver_key: VerKeyVar,
    /// Stake amount
    pub stake_amount: Variable,
}

/// Light client state Variable
/// The stake table commitment is a triple `(qc_keys_comm, state_keys_comm, stake_amount_comm)`.
/// Variable for a stake table commitment
#[derive(Clone, Debug)]
pub struct StakeTableVar {
    /// Commitment for QC verification keys
    pub qc_keys_comm: Variable,
    /// Commitment for state verification keys
    pub state_keys_comm: Variable,
    /// Commitment for stake amount
    pub stake_amount_comm: Variable,
    /// Threshold for quorum signatures
    pub threshold: Variable,
}

impl StakeTableVar {
    /// # Errors
    /// if unable to create any of the public variables
    pub fn new<F: PrimeField>(
        circuit: &mut PlonkCircuit<F>,
        st: &GenericStakeTableState<F>,
    ) -> Result<Self, CircuitError> {
        Ok(Self {
            qc_keys_comm: circuit.create_public_variable(st.bls_key_comm)?,
            state_keys_comm: circuit.create_public_variable(st.schnorr_key_comm)?,
            stake_amount_comm: circuit.create_public_variable(st.amount_comm)?,
            threshold: circuit.create_public_variable(st.threshold)?,
        })
    }
}

/// Light client state Variable
#[derive(Clone, Debug)]
pub struct LightClientStateVar {
    pub(crate) view_num: Variable,
    pub(crate) block_height: Variable,
    pub(crate) block_comm_root: Variable,
}

impl LightClientStateVar {
    /// # Errors
    /// if unable to create any of the public variables
    pub fn new<F: PrimeField>(
        circuit: &mut PlonkCircuit<F>,
        state: &GenericLightClientState<F>,
    ) -> Result<Self, CircuitError> {
        Ok(Self {
            view_num: circuit.create_public_variable(F::from(state.view_number as u64))?,
            block_height: circuit.create_public_variable(F::from(state.block_height as u64))?,
            block_comm_root: circuit.create_public_variable(state.block_comm_root)?,
        })
    }
}

/// A function that takes as input:
/// - a list of stake table entries (`Vec<(SchnorrVerKey, Amount)>`)
/// - a bit vector indicates the signers
/// - a list of schnorr signatures of the updated states (`Vec<SchnorrSignature>`), default if the node doesn't sign the state
/// - updated light client state (`(view_number, block_height, block_comm_root)`)
/// - the static stake table state (containing 3 commitments to the 3 columns of the stake table and a threshold)
///
/// Lengths of input vectors should not exceed the `stake_table_capacity`.
/// The list of stake table entries, bit indicators and signatures will be padded to the `stake_table_capacity`.
/// It checks that
/// - the vector that indicates who signed is a bit vector
/// - the signers' accumulated weight exceeds the quorum threshold
/// - the stake table corresponds to the one committed in the light client state
/// - all Schnorr signatures over the light client state are valid
///
/// and returns
/// - A circuit for proof generation
/// - A list of public inputs for verification
/// - A `PlonkError` if any error happens when building the circuit
#[allow(clippy::too_many_lines)]
pub(crate) fn build<F, P, STIter, BitIter, SigIter>(
    stake_table_entries: STIter,
    signer_bit_vec: BitIter,
    signatures: SigIter,
    lightclient_state: &GenericLightClientState<F>,
    stake_table_state: &GenericStakeTableState<F>,
    stake_table_capacity: usize,
) -> Result<(PlonkCircuit<F>, GenericPublicInput<F>), PlonkError>
where
    F: RescueParameter,
    P: TECurveConfig<BaseField = F>,
    STIter: IntoIterator,
    STIter::Item: Borrow<(SchnorrVerKey<P>, U256)>,
    STIter::IntoIter: ExactSizeIterator,
    BitIter: IntoIterator,
    BitIter::Item: Borrow<F>,
    BitIter::IntoIter: ExactSizeIterator,
    SigIter: IntoIterator,
    SigIter::Item: Borrow<Signature<P>>,
    SigIter::IntoIter: ExactSizeIterator,
{
    let stake_table_entries = stake_table_entries.into_iter();
    let signer_bit_vec = signer_bit_vec.into_iter();
    let signatures = signatures.into_iter();
    if stake_table_entries.len() > stake_table_capacity {
        return Err(PlonkError::CircuitError(CircuitError::ParameterError(
            format!(
                "Number of input stake table entries {} exceeds the capacity {}",
                stake_table_entries.len(),
                stake_table_capacity,
            ),
        )));
    }
    if signer_bit_vec.len() > stake_table_capacity {
        return Err(PlonkError::CircuitError(CircuitError::ParameterError(
            format!(
                "Length of input bit vector {} exceeds the capacity {}",
                signer_bit_vec.len(),
                stake_table_capacity,
            ),
        )));
    }
    if signatures.len() > stake_table_capacity {
        return Err(PlonkError::CircuitError(CircuitError::ParameterError(
            format!(
                "Number of input signatures {} exceeds the capacity {}",
                signatures.len(),
                stake_table_capacity,
            ),
        )));
    }

    let mut circuit = PlonkCircuit::new_turbo_plonk();

    // creating variables for stake table entries
    let stake_table_entries_pad_len = stake_table_capacity - stake_table_entries.len();
    let mut stake_table_var = stake_table_entries
        .map(|item| {
            let item = item.borrow();
            let state_ver_key = circuit.create_signature_vk_variable(&item.0)?;
            let stake_amount = circuit.create_variable(u256_to_field::<F>(&item.1))?;
            Ok(StakeTableEntryVar {
                state_ver_key,
                stake_amount,
            })
        })
        .collect::<Result<Vec<_>, CircuitError>>()?;
    stake_table_var.extend(
        (0..stake_table_entries_pad_len)
            .map(|_| {
                let state_ver_key =
                    circuit.create_signature_vk_variable(&SchnorrVerKey::<P>::default())?;
                let stake_amount = circuit.create_variable(F::default())?;
                Ok(StakeTableEntryVar {
                    state_ver_key,
                    stake_amount,
                })
            })
            .collect::<Result<Vec<_>, CircuitError>>()?,
    );

    // creating variables for signatures
    let sig_pad_len = stake_table_capacity - signatures.len();
    let mut sig_vars = signatures
        .map(|sig| circuit.create_signature_variable(sig.borrow()))
        .collect::<Result<Vec<_>, CircuitError>>()?;
    sig_vars.extend(
        (0..sig_pad_len)
            .map(|_| circuit.create_signature_variable(&Signature::<P>::default()))
            .collect::<Result<Vec<_>, CircuitError>>()?,
    );

    // creating Boolean variables for the bit vector
    let bit_vec_pad_len = stake_table_capacity - signer_bit_vec.len();
    let collect = signer_bit_vec
        .map(|b| {
            let var = circuit.create_variable(*b.borrow())?;
            circuit.enforce_bool(var)?;
            Ok(BoolVar(var))
        })
        .collect::<Result<Vec<_>, CircuitError>>();
    let mut signer_bit_vec_var = collect?;
    signer_bit_vec_var.extend(
        (0..bit_vec_pad_len)
            .map(|_| circuit.create_boolean_variable(false))
            .collect::<Result<Vec<_>, CircuitError>>()?,
    );

    // public inputs
    let lightclient_state_pub_var = LightClientStateVar::new(&mut circuit, lightclient_state)?;
    let stake_table_state_pub_var = StakeTableVar::new(&mut circuit, stake_table_state)?;

    // Checking whether the accumulated weight exceeds the quorum threshold
    let mut signed_amount_var = (0..stake_table_capacity / 2)
        .map(|i| {
            circuit.mul_add(
                &[
                    stake_table_var[2 * i].stake_amount,
                    signer_bit_vec_var[2 * i].0,
                    stake_table_var[2 * i + 1].stake_amount,
                    signer_bit_vec_var[2 * i + 1].0,
                ],
                &[F::one(), F::one()],
            )
        })
        .collect::<Result<Vec<_>, CircuitError>>()?;
    // Adding the last if stake_table_capacity is not a multiple of 2
    if stake_table_capacity % 2 == 1 {
        signed_amount_var.push(circuit.mul(
            stake_table_var[stake_table_capacity - 1].stake_amount,
            signer_bit_vec_var[stake_table_capacity - 1].0,
        )?);
    }
    let acc_amount_var = PlonkCircuit::sum(&mut circuit, &signed_amount_var)?;
    circuit.enforce_leq(stake_table_state_pub_var.threshold, acc_amount_var)?;

    // checking the commitment for the list of schnorr keys
    let state_ver_key_preimage_vars = stake_table_var
        .iter()
        .flat_map(|var| [var.state_ver_key.0.get_x(), var.state_ver_key.0.get_y()])
        .collect::<Vec<_>>();
    let state_ver_key_comm = RescueNativeGadget::<F>::rescue_sponge_with_padding(
        &mut circuit,
        &state_ver_key_preimage_vars,
        1,
    )?[0];
    circuit.enforce_equal(
        state_ver_key_comm,
        stake_table_state_pub_var.state_keys_comm,
    )?;

    // checking the commitment for the list of stake amounts
    let stake_amount_preimage_vars = stake_table_var
        .iter()
        .map(|var| var.stake_amount)
        .collect::<Vec<_>>();
    let stake_amount_comm = RescueNativeGadget::<F>::rescue_sponge_with_padding(
        &mut circuit,
        &stake_amount_preimage_vars,
        1,
    )?[0];
    circuit.enforce_equal(
        stake_amount_comm,
        stake_table_state_pub_var.stake_amount_comm,
    )?;

    // checking all signatures
    let verification_result_vars = stake_table_var
        .iter()
        .zip(sig_vars)
        .map(|(entry, sig)| {
            SignatureGadget::<_, P>::check_signature_validity(
                &mut circuit,
                &entry.state_ver_key,
                &[
                    lightclient_state_pub_var.view_num,
                    lightclient_state_pub_var.block_height,
                    lightclient_state_pub_var.block_comm_root,
                ],
                &sig,
            )
        })
        .collect::<Result<Vec<_>, CircuitError>>()?;
    let bit_x_result_vars = signer_bit_vec_var
        .iter()
        .zip(verification_result_vars)
        .map(|(&bit, result)| {
            let neg_bit = circuit.logic_neg(bit)?;
            circuit.logic_or(neg_bit, result)
        })
        .collect::<Result<Vec<_>, CircuitError>>()?;
    let sig_ver_result = circuit.logic_and_all(&bit_x_result_vars)?;
    circuit.enforce_true(sig_ver_result.0)?;

    circuit.finalize_for_arithmetization()?;
    Ok((
        circuit,
        GenericPublicInput::new(lightclient_state.clone(), *stake_table_state),
    ))
}

/// Internal function to build a dummy circuit
pub(crate) fn build_for_preprocessing<F, P>(
    stake_table_capacity: usize,
) -> Result<(PlonkCircuit<F>, GenericPublicInput<F>), PlonkError>
where
    F: RescueParameter,
    P: TECurveConfig<BaseField = F>,
{
    let lightclient_state = ParsedLightClientState::dummy_genesis().into();
    let stake_table_state = ParsedStakeTableState::dummy_genesis().into();

    build::<F, P, _, _, _>(
        &[],
        &[],
        &[],
        &lightclient_state,
        &stake_table_state,
        stake_table_capacity,
    )
}

#[cfg(test)]
mod tests {
    use ark_ed_on_bn254::EdwardsConfig as Config;
    use hotshot_types::light_client::LightClientState;
    use hotshot_types::traits::stake_table::{SnapshotVersion, StakeTableScheme};
    use jf_crhf::CRHF;
    use jf_relation::Circuit;
    use jf_rescue::crhf::VariableLengthRescueCRHF;
    use jf_signature::{
        SignatureScheme,
        schnorr::{SchnorrSignatureScheme, Signature},
    };
    use jf_utils::test_rng;

    use super::build;
    use crate::test_utils::{
        genesis_stake_table_state, key_pairs_for_testing, stake_table_for_testing,
    };

    type F = ark_ed_on_bn254::Fq;
    const ST_CAPACITY: usize = 20;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_circuit_building() {
        let num_validators = 10;
        let mut prng = test_rng();

        let (qc_keys, state_keys) = key_pairs_for_testing(num_validators, &mut prng);
        let st = stake_table_for_testing(ST_CAPACITY, &qc_keys, &state_keys);
        let st_state = genesis_stake_table_state(&st);

        let entries = st
            .try_iter(SnapshotVersion::LastEpochStart)
            .unwrap()
            .map(|(_, stake_amount, state_key)| (state_key, stake_amount))
            .collect::<Vec<_>>();

        let block_comm_root =
            VariableLengthRescueCRHF::<F, 1>::evaluate(vec![F::from(1u32), F::from(2u32)]).unwrap()
                [0];

        let lightclient_state = LightClientState {
            view_number: 100,
            block_height: 73,
            block_comm_root,
        };
        let state_msg: [F; 3] = lightclient_state.clone().into();

        let sigs = state_keys
            .iter()
            .map(|(key, _)| SchnorrSignatureScheme::<Config>::sign(&(), key, state_msg, &mut prng))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        // bit vector with total weight 26
        let bit_vec = [
            true, true, true, false, true, true, false, false, true, false,
        ];
        let bit_masked_sigs = bit_vec
            .iter()
            .zip(sigs.iter())
            .map(|(bit, sig)| {
                if *bit {
                    sig.clone()
                } else {
                    Signature::<Config>::default()
                }
            })
            .collect::<Vec<_>>();
        let bit_vec = bit_vec
            .into_iter()
            .map(|b| if b { F::from(1u64) } else { F::from(0u64) })
            .collect::<Vec<_>>();
        // good path
        let (circuit, public_inputs) = build(
            &entries,
            &bit_vec,
            &bit_masked_sigs,
            &lightclient_state,
            &st_state,
            ST_CAPACITY,
        )
        .unwrap();
        assert!(
            circuit
                .check_circuit_satisfiability(public_inputs.as_ref())
                .is_ok()
        );

        // lower threshold should also pass
        let mut good_st_state = st_state;
        good_st_state.threshold = F::from(10u32);
        let (circuit, public_inputs) = build(
            &entries,
            &bit_vec,
            &bit_masked_sigs,
            &lightclient_state,
            &good_st_state,
            ST_CAPACITY,
        )
        .unwrap();
        assert!(
            circuit
                .check_circuit_satisfiability(public_inputs.as_ref())
                .is_ok()
        );

        // bad path: feeding non-bit vector
        let bit_vec = [F::from(2u64); 10];
        let (circuit, public_inputs) = build(
            &entries,
            &bit_vec,
            &bit_masked_sigs,
            &lightclient_state,
            &st_state,
            ST_CAPACITY,
        )
        .unwrap();
        assert!(
            circuit
                .check_circuit_satisfiability(public_inputs.as_ref())
                .is_err()
        );

        // bad path: total weight doesn't meet the threshold
        let bad_bit_vec = [
            false, false, true, false, true, false, false, true, false, false,
        ];
        let bad_bit_masked_sigs = bad_bit_vec
            .iter()
            .zip(sigs.iter())
            .map(|(bit, sig)| {
                if *bit {
                    sig.clone()
                } else {
                    Signature::<Config>::default()
                }
            })
            .collect::<Vec<_>>();

        let bad_bit_vec = bad_bit_vec
            .into_iter()
            .map(|b| if b { F::from(1u64) } else { F::from(0u64) })
            .collect::<Vec<_>>();
        let (bad_circuit, public_inputs) = build(
            &entries,
            &bad_bit_vec,
            &bad_bit_masked_sigs,
            &lightclient_state,
            &st_state,
            ST_CAPACITY,
        )
        .unwrap();
        assert!(
            bad_circuit
                .check_circuit_satisfiability(public_inputs.as_ref())
                .is_err()
        );

        // bad path: bad stake table commitment
        let bad_lightclient_state = lightclient_state.clone();
        // bad_lightclient_state.stake_table_comm.1 = F::default();
        let bad_state_msg: [F; 3] = bad_lightclient_state.clone().into();
        let sig_for_bad_state = state_keys
            .iter()
            .map(|(key, _)| {
                SchnorrSignatureScheme::<Config>::sign(&(), key, bad_state_msg, &mut prng)
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let (bad_circuit, public_inputs) = build(
            &entries,
            &bit_vec,
            &sig_for_bad_state,
            &bad_lightclient_state,
            &st_state,
            ST_CAPACITY,
        )
        .unwrap();
        assert!(
            bad_circuit
                .check_circuit_satisfiability(public_inputs.as_ref())
                .is_err()
        );

        // bad path: incorrect signatures
        let wrong_light_client_state = lightclient_state.clone();
        // state with a different qc key commitment
        // wrong_light_client_state.stake_table_comm.0 = F::default();
        let wrong_state_msg: [F; 3] = wrong_light_client_state.into();
        let wrong_sigs = state_keys
            .iter()
            .map(|(key, _)| {
                SchnorrSignatureScheme::<Config>::sign(&(), key, wrong_state_msg, &mut prng)
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let (bad_circuit, public_inputs) = build(
            &entries,
            &bit_vec,
            &wrong_sigs,
            &lightclient_state,
            &st_state,
            ST_CAPACITY,
        )
        .unwrap();
        assert!(
            bad_circuit
                .check_circuit_satisfiability(public_inputs.as_ref())
                .is_err()
        );

        // bad path: overflowing stake table size
        assert!(
            build(
                &entries,
                &bit_vec,
                &bit_masked_sigs,
                &lightclient_state,
                &st_state,
                9
            )
            .is_err()
        );
    }
}
