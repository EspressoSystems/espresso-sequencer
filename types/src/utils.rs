//! Utility functions, type aliases, helper structs and enum definitions.

use crate::{
    data::{LeafBlock, LeafType},
    traits::node_implementation::NodeType,
};
use commit::Commitment;
use std::ops::Deref;

/// A view's state
#[derive(Debug)]
pub enum ViewInner<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> {
    /// A pending view with an available block but not leaf proposal yet.
    ///
    /// Storing this state allows us to garbage collect blocks for views where a proposal is never
    /// made. This saves memory when a leader fails and subverts a DoS attack where malicious
    /// leaders repeatedly request availability for blocks that they never propose.
    DA {
        /// Available block.
        block: Commitment<LeafBlock<LEAF>>,
    },
    /// Undecided view
    Leaf {
        /// Proposed leaf
        leaf: Commitment<LEAF>,
    },
    /// Leaf has failed
    Failed,
}

impl<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> ViewInner<TYPES, LEAF> {
    /// return the underlying leaf hash if it exists
    #[must_use]
    pub fn get_leaf_commitment(&self) -> Option<Commitment<LEAF>> {
        if let Self::Leaf { leaf } = self {
            Some(*leaf)
        } else {
            None
        }
    }

    /// return the underlying block hash if it exists
    #[must_use]
    pub fn get_block_commitment(&self) -> Option<Commitment<LeafBlock<LEAF>>> {
        if let Self::DA { block } = self {
            Some(*block)
        } else {
            None
        }
    }
}

impl<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> Deref for View<TYPES, LEAF> {
    type Target = ViewInner<TYPES, LEAF>;

    fn deref(&self) -> &Self::Target {
        &self.view_inner
    }
}

/// This exists so we can perform state transitions mutably
#[derive(Debug)]
pub struct View<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> {
    /// The view data. Wrapped in a struct so we can mutate
    pub view_inner: ViewInner<TYPES, LEAF>,
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
