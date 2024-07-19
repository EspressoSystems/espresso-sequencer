use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{bail, Result};
use async_broadcast::{Receiver, Sender};
use async_compatibility_layer::art::async_sleep;
use async_lock::RwLock;
use async_trait::async_trait;
use futures::{stream::FuturesUnordered, StreamExt};
use hotshot_builder_api::v0_1::block_info::AvailableBlockInfo;
use hotshot_task::task::TaskState;
use hotshot_types::{
    consensus::OuterConsensus,
    data::{null_block, Leaf, PackedBundle},
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
    vid::{VidCommitment, VidPrecomputeData},
};
use tracing::{debug, error, instrument, warn};
use vbs::version::{StaticVersionType, Version};

use crate::{
    builder::{
        v0_1::BuilderClient as BuilderClientBase, v0_3::BuilderClient as BuilderClientMarketplace,
    },
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

/// The version of the builder API used by the marketplace
type MarketplaceVersion = crate::builder::v0_3::Version;

/// Builder Provided Responses
pub struct BuilderResponse<TYPES: NodeType> {
    /// Fee information
    pub fee: BuilderFee<TYPES>,
    /// Block payload
    pub block_payload: TYPES::BlockPayload,
    /// Block metadata
    pub metadata: <TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata,
    /// Optional precomputed commitment
    pub precompute_data: Option<VidPrecomputeData>,
}

/// The Bundle for a portion of a block, provided by a downstream builder that exists in a bundle
/// auction.
pub struct Bundle<TYPES: NodeType> {
    /// The bundle transactions sent by the builder.
    pub transactions: Vec<<TYPES::BlockPayload as BlockPayload<TYPES>>::Transaction>,

    /// The signature over the bundle.
    pub signature: TYPES::SignatureKey,

    /// The fee for submitting a bid.
    pub bid_fee: BuilderFee<TYPES>,

    /// The fee for sequencing
    pub sequencing_fee: BuilderFee<TYPES>,
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

    /// Builder 0.1 API clients
    pub builder_clients: Vec<BuilderClientBase<TYPES>>,

    /// Builder 0.3 API clients
    pub builder_clients_marketplace: Vec<BuilderClientMarketplace<TYPES>>,

    /// This Nodes Public Key
    pub public_key: TYPES::SignatureKey,
    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// InstanceState
    pub instance_state: Arc<TYPES::InstanceState>,
    /// This state's ID
    pub id: u64,
    /// Decided upgrade certificate
    pub decided_upgrade_certificate: Arc<RwLock<Option<UpgradeCertificate<TYPES>>>>,
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

                let version = match hotshot_types::simple_certificate::version(
                    block_view,
                    &self
                        .decided_upgrade_certificate
                        .read()
                        .await
                        .as_ref()
                        .cloned(),
                ) {
                    Ok(v) => v,
                    Err(err) => {
                        error!("Upgrade certificate requires unsupported version, refusing to request blocks: {}", err);
                        return None;
                    }
                };

                // Request a block from the builder unless we are between versions.
                let block = {
                    if self
                        .decided_upgrade_certificate
                        .read()
                        .await
                        .as_ref()
                        .is_some_and(|cert| cert.upgrading_in(block_view))
                    {
                        None
                    } else {
                        self.wait_for_block(block_view, version).await
                    }
                };

                if let Some(BuilderResponse {
                    block_payload,
                    metadata,
                    fee,
                    precompute_data,
                }) = block
                {
                    let Some(bid_fee) =
                        null_block::builder_fee(self.membership.total_nodes(), version)
                    else {
                        error!("Failed to get bid fee");
                        return None;
                    };

                    broadcast_event(
                        Arc::new(HotShotEvent::BlockRecv(PackedBundle::new(
                            block_payload.encode(),
                            metadata,
                            block_view,
                            vec1::vec1![bid_fee],
                            vec1::vec1![fee],
                            precompute_data,
                        ))),
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
                    let Some(null_fee) =
                        null_block::builder_fee(self.membership.total_nodes(), version)
                    else {
                        error!("Failed to get null fee");
                        return None;
                    };

                    // Create an empty block payload and metadata
                    let (_, metadata) = <TYPES as NodeType>::BlockPayload::empty();

                    let (_, precompute_data) =
                        precompute_vid_commitment(&[], membership_total_nodes);

                    // Broadcast the empty block
                    broadcast_event(
                        Arc::new(HotShotEvent::BlockRecv(PackedBundle::new(
                            vec![].into(),
                            metadata,
                            block_view,
                            vec1::vec1![null_fee.clone()],
                            vec1::vec1![null_fee],
                            Some(precompute_data),
                        ))),
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
    #[instrument(skip_all, target = "TransactionTaskState", fields(id = self.id, cur_view = *self.cur_view, block_view = *block_view))]
    async fn latest_known_vid_commitment(
        &self,
        block_view: TYPES::Time,
    ) -> (TYPES::Time, VidCommitment) {
        let consensus = self.consensus.read().await;
        let mut prev_view = TYPES::Time::new(block_view.saturating_sub(1));

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

    #[instrument(skip_all, fields(id = self.id, cur_view = *self.cur_view, block_view = *block_view), name = "wait_for_block", level = "error")]
    async fn wait_for_block(
        &self,
        block_view: TYPES::Time,
        version: Version,
    ) -> Option<BuilderResponse<TYPES>> {
        let task_start_time = Instant::now();

        // Find commitment to the block we want to build upon
        let (view_num, parent_comm) = self.latest_known_vid_commitment(block_view).await;
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
                self.block_from_builder(parent_comm, view_num, &parent_comm_sig, version),
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
        version: Version,
    ) -> Vec<(AvailableBlockInfo<TYPES>, usize)> {
        /// Implementations between versions are essentially the same except for the builder
        /// clients used. The most conscise way to express this is with a macro.
        macro_rules! inner_impl {
            ($clients:ident) => {{
                // Create a collection of futures that call available_blocks endpoint for every builder
                let tasks = self
                    .$clients
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
                let mut results = Vec::with_capacity(self.$clients.len());

                // Instant we start querying builders for available blocks
                let query_start = Instant::now();

                // First we complete the query to the fastest fraction of the builders
                let threshold = (self.$clients.len() * BUILDER_MAIN_BATCH_THRESHOLD_DIVIDEND)
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
            }}
        }

        if version >= MarketplaceVersion::version() {
            inner_impl!(builder_clients_marketplace)
        } else {
            inner_impl!(builder_clients)
        }
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
        version: Version,
    ) -> anyhow::Result<BuilderResponse<TYPES>> {
        let mut available_blocks = self
            .get_available_blocks(parent_comm, view_number, parent_comm_sig, version)
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

            let response = if version >= MarketplaceVersion::version() {
                let client = &self.builder_clients_marketplace[builder_idx];

                let block = client
                    .claim_block(
                        block_info.block_hash.clone(),
                        view_number.u64(),
                        self.public_key.clone(),
                        &request_signature,
                    )
                    .await;

                let block_data = match block {
                    Ok(block_data) => block_data,
                    Err(err) => {
                        tracing::warn!(%err, "Error claiming block data");
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
                    tracing::warn!(
                        "Failed to verify available block data response message signature"
                    );
                    continue;
                }

                let fee = BuilderFee {
                    fee_amount: block_info.offered_fee,
                    fee_account: block_data.sender,
                    fee_signature: block_data.signature,
                };

                BuilderResponse {
                    fee,
                    block_payload: block_data.block_payload,
                    metadata: block_data.metadata,
                    precompute_data: None,
                }
            } else {
                let client = &self.builder_clients[builder_idx];

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

                // verify the signature over the message
                if !block_data.validate_signature() {
                    tracing::warn!(
                        "Failed to verify available block data response message signature"
                    );
                    continue;
                }

                // verify the message signature and the fee_signature
                if !header_input.validate_signature(block_info.offered_fee, &block_data.metadata) {
                    tracing::warn!(
                    "Failed to verify available block header input data response message signature"
                );
                    continue;
                }

                let fee = BuilderFee {
                    fee_amount: block_info.offered_fee,
                    fee_account: header_input.sender,
                    fee_signature: header_input.fee_signature,
                };

                BuilderResponse {
                    fee,
                    block_payload: block_data.block_payload,
                    metadata: block_data.metadata,
                    precompute_data: Some(header_input.vid_precompute_data),
                }
            };

            return Ok(response);
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
