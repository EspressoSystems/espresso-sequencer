// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! configurable constants for hotshot

use std::time::Duration;

/// timeout for fetching auction results from the solver
pub const AUCTION_RESULTS_FETCH_TIMEOUT: Duration = Duration::from_millis(500);

/// timeout for fetching bundles from builders
pub const BUNDLE_FETCH_TIMEOUT: Duration = Duration::from_millis(500);

/// the number of views to gather information for ahead of time
pub const LOOK_AHEAD: u64 = 5;

/// the default kademlia record republication interval (in seconds)
pub const KAD_DEFAULT_REPUB_INTERVAL_SEC: u64 = 28800;

/// the number of messages to cache in the combined network
pub const COMBINED_NETWORK_CACHE_SIZE: usize = 1000;

/// the number of messages to attempt to send over the primary network before switching to prefer the secondary network
pub const COMBINED_NETWORK_MIN_PRIMARY_FAILURES: u64 = 5;

/// the number of messages to send over the secondary network without delay before re-attempting the (presumed down) primary network
pub const COMBINED_NETWORK_PRIMARY_CHECK_INTERVAL: u64 = 50;

/// the default delay duration value in milliseconds of sending on the secondary in the combined networks
pub const COMBINED_NETWORK_DELAY_DURATION: u64 = 5000;

/// The default network data request delay in milliseconds
pub const REQUEST_DATA_DELAY: u64 = 5000;

/// Default channel size for consensus event sharing
pub const EVENT_CHANNEL_SIZE: usize = 100_000;

/// Default channel size for HotShot -> application communication
pub const EXTERNAL_EVENT_CHANNEL_SIZE: usize = 100_000;

/// The offset for how far in the future we will send out a `QuorumProposal` with an `UpgradeCertificate` we form. This is also how far in advance of sending a `QuorumProposal` we begin collecting votes on an `UpgradeProposal`.
pub const UPGRADE_PROPOSE_OFFSET: u64 = 5;

#[cfg(test)]
/// The offset for how far in the future the upgrade certificate we attach should be decided on (or else discarded).
pub const UPGRADE_DECIDE_BY_OFFSET: u64 = UPGRADE_PROPOSE_OFFSET + 5;
#[cfg(not(test))]
/// The offset for how far in the future the upgrade certificate we attach should be decided on (or else discarded).
pub const UPGRADE_DECIDE_BY_OFFSET: u64 = UPGRADE_PROPOSE_OFFSET + 100;

/// The offset for how far in the future the upgrade actually begins.
pub const UPGRADE_BEGIN_OFFSET: u64 = UPGRADE_DECIDE_BY_OFFSET + 5;

/// The offset for how far in the future the upgrade ends.
pub const UPGRADE_FINISH_OFFSET: u64 = UPGRADE_BEGIN_OFFSET + 5;

/// For `STAKE_TABLE_CAPACITY=200`, the light client prover (a.k.a. `hotshot-state-prover`)
/// would need to generate proof for a circuit of slightly below 2^20 gates.
/// Thus we need to support this upperbounded degree in our Structured Reference String (SRS),
/// the `+2` is just an artifact from the jellyfish's Plonk proof system.
#[allow(clippy::cast_possible_truncation)]
pub const SRS_DEGREE: usize = 2u64.pow(20) as usize + 2;

/// The `tide` module name for the legacy builder
pub const LEGACY_BUILDER_MODULE: &str = "block_info";

/// The `tide` module name for the marketplace builder
pub const MARKETPLACE_BUILDER_MODULE: &str = "bundle_info";
