//! Mock implementation of persistence, for testing.
#![cfg(any(test, feature = "testing"))]

use super::{NetworkConfig, PersistenceOptions, SequencerPersistence};
use crate::{Header, Leaf, SeqTypes, ValidatedState, ViewNumber};
use anyhow::bail;
use async_trait::async_trait;
use hotshot_types::{
    data::{DAProposal, VidDisperseShare},
    event::HotShotAction,
    message::Proposal,
    simple_certificate::QuorumCertificate,
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

    async fn collect_garbage(&mut self, _view: ViewNumber) -> anyhow::Result<()> {
        Ok(())
    }

    async fn save_anchor_leaf(
        &mut self,
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

    async fn load_da_proposal(
        &self,
        _view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, DAProposal<SeqTypes>>>> {
        Ok(None)
    }

    async fn load_vid_share(
        &self,
        _view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, VidDisperseShare<SeqTypes>>>> {
        Ok(None)
    }

    async fn append_vid(
        &mut self,
        _proposal: &Proposal<SeqTypes, VidDisperseShare<SeqTypes>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn append_da(
        &mut self,
        _proposal: &Proposal<SeqTypes, DAProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn record_action(
        &mut self,
        _view: ViewNumber,
        _action: HotShotAction,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn load_validated_state(&self, _header: &Header) -> anyhow::Result<ValidatedState> {
        bail!("state persistence not implemented");
    }
}
