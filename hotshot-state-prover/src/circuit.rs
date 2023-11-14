//! Circuit implementation for verifying light client state update

use ark_ec::twisted_edwards::TECurveConfig;
use ark_ff::PrimeField;
use ethereum_types::U256;
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;
use hotshot_types::traits::{
    stake_table::{SnapshotVersion, StakeTableScheme},
    state::LightClientState,
};
use jf_plonk::errors::PlonkError;
use jf_primitives::{
    circuit::{
        rescue::RescueNativeGadget,
        signature::schnorr::{SignatureGadget, SignatureVar, VerKeyVar},
    },
    rescue::RescueParameter,
    signatures::{
        bls_over_bn254::VerKey as BLSVerKey,
        schnorr::{Signature, VerKey as SchnorrVerKey},
    },
};
use jf_relation::{errors::CircuitError, BoolVar, Circuit, PlonkCircuit, Variable};

/// Lossy conversion of a U256 into a field element.
pub(crate) fn u256_to_field<F: PrimeField>(v: &U256) -> F {
    let mut bytes = vec![0u8; 32];
    v.to_little_endian(&mut bytes);
    F::from_le_bytes_mod_order(&bytes)
}

/// Variable for stake table entry
#[derive(Clone, Debug)]
pub struct StakeTableEntryVar {
    /// Schnorr verification keys
    pub schnorr_ver_key: VerKeyVar,
    /// Stake amount
    pub stake_amount: Variable,
}

/// Light client state Variable
/// The stake table commitment is a triple (bls_keys_comm, schnorr_keys_comm, stake_amount_comm).
/// Variable for a stake table commitment
#[derive(Clone, Debug)]
pub struct StakeTableCommVar {
    /// Commitment for BLS keys
    pub bls_keys_comm: Variable,
    /// Commitment for Schnorr keys
    pub schnorr_keys_comm: Variable,
    /// Commitment for stake amount
    pub stake_amount_comm: Variable,
}

/// Light client state Variable
#[derive(Clone, Debug)]
pub struct LightClientStateVar {
    /// Private list holding all variables
    ///  vars[0]: view number
    ///  vars[1]: block height
    ///  vars[2]: block commitment
    ///  vars[3]: fee ledger commitment
    ///  vars[4-6]: stake table commitment
    vars: [Variable; 7],
}

impl LightClientStateVar {
    pub fn new<F: PrimeField>(
        circuit: &mut PlonkCircuit<F>,
        state: &LightClientState<F>,
    ) -> Result<Self, CircuitError> {
        let view_number_f = F::from(state.view_number as u64);
        let block_height_f = F::from(state.block_height as u64);
        Ok(Self {
            vars: [
                circuit.create_public_variable(view_number_f)?,
                circuit.create_public_variable(block_height_f)?,
                circuit.create_public_variable(state.block_comm)?,
                circuit.create_public_variable(state.fee_ledger_comm)?,
                circuit.create_public_variable(state.stake_table_comm.0)?,
                circuit.create_public_variable(state.stake_table_comm.1)?,
                circuit.create_public_variable(state.stake_table_comm.2)?,
            ],
        })
    }

    pub fn view_number(&self) -> Variable {
        self.vars[0]
    }

    pub fn block_height(&self) -> Variable {
        self.vars[1]
    }

    pub fn block_comm(&self) -> Variable {
        self.vars[2]
    }

    pub fn fee_ledger_comm(&self) -> Variable {
        self.vars[3]
    }

    pub fn stake_table_comm(&self) -> StakeTableCommVar {
        StakeTableCommVar {
            bls_keys_comm: self.vars[4],
            schnorr_keys_comm: self.vars[5],
            stake_amount_comm: self.vars[6],
        }
    }
}

impl AsRef<[Variable]> for LightClientStateVar {
    fn as_ref(&self) -> &[Variable] {
        &self.vars
    }
}

