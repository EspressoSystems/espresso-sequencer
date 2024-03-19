//! Abstract storage type for storing DA proposals and VID shares
//!
//! This modules provides the [`Storage`] trait.
//!

use anyhow::Result;
use async_trait::async_trait;

use crate::{
    data::{DAProposal, VidDisperse},
    message::Proposal,
};

use super::node_implementation::NodeType;

/// Abstraction for storing a variety of consensus payload datum.
#[async_trait]
pub trait Storage<TYPES: NodeType>: Send + Sync + Clone {
    async fn append_vid(&self, proposal: &Proposal<TYPES, VidDisperse<TYPES>>) -> Result<()>;
    async fn append_da(&self, proposal: &Proposal<TYPES, DAProposal<TYPES>>) -> Result<()>;
}
