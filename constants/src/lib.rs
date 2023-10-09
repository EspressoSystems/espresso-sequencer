//! configurable constants for hotshot

/// the ID of the genesis block proposer
pub const GENESIS_PROPOSER_ID: [u8; 2] = [4, 2];

/// the number of views to gather information for ahead of time
pub const LOOK_AHEAD: u64 = 5;

/// the default kademlia record republication interval (in seconds)
pub const KAD_DEFAULT_REPUB_INTERVAL_SEC: u64 = 28800;
