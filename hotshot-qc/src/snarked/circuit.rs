//! Circuit implementation of stake key aggregation for quorum certificates verification.

use ark_ff::PrimeField;
use ark_std::{format, vec, vec::Vec};
use jf_relation::{
    errors::CircuitError,
    gadgets::{
        ecc::{
            emulated::{EmulatedSWPointVariable, EmulatedTEPointVariable, SWPoint},
            TEPoint,
        },
        EmulationConfig, SerializableEmulatedStruct,
    },
    BoolVar, Circuit, PlonkCircuit, Variable,
};
use jf_rescue::{crhf::RescueCRHF, gadgets::RescueNativeGadget, RescueParameter};

/// Digest a list of verification keys and their associated stake amounts
/// * `stack_amts` - stake amounts
/// * `keys` - list of verification keys
#[allow(dead_code)]
pub fn compute_stake_table_hash<F: RescueParameter, T: SerializableEmulatedStruct<F>>(
    stake_amts: &[F],
    keys: &[T],
) -> F {
    let mut input_vec = vec![];
    for (&amt, key) in stake_amts.iter().zip(keys.iter()) {
        input_vec.extend(key.serialize_to_native_elements());
        input_vec.push(amt);
    }
    RescueCRHF::sponge_with_bit_padding(&input_vec[..], 1)[0]
}

/// Traits for verification keys
pub trait VerKeyVar<E>: Sized + Clone {
    /// The type of key
    type KeyType: Default;

    /// Returns a list of variables associated with this key variable.
    fn native_vars(&self) -> Vec<Variable>;

    /// Aggregate the verification keys with Boolean selectors.
    /// * `circuit` - associated Plonk circuit.
    /// * `keys` - list of input verification keys.
    /// * `selectors` - list of Boolean selectors.
    /// * `coef` - the internal curve parameter.
    /// * Returns an aggregated key variable.
    fn aggregate_with_selectors<F>(
        circuit: &mut PlonkCircuit<F>,
        keys: &[Self],
        selectors: &[BoolVar],
        coef: E,
    ) -> Result<Self, CircuitError>
    where
        F: PrimeField,
        E: EmulationConfig<F>;

    /// Check whether two input verification key variables are equal.
    /// * `circuit` - associated Plonk circuit.
    /// * `p0` - first verification key variable.
    /// * `p1` - second verification key variable.
    /// * Returns a Boolean variable indicates whether `p0 == p1`.
    fn is_equal<F>(
        circuit: &mut PlonkCircuit<F>,
        p0: &Self,
        p1: &Self,
    ) -> Result<BoolVar, CircuitError>
    where
        F: PrimeField,
        E: EmulationConfig<F>;

    /// Enforce that two input verification key variables are equal.
    /// * `circuit` - associated Plonk circuit.
    /// * `p0` - first verification key variable.
    /// * `p1` - second verification key variable.
    fn enforce_equal<F>(
        circuit: &mut PlonkCircuit<F>,
        p0: &Self,
        p1: &Self,
    ) -> Result<(), CircuitError>
    where
        F: PrimeField,
        E: EmulationConfig<F>;
}

/// Plonk circuit gadget for stake key aggregation for quorum certificates.
pub trait QcKeyAggregateGadget<F>
where
    F: RescueParameter,
{
    /// Key aggregation circuit
    /// * `vks` - list of stake public keys.
    /// * `bit_vec` - the indicator vector for the quorum set, `bit_vec[i] = 1` if `i` is in the quorum set, o/w `bit_vec[i] = 0`.
    /// * `agg_vk` - the public aggregated stake key.
    /// * `coef` - the internal curve parameter
    fn check_aggregate_vk<E: EmulationConfig<F>, V: VerKeyVar<E>>(
        &mut self,
        vks: &[V],
        bit_vec: &[BoolVar],
        agg_vk: &V,
        coef: E,
    ) -> Result<(), CircuitError>;

    /// Stake table commitment checking circuit
    /// * `vk` - list of stake public keys.
    /// * `stake_amts` - list of stake amounts for the corresponding stake keys.
    /// * `digest` - the hash of the stake table.
    fn check_stake_table_digest<E: EmulationConfig<F>, V: VerKeyVar<E>>(
        &mut self,
        vks: &[V],
        stake_amts: &[Variable],
        digest: Variable,
    ) -> Result<(), CircuitError>;

    /// Quorum threshold checking circuit
    /// * `stake_amts` - list of stake amounts for the corresponding stake keys.
    /// * `bit_vec` - the indicator vector for the quorum set.
    /// * `threshold` - the public quorum threshold.
    fn check_threshold(
        &mut self,
        stake_amts: &[Variable],
        bit_vec: &[BoolVar],
        threshold: Variable,
    ) -> Result<(), CircuitError>;
}

