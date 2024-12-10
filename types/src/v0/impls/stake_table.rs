use super::{L1Client, NodeState, PubKey, SeqTypes};
use contract_bindings::permissioned_stake_table::{NodeInfo, StakersUpdatedFilter};
use derive_more::derive::{From, Into};
use ethers::{abi::Address, types::U256};
use hotshot::types::SignatureKey as _;
use hotshot_contract_adapter::stake_table::NodeInfoJf;
use hotshot_types::{
    stake_table::StakeTableEntry,
    traits::{
        election::Membership, node_implementation::NodeType, signature_key::StakeTableEntryType,
    },
    PeerConfig,
};
use itertools::Itertools;
use std::{
    cmp::max,
    collections::{BTreeMap, BTreeSet, HashSet},
    num::NonZeroU64,
    str::FromStr,
};
use thiserror::Error;
use url::Url;

type Epoch = <SeqTypes as NodeType>::Epoch;

// NewTypes for two types of stake tables to avoid confusion
#[derive(Clone, Debug, From, Into)]
pub struct DAStakeTable(pub Vec<StakeTableEntry<PubKey>>);

#[derive(Clone, Debug, From, Into)]
pub struct ConsensusStakeTable(pub Vec<StakeTableEntry<PubKey>>);

#[derive(Clone, Debug)]
pub struct StakeTables {
    pub consensus_stake_table: ConsensusStakeTable,
    pub da_stake_table: DAStakeTable,
}

