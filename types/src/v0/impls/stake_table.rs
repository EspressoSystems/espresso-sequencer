use super::{
    v0_3::{DAStakeTable, QuorumStakeTable, StakeTables},
    L1Client, NodeState, PubKey, SeqTypes,
};

use async_lock::RwLock;
use contract_bindings::permissioned_stake_table::StakersUpdatedFilter;
use ethers::types::U256;
use hotshot::types::SignatureKey as _;
use hotshot_contract_adapter::stake_table::NodeInfoJf;
use hotshot_types::{
    data::EpochNumber,
    stake_table::StakeTableEntry,
    traits::{
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
        signature_key::StakeTableEntryType,
    },
    PeerConfig,
};
use itertools::Itertools;
use std::{
    cmp::max,
    collections::{BTreeSet, HashMap},
    num::NonZeroU64,
    str::FromStr,
    sync::Arc,
};
use thiserror::Error;

use url::Url;

type Epoch = <SeqTypes as NodeType>::Epoch;

impl StakeTables {
    pub fn new(quorum: QuorumStakeTable, da: DAStakeTable) -> Self {
        Self { quorum, da }
    }

    /// Create the consensus and DA stake tables from L1 events
    ///
    /// This is a pure function, to make it easily testable.
    ///
    /// We expect have at most a few hundred EVM events in the
    /// PermissionedStakeTable contract over the liftetime of the contract so it
    /// should not significantly affect performance to fetch all events and
    /// perform the computation in this functions once per epoch.
    pub fn from_l1_events(updates: Vec<StakersUpdatedFilter>) -> Self {
        let changes_per_node = updates
            .into_iter()
            .flat_map(|event| {
                event
                    .removed
                    .into_iter()
                    .map(|node_info| StakeTableDelta::remove(node_info.into()))
                    .chain(
                        event
                            .added
                            .into_iter()
                            .map(|node_info| StakeTableDelta::add(node_info.into())),
                    )
            })
            .group_by(|delta| delta.node_info.stake_table_key);

        // If the last event for a stakers is `Added` the staker is currently
        // staking, if the last event is removed or (or the staker is not present)
        // they are not staking.
        let currently_staking = changes_per_node
            .into_iter()
            .map(|(_pub_key, deltas)| deltas.last().expect("deltas non-empty").clone())
            .filter_map(|delta| match delta.change {
                StakeTableChange::Add => Some(delta.node_info),
                StakeTableChange::Remove => None,
            });

        let mut consensus_stake_table: Vec<StakeTableEntry<PubKey>> = vec![];
        let mut da_stake_table: Vec<StakeTableEntry<PubKey>> = vec![];
        for node in currently_staking {
            consensus_stake_table.push(node.clone().into());
            if node.da {
                da_stake_table.push(node.into());
            }
        }
        Self::new(consensus_stake_table.into(), da_stake_table.into())
    }
}

#[derive(Clone, Debug)]
/// Type to describe DA and Stake memberships
pub struct MembershipCommittee {
    /// Holds Stake table and da stake
    state: Arc<RwLock<HashMap<Epoch, StaticCommittee>>>,

    /// Number of blocks in an epoch
    _epoch_size: u64,

    /// L1 provider
    l1_client: L1Client,
}

#[derive(Debug, Clone, PartialEq)]
enum StakeTableChange {
    Add,
    Remove,
}

#[derive(Debug, Clone)]
struct StakeTableDelta {
    change: StakeTableChange,
    node_info: NodeInfoJf,
}

impl StakeTableDelta {
    fn add(node_info: NodeInfoJf) -> Self {
        Self {
            change: StakeTableChange::Add,
            node_info,
        }
    }
    fn remove(node_info: NodeInfoJf) -> Self {
        Self {
            change: StakeTableChange::Remove,
            node_info,
        }
    }
}
/// Holds Stake table and da stake
#[derive(Clone, Debug)]
struct StaticCommittee {
    /// The nodes eligible for leadership.
    /// NOTE: This is currently a hack because the DA leader needs to be the quorum
    /// leader but without voting rights.
    eligible_leaders: Vec<StakeTableEntry<PubKey>>,

    /// TODO: add comment
    quorum: HashMap<PubKey, StakeTableEntry<PubKey>>,

    /// TODO: comment
    da: HashMap<PubKey, StakeTableEntry<PubKey>>,
}

