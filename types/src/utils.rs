//! Utility functions, type aliases, helper structs and enum definitions.

use std::{ops::Deref, sync::Arc};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use bincode::{
    config::{
        FixintEncoding, LittleEndian, RejectTrailing, WithOtherEndian, WithOtherIntEncoding,
        WithOtherLimit, WithOtherTrailing,
    },
    DefaultOptions, Options,
};
use committable::Commitment;
use digest::OutputSizeUser;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use tagged_base64::tagged;
use typenum::Unsigned;

use crate::{
    data::Leaf,
    traits::{node_implementation::NodeType, ValidatedState},
    vid::VidCommitment,
};

/// A view's state
#[derive(Debug, Deserialize, Serialize)]
#[serde(bound = "")]
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
        leaf: LeafCommitment<TYPES>,
        /// Validated state.
        state: Arc<TYPES::ValidatedState>,
        /// Optional state delta.
        delta: Option<Arc<<TYPES::ValidatedState as ValidatedState<TYPES>>::Delta>>,
    },
    /// Leaf has failed
    Failed,
}
impl<TYPES: NodeType> Clone for ViewInner<TYPES> {
    fn clone(&self) -> Self {
        match self {
            Self::DA { payload_commitment } => Self::DA {
                payload_commitment: *payload_commitment,
            },
            Self::Leaf { leaf, state, delta } => Self::Leaf {
                leaf: *leaf,
                state: Arc::clone(state),
                delta: delta.clone(),
            },
            Self::Failed => Self::Failed,
        }
    }
}
/// The hash of a leaf.
type LeafCommitment<TYPES> = Commitment<Leaf<TYPES>>;

/// Optional validated state and state delta.
pub type StateAndDelta<TYPES> = (
    Option<Arc<<TYPES as NodeType>::ValidatedState>>,
    Option<Arc<<<TYPES as NodeType>::ValidatedState as ValidatedState<TYPES>>::Delta>>,
);

impl<TYPES: NodeType> ViewInner<TYPES> {
    /// Return the underlying undecide leaf commitment and validated state if they exist.
    #[must_use]
    pub fn get_leaf_and_state(
        &self,
    ) -> Option<(LeafCommitment<TYPES>, &Arc<TYPES::ValidatedState>)> {
        if let Self::Leaf { leaf, state, .. } = self {
            Some((*leaf, state))
        } else {
            None
        }
    }

    /// return the underlying leaf hash if it exists
    #[must_use]
    pub fn get_leaf_commitment(&self) -> Option<LeafCommitment<TYPES>> {
        if let Self::Leaf { leaf, .. } = self {
            Some(*leaf)
        } else {
            None
        }
    }

    /// return the underlying validated state if it exists
    #[must_use]
    pub fn get_state(&self) -> Option<&Arc<TYPES::ValidatedState>> {
        if let Self::Leaf { state, .. } = self {
            Some(state)
        } else {
            None
        }
    }

    /// Return the underlying validated state and state delta if they exist.
    #[must_use]
    pub fn get_state_and_delta(&self) -> StateAndDelta<TYPES> {
        if let Self::Leaf { state, delta, .. } = self {
            (Some(Arc::clone(state)), delta.clone())
        } else {
            (None, None)
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
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(bound = "")]
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

/// For the wire format, we use bincode with the following options:
///   - No upper size limit
///   - Little endian encoding
///   - Varint encoding
///   - Reject trailing bytes
#[allow(clippy::type_complexity)]
#[must_use]
#[allow(clippy::type_complexity)]
pub fn bincode_opts() -> WithOtherTrailing<
    WithOtherIntEncoding<
        WithOtherEndian<WithOtherLimit<DefaultOptions, bincode::config::Infinite>, LittleEndian>,
        FixintEncoding,
    >,
    RejectTrailing,
> {
    bincode::DefaultOptions::new()
        .with_no_limit()
        .with_little_endian()
        .with_fixint_encoding()
        .reject_trailing_bytes()
}
