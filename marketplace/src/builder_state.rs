use hotshot_types::{
    data::{Leaf, QuorumProposal},
    message::Proposal,
    traits::{
        block_contents::{BlockHeader, BlockPayload},
        node_implementation::{ConsensusTime, NodeType},
        EncodeBytes,
    },
    utils::BuilderCommitment,
};

use committable::Commitment;

use crate::{
    service::{BroadcastReceivers, GlobalState, ReceivedTransaction},
    utils::{BlockId, BuilderStateId, LegacyCommit as _, ParentBlockReferences, RotatingSet},
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

/// Quorum proposal message to be put on the quorum proposal channel
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

/// Builder State to hold the state of the builder
#[derive(Debug)]
pub struct BuilderState<TYPES: NodeType> {
    /// txns that have been included in recent blocks that have
    /// been built.  This is used to try and guarantee that a transaction
    /// isn't duplicated.
    /// Keeps a history of the last 3 proposals.
    pub included_txns: RotatingSet<Commitment<TYPES::Transaction>>,

    /// txn commits currently in the tx_queue.  This is used as a quick
    /// check for whether a transaction is already in the tx_queue or
    /// not.
    ///
    /// This should be kept up-to-date with the tx_queue as it acts as an
    /// accessory to the tx_queue.
    pub txn_commits_in_queue: HashSet<Commitment<TYPES::Transaction>>,

    /// filtered queue of available transactions, taken from tx_receiver
    pub tx_queue: Vec<Arc<ReceivedTransaction<TYPES>>>,

    /// da_proposal_payload_commit to (da_proposal, node_count)
    #[allow(clippy::type_complexity)]
    pub da_proposal_payload_commit_to_da_proposal:
        HashMap<(BuilderCommitment, TYPES::Time), Arc<DaProposalMessage<TYPES>>>,

    /// quorum_proposal_payload_commit to quorum_proposal
    #[allow(clippy::type_complexity)]
    pub quorum_proposal_payload_commit_to_quorum_proposal:
        HashMap<(BuilderCommitment, TYPES::Time), Arc<Proposal<TYPES, QuorumProposal<TYPES>>>>,

    /// Spawned-from references to the parent block.
    pub parent_block_references: ParentBlockReferences<TYPES>,

    // Channel Receivers for the HotShot events, Tx_receiver could also receive the external transactions
    /// decide receiver
    pub decide_receiver: BroadcastReceiver<MessageType<TYPES>>,

    /// da proposal receiver
    pub da_proposal_receiver: BroadcastReceiver<MessageType<TYPES>>,

    /// quorum proposal receiver
    pub quorum_proposal_receiver: BroadcastReceiver<MessageType<TYPES>>,

    /// channel receiver for the block requests
    pub req_receiver: BroadcastReceiver<MessageType<TYPES>>,

    /// incoming stream of transactions
    pub tx_receiver: BroadcastReceiver<Arc<ReceivedTransaction<TYPES>>>,

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
        .filter(|(builder_state_id, (leaf_commit, _))| match leaf_commit {
            None => false,
            Some(leaf_commit) => {
                *leaf_commit == justify_qc.data.leaf_commit
                    && builder_state_id.parent_view == justify_qc.view_number
            }
        })
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
            self.parent_block_references.vid_commitment,
            self.parent_block_references.view_number.u64(),
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
            parent_commitment: self.parent_block_references.vid_commitment,
            parent_view: self.parent_block_references.view_number,
        })
    }

    /// [process_da_proposal] is a method that is used to handle incoming DA
    /// proposal messages from an incoming HotShot [Event]. A DA Proposals is
    /// a proposals that is meant to be voted on by consensus nodes in order to
    /// determine which transactions should be included for this view.
    ///
    /// A DA Proposal in conjunction with a Quorum Proposal is an indicator
    /// that a new Block / Leaf is being proposed for the HotShot network. So
    /// we need to be able to propose new Bundles on top of these proposals.
    ///
    /// In order to do so we must first wait until we have both a DA Proposal
    /// and a Quorum Proposal.  If we do not, then we can just record the
    /// proposal we have and wait for the other to arrive.
    ///
    /// If we do have a matching Quorum Proposal, then we can proceed to make
    /// a decision about extending the current [BuilderState] via
    /// [BuilderState::spawn_clone_that_extends_self].
    ///
    /// > Note: In the case of `process_da_proposal` if we don't have a corresponding
    /// > Quorum Proposal, then we will have to wait for `process_quorum_proposal`
    /// > to be called with the matching Quorum Proposal.  Until that point we
    /// > exit knowing we have fulfilled the DA proposal portion.
    #[tracing::instrument(skip_all, name = "process da proposal",
                                    fields(builder_parent_block_references = %self.parent_block_references))]
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

        let Entry::Vacant(e) = self
            .da_proposal_payload_commit_to_da_proposal
            .entry((da_msg.builder_commitment.clone(), da_msg.view_number))
        else {
            tracing::debug!("Payload commitment already exists in the da_proposal_payload_commit_to_da_proposal hashmap, so ignoring it");
            return;
        };

        // if we have matching da and quorum proposals, we can skip storing the one, and remove
        // the other from storage, and call build_block with both, to save a little space.
        let Entry::Occupied(quorum_proposal) = self
            .quorum_proposal_payload_commit_to_quorum_proposal
            .entry((da_msg.builder_commitment.clone(), da_msg.view_number))
        else {
            e.insert(da_msg);
            return;
        };

        let quorum_proposal = quorum_proposal.remove();

        if quorum_proposal.data.view_number != da_msg.view_number {
            tracing::debug!("Not spawning a clone despite matching DA and QC payload commitments, as they corresponds to different view numbers");
            return;
        }

        self.spawn_clone_that_extends_self(da_msg, quorum_proposal.clone())
            .await;
    }

    /// [process_quorum_proposal] is a method that is used to handle incoming
    /// Quorum Proposal messages from an incoming HotShot [Event]. A Quorum
    /// Proposal is a proposal that indicates the next potential Block of the
    /// chain is being proposed for the HotShot network.  This proposal is
    /// voted on by the consensus nodes in order to determine if whether this
    /// will be the next Block of the chain or not.
    ///
    /// A Quorum Proposal in conjunction with a DA Proposal is an indicator
    /// that a new Block / Leaf is being proposed for the HotShot network. So
    /// we need to be able to propose new Bundles on top of these proposals.
    ///
    /// In order to do so we must first wait until we have both a DA Proposal
    /// and a Quorum Proposal.  If we do not, then we can just record the
    /// proposal we have and wait for the other to arrive.
    ///
    /// If we do have a matching DA Proposal, then we can proceed to make
    /// a decision about extending the current [BuilderState] via
    /// [BuilderState::spawn_clone_that_extends_self].
    ///
    /// > Note: In the case of `process_quorum_proposal` if we don't have a
    /// > corresponding DA Proposal, then we will have to wait for
    /// > `process_da_proposal` to be called with the matching DA Proposal.
    /// > Until that point we exit knowing we have fulfilled the Quorum proposal
    /// > portion.
    //#[tracing::instrument(skip_all, name = "Process Quorum Proposal")]
    #[tracing::instrument(skip_all, name = "process quorum proposal",
                                    fields(builder_parent_block_references = %self.parent_block_references))]
    async fn process_quorum_proposal(&mut self, quorum_msg: QuorumProposalMessage<TYPES>) {
        tracing::debug!(
            "Builder Received Quorum proposal message for view {:?}",
            quorum_msg.proposal.data.view_number
        );

        // Two cases to handle:
        // Case 1: Bootstrapping phase
        // Case 2: No intended builder state exist
        let quorum_proposal = &quorum_msg.proposal;
        let view_number = quorum_proposal.data.view_number;
        let payload_builder_commitment = quorum_proposal.data.block_header.builder_commitment();

        tracing::debug!(
            "Extracted payload builder commitment from the quorum proposal: {:?}",
            payload_builder_commitment
        );

        // first check whether vid_commitment exists in the
        // quorum_proposal_payload_commit_to_quorum_proposal hashmap, if yes, ignore it, otherwise
        // validate it and later insert in

        let Entry::Vacant(e) = self
            .quorum_proposal_payload_commit_to_quorum_proposal
            .entry((payload_builder_commitment.clone(), view_number))
        else {
            tracing::debug!("Payload commitment already exists in the quorum_proposal_payload_commit_to_quorum_proposal hashmap, so ignoring it");
            return;
        };

        // if we have matching da and quorum proposals, we can skip storing
        // the one, and remove the other from storage, and call build_block
        // with both, to save a little space.
        let Entry::Occupied(da_proposal) = self
            .da_proposal_payload_commit_to_da_proposal
            .entry((payload_builder_commitment.clone(), view_number))
        else {
            e.insert(quorum_proposal.clone());
            return;
        };

        let da_proposal_info = da_proposal.remove();
        // remove the entry from the da_proposal_payload_commit_to_da_proposal hashmap
        self.da_proposal_payload_commit_to_da_proposal
            .remove(&(payload_builder_commitment.clone(), view_number));

        // also make sure we clone for the same view number
        // (check incase payload commitments are same)
        if da_proposal_info.view_number != view_number {
            tracing::debug!("Not spawning a clone despite matching DA and QC payload commitments, as they corresponds to different view numbers");
        }

        self.spawn_clone_that_extends_self(da_proposal_info, quorum_proposal.clone())
            .await;
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
                "{} is not the best fit for forking, {}@{}, so ignoring the Quorum proposal, and leaving it to another BuilderState",
                self.parent_block_references,
                quorum_proposal.data.block_header.payload_commitment(),
                quorum_proposal.data.view_number.u64(),
            );
            return;
        }

        let (req_sender, req_receiver) = broadcast(self.req_receiver.capacity());

        tracing::debug!(
            "extending BuilderState with a clone from {} with new proposal {}@{}",
            self.parent_block_references,
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
                                   fields(builder_parent_block_references = %self.parent_block_references))]
    async fn process_decide_event(&mut self, decide_msg: DecideMessage<TYPES>) -> Option<Status> {
        // Exit out all the builder states if their parent_block_references.view_number is less than the latest_decide_view_number
        // The only exception is that we want to keep the highest view number builder state active to ensure that
        // we have a builder state to handle the incoming DA and Quorum proposals
        let decide_view_number = decide_msg.latest_decide_view_number;

        let retained_view_cutoff = self
            .global_state
            .write_arc()
            .await
            .remove_handles(decide_view_number);
        if self.parent_block_references.view_number < retained_view_cutoff {
            tracing::info!(
                "Decide@{:?}; Task@{:?} exiting; views < {:?} being reclaimed",
                decide_view_number.u64(),
                self.parent_block_references.view_number.u64(),
                retained_view_cutoff.u64(),
            );
            return Some(Status::ShouldExit);
        }

        tracing::info!(
            "Decide@{:?}; Task@{:?} not exiting; views >= {:?} being retained",
            decide_view_number.u64(),
            self.parent_block_references.view_number.u64(),
            retained_view_cutoff.u64(),
        );

        Some(Status::ShouldContinue)
    }

    /// spawn a clone of the builder state
    #[tracing::instrument(skip_all, name = "spawn_clone",
                                    fields(builder_parent_block_references = %self.parent_block_references))]
    async fn spawn_clone(
        mut self,
        da_proposal_info: Arc<DaProposalMessage<TYPES>>,
        quorum_proposal: Arc<Proposal<TYPES, QuorumProposal<TYPES>>>,
        req_sender: BroadcastSender<MessageType<TYPES>>,
    ) {
        let leaf = Leaf::from_quorum_proposal(&quorum_proposal.data);

        // We replace our parent_block_references with information from the
        // quorum proposal.  This is identifying the block that this specific
        // instance of [BuilderState] is attempting to build for.
        self.parent_block_references = ParentBlockReferences {
            view_number: quorum_proposal.data.view_number,
            vid_commitment: quorum_proposal.data.block_header.payload_commitment(),
            leaf_commit: leaf.legacy_commit(),
            builder_commitment: quorum_proposal.data.block_header.builder_commitment(),
        };

        let builder_state_id = BuilderStateId {
            parent_commitment: self.parent_block_references.vid_commitment,
            parent_view: self.parent_block_references.view_number,
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
            self.txn_commits_in_queue.remove(tx);
        }

        // We add the included transactions to the included_txns set, so we can
        // also filter them should they be included in a future transaction
        // submission.
        self.included_txns
            .extend(da_proposal_info.txn_commitments.iter().cloned());

        // We wish to keep only the transactions in the tx_queue to those that
        // also exist in the txns_in_queue set.
        self.tx_queue
            .retain(|tx| self.txn_commits_in_queue.contains(&tx.commit));

        // register the spawned builder state to spawned_builder_states in the
        // global state We register this new child within the global_state, so
        // that it can be looked up via the [BuilderStateId] in the future.
        self.global_state.write_arc().await.register_builder_state(
            builder_state_id,
            self.parent_block_references.clone(),
            req_sender,
        );

        self.event_loop();
    }

    /// [build_block] is a method that will return a [BuildBlockInfo] if it is
    /// able to build a block.  If it encounters an error building a block, then
    /// it will return None.
    ///
    /// This first starts by collecting transactions to include in the block. It
    /// will wait until it has at least one transaction to include in the block,
    /// or up to the configured `maximize_txn_capture_timeout` duration elapses.
    /// At which point it will attempt to build a block with the transactions it
    /// has collected.
    ///
    /// Upon successfully building a block, a commitment for the [BuilderStateId]
    /// and Block payload pair are stored, and a [BuildBlockInfo] is created
    /// and returned.
    #[tracing::instrument(skip_all, name = "build block",
                                    fields(builder_parent_block_references = %self.parent_block_references))]
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

        let Ok((payload, metadata)) =
            <TYPES::BlockPayload as BlockPayload<TYPES>>::from_transactions(
                self.tx_queue.iter().map(|tx| tx.tx.clone()),
                &self.validated_state,
                &self.instance_state,
            )
            .await
        else {
            tracing::warn!("Failed to build block payload");
            return None;
        };

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
            self.parent_block_references.view_number,
            txn_count,
            builder_hash
        );

        Some(BuildBlockInfo {
            id: BlockId {
                view: self.parent_block_references.view_number,
                hash: builder_hash,
            },
            block_size,
            offered_fee,
            block_payload: payload,
            metadata,
        })
    }

    /// [process_block_request] is a method that is used to handle incoming
    /// [RequestMessage]s.  These [RequestMessage]s are looking for a bundle
    /// of transactions to be included in the next block. Instead of returning
    /// a value, this method's response will be provided to the [Sender] that
    /// is included in the [RequestMessage].
    ///
    /// At this point this particular [BuilderState] has already been deemed
    /// as the [BuilderState] that should handle this request, and it is up
    /// to this [BuilderState] to provide the response, if it is able to do
    /// so.
    ///
    /// The response will be a [ResponseMessage] that contains the transactions
    /// the `Builder` wants to include in the next block in addition to the
    /// expected block size, offered fee, and the
    /// Builder's commit block of the data being returned.
    async fn process_block_request(&mut self, req: RequestMessage<TYPES>) {
        let requested_view_number = req.requested_view_number;
        // If a spawned clone is active then it will handle the request, otherwise the highest view num builder will handle
        if requested_view_number != self.parent_block_references.view_number {
            tracing::debug!(
                "Builder {:?} Requested view number does not match the built_from_view, so ignoring it",
                self.parent_block_references.view_number
            );
            return;
        }

        tracing::info!(
            "Request handled by builder with view {}@{:?} for (view_num: {:?})",
            self.parent_block_references.vid_commitment,
            self.parent_block_references.view_number,
            requested_view_number
        );

        let response = self
            .build_block(BuilderStateId {
                parent_commitment: self.parent_block_references.vid_commitment,
                parent_view: requested_view_number,
            })
            .await;

        let Some(response) = response else {
            tracing::debug!("No response to send");
            return;
        };

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
                parent_commitment: self.parent_block_references.vid_commitment,
                parent_view: requested_view_number,
            },
            response,
            response_msg.clone(),
        );

        // ... and finally, send the response
        if let Err(e) = req.response_channel.send(response_msg).await {
            tracing::warn!(
                "Builder {:?} failed to send response to {:?} with builder hash {:?}, Err: {:?}",
                self.parent_block_references.view_number,
                req,
                builder_hash,
                e
            );
            return;
        }

        tracing::info!(
            "Builder {:?} Sent response to the request{:?} with builder hash {:?}",
            self.parent_block_references.view_number,
            req,
            builder_hash
        );
    }

    // MARK: event loop processing for [BuilderState]

    /// [event_loop_helper_handle_request] is a helper function that is used
    /// to handle incoming [MessageType]s, specifically [RequestMessage]s, that
    /// are received by the [BuilderState::req_receiver] channel.
    ///
    /// This method is used to process block requests.
    async fn event_loop_helper_handle_request(&mut self, req: Option<MessageType<TYPES>>) {
        tracing::debug!(
            "Received request msg in builder {:?}: {:?}",
            self.parent_block_references.view_number,
            req
        );

        let Some(req) = req else {
            tracing::warn!("No more request messages to consume");
            return;
        };

        let MessageType::RequestMessage(req) = req else {
            tracing::warn!("Unexpected message on requests channel: {:?}", req);
            return;
        };

        tracing::debug!(
            "Received request msg in builder {:?}: {:?}",
            self.parent_block_references.view_number,
            req
        );

        self.process_block_request(req).await;
    }

    /// [event_loop_helper_handle_da_proposal] is a helper function that is used
    /// to handle incoming [MessageType]s, specifically [DaProposalMessage]s,
    /// that are received by the [BuilderState::da_proposal_receiver] channel.
    async fn event_loop_helper_handle_da_proposal(&mut self, da: Option<MessageType<TYPES>>) {
        let Some(da) = da else {
            tracing::warn!("No more da proposal messages to consume");
            return;
        };

        let MessageType::DaProposalMessage(rda_msg) = da else {
            tracing::warn!("Unexpected message on da proposals channel: {:?}", da);
            return;
        };

        tracing::debug!(
            "Received da proposal msg in builder {:?}:\n {:?}",
            self.parent_block_references,
            rda_msg.view_number
        );

        self.process_da_proposal(rda_msg).await;
    }

    /// [event_loop_helper_handle_quorum_proposal] is a helper function that is used
    /// to handle incoming [MessageType]s, specifically [QuorumProposalMessage]s,
    /// that are received by the [BuilderState::quorum_proposal_receiver] channel.
    async fn event_loop_helper_handle_quorum_proposal(
        &mut self,
        quorum: Option<MessageType<TYPES>>,
    ) {
        let Some(quorum) = quorum else {
            tracing::warn!("No more quorum proposal messages to consume");
            return;
        };

        let MessageType::QuorumProposalMessage(quorum_proposal_message) = quorum else {
            tracing::warn!(
                "Unexpected message on quorum proposals channel: {:?}",
                quorum
            );
            return;
        };

        tracing::debug!(
            "Received quorum proposal msg in builder {:?}:\n {:?} for view ",
            self.parent_block_references,
            quorum_proposal_message.proposal.data.view_number
        );

        self.process_quorum_proposal(quorum_proposal_message).await;
    }

    /// [event_loop_helper_handle_decide] is a helper function that is used to
    /// handle incoming [MessageType]s, specifically [DecideMessage]s, that are
    /// received by the [BuilderState::decide_receiver] channel.
    ///
    /// This method can trigger the exit of the [BuilderState::event_loop] async
    /// task via the returned [std::ops::ControlFlow] type.  If the returned
    /// value is a [std::ops::ControlFlow::Break], then the
    /// [BuilderState::event_loop]
    /// async task should exit.
    async fn event_loop_helper_handle_decide(
        &mut self,
        decide: Option<MessageType<TYPES>>,
    ) -> std::ops::ControlFlow<()> {
        let Some(decide) = decide else {
            tracing::warn!("No more decide messages to consume");
            return std::ops::ControlFlow::Continue(());
        };

        let MessageType::DecideMessage(rdecide_msg) = decide else {
            tracing::warn!("Unexpected message on decide channel: {:?}", decide);
            return std::ops::ControlFlow::Continue(());
        };

        let latest_decide_view_num = rdecide_msg.latest_decide_view_number;
        tracing::debug!(
            "Received decide msg view {:?} in builder {:?}",
            &latest_decide_view_num,
            self.parent_block_references
        );
        let decide_status = self.process_decide_event(rdecide_msg).await;

        let Some(decide_status) = decide_status else {
            tracing::warn!(
                "decide_status was None; Continuing builder {:?}",
                self.parent_block_references
            );
            return std::ops::ControlFlow::Continue(());
        };

        match decide_status {
            Status::ShouldContinue => {
                tracing::debug!("Continuing builder {:?}", self.parent_block_references);
                std::ops::ControlFlow::Continue(())
            }
            _ => {
                tracing::info!(
                    "Exiting builder {:?} with decide view {:?}",
                    self.parent_block_references,
                    &latest_decide_view_num
                );
                std::ops::ControlFlow::Break(())
            }
        }
    }

    /// [event_loop] is a method that spawns an async task that attempts to
    /// handle messages being received across the [BuilderState]s various
    /// channels.
    ///
    /// This async task will continue to run until it receives a message that
    /// indicates that it should exit.  This exit message is sent via the
    /// [DecideMessage] channel.
    ///
    /// The main body of the loop listens to four channels at once, and when
    /// a message is received it will process the message with the appropriate
    /// handler accordingly.
    ///
    /// > Note: There is potential for improvement in typing here, as each of
    /// > these receivers returns the exact same type despite being separate
    /// > Channels.  These channels may want to convey separate types so that
    /// > the contained message can pertain to its specific channel
    /// > accordingly.
    #[tracing::instrument(skip_all, name = "event loop",
                                    fields(builder_parent_block_references = %self.parent_block_references))]
    pub fn event_loop(mut self) {
        let _builder_handle = async_spawn(async move {
            loop {
                tracing::debug!(
                    "Builder {:?} event loop",
                    self.parent_block_references.view_number
                );

                futures::select! {
                    req = self.req_receiver.next() => self.event_loop_helper_handle_request(req).await,
                    da = self.da_proposal_receiver.next() => self.event_loop_helper_handle_da_proposal(da).await,
                    quorum = self.quorum_proposal_receiver.next() => self.event_loop_helper_handle_quorum_proposal(quorum).await,
                    decide = self.decide_receiver.next() => if let std::ops::ControlFlow::Break(_) = self.event_loop_helper_handle_decide(decide).await { return; },
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
        parent_block_references: ParentBlockReferences<TYPES>,
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
            txn_commits_in_queue: txns_in_queue,
            parent_block_references,
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
            quorum_proposal_receiver: receivers.quorum_proposal.activate_cloned(),
            tx_receiver: receivers.transactions.activate_cloned(),
        }
    }
    pub fn clone_with_receiver(&self, req_receiver: BroadcastReceiver<MessageType<TYPES>>) -> Self {
        let mut included_txns = self.included_txns.clone();
        included_txns.rotate();

        BuilderState {
            included_txns,
            txn_commits_in_queue: self.txn_commits_in_queue.clone(),
            parent_block_references: self.parent_block_references.clone(),
            decide_receiver: self.decide_receiver.clone(),
            da_proposal_receiver: self.da_proposal_receiver.clone(),
            quorum_proposal_receiver: self.quorum_proposal_receiver.clone(),
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
                    if self.included_txns.contains(&tx.commit) {
                        // We've included this transaction in one of our
                        // recent blocks, and we do not wish to include it
                        // again.
                        continue;
                    }

                    if self.txn_commits_in_queue.contains(&tx.commit) {
                        // We already have this transaction in our current
                        // queue, so we do not want to include it again
                        continue;
                    }

                    self.txn_commits_in_queue.insert(tx.commit);
                    self.tx_queue.push(tx);
                }

                Err(async_broadcast::TryRecvError::Empty)
                | Err(async_broadcast::TryRecvError::Closed) => {
                    // The transaction receiver is empty, or it's been closed.
                    // If it's closed that's a big problem and we should
                    // probably indicate it as such.
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
    use std::sync::Arc;

    use async_broadcast::broadcast;
    use committable::RawCommitmentBuilder;
    use hotshot_example_types::block_types::TestTransaction;
    use hotshot_types::data::ViewNumber;
    use hotshot_types::data::{Leaf, QuorumProposal};
    use hotshot_types::message::Proposal;
    use hotshot_types::traits::node_implementation::{ConsensusTime, NodeType};
    use hotshot_types::utils::BuilderCommitment;

    use super::DaProposalMessage;
    use super::MessageType;
    use super::ParentBlockReferences;
    use crate::testing::{calc_proposal_msg, start_builder_state_without_event_loop, TestTypes};

    /// check whether the da_proposal_payload_commit_to_da_proposal has correct (key, value) pair after processing da proposal messages
    /// used for testing only
    fn check_equal_da_proposal_hashmap<TYPES: NodeType>(
        da_proposal_payload_commit_to_da_proposal: HashMap<
            (BuilderCommitment, <TYPES>::Time),
            Arc<DaProposalMessage<TYPES>>,
        >,
        correct_da_proposal_payload_commit_to_da_proposal: HashMap<
            (BuilderCommitment, <TYPES>::Time),
            Arc<DaProposalMessage<TYPES>>,
        >,
    ) {
        let deserialized_map: HashMap<_, _> = da_proposal_payload_commit_to_da_proposal.clone();
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
    }

    /// check whether the quorum_proposal_payload_commit_to_da_proposal has correct (key, value) pair after processing quorum proposal messages
    /// used for testing only
    type QuorumProposalMap<TYPES> = HashMap<
        (BuilderCommitment, <TYPES as NodeType>::Time),
        Arc<Proposal<TYPES, QuorumProposal<TYPES>>>,
    >;
    fn check_equal_quorum_proposal_hashmap<TYPES: NodeType>(
        quorum_proposal_payload_commit_to_da_proposal: QuorumProposalMap<TYPES>,
        correct_quorum_proposal_payload_commit_to_da_proposal: QuorumProposalMap<TYPES>,
    ) {
        let deserialized_map: HashMap<_, _> = quorum_proposal_payload_commit_to_da_proposal.clone();
        for (key, value) in deserialized_map.iter() {
            let correct_value = correct_quorum_proposal_payload_commit_to_da_proposal.get(key);
            assert_eq!(
                value.as_ref().clone(),
                rkyv::option::ArchivedOption::Some(correct_value)
                    .unwrap()
                    .unwrap()
                    .as_ref()
                    .clone()
            );
        }
    }

    /// This test the function `process_da_propsal`.
    /// It checkes da_proposal_payload_commit_to_da_proposal change appropriately
    /// when receiving a da proposal message.
    /// This test also checks whether corresponding BuilderStateId is in global_state.
    #[async_std::test]
    async fn test_process_da_proposal() {
        async_compatibility_layer::logging::setup_logging();
        async_compatibility_layer::logging::setup_backtrace();
        tracing::info!("Testing the function `process_da_proposal` in `builder_state.rs`");

        // Number of views to simulate
        const NUM_ROUNDS: usize = 5;
        // Capacity of broadcast channels
        const CHANNEL_CAPACITY: usize = NUM_ROUNDS * 5;
        // Number of nodes on DA committee
        const NUM_STORAGE_NODES: usize = 4;

        // start builder_state without entering event loop
        let (_senders, global_state, mut builder_state) =
            start_builder_state_without_event_loop(CHANNEL_CAPACITY, NUM_STORAGE_NODES).await;

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
            panic!("Not a da_proposal_message in correct format");
        }
        check_equal_da_proposal_hashmap(
            builder_state
                .da_proposal_payload_commit_to_da_proposal
                .clone(),
            correct_da_proposal_payload_commit_to_da_proposal.clone(),
        );
        // check global_state didn't change
        if let Some(_x) = global_state
            .read_arc()
            .await
            .spawned_builder_states
            .get(&builder_state_id)
        {
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
            panic!("Not a da_proposal_message in correct format");
        }
        check_equal_da_proposal_hashmap(
            builder_state
                .da_proposal_payload_commit_to_da_proposal
                .clone(),
            correct_da_proposal_payload_commit_to_da_proposal.clone(),
        );
        // check global_state didn't change
        if let Some(_x) = global_state
            .read_arc()
            .await
            .spawned_builder_states
            .get(&builder_state_id_1)
        {
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
        if let MessageType::QuorumProposalMessage(practice_quorum_msg_2) =
            quorum_proposal_msg_2.clone()
        {
            builder_state
                .process_quorum_proposal(practice_quorum_msg_2.clone())
                .await;
        } else {
            panic!("Not a quorum_proposal_message in correct format");
        }

        // process da proposal message and do the check
        if let MessageType::DaProposalMessage(practice_da_msg_2) = da_proposal_msg_2.clone() {
            builder_state
                .process_da_proposal(practice_da_msg_2.clone())
                .await;
        } else {
            panic!("Not a da_proposal_message in correct format");
        }
        check_equal_da_proposal_hashmap(
            builder_state.da_proposal_payload_commit_to_da_proposal,
            correct_da_proposal_payload_commit_to_da_proposal,
        );
        // check global_state has this new builder_state_id
        if let Some(_x) = global_state
            .read_arc()
            .await
            .spawned_builder_states
            .get(&builder_state_id_2)
        {
            tracing::debug!("global_state updated successfully");
        } else {
            panic!("global_state should have cooresponding builder_state_id as now we have matching quorum proposal.");
        }
    }

    /// This test the function `process_quorum_propsal`.
    /// It checkes quorum_proposal_payload_commit_to_quorum_proposal change appropriately
    /// when receiving a quorum proposal message.
    /// This test also checks whether corresponding BuilderStateId is in global_state.
    #[async_std::test]
    async fn test_process_quorum_proposal() {
        async_compatibility_layer::logging::setup_logging();
        async_compatibility_layer::logging::setup_backtrace();
        tracing::info!("Testing the function `process_quorum_proposal` in `builder_state.rs`");

        // Number of views to simulate
        const NUM_ROUNDS: usize = 5;
        // Capacity of broadcast channels
        const CHANNEL_CAPACITY: usize = NUM_ROUNDS * 5;
        // Number of nodes on DA committee
        const NUM_STORAGE_NODES: usize = 4;

        // start builder_state without entering event loop
        let (_senders, global_state, mut builder_state) =
            start_builder_state_without_event_loop(CHANNEL_CAPACITY, NUM_STORAGE_NODES).await;

        // randomly generate a transaction
        let transactions = vec![TestTransaction::new(vec![1, 2, 3]); 3];
        let (_quorum_proposal, quorum_proposal_msg, _da_proposal_msg, builder_state_id) =
            calc_proposal_msg(NUM_STORAGE_NODES, 0, None, transactions.clone()).await;

        // sub-test one
        // call process_quorum_proposal without matching da proposal message
        // quorum_proposal_payload_commit_to_quorum_proposal should insert the message
        let mut correct_quorum_proposal_payload_commit_to_quorum_proposal = HashMap::new();
        if let MessageType::QuorumProposalMessage(practice_quorum_msg) = quorum_proposal_msg.clone()
        {
            builder_state
                .process_quorum_proposal(practice_quorum_msg.clone())
                .await;
            correct_quorum_proposal_payload_commit_to_quorum_proposal.insert(
                (
                    practice_quorum_msg
                        .proposal
                        .data
                        .block_header
                        .builder_commitment
                        .clone(),
                    practice_quorum_msg.proposal.data.view_number,
                ),
                practice_quorum_msg.proposal,
            );
        } else {
            panic!("Not a quorum_proposal_message in correct format");
        }
        check_equal_quorum_proposal_hashmap(
            builder_state
                .quorum_proposal_payload_commit_to_quorum_proposal
                .clone(),
            correct_quorum_proposal_payload_commit_to_quorum_proposal.clone(),
        );
        // check global_state didn't change
        if let Some(_x) = global_state
            .read_arc()
            .await
            .spawned_builder_states
            .get(&builder_state_id)
        {
            panic!("global_state shouldn't have cooresponding builder_state_id without matching quorum proposal.");
        }

        // sub-test two
        // add the matching da proposal message with different tx
        // and call process_da_proposal with this matching quorum proposal message and quorum da message
        // we should spawn_clone here
        // and check whether global_state has correct BuilderStateId
        let transactions_2 = vec![TestTransaction::new(vec![2, 3, 4]); 2];
        let (_quorum_proposal_2, quorum_proposal_msg_2, da_proposal_msg_2, builder_state_id_2) =
            calc_proposal_msg(NUM_STORAGE_NODES, 0, None, transactions_2).await;

        // process da proposal message first, so that later when process_da_proposal we can directly call `build_block` and skip storage
        if let MessageType::DaProposalMessage(practice_da_msg_2) = da_proposal_msg_2.clone() {
            builder_state
                .process_da_proposal(practice_da_msg_2.clone())
                .await;
        } else {
            panic!("Not a da_proposal_message in correct format");
        }

        // process quorum proposal, and do the check
        if let MessageType::QuorumProposalMessage(practice_quorum_msg_2) =
            quorum_proposal_msg_2.clone()
        {
            builder_state
                .process_quorum_proposal(practice_quorum_msg_2.clone())
                .await;
        } else {
            panic!("Not a quorum_proposal_message in correct format");
        }

        check_equal_quorum_proposal_hashmap(
            builder_state.quorum_proposal_payload_commit_to_quorum_proposal,
            correct_quorum_proposal_payload_commit_to_quorum_proposal,
        );
        // check global_state has this new builder_state_id
        if let Some(_x) = global_state
            .read_arc()
            .await
            .spawned_builder_states
            .get(&builder_state_id_2)
        {
            tracing::debug!("global_state updated successfully");
        } else {
            panic!("global_state should have cooresponding builder_state_id as now we have matching da proposal.");
        }
    }

    /// This test the function `process_decide_event`.
    /// It checkes whether we exit out correct builder states when there's a decide event coming in.
    /// This test also checks whether corresponding BuilderStateId is removed in global_state.
    #[async_std::test]
    async fn test_process_decide_event() {
        async_compatibility_layer::logging::setup_logging();
        async_compatibility_layer::logging::setup_backtrace();
        tracing::info!("Testing the builder core with multiple messages from the channels");

        // Number of views to simulate
        const NUM_ROUNDS: usize = 5;
        // Number of transactions to submit per round
        const NUM_TXNS_PER_ROUND: usize = 4;
        // Capacity of broadcast channels
        const CHANNEL_CAPACITY: usize = NUM_ROUNDS * 5;
        // Number of nodes on DA committee
        const NUM_STORAGE_NODES: usize = 4;

        // start builder_state without entering event loop
        let (_senders, global_state, mut builder_state) =
            start_builder_state_without_event_loop(CHANNEL_CAPACITY, NUM_STORAGE_NODES).await;

        // insert some builder states

        // Transactions to send
        let all_transactions = (0..NUM_ROUNDS)
            .map(|round| {
                (0..NUM_TXNS_PER_ROUND)
                    .map(|tx_num| TestTransaction::new(vec![round as u8, tx_num as u8]))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut prev_quorum_proposal: Option<QuorumProposal<TestTypes>> = None;
        #[allow(clippy::needless_range_loop)]
        for round in 0..NUM_ROUNDS {
            let transactions = all_transactions[round].clone();
            let (quorum_proposal, _quorum_proposal_msg, _da_proposal_msg, builder_state_id) =
                calc_proposal_msg(NUM_STORAGE_NODES, round, prev_quorum_proposal, transactions)
                    .await;
            prev_quorum_proposal = Some(quorum_proposal.clone());
            let (req_sender, _req_receiver) = broadcast(CHANNEL_CAPACITY);
            let leaf: Leaf<TestTypes> = Leaf::from_quorum_proposal(&quorum_proposal);
            let leaf_commit = RawCommitmentBuilder::new("leaf commitment")
                .u64_field("view number", leaf.view_number().u64())
                .u64_field("block number", leaf.height())
                .field("parent Leaf commitment", leaf.parent_commitment())
                .var_size_field(
                    "block payload commitment",
                    leaf.payload_commitment().as_ref(),
                )
                .finalize();
            global_state.write_arc().await.register_builder_state(
                builder_state_id,
                ParentBlockReferences {
                    view_number: quorum_proposal.view_number,
                    vid_commitment: quorum_proposal.block_header.payload_commitment,
                    leaf_commit,
                    builder_commitment: quorum_proposal.block_header.builder_commitment,
                },
                req_sender,
            );
        }

        // send out a decide event
        // randomly choose a latest_decide_view_number between [0, NUM_ROUNDS]
        let latest_decide_view_number = ViewNumber::new(3);

        let decide_message = MessageType::DecideMessage(crate::builder_state::DecideMessage {
            latest_decide_view_number,
        });
        if let MessageType::DecideMessage(practice_decide_msg) = decide_message.clone() {
            builder_state
                .process_decide_event(practice_decide_msg.clone())
                .await;
        } else {
            panic!("Not a decide_message in correct format");
        }
        // check whether spawned_builder_states have correct builder_state_id and already exit-ed builder_states older than decides
        let current_spawned_builder_states =
            global_state.read_arc().await.spawned_builder_states.clone();
        current_spawned_builder_states
            .iter()
            .for_each(|(builder_state_id, _)| {
                assert!(builder_state_id.parent_view >= latest_decide_view_number)
            });
    }
}