impl MembershipCommittee {
    /// Updates `Self.stake_table` with stake_table for
    /// `Self.contract_address` at `l1_block_height`. This is intended
    /// to be called before calling `self.stake()` so that
    /// `Self.stake_table` only needs to be updated once in a given
    /// life-cycle but may be read from many times.
    fn update_stake_table(&self, st: StakeTables) {
        // This works because `get_stake_table` is fetching *all*
        // update events and building the table for us. We will need
        // more subtlety when start fetching only the events since last update.

        let indexed_stake_table: HashMap<PubKey, _> = st
            .quorum
            .0
            .iter()
            .map(|entry| (PubKey::public_key(entry), entry.clone()))
            .collect();

        let indexed_da_members: HashMap<PubKey, _> = st
            .da
            .0
            .iter()
            .map(|entry| (PubKey::public_key(entry), entry.clone()))
            .collect();

        let eligible_leaders: Vec<_> = st
            .quorum
            .0
            .into_iter()
            .filter(|entry| entry.stake() > U256::zero())
            .collect();

        let mut state = self.state.write_blocking();

        state.insert(
            EpochNumber::genesis(),
            StaticCommittee {
                eligible_leaders,
                quorum: indexed_stake_table,
                da: indexed_da_members,
            },
        );
    }

    // We need a constructor to match our concrete type.
    pub fn new_stake(
        // TODO remove `new` from trait and rename this to `new`.
        // https://github.com/EspressoSystems/HotShot/commit/fcb7d54a4443e29d643b3bbc53761856aef4de8b
        committee_members: Vec<PeerConfig<PubKey>>,
        da_members: Vec<PeerConfig<PubKey>>,
        instance_state: &NodeState,
        epoch_size: u64,
    ) -> Self {
        // For each eligible leader, get the stake table entry
        let eligible_leaders: Vec<_> = committee_members
            .iter()
            .map(|member| member.stake_table_entry.clone())
            .filter(|entry| entry.stake() > U256::zero())
            .collect();

        // For each member, get the stake table entry
        let members: Vec<_> = committee_members
            .iter()
            .map(|member| member.stake_table_entry.clone())
            .filter(|entry| entry.stake() > U256::zero())
            .collect();

        // For each member, get the stake table entry
        let da_members: Vec<_> = da_members
            .iter()
            .map(|member| member.stake_table_entry.clone())
            .filter(|entry| entry.stake() > U256::zero())
            .collect();

        // Index the stake table by public key
        let indexed_stake_table: HashMap<PubKey, _> = members
            .iter()
            .map(|entry| (PubKey::public_key(entry), entry.clone()))
            .collect();

        // Index the stake table by public key
        let indexed_da_stake_table: HashMap<PubKey, _> = da_members
            .iter()
            .map(|entry| (PubKey::public_key(entry), entry.clone()))
            .collect();

        let members = StaticCommittee {
            eligible_leaders,
            quorum: indexed_stake_table,
            da: indexed_da_stake_table,
        };

        let mut map = HashMap::new();
        map.insert(Epoch::genesis(), members);

        Self {
            state: Arc::new(RwLock::new(map)),
            _epoch_size: epoch_size,
            l1_client: instance_state.l1_client.clone(),
        }
    }
}

#[derive(Error, Debug)]
#[error("Could not lookup leader")] // TODO error variants? message?
pub struct LeaderLookupError;

impl Membership<SeqTypes> for MembershipCommittee {
    type Error = LeaderLookupError;

    // DO NOT USE. Dummy constructor to comply w/ trait.
    fn new(
        // TODO remove `new` from trait and remove this fn as well.
        // https://github.com/EspressoSystems/HotShot/commit/fcb7d54a4443e29d643b3bbc53761856aef4de8b
        committee_members: Vec<PeerConfig<PubKey>>,
        da_members: Vec<PeerConfig<PubKey>>,
    ) -> Self {
        // For each eligible leader, get the stake table entry
        let eligible_leaders: Vec<_> = committee_members
            .iter()
            .map(|member| member.stake_table_entry.clone())
            .filter(|entry| entry.stake() > U256::zero())
            .collect();

        // For each member, get the stake table entry
        let members: Vec<_> = committee_members
            .iter()
            .map(|member| member.stake_table_entry.clone())
            .filter(|entry| entry.stake() > U256::zero())
            .collect();

        // For each member, get the stake table entry
        let da_members: Vec<_> = da_members
            .iter()
            .map(|member| member.stake_table_entry.clone())
            .filter(|entry| entry.stake() > U256::zero())
            .collect();

        // Index the stake table by public key
        let indexed_stake_table: HashMap<PubKey, _> = members
            .iter()
            .map(|entry| (PubKey::public_key(entry), entry.clone()))
            .collect();

        // Index the stake table by public key
        let indexed_da_stake_table: HashMap<PubKey, _> = da_members
            .iter()
            .map(|entry| (PubKey::public_key(entry), entry.clone()))
            .collect();

        let members = StaticCommittee {
            eligible_leaders,
            quorum: indexed_stake_table,
            da: indexed_da_stake_table,
        };

        let mut map = HashMap::new();
        map.insert(Epoch::genesis(), members);

        Self {
            state: Arc::new(RwLock::new(map)),
            _epoch_size: 12,
            l1_client: L1Client::http(Url::from_str("http:://ab.b").unwrap()),
        }
    }