/// A function that takes as input:
/// - stake table entries (`Vec<(BLSVerKey, Amount, SchnorrVerKey)>`)
/// - schnorr signatures of the updated states (`Vec<SchnorrSignature>`)
/// - updated light client state (`(view_number, block_height, block_comm, fee_ledger_comm, stake_table_comm)`)
/// - signer bit vector
/// - quorum threshold
/// checks that
/// - the signer's accumulated weight exceeds the quorum threshold
/// - the commitment of the stake table
/// - all schnorr signatures are valid
pub(crate) fn build_state_verifier_circuit<F, ST, P>(
    stake_table: &ST,
    sigs: &[Signature<P>],
    lightclient_state: &LightClientState<F>,
    signer_bit_vec: &[bool],
    threshold: &U256,
) -> Result<(PlonkCircuit<F>, Vec<F>), PlonkError>
where
    F: RescueParameter,
    ST: StakeTableScheme<Key = BLSVerKey, Amount = U256, Aux = SchnorrVerKey<P>>,
    P: TECurveConfig<BaseField = F>,
{
    let mut circuit = PlonkCircuit::new_turbo_plonk();

    // creating variables for stake table entries
    let mut stake_table_var = stake_table
        .try_iter(SnapshotVersion::LastEpochStart)?
        .map(|(_bls_ver_key, amount, schnorr_ver_key)| {
            let schnorr_ver_key = circuit.create_signature_vk_variable(&schnorr_ver_key)?;
            let stake_amount = circuit.create_variable(u256_to_field::<F>(&amount))?;
            Ok(StakeTableEntryVar {
                schnorr_ver_key,
                stake_amount,
            })
        })
        .collect::<Result<Vec<_>, CircuitError>>()?;
    let dummy_ver_key_var = VerKeyVar(circuit.neutral_point_variable());
    stake_table_var.resize(
        STAKE_TABLE_CAPACITY,
        StakeTableEntryVar {
            schnorr_ver_key: dummy_ver_key_var,
            stake_amount: 0,
        },
    );

    // creating variables for signatures
    let mut sig_vars = sigs
        .iter()
        .map(|sig| circuit.create_signature_variable(sig))
        .collect::<Result<Vec<_>, CircuitError>>()?;
    sig_vars.resize(
        STAKE_TABLE_CAPACITY,
        SignatureVar {
            s: circuit.zero(),
            R: circuit.neutral_point_variable(),
        },
    );

    // creating Boolean variables for the bit vector
    let mut signer_bit_vec_var = signer_bit_vec
        .iter()
        .map(|&b| circuit.create_boolean_variable(b))
        .collect::<Result<Vec<_>, CircuitError>>()?;
    signer_bit_vec_var.resize(STAKE_TABLE_CAPACITY, BoolVar(circuit.zero()));

    let threshold = u256_to_field::<F>(threshold);
    let threshold_var = circuit.create_public_variable(threshold)?;

    let lightclient_state_var = LightClientStateVar::new(&mut circuit, lightclient_state)?;

    let view_number_f = F::from(lightclient_state.view_number as u64);
    let block_height_f = F::from(lightclient_state.block_height as u64);
    let public_inputs = vec![
        threshold,
        view_number_f,
        block_height_f,
        lightclient_state.block_comm,
        lightclient_state.fee_ledger_comm,
        lightclient_state.stake_table_comm.0,
        lightclient_state.stake_table_comm.1,
        lightclient_state.stake_table_comm.2,
    ];

    // Checking whether the accumulated weight exceeds the quorum threshold
    let mut signed_amount_var = (0..STAKE_TABLE_CAPACITY / 2)
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
    // Adding the last if STAKE_TABLE_CAPACITY is not a multiple of 2
    if STAKE_TABLE_CAPACITY % 2 == 1 {
        signed_amount_var.push(circuit.mul(
            stake_table_var[STAKE_TABLE_CAPACITY - 1].stake_amount,
            signer_bit_vec_var[STAKE_TABLE_CAPACITY - 1].0,
        )?);
    }
    let acc_amount_var = circuit.sum(&signed_amount_var)?;
    circuit.enforce_leq(threshold_var, acc_amount_var)?;

    // checking the commitment for the list of schnorr keys
    let schnorr_ver_key_preimage_vars = stake_table_var
        .iter()
        .flat_map(|var| [var.schnorr_ver_key.0.get_x(), var.schnorr_ver_key.0.get_y()])
        .collect::<Vec<_>>();
    let schnorr_ver_key_comm = RescueNativeGadget::<F>::rescue_sponge_with_padding(
        &mut circuit,
        &schnorr_ver_key_preimage_vars,
        1,
    )?[0];
    circuit.enforce_equal(
        schnorr_ver_key_comm,
        lightclient_state_var.stake_table_comm().schnorr_keys_comm,
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
        lightclient_state_var.stake_table_comm().stake_amount_comm,
    )?;

    // checking all signatures
    let verification_result_vars = stake_table_var
        .iter()
        .zip(sig_vars)
        .map(|(entry, sig)| {
            SignatureGadget::<_, P>::check_signature_validity(
                &mut circuit,
                &entry.schnorr_ver_key,
                lightclient_state_var.as_ref(),
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
    Ok((circuit, public_inputs))
}

#[cfg(test)]
mod tests {
    use super::{build_state_verifier_circuit, LightClientState};
    use crate::utils::{key_pairs_for_testing, stake_table_for_testing};
    use ark_ed_on_bn254::EdwardsConfig as Config;
    use ethereum_types::U256;
    use hotshot_types::traits::stake_table::{SnapshotVersion, StakeTableScheme};
    use jf_primitives::{
        crhf::{VariableLengthRescueCRHF, CRHF},
        errors::PrimitivesError,
        signatures::{schnorr::Signature, SchnorrSignatureScheme, SignatureScheme},
    };
    use jf_relation::Circuit;
    use jf_utils::test_rng;

    type F = ark_ed_on_bn254::Fq;

    #[test]
    fn test_circuit_building() {
        let num_validators = 10;
        let mut prng = test_rng();

        let (bls_keys, schnorr_keys) = key_pairs_for_testing(num_validators, &mut prng);
        let st = stake_table_for_testing(&bls_keys, &schnorr_keys);

        let block_comm =
            VariableLengthRescueCRHF::<F, 1>::evaluate(vec![F::from(1u32), F::from(2u32)]).unwrap()
                [0];
        let fee_ledger_comm =
            VariableLengthRescueCRHF::<F, 1>::evaluate(vec![F::from(3u32), F::from(5u32)]).unwrap()
                [0];

        let lightclient_state = LightClientState {
            view_number: 100,
            block_height: 73,
            block_comm,
            fee_ledger_comm,
            stake_table_comm: st.commitment(SnapshotVersion::LastEpochStart).unwrap(),
        };
        let state_msg = [
            F::from(lightclient_state.view_number as u64),
            F::from(lightclient_state.block_height as u64),
            lightclient_state.block_comm,
            lightclient_state.fee_ledger_comm,
            lightclient_state.stake_table_comm.0,
            lightclient_state.stake_table_comm.1,
            lightclient_state.stake_table_comm.2,
        ];

        let sigs = schnorr_keys
            .iter()
            .map(|(key, _)| SchnorrSignatureScheme::<Config>::sign(&(), key, state_msg, &mut prng))
            .collect::<Result<Vec<_>, PrimitivesError>>()
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
        // good path
        let (circuit, public_inputs) = build_state_verifier_circuit(
            &st,
            &bit_masked_sigs,
            &lightclient_state,
            &bit_vec,
            &U256::from(26u32),
        )
        .unwrap();
        assert!(circuit.check_circuit_satisfiability(&public_inputs).is_ok());

        let (circuit, public_inputs) = build_state_verifier_circuit(
            &st,
            &bit_masked_sigs,
            &lightclient_state,
            &bit_vec,
            &U256::from(10u32),
        )
        .unwrap();
        assert!(circuit.check_circuit_satisfiability(&public_inputs).is_ok());

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
        let (bad_circuit, public_inputs) = build_state_verifier_circuit(
            &st,
            &bad_bit_masked_sigs,
            &lightclient_state,
            &bad_bit_vec,
            &U256::from(25u32),
        )
        .unwrap();
        assert!(bad_circuit
            .check_circuit_satisfiability(&public_inputs)
            .is_err());

        // bad path: bad stake table commitment
        let mut bad_lightclient_state = lightclient_state.clone();
        bad_lightclient_state.stake_table_comm.1 = F::default();
        let bad_state_msg = [
            F::from(bad_lightclient_state.view_number as u64),
            F::from(bad_lightclient_state.block_height as u64),
            bad_lightclient_state.block_comm,
            bad_lightclient_state.fee_ledger_comm,
            bad_lightclient_state.stake_table_comm.0,
            bad_lightclient_state.stake_table_comm.1,
            bad_lightclient_state.stake_table_comm.2,
        ];
        let sig_for_bad_state = schnorr_keys
            .iter()
            .map(|(key, _)| {
                SchnorrSignatureScheme::<Config>::sign(&(), key, bad_state_msg, &mut prng)
            })
            .collect::<Result<Vec<_>, PrimitivesError>>()
            .unwrap();
        let (bad_circuit, public_inputs) = build_state_verifier_circuit(
            &st,
            &sig_for_bad_state,
            &bad_lightclient_state,
            &bit_vec,
            &U256::from(26u32),
        )
        .unwrap();
        assert!(bad_circuit
            .check_circuit_satisfiability(&public_inputs)
            .is_err());

        // bad path: incorrect signatures
        let mut wrong_light_client_state = lightclient_state.clone();
        // state with a different bls key commitment
        wrong_light_client_state.stake_table_comm.0 = F::default();
        let wrong_state_msg = [
            F::from(wrong_light_client_state.view_number as u64),
            F::from(wrong_light_client_state.block_height as u64),
            wrong_light_client_state.block_comm,
            wrong_light_client_state.fee_ledger_comm,
            wrong_light_client_state.stake_table_comm.0,
            wrong_light_client_state.stake_table_comm.1,
            wrong_light_client_state.stake_table_comm.2,
        ];
        let wrong_sigs = schnorr_keys
            .iter()
            .map(|(key, _)| {
                SchnorrSignatureScheme::<Config>::sign(&(), key, wrong_state_msg, &mut prng)
            })
            .collect::<Result<Vec<_>, PrimitivesError>>()
            .unwrap();
        let (bad_circuit, public_inputs) = build_state_verifier_circuit(
            &st,
            &wrong_sigs,
            &lightclient_state,
            &bit_vec,
            &U256::from(26u32),
        )
        .unwrap();
        assert!(bad_circuit
            .check_circuit_satisfiability(&public_inputs)
            .is_err());
    }
}
