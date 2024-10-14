//! Mock implementation of persistence, for testing.
#![cfg(any(test, feature = "testing"))]

use std::collections::BTreeMap;

use anyhow::bail;
use async_trait::async_trait;
use espresso_types::{
    v0::traits::{PersistenceOptions, SequencerPersistence},
    Leaf, NetworkConfig,
};
use hotshot_types::{
    consensus::CommitmentMap,
    data::{DaProposal, QuorumProposal, VidDisperseShare},
    event::HotShotAction,
    message::Proposal,
    simple_certificate::QuorumCertificate,
    utils::View,
};

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

    async fn collect_garbage(&self, _view: ViewNumber) -> anyhow::Result<()> {
        Ok(())
    }

    async fn save_anchor_leaf(
        &self,
        _: &Leaf,
        _: &QuorumCertificate<SeqTypes>,
    ) -> anyhow::Result<()> {
        Ok(())
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
    ) -> anyhow::Result<Option<BTreeMap<ViewNumber, Proposal<SeqTypes, QuorumProposal<SeqTypes>>>>>
    {
        Ok(None)
    }

    async fn load_quorum_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Proposal<SeqTypes, QuorumProposal<SeqTypes>>> {
        bail!("proposal {view:?} not available");
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
}