    /// Get the stake table for the current view
    fn stake_table(&self, epoch: Epoch) -> Vec<StakeTableEntry<PubKey>> {
        let state = self.state.read_blocking();

        match state.get(&epoch) {
            Some(st) => st.quorum.clone().into_values().collect(),
            None => {
                // TODO can we make state updates reusable? Otherwise we
                // have to repeat this for the other methods.
                let stake_tables = self.l1_client.stake_table(&epoch);
                self.update_stake_table(stake_tables.clone());
                stake_tables.quorum.0
            }
        }
    }
    /// Get the stake table for the current view
    fn da_stake_table(&self, epoch: Epoch) -> Vec<StakeTableEntry<PubKey>> {
        let state = self.state.read_blocking();
        match state.get(&epoch) {
            Some(sc) => sc.da.clone().into_values().collect(),
            None => {
                let stake_tables = self.l1_client.stake_table(&epoch);
                self.update_stake_table(stake_tables.clone());
                stake_tables.da.0
            }
        }
    }

    /// Get all members of the committee for the current view
    fn committee_members(
        &self,
        _view_number: <SeqTypes as NodeType>::View,
        epoch: Epoch,
    ) -> BTreeSet<PubKey> {
        let state = self.state.read_blocking();

        match state.get(&epoch) {
            Some(sc) => sc.quorum.clone().into_keys().collect(),
            None => {
                let stake_tables = self.l1_client.stake_table(&epoch);
                self.update_stake_table(stake_tables.clone());
                stake_tables
                    .quorum
                    .0
                    .iter()
                    .map(PubKey::public_key)
                    .collect()
            }
        }
    }

    /// Get all members of the committee for the current view
    fn da_committee_members(
        &self,
        _view_number: <SeqTypes as NodeType>::View,
        epoch: Epoch,
    ) -> BTreeSet<PubKey> {
        let sc = self.state.read_blocking();

        match sc.get(&epoch) {
            Some(sc) => sc.da.clone().into_keys().collect(),
            None => {
                let stake_tables = self.l1_client.stake_table(&epoch);
                self.update_stake_table(stake_tables.clone());
                stake_tables.da.0.iter().map(PubKey::public_key).collect()
            }
        }
    }

    /// Get all eligible leaders of the committee for the current view
    fn committee_leaders(
        &self,
        _view_number: <SeqTypes as NodeType>::View,
        epoch: Epoch,
    ) -> BTreeSet<PubKey> {
        let state = self.state.read_blocking();

        state
            .get(&epoch)
            .unwrap()
            .eligible_leaders
            .iter()
            .map(PubKey::public_key)
            .collect()
    }

    /// Get the stake table entry for a public key
    fn stake(&self, pub_key: &PubKey, epoch: Epoch) -> Option<StakeTableEntry<PubKey>> {
        let state = self.state.read_blocking();

        // Only return the stake if it is above zero
        state
            .get(&epoch)
            .and_then(|h| h.quorum.get(pub_key).cloned())
    }

    /// Get the DA stake table entry for a public key
    fn da_stake(&self, pub_key: &PubKey, epoch: Epoch) -> Option<StakeTableEntry<PubKey>> {
        let state = self.state.read_blocking();

        // Only return the stake if it is above zero
        state.get(&epoch).and_then(|h| h.da.get(pub_key).cloned())
    }

    /// Check if a node has stake in the committee
    fn has_stake(&self, pub_key: &PubKey, epoch: Epoch) -> bool {
        let state = self.state.read_blocking();

        state
            .get(&epoch)
            .and_then(|h| h.quorum.get(pub_key))
            .map_or(false, |x| x.stake() > U256::zero())
    }

    /// Check if a node has stake in the committee
    fn has_da_stake(&self, pub_key: &PubKey, epoch: Epoch) -> bool {
        let state = self.state.read_blocking();

        state
            .get(&epoch)
            .and_then(|h| h.da.get(pub_key))
            .map_or(false, |x| x.stake() > U256::zero())
    }

    /// Index the vector of public keys with the current view number
    fn lookup_leader(
        &self,
        view_number: <SeqTypes as NodeType>::View,
        epoch: Epoch,
    ) -> Result<PubKey, Self::Error> {
        let state = self.state.read_blocking();
        let leaders = state
            .get(&epoch)
            .ok_or(LeaderLookupError)?
            .eligible_leaders
            .clone();

        let index = *view_number as usize % leaders.len();
        let res = leaders[index].clone();
        Ok(PubKey::public_key(&res))
    }

