//! configurable constants for hotshot

use vbs::version::{StaticVersion, Version};

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

/// CONSTANT for protocol major version
pub const VERSION_MAJ: u16 = 0;

/// CONSTANT for protocol major version
pub const VERSION_MIN: u16 = 1;

/// Constant for protocol version 0.1.
pub const VERSION_0_1: Version = Version {
    major: VERSION_MAJ,
    minor: VERSION_MIN,
};

/// Constant for the base protocol version in this instance of HotShot.
pub const BASE_VERSION: Version = VERSION_0_1;

/// Type for protocol static version 0.1.
pub type Version01 = StaticVersion<VERSION_MAJ, VERSION_MIN>;

/// Constant for protocol static version 0.1.
pub const STATIC_VER_0_1: Version01 = StaticVersion {};

/// Default Channel Size for consensus event sharing
pub const EVENT_CHANNEL_SIZE: usize = 100_000;

/// Constants for `WebServerNetwork` and `WebServer`
/// The Web CDN is not, strictly speaking, bound to the network; it can have its own versioning.
/// Web Server CDN Version (major)
pub const WEB_SERVER_MAJOR_VERSION: u16 = 0;
/// Web Server CDN Version (minor)
pub const WEB_SERVER_MINOR_VERSION: u16 = 1;

/// Type for Web Server CDN Version
pub type WebServerVersion = StaticVersion<WEB_SERVER_MAJOR_VERSION, WEB_SERVER_MINOR_VERSION>;

/// Constant for Web Server CDN Version
pub const WEB_SERVER_VERSION: WebServerVersion = StaticVersion {};

/// For `STAKE_TABLE_CAPACITY=200`, the light client prover (a.k.a. `hotshot-state-prover`)
/// would need to generate proof for a circuit of slightly below 2^20 gates.
/// Thus we need to support this upperbounded degree in our Structured Reference String (SRS),
/// the `+2` is just an artifact from the jellyfish's Plonk proof system.
#[allow(clippy::cast_possible_truncation)]
pub const SRS_DEGREE: usize = 2u64.pow(20) as usize + 2;
