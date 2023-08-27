//! Contains the [`DALeader`], [`ConsensusLeader`] and [`ConsensusNextLeader`] structs used for the
//! leader steps in the consensus algorithm with DA committee, i.e. in the sequencing consensus.

use crate::{CommitmentMap, Consensus, SequencingConsensusApi};
use async_compatibility_layer::{
    art::async_timeout,
    async_primitives::subscribable_rwlock::{ReadView, SubscribableRwLock},
    channel::UnboundedReceiver,
};
use async_lock::{Mutex, RwLock};
use bitvec::prelude::*;
use commit::{Commitment, Committable};
use either::{Either, Left, Right};
use hotshot_types::{
    certificate::{AssembledSignature, DACertificate, QuorumCertificate},
    data::{DAProposal, QuorumProposal, SequencingLeaf},
    message::{
        CommitteeConsensusMessage, ConsensusMessageType, GeneralConsensusMessage, InternalTrigger,
        Message, ProcessedCommitteeConsensusMessage, ProcessedGeneralConsensusMessage,
        ProcessedSequencingMessage, Proposal, SequencingMessage,
    },
    traits::{
        election::{
            CommitteeExchangeType, ConsensusExchange, QuorumExchangeType, SignedCertificate,
        },
        node_implementation::{
            CommitteeEx, NodeImplementation, NodeType, QuorumProposalType, QuorumVoteType,
            SequencingQuorumEx,
        },
        signature_key::SignatureKey,
        state::State,
        Block,
    },
    vote::{QuorumVote, VoteAccumulator},
};
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    num::NonZeroU64,
    sync::Arc,
    time::Instant,
};
use tracing::{error, info, instrument, warn};
/// This view's DA committee leader
#[derive(Debug, Clone)]
pub struct DALeader<
    A: SequencingConsensusApi<TYPES, SequencingLeaf<TYPES>, I>,
    TYPES: NodeType,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
