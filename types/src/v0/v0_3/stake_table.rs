use std::collections::HashMap;

use crate::PubKey;
use alloy::primitives::map::HashSet;
use derive_more::derive::{From, Into};
use ethers::types::Address;
use hotshot::types::SignatureKey;
use hotshot_contract_adapter::stake_table::NodeInfoJf;
use hotshot_types::{light_client::StateVerKey, network::PeerConfigKeys, PeerConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub struct PermissionedStakeTableEntry(NodeInfoJf);

/// Stake table holding all staking information (DA and non-DA stakers)
#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub struct CombinedStakeTable(Vec<PeerConfigKeys<PubKey>>);

#[derive(Clone, Debug, From, Into)]
/// NewType to disambiguate DA Membership
pub struct DAMembers(pub Vec<PeerConfig<PubKey>>);

#[derive(Clone, Debug, From, Into)]
/// NewType to disambiguate StakeTable
pub struct StakeTable(pub Vec<PeerConfig<PubKey>>);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(bound(deserialize = ""))]
pub struct StakerConfig<KEY: SignatureKey> {
    pub account: Address,
    /// The peer's public key
    pub stake_table_key: KEY,
    /// the peer's state public key
    pub state_ver_key: StateVerKey,
    /// the peer's stake
    pub stake: u64,
    // commission
    pub commission: u16,
    pub delegators: HashMap<(Address, Address), u64>,
}

#[derive(serde::Serialize, serde::Deserialize, std::hash::Hash, Clone, Debug, PartialEq, Eq)]
#[serde(bound(deserialize = ""))]
pub struct Delegator {
    pub address: Address,
    pub validator: Address,
    pub stake: u64,
}
