use std::collections::BTreeMap;

use std::sync::Arc;

use hotshot_types::HotShotConfig;
use sequencer_utils::ser::FromStringOrInteger;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::{
    v0::traits::StateCatchup, ChainConfig, GenesisHeader, L1BlockInfo, PubKey, Timestamp,
    ValidatedState,
};
use vbs::version::Version;

use super::l1::L1Client;

/// Represents the specific type of upgrade.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum UpgradeType {
    // Note: Wrapping this in a tuple variant causes deserialization to fail because
    // the 'chain_config' name is also provided in the TOML input.
    ChainConfig { chain_config: ChainConfig },
}

/// Represents an upgrade based on time (unix timestamp).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TimeBasedUpgrade {
    /// the earliest unix timestamp in which the node can propose an upgrade
    pub start_proposing_time: Timestamp,
    /// timestamp after which the node stops proposing an upgrade
    pub stop_proposing_time: Timestamp,
    /// The timestamp at which voting for the upgrade proposal starts
    pub start_voting_time: Option<Timestamp>,
    /// The timestamp at which voting for the upgrade proposal stops
    pub stop_voting_time: Option<Timestamp>,
}

/// Represents an upgrade based on view.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ViewBasedUpgrade {
    /// the earliest view in which the node can propose an upgrade
    pub start_proposing_view: u64,
    /// view after which the node stops proposing an upgrade
    pub stop_proposing_view: u64,
    /// The view at which voting for the upgrade proposal starts
    pub start_voting_view: Option<u64>,
    /// The view at which voting for the upgrade proposal stops
    pub stop_voting_view: Option<u64>,
}

/// Represents the specific type of upgrade.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UpgradeMode {
    /// Upgrade based on unix timestamp.
    Time(TimeBasedUpgrade),
    /// Upgrade based on view.
    View(ViewBasedUpgrade),
}

/// Represents a general upgrade with mode and type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Upgrade {
    /// The mode of the upgrade (time-based or view-based).
    pub mode: UpgradeMode,
    /// The type of the upgrade.
    pub upgrade_type: UpgradeType,
}

impl Upgrade {
    pub fn set_hotshot_config(&self, config: &mut HotShotConfig<PubKey>) {
        match &self.mode {
            UpgradeMode::View(v) => {
                config.start_proposing_view = v.start_proposing_view;
                config.stop_proposing_view = v.stop_proposing_view;
                config.start_voting_view = v.start_voting_view.unwrap_or(0);
                config.stop_voting_view = v.stop_voting_view.unwrap_or(u64::MAX);
                config.start_proposing_time = 0;
                config.stop_proposing_time = u64::MAX;
                config.start_voting_time = 0;
                config.stop_voting_time = u64::MAX;
            }
            UpgradeMode::Time(t) => {
                config.start_proposing_time = t.start_proposing_time.unix_timestamp();
                config.stop_proposing_time = t.stop_proposing_time.unix_timestamp();
                config.start_voting_time = t.start_voting_time.unwrap_or_default().unix_timestamp();
                config.stop_voting_time = t
                    .stop_voting_time
                    .unwrap_or(Timestamp::from_integer(u64::MAX).unwrap())
                    .unix_timestamp();
                config.start_proposing_view = 0;
                config.stop_proposing_view = u64::MAX;
                config.start_voting_view = 0;
                config.stop_voting_view = u64::MAX;
            }
        }
    }
}

/// Represents the immutable state of a node.
///
/// For mutable state, use `ValidatedState`.
#[derive(Debug, Clone)]
pub struct NodeState {
    pub node_id: u64,
    pub chain_config: ChainConfig,
    pub l1_client: L1Client,
    pub peers: Arc<dyn StateCatchup>,
    pub genesis_header: GenesisHeader,
    pub genesis_state: ValidatedState,
    pub l1_genesis: Option<L1BlockInfo>,

    /// Map containing all planned and executed upgrades.
    ///
    /// Currently, only one upgrade can be executed at a time.
    /// For multiple upgrades, the node needs to be restarted after each upgrade.
    ///
    /// This field serves as a record for planned and past upgrades,
    /// listed in the genesis TOML file. It will be very useful if multiple upgrades
    /// are supported in the future.
    pub upgrades: BTreeMap<Version, Upgrade>,
    /// Current version of the sequencer.
    ///
    /// This version is checked to determine if an upgrade is planned,
    /// and which version variant for versioned types  
    /// to use in functions such as genesis.
    /// (example: genesis returns V2 Header if version is 0.2)
    pub current_version: Version,
}
