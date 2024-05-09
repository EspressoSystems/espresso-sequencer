use crate::{
    options::parse_size,
    state::{FeeAccount, FeeAmount},
};
use anyhow::{bail, Context};
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

impl ChainId {
    pub fn from_toml(toml: &toml::Value) -> anyhow::Result<Self> {
        if let Some(s) = toml.as_str() {
            if s.starts_with("0x") {
                Ok(Self(U256::from_str(s)?))
            } else {
                Ok(Self(U256::from_dec_str(s)?))
            }
        } else if let Some(n) = toml.as_integer() {
            Ok(u64::try_from(n)
                .context("must be an unsigned integer")?
                .into())
        } else {
            bail!("must be an integer or an integral string");
        }
    }
}

/// Global variables for an Espresso blockchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChainConfig {
    /// Espresso chain ID
    pub chain_id: ChainId,

    /// Maximum size in bytes of a block
    pub max_block_size: u64,

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

impl ChainConfig {
    pub fn from_toml(toml: &toml::Value) -> anyhow::Result<Self> {
        let cfg = toml.as_table().context("must be table")?;
        let chain_id = ChainId::from_toml(cfg.get("chain_id").context("missing chain_id")?)
            .context("invalid chain ID")?;
        let max_block_size = match cfg
            .get("max_block_size")
            .context("missing max_block_size")?
        {
            toml::Value::String(s) => parse_size(s).context("invalid max block size")?,
            toml::Value::Integer(n) => (*n)
                .try_into()
                .context("max_block_size must be an unsigned integer")?,
            _ => bail!("max_block_size must be an integer or an integral string"),
        };
        let base_fee = FeeAmount::from_toml(cfg.get("base_fee").context("missing base_fee")?)
            .context("invalid base fee")?;
        let fee_contract = match cfg.get("fee_contract") {
            Some(toml::Value::String(s)) => {
                Some(s.parse().context("invalid fee_contract address")?)
            }
            Some(_) => bail!("fee_contract must be an address string"),
            None => None,
        };
        let fee_recipient = cfg
            .get("fee_recipient")
            .context("missing fee_recipient")?
            .as_str()
            .context("fee_recipient must be an address string")?
            .parse()
            .context("invalid fee_recipient")?;
        Ok(Self {
            chain_id,
            max_block_size,
            base_fee,
            fee_contract,
            fee_recipient,
        })
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
