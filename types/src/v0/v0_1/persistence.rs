use std::{cmp::max, collections::BTreeMap, sync::Arc};

use crate::{Event, NetworkConfig, NodeState, SeqTypes, ValidatedState};
use anyhow::{bail, ensure, Context};
use async_std::sync::RwLock;
use async_trait::async_trait;
use committable::Committable;
use hotshot::{traits::ValidatedState as _, types::EventType, HotShotInitializer};
use hotshot_types::{
    consensus::CommitmentMap,
    data::{DaProposal, QuorumProposal, VidDisperseShare, ViewNumber},
    event::{HotShotAction, LeafInfo},
    message::Proposal,
    simple_certificate::QuorumCertificate,
    traits::{node_implementation::ConsensusTime, storage::Storage},
    utils::View,
};

use crate::{traits::StateCatchup, Leaf};

#[async_trait]
pub trait PersistenceOptions: Clone + Send + Sync + 'static {
    type Persistence: SequencerPersistence;

    async fn create(self) -> anyhow::Result<Self::Persistence>;
    async fn reset(self) -> anyhow::Result<()>;

    async fn create_catchup_provider(self) -> anyhow::Result<Arc<dyn StateCatchup>> {
        self.create().await?.into_catchup_provider()
    }
}

#[async_trait]
pub trait SequencerPersistence: Sized + Send + Sync + 'static {
    /// Use this storage as a state catchup backend, if supported.
    fn into_catchup_provider(self) -> anyhow::Result<Arc<dyn StateCatchup>> {
        bail!("state catchup is not implemented for this persistence type");
    }

    /// Load the orchestrator config from storage.
    ///
    /// Returns `None` if no config exists (we are joining a network for the first time). Fails with
    /// `Err` if it could not be determined whether a config exists or not.
    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>>;

    /// Save the orchestrator config to storage.
    async fn save_config(&mut self, cfg: &NetworkConfig) -> anyhow::Result<()>;

    async fn collect_garbage(&mut self, view: ViewNumber) -> anyhow::Result<()>;

    /// Saves the latest decided leaf.
    ///
    /// If the height of the new leaf is not greater than the height of the previous decided leaf,
    /// storage is not updated.
    async fn save_anchor_leaf(
        &mut self,
        leaf: &Leaf,
        qc: &QuorumCertificate<SeqTypes>,
    ) -> anyhow::Result<()>;

    /// Load the highest view saved with [`save_voted_view`](Self::save_voted_view).
    async fn load_latest_acted_view(&self) -> anyhow::Result<Option<ViewNumber>>;

    /// Load the latest leaf saved with [`save_anchor_leaf`](Self::save_anchor_leaf).
    async fn load_anchor_leaf(&self)
        -> anyhow::Result<Option<(Leaf, QuorumCertificate<SeqTypes>)>>;

    /// Load undecided state saved by consensus before we shut down.
    async fn load_undecided_state(
        &self,
    ) -> anyhow::Result<Option<(CommitmentMap<Leaf>, BTreeMap<ViewNumber, View<SeqTypes>>)>>;

    /// Load the proposals saved by consensus
    async fn load_quorum_proposals(
        &self,
    ) -> anyhow::Result<Option<BTreeMap<ViewNumber, Proposal<SeqTypes, QuorumProposal<SeqTypes>>>>>;

    async fn load_vid_share(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, VidDisperseShare<SeqTypes>>>>;
    async fn load_da_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, DaProposal<SeqTypes>>>>;

    /// Load the latest known consensus state.
    ///
    /// Returns an initializer to resume HotShot from the latest saved state (or start from genesis,
    /// if there is no saved state).
    async fn load_consensus_state(
        &self,
        state: NodeState,
    ) -> anyhow::Result<HotShotInitializer<SeqTypes>> {
        let genesis_validated_state = ValidatedState::genesis(&state).0;
        let highest_voted_view = match self
            .load_latest_acted_view()
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
        let (leaf, high_qc) = match self
            .load_anchor_leaf()
            .await
            .context("loading anchor leaf")?
        {
            Some((leaf, high_qc)) => {
                tracing::info!(?leaf, ?high_qc, "starting from saved leaf");
                ensure!(
                    leaf.view_number() == high_qc.view_number,
                    format!(
                        "loaded anchor leaf from view {:?}, but high QC is from view {:?}",
                        leaf.view_number(),
                        high_qc.view_number
                    )
                );
                (leaf, high_qc)
            }
            None => {
                tracing::info!("no saved leaf, starting from genesis leaf");
                (
                    Leaf::genesis(&genesis_validated_state, &state).await,
                    QuorumCertificate::genesis(&genesis_validated_state, &state).await,
                )
            }
        };
        let validated_state = if leaf.block_header().height() == 0 {
            // If we are starting from genesis, we can provide the full state.
            Some(Arc::new(genesis_validated_state))
        } else {
            // Otherwise, we will have to construct a sparse state and fetch missing data during
            // catchup.
            None
        };

        // If we are not starting from genesis, we start from the view following the maximum view
        // between `highest_voted_view` and `leaf.view_number`. This prevents double votes from
        // starting in a view in which we had already voted before the restart, and prevents
        // unnecessary catchup from starting in a view earlier than the anchor leaf.
        let mut view = max(highest_voted_view, leaf.view_number());
        if view != ViewNumber::genesis() {
            view += 1;
        }

        let (undecided_leaves, undecided_state) = self
            .load_undecided_state()
            .await
            .context("loading undecided state")?
            .unwrap_or_default();

        let saved_proposals = self
            .load_quorum_proposals()
            .await
            .context("loading saved proposals")
            .unwrap_or_default()
            .unwrap_or_default();

        tracing::info!(
            ?leaf,
            ?view,
            ?high_qc,
            ?validated_state,
            ?undecided_leaves,
            ?undecided_state,
            ?saved_proposals,
            "loaded consensus state"
        );
        Ok(HotShotInitializer::from_reload(
            leaf,
            state,
            validated_state,
            view,
            saved_proposals,
            high_qc,
            undecided_leaves.into_values().collect(),
            undecided_state,
        ))
    }

    /// Update storage based on an event from consensus.
    async fn handle_event(&mut self, event: &Event) {
        if let EventType::Decide { leaf_chain, qc, .. } = &event.event {
            if let Some(LeafInfo { leaf, .. }) = leaf_chain.first() {
                if qc.view_number != leaf.view_number() {
                    tracing::error!(
                        leaf_view = ?leaf.view_number(),
                        qc_view = ?qc.view_number,
                        "latest leaf and QC are from different views!",
                    );
                    return;
                }
                if let Err(err) = self.save_anchor_leaf(leaf, qc).await {
                    tracing::error!(
                        ?leaf,
                        hash = %leaf.commit(),
                        "Failed to save anchor leaf. When restarting make sure anchor leaf is at least as recent as this leaf. {err:#}",
                    );
                }

                if let Err(err) = self.collect_garbage(leaf.view_number()).await {
                    tracing::error!("Failed to garbage collect. {err:#}",);
                }
            }
        }
    }

    async fn append_vid(
        &mut self,
        proposal: &Proposal<SeqTypes, VidDisperseShare<SeqTypes>>,
    ) -> anyhow::Result<()>;
    async fn append_da(
        &mut self,
        proposal: &Proposal<SeqTypes, DaProposal<SeqTypes>>,
    ) -> anyhow::Result<()>;
    async fn record_action(
        &mut self,
        view: ViewNumber,
        action: HotShotAction,
    ) -> anyhow::Result<()>;
    async fn update_undecided_state(
        &mut self,
        leaves: CommitmentMap<Leaf>,
        state: BTreeMap<ViewNumber, View<SeqTypes>>,
    ) -> anyhow::Result<()>;
    async fn append_quorum_proposal(
        &mut self,
        proposal: &Proposal<SeqTypes, QuorumProposal<SeqTypes>>,
    ) -> anyhow::Result<()>;
}

