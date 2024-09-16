use hotshot_types::{
    data::{Leaf, QuorumProposal},
    message::Proposal,
    traits::block_contents::{BlockHeader, BlockPayload},
    traits::{
        node_implementation::{ConsensusTime, NodeType},
        EncodeBytes,
    },
    utils::BuilderCommitment,
};

use committable::{Commitment, Committable};

use crate::{
    service::{BroadcastReceivers, GlobalState, ReceivedTransaction},
    utils::{BlockId, BuilderStateId, BuiltFromProposedBlock, RotatingSet},
};
use async_broadcast::broadcast;
use async_broadcast::Receiver as BroadcastReceiver;
use async_broadcast::Sender as BroadcastSender;
use async_compatibility_layer::channel::UnboundedSender;
use async_compatibility_layer::{art::async_sleep, art::async_spawn};
use async_lock::RwLock;
use core::panic;
use futures::StreamExt;

use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Instant;
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
#[derive(Debug, Clone, PartialEq)]
pub struct DaProposalMessage<TYPES: NodeType> {
    pub view_number: TYPES::Time,
    pub txn_commitments: Vec<Commitment<TYPES::Transaction>>,
    pub sender: <TYPES as NodeType>::SignatureKey,
    pub builder_commitment: BuilderCommitment,
}

/// QC Message to be put on the quorum proposal channel
#[derive(Clone, Debug, PartialEq)]
pub struct QuorumProposalMessage<TYPES: NodeType> {
    pub proposal: Arc<Proposal<TYPES, QuorumProposal<TYPES>>>,
    pub sender: TYPES::SignatureKey,
}
/// Request Message to be put on the request channel
#[derive(Clone, Debug)]
pub struct RequestMessage<TYPES: NodeType> {
    pub requested_view_number: TYPES::Time,
    pub response_channel: UnboundedSender<ResponseMessage<TYPES>>,
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
}

/// Response Message to be put on the response channel
#[derive(Debug, Clone)]
pub struct ResponseMessage<TYPES: NodeType> {
    pub builder_hash: BuilderCommitment,
    pub transactions: Vec<TYPES::Transaction>,
    pub block_size: u64,
    pub offered_fee: u64,
}
#[derive(Debug, Clone)]
/// Enum to hold the status out of the decide event
pub enum Status {
    ShouldExit,
    ShouldContinue,
}

// implement display for the derived info
impl<TYPES: NodeType> std::fmt::Display for BuiltFromProposedBlock<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "View Number: {:?}", self.view_number)
    }
}

/// Builder State to hold the state of the builder
#[derive(Debug)]
pub struct BuilderState<TYPES: NodeType> {
    pub included_txns: RotatingSet<Commitment<TYPES::Transaction>>,

    /// txns currently in the tx_queue
    pub txns_in_queue: HashSet<Commitment<TYPES::Transaction>>,

    /// da_proposal_payload_commit to (da_proposal, node_count)
    #[allow(clippy::type_complexity)]
    pub da_proposal_payload_commit_to_da_proposal:
        HashMap<(BuilderCommitment, TYPES::Time), Arc<DaProposalMessage<TYPES>>>,

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
    pub tx_queue: Vec<Arc<ReceivedTransaction<TYPES>>>,

    /// global state handle, defined in the service.rs
    pub global_state: Arc<RwLock<GlobalState<TYPES>>>,

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
}

