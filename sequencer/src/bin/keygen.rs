//! Utility program to generate keypairs

use async_compatibility_layer::logging::setup_logging;

use anyhow::{self, bail, Context};
use clap::{Parser, ValueEnum};
use derive_more::Display;
use hotshot::types::SignatureKey;
use hotshot_types::signature_key::BLSPubKey;
use rand::{RngCore, SeedableRng};
use sequencer::SchnorrPrivKey;

#[derive(Clone, Copy, Debug, Display, Default, ValueEnum)]
enum Scheme {
    #[default]
    #[display(fmt = "bls")]
    Bls,
    #[display(fmt = "schnorr")]
    Schnorr,
}

/// Utility program to generate keypairs
#[derive(Clone, Debug, Parser)]
struct Options {
    /// Seed for generating keypair.
    /// If not provided, a random seed will be generated.
    #[clap(long, short = 's', value_parser = parse_seed)]
    seed: Option<[u8; 32]>,

    /// Signature scheme to generate.
    #[clap(long, default_value = "bls")]
    scheme: Scheme,

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

fn gen_default_seed() -> [u8; 32] {
    let mut seed = [0u8; 32];
    let mut rng = rand_chacha::ChaChaRng::from_entropy();
    rng.fill_bytes(&mut seed);

    seed
}

fn main() {
    setup_logging();

    let opts = Options::parse();

    tracing::info!("Generating {} {} keypairs", opts.num, opts.scheme);

    let seed = opts.seed.unwrap_or_else(|| {
        tracing::info!("No seed provided, generating a random seed");
        gen_default_seed()
    });

    println!("Seed: {:?}\n", seed);
    for index in 0..opts.num {
        match opts.scheme {
            Scheme::Bls => {
                let (pubkey, priv_key) = BLSPubKey::generated_from_seed_indexed(seed, index as u64);
                println!("PrivateKey: {} PublicKey: {}\n", priv_key, pubkey);
            }
            Scheme::Schnorr => {
                let priv_key = SchnorrPrivKey::generate_from_seed_indexed(seed, index as u64);
                println!("PrivateKey: {priv_key} PublicKey: {}\n", priv_key.pub_key(),);
            }
        }
    }
}
