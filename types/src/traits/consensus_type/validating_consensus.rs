//! The [`ValidatingConsensusType`] trait allows consensus-specific customization points.

use crate::traits::consensus_type::ConsensusType;

/// Marker trait for consensus which provides ordering and execution.
pub trait ValidatingConsensusType
where
    Self: ConsensusType,
{
}

/// Consensus which provides ordering and execution.
#[derive(Clone, Debug)]
pub struct ValidatingConsensus;
impl ConsensusType for ValidatingConsensus {}
impl ValidatingConsensusType for ValidatingConsensus {}
