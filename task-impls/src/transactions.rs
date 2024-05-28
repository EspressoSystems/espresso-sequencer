use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{bail, Context};
use async_broadcast::Sender;
use async_compatibility_layer::art::async_sleep;
use async_lock::RwLock;
use hotshot_builder_api::block_info::{
    AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo,
};
use hotshot_task::task::{Task, TaskState};
use hotshot_types::{
    consensus::Consensus,
    data::{null_block, Leaf},
    event::{Event, EventType},
    traits::{
        block_contents::{precompute_vid_commitment, BuilderFee, EncodeBytes},
        consensus_api::ConsensusApi,
        election::Membership,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::{BuilderSignatureKey, SignatureKey},
        BlockPayload,
    },
    utils::ViewInner,
    vid::VidCommitment,
};
use tracing::{debug, error, instrument, warn};
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
    /// InstanceState
    pub instance_state: Arc<TYPES::InstanceState>,
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
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Transaction task", level = "error")]
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
                    make_block = self.membership.leader(view) == self.public_key;
                }
                self.cur_view = view;

                // return if we aren't the next leader or we skipped last view and aren't the current leader.
                if !make_block && self.membership.leader(self.cur_view + 1) != self.public_key {
                    debug!("Not next leader for view {:?}", self.cur_view);
                    return None;
                }
                let block_view = if make_block { view } else { view + 1 };

                if let Some(BuilderResponses {
                    block_data,
                    blocks_initial_info,
                    block_header,
                }) = self.wait_for_block().await
                {
                    broadcast_event(
                        Arc::new(HotShotEvent::BlockRecv(
                            block_data.block_payload.encode(),
                            block_data.metadata,
                            block_view,
                            BuilderFee {
                                fee_amount: blocks_initial_info.offered_fee,
                                fee_account: block_data.sender,
                                fee_signature: block_header.fee_signature,
                            },
                            block_header.vid_precompute_data,
                        )),
                        &event_stream,
                    )
                    .await;
                } else {
                    // If we couldn't get a block, send an empty block
                    warn!(
                        "Failed to get a block for view {:?}, proposing empty block",
                        view
                    );

                    // Increment the metric for number of empty blocks proposed
                    self.consensus
                        .write()
                        .await
                        .metrics
                        .number_of_empty_blocks_proposed
                        .add(1);

                    let membership_total_nodes = self.membership.total_nodes();

                    // Calculate the builder fee for the empty block
                    let Some(builder_fee) = null_block::builder_fee(membership_total_nodes) else {
                        error!("Failed to get builder fee");
                        return None;
                    };

                    // Create an empty block payload and metadata
                    let (_, metadata) = <TYPES as NodeType>::BlockPayload::empty();

                    let (_, precompute_data) =
                        precompute_vid_commitment(&[], membership_total_nodes);

                    // Broadcast the empty block
                    broadcast_event(
                        Arc::new(HotShotEvent::BlockRecv(
                            vec![].into(),
                            metadata,
                            block_view,
                            builder_fee,
                            precompute_data,
                        )),
                        &event_stream,
                    )
                    .await;
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
    async fn latest_known_vid_commitment(&self) -> (TYPES::Time, VidCommitment) {
        let consensus = self.consensus.read().await;

        let mut prev_view = TYPES::Time::new(self.cur_view.saturating_sub(1));

        // Search through all previous views...
        while prev_view != TYPES::Time::genesis() {
            if let Some(commitment) =
                consensus
                    .validated_state_map()
                    .get(&prev_view)
                    .and_then(|view| match view.view_inner {
                        // For a view for which we have a Leaf stored
                        ViewInner::Da { payload_commitment } => Some(payload_commitment),
                        ViewInner::Leaf { leaf, .. } => consensus
                            .saved_leaves()
                            .get(&leaf)
                            .map(Leaf::payload_commitment),
                        ViewInner::Failed => None,
                    })
            {
                return (prev_view, commitment);
            }
            prev_view = prev_view - 1;
        }

        // If not found, return commitment for last decided block
        (prev_view, consensus.decided_leaf().payload_commitment())
    }

    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "wait_for_block", level = "error")]
    async fn wait_for_block(&self) -> Option<BuilderResponses<TYPES>> {
        let task_start_time = Instant::now();

        // Find commitment to the block we want to build upon
        let (view_num, parent_comm) = self.latest_known_vid_commitment().await;
        let parent_comm_sig = match <<TYPES as NodeType>::SignatureKey as SignatureKey>::sign(
            &self.private_key,
            parent_comm.as_ref(),
        ) {
            Ok(sig) => sig,
            Err(err) => {
                error!(%err, "Failed to sign block hash");
                return None;
            }
        };

        while task_start_time.elapsed() < self.api.builder_timeout() {
            match async_compatibility_layer::art::async_timeout(
                self.api
                    .builder_timeout()
                    .saturating_sub(task_start_time.elapsed()),
                self.block_from_builder(parent_comm, view_num, &parent_comm_sig),
            )
            .await
            {
                // We got a block
                Ok(Ok(block)) => {
                    return Some(block);
                }

                // We failed to get a block
                Ok(Err(err)) => {
                    tracing::warn!("Couldn't get a block: {err:#}");
                    // pause a bit
                    async_sleep(Duration::from_millis(100)).await;
                    continue;
                }

                // We timed out while getting available blocks
                Err(err) => {
                    error!(%err, "Timeout while getting available blocks");
                    return None;
                }
            }
        }

        tracing::warn!("could not get a block from the builder in time");
        None
    }

    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "block_from_builder", level = "error")]
    async fn block_from_builder(
        &self,
        parent_comm: VidCommitment,
        view_number: TYPES::Time,
        parent_comm_sig: &<<TYPES as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> anyhow::Result<BuilderResponses<TYPES>> {
        let available_blocks = self
            .builder_client
            .available_blocks(
                parent_comm,
                view_number.u64(),
                self.public_key.clone(),
                parent_comm_sig,
            )
            .await
            .context("getting available blocks")?;
        tracing::debug!("Got available blocks: {available_blocks:?}");

        let block_info = available_blocks
            .into_iter()
            .max_by(|l, r| {
                // We want the block with the highest fee per byte of data we're going to have to
                // process, thus our comparision function is:
                //      (l.offered_fee / l.block_size) < (r.offered_fee / r.block_size)
                // To avoid floating point math (which doesn't even have an `Ord` impl) we multiply
                // through by the denominators to get
                //      l.offered_fee * r.block_size < r.offered_fee * l.block_size
                // We cast up to u128 to avoid overflow.
                (u128::from(l.offered_fee) * u128::from(r.block_size))
                    .cmp(&(u128::from(r.offered_fee) * u128::from(l.block_size)))
            })
            .context("no available blocks")?;
        tracing::debug!("Selected block: {block_info:?}");

        // Verify signature over chosen block.
        if !block_info.sender.validate_block_info_signature(
            &block_info.signature,
            block_info.block_size,
            block_info.offered_fee,
            &block_info.block_hash,
        ) {
            bail!("Failed to verify available block info response message signature");
        }

        let request_signature = <<TYPES as NodeType>::SignatureKey as SignatureKey>::sign(
            &self.private_key,
            block_info.block_hash.as_ref(),
        )
        .context("signing block hash")?;

        let (block, header_input) = futures::join! {
            self.builder_client.claim_block(block_info.block_hash.clone(), view_number.u64(), self.public_key.clone(), &request_signature),
            self.builder_client.claim_block_header_input(block_info.block_hash.clone(), view_number.u64(), self.public_key.clone(), &request_signature)
        };

        let block_data = block.context("claiming block data")?;

        // verify the signature over the message, construct the builder commitment
        let builder_commitment = block_data
            .block_payload
            .builder_commitment(&block_data.metadata);
        if !block_data
            .sender
            .validate_builder_signature(&block_data.signature, builder_commitment.as_ref())
        {
            bail!("Failed to verify available block data response message signature");
        }

        let header_input = header_input.context("claiming header input")?;

        // first verify the message signature and later verify the fee_signature
        if !header_input.sender.validate_builder_signature(
            &header_input.message_signature,
            header_input.vid_commitment.as_ref(),
        ) {
            bail!("Failed to verify available block header input data response message signature");
        }

        // verify the signature over the message
        if !header_input.sender.validate_fee_signature(
            &header_input.fee_signature,
            block_info.offered_fee,
            &block_data.metadata,
            &header_input.vid_commitment,
        ) {
            bail!("Failed to verify fee signature");
        }

        Ok(BuilderResponses {
            blocks_initial_info: block_info,
            block_data,
            block_header: header_input,
        })
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
