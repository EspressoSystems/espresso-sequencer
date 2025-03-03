// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{
    cmp::max,
    collections::{BTreeMap, BTreeSet},
    num::NonZeroU64,
};

use hotshot_types::{
    drb::{
        election::{generate_stake_cdf, select_randomized_leader, RandomizedCommittee},
        DrbResult,
    },
    traits::{
        election::Membership,
        node_implementation::NodeType,
        signature_key::{SignatureKey, StakeTableEntryType},
    },
    PeerConfig,
};
use hotshot_utils::anytrace::*;
use primitive_types::U256;

#[derive(Clone, Debug)]

/// The static committee election
pub struct Committee<T: NodeType> {
    /// The nodes eligible for leadership.
    /// NOTE: This is currently a hack because the DA leader needs to be the quorum
    /// leader but without voting rights.
    eligible_leaders: Vec<PeerConfig<T::SignatureKey>>,

    /// The nodes on the committee and their stake
    stake_table: Vec<PeerConfig<T::SignatureKey>>,

    /// The nodes on the committee and their stake
    da_stake_table: Vec<PeerConfig<T::SignatureKey>>,

    /// Stake tables randomized with the DRB, used (only) for leader election
    randomized_committee: RandomizedCommittee<<T::SignatureKey as SignatureKey>::StakeTableEntry>,

    /// The nodes on the committee and their stake, indexed by public key
    indexed_stake_table: BTreeMap<T::SignatureKey, PeerConfig<T::SignatureKey>>,

    /// The nodes on the committee and their stake, indexed by public key
    indexed_da_stake_table: BTreeMap<T::SignatureKey, PeerConfig<T::SignatureKey>>,
}

impl<TYPES: NodeType> Membership<TYPES> for Committee<TYPES> {
    type Error = hotshot_utils::anytrace::Error;
    /// Create a new election
    fn new(
        committee_members: Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>>,
        da_members: Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>>,
    ) -> Self {
        // For each eligible leader, get the stake table entry
        let eligible_leaders: Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>> =
            committee_members
                .iter()
                .filter(|&member| member.stake_table_entry.stake() > U256::zero())
                .cloned()
                .collect();

        // For each member, get the stake table entry
        let members: Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>> = committee_members
            .iter()
            .filter(|&member| member.stake_table_entry.stake() > U256::zero())
            .cloned()
            .collect();

        // For each member, get the stake table entry
        let da_members: Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>> = da_members
            .iter()
            .filter(|&member| member.stake_table_entry.stake() > U256::zero())
            .cloned()
            .collect();

        // Index the stake table by public key
        let indexed_stake_table: BTreeMap<TYPES::SignatureKey, PeerConfig<TYPES::SignatureKey>> =
            members
                .iter()
                .map(|config| {
                    (
                        TYPES::SignatureKey::public_key(&config.stake_table_entry),
                        config.clone(),
                    )
                })
                .collect();

        // Index the stake table by public key
        let indexed_da_stake_table: BTreeMap<TYPES::SignatureKey, PeerConfig<TYPES::SignatureKey>> =
            da_members
                .iter()
                .map(|config| {
                    (
                        TYPES::SignatureKey::public_key(&config.stake_table_entry),
                        config.clone(),
                    )
                })
                .collect();

        // We use a constant value of `[0u8; 32]` for the drb, since this is just meant to be used in tests
        let randomized_committee = generate_stake_cdf(
            eligible_leaders
                .clone()
                .into_iter()
                .map(|leader| leader.stake_table_entry)
                .collect::<Vec<_>>(),
            [0u8; 32],
        );

        Self {
            eligible_leaders,
            stake_table: members,
            da_stake_table: da_members,
            randomized_committee,
            indexed_stake_table,
            indexed_da_stake_table,
        }
    }

