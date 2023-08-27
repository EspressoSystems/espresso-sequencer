//! Contains the [`SequencingReplica`] struct used for the replica step in the consensus algorithm with DA
//! committee, i.e. in the sequencing consensus.

use crate::{
    utils::{Terminator, View, ViewInner},
    Consensus, SequencingConsensusApi,
};
use async_compatibility_layer::channel::UnboundedReceiver;
use async_lock::{Mutex, RwLock, RwLockUpgradableReadGuard, RwLockWriteGuard};
use bincode::Options;
use commit::Committable;
use either::{Left, Right};
use hotshot_types::{
    certificate::{DACertificate, QuorumCertificate},
    data::{LeafType, QuorumProposal, SequencingLeaf},
    message::{
        ConsensusMessageType, InternalTrigger, Message, ProcessedCommitteeConsensusMessage,
        ProcessedGeneralConsensusMessage, ProcessedSequencingMessage, SequencingMessage,
    },
    traits::{
        election::{ConsensusExchange, QuorumExchangeType, SignedCertificate},
        node_implementation::{
            CommitteeEx, NodeImplementation, NodeType, QuorumProposalType, QuorumVoteType,
            SequencingQuorumEx,
        },
        signature_key::SignatureKey,
        state::ConsensusTime,
        Block,
    },
};
use hotshot_utils::bincode::bincode_opts;
use std::{
    collections::HashSet,
    marker::PhantomData,
    ops::Bound::{Excluded, Included},
    sync::Arc,
};
use tracing::{error, info, instrument, warn};
/// This view's replica for sequencing consensus.
#[derive(Debug, Clone)]
pub struct SequencingReplica<
    A: SequencingConsensusApi<TYPES, SequencingLeaf<TYPES>, I>,
    TYPES: NodeType,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
> {
    /// ID of node.
    pub id: u64,
    /// Reference to consensus. The replica will require a write lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES, SequencingLeaf<TYPES>>>>,
    /// Channel for accepting leader proposals and timeouts messages.
    #[allow(clippy::type_complexity)]
    pub proposal_collection_chan:
        Arc<Mutex<UnboundedReceiver<ProcessedSequencingMessage<TYPES, I>>>>,
    /// View number this view is executing in.
    pub cur_view: TYPES::Time,
    /// The High QC.
    pub high_qc: QuorumCertificate<TYPES, SequencingLeaf<TYPES>>,
    /// HotShot consensus API.
    pub api: A,

    /// the committee exchange
    pub committee_exchange: Arc<CommitteeEx<TYPES, I>>,

    /// the quorum exchange
    pub quorum_exchange: Arc<SequencingQuorumEx<TYPES, I>>,

    /// needed to typecheck
    pub _pd: PhantomData<I>,
}

impl<
        A: SequencingConsensusApi<TYPES, SequencingLeaf<TYPES>, I>,
        TYPES: NodeType,
        I: NodeImplementation<
            TYPES,
            Leaf = SequencingLeaf<TYPES>,
            ConsensusMessage = SequencingMessage<TYPES, I>,
        >,
    > SequencingReplica<A, TYPES, I>
