//! Helpers and test mocks for Light Client logic

use alloy::primitives::U256;
use ark_ff::PrimeField;
use contract_bindings::{iplonkverifier::IPlonkVerifier::PlonkProof, lightclient::LightClient::finalizedStateReturn};
use diff_test_bn254::{field_to_u256, u256_to_field};
use ethers_conv::{ToAlloy, ToEthers};
use hotshot_types::light_client::{GenericLightClientState, GenericStakeTableState, PublicInput};

use crate::jellyfish::ParsedPlonkProof;

/// Intermediate representations for `LightClientState` in Solidity
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedLightClientState {
    pub view_num: u64,
    pub block_height: u64,
    pub block_comm_root: U256,
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
            block_comm_root: U256::ZERO,
        }
    }
}

impl From<contract_bindings::lightclient::LightClient::LightClientState>
    for ParsedLightClientState
{
    fn from(state: contract_bindings::lightclient::LightClient::LightClientState) -> Self {
        Self {
            view_num: state.viewNum,
            block_height: state.blockHeight,
            block_comm_root: state.blockCommRoot,
        }
    }
}

impl From<finalizedStateReturn> for ParsedLightClientState {
    fn from(state: finalizedStateReturn) -> Self {
        Self {
            view_num: state.viewNum,
            block_height: state.blockHeight,
            block_comm_root: state.blockCommRoot,
        }
    }
}

impl From<contract_bindings::lightclientmock::LightClient::LightClientState>
    for ParsedLightClientState
{
    fn from(state: contract_bindings::lightclientmock::LightClient::LightClientState) -> Self {
        Self {
            view_num: state.viewNum,
            block_height: state.blockHeight,
            block_comm_root: state.blockCommRoot,
        }
    }
}

impl<F: PrimeField> From<ParsedLightClientState> for GenericLightClientState<F> {
    fn from(v: ParsedLightClientState) -> Self {
        Self {
            view_number: v.view_num as usize,
            block_height: v.block_height as usize,
            block_comm_root: u256_to_field(v.block_comm_root.to_ethers()),
        }
    }
}

impl<F: PrimeField> From<GenericLightClientState<F>> for ParsedLightClientState {
    fn from(v: GenericLightClientState<F>) -> Self {
        Self {
            view_num: v.view_number as u64,
            block_height: v.block_height as u64,
            block_comm_root: field_to_u256(v.block_comm_root).to_alloy(),
        }
    }
}

impl From<PublicInput> for ParsedLightClientState {
    fn from(pi: PublicInput) -> Self {
        Self {
            view_num: field_to_u256(pi.view_number()).as_u64(),
            block_height: field_to_u256(pi.block_height()).as_u64(),
            block_comm_root: field_to_u256(pi.block_comm_root()).to_alloy(),
        }
    }
}

impl From<(u64, u64, U256)> for ParsedLightClientState {
    fn from(s: (u64, u64, U256)) -> Self {
        Self {
            view_num: s.0,
            block_height: s.1,
            block_comm_root: s.2,
        }
    }
}

impl From<ParsedLightClientState>
    for contract_bindings::lightclient::LightClient::LightClientState
{
    fn from(s: ParsedLightClientState) -> Self {
        // exactly the same struct with same field types, safe to transmute
        unsafe { std::mem::transmute(s) }
    }
}

impl From<ParsedLightClientState>
    for contract_bindings::lightclientmock::LightClient::LightClientState
{
    fn from(s: ParsedLightClientState) -> Self {
        // exactly the same struct with same field types, safe to transmute
        unsafe { std::mem::transmute(s) }
    }
}

/// Parsed Stake State
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedStakeTableState {
    pub threshold: U256,
    pub bls_key_comm: U256,
    pub schnorr_key_comm: U256,
    pub amount_comm: U256,
}