    /// Get the total number of nodes in the committee
    fn total_nodes(&self, epoch: Epoch) -> usize {
        let state = self.state.read_blocking();
        state
            .get(&epoch)
            .map(|sc| sc.quorum.len())
            .unwrap_or_default()
    }

    /// Get the total number of DA nodes in the committee
    fn da_total_nodes(&self, epoch: Epoch) -> usize {
        let state = self.state.read_blocking();
        state
            .get(&epoch)
            .map(|sc: &StaticCommittee| sc.da.len())
            .unwrap_or_default()
    }

    /// Get the voting success threshold for the committee
    fn success_threshold(&self, epoch: Epoch) -> NonZeroU64 {
        let state = self.state.read_blocking();
        let quorum = state.get(&epoch).unwrap().quorum.clone();
        NonZeroU64::new(((quorum.len() as u64 * 2) / 3) + 1).unwrap()
    }

    /// Get the voting success threshold for the committee
    fn da_success_threshold(&self, epoch: Epoch) -> NonZeroU64 {
        let state = self.state.read_blocking();
        let da = state.get(&epoch).unwrap().da.clone();
        NonZeroU64::new(((da.len() as u64 * 2) / 3) + 1).unwrap()
    }

    /// Get the voting failure threshold for the committee
    fn failure_threshold(&self, epoch: Epoch) -> NonZeroU64 {
        let state = self.state.read_blocking();
        let quorum = state.get(&epoch).unwrap().quorum.clone();

        NonZeroU64::new(((quorum.len() as u64) / 3) + 1).unwrap()
    }

    /// Get the voting upgrade threshold for the committee
    fn upgrade_threshold(&self, epoch: Epoch) -> NonZeroU64 {
        let state = self.state.read_blocking();
        let quorum = state.get(&epoch).unwrap().quorum.clone();

        NonZeroU64::new(max(
            (quorum.len() as u64 * 9) / 10,
            ((quorum.len() as u64 * 2) / 3) + 1,
        ))
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use contract_bindings::permissioned_stake_table::NodeInfo;

    use super::*;

    #[test]
    fn test_stake_table_from_l1_events() {
        let mut rng = rand::thread_rng();

        // Build a stake table with one DA node and one consensus node.
        let mut da_node = NodeInfoJf::random(&mut rng);
        da_node.da = true;
        let mut consensus_node = NodeInfoJf::random(&mut rng);
        consensus_node.da = false;
        let added: Vec<NodeInfo> = vec![da_node.clone().into(), consensus_node.clone().into()];
        let mut updates = vec![StakersUpdatedFilter {
            removed: vec![],
            added,
        }];

        let st = StakeTables::from_l1_events(updates.clone());

        // The DA stake table contains the DA node only
        assert_eq!(st.da.0.len(), 1);
        assert_eq!(st.da.0[0].stake_key, da_node.stake_table_key);

        // The consensus stake table contains both nodes
        assert_eq!(st.quorum.0.len(), 2);
        assert_eq!(st.quorum.0[0].stake_key, da_node.stake_table_key);
        assert_eq!(st.da.0[1].stake_key, consensus_node.stake_table_key);

        // Simulate making the consensus node a DA node. This is accomplished by
        // sending a transaction removes and re-adds the same node with updated
        // DA status.
        let mut new_da_node = consensus_node.clone();
        new_da_node.da = true;
        updates.push(StakersUpdatedFilter {
            removed: vec![consensus_node.clone().into()],
            added: vec![new_da_node.clone().into()],
        });
        let st = StakeTables::from_l1_events(updates.clone());

        // The DA stake stable now contains both nodes
        assert_eq!(st.da.0.len(), 2);
        assert_eq!(st.da.0[0].stake_key, da_node.stake_table_key);
        assert_eq!(st.da.0[1].stake_key, new_da_node.stake_table_key);

        // The consensus stake stable (still) contains both nodes
        assert_eq!(st.quorum.0.len(), 2);
        assert_eq!(st.quorum.0[0].stake_key, da_node.stake_table_key);
        assert_eq!(st.quorum.0[1].stake_key, new_da_node.stake_table_key);

        // Simulate removing the second node
        updates.push(StakersUpdatedFilter {
            removed: vec![new_da_node.clone().into()],
            added: vec![],
        });
        let st = StakeTables::from_l1_events(updates);

        // The DA stake table contains only the original DA node
        assert_eq!(st.da.0.len(), 1);
        assert_eq!(st.da.0[0].stake_key, da_node.stake_table_key);

        // The consensus stake table also contains only the original DA node
        assert_eq!(st.quorum.0.len(), 1);
        assert_eq!(st.quorum.0[0].stake_key, da_node.stake_table_key);
    }

    // TODO: test that repeatedly removes and adds more nodes

    // TODO: the contract prevents removing nodes that aren't stakers and adding
    //       stakers multiple times, but should the rust code handle it too?
}
