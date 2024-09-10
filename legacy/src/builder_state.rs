use hotshot_types::{
    data::{DaProposal, Leaf, QuorumProposal},
    message::Proposal,
    traits::block_contents::{BlockHeader, BlockPayload},
    traits::{
        block_contents::precompute_vid_commitment,
        node_implementation::{ConsensusTime, NodeType},
        EncodeBytes,
    },
    utils::BuilderCommitment,
    vid::{VidCommitment, VidPrecomputeData},
};

use committable::{Commitment, Committable};

use crate::{
    service::{GlobalState, ReceivedTransaction},
    BlockId, BuilderStateId,
};
use async_broadcast::broadcast;
use async_broadcast::Receiver as BroadcastReceiver;
use async_broadcast::Sender as BroadcastSender;
use async_compatibility_layer::channel::{oneshot, unbounded, UnboundedSender};
use async_compatibility_layer::{art::async_sleep, channel::OneShotSender};
use async_compatibility_layer::{art::async_spawn, channel::UnboundedReceiver};
use async_lock::RwLock;
use core::panic;
use futures::StreamExt;

#[cfg(async_executor_impl = "async-std")]
use async_std::task::spawn_blocking;
#[cfg(async_executor_impl = "tokio")]
use tokio::task::spawn_blocking;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Instant;
use std::{cmp::PartialEq, num::NonZeroUsize};
use std::{collections::hash_map::Entry, time::Duration};

pub type TxTimeStamp = u128;

/// Enum to hold the different sources of the transaction
#[derive(Clone, Debug, PartialEq)]
pub enum TransactionSource {
    External, // txn from the external source i.e private mempool
    HotShot,  // txn from the HotShot network i.e public mempool
}

/// Decide Message to be put on the decide channel
#[derive(Clone, Debug)]
pub struct DecideMessage<TYPES: NodeType> {
    pub latest_decide_view_number: TYPES::Time,
}
/// DA Proposal Message to be put on the da proposal channel
#[derive(Clone, Debug, PartialEq)]
pub struct DaProposalMessage<TYPES: NodeType> {
    pub proposal: Arc<Proposal<TYPES, DaProposal<TYPES>>>,
    pub sender: TYPES::SignatureKey,
    pub total_nodes: usize,
}
/// QC Message to be put on the quorum proposal channel
#[derive(Clone, Debug, PartialEq)]
pub struct QCMessage<TYPES: NodeType> {
    pub proposal: Arc<Proposal<TYPES, QuorumProposal<TYPES>>>,
    pub sender: TYPES::SignatureKey,
}
/// Request Message to be put on the request channel
#[derive(Clone, Debug)]
pub struct RequestMessage<Types: NodeType> {
    pub state_id: BuilderStateId<Types>,
    pub response_channel: UnboundedSender<ResponseMessage>,
}
pub enum TriggerStatus {
    Start,
    Exit,
}

/// Response Message to be put on the response channel
#[derive(Debug)]
pub struct BuildBlockInfo<TYPES: NodeType> {
    pub id: BlockId<TYPES>,
    pub block_size: u64,
    pub offered_fee: u64,
    pub block_payload: TYPES::BlockPayload,
    pub metadata: <<TYPES as NodeType>::BlockPayload as BlockPayload<TYPES>>::Metadata,
    pub vid_trigger: OneShotSender<TriggerStatus>,
    pub vid_receiver: UnboundedReceiver<(VidCommitment, VidPrecomputeData)>,
    // Could we have included more transactions, but chose not to?
    pub truncated: bool,
}

/// Response Message to be put on the response channel
#[derive(Debug, Clone)]
pub struct ResponseMessage {
    pub builder_hash: BuilderCommitment,
    pub block_size: u64,
    pub offered_fee: u64,
}
#[derive(Debug, Clone)]
/// Enum to hold the status out of the decide event
pub enum Status {
    ShouldExit,
    ShouldContinue,
}

/// Builder State to hold the state of the builder
#[derive(Debug, Clone)]
pub struct BuiltFromProposedBlock<TYPES: NodeType> {
    pub view_number: TYPES::Time,
    pub vid_commitment: VidCommitment,
    pub leaf_commit: Commitment<Leaf<TYPES>>,
    pub builder_commitment: BuilderCommitment,
}
// implement display for the derived info
impl<TYPES: NodeType> std::fmt::Display for BuiltFromProposedBlock<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "View Number: {:?}", self.view_number)
    }
}

