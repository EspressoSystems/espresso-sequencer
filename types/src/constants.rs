//! configurable constants for hotshot

use vbs::version::StaticVersion;

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

/// Type for semver representation of "Base" version
pub type BaseVersion = StaticVersion<0, 1>;
/// Constant for semver representation of "Base" version
pub const BASE_VERSION: BaseVersion = StaticVersion {};

/// Type for semver representation of "Marketplace" version
pub type MarketplaceVersion = StaticVersion<0, 3>;
/// Constant for semver representation of "Marketplace" version
pub const MARKETPLACE_VERSION: MarketplaceVersion = StaticVersion {};

/// The offset for how far in the future we will send out a `QuorumProposal` with an `UpgradeCertificate` we form. This is also how far in advance of sending a `QuorumProposal` we begin collecting votes on an `UpgradeProposal`.
pub const UPGRADE_PROPOSE_OFFSET: u64 = 5;

/// The offset for how far in the future the upgrade certificate we attach should be decided on (or else discarded).
pub const UPGRADE_DECIDE_BY_OFFSET: u64 = UPGRADE_PROPOSE_OFFSET + 5;

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