where
    SequencingQuorumEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, SequencingLeaf<TYPES>>,
        Certificate = QuorumCertificate<TYPES, SequencingLeaf<TYPES>>,
        Commitment = SequencingLeaf<TYPES>,
    >,
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = TYPES::BlockType,
    >,
{
    /// The leaf from the genesis view.
    ///
    /// This will be used as the parent leaf for the proposal in the first view after genesis.
    async fn genesis_leaf(&self) -> Option<SequencingLeaf<TYPES>> {
        let consensus = self.consensus.read().await;
        let Some(genesis_view) = consensus.state_map.get(&TYPES::Time::genesis()) else {
            warn!("Couldn't find genesis view in state map.");
            return None;
        };
        let Some(leaf) = genesis_view.get_leaf_commitment() else {
            warn!(
                ?genesis_view,
                "Genesis view points to a view without a leaf"
            );
            return None;
        };
        let Some(leaf) = consensus.saved_leaves.get(&leaf) else {
            warn!("Failed to find genesis leaf.");
            return None;
        };
        Some(leaf.clone())
    }

    /// Replica task for sequencing consensus that spins until a vote can be made or timeout is
    /// hit.
    ///
    /// Returns the new leaf if it's valid.
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Sequencing Replica Task", level = "error")]
    #[allow(clippy::type_complexity)]
    async fn find_valid_msg<'a>(
        &self,
        view_leader_key: TYPES::SignatureKey,
        consensus: RwLockUpgradableReadGuard<'a, Consensus<TYPES, SequencingLeaf<TYPES>>>,
    ) -> (
        RwLockUpgradableReadGuard<'a, Consensus<TYPES, SequencingLeaf<TYPES>>>,
        Option<SequencingLeaf<TYPES>>,
    ) {
        let lock = self.proposal_collection_chan.lock().await;
        let mut invalid_qc = false;
        let leaf = loop {
            let msg = lock.recv().await;
            info!("recv-ed message {:?}", msg.clone());
            if let Ok(msg) = msg {
                // stale/newer view messages should never reach this specific task's receive channel
                if Into::<SequencingMessage<_, _>>::into(msg.clone()).view_number() != self.cur_view
                {
                    continue;
                }
                match msg {
                    Left(general_message) => {
                        match general_message {
                            ProcessedGeneralConsensusMessage::Proposal(p, sender) => {
                                if view_leader_key != sender {
                                    continue;
                                }

                                let mut valid_leaf = None;
                                let vote_token =
                                    self.quorum_exchange.make_vote_token(self.cur_view);
                                match vote_token {
                                    Err(e) => {
                                        error!(
                                            "Failed to generate vote token for {:?} {:?}",
                                            self.cur_view, e
                                        );
                                    }
                                    Ok(None) => {
                                        info!(
                                            "We were not chosen for consensus committee on {:?}",
                                            self.cur_view
                                        );
                                    }
                                    Ok(Some(vote_token)) => {
                                        info!(
                                            "We were chosen for consensus committee on {:?}",
                                            self.cur_view
                                        );

                                        let message;

                                        // Construct the leaf.
                                        let justify_qc = p.data.justify_qc;
                                        let parent = if justify_qc.is_genesis() {
                                            self.genesis_leaf().await
                                        } else {
                                            consensus
                                                .saved_leaves
                                                .get(&justify_qc.leaf_commitment())
                                                .cloned()
                                        };
                                        let Some(parent) = parent else {
                                            warn!("Proposal's parent missing from storage");
                                            continue;
                                        };
                                        let parent_commitment = parent.commit();
                                        let block_commitment = p.data.block_commitment;
                                        let leaf = SequencingLeaf {
                                            view_number: self.cur_view,
                                            height: p.data.height,
                                            justify_qc: justify_qc.clone(),
                                            parent_commitment,
                                            deltas: Right(p.data.block_commitment),
                                            rejected: Vec::new(),
                                            timestamp: time::OffsetDateTime::now_utc()
                                                .unix_timestamp_nanos(),
                                            proposer_id: sender.to_bytes(),
                                        };
                                        let justify_qc_commitment = justify_qc.commit();
                                        let leaf_commitment = leaf.commit();
                                        let Some(dac) = p.data.dac else {
                                            warn!("No DAC in proposal! Skipping proposal.");
                                            continue;
                                        };

                                        // Validate the `justify_qc`.
                                        if !self
                                            .quorum_exchange
                                            .is_valid_cert(&justify_qc, parent_commitment)
                                        {
                                            invalid_qc = true;
                                            warn!("Invalid justify_qc in proposal!.");
                                            message = self.quorum_exchange.create_no_message(
                                                justify_qc_commitment,
                                                leaf_commitment,
                                                self.cur_view,
                                                vote_token,
                                            );
                                        }
                                        // Validate the `height`.
                                        else if leaf.height != parent.height + 1 {
                                            invalid_qc = true;
                                            warn!(
                                                "Incorrect height in proposal (expected {}, got {})",
                                                parent.height + 1,
                                                leaf.height
                                            );
                                            message = self.quorum_exchange.create_no_message(
                                                justify_qc_commitment,
                                                leaf_commitment,
                                                self.cur_view,
                                                vote_token,
                                            );
                                        }
                                        // Validate the DAC.
                                        else if !self
                                            .committee_exchange
                                            .is_valid_cert(&dac, block_commitment)
                                        {
                                            warn!("Invalid DAC in proposal! Skipping proposal.");
                                            message = self.quorum_exchange.create_no_message(
                                                justify_qc_commitment,
                                                leaf_commitment,
                                                self.cur_view,
                                                vote_token,
                                            );
                                        }
                                        // Validate the signature.
                                        else if !view_leader_key
                                            .validate(&p.signature, leaf_commitment.as_ref())
                                        {
                                            warn!(?p.signature, "Could not verify proposal.");
                                            message = self.quorum_exchange.create_no_message(
                                                justify_qc_commitment,
                                                leaf_commitment,
                                                self.cur_view,
                                                vote_token,
                                            );
                                        }
                                        // Create a positive vote if either liveness or safety check
                                        // passes.
                                        else {
                                            // Liveness check.
                                            let liveness_check =
                                                justify_qc.view_number > consensus.locked_view;

                                            // Safety check.
                                            // Check if proposal extends from the locked leaf.
                                            let outcome = consensus.visit_leaf_ancestors(
                                                justify_qc.view_number,
                                                Terminator::Inclusive(consensus.locked_view),
                                                false,
                                                |leaf| {
                                                    // if leaf view no == locked view no then we're done, report success by
                                                    // returning true
                                                    leaf.view_number != consensus.locked_view
                                                },
                                            );
                                            let safety_check = outcome.is_ok();
                                            if let Err(e) = outcome {
                                                self.api
                                                    .send_view_error(self.cur_view, Arc::new(e))
                                                    .await;
                                            }

                                            // Skip if both saftey and liveness checks fail.
                                            if !safety_check && !liveness_check {
                                                warn!("Failed safety check and liveness check");
                                                message = self.quorum_exchange.create_no_message(
                                                    justify_qc_commitment,
                                                    leaf_commitment,
                                                    self.cur_view,
                                                    vote_token,
                                                );
                                            } else {
                                                // A valid leaf is found.
                                                valid_leaf = Some(leaf);

                                                // Generate a message with yes vote.
                                                message = self.quorum_exchange.create_yes_message(
                                                    justify_qc_commitment,
                                                    leaf_commitment,
                                                    self.cur_view,
                                                    vote_token,
                                                );
                                            }
                                        }

                                        info!("Sending vote to next leader {:?}", message);
                                        let next_leader =
                                            self.quorum_exchange.get_leader(self.cur_view + 1);
                                        if self
                                            .api
                                            .send_direct_message::<QuorumProposalType<TYPES, I>, QuorumVoteType<TYPES, I>>(next_leader, SequencingMessage(Left(message)))
                                            .await
                                            .is_err()
                                        {
                                            consensus.metrics.failed_to_send_messages.add(1);
                                            warn!("Failed to send vote to next leader");
                                        } else {
                                            consensus.metrics.outgoing_direct_messages.add(1);
                                        }
                                    }
                                }
                                break valid_leaf;
                            }
                            ProcessedGeneralConsensusMessage::InternalTrigger(trigger) => {
                                match trigger {
                                    InternalTrigger::Timeout(_) => {
                                        let next_leader =
                                            self.quorum_exchange.get_leader(self.cur_view + 1);

                                        consensus.metrics.number_of_timeouts.add(1);

                                        let vote_token =
                                            self.quorum_exchange.make_vote_token(self.cur_view);

                                        match vote_token {
                                            Err(e) => {
                                                error!(
                                                    "Failed to generate vote token for {:?} {:?}",
                                                    self.cur_view, e
                                                );
                                            }
                                            Ok(None) => {
                                                info!(
                                                    "We were not chosen for committee on {:?}",
                                                    self.cur_view
                                                );
                                            }
                                            Ok(Some(vote_token)) => {
                                                let timed_out_msg =
                                                    self.quorum_exchange.create_timeout_message(
                                                        self.high_qc.clone(),
                                                        self.cur_view,
                                                        vote_token,
                                                    );
                                                warn!(
                                                    "Timed out! Sending timeout to next leader {:?}",
                                                    timed_out_msg
                                                );

                                                // send timedout message to the next leader
                                                if let Err(e) = self
                                                    .api
                                                    .send_direct_message::<QuorumProposalType<
                                                        TYPES,
                                                        I,
                                                    >, QuorumVoteType<
                                                        TYPES,
                                                        I,
                                                    >>(
                                                        next_leader.clone(),
                                                        SequencingMessage(Left(timed_out_msg)),
                                                    )
                                                    .await
                                                {
                                                    consensus
                                                        .metrics
                                                        .failed_to_send_messages
                                                        .add(1);
                                                    warn!(
                                                        ?next_leader,
                                                        ?e,
                                                        "Could not send time out message to next_leader"
                                                    );
                                                } else {
                                                    consensus
                                                        .metrics
                                                        .outgoing_direct_messages
                                                        .add(1);
                                                }

                                                // exits from entire function
                                                self.api.send_replica_timeout(self.cur_view).await;
                                            }
                                        }
                                        return (consensus, None);
                                    }
                                }
                            }
                            ProcessedGeneralConsensusMessage::Vote(_, _) => {
                                // should only be for leader, never replica
                                warn!("Replica receieved a vote message. This is not what the replica expects. Skipping.");
                                continue;
                            }
                            ProcessedGeneralConsensusMessage::ViewSyncCertificate(_) => todo!(),
                            ProcessedGeneralConsensusMessage::ViewSyncVote(_) => todo!(),
                        }
                    }
                    Right(committee_message) => {
                        match committee_message {
                            ProcessedCommitteeConsensusMessage::DAProposal(_p, _sender) => {
                                warn!("Replica receieved a DA Proposal. This is not what the replica expects. Skipping.");
                                // TODO (Keyao) why not continue here?
                            }
                            ProcessedCommitteeConsensusMessage::DAVote(_, _) => {
                                // should only be for leader, never replica
                                warn!("Replica receieved a vote message. This is not what the replica expects. Skipping.");
                                continue;
                            }
                            ProcessedCommitteeConsensusMessage::DACertificate(_, _) => {
                                continue;
                            }
                        }
                    }
                }
            }
            // fall through logic if we did not receive successfully from channel
            warn!("Replica did not receive successfully from channel. Terminating Replica.");
            return (consensus, None);
        };
        let mut consensus = RwLockUpgradableReadGuard::upgrade(consensus).await;
        if invalid_qc {
            consensus.invalid_qc += 1;
        }
        (RwLockWriteGuard::downgrade_to_upgradable(consensus), leaf)
    }

    /// Run one view of the replica for sequencing consensus.
    #[instrument(skip(self), fields(id = self.id, view = *self.cur_view), name = "Sequencing Replica Task", level = "error")]
    pub async fn run_view(self) -> QuorumCertificate<TYPES, SequencingLeaf<TYPES>> {
        info!("Sequencing replica task started!");
        let view_leader_key = self.quorum_exchange.get_leader(self.cur_view);
        let consensus = self.consensus.upgradable_read().await;

        let (consensus, maybe_leaf) = self.find_valid_msg(view_leader_key, consensus).await;

        let Some(leaf) = maybe_leaf else {
            // We either timed out or for some reason could not vote on a proposal.
            return self.high_qc;
        };

        let mut new_anchor_view = consensus.last_decided_view;
        let mut new_locked_view = consensus.locked_view;
        let mut last_view_number_visited = self.cur_view;
        let mut new_commit_reached: bool = false;
        let mut new_decide_reached = false;
        let mut new_decide_qc = None;
        let mut leaf_views = Vec::new();
        let mut included_txns = HashSet::new();
        let old_anchor_view = consensus.last_decided_view;
        let parent_view = leaf.justify_qc.view_number;
        let mut current_chain_length = 0usize;
        if parent_view + 1 == self.cur_view {
            current_chain_length += 1;
            if let Err(e) = consensus.visit_leaf_ancestors(
                parent_view,
                Terminator::Exclusive(old_anchor_view),
                true,
                |leaf| {
                    if !new_decide_reached {
                        if last_view_number_visited == leaf.view_number + 1 {
                            last_view_number_visited = leaf.view_number;
                            current_chain_length += 1;
                            if current_chain_length == 2 {
                                new_locked_view = leaf.view_number;
                                new_commit_reached = true;
                                // The next leaf in the chain, if there is one, is decided, so this
                                // leaf's justify_qc would become the QC for the decided chain.
                                new_decide_qc = Some(leaf.justify_qc.clone());
                            } else if current_chain_length == 3 {
                                new_anchor_view = leaf.view_number;
                                new_decide_reached = true;
                            }
                        } else {
                            // nothing more to do here... we don't have a new chain extension
                            return false;
                        }
                    }
                    // starting from the first iteration with a three chain, e.g. right after the else if case nested in the if case above
                    if new_decide_reached {
                        let mut leaf = leaf.clone();

                        // If the full block is available for this leaf, include it in the leaf
                        // chain that we send to the client.
                        if let Some(block) =
                            consensus.saved_blocks.get(leaf.get_deltas_commitment())
                        {
                            if let Err(err) = leaf.fill_deltas(block.clone()) {
                                warn!("unable to fill leaf {} with block {}, block will not be available: {}",
                                    leaf.commit(), block.commit(), err);
                            }
                        }

                        leaf_views.push(leaf.clone());
                        if let Left(block) = &leaf.deltas {
                            let txns = block.contained_transactions();
                            for txn in txns {
                                included_txns.insert(txn);
                            }
                        }
                    }
                    true
                },
            ) {
                self.api.send_view_error(self.cur_view, Arc::new(e)).await;
            }
        }

        let included_txns_set: HashSet<_> = if new_decide_reached {
            included_txns
        } else {
            HashSet::new()
        };

        // promote lock here
        let mut consensus = RwLockUpgradableReadGuard::upgrade(consensus).await;
        consensus.state_map.insert(
            self.cur_view,
            View {
                view_inner: ViewInner::Leaf {
                    leaf: leaf.commit(),
                },
            },
        );

        consensus.metrics.number_of_views_since_last_commit.set(
            consensus
                .state_map
                .range((
                    Excluded(consensus.last_decided_view),
                    Included(self.cur_view),
                ))
                .count(),
        );

        consensus.saved_leaves.insert(leaf.commit(), leaf.clone());
        if new_commit_reached {
            consensus.locked_view = new_locked_view;
        }
        #[allow(clippy::cast_precision_loss)]
        if new_decide_reached {
            let num_views_since_last_anchor =
                (*self.cur_view - *consensus.last_decided_view) as f64;
            let views_seen = consensus
                .state_map
                .range((
                    Excluded(consensus.last_decided_view),
                    Included(self.cur_view),
                ))
                .count();
            // A count of all veiws we saw that aren't in the current chain (so won't be commited)
            consensus
                .metrics
                .discarded_views_per_decide_event
                .add_point((views_seen - current_chain_length) as f64);
            // An empty view is one we didn't see a leaf for but we moved past that view number
            consensus
                .metrics
                .empty_views_per_decide_event
                .add_point(num_views_since_last_anchor - views_seen as f64);
            consensus
                .metrics
                .number_of_views_per_decide_event
                .add_point(num_views_since_last_anchor);
            consensus
                .metrics
                .invalid_qc_views
                .add_point(consensus.invalid_qc as f64);

            let mut included_txn_size = 0;
            consensus
                .transactions
                .modify(|txns| {
                    *txns = txns
                        .drain()
                        .filter(|(txn_hash, txn)| {
                            if included_txns_set.contains(txn_hash) {
                                included_txn_size +=
                                    bincode_opts().serialized_size(txn).unwrap_or_default();
                                false
                            } else {
                                true
                            }
                        })
                        .collect();
                })
                .await;
            consensus
                .metrics
                .outstanding_transactions
                .update(-(included_txns_set.len() as i64));
            consensus
                .metrics
                .outstanding_transactions_memory_size
                .update(-(i64::try_from(included_txn_size).unwrap_or(i64::MAX)));

            consensus
                .metrics
                .rejected_transactions
                .add(leaf.rejected.len());

            let decide_sent = self.api.send_decide(
                consensus.last_decided_view,
                leaf_views,
                new_decide_qc.unwrap(),
            );
            let old_anchor_view = consensus.last_decided_view;
            consensus
                .collect_garbage(old_anchor_view, new_anchor_view)
                .await;
            consensus.last_decided_view = new_anchor_view;
            consensus.invalid_qc = 0;

            // We're only storing the last QC. We could store more but we're realistically only going to retrieve the last one.
            if let Err(e) = self.api.store_leaf(old_anchor_view, leaf).await {
                error!("Could not insert new anchor into the storage API: {:?}", e);
            }

            decide_sent.await;
        }
        self.high_qc
    }
}
