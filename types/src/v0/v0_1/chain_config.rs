use super::{FeeAccount, FeeAmount};
use committable::{Commitment, Committable, RawCommitmentBuilder};
use derive_more::{From, Into};
use ethers::prelude::{Address, U256};
use itertools::Either;
use sequencer_utils::impl_to_fixed_bytes;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Hash, Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, From, Into)]
pub struct ChainId(U256);

/// Global variables for an Espresso blockchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChainConfig {
    pub(crate) chain_id: ChainId,
    pub(crate) max_block_size: u64,
    pub(crate) base_fee: FeeAmount,
    pub(crate) fee_contract: Option<Address>,
    pub(crate) fee_recipient: FeeAccount,
}

#[derive(Clone, Debug, Copy, PartialEq, Deserialize, Serialize, Eq, Hash)]
pub struct ResolvableChainConfig {
    chain_config: Either<ChainConfig, Commitment<ChainConfig>>,
}
