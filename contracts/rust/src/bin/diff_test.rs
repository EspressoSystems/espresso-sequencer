use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use ark_poly::domain::radix2::Radix2EvaluationDomain;
use ark_poly::EvaluationDomain;
use clap::{Parser, ValueEnum};
use ethers::{
    abi::{AbiDecode, AbiEncode},
    types::U256,
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
            let arg3 = cli.arg2.as_ref().expect("Should provide arg3=publicInput");
            let log_size = arg1.parse::<u32>().unwrap();
            let zeta = u256_to_field::<Fr>(arg2.parse::<U256>().unwrap());
            // let pi =
        }
        Action::TestOnly => {
            let arg1 = cli.arg1.as_ref().expect("Should provide arg1=array");
            let array: Vec<U256> = AbiDecode::decode_hex(arg1).unwrap();
            println!("rust side: {:?}", array);
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
