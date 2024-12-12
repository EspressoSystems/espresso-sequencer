// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use anyhow::{bail, Result};
use async_lock::RwLock;
use async_trait::async_trait;
use hotshot_types::{
    consensus::CommitmentMap,
    data::{
        DaProposal, DaProposal2, Leaf, Leaf2, QuorumProposal, QuorumProposal2, VidDisperseShare,
        VidDisperseShare2,
    },
    event::HotShotAction,
    message::Proposal,
    simple_certificate::{QuorumCertificate2, UpgradeCertificate},
    traits::{
        node_implementation::{ConsensusTime, NodeType},
        storage::Storage,
    },
    utils::View,
    vid::VidSchemeType,
    vote::HasViewNumber,
};
use jf_vid::VidScheme;

use crate::testable_delay::{DelayConfig, SupportedTraitTypesForAsyncDelay, TestableDelay};

type VidShares<TYPES> = HashMap<
    <TYPES as NodeType>::View,
    HashMap<<TYPES as NodeType>::SignatureKey, Proposal<TYPES, VidDisperseShare<TYPES>>>,
>;
type VidShares2<TYPES> = HashMap<
    <TYPES as NodeType>::View,
    HashMap<<TYPES as NodeType>::SignatureKey, Proposal<TYPES, VidDisperseShare2<TYPES>>>,
>;

#[derive(Clone, Debug)]
pub struct TestStorageState<TYPES: NodeType> {
    vids: VidShares<TYPES>,
    vid2: VidShares2<TYPES>,
    das: HashMap<TYPES::View, Proposal<TYPES, DaProposal<TYPES>>>,
    da2s: HashMap<TYPES::View, Proposal<TYPES, DaProposal2<TYPES>>>,
    proposals: BTreeMap<TYPES::View, Proposal<TYPES, QuorumProposal<TYPES>>>,
    proposals2: BTreeMap<TYPES::View, Proposal<TYPES, QuorumProposal2<TYPES>>>,
    high_qc: Option<hotshot_types::simple_certificate::QuorumCertificate<TYPES>>,
    high_qc2: Option<hotshot_types::simple_certificate::QuorumCertificate2<TYPES>>,
    action: TYPES::View,
    epoch: TYPES::Epoch,
}

