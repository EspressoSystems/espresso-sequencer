//! Helpers for connecting types between Jellyfish and Solidity.
//! Usually used during differential testing (via FFI).

use anyhow::Result;
use ark_bn254::{Bn254, Fq, Fr, G1Affine, G2Affine};
use ark_ff::{Fp2, MontFp, PrimeField};
use ark_std::{rand::Rng, str::FromStr, UniformRand};
pub use diff_test_bn254::{field_to_u256, u256_to_field, ParsedG1Point};
use ethers::{
    abi::AbiDecode,
    prelude::{AbiError, EthAbiCodec, EthAbiType},
    types::{Bytes, H256, U256},
};
use itertools::izip;
use jf_plonk::proof_system::structs::{OpenKey, Proof, ProofEvaluations, VerifyingKey};
use jf_plonk::proof_system::{PlonkKzgSnark, UniversalSNARK};
use jf_plonk::testing_apis::Challenges;
use jf_plonk::{constants::KECCAK256_STATE_SIZE, transcript::SolidityTranscript};
use jf_primitives::pcs::prelude::{Commitment, UnivariateUniversalParams};
use jf_relation::{Arithmetization, Circuit, PlonkCircuit};
use num_bigint::BigUint;
use num_traits::Num;

// constant in hex string copied from hardcoded constants from solidity contracts

const COSET: [&str; 5] = [
    "1",
    "2f8dd1f1a7583c42c4e12a44e110404c73ca6c94813f85835da4fb7bb1301d4a",
    "1ee678a0470a75a6eaa8fe837060498ba828a3703b311d0f77f010424afeb025",
    "2042a587a90c187b0a087c03e29c968b950b1db26d5c82d666905a6895790c0a",
    "2e2b91456103698adf57b799969dea1c8f739da5d8d40dd3eb9222db7c81e881",
];

// H: G2Affine(x: Fp2, y:Fp2), x = x0 + u * x1, y = y0 + u * y1
// NOTE: extra careful with discrepancy with current version of BN254.G2Point
// which assume Fp2 = x0 * u + x1 !
const H: [&str; 4] = [
    "1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed", // x0
    "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2", // x1
    "12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa", // y0
    "090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b", // y1
];

// See notes about `const H` above.
const BETA_H: [&str; 4] = [
    "0118c4d5b837bcc2bc89b5b398b5974e9f5944073b32078b7e231fec938883b0",
    "260e01b251f6f1c7e7ff4e580791dee8ea51d87a358e038b4efe30fac09383c1",
    "22febda3c0c0632a56475b4214e5615e11e6dd3f96e6cea2854a87d4dacc5e55",
    "04fc6369f7110fe3d25156c1bb9a72859cf2a04641f99ba4ee413c80da6a5fe4",
];

// TODO: (alex) change to simply using `MontFp!("0x..")` after
// <https://github.com/arkworks-rs/algebra/pull/635> is on a tag release
// Return cosets coefficients for circuits over BN254.
pub fn coset_k() -> Vec<Fr> {
    vec![
        Fr::from(BigUint::from_str_radix(COSET[0], 16).unwrap()),
        Fr::from(BigUint::from_str_radix(COSET[1], 16).unwrap()),
        Fr::from(BigUint::from_str_radix(COSET[2], 16).unwrap()),
        Fr::from(BigUint::from_str_radix(COSET[3], 16).unwrap()),
        Fr::from(BigUint::from_str_radix(COSET[4], 16).unwrap()),
    ]
}

