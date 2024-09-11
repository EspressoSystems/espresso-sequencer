//! Update loop for query API state.

use async_std::sync::Arc;
use async_trait::async_trait;
use derivative::Derivative;
use derive_more::From;
use espresso_types::{v0::traits::SequencerPersistence, PubKey};
use hotshot::types::Event;
use hotshot_query_service::data_source::{Transaction, UpdateDataSource, VersionedDataSource};
use hotshot_types::traits::{network::ConnectedNetwork, node_implementation::Versions};
use std::fmt::Debug;

use super::{data_source::SequencerDataSource, StorageState};
use crate::{EventConsumer, SeqTypes};

#[derive(Derivative, From)]
#[derivative(Clone(bound = ""), Debug(bound = "D: Debug"))]
pub(crate) struct ApiEventConsumer<N, P, D, V>
where
    N: ConnectedNetwork<PubKey>,
    P: SequencerPersistence,
    V: Versions,
{
    inner: Arc<StorageState<N, P, D, V>>,
}

#[async_trait]
impl<N, P, D, V> EventConsumer for ApiEventConsumer<N, P, D, V>
where
    N: ConnectedNetwork<PubKey>,
    P: SequencerPersistence,
    D: SequencerDataSource + Debug + Send + Sync + 'static,
    for<'a> D::Transaction<'a>: UpdateDataSource<SeqTypes>,
    V: Versions,
{
    async fn handle_event(&self, event: &Event<SeqTypes>) -> anyhow::Result<()> {
        if let Err(err) = update_state(&self.inner, event).await {
            tracing::error!(
                ?event,
                %err,
                "failed to update API state",
            );
            Err(err)
        } else {
            Ok(())
        }
    }
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
