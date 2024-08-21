use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
};

use anyhow::Context;
use espresso_types::{
    v0_3::ChainConfig, FeeAccount, FeeAmount, GenesisHeader, L1BlockInfo, Upgrade, UpgradeType,
};
use serde::{Deserialize, Serialize};
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

impl Genesis {
    pub fn max_base_fee(&self) -> FeeAmount {
        let mut base_fee = self.chain_config.base_fee;

        let upgrades: Vec<&Upgrade> = self.upgrades.values().collect();

        for upgrade in upgrades {
            if let UpgradeType::Fee { chain_config } = upgrade.upgrade_type {
                base_fee = std::cmp::max(chain_config.base_fee, base_fee);
            }
        }

        base_fee
    }
}

mod upgrade_serialization {

    use std::{collections::BTreeMap, fmt};

    use espresso_types::{
        v0_1::{TimeBasedUpgrade, UpgradeMode, ViewBasedUpgrade},
        Upgrade, UpgradeType,
    };
    use serde::{
        de::{self, SeqAccess, Visitor},
        ser::SerializeSeq,
        Deserialize, Deserializer, Serialize, Serializer,
    };
    use vbs::version::Version;

    pub fn serialize<S>(map: &BTreeMap<Version, Upgrade>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Fields {
            pub version: String,
            #[serde(flatten)]
            pub mode: UpgradeMode,
            #[serde(flatten)]
            pub upgrade_type: UpgradeType,
        }

        let mut seq = serializer.serialize_seq(Some(map.len()))?;
        for (version, upgrade) in map {
            seq.serialize_element(&Fields {
                version: version.to_string(),
                mode: upgrade.mode.clone(),
                upgrade_type: upgrade.upgrade_type.clone(),
            })?
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BTreeMap<Version, Upgrade>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VecToHashMap;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Fields {
            pub version: String,
            // If both `time_based` and `view_based` fields are provided
            // and we use an enum for deserialization, then one of the variant fields will be ignored.
            // We want to raise an error in such a case to avoid ambiguity
            #[serde(flatten)]
            pub time_based: Option<TimeBasedUpgrade>,
            #[serde(flatten)]
            pub view_based: Option<ViewBasedUpgrade>,
            #[serde(flatten)]
            pub upgrade_type: UpgradeType,
        }

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

                while let Some(fields) = seq.next_element::<Fields>()? {
                    // add try_from in Version
                    let version: Vec<_> = fields.version.split('.').collect();

                    let version = Version {
                        major: version[0].parse().expect("invalid version"),
                        minor: version[1].parse().expect("invalid version"),
                    };

                    match (fields.time_based, fields.view_based) {
                        (Some(_), Some(_)) => {
                            return Err(de::Error::custom(
                                "both view and time mode parameters are set",
                            ))
                        }
                        (None, None) => {
                            return Err(de::Error::custom(
                                "no view or time mode parameters provided",
                            ))
                        }
                        (None, Some(v)) => {
                            if v.start_proposing_view > v.stop_proposing_view {
                                return Err(de::Error::custom(
                                    "stop_proposing_view is less than start_proposing_view",
                                ));
                            }

                            map.insert(
                                version,
                                Upgrade {
                                    mode: UpgradeMode::View(v),
                                    upgrade_type: fields.upgrade_type,
                                },
                            );
                        }
                        (Some(t), None) => {
                            if t.start_proposing_time.unix_timestamp()
                                > t.stop_proposing_time.unix_timestamp()
                            {
                                return Err(de::Error::custom(
                                    "stop_proposing_time is less than start_proposing_time",
                                ));
                            }

                            map.insert(
                                version,
                                Upgrade {
                                    mode: UpgradeMode::Time(t),
                                    upgrade_type: fields.upgrade_type.clone(),
                                },
                            );
                        }
                    }
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
    use espresso_types::{
        L1BlockInfo, TimeBasedUpgrade, Timestamp, UpgradeMode, UpgradeType, ViewBasedUpgrade,
    };
    use ethers::prelude::{Address, H160, H256};
    use sequencer_utils::ser::FromStringOrInteger;
    use toml::toml;

    use super::*;

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
                fee_contract: Some(Address::default()),
                bid_recipient: None
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
                bid_recipient: None,
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

    #[test]
    fn test_genesis_toml_fee_upgrade_view_mode() {
        // without optional fields
        // with view settings
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
            version = "0.2"
            start_proposing_view = 1
            stop_proposing_view = 15

            [upgrade.fee]

            [upgrade.fee.chain_config]
            chain_id = 12345
            max_block_size = 30000
            base_fee = 1
            fee_recipient = "0x0000000000000000000000000000000000000000"
            fee_contract = "0x0000000000000000000000000000000000000000"
        }
        .to_string();

        let genesis: Genesis = toml::from_str(&toml).unwrap_or_else(|err| panic!("{err:#}"));

        let (version, genesis_upgrade) = genesis.upgrades.last_key_value().unwrap();

        assert_eq!(*version, Version { major: 0, minor: 2 });

        let upgrade = Upgrade {
            mode: UpgradeMode::View(ViewBasedUpgrade {
                start_voting_view: None,
                stop_voting_view: None,
                start_proposing_view: 1,
                stop_proposing_view: 15,
            }),
            upgrade_type: UpgradeType::Fee {
                chain_config: genesis.chain_config,
            },
        };

        assert_eq!(*genesis_upgrade, upgrade);
    }

    #[test]
    fn test_genesis_toml_fee_upgrade_time_mode() {
        // without optional fields
        // with time settings
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
            version = "0.2"
            start_proposing_time = "2024-01-01T00:00:00Z"
            stop_proposing_time = "2024-01-02T00:00:00Z"

            [upgrade.fee]

            [upgrade.fee.chain_config]
            chain_id = 12345
            max_block_size = 30000
            base_fee = 1
            fee_recipient = "0x0000000000000000000000000000000000000000"
            fee_contract = "0x0000000000000000000000000000000000000000"
        }
        .to_string();

        let genesis: Genesis = toml::from_str(&toml).unwrap_or_else(|err| panic!("{err:#}"));

        let (version, genesis_upgrade) = genesis.upgrades.last_key_value().unwrap();

        assert_eq!(*version, Version { major: 0, minor: 2 });

        let upgrade = Upgrade {
            mode: UpgradeMode::Time(TimeBasedUpgrade {
                start_voting_time: None,
                stop_voting_time: None,
                start_proposing_time: Timestamp::from_string("2024-01-01T00:00:00Z".to_string())
                    .unwrap(),
                stop_proposing_time: Timestamp::from_string("2024-01-02T00:00:00Z".to_string())
                    .unwrap(),
            }),
            upgrade_type: UpgradeType::Fee {
                chain_config: genesis.chain_config,
            },
        };

        assert_eq!(*genesis_upgrade, upgrade);
    }

    #[test]
    fn test_genesis_toml_fee_upgrade_view_and_time_mode() {
        // set both time and view parameters
        // this should err
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
            version = "0.2"
            start_proposing_view = 1
            stop_proposing_view = 10
            start_proposing_time = 1
            stop_proposing_time = 10

            [upgrade.fee]

            [upgrade.fee.chain_config]
            chain_id = 12345
            max_block_size = 30000
            base_fee = 1
            fee_recipient = "0x0000000000000000000000000000000000000000"
            fee_contract = "0x0000000000000000000000000000000000000000"
        }
        .to_string();

        toml::from_str::<Genesis>(&toml).unwrap_err();
    }