/// Returns `OpenKeys` for KZG10 over BN254 curve from Aztec's SRS
pub fn open_key() -> OpenKey<Bn254> {
    let g = G1Affine::new_unchecked(MontFp!("1"), MontFp!("2"));
    let h = G2Affine::new(
        Fp2::new(
            Fq::from(BigUint::from_str_radix(H[0], 16).unwrap()),
            Fq::from(BigUint::from_str_radix(H[1], 16).unwrap()),
        ),
        Fp2::new(
            Fq::from(BigUint::from_str_radix(H[2], 16).unwrap()),
            Fq::from(BigUint::from_str_radix(H[3], 16).unwrap()),
        ),
    );
    let beta_h = G2Affine::new(
        Fp2::new(
            Fq::from(BigUint::from_str_radix(BETA_H[0], 16).unwrap()),
            Fq::from(BigUint::from_str_radix(BETA_H[1], 16).unwrap()),
        ),
        Fp2::new(
            Fq::from(BigUint::from_str_radix(BETA_H[2], 16).unwrap()),
            Fq::from(BigUint::from_str_radix(BETA_H[3], 16).unwrap()),
        ),
    );

    OpenKey { g, h, beta_h }
}

/// an intermediate representation of the transcript parsed from abi.encode(transcript) from Solidity.
#[derive(Clone, EthAbiType, EthAbiCodec)]
pub struct ParsedTranscript {
    pub(crate) transcript: Bytes,
    pub(crate) state: [H256; 2],
}

impl FromStr for ParsedTranscript {
    type Err = AbiError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: (ParsedTranscript,) = AbiDecode::decode_hex(s)?;
        Ok(parsed.0)
    }
}

impl From<SolidityTranscript> for ParsedTranscript {
    fn from(t: SolidityTranscript) -> Self {
        let (transcript, state) = t.internal();
        Self {
            transcript: transcript.into(),
            state: [
                H256::from_slice(&state[..32]),
                H256::from_slice(&state[32..]),
            ],
        }
    }
}

impl From<ParsedTranscript> for SolidityTranscript {
    fn from(t: ParsedTranscript) -> Self {
        let mut state = [0u8; KECCAK256_STATE_SIZE];
        state[..32].copy_from_slice(&t.state[0].to_fixed_bytes());
        state[32..].copy_from_slice(&t.state[1].to_fixed_bytes());
        Self::from_internal(t.transcript.to_vec(), state)
    }
}

/// intermediate representation of `VerifyingKey` in solidity.
#[derive(Clone, Debug, EthAbiType, EthAbiCodec)]
pub struct ParsedVerifyingKey {
    pub domain_size: U256,
    pub num_inputs: U256,
    pub sigma_0: ParsedG1Point,
    pub sigma_1: ParsedG1Point,
    pub sigma_2: ParsedG1Point,
    pub sigma_3: ParsedG1Point,
    pub sigma_4: ParsedG1Point,
    pub q_1: ParsedG1Point,
    pub q_2: ParsedG1Point,
    pub q_3: ParsedG1Point,
    pub q_4: ParsedG1Point,
    pub q_m_12: ParsedG1Point,
    pub q_m_34: ParsedG1Point,
    pub q_o: ParsedG1Point,
    pub q_c: ParsedG1Point,
    pub q_h_1: ParsedG1Point,
    pub q_h_2: ParsedG1Point,
    pub q_h_3: ParsedG1Point,
    pub q_h_4: ParsedG1Point,
    pub q_ecc: ParsedG1Point,
}

impl FromStr for ParsedVerifyingKey {
    type Err = AbiError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: (Self,) = AbiDecode::decode_hex(s)?;
        Ok(parsed.0)
    }
}

impl From<VerifyingKey<Bn254>> for ParsedVerifyingKey {
    fn from(vk: VerifyingKey<Bn254>) -> Self {
        Self {
            domain_size: U256::from(vk.domain_size),
            num_inputs: U256::from(vk.num_inputs),
            sigma_0: vk.sigma_comms[0].0.into(),
            sigma_1: vk.sigma_comms[1].0.into(),
            sigma_2: vk.sigma_comms[2].0.into(),
            sigma_3: vk.sigma_comms[3].0.into(),
            sigma_4: vk.sigma_comms[4].0.into(),
            q_1: vk.selector_comms[0].0.into(),
            q_2: vk.selector_comms[1].0.into(),
            q_3: vk.selector_comms[2].0.into(),
            q_4: vk.selector_comms[3].0.into(),
            q_m_12: vk.selector_comms[4].0.into(),
            q_m_34: vk.selector_comms[5].0.into(),
            q_h_1: vk.selector_comms[6].0.into(),
            q_h_2: vk.selector_comms[7].0.into(),
            q_h_3: vk.selector_comms[8].0.into(),
            q_h_4: vk.selector_comms[9].0.into(),
            q_o: vk.selector_comms[10].0.into(),
            q_c: vk.selector_comms[11].0.into(),
            q_ecc: vk.selector_comms[12].0.into(),
        }
    }
}

