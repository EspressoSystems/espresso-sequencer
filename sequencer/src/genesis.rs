use crate::{
    l1_client::L1BlockInfo,
    state::{FeeAccount, FeeAmount},
    ChainConfig,
};
use anyhow::Context;
use derive_more::{Display, From, Into};
use sequencer_utils::{impl_serde_from_string_or_integer, ser::FromStringOrInteger};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
};
use time::{format_description::well_known::Rfc3339 as TimestampFormat, OffsetDateTime};
use vbs::version::Version;

/// Initial configuration of an Espresso stake table.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct StakeTableConfig {
    pub capacity: u64,
}

/// An L1 block from which an Espresso chain should start syncing.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum L1Finalized {
    /// Complete block info.
    ///
    /// This allows a validator to specify the exact, existing L1 block to start syncing from. A
    /// validator that specifies a specific L1 block will not be able to reach consensus with a
    /// malicious validator that starts from a different L1 block.
    Block(L1BlockInfo),

    /// An L1 block number to sync from.
    ///
    /// This allows a validator to specify a future L1 block whose hash is not yet known, and start
    /// syncing only when a finalized block with the given number becomes available. The configured
    /// L1 client will be used to fetch the rest of the block info once available.
    Number { number: u64 },
}

#[derive(Hash, Copy, Clone, Debug, Display, PartialEq, Eq, From, Into)]
#[display(fmt = "{}", "_0.format(&TimestampFormat).unwrap()")]
pub struct Timestamp(OffsetDateTime);

impl_serde_from_string_or_integer!(Timestamp);

impl Default for Timestamp {
    fn default() -> Self {
        Self::from_integer(0).unwrap()
    }
}

impl Timestamp {
    pub fn unix_timestamp(&self) -> u64 {
        self.0.unix_timestamp() as u64
    }
}

impl FromStringOrInteger for Timestamp {
    type Binary = u64;
    type Integer = u64;

    fn from_binary(b: Self::Binary) -> anyhow::Result<Self> {
        Self::from_integer(b)
    }

    fn from_integer(i: Self::Integer) -> anyhow::Result<Self> {
        let unix = i.try_into().context("timestamp out of range")?;
        Ok(Self(
            OffsetDateTime::from_unix_timestamp(unix).context("invalid timestamp")?,
        ))
    }

    fn from_string(s: String) -> anyhow::Result<Self> {
        Ok(Self(
            OffsetDateTime::parse(&s, &TimestampFormat).context("invalid timestamp")?,
        ))
    }

    fn to_binary(&self) -> anyhow::Result<Self::Binary> {
        Ok(self.unix_timestamp())
    }

    fn to_string(&self) -> anyhow::Result<String> {
        Ok(format!("{self}"))
    }
}

/// Information about the genesis state which feeds into the genesis block header.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct GenesisHeader {
    pub timestamp: Timestamp,
}

