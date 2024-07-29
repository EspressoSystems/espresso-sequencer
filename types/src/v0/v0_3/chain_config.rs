use crate::{v0_1, BlockSize, ChainId, FeeAccount, FeeAmount};
use anyhow::Chain;
use committable::{Commitment, Committable};
use ethers::types::{Address, U256};
use itertools::Either;
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::{Map, Value};
use std::{fmt, str::FromStr};

/// Global variables for an Espresso blockchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Hash)]
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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    /// Account that receives sequencing bids.
    pub bid_recipient: Option<FeeAccount>,
}

// impl Serialize for ChainConfig {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut len = 5;
//         if self.bid_recipient.is_some() {
//             len = 6;
//         }

//         let mut state = serializer.serialize_struct("ChainConfig", len)?;
//         state.serialize_field("chain_id", &self.chain_id)?;
//         state.serialize_field("max_block_size", &self.max_block_size)?;
//         state.serialize_field("base_fee", &self.base_fee)?;
//         state.serialize_field("fee_contract", &self.fee_contract)?;
//         state.serialize_field("fee_recipient", &self.fee_recipient)?;

//         if self.bid_recipient.is_some() {
//             state.serialize_field("bid_recipient", &self.bid_recipient)?;
//         }

//         state.end()
//     }
// }

impl<'de> Deserialize<'de> for ChainConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChainConfigVisitor;

        impl<'de> Visitor<'de> for ChainConfigVisitor {
            type Value = ChainConfig;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ChainConfig")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let chain_config = ChainConfig {
                    chain_id: seq
                        .next_element()?
                        .ok_or_else(|| de::Error::missing_field("chain_id"))?,
                    max_block_size: seq
                        .next_element()?
                        .ok_or_else(|| de::Error::missing_field("max_block_size"))?,
                    base_fee: seq
                        .next_element()?
                        .ok_or_else(|| de::Error::missing_field("base_fee"))?,
                    fee_contract: seq
                        .next_element()?
                        .ok_or_else(|| de::Error::missing_field("fee_contract"))?,
                    fee_recipient: seq
                        .next_element()?
                        .ok_or_else(|| de::Error::missing_field("fee_recipient"))?,
                    bid_recipient: if let Ok(val) = seq.next_element() {
                        val.ok_or_else(|| de::Error::custom("failed to deserialize bid_recipient"))?
                    } else {
                        None
                    },
                };

                Ok(chain_config)
            }

            fn visit_map<V>(self, mut map: V) -> Result<ChainConfig, V::Error>
            where
                V: MapAccess<'de>,
            {
                // insert all the fields in the serde_map as the map may have out of order fields.
                let mut serde_map: Map<String, Value> = Map::new();

                while let Some(key) = map.next_key::<String>()? {
                    serde_map.insert(key.trim().to_owned(), map.next_value()?);
                }

                let bid_recipient = if let Some(val) = serde_map.get("bid_recipient") {
                    serde_json::from_value(val.clone()).unwrap()
                } else {
                    None
                };

                let chain_config = ChainConfig {
                    chain_id: serde_json::from_value(
                        serde_map
                            .get("chain_id")
                            .ok_or_else(|| de::Error::missing_field("chain_id"))?
                            .clone(),
                    )
                    .map_err(de::Error::custom)?,
                    max_block_size: serde_json::from_value(
                        serde_map
                            .get("max_block_size")
                            .ok_or_else(|| de::Error::missing_field("max_block_size"))?
                            .clone(),
                    )
                    .map_err(de::Error::custom)?,
                    base_fee: serde_json::from_value(
                        serde_map
                            .get("base_fee")
                            .ok_or_else(|| de::Error::missing_field("base_fee"))?
                            .clone(),
                    )
                    .map_err(de::Error::custom)?,
                    fee_contract: serde_json::from_value(
                        serde_map
                            .get("fee_contract")
                            .ok_or_else(|| de::Error::missing_field("fee_contract"))?
                            .clone(),
                    )
                    .map_err(de::Error::custom)?,
                    fee_recipient: serde_json::from_value(
                        serde_map
                            .get("fee_recipient")
                            .ok_or_else(|| de::Error::missing_field("fee_recipient"))?
                            .clone(),
                    )
                    .map_err(de::Error::custom)?,
                    bid_recipient,
                };

                Ok(chain_config)
            }
        }

        let fields: &[&str] = &[
            "chain_id",
            "max_block_size",
            "base_fee",
            "fee_contract",
            "fee_recipient",
            "bid_recipient",
        ];

        deserializer.deserialize_struct("ChainConfig", fields, ChainConfigVisitor)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Deserialize, Serialize, Eq, Hash)]
/// A commitment to a ChainConfig or a full ChainConfig.
pub struct ResolvableChainConfig {
    pub(crate) chain_config: Either<ChainConfig, Commitment<ChainConfig>>,
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

        // With `ChainConfig` upgrades we want commitments w/out
        // fields added >= v0_3 to have the same commitment as <= v0_3
        // commitment. Therefore `None` values are simply ignored.
        let comm = if let Some(bid_recipient) = self.bid_recipient {
            comm.fixed_size_field("bid_recipient", &bid_recipient.to_fixed_bytes())
        } else {
            comm
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

impl From<&v0_1::ResolvableChainConfig> for ResolvableChainConfig {
    fn from(
        &v0_1::ResolvableChainConfig { chain_config }: &v0_1::ResolvableChainConfig,
    ) -> ResolvableChainConfig {
        match chain_config {
            Either::Left(chain_config) => ResolvableChainConfig {
                chain_config: Either::Left(ChainConfig::from(chain_config)),
            },
            // TODO does this work? is there a better way?
            Either::Right(c) => ResolvableChainConfig {
                chain_config: Either::Right(Commitment::from_str(&c.to_string()).unwrap()),
            },
        }
    }
}

impl From<ChainConfig> for v0_1::ResolvableChainConfig {
    fn from(cf: ChainConfig) -> Self {
        let v01_chain_config = v0_1::ChainConfig::from(cf);

        Self {
            chain_config: Either::Left(v01_chain_config),
        }
    }
}

// impl From<ChainConfig> for v0_1::ChainConfig {
//     fn from(chain_config: ChainConfig) -> v0_1::ChainConfig {
//         let ChainConfig {
//             chain_id,
//             max_block_size,
//             base_fee,
//             fee_contract,
//             fee_recipient,
//             ..
//         } = chain_config;

//         v0_1::ChainConfig {
//             chain_id,
//             max_block_size,
//             base_fee,
//             fee_contract,
//             fee_recipient,
//         }
//     }
// }

impl From<v0_1::ChainConfig> for ChainConfig {
    fn from(chain_config: v0_1::ChainConfig) -> ChainConfig {
        let v0_1::ChainConfig {
            chain_id,
            max_block_size,
            base_fee,
            fee_contract,
            fee_recipient,
            ..
        } = chain_config;

        ChainConfig {
            chain_id,
            max_block_size,
            base_fee,
            fee_contract,
            fee_recipient,
            bid_recipient: None,
        }
    }
}

impl From<ChainConfig> for v0_1::ChainConfig {
    fn from(chain_config: ChainConfig) -> v0_1::ChainConfig {
        let ChainConfig {
            chain_id,
            max_block_size,
            base_fee,
            fee_contract,
            fee_recipient,
            ..
        } = chain_config;

        v0_1::ChainConfig {
            chain_id,
            max_block_size,
            base_fee,
            fee_contract,
            fee_recipient,
        }
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
            bid_recipient: Default::default(),
        }
    }
}
