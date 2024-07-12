//! Update loop for query API state.

use async_std::sync::{Arc, RwLock};
use async_trait::async_trait;
use derivative::Derivative;
use derive_more::From;
use espresso_types::{
    v0::traits::{EventConsumer, SequencerPersistence},
    PubKey,
};
use hotshot::types::Event;
use hotshot_query_service::data_source::{UpdateDataSource, VersionedDataSource};
use hotshot_types::traits::network::ConnectedNetwork;
use std::fmt::Debug;
use vbs::version::StaticVersionType;

use super::{data_source::SequencerDataSource, StorageState};
use crate::SeqTypes;

#[derive(Derivative, From)]
#[derivative(Clone(bound = ""), Debug(bound = "D: Debug"))]
pub(crate) struct ApiEventConsumer<N, P, D, Ver>
where
    N: ConnectedNetwork<PubKey>,
    P: SequencerPersistence,
    Ver: StaticVersionType,
{
    inner: Arc<RwLock<StorageState<N, P, D, Ver>>>,
}

#[async_trait]
impl<N, P, D, Ver> EventConsumer for ApiEventConsumer<N, P, D, Ver>
where
    N: ConnectedNetwork<PubKey>,
    P: SequencerPersistence,
    D: SequencerDataSource + Debug + Send + Sync,
    Ver: StaticVersionType,
{
    async fn handle_event(&self, event: &Event<SeqTypes>) -> anyhow::Result<()> {
        let mut state = self.inner.write().await;

        // If update results in an error, revert to undo partial state changes. We will continue
        // streaming events, as we can update our state based on future events and then filling in
        // the missing part of the state later, by fetching from a peer.
        if let Err(err) = update_state(&mut *state, event).await {
            tracing::error!(
                ?event,
                %err,
                "failed to update API state",
            );
            state.revert().await;
            Err(err)
        } else {
            Ok(())
        }
    }
}

async fn update_state<N, P, D, Ver: StaticVersionType>(
    state: &mut StorageState<N, P, D, Ver>,
    event: &Event<SeqTypes>,
) -> anyhow::Result<()>
where
    N: ConnectedNetwork<PubKey>,
    P: SequencerPersistence,
    D: SequencerDataSource + Send + Sync,
{
    state.update(event).await?;
    state.commit().await?;

    Ok(())
}
