// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{cmp::max, collections::BTreeMap, num::NonZeroU64};

use hotshot_types::{
    traits::{
        election::Membership,
        node_implementation::NodeType,
        signature_key::{SignatureKey, StakeTableEntryType},
    },
    PeerConfig,
};
use primitive_types::U256;
use utils::anytrace::Result;

/// Tuple type for eligible leaders
type EligibleLeaders<T> = (
    Vec<<<T as NodeType>::SignatureKey as SignatureKey>::StakeTableEntry>,
    Vec<<<T as NodeType>::SignatureKey as SignatureKey>::StakeTableEntry>,
);

/// Tuple type for stake tables
type StakeTables<T> = (
    Vec<<<T as NodeType>::SignatureKey as SignatureKey>::StakeTableEntry>,
    Vec<<<T as NodeType>::SignatureKey as SignatureKey>::StakeTableEntry>,
);

/// Tuple type for indexed stake tables
type IndexedStakeTables<T> = (
    BTreeMap<
        <T as NodeType>::SignatureKey,
        <<T as NodeType>::SignatureKey as SignatureKey>::StakeTableEntry,
    >,
    BTreeMap<
        <T as NodeType>::SignatureKey,
        <<T as NodeType>::SignatureKey as SignatureKey>::StakeTableEntry,
    >,
);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
/// The static committee election
pub struct TwoStaticCommittees<T: NodeType> {
    /// The nodes eligible for leadership.
    /// NOTE: This is currently a hack because the DA leader needs to be the quorum
    /// leader but without voting rights.
    eligible_leaders: EligibleLeaders<T>,

    /// The nodes on the committee and their stake
    stake_table: StakeTables<T>,

    /// The nodes on the committee and their stake
    da_stake_table: StakeTables<T>,

    /// The nodes on the committee and their stake, indexed by public key
    indexed_stake_table: IndexedStakeTables<T>,

    /// The nodes on the committee and their stake, indexed by public key
    indexed_da_stake_table: IndexedStakeTables<T>,
}

impl<TYPES: NodeType> Membership<TYPES> for TwoStaticCommittees<TYPES> {
    type Error = utils::anytrace::Error;

