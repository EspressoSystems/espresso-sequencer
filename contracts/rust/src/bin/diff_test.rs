use std::str::FromStr;

use ark_bn254::{Bn254, Fq, Fr, G1Affine, G2Affine};
use ark_ec::short_weierstrass::{Affine, SWCurveConfig};
use ark_ec::AffineRepr;
use ark_ff::{BigInteger, Fp2, MontFp, PrimeField};
use ark_poly::domain::radix2::Radix2EvaluationDomain;
use ark_poly::EvaluationDomain;
use clap::{Parser, ValueEnum};
use ethers::{
    abi::{AbiDecode, AbiEncode},
    prelude::{AbiError, EthAbiCodec, EthAbiType},
    types::{Bytes, H256, U256},
};
use jf_plonk::proof_system::structs::{OpenKey, VerifyingKey};
use jf_plonk::{
    constants::KECCAK256_STATE_SIZE,
    testing_apis::Verifier,
    transcript::{PlonkTranscript, SolidityTranscript},
};
use jf_primitives::pcs::prelude::Commitment;
use num_bigint::BigUint;
use num_traits::Num;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// Identifier for the functions to invoke in Jellyfish
    #[arg(value_enum)]
    action: Action,
    /// Optional 1st argument for the `action`
    arg1: Option<String>,
    /// Optional 2nd argument for the `action`
    arg2: Option<String>,
    /// Optional 3rd argument for the `action`
    arg3: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Action {
    /// Get ark_poly::Radix2EvaluationDomain::new()
    NewPolyEvalDomain,
    /// Get ark_poly::Radix2EvaluationDomain::elements()
    EvalDomainElements,
    /// Get some poly evals during jf_plonk::prepare_pcs_info()
    EvalDataGen,
    /// Get jf_plonk::Transcript::append_message()
    TranscriptAppendMsg,
    /// Get jf_plonk::Transcript::append_challenge()
    TranscriptAppendField,
    /// Get jf_plonk::Transcript::append_commitment()
    TranscriptAppendGroup,
    /// Get jf_plonk::Transcript::get_and_append_challenge()
    TranscriptGetChal,
    /// Get jf_plonk::Transcript::append_vk_and_pub_input()
    TranscriptAppendVkAndPi,
    /// Return the Plonk Verifier related constants
    PlonkConstants,
    /// Test only logic
    TestOnly,
}