impl From<ParsedVerifyingKey> for VerifyingKey<Bn254> {
    fn from(vk: ParsedVerifyingKey) -> Self {
        let sigma_comms = vec![
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.sigma_0)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.sigma_1)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.sigma_2)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.sigma_3)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.sigma_4)),
        ];

        let selector_comms = vec![
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.q_1)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.q_2)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.q_3)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.q_4)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.q_m_12)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.q_m_34)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.q_h_1)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.q_h_2)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.q_h_3)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.q_h_4)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.q_o)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.q_c)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(vk.q_ecc)),
        ];

        Self {
            domain_size: vk.domain_size.as_usize(),
            num_inputs: vk.num_inputs.as_usize(),
            sigma_comms,
            selector_comms,
            k: coset_k(),
            open_key: open_key(),
            is_merged: false,
            plookup_vk: None,
        }
    }
}

/// intermediate representation of `PlonkProof` in solidity
#[derive(Clone, Debug, Default, EthAbiType, EthAbiCodec)]
pub struct ParsedPlonkProof {
    // commitments
    wire_0: ParsedG1Point,
    wire_1: ParsedG1Point,
    wire_2: ParsedG1Point,
    wire_3: ParsedG1Point,
    wire_4: ParsedG1Point,
    prod_perm: ParsedG1Point,
    split_0: ParsedG1Point,
    split_1: ParsedG1Point,
    split_2: ParsedG1Point,
    split_3: ParsedG1Point,
    split_4: ParsedG1Point,
    zeta: ParsedG1Point,
    zeta_omega: ParsedG1Point,
    // proof evals
    wire_eval_0: U256,
    wire_eval_1: U256,
    wire_eval_2: U256,
    wire_eval_3: U256,
    wire_eval_4: U256,
    sigma_eval_0: U256,
    sigma_eval_1: U256,
    sigma_eval_2: U256,
    sigma_eval_3: U256,
    prod_perm_zeta_omega_eval: U256,
}

impl FromStr for ParsedPlonkProof {
    type Err = AbiError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: (Self,) = AbiDecode::decode_hex(s)?;
        Ok(parsed.0)
    }
}

impl From<Proof<Bn254>> for ParsedPlonkProof {
    fn from(proof: Proof<Bn254>) -> Self {
        Self {
            wire_0: proof.wires_poly_comms[0].0.into(),
            wire_1: proof.wires_poly_comms[1].0.into(),
            wire_2: proof.wires_poly_comms[2].0.into(),
            wire_3: proof.wires_poly_comms[3].0.into(),
            wire_4: proof.wires_poly_comms[4].0.into(),
            prod_perm: proof.prod_perm_poly_comm.0.into(),
            split_0: proof.split_quot_poly_comms[0].0.into(),
            split_1: proof.split_quot_poly_comms[1].0.into(),
            split_2: proof.split_quot_poly_comms[2].0.into(),
            split_3: proof.split_quot_poly_comms[3].0.into(),
            split_4: proof.split_quot_poly_comms[4].0.into(),
            zeta: proof.opening_proof.0.into(),
            zeta_omega: proof.shifted_opening_proof.0.into(),
            wire_eval_0: field_to_u256(proof.poly_evals.wires_evals[0]),
            wire_eval_1: field_to_u256(proof.poly_evals.wires_evals[1]),
            wire_eval_2: field_to_u256(proof.poly_evals.wires_evals[2]),
            wire_eval_3: field_to_u256(proof.poly_evals.wires_evals[3]),
            wire_eval_4: field_to_u256(proof.poly_evals.wires_evals[4]),
            sigma_eval_0: field_to_u256(proof.poly_evals.wire_sigma_evals[0]),
            sigma_eval_1: field_to_u256(proof.poly_evals.wire_sigma_evals[1]),
            sigma_eval_2: field_to_u256(proof.poly_evals.wire_sigma_evals[2]),
            sigma_eval_3: field_to_u256(proof.poly_evals.wire_sigma_evals[3]),
            prod_perm_zeta_omega_eval: field_to_u256(proof.poly_evals.perm_next_eval),
        }
    }
}