impl<F> QcKeyAggregateGadget<F> for PlonkCircuit<F>
where
    F: RescueParameter,
{
    fn check_aggregate_vk<E: EmulationConfig<F>, V: VerKeyVar<E>>(
        &mut self,
        vks: &[V],
        bit_vec: &[BoolVar],
        agg_vk: &V,
        coef: E,
    ) -> Result<(), CircuitError> {
        if vks.len() != bit_vec.len() {
            return Err(CircuitError::ParameterError(format!(
                "bit vector len {} != the number of stake keys {}",
                bit_vec.len(),
                vks.len(),
            )));
        }
        let agg_key_var = V::aggregate_with_selectors::<F>(self, vks, bit_vec, coef)?;
        V::enforce_equal(self, &agg_key_var, agg_vk)
    }

    fn check_stake_table_digest<E: EmulationConfig<F>, V: VerKeyVar<E>>(
        &mut self,
        vks: &[V],
        stake_amts: &[Variable],
        digest: Variable,
    ) -> Result<(), CircuitError> {
        if stake_amts.len() != vks.len() {
            return Err(CircuitError::ParameterError(format!(
                "the number of stake amounts {} != the number of stake verification keys {}",
                stake_amts.len(),
                vks.len(),
            )));
        }
        let mut hash_input = vec![];
        for (vk, &stake_amt) in vks.iter().zip(stake_amts.iter()) {
            hash_input.append(&mut vk.native_vars());
            hash_input.push(stake_amt);
        }
        let expected_digest =
            RescueNativeGadget::<F>::rescue_sponge_with_padding(self, &hash_input, 1)?[0];
        self.enforce_equal(expected_digest, digest)
    }

    fn check_threshold(
        &mut self,
        stake_amts: &[Variable],
        bit_vec: &[BoolVar],
        threshold: Variable,
    ) -> Result<(), CircuitError> {
        if stake_amts.len() != bit_vec.len() {
            return Err(CircuitError::ParameterError(format!(
                "bit vector len {} != the number of stake entries {}",
                bit_vec.len(),
                stake_amts.len(),
            )));
        }
        let mut active_amts = vec![];
        for (&stake_amt, &bit) in stake_amts.iter().zip(bit_vec.iter()) {
            active_amts.push(self.mul(stake_amt, bit.into())?);
        }
        let sum = self.sum(&active_amts[..])?;
        self.enforce_geq(sum, threshold)
    }
}

impl<E> VerKeyVar<E> for EmulatedSWPointVariable<E>
where
    E: PrimeField,
{
    type KeyType = SWPoint<E>;

    fn native_vars(&self) -> Vec<Variable> {
        let mut ret = self.0.native_vars();
        ret.append(&mut self.1.native_vars());
        ret.push(self.2 .0);
        ret
    }

    fn aggregate_with_selectors<F>(
        circuit: &mut PlonkCircuit<F>,
        keys: &[Self],
        selectors: &[BoolVar],
        coef: E,
    ) -> Result<Self, CircuitError>
    where
        F: PrimeField,
        E: EmulationConfig<F>,
    {
        let neutral_point = Self::KeyType::default();
        let emulated_neutral_point_var =
            circuit.create_constant_emulated_sw_point_variable(neutral_point)?;
        let mut agg_key_var = emulated_neutral_point_var.clone();
        for (key, &bit) in keys.iter().zip(selectors.iter()) {
            let point_var = circuit.binary_emulated_sw_point_vars_select(
                bit,
                &emulated_neutral_point_var,
                key,
            )?;
            agg_key_var = circuit.emulated_sw_ecc_add::<E>(&agg_key_var, &point_var, coef)?;
        }
        Ok(agg_key_var)
    }

    fn is_equal<F>(
        circuit: &mut PlonkCircuit<F>,
        p0: &Self,
        p1: &Self,
    ) -> Result<BoolVar, CircuitError>
    where
        F: PrimeField,
        E: EmulationConfig<F>,
    {
        circuit.is_emulated_sw_point_equal(p0, p1)
    }

    fn enforce_equal<F>(
        circuit: &mut PlonkCircuit<F>,
        p0: &Self,
        p1: &Self,
    ) -> Result<(), CircuitError>
    where
        F: PrimeField,
        E: EmulationConfig<F>,
    {
        circuit.enforce_emulated_sw_point_equal(p0, p1)
    }
}

