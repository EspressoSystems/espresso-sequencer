use crate::PubKey;
use derive_more::derive::From;
use hotshot_contract_adapter::stake_table::NodeInfoJf;
use hotshot_types::network::PeerConfigKeys;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub struct PermissionedStakeTableEntry(NodeInfoJf);

/// Stake table holding all staking information (DA and non-DA stakers)
#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub struct CombinedStakeTable(Vec<PeerConfigKeys<PubKey>>);
