pub mod client_id;
pub mod client_message;
pub mod client_state;
pub mod data_state;
pub mod espresso_inscription;
pub mod server_message;
pub mod storage;

use alloy::{
    primitives::Address,
    signers::Signature,
    sol_types::{eip712_domain, Eip712Domain, SolStruct},
};
use espresso_inscription::{
    EspressoInscription, InscriptionAndSignature, InscriptionAndSignatureFromService,
};
use serde::{Deserialize, Serialize};

/// [ESPRESSO_INSCRIPTION_MESSAGE] represents the text contents of the message
/// to be signed.
pub const ESPRESSO_INSCRIPTION_MESSAGE: &str = "An Infinite Garden has no walls";

/// [ESPRESSO_EIP712_DOMAIN] represents the EIP712 Domain for the Espresso
/// Inscription dAPP Domain.
pub const ESPRESSO_EIP712_DOMAIN: Eip712Domain = eip712_domain! {
    name: "Espresso Inscription",
};

/// [InscriptionVerificationResult] represents the potential results of
/// verifying an Inscription and a Signature from the Espresso Inscription
/// Front-End.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InscriptionVerificationError {
    InvalidInscriptionMessage,
    InvalidSignature,
}

impl std::fmt::Display for InscriptionVerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InscriptionVerificationError::InvalidInscriptionMessage => {
                write!(f, "InvalidInscriptionMessage")
            }
            InscriptionVerificationError::InvalidSignature => write!(f, "InvalidSignature"),
        }
    }
}

impl std::error::Error for InscriptionVerificationError {}

/// [validate_inscription_and_signature_matches_address] validates an
/// [InscriptionAndSignatureFromService] by ensuring that the signature matches
/// the address given in the function.
pub fn validate_inscription_and_signature_matches_address(
    inscription: &EspressoInscription,
    signature: &Signature,
    address: Address,
) -> Result<(), InscriptionVerificationError> {
    if inscription.message != ESPRESSO_INSCRIPTION_MESSAGE {
        return Err(InscriptionVerificationError::InvalidInscriptionMessage);
    }

    let recovered_address = signature
        .recover_address_from_prehash(&inscription.eip712_signing_hash(&ESPRESSO_EIP712_DOMAIN))
        .unwrap();

    if recovered_address != address {
        return Err(InscriptionVerificationError::InvalidSignature);
    }

    Ok(())
}

/// [validate_inscription_and_signature_from_service] validates an
/// [InscriptionAndSignatureFromService] by ensuring that the signature
/// matches the address given in the function.
pub fn validate_inscription_and_signature_from_service(
    inscription_and_signature: &InscriptionAndSignatureFromService,
    address: Address,
) -> Result<(), InscriptionVerificationError> {
    let inscription = &inscription_and_signature.inscription;
    let signature = &inscription_and_signature.signature;

    validate_inscription_and_signature_matches_address(inscription, &signature.0, address)
}

/// [validate_inscription_and_signature] validates an Inscription by ensuring
/// that the signed message matches what is expected, and that the signature
/// does indeed match the address contained within the [EspressoInscription].
pub fn validate_inscription_and_signature(
    inscription_and_signature: &InscriptionAndSignature,
) -> Result<(), InscriptionVerificationError> {
    let inscription = &inscription_and_signature.inscription;
    let signature = &inscription_and_signature.signature;

    validate_inscription_and_signature_matches_address(
        inscription,
        &signature.0,
        inscription.address,
    )
}

/// [generate_inscription_and_signature] generates a completely valid
/// [InscriptionAndSignature] for testing purposes.
#[cfg(test)]
fn generate_inscription_and_signature(message: &str) -> InscriptionAndSignature {
    use alloy::signers::{local::PrivateKeySigner, SignerSync};
    use espresso_inscription::HexSignature;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let signer = PrivateKeySigner::random();
    let message = EspressoInscription {
        address: signer.address(),
        message: message.to_string(),
        time: now,
    };

    let signature = signer
        .sign_hash_sync(&message.eip712_signing_hash(&ESPRESSO_EIP712_DOMAIN))
        .unwrap();

    InscriptionAndSignature {
        inscription: message,
        signature: HexSignature(signature),
    }
}

