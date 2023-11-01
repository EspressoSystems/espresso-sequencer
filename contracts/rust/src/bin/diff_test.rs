use ark_bn254::{Bn254, Fr};
use ark_ff::{BigInteger, PrimeField};
use ark_poly::domain::radix2::Radix2EvaluationDomain;
use ark_poly::EvaluationDomain;
use clap::{Parser, ValueEnum};
use ethers::{
    abi::{AbiDecode, AbiEncode},
    prelude::{EthAbiCodec, EthAbiType},
    types::{Bytes, H256, U256},
};
use jf_plonk::{
    constants::KECCAK256_STATE_SIZE,
    testing_apis::Verifier,
    transcript::{PlonkTranscript, SolidityTranscript},
};

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
            let t_parsed = {
                let parsed: (ParsedTranscript,) = AbiDecode::decode_hex(arg1).unwrap();
                parsed.0
            };
            let msg = {
                let parsed: Bytes = AbiDecode::decode_hex(arg2).unwrap();
                parsed.0.to_vec()
            };

            let mut t: SolidityTranscript = t_parsed.into();
            <SolidityTranscript as PlonkTranscript<Fr>>::append_message(&mut t, &[], &msg).unwrap();
            let res: ParsedTranscript = t.into();
            println!("{}", (res,).encode_hex());
        }
        Action::TestOnly => {
            let arg1 = cli.arg1.as_ref().expect("Should provide arg1=transcript");
            let t_parsed = {
                let parsed: (ParsedTranscript,) = AbiDecode::decode_hex(arg1).unwrap();
                parsed.0
            };
            println!("{}", (t_parsed,).encode_hex());
        }
    };
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
