use crate::state::Nonce;
use ethers::abi::Address;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

#[derive(Snafu, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RollupError {
    #[snafu(display("Error validating the transaction signature."))]
    SignatureError,
    #[snafu(display("Insufficient balance for sender: {address}."))]
    InsufficientBalance { address: Address },
    #[snafu(display("Invalid nonce for sender {address}. Found {actual}, expected {expected}"))]
    InvalidNonce {
        address: Address,
        expected: Nonce,
        actual: Nonce,
    },
}
