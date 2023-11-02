//! Abstraction over on-disk storage of node state

use super::{node_implementation::NodeType, signature_key::EncodedPublicKey};
use crate::{
    data::LeafType,
    simple_certificate::QuorumCertificate2,
    traits::{election::SignedCertificate, BlockPayload},
    vote2::HasViewNumber,
};
use async_trait::async_trait;
use commit::Commitment;
use derivative::Derivative;
use snafu::Snafu;
use std::collections::{BTreeMap, BTreeSet};
/// Errors that can occur in the storage layer.
#[derive(Clone, Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum StorageError {
    /// No genesis view was inserted
    NoGenesisView,
}

/// Result for a storage type
pub type Result<T = ()> = std::result::Result<T, StorageError>;

/// Abstraction over on disk persistence of node state
///
/// This should be a cloneable handle to an underlying storage, with each clone pointing to the same
/// underlying storage.
///
/// This trait has been constructed for object saftey over convenience.
#[async_trait]
pub trait Storage<TYPES, LEAF>: Clone + Send + Sync + Sized + 'static
where
    TYPES: NodeType + 'static,
    LEAF: LeafType<NodeType = TYPES> + 'static,
{
    /// Append the list of views to this storage
    async fn append(&self, views: Vec<ViewEntry<TYPES, LEAF>>) -> Result;
    /// Cleans up the storage up to the given view. The given view number will still persist in this storage afterwards.
    async fn cleanup_storage_up_to_view(&self, view: TYPES::Time) -> Result<usize>;
    /// Get the latest anchored view
    async fn get_anchored_view(&self) -> Result<StoredView<TYPES, LEAF>>;
    /// Commit this storage.
    async fn commit(&self) -> Result;

    /// Insert a single view. Shorthand for
    /// ```rust,ignore
    /// storage.append(vec![ViewEntry::Success(view)]).await
    /// ```
    async fn append_single_view(&self, view: StoredView<TYPES, LEAF>) -> Result {
        self.append(vec![ViewEntry::Success(view)]).await
    }
    // future improvement:
    // async fn get_future_views(&self) -> Vec<FutureView>;
    //     async fn add_transaction(&self, transactions: Transaction) -> TransactionHash;
    //     async fn get_transactions(&self) -> Vec<Transaction>;
    //     async fn get_transaction(&self, hash: TransactionHash) -> Option<Transaction>;
    //     async fn remove_transaction(&self, hash: TransactionHash) -> Option<Transaction>;
}

/// Extra requirements on Storage implementations required for testing
#[async_trait]
pub trait TestableStorage<TYPES, LEAF: LeafType<NodeType = TYPES>>:
    Clone + Send + Sync + Storage<TYPES, LEAF>
where
    TYPES: NodeType + 'static,
{
    /// Create ephemeral storage
    /// Will be deleted/lost immediately after storage is dropped
    /// # Errors
    /// Errors if it is not possible to construct temporary storage.
    fn construct_tmp_storage() -> Result<Self>;

    /// Return the full internal state. This is useful for debugging.
    async fn get_full_state(&self) -> StorageState<TYPES, LEAF>;
}

/// An internal representation of the data stored in a [`Storage`].
///
/// This should only be used for testing, never in production code.
#[derive(Debug, PartialEq)]
pub struct StorageState<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> {
    /// The views that have been successful
    pub stored: BTreeMap<TYPES::Time, StoredView<TYPES, LEAF>>,
    /// The views that have failed
    pub failed: BTreeSet<TYPES::Time>,
}

/// An entry to `Storage::append`. This makes it possible to commit both succeeded and failed views at the same time
#[derive(Debug, PartialEq)]
pub enum ViewEntry<TYPES, LEAF>
where
    TYPES: NodeType,
    LEAF: LeafType<NodeType = TYPES>,
{
    /// A succeeded view
    Success(StoredView<TYPES, LEAF>),
    /// A failed view
    Failed(TYPES::Time),
    // future improvement:
    // InProgress(InProgressView),
}

impl<TYPES, LEAF> From<StoredView<TYPES, LEAF>> for ViewEntry<TYPES, LEAF>
where
    TYPES: NodeType,
    LEAF: LeafType<NodeType = TYPES>,
{
    fn from(view: StoredView<TYPES, LEAF>) -> Self {
        Self::Success(view)
    }
}

impl<TYPES, LEAF> From<LEAF> for ViewEntry<TYPES, LEAF>
where
    TYPES: NodeType,
    LEAF: LeafType<NodeType = TYPES>,
{
    fn from(leaf: LEAF) -> Self {
        Self::Success(StoredView::from(leaf))
    }
}

/// A view stored in the [`Storage`]
#[derive(Clone, Debug, Derivative)]
#[derivative(PartialEq)]
pub struct StoredView<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> {
    /// The view number of this view
    pub view_number: TYPES::Time,
    /// The parent of this view
    pub parent: Commitment<LEAF>,
    /// The justify QC of this view. See the hotstuff paper for more information on this.
    pub justify_qc: QuorumCertificate2<TYPES, LEAF>,
    /// The state of this view
    pub state: LEAF::MaybeState,
    /// Block header.
    pub block_header: TYPES::BlockHeader,
    /// Optional block payload.
    ///
    /// It may be empty for nodes not in the DA committee.
    pub block_payload: Option<TYPES::BlockPayload>,
    /// transactions rejected in this view
    pub rejected: Vec<TYPES::Transaction>,
    /// the timestamp this view was recv-ed in nanonseconds
    #[derivative(PartialEq = "ignore")]
    pub timestamp: i128,
    /// the proposer id
    #[derivative(PartialEq = "ignore")]
    pub proposer_id: EncodedPublicKey,
}

impl<TYPES, LEAF> StoredView<TYPES, LEAF>
where
    TYPES: NodeType,
    LEAF: LeafType<NodeType = TYPES>,
{
    /// Create a new `StoredView` from the given QC, `BlockHeader`, `BlockPayload` and State.
    ///
    /// Note that this will set the `parent` to `LeafHash::default()`, so this will not have a
    /// parent.
    pub fn from_qc_block_and_state(
        qc: QuorumCertificate2<TYPES, LEAF>,
        block_header: TYPES::BlockHeader,
        block_payload: Option<TYPES::BlockPayload>,
        state: LEAF::MaybeState,
        parent_commitment: Commitment<LEAF>,
        rejected: Vec<<TYPES::BlockPayload as BlockPayload>::Transaction>,
        proposer_id: EncodedPublicKey,
    ) -> Self {
        Self {
            view_number: qc.get_view_number(),
            parent: parent_commitment,
            justify_qc: qc,
            state,
            block_header,
            block_payload,
            rejected,
            timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
            proposer_id,
        }
    }
}
