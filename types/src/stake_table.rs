//! Types and structs related to the stake table

use ethereum_types::U256;
use serde::{Deserialize, Serialize};

use crate::traits::signature_key::{SignatureKey, StakeTableEntryType};

/// Stake table entry
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash, Eq)]
#[serde(bound(deserialize = ""))]
pub struct StakeTableEntry<K: SignatureKey> {
    /// The public key
    pub stake_key: K,
    /// The associated stake amount
    pub stake_amount: U256,
}

impl<K: SignatureKey> StakeTableEntryType for StakeTableEntry<K> {
    /// Get the stake amount
    fn stake(&self) -> U256 {
        self.stake_amount
    }
}

impl<K: SignatureKey> StakeTableEntry<K> {
    /// Get the public key
    pub fn key(&self) -> &K {
        &self.stake_key
    }
}

// TODO(Chengyu): add stake table snapshot here
