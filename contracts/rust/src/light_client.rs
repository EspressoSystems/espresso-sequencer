//! Helpers and test mocks for Light Client logic

use ark_ed_on_bn254::EdwardsConfig;
use ark_std::rand::rngs::StdRng;
use ark_std::rand::{CryptoRng, Rng, RngCore};
use ark_std::str::FromStr;
use ark_std::UniformRand;
use diff_test_bn254::{field_to_u256, u256_to_field};
use ethers::{abi, utils};
use ethers::{
    abi::{AbiDecode, Token},
    prelude::{AbiError, EthAbiCodec, EthAbiType},
    types::{H256, U256},
};
use hotshot_stake_table::vec_based::StakeTable;
use hotshot_state_prover::circuit::PublicInput;
use hotshot_state_prover::Proof;
use hotshot_types::traits::stake_table::StakeTableScheme;
use hotshot_types::{light_client::LightClientState, traits::stake_table::SnapshotVersion};
use jf_primitives::pcs::prelude::UnivariateUniversalParams;
use jf_primitives::signatures::schnorr::Signature;
use jf_primitives::signatures::{
    bls_over_bn254::{BLSOverBN254CurveSignatureScheme, VerKey as BLSVerKey},
    SchnorrSignatureScheme, SignatureScheme,
};
use jf_utils::test_rng;

type F = ark_ed_on_bn254::Fq;
type SchnorrVerKey = jf_primitives::signatures::schnorr::VerKey<EdwardsConfig>;
type SchnorrSignKey = jf_primitives::signatures::schnorr::SignKey<ark_ed_on_bn254::Fr>;

const STAKE_TABLE_CAPACITY: usize = 40;

/// Mock for system parameter of `MockLedger`
pub struct MockSystemParam {
    /// max capcity of stake table
    st_cap: usize,
    /// number of blocks per epoch
    blk_per_epoch: u32,
}

impl MockSystemParam {
    /// Init the system parameters (some fixed, some adjustable)
    pub fn init(blk_per_epoch: u32) -> Self {
        Self {
            st_cap: STAKE_TABLE_CAPACITY,
            blk_per_epoch,
        }
    }
}

/// Mock of hotshot ledger for testing LightClient.sol functionalities only
/// its logic is completely divergent from a real light client or HotShot
pub struct MockLedger {
    pp: MockSystemParam,
    rng: StdRng,
    epoch: u64,
    state: LightClientState<F>,
    st: StakeTable<BLSVerKey, SchnorrVerKey, F>,
    threshold: U256, // quorum threshold for SnapShot::LastEpochStart
    qc_keys: Vec<BLSVerKey>,
    state_keys: Vec<(SchnorrSignKey, SchnorrVerKey)>,
}

impl MockLedger {
    /// Initialzie the ledger with genesis state
    pub fn init(pp: MockSystemParam, num_validators: usize) -> Self {
        // credit: https://github.com/EspressoSystems/HotShot/blob/5554b7013b00e6034691b533299b44f3295fa10d/crates/hotshot-state-prover/src/lib.rs#L176
        let mut rng = test_rng();
        let (qc_keys, state_keys) = key_pairs_for_testing(num_validators, &mut rng);
        let st = stake_table_for_testing(pp.st_cap, &qc_keys, &state_keys);
        let threshold = st.total_stake(SnapshotVersion::LastEpochStart).unwrap() * 2 / 3;

        // arbitrary commitment values as they don't affect logic being tested
        let block_comm_root = F::from(1234);
        let fee_ledger_comm = F::from(5678);

        let genesis = LightClientState {
            view_number: 0,
            block_height: 0,
            block_comm_root,
            fee_ledger_comm,
            stake_table_comm: st.commitment(SnapshotVersion::LastEpochStart).unwrap(),
        };
        Self {
            pp,
            rng,
            epoch: 0,
            state: genesis,
            st,
            threshold,
            qc_keys,
            state_keys,
        }
    }

