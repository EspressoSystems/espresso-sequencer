// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.
use std::{
    cmp::max,
    collections::{BTreeMap, BTreeSet},
    marker::PhantomData,
    num::NonZeroU64,
};

use hotshot_types::{
    drb::DrbResult,
    traits::{
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
        signature_key::{SignatureKey, StakeTableEntryType},
    },
    PeerConfig,
};
use hotshot_utils::anytrace::*;
use primitive_types::U256;
use rand::{rngs::StdRng, Rng};

use crate::traits::election::helpers::QuorumFilterConfig;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
/// The static committee election
pub struct RandomizedCommitteeMembers<T: NodeType, C: QuorumFilterConfig> {
    /// The nodes eligible for leadership.
    /// NOTE: This is currently a hack because the DA leader needs to be the quorum
    /// leader but without voting rights.
    eligible_leaders: Vec<PeerConfig<T::SignatureKey>>,

    /// The nodes on the committee and their stake
    stake_table: Vec<PeerConfig<T::SignatureKey>>,

    /// The nodes on the da committee and their stake
    da_stake_table: Vec<PeerConfig<T::SignatureKey>>,

    /// The nodes on the committee and their stake, indexed by public key
    indexed_stake_table: BTreeMap<T::SignatureKey, PeerConfig<T::SignatureKey>>,

    /// The nodes on the da committee and their stake, indexed by public key
    indexed_da_stake_table: BTreeMap<T::SignatureKey, PeerConfig<T::SignatureKey>>,

    /// Phantom
    _pd: PhantomData<C>,
}

impl<TYPES: NodeType, CONFIG: QuorumFilterConfig> RandomizedCommitteeMembers<TYPES, CONFIG> {
    /// Creates a set of indices into the stake_table which reference the nodes selected for this epoch's committee
    fn make_quorum_filter(&self, epoch: <TYPES as NodeType>::Epoch) -> BTreeSet<usize> {
        CONFIG::execute(epoch.u64(), self.stake_table.len())
    }

    /// Creates a set of indices into the da_stake_table which reference the nodes selected for this epoch's da committee
    fn make_da_quorum_filter(&self, epoch: <TYPES as NodeType>::Epoch) -> BTreeSet<usize> {
        CONFIG::execute(epoch.u64(), self.da_stake_table.len())
    }

    /// Writes the offsets used for the quorum filter and da_quorum filter to stdout
    fn debug_display_offsets(&self) {
        /// Ensures that the quorum filters are only displayed once
        static START: std::sync::Once = std::sync::Once::new();

        START.call_once(|| {
            tracing::error!(
                "{} offsets for Quorum filter:",
                std::any::type_name::<CONFIG>()
            );
            for epoch in 1..=10 {
                tracing::error!(
                    "  epoch {epoch}: {:?}",
                    self.make_quorum_filter(<TYPES as NodeType>::Epoch::new(epoch))
                );
            }

            tracing::error!(
                "{} offsets for DA Quorum filter:",
                std::any::type_name::<CONFIG>()
            );
            for epoch in 1..=10 {
                tracing::error!(
                    "  epoch {epoch}: {:?}",
                    self.make_da_quorum_filter(<TYPES as NodeType>::Epoch::new(epoch))
                );
            }
        });
    }
}