    #[test]
    fn test_marketplace_upgrade_toml() {
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
            version = "0.3"
            start_proposing_view = 1
            stop_proposing_view = 10

            [upgrade.marketplace]
            [upgrade.marketplace.chain_config]
            chain_id = 12345
            max_block_size = 30000
            base_fee = 1
            fee_recipient = "0x0000000000000000000000000000000000000000"
            bid_recipient = "0x0000000000000000000000000000000000000000"
            fee_contract = "0x0000000000000000000000000000000000000000"

        }
        .to_string();

        toml::from_str::<Genesis>(&toml).unwrap();
    }

    #[test]
    fn test_marketplace_and_fee_upgrade_toml() {
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
            version = "0.3"
            start_proposing_view = 1
            stop_proposing_view = 10

            [upgrade.marketplace]
            [upgrade.marketplace.chain_config]
            chain_id = 12345
            max_block_size = 30000
            base_fee = 1
            fee_recipient = "0x0000000000000000000000000000000000000000"
            bid_recipient = "0x0000000000000000000000000000000000000000"
            fee_contract = "0x0000000000000000000000000000000000000000"

            [[upgrade]]
            version = "0.2"
            start_proposing_view = 1
            stop_proposing_view = 15

            [upgrade.fee]

            [upgrade.fee.chain_config]
            chain_id = 12345
            max_block_size = 30000
            base_fee = 1
            fee_recipient = "0x0000000000000000000000000000000000000000"
            fee_contract = "0x0000000000000000000000000000000000000000"
        }
        .to_string();

        toml::from_str::<Genesis>(&toml).unwrap();
    }
}