// move into impls

#[async_trait]
impl<P: SequencerPersistence> Storage<SeqTypes> for Arc<RwLock<P>> {
    async fn append_vid(
        &self,
        proposal: &Proposal<SeqTypes, VidDisperseShare<SeqTypes>>,
    ) -> anyhow::Result<()> {
        self.write().await.append_vid(proposal).await
    }

    async fn append_da(
        &self,
        proposal: &Proposal<SeqTypes, DaProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        self.write().await.append_da(proposal).await
    }
    async fn record_action(&self, view: ViewNumber, action: HotShotAction) -> anyhow::Result<()> {
        self.write().await.record_action(view, action).await
    }
    async fn update_high_qc(&self, _high_qc: QuorumCertificate<SeqTypes>) -> anyhow::Result<()> {
        Ok(())
    }

    async fn update_undecided_state(
        &self,
        leaves: CommitmentMap<Leaf>,
        state: BTreeMap<ViewNumber, View<SeqTypes>>,
    ) -> anyhow::Result<()> {
        self.write()
            .await
            .update_undecided_state(leaves, state)
            .await
    }

    async fn append_proposal(
        &self,
        proposal: &Proposal<SeqTypes, QuorumProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        self.write().await.append_quorum_proposal(proposal).await
    }
}