fn main() {
    let cli = Cli::parse();

    match cli.action {
        Action::NewPolyEvalDomain => {
            let arg1 = cli.arg1.as_ref().expect("Should provide arg1=logSize");
            let log_size = arg1.parse::<u32>().unwrap();

            let domain = Radix2EvaluationDomain::<Fr>::new(2u32.pow(log_size) as usize).unwrap();
            let res = (
                field_to_u256(domain.size_inv),
                field_to_u256(domain.group_gen),
                field_to_u256(domain.group_gen_inv),
            );
            println!("{}", res.encode_hex());
        }
        Action::EvalDomainElements => {
            let arg1 = cli.arg1.as_ref().expect("Should provide arg1=logSize");
            let arg2 = cli.arg2.as_ref().expect("Should provide arg2=length");
            let log_size = arg1.parse::<u32>().unwrap();
            let length = arg2.parse::<usize>().unwrap();

            let domain = Radix2EvaluationDomain::<Fr>::new(2u32.pow(log_size) as usize).unwrap();
            let res = domain
                .elements()
                .take(length)
                .map(field_to_u256)
                .collect::<Vec<_>>();
            println!("{}", res.encode_hex());
        }
        Action::EvalDataGen => {
            let arg1 = cli.arg1.as_ref().expect("Should provide arg1=logSize");
            let arg2 = cli.arg2.as_ref().expect("Should provide arg2=zeta");
            let arg3 = cli.arg3.as_ref().expect("Should provide arg3=publicInput");

            let log_size = arg1.parse::<u32>().unwrap();
            let zeta = u256_to_field::<Fr>(arg2.parse::<U256>().unwrap());
            let pi_u256: Vec<U256> = AbiDecode::decode_hex(arg3).unwrap();
            let pi: Vec<Fr> = pi_u256.into_iter().map(u256_to_field).collect();

            let verifier = Verifier::<Bn254>::new(2u32.pow(log_size) as usize).unwrap();
            let (vanish_eval, lagrange_one, pi_eval) = verifier
                .compute_poly_evals_for_pcs_info(&zeta, &pi)
                .unwrap();
            let res = (
                field_to_u256(vanish_eval),
                field_to_u256(lagrange_one),
                field_to_u256(pi_eval),
            );
            println!("{}", res.encode_hex());
        }
        Action::TranscriptAppendMsg => {
            let arg1 = cli.arg1.as_ref().expect("Should provide arg1=transcript");
            let arg2 = cli.arg2.as_ref().expect("Should provide arg2=message");
            let t_parsed = arg1.parse::<ParsedTranscript>().unwrap();
            let msg = {
                let parsed: Bytes = AbiDecode::decode_hex(arg2).unwrap();
                parsed.0.to_vec()
            };

            let mut t: SolidityTranscript = t_parsed.into();
            <SolidityTranscript as PlonkTranscript<Fr>>::append_message(&mut t, &[], &msg).unwrap();
            let res: ParsedTranscript = t.into();
            println!("{}", (res,).encode_hex());
        }
        Action::TranscriptAppendField => {
            let arg1 = cli.arg1.as_ref().expect("Should provide arg1=transcript");
            let arg2 = cli.arg2.as_ref().expect("Should provide arg2=fieldElement");
            let t_parsed = arg1.parse::<ParsedTranscript>().unwrap();
            let field = u256_to_field::<Fr>(arg2.parse::<U256>().unwrap());

            let mut t: SolidityTranscript = t_parsed.into();
            t.append_challenge::<Bn254>(&[], &field).unwrap();
            let res: ParsedTranscript = t.into();
            println!("{}", (res,).encode_hex());
        }
        Action::TranscriptAppendGroup => {
            let arg1 = cli.arg1.as_ref().expect("Should provide arg1=transcript");
            let arg2 = cli.arg2.as_ref().expect("Should provide arg2=groupElement");
            let t_parsed = arg1.parse::<ParsedTranscript>().unwrap();
            let point: G1Affine = arg2.parse::<ParsedG1Point>().unwrap().into();

            let mut t: SolidityTranscript = t_parsed.into();
            t.append_commitment::<Bn254, ark_bn254::g1::Config>(&[], &Commitment::from(point))
                .unwrap();
            let res: ParsedTranscript = t.into();
            println!("{}", (res,).encode_hex());
        }
        Action::TranscriptGetChal => {
            let arg1 = cli.arg1.as_ref().expect("Should provide arg1=transcript");
            let t_parsed = arg1.parse::<ParsedTranscript>().unwrap();

            let mut t: SolidityTranscript = t_parsed.into();
            let chal = t.get_and_append_challenge::<Bn254>(&[]).unwrap();

            let updated_t: ParsedTranscript = t.into();
            let res = (updated_t, field_to_u256(chal));
            println!("{}", res.encode_hex());
        }
        Action::TranscriptAppendVkAndPi => {
            let arg1 = cli.arg1.as_ref().expect("Should provide arg1=transcript");
            let arg2 = cli.arg2.as_ref().expect("Should provide arg2=verifyingKey");
            let arg3 = cli.arg3.as_ref().expect("Should provide arg3=publicInput");

            let t_parsed = arg1.parse::<ParsedTranscript>().unwrap();
            let vk_parsed = arg2.parse::<ParsedVerifyingKey>().unwrap();
            let pi_u256: Vec<U256> = AbiDecode::decode_hex(arg3).unwrap();
            let pi: Vec<Fr> = pi_u256.into_iter().map(u256_to_field).collect();

            let mut t: SolidityTranscript = t_parsed.into();
            let vk: VerifyingKey<Bn254> = vk_parsed.into();
            t.append_vk_and_pub_input(&vk, &pi).unwrap();

            let res: ParsedTranscript = t.into();
            println!("{}", (res,).encode_hex());
        }
        Action::PlonkConstants => {
            unimplemented!()
        }
        Action::TestOnly => {
            let arg1 = cli.arg1.as_ref().expect("Should provide arg1=transcript");
            let vk_parsed = arg1.parse::<ParsedVerifyingKey>().unwrap();

            let vk: VerifyingKey<Bn254> = vk_parsed.into();

            let res: ParsedVerifyingKey = vk.into();
            println!("{}", (res,).encode_hex());
        }
    };
}

// ------- Helper functions and structs --------
// ---------------------------------------------

// constant in hex string copied from hardcoded constants from solidity contracts

// TODO: (alex) change to simply using `MontFp!("0x..")` after
// <https://github.com/arkworks-rs/algebra/pull/635> is on a tag release
// Return cosets coefficients for circuits over BN254.
fn coset_k() -> Vec<Fr> {
    vec![
        Fr::from(BigUint::from_str_radix("1", 16).unwrap()),
        Fr::from(
            BigUint::from_str_radix(
                "2f8dd1f1a7583c42c4e12a44e110404c73ca6c94813f85835da4fb7bb1301d4a",
                16,
            )
            .unwrap(),
        ),
        Fr::from(
            BigUint::from_str_radix(
                "1ee678a0470a75a6eaa8fe837060498ba828a3703b311d0f77f010424afeb025",
                16,
            )
            .unwrap(),
        ),
        Fr::from(
            BigUint::from_str_radix(
                "2042a587a90c187b0a087c03e29c968b950b1db26d5c82d666905a6895790c0a",
                16,
            )
            .unwrap(),
        ),
        Fr::from(
            BigUint::from_str_radix(
                "2e2b91456103698adf57b799969dea1c8f739da5d8d40dd3eb9222db7c81e881",
                16,
            )
            .unwrap(),
        ),
    ]
}

