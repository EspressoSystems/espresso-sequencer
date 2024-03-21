//! Update loop for query API state.

use super::{data_source::SequencerDataSource, StorageState};
use crate::{
    network,
    state::{BlockMerkleTree, FeeAccount, FeeMerkleTree},
    Delta, SeqTypes, ValidatedState,
};
use anyhow::Context;
use async_std::sync::{Arc, RwLock};
use async_trait::async_trait;
use futures::stream::{Stream, StreamExt};
use hotshot::types::Event;
use hotshot_query_service::{
    data_source::{UpdateDataSource, VersionedDataSource},
    merklized_state::{UpdateStateData, UpdateStateStorage},
    Leaf,
};
use jf_primitives::merkle_tree::{MerkleTreeScheme, ToTraversalPath, UniversalMerkleTreeScheme};
use versioned_binary_serialization::version::StaticVersionType;

pub(super) async fn update_loop<N, D, Ver: StaticVersionType>(
    state: Arc<RwLock<StorageState<N, D, Ver>>>,
    mut events: impl Stream<Item = Event<SeqTypes>> + Unpin,
) where
    N: network::Type,
    D: SequencerDataSource + Send + Sync,
{
    tracing::debug!("waiting for event");
    while let Some(event) = events.next().await {
        let mut state = state.write().await;

        // If update results in an error, revert to undo partial state changes. We will continue
        // streaming events, as we can update our state based on future events and then filling in
        // the missing part of the state later, by fetching from a peer.
        if let Err(err) = update_state(&mut *state, &event).await {
            tracing::error!(
                ?event,
                %err,
                "failed to update API state",
            );
            state.revert().await;
        }
    }
    tracing::warn!("end of HotShot event stream, updater task will exit");
}

async fn update_state<N, D, Ver: StaticVersionType>(
    state: &mut StorageState<N, D, Ver>,
    event: &Event<SeqTypes>,
) -> anyhow::Result<()>
where
    N: network::Type,
    D: SequencerDataSource + Send + Sync,
{
    state.update(event).await?;
    state.commit().await?;

    Ok(())
}

#[async_trait]
impl<D: UpdateStateData<SeqTypes, BlockMerkleTree> + UpdateStateData<SeqTypes, FeeMerkleTree>>
    UpdateStateStorage<SeqTypes, D> for ValidatedState
{
    async fn update_storage(
        &self,
        storage: &mut D,
        leaf: &Leaf<SeqTypes>,
        delta: Arc<Delta>,
    ) -> anyhow::Result<()> {
        let block_number = leaf.get_height();
        let ValidatedState {
            fee_merkle_tree,
            block_merkle_tree,
        } = self;

        let Delta {
            blocks_delta,
            fees_delta,
        } = delta.as_ref();

        // Insert block merkle tree nodes
        for delta in blocks_delta {
            let (_, proof) = block_merkle_tree
                .lookup(delta)
                .expect_ok()
                .context("Index not found in block merkle tree")?;
            let path = <u64 as ToTraversalPath<typenum::U3>>::to_traversal_path(
                delta,
                block_merkle_tree.height(),
            );

            <D as UpdateStateData<SeqTypes, BlockMerkleTree>>::insert_merkle_nodes(
                storage,
                proof.proof,
                path,
                block_number,
            )
            .await
            .context("failed to insert merkle nodes for block merkle tree")?;
        }

        // Insert fee merkle tree nodes
        for delta in fees_delta {
            let (_, proof) = fee_merkle_tree
                .universal_lookup(delta)
                .expect_ok()
                .context("Index not found in fee merkle tree")?;
            let path: Vec<usize> =
                <FeeAccount as ToTraversalPath<typenum::U256>>::to_traversal_path(
                    delta,
                    fee_merkle_tree.height(),
                );

            <D as UpdateStateData<SeqTypes, FeeMerkleTree>>::insert_merkle_nodes(
                storage,
                proof.proof,
                path,
                block_number,
            )
            .await
            .context("failed to insert merkle nodes for fee merkle tree")?;
        }

        Ok(())
    }
}
