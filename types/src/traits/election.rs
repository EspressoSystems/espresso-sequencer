// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! The election trait, used to decide which node is the leader and determine if a vote is valid.
use std::{collections::BTreeSet, fmt::Debug, hash::Hash, num::NonZeroU64};

use super::{network::Topic, node_implementation::NodeType};
use crate::{traits::signature_key::SignatureKey, PeerConfig};

/// A protocol for determining membership in and participating in a committee.
pub trait Membership<TYPES: NodeType>:
    Clone + Debug + Eq + PartialEq + Send + Sync + Hash + 'static
{
    /// Create a committee
    fn new(
        // Note: eligible_leaders is currently a hack because the DA leader == the quorum leader
        // but they should not have voting power.
        eligible_leaders: Vec<PeerConfig<TYPES::SignatureKey>>,
        committee_members: Vec<PeerConfig<TYPES::SignatureKey>>,
        committee_topic: Topic,
        #[cfg(feature = "fixed-leader-election")] fixed_leader_for_gpuvid: usize,
    ) -> Self;

    /// Get all participants in the committee (including their stake)
    fn stake_table(&self) -> Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>;

    /// Get all participants in the committee for a specific view
    fn committee_members(&self, view_number: TYPES::Time) -> BTreeSet<TYPES::SignatureKey>;

    /// Get the stake table entry for a public key, returns `None` if the
    /// key is not in the table
    fn stake(
        &self,
        pub_key: &TYPES::SignatureKey,
    ) -> Option<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>;

    /// See if a node has stake in the committee
    fn has_stake(&self, pub_key: &TYPES::SignatureKey) -> bool;

    /// The leader of the committee for view `view_number`.
    fn leader(&self, view_number: TYPES::Time) -> TYPES::SignatureKey;

    /// Get the network topic for the committee
    fn committee_topic(&self) -> Topic;

    /// Returns the number of total nodes in the committee
    fn total_nodes(&self) -> usize;

    /// Returns the threshold for a specific `Membership` implementation
    fn success_threshold(&self) -> NonZeroU64;

    /// Returns the threshold for a specific `Membership` implementation
    fn failure_threshold(&self) -> NonZeroU64;

    /// Returns the threshold required to upgrade the network protocol
    fn upgrade_threshold(&self) -> NonZeroU64;
}