impl ParsedStakeTableState {
    /// Return a dummy new genesis stake state that will pass constructor/initializer sanity checks
    /// in the contract.
    ///
    /// # Warning
    /// NEVER use this for production, this is test only.
    pub fn dummy_genesis() -> Self {
        Self {
            threshold: U256::from(1),
            bls_key_comm: U256::from(123),
            schnorr_key_comm: U256::from(123),
            amount_comm: U256::from(20),
        }
    }
}

impl<F: PrimeField> From<ParsedStakeTableState> for GenericStakeTableState<F> {
    fn from(s: ParsedStakeTableState) -> Self {
        Self {
            threshold: u256_to_field(s.threshold.to_ethers()),
            bls_key_comm: u256_to_field(s.bls_key_comm.to_ethers()),
            schnorr_key_comm: u256_to_field(s.schnorr_key_comm.to_ethers()),
            amount_comm: u256_to_field(s.amount_comm.to_ethers()),
        }
    }
}

impl From<contract_bindings::lightclient::LightClient::StakeTableState> for ParsedStakeTableState {
    fn from(state: contract_bindings::lightclient::LightClient::StakeTableState) -> Self {
        Self {
            threshold: state.threshold,
            bls_key_comm: state.blsKeyComm,
            schnorr_key_comm: state.schnorrKeyComm,
            amount_comm: state.amountComm,
        }
    }
}


impl From<contract_bindings::lightclient::LightClient::genesisStakeTableStateReturn> for ParsedStakeTableState {
    fn from(state: contract_bindings::lightclient::LightClient::genesisStakeTableStateReturn) -> Self {
        Self {
            threshold: state.threshold,
            bls_key_comm: state.blsKeyComm,
            schnorr_key_comm: state.schnorrKeyComm,
            amount_comm: state.amountComm,
        }
    }
}

impl From<contract_bindings::lightclientmock::LightClient::StakeTableState>
    for ParsedStakeTableState
{
    fn from(state: contract_bindings::lightclientmock::LightClient::StakeTableState) -> Self {
        Self {
            threshold: state.threshold,
            bls_key_comm: state.blsKeyComm,
            schnorr_key_comm: state.schnorrKeyComm,
            amount_comm: state.amountComm,
        }
    }
}

impl From<(U256, U256, U256, U256)> for ParsedStakeTableState {
    fn from(s: (U256, U256, U256, U256)) -> Self {
        Self {
            threshold: s.0,
            bls_key_comm: s.1,
            schnorr_key_comm: s.2,
            amount_comm: s.3,
        }
    }
}

impl From<ParsedStakeTableState> for contract_bindings::lightclient::LightClient::StakeTableState {
    fn from(s: ParsedStakeTableState) -> Self {
        // exactly the same struct with same field types, safe to transmute
        unsafe { std::mem::transmute(s) }
    }
}

impl From<ParsedStakeTableState>
    for contract_bindings::lightclientmock::LightClient::StakeTableState
{
    fn from(s: ParsedStakeTableState) -> Self {
        // exactly the same struct with same field types, safe to transmute
        unsafe { std::mem::transmute(s) }
    }
}

impl<F: PrimeField> From<GenericStakeTableState<F>> for ParsedStakeTableState {
    fn from(v: GenericStakeTableState<F>) -> Self {
        Self {
            bls_key_comm: field_to_u256(v.bls_key_comm).to_alloy(),
            schnorr_key_comm: field_to_u256(v.schnorr_key_comm).to_alloy(),
            amount_comm: field_to_u256(v.amount_comm).to_alloy(),
            threshold: field_to_u256(v.threshold).to_alloy(),
        }
    }
}

/// `LightClientConstructorArgs` holds the arguments required to initialize a light client contract.
pub struct LightClientConstructorArgs {
    pub light_client_state: ParsedLightClientState,
    pub stake_table_state: ParsedStakeTableState,
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
            stake_table_state: ParsedStakeTableState::dummy_genesis(),
            max_history_seconds: 864000,
        }
    }
}
