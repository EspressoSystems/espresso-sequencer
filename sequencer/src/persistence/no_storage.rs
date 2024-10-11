//! Mock implementation of persistence, for testing.
#![cfg(any(test, feature = "testing"))]

use async_std::sync::Arc;
use async_trait::async_trait;
use espresso_types::{
    v0::traits::{EventConsumer, PersistenceOptions, SequencerPersistence},
    Leaf, NetworkConfig,
};
use hotshot_types::{
    consensus::CommitmentMap,
    data::{DaProposal, QuorumProposal, VidDisperseShare},
    event::{Event, EventType, HotShotAction, LeafInfo},
    message::Proposal,
    simple_certificate::{QuorumCertificate, UpgradeCertificate},
    utils::View,
};
use std::collections::BTreeMap;

use crate::{SeqTypes, ViewNumber};

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

    async fn save_config(&self, _: &NetworkConfig) -> anyhow::Result<()> {
        Ok(())
    }

    async fn append_decided_leaves(
        &self,
        view_number: ViewNumber,
        leaves: impl IntoIterator<Item = (&LeafInfo<SeqTypes>, QuorumCertificate<SeqTypes>)> + Send,
        consumer: &impl EventConsumer,
    ) -> anyhow::Result<()> {
        let (mut leaf_chain, mut qcs): (Vec<_>, Vec<_>) = leaves
            .into_iter()
            .map(|(info, qc)| (info.clone(), qc))
            .unzip();

        // Put in reverse chronological order, as expected from Decide events.
        leaf_chain.reverse();
        qcs.reverse();

        // Generate decide event for the consumer.
        let final_qc = qcs.pop().unwrap();

        consumer
            .handle_event(&Event {
                view_number,
                event: EventType::Decide {
                    leaf_chain: Arc::new(leaf_chain),
                    qc: Arc::new(final_qc),
                    block_size: None,
                },
            })
            .await
    }

    async fn load_latest_acted_view(&self) -> anyhow::Result<Option<ViewNumber>> {
        Ok(None)
    }

    async fn load_anchor_leaf(
        &self,
    ) -> anyhow::Result<Option<(Leaf, QuorumCertificate<SeqTypes>)>> {
        Ok(None)
    }

    async fn load_undecided_state(
        &self,
    ) -> anyhow::Result<Option<(CommitmentMap<Leaf>, BTreeMap<ViewNumber, View<SeqTypes>>)>> {
        Ok(None)
    }

    async fn load_da_proposal(
        &self,
        _view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, DaProposal<SeqTypes>>>> {
        Ok(None)
    }

    async fn load_vid_share(
        &self,
        _view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, VidDisperseShare<SeqTypes>>>> {
        Ok(None)
    }

    async fn load_quorum_proposals(
        &self,
    ) -> anyhow::Result<BTreeMap<ViewNumber, Proposal<SeqTypes, QuorumProposal<SeqTypes>>>> {
        Ok(Default::default())
    }
    async fn load_upgrade_certificate(
        &self,
    ) -> anyhow::Result<Option<UpgradeCertificate<SeqTypes>>> {
        Ok(None)
    }

    async fn append_vid(
        &self,
        _proposal: &Proposal<SeqTypes, VidDisperseShare<SeqTypes>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn append_da(
        &self,
        _proposal: &Proposal<SeqTypes, DaProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn record_action(&self, _view: ViewNumber, _action: HotShotAction) -> anyhow::Result<()> {
        Ok(())
    }
    async fn update_undecided_state(
        &self,
        _leaves: CommitmentMap<Leaf>,
        _state: BTreeMap<ViewNumber, View<SeqTypes>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn append_quorum_proposal(
        &self,
        _proposal: &Proposal<SeqTypes, QuorumProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn store_upgrade_certificate(
        &self,
        _decided_upgrade_certificate: Option<UpgradeCertificate<SeqTypes>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
