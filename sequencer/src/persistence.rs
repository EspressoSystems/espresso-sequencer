//! Sequencer node persistence.
//!
//! This module implements the persistence required for a sequencer node to rejoin the network and
//! resume participating in consensus, in the event that its process crashes or is killed and loses
//! all in-memory state.
//!
//! This is distinct from the query service persistent storage found in the `api` module, which is
//! an extension that node operators can opt into. This module defines the minimum level of
//! persistence which is _required_ to run a node.

use crate::{ElectionConfig, Leaf, NodeState, PubKey, SeqTypes, ViewNumber};
use async_trait::async_trait;
use hotshot::HotShotInitializer;

pub mod fs;
pub mod no_storage;
pub mod sql;

pub type NetworkConfig = hotshot_orchestrator::config::NetworkConfig<PubKey, ElectionConfig>;

#[async_trait]
pub trait PersistenceOptions: Clone {
    type Persistence: SequencerPersistence;

    async fn create(self) -> anyhow::Result<Self::Persistence>;
    async fn reset(self) -> anyhow::Result<()>;
}

#[async_trait]
pub trait SequencerPersistence: Send + Sync + 'static {
    /// Load the orchestrator config from storage.
    ///
    /// Returns `None` if no config exists (we are joining a network for the first time). Fails with
    /// `Err` if it could not be determined whether a config exists or not.
    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>>;

    /// Save the orchestrator config to storage.
    async fn save_config(&mut self, cfg: &NetworkConfig) -> anyhow::Result<()>;

    /// Saves the highest view known to this node.
    ///
    /// If the new view is not greater than the previous highest view, storage is not updated.
    async fn save_highest_view(&mut self, view: ViewNumber) -> anyhow::Result<()>;

    /// Saves the latest decided leaf.
    ///
    /// If the height of the new leaf is not greater than the height of the previous decided leaf,
    /// storage is not updated.
    async fn save_anchor_leaf(&mut self, leaf: &Leaf) -> anyhow::Result<()>;

    /// Load the latest known consensus state.
    ///
    /// Returns an initializer to resume HotShot from the latest saved state (or start from genesis,
    /// if there is no saved state).
    async fn load_consensus_state(
        &self,
        genesis: NodeState,
    ) -> anyhow::Result<HotShotInitializer<SeqTypes>>;
}
