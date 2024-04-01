//! Sequencer node persistence.
//!
//! This module implements the persistence required for a sequencer node to rejoin the network and
//! resume participating in consensus, in the event that its process crashes or is killed and loses
//! all in-memory state.
//!
//! This is distinct from the query service persistent storage found in the `api` module, which is
//! an extension that node operators can opt into. This module defines the minimum level of
//! persistence which is _required_ to run a node.

use crate::{ElectionConfig, Leaf, NodeState, PubKey, SeqTypes, ValidatedState, ViewNumber};
use anyhow::Context;
use async_std::sync::Arc;
use async_trait::async_trait;
use commit::Committable;
use hotshot::{
    traits::ValidatedState as _,
    types::{Event, EventType},
    HotShotInitializer,
};
use hotshot_types::{
    event::LeafInfo,
    simple_certificate::QuorumCertificate,
    traits::{node_implementation::ConsensusTime, storage::Storage},
};
use std::cmp::max;

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
pub trait SequencerPersistence: Send + Sync + Storage<SeqTypes> + 'static {
    /// Load the orchestrator config from storage.
    ///
    /// Returns `None` if no config exists (we are joining a network for the first time). Fails with
    /// `Err` if it could not be determined whether a config exists or not.
    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>>;

    /// Save the orchestrator config to storage.
    async fn save_config(&mut self, cfg: &NetworkConfig) -> anyhow::Result<()>;

    async fn garbage_collect(&mut self, view: ViewNumber) -> anyhow::Result<()>;

    /// Saves the latest decided leaf.
    ///
    /// If the height of the new leaf is not greater than the height of the previous decided leaf,
    /// storage is not updated.
    async fn save_anchor_leaf(&mut self, leaf: &Leaf) -> anyhow::Result<()>;

    /// Load the highest view saved with [`save_voted_view`](Self::save_voted_view).
    async fn load_voted_view(&self) -> anyhow::Result<Option<ViewNumber>>;

    /// Load the latest leaf saved with [`save_anchor_leaf`](Self::save_anchor_leaf).
    async fn load_anchor_leaf(&self) -> anyhow::Result<Option<Leaf>>;

    async fn load_high_qc(&self) -> anyhow::Result<Option<QuorumCertificate<SeqTypes>>>;

    /// Load the validated state after block `height`, if available.
    async fn load_validated_state(&self, height: u64) -> anyhow::Result<ValidatedState>;

    /// Load the latest known consensus state.
    ///
    /// Returns an initializer to resume HotShot from the latest saved state (or start from genesis,
    /// if there is no saved state).
    async fn load_consensus_state(
        &self,
        state: NodeState,
    ) -> anyhow::Result<HotShotInitializer<SeqTypes>> {
        let highest_voted_view = match self
            .load_voted_view()
            .await
            .context("loading last voted view")?
        {
            Some(view) => {
                tracing::info!(?view, "starting from saved view");
                view
            }
            None => {
                tracing::info!("no saved view, starting from genesis");
                ViewNumber::genesis()
            }
        };
        let (leaf, validated_state) = match self
            .load_anchor_leaf()
            .await
            .context("loading anchor leaf")?
        {
            Some(leaf) => {
                tracing::info!(?leaf, "starting from saved leaf");
                let validated_state = match self.load_validated_state(leaf.get_height()).await {
                    Ok(validated_state) => Some(Arc::new(validated_state)),
                    Err(err) => {
                        tracing::error!(
                            "unable to load validated state, will need to catchup: {err:#}"
                        );
                        None
                    }
                };
                (leaf, validated_state)
            }
            None => {
                tracing::info!("no saved leaf, starting from genesis leaf");
                (
                    Leaf::genesis(&state),
                    Some(Arc::new(ValidatedState::genesis(&state).0)),
                )
            }
        };

        // We start from the maximum view between `highest_voted_view` and `leaf.view_number`. This
        // prevents double votes from starting in a view in which we had already voted before the
        // restart, and prevents unnecessary catchup from starting in a view earlier than the anchor
        // leaf.
        let view = max(highest_voted_view, leaf.get_view_number());
        tracing::info!(?leaf, ?view, "loaded consensus state");

        let high_qc = self
            .load_high_qc()
            .await?
            .unwrap_or_else(QuorumCertificate::genesis);

        Ok(HotShotInitializer::from_reload(
            leaf,
            state,
            validated_state,
            view,
            high_qc,
            Default::default(),
            Default::default(),
        ))
    }

    /// Update storage based on an event from consensus.
    async fn handle_event(&mut self, event: &Event<SeqTypes>) {
        if let EventType::Decide { leaf_chain, .. } = &event.event {
            if let Some(LeafInfo { leaf, .. }) = leaf_chain.first() {
                if let Err(err) = self.save_anchor_leaf(leaf).await {
                    tracing::error!(
                        ?leaf,
                        hash = %leaf.commit(),
                        "Failed to save anchor leaf. When restarting make sure anchor leaf is at least as recent as this leaf. {err:#}",
                    );
                }

                if let Err(err) = self.garbage_collect(leaf.get_view_number()).await {
                    tracing::error!("Failed to garbage collect. {err:#}",);
                }
            }
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[async_trait]
    pub trait TestablePersistence: SequencerPersistence {
        type Storage;

        async fn tmp_storage() -> Self::Storage;
        async fn connect(storage: &Self::Storage) -> Self;
    }
}

#[cfg(test)]
#[espresso_macros::generic_tests]
mod persistence_tests {

    use super::*;
    use crate::NodeState;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};

    use hotshot_types::{event::HotShotAction, vote::HasViewNumber};
    use testing::TestablePersistence;

    #[async_std::test]
    pub async fn test_anchor_leaf<P: TestablePersistence>() {
        setup_logging();
        setup_backtrace();

        let tmp = P::tmp_storage().await;
        let mut storage = P::connect(&tmp).await;

        // Initially, there is no saved leaf.
        assert_eq!(storage.load_anchor_leaf().await.unwrap(), None);

        // Store a leaf.
        let leaf1 = Leaf::genesis(&NodeState::mock());
        storage.save_anchor_leaf(&leaf1).await.unwrap();
        assert_eq!(storage.load_anchor_leaf().await.unwrap().unwrap(), leaf1);

        // Store a newer leaf, make sure storage gets updated.
        let mut leaf2 = leaf1.clone();
        leaf2.get_block_header_mut().height += 1;
        storage.save_anchor_leaf(&leaf2).await.unwrap();
        assert_eq!(storage.load_anchor_leaf().await.unwrap().unwrap(), leaf2);

        // Store an old leaf, make sure storage is unchanged.
        storage.save_anchor_leaf(&leaf1).await.unwrap();
        assert_eq!(storage.load_anchor_leaf().await.unwrap().unwrap(), leaf2);
    }

    #[async_std::test]
    pub async fn test_voted_view<P: TestablePersistence>() {
        setup_logging();
        setup_backtrace();

        let tmp = P::tmp_storage().await;
        let storage = P::connect(&tmp).await;

        // Initially, there is no saved view.
        assert_eq!(storage.load_voted_view().await.unwrap(), None);

        // Store a view.
        let view1 = ViewNumber::genesis();
        storage
            .record_action(view1, HotShotAction::Vote)
            .await
            .unwrap();
        assert_eq!(storage.load_voted_view().await.unwrap().unwrap(), view1);

        // Store a newer view, make sure storage gets updated.
        let view2 = view1 + 1;
        storage
            .record_action(view2, HotShotAction::Vote)
            .await
            .unwrap();
        assert_eq!(storage.load_voted_view().await.unwrap().unwrap(), view2);

        // Store an old view, make sure storage is unchanged.
        storage
            .record_action(view1, HotShotAction::Vote)
            .await
            .unwrap();
        assert_eq!(storage.load_voted_view().await.unwrap().unwrap(), view2);
    }

    #[async_std::test]
    pub async fn test_high_qc<P: TestablePersistence>() {
        setup_logging();
        setup_backtrace();

        let tmp = P::tmp_storage().await;
        let storage = P::connect(&tmp).await;

        assert_eq!(storage.load_high_qc().await.unwrap(), None);

        let mut qc = QuorumCertificate::genesis();

        storage.update_high_qc(qc.clone()).await.unwrap();
        assert_eq!(
            storage.load_high_qc().await.unwrap().unwrap(),
            QuorumCertificate::genesis()
        );

        // store qcs with different view number
        qc.view_number += 1;
        storage.update_high_qc(qc.clone()).await.unwrap();

        qc.view_number += 1;
        storage.update_high_qc(qc.clone()).await.unwrap();

        assert_eq!(
            storage
                .load_high_qc()
                .await
                .unwrap()
                .unwrap()
                .get_view_number(),
            qc.get_view_number()
        );
    }
}