    /// Create a new election
    fn new(
        committee_members: Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>>,
        da_members: Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>>,
    ) -> Self {
        // For each eligible leader, get the stake table entry
        let eligible_leaders: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> =
            committee_members
                .iter()
                .map(|member| member.stake_table_entry.clone())
                .filter(|entry| entry.stake() > U256::zero())
                .collect();

        let eligible_leaders1 = eligible_leaders
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx % 2 == 0)
            .map(|(_, leader)| leader.clone())
            .collect();
        let eligible_leaders2 = eligible_leaders
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx % 2 == 1)
            .map(|(_, leader)| leader.clone())
            .collect();

        // For each member, get the stake table entry
        let members: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> =
            committee_members
                .iter()
                .map(|member| member.stake_table_entry.clone())
                .filter(|entry| entry.stake() > U256::zero())
                .collect();

        let members1: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> = members
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx % 2 == 0)
            .map(|(_, leader)| leader.clone())
            .collect();
        let members2: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> = members
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx % 2 == 1)
            .map(|(_, leader)| leader.clone())
            .collect();

        // For each member, get the stake table entry
        let da_members: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> = da_members
            .iter()
            .map(|member| member.stake_table_entry.clone())
            .filter(|entry| entry.stake() > U256::zero())
            .collect();

        let da_members1: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> = da_members
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx % 2 == 0)
            .map(|(_, leader)| leader.clone())
            .collect();
        let da_members2: Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> = da_members
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx % 2 == 1)
            .map(|(_, leader)| leader.clone())
            .collect();

        // Index the stake table by public key
        let indexed_stake_table1: BTreeMap<
            TYPES::SignatureKey,
            <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        > = members1
            .iter()
            .map(|entry| (TYPES::SignatureKey::public_key(entry), entry.clone()))
            .collect();

        let indexed_stake_table2: BTreeMap<
            TYPES::SignatureKey,
            <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        > = members2
            .iter()
            .map(|entry| (TYPES::SignatureKey::public_key(entry), entry.clone()))
            .collect();

        // Index the stake table by public key
        let indexed_da_stake_table1: BTreeMap<
            TYPES::SignatureKey,
            <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        > = da_members1
            .iter()
            .map(|entry| (TYPES::SignatureKey::public_key(entry), entry.clone()))
            .collect();

        let indexed_da_stake_table2: BTreeMap<
            TYPES::SignatureKey,
            <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        > = da_members2
            .iter()
            .map(|entry| (TYPES::SignatureKey::public_key(entry), entry.clone()))
            .collect();

        Self {
            eligible_leaders: (eligible_leaders1, eligible_leaders2),
            stake_table: (members1, members2),
            da_stake_table: (da_members1, da_members2),
            indexed_stake_table: (indexed_stake_table1, indexed_stake_table2),
            indexed_da_stake_table: (indexed_da_stake_table1, indexed_da_stake_table2),
        }
    }

    /// Get the stake table for the current view
    fn stake_table(
        &self,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Vec<<<TYPES as NodeType>::SignatureKey as SignatureKey>::StakeTableEntry> {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            self.stake_table.0.clone()
        } else {
            self.stake_table.1.clone()
        }
    }

    /// Get the stake table for the current view
    fn da_stake_table(
        &self,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Vec<<<TYPES as NodeType>::SignatureKey as SignatureKey>::StakeTableEntry> {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            self.da_stake_table.0.clone()
        } else {
            self.da_stake_table.1.clone()
        }
    }

    /// Get all members of the committee for the current view
    fn committee_members(
        &self,
        _view_number: <TYPES as NodeType>::View,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> std::collections::BTreeSet<<TYPES as NodeType>::SignatureKey> {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            self.stake_table
                .0
                .iter()
                .map(TYPES::SignatureKey::public_key)
                .collect()
        } else {
            self.stake_table
                .1
                .iter()
                .map(TYPES::SignatureKey::public_key)
                .collect()
        }
    }

    /// Get all members of the committee for the current view
    fn da_committee_members(
        &self,
        _view_number: <TYPES as NodeType>::View,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> std::collections::BTreeSet<<TYPES as NodeType>::SignatureKey> {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            self.da_stake_table
                .0
                .iter()
                .map(TYPES::SignatureKey::public_key)
                .collect()
        } else {
            self.da_stake_table
                .1
                .iter()
                .map(TYPES::SignatureKey::public_key)
                .collect()
        }
    }

    /// Get all eligible leaders of the committee for the current view
    fn committee_leaders(
        &self,
        _view_number: <TYPES as NodeType>::View,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> std::collections::BTreeSet<<TYPES as NodeType>::SignatureKey> {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            self.eligible_leaders
                .0
                .iter()
                .map(TYPES::SignatureKey::public_key)
                .collect()
        } else {
            self.eligible_leaders
                .1
                .iter()
                .map(TYPES::SignatureKey::public_key)
                .collect()
        }
    }

    /// Get the stake table entry for a public key
    fn stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Option<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> {
        // Only return the stake if it is above zero
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            self.indexed_stake_table.0.get(pub_key).cloned()
        } else {
            self.indexed_stake_table.1.get(pub_key).cloned()
        }
    }

    /// Get the DA stake table entry for a public key
    fn da_stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Option<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> {
        // Only return the stake if it is above zero
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            self.indexed_da_stake_table.0.get(pub_key).cloned()
        } else {
            self.indexed_da_stake_table.1.get(pub_key).cloned()
        }
    }

    /// Check if a node has stake in the committee
    fn has_stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> bool {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            self.indexed_stake_table
                .0
                .get(pub_key)
                .is_some_and(|x| x.stake() > U256::zero())
        } else {
            self.indexed_stake_table
                .1
                .get(pub_key)
                .is_some_and(|x| x.stake() > U256::zero())
        }
    }

    /// Check if a node has stake in the committee
    fn has_da_stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> bool {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            self.indexed_da_stake_table
                .0
                .get(pub_key)
                .is_some_and(|x| x.stake() > U256::zero())
        } else {
            self.indexed_da_stake_table
                .1
                .get(pub_key)
                .is_some_and(|x| x.stake() > U256::zero())
        }
    }

    /// Index the vector of public keys with the current view number
    fn lookup_leader(
        &self,
        view_number: <TYPES as NodeType>::View,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<TYPES::SignatureKey> {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            #[allow(clippy::cast_possible_truncation)]
            let index = *view_number as usize % self.eligible_leaders.0.len();
            let res = self.eligible_leaders.0[index].clone();
            Ok(TYPES::SignatureKey::public_key(&res))
        } else {
            #[allow(clippy::cast_possible_truncation)]
            let index = *view_number as usize % self.eligible_leaders.1.len();
            let res = self.eligible_leaders.1[index].clone();
            Ok(TYPES::SignatureKey::public_key(&res))
        }
    }

    /// Get the total number of nodes in the committee
    fn total_nodes(&self, epoch: Option<<TYPES as NodeType>::Epoch>) -> usize {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            self.stake_table.0.len()
        } else {
            self.stake_table.1.len()
        }
    }

    /// Get the total number of DA nodes in the committee
    fn da_total_nodes(&self, epoch: Option<<TYPES as NodeType>::Epoch>) -> usize {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            self.da_stake_table.0.len()
        } else {
            self.da_stake_table.1.len()
        }
    }

    /// Get the voting success threshold for the committee
    fn success_threshold(&self, epoch: Option<<TYPES as NodeType>::Epoch>) -> NonZeroU64 {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            NonZeroU64::new(((self.stake_table.0.len() as u64 * 2) / 3) + 1).unwrap()
        } else {
            NonZeroU64::new(((self.stake_table.1.len() as u64 * 2) / 3) + 1).unwrap()
        }
    }

    /// Get the voting success threshold for the committee
    fn da_success_threshold(&self, epoch: Option<TYPES::Epoch>) -> NonZeroU64 {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            NonZeroU64::new(((self.da_stake_table.0.len() as u64 * 2) / 3) + 1).unwrap()
        } else {
            NonZeroU64::new(((self.da_stake_table.1.len() as u64 * 2) / 3) + 1).unwrap()
        }
    }

    /// Get the voting failure threshold for the committee
    fn failure_threshold(&self, epoch: Option<<TYPES as NodeType>::Epoch>) -> NonZeroU64 {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            NonZeroU64::new(((self.stake_table.0.len() as u64) / 3) + 1).unwrap()
        } else {
            NonZeroU64::new(((self.stake_table.1.len() as u64) / 3) + 1).unwrap()
        }
    }

    /// Get the voting upgrade threshold for the committee
    fn upgrade_threshold(&self, epoch: Option<<TYPES as NodeType>::Epoch>) -> NonZeroU64 {
        let epoch = epoch.expect("epochs cannot be disabled with TwoStaticCommittees");
        if *epoch != 0 && *epoch % 2 == 0 {
            NonZeroU64::new(max(
                (self.stake_table.0.len() as u64 * 9) / 10,
                ((self.stake_table.0.len() as u64 * 2) / 3) + 1,
            ))
            .unwrap()
        } else {
            NonZeroU64::new(max(
                (self.stake_table.1.len() as u64 * 9) / 10,
                ((self.stake_table.1.len() as u64 * 2) / 3) + 1,
            ))
            .unwrap()
        }
    }
}
