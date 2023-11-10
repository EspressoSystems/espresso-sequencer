//! Circuit implementation for verifying light client state update

use std::marker::PhantomData;

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
    circuit::{rescue::RescueNativeGadget, signature::schnorr::VerKeyVar},
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
    pub schnorr_ver_key: VerKeyVar,
    pub stake_amount: Variable,
}

/// Light client state Variable
/// The stake table commitment is a triple (bls_keys_comm, schnorr_keys_comm, stake_amount_comm).
#[derive(Clone, Debug)]
pub struct LightClientStateVar {
    pub view_number_var: Variable,
    pub block_height_var: Variable,
    pub block_comm_var: Variable,
    pub fee_ledger_comm_var: Variable,
    pub stake_table_comm_var: (Variable, Variable, Variable),
}

#[derive(Clone, Debug)]
pub struct StateUpdateBuilder<F: RescueParameter>(PhantomData<F>);

impl<F> StateUpdateBuilder<F>
where
    F: RescueParameter,
{
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
    pub fn build<ST, P>(
        stake_table: &ST,
        _sigs: &[Signature<P>],
        lightclient_state: &LightClientState<F>,
        signer_bit_vec: &[bool],
        threshold: &U256,
    ) -> Result<(PlonkCircuit<F>, Vec<F>), PlonkError>
    where
        ST: StakeTableScheme<Key = BLSVerKey, Amount = U256, Aux = SchnorrVerKey<P>>,
        P: TECurveConfig<BaseField = F>,
    {
        let mut circuit = PlonkCircuit::new_turbo_plonk();

        // Dummy circuit implementation, fill in the details later
        // TODO(Chengyu):
        // - [DONE] the signer's accumulated weight exceeds the quorum threshold
        // - [DONE] The commitment of the stake table as [https://www.notion.so/espressosys/Light-Client-Contract-a416ebbfa9f342d79fccbf90de9706ef?pvs=4#6c0e26d753cd42e9bb0f22db1c519f45]
        // - Batch Schnorr signature verification

        // creating variables for stake table entries
        let mut stake_table_var = stake_table
            .try_iter(SnapshotVersion::LastEpochStart)?
            .map(|(_bls_ver_key, amount, schnorr_ver_key)| {
                let schnorr_ver_key =
                    VerKeyVar(circuit.create_point_variable(schnorr_ver_key.to_affine().into())?);
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

        let mut signer_bit_vec_var = signer_bit_vec
            .iter()
            .map(|&b| circuit.create_boolean_variable(b))
            .collect::<Result<Vec<_>, CircuitError>>()?;
        signer_bit_vec_var.resize(STAKE_TABLE_CAPACITY, BoolVar(circuit.zero()));

        let threshold = u256_to_field::<F>(threshold);
        let threshold_var = circuit.create_public_variable(threshold)?;

        let view_number_f = F::from(lightclient_state.view_number as u64);
        let block_height_f = F::from(lightclient_state.block_height as u64);
        let lightclient_state_var = LightClientStateVar {
            view_number_var: circuit.create_public_variable(view_number_f)?,
            block_height_var: circuit.create_public_variable(block_height_f)?,
            block_comm_var: circuit.create_public_variable(lightclient_state.block_comm)?,
            fee_ledger_comm_var: circuit
                .create_public_variable(lightclient_state.fee_ledger_comm)?,
            stake_table_comm_var: (
                circuit.create_public_variable(lightclient_state.stake_table_comm.0)?,
                circuit.create_public_variable(lightclient_state.stake_table_comm.1)?,
                circuit.create_public_variable(lightclient_state.stake_table_comm.2)?,
            ),
        };

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
            lightclient_state_var.stake_table_comm_var.1,
        )?;

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
            lightclient_state_var.stake_table_comm_var.2,
        )?;

        circuit.finalize_for_arithmetization()?;
        Ok((circuit, public_inputs))
    }
}

#[cfg(test)]
mod tests {
    use super::{LightClientState, StateUpdateBuilder};
    use ark_ed_on_bn254::EdwardsConfig as Config;
    use ethereum_types::U256;
    use hotshot_stake_table::vec_based::StakeTable;
    use hotshot_types::traits::stake_table::{SnapshotVersion, StakeTableScheme};
    use jf_primitives::signatures::{
        bls_over_bn254::{BLSOverBN254CurveSignatureScheme, VerKey as BLSVerKey},
        SchnorrSignatureScheme, SignatureScheme,
    };
    use jf_relation::Circuit;

    type F = ark_ed_on_bn254::Fq;
    type SchnorrVerKey = jf_primitives::signatures::schnorr::VerKey<Config>;

    fn key_pairs_for_testing() -> Vec<(BLSVerKey, SchnorrVerKey)> {
        let mut prng = jf_utils::test_rng();
        (0..10)
            .map(|_| {
                (
                    BLSOverBN254CurveSignatureScheme::key_gen(&(), &mut prng)
                        .unwrap()
                        .1,
                    SchnorrSignatureScheme::key_gen(&(), &mut prng).unwrap().1,
                )
            })
            .collect::<Vec<_>>()
    }

    fn stake_table_for_testing(
        keys: &[(BLSVerKey, SchnorrVerKey)],
    ) -> StakeTable<BLSVerKey, SchnorrVerKey, F> {
        let mut st = StakeTable::<BLSVerKey, SchnorrVerKey, F>::new();
        // Registering keys
        keys.iter().enumerate().for_each(|(i, key)| {
            st.register(key.0, U256::from((i + 1) as u32), key.1.clone())
                .unwrap()
        });
        // Freeze the stake table
        st.advance();
        st.advance();
        st
    }

    #[test]
    fn test_circuit_building() {
        let keys = key_pairs_for_testing();
        let st = stake_table_for_testing(&keys);

        let lightclient_state = LightClientState {
            view_number: 0,
            block_height: 0,
            block_comm: F::default(),
            fee_ledger_comm: F::default(),
            stake_table_comm: st.commitment(SnapshotVersion::LastEpochStart).unwrap(),
        };

        // bit vector with total weight 26
        let bit_vec = [
            true, true, true, false, true, true, false, false, true, false,
        ];
        // good path
        let (circuit, public_inputs) = StateUpdateBuilder::<F>::build(
            &st,
            &[],
            &lightclient_state,
            &bit_vec,
            &U256::from(26u32),
        )
        .unwrap();
        assert!(circuit.check_circuit_satisfiability(&public_inputs).is_ok());

        let (circuit, public_inputs) = StateUpdateBuilder::<F>::build(
            &st,
            &[],
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
        let (bad_circuit, public_inputs) = StateUpdateBuilder::<F>::build(
            &st,
            &[],
            &lightclient_state,
            &bad_bit_vec,
            &U256::from(25u32),
        )
        .unwrap();
        assert!(bad_circuit
            .check_circuit_satisfiability(&public_inputs)
            .is_err());

        // bad path: bad stake table commitment
        let bad_lightclient_state = LightClientState {
            view_number: 0,
            block_height: 0,
            block_comm: F::default(),
            fee_ledger_comm: F::default(),
            stake_table_comm: (F::default(), F::default(), F::default()),
        };
        let (bad_circuit, public_inputs) = StateUpdateBuilder::<F>::build(
            &st,
            &[],
            &bad_lightclient_state,
            &bit_vec,
            &U256::from(26u32),
        )
        .unwrap();
        assert!(bad_circuit
            .check_circuit_satisfiability(&public_inputs)
            .is_err());
    }
}
