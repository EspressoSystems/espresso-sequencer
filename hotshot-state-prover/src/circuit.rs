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
    circuit::signature::schnorr::VerKeyVar,
    rescue::RescueParameter,
    signatures::{
        bls_over_bn254::VerKey as BLSVerKey,
        schnorr::{Signature, VerKey as SchnorrVerKey},
    },
};
use jf_relation::{
    errors::CircuitError, gadgets::ecc::TEPoint, BoolVar, Circuit, PlonkCircuit, Variable,
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
    pub schnorr_ver_key: VerKeyVar,
    pub stake_amount: Variable,
}

/// HotShot state Variable
/// The stake table commitment is a triple (bls_keys_comm, stake_amount_comm, schnorr_keys_comm).
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
    /// - updated hotshot state (`(view_number, block_height, block_comm, fee_ledger_comm, stake_table_comm)`)
    /// - signer bit vector
    /// - quorum threshold
    /// checks that
    /// - the signer's accumulated weight exceeds the quorum threshold
    /// - the commitment of the stake table
    /// - all schnorr signatures are valid
    pub fn build<ST, P>(
        stake_table: &ST,
        _sigs: &[Signature<P>],
        _hotshot_state: &LightClientState<F>,
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
        // - The commitment of the stake table as [https://www.notion.so/espressosys/Light-Client-Contract-a416ebbfa9f342d79fccbf90de9706ef?pvs=4#6c0e26d753cd42e9bb0f22db1c519f45]
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
        let dummy_ver_key_var =
            VerKeyVar(circuit.create_constant_point_variable(TEPoint::default())?);
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

        // TODO(Chengyu): put in the hotshot state
        let public_inputs = vec![threshold];

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

        // circuit.mul_add(wires_in, q_muls)
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
    use hotshot_types::traits::stake_table::StakeTableScheme;
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

        // bit vector with total weight 26
        let bit_vec = [
            true, true, true, false, true, true, false, false, true, false,
        ];
        // good path
        let (circuit, public_inputs) = StateUpdateBuilder::<F>::build(
            &st,
            &[],
            &LightClientState::default(),
            &bit_vec,
            &U256::from(25u32),
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
            &LightClientState::default(),
            &bad_bit_vec,
            &U256::from(25u32),
        )
        .unwrap();
        assert!(bad_circuit
            .check_circuit_satisfiability(&public_inputs)
            .is_err());
    }
}
