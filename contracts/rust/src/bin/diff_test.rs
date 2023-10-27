use anyhow::{anyhow, Result};
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use ark_poly::domain::radix2::Radix2EvaluationDomain;
use ark_poly::EvaluationDomain;
use clap::{Parser, ValueEnum};
use ethers::{abi::AbiEncode, types::U256};

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
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Action {
    /// Getting ark_poly::Radix2EvaluationDomain::new()
    NewPolyEvalDomain,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.action {
        Action::NewPolyEvalDomain => {
            if let Some(arg1) = cli.arg1.as_ref() {
                let log_size = arg1.parse::<u32>()?;
                let domain =
                    Radix2EvaluationDomain::<Fr>::new(2u32.pow(log_size) as usize).unwrap();
                let res = (
                    field_to_u256(domain.size_inv)?,
                    field_to_u256(domain.group_gen)?,
                    field_to_u256(domain.group_gen_inv)?,
                );
                println!("{}", res.encode_hex());
            }
        }
    };

    Ok(())
}

fn field_to_u256<F: PrimeField>(f: F) -> Result<U256> {
    if F::MODULUS_BIT_SIZE > 256 {
        return Err(anyhow!("Shouldn't convert a > 256-bit field to U256"));
    }
    Ok(U256::from_little_endian(&f.into_bigint().to_bytes_le()))
}
