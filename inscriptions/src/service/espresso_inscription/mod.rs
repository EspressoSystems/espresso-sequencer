use std::str::FromStr;

use alloy::{signers::Signature, sol};
use const_hex::hex;

sol! {
    /// [EspressoInscription] represents the raw Inscription data that the user
    /// will be signing utilizing his/her wallet to add to the Block Chain.
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct EspressoInscription {
        address address;
        string message;
        uint64 time;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HexSignature(pub Signature);

impl<'de> serde::de::Deserialize<'de> for HexSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        Signature::from_str(&string)
            .map(HexSignature)
            .map_err(serde::de::Error::custom)
    }
}

impl serde::ser::Serialize for HexSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let bytes: [u8; 65] = self.0.into();
        let string = hex::encode(bytes);
        serializer.serialize_str(&string)
    }
}

/// [InscriptionAndSignature] represents a combination of an [EspressoInscription]
/// and a [Signature] that is coming from a user's submitted data.  This
/// combination is used to represent the raw message that the user is signing,
/// along with a signature to verify that the signature matches the contents
/// of the data being signed.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct InscriptionAndSignature {
    pub(crate) inscription: EspressoInscription,
    pub(crate) signature: HexSignature,
}

/// [InscriptionAndSignatureFromService] represents a combination of an
/// [EspressoInscription] that has been validated and found to be valid.
/// The [Signature] contained within is meant to come from the Service itself
/// and is used to verify that information coming into the service from the
/// Espresso Block Chain itself is valid, and accurate.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct InscriptionAndSignatureFromService {
    pub(crate) inscription: EspressoInscription,
    pub(crate) signature: HexSignature,
}

/// [ChainDetails] represents the details of the chain that corresponds to the
/// Espresso Block Chain. This information can be utilized to determine the
/// appropriate Block Explorer link.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ChainDetails {
    pub(crate) block: u64,
    pub(crate) offset: u64,
}

/// [InscriptionAndChainDetails] represents a combination of an
/// [EspressoInscription] and [ChainDetails] that is used to represent the
/// Inscription that has been submitted to the Espresso Block Chain, along with
/// the details of the block and transaction offset that this applies to.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct InscriptionAndChainDetails {
    pub(crate) inscription: EspressoInscription,
    pub(crate) chain_details: ChainDetails,
}
