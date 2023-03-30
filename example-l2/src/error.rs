use ethers::abi::Address;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use tide_disco::StatusCode;

#[derive(Snafu, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RollupError {
    #[snafu(display("Error validating the transaction signature."))]
    SignatureError,
    #[snafu(display("Insufficient balance for sender: {address}."))]
    InsufficientBalance { address: Address },
    #[snafu(display("Invalid nonce. Nonces must increase consecutively."))]
    InvalidNonce,
}

impl tide_disco::Error for RollupError {
    fn catch_all(_status: StatusCode, _message: String) -> Self {
        RollupError::InvalidNonce
    }

    fn status(&self) -> StatusCode {
        match self {
            RollupError::SignatureError => StatusCode::BadRequest,
            RollupError::InsufficientBalance { .. } => StatusCode::BadRequest,
            RollupError::InvalidNonce => StatusCode::BadRequest,
        }
    }
}
