//! Shared types dealing with block information

use committable::Commitment;
use hotshot_types::{
    data::Leaf, traits::node_implementation::NodeType, utils::BuilderCommitment, vid::VidCommitment,
};

/// Unique identifier for a block
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BlockId<Types: NodeType> {
    /// Block hash
    pub hash: BuilderCommitment,
    /// Block view
    pub view: Types::View,
}

impl<Types: NodeType> std::fmt::Display for BlockId<Types> {
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
pub struct BuilderStateId<Types: NodeType> {
    /// View number of the parent block
    pub parent_view: Types::View,
    /// VID commitment of the parent block
    pub parent_commitment: VidCommitment,
}

impl<Types: NodeType> std::fmt::Display for BuilderStateId<Types> {
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
pub struct ParentBlockReferences<Types: NodeType> {
    pub view_number: Types::View,
    pub vid_commitment: VidCommitment,
    pub leaf_commit: Commitment<Leaf<Types>>,
    pub builder_commitment: BuilderCommitment,
}

// implement display for the derived info
impl<Types: NodeType> std::fmt::Display for ParentBlockReferences<Types> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "View Number: {:?}", self.view_number)
    }
}
