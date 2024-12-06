use std::{future::Future, hash::Hash, sync::Arc};

use futures::{
    future::{RemoteHandle, Shared},
    FutureExt,
};
use hotshot::traits::BlockPayload;
use hotshot_types::{
    data::{DaProposal, QuorumProposal2},
    traits::{
        block_contents::BlockHeader, node_implementation::NodeType,
        signature_key::BuilderSignatureKey,
    },
    utils::BuilderCommitment,
};
use tokio::{spawn, sync::Notify};

pub mod rotating_set;
pub use rotating_set::RotatingSet;

pub mod event_serivce_wrapper;
pub use event_serivce_wrapper::EventServiceStream;

/// A convenience type alias for a tuple of builder keys
/// `(public_key, private_key)`
pub type BuilderKeys<Types> = (
    <Types as NodeType>::BuilderSignatureKey,
    <<Types as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::BuilderPrivateKey,
);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ProposalId<Types>
where
    Types: NodeType,
{
    view_number: Types::View,
    builder_commitment: BuilderCommitment,
}

impl<Types> ProposalId<Types>
where
    Types: NodeType,
{
    pub fn from_quorum_proposal(proposal: &QuorumProposal2<Types>) -> Self {
        Self {
            builder_commitment: proposal.block_header.builder_commitment(),
            view_number: proposal.view_number,
        }
    }

    pub fn from_da_proposal(proposal: &DaProposal<Types>) -> Self {
        let builder_commitment = <Types::BlockPayload as BlockPayload<Types>>::from_bytes(
            &proposal.encoded_transactions,
            &proposal.metadata,
        )
        .builder_commitment(&proposal.metadata);

        Self {
            builder_commitment,
            view_number: proposal.view_number,
        }
    }

    pub fn view_number(&self) -> <Types as NodeType>::View {
        self.view_number
    }

    pub fn builder_commitment(&self) -> &BuilderCommitment {
        &self.builder_commitment
    }
}

#[derive(derive_more::Debug, Clone)]
pub struct WaitAndKeep<T>
where
    T: Clone + Sync + Send + 'static,
{
    handle: Shared<RemoteHandle<T>>,
    notifier: Arc<Notify>,
}

impl<T> WaitAndKeep<T>
where
    T: Clone + Sync + Send + 'static,
{
    /// Creates a new [`WaitAndKeep`] wrapping the provided future.
    /// Future will be essentially paused until [`WaitAndKeep::start`] is called
    pub fn new<F: Future<Output = T> + Send + 'static>(fut: F) -> Self {
        let notifier = Arc::new(Notify::new());
        let (fut, handle) = {
            let notifier = Arc::clone(&notifier);
            async move {
                let _ = notifier.notified().await;
                fut.await
            }
            .remote_handle()
        };
        spawn(fut);
        Self {
            notifier,
            handle: handle.shared(),
        }
    }

    /// Signals the underlying future to start running
    pub fn start(&self) {
        self.notifier.notify_one();
    }

    /// Will consume self and return result of underlying future
    pub async fn resolve(self) -> T {
        self.start();
        self.handle.await
    }
}
