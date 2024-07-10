use core::fmt::Debug;

use serde::{Deserialize, Serialize};
use snafu::Snafu;

// TODO: Remove issue #1681 (https://github.com/EspressoSystems/espresso-sequencer/issues/1681)
#[derive(Clone, Debug, Snafu, Deserialize, Serialize)]
pub enum Error {
    // TODO: Can we nest these errors in a `ValidationError` to group them?

    // Parent state commitment of block doesn't match current state commitment
    IncorrectParent,

    // New view number isn't strictly after current view
    IncorrectView,

    // Genesis block either has zero or more than one transaction
    GenesisWrongSize,

    // Genesis transaction not present in genesis block
    MissingGenesis,

    // Genesis transaction in non-genesis block
    UnexpectedGenesis,

    // Merkle tree error
    MerkleTreeError { error: String },

    BlockBuilding,
}
