// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! Abstract storage type for storing DA proposals and VID shares
//!
//! This modules provides the [`Storage`] trait.
//!

use std::collections::BTreeMap;

use anyhow::Result;
use async_trait::async_trait;
use jf_vid::VidScheme;

use super::node_implementation::NodeType;
use crate::{
    consensus::{CommitmentMap, View},
    data::{
        DaProposal, DaProposal2, Leaf, Leaf2, QuorumProposal, QuorumProposal2, VidDisperseShare,
        VidDisperseShare2,
    },
    event::HotShotAction,
    message::Proposal,
    simple_certificate::{QuorumCertificate, QuorumCertificate2, UpgradeCertificate},
    vid::VidSchemeType,
};

/// Abstraction for storing a variety of consensus payload datum.
#[async_trait]
pub trait Storage<TYPES: NodeType>: Send + Sync + Clone {
    /// Add a proposal to the stored VID proposals.
    async fn append_vid(&self, proposal: &Proposal<TYPES, VidDisperseShare<TYPES>>) -> Result<()>;
    /// Add a proposal to the stored VID proposals.
    async fn append_vid2(&self, proposal: &Proposal<TYPES, VidDisperseShare2<TYPES>>)
        -> Result<()>;
    /// Add a proposal to the stored DA proposals.
    async fn append_da(
        &self,
        proposal: &Proposal<TYPES, DaProposal<TYPES>>,
        vid_commit: <VidSchemeType as VidScheme>::Commit,
    ) -> Result<()>;
    /// Add a proposal to the stored DA proposals.
    async fn append_da2(
        &self,
        proposal: &Proposal<TYPES, DaProposal2<TYPES>>,
        vid_commit: <VidSchemeType as VidScheme>::Commit,
    ) -> Result<()>;
    /// Add a proposal we sent to the store
    async fn append_proposal(
        &self,
        proposal: &Proposal<TYPES, QuorumProposal<TYPES>>,
    ) -> Result<()>;
    /// Add a proposal we sent to the store
    async fn append_proposal2(
        &self,
        proposal: &Proposal<TYPES, QuorumProposal2<TYPES>>,
    ) -> Result<()>;
    /// Record a HotShotAction taken.
    async fn record_action(&self, view: TYPES::View, action: HotShotAction) -> Result<()>;
    /// Update the current high QC in storage.
    async fn update_high_qc(&self, high_qc: QuorumCertificate<TYPES>) -> Result<()>;
    /// Update the current high QC in storage.
    async fn update_high_qc2(&self, high_qc: QuorumCertificate2<TYPES>) -> Result<()>;
    /// Update the currently undecided state of consensus.  This includes the undecided leaf chain,
    /// and the undecided state.
    async fn update_undecided_state(
        &self,
        leaves: CommitmentMap<Leaf<TYPES>>,
        state: BTreeMap<TYPES::View, View<TYPES>>,
    ) -> Result<()>;
    /// Update the currently undecided state of consensus.  This includes the undecided leaf chain,
    /// and the undecided state.
    async fn update_undecided_state2(
        &self,
        leaves: CommitmentMap<Leaf2<TYPES>>,
        state: BTreeMap<TYPES::View, View<TYPES>>,
    ) -> Result<()>;
    /// Upgrade the current decided upgrade certificate in storage.
    async fn update_decided_upgrade_certificate(
        &self,
        decided_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
    ) -> Result<()>;
    /// Migrate leaves from `Leaf` to `Leaf2`, and proposals from `QuorumProposal` to `QuorumProposal2`
    async fn migrate_consensus(
        &self,
        convert_leaf: fn(Leaf<TYPES>) -> Leaf2<TYPES>,
        convert_proposal: fn(
            Proposal<TYPES, QuorumProposal<TYPES>>,
        ) -> Proposal<TYPES, QuorumProposal2<TYPES>>,
    ) -> Result<()>;
}
