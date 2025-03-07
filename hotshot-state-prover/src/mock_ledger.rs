use std::collections::HashMap;

use anyhow::Result;
use ark_bn254::Bn254;
use ark_ed_on_bn254::EdwardsConfig;
use ark_ff::PrimeField;
use ark_std::{
    rand::{rngs::StdRng, CryptoRng, Rng, RngCore},
    UniformRand,
};
use ethers::types::U256;
use hotshot_contract_adapter::jellyfish::{field_to_u256, open_key};
use hotshot_stake_table::vec_based::StakeTable;
use hotshot_types::utils::is_last_block_in_epoch;
use hotshot_types::{
    light_client::{
        GenericLightClientState, GenericPublicInput, GenericStakeTableState, LightClientState,
    },
    traits::stake_table::{SnapshotVersion, StakeTableScheme},
};
use itertools::izip;
use jf_pcs::prelude::UnivariateUniversalParams;
use jf_plonk::{
    proof_system::{PlonkKzgSnark, UniversalSNARK},
    transcript::SolidityTranscript,
};
use jf_relation::{Arithmetization, Circuit, PlonkCircuit};
use jf_signature::{
    bls_over_bn254::{BLSOverBN254CurveSignatureScheme, VerKey as BLSVerKey},
    schnorr::{SchnorrSignatureScheme, Signature},
    SignatureScheme,
};
use jf_utils::test_rng;

use crate::{generate_state_update_proof, preprocess, Proof, VerifyingKey};

type F = ark_ed_on_bn254::Fq;
type SchnorrVerKey = jf_signature::schnorr::VerKey<EdwardsConfig>;
type SchnorrSignKey = jf_signature::schnorr::SignKey<ark_ed_on_bn254::Fr>;

/// Stake table capacity used for testing
pub const STAKE_TABLE_CAPACITY_FOR_TEST: usize = 10;
/// Number of block per epoch for testing
pub const EPOCH_HEIGHT_FOR_TEST: usize = 4;

/// Mock for system parameter of `MockLedger`
pub struct MockSystemParam {
    /// max capacity of stake table
    st_cap: usize,
    /// number of block per epoch
    epoch_height: usize,
}

impl MockSystemParam {
    /// Init the system parameters (some fixed, some adjustable)
    pub fn init() -> Self {
        Self {
            st_cap: STAKE_TABLE_CAPACITY_FOR_TEST,
            epoch_height: EPOCH_HEIGHT_FOR_TEST,
        }
    }
}

/// Mock of hotshot ledger for testing LightClient.sol functionalities only.
/// Its logic is completely divergent from a real light client or HotShot
pub struct MockLedger {
    pp: MockSystemParam,
    pub rng: StdRng,
    epoch: u64,
    state: GenericLightClientState<F>,
    pub(crate) st: StakeTable<BLSVerKey, SchnorrVerKey, F>,
    pub(crate) qc_keys: Vec<BLSVerKey>,
    pub(crate) state_keys: Vec<(SchnorrSignKey, SchnorrVerKey)>,
    key_archive: HashMap<BLSVerKey, SchnorrSignKey>,
}

impl MockLedger {
    /// Initialize the ledger with genesis state
    pub fn init(pp: MockSystemParam, num_validators: usize) -> Self {
        // credit: https://github.com/EspressoSystems/HotShot/blob/5554b7013b00e6034691b533299b44f3295fa10d/crates/hotshot-state-prover/src/lib.rs#L176
        let mut rng = test_rng();
        let (qc_keys, state_keys) = key_pairs_for_testing(num_validators, &mut rng);
        let mut key_archive = HashMap::new();
        for i in 0..qc_keys.len() {
            key_archive.insert(qc_keys[i], state_keys[i].0.clone());
        }
        let st = stake_table_for_testing(&qc_keys, &state_keys);

        // arbitrary commitment values as they don't affect logic being tested
        let block_comm_root = F::from(1234);
        let genesis = LightClientState {
            view_number: 0,
            block_height: 0,
            block_comm_root,
        };

        Self {
            pp,
            rng,
            epoch: 0,
            state: genesis,
            st,
            qc_keys,
            state_keys,
            key_archive,
        }
    }

