// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! The election trait, used to decide which node is the leader and determine if a vote is valid.
use std::{collections::BTreeSet, fmt::Debug, num::NonZeroU64};

use sha2::{Digest, Sha256, Sha512};

use crate::traits::signature_key::StakeTableEntryType;
use async_trait::async_trait;
use hotshot_utils::anytrace::Result;
use primitive_types::{U256, U512};

use super::node_implementation::NodeType;
use crate::{traits::signature_key::SignatureKey, PeerConfig};

#[async_trait]
/// A protocol for determining membership in and participating in a committee.
pub trait Membership<TYPES: NodeType>: Debug + Send + Sync {
    /// The error type returned by methods like `lookup_leader`.
    type Error: std::fmt::Display;

    /// Create a committee
    fn new(
        // Note: eligible_leaders is currently a hack because the DA leader == the quorum leader
        // but they should not have voting power.
        stake_committee_members: Vec<PeerConfig<TYPES::SignatureKey>>,
        da_committee_members: Vec<PeerConfig<TYPES::SignatureKey>>,
    ) -> Self;

    /// Get all participants in the committee (including their stake) for a specific epoch
    fn stake_table(
        &self,
        epoch: Option<TYPES::Epoch>,
    ) -> Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>;

    /// Get all participants in the committee (including their stake) for a specific epoch
    fn da_stake_table(
        &self,
        epoch: Option<TYPES::Epoch>,
    ) -> Vec<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>;

    /// Get all participants in the committee for a specific view for a specific epoch
    fn committee_members(
        &self,
        view_number: TYPES::View,
        epoch: Option<TYPES::Epoch>,
    ) -> BTreeSet<TYPES::SignatureKey>;

    /// Get all participants in the committee for a specific view for a specific epoch
    fn da_committee_members(
        &self,
        view_number: TYPES::View,
        epoch: Option<TYPES::Epoch>,
    ) -> BTreeSet<TYPES::SignatureKey>;

    /// Get all leaders in the committee for a specific view for a specific epoch
    fn committee_leaders(
        &self,
        view_number: TYPES::View,
        epoch: Option<TYPES::Epoch>,
    ) -> BTreeSet<TYPES::SignatureKey>;

    /// Get the stake table entry for a public key, returns `None` if the
    /// key is not in the table for a specific epoch
    fn stake(
        &self,
        pub_key: &TYPES::SignatureKey,
        epoch: Option<TYPES::Epoch>,
    ) -> Option<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>;

    /// Get the DA stake table entry for a public key, returns `None` if the
    /// key is not in the table for a specific epoch
    fn da_stake(
        &self,
        pub_key: &TYPES::SignatureKey,
        epoch: Option<TYPES::Epoch>,
    ) -> Option<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry>;

    /// See if a node has stake in the committee in a specific epoch
    fn has_stake(&self, pub_key: &TYPES::SignatureKey, epoch: Option<TYPES::Epoch>) -> bool;

    /// See if a node has stake in the committee in a specific epoch
    fn has_da_stake(&self, pub_key: &TYPES::SignatureKey, epoch: Option<TYPES::Epoch>) -> bool;

    /// The leader of the committee for view `view_number` in `epoch`.
    ///
    /// Note: this function uses a HotShot-internal error type.
    /// You should implement `lookup_leader`, rather than implementing this function directly.
    ///
    /// # Errors
    /// Returns an error if the leader cannot be calculated.
    fn leader(
        &self,
        view: TYPES::View,
        epoch: Option<TYPES::Epoch>,
    ) -> Result<TYPES::SignatureKey> {
        use hotshot_utils::anytrace::*;

        self.lookup_leader(view, epoch).wrap().context(info!(
            "Failed to get leader for view {view} in epoch {epoch}"
        ))
    }

    /// The leader of the committee for view `view_number` in `epoch`.
    ///
    /// Note: There is no such thing as a DA leader, so any consumer
    /// requiring a leader should call this.
    ///
    /// # Errors
    /// Returns an error if the leader cannot be calculated
    fn lookup_leader(
        &self,
        view: TYPES::View,
        epoch: Option<TYPES::Epoch>,
    ) -> std::result::Result<TYPES::SignatureKey, Self::Error>;

    /// Returns the number of total nodes in the committee in an epoch `epoch`
    fn total_nodes(&self, epoch: Option<TYPES::Epoch>) -> usize;

