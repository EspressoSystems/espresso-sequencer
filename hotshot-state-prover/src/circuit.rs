//! Circuit implementation for verifying light client state update

use ark_ec::twisted_edwards::TECurveConfig;
use ark_ff::PrimeField;
use ark_std::borrow::Borrow;
use ethers::types::U256;
use hotshot_types::light_client::{GenericLightClientState, GenericPublicInput};
use jf_plonk::PlonkError;
use jf_relation::{BoolVar, Circuit, CircuitError, PlonkCircuit, Variable};
use jf_rescue::RescueParameter;
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
pub struct StakeTableCommVar {
    /// Commitment for QC verification keys
    pub qc_keys_comm: Variable,
    /// Commitment for state verification keys
    pub state_keys_comm: Variable,
    /// Commitment for stake amount
    pub stake_amount_comm: Variable,
}

/// Light client state Variable
#[derive(Clone, Debug)]
pub struct LightClientStateVar {
    /// Private list holding all variables
    ///  `vars[0]`: view number
    ///  `vars[1]`: block height
    ///  `vars[2]`: block commitment root
    vars: [Variable; 3],
}

impl LightClientStateVar {
    /// # Errors
    /// if unable to create any of the public variables
    pub fn new<F: PrimeField>(
        circuit: &mut PlonkCircuit<F>,
        state: &GenericLightClientState<F>,
    ) -> Result<Self, CircuitError> {
        let view_number_f = F::from(state.view_number as u64);
        let block_height_f = F::from(state.block_height as u64);
        Ok(Self {
            vars: [
                circuit.create_public_variable(view_number_f)?,
                circuit.create_public_variable(block_height_f)?,
                circuit.create_public_variable(state.block_comm_root)?,
            ],
        })
    }

    /// Returns the view number
    #[must_use]
    pub fn view_number(&self) -> Variable {
        self.vars[0]
    }

    /// Returns the block height
    #[must_use]
    pub fn block_height(&self) -> Variable {
        self.vars[1]
    }

    /// Returns the Merkle root of the block commitments
    #[must_use]
    pub fn block_comm_root(&self) -> Variable {
        self.vars[2]
    }
}

impl AsRef<[Variable]> for LightClientStateVar {
    fn as_ref(&self) -> &[Variable] {
        &self.vars
    }
}

