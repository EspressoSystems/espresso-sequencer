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
    data::{DAProposal, VidDisperseShare},
    event::LeafInfo,
    message::Proposal,
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
    async fn save_config(&self, cfg: &NetworkConfig) -> anyhow::Result<()>;

    async fn collect_garbage(&self, view: ViewNumber) -> anyhow::Result<()>;

    /// Saves the latest decided leaf.
    ///
    /// If the height of the new leaf is not greater than the height of the previous decided leaf,
    /// storage is not updated.
    async fn save_anchor_leaf(&self, leaf: &Leaf) -> anyhow::Result<()>;

    /// Load the highest view saved with [`save_voted_view`](Self::save_voted_view).
    async fn load_voted_view(&self) -> anyhow::Result<Option<ViewNumber>>;

    /// Load the latest leaf saved with [`save_anchor_leaf`](Self::save_anchor_leaf).
    async fn load_anchor_leaf(&self) -> anyhow::Result<Option<Leaf>>;

    async fn load_high_qc(&self) -> anyhow::Result<Option<QuorumCertificate<SeqTypes>>>;

    async fn load_vid_shares(
        &self,
    ) -> anyhow::Result<Vec<Proposal<SeqTypes, VidDisperseShare<SeqTypes>>>>;
    async fn load_da_proposals(
        &self,
    ) -> anyhow::Result<Vec<Proposal<SeqTypes, DAProposal<SeqTypes>>>>;

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
    async fn handle_event(&self, event: &Event<SeqTypes>) {
        if let EventType::Decide { leaf_chain, .. } = &event.event {
            if let Some(LeafInfo { leaf, .. }) = leaf_chain.first() {
                if let Err(err) = self.save_anchor_leaf(leaf).await {
                    tracing::error!(
                        ?leaf,
                        hash = %leaf.commit(),
                        "Failed to save anchor leaf. When restarting make sure anchor leaf is at least as recent as this leaf. {err:#}",
                    );
                }

                if let Err(err) = self.collect_garbage(leaf.get_view_number()).await {
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
    use crate::{NodeState, Transaction};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};

    use hotshot::types::SignatureKey;
    use hotshot::{traits::BlockPayload, types::BLSPubKey};
    use hotshot_types::{event::HotShotAction, vid::vid_scheme, vote::HasViewNumber};
    use jf_primitives::vid::VidScheme;
    use rand::{RngCore, SeedableRng};
    use sha2::{Digest, Sha256};
    use testing::TestablePersistence;

    #[async_std::test]
    pub async fn test_anchor_leaf<P: TestablePersistence>() {
        setup_logging();
        setup_backtrace();

        let tmp = P::tmp_storage().await;
        let storage = P::connect(&tmp).await;

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

    #[async_std::test]
    pub async fn test_append_and_collect_garbageion<P: TestablePersistence>() {
        setup_logging();
        setup_backtrace();

        let tmp = P::tmp_storage().await;
        let storage = P::connect(&tmp).await;

        // Test append VID
        assert_eq!(storage.load_vid_shares().await.unwrap(), vec![]);

        let leaf = Leaf::genesis(&NodeState::mock());
        let payload = leaf.get_block_payload().unwrap();
        let bytes = payload.encode().unwrap().collect::<Vec<_>>();
        let disperse = vid_scheme(2).disperse(bytes).unwrap();
        let (pubkey, privkey) = BLSPubKey::generated_from_seed_indexed([0; 32], 1);
        let mut vid = VidDisperseShare::<SeqTypes> {
            view_number: ViewNumber::new(1),
            payload_commitment: Default::default(),
            share: disperse.shares[0].clone(),
            common: disperse.common,
            recipient_key: pubkey,
        };

        let vid_proposal1 = vid.clone().to_proposal(&privkey).unwrap().clone();

        storage.append_vid(&vid_proposal1).await.unwrap();
        vid.view_number = ViewNumber::new(2);

        let vid_proposal2 = vid.clone().to_proposal(&privkey).unwrap().clone();
        storage.append_vid(&vid_proposal2).await.unwrap();

        vid.view_number = ViewNumber::new(3);

        let vid_proposal3 = vid.to_proposal(&privkey).unwrap().clone();
        storage.append_vid(&vid_proposal3).await.unwrap();

        assert_eq!(
            storage.load_vid_shares().await.unwrap(),
            vec![vid_proposal1, vid_proposal2, vid_proposal3.clone()]
        );

        // Test append DA proposals
        assert_eq!(storage.load_da_proposals().await.unwrap(), vec![]);

        let mut seed = [0u8; 32];
        let mut rng = rand_chacha::ChaChaRng::from_entropy();
        rng.fill_bytes(&mut seed);

        let tx = Transaction::random(&mut rng);
        let tx_hash = Sha256::digest(tx.payload());
        let block_payload_signature =
            BLSPubKey::sign(&privkey, &tx_hash).expect("Failed to sign tx hash");

        let da_proposal_inner = DAProposal::<SeqTypes> {
            encoded_transactions: tx_hash.to_vec(),
            metadata: Default::default(),
            view_number: ViewNumber::new(1),
        };

        let da_proposal = Proposal {
            data: da_proposal_inner,
            signature: block_payload_signature,
            _pd: Default::default(),
        };

        storage.append_da(&da_proposal).await.unwrap();

        let mut da_proposal2 = da_proposal.clone();
        da_proposal2.data.view_number = ViewNumber::new(2);
        storage.append_da(&da_proposal2).await.unwrap();

        let mut da_proposal3 = da_proposal2.clone();
        da_proposal3.data.view_number = ViewNumber::new(3);
        storage.append_da(&da_proposal3).await.unwrap();

        assert_eq!(
            storage.load_da_proposals().await.unwrap(),
            vec![da_proposal, da_proposal2, da_proposal3.clone()]
        );

        // Test garbage collection
        // Deleting da proposals and vid shares with view number <=2
        storage.collect_garbage(ViewNumber::new(2)).await.unwrap();
        assert_eq!(
            storage.load_da_proposals().await.unwrap(),
            vec![da_proposal3]
        );

        assert_eq!(
            storage.load_vid_shares().await.unwrap(),
            vec![vid_proposal3]
        );
    }
}
