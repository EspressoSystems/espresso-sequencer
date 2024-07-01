use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{bail, Result};
use async_broadcast::{Receiver, Sender};
use async_compatibility_layer::art::async_sleep;
use async_trait::async_trait;
use futures::{stream::FuturesUnordered, StreamExt};
use hotshot_builder_api::block_info::{
    AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo,
};
use hotshot_task::task::TaskState;
use hotshot_types::{
    consensus::OuterConsensus,
    data::{null_block, Leaf},
    event::{Event, EventType},
    simple_certificate::UpgradeCertificate,
    traits::{
        block_contents::{precompute_vid_commitment, BuilderFee, EncodeBytes},
        election::Membership,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::{BuilderSignatureKey, SignatureKey},
        BlockPayload,
    },
    utils::ViewInner,
    vid::VidCommitment,
};
use tracing::{debug, error, instrument, warn};

use crate::{
    builder::BuilderClient,
    events::{HotShotEvent, HotShotTaskCompleted},
    helpers::broadcast_event,
};

// Parameters for builder querying algorithm

/// Proportion of builders queried in first batch, dividend
const BUILDER_MAIN_BATCH_THRESHOLD_DIVIDEND: usize = 2;
/// Proportion of builders queried in the first batch, divisor
const BUILDER_MAIN_BATCH_THRESHOLD_DIVISOR: usize = 3;
/// Time the first batch of builders has to respond
const BUILDER_MAIN_BATCH_CUTOFF: Duration = Duration::from_millis(700);
/// Multiplier for extra time to give to the second batch of builders
const BUILDER_ADDITIONAL_TIME_MULTIPLIER: f32 = 0.2;
/// Minimum amount of time allotted to both batches, cannot be cut shorter if the first batch
/// responds extremely fast.
const BUILDER_MINIMUM_QUERY_TIME: Duration = Duration::from_millis(300);

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
pub struct TransactionTaskState<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// The state's api
    pub builder_timeout: Duration,

    /// Output events to application
    pub output_event_stream: async_broadcast::Sender<Event<TYPES>>,

    /// View number this view is executing in.
    pub cur_view: TYPES::Time,

    /// Reference to consensus. Leader will require a read lock on this.
    pub consensus: OuterConsensus<TYPES>,

    /// The underlying network
    pub network: Arc<I::Network>,

    /// Membership for the quorum
    pub membership: Arc<TYPES::Membership>,

    /// Builder API client
    pub builder_clients: Vec<BuilderClient<TYPES, TYPES::Base>>,

    /// This Nodes Public Key
    pub public_key: TYPES::SignatureKey,
    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// InstanceState
    pub instance_state: Arc<TYPES::InstanceState>,
    /// This state's ID
    pub id: u64,
    /// Decided upgrade certificate
    pub decided_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> TransactionTaskState<TYPES, I> {
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Transaction task", level = "error", target = "TransactionTaskState")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    ) -> Option<HotShotTaskCompleted> {
        match event.as_ref() {
            HotShotEvent::TransactionsRecv(transactions) => {
                broadcast_event(
                    Event {
                        view_number: self.cur_view,
                        event: EventType::Transactions {
                            transactions: transactions.clone(),
                        },
                    },
                    &self.output_event_stream,
                )
                .await;

                return None;
            }
            HotShotEvent::UpgradeDecided(cert) => {
                self.decided_upgrade_certificate = Some(cert.clone());
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

                // Request a block from the builder unless we are between versions.
                let block = {
                    if self
                        .decided_upgrade_certificate
                        .as_ref()
                        .is_some_and(|cert| cert.upgrading_in(block_view))
                    {
                        None
                    } else {
                        self.wait_for_block().await
                    }
                };

                if let Some(BuilderResponses {
                    block_data,
                    blocks_initial_info,
                    block_header,
                }) = block
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
    #[instrument(skip_all, target = "TransactionTaskState", fields(id = self.id, view = *self.cur_view))]
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

        while task_start_time.elapsed() < self.builder_timeout {
            match async_compatibility_layer::art::async_timeout(
                self.builder_timeout
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

    /// Query the builders for available blocks. Queries only fraction of the builders
    /// based on the response time.
    async fn get_available_blocks(
        &self,
        parent_comm: VidCommitment,
        view_number: TYPES::Time,
        parent_comm_sig: &<<TYPES as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Vec<(AvailableBlockInfo<TYPES>, usize)> {
        // Create a collection of futures that call available_blocks endpoint for every builder
        let tasks = self
            .builder_clients
            .iter()
            .enumerate()
            .map(|(builder_idx, client)| async move {
                client
                    .available_blocks(
                        parent_comm,
                        view_number.u64(),
                        self.public_key.clone(),
                        parent_comm_sig,
                    )
                    .await
                    .map(move |blocks| {
                        // Add index into `self.builder_clients` for each block so that we know
                        // where to claim it from later
                        blocks
                            .into_iter()
                            .map(move |block_info| (block_info, builder_idx))
                    })
            })
            .collect::<FuturesUnordered<_>>();

        // A vector of resolved builder responses
        let mut results = Vec::with_capacity(self.builder_clients.len());

        // Instant we start querying builders for available blocks
        let query_start = Instant::now();

        // First we complete the query to the fastest fraction of the builders
        let threshold = (self.builder_clients.len() * BUILDER_MAIN_BATCH_THRESHOLD_DIVIDEND)
            .div_ceil(BUILDER_MAIN_BATCH_THRESHOLD_DIVISOR);
        let mut tasks = tasks.take(threshold);
        while let Some(result) = tasks.next().await {
            results.push(result);
            if query_start.elapsed() > BUILDER_MAIN_BATCH_CUTOFF {
                break;
            }
        }

        // Then we query the rest, alotting additional `elapsed * BUILDER_ADDITIONAL_TIME_MULTIPLIER`
        // for them to respond. There's a fixed floor of `BUILDER_MINIMUM_QUERY_TIME` for both
        // phases
        let timeout = async_sleep(std::cmp::max(
            query_start
                .elapsed()
                .mul_f32(BUILDER_ADDITIONAL_TIME_MULTIPLIER),
            BUILDER_MINIMUM_QUERY_TIME.saturating_sub(query_start.elapsed()),
        ));
        futures::pin_mut!(timeout); // Stream::next requires Self::Unpin
        let mut tasks = tasks.into_inner().take_until(timeout);
        while let Some(result) = tasks.next().await {
            results.push(result);
        }

        results
            .into_iter()
            .filter_map(|result| match result {
                Ok(value) => Some(value),
                Err(err) => {
                    tracing::warn!(%err, "Error getting available blocks");
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    /// Get a block from builder.
    /// Queries the sufficiently fast builders for available blocks and chooses the one with the
    /// best fee/byte ratio, re-trying with the next best one in case of failure.
    ///
    /// # Errors
    /// If none of the builder reports any available blocks or claiming block fails for all of the
    /// builders.
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "block_from_builder", level = "error")]
    async fn block_from_builder(
        &self,
        parent_comm: VidCommitment,
        view_number: TYPES::Time,
        parent_comm_sig: &<<TYPES as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> anyhow::Result<BuilderResponses<TYPES>> {
        let mut available_blocks = self
            .get_available_blocks(parent_comm, view_number, parent_comm_sig)
            .await;

        available_blocks.sort_by(|(l, _), (r, _)| {
            // We want the block with the highest fee per byte of data we're going to have to
            // process, thus our comparision function is:
            //      (l.offered_fee / l.block_size) < (r.offered_fee / r.block_size)
            // To avoid floating point math (which doesn't even have an `Ord` impl) we multiply
            // through by the denominators to get
            //      l.offered_fee * r.block_size < r.offered_fee * l.block_size
            // We cast up to u128 to avoid overflow.
            (u128::from(l.offered_fee) * u128::from(r.block_size))
                .cmp(&(u128::from(r.offered_fee) * u128::from(l.block_size)))
        });

        if available_blocks.is_empty() {
            bail!("No available blocks");
        }

        for (block_info, builder_idx) in available_blocks {
            let client = &self.builder_clients[builder_idx];

            // Verify signature over chosen block.
            if !block_info.sender.validate_block_info_signature(
                &block_info.signature,
                block_info.block_size,
                block_info.offered_fee,
                &block_info.block_hash,
            ) {
                tracing::warn!("Failed to verify available block info response message signature");
                continue;
            }

            let request_signature = match <<TYPES as NodeType>::SignatureKey as SignatureKey>::sign(
                &self.private_key,
                block_info.block_hash.as_ref(),
            ) {
                Ok(request_signature) => request_signature,
                Err(err) => {
                    tracing::warn!(%err, "Failed to sign block hash");
                    continue;
                }
            };

            let (block, header_input) = futures::join! {
                client.claim_block(block_info.block_hash.clone(), view_number.u64(), self.public_key.clone(), &request_signature),
                client.claim_block_header_input(block_info.block_hash.clone(), view_number.u64(), self.public_key.clone(), &request_signature)
            };

            let block_data = match block {
                Ok(block_data) => block_data,
                Err(err) => {
                    tracing::warn!(%err, "Error claiming block data");
                    continue;
                }
            };

            let header_input = match header_input {
                Ok(block_data) => block_data,
                Err(err) => {
                    tracing::warn!(%err, "Error claiming header input");
                    continue;
                }
            };

            // verify the signature over the message, construct the builder commitment
            let builder_commitment = block_data
                .block_payload
                .builder_commitment(&block_data.metadata);
            if !block_data
                .sender
                .validate_builder_signature(&block_data.signature, builder_commitment.as_ref())
            {
                tracing::warn!("Failed to verify available block data response message signature");
                continue;
            }

            // first verify the message signature and later verify the fee_signature
            if !header_input.sender.validate_builder_signature(
                &header_input.message_signature,
                header_input.vid_commitment.as_ref(),
            ) {
                tracing::warn!(
                    "Failed to verify available block header input data response message signature"
                );
                continue;
            }

            // verify the signature over the message
            if !header_input.sender.validate_fee_signature(
                &header_input.fee_signature,
                block_info.offered_fee,
                &block_data.metadata,
                &header_input.vid_commitment,
            ) {
                tracing::warn!("Failed to verify fee signature");
                continue;
            }

            return Ok(BuilderResponses {
                blocks_initial_info: block_info,
                block_data,
                block_header: header_input,
            });
        }

        bail!("Couldn't claim a block from any of the builders");
    }
}

#[async_trait]
/// task state implementation for Transactions Task
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> TaskState for TransactionTaskState<TYPES, I> {
    type Event = HotShotEvent<TYPES>;

    async fn handle_event(
        &mut self,
        event: Arc<Self::Event>,
        sender: &Sender<Arc<Self::Event>>,
        _receiver: &Receiver<Arc<Self::Event>>,
    ) -> Result<()> {
        self.handle(event, sender.clone()).await;

        Ok(())
    }

    async fn cancel_subtasks(&mut self) {}
}
