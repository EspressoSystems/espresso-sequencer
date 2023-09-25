//! Events that a `HotShot` instance can emit

use crate::{
    certificate::QuorumCertificate, data::LeafType, error::HotShotError,
    traits::node_implementation::NodeType,
};
use commit::Commitment;
use std::sync::Arc;
/// A status event emitted by a `HotShot` instance
///
/// This includes some metadata, such as the stage and view number that the event was generated in,
/// as well as an inner [`EventType`] describing the event proper.
#[derive(Clone, Debug)]
pub struct Event<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> {
    /// The view number that this event originates from
    pub view_number: TYPES::Time,
    /// The underlying event
    pub event: EventType<TYPES, LEAF>,
}

/// The type and contents of a status event emitted by a `HotShot` instance
///
/// This enum does not include metadata shared among all variants, such as the stage and view
/// number, and is thus always returned wrapped in an [`Event`].
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum EventType<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> {
    /// A view encountered an error and was interrupted
    Error {
        /// The underlying error
        error: Arc<HotShotError<TYPES>>,
    },
    /// A new decision event was issued
    Decide {
        /// The chain of Leafs that were committed by this decision
        ///
        /// This list is sorted in reverse view number order, with the newest (highest view number)
        /// block first in the list.
        ///
        /// This list may be incomplete if the node is currently performing catchup.
        leaf_chain: Arc<Vec<LEAF>>,
        /// The QC signing the most recent leaf in `leaf_chain`.
        ///
        /// Note that the QC for each additional leaf in the chain can be obtained from the leaf
        /// before it using
        qc: Arc<QuorumCertificate<TYPES, Commitment<LEAF>>>,
        /// Optional information of the number of transactions in the block, for logging purposes.
        block_size: Option<u64>,
    },
    /// A replica task was canceled by a timeout interrupt
    ReplicaViewTimeout {
        /// The view that timed out
        view_number: TYPES::Time,
    },
    /// A next leader task was canceled by a timeout interrupt
    NextLeaderViewTimeout {
        /// The view that timed out
        view_number: TYPES::Time,
    },
    /// The view has finished.  If values were decided on, a `Decide` event will also be emitted.
    ViewFinished {
        /// The view number that has just finished
        view_number: TYPES::Time,
    },
}
