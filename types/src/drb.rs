// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::hash::{DefaultHasher, Hash, Hasher};

use sha2::{Digest, Sha256};

use crate::traits::{node_implementation::NodeType, signature_key::SignatureKey};

// TODO: Add the following consts once we bench the hash time.
// <https://github.com/EspressoSystems/HotShot/issues/3880>
// /// Highest number of hashes that a hardware can complete in a second.
// const `HASHES_PER_SECOND`
// /// Time a DRB calculation will take, in terms of number of views.
// const `DRB_CALCULATION_NUM_VIEW`: u64 = 300;

// TODO: Replace this with an accurate number calculated by `fn difficulty_level()` once we bench
// the hash time.
// <https://github.com/EspressoSystems/HotShot/issues/3880>
/// Arbitrary number of times the hash function will be repeatedly called.
const DIFFICULTY_LEVEL: u64 = 10;

/// DRB seed input for epoch 1 and 2.
pub const INITIAL_DRB_SEED_INPUT: [u8; 32] = [0; 32];
/// DRB result for epoch 1 and 2.
pub const INITIAL_DRB_RESULT: [u8; 32] = [0; 32];

/// Alias for DRB seed input for `compute_drb_result`, serialized from the QC signature.
pub type DrbSeedInput = [u8; 32];

/// Alias for DRB result from `compute_drb_result`.
pub type DrbResult = [u8; 32];

// TODO: Use `HASHES_PER_SECOND` * `VIEW_TIMEOUT` * `DRB_CALCULATION_NUM_VIEW` to calculate this
// once we bench the hash time.
// <https://github.com/EspressoSystems/HotShot/issues/3880>
/// Difficulty level of the DRB calculation.
///
/// Represents the number of times the hash function will be repeatedly called.
#[must_use]
pub fn difficulty_level() -> u64 {
    unimplemented!("Use an arbitrary `DIFFICULTY_LEVEL` for now before we bench the hash time.");
}

/// Compute the DRB result for the leader rotation.
///
/// This is to be started two epochs in advance and spawned in a non-blocking thread.
///
/// # Arguments
/// * `drb_seed_input` - Serialized QC signature.
#[must_use]
pub fn compute_drb_result<TYPES: NodeType>(drb_seed_input: DrbSeedInput) -> DrbResult {
    let mut hash = drb_seed_input.to_vec();
    for _iter in 0..DIFFICULTY_LEVEL {
        // TODO: This may be optimized to avoid memcopies after we bench the hash time.
        // <https://github.com/EspressoSystems/HotShot/issues/3880>
        hash = Sha256::digest(hash).to_vec();
    }

    // Convert the hash to the DRB result.
    let mut drb_result = [0u8; 32];
    drb_result.copy_from_slice(&hash);
    drb_result
}

/// Use the DRB result to get the leader.
///
/// The DRB result is the output of a spawned `compute_drb_result` call.
#[must_use]
pub fn leader<TYPES: NodeType>(
    view_number: usize,
    stake_table: &[<TYPES::SignatureKey as SignatureKey>::StakeTableEntry],
    drb_result: DrbResult,
) -> TYPES::SignatureKey {
    let mut hasher = DefaultHasher::new();
    drb_result.hash(&mut hasher);
    view_number.hash(&mut hasher);
    #[allow(clippy::cast_possible_truncation)]
    // TODO: Use the total stake rather than `len()` and update the indexing after switching to
    // a weighted stake table.
    // <https://github.com/EspressoSystems/HotShot/issues/3898>
    let index = (hasher.finish() as usize) % stake_table.len();
    let entry = stake_table[index].clone();
    TYPES::SignatureKey::public_key(&entry)
}
