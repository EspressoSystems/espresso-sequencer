//! Utility program to generate keypairs

use async_compatibility_layer::logging::setup_logging;

use anyhow::{self, bail, Context};
use clap::Parser;
use hotshot::types::SignatureKey;
use hotshot_types::signature_key::BLSPubKey;

const DEFAULT_SEED : &str = "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]";

/// Utility program to generate keypairs
#[derive(Clone, Debug, Parser)]
struct Options {
    /// Seed for generating keypair.
    /// Default is all zeros.
    #[clap(long, short = 's',  default_value = DEFAULT_SEED, value_parser = parse_seed)]
    seed: [u8; 32],

    /// Number of keypairs to generate.
    /// Default is 1.
    #[clap(long, short = 'n', default_value = "1")]
    num: usize,
}

fn parse_seed(s: &str) -> Result<[u8; 32], anyhow::Error> {
    // Remove the leading and trailing brackets if present
    let s = s.trim_start_matches('[').trim_end_matches(']');

    // Split the string by comma and parse each element as u8
    let mut seed: [u8; 32] = [0; 32];
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 32 {
        bail!("Invalid seed length: {}", parts.len());
    }

    for (i, part) in parts.iter().enumerate() {
        seed[i] = part.trim().parse().context("Failed to parse seed")?;
    }
    Ok(seed)
}

fn main() {
    setup_logging();

    let opts = Options::parse();

    tracing::info!("Generating {} keypairs", opts.num);

    for index in 0..opts.num {
        let (pubkey, priv_key) = BLSPubKey::generated_from_seed_indexed(opts.seed, index as u64);
        println!("PrivateKey: {} PublicKey: {}\n", priv_key, pubkey);
    }
}
