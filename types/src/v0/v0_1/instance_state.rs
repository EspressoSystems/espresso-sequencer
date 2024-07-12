use std::collections::BTreeMap;

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::{v0::traits::StateCatchup, ChainConfig, GenesisHeader, L1BlockInfo, ValidatedState};
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

/// Represents the  upgrade config including the type of upgrade and upgrade parameters for hotshot config.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Upgrade {
    pub start_voting_time: Option<u64>,
    pub stop_voting_time: Option<u64>,
    pub start_proposing_time: u64,
    pub stop_proposing_time: u64,
    pub start_voting_view: Option<u64>,
    pub stop_voting_view: Option<u64>,
    pub start_proposing_view: u64,
    pub stop_proposing_view: u64,
    // View or time based
    pub mode: UpgradeMode,
    /// The specific type of upgrade configuration.
    ///
    /// Currently, we only support chain configuration upgrades (`upgrade.chain_config` in genesis toml file).
    #[serde(flatten)]
    pub upgrade_type: UpgradeType,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum UpgradeMode {
    View,
    Time,
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
