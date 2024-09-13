//! Update loop for query API state.

use async_std::sync::Arc;
use espresso_types::{v0::traits::SequencerPersistence, PubKey};
use futures::stream::{Stream, StreamExt};
use hotshot::types::Event;
use hotshot_query_service::data_source::{Transaction, UpdateDataSource, VersionedDataSource};
use hotshot_types::traits::{network::ConnectedNetwork, node_implementation::Versions};

use super::{data_source::SequencerDataSource, StorageState};
use crate::SeqTypes;

pub(super) async fn update_loop<N, P, D, V: Versions>(
    state: Arc<StorageState<N, P, D, V>>,
    mut events: impl Stream<Item = Event<SeqTypes>> + Unpin,
) where
    N: ConnectedNetwork<PubKey>,
    P: SequencerPersistence,
    D: SequencerDataSource + Send + Sync,
    for<'a> D::Transaction<'a>: UpdateDataSource<SeqTypes>,
{
    tracing::debug!("waiting for event");
    while let Some(event) = events.next().await {
        if let Err(err) = update_state(&state, &event).await {
            tracing::error!(
                ?event,
                %err,
                "failed to update API state",
            );
        }
    }
    tracing::warn!("end of HotShot event stream, updater task will exit");
}

async fn update_state<N, P, D, V: Versions>(
    state: &StorageState<N, P, D, V>,
    event: &Event<SeqTypes>,
) -> anyhow::Result<()>
where
    N: ConnectedNetwork<PubKey>,
    P: SequencerPersistence,
    D: SequencerDataSource + Send + Sync,
    for<'a> D::Transaction<'a>: UpdateDataSource<SeqTypes>,
{
    let mut tx = state.write().await?;
    tx.update(event).await?;
    tx.commit().await?;

    Ok(())
}
