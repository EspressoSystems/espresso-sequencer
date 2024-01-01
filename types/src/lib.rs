//! Types and Traits for the `HotShot` consensus module
use displaydoc::Display;
use std::{num::NonZeroUsize, time::Duration};
use traits::{election::ElectionConfig, signature_key::SignatureKey};
pub mod consensus;
pub mod data;
pub mod error;
pub mod event;
pub mod light_client;
pub mod message;
pub mod simple_certificate;
pub mod simple_vote;
pub mod traits;
pub mod utils;
pub mod vote;

/// the type of consensus to run. Either:
/// wait for a signal to start a view,
/// or constantly run
/// you almost always want continuous
/// incremental is just for testing
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum ExecutionType {
    /// constantly increment view as soon as view finishes
    Continuous,
    /// wait for a signal
    Incremental,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Display)]
#[serde(bound(deserialize = ""))]
/// config for validator, including public key, private key, stake value
pub struct ValidatorConfig<KEY: SignatureKey> {
    /// The validator's public key and stake value
    pub public_key: KEY,
    /// The validator's private key, should be in the mempool, not public
    pub private_key: KEY::PrivateKey,
    /// The validator's stake
    pub stake_value: u64,
    /// the validator's key pairs for state signing/verification
    pub state_key_pair: light_client::StateKeyPair,
}

impl<KEY: SignatureKey> ValidatorConfig<KEY> {
    /// generate validator config from input seed, index and stake value
    #[must_use]
    pub fn generated_from_seed_indexed(seed: [u8; 32], index: u64, stake_value: u64) -> Self {
        let (public_key, private_key) = KEY::generated_from_seed_indexed(seed, index);
        let state_key_pairs = light_client::StateKeyPair::generate_from_seed_indexed(seed, index);
        Self {
            public_key,
            private_key,
            stake_value,
            state_key_pair: state_key_pairs,
        }
    }
}

impl<KEY: SignatureKey> Default for ValidatorConfig<KEY> {
    fn default() -> Self {
        Self::generated_from_seed_indexed([0u8; 32], 0, 1)
    }
}

/// Holds configuration for a `HotShot`
#[derive(Clone, custom_debug::Debug, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = ""))]
pub struct HotShotConfig<KEY: SignatureKey, ELECTIONCONFIG: ElectionConfig> {
    /// Whether to run one view or continuous views
    pub execution_type: ExecutionType,
    /// Total number of nodes in the network
    pub total_nodes: NonZeroUsize,
    /// Minimum transactions per block
    pub min_transactions: usize,
    /// Maximum transactions per block
    pub max_transactions: NonZeroUsize,
    /// List of known node's public keys and stake value for certificate aggregation, serving as public parameter
    pub known_nodes_with_stake: Vec<KEY::StakeTableEntry>,
    /// My own validator config, including my public key, private key, stake value, serving as private parameter
    pub my_own_validator_config: ValidatorConfig<KEY>,
    /// List of DA committee nodes for static DA committe
    pub da_committee_size: usize,
    /// Base duration for next-view timeout, in milliseconds
    pub next_view_timeout: u64,
    /// The exponential backoff ration for the next-view timeout
    pub timeout_ratio: (u64, u64),
    /// The delay a leader inserts before starting pre-commit, in milliseconds
    pub round_start_delay: u64,
    /// Delay after init before starting consensus, in milliseconds
    pub start_delay: u64,
    /// Number of network bootstrap nodes
    pub num_bootstrap: usize,
    /// The minimum amount of time a leader has to wait to start a round
    pub propose_min_round_time: Duration,
    /// The maximum amount of time a leader can wait to start a round
    pub propose_max_round_time: Duration,
    /// the election configuration
    pub election_config: Option<ELECTIONCONFIG>,
}
