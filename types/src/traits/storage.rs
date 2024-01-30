//! Abstraction over on-disk storage of node state

use super::node_implementation::NodeType;
use crate::{data::Leaf, simple_certificate::QuorumCertificate, vote::HasViewNumber};
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
pub trait Storage<TYPES>: Clone + Send + Sync + Sized + 'static
where
    TYPES: NodeType + 'static,
{
    /// Append the list of views to this storage
    async fn append(&self, views: Vec<ViewEntry<TYPES>>) -> Result;
    /// Cleans up the storage up to the given view. The given view number will still persist in this storage afterwards.
    async fn cleanup_storage_up_to_view(&self, view: TYPES::Time) -> Result<usize>;
    /// Get the latest anchored view
    async fn get_anchored_view(&self) -> Result<StoredView<TYPES>>;
    /// Commit this storage.
    async fn commit(&self) -> Result;

    /// Insert a single view. Shorthand for
    /// ```rust,ignore
    /// storage.append(vec![ViewEntry::Success(view)]).await
    /// ```
    async fn append_single_view(&self, view: StoredView<TYPES>) -> Result {
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
pub trait TestableStorage<TYPES>: Clone + Send + Sync + Storage<TYPES>
where
    TYPES: NodeType + 'static,
{
    /// Create ephemeral storage
    /// Will be deleted/lost immediately after storage is dropped
    /// # Errors
    /// Errors if it is not possible to construct temporary storage.
    fn construct_tmp_storage() -> Result<Self>;

    /// Return the full internal state. This is useful for debugging.
    async fn get_full_state(&self) -> StorageState<TYPES>;
}

/// An internal representation of the data stored in a [`Storage`].
///
/// This should only be used for testing, never in production code.
#[derive(Debug, PartialEq)]
pub struct StorageState<TYPES: NodeType> {
    /// The views that have been successful
    pub stored: BTreeMap<TYPES::Time, StoredView<TYPES>>,
    /// The views that have failed
    pub failed: BTreeSet<TYPES::Time>,
}

/// An entry to `Storage::append`. This makes it possible to commit both succeeded and failed views at the same time
#[derive(Debug, PartialEq)]
pub enum ViewEntry<TYPES>
where
    TYPES: NodeType,
{
    /// A succeeded view
    Success(StoredView<TYPES>),
    /// A failed view
    Failed(TYPES::Time),
    // future improvement:
    // InProgress(InProgressView),
}

impl<TYPES> From<StoredView<TYPES>> for ViewEntry<TYPES>
where
    TYPES: NodeType,
{
    fn from(view: StoredView<TYPES>) -> Self {
        Self::Success(view)
    }
}

impl<TYPES> From<Leaf<TYPES>> for ViewEntry<TYPES>
where
    TYPES: NodeType,
{
    fn from(leaf: Leaf<TYPES>) -> Self {
        Self::Success(StoredView::from(leaf))
    }
}

/// A view stored in the [`Storage`]
#[derive(Clone, Debug, Derivative)]
#[derivative(PartialEq)]
pub struct StoredView<TYPES: NodeType> {
    /// The view number of this view
    pub view_number: TYPES::Time,
    /// The parent of this view
    pub parent: Commitment<Leaf<TYPES>>,
    /// The justify QC of this view. See the hotstuff paper for more information on this.
    pub justify_qc: QuorumCertificate<TYPES>,
    /// Block header.
    pub block_header: TYPES::BlockHeader,
    /// Optional block payload.
    ///
    /// It may be empty for nodes not in the DA committee.
    pub block_payload: Option<TYPES::BlockPayload>,
    /// the proposer id
    #[derivative(PartialEq = "ignore")]
    pub proposer_id: TYPES::SignatureKey,
}

impl<TYPES> StoredView<TYPES>
where
    TYPES: NodeType,
{
    /// Create a new `StoredView` from the given QC, `BlockHeader`, `BlockPayload` and State.
    ///
    /// Note that this will set the `parent` to `LeafHash::default()`, so this will not have a
    /// parent.
    pub fn from_qc_block_and_state(
        qc: QuorumCertificate<TYPES>,
        block_header: TYPES::BlockHeader,
        block_payload: Option<TYPES::BlockPayload>,
        parent_commitment: Commitment<Leaf<TYPES>>,
        proposer_id: TYPES::SignatureKey,
    ) -> Self {
        Self {
            view_number: qc.get_view_number(),
            parent: parent_commitment,
            justify_qc: qc,
            block_header,
            block_payload,
            proposer_id,
        }
    }
}
