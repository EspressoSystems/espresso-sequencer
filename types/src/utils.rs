//! Utility functions, type aliases, helper structs and enum definitions.

use crate::{
    data::{Leaf, VidCommitment},
    traits::node_implementation::NodeType,
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use commit::Commitment;
use digest::OutputSizeUser;
use sha2::Digest;
use std::ops::Deref;
use tagged_base64::tagged;
use typenum::Unsigned;

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
        /// Validated state.
        state: TYPES::ValidatedState,
    },
    /// Leaf has failed
    Failed,
}

impl<TYPES: NodeType> ViewInner<TYPES> {
    /// Return the underlying undecide leaf view if it exists.
    pub fn get_leaf(&self) -> Option<(Commitment<Leaf<TYPES>>, &TYPES::ValidatedState)> {
        if let Self::Leaf { leaf, state } = self {
            Some((*leaf, state))
        } else {
            None
        }
    }

    /// return the underlying leaf hash if it exists
    #[must_use]
    pub fn get_leaf_commitment(&self) -> Option<Commitment<Leaf<TYPES>>> {
        if let Self::Leaf { leaf, .. } = self {
            Some(*leaf)
        } else {
            None
        }
    }

    /// return the underlying validated state if it exists
    #[must_use]
    pub fn get_state(&self) -> Option<&TYPES::ValidatedState> {
        if let Self::Leaf { state, .. } = self {
            Some(state)
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

/// Type alias for byte array of SHA256 digest length
type Sha256Digest = [u8; <sha2::Sha256 as OutputSizeUser>::OutputSize::USIZE];

#[tagged("BUILDER_COMMITMENT")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
/// Commitment that builders use to sign block options.
/// A thin wrapper around a Sha256 digest.
pub struct BuilderCommitment(Sha256Digest);

impl BuilderCommitment {
    /// Create new commitment for `data`
    pub fn from_bytes(data: impl AsRef<[u8]>) -> Self {
        Self(sha2::Sha256::digest(data.as_ref()).into())
    }

    /// Create a new commitment from a raw Sha256 digest
    pub fn from_raw_digest(digest: impl Into<Sha256Digest>) -> Self {
        Self(digest.into())
    }
}

impl AsRef<Sha256Digest> for BuilderCommitment {
    fn as_ref(&self) -> &Sha256Digest {
        &self.0
    }
}
