use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use async_broadcast::Sender;
use async_compatibility_layer::art::async_sleep;
use async_lock::RwLock;
use hotshot_builder_api::block_info::{AvailableBlockData, AvailableBlockHeaderInput};
use hotshot_task::task::{Task, TaskState};
use hotshot_types::{
    consensus::Consensus,
    event::{Event, EventType},
    traits::{
        block_contents::BlockHeader,
        consensus_api::ConsensusApi,
        election::Membership,
        node_implementation::{NodeImplementation, NodeType},
        signature_key::SignatureKey,
        BlockPayload,
    },
};
use tracing::{debug, error, instrument};
use vbs::version::StaticVersionType;

use crate::{
    builder::BuilderClient,
    events::{HotShotEvent, HotShotTaskCompleted},
    helpers::broadcast_event,
};

/// Tracks state of a Transaction task
pub struct TransactionTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    A: ConsensusApi<TYPES, I> + 'static,
    Ver: StaticVersionType,
> {
    /// The state's api
    pub api: A,

    /// View number this view is executing in.
    pub cur_view: TYPES::Time,

    /// Reference to consensus. Leader will require a read lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES>>>,

    /// Network for all nodes
    pub network: Arc<I::QuorumNetwork>,

    /// Membership for the quorum
    pub membership: Arc<TYPES::Membership>,

    /// Builder API client
    pub builder_client: BuilderClient<TYPES, Ver>,

    /// This Nodes Public Key
    pub public_key: TYPES::SignatureKey,
    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// This state's ID
    pub id: u64,
}

impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES>,
        A: ConsensusApi<TYPES, I> + 'static,
        Ver: StaticVersionType,
    > TransactionTaskState<TYPES, I, A, Ver>
{
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Transaction Handling Task", level = "error")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    ) -> Option<HotShotTaskCompleted> {
        match event.as_ref() {
            HotShotEvent::TransactionsRecv(transactions) => {
                self.api
                    .send_event(Event {
                        view_number: self.cur_view,
                        event: EventType::Transactions {
                            transactions: transactions.clone(),
                        },
                    })
                    .await;
                return None;
            }
            HotShotEvent::ViewChange(view) => {
                let view = *view;
                debug!("view change in transactions to view {:?}", view);
                if (*view != 0 || *self.cur_view > 0) && *self.cur_view >= *view {
                    return None;
                }

                let mut make_block = false;
                if *view - *self.cur_view > 1 {
                    error!("View changed by more than 1 going to view {:?}", view);
                    make_block = self.membership.get_leader(view) == self.public_key;
                }
                self.cur_view = view;

                // return if we aren't the next leader or we skipped last view and aren't the current leader.
                if !make_block && self.membership.get_leader(self.cur_view + 1) != self.public_key {
                    debug!("Not next leader for view {:?}", self.cur_view);
                    return None;
                }

                if let Some((block, _)) = self.wait_for_block().await {
                    // send the sequenced transactions to VID and DA tasks
                    let block_view = if make_block { view } else { view + 1 };
                    let encoded_transactions = match block.block_payload.encode() {
                        Ok(encoded) => encoded.into_iter().collect::<Vec<u8>>(),
                        Err(e) => {
                            error!("Failed to encode the block payload: {:?}.", e);
                            return None;
                        }
                    };
                    broadcast_event(
                        Arc::new(HotShotEvent::BlockRecv(
                            encoded_transactions,
                            block.metadata,
                            block_view,
                        )),
                        &event_stream,
                    )
                    .await;
                } else {
                    error!("Failed to get a block from the builder");
                };

                return None;
            }
            HotShotEvent::Shutdown => {
                return Some(HotShotTaskCompleted);
            }
            _ => {}
        }
        None
    }

    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Transaction Handling Task", level = "error")]
    async fn wait_for_block(
        &self,
    ) -> Option<(AvailableBlockData<TYPES>, AvailableBlockHeaderInput<TYPES>)> {
        let task_start_time = Instant::now();

        let last_leaf = self.consensus.read().await.get_decided_leaf();
        let mut latest_block: Option<(
            AvailableBlockData<TYPES>,
            AvailableBlockHeaderInput<TYPES>,
        )> = None;
        while task_start_time.elapsed() < self.api.propose_max_round_time()
            && latest_block.as_ref().map_or(true, |(data, _)| {
                data.block_payload.num_transactions(&data.metadata) < self.api.min_transactions()
            })
        {
            let mut available_blocks = match self
                .builder_client
                .get_available_blocks(last_leaf.get_block_header().payload_commitment())
                .await
            {
                Ok(blocks) => blocks,
                Err(err) => {
                    error!(%err, "Couldn't get available blocks");
                    continue;
                }
            };

            available_blocks.sort_by_key(|block_info| block_info.offered_fee);

            let Some(block_info) = available_blocks.pop() else {
                continue;
            };

            // Don't try to re-claim the same block if builder advertises it again
            if latest_block.as_ref().map_or(false, |block| {
                block.0.block_payload.builder_commitment(&block.0.metadata) == block_info.block_hash
            }) {
                continue;
            }

            let Ok(signature) = <<TYPES as NodeType>::SignatureKey as SignatureKey>::sign(
                &self.private_key,
                block_info.block_hash.as_ref(),
            ) else {
                error!("Failed to sign block hash");
                continue;
            };

            let (block, header_input) = futures::join! {
                self.builder_client.claim_block(block_info.block_hash.clone(), &signature),
                self.builder_client.claim_block_header_input(block_info.block_hash, &signature)
            };

            let block = match block {
                Ok(val) => val,
                Err(err) => {
                    error!(%err, "Failed to claim block");
                    continue;
                }
            };

            let header_input = match header_input {
                Ok(val) => val,
                Err(err) => {
                    error!(%err, "Failed to claim block");
                    continue;
                }
            };

            let num_txns = block.block_payload.num_transactions(&block.metadata);

            latest_block = Some((block, header_input));
            if num_txns >= self.api.min_transactions() {
                return latest_block;
            }
            async_sleep(Duration::from_millis(100)).await;
        }
        latest_block
    }
}

/// task state implementation for Transactions Task
impl<
        TYPES: NodeType,
        I: NodeImplementation<TYPES>,
        A: ConsensusApi<TYPES, I> + 'static,
        Ver: StaticVersionType + 'static,
    > TaskState for TransactionTaskState<TYPES, I, A, Ver>
{
    type Event = Arc<HotShotEvent<TYPES>>;

    type Output = HotShotTaskCompleted;

    fn filter(&self, event: &Arc<HotShotEvent<TYPES>>) -> bool {
        !matches!(
            event.as_ref(),
            HotShotEvent::TransactionsRecv(_)
                | HotShotEvent::Shutdown
                | HotShotEvent::ViewChange(_)
        )
    }

    async fn handle_event(
        event: Self::Event,
        task: &mut Task<Self>,
    ) -> Option<HotShotTaskCompleted> {
        let sender = task.clone_sender();
        task.state_mut().handle(event, sender).await
    }

    fn should_shutdown(event: &Self::Event) -> bool {
        matches!(event.as_ref(), HotShotEvent::Shutdown)
    }
}
