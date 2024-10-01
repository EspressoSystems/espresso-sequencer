// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{collections::BTreeMap, num::NonZeroU64};

use ethereum_types::U256;
use hotshot_types::{
    traits::{
        election::Membership,
        network::Topic,
        node_implementation::NodeType,
        signature_key::{SignatureKey, StakeTableEntryType},
    },
    PeerConfig,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]

/// The static committee election
pub struct StaticCommitteeLeaderForTwoViews<T: NodeType> {
    /// The nodes eligible for leadership.
    /// NOTE: This is currently a hack because the DA leader needs to be the quorum
    /// leader but without voting rights.
    eligible_leaders: Vec<<T::SignatureKey as SignatureKey>::StakeTableEntry>,

    /// The nodes on the committee and their stake
    stake_table: Vec<<T::SignatureKey as SignatureKey>::StakeTableEntry>,

    /// The nodes on the committee and their stake, indexed by public key
    indexed_stake_table:
        BTreeMap<T::SignatureKey, <T::SignatureKey as SignatureKey>::StakeTableEntry>,

    /// The network topic of the committee
    committee_topic: Topic,
}

impl<TYPES: NodeType> Membership<TYPES> for StaticCommitteeLeaderForTwoViews<TYPES> {
    /// Create a new election
    fn new(
        eligible_leaders: Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>>,
        committee_members: Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>>,
        committee_topic: Topic,
    ) -> Self {
        // For each eligible leader, get the stake table entry
        let eligible_leaders: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> =
            eligible_leaders
                .iter()
                .map(|member| member.stake_table_entry.clone())
                .filter(|entry| entry.stake() > U256::zero())
                .collect();

        // For each member, get the stake table entry
        let members: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> =
            committee_members
                .iter()
                .map(|member| member.stake_table_entry.clone())
                .filter(|entry| entry.stake() > U256::zero())
                .collect();

        // Index the stake table by public key
        let indexed_stake_table: BTreeMap<
            TYPES::SignatureKey,
            <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        > = members
            .iter()
            .map(|entry| (TYPES::SignatureKey::public_key(entry), entry.clone()))
            .collect();

        Self {
            eligible_leaders,
            stake_table: members,
            indexed_stake_table,
            committee_topic,
        }
    }

    /// Get the stake table for the current view
    fn stake_table(
        &self,
    ) -> Vec<<<TYPES as NodeType>::SignatureKey as SignatureKey>::StakeTableEntry> {
        self.stake_table.clone()
    }

    /// Get all members of the committee for the current view
    fn committee_members(
        &self,
        _view_number: <TYPES as NodeType>::Time,
    ) -> std::collections::BTreeSet<<TYPES as NodeType>::SignatureKey> {
        self.stake_table
            .iter()
            .map(TYPES::SignatureKey::public_key)
            .collect()
    }

    /// Get all eligible leaders of the committee for the current view
    fn committee_leaders(
        &self,
        _view_number: <TYPES as NodeType>::Time,
    ) -> std::collections::BTreeSet<<TYPES as NodeType>::SignatureKey> {
        self.eligible_leaders
            .iter()
            .map(TYPES::SignatureKey::public_key)
            .collect()
    }

    /// Get the stake table entry for a public key
    fn stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
    ) -> Option<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> {
        // Only return the stake if it is above zero
        self.indexed_stake_table.get(pub_key).cloned()
    }

    /// Check if a node has stake in the committee
    fn has_stake(&self, pub_key: &<TYPES as NodeType>::SignatureKey) -> bool {
        self.indexed_stake_table
            .get(pub_key)
            .is_some_and(|x| x.stake() > U256::zero())
    }

    /// Get the network topic for the committee
    fn committee_topic(&self) -> Topic {
        self.committee_topic.clone()
    }

    /// Index the vector of public keys with the current view number
    fn leader(&self, view_number: TYPES::Time) -> TYPES::SignatureKey {
        let index =
            usize::try_from((*view_number / 2) % self.eligible_leaders.len() as u64).unwrap();
        let res = self.eligible_leaders[index].clone();
        TYPES::SignatureKey::public_key(&res)
    }

    /// Get the total number of nodes in the committee
    fn total_nodes(&self) -> usize {
        self.stake_table.len()
    }

    /// Get the voting success threshold for the committee
    fn success_threshold(&self) -> NonZeroU64 {
        NonZeroU64::new(((self.stake_table.len() as u64 * 2) / 3) + 1).unwrap()
    }

    /// Get the voting failure threshold for the committee
    fn failure_threshold(&self) -> NonZeroU64 {
        NonZeroU64::new(((self.stake_table.len() as u64) / 3) + 1).unwrap()
    }

    /// Get the voting upgrade threshold for the committee
    fn upgrade_threshold(&self) -> NonZeroU64 {
        NonZeroU64::new(((self.stake_table.len() as u64 * 9) / 10) + 1).unwrap()
    }
}
