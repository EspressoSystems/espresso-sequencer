//! configurable constants for hotshot

/// the ID of the genesis block proposer
pub const GENESIS_PROPOSER_ID: [u8; 2] = [4, 2];

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

/// the amount of time to wait for async_std tests to spin down the Libp2p listeners
/// and allow future tests to run
pub const ASYNC_STD_LIBP2P_LISTENER_SPINDOWN_TIME: u64 = 4;