#[derive(Debug, Clone)]
pub struct DAProposalInfo<TYPES: NodeType> {
    pub view_number: TYPES::Time,
    pub proposal: Arc<Proposal<TYPES, DaProposal<TYPES>>>,
    pub num_nodes: usize,
}

#[derive(Debug)]
pub struct BuilderState<TYPES: NodeType> {
    /// Recent included txs set while building blocks
    pub included_txns: HashSet<Commitment<TYPES::Transaction>>,

    /// Old txs to be garbage collected
    pub included_txns_old: HashSet<Commitment<TYPES::Transaction>>,

    /// Expiring txs to be garbage collected
    pub included_txns_expiring: HashSet<Commitment<TYPES::Transaction>>,

    /// txns currently in the tx_queue
    pub txns_in_queue: HashSet<Commitment<TYPES::Transaction>>,

    /// da_proposal_payload_commit to (da_proposal, node_count)
    #[allow(clippy::type_complexity)]
    pub da_proposal_payload_commit_to_da_proposal:
        HashMap<(BuilderCommitment, TYPES::Time), DAProposalInfo<TYPES>>,

    /// quorum_proposal_payload_commit to quorum_proposal
    #[allow(clippy::type_complexity)]
    pub quorum_proposal_payload_commit_to_quorum_proposal:
        HashMap<(BuilderCommitment, TYPES::Time), Arc<Proposal<TYPES, QuorumProposal<TYPES>>>>,

    /// the spawned from info for a builder state
    pub built_from_proposed_block: BuiltFromProposedBlock<TYPES>,

    // Channel Receivers for the HotShot events, Tx_receiver could also receive the external transactions
    /// decide receiver
    pub decide_receiver: BroadcastReceiver<MessageType<TYPES>>,

    /// da proposal receiver
    pub da_proposal_receiver: BroadcastReceiver<MessageType<TYPES>>,

    /// quorum proposal receiver
    pub qc_receiver: BroadcastReceiver<MessageType<TYPES>>,

    /// channel receiver for the block requests
    pub req_receiver: BroadcastReceiver<MessageType<TYPES>>,

    /// incoming stream of transactions
    pub tx_receiver: BroadcastReceiver<Arc<ReceivedTransaction<TYPES>>>,

    /// filtered queue of available transactions, taken from tx_receiver
    pub tx_queue: VecDeque<Arc<ReceivedTransaction<TYPES>>>,

    /// global state handle, defined in the service.rs
    pub global_state: Arc<RwLock<GlobalState<TYPES>>>,

    /// total nodes required for the VID computation as part of block header input response
    pub total_nodes: NonZeroUsize,

    /// locally spawned builder Commitements
    pub builder_commitments: HashSet<(BuilderStateId<TYPES>, BuilderCommitment)>,

    /// timeout for maximising the txns in the block
    pub maximize_txn_capture_timeout: Duration,

    /// constant fee that the builder will offer per byte of data sequenced
    pub base_fee: u64,

    /// validated state that is required for a proposal to be considered valid. Needed for the
    /// purposes of building a valid block payload within the sequencer.
    pub validated_state: Arc<TYPES::ValidatedState>,

    /// instance state to enfoce max_block_size
    pub instance_state: Arc<TYPES::InstanceState>,

    /// txn garbage collection every duration time
    pub txn_garbage_collect_duration: Duration,

    /// time of next garbage collection for txns
    pub next_txn_garbage_collect_time: Instant,
}

