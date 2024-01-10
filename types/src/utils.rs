//! Utility functions, type aliases, helper structs and enum definitions.

use crate::{
    data::{Leaf, VidCommitment},
    traits::{node_implementation::NodeType, State},
};
use commit::Commitment;
use std::ops::Deref;

/// A view's state
#[derive(Debug)]
pub enum ViewInner<TYPES: NodeType> {
    /// A pending view with an available block but not leaf proposal yet.
    ///
    /// Storing this state allows us to garbage collect blocks for views where a proposal is never
    /// made. This saves memory when a leader fails and subverts a DoS attack where malicious
    /// leaders repeatedly request availability for blocks that they never propose.
    DA {
        /// Payload commitment to the available block.
        payload_commitment: VidCommitment,
    },
    /// Undecided view
    Leaf {
        /// Proposed leaf
        leaf: Commitment<Leaf<TYPES>>,
        /// Application-specific data.
        metadata: <TYPES::StateType as State>::Metadata,
    },
    /// Leaf has failed
    Failed,
}

impl<TYPES: NodeType> ViewInner<TYPES> {
    /// return the underlying leaf hash if it exists
    #[must_use]
    pub fn get_leaf_commitment(&self) -> Option<Commitment<Leaf<TYPES>>> {
        if let Self::Leaf { leaf, .. } = self {
            Some(*leaf)
        } else {
            None
        }
    }

    /// return the underlying block paylod commitment if it exists
    #[must_use]
    pub fn get_payload_commitment(&self) -> Option<VidCommitment> {
        if let Self::DA { payload_commitment } = self {
            Some(*payload_commitment)
        } else {
            None
        }
    }
}

impl<TYPES: NodeType> Deref for View<TYPES> {
    type Target = ViewInner<TYPES>;

    fn deref(&self) -> &Self::Target {
        &self.view_inner
    }
}

/// This exists so we can perform state transitions mutably
#[derive(Debug)]
pub struct View<TYPES: NodeType> {
    /// The view data. Wrapped in a struct so we can mutate
    pub view_inner: ViewInner<TYPES>,
}

/// A struct containing information about a finished round.
#[derive(Debug, Clone)]
pub struct RoundFinishedEvent<TYPES: NodeType> {
    /// The round that finished
    pub view_number: TYPES::Time,
}

/// Whether or not to stop inclusively or exclusively when walking
#[derive(Copy, Clone, Debug)]
pub enum Terminator<T> {
    /// Stop right before this view number
    Exclusive(T),
    /// Stop including this view number
    Inclusive(T),
}
