//! Update loop for query API state.

use super::{data_source::SequencerDataSource, StorageState};
use crate::{network, SeqTypes};
use async_std::sync::{Arc, RwLock};
use futures::stream::{Stream, StreamExt};
use hotshot::types::Event;
use hotshot_query_service::{
    data_source::{UpdateDataSource, VersionedDataSource},
    status::StatusDataSource,
};

pub(super) async fn update_loop<N, D>(
    state: Arc<RwLock<StorageState<N, D>>>,
    mut events: impl Stream<Item = Event<SeqTypes>> + Unpin,
) where
    N: network::Type,
    D: SequencerDataSource + Send + Sync,
{
    tracing::debug!("waiting for event");
    while let Some(event) = events.next().await {
        tracing::debug!("got event {:?}", event);

        // If update results in an error, program state is unrecoverable
        if let Err(err) = update_state(&mut *state.write().await, &event).await {
            tracing::error!(
                "failed to update event {:?}: {}; updater task will exit",
                event,
                err
            );
            panic!();
        }
    }
    tracing::warn!("end of HotShot event stream, updater task will exit");
}

async fn update_state<N, D>(
    state: &mut StorageState<N, D>,
    event: &Event<SeqTypes>,
) -> anyhow::Result<()>
where
    N: network::Type,
    D: SequencerDataSource + Send + Sync,
{
    // Remember the current block height, so we can update our local index
    // based on any new blocks that get added.
    let prev_block_height = state.block_height().await?;

    state.update(event).await?;
    state.inner_mut().refresh_indices(prev_block_height).await?;
    state.commit().await?;

    Ok(())
}