impl<TYPES: NodeType> BuilderState<TYPES> {
    /// processing the DA proposal
    #[tracing::instrument(skip_all, name = "process da proposal",
                                    fields(builder_built_from_proposed_block = %self.built_from_proposed_block))]
    async fn process_da_proposal(&mut self, da_msg: DaProposalMessage<TYPES>) {
        tracing::debug!(
            "Builder Received DA message for view {:?}",
            da_msg.proposal.data.view_number
        );

        // we do not have the option to ignore DA proposals if we want to be able to handle failed view reorgs.

        // If the respective builder state exists to handle the request
        let proposal = da_msg.proposal.clone();
        let sender = &da_msg.sender;

        // get the view number and encoded txns from the da_proposal_data
        let view_number = proposal.data.view_number;
        let encoded_txns = &proposal.data.encoded_transactions;

        let metadata = &proposal.data.metadata;

        let num_nodes = da_msg.total_nodes;

        // form a block payload from the encoded transactions
        let block_payload =
            <TYPES::BlockPayload as BlockPayload<TYPES>>::from_bytes(encoded_txns, metadata);
        // get the builder commitment from the block payload
        let payload_builder_commitment = block_payload.builder_commitment(metadata);

        tracing::debug!(
            "Extracted builder commitment from the da proposal: {:?}",
            payload_builder_commitment
        );

        // form the DA proposal info
        let da_proposal_info = DAProposalInfo {
            view_number,
            proposal,
            num_nodes,
        };

        if let std::collections::hash_map::Entry::Vacant(e) = self
            .da_proposal_payload_commit_to_da_proposal
            .entry((payload_builder_commitment.clone(), view_number))
        {
            // if we have matching da and quorum proposals, we can skip storing the one, and remove
            // the other from storage, and call build_block with both, to save a little space.
            if let Entry::Occupied(qc_proposal) = self
                .quorum_proposal_payload_commit_to_quorum_proposal
                .entry((payload_builder_commitment.clone(), view_number))
            {
                let qc_proposal = qc_proposal.remove();

                // if we have a matching quorum proposal
                //  if (this is the correct parent or
                //      (the correct parent is missing and this is the highest view))
                //    spawn a clone
                if qc_proposal.data.view_number == view_number {
                    tracing::info!(
                        "Spawning a clone from process DA proposal for view number: {:?}",
                        view_number
                    );
                    // remove this entry from qc_proposal_payload_commit_to_quorum_proposal
                    self.quorum_proposal_payload_commit_to_quorum_proposal
                        .remove(&(payload_builder_commitment.clone(), view_number));

                    let (req_sender, req_receiver) = broadcast(self.req_receiver.capacity());
                    self.clone_with_receiver(req_receiver)
                        .spawn_clone(da_proposal_info, qc_proposal, sender.clone(), req_sender)
                        .await;
                } else {
                    tracing::debug!("Not spawning a clone despite matching DA and QC payload commitments, as they corresponds to different view numbers");
                }
            } else {
                e.insert(da_proposal_info);
            }
        } else {
            tracing::debug!("Payload commitment already exists in the da_proposal_payload_commit_to_da_proposal hashmap, so ignoring it");
        }
    }

    /// processing the quorum proposal
    //#[tracing::instrument(skip_all, name = "Process Quorum Proposal")]
    #[tracing::instrument(skip_all, name = "process quorum proposal",
                                    fields(builder_built_from_proposed_block = %self.built_from_proposed_block))]
    async fn process_quorum_proposal(&mut self, qc_msg: QCMessage<TYPES>) {
        tracing::debug!(
            "Builder Received QC Message for view {:?}",
            qc_msg.proposal.data.view_number
        );

        // Two cases to handle:
        // Case 1: Bootstrapping phase
        // Case 2: No intended builder state exist
        // To handle both cases, we can have the highest view number builder state running
        // and only doing the insertion if and only if intended builder state for a particulat view is not present
        // check the presence of quorum_proposal.data.view_number-1 in the spawned_builder_states list
        if qc_msg.proposal.data.justify_qc.view_number != self.built_from_proposed_block.view_number
        {
            tracing::debug!(
                "View number {:?} from justify qc does not match for builder {:?}",
                qc_msg.proposal.data.justify_qc.view_number,
                self.built_from_proposed_block
            );
            if !self
                .global_state
                .read_arc()
                .await
                .should_view_handle_other_proposals(
                    &self.built_from_proposed_block.view_number,
                    &qc_msg.proposal.data.justify_qc.view_number,
                )
            {
                tracing::debug!(
                    "Builder {:?} is not currently bootstrapping.",
                    self.built_from_proposed_block
                );
                // if we have the matching da proposal, we now know we don't need to keep it.
                self.da_proposal_payload_commit_to_da_proposal.remove(&(
                    qc_msg
                        .proposal
                        .data
                        .block_header
                        .builder_commitment()
                        .clone(),
                    qc_msg.proposal.data.view_number,
                ));
                return;
            }
            tracing::debug!(
                "Builder {:?} handling proposal as bootstrap.",
                self.built_from_proposed_block
            );
        }
        let qc_proposal = &qc_msg.proposal;
        let sender = &qc_msg.sender;
        let view_number = qc_proposal.data.view_number;
        let payload_builder_commitment = qc_proposal.data.block_header.builder_commitment();

        tracing::debug!(
            "Extracted payload builder commitment from the quorum proposal: {:?}",
            payload_builder_commitment
        );

        // first check whether vid_commitment exists in the qc_payload_commit_to_qc hashmap, if yer, ignore it, otherwise validate it and later insert in
        if let std::collections::hash_map::Entry::Vacant(e) = self
            .quorum_proposal_payload_commit_to_quorum_proposal
            .entry((payload_builder_commitment.clone(), view_number))
        {
            // if we have matching da and quorum proposals, we can skip storing the one, and remove the other from storage, and call build_block with both, to save a little space.
            if let Entry::Occupied(da_proposal) = self
                .da_proposal_payload_commit_to_da_proposal
                .entry((payload_builder_commitment.clone(), view_number))
            {
                let da_proposal_info = da_proposal.remove();
                // remove the entry from the da_proposal_payload_commit_to_da_proposal hashmap
                self.da_proposal_payload_commit_to_da_proposal
                    .remove(&(payload_builder_commitment.clone(), view_number));

                // also make sure we clone for the same view number( check incase payload commitments are same)
                if da_proposal_info.view_number == view_number {
                    tracing::info!(
                        "Spawning a clone from process QC proposal for view number: {:?}",
                        view_number
                    );

                    let (req_sender, req_receiver) = broadcast(self.req_receiver.capacity());
                    self.clone_with_receiver(req_receiver)
                        .spawn_clone(
                            da_proposal_info,
                            qc_proposal.clone(),
                            sender.clone(),
                            req_sender,
                        )
                        .await;
                } else {
                    tracing::debug!("Not spawning a clone despite matching DA and QC payload commitments, as they corresponds to different view numbers");
                }
            } else {
                e.insert(qc_proposal.clone());
            }
        } else {
            tracing::debug!("Payload commitment already exists in the quorum_proposal_payload_commit_to_quorum_proposal hashmap, so ignoring it");
        }
    }

    /// processing the decide event
    #[tracing::instrument(skip_all, name = "process decide event",
                                   fields(builder_built_from_proposed_block = %self.built_from_proposed_block))]
    async fn process_decide_event(&mut self, decide_msg: DecideMessage<TYPES>) -> Option<Status> {
        // Exit out all the builder states if their built_from_proposed_block.view_number is less than the latest_decide_view_number
        // The only exception is that we want to keep the highest view number builder state active to ensure that
        // we have a builder state to handle the incoming DA and QC proposals
        let decide_view_number = decide_msg.latest_decide_view_number;

        let retained_view_cutoff = self
            .global_state
            .write_arc()
            .await
            .remove_handles(decide_view_number);
        if self.built_from_proposed_block.view_number < retained_view_cutoff {
            tracing::info!(
                "Decide@{:?}; Task@{:?} exiting; views < {:?} being reclaimed",
                decide_view_number.u64(),
                self.built_from_proposed_block.view_number.u64(),
                retained_view_cutoff.u64(),
            );
            return Some(Status::ShouldExit);
        }
        tracing::info!(
            "Decide@{:?}; Task@{:?} not exiting; views >= {:?} being retained",
            decide_view_number.u64(),
            self.built_from_proposed_block.view_number.u64(),
            retained_view_cutoff.u64(),
        );

        Some(Status::ShouldContinue)
    }

    // spawn a clone of the builder state
    #[tracing::instrument(skip_all, name = "spawn_clone",
                                    fields(builder_built_from_proposed_block = %self.built_from_proposed_block))]
    async fn spawn_clone(
        mut self,
        da_proposal_info: DAProposalInfo<TYPES>,
        quorum_proposal: Arc<Proposal<TYPES, QuorumProposal<TYPES>>>,
        _leader: TYPES::SignatureKey,
        req_sender: BroadcastSender<MessageType<TYPES>>,
    ) {
        self.total_nodes =
            NonZeroUsize::new(da_proposal_info.num_nodes).unwrap_or(self.total_nodes);
        self.built_from_proposed_block.view_number = quorum_proposal.data.view_number;
        self.built_from_proposed_block.vid_commitment =
            quorum_proposal.data.block_header.payload_commitment();
        self.built_from_proposed_block.builder_commitment =
            quorum_proposal.data.block_header.builder_commitment();
        let leaf = Leaf::from_quorum_proposal(&quorum_proposal.data);

        self.built_from_proposed_block.leaf_commit = leaf.commit();

        let encoded_txns = &da_proposal_info.proposal.data.encoded_transactions;

        let metadata = &da_proposal_info.proposal.data.metadata;

        let block_payload =
            <TYPES::BlockPayload as BlockPayload<TYPES>>::from_bytes(encoded_txns, metadata);
        let txn_commitments = block_payload.transaction_commitments(metadata);

        for tx in txn_commitments.iter() {
            self.txns_in_queue.remove(tx);
        }

        self.included_txns.extend(txn_commitments.iter());
        self.tx_queue
            .retain(|tx| self.txns_in_queue.contains(&tx.commit));

        // register the spawned builder state to spawned_builder_states in the global state
        self.global_state.write_arc().await.register_builder_state(
            BuilderStateId {
                parent_commitment: self.built_from_proposed_block.vid_commitment,
                view: self.built_from_proposed_block.view_number,
            },
            req_sender,
        );

        self.event_loop();
    }

    // build a block
    #[tracing::instrument(skip_all, name = "build block",
                                    fields(builder_built_from_proposed_block = %self.built_from_proposed_block))]
    async fn build_block(
        &mut self,
        state_id: BuilderStateId<TYPES>,
    ) -> Option<BuildBlockInfo<TYPES>> {
        let timeout_after = Instant::now() + self.maximize_txn_capture_timeout;
        let sleep_interval = self.maximize_txn_capture_timeout / 10;
        while Instant::now() <= timeout_after {
            self.collect_txns(timeout_after).await;

            if !self.tx_queue.is_empty() // we have transactions
            || Instant::now() + sleep_interval > timeout_after
            // we don't have time for another iteration
            {
                break;
            }

            async_sleep(sleep_interval).await
        }

        // Don't build an empty block
        if self.tx_queue.is_empty() {
            return None;
        }

        let max_block_size = self.global_state.read_arc().await.max_block_size;
        let transactions_to_include = self.tx_queue.iter().scan(0, |total_size, tx| {
            let prev_size = *total_size;
            *total_size += tx.len;
            // We will include one transaction over our target block length
            // if it's the first transaction in queue, otherwise we'd have a possible failure
            // state where a single transaction larger than target block state is stuck in
            // queue and we just build empty blocks forever
            if *total_size >= max_block_size && prev_size != 0 {
                None
            } else {
                Some(tx.tx.clone())
            }
        });

        if let Ok((payload, metadata)) =
            <TYPES::BlockPayload as BlockPayload<TYPES>>::from_transactions(
                transactions_to_include,
                &self.validated_state,
                &self.instance_state,
            )
            .await
        {
            let builder_hash = payload.builder_commitment(&metadata);
            // count the number of txns
            let actual_txn_count = payload.num_transactions(&metadata);

            // Payload is empty despite us checking that tx_queue isn't empty earlier.
            //
            // This means that the block was truncated due to *sequencer* block length
            // limits, which are different from our `max_block_size`. There's no good way
            // for us to check for this in advance, so we detect transactions too big for
            // the sequencer indirectly, by observing that we passed some transactions
            // to `<TYPES::BlockPayload as BlockPayload<TYPES>>::from_transactions`, but
            // it returned an empty block.
            // Thus we deduce that the first transaction in our queue is too big to *ever*
            // be included, because it alone goes over sequencer's block size limit.
            // We need to drop it and mark as "included" so that if we receive
            // it again we don't even bother with it.
            if actual_txn_count == 0 {
                if let Some(txn) = self.tx_queue.pop_front() {
                    self.txns_in_queue.remove(&txn.commit);
                    self.included_txns.insert(txn.commit);
                };
                return None;
            }

            // insert the recently built block into the builder commitments
            self.builder_commitments
                .insert((state_id, builder_hash.clone()));

            let encoded_txns: Vec<u8> = payload.encode().to_vec();
            let block_size: u64 = encoded_txns.len() as u64;
            let offered_fee: u64 = self.base_fee * block_size;

            // get the total nodes from the builder state.
            // stored while processing the DA Proposal
            let vid_num_nodes = self.total_nodes.get();

            let (trigger_send, trigger_recv) = oneshot();

            // spawn a task to calculate the VID commitment, and pass the handle to the global state
            // later global state can await on it before replying to the proposer
            let (unbounded_sender, unbounded_receiver) = unbounded();
            #[allow(unused_must_use)]
            async_spawn(async move {
                if let Ok(TriggerStatus::Start) = trigger_recv.recv().await {
                    let join_handle = spawn_blocking(move || {
                        precompute_vid_commitment(&encoded_txns, vid_num_nodes)
                    });
                    #[cfg(async_executor_impl = "tokio")]
                    let (vidc, pre_compute_data) = join_handle.await.unwrap();
                    #[cfg(async_executor_impl = "async-std")]
                    let (vidc, pre_compute_data) = join_handle.await;
                    unbounded_sender.send((vidc, pre_compute_data)).await;
                }
            });

            tracing::info!(
                "Builder view num {:?}, building block with {:?} txns, with builder hash {:?}",
                self.built_from_proposed_block.view_number,
                actual_txn_count,
                builder_hash
            );

            Some(BuildBlockInfo {
                id: BlockId {
                    view: self.built_from_proposed_block.view_number,
                    hash: builder_hash,
                },
                block_size,
                offered_fee,
                block_payload: payload,
                metadata,
                vid_trigger: trigger_send,
                vid_receiver: unbounded_receiver,
                truncated: actual_txn_count < self.tx_queue.len(),
            })
        } else {
            tracing::warn!("build block, returning None");
            None
        }
    }

    async fn process_block_request(&mut self, req: RequestMessage<TYPES>) {
        // If a spawned clone is active then it will handle the request, otherwise the highest view num builder will handle
        if (req.state_id.parent_commitment == self.built_from_proposed_block.vid_commitment
            && req.state_id.view == self.built_from_proposed_block.view_number)
            || (self.built_from_proposed_block.view_number.u64()
                == self
                    .global_state
                    .read_arc()
                    .await
                    .highest_view_num_builder_id
                    .view
                    .u64())
        {
            tracing::info!(
                "Request for parent {} handled by builder with view {:?}",
                req.state_id,
                self.built_from_proposed_block.view_number,
            );
            let response = self.build_block(req.state_id.clone()).await;

            match response {
                Some(response) => {
                    // form the response message
                    let response_msg = ResponseMessage {
                        builder_hash: response.id.hash.clone(),
                        block_size: response.block_size,
                        offered_fee: response.offered_fee,
                    };

                    let builder_hash = response.id.hash.clone();
                    self.global_state.write_arc().await.update_global_state(
                        req.state_id.clone(),
                        response,
                        response_msg.clone(),
                    );

                    // ... and finally, send the response
                    match req.response_channel.send(response_msg).await {
                        Ok(_sent) => {
                            tracing::info!(
                                "Builder {:?} Sent response to the request{:?} with builder hash {:?}",
                                self.built_from_proposed_block.view_number,
                                req,
                                builder_hash
                            );
                        }
                        Err(e) => {
                            tracing::warn!(
                                "Builder {:?} failed to send response to {:?} with builder hash {:?}, Err: {:?}",
                                self.built_from_proposed_block.view_number,
                                req,
                                builder_hash,
                                e
                            );
                        }
                    }
                }
                None => {
                    tracing::debug!("No response to send");
                }
            }
        } else {
            tracing::debug!(
                "Builder {:?} Requested Builder commitment does not match the built_from_view, so ignoring it",
                 self.built_from_proposed_block.view_number);
        }
    }
    #[tracing::instrument(skip_all, name = "event loop",
                                    fields(builder_built_from_proposed_block = %self.built_from_proposed_block))]
    pub fn event_loop(mut self) {
        let _builder_handle = async_spawn(async move {
            loop {
                tracing::debug!(
                    "Builder {:?} event loop",
                    self.built_from_proposed_block.view_number
                );
                futures::select! {
                    req = self.req_receiver.next() => {
                        tracing::debug!("Received request msg in builder {:?}: {:?}", self.built_from_proposed_block.view_number, req);
                        match req {
                            Some(req) => {
                                if let MessageType::RequestMessage(req) = req {
                                    tracing::debug!(
                                        "Received request msg in builder {:?}: {:?}",
                                        self.built_from_proposed_block.view_number,
                                        req
                                    );
                                    self.process_block_request(req).await;
                                } else {
                                    tracing::warn!("Unexpected message on requests channel: {:?}", req);
                                }
                            }
                            None => {
                                tracing::warn!("No more request messages to consume");
                            }
                        }
                    },
                    da = self.da_proposal_receiver.next() => {
                        match da {
                            Some(da) => {
                                if let MessageType::DaProposalMessage(rda_msg) = da {
                                    tracing::debug!("Received da proposal msg in builder {:?}:\n {:?}", self.built_from_proposed_block, rda_msg.proposal.data.view_number);
                                    self.process_da_proposal(rda_msg).await;
                                } else {
                                    tracing::warn!("Unexpected message on da proposals channel: {:?}", da);
                                }
                            }
                            None => {
                                tracing::warn!("No more da proposal messages to consume");
                            }
                        }
                    },
                    qc = self.qc_receiver.next() => {
                        match qc {
                            Some(qc) => {
                                if let MessageType::QCMessage(rqc_msg) = qc {
                                    tracing::debug!("Received quorum proposal msg in builder {:?}:\n {:?} for view ", self.built_from_proposed_block, rqc_msg.proposal.data.view_number);
                                    self.process_quorum_proposal(rqc_msg).await;
                                } else {
                                    tracing::warn!("Unexpected message on quorum proposals channel: {:?}", qc);
                                }
                            }
                            None => {
                                tracing::warn!("No more quorum proposal messages to consume");
                            }
                        }
                    },
                    decide = self.decide_receiver.next() => {
                        match decide {
                            Some(decide) => {
                                if let MessageType::DecideMessage(rdecide_msg) = decide {
                                    let latest_decide_view_num = rdecide_msg.latest_decide_view_number;
                                    tracing::debug!("Received decide msg view {:?} in builder {:?}",
                                        &latest_decide_view_num,
                                        self.built_from_proposed_block);
                                    let decide_status = self.process_decide_event(rdecide_msg).await;
                                    match decide_status{
                                        Some(Status::ShouldExit) => {
                                            tracing::info!("Exiting builder {:?} with decide view {:?}",
                                                self.built_from_proposed_block,
                                                &latest_decide_view_num);
                                            return;
                                        }
                                        Some(Status::ShouldContinue) => {
                                            tracing::debug!("Continuing builder {:?}",
                                                self.built_from_proposed_block);
                                            continue;
                                        }
                                        None => {
                                            tracing::warn!("decide_status was None; Continuing builder {:?}",
                                                self.built_from_proposed_block);
                                            continue;
                                        }
                                    }
                                } else {
                                    tracing::warn!("Unexpected message on decide channel: {:?}", decide);
                                }
                            }
                            None => {
                                tracing::warn!("No more decide messages to consume");
                            }
                        }
                    },
                };
            }
        });
    }
}
/// Unifies the possible messages that can be received by the builder
#[derive(Debug, Clone)]
pub enum MessageType<TYPES: NodeType> {
    DecideMessage(DecideMessage<TYPES>),
    DaProposalMessage(DaProposalMessage<TYPES>),
    QCMessage(QCMessage<TYPES>),
    RequestMessage(RequestMessage<TYPES>),
}

