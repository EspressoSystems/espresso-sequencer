//! A vector based stake table implementation. The commitment is the rescue hash of the list of (key, amount) pairs;

use ark_std::{collections::HashMap, hash::Hash, rand::SeedableRng};
use digest::crypto_common::rand_core::CryptoRngCore;
use ethereum_types::{U256, U512};
use hotshot_types::traits::stake_table::{SnapshotVersion, StakeTableError, StakeTableScheme};
use jf_primitives::rescue::{sponge::RescueCRHF, RescueParameter};
use serde::{Deserialize, Serialize};

use crate::utils::{u256_to_field, ToFields};

/// Locally maintained stake table, generic over public key type `K`.
/// Whose commitment is a rescue hash over field `F`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StakeTable<K: Eq + Hash + Clone + ToFields<F>, F: RescueParameter> {
    /// The most up-to-date stake table, where the incoming transactions shall be performed on.
    head: Vec<(K, U256)>,
    /// The snapshot of stake table at the beginning of the current epoch
    epoch_start: Vec<(K, U256)>,
    /// The stake table used for leader election.
    last_epoch_start: Vec<(K, U256)>,

    /// Total stakes for different versions
    head_total_stake: U256,
    epoch_start_total_stake: U256,
    last_epoch_start_total_stake: U256,

    /// Commitment for finalized versions
    epoch_start_comm: F,
    last_epoch_start_comm: F,

    /// The mapping from public keys to their location in the Merkle tree.
    #[serde(skip)]
    mapping: HashMap<K, usize>,
}

impl<K, F> StakeTableScheme for StakeTable<K, F>
where
    K: Eq + Hash + Clone + ToFields<F>,
    F: RescueParameter,
{
    type Key = K;
    type Amount = U256;
    type Commitment = F;
    type LookupProof = ();
    type IntoIter = <Vec<(K, U256)> as ark_std::iter::IntoIterator>::IntoIter;
    // type IntoIter = ark_std::slice::Iter<'a, &'a (K, U256)>;

    fn register(
        &mut self,
        new_key: Self::Key,
        amount: Self::Amount,
    ) -> Result<(), StakeTableError> {
        match self.mapping.get(&new_key) {
            Some(_) => Err(StakeTableError::ExistingKey),
            None => {
                let pos = self.mapping.len();
                self.head.push((new_key.clone(), amount));
                self.mapping.insert(new_key, pos);
                Ok(())
            }
        }
    }

    fn deregister(&mut self, existing_key: &Self::Key) -> Result<(), StakeTableError> {
        match self.mapping.get(existing_key) {
            Some(pos) => {
                self.head_total_stake -= self.head[*pos].1;
                self.head[*pos].1 = U256::zero();
                Ok(())
            }
            None => Err(StakeTableError::KeyNotFound),
        }
    }

    fn commitment(&self, version: SnapshotVersion) -> Result<Self::Commitment, StakeTableError> {
        match version {
            // IMPORTANT: we don't support committing the head version b/c it's not finalized.
            SnapshotVersion::Head => Err(StakeTableError::SnapshotUnsupported),
            SnapshotVersion::EpochStart => Ok(self.epoch_start_comm),
            SnapshotVersion::LastEpochStart => Ok(self.last_epoch_start_comm),
            SnapshotVersion::BlockNum(_) => Err(StakeTableError::SnapshotUnsupported),
        }
    }

    fn total_stake(&self, version: SnapshotVersion) -> Result<Self::Amount, StakeTableError> {
        match version {
            SnapshotVersion::Head => Ok(self.head_total_stake),
            SnapshotVersion::EpochStart => Ok(self.epoch_start_total_stake),
            SnapshotVersion::LastEpochStart => Ok(self.last_epoch_start_total_stake),
            SnapshotVersion::BlockNum(_) => Err(StakeTableError::SnapshotUnsupported),
        }
    }

    fn len(&self, version: SnapshotVersion) -> Result<usize, StakeTableError> {
        match version {
            SnapshotVersion::Head => Ok(self.head.len()),
            SnapshotVersion::EpochStart => Ok(self.epoch_start.len()),
            SnapshotVersion::LastEpochStart => Ok(self.last_epoch_start.len()),
            SnapshotVersion::BlockNum(_) => Err(StakeTableError::SnapshotUnsupported),
        }
    }

    fn contains_key(&self, key: &Self::Key) -> bool {
        self.mapping.contains_key(key)
    }

    fn lookup(
        &self,
        version: SnapshotVersion,
        key: &Self::Key,
    ) -> Result<(Self::Amount, Self::LookupProof), StakeTableError> {
        match self.mapping.get(key) {
            Some(&pos) => match version {
                SnapshotVersion::Head => {
                    if pos >= self.head.len() {
                        Err(StakeTableError::KeyNotFound)
                    } else {
                        Ok((self.head[pos].1, ()))
                    }
                }
                SnapshotVersion::EpochStart => {
                    if pos >= self.epoch_start.len() {
                        Err(StakeTableError::KeyNotFound)
                    } else {
                        Ok((self.epoch_start[pos].1, ()))
                    }
                }
                SnapshotVersion::LastEpochStart => {
                    if pos >= self.last_epoch_start.len() {
                        Err(StakeTableError::KeyNotFound)
                    } else {
                        Ok((self.last_epoch_start[pos].1, ()))
                    }
                }
                SnapshotVersion::BlockNum(_) => Err(StakeTableError::SnapshotUnsupported),
            },
            None => Err(StakeTableError::KeyNotFound),
        }
    }

    fn simple_lookup(
        &self,
        version: SnapshotVersion,
        key: &Self::Key,
    ) -> Result<Self::Amount, StakeTableError> {
        match self.mapping.get(key) {
            Some(&pos) => match version {
                SnapshotVersion::Head => {
                    if pos >= self.head.len() {
                        Err(StakeTableError::KeyNotFound)
                    } else {
                        Ok(self.head[pos].1)
                    }
                }
                SnapshotVersion::EpochStart => {
                    if pos >= self.epoch_start.len() {
                        Err(StakeTableError::KeyNotFound)
                    } else {
                        Ok(self.epoch_start[pos].1)
                    }
                }
                SnapshotVersion::LastEpochStart => {
                    if pos >= self.last_epoch_start.len() {
                        Err(StakeTableError::KeyNotFound)
                    } else {
                        Ok(self.last_epoch_start[pos].1)
                    }
                }
                SnapshotVersion::BlockNum(_) => Err(StakeTableError::SnapshotUnsupported),
            },
            None => Err(StakeTableError::KeyNotFound),
        }
    }

    fn update(
        &mut self,
        key: &Self::Key,
        delta: Self::Amount,
        negative: bool,
    ) -> Result<Self::Amount, StakeTableError> {
        match self.mapping.get(key) {
            Some(&pos) => {
                let old_amount = self.head[pos].1;
                if negative {
                    if delta > self.head[pos].1 {
                        return Err(StakeTableError::InsufficientFund);
                    }
                    self.head_total_stake -= delta;
                    self.head[pos].1 -= delta;
                }
                self.head_total_stake += delta;
                self.head[pos].1 += delta;
                Ok(old_amount)
            }
            None => Err(StakeTableError::KeyNotFound),
        }
    }

    fn sample(
        &self,
        rng: &mut (impl SeedableRng + CryptoRngCore),
    ) -> Option<(&Self::Key, &Self::Amount)> {
        let mut bytes = [0u8; 64];
        rng.fill_bytes(&mut bytes);
        let r = U512::from_big_endian(&bytes);
        let m = U512::from(self.last_epoch_start_total_stake);
        let mut pos: U256 = (r % m).try_into().unwrap(); // won't fail
        let idx = 0;
        while pos > self.last_epoch_start[idx].1 {
            pos -= self.last_epoch_start[idx].1;
        }
        Some((&self.last_epoch_start[idx].0, &self.last_epoch_start[idx].1))
    }

    fn try_iter(&self, version: SnapshotVersion) -> Result<Self::IntoIter, StakeTableError> {
        match version {
            SnapshotVersion::Head => Ok(self.head.clone().into_iter()),
            SnapshotVersion::EpochStart => Ok(self.epoch_start.clone().into_iter()),
            SnapshotVersion::LastEpochStart => Ok(self.last_epoch_start.clone().into_iter()),
            SnapshotVersion::BlockNum(_) => Err(StakeTableError::SnapshotUnsupported),
        }
    }
}