impl From<ParsedPlonkProof> for Proof<Bn254> {
    fn from(proof: ParsedPlonkProof) -> Self {
        let wires_poly_comms = vec![
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(proof.wire_0)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(proof.wire_1)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(proof.wire_2)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(proof.wire_3)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(proof.wire_4)),
        ];
        let split_quot_poly_comms = vec![
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(proof.split_0)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(proof.split_1)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(proof.split_2)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(proof.split_3)),
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(proof.split_4)),
        ];
        let prod_perm_poly_comm =
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(proof.prod_perm));
        let opening_proof = Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(proof.zeta));
        let shifted_opening_proof =
            Commitment::from(<ParsedG1Point as Into<G1Affine>>::into(proof.zeta_omega));

        let wires_evals = vec![
            u256_to_field(proof.wire_eval_0),
            u256_to_field(proof.wire_eval_1),
            u256_to_field(proof.wire_eval_2),
            u256_to_field(proof.wire_eval_3),
            u256_to_field(proof.wire_eval_4),
        ];
        let wire_sigma_evals = vec![
            u256_to_field(proof.sigma_eval_0),
            u256_to_field(proof.sigma_eval_1),
            u256_to_field(proof.sigma_eval_2),
            u256_to_field(proof.sigma_eval_3),
        ];
        let perm_next_eval = u256_to_field(proof.prod_perm_zeta_omega_eval);

        Self {
            wires_poly_comms,
            prod_perm_poly_comm,
            split_quot_poly_comms,
            opening_proof,
            shifted_opening_proof,
            poly_evals: ProofEvaluations {
                wires_evals,
                wire_sigma_evals,
                perm_next_eval,
            },
            plookup_proof: None,
        }
    }
}

impl ParsedPlonkProof {
    /// return a dummy proof instance with random ProofEvaluations fields.
    pub fn dummy_with_rand_proof_evals<R: Rng>(rng: &mut R) -> Self {
        Self {
            wire_eval_0: field_to_u256(Fr::rand(rng)),
            wire_eval_1: field_to_u256(Fr::rand(rng)),
            wire_eval_2: field_to_u256(Fr::rand(rng)),
            wire_eval_3: field_to_u256(Fr::rand(rng)),
            wire_eval_4: field_to_u256(Fr::rand(rng)),
            sigma_eval_0: field_to_u256(Fr::rand(rng)),
            sigma_eval_1: field_to_u256(Fr::rand(rng)),
            sigma_eval_2: field_to_u256(Fr::rand(rng)),
            sigma_eval_3: field_to_u256(Fr::rand(rng)),
            prod_perm_zeta_omega_eval: field_to_u256(Fr::rand(rng)),
            ..Default::default()
        }
    }

    /// return a dummy proof instance with all random fields
    pub fn dummy<R: Rng>(rng: &mut R) -> Self {
        let mut proof = Self::dummy_with_rand_proof_evals(rng);
        proof.wire_0 = G1Affine::rand(rng).into();
        proof.wire_1 = G1Affine::rand(rng).into();
        proof.wire_2 = G1Affine::rand(rng).into();
        proof.wire_3 = G1Affine::rand(rng).into();
        proof.wire_4 = G1Affine::rand(rng).into();
        proof.prod_perm = G1Affine::rand(rng).into();
        proof.split_0 = G1Affine::rand(rng).into();
        proof.split_1 = G1Affine::rand(rng).into();
        proof.split_2 = G1Affine::rand(rng).into();
        proof.split_3 = G1Affine::rand(rng).into();
        proof.split_4 = G1Affine::rand(rng).into();
        proof.zeta = G1Affine::rand(rng).into();
        proof.zeta_omega = G1Affine::rand(rng).into();
        proof
    }
}