    /// Returns the number of total DA nodes in the committee in an epoch `epoch`
    fn da_total_nodes(&self, epoch: Option<TYPES::Epoch>) -> usize;

    /// Returns the threshold for a specific `Membership` implementation
    fn success_threshold(&self, epoch: Option<TYPES::Epoch>) -> NonZeroU64;

    /// Returns the DA threshold for a specific `Membership` implementation
    fn da_success_threshold(&self, epoch: Option<TYPES::Epoch>) -> NonZeroU64;

    /// Returns the threshold for a specific `Membership` implementation
    fn failure_threshold(&self, epoch: Option<TYPES::Epoch>) -> NonZeroU64;

    /// Returns the threshold required to upgrade the network protocol
    fn upgrade_threshold(&self, epoch: Option<TYPES::Epoch>) -> NonZeroU64;

    #[allow(clippy::type_complexity)]
    /// Handles notifications that a new epoch root has been created
    /// Is called under a read lock to the Membership. Return a callback
    /// with Some to have that callback invoked under a write lock.
    ///
    /// #3967 REVIEW NOTE: this is only called if epoch is Some. Is there any reason to do otherwise?
    async fn add_epoch_root(
        &self,
        _epoch: TYPES::Epoch,
        _block_header: TYPES::BlockHeader,
    ) -> Option<Box<dyn FnOnce(&mut Self) + Send>> {
        None
    }

    #[allow(clippy::type_complexity)]
    /// Called after add_epoch_root runs and any callback has been invoked.
    /// Causes a read lock to be reacquired for this functionality.
    async fn sync_l1(&self) -> Option<Box<dyn FnOnce(&mut Self) + Send>> {
        None
    }
}

/// Calculate `xor(drb.cycle(), public_key)`, returning the result as a vector of bytes
fn cyclic_xor(drb: [u8; 32], public_key: Vec<u8>) -> Vec<u8> {
    let drb: Vec<u8> = drb.to_vec();

    let mut result: Vec<u8> = vec![];

    for (drb_byte, public_key_byte) in public_key.iter().zip(drb.iter().cycle()) {
        result.push(drb_byte ^ public_key_byte);
    }

    result
}

/// Generate the stake table CDF, as well as a hash of the resulting stake table
pub fn generate_stake_cdf<Key: SignatureKey, Entry: StakeTableEntryType<Key>>(
    mut stake_table: Vec<Entry>,
    drb: [u8; 32],
) -> (Vec<(Entry, U256)>, [u8; 32]) {
    // sort by xor(public_key, drb_result)
    stake_table.sort_by(|a, b| {
        cyclic_xor(drb, a.public_key().to_bytes()).cmp(&cyclic_xor(drb, b.public_key().to_bytes()))
    });

    let mut hasher = Sha256::new();

    let mut cumulative_stake = U256::from(0);
    let mut cdf = vec![];

    for entry in stake_table {
        cumulative_stake += entry.stake();
        hasher.update(entry.public_key().to_bytes());

        cdf.push((entry, cumulative_stake));
    }

    (cdf, hasher.finalize().into())
}

/// select the leader for a view
///
/// # Panics
/// Panics if the cdf is empty
pub fn select_randomized_leader<SignatureKey, Entry: StakeTableEntryType<SignatureKey> + Clone>(
    cdf: Vec<(Entry, U256)>,
    stake_table_hash: [u8; 32],
    drb: [u8; 32],
    view: u64,
) -> Entry {
    // We hash the concatenated drb, view and stake table hash.
    let mut hasher = Sha512::new();
    hasher.update(drb);
    hasher.update(view.to_le_bytes());
    hasher.update(stake_table_hash);
    let raw_breakpoint: [u8; 64] = hasher.finalize().into();

    // then calculate the remainder modulo the total stake as a U512
    let remainder: U512 =
        U512::from_little_endian(&raw_breakpoint) % U512::from(cdf.last().unwrap().1);

    // and drop the top 32 bytes, downcasting to a U256
    let breakpoint: U256 = {
        let mut result = [0u8; 64];

        remainder.to_little_endian(&mut result);

        U256::from_little_endian(&result[32..64])
    };

    // now find the first index where the breakpoint is strictly smaller than the cdf
    let index = cdf.partition_point(|(_, cumulative_stake)| breakpoint < *cumulative_stake);

    // and return the corresponding entry
    cdf[index].0.clone()
}