    /// attempt to advance epoch, should be invoked at the *beginning* of every `fn elapse_xx()`
    fn try_advance_epoch(&mut self) {
        // if the new block is the first block of an epoch, update epoch
        if is_last_block_in_epoch(self.state.block_height as u64, self.pp.epoch_height as u64) {
            self.epoch += 1;
            self.st.advance();
        }
    }

    /// Elapse a view with a new finalized block
    pub fn elapse_with_block(&mut self) {
        self.try_advance_epoch();

        let new_root = self.new_dummy_comm();
        // let new_fee_ledger_comm = self.new_dummy_comm();

        self.state.view_number += 1;
        self.state.block_height += 1;
        self.state.block_comm_root = new_root;
    }

    /// Elapse a view without a new finalized block
    /// (e.g. insufficient votes, malicious leaders or inconsecutive noterized views)
    pub fn elapse_without_block(&mut self) {
        self.try_advance_epoch();
        self.state.view_number += 1;
    }

    /// Update stake table with `num_reg` number of new registrations and `num_exit` number of exits on L1
    pub fn sync_stake_table(&mut self, num_reg: usize, num_exit: usize) {
        // ensure input parameter won't exceed stake table capacity
        let before_st_size = self.qc_keys.len();
        assert!(self.qc_keys.len() + num_reg - num_exit <= self.pp.st_cap);

        // process exits/deregister
        for _ in 0..num_exit {
            let exit_idx = self.rng.next_u32() as usize % self.qc_keys.len();
            let exit_qc_key = self.qc_keys[exit_idx];

            self.st
                .deregister(&exit_qc_key)
                .unwrap_or_else(|_| panic!("failed to deregister {}-th key", exit_idx));
            self.qc_keys.remove(exit_idx);
            self.state_keys.remove(exit_idx);
        }

        // process register
        for i in 0..num_reg {
            let bls_key: BLSVerKey = BLSOverBN254CurveSignatureScheme::key_gen(&(), &mut self.rng)
                .unwrap()
                .1;
            let schnorr_key: (SchnorrSignKey, SchnorrVerKey) =
                SchnorrSignatureScheme::key_gen(&(), &mut self.rng).unwrap();
            let amount = U256::from(self.rng.gen_range(1..1000u32));

            self.st
                .register(bls_key, amount, schnorr_key.1.clone())
                .unwrap_or_else(|_| panic!("failed to deregister {i}-th key"));
            self.key_archive.insert(bls_key, schnorr_key.0.clone());
            self.qc_keys.push(bls_key);
            self.state_keys.push(schnorr_key);
        }

        assert!(self.qc_keys.len() == self.state_keys.len());
        assert!(self.qc_keys.len() == before_st_size + num_reg - num_exit);
    }

