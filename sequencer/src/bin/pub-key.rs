use anyhow::bail;
use clap::Parser;
use hotshot_types::{
    light_client::{StateKeyPair, StateSignKey},
    traits::signature_key::SignatureKey,
};
use sequencer::{PrivKey, PubKey};
use std::str::FromStr;

#[derive(Clone, Debug)]
enum PrivateKey {
    Bls(PrivKey),
    Schnorr(StateSignKey),
}

impl FromStr for PrivateKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(key) = s.parse() {
            Ok(Self::Bls(key))
        } else if let Ok(key) = s.parse() {
            Ok(Self::Schnorr(key))
        } else {
            bail!("unrecognized key type")
        }
    }
}

/// Get the public key corresponding to a private key.
#[derive(Clone, Debug, Parser)]
struct Options {
    key: PrivateKey,
}

fn main() {
    let opt = Options::parse();
    match opt.key {
        PrivateKey::Bls(key) => println!("{}", PubKey::from_private(&key)),
        PrivateKey::Schnorr(key) => println!("{}", StateKeyPair::from_sign_key(key).ver_key()),
    }
}