impl<E> VerKeyVar<E> for EmulatedTEPointVariable<E>
where
    E: PrimeField,
{
    type KeyType = TEPoint<E>;

    fn native_vars(&self) -> Vec<Variable> {
        let mut ret = self.0.native_vars();
        ret.append(&mut self.1.native_vars());
        ret
    }

    fn aggregate_with_selectors<F>(
        circuit: &mut PlonkCircuit<F>,
        keys: &[Self],
        selectors: &[BoolVar],
        coef: E,
    ) -> Result<Self, CircuitError>
    where
        F: PrimeField,
        E: EmulationConfig<F>,
    {
        let neutral_point = Self::KeyType::default();
        let emulated_neutral_point_var =
            circuit.create_constant_emulated_te_point_variable(neutral_point)?;
        let mut agg_key_var = emulated_neutral_point_var.clone();
        for (key, &bit) in keys.iter().zip(selectors.iter()) {
            let point_var = circuit.binary_emulated_te_point_vars_select(
                bit,
                &emulated_neutral_point_var,
                key,
            )?;
            agg_key_var = circuit.emulated_te_ecc_add::<E>(&agg_key_var, &point_var, coef)?;
        }
        Ok(agg_key_var)
    }

    fn is_equal<F>(
        circuit: &mut PlonkCircuit<F>,
        p0: &Self,
        p1: &Self,
    ) -> Result<BoolVar, CircuitError>
    where
        F: PrimeField,
        E: EmulationConfig<F>,
    {
        circuit.is_emulated_te_point_equal(p0, p1)
    }

    fn enforce_equal<F>(
        circuit: &mut PlonkCircuit<F>,
        p0: &Self,
        p1: &Self,
    ) -> Result<(), CircuitError>
    where
        F: PrimeField,
        E: EmulationConfig<F>,
    {
        circuit.enforce_emulated_te_point_equal(p0, p1)
    }
}

#[cfg(test)]
mod tests {
    use ark_bls12_377::{g1::Config as Param377, Fq as Fq377};
    use ark_bn254::{g1::Config as Param254, Fq as Fq254, Fr as Fr254};
    use ark_ec::{
        short_weierstrass::{Projective, SWCurveConfig},
        CurveGroup,
    };
    use ark_ff::MontFp;
    use ark_std::{vec::Vec, UniformRand, Zero};
    use jf_relation::{
        errors::CircuitError, gadgets::ecc::SWToTEConParam, Circuit, PlonkCircuit, Variable,
    };

    use super::*;

    #[test]
    fn crypto_test_vk_aggregate_sw_circuit() -> Result<(), CircuitError> {
        let a_ecc = Fq377::zero();
        test_vk_aggregate_sw_circuit_helper::<Fq377, Fr254, Param377>(a_ecc)?;
        let a_ecc = Fq254::zero();
        test_vk_aggregate_sw_circuit_helper::<Fq254, Fr254, Param254>(a_ecc)
    }