    /// Return the light client state and proof of consensus on this finalized state
    pub fn gen_state_proof(&mut self) -> (GenericPublicInput<F>, Proof) {
        let voting_st_state = self.voting_stake_table_state();
        let next_st_state = self.next_stake_table_state();

        let mut msg = Vec::with_capacity(7);
        let state_msg: [F; 3] = self.state.clone().into();
        msg.extend_from_slice(&state_msg);
        let st_state_msg: [F; 4] = next_st_state.into();
        msg.extend_from_slice(&st_state_msg);

        let st: Vec<(BLSVerKey, U256, SchnorrVerKey)> = self
            .st
            .try_iter(SnapshotVersion::LastEpochStart)
            .unwrap()
            .collect();
        let st_size = st.len();

        // find a quorum whose accumulated weights exceed threshold
        let mut bit_vec = vec![false; st_size];
        let mut total_weight = U256::from(0);
        while total_weight < field_to_u256(voting_st_state.threshold) {
            let signer_idx = self.rng.gen_range(0..st_size);
            // if already selected, skip to next random sample
            if bit_vec[signer_idx] {
                continue;
            }

            bit_vec[signer_idx] = true;
            total_weight += st[signer_idx].1;
        }

        let sigs = bit_vec
            .iter()
            .enumerate()
            .map(|(i, b)| {
                if *b {
                    SchnorrSignatureScheme::<EdwardsConfig>::sign(
                        &(),
                        self.key_archive.get(&st[i].0).unwrap(),
                        &msg,
                        &mut self.rng,
                    )
                } else {
                    Ok(Signature::<EdwardsConfig>::default())
                }
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let srs = {
            // load SRS from Aztec's ceremony
            let srs = ark_srs::kzg10::aztec20::setup(2u64.pow(16) as usize + 2)
                .expect("Aztec SRS fail to load");
            // convert to Jellyfish type
            // TODO: (alex) use constructor instead https://github.com/EspressoSystems/jellyfish/issues/440
            UnivariateUniversalParams {
                powers_of_g: srs.powers_of_g,
                h: srs.h,
                beta_h: srs.beta_h,
                powers_of_h: vec![srs.h, srs.beta_h],
            }
        };
        let (pk, _) = preprocess(&srs, STAKE_TABLE_CAPACITY_FOR_TEST)
            .expect("Fail to preprocess state prover circuit");
        let stake_table_entries = self
            .st
            .try_iter(SnapshotVersion::LastEpochStart)
            .unwrap()
            .map(|(_, stake_amount, schnorr_key)| (schnorr_key, stake_amount))
            .collect::<Vec<_>>();
        let (proof, pi) = generate_state_update_proof::<_, _, _, _>(
            &mut self.rng,
            &pk,
            &stake_table_entries,
            &bit_vec,
            &sigs,
            &self.state,
            &voting_st_state,
            STAKE_TABLE_CAPACITY_FOR_TEST,
            &next_st_state,
        )
        .expect("Fail to generate state proof");

        (pi, proof)
    }

    /// a malicious attack, generating a fake stake table full of adversarial stakers
    /// adv-controlled stakers signed the state and replace the stake table commitment with that of the fake one
    /// in an attempt to hijack the correct stake table.
    pub fn gen_state_proof_with_fake_stakers(
        &mut self,
    ) -> (GenericPublicInput<F>, Proof, GenericStakeTableState<F>) {
        let new_state = self.state.clone();

        let (adv_qc_keys, adv_state_keys) =
            key_pairs_for_testing(STAKE_TABLE_CAPACITY_FOR_TEST, &mut self.rng);
        let adv_st = stake_table_for_testing(&adv_qc_keys, &adv_state_keys);
        let adv_st_state = adv_st.voting_state().unwrap();

        // replace new state with adversarial stake table commitment
        let mut msg = Vec::with_capacity(7);
        let state_msg: [F; 3] = new_state.clone().into();
        msg.extend_from_slice(&state_msg);
        let adv_st_state_msg: [F; 4] = adv_st_state.into();
        msg.extend_from_slice(&adv_st_state_msg);

        // every fake stakers sign on the adverarial new state
        let bit_vec = vec![true; STAKE_TABLE_CAPACITY_FOR_TEST];
        let sigs = adv_state_keys
            .iter()
            .map(|(sk, _)| {
                SchnorrSignatureScheme::<EdwardsConfig>::sign(&(), sk, &msg, &mut self.rng)
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let srs = {
            // load SRS from Aztec's ceremony
            let srs = ark_srs::kzg10::aztec20::setup(2u64.pow(16) as usize + 2)
                .expect("Aztec SRS fail to load");
            // convert to Jellyfish type
            // TODO: (alex) use constructor instead https://github.com/EspressoSystems/jellyfish/issues/440
            UnivariateUniversalParams {
                powers_of_g: srs.powers_of_g,
                h: srs.h,
                beta_h: srs.beta_h,
                powers_of_h: vec![srs.h, srs.beta_h],
            }
        };
        let (pk, _) = preprocess(&srs, STAKE_TABLE_CAPACITY_FOR_TEST)
            .expect("Fail to preprocess state prover circuit");
        let stake_table_entries = adv_st
            .try_iter(SnapshotVersion::LastEpochStart)
            .unwrap()
            .map(|(_, stake_amount, schnorr_key)| (schnorr_key, stake_amount))
            .collect::<Vec<_>>();
        let (proof, pi) = generate_state_update_proof::<_, _, _, _>(
            &mut self.rng,
            &pk,
            &stake_table_entries,
            &bit_vec,
            &sigs,
            &new_state,
            &adv_st_state,
            STAKE_TABLE_CAPACITY_FOR_TEST,
            &adv_st_state,
        )
        .expect("Fail to generate state proof");

        (pi, proof, adv_st.voting_state().unwrap())
    }

    /// Returns the stake table state for current voting
    pub fn voting_stake_table_state(&self) -> GenericStakeTableState<F> {
        self.st.voting_state().unwrap()
    }

    /// Returns epoch-aware stake table state for the next block.
    /// This will be the same most of the time as `self.voting_st_state()` except during epoch change
    pub fn next_stake_table_state(&self) -> GenericStakeTableState<F> {
        if is_last_block_in_epoch(self.state.block_height as u64, self.pp.epoch_height as u64) {
            self.st.next_voting_state().unwrap()
        } else {
            self.voting_stake_table_state()
        }
    }

    /// Returns the light client state
    pub fn light_client_state(&self) -> GenericLightClientState<F> {
        self.state.clone()
    }

    // return a dummy commitment value
    fn new_dummy_comm(&mut self) -> F {
        F::rand(&mut self.rng)
    }
}

/// Helper function for test
fn key_pairs_for_testing<R: CryptoRng + RngCore>(
    num_validators: usize,
    prng: &mut R,
) -> (Vec<BLSVerKey>, Vec<(SchnorrSignKey, SchnorrVerKey)>) {
    let bls_keys = (0..num_validators)
        .map(|_| {
            BLSOverBN254CurveSignatureScheme::key_gen(&(), prng)
                .unwrap()
                .1
        })
        .collect::<Vec<_>>();
    let schnorr_keys = (0..num_validators)
        .map(|_| SchnorrSignatureScheme::key_gen(&(), prng).unwrap())
        .collect::<Vec<_>>();
    (bls_keys, schnorr_keys)
}

/// Helper function for test
fn stake_table_for_testing(
    bls_keys: &[BLSVerKey],
    schnorr_keys: &[(SchnorrSignKey, SchnorrVerKey)],
) -> StakeTable<BLSVerKey, SchnorrVerKey, F> {
    let mut st = StakeTable::<BLSVerKey, SchnorrVerKey, F>::new(STAKE_TABLE_CAPACITY_FOR_TEST);
    // Registering keys
    bls_keys
        .iter()
        .enumerate()
        .zip(schnorr_keys)
        .for_each(|((i, bls_key), schnorr_key)| {
            st.register(*bls_key, U256::from((i + 10) as u32), schnorr_key.1.clone())
                .unwrap()
        });
    // Freeze the stake table
    st.advance();
    st.advance();
    st
}

// modify from <https://github.com/EspressoSystems/cape/blob/main/contracts/rust/src/plonk_verifier/helpers.rs>
/// return list of (proof, ver_key, public_input, extra_msg, domain_size)
#[allow(clippy::type_complexity)]
pub fn gen_plonk_proof_for_test(
    num_proof: usize,
) -> Vec<(Proof, VerifyingKey, Vec<F>, Option<Vec<u8>>, usize)> {
    // 1. Simulate universal setup
    let rng = &mut jf_utils::test_rng();
    let srs = {
        let aztec_srs = ark_srs::kzg10::aztec20::setup(1024).expect("Aztec SRS fail to load");

        UnivariateUniversalParams {
            powers_of_g: aztec_srs.powers_of_g,
            h: aztec_srs.h,
            beta_h: aztec_srs.beta_h,
            powers_of_h: vec![aztec_srs.h, aztec_srs.beta_h],
        }
    };
    let open_key = open_key();
    assert_eq!(srs.h, open_key.h);
    assert_eq!(srs.beta_h, open_key.beta_h);
    assert_eq!(srs.powers_of_g[0], open_key.g);

    // 2. Create circuits
    let circuits = (0..num_proof)
        .map(|i| {
            let m = 2 + i / 3;
            let a0 = 1 + i % 3;
            gen_circuit_for_test::<F>(m, a0)
        })
        .collect::<Result<Vec<_>>>()
        .expect("Test circuits fail to create");
    let domain_sizes: Vec<usize> = circuits
        .iter()
        .map(|c| c.eval_domain_size().unwrap())
        .collect();

    // 3. Preprocessing
    let mut prove_keys = vec![];
    let mut ver_keys = vec![];
    for c in circuits.iter() {
        let (pk, vk) =
            PlonkKzgSnark::<Bn254>::preprocess(&srs, c).expect("Circuit preprocessing failed");
        prove_keys.push(pk);
        ver_keys.push(vk);
    }

    // 4. Proving
    let mut proofs = vec![];
    let mut extra_msgs = vec![];

    circuits.iter().zip(prove_keys.iter()).for_each(|(cs, pk)| {
        let extra_msg = Some(vec![]); // We set extra_msg="" for the contract tests to pass
        proofs.push(
            PlonkKzgSnark::<Bn254>::prove::<_, _, SolidityTranscript>(
                rng,
                cs,
                pk,
                extra_msg.clone(),
            )
            .unwrap(),
        );
        extra_msgs.push(extra_msg);
    });

    let public_inputs: Vec<Vec<F>> = circuits
        .iter()
        .map(|cs| cs.public_input().unwrap())
        .collect();

    izip!(proofs, ver_keys, public_inputs, extra_msgs, domain_sizes).collect()
}

// Different `m`s lead to different circuits.
// Different `a0`s lead to different witness values.
pub fn gen_circuit_for_test<F: PrimeField>(m: usize, a0: usize) -> Result<PlonkCircuit<F>> {
    let mut cs: PlonkCircuit<F> = PlonkCircuit::new_turbo_plonk();
    // Create variables
    let mut a = vec![];
    for i in a0..(a0 + 4 * m) {
        a.push(cs.create_variable(F::from(i as u64))?);
    }
    let b = [
        cs.create_public_variable(F::from(m as u64 * 2))?,
        cs.create_public_variable(F::from(a0 as u64 * 2 + m as u64 * 4 - 1))?,
    ];
    let c = cs.create_public_variable(
        (cs.witness(b[1])? + cs.witness(a[0])?) * (cs.witness(b[1])? - cs.witness(a[0])?),
    )?;

    // Create other public variables so that the number of public inputs is 11
    for i in 0..8u32 {
        cs.create_public_variable(F::from(i))?;
    }

    // Create gates:
    // 1. a0 + ... + a_{4*m-1} = b0 * b1
    // 2. (b1 + a0) * (b1 - a0) = c
    // 3. b0 = 2 * m
    let mut acc = cs.zero();
    a.iter().for_each(|&elem| acc = cs.add(acc, elem).unwrap());
    let b_mul = cs.mul(b[0], b[1])?;
    cs.enforce_equal(acc, b_mul)?;
    let b1_plus_a0 = cs.add(b[1], a[0])?;
    let b1_minus_a0 = cs.sub(b[1], a[0])?;
    cs.mul_gate(b1_plus_a0, b1_minus_a0, c)?;
    cs.enforce_constant(b[0], F::from(m as u64 * 2))?;

    // Finalize the circuit.
    cs.finalize_for_arithmetization()?;

    Ok(cs)
}
