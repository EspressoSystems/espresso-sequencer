//! Helpers and test mocks for Light Client logic

use ark_std::str::FromStr;
use diff_test_bn254::{field_to_u256, u256_to_field};
use ethers::{
    abi::AbiDecode,
    abi::Token,
    abi::Tokenize,
    prelude::{AbiError, EthAbiCodec, EthAbiType},
    types::U256,
};
use hotshot_types::light_client::{CircuitField, LightClientState, PublicInput};

/// Intermediate representations for `LightClientState` in Solidity
#[derive(Clone, Debug, EthAbiType, EthAbiCodec, PartialEq)]
pub struct ParsedLightClientState {
    pub view_num: u64,
    pub block_height: u64,
    pub block_comm_root: U256,
    pub bls_key_comm: U256,
    pub schnorr_key_comm: U256,
    pub amount_comm: U256,
    pub threshold: U256,
}

impl ParsedLightClientState {
    /// Return a dummy new genesis that will pass constructor/initializer sanity checks
    /// in the contract.
    ///
    /// # Warning
    /// NEVER use this for production, this is test only.
    pub fn dummy_genesis() -> Self {
        Self {
            view_num: 0,
            block_height: 0,
            block_comm_root: U256::from(0),
            bls_key_comm: U256::from(123),
            schnorr_key_comm: U256::from(123),
            amount_comm: U256::from(20),
            threshold: U256::from(1),
        }
    }
}

impl FromStr for ParsedLightClientState {
    type Err = AbiError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: (Self,) = AbiDecode::decode_hex(s)?;
        Ok(parsed.0)
    }
}

impl From<PublicInput> for ParsedLightClientState {
    fn from(pi: PublicInput) -> Self {
        Self {
            view_num: field_to_u256(pi.view_number()).as_u64(),
            block_height: field_to_u256(pi.block_height()).as_u64(),
            block_comm_root: field_to_u256(pi.block_comm_root()),
            bls_key_comm: field_to_u256(pi.qc_key_comm()),
            schnorr_key_comm: field_to_u256(pi.state_key_comm()),
            amount_comm: field_to_u256(pi.stake_amount_comm()),
            threshold: field_to_u256(pi.threshold()),
        }
    }
}

impl From<contract_bindings::light_client::LightClientState> for ParsedLightClientState {
    fn from(state: contract_bindings::light_client::LightClientState) -> Self {
        Self {
            view_num: state.view_num,
            block_height: state.block_height,
            block_comm_root: state.block_comm_root,
            bls_key_comm: state.stake_table_bls_key_comm,
            schnorr_key_comm: state.stake_table_schnorr_key_comm,
            amount_comm: state.stake_table_amount_comm,
            threshold: state.threshold,
        }
    }
}

impl From<ParsedLightClientState> for PublicInput {
    fn from(s: ParsedLightClientState) -> Self {
        let fields = vec![
            u256_to_field(s.threshold),
            CircuitField::from(s.view_num),
            CircuitField::from(s.block_height),
            u256_to_field(s.block_comm_root),
            u256_to_field(s.bls_key_comm),
            u256_to_field(s.schnorr_key_comm),
            u256_to_field(s.amount_comm),
        ];
        Self::from(fields)
    }
}

impl From<(u64, u64, U256, U256, U256, U256, U256)> for ParsedLightClientState {
    fn from(s: (u64, u64, U256, U256, U256, U256, U256)) -> Self {
        Self {
            view_num: s.0,
            block_height: s.1,
            block_comm_root: s.2,
            bls_key_comm: s.3,
            schnorr_key_comm: s.4,
            amount_comm: s.5,
            threshold: s.6,
        }
    }
}

impl From<ParsedLightClientState> for LightClientState {
    fn from(s: ParsedLightClientState) -> Self {
        Self {
            view_number: s.view_num as usize,
            block_height: s.block_height as usize,
            block_comm_root: u256_to_field(s.block_comm_root),
        }
    }
}

impl From<ParsedLightClientState> for contract_bindings::light_client::LightClientState {
    fn from(s: ParsedLightClientState) -> Self {
        // exactly the same struct with same field types, safe to transmute
        unsafe { std::mem::transmute(s) }
    }
}

/// `LightClientConstructorArgs` holds the arguments required to initialize a light client contract.
pub struct LightClientConstructorArgs {
    pub light_client_state: ParsedLightClientState,
    pub max_history_seconds: u32,
}

impl LightClientConstructorArgs {
    /// Creates a `LightClientConstructorArgs` instance with dummy genesis data.
    ///
    /// # Warning
    /// NEVER use this for production, this is test only.
    pub fn dummy_genesis() -> Self {
        Self {
            light_client_state: ParsedLightClientState::dummy_genesis(),
            max_history_seconds: 864000,
        }
    }
}

impl Tokenize for LightClientConstructorArgs {
    /// Converts the `LightClientConstructorArgs` into a vector of tokens.
    /// the Tokenize trait is used to convert types into Ethereum-compatible ABI tokens
    ///
    /// This method is used to serialize the constructor arguments into an
    /// Ethereum-compatible token that can be read by a smart contract.
    fn into_tokens(self) -> Vec<Token> {
        vec![
            ethers::abi::Token::Tuple(self.light_client_state.into_tokens()),
            ethers::abi::Token::Uint(U256::from(self.max_history_seconds)),
        ]
    }
}