/// [best_builder_states_to_extend] is a utility function that is used to
/// in order to determine which [BuilderState]s are the best fit to extend
/// from.
///
/// This function is designed to inspect the current state of the global state
/// in order to determine which [BuilderState]s are the best fit to extend
/// from. We only want to use information from [GlobalState] as otherwise
/// we would have some insider knowledge unique to our specific [BuilderState]
/// rather than knowledge that is available to all [BuilderState]s. In fact,
/// in order to ensure this, this function lives outside of the [BuilderState]
/// itself.
///
/// In an ideal circumstance the best [BuilderState] to extend from is going to
/// be the one that is immediately preceding the [QuorumProposal] that we are
/// attempting to extend from. However, if all we know is the [ViewNumber] of
/// the [QuorumProposal] that we are attempting to extend from, then we may end
/// up in a scenario where we have multiple [BuilderState]s that are all equally
/// valid to extend from.  When this happens, we have the potential for a data
/// race.
///
/// The primary cause of this has to due with the interface of the
/// [ProxyGlobalState]'s API.  In general, we want to be able to retrieve a
/// [BuilderState] via the [BuilderStateId].  The [BuilderStateId] only
/// references a [ViewNumber] and a [VidCommitment]. While this information is
/// available in the [QuorumProposal], it only helps us to rule out
/// [BuilderState]s that already exist.  It does **NOT** help us to pick a
/// [BuilderState] that is the best fit to extend from.
///
/// This is where the `justify_qc` comes in to consideration.  The `justify_qc`
/// contains the previous [ViewNumber] that is being extended from, and in
/// addition it also contains the previous [Commitment<Leaf<TYPES>>] that is
/// being built on top of.  Since our [BuilderState]s store identifying
/// information that contains this same `leaf_commit` we can compare these
/// directly to ensure that we are extending from the correct [BuilderState].
///
/// This function determines the best [BuilderState] in the following steps:
///
/// 1. If we have a [BuilderState] that is already spawned for the current
///    [QuorumProposal], then we should should return no states, as one already
///    exists.  This will prevent us from attempting to spawn duplicate
///    [BuilderState]s.
/// 2. Attempt to find all [BuilderState]s that are recorded within
///    [GlobalState] that have matching view number and leaf commitments. There
///    *should* only be one of these.  But all would be valid extension points.
/// 3. If we can't find any [BuilderState]s that match the view number
///    and leaf commitment, then we should return for the maximum stored view
///    number that is smaller than the current [QuorumProposal].
/// 4. If there is is only one [BuilderState] stored in the [GlobalState], then
///    we should return that [BuilderState] as the best fit.
/// 5. If none of the other criteria match, we return an empty result as it is
///    unclear what to do in this case.
///
/// > Note: Any time this function returns more than a single entry in its
/// > [HashSet] result, there is a potential for a race condition.  This is
/// > because there are multiple [BuilderState]s that are equally valid to
/// > extend from.  This race could be avoided by just picking one of the
/// > entries in the resulting [HashSet], but this is not done here in order
/// > to allow us to highlight the possibility of the race.
async fn best_builder_states_to_extend<TYPES: NodeType>(
    quorum_proposal: Arc<Proposal<TYPES, QuorumProposal<TYPES>>>,
    global_state: Arc<RwLock<GlobalState<TYPES>>>,
) -> HashSet<BuilderStateId<TYPES>> {
    let current_view_number = quorum_proposal.data.view_number;
    let current_commitment = quorum_proposal.data.block_header.payload_commitment();
    let current_builder_state_id = BuilderStateId::<TYPES> {
        parent_commitment: current_commitment,
        parent_view: current_view_number,
    };

    let global_state_read_lock = global_state.read_arc().await;

    // The first step is to check if we already have a spawned [BuilderState].
    // If we do, then we should indicate that there is no best fit, as we
    // don't want to spawn another [BuilderState].
    if global_state_read_lock
        .spawned_builder_states
        .contains_key(&current_builder_state_id)
    {
        // We already have a spawned [BuilderState] for this proposal.
        // So we should just ignore it.
        return HashSet::new();
    }

    // Next we want to see if there is an immediate match for a [BuilderState]
    // that we can extend from.  This is the most ideal situation, as it
    // implies that we are extending from the correct [BuilderState].
    // We do this by checking the `justify_qc` stored within the
    // [QuorumProposal], and checking it against the current spawned
    // [BuilderState]s
    let justify_qc = &quorum_proposal.data.justify_qc;
    let existing_states: HashSet<_> = global_state_read_lock
        .spawned_builder_states
        .iter()
        .filter(
            |(_, (built_from_proposed_block, _))| match built_from_proposed_block {
                None => false,
                Some(built_from_proposed_block) => {
                    built_from_proposed_block.leaf_commit == justify_qc.data.leaf_commit
                        && built_from_proposed_block.view_number == justify_qc.view_number
                }
            },
        )
        .map(|(builder_state_id, _)| builder_state_id.clone())
        .collect();

    // If we found any matching [BuilderState]s, then we should return them
    // as the best fit.
    if !existing_states.is_empty() {
        return existing_states;
    }

    // At this point, we don't have any "ideal" matches or scenarios.  So we
    // need to look for a suitable fall-back. The best fallback condition to
    // start with is any [BuilderState] that has the maximum spawned view
    // number whose value is smaller than the current [QuorumProposal].
    let maximum_stored_view_number_smaller_than_quorum_proposal = global_state_read_lock
        .spawned_builder_states
        .keys()
        .map(|builder_state_id| *builder_state_id.parent_view)
        .filter(|view_number| view_number < &current_view_number)
        .max();

    // If we have a maximum view number that meets our criteria, then we should
    // return all [BuilderStateId]s that match this view number.
    // This can lead to multiple [BuilderStateId]s being returned.
    if let Some(maximum_stored_view_number_smaller_than_quorum_proposal) =
        maximum_stored_view_number_smaller_than_quorum_proposal
    {
        // If we are the maximum stored view number smaller than the quorum
        // proposal's view number, then we are the best fit.
        let mut result = HashSet::new();
        for builder_state_id in
            global_state_read_lock
                .spawned_builder_states
                .keys()
                .filter(|builder_state_id| {
                    builder_state_id.parent_view.u64()
                        == maximum_stored_view_number_smaller_than_quorum_proposal
                })
        {
            result.insert(builder_state_id.clone());
        }
        return result;
    }

    // This is our last ditch effort to continue making progress.  If there is
    // only one [BuilderState] active, then we should return that as the best
    // fit, as it will be the only way we can continue making progress with
    // the builder.
    if global_state_read_lock.spawned_builder_states.len() == 1 {
        let mut result = HashSet::new();
        for builder_state_id in global_state_read_lock.spawned_builder_states.keys() {
            result.insert(builder_state_id.clone());
        }
        return result;
    }

    // This implies that there are only larger [BuilderState]s active than
    // the one we are.  This is weird, it implies that some sort of time
    // travel has occurred view-wise.  It is unclear what to do in this
    // situation.

    HashSet::new()
}