impl StakeTables {
    pub fn new(consensus_stake_table: ConsensusStakeTable, da_stake_table: DAStakeTable) -> Self {
        Self {
            consensus_stake_table,
            da_stake_table,
        }
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
pub struct StaticCommittee {
    /// The nodes eligible for leadership.
    /// NOTE: This is currently a hack because the DA leader needs to be the quorum
    /// leader but without voting rights.
    eligible_leaders: Vec<StakeTableEntry<PubKey>>,

    /// The nodes on the committee and their stake
    stake_table: HashSet<StakeTableEntry<PubKey>>,

    /// The nodes on the committee and their stake
    da_stake_table: HashSet<StakeTableEntry<PubKey>>,

    /// The nodes on the committee and their stake, indexed by public key
    indexed_stake_table: BTreeMap<PubKey, StakeTableEntry<PubKey>>,

    /// The nodes on the committee and their stake, indexed by public key
    indexed_da_stake_table: BTreeMap<PubKey, StakeTableEntry<PubKey>>,

    /// Number of blocks in an epoch
    epoch_size: u64,

    /// Address of StakeTable contract (proxy address)
    contract_address: Option<Address>,

    /// L1 provider
    provider: L1Client,
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

impl StaticCommittee {
    /// Updates `Self.stake_table` with stake_table for
    /// `Self.contract_address` at `l1_block_height`. This is intended
    /// to be called before calling `self.stake()` so that
    /// `Self.stake_table` only needs to be updated once in a given
    /// life-cycle but may be read from many times.
    async fn update_stake_table(&mut self, l1_block_height: u64) {
        // TODO also deal w/ removed entries
        let updates: Vec<StakersUpdatedFilter> = self
            .provider
            .get_stake_table(l1_block_height, self.contract_address.unwrap())
            .await;

        let added: Vec<_> = updates
            .into_iter()
            .flat_map(|e: StakersUpdatedFilter| e.added.into_iter().map(NodeInfoJf::from))
            .collect();
        for node in added {
            if !node.da {
                let entry: StakeTableEntry<PubKey> = StakeTableEntry {
                    stake_key: node.stake_table_key,
                    stake_amount: U256::from(1),
                };
                self.stake_table.insert(entry);
            }
        }
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
        let indexed_stake_table: BTreeMap<PubKey, _> = members
            .iter()
            .map(|entry| (PubKey::public_key(entry), entry.clone()))
            .collect();

        // Index the stake table by public key
        let indexed_da_stake_table: BTreeMap<PubKey, _> = da_members
            .iter()
            .map(|entry| (PubKey::public_key(entry), entry.clone()))
            .collect();

        Self {
            eligible_leaders,
            stake_table: HashSet::from_iter(members),
            da_stake_table: HashSet::from_iter(da_members),
            indexed_stake_table,
            indexed_da_stake_table,
            epoch_size,
            provider: instance_state.l1_client.clone(),
            contract_address: instance_state.chain_config.stake_table_contract,
        }
    }
}

#[derive(Error, Debug)]
#[error("Could not lookup leader")] // TODO error variants? message?
pub struct LeaderLookupError;

impl Membership<SeqTypes> for StaticCommittee {
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
        let indexed_stake_table: BTreeMap<PubKey, _> = members
            .iter()
            .map(|entry| (PubKey::public_key(entry), entry.clone()))
            .collect();

        // Index the stake table by public key
        let indexed_da_stake_table: BTreeMap<PubKey, _> = da_members
            .iter()
            .map(|entry| (PubKey::public_key(entry), entry.clone()))
            .collect();

        Self {
            eligible_leaders,
            stake_table: HashSet::from_iter(members),
            da_stake_table: HashSet::from_iter(da_members),
            indexed_stake_table,
            indexed_da_stake_table,
            epoch_size: 12, // TODO get the real number from config (I think)
            provider: L1Client::http(Url::from_str("http:://ab.b").unwrap()),
            contract_address: None,
        }
    }
    /// Get the stake table for the current view
    fn stake_table(&self, _epoch: Epoch) -> Vec<StakeTableEntry<PubKey>> {
        self.stake_table.clone().into_iter().collect()
    }
    /// Get the stake table for the current view
    fn da_stake_table(&self, _epoch: Epoch) -> Vec<StakeTableEntry<PubKey>> {
        self.da_stake_table.clone().into_iter().collect()
    }

    /// Get all members of the committee for the current view
    fn committee_members(
        &self,
        _view_number: <SeqTypes as NodeType>::View,
        _epoch: Epoch,
    ) -> BTreeSet<PubKey> {
        self.stake_table.iter().map(PubKey::public_key).collect()
    }

    /// Get all members of the committee for the current view
    fn da_committee_members(
        &self,
        _view_number: <SeqTypes as NodeType>::View,
        _epoch: Epoch,
    ) -> BTreeSet<PubKey> {
        self.da_stake_table.iter().map(PubKey::public_key).collect()
    }

    /// Get all eligible leaders of the committee for the current view
    fn committee_leaders(
        &self,
        _view_number: <SeqTypes as NodeType>::View,
        _epoch: Epoch,
    ) -> BTreeSet<PubKey> {
        self.eligible_leaders
            .iter()
            .map(PubKey::public_key)
            .collect()
    }

    /// Get the stake table entry for a public key
    fn stake(&self, pub_key: &PubKey, _epoch: Epoch) -> Option<StakeTableEntry<PubKey>> {
        // Only return the stake if it is above zero
        self.indexed_stake_table.get(pub_key).cloned()
    }

    /// Get the DA stake table entry for a public key
    fn da_stake(&self, pub_key: &PubKey, _epoch: Epoch) -> Option<StakeTableEntry<PubKey>> {
        // Only return the stake if it is above zero
        self.indexed_da_stake_table.get(pub_key).cloned()
    }

    /// Check if a node has stake in the committee
    fn has_stake(&self, pub_key: &PubKey, _epoch: Epoch) -> bool {
        self.indexed_stake_table
            .get(pub_key)
            .is_some_and(|x| x.stake() > U256::zero())
    }

    /// Check if a node has stake in the committee
    fn has_da_stake(&self, pub_key: &PubKey, _epoch: Epoch) -> bool {
        self.indexed_da_stake_table
            .get(pub_key)
            .is_some_and(|x| x.stake() > U256::zero())
    }

    /// Index the vector of public keys with the current view number
    fn lookup_leader(
        &self,
        view_number: <SeqTypes as NodeType>::View,
        _epoch: Epoch,
    ) -> Result<PubKey, Self::Error> {
        let index = *view_number as usize % self.eligible_leaders.len();
        let res = self.eligible_leaders[index].clone();
        Ok(PubKey::public_key(&res))
    }

    /// Get the total number of nodes in the committee
    fn total_nodes(&self, _epoch: Epoch) -> usize {
        self.stake_table.len()
    }

    /// Get the total number of DA nodes in the committee
    fn da_total_nodes(&self, _epoch: Epoch) -> usize {
        self.da_stake_table.len()
    }

    /// Get the voting success threshold for the committee
    fn success_threshold(&self, _epoch: Epoch) -> NonZeroU64 {
        NonZeroU64::new(((self.stake_table.len() as u64 * 2) / 3) + 1).unwrap()
    }

    /// Get the voting success threshold for the committee
    fn da_success_threshold(&self, _epoch: Epoch) -> NonZeroU64 {
        NonZeroU64::new(((self.da_stake_table.len() as u64 * 2) / 3) + 1).unwrap()
    }

    /// Get the voting failure threshold for the committee
    fn failure_threshold(&self, _epoch: Epoch) -> NonZeroU64 {
        NonZeroU64::new(((self.stake_table.len() as u64) / 3) + 1).unwrap()
    }

    /// Get the voting upgrade threshold for the committee
    fn upgrade_threshold(&self, _epoch: Epoch) -> NonZeroU64 {
        NonZeroU64::new(max(
            (self.stake_table.len() as u64 * 9) / 10,
            ((self.stake_table.len() as u64 * 2) / 3) + 1,
        ))
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(st.da_stake_table.0.len(), 1);
        assert_eq!(st.da_stake_table.0[0].stake_key, da_node.stake_table_key);

        // The consensus stake table contains both nodes
        assert_eq!(st.consensus_stake_table.0.len(), 2);
        assert_eq!(
            st.consensus_stake_table.0[0].stake_key,
            da_node.stake_table_key
        );
        assert_eq!(
            st.consensus_stake_table.0[1].stake_key,
            consensus_node.stake_table_key
        );

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
        assert_eq!(st.da_stake_table.0.len(), 2);
        assert_eq!(st.da_stake_table.0[0].stake_key, da_node.stake_table_key);
        assert_eq!(
            st.da_stake_table.0[1].stake_key,
            new_da_node.stake_table_key
        );

        // The consensus stake stable (still) contains both nodes
        assert_eq!(st.consensus_stake_table.0.len(), 2);
        assert_eq!(
            st.consensus_stake_table.0[0].stake_key,
            da_node.stake_table_key
        );
        assert_eq!(
            st.consensus_stake_table.0[1].stake_key,
            new_da_node.stake_table_key
        );

        // Simulate removing the second node
        updates.push(StakersUpdatedFilter {
            removed: vec![new_da_node.clone().into()],
            added: vec![],
        });
        let st = StakeTables::from_l1_events(updates);

        // The DA stake table contains only the original DA node
        assert_eq!(st.da_stake_table.0.len(), 1);
        assert_eq!(st.da_stake_table.0[0].stake_key, da_node.stake_table_key);

        // The consensus stake table also contains only the original DA node
        assert_eq!(st.consensus_stake_table.0.len(), 1);
        assert_eq!(
            st.consensus_stake_table.0[0].stake_key,
            da_node.stake_table_key
        );
    }

    // TODO: test that repeatedly removes and adds more nodes

    // TODO: the contract prevents removing nodes that aren't stakers and adding
    //       stakers multiple times, but should the rust code handle it too?
}
