use coins_bip32::path::DerivationPath;
use ethers::{
    signers::{
        coins_bip39::{English, Mnemonic},
        LocalWallet, WalletError,
    },
    types::{Address, Signature},
    utils::public_key_to_address,
};
use hotshot_types::traits::signature_key::BuilderSignatureKey;
use k256::ecdsa::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{
    fmt::{Display, Formatter},
    hash::Hash,
    str::FromStr as _,
};

// Newtype because type doesn't implement Hash, Display, SerDe, Ord, PartialOrd
#[derive(PartialEq, Eq, Clone)]
pub struct EthSigningKey {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    address: Address,
}

impl From<SigningKey> for EthSigningKey {
    fn from(signing_key: SigningKey) -> Self {
        let verifying_key = VerifyingKey::from(&signing_key);
        let address = public_key_to_address(&verifying_key);
        EthSigningKey {
            signing_key,
            verifying_key,
            address,
        }
    }
}

impl EthSigningKey {
    pub fn from_mnemonic(phrase: &str, index: impl Into<u32>) -> Result<Self, WalletError> {
        let index: u32 = index.into();
        let derivation_path = DerivationPath::from_str(&format!("m/44'/60'/0'/0/{index}"))?;
        let mnemonic = Mnemonic::<English>::new_from_phrase(phrase)?;
        let derived_priv_key = mnemonic.derive_key(derivation_path, /* password */ None)?;
        let key: &coins_bip32::prelude::SigningKey = derived_priv_key.as_ref();
        let signing_key = SigningKey::from_bytes(&key.to_bytes())?;
        Ok(signing_key.into())
    }

    pub fn address(&self) -> Address {
        self.address
    }
}

impl Hash for EthSigningKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.signing_key.to_bytes().hash(state);
    }
}

// Always display the address, not the private key
impl Display for EthSigningKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EthSigningKey(address={:?})", self.address())
    }
}

impl std::fmt::Debug for EthSigningKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Serialize for EthSigningKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.signing_key.to_bytes().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EthSigningKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes = <[u8; 32]>::deserialize(deserializer)?;
        Ok(EthSigningKey::from(
            SigningKey::from_slice(&bytes).map_err(serde::de::Error::custom)?,
        ))
    }
}

impl PartialOrd for EthSigningKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EthSigningKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.signing_key
            .as_nonzero_scalar()
            .cmp(other.signing_key.as_nonzero_scalar())
    }
}

#[derive(Clone, Debug, Snafu)]
pub struct SigningError;

impl BuilderSignatureKey for EthSigningKey {
    type BuilderPrivateKey = Self;
    type BuilderSignature = Signature;
    type SignError = SigningError;

    fn validate_builder_signature(&self, signature: &Self::BuilderSignature, data: &[u8]) -> bool {
        signature.verify(data, self.address()).is_ok()
    }

    fn sign_builder_message(
        private_key: &Self::BuilderPrivateKey,
        data: &[u8],
    ) -> Result<Self::BuilderSignature, Self::SignError> {
        let wallet = LocalWallet::from_bytes(&private_key.signing_key.to_bytes()).unwrap();
        let message_hash = ethers::utils::hash_message(data);
        wallet.sign_hash(message_hash).map_err(|_| SigningError)
    }

    fn generated_from_seed_indexed(seed: [u8; 32], index: u64) -> (Self, Self::BuilderPrivateKey) {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&seed);
        hasher.update(&index.to_le_bytes());
        let new_seed = *hasher.finalize().as_bytes();
        let signing_key = EthSigningKey::from(SigningKey::from_slice(&new_seed).unwrap());
        (signing_key.clone(), signing_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hotshot_types::traits::signature_key::BuilderSignatureKey;

    impl EthSigningKey {
        fn for_test() -> Self {
            EthSigningKey::generated_from_seed_indexed([0u8; 32], 0).0
        }
    }

    #[test]
    fn test_fmt() {
        let key = EthSigningKey::for_test();
        let expected = "EthSigningKey(address=0xb0cfa4e5893107e2995974ef032957752bb526e9)";
        assert_eq!(format!("{}", key), expected);
        assert_eq!(format!("{:?}", key), expected);
    }

    #[test]
    fn test_derivation_from_mnemonic() {
        let mnemonic = "test test test test test test test test test test test junk";
        let key0 = EthSigningKey::from_mnemonic(mnemonic, 0u32).unwrap();
        assert_eq!(
            key0.address(),
            "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
                .parse()
                .unwrap()
        );
        let key1 = EthSigningKey::from_mnemonic(mnemonic, 1u32).unwrap();
        assert_eq!(
            key1.address(),
            "0x70997970C51812dc3A010C7d01b50e0d17dc79C8"
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn test_key_serde() {
        let key = EthSigningKey::for_test();
        let serialized = bincode::serialize(&key).unwrap();
        let deserialized: EthSigningKey = bincode::deserialize(&serialized).unwrap();
        assert_eq!(key, deserialized);
    }

    #[test]
    fn test_signing_and_recovery() {
        // Recovery works
        let key = EthSigningKey::for_test();
        let msg = b"hello world";
        let sig = EthSigningKey::sign_builder_message(&key, msg).unwrap();
        assert!(key.validate_builder_signature(&sig, msg));

        // Recovery fails if signed with other key
        let other_key = EthSigningKey::generated_from_seed_indexed([0u8; 32], 1).0;
        let sig = EthSigningKey::sign_builder_message(&other_key, msg).unwrap();
        assert!(!key.validate_builder_signature(&sig, msg));

        // Recovery fails if another message was signed
        let sig = EthSigningKey::sign_builder_message(&key, b"hello world XYZ").unwrap();
        assert!(!key.validate_builder_signature(&sig, msg));
    }
}
