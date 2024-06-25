use crate::{v0_1::BlockSize, ChainConfig, ChainId, ResolvableChainConfig};
use bytesize::ByteSize;
use committable::{Commitment, Committable};
use derive_more::From;
use ethers::types::U256;
use itertools::Either;
use sequencer_utils::{
    impl_serde_from_string_or_integer, impl_to_fixed_bytes, ser::FromStringOrInteger,
};
use snafu::Snafu;
use std::str::FromStr;

impl_serde_from_string_or_integer!(ChainId);
impl_to_fixed_bytes!(ChainId, U256);

impl FromStringOrInteger for ChainId {
    type Binary = U256;
    type Integer = u64;

    fn from_binary(b: Self::Binary) -> anyhow::Result<Self> {
        Ok(Self(b))
    }

    fn from_integer(i: Self::Integer) -> anyhow::Result<Self> {
        Ok(i.into())
    }

    fn from_string(s: String) -> anyhow::Result<Self> {
        if s.starts_with("0x") {
            Ok(Self(U256::from_str(&s)?))
        } else {
            Ok(Self(U256::from_dec_str(&s)?))
        }
    }

    fn to_binary(&self) -> anyhow::Result<Self::Binary> {
        Ok(self.0)
    }

    fn to_string(&self) -> anyhow::Result<String> {
        Ok(format!("{self}"))
    }
}

impl From<u64> for ChainId {
    fn from(id: u64) -> Self {
        Self(id.into())
    }
}

impl_serde_from_string_or_integer!(BlockSize);

impl FromStringOrInteger for BlockSize {
    type Binary = u64;
    type Integer = u64;

    fn from_binary(b: Self::Binary) -> anyhow::Result<Self> {
        Ok(Self(b))
    }

    fn from_integer(i: Self::Integer) -> anyhow::Result<Self> {
        Ok(Self(i))
    }

    fn from_string(s: String) -> anyhow::Result<Self> {
        Ok(parse_size(&s)?.into())
    }

    fn to_binary(&self) -> anyhow::Result<Self::Binary> {
        Ok(self.0)
    }

    fn to_string(&self) -> anyhow::Result<String> {
        Ok(format!("{self}"))
    }
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            chain_id: U256::from(35353).into(), // arbitrarily chosen chain ID
            max_block_size: 10240.into(),
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
            .u64_field("max_block_size", *self.max_block_size)
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

#[derive(Clone, Debug, From, Snafu)]
pub struct ParseSizeError {
    msg: String,
}

pub fn parse_size(s: &str) -> Result<u64, ParseSizeError> {
    Ok(s.parse::<ByteSize>()?.0)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_chainid_serde_json_as_decimal() {
//         let id = ChainId::from(123);
//         let serialized = serde_json::to_string(&id).unwrap();

//         // The value is serialized as a decimal string.
//         assert_eq!(serialized, "\"123\"");

//         // Deserialization produces the original value
//         let deserialized: ChainId = serde_json::from_str(&serialized).unwrap();
//         assert_eq!(deserialized, id);
//     }

//     #[test]
//     fn test_chainid_serde_json_from_hex() {
//         // For backwards compatibility, chain ID can also be deserialized from a 0x-prefixed hex
//         // string.
//         let id: ChainId = serde_json::from_str("\"0x123\"").unwrap();
//         assert_eq!(id, ChainId::from(0x123));
//     }

//     #[test]
//     fn test_chainid_serde_json_from_number() {
//         // For convenience, chain ID can also be deserialized from a decimal number.
//         let id: ChainId = serde_json::from_str("123").unwrap();
//         assert_eq!(id, ChainId::from(123));
//     }

//     #[test]
//     fn test_chainid_serde_bincode_unchanged() {
//         // For non-human-readable formats, ChainId just serializes as the underlying U256.
//         let n = U256::from(123);
//         let id = ChainId(n);
//         assert_eq!(
//             bincode::serialize(&n).unwrap(),
//             bincode::serialize(&id).unwrap(),
//         );
//     }

//     #[test]
//     fn test_block_size_serde_json_as_decimal() {
//         let size = BlockSize::from(123);
//         let serialized = serde_json::to_string(&size).unwrap();

//         // The value is serialized as a decimal string.
//         assert_eq!(serialized, "\"123\"");

//         // Deserialization produces the original value
//         let deserialized: BlockSize = serde_json::from_str(&serialized).unwrap();
//         assert_eq!(deserialized, size);
//     }

//     #[test]
//     fn test_block_size_serde_json_from_number() {
//         // For backwards compatibility, block size can also be deserialized from a decimal number.
//         let size: BlockSize = serde_json::from_str("123").unwrap();
//         assert_eq!(size, BlockSize::from(123));
//     }

//     #[test]
//     fn test_block_size_serde_bincode_unchanged() {
//         // For non-human-readable formats, BlockSize just serializes as the underlying u64.
//         let n = 123u64;
//         let size = BlockSize(n);
//         assert_eq!(
//             bincode::serialize(&n).unwrap(),
//             bincode::serialize(&size).unwrap(),
//         );
//     }

//     #[test]
//     fn test_chain_config_equality() {
//         let chain_config = ChainConfig::default();
//         assert_eq!(chain_config, chain_config.clone());
//         let ChainConfig {
//             chain_id,
//             max_block_size,
//             ..
//         } = chain_config;
//         let other_config = ChainConfig {
//             chain_id,
//             max_block_size,
//             base_fee: 1.into(),
//             ..Default::default()
//         };
//         assert!(chain_config != other_config);
//     }

//     #[test]
//     fn test_resolve_chain_config() {
//         let chain_config = ChainConfig::default();
//         let resolveable: ResolvableChainConfig = chain_config.into();
//         assert_eq!(chain_config, resolveable.resolve().unwrap());
//     }
// }
