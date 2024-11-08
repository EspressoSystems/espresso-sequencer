use std::sync::Arc;

use alloy::primitives::Address;
use async_std::{sync::RwLock, task::JoinHandle};
use circular_buffer::CircularBuffer;
use espresso_types::{NamespaceId, Payload, SeqTypes};
use futures::{channel::mpsc::SendError, Sink, SinkExt, Stream, StreamExt};
use hotshot_query_service::availability::{BlockQueryData, QueryablePayload};
use hotshot_types::traits::block_contents::BlockHeader;
use sqlx::types::time::OffsetDateTime;

use super::{
    espresso_inscription::{
        ChainDetails, InscriptionAndChainDetails, InscriptionAndSignatureFromService,
    },
    storage::InscriptionPersistence,
    validate_inscription_and_signature_from_service,
};

/// MAX_LOCAL_INSCRIPTION_HISTORY represents the last N records that are stored within the
/// DataState structure for the various different sample types.
pub const MAX_LOCAL_INSCRIPTION_HISTORY: usize = 100;

#[derive(Debug, Clone, PartialEq)]
pub enum SubmitInscriptionError {
    BufferIsFull,
}

/// [DataState] represents the state of the data that is being stored within
/// the service.
pub struct DataState<Persistence> {
    latest_inscriptions: CircularBuffer<MAX_LOCAL_INSCRIPTION_HISTORY, InscriptionAndChainDetails>,

    persistence: Arc<Persistence>,

    address: Address,
}

impl<Persistence> DataState<Persistence> {
    /// [new] creates a new [DataState] structure that will store the latest
    /// inscriptions that are being processed by the service.
    pub fn new(
        latest_inscriptions: CircularBuffer<
            MAX_LOCAL_INSCRIPTION_HISTORY,
            InscriptionAndChainDetails,
        >,
        persistence: Arc<Persistence>,
        address: Address,
    ) -> Self {
        Self {
            latest_inscriptions,
            persistence,
            address,
        }
    }

    /// [latest_inscriptions] returns an iterator that will iterate over the
    /// latest inscriptions that are stored within the [DataState].
    pub fn latest_inscriptions(&self) -> impl Iterator<Item = &InscriptionAndChainDetails> {
        self.latest_inscriptions.iter()
    }

    /// [add_latest_inscription] adds a new inscription to the [DataState].
    /// If the buffer is full, an error will be returned.
    pub fn add_latest_inscription(&mut self, block: InscriptionAndChainDetails) {
        self.latest_inscriptions.push_back(block);
    }

    /// [address] returns the address that is associated with the [DataState].
    pub fn address(&self) -> Address {
        self.address
    }

    /// [current_inscriptions] returns the current inscriptions that are stored
    /// within the [DataState].
    pub fn current_inscriptions(&self) -> Vec<InscriptionAndChainDetails> {
        self.latest_inscriptions.iter().cloned().collect()
    }

    pub fn persistence(&self) -> &Persistence {
        &self.persistence
    }
}

/// [ProcessBlockError] represents the error that can occur when processing
/// a [Block].
#[derive(Debug)]
pub enum ProcessBlockError {
    BlockSendError(SendError),
}

impl std::fmt::Display for ProcessBlockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessBlockError::BlockSendError(err) => {
                write!(f, "error sending block detail to sender: {}", err)
            }
        }
    }
}

impl std::error::Error for ProcessBlockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProcessBlockError::BlockSendError(err) => Some(err),
        }
    }
}

/// [process_incoming_block] is a helper function that will process an incoming
/// [Block] and update the [DataState] with the new information.
/// Additionally, the block that is contained within the [Block] will be
/// computed into a [BlockDetail] and sent to the [Sink] so that it can be
/// processed for real-time considerations.
async fn process_incoming_block<BDSink, Persistence, E>(
    inscription_namespace: NamespaceId,
    block: BlockQueryData<SeqTypes>,
    data_state: Arc<RwLock<DataState<Persistence>>>,
    mut inscription_sender: BDSink,
) -> Result<(), ProcessBlockError>
where
    Payload: QueryablePayload<SeqTypes>,
    E: std::fmt::Display + std::fmt::Debug,
    BDSink: Sink<InscriptionAndChainDetails, Error = E> + Unpin,
    Persistence: InscriptionPersistence,
{
    let mut inscriptions = Vec::<InscriptionAndChainDetails>::new();

    let address = {
        let data_state_read_lock_guard = data_state.read().await;
        data_state_read_lock_guard.address()
    };

    for (offset, (_, transaction)) in block.payload().enumerate(block.metadata()).enumerate() {
        let block_height = block.header().block_number();

        if transaction.namespace() != inscription_namespace {
            // Skip anything that isn't in the correct namespace
            continue;
        }

        tracing::debug!("processing transaction with correct namespace ({block_height}-{offset})");

        let decode_result =
            bincode::deserialize::<InscriptionAndSignatureFromService>(transaction.payload());
        let Ok(inscription_and_signature) = decode_result else {
            // We failed to decode the transaction, this can happen if some other
            // service is utilizing our namespace id.
            tracing::info!(
                "failed to decode inscription from transaction ({block_height}-{offset}), is not the type of data expected.  This indicates someone else is also using our namespace id, or an alternative serialization scheme",
            );
            continue;
        };

        // Alright, we have a valid inscription, but did we make it, or is it
        // from some other third party?

        let validation_result =
            validate_inscription_and_signature_from_service(&inscription_and_signature, address);

        if let Err(err) = validation_result {
            // We have an error, the specific error type does not matter that
            // much.  Ultimately it means that this is a message we didn't
            // sign.

            tracing::info!("skipping inscription from transaction ({block_height}-{offset}), validation of inscription failed: {}", err);
            continue;
        }

        inscriptions.push(InscriptionAndChainDetails {
            inscription: inscription_and_signature.inscription,
            chain_details: ChainDetails {
                block: block.header().block_number(),
                offset: offset as u64,
            },
        });
    }

    {
        let mut data_state_write_lock_guard = data_state.write().await;
        for inscription in &inscriptions {
            data_state_write_lock_guard.add_latest_inscription(inscription.clone());
        }
    }

    if inscriptions.is_empty() {
        // We have no inscriptions to process
        return Ok(());
    }

    let inscriptions_count = inscriptions.len();
    for inscription in inscriptions {
        {
            let state = data_state.read_arc().await;
            if let Err(err) = state
                .persistence()
                .record_confirmed_inscription_and_chain_details(&inscription)
                .await
            {
                tracing::error!(
                    "failed to record confirmed inscription and chain details, encountered error: {:?}",
                    err
                );
            }
        }

        let feed_result = inscription_sender.feed(inscription).await;
        if let Err(err) = feed_result {
            tracing::error!(
                "failed to enqueue inscription for dissemination, encountered error: {:?}",
                err
            );
            // We skipped an inscription?
            continue;
        }
    }

    let Err(err) = inscription_sender.flush().await else {
        // We have an error flushing the sink.
        tracing::debug!(
            "successfully flushed {} inscriptions to the sender",
            inscriptions_count
        );
        return Ok(());
    };

    tracing::error!(
        "failed to flush inscription sender, encountered error: {:?}",
        err
    );
    Ok(())
}