impl<TYPES: NodeType> BuilderState<TYPES> {
    /// [am_i_the_best_builder_state_to_extend] is a utility method that
    /// attempts to determine whether we are among the best [BuilderState]s to
    /// extend from.
    async fn am_i_the_best_builder_state_to_extend(
        &self,
        quorum_proposal: Arc<Proposal<TYPES, QuorumProposal<TYPES>>>,
    ) -> bool {
        let best_builder_states_to_extend =
            best_builder_states_to_extend(quorum_proposal.clone(), self.global_state.clone()).await;

        tracing::debug!(
            "{}@{} thinks these are the best builder states to extend from: {:?} for proposal {}@{}",
            self.built_from_proposed_block.vid_commitment,
            self.built_from_proposed_block.view_number.u64(),
            best_builder_states_to_extend
                .iter()
                .map(|builder_state_id| format!(
                    "{}@{}",
                    builder_state_id.parent_commitment,
                    builder_state_id.parent_view.u64()
                ))
                .collect::<Vec<String>>(),
            quorum_proposal.data.block_header.payload_commitment(),
            quorum_proposal.data.view_number.u64(),
        );

        // We are a best fit if we are contained within the returned set of
        // best [BuilderState]s to extend from.
        best_builder_states_to_extend.contains(&BuilderStateId {
            parent_commitment: self.built_from_proposed_block.vid_commitment,
            parent_view: self.built_from_proposed_block.view_number,
        })
    }

