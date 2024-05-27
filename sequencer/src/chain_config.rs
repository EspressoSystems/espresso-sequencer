use crate::{
    options::parse_size,
    state::{FeeAccount, FeeAmount},
};
use clap::Args;
use committable::{Commitment, Committable};
use derive_more::{From, Into};
use ethers::types::{Address, U256};
use itertools::Either;
use sequencer_utils::{deserialize_from_decimal, impl_to_fixed_bytes, serialize_as_decimal};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Hash, Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, From, Into)]
pub struct ChainId(
    #[serde(
        serialize_with = "serialize_as_decimal",
        deserialize_with = "deserialize_from_decimal"
    )]
    U256,
);

impl_to_fixed_bytes!(ChainId, U256);

impl From<u64> for ChainId {
    fn from(id: u64) -> Self {
        Self(id.into())
    }
}

impl FromStr for ChainId {
    type Err = <u64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(u64::from_str(s)?.into())
    }
}

/// Global variables for an Espresso blockchain.
#[derive(Args, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChainConfig {
    /// Espresso chain ID
    #[clap(long, env = "ESPRESSO_SEQUENCER_CHAIN_ID", default_value = "0")]
    pub chain_id: ChainId,
    /// Maximum size in bytes of a block
    #[clap(long, env = "ESPRESSO_SEQUENCER_MAX_BLOCK_SIZE", default_value = "30mb", value_parser = parse_size)]
    pub max_block_size: u64,
    /// Minimum fee in WEI per byte of payload
    #[clap(long, env = "ESPRESSO_SEQUENCER_BASE_FEE", default_value = "0")]
    pub base_fee: FeeAmount,
    /// Fee contract address on L1.
    ///
    /// This is optional so that fees can easily be toggled on/off, with no need to deploy a
    /// contract when they are off. In a future release, after fees are switched on and thoroughly
    /// tested, this may be made mandatory.
    #[clap(long, env = "ESPRESSO_SEQUENCER_FEE_CONTRACT_PROXY_ADDRESS")]
    pub fee_contract: Option<Address>,
    /// Account that receives sequencing fees.
    ///
    /// This account in the Espresso fee ledger will always receive every fee paid in Espresso,
    /// regardless of whether or not their is a `fee_contract` deployed. Once deployed, the fee
    /// contract can decide what to do with tokens locked in this account in Espresso.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_FEE_RECIPIENT",
        default_value = "0x0000000000000000000000000000000000000000"
    )]
    pub fee_recipient: FeeAccount,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            chain_id: U256::from(35353).into(), // arbitrarily chosen chain ID
            max_block_size: 10240,
            base_fee: 0.into(),
            fee_contract: None,
            fee_recipient: Default::default(),
        }
    }
}

impl Committable for ChainConfig {
    fn tag() -> String {
        "CHAIN_CONFIG".to_string()
    }

    fn commit(&self) -> Commitment<Self> {
        let comm = committable::RawCommitmentBuilder::new(&Self::tag())
            .fixed_size_field("chain_id", &self.chain_id.to_fixed_bytes())
            .u64_field("max_block_size", self.max_block_size)
            .fixed_size_field("base_fee", &self.base_fee.to_fixed_bytes())
            .fixed_size_field("fee_recipient", &self.fee_recipient.to_fixed_bytes());
        let comm = if let Some(addr) = self.fee_contract {
            comm.u64_field("fee_contract", 1).fixed_size_bytes(&addr.0)
        } else {
            comm.u64_field("fee_contract", 0)
        };
        comm.finalize()
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
        let other_config = ChainConfig {
            chain_id,
            max_block_size,
            base_fee: 1.into(),
            ..Default::default()
        };
        assert!(chain_config != other_config);
    }

    #[test]
    fn test_resolve_chain_config() {
        let chain_config = ChainConfig::default();
        let resolveable: ResolvableChainConfig = chain_config.into();
        assert_eq!(chain_config, resolveable.resolve().unwrap());
    }
}
