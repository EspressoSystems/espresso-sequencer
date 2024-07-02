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
}

#[derive(Clone, Debug, Copy, PartialEq, Deserialize, Serialize, Eq, Hash)]
pub struct ResolvableChainConfig {
    pub(crate) chain_config: Either<ChainConfig, Commitment<ChainConfig>>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_chainid_serde_json_as_decimal() {
        let id = ChainId::from(123);
        let serialized = serde_json::to_string(&id).unwrap();

        // The value is serialized as a decimal string.
        assert_eq!(serialized, "\"123\"");

        // Deserialization produces the original value
        let deserialized: ChainId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_chainid_serde_json_from_hex() {
        // For backwards compatibility, chain ID can also be deserialized from a 0x-prefixed hex
        // string.
        let id: ChainId = serde_json::from_str("\"0x123\"").unwrap();
        assert_eq!(id, ChainId::from(0x123));
    }

    #[test]
    fn test_chainid_serde_json_from_number() {
        // For convenience, chain ID can also be deserialized from a decimal number.
        let id: ChainId = serde_json::from_str("123").unwrap();
        assert_eq!(id, ChainId::from(123));
    }

    #[test]
    fn test_chainid_serde_bincode_unchanged() {
        // For non-human-readable formats, ChainId just serializes as the underlying U256.
        let n = U256::from(123);
        let id = ChainId(n);
        assert_eq!(
            bincode::serialize(&n).unwrap(),
            bincode::serialize(&id).unwrap(),
        );
    }

    #[test]
    fn test_block_size_serde_json_as_decimal() {
        let size = BlockSize::from(123);
        let serialized = serde_json::to_string(&size).unwrap();

        // The value is serialized as a decimal string.
        assert_eq!(serialized, "\"123\"");

        // Deserialization produces the original value
        let deserialized: BlockSize = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, size);
    }

    #[test]
    fn test_block_size_serde_json_from_number() {
        // For backwards compatibility, block size can also be deserialized from a decimal number.
        let size: BlockSize = serde_json::from_str("123").unwrap();
        assert_eq!(size, BlockSize::from(123));
    }

    #[test]
    fn test_block_size_serde_bincode_unchanged() {
        // For non-human-readable formats, BlockSize just serializes as the underlying u64.
        let n = 123u64;
        let size = BlockSize(n);
        assert_eq!(
            bincode::serialize(&n).unwrap(),
            bincode::serialize(&size).unwrap(),
        );
    }

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