    // TODO: use Aggregate signature APIs to aggregate the keys outside the circuit
    fn test_vk_aggregate_sw_circuit_helper<E, F, P>(a_ecc: E) -> Result<(), CircuitError>
    where
        E: EmulationConfig<F>,
        F: RescueParameter,
        P: SWCurveConfig<BaseField = E>,
    {
        let mut rng = jf_utils::test_rng();
        let vk_points: Vec<Projective<P>> =
            (0..5).map(|_| Projective::<P>::rand(&mut rng)).collect();
        let selector = [false, true, false, true, false];
        let agg_vk_point =
            vk_points
                .iter()
                .zip(selector.iter())
                .fold(
                    Projective::<P>::zero(),
                    |acc, (x, &b)| {
                        if b {
                            acc + x
                        } else {
                            acc
                        }
                    },
                );
        let agg_vk_point: SWPoint<E> = agg_vk_point.into_affine().into();
        let vk_points: Vec<SWPoint<E>> = vk_points.iter().map(|p| p.into_affine().into()).collect();
        #[allow(clippy::cast_sign_loss)]
        let stake_amts: Vec<F> = (0..5).map(|i| F::from((i + 1) as u32)).collect();
        let threshold = F::from(6u8);
        let digest = compute_stake_table_hash::<F, SWPoint<E>>(&stake_amts[..], &vk_points[..]);

        let mut circuit = PlonkCircuit::<F>::new_ultra_plonk(20);
        // public input
        let agg_vk_var = circuit.create_public_emulated_sw_point_variable(agg_vk_point)?;
        let public_input = agg_vk_point.serialize_to_native_elements();
        let threshold_var = circuit.create_variable(threshold)?;
        let digest_var = circuit.create_variable(digest)?;

        // add witness
        let vk_vars: Vec<EmulatedSWPointVariable<E>> = vk_points
            .iter()
            .map(|&p| circuit.create_emulated_sw_point_variable(p).unwrap())
            .collect();
        let stake_amt_vars: Vec<Variable> = stake_amts
            .iter()
            .map(|&amt| circuit.create_variable(amt).unwrap())
            .collect();
        let selector_vars: Vec<BoolVar> = selector
            .iter()
            .map(|&b| circuit.create_boolean_variable(b).unwrap())
            .collect();
        // add circuit gadgets
        circuit.check_aggregate_vk::<E, EmulatedSWPointVariable<E>>(
            &vk_vars[..],
            &selector_vars[..],
            &agg_vk_var,
            a_ecc,
        )?;
        circuit.check_stake_table_digest(&vk_vars[..], &stake_amt_vars[..], digest_var)?;
        circuit.check_threshold(&stake_amt_vars[..], &selector_vars[..], threshold_var)?;
        assert!(circuit.check_circuit_satisfiability(&public_input).is_ok());

        // bad path: wrong aggregated vk
        let tmp_var = agg_vk_var.native_vars()[0];
        let tmp = circuit.witness(tmp_var)?;
        *circuit.witness_mut(tmp_var) = F::zero();
        assert!(circuit.check_circuit_satisfiability(&public_input).is_err());
        *circuit.witness_mut(tmp_var) = tmp;

        // bad path: wrong digest
        let tmp = circuit.witness(digest_var)?;
        *circuit.witness_mut(digest_var) = F::zero();
        assert!(circuit.check_circuit_satisfiability(&public_input).is_err());
        *circuit.witness_mut(digest_var) = tmp;

        // bad path: bad threshold
        *circuit.witness_mut(threshold_var) = F::from(7u8);
        assert!(circuit.check_circuit_satisfiability(&public_input).is_err());

        // check input parameter errors
        assert!(circuit
            .check_aggregate_vk::<E, EmulatedSWPointVariable<E>>(
                &vk_vars[..],
                &selector_vars[1..],
                &agg_vk_var,
                a_ecc
            )
            .is_err());
        assert!(circuit
            .check_stake_table_digest(&vk_vars[..], &stake_amt_vars[1..], digest_var)
            .is_err());
        assert!(circuit
            .check_threshold(&stake_amt_vars[..], &selector_vars[1..], threshold_var)
            .is_err());

        Ok(())
    }

    #[test]
    fn crypto_test_vk_aggregate_te_circuit() -> Result<(), CircuitError> {
        let d_ecc : Fq377 = MontFp!("122268283598675559488486339158635529096981886914877139579534153582033676785385790730042363341236035746924960903179");
        test_vk_aggregate_te_circuit_helper::<Fq377, Fr254, Param377>(d_ecc)
    }