#[allow(clippy::too_many_arguments)]
impl<TYPES: NodeType> BuilderState<TYPES> {
    pub fn new(
        built_from_proposed_block: BuiltFromProposedBlock<TYPES>,
        decide_receiver: BroadcastReceiver<MessageType<TYPES>>,
        da_proposal_receiver: BroadcastReceiver<MessageType<TYPES>>,
        qc_receiver: BroadcastReceiver<MessageType<TYPES>>,
        req_receiver: BroadcastReceiver<MessageType<TYPES>>,
        tx_receiver: BroadcastReceiver<Arc<ReceivedTransaction<TYPES>>>,
        tx_queue: VecDeque<Arc<ReceivedTransaction<TYPES>>>,
        global_state: Arc<RwLock<GlobalState<TYPES>>>,
        num_nodes: NonZeroUsize,
        maximize_txn_capture_timeout: Duration,
        base_fee: u64,
        instance_state: Arc<TYPES::InstanceState>,
        txn_garbage_collect_duration: Duration,
        validated_state: Arc<TYPES::ValidatedState>,
    ) -> Self {
        let txns_in_queue: HashSet<_> = tx_queue.iter().map(|tx| tx.commit).collect();
        BuilderState {
            included_txns: HashSet::new(),
            included_txns_old: HashSet::new(),
            included_txns_expiring: HashSet::new(),
            txns_in_queue,
            built_from_proposed_block,
            decide_receiver,
            da_proposal_receiver,
            qc_receiver,
            req_receiver,
            da_proposal_payload_commit_to_da_proposal: HashMap::new(),
            quorum_proposal_payload_commit_to_quorum_proposal: HashMap::new(),
            tx_receiver,
            tx_queue,
            global_state,
            builder_commitments: HashSet::new(),
            total_nodes: num_nodes,
            maximize_txn_capture_timeout,
            base_fee,
            instance_state,
            txn_garbage_collect_duration,
            next_txn_garbage_collect_time: Instant::now() + txn_garbage_collect_duration,
            validated_state,
        }
    }
    pub fn clone_with_receiver(&self, req_receiver: BroadcastReceiver<MessageType<TYPES>>) -> Self {
        // Handle the garbage collection of txns
        let (
            included_txns,
            included_txns_old,
            included_txns_expiring,
            next_txn_garbage_collect_time,
        ) = if Instant::now() >= self.next_txn_garbage_collect_time {
            (
                HashSet::new(),
                self.included_txns.clone(),
                self.included_txns_old.clone(),
                Instant::now() + self.txn_garbage_collect_duration,
            )
        } else {
            (
                self.included_txns.clone(),
                self.included_txns_old.clone(),
                self.included_txns_expiring.clone(),
                self.next_txn_garbage_collect_time,
            )
        };

        BuilderState {
            included_txns,
            included_txns_old,
            included_txns_expiring,
            txns_in_queue: self.txns_in_queue.clone(),
            built_from_proposed_block: self.built_from_proposed_block.clone(),
            decide_receiver: self.decide_receiver.clone(),
            da_proposal_receiver: self.da_proposal_receiver.clone(),
            qc_receiver: self.qc_receiver.clone(),
            req_receiver,
            da_proposal_payload_commit_to_da_proposal: HashMap::new(),
            quorum_proposal_payload_commit_to_quorum_proposal: HashMap::new(),
            tx_receiver: self.tx_receiver.clone(),
            tx_queue: self.tx_queue.clone(),
            global_state: self.global_state.clone(),
            builder_commitments: self.builder_commitments.clone(),
            total_nodes: self.total_nodes,
            maximize_txn_capture_timeout: self.maximize_txn_capture_timeout,
            base_fee: self.base_fee,
            instance_state: self.instance_state.clone(),
            txn_garbage_collect_duration: self.txn_garbage_collect_duration,
            next_txn_garbage_collect_time,
            validated_state: self.validated_state.clone(),
        }
    }

    // collect outstanding transactions
    async fn collect_txns(&mut self, timeout_after: Instant) {
        while Instant::now() <= timeout_after {
            match self.tx_receiver.try_recv() {
                Ok(tx) => {
                    if self.included_txns.contains(&tx.commit)
                        || self.included_txns_old.contains(&tx.commit)
                        || self.included_txns_expiring.contains(&tx.commit)
                        || self.txns_in_queue.contains(&tx.commit)
                    {
                        continue;
                    }
                    self.txns_in_queue.insert(tx.commit);
                    self.tx_queue.push_back(tx);
                }
                Err(async_broadcast::TryRecvError::Empty)
                | Err(async_broadcast::TryRecvError::Closed) => {
                    break;
                }
                Err(async_broadcast::TryRecvError::Overflowed(lost)) => {
                    tracing::warn!("Missed {lost} transactions due to backlog");
                    continue;
                }
            }
        }
    }
}