impl<TYPES: NodeType, CONFIG: QuorumFilterConfig> Membership<TYPES>
    for RandomizedCommitteeMembers<TYPES, CONFIG>
{
    /// The error type returned by methods like `lookup_leader`.
    type Error = hotshot_utils::anytrace::Error;
    /// Create a new election
    fn new(
        committee_members: Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>>,
        da_members: Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>>,
    ) -> Self {
        // For each eligible leader, get the stake table entry
        let eligible_leaders = committee_members
            .iter()
            .filter(|&member| member.stake_table_entry.stake() > U256::zero())
            .cloned()
            .collect();

        // For each member, get the stake table entry
        let members: Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>> = committee_members
            .iter()
            .filter(|&entry| entry.stake_table_entry.stake() > U256::zero())
            .cloned()
            .collect();

        // For each da member, get the stake table entry
        let da_members: Vec<PeerConfig<<TYPES as NodeType>::SignatureKey>> = da_members
            .iter()
            .filter(|&entry| entry.stake_table_entry.stake() > U256::zero())
            .cloned()
            .collect();

        // Index the stake table by public key
        let indexed_stake_table = members
            .iter()
            .map(|entry| {
                (
                    TYPES::SignatureKey::public_key(&entry.stake_table_entry),
                    entry.clone(),
                )
            })
            .collect();

        // Index the stake table by public key
        let indexed_da_stake_table = da_members
            .iter()
            .map(|entry| {
                (
                    TYPES::SignatureKey::public_key(&entry.stake_table_entry),
                    entry.clone(),
                )
            })
            .collect();

        let s = Self {
            eligible_leaders,
            stake_table: members,
            da_stake_table: da_members,
            indexed_stake_table,
            indexed_da_stake_table,
            _pd: PhantomData,
        };

        s.debug_display_offsets();

        s
    }

    /// Get the stake table for the current view
    fn stake_table(
        &self,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<Vec<PeerConfig<TYPES::SignatureKey>>> {
        if let Some(epoch) = epoch {
            let filter = self.make_quorum_filter(epoch);
            //self.stake_table.clone()s
            Ok(self
                .stake_table
                .iter()
                .enumerate()
                .filter(|(idx, _)| filter.contains(idx))
                .map(|(_, v)| v.clone())
                .collect())
        } else {
            Ok(self.stake_table.clone())
        }
    }

    /// Get the da stake table for the current view
    fn da_stake_table(
        &self,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<Vec<PeerConfig<TYPES::SignatureKey>>> {
        if let Some(epoch) = epoch {
            let filter = self.make_da_quorum_filter(epoch);
            //self.stake_table.clone()s
            Ok(self
                .da_stake_table
                .iter()
                .enumerate()
                .filter(|(idx, _)| filter.contains(idx))
                .map(|(_, v)| v.clone())
                .collect())
        } else {
            Ok(self.da_stake_table.clone())
        }
    }

    /// Get all members of the committee for the current view
    fn committee_members(
        &self,
        _view_number: <TYPES as NodeType>::View,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<BTreeSet<<TYPES as NodeType>::SignatureKey>> {
        if let Some(epoch) = epoch {
            let filter = self.make_quorum_filter(epoch);
            Ok(self
                .stake_table
                .iter()
                .enumerate()
                .filter(|(idx, _)| filter.contains(idx))
                .map(|(_, v)| TYPES::SignatureKey::public_key(&v.stake_table_entry))
                .collect())
        } else {
            Ok(self
                .stake_table
                .iter()
                .map(|config| TYPES::SignatureKey::public_key(&config.stake_table_entry))
                .collect())
        }
    }

    /// Get all members of the committee for the current view
    fn da_committee_members(
        &self,
        _view_number: <TYPES as NodeType>::View,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<BTreeSet<<TYPES as NodeType>::SignatureKey>> {
        if let Some(epoch) = epoch {
            let filter = self.make_da_quorum_filter(epoch);
            Ok(self
                .da_stake_table
                .iter()
                .enumerate()
                .filter(|(idx, _)| filter.contains(idx))
                .map(|(_, v)| TYPES::SignatureKey::public_key(&v.stake_table_entry))
                .collect())
        } else {
            Ok(self
                .da_stake_table
                .iter()
                .map(|config| TYPES::SignatureKey::public_key(&config.stake_table_entry))
                .collect())
        }
    }

    /// Get all eligible leaders of the committee for the current view
    fn committee_leaders(
        &self,
        view_number: <TYPES as NodeType>::View,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<BTreeSet<<TYPES as NodeType>::SignatureKey>> {
        self.committee_members(view_number, epoch)
    }

    /// Get the stake table entry for a public key
    fn stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Option<PeerConfig<<TYPES as NodeType>::SignatureKey>> {
        if let Some(epoch) = epoch {
            let filter = self.make_quorum_filter(epoch);
            let actual_members: BTreeSet<_> = self
                .stake_table
                .iter()
                .enumerate()
                .filter(|(idx, _)| filter.contains(idx))
                .map(|(_, v)| TYPES::SignatureKey::public_key(&v.stake_table_entry))
                .collect();

            if actual_members.contains(pub_key) {
                // Only return the stake if it is above zero
                self.indexed_stake_table.get(pub_key).cloned()
            } else {
                // Skip members which aren't included based on the quorum filter
                None
            }
        } else {
            self.indexed_stake_table.get(pub_key).cloned()
        }
    }

    /// Get the da stake table entry for a public key
    fn da_stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Option<PeerConfig<<TYPES as NodeType>::SignatureKey>> {
        if let Some(epoch) = epoch {
            let filter = self.make_da_quorum_filter(epoch);
            let actual_members: BTreeSet<_> = self
                .da_stake_table
                .iter()
                .enumerate()
                .filter(|(idx, _)| filter.contains(idx))
                .map(|(_, v)| TYPES::SignatureKey::public_key(&v.stake_table_entry))
                .collect();

            if actual_members.contains(pub_key) {
                // Only return the stake if it is above zero
                self.indexed_da_stake_table.get(pub_key).cloned()
            } else {
                // Skip members which aren't included based on the quorum filter
                None
            }
        } else {
            self.indexed_da_stake_table.get(pub_key).cloned()
        }
    }

    /// Check if a node has stake in the committee
    fn has_stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<bool> {
        if let Some(epoch) = epoch {
            let filter = self.make_quorum_filter(epoch);
            let actual_members: BTreeSet<_> = self
                .stake_table
                .iter()
                .enumerate()
                .filter(|(idx, _)| filter.contains(idx))
                .map(|(_, v)| TYPES::SignatureKey::public_key(&v.stake_table_entry))
                .collect();

            if actual_members.contains(pub_key) {
                Ok(self
                    .indexed_stake_table
                    .get(pub_key)
                    .context(hotshot_utils::anytrace::error!(
                        "No stake found for public key"
                    ))?
                    .stake_table_entry
                    .stake()
                    > U256::zero())
            } else {
                // Skip members which aren't included based on the quorum filter
                Ok(false)
            }
        } else {
            Ok(self
                .indexed_stake_table
                .get(pub_key)
                .context(hotshot_utils::anytrace::error!(
                    "No stake found for public key"
                ))?
                .stake_table_entry
                .stake()
                > U256::zero())
        }
    }

    /// Check if a node has stake in the committee
    fn has_da_stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<bool> {
        if let Some(epoch) = epoch {
            let filter = self.make_da_quorum_filter(epoch);
            let actual_members: BTreeSet<_> = self
                .da_stake_table
                .iter()
                .enumerate()
                .filter(|(idx, _)| filter.contains(idx))
                .map(|(_, v)| TYPES::SignatureKey::public_key(&v.stake_table_entry))
                .collect();

            if actual_members.contains(pub_key) {
                Ok(self
                    .indexed_da_stake_table
                    .get(pub_key)
                    .context(hotshot_utils::anytrace::error!(
                        "not found in indexed_da_stake_table"
                    ))?
                    .stake_table_entry
                    .stake()
                    > U256::zero())
            } else {
                // Skip members which aren't included based on the quorum filter
                Ok(false)
            }
        } else {
            Ok(self
                .indexed_da_stake_table
                .get(pub_key)
                .context(hotshot_utils::anytrace::error!(
                    "not found in indexed_da_stake_table"
                ))?
                .stake_table_entry
                .stake()
                > U256::zero())
        }
    }

    /// Index the vector of public keys with the current view number
    fn lookup_leader(
        &self,
        view_number: <TYPES as NodeType>::View,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<TYPES::SignatureKey> {
        if let Some(epoch) = epoch {
            let filter = self.make_quorum_filter(epoch);
            let leader_vec: Vec<_> = self
                .stake_table
                .iter()
                .enumerate()
                .filter(|(idx, _)| filter.contains(idx))
                .map(|(_, v)| v.clone())
                .collect();

            let mut rng: StdRng = rand::SeedableRng::seed_from_u64(*view_number);

            let randomized_view_number: u64 = rng.gen_range(0..=u64::MAX);
            #[allow(clippy::cast_possible_truncation)]
            let index = randomized_view_number as usize % leader_vec.len();

            let res = leader_vec[index].clone();

            Ok(TYPES::SignatureKey::public_key(&res.stake_table_entry))
        } else {
            let mut rng: StdRng = rand::SeedableRng::seed_from_u64(*view_number);

            let randomized_view_number: u64 = rng.gen_range(0..=u64::MAX);
            #[allow(clippy::cast_possible_truncation)]
            let index = randomized_view_number as usize % self.eligible_leaders.len();

            let res = self.eligible_leaders[index].clone();

            Ok(TYPES::SignatureKey::public_key(&res.stake_table_entry))
        }
    }

    /// Get the total number of nodes in the committee
    fn total_nodes(&self, epoch: Option<<TYPES as NodeType>::Epoch>) -> Result<usize> {
        if let Some(epoch) = epoch {
            Ok(self.make_quorum_filter(epoch).len())
        } else {
            Ok(self.stake_table.len())
        }
    }

    /// Get the total number of nodes in the committee
    fn da_total_nodes(&self, epoch: Option<<TYPES as NodeType>::Epoch>) -> Result<usize> {
        if let Some(epoch) = epoch {
            Ok(self.make_da_quorum_filter(epoch).len())
        } else {
            Ok(self.da_stake_table.len())
        }
    }

    /// Get the voting success threshold for the committee
    fn success_threshold(&self, epoch: Option<<TYPES as NodeType>::Epoch>) -> Result<NonZeroU64> {
        let len = self.total_nodes(epoch)?;
        NonZeroU64::new(((len as u64 * 2) / 3) + 1).context(error!("success_threshold is 0"))
    }

    /// Get the voting success threshold for the committee
    fn da_success_threshold(
        &self,
        epoch: Option<<TYPES as NodeType>::Epoch>,
    ) -> Result<NonZeroU64> {
        let len = self.da_total_nodes(epoch)?;
        NonZeroU64::new(((len as u64 * 2) / 3) + 1).context(error!("da_success_threshold is 0"))
    }

    /// Get the voting failure threshold for the committee
    fn failure_threshold(&self, epoch: Option<<TYPES as NodeType>::Epoch>) -> Result<NonZeroU64> {
        let len = self.total_nodes(epoch)?;
        NonZeroU64::new(((len as u64) / 3) + 1).context(error!("failure_threshold is 0"))
    }

    /// Get the voting upgrade threshold for the committee
    fn upgrade_threshold(&self, epoch: Option<<TYPES as NodeType>::Epoch>) -> Result<NonZeroU64> {
        let len = self.total_nodes(epoch)?;
        NonZeroU64::new(max((len as u64 * 9) / 10, ((len as u64 * 2) / 3) + 1))
            .context(error!("upgrade_threshold is 0"))
    }

    fn add_drb_result(&mut self, _epoch: <TYPES as NodeType>::Epoch, _drb_result: DrbResult) {}
}