    /// Get the stake table for the current view
    fn stake_table(
        &self,
        _epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>>> {
        Ok(self.stake_table.clone())
    }

    /// Get the stake table for the current view
    fn da_stake_table(
        &self,
        _epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>>> {
        Ok(self.da_stake_table.clone())
    }

    /// Get all members of the committee for the current view
    fn committee_members(
        &self,
        _view_number: <TYPES as NodeType>::View,
        _epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<BTreeSet<<TYPES as NodeType>::SignatureKey>> {
        Ok(self
            .stake_table
            .iter()
            .map(|x| TYPES::SignatureKey::public_key(&x.stake_table_entry))
            .collect())
    }

    /// Get all members of the committee for the current view
    fn da_committee_members(
        &self,
        _view_number: <TYPES as NodeType>::View,
        _epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<BTreeSet<<TYPES as NodeType>::SignatureKey>> {
        Ok(self
            .da_stake_table
            .iter()
            .map(|x| TYPES::SignatureKey::public_key(&x.stake_table_entry))
            .collect())
    }

    /// Get all eligible leaders of the committee for the current view
    fn committee_leaders(
        &self,
        _view_number: <TYPES as NodeType>::View,
        _epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<BTreeSet<<TYPES as NodeType>::SignatureKey>> {
        Ok(self
            .eligible_leaders
            .iter()
            .map(|x| TYPES::SignatureKey::public_key(&x.stake_table_entry))
            .collect())
    }

    /// Get the stake table entry for a public key
    fn stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
        _epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Option<PeerConfig<TYPES::SignatureKey>> {
        // Only return the stake if it is above zero
        self.indexed_stake_table.get(pub_key).cloned()
    }

    /// Get the stake table entry for a public key
    fn da_stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
        _epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Option<PeerConfig<TYPES::SignatureKey>> {
        // Only return the stake if it is above zero
        self.indexed_da_stake_table.get(pub_key).cloned()
    }

    /// Check if a node has stake in the committee
    fn has_stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
        _epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<bool> {
        Ok(self
            .indexed_stake_table
            .get(pub_key)
            .context(error!("Error getting stake for public key"))?
            .stake_table_entry
            .stake()
            > U256::zero())
    }

    /// Check if a node has stake in the committee
    fn has_da_stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
        _epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<bool> {
        Ok(self
            .indexed_da_stake_table
            .get(pub_key)
            .context(error!("Error getting stake for public key"))?
            .stake_table_entry
            .stake()
            > U256::zero())
    }

    // /// Get the network topic for the committee
    // fn committee_topic(&self) -> Topic {
    //     self.committee_topic.clone()
    // }

    /// Index the vector of public keys with the current view number
    fn lookup_leader(
        &self,
        view_number: <TYPES as NodeType>::View,
        _epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<TYPES::SignatureKey> {
        let res = select_randomized_leader(&self.randomized_committee, *view_number);

        Ok(TYPES::SignatureKey::public_key(&res))
    }

    /// Get the total number of nodes in the committee
    fn total_nodes(&self, _epoch: Option<<TYPES as NodeType>::Epoch>) -> Result<usize> {
        Ok(self.stake_table.len())
    }
    /// Get the total number of nodes in the committee
    fn da_total_nodes(&self, _epoch: Option<<TYPES as NodeType>::Epoch>) -> Result<usize> {
        Ok(self.da_stake_table.len())
    }
    /// Get the voting success threshold for the committee
    fn success_threshold(&self, _epoch: Option<<TYPES as NodeType>::Epoch>) -> Result<NonZeroU64> {
        NonZeroU64::new(((self.stake_table.len() as u64 * 2) / 3) + 1)
            .context(error!("success threshold is zero"))
    }

    /// Get the voting success threshold for the committee
    fn da_success_threshold(
        &self,
        _epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<NonZeroU64> {
        NonZeroU64::new(((self.da_stake_table.len() as u64 * 2) / 3) + 1)
            .context(error!("da success threshold is zero"))
    }

    /// Get the voting failure threshold for the committee
    fn failure_threshold(&self, _epoch: Option<<TYPES as NodeType>::Epoch>) -> Result<NonZeroU64> {
        NonZeroU64::new(((self.stake_table.len() as u64) / 3) + 1)
            .context(error!("failure threshold is zero"))
    }

    /// Get the voting upgrade threshold for the committee
    fn upgrade_threshold(&self, _epoch: Option<<TYPES as NodeType>::Epoch>) -> Result<NonZeroU64> {
        NonZeroU64::new(max(
            (self.stake_table.len() as u64 * 9) / 10,
            ((self.stake_table.len() as u64 * 2) / 3) + 1,
        ))
        .context(error!("upgrade threshold is zero"))
    }

    fn add_drb_result(&mut self, _epoch: <TYPES as NodeType>::Epoch, _drb: DrbResult) {}
}
