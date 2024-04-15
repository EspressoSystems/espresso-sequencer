use crate::state::FeeAmount;
use commit::{Commitment, Committable};
use derive_more::{From, Into};
use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Default, Hash, Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, From, Into)]
pub struct ChainId(U256);

impl From<u16> for ChainId {
    fn from(id: u16) -> Self {
        Self(id.into())
    }
}

impl ChainId {
    // TODO: this duplicate code, consider a macro for U256 newtypes
    pub(crate) fn to_fixed_bytes(self) -> [u8; 32] {
        let mut bytes = [0u8; core::mem::size_of::<U256>()];
        self.0.to_little_endian(&mut bytes);
        bytes
    }
}

/// Global variables for an Espresso blockchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChainConfig {
    /// Espresso chain ID
    chain_id: ChainId,
    /// Maximum size in bytes of a block
    max_block_size: u64,
    /// Minimum fee in WEI per byte of payload
    base_fee: FeeAmount,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self::new(
            U256::from(35353), // arbitrarily chosen chain ID
            10240,             // 10 kB max_block_size
            0,                 // no fees
        )
    }
}

impl ChainConfig {
    pub fn new(
        chain_id: impl Into<ChainId>,
        max_block_size: u64,
        base_fee: impl Into<FeeAmount>,
    ) -> Self {
        Self {
            chain_id: chain_id.into(),
            max_block_size,
            base_fee: base_fee.into(),
        }
    }
}

impl Committable for ChainConfig {
    fn tag() -> String {
        "CHAIN_CONFIG".to_string()
    }

    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new(&Self::tag())
            .fixed_size_field("chain_id", &self.chain_id.to_fixed_bytes())
            .u64_field("max_block_size", self.max_block_size)
            .fixed_size_field("base_fee", &self.base_fee.to_fixed_bytes())
            .finalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_config_equality() {
        let chain_config = ChainConfig::default();
        assert_eq!(chain_config, chain_config.clone());
        let ChainConfig {
            chain_id,
            max_block_size,
            ..
        } = chain_config;
        let other_config = ChainConfig::new(chain_id, max_block_size, 1);
        assert!(chain_config != other_config);
    }
}