impl<TYPES: NodeType> Default for TestStorageState<TYPES> {
    fn default() -> Self {
        Self {
            vids: HashMap::new(),
            vid2: HashMap::new(),
            das: HashMap::new(),
            da2s: HashMap::new(),
            proposals: BTreeMap::new(),
            proposals2: BTreeMap::new(),
            high_qc: None,
            high_qc2: None,
            action: TYPES::View::genesis(),
            epoch: TYPES::Epoch::genesis(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TestStorage<TYPES: NodeType> {
    inner: Arc<RwLock<TestStorageState<TYPES>>>,
    /// `should_return_err` is a testing utility to validate negative cases.
    pub should_return_err: bool,
    pub delay_config: DelayConfig,
    pub decided_upgrade_certificate: Arc<RwLock<Option<UpgradeCertificate<TYPES>>>>,
}

impl<TYPES: NodeType> Default for TestStorage<TYPES> {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(TestStorageState::default())),
            should_return_err: false,
            delay_config: DelayConfig::default(),
            decided_upgrade_certificate: Arc::new(RwLock::new(None)),
        }
    }
}

#[async_trait]
impl<TYPES: NodeType> TestableDelay for TestStorage<TYPES> {
    async fn run_delay_settings_from_config(delay_config: &DelayConfig) {
        if let Some(settings) = delay_config.get_setting(&SupportedTraitTypesForAsyncDelay::Storage)
        {
            Self::handle_async_delay(settings).await;
        }
    }
}

impl<TYPES: NodeType> TestStorage<TYPES> {
    pub async fn proposals_cloned(
        &self,
    ) -> BTreeMap<TYPES::View, Proposal<TYPES, QuorumProposal2<TYPES>>> {
        self.inner.read().await.proposals2.clone()
    }
    pub async fn high_qc_cloned(&self) -> Option<QuorumCertificate2<TYPES>> {
        self.inner.read().await.high_qc2.clone()
    }
    pub async fn decided_upgrade_certificate(&self) -> Option<UpgradeCertificate<TYPES>> {
        self.decided_upgrade_certificate.read().await.clone()
    }
    pub async fn last_actioned_view(&self) -> TYPES::View {
        self.inner.read().await.action
    }
    pub async fn last_actioned_epoch(&self) -> TYPES::Epoch {
        self.inner.read().await.epoch
    }
}

#[async_trait]
impl<TYPES: NodeType> Storage<TYPES> for TestStorage<TYPES> {
    async fn append_vid(&self, proposal: &Proposal<TYPES, VidDisperseShare<TYPES>>) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to append VID proposal to storage");
        }
        Self::run_delay_settings_from_config(&self.delay_config).await;
        let mut inner = self.inner.write().await;
        inner
            .vids
            .entry(proposal.data.view_number)
            .or_default()
            .insert(proposal.data.recipient_key.clone(), proposal.clone());
        Ok(())
    }

    async fn append_vid2(
        &self,
        proposal: &Proposal<TYPES, VidDisperseShare2<TYPES>>,
    ) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to append VID proposal to storage");
        }
        Self::run_delay_settings_from_config(&self.delay_config).await;
        let mut inner = self.inner.write().await;
        inner
            .vid2
            .entry(proposal.data.view_number)
            .or_default()
            .insert(proposal.data.recipient_key.clone(), proposal.clone());
        Ok(())
    }

    async fn append_da(
        &self,
        proposal: &Proposal<TYPES, DaProposal<TYPES>>,
        _vid_commit: <VidSchemeType as VidScheme>::Commit,
    ) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to append VID proposal to storage");
        }
        Self::run_delay_settings_from_config(&self.delay_config).await;
        let mut inner = self.inner.write().await;
        inner
            .das
            .insert(proposal.data.view_number, proposal.clone());
        Ok(())
    }
    async fn append_da2(
        &self,
        proposal: &Proposal<TYPES, DaProposal2<TYPES>>,
        _vid_commit: <VidSchemeType as VidScheme>::Commit,
    ) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to append VID proposal to storage");
        }
        Self::run_delay_settings_from_config(&self.delay_config).await;
        let mut inner = self.inner.write().await;
        inner
            .da2s
            .insert(proposal.data.view_number, proposal.clone());
        Ok(())
    }
    async fn append_proposal(
        &self,
        proposal: &Proposal<TYPES, QuorumProposal<TYPES>>,
    ) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to append VID proposal to storage");
        }
        Self::run_delay_settings_from_config(&self.delay_config).await;
        let mut inner = self.inner.write().await;
        inner
            .proposals
            .insert(proposal.data.view_number, proposal.clone());
        Ok(())
    }
    async fn append_proposal2(
        &self,
        proposal: &Proposal<TYPES, QuorumProposal2<TYPES>>,
    ) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to append VID proposal to storage");
        }
        Self::run_delay_settings_from_config(&self.delay_config).await;
        let mut inner = self.inner.write().await;
        inner
            .proposals2
            .insert(proposal.data.view_number, proposal.clone());
        Ok(())
    }

    async fn record_action(
        &self,
        view: <TYPES as NodeType>::View,
        action: hotshot_types::event::HotShotAction,
    ) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to append Action to storage");
        }
        let mut inner = self.inner.write().await;
        if view > inner.action && matches!(action, HotShotAction::Vote | HotShotAction::Propose) {
            inner.action = view;
        }
        Self::run_delay_settings_from_config(&self.delay_config).await;
        Ok(())
    }

    async fn update_high_qc(
        &self,
        new_high_qc: hotshot_types::simple_certificate::QuorumCertificate<TYPES>,
    ) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to update high qc to storage");
        }
        Self::run_delay_settings_from_config(&self.delay_config).await;
        let mut inner = self.inner.write().await;
        if let Some(ref current_high_qc) = inner.high_qc {
            if new_high_qc.view_number() > current_high_qc.view_number() {
                inner.high_qc = Some(new_high_qc);
            }
        } else {
            inner.high_qc = Some(new_high_qc);
        }
        Ok(())
    }

    async fn update_high_qc2(
        &self,
        new_high_qc: hotshot_types::simple_certificate::QuorumCertificate2<TYPES>,
    ) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to update high qc to storage");
        }
        Self::run_delay_settings_from_config(&self.delay_config).await;
        let mut inner = self.inner.write().await;
        if let Some(ref current_high_qc) = inner.high_qc2 {
            if new_high_qc.view_number() > current_high_qc.view_number() {
                inner.high_qc2 = Some(new_high_qc);
            }
        } else {
            inner.high_qc2 = Some(new_high_qc);
        }
        Ok(())
    }
    async fn update_undecided_state(
        &self,
        _leaves: CommitmentMap<Leaf<TYPES>>,
        _state: BTreeMap<TYPES::View, View<TYPES>>,
    ) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to update high qc to storage");
        }
        Self::run_delay_settings_from_config(&self.delay_config).await;
        Ok(())
    }
    async fn update_undecided_state2(
        &self,
        _leaves: CommitmentMap<Leaf2<TYPES>>,
        _state: BTreeMap<TYPES::View, View<TYPES>>,
    ) -> Result<()> {
        if self.should_return_err {
            bail!("Failed to update high qc to storage");
        }
        Self::run_delay_settings_from_config(&self.delay_config).await;
        Ok(())
    }
    async fn update_decided_upgrade_certificate(
        &self,
        decided_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
    ) -> Result<()> {
        *self.decided_upgrade_certificate.write().await = decided_upgrade_certificate;

        Ok(())
    }

    async fn migrate_consensus(
        &self,
        _convert_leaf: fn(Leaf<TYPES>) -> Leaf2<TYPES>,
        convert_proposal: fn(
            Proposal<TYPES, QuorumProposal<TYPES>>,
        ) -> Proposal<TYPES, QuorumProposal2<TYPES>>,
    ) -> Result<()> {
        let mut storage_writer = self.inner.write().await;

        for (view, proposal) in storage_writer.proposals.clone().iter() {
            storage_writer
                .proposals2
                .insert(*view, convert_proposal(proposal.clone()));
        }

        Ok(())
    }
}
