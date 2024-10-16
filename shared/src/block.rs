//! Shared types dealing with block information

use committable::Commitment;
use hotshot_types::{
    data::Leaf, traits::node_implementation::NodeType, utils::BuilderCommitment, vid::VidCommitment,
};

/// Unique identifier for a block
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BlockId<TYPES: NodeType> {
    /// Block hash
    pub hash: BuilderCommitment,
    /// Block view
    pub view: TYPES::Time,
}

impl<TYPES: NodeType> std::fmt::Display for BlockId<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block({}@{})",
            hex::encode(self.hash.as_ref()),
            *self.view
        )
    }
}

/// Unique identifier for a builder state
///
/// Builder state is identified by the VID commitment
/// and view of the block it targets to extend, i.e.
/// builder with given state ID assumes blocks/bundles it's building
/// are going to be included immediately after the parent block.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BuilderStateId<TYPES: NodeType> {
    /// View number of the parent block
    pub parent_view: TYPES::Time,
    /// VID commitment of the parent block
    pub parent_commitment: VidCommitment,
}

impl<TYPES: NodeType> std::fmt::Display for BuilderStateId<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BuilderState({}@{})",
            self.parent_commitment, *self.parent_view
        )
    }
}

/// References to the parent block that is extended to spawn the new builder state.
#[derive(Debug, Clone)]
pub struct ParentBlockReferences<TYPES: NodeType> {
    pub view_number: TYPES::Time,
    pub vid_commitment: VidCommitment,
    pub leaf_commit: Commitment<Leaf<TYPES>>,
    pub builder_commitment: BuilderCommitment,
}

// implement display for the derived info
impl<TYPES: NodeType> std::fmt::Display for ParentBlockReferences<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "View Number: {:?}", self.view_number)
    }
}
