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

use super::node_implementation::NodeType;
use crate::{
    consensus::{CommitmentMap, View},
    data::{DaProposal, Leaf, QuorumProposal, VidDisperseShare},
    event::HotShotAction,
    message::Proposal,
    simple_certificate::QuorumCertificate,
};

/// Abstraction for storing a variety of consensus payload datum.
#[async_trait]
pub trait Storage<TYPES: NodeType>: Send + Sync + Clone {
    /// Add a proposal to the stored VID proposals.
    async fn append_vid(&self, proposal: &Proposal<TYPES, VidDisperseShare<TYPES>>) -> Result<()>;
    /// Add a proposal to the stored DA proposals.
    async fn append_da(&self, proposal: &Proposal<TYPES, DaProposal<TYPES>>) -> Result<()>;
    /// Add a proposal we sent to the store
    async fn append_proposal(
        &self,
        proposal: &Proposal<TYPES, QuorumProposal<TYPES>>,
    ) -> Result<()>;
    /// Record a HotShotAction taken.
    async fn record_action(&self, view: TYPES::Time, action: HotShotAction) -> Result<()>;
    /// Update the current high QC in storage.
    async fn update_high_qc(&self, high_qc: QuorumCertificate<TYPES>) -> Result<()>;
    /// Update the currently undecided state of consensus.  This includes the undecided leaf chain,
    /// and the undecided state.
    async fn update_undecided_state(
        &self,
        leafs: CommitmentMap<Leaf<TYPES>>,
        state: BTreeMap<TYPES::Time, View<TYPES>>,
    ) -> Result<()>;
}
