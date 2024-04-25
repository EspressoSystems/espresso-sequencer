use crate::state::FeeAmount;
use committable::{Commitment, Committable};
use derive_more::{From, Into};
use ethers::types::U256;
use itertools::Either;
use sequencer_utils::impl_to_fixed_bytes;
use serde::{Deserialize, Serialize};

#[derive(Default, Hash, Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, From, Into)]
pub struct ChainId(U256);

impl_to_fixed_bytes!(ChainId, U256);

impl From<u16> for ChainId {
    fn from(id: u16) -> Self {
        Self(id.into())
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

    pub fn chain_id(&self) -> ChainId {
        self.chain_id
    }

    pub fn max_block_size(&self) -> u64 {
        self.max_block_size
    }

    pub fn base_fee(&self) -> FeeAmount {
        self.base_fee
    }
}

impl Committable for ChainConfig {
    fn tag() -> String {
        "CHAIN_CONFIG".to_string()
    }

    fn commit(&self) -> Commitment<Self> {
        committable::RawCommitmentBuilder::new(&Self::tag())
            .fixed_size_field("chain_id", &self.chain_id.to_fixed_bytes())
            .u64_field("max_block_size", self.max_block_size)
            .fixed_size_field("base_fee", &self.base_fee.to_fixed_bytes())
            .finalize()
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Deserialize, Serialize, Eq, Hash)]
pub struct ResolvableChainConfig {
    chain_config: Either<ChainConfig, Commitment<ChainConfig>>,
}

impl ResolvableChainConfig {
    pub fn commit(&self) -> Commitment<ChainConfig> {
        match self.chain_config {
            Either::Left(config) => config.commit(),
            Either::Right(commitment) => commitment,
        }
    }
    pub fn resolve(self) -> Option<ChainConfig> {
        match self.chain_config {
            Either::Left(config) => Some(config),
            Either::Right(_) => None,
        }
    }
}

impl From<Commitment<ChainConfig>> for ResolvableChainConfig {
    fn from(value: Commitment<ChainConfig>) -> Self {
        Self {
            chain_config: Either::Right(value),
        }
    }
}

impl From<ChainConfig> for ResolvableChainConfig {
    fn from(value: ChainConfig) -> Self {
        Self {
            chain_config: Either::Left(value),
        }
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

    #[test]
    fn test_resolve_chain_config() {
        let chain_config = ChainConfig::default();
        let resolveable: ResolvableChainConfig = chain_config.into();
        assert_eq!(chain_config, resolveable.resolve().unwrap());
    }
}
