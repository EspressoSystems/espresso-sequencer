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
    data::{DAProposal, Leaf, VidDisperseShare},
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
    async fn append_da(&self, proposal: &Proposal<TYPES, DAProposal<TYPES>>) -> Result<()>;
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