/// Returns `OpenKeys` for KZG10 over BN254 curve from Aztec's SRS
fn open_key() -> OpenKey<Bn254> {
    let g = G1Affine::new_unchecked(MontFp!("1"), MontFp!("2"));
    let h = G2Affine::new_unchecked(
        Fp2::new(
            Fq::from(
                BigUint::from_str_radix(
                    "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2",
                    16,
                )
                .unwrap(),
            ),
            Fq::from(
                BigUint::from_str_radix(
                    "1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed",
                    16,
                )
                .unwrap(),
            ),
        ),
        Fp2::new(
            Fq::from(
                BigUint::from_str_radix(
                    "090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b",
                    16,
                )
                .unwrap(),
            ),
            Fq::from(
                BigUint::from_str_radix(
                    "12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
                    16,
                )
                .unwrap(),
            ),
        ),
    );
    let beta_h = G2Affine::new_unchecked(
        Fp2::new(
            Fq::from(
                BigUint::from_str_radix(
                    "260e01b251f6f1c7e7ff4e580791dee8ea51d87a358e038b4efe30fac09383c1",
                    16,
                )
                .unwrap(),
            ),
            Fq::from(
                BigUint::from_str_radix(
                    "0118c4d5b837bcc2bc89b5b398b5974e9f5944073b32078b7e231fec938883b0",
                    16,
                )
                .unwrap(),
            ),
        ),
        Fp2::new(
            Fq::from(
                BigUint::from_str_radix(
                    "04fc6369f7110fe3d25156c1bb9a72859cf2a04641f99ba4ee413c80da6a5fe4",
                    16,
                )
                .unwrap(),
            ),
            Fq::from(
                BigUint::from_str_radix(
                    "22febda3c0c0632a56475b4214e5615e11e6dd3f96e6cea2854a87d4dacc5e55",
                    16,
                )
                .unwrap(),
            ),
        ),
    );

    OpenKey { g, h, beta_h }
}

fn field_to_u256<F: PrimeField>(f: F) -> U256 {
    if F::MODULUS_BIT_SIZE > 256 {
        panic!("Shouldn't convert a >256-bit field to U256");
    }
    U256::from_little_endian(&f.into_bigint().to_bytes_le())
}

fn u256_to_field<F: PrimeField>(x: U256) -> F {
    let mut bytes = [0u8; 32];
    x.to_little_endian(&mut bytes);
    F::from_le_bytes_mod_order(&bytes)
}

/// an intermediate representation of the transcript parsed from abi.encode(transcript) from Solidity.
#[derive(Clone, EthAbiType, EthAbiCodec)]
struct ParsedTranscript {
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

/// an intermediate representation of `BN254.G1Point` in solidity.
#[derive(Clone, Debug, EthAbiType, EthAbiCodec)]
struct ParsedG1Point {
    x: U256,
    y: U256,
}

impl FromStr for ParsedG1Point {
    type Err = AbiError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: (Self,) = AbiDecode::decode_hex(s)?;
        Ok(parsed.0)
    }
}

impl<P: SWCurveConfig> From<Affine<P>> for ParsedG1Point
where
    P::BaseField: PrimeField,
{
    fn from(p: Affine<P>) -> Self {
        Self {
            x: field_to_u256::<P::BaseField>(*p.x().unwrap()),
            y: field_to_u256::<P::BaseField>(*p.y().unwrap()),
        }
    }
}

impl<P: SWCurveConfig> From<ParsedG1Point> for Affine<P>
where
    P::BaseField: PrimeField,
{
    fn from(p: ParsedG1Point) -> Self {
        Self::new(
            u256_to_field::<P::BaseField>(p.x),
            u256_to_field::<P::BaseField>(p.y),
        )
    }
}

/// intermediate representation of `VerifyingKey` in solidity.
#[derive(Clone, Debug, EthAbiType, EthAbiCodec)]
struct ParsedVerifyingKey {
    domain_size: U256,
    num_inputs: U256,
    sigma_0: ParsedG1Point,
    sigma_1: ParsedG1Point,
    sigma_2: ParsedG1Point,
    sigma_3: ParsedG1Point,
    sigma_4: ParsedG1Point,
    q_1: ParsedG1Point,
    q_2: ParsedG1Point,
    q_3: ParsedG1Point,
    q_4: ParsedG1Point,
    q_m_12: ParsedG1Point,
    q_m_34: ParsedG1Point,
    q_o: ParsedG1Point,
    q_c: ParsedG1Point,
    q_h_1: ParsedG1Point,
    q_h_2: ParsedG1Point,
    q_h_3: ParsedG1Point,
    q_h_4: ParsedG1Point,
    q_ecc: ParsedG1Point,
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