/// [ProcessBlockStreamTask] represents the task that is responsible for
/// processing a stream of incoming [Block]s.
pub struct ProcessBlockStreamTask {
    pub task_handle: Option<JoinHandle<()>>,
}

impl ProcessBlockStreamTask {
    /// [new] creates a new [ProcessBlockStreamTask] that will process a stream
    /// of incoming [Block]s.
    ///
    /// Calling this function will create an asynchronous task that will start
    /// processing immediately. The handle for the task will be stored within
    /// the returned structure.
    pub fn new<S, Persistence, K>(
        inscription_namespace_id: NamespaceId,
        block_receiver: S,
        data_state: Arc<RwLock<DataState<Persistence>>>,
        inscription_sender: K,
    ) -> Self
    where
        S: Stream<Item = BlockQueryData<SeqTypes>> + Send + Sync + Unpin + 'static,
        K: Sink<InscriptionAndChainDetails, Error = SendError>
            + Clone
            + Send
            + Sync
            + Unpin
            + 'static,
        Persistence: InscriptionPersistence + Send + Sync + 'static,
    {
        let task_handle = async_std::task::spawn(Self::process_block_stream(
            block_receiver,
            inscription_namespace_id,
            data_state.clone(),
            inscription_sender,
        ));

        Self {
            task_handle: Some(task_handle),
        }
    }

    /// [process_block_stream] allows for the consumption of a [Stream] when
    /// attempting to process new incoming [Block]s.
    async fn process_block_stream<S, Persistence, ISink>(
        mut stream: S,
        inscription_namespace_id: NamespaceId,
        data_state: Arc<RwLock<DataState<Persistence>>>,
        inscription_sender: ISink,
    ) where
        S: Stream<Item = BlockQueryData<SeqTypes>> + Unpin,
        ISink: Sink<InscriptionAndChainDetails, Error = SendError> + Clone + Unpin,
        Persistence: InscriptionPersistence,
    {
        loop {
            let block_result = stream.next().await;
            let block = if let Some(block) = block_result {
                block
            } else {
                // We have reached the end of the stream
                tracing::error!("process block stream: end of stream reached for leaf stream.");
                return;
            };

            {
                // Record the last received block
                let state = data_state.read_arc().await;
                if let Err(err) = state
                    .persistence()
                    .record_last_received_block(block.header().block_number())
                    .await
                {
                    tracing::error!("failed to record last received block: {:?}", err);
                }
            }

            let now_timestamp = OffsetDateTime::now_utc().unix_timestamp() as u64;
            let block_timestamp = block.header().timestamp();
            tracing::debug!(
                "received block from stream: {}, block timestamp: {block_timestamp}, now timestamp: {now_timestamp}, potential latency / clock drift combination: {}",
                block.header().block_number(),
                now_timestamp - block_timestamp,
            );

            if let Err(err) = process_incoming_block(
                inscription_namespace_id,
                block,
                data_state.clone(),
                inscription_sender.clone(),
            )
            .await
            {
                // We have an error that prevents us from continuing
                tracing::error!("process leaf stream: error processing leaf: {}", err);

                // At the moment, all underlying errors are due to `SendError`
                // which will ultimately mean that further processing attempts
                // will fail, and be fruitless.
                match err {
                    ProcessBlockError::BlockSendError(_) => {
                        panic!("ProcessBlockStreamTask: process_incoming_block failed, underlying sink is closed, blocks will stagnate: {}", err)
                    }
                }
            }
        }
    }
}

/// [Drop] implementation for [ProcessBlockStreamTask] that will cancel the
/// task if it is dropped.
impl Drop for ProcessBlockStreamTask {
    fn drop(&mut self) {
        let task_handle = self.task_handle.take();
        if let Some(task_handle) = task_handle {
            async_std::task::block_on(task_handle.cancel());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use governor::{Quota, RateLimiter};

    #[test]
    fn test_governor_rate_limiter() {
        let quota = Quota::per_second(NonZeroU32::new(1).unwrap());
        let rate_limiter = RateLimiter::direct(quota);

        assert!(rate_limiter.check().is_ok());
        assert!(rate_limiter.check().is_err());
    }
}
