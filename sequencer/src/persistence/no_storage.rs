//! Mock implementation of persistence, for testing.
#![cfg(any(test, feature = "testing"))]

use super::{NetworkConfig, PersistenceOptions, SequencerPersistence};
use crate::{Leaf, NodeState, SeqTypes, ViewNumber};
use async_trait::async_trait;
use hotshot::HotShotInitializer;

#[derive(Clone, Copy, Debug)]
pub struct Options;

#[async_trait]
impl PersistenceOptions for Options {
    type Persistence = NoStorage;

    async fn create(self) -> anyhow::Result<Self::Persistence> {
        Ok(NoStorage)
    }

    async fn reset(self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NoStorage;

#[async_trait]
impl SequencerPersistence for NoStorage {
    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>> {
        Ok(None)
    }

    async fn save_config(&mut self, _: &NetworkConfig) -> anyhow::Result<()> {
        Ok(())
    }
    async fn save_highest_view(&mut self, _: ViewNumber) -> anyhow::Result<()> {
        Ok(())
    }

    async fn save_anchor_leaf(&mut self, _: &Leaf) -> anyhow::Result<()> {
        Ok(())
    }

    async fn load_consensus_state(
        &self,
        genesis: NodeState,
    ) -> anyhow::Result<HotShotInitializer<SeqTypes>> {
        Ok(HotShotInitializer::from_genesis(&genesis)?)
    }
}