/// Genesis of an Espresso chain.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Genesis {
    pub chain_config: ChainConfig,
    pub stake_table: StakeTableConfig,
    #[serde(default)]
    pub accounts: HashMap<FeeAccount, FeeAmount>,
    pub l1_finalized: Option<L1Finalized>,
    pub header: GenesisHeader,
    #[serde(rename = "upgrade", with = "upgrade_serialization")]
    #[serde(default)]
    pub upgrades: BTreeMap<Version, Upgrade>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum UpgradeType {
    // Note: Wrapping this in a tuple variant causes deserialization to fail because
    // the 'chain_config' name is also provided in the TOML input.
    ChainConfig { chain_config: ChainConfig },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Upgrade {
    pub view: u64,
    pub propose_window: u64,
    #[serde(flatten)]
    pub upgrade_type: UpgradeType,
}

mod upgrade_serialization {
    use crate::genesis::{Upgrade, UpgradeType};
    use serde::ser::SerializeSeq;
    use serde::{
        de::{SeqAccess, Visitor},
        Deserialize, Deserializer, Serializer,
    };
    use std::{collections::BTreeMap, fmt};
    use vbs::version::Version;

    pub fn serialize<S>(map: &BTreeMap<Version, Upgrade>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(map.len()))?;
        for (version, upgrade) in map {
            seq.serialize_element(&(
                version.to_string(),
                upgrade.view,
                upgrade.propose_window,
                upgrade.upgrade_type.clone(),
            ))?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BTreeMap<Version, Upgrade>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VecToHashMap;

        impl<'de> Visitor<'de> for VecToHashMap {
            type Value = BTreeMap<Version, Upgrade>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a vector of tuples (key-value pairs)")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<BTreeMap<Version, Upgrade>, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut map = BTreeMap::new();

                #[derive(Deserialize)]
                struct UpgradeFields {
                    version: String,
                    view: u64,
                    propose_window: u64,
                    #[serde(flatten)]
                    upgrade_type: UpgradeType,
                }

                while let Some(fields) = seq.next_element::<UpgradeFields>()? {
                    // add try_from in Version
                    let version: Vec<_> = fields.version.split('.').collect();

                    let version = Version {
                        major: version[0].parse().expect("invalid version"),
                        minor: version[1].parse().expect("invalid version"),
                    };

                    map.insert(
                        version,
                        Upgrade {
                            view: fields.view,
                            propose_window: fields.propose_window,
                            upgrade_type: fields.upgrade_type,
                        },
                    );
                }

                Ok(map)
            }
        }

        deserializer.deserialize_seq(VecToHashMap)
    }
}

impl Genesis {
    pub fn to_file(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let toml = toml::to_string_pretty(self)?;
        std::fs::write(path, toml.as_bytes())?;
        Ok(())
    }

    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let bytes = std::fs::read(path).context(format!("genesis file {}", path.display()))?;
        let text = std::str::from_utf8(&bytes).context("genesis file must be UTF-8")?;

        toml::from_str(text).context("malformed genesis file")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use ethers::prelude::{Address, H160, H256};
    use toml::toml;

    #[test]
    fn test_genesis_from_toml_with_optional_fields() {
        let toml = toml! {
            [stake_table]
            capacity = 10

            [chain_config]
            chain_id = 12345
            max_block_size = 30000
            base_fee = 1
            fee_recipient = "0x0000000000000000000000000000000000000000"
            fee_contract = "0x0000000000000000000000000000000000000000"

            [header]
            timestamp = 123456

            [accounts]
            "0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f" = 100000
            "0x0000000000000000000000000000000000000000" = 42

            [l1_finalized]
            number = 64
            timestamp = "0x123def"
            hash = "0x80f5dd11f2bdda2814cb1ad94ef30a47de02cf28ad68c89e104c00c4e51bb7a5"

            [[upgrade]]
            version = "1.0"
            view = 1
            propose_window = 10

            [upgrade.chain_config]
            chain_id = 12345
            max_block_size = 30000
            base_fee = 1
            fee_recipient = "0x0000000000000000000000000000000000000000"
            fee_contract = "0x0000000000000000000000000000000000000000"
        }
        .to_string();

        let genesis: Genesis = toml::from_str(&toml).unwrap_or_else(|err| panic!("{err:#}"));
        assert_eq!(genesis.stake_table, StakeTableConfig { capacity: 10 });
        assert_eq!(
            genesis.chain_config,
            ChainConfig {
                chain_id: 12345.into(),
                max_block_size: 30000.into(),
                base_fee: 1.into(),
                fee_recipient: FeeAccount::default(),
                bid_recipient: FeeAccount::default(),
                fee_contract: Some(Address::default())
            }
        );
        assert_eq!(
            genesis.header,
            GenesisHeader {
                timestamp: Timestamp::from_integer(123456).unwrap(),
            }
        );
        assert_eq!(
            genesis.accounts,
            [
                (
                    FeeAccount::from(H160([
                        0x23, 0x61, 0x8e, 0x81, 0xe3, 0xf5, 0xcd, 0xf7, 0xf5, 0x4c, 0x3d, 0x65,
                        0xf7, 0xfb, 0xc0, 0xab, 0xf5, 0xb2, 0x1e, 0x8f
                    ])),
                    100000.into()
                ),
                (FeeAccount::default(), 42.into())
            ]
            .into_iter()
            .collect::<HashMap<_, _>>()
        );
        assert_eq!(
            genesis.l1_finalized,
            Some(L1Finalized::Block(L1BlockInfo {
                number: 64,
                timestamp: 0x123def.into(),
                hash: H256([
                    0x80, 0xf5, 0xdd, 0x11, 0xf2, 0xbd, 0xda, 0x28, 0x14, 0xcb, 0x1a, 0xd9, 0x4e,
                    0xf3, 0x0a, 0x47, 0xde, 0x02, 0xcf, 0x28, 0xad, 0x68, 0xc8, 0x9e, 0x10, 0x4c,
                    0x00, 0xc4, 0xe5, 0x1b, 0xb7, 0xa5
                ])
            }))
        );
    }