    /// processing the DA proposal
    /// pending whether it's processed already, if so, skip, or else, process it
    /// deciding whether we have matching quorum proposal, if so, we remove them from the storage and
    /// spawn a clone
    #[tracing::instrument(skip_all, name = "process da proposal",
                                    fields(builder_built_from_proposed_block = %self.built_from_proposed_block))]
    async fn process_da_proposal(&mut self, da_msg: Arc<DaProposalMessage<TYPES>>) {
        tracing::debug!(
            "Builder Received DA message for view {:?}",
            da_msg.view_number
        );

        // we do not have the option to ignore DA proposals if we want to be able to handle failed view reorgs.

        // If the respective builder state exists to handle the request
        tracing::debug!(
            "Extracted builder commitment from the da proposal: {:?}",
            da_msg.builder_commitment
        );

        // If we've never processed this da proposal before, process it
        if let std::collections::hash_map::Entry::Vacant(e) = self
            .da_proposal_payload_commit_to_da_proposal
            .entry((da_msg.builder_commitment.clone(), da_msg.view_number))
        {
            // if we have matching da and quorum proposals, we can skip storing the one, and remove
            // the other from storage, and call build_block with both, to save a little space.
            if let Entry::Occupied(quorum_proposal) = self
                .quorum_proposal_payload_commit_to_quorum_proposal
                .entry((da_msg.builder_commitment.clone(), da_msg.view_number))
            {
                let quorum_proposal = quorum_proposal.remove();

                // if we have a matching quorum proposal
                // which means they have matching BuilderCommitment and view number
                //  if (this is the correct parent or
                //      (the correct parent is missing and this is the highest view))
                //    spawn a clone
                if quorum_proposal.data.view_number == da_msg.view_number {
                    tracing::info!(
                        "Spawning a clone from process DA proposal for view number: {:?}",
                        da_msg.view_number
                    );
                    // remove this entry from quorum_proposal_payload_commit_to_quorum_proposal
                    self.quorum_proposal_payload_commit_to_quorum_proposal
                        .remove(&(da_msg.builder_commitment.clone(), da_msg.view_number));

                    self.spawn_clone_that_extends_self(da_msg, quorum_proposal)
                        .await;
                } else {
                    tracing::debug!("Not spawning a clone despite matching DA and QC payload commitments, as they corresponds to different view numbers");
                }
            } else {
                e.insert(da_msg);
            }
        } else {
            tracing::debug!("Payload commitment already exists in the da_proposal_payload_commit_to_da_proposal hashmap, so ignoring it");
        }
    }

    /// processing the quorum proposal
    //#[tracing::instrument(skip_all, name = "Process Quorum Proposal")]
    #[tracing::instrument(skip_all, name = "process quorum proposal",
                                    fields(builder_built_from_proposed_block = %self.built_from_proposed_block))]
    async fn process_quorum_proposal(&mut self, qc_msg: QuorumProposalMessage<TYPES>) {
        tracing::debug!(
            "Builder Received QC Message for view {:?}",
            qc_msg.proposal.data.view_number
        );

        // Two cases to handle:
        // Case 1: Bootstrapping phase
        // Case 2: No intended builder state exist
        // To handle both cases, we can have the highest view number builder state running
        // and only doing the insertion if and only if intended builder state for a particular view is not present
        // check the presence of quorum_proposal.data.view_number-1 in the spawned_builder_states list
        let quorum_proposal = &qc_msg.proposal;
        let view_number = quorum_proposal.data.view_number;
        let payload_builder_commitment = quorum_proposal.data.block_header.builder_commitment();

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

                    self.spawn_clone_that_extends_self(da_proposal_info, quorum_proposal.clone())
                        .await;
                } else {
                    tracing::debug!("Not spawning a clone despite matching DA and QC payload commitments, as they corresponds to different view numbers");
                }
            } else {
                e.insert(quorum_proposal.clone());
            }
        } else {
            tracing::debug!("Payload commitment already exists in the quorum_proposal_payload_commit_to_quorum_proposal hashmap, so ignoring it");
        }
    }

    /// [spawn_a_clone_that_extends_self] is a helper function that is used by
    /// both [process_da_proposal] and [process_quorum_proposal] to spawn a
    /// new [BuilderState] that extends from the current [BuilderState].
    ///
    /// This helper function also adds additional checks in order to ensure
    /// that the [BuilderState] that is being spawned is the best fit for the
    /// [QuorumProposal] that is being extended from.
    async fn spawn_clone_that_extends_self(
        &mut self,
        da_proposal_info: Arc<DaProposalMessage<TYPES>>,
        quorum_proposal: Arc<Proposal<TYPES, QuorumProposal<TYPES>>>,
    ) {
        if !self
            .am_i_the_best_builder_state_to_extend(quorum_proposal.clone())
            .await
        {
            tracing::debug!(
                "{} is not the best fit for forking, {}@{}, so ignoring the QC proposal, and leaving it to another BuilderState",
                self.built_from_proposed_block,
                quorum_proposal.data.block_header.payload_commitment(),
                quorum_proposal.data.view_number.u64(),
            );
            return;
        }

        let (req_sender, req_receiver) = broadcast(self.req_receiver.capacity());

        tracing::debug!(
            "extending BuilderState with a clone from {} with new proposal {}@{}",
            self.built_from_proposed_block,
            quorum_proposal.data.block_header.payload_commitment(),
            quorum_proposal.data.view_number.u64()
        );

        // We literally fork ourselves
        self.clone_with_receiver(req_receiver)
            .spawn_clone(da_proposal_info, quorum_proposal.clone(), req_sender)
            .await;
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

    /// spawn a clone of the builder state
    #[tracing::instrument(skip_all, name = "spawn_clone",
                                    fields(builder_built_from_proposed_block = %self.built_from_proposed_block))]
    async fn spawn_clone(
        mut self,
        da_proposal_info: Arc<DaProposalMessage<TYPES>>,
        quorum_proposal: Arc<Proposal<TYPES, QuorumProposal<TYPES>>>,
        req_sender: BroadcastSender<MessageType<TYPES>>,
    ) {
        let leaf = Leaf::from_quorum_proposal(&quorum_proposal.data);

        // We replace our built_from_proposed_block with information from the
        // quorum proposal.  This is identifying the block that this specific
        // instance of [BuilderState] is attempting to build for.
        self.built_from_proposed_block = BuiltFromProposedBlock {
            view_number: quorum_proposal.data.view_number,
            vid_commitment: quorum_proposal.data.block_header.payload_commitment(),
            leaf_commit: leaf.commit(),
            builder_commitment: quorum_proposal.data.block_header.builder_commitment(),
        };

        let builder_state_id = BuilderStateId {
            parent_commitment: self.built_from_proposed_block.vid_commitment,
            parent_view: self.built_from_proposed_block.view_number,
        };

        {
            // Let's ensure that we don't already have one of these BuilderStates
            // running already.

            let global_state_read_lock = self.global_state.read_arc().await;
            if global_state_read_lock
                .spawned_builder_states
                .contains_key(&builder_state_id)
            {
                tracing::warn!(
                    "Aborting spawn_clone, builder state already exists in spawned_builder_states: {:?}",
                    builder_state_id
                );
                return;
            }
        }

        for tx in da_proposal_info.txn_commitments.iter() {
            self.txns_in_queue.remove(tx);
        }
        self.included_txns
            .extend(da_proposal_info.txn_commitments.iter().cloned());

        self.tx_queue
            .retain(|tx| self.txns_in_queue.contains(&tx.commit));

        // register the spawned builder state to spawned_builder_states in the
        // global state We register this new child within the global_state, so
        // that it can be looked up via the [BuilderStateId] in the future.
        self.global_state.write_arc().await.register_builder_state(
            builder_state_id,
            self.built_from_proposed_block.clone(),
            req_sender,
        );

        self.event_loop();
    }

    // Sishan TODO: the function returns a vec of transactions
    /// build a block from the BuilderStateId
    /// This function collects available transactions not already in `included_txns` from near future
    /// then form them into a block and calculate the block's `builder_hash` `block_payload` and `metadata`
    /// insert `builder_hash` to `builder_commitments`
    /// and finally return a struct in `BuildBlockInfo`
    #[tracing::instrument(skip_all, name = "build block",
                                    fields(builder_built_from_proposed_block = %self.built_from_proposed_block))]
    async fn build_block(
        &mut self,
        state_id: BuilderStateId<TYPES>,
    ) -> Option<BuildBlockInfo<TYPES>> {
        // collect all the transactions from the near future
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
        if let Ok((payload, metadata)) =
            <TYPES::BlockPayload as BlockPayload<TYPES>>::from_transactions(
                self.tx_queue.iter().map(|tx| tx.tx.clone()),
                &self.validated_state,
                &self.instance_state,
            )
            .await
        {
            let builder_hash = payload.builder_commitment(&metadata);
            // count the number of txns
            let txn_count = payload.num_transactions(&metadata);

            // insert the recently built block into the builder commitments
            self.builder_commitments
                .insert((state_id, builder_hash.clone()));

            let encoded_txns: Vec<u8> = payload.encode().to_vec();
            let block_size: u64 = encoded_txns.len() as u64;
            let offered_fee: u64 = self.base_fee * block_size;

            tracing::info!(
                "Builder view num {:?}, building block with {:?} txns, with builder hash {:?}",
                self.built_from_proposed_block.view_number,
                txn_count,
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
            })
        } else {
            tracing::warn!("build block, returning None");
            None
        }
    }

    async fn process_block_request(&mut self, req: RequestMessage<TYPES>) {
        let requested_view_number = req.requested_view_number;
        // If a spawned clone is active then it will handle the request, otherwise the highest view num builder will handle
        if requested_view_number == self.built_from_proposed_block.view_number {
            tracing::info!(
                "Request handled by builder with view {}@{:?} for (view_num: {:?})",
                self.built_from_proposed_block.vid_commitment,
                self.built_from_proposed_block.view_number,
                requested_view_number
            );
            let response = self
                .build_block(BuilderStateId {
                    parent_commitment: self.built_from_proposed_block.vid_commitment,
                    parent_view: requested_view_number,
                })
                .await;

            match response {
                Some(response) => {
                    // form the response message
                    let response_msg = ResponseMessage {
                        builder_hash: response.id.hash.clone(),
                        block_size: response.block_size,
                        offered_fee: response.offered_fee,
                        transactions: response
                            .block_payload
                            .transactions(&response.metadata)
                            .collect(),
                    };

                    let builder_hash = response.id.hash.clone();
                    self.global_state.write_arc().await.update_global_state(
                        BuilderStateId {
                            parent_commitment: self.built_from_proposed_block.vid_commitment,
                            parent_view: requested_view_number,
                        },
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
                                    tracing::debug!("Received da proposal msg in builder {:?}:\n {:?}", self.built_from_proposed_block, rda_msg.view_number);
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
                                if let MessageType::QuorumProposalMessage(rqc_msg) = qc {
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
    DaProposalMessage(Arc<DaProposalMessage<TYPES>>),
    QuorumProposalMessage(QuorumProposalMessage<TYPES>),
    RequestMessage(RequestMessage<TYPES>),
}

#[allow(clippy::too_many_arguments)]
impl<TYPES: NodeType> BuilderState<TYPES> {
    pub fn new(
        built_from_proposed_block: BuiltFromProposedBlock<TYPES>,
        receivers: &BroadcastReceivers<TYPES>,
        req_receiver: BroadcastReceiver<MessageType<TYPES>>,
        tx_queue: Vec<Arc<ReceivedTransaction<TYPES>>>,
        global_state: Arc<RwLock<GlobalState<TYPES>>>,
        maximize_txn_capture_timeout: Duration,
        base_fee: u64,
        instance_state: Arc<TYPES::InstanceState>,
        txn_garbage_collect_duration: Duration,
        validated_state: Arc<TYPES::ValidatedState>,
    ) -> Self {
        let txns_in_queue: HashSet<_> = tx_queue.iter().map(|tx| tx.commit).collect();
        BuilderState {
            txns_in_queue,
            built_from_proposed_block,
            req_receiver,
            tx_queue,
            global_state,
            maximize_txn_capture_timeout,
            base_fee,
            instance_state,
            validated_state,
            included_txns: RotatingSet::new(txn_garbage_collect_duration),
            da_proposal_payload_commit_to_da_proposal: HashMap::new(),
            quorum_proposal_payload_commit_to_quorum_proposal: HashMap::new(),
            builder_commitments: HashSet::new(),
            decide_receiver: receivers.decide.activate_cloned(),
            da_proposal_receiver: receivers.da_proposal.activate_cloned(),
            qc_receiver: receivers.quorum_proposal.activate_cloned(),
            tx_receiver: receivers.transactions.activate_cloned(),
        }
    }
    pub fn clone_with_receiver(&self, req_receiver: BroadcastReceiver<MessageType<TYPES>>) -> Self {
        let mut included_txns = self.included_txns.clone();
        included_txns.rotate();

        BuilderState {
            included_txns,
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
            maximize_txn_capture_timeout: self.maximize_txn_capture_timeout,
            base_fee: self.base_fee,
            instance_state: self.instance_state.clone(),
            validated_state: self.validated_state.clone(),
        }
    }

    // collect outstanding transactions
    async fn collect_txns(&mut self, timeout_after: Instant) {
        while Instant::now() <= timeout_after {
            match self.tx_receiver.try_recv() {
                Ok(tx) => {
                    if self.included_txns.contains(&tx.commit)
                        || self.txns_in_queue.contains(&tx.commit)
                    {
                        continue;
                    }
                    self.txns_in_queue.insert(tx.commit);
                    self.tx_queue.push(tx);
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::marker::PhantomData;
    use std::sync::Arc;
    use std::time::Duration;

    use async_broadcast::broadcast;
    use async_lock::RwLock;
    use committable::{Commitment, CommitmentBoundsArkless, Committable};
    use hotshot::traits::BlockPayload;
    use hotshot::types::{BLSPubKey, SignatureKey};
    use hotshot_example_types::block_types::{TestBlockHeader, TestBlockPayload, TestMetadata};
    use hotshot_example_types::state_types::{TestInstanceState, TestValidatedState};
    use hotshot_example_types::block_types::TestTransaction;
    use hotshot_task_impls::transactions;
    use hotshot_types::data::{Leaf, QuorumProposal};
    use hotshot_types::simple_certificate::{QuorumCertificate, SimpleCertificate, SuccessThreshold};
    use hotshot_types::simple_vote::QuorumData;
    use hotshot_types::traits::block_contents::vid_commitment;
    use hotshot_types::traits::node_implementation::{ConsensusTime, NodeType};
    use hotshot_types::utils::BuilderCommitment;
    use hotshot_types::{data::ViewNumber, traits::signature_key::BuilderSignatureKey};
    use rkyv::collections::ArchivedHashMap;

    use crate::service::{broadcast_channels, GlobalState};
    use crate::utils::BuilderStateId;

    use super::BuilderState;
    use super::BuiltFromProposedBlock;
    use super::DaProposalMessage;
    use super::MessageType;
    use rkyv::{Deserialize, Infallible};
    use crate::testing::{calc_proposal_msg, TestTypes};

    /// This test checkes da_proposal_payload_commit_to_da_proposal and
    /// quorum_proposal_payload_commit_to_quorum_proposal change appropriately
    /// when receiving a da message.
    /// This test also checks whether corresponding BuilderStateId is in global_state.
    #[async_std::test]
    async fn test_process_da_proposal() {
        async_compatibility_layer::logging::setup_logging();
        async_compatibility_layer::logging::setup_backtrace();
        tracing::info!("Testing the function `test_process_da_proposal` in `builder_state.rs`");

        // Number of views to simulate
        const NUM_ROUNDS: usize = 5;
        // Number of transactions to submit per round
        const NUM_TXNS_PER_ROUND: usize = 4;
        // Capacity of broadcast channels
        const CHANNEL_CAPACITY: usize = NUM_ROUNDS * 5;
        // Number of nodes on DA committee
        const NUM_STORAGE_NODES: usize = 4;

        let (sender_public_key, sender_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);

        let (leader_public_key, leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);

        let channel_capacity = CHANNEL_CAPACITY;
        let num_storage_nodes = NUM_STORAGE_NODES;
        // set up the broadcast channels
        let (bootstrap_sender, bootstrap_receiver) =
            broadcast::<MessageType<TestTypes>>(channel_capacity);
        let (senders, receivers) = broadcast_channels(channel_capacity);

        let genesis_vid_commitment = vid_commitment(&[], num_storage_nodes);
        let genesis_builder_commitment = BuilderCommitment::from_bytes([]);
        let built_from_info = BuiltFromProposedBlock {
            view_number: ViewNumber::genesis(),
            vid_commitment: genesis_vid_commitment,
            leaf_commit: Commitment::<Leaf<TestTypes>>::default_commitment_no_preimage(),
            builder_commitment: genesis_builder_commitment,
        };

        // instantiate the global state
        let global_state = Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
            bootstrap_sender,
            senders.transactions.clone(),
            genesis_vid_commitment,
            ViewNumber::genesis(),
        )));

        // instantiate the bootstrap builder state
        let mut builder_state = BuilderState::<TestTypes>::new(
            built_from_info,
            &receivers,
            bootstrap_receiver,
            Vec::new(),
            Arc::clone(&global_state),
            Duration::from_millis(10), // max time to wait for non-zero txn block
            0,                         // base fee
            Arc::new(TestInstanceState::default()),
            Duration::from_secs(3600), // duration for txn garbage collection
            Arc::new(TestValidatedState::default()),
        );

        // randomly generate a transaction
        let transactions = vec![TestTransaction::new(vec![1, 2, 3]); 3];
        let (_quorum_proposal, _quorum_proposal_msg, da_proposal_msg, builder_state_id) =
            calc_proposal_msg(NUM_STORAGE_NODES, 0, None, transactions.clone()).await;

        // sub-test one
        // call process_da_proposal without matching quorum proposal message
        // da_proposal_payload_commit_to_da_proposal should insert the message
        let mut correct_da_proposal_payload_commit_to_da_proposal: HashMap<
            (BuilderCommitment, <TestTypes as NodeType>::Time),
            Arc<DaProposalMessage<TestTypes>>,
        > = HashMap::new();
        if let MessageType::DaProposalMessage(practice_da_msg) = da_proposal_msg.clone() {
            builder_state
                .process_da_proposal(practice_da_msg.clone())
                .await;
            correct_da_proposal_payload_commit_to_da_proposal.insert(
                (
                    practice_da_msg.builder_commitment.clone(),
                    practice_da_msg.view_number,
                ),
                practice_da_msg,
            );
        } else {
            tracing::error!("Not a da_proposal_message in correct format");
        }
        let deserialized_map: HashMap<_, _> =
            builder_state.da_proposal_payload_commit_to_da_proposal.clone();
        for (key, value) in deserialized_map.iter() {
            let correct_value = correct_da_proposal_payload_commit_to_da_proposal.get(key);
            assert_eq!(
                value.as_ref().clone(),
                rkyv::option::ArchivedOption::Some(correct_value)
                    .unwrap()
                    .unwrap()
                    .as_ref()
                    .clone()
            );
        }
        // check global_state didn't change
        if let Some(_x) = global_state
            .read_arc()
            .await
            .spawned_builder_states
            .get(&builder_state_id) {
                panic!("global_state shouldn't have cooresponding builder_state_id without matching quorum proposal.");
        }
        
        // sub-test two
        // call process_da_proposal with the same msg again
        // we should skip the process and everything should be the same
        let transactions_1 = transactions.clone();
        let (_quorum_proposal_1, _quorum_proposal_msg_1, da_proposal_msg_1, builder_state_id_1) =
            calc_proposal_msg(NUM_STORAGE_NODES, 0, None, transactions_1).await;
        if let MessageType::DaProposalMessage(practice_da_msg_1) = da_proposal_msg_1.clone() {
            builder_state
                .process_da_proposal(practice_da_msg_1.clone())
                .await;
        } else {
            tracing::error!("Not a da_proposal_message in correct format");
        }
        let deserialized_map: HashMap<_, _> =
            builder_state.da_proposal_payload_commit_to_da_proposal.clone();
        for (key, value) in deserialized_map.iter() {
            let correct_value = correct_da_proposal_payload_commit_to_da_proposal.get(key);
            assert_eq!(
                value.as_ref().clone(),
                rkyv::option::ArchivedOption::Some(correct_value)
                    .unwrap()
                    .unwrap()
                    .as_ref()
                    .clone()
            );
        }
        // check global_state didn't change
        if let Some(_x) = global_state
            .read_arc()
            .await
            .spawned_builder_states
            .get(&builder_state_id_1) {
                panic!("global_state shouldn't have cooresponding builder_state_id without matching quorum proposal.");
        }

        // sub-test three
        // add the matching quorum proposal message with different tx
        // and call process_da_proposal with this matching da proposal message and quorum proposal message
        // we should spawn_clone here
        // and check whether global_state has correct BuilderStateId
        let transactions_2 = vec![TestTransaction::new(vec![1, 2, 3, 4]); 2];
        let (_quorum_proposal_2, quorum_proposal_msg_2, da_proposal_msg_2, builder_state_id_2) =
            calc_proposal_msg(NUM_STORAGE_NODES, 0, None, transactions_2).await;

        // process quorum proposal first, so that later when process_da_proposal we can directly call `build_block` and skip storage 
        if let MessageType::QuorumProposalMessage(practice_qc_msg_2) = quorum_proposal_msg_2.clone() {
            builder_state
                .process_quorum_proposal(practice_qc_msg_2.clone())
                .await;
        } else {
            tracing::error!("Not a quorum_proposal_message in correct format");
        }

        // process da proposal message and do the check
        if let MessageType::DaProposalMessage(practice_da_msg_2) = da_proposal_msg_2.clone() {
            builder_state
                .process_da_proposal(practice_da_msg_2.clone())
                .await;
        } else {
            tracing::error!("Not a da_proposal_message in correct format");
        }
        let deserialized_map_2: HashMap<_, _> =
            builder_state.da_proposal_payload_commit_to_da_proposal;
        for (key, value) in deserialized_map_2.iter() {
            let correct_value = correct_da_proposal_payload_commit_to_da_proposal.get(key);
            assert_eq!(
                value.as_ref().clone(),
                rkyv::option::ArchivedOption::Some(correct_value)
                    .unwrap()
                    .unwrap()
                    .as_ref()
                    .clone()
            );
        }
        // check global_state has this new builder_state_id
        if let Some(_x) = global_state
            .read_arc()
            .await
            .spawned_builder_states
            .get(&builder_state_id_2) {
                tracing::debug!("global_state updated successfully");
        } else {
            panic!("global_state shouldn't have cooresponding builder_state_id without matching quorum proposal.");
        }
        

    }
}
