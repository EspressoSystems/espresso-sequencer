use committable::Commitment;
use derive_more::{Deref, Display, From, Into};
use ethers::types::{Address, U256};
use itertools::Either;

use serde::{Deserialize, Serialize};

use crate::{FeeAccount, FeeAmount};

#[derive(Default, Hash, Copy, Clone, Debug, Display, PartialEq, Eq, From, Into)]
#[display(fmt = "{_0}")]
pub struct ChainId(pub U256);

#[derive(Hash, Copy, Clone, Debug, Default, Display, PartialEq, Eq, From, Into, Deref)]
#[display(fmt = "{_0}")]
pub struct BlockSize(pub(crate) u64);

/// Global variables for an Espresso blockchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChainConfig {
    /// Espresso chain ID
    pub chain_id: ChainId,

    /// Maximum size in bytes of a block
    pub max_block_size: BlockSize,

    /// Minimum fee in WEI per byte of payload
    pub base_fee: FeeAmount,

    /// Fee contract address on L1.
    ///
    /// This is optional so that fees can easily be toggled on/off, with no need to deploy a
    /// contract when they are off. In a future release, after fees are switched on and thoroughly
    /// tested, this may be made mandatory.
    pub fee_contract: Option<Address>,

    /// Account that receives sequencing fees.
    ///
    /// This account in the Espresso fee ledger will always receive every fee paid in Espresso,
    /// regardless of whether or not their is a `fee_contract` deployed. Once deployed, the fee
    /// contract can decide what to do with tokens locked in this account in Espresso.
    pub fee_recipient: FeeAccount,

    /// Account that receives sequencing bids.
    pub bid_recipient: FeeAccount,
}

#[derive(Clone, Debug, Copy, PartialEq, Deserialize, Serialize, Eq, Hash)]
pub struct ResolvableChainConfig {
    pub(crate) chain_config: Either<ChainConfig, Commitment<ChainConfig>>,
}
