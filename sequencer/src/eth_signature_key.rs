use derive_more::From;
use ethers::{signers::LocalWallet, types::Signature, utils::public_key_to_address};
use hotshot_types::traits::signature_key::BuilderSignatureKey;
use k256::ecdsa::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{
    fmt::{Display, Formatter},
    hash::Hash,
};

// Newtype because type doesn't implement Hash, Display, SerDe, Ord, PartialOrd
#[derive(Debug, From, PartialEq, Eq, Clone)]
pub struct EthSigKey(SigningKey);

impl Hash for EthSigKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bytes().hash(state);
    }
}

impl Display for EthSigKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EthSignatureKey")
    }
}

impl Serialize for EthSigKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.to_bytes().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EthSigKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        todo!()
    }
}

impl PartialOrd for EthSigKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl Ord for EthSigKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}

#[derive(Clone, Debug, Snafu)]
pub struct SigningError;

impl BuilderSignatureKey for EthSigKey {
    type BuilderPrivateKey = Self;
    type BuilderSignature = Signature;
    type SignError = SigningError;

    fn validate_builder_signature(&self, signature: &Self::BuilderSignature, data: &[u8]) -> bool {
        let verifying_key = VerifyingKey::from(&self.0);
        let address = public_key_to_address(&verifying_key);
        signature.verify(data, address).is_ok()
    }

    fn sign_builder_message(
        private_key: &Self::BuilderPrivateKey,
        data: &[u8],
    ) -> Result<Self::BuilderSignature, Self::SignError> {
        let wallet = LocalWallet::from_bytes(&private_key.0.to_bytes()).unwrap();
        let message_hash = ethers::utils::hash_message(data);
        wallet.sign_hash(message_hash).map_err(|_| SigningError)
    }

    fn generated_from_seed_indexed(seed: [u8; 32], index: u64) -> (Self, Self::BuilderPrivateKey) {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&seed);
        hasher.update(&index.to_le_bytes());
        let new_seed = *hasher.finalize().as_bytes();
        let signing_key = EthSigKey::from(SigningKey::from_slice(&new_seed).unwrap());
        (signing_key.clone(), signing_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hotshot_types::traits::signature_key::BuilderSignatureKey;

    #[test]
    fn test_signing_and_recovery() {
        let seed = [0u8; 32];
        let index = 0;
        let (signing_key, _) = EthSigKey::generated_from_seed_indexed(seed, index);
        let message = b"hello world";
        let signature = EthSigKey::sign_builder_message(&signing_key, message).unwrap();
        assert!(signing_key.validate_builder_signature(&signature, message));
    }
}
