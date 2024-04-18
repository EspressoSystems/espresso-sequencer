use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use async_broadcast::Sender;
use async_compatibility_layer::art::async_sleep;
use async_lock::RwLock;
use hotshot_builder_api::block_info::{
    AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo,
};
use hotshot_task::task::{Task, TaskState};
use hotshot_types::{
    consensus::Consensus,
    data::Leaf,
    event::{Event, EventType},
    traits::{
        block_contents::{BlockHeader, BuilderFee},
        consensus_api::ConsensusApi,
        election::Membership,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::{BuilderSignatureKey, SignatureKey},
        BlockPayload,
    },
    utils::BuilderCommitment,
};
use tracing::{debug, error, instrument};
use vbs::version::StaticVersionType;

use crate::{
    builder::BuilderClient,
    events::{HotShotEvent, HotShotTaskCompleted},
    helpers::broadcast_event,
};

/// Builder Provided Responses
pub struct BuilderResponses<TYPES: NodeType> {
    /// Initial API response
    /// It contains information about the available blocks
    pub blocks_initial_info: AvailableBlockInfo<TYPES>,
    /// Second API response
    /// It contains information about the chosen blocks
    pub block_data: AvailableBlockData<TYPES>,
    /// Third API response
    /// It contains the final block information
    pub block_header: AvailableBlockHeaderInput<TYPES>,
}
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

                if let Some(BuilderResponses {
                    block_data,
                    blocks_initial_info,
                    block_header,
                }) = self.wait_for_block().await
                {
                    // send the sequenced transactions to VID and DA tasks
                    let block_view = if make_block { view } else { view + 1 };
                    let encoded_transactions = match block_data.block_payload.encode() {
                        Ok(encoded) => encoded.into_iter().collect::<Vec<u8>>(),
                        Err(e) => {
                            error!("Failed to encode the block payload: {:?}.", e);
                            return None;
                        }
                    };
                    broadcast_event(
                        Arc::new(HotShotEvent::BlockRecv(
                            encoded_transactions,
                            block_data.metadata,
                            block_view,
                            BuilderFee {
                                fee_amount: blocks_initial_info.offered_fee,
                                fee_signature: block_header.fee_signature,
                            },
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

    /// Get last known builder commitment from consensus.
    async fn latest_known_builder_commitment(&self) -> BuilderCommitment {
        let consensus = self.consensus.read().await;

        let mut prev_view = TYPES::Time::new(self.cur_view.saturating_sub(1));

        // Search through all previous views...
        while prev_view != TYPES::Time::genesis() {
            if let Some(commitment) =
                consensus
                    .validated_state_map
                    .get(&prev_view)
                    .and_then(|view| match view.view_inner {
                        // For a view for which we have a Leaf stored
                        hotshot_types::utils::ViewInner::Leaf { leaf, .. } => consensus
                            .saved_leaves
                            .get(&leaf)
                            .map(Leaf::get_block_header)
                            .map(BlockHeader::builder_commitment), // and return it's commitment
                        _ => None,
                    })
            {
                return commitment;
            }
            prev_view = prev_view - 1;
        }

        // If not found, return commitment for last decided block
        consensus
            .get_decided_leaf()
            .get_block_header()
            .builder_commitment()
    }

    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Transaction Handling Task", level = "error")]
    async fn wait_for_block(&self) -> Option<BuilderResponses<TYPES>> {
        let task_start_time = Instant::now();

        // Find commitment to the block we want to build upon
        let parent_commitment = self.latest_known_builder_commitment().await;

        let mut latest_block: Option<BuilderResponses<TYPES>> = None;
        let mut first_iteration = true;
        while task_start_time.elapsed() < self.api.propose_max_round_time()
            && latest_block.as_ref().map_or(true, |builder_response| {
                builder_response
                    .block_data
                    .block_payload
                    .num_transactions(&builder_response.block_data.metadata)
                    < self.api.min_transactions()
            })
        {
            // Sleep if this isn't the first iteration
            if first_iteration {
                first_iteration = false;
            } else {
                async_sleep(Duration::from_millis(100)).await;
            }

            let Ok(request_signature) = <<TYPES as NodeType>::SignatureKey as SignatureKey>::sign(
                &self.private_key,
                parent_commitment.as_ref(),
            ) else {
                error!("Failed to sign block hash");
                continue;
            };

            let mut available_blocks = match self
                .builder_client
                .get_available_blocks(
                    parent_commitment.clone(),
                    self.public_key.clone(),
                    &request_signature,
                )
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

            // Verify signature over chosen block instead of
            // verifying the signature over all the blocks received from builder
            let combined_message_bytes = {
                let mut combined_response_bytes: Vec<u8> = Vec::new();
                combined_response_bytes
                    .extend_from_slice(block_info.block_size.to_be_bytes().as_ref());
                combined_response_bytes
                    .extend_from_slice(block_info.offered_fee.to_be_bytes().as_ref());
                combined_response_bytes.extend_from_slice(block_info.block_hash.as_ref());
                combined_response_bytes
            };
            if !block_info
                .sender
                .validate_builder_signature(&block_info.signature, &combined_message_bytes)
            {
                error!("Failed to verify available block info response message signature");
                continue;
            }

            // Don't try to re-claim the same block if builder advertises it again
            if latest_block.as_ref().map_or(false, |builder_response| {
                builder_response
                    .block_data
                    .block_payload
                    .builder_commitment(&builder_response.block_data.metadata)
                    == block_info.block_hash
            }) {
                continue;
            }

            let Ok(request_signature) = <<TYPES as NodeType>::SignatureKey as SignatureKey>::sign(
                &self.private_key,
                block_info.block_hash.as_ref(),
            ) else {
                error!("Failed to sign block hash");
                continue;
            };

            let (block, header_input) = futures::join! {
                self.builder_client.claim_block(block_info.block_hash.clone(), self.public_key.clone(), &request_signature),
                self.builder_client.claim_block_header_input(block_info.block_hash.clone(), self.public_key.clone(), &request_signature)
            };

            let block_data = match block {
                Ok(block_data) => {
                    // verify the signature over the message, construct the builder commitment
                    let builder_commitment = block_data
                        .block_payload
                        .builder_commitment(&block_data.metadata);
                    if !block_data.sender.validate_builder_signature(
                        &block_data.signature,
                        builder_commitment.as_ref(),
                    ) {
                        error!("Failed to verify available block data response message signature");
                        continue;
                    }
                    block_data
                }
                Err(err) => {
                    error!(%err, "Failed to claim block");
                    continue;
                }
            };

            let header_input = match header_input {
                Ok(header_input) => {
                    // first verify the message signature and later verify the fee_signature
                    if !header_input.sender.validate_builder_signature(
                        &header_input.message_signature,
                        header_input.vid_commitment.as_ref(),
                    ) {
                        error!("Failed to verify available block header input data response message signature");
                        continue;
                    }

                    let offered_fee = block_info.offered_fee;
                    let builder_commitment = block_data
                        .block_payload
                        .builder_commitment(&block_data.metadata);
                    let vid_commitment = header_input.vid_commitment;
                    let combined_response_bytes = {
                        let mut combined_response_bytes: Vec<u8> = Vec::new();
                        combined_response_bytes
                            .extend_from_slice(offered_fee.to_be_bytes().as_ref());
                        combined_response_bytes.extend_from_slice(builder_commitment.as_ref());
                        combined_response_bytes.extend_from_slice(vid_commitment.as_ref());
                        combined_response_bytes
                    };
                    // verify the signature over the message
                    if !header_input.sender.validate_builder_signature(
                        &header_input.fee_signature,
                        combined_response_bytes.as_ref(),
                    ) {
                        error!("Failed to verify fee signature");
                        continue;
                    }
                    header_input
                }
                Err(err) => {
                    error!(%err, "Failed to claim block");
                    continue;
                }
            };

            let num_txns = block_data
                .block_payload
                .num_transactions(&block_data.metadata);

            latest_block = Some(BuilderResponses {
                blocks_initial_info: block_info,
                block_data,
                block_header: header_input,
            });
            if num_txns >= self.api.min_transactions() {
                return latest_block;
            }
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
