use std::collections::BTreeMap;

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::{v0::traits::StateCatchup, ChainConfig, GenesisHeader, L1BlockInfo, ValidatedState};
use vbs::version::Version;

use super::l1::L1Client;

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

#[derive(Debug, Clone)]
pub struct NodeState {
    pub node_id: u64,
    pub chain_config: ChainConfig,
    pub l1_client: L1Client,
    pub peers: Arc<dyn StateCatchup>,
    pub genesis_header: GenesisHeader,
    pub genesis_state: ValidatedState,
    pub l1_genesis: Option<L1BlockInfo>,
    pub upgrades: BTreeMap<Version, Upgrade>,
    pub current_version: Version,
}