    /// Elapse a view with a new finalized block
    pub fn elapse_with_block(&mut self) {
        // if the new block is the first block of an epoch, update epoch
        if (self.state.block_height + 1) % (self.pp.blk_per_epoch as usize) == 0 {
            self.epoch += 1;
            self.st.advance();
            self.threshold = self
                .st
                .total_stake(SnapshotVersion::LastEpochStart)
                .unwrap()
                * 2
                / 3;
        }

        let new_root = self.new_dummy_comm();
        let new_fee_ledger_comm = self.new_dummy_comm();

        self.state.view_number += 1;
        self.state.block_height += 1;
        self.state.block_comm_root = new_root;
        self.state.fee_ledger_comm = new_fee_ledger_comm;
    }

    /// Elapse a view without a new finalized block
    /// (e.g. insufficient votes, malicious leaders or inconsecutive noterized views)
    pub fn elapse_without_block(&mut self) {
        self.state.view_number += 1;
    }

    /// Update stake table with `num_reg` number of new registrations and `num_exit` number of exits on L1
    pub fn sync_stake_table(&mut self, num_reg: usize, num_exit: usize) {
        // ensure input parameter won't exceed stake table capacity
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
            self.qc_keys.push(bls_key);
            self.state_keys.push(schnorr_key);
        }
    }

    /// Elapse an epoch with `num_reg` of new registration, `num_exit` of key deregistration
    /// Return the new light client state and a proof.
    pub fn elapse_epoch(&mut self, num_reg: usize, num_exit: usize) {
        assert!(self.qc_keys.len() + num_reg - num_exit <= self.pp.st_cap);

        // random number of notorized but not finalized block
        let num_non_blk = self.rng.gen_range(0..10);
        for _ in 0..num_non_blk {
            self.elapse_without_block();
        }

        for _ in 0..self.pp.blk_per_epoch {
            self.elapse_with_block();
        }

        self.sync_stake_table(num_reg, num_exit);
    }

    /// Return the light client state and proof of consensus on this finalized state
    pub fn gen_state_proof(&mut self) -> (PublicInput<F>, Proof) {
        let state_msg: [F; 7] = self.state.clone().into();

        let st_size = self.qc_keys.len();
        // find a quorum whose accumulated weights exceed threshold
        let mut bit_vec = vec![false; st_size];
        let mut total_weight = U256::from(0);
        while total_weight < self.threshold {
            let signer_idx = self.rng.gen_range(0..st_size);
            // if already selected, skip to next random sample
            if bit_vec[signer_idx] {
                continue;
            }

            bit_vec[signer_idx] = true;
            total_weight += self
                .st
                .lookup(SnapshotVersion::LastEpochStart, &self.qc_keys[signer_idx])
                .unwrap();
        }

        let sigs = bit_vec
            .iter()
            .enumerate()
            .map(|(i, b)| {
                if *b {
                    SchnorrSignatureScheme::<EdwardsConfig>::sign(
                        &(),
                        &self.state_keys[i].0,
                        state_msg,
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
            let srs = crs::aztec20::kzg10_setup(2u64.pow(20) as usize + 2)
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
        let (pk, _) = hotshot_state_prover::preprocess::<STAKE_TABLE_CAPACITY>(&srs)
            .expect("Fail to preprocess state prover circuit");
        let (proof, pi) =
            hotshot_state_prover::generate_state_update_proof::<_, _, _, _, STAKE_TABLE_CAPACITY>(
                &mut self.rng,
                &pk,
                &self.st,
                &bit_vec,
                &sigs,
                &self.state,
                &self.threshold,
            )
            .expect("Fail to generate state proof");
        (pi, proof)
    }

    /// Returns the `LightClientState` for solidity
    pub fn get_state(&self) -> ParsedLightClientState {
        // The ugly conversion due to slight difference of `LightClientState` in solidity containing `threshold`
        let pi = vec![
            u256_to_field(self.threshold),
            F::from(self.state.view_number as u64),
            F::from(self.state.block_height as u64),
            self.state.block_comm_root,
            self.state.fee_ledger_comm,
            self.state.stake_table_comm.0,
            self.state.stake_table_comm.1,
            self.state.stake_table_comm.2,
        ];
        let pi: PublicInput<F> = pi.into();
        pi.into()
    }

    /// Returns the (bytes32 votingStakeTableComm, bytes32 frozenStakeTableComm) used in contract
    pub fn get_stake_table_comms(&self) -> (H256, H256) {
        let (bls_key_comm, schnorr_key_comm, amount_comm) =
            self.st.commitment(SnapshotVersion::EpochStart).unwrap();
        let frozen_st_comm = utils::keccak256(
            abi::encode_packed(&[
                Token::Uint(field_to_u256(bls_key_comm)),
                Token::Uint(field_to_u256(schnorr_key_comm)),
                Token::Uint(field_to_u256(amount_comm)),
            ])
            .unwrap(),
        );

        let (bls_key_comm, schnorr_key_comm, amount_comm) =
            self.st.commitment(SnapshotVersion::LastEpochStart).unwrap();
        let voting_st_comm = utils::keccak256(
            abi::encode_packed(&[
                Token::Uint(field_to_u256(bls_key_comm)),
                Token::Uint(field_to_u256(schnorr_key_comm)),
                Token::Uint(field_to_u256(amount_comm)),
            ])
            .unwrap(),
        );

        (voting_st_comm.into(), frozen_st_comm.into())
    }

    // return a dummy commitment value
    fn new_dummy_comm(&mut self) -> F {
        F::rand(&mut self.rng)
    }
}

/// Helper function for test
pub(crate) fn key_pairs_for_testing<R: CryptoRng + RngCore>(
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
pub(crate) fn stake_table_for_testing(
    capacity: usize,
    bls_keys: &[BLSVerKey],
    schnorr_keys: &[(SchnorrSignKey, SchnorrVerKey)],
) -> StakeTable<BLSVerKey, SchnorrVerKey, F> {
    let mut st = StakeTable::<BLSVerKey, SchnorrVerKey, F>::new(capacity);
    // Registering keys
    bls_keys
        .iter()
        .enumerate()
        .zip(schnorr_keys)
        .for_each(|((i, bls_key), schnorr_key)| {
            st.register(*bls_key, U256::from((i + 1) as u32), schnorr_key.1.clone())
                .unwrap()
        });
    // Freeze the stake table
    st.advance();
    st.advance();
    st
}

/// Intermediate representations for `LightClientState` in Solidity
#[derive(Clone, Debug, EthAbiType, EthAbiCodec)]
pub struct ParsedLightClientState {
    view_num: u64,
    block_height: u64,
    block_comm_root: U256,
    fee_ledger_comm: U256,
    bls_key_comm: U256,
    schnorr_key_comm: U256,
    amount_comm: U256,
    threshold: U256,
}

impl FromStr for ParsedLightClientState {
    type Err = AbiError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: (Self,) = AbiDecode::decode_hex(s)?;
        Ok(parsed.0)
    }
}

impl From<PublicInput<F>> for ParsedLightClientState {
    fn from(pi: PublicInput<F>) -> Self {
        Self {
            view_num: field_to_u256(pi.view_number()).as_u64(),
            block_height: field_to_u256(pi.block_height()).as_u64(),
            block_comm_root: field_to_u256(pi.block_comm_root()),
            fee_ledger_comm: field_to_u256(pi.fee_ledger_comm()),
            bls_key_comm: field_to_u256(pi.qc_key_comm()),
            schnorr_key_comm: field_to_u256(pi.state_key_comm()),
            amount_comm: field_to_u256(pi.stake_amount_comm()),
            threshold: field_to_u256(pi.threshold()),
        }
    }
}

impl From<ParsedLightClientState> for PublicInput<F> {
    fn from(s: ParsedLightClientState) -> Self {
        let fields = vec![
            u256_to_field(s.threshold),
            F::from(s.view_num),
            F::from(s.block_height),
            u256_to_field(s.block_comm_root),
            u256_to_field(s.fee_ledger_comm),
            u256_to_field(s.bls_key_comm),
            u256_to_field(s.schnorr_key_comm),
            u256_to_field(s.amount_comm),
        ];
        Self::from(fields)
    }
}