    // TODO: use Aggregate signature APIs to aggregate the keys outside the circuit
    fn test_vk_aggregate_te_circuit_helper<E, F, P>(d_ecc: E) -> Result<(), CircuitError>
    where
        E: EmulationConfig<F> + SWToTEConParam,
        F: RescueParameter,
        P: SWCurveConfig<BaseField = E>,
    {
        let mut rng = jf_utils::test_rng();
        let vk_points: Vec<Projective<P>> =
            (0..5).map(|_| Projective::<P>::rand(&mut rng)).collect();
        let selector = [false, true, false, true, false];
        let agg_vk_point =
            vk_points
                .iter()
                .zip(selector.iter())
                .fold(
                    Projective::<P>::zero(),
                    |acc, (x, &b)| {
                        if b {
                            acc + x
                        } else {
                            acc
                        }
                    },
                );
        let agg_vk_point: TEPoint<E> = agg_vk_point.into_affine().into();
        let vk_points: Vec<TEPoint<E>> = vk_points.iter().map(|p| p.into_affine().into()).collect();
        #[allow(clippy::cast_sign_loss)]
        let stake_amts: Vec<F> = (0..5).map(|i| F::from((i + 1) as u32)).collect();
        let threshold = F::from(6u8);
        let digest = compute_stake_table_hash::<F, TEPoint<E>>(&stake_amts[..], &vk_points[..]);

        let mut circuit = PlonkCircuit::<F>::new_ultra_plonk(20);
        // public input
        let agg_vk_var = circuit.create_public_emulated_te_point_variable(agg_vk_point)?;
        let public_input = agg_vk_point.serialize_to_native_elements();
        let threshold_var = circuit.create_variable(threshold)?;
        let digest_var = circuit.create_variable(digest)?;

        // add witness
        let vk_vars: Vec<EmulatedTEPointVariable<E>> = vk_points
            .iter()
            .map(|&p| circuit.create_emulated_te_point_variable(p).unwrap())
            .collect();
        let stake_amt_vars: Vec<Variable> = stake_amts
            .iter()
            .map(|&amt| circuit.create_variable(amt).unwrap())
            .collect();
        let selector_vars: Vec<BoolVar> = selector
            .iter()
            .map(|&b| circuit.create_boolean_variable(b).unwrap())
            .collect();
        // add circuit gadgets
        circuit.check_aggregate_vk::<E, EmulatedTEPointVariable<E>>(
            &vk_vars[..],
            &selector_vars[..],
            &agg_vk_var,
            d_ecc,
        )?;
        circuit.check_stake_table_digest(&vk_vars[..], &stake_amt_vars[..], digest_var)?;
        circuit.check_threshold(&stake_amt_vars[..], &selector_vars[..], threshold_var)?;
        assert!(circuit.check_circuit_satisfiability(&public_input).is_ok());

        // bad path: wrong aggregated vk
        let tmp_var = agg_vk_var.native_vars()[0];
        let tmp = circuit.witness(tmp_var)?;
        *circuit.witness_mut(tmp_var) = F::zero();
        assert!(circuit.check_circuit_satisfiability(&public_input).is_err());
        *circuit.witness_mut(tmp_var) = tmp;

        // bad path: wrong digest
        let tmp = circuit.witness(digest_var)?;
        *circuit.witness_mut(digest_var) = F::zero();
        assert!(circuit.check_circuit_satisfiability(&public_input).is_err());
        *circuit.witness_mut(digest_var) = tmp;

        // bad path: bad threshold
        *circuit.witness_mut(threshold_var) = F::from(7u8);
        assert!(circuit.check_circuit_satisfiability(&public_input).is_err());

        // check input parameter errors
        assert!(circuit
            .check_aggregate_vk::<E, EmulatedTEPointVariable<E>>(
                &vk_vars[..],
                &selector_vars[1..],
                &agg_vk_var,
                d_ecc
            )
            .is_err());
        assert!(circuit
            .check_stake_table_digest(&vk_vars[..], &stake_amt_vars[1..], digest_var)
            .is_err());
        assert!(circuit
            .check_threshold(&stake_amt_vars[..], &selector_vars[1..], threshold_var)
            .is_err());

        Ok(())
    }
}
