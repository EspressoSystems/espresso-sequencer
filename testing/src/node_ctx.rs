use std::{collections::HashMap, sync::Arc};

use hotshot::{traits::TestableNodeImplementation, HotShotError};
use hotshot_types::{data::LeafType, traits::node_implementation::NodeType};

/// context for a round
// TODO eventually we want these to just be futures
// that we poll when things are event driven
// this context will be passed around
#[derive(Debug, Clone)]
pub struct NodeCtx<TYPES: NodeType, I: TestableNodeImplementation<TYPES::ConsensusType, TYPES>> {
    /// results from previous rounds
    pub round_results: HashMap<TYPES::Time, ViewStatus<TYPES, I>>,
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES::ConsensusType, TYPES>> Default
    for NodeCtx<TYPES, I>
{
    fn default() -> Self {
        Self {
            round_results: Default::default(),
        }
    }
}

/// Status of a view.
#[derive(Debug, Clone)]
pub enum ViewStatus<TYPES: NodeType, I: TestableNodeImplementation<TYPES::ConsensusType, TYPES>> {
    /// The view is in progress.
    InProgress(InProgress),
    /// The view is failed.
    ViewFailed(ViewFailed<TYPES>),
    /// The view is a success.
    ViewSuccess(ViewSuccess<TYPES, I::Leaf>),
}

/// In-progress status of a view.
#[derive(Debug, Clone)]
pub struct InProgress {}

/// Failed status of a view.
#[derive(Debug, Clone)]
pub struct ViewFailed<TYPES: NodeType>(pub Arc<HotShotError<TYPES>>);

/// Success status of a view.
#[derive(Debug, Clone)]
pub struct ViewSuccess<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> {
    /// state after decide event
    pub agreed_state: LEAF::MaybeState,

    /// block after decide event
    pub agreed_block: LEAF::DeltasType,

    /// leaf after decide event
    pub agreed_leaf: LEAF,
}