    #[test]
    fn test_genesis_from_toml_without_optional_fields() {
        let toml = toml! {
            [stake_table]
            capacity = 10

            [chain_config]
            chain_id = 12345
            max_block_size = 30000
            base_fee = 1
            fee_recipient = "0x0000000000000000000000000000000000000000"

            [header]
            timestamp = 123456
        }
        .to_string();

        let genesis: Genesis = toml::from_str(&toml).unwrap_or_else(|err| panic!("{err:#}"));
        assert_eq!(genesis.stake_table, StakeTableConfig { capacity: 10 });
        assert_eq!(
            genesis.chain_config,
            ChainConfig {
                chain_id: 12345.into(),
                max_block_size: 30000.into(),
                base_fee: 1.into(),
                fee_recipient: FeeAccount::default(),
                bid_recipient: FeeAccount::default(),
                fee_contract: None,
            }
        );
        assert_eq!(
            genesis.header,
            GenesisHeader {
                timestamp: Timestamp::from_integer(123456).unwrap(),
            }
        );
        assert_eq!(genesis.accounts, HashMap::default());
        assert_eq!(genesis.l1_finalized, None);
    }

    #[test]
    fn test_genesis_l1_finalized_number_only() {
        let toml = toml! {
            [stake_table]
            capacity = 10

            [chain_config]
            chain_id = 12345
            max_block_size = 30000
            base_fee = 1
            fee_recipient = "0x0000000000000000000000000000000000000000"

            [header]
            timestamp = 123456

            [l1_finalized]
            number = 42
        }
        .to_string();

        let genesis: Genesis = toml::from_str(&toml).unwrap_or_else(|err| panic!("{err:#}"));
        assert_eq!(
            genesis.l1_finalized,
            Some(L1Finalized::Number { number: 42 })
        );
    }

    #[test]
    fn test_genesis_from_toml_units() {
        let toml = toml! {
            [stake_table]
            capacity = 10

            [chain_config]
            chain_id = 12345
            max_block_size = "30mb"
            base_fee = "1 gwei"
            fee_recipient = "0x0000000000000000000000000000000000000000"

            [header]
            timestamp = "2024-05-16T11:20:28-04:00"
        }
        .to_string();

        let genesis: Genesis = toml::from_str(&toml).unwrap_or_else(|err| panic!("{err:#}"));
        assert_eq!(genesis.stake_table, StakeTableConfig { capacity: 10 });
        assert_eq!(*genesis.chain_config.max_block_size, 30000000);
        assert_eq!(genesis.chain_config.base_fee, 1_000_000_000.into());
        assert_eq!(
            genesis.header,
            GenesisHeader {
                timestamp: Timestamp::from_integer(1715872828).unwrap(),
            }
        )
    }
}