/// [test_signing_and_verifying_signature] tests and ensures that the
/// [validate_inscription_and_signature] returns [InscriptionVerificationResult::Valid]
/// when provided a completely valid [InscriptionAndSignature].
#[test]
fn test_signing_and_verifying_signature_result_valid() {
    let inscription_and_signature =
        generate_inscription_and_signature(ESPRESSO_INSCRIPTION_MESSAGE);

    assert_eq!(
        validate_inscription_and_signature(&inscription_and_signature),
        Ok(())
    );
}

/// [test_signing_and_verifying_signature_result_invalid_message] tests and
/// ensures that the [validate_inscription_and_signature] returns
/// [InscriptionVerificationResult::InvalidInscriptionMessage] when provided an
/// [InscriptionAndSignature] whose [EspressoInscription]'s message does not
/// match the expected message, [ESPRESSO_INSCRIPTION_MESSAGE].
#[test]
fn test_signing_and_verifying_signature_result_invalid_message() {
    let inscription_and_signature = generate_inscription_and_signature("Invalid Message");

    assert_eq!(
        validate_inscription_and_signature(&inscription_and_signature),
        Err(InscriptionVerificationError::InvalidInscriptionMessage)
    );
}

/// [test_signing_and_verifying_signature_result_invalid_message] tests and
/// ensures that the [validate_inscription_and_signature] returns
/// [InscriptionVerificationResult::InvalidInscriptionMessage] when provided an
/// [InscriptionAndSignature] whose [EspressoInscription]'s address does not
/// match the recovered address from the signature.
#[test]
fn test_signing_and_verifying_signature_result_invalid_signature() {
    use alloy::primitives::address;

    let inscription_and_signature =
        generate_inscription_and_signature(ESPRESSO_INSCRIPTION_MESSAGE);
    let InscriptionAndSignature {
        inscription,
        signature,
    } = inscription_and_signature;
    let EspressoInscription { message, time, .. } = inscription;

    let inscription_and_signature = InscriptionAndSignature {
        inscription: EspressoInscription {
            address: address!("0000000000000000000000000000000000000001"),
            message,
            time,
        },
        signature,
    };

    assert_eq!(
        validate_inscription_and_signature(&inscription_and_signature),
        Err(InscriptionVerificationError::InvalidSignature)
    );
}

/// [test_signing_and_verifying_signature_result_is_valid_for_sample_javascript_signature]
/// tests and ensures that the [validate_inscription_and_signature] returns
/// [InscriptionVerificationResult::Valid] when provided an
/// [InscriptionAndSignature] that was signed by the sample JavaScript code.
#[test]
fn test_signing_and_verifying_signature_result_is_valid_for_sample_javascript_signature() {
    use alloy::primitives::address;
    use espresso_inscription::HexSignature;
    use std::str::FromStr;

    let inscription_and_signature = InscriptionAndSignature {
        inscription: EspressoInscription {
            address: address!("2BB6723A754D21D5BA0F22Fc062e34E6CE47A0Cc"),
            message: ESPRESSO_INSCRIPTION_MESSAGE.to_string(),
            time: 1730469480,

        },
        signature: HexSignature(Signature::from_str("0xbc72f48c817fc6865e7051c64fd1194bb5b696eb0d70e6e5a35c99378db1e3c17e70ee1e65b81b7d8dfee918cf7cab9691fa5bc59ebb64b84d070ff3a872e1f41b").unwrap()),
    };

    assert_eq!(
        validate_inscription_and_signature(&inscription_and_signature),
        Ok(())
    );
}
