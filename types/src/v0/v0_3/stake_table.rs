use crate::PubKey;
use derive_more::derive::{From, Into};
use hotshot_contract_adapter::stake_table::NodeInfoJf;
use hotshot_types::{network::PeerConfigKeys, stake_table::StakeTableEntry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub struct PermissionedStakeTableEntry(NodeInfoJf);

/// Stake table holding all staking information (DA and non-DA stakers)
#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub struct CombinedStakeTable(Vec<PeerConfigKeys<PubKey>>);

// NewTypes for two types of stake tables to avoid confusion
#[derive(Clone, Debug, From, Into)]
pub struct DAStakeTable(pub Vec<StakeTableEntry<PubKey>>);

#[derive(Clone, Debug, From, Into)]
pub struct QuorumStakeTable(pub Vec<StakeTableEntry<PubKey>>);

#[derive(Clone, Debug)]
pub struct StakeTables {
    pub quorum: QuorumStakeTable,
    pub da: DAStakeTable,
}
