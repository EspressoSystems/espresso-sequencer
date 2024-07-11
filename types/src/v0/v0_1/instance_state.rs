use std::collections::BTreeMap;

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::{v0::traits::StateCatchup, ChainConfig, GenesisHeader, L1BlockInfo, ValidatedState};
use vbs::version::Version;

use super::l1::L1Client;

/// Represents the specific type of upgrade.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum UpgradeType {
    // Note: Wrapping this in a tuple variant causes deserialization to fail because
    // the 'chain_config' name is also provided in the TOML input.
    ChainConfig { chain_config: ChainConfig },
}

/// Represents the  upgrade config including the type of upgrade and upgrade parameters for hotshot config.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Upgrade {
    /// The view at which the upgrade is proposed.
    ///
    /// Note: Voting for the proposal begins before the upgrade is formally proposed.
    /// In our implementation, `start_proposing_view` is set to `1`` for all upgrades,
    /// so if an upgrade is planned then the voting starts as soon as node is started.
    #[serde(rename = "view")]
    pub start_proposing_view: u64,

    /// The time window during which the upgrade can be proposed.
    ///
    /// This parameter is used for setting the `stop_propose_window_view`.
    /// `stop_proposing_view` is calculated as `start_proposing_view + propose_window`.
    pub propose_window: u64,

    /// The specific type of upgrade configuration.
    ///
    /// Currently, we only support chain configuration upgrades (`upgrade.chain_config` in genesis toml file).
    #[serde(flatten)]
    pub upgrade_type: UpgradeType,
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
    /// and which version variant for versioned types (e.g., build V2 header is version is 0,2) to use
    /// in functions such as genesis.
    pub current_version: Version,
}