impl<K, F> StakeTable<K, F>
where
    K: Eq + Hash + Clone + ToFields<F>,
    F: RescueParameter,
{
    /// Initiating an empty stake table.
    /// Overall capacity is `TREE_BRANCH.pow(height)`.
    pub fn new() -> Self {
        let comm = RescueCRHF::sponge_with_zero_padding(&[], 1)[0];
        Self {
            head: vec![],
            epoch_start: vec![],
            last_epoch_start: vec![],
            head_total_stake: U256::zero(),
            epoch_start_total_stake: U256::zero(),
            last_epoch_start_total_stake: U256::zero(),
            mapping: HashMap::new(),
            epoch_start_comm: comm,
            last_epoch_start_comm: comm,
        }
    }

    /// Update the stake table when the epoch number advances, should be manually called.
    pub fn advance(&mut self) {
        // Could we avoid this `clone()`?
        self.last_epoch_start = self.epoch_start.clone();
        self.last_epoch_start_total_stake = self.epoch_start_total_stake;
        self.last_epoch_start_comm = self.epoch_start_comm;
        self.epoch_start = self.head.clone();
        self.epoch_start_total_stake = self.head_total_stake;
        self.epoch_start_comm = self.compute_head_comm();
    }

    /// Set the stake withheld by `key` to be `value`.
    /// Return the previous stake if succeed.
    pub fn set_value(&mut self, key: &K, value: U256) -> Result<U256, StakeTableError> {
        match self.mapping.get(key) {
            Some(pos) => {
                let old_value = self.head[*pos].1;
                self.head[*pos].1 = value;
                Ok(old_value)
            }
            None => Err(StakeTableError::KeyNotFound),
        }
    }

    /// Helper function to recompute the stake table commitment for head version
    fn compute_head_comm(&mut self) -> F {
        if self.head.is_empty() {
            return RescueCRHF::sponge_with_zero_padding(&[], 1)[0];
        }
        let mut to_be_hashed = vec![];
        self.head.iter().for_each(|(key, amount)| {
            to_be_hashed.extend(key.to_fields());
            to_be_hashed.push(u256_to_field(amount));
        });
        let mut comm = to_be_hashed[0];
        for i in (1..self.head.len()).step_by(2) {
            comm = RescueCRHF::sponge_with_zero_padding(
                &[
                    comm,
                    to_be_hashed[i],
                    if i + 1 < to_be_hashed.len() {
                        to_be_hashed[i + 1]
                    } else {
                        F::zero()
                    },
                ],
                1,
            )[0];
        }
        comm
    }
}

impl<K, F> Default for StakeTable<K, F>
where
    K: Eq + Hash + Clone + ToFields<F>,
    F: RescueParameter,
{
    fn default() -> Self {
        Self::new()
    }
}
