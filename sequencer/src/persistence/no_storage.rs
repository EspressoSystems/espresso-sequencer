//! Mock implementation of persistence, for testing.
#![cfg(any(test, feature = "testing"))]

use std::collections::BTreeMap;

use super::{NetworkConfig, PersistenceOptions, SequencerPersistence};
use crate::{Leaf, SeqTypes, ValidatedState, ViewNumber};
use anyhow::bail;
use async_trait::async_trait;
use hotshot_types::{
    consensus::CommitmentMap,
    data::{DAProposal, VidDisperseShare},
    event::HotShotAction,
    message::Proposal,
    simple_certificate::QuorumCertificate,
    traits::storage::Storage,
    utils::View,
};

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

    async fn garbage_collect(&mut self, _view: ViewNumber) -> anyhow::Result<()> {
        Ok(())
    }
    async fn save_anchor_leaf(&mut self, _: &Leaf) -> anyhow::Result<()> {
        Ok(())
    }

    async fn load_voted_view(&self) -> anyhow::Result<Option<ViewNumber>> {
        Ok(None)
    }

    async fn load_anchor_leaf(&self) -> anyhow::Result<Option<Leaf>> {
        Ok(None)
    }

    async fn load_high_qc(&self) -> anyhow::Result<Option<QuorumCertificate<SeqTypes>>> {
        Ok(None)
    }

    async fn load_da_proposals(
        &self,
    ) -> anyhow::Result<Vec<Proposal<SeqTypes, DAProposal<SeqTypes>>>> {
        Ok(Vec::new())
    }

    async fn load_vid_shares(
        &self,
    ) -> anyhow::Result<Vec<Proposal<SeqTypes, VidDisperseShare<SeqTypes>>>> {
        Ok(Vec::new())
    }

    async fn load_validated_state(&self, _height: u64) -> anyhow::Result<ValidatedState> {
        bail!("state persistence not implemented");
    }
}

#[async_trait]
impl Storage<SeqTypes> for NoStorage {
    async fn append_vid(
        &self,
        _proposal: &Proposal<SeqTypes, VidDisperseShare<SeqTypes>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn append_da(
        &self,
        _proposal: &Proposal<SeqTypes, DAProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn record_action(&self, _view: ViewNumber, _action: HotShotAction) -> anyhow::Result<()> {
        Ok(())
    }
    async fn update_high_qc(&self, _high_qc: QuorumCertificate<SeqTypes>) -> anyhow::Result<()> {
        Ok(())
    }
    /// Update the currently undecided state of consensus.  This includes the undecided leaf chain,
    /// and the undecided state.
    async fn update_undecided_state(
        &self,
        _leafs: CommitmentMap<Leaf>,
        _state: BTreeMap<ViewNumber, View<SeqTypes>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
