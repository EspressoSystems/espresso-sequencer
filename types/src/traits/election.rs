//! The election trait, used to decide which node is the leader and determine if a vote is valid.

// Needed to avoid the non-binding `let` warning.
#![allow(clippy::let_underscore_untyped)]

use super::node_implementation::NodeType;

use crate::{traits::signature_key::SignatureKey, PeerConfig};

use snafu::Snafu;
use std::{collections::BTreeSet, fmt::Debug, hash::Hash, num::NonZeroU64};

/// Error for election problems
#[derive(Snafu, Debug)]
pub enum ElectionError {
    /// stub error to be filled in
    StubError,
    /// Math error doing something
    /// NOTE: it would be better to make Election polymorphic over
    /// the election error and then have specific math errors
    MathError,
}

/// election config
pub trait ElectionConfig:
    Default
    + Clone
    + serde::Serialize
    + for<'de> serde::Deserialize<'de>
    + Sync
    + Send
    + core::fmt::Debug
{
}

/// A protocol for determining membership in and participating in a committee.
pub trait Membership<TYPES: NodeType>:
    Clone + Debug + Eq + PartialEq + Send + Sync + Hash + 'static
{
    /// generate a default election configuration
    fn default_election_config(
        num_nodes_with_stake: u64,
        num_nodes_without_stake: u64,
    ) -> TYPES::ElectionConfigType;

    /// create an election
    /// TODO may want to move this to a testableelection trait
    fn create_election(
        entries: Vec<PeerConfig<TYPES::SignatureKey>>,
        config: TYPES::ElectionConfigType,
        fixed_leader_for_gpuvid: usize,
    ) -> Self;

    /// Clone the public key and corresponding stake table for current elected committee
    fn get_committee_qc_stake_table(
        &self,
    ) -> Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>;

    /// The leader of the committee for view `view_number`.
    fn get_leader(&self, view_number: TYPES::Time) -> TYPES::SignatureKey;

    /// The staked members of the committee for view `view_number`.
    fn get_staked_committee(&self, view_number: TYPES::Time) -> BTreeSet<TYPES::SignatureKey>;

    /// The non-staked members of the committee for view `view_number`.
    fn get_non_staked_committee(&self, view_number: TYPES::Time) -> BTreeSet<TYPES::SignatureKey>;

    /// Get whole (staked + non-staked) committee for view `view_number`.
    fn get_whole_committee(&self, view_number: TYPES::Time) -> BTreeSet<TYPES::SignatureKey>;

    /// Check if a key has stake
    fn has_stake(&self, pub_key: &TYPES::SignatureKey) -> bool;

    /// Get the stake table entry for a public key, returns `None` if the
    /// key is not in the table
    fn get_stake(
        &self,
        pub_key: &TYPES::SignatureKey,
    ) -> Option<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>;

    /// Returns the number of total nodes in the committee
    fn total_nodes(&self) -> usize;

    /// Returns the threshold for a specific `Membership` implementation
    fn success_threshold(&self) -> NonZeroU64;

    /// Returns the threshold for a specific `Membership` implementation
    fn failure_threshold(&self) -> NonZeroU64;

    /// Returns the threshold required to upgrade the network protocol
    fn upgrade_threshold(&self) -> NonZeroU64;
}