/// A function that takes as input:
/// - a list of stake table entries (`Vec<(SchnorrVerKey, Amount)>`)
/// - a bit vector indicates the signers
/// - a list of schnorr signatures of the updated states (`Vec<SchnorrSignature>`), default if the node doesn't sign the state
/// - updated light client state (`(view_number, block_height, block_comm_root, fee_ledger_comm, stake_table_comm)`)
/// - a quorum threshold
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
    threshold: &U256,
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

    let threshold = u256_to_field::<F>(threshold);
    let threshold_pub_var = circuit.create_public_variable(threshold)?;

    let lightclient_state_pub_var = LightClientStateVar::new(&mut circuit, lightclient_state)?;

    let view_number_f = F::from(lightclient_state.view_number as u64);
    let block_height_f = F::from(lightclient_state.block_height as u64);
    let public_inputs = vec![
        threshold,
        view_number_f,
        block_height_f,
        lightclient_state.block_comm_root,
    ];

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
    let acc_amount_var = circuit.sum(&signed_amount_var)?;
    circuit.enforce_leq(threshold_pub_var, acc_amount_var)?;

    // checking all signatures
    let verification_result_vars = stake_table_var
        .iter()
        .zip(sig_vars)
        .map(|(entry, sig)| {
            SignatureGadget::<_, P>::check_signature_validity(
                &mut circuit,
                &entry.state_ver_key,
                lightclient_state_pub_var.as_ref(),
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
    Ok((circuit, public_inputs.into()))
}

/// Internal function to build a dummy circuit
pub(crate) fn build_for_preprocessing<F, P>(
    stake_table_capacity: usize,
) -> Result<(PlonkCircuit<F>, GenericPublicInput<F>), PlonkError>
where
    F: RescueParameter,
    P: TECurveConfig<BaseField = F>,
{
    let lightclient_state = GenericLightClientState {
        view_number: 0,
        block_height: 0,
        block_comm_root: F::default(),
    };
    build::<F, P, _, _, _>(
        &[],
        &[],
        &[],
        &lightclient_state,
        &U256::zero(),
        stake_table_capacity,
    )
}

#[cfg(test)]
mod tests {
    use ark_ed_on_bn254::EdwardsConfig as Config;
    use ethers::types::U256;
    use hotshot_types::traits::stake_table::{SnapshotVersion, StakeTableScheme};
    use jf_crhf::CRHF;
    use jf_relation::Circuit;
    use jf_rescue::crhf::VariableLengthRescueCRHF;
    use jf_signature::{
        schnorr::{SchnorrSignatureScheme, Signature},
        SignatureScheme,
    };
    use jf_utils::test_rng;

    use super::{build, GenericLightClientState};
    use crate::test_utils::{key_pairs_for_testing, stake_table_for_testing};

    type F = ark_ed_on_bn254::Fq;
    const ST_CAPACITY: usize = 20;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_circuit_building() {
        let num_validators = 10;
        let mut prng = test_rng();

        let (qc_keys, state_keys) = key_pairs_for_testing(num_validators, &mut prng);
        let st = stake_table_for_testing(ST_CAPACITY, &qc_keys, &state_keys);

        let entries = st
            .try_iter(SnapshotVersion::LastEpochStart)
            .unwrap()
            .map(|(_, stake_amount, state_key)| (state_key, stake_amount))
            .collect::<Vec<_>>();

        let block_comm_root =
            VariableLengthRescueCRHF::<F, 1>::evaluate(vec![F::from(1u32), F::from(2u32)]).unwrap()
                [0];
        let lightclient_state = GenericLightClientState {
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
            &U256::from(26u32),
            ST_CAPACITY,
        )
        .unwrap();
        assert!(circuit
            .check_circuit_satisfiability(public_inputs.as_ref())
            .is_ok());

        let (circuit, public_inputs) = build(
            &entries,
            &bit_vec,
            &bit_masked_sigs,
            &lightclient_state,
            &U256::from(10u32),
            ST_CAPACITY,
        )
        .unwrap();
        assert!(circuit
            .check_circuit_satisfiability(public_inputs.as_ref())
            .is_ok());

        // bad path: feeding non-bit vector
        let bit_vec = [F::from(2u64); 10];
        let (circuit, public_inputs) = build(
            &entries,
            &bit_vec,
            &bit_masked_sigs,
            &lightclient_state,
            &U256::from(26u32),
            ST_CAPACITY,
        )
        .unwrap();
        assert!(circuit
            .check_circuit_satisfiability(public_inputs.as_ref())
            .is_err());

        // bad path: total weight doesn't meet the threshold
        // bit vector with total weight 23
        let bad_bit_vec = [
            true, true, true, true, true, false, false, true, false, false,
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
            &U256::from(25u32),
            ST_CAPACITY,
        )
        .unwrap();
        assert!(bad_circuit
            .check_circuit_satisfiability(public_inputs.as_ref())
            .is_err());

        // bad path: bad stake table commitment
        let bad_lightclient_state = lightclient_state.clone();
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
            &U256::from(26u32),
            ST_CAPACITY,
        )
        .unwrap();
        assert!(bad_circuit
            .check_circuit_satisfiability(public_inputs.as_ref())
            .is_err());

        // bad path: incorrect signatures
        let wrong_light_client_state = lightclient_state.clone();
        // state with a different qc key commitment
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
            &U256::from(26u32),
            ST_CAPACITY,
        )
        .unwrap();
        assert!(bad_circuit
            .check_circuit_satisfiability(public_inputs.as_ref())
            .is_err());

        // bad path: overflowing stake table size
        assert!(build(
            &entries,
            &bit_vec,
            &bit_masked_sigs,
            &lightclient_state,
            &U256::from(26u32),
            9
        )
        .is_err());
    }
}