/// intermediate representation of `Challenges` in solidity
#[derive(Clone, Debug, Default, EthAbiType, EthAbiCodec)]
pub struct ParsedChallenges {
    alpha: U256,
    alpha_2: U256,
    alpha_3: U256,
    beta: U256,
    gamma: U256,
    zeta: U256,
    v: U256,
    u: U256,
}

impl ParsedChallenges {
    /// dummy challenges
    #[allow(dead_code)]
    pub fn dummy<R: Rng>(rng: &mut R) -> Self {
        let alpha = Fr::rand(rng);
        let alpha_2 = alpha * alpha;
        let alpha_3 = alpha * alpha_2;
        Self {
            alpha: field_to_u256(alpha),
            alpha_2: field_to_u256(alpha_2),
            alpha_3: field_to_u256(alpha_3),
            beta: field_to_u256(Fr::rand(rng)),
            gamma: field_to_u256(Fr::rand(rng)),
            zeta: field_to_u256(Fr::rand(rng)),
            v: field_to_u256(Fr::rand(rng)),
            u: field_to_u256(Fr::rand(rng)),
        }
    }
}

impl FromStr for ParsedChallenges {
    type Err = AbiError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: (Self,) = AbiDecode::decode_hex(s)?;
        Ok(parsed.0)
    }
}

impl From<Challenges<Fr>> for ParsedChallenges {
    fn from(c: Challenges<Fr>) -> Self {
        let alpha_2 = c.alpha * c.alpha;
        Self {
            alpha: field_to_u256::<Fr>(c.alpha),
            alpha_2: field_to_u256::<Fr>(alpha_2),
            alpha_3: field_to_u256::<Fr>(c.alpha * alpha_2),
            beta: field_to_u256::<Fr>(c.beta),
            gamma: field_to_u256::<Fr>(c.gamma),
            zeta: field_to_u256::<Fr>(c.zeta),
            v: field_to_u256::<Fr>(c.v),
            u: field_to_u256::<Fr>(c.u),
        }
    }
}

impl From<ParsedChallenges> for Challenges<Fr> {
    fn from(c: ParsedChallenges) -> Self {
        Self {
            tau: Fr::from(0u32),
            alpha: u256_to_field(c.alpha),
            beta: u256_to_field(c.beta),
            gamma: u256_to_field(c.gamma),
            zeta: u256_to_field(c.zeta),
            v: u256_to_field(c.v),
            u: u256_to_field(c.u),
        }
    }
}

// modify from <https://github.com/EspressoSystems/cape/blob/main/contracts/rust/src/plonk_verifier/helpers.rs>
/// return list of (proof, ver_key, public_input, extra_msg, domain_size)
#[allow(clippy::type_complexity)]
pub fn gen_plonk_proof_for_test(
    num_proof: usize,
) -> Vec<(
    Proof<Bn254>,
    VerifyingKey<Bn254>,
    Vec<Fr>,
    Option<Vec<u8>>,
    usize,
)> {
    // 1. Simulate universal setup
    let rng = &mut jf_utils::test_rng();
    let srs = {
        let aztec_srs = crs::aztec20::kzg10_setup(1024).expect("Aztec SRS fail to load");

        UnivariateUniversalParams {
            powers_of_g: aztec_srs.powers_of_g,
            h: aztec_srs.h,
            beta_h: aztec_srs.beta_h,
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
            gen_circuit_for_test::<Fr>(m, a0)
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

    circuits
        .iter()
        .zip(prove_keys.iter())
        .enumerate()
        .for_each(|(i, (cs, pk))| {
            let extra_msg = Some(format!("extra message: {}", i).into_bytes());
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

    let public_inputs: Vec<Vec<Fr>> = circuits
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
