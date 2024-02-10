//! configurable constants for hotshot

use serde::{Deserialize, Serialize};

/// the number of views to gather information for ahead of time
pub const LOOK_AHEAD: u64 = 5;

/// the default kademlia record republication interval (in seconds)
pub const KAD_DEFAULT_REPUB_INTERVAL_SEC: u64 = 28800;

/// the number of messages to cache in the combined network
pub const COMBINED_NETWORK_CACHE_SIZE: usize = 1000;

/// the number of messages to attempt to send over the primary network before switching to prefer the secondary network
pub const COMBINED_NETWORK_MIN_PRIMARY_FAILURES: u64 = 5;

/// the number of messages to send over the secondary network before re-attempting the (presumed down) primary network
pub const COMBINED_NETWORK_PRIMARY_CHECK_INTERVAL: u64 = 5;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Hash, Eq)]
/// Type for protocol version number
pub struct Version {
    /// major version number
    pub major: u16,
    /// minor version number
    pub minor: u16,
}

/// Constant for protocol version 0.1.
pub const VERSION_0_1: Version = Version { major: 0, minor: 1 };

/// Default Channel Size for consensus event sharing
pub const EVENT_CHANNEL_SIZE: usize = 100_000;