> {
    /// id of node
    pub id: u64,
    /// Reference to consensus. Leader will require a read lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES, SequencingLeaf<TYPES>>>>,
    /// The `high_qc` per spec
    pub high_qc: QuorumCertificate<TYPES, SequencingLeaf<TYPES>>,
    /// The view number we're running on
    pub cur_view: TYPES::Time,
    /// Lock over the transactions list
    pub transactions: Arc<SubscribableRwLock<CommitmentMap<TYPES::Transaction>>>,
    /// Limited access to the consensus protocol
    pub api: A,

    /// the committee exchange
    pub committee_exchange: Arc<CommitteeEx<TYPES, I>>,
    /// the quorum exchange
    pub quorum_exchange: Arc<SequencingQuorumEx<TYPES, I>>,
    /// channel through which the leader collects votes
    #[allow(clippy::type_complexity)]
    pub vote_collection_chan: Arc<Mutex<UnboundedReceiver<ProcessedSequencingMessage<TYPES, I>>>>,
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
    > DALeader<A, TYPES, I>
where
    CommitteeEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = DACertificate<TYPES>,
        Commitment = TYPES::BlockType,
    >,
{
    /// Accumulate votes for a proposal and return either the cert or None if the threshold was not reached in time
    async fn wait_for_votes(
        &self,
        cur_view: TYPES::Time,
        threshold: NonZeroU64,
        total_nodes_num: usize,
        block_commitment: Commitment<<TYPES as NodeType>::BlockType>,
    ) -> Option<DACertificate<TYPES>> {
        let lock = self.vote_collection_chan.lock().await;
        let mut accumulator = VoteAccumulator {
            total_vote_outcomes: HashMap::new(),
            da_vote_outcomes: HashMap::new(),
            yes_vote_outcomes: HashMap::new(),
            no_vote_outcomes: HashMap::new(),
            viewsync_precommit_vote_outcomes: HashMap::new(),
            viewsync_commit_vote_outcomes: HashMap::new(),
            viewsync_finalize_vote_outcomes: HashMap::new(),
            success_threshold: threshold,
            failure_threshold: threshold,
            sig_lists: Vec::new(),
            signers: bitvec![0; total_nodes_num],
        };

        while let Ok(msg) = lock.recv().await {
            if Into::<SequencingMessage<_, _>>::into(msg.clone()).view_number() != cur_view {
                continue;
            }
            match msg {
                Left(general_message) => match general_message {
                    ProcessedGeneralConsensusMessage::Vote(_vote, _sender) => {
                        warn!("The leader received an unexpext Quorum Vote!");
                        continue;
                    }
                    ProcessedGeneralConsensusMessage::InternalTrigger(trigger) => match trigger {
                        InternalTrigger::Timeout(_) => {
                            self.api.send_next_leader_timeout(self.cur_view).await;
                            break;
                        }
                    },
                    ProcessedGeneralConsensusMessage::Proposal(_p, _sender) => {
                        warn!("The next leader has received an unexpected proposal!");
                    }
                    ProcessedGeneralConsensusMessage::ViewSyncCertificate(_) => todo!(),
                    ProcessedGeneralConsensusMessage::ViewSyncVote(_) => todo!(),
                },
                Right(committee_message) => match committee_message {
                    ProcessedCommitteeConsensusMessage::DAVote(vote, sender) => {
                        if vote.signature.0
                            != <TYPES::SignatureKey as SignatureKey>::to_bytes(&sender)
                        {
                            continue;
                        }
                        if vote.block_commitment != block_commitment {
                            continue;
                        }
                        match self.committee_exchange.accumulate_vote(
                            &vote.signature.0,
                            &vote.signature.1,
                            vote.block_commitment,
                            vote.vote_data,
                            vote.vote_token.clone(),
                            self.cur_view,
                            accumulator,
                            None,
                        ) {
                            Either::Left(acc) => {
                                accumulator = acc;
                            }
                            Either::Right(qc) => {
                                match qc.clone().signatures {
                                    AssembledSignature::Yes(_signature) => {}
                                    AssembledSignature::DA(_signature) => {}
                                    _ => unimplemented!(),
                                };
                                return Some(qc);
                            }
                        }
                    }
                    ProcessedCommitteeConsensusMessage::DAProposal(_p, _sender) => {
                        warn!("The next leader has received an unexpected proposal!");
                    }
                    ProcessedCommitteeConsensusMessage::DACertificate(_, _) => {
                        continue;
                    }
                },
            }
        }
        None
    }
    /// Returns the parent leaf of the proposal we are building
    async fn parent_leaf(&self) -> Option<SequencingLeaf<TYPES>> {
        let parent_view_number = &self.high_qc.view_number();
        let consensus = self.consensus.read().await;
        let Some(parent_view) = consensus.state_map.get(parent_view_number) else {
            warn!("Couldn't find high QC parent in state map.");
            return None;
        };
        let Some(leaf) = parent_view.get_leaf_commitment() else {
            warn!(
                ?parent_view_number,
                ?parent_view,
                "Parent of high QC points to a view without a proposal"
            );
            return None;
        };
        let Some(leaf) = consensus.saved_leaves.get(&leaf) else {
            warn!("Failed to find high QC parent.");
            return None;
        };
        Some(leaf.clone())
    }
    /// return None if we can't get transactions
    async fn wait_for_transactions(&self) -> Option<Vec<TYPES::Transaction>> {
        let task_start_time = Instant::now();

        let parent_leaf = self.parent_leaf().await?;
        let previous_used_txns = match parent_leaf.deltas {
            Either::Left(block) => block.contained_transactions(),
            Either::Right(_commitment) => HashSet::new(),
        };
        let receiver = self.transactions.subscribe().await;

        while task_start_time.elapsed() < self.api.propose_max_round_time() {
            let txns = self.transactions.cloned().await;
            let unclaimed_txns: Vec<_> = txns
                .iter()
                .filter(|(txn_hash, _txn)| !previous_used_txns.contains(txn_hash))
                .collect();

            let time_past = task_start_time.elapsed();
            if unclaimed_txns.len() < self.api.min_transactions()
                && (time_past < self.api.propose_max_round_time())
            {
                let duration = self.api.propose_max_round_time() - time_past;
                let result = async_timeout(duration, receiver.recv()).await;
                match result {
                    Err(_) => {
                        // Fall through below to updating new block
                        info!("propose_max_round_time passed, sending transactions we have so far");
                    }
                    Ok(Err(e)) => {
                        // Something unprecedented is wrong, and `transactions` has been dropped
                        error!("Channel receiver error for SubscribableRwLock {:?}", e);
                        return None;
                    }
                    Ok(Ok(_)) => continue,
                }
            }
            let mut txns = vec![];
            for (_hash, txn) in unclaimed_txns {
                txns.push(txn.clone());
            }
            return Some(txns);
        }
        None
    }
    /// Run one view of the DA leader task
    #[instrument(skip(self), fields(id = self.id, view = *self.cur_view), name = "Sequencing DALeader Task", level = "error")]
    pub async fn run_view(
        self,
    ) -> Option<(
        DACertificate<TYPES>,
        TYPES::BlockType,
        SequencingLeaf<TYPES>,
    )> {
        // Prepare the DA Proposal
        let Some(parent_leaf) = self.parent_leaf().await else {
            warn!("Couldn't find high QC parent in state map.");
            return None;
        };

        let mut block = <TYPES as NodeType>::StateType::next_block(None);
        let txns = self.wait_for_transactions().await?;

        for txn in txns {
            if let Ok(new_block) = block.add_transaction_raw(&txn) {
                block = new_block;
                continue;
            }
        }
        let block_commitment = block.commit();

        let consensus = self.consensus.read().await;
        let signature = self.committee_exchange.sign_da_proposal(&block.commit());
        let data: DAProposal<TYPES> = DAProposal {
            deltas: block.clone(),
            view_number: self.cur_view,
        };
        let message =
            SequencingMessage::<TYPES, I>(Right(CommitteeConsensusMessage::DAProposal(Proposal {
                data,
                signature,
            })));
        // Brodcast DA proposal
        if let Err(e) = self.api.send_da_broadcast(message.clone()).await {
            consensus.metrics.failed_to_send_messages.add(1);
            warn!(?message, ?e, "Could not broadcast leader proposal");
        } else {
            consensus.metrics.outgoing_broadcast_messages.add(1);
        }

        // Drop the lock on the consensus.
        drop(consensus);

        // Wait for DA votes or Timeout
        if let Some(cert) = self
            .wait_for_votes(
                self.cur_view,
                self.committee_exchange.success_threshold(),
                self.committee_exchange.total_nodes(),
                block_commitment,
            )
            .await
        {
            return Some((cert, block, parent_leaf));
        }
        None
    }
}

/// Implemenation of the consensus leader for a DA/Sequencing consensus.  Handles sending out a proposal to the entire network
/// For now this step happens after the `DALeader` completes it's proposal and collects enough votes.
pub struct ConsensusLeader<
    A: SequencingConsensusApi<TYPES, SequencingLeaf<TYPES>, I>,
    TYPES: NodeType,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
> {
    /// id of node
    pub id: u64,
    /// Reference to consensus. Leader will require a read lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES, SequencingLeaf<TYPES>>>>,
    /// The `high_qc` per spec
    pub high_qc: QuorumCertificate<TYPES, SequencingLeaf<TYPES>>,
    /// The view number we're running on
    pub cur_view: TYPES::Time,
    /// The Certificate generated for the transactions commited to in the proposal the leader will build
    pub cert: DACertificate<TYPES>,
    /// The block corresponding to the DA cert
    pub block: TYPES::BlockType,
    /// Leaf this proposal will chain from
    pub parent: SequencingLeaf<TYPES>,
    /// Limited access to the consensus protocol
    pub api: A,

    /// the quorum exchange
    pub quorum_exchange: Arc<SequencingQuorumEx<TYPES, I>>,

    /// needed to tyep check
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
    > ConsensusLeader<A, TYPES, I>
where
    SequencingQuorumEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Proposal = QuorumProposal<TYPES, SequencingLeaf<TYPES>>,
    >,
{
    /// Run one view of the DA leader task
    #[instrument(skip(self), fields(id = self.id, view = *self.cur_view), name = "Sequencing DALeader Task", level = "error")]
    pub async fn run_view(self) -> QuorumCertificate<TYPES, SequencingLeaf<TYPES>> {
        let block_commitment = self.block.commit();
        let leaf = SequencingLeaf {
            view_number: self.cur_view,
            height: self.parent.height + 1,
            justify_qc: self.high_qc.clone(),
            parent_commitment: self.parent.commit(),
            // Use the block commitment rather than the block, so that the replica can construct
            // the same leaf with the commitment.
            deltas: Right(block_commitment),
            rejected: vec![],
            timestamp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
            proposer_id: self.api.public_key().to_bytes(),
        };
        let signature = self
            .quorum_exchange
            .sign_validating_or_commitment_proposal::<I>(&leaf.commit());
        // TODO: DA cert is sent as part of the proposal here, we should split this out so we don't have to wait for it.
        let proposal = QuorumProposal {
            block_commitment,
            view_number: leaf.view_number,
            height: leaf.height,
            justify_qc: self.high_qc.clone(),
            timeout_certificate: None,
            dac: Some(self.cert),
            proposer_id: leaf.proposer_id,
        };
        let message =
            SequencingMessage::<TYPES, I>(Left(GeneralConsensusMessage::Proposal(Proposal {
                data: proposal,
                signature,
            })));
        if let Err(e) = self
            .api
            .send_broadcast_message::<QuorumProposalType<TYPES, I>, QuorumVoteType<TYPES, I>>(
                message.clone(),
            )
            .await
        {
            warn!(?message, ?e, "Could not broadcast leader proposal");
        }
        self.high_qc
    }
}

/// Implenting the next leader.  Collect votes on the previous leaders proposal and return the QC
pub struct ConsensusNextLeader<
    A: SequencingConsensusApi<TYPES, SequencingLeaf<TYPES>, I>,
    TYPES: NodeType,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
> {
    /// id of node
    pub id: u64,
    /// Reference to consensus. Leader will require a read lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES, SequencingLeaf<TYPES>>>>,
    /// The view number we're running on
    pub cur_view: TYPES::Time,
    /// Limited access to the consensus protocol
    pub api: A,
    /// generic_qc before starting this
    pub generic_qc: QuorumCertificate<TYPES, SequencingLeaf<TYPES>>,
    /// channel through which the leader collects votes
    #[allow(clippy::type_complexity)]
    pub vote_collection_chan: Arc<Mutex<UnboundedReceiver<ProcessedSequencingMessage<TYPES, I>>>>,

    /// the quorum exchnage
    pub quorum_exchange: Arc<SequencingQuorumEx<TYPES, I>>,

    /// needed to type check
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
    > ConsensusNextLeader<A, TYPES, I>
where
    SequencingQuorumEx<TYPES, I>: ConsensusExchange<
        TYPES,
        Message<TYPES, I>,
        Certificate = QuorumCertificate<TYPES, SequencingLeaf<TYPES>>,
        Commitment = SequencingLeaf<TYPES>,
    >,
{
    /// Run one view of the next leader, collect votes and build a QC for the last views `QuorumProposal`
    /// # Panics
    /// While we are unwrapping, this function can logically never panic
    /// unless there is a bug in std
    pub async fn run_view(self) -> QuorumCertificate<TYPES, SequencingLeaf<TYPES>> {
        let mut qcs = HashSet::<QuorumCertificate<TYPES, SequencingLeaf<TYPES>>>::new();
        qcs.insert(self.generic_qc.clone());
        let mut accumulator = VoteAccumulator {
            total_vote_outcomes: HashMap::new(),
            da_vote_outcomes: HashMap::new(),
            yes_vote_outcomes: HashMap::new(),
            no_vote_outcomes: HashMap::new(),
            viewsync_precommit_vote_outcomes: HashMap::new(),
            viewsync_commit_vote_outcomes: HashMap::new(),
            viewsync_finalize_vote_outcomes: HashMap::new(),
            success_threshold: self.quorum_exchange.success_threshold(),
            failure_threshold: self.quorum_exchange.failure_threshold(),
            sig_lists: Vec::new(),
            signers: bitvec![0; self.quorum_exchange.total_nodes()],
        };

        let lock = self.vote_collection_chan.lock().await;
        while let Ok(msg) = lock.recv().await {
            // If the message is for a different view number, skip it.
            if Into::<SequencingMessage<_, _>>::into(msg.clone()).view_number() != self.cur_view {
                continue;
            }
            match msg {
                Left(general_message) => match general_message {
                    ProcessedGeneralConsensusMessage::Vote(vote_message, sender) => {
                        match vote_message {
                            QuorumVote::Yes(vote) => {
                                if vote.signature.0
                                    != <TYPES::SignatureKey as SignatureKey>::to_bytes(&sender)
                                {
                                    continue;
                                }

                                match self.quorum_exchange.accumulate_vote(
                                    &vote.signature.0,
                                    &vote.signature.1,
                                    vote.leaf_commitment,
                                    vote.vote_data,
                                    vote.vote_token.clone(),
                                    self.cur_view,
                                    accumulator,
                                    None,
                                ) {
                                    Either::Left(acc) => {
                                        accumulator = acc;
                                    }
                                    Either::Right(qc) => {
                                        match qc.clone().signatures {
                                            AssembledSignature::Yes(_signature) => {}
                                            _ => unimplemented!(),
                                        };
                                        return qc;
                                    }
                                }
                            }
                            QuorumVote::Timeout(vote) => {
                                qcs.insert(vote.high_qc);
                            }
                            QuorumVote::No(_) => {
                                warn!("The next leader has received an unexpected vote!");
                            }
                        }
                    }
                    ProcessedGeneralConsensusMessage::InternalTrigger(trigger) => match trigger {
                        InternalTrigger::Timeout(_) => {
                            self.api.send_next_leader_timeout(self.cur_view).await;
                            break;
                        }
                    },
                    ProcessedGeneralConsensusMessage::Proposal(_p, _sender) => {
                        warn!("The next leader has received an unexpected proposal!");
                    }
                    ProcessedGeneralConsensusMessage::ViewSyncCertificate(_) => todo!(),
                    ProcessedGeneralConsensusMessage::ViewSyncVote(_) => todo!(),
                },
                Right(committee_message) => match committee_message {
                    ProcessedCommitteeConsensusMessage::DAProposal(_p, _sender) => {
                        warn!("The next leader has received an unexpected proposal!");
                    }
                    ProcessedCommitteeConsensusMessage::DAVote(_, _sender) => {
                        warn!("The next leader has received an unexpected DA vote!");
                    }
                    ProcessedCommitteeConsensusMessage::DACertificate(_, _) => {
                        continue;
                    }
                },
            }
        }
        qcs.into_iter().max_by_key(|qc| qc.view_number).unwrap()
    }
}
