//! Contains the [`DAMember`] struct used for the committee member step in the consensus algorithm
//! with DA committee, i.e. in the sequencing consensus.

use crate::{
    utils::{View, ViewInner},
    Consensus, SequencingConsensusApi,
};
use async_compatibility_layer::channel::UnboundedReceiver;
use async_lock::{Mutex, RwLock};
use commit::Committable;
use either::{Left, Right};
use hotshot_types::{
    certificate::QuorumCertificate,
    data::SequencingLeaf,
    message::{
        ConsensusMessageType, ProcessedCommitteeConsensusMessage, ProcessedGeneralConsensusMessage,
        ProcessedSequencingMessage, SequencingMessage,
    },
    traits::{
        election::{CommitteeExchangeType, ConsensusExchange},
        node_implementation::{
            CommitteeEx, CommitteeProposalType, CommitteeVote, NodeImplementation, NodeType,
        },
        signature_key::SignatureKey,
    },
};
use std::{marker::PhantomData, sync::Arc};
use tracing::{error, info, instrument, warn};

/// This view's DA committee member.
#[derive(Debug, Clone)]
pub struct DAMember<
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
    /// Reference to consensus. DA committee member will require a write lock on this.
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
    pub exchange: Arc<CommitteeEx<TYPES, I>>,

    /// needed for type checking
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
    > DAMember<A, TYPES, I>
{
    /// DA committee member task that spins until a valid DA proposal can be signed or timeout is
    /// hit.
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "DA Member Task", level = "error")]
    #[allow(clippy::type_complexity)]
    async fn find_valid_msg<'a>(
        &self,
        view_leader_key: TYPES::SignatureKey,
    ) -> Option<TYPES::BlockType> {
        let lock = self.proposal_collection_chan.lock().await;
        let leaf = loop {
            let msg = lock.recv().await;
            info!("recv-ed message {:?}", msg.clone());
            if let Ok(msg) = msg {
                // If the message is for a different view number, skip it.
                if Into::<SequencingMessage<_, _>>::into(msg.clone()).view_number() != self.cur_view
                {
                    continue;
                }
                match msg {
                    Left(general_message) => {
                        match general_message {
                            ProcessedGeneralConsensusMessage::InternalTrigger(_trigger) => {
                                warn!("DA committee member receieved an internal trigger message. This is not what the member expects. Skipping.");
                                // After run_view refactor we will handle timeout triggers properly
                                return None;
                            }
                            ProcessedGeneralConsensusMessage::Vote(_, _) => {
                                // Should only be for DA leader, never member.
                                warn!("DA committee member receieved a vote message. This is not what the member expects. Skipping.");
                                continue;
                            }
                            ProcessedGeneralConsensusMessage::Proposal(_, _) => {
                                warn!("DA committee member receieved a Non DA Proposal message. This is not what the member expects. Skipping.");
                                continue;
                            }
                            ProcessedGeneralConsensusMessage::ViewSyncCertificate(_) => {
                                todo!()
                            }
                            ProcessedGeneralConsensusMessage::ViewSyncVote(_) => {
                                todo!()
                            }
                        }
                    }
                    Right(committee_message) => {
                        match committee_message {
                            ProcessedCommitteeConsensusMessage::DAProposal(p, sender) => {
                                if view_leader_key != sender {
                                    continue;
                                }
                                let block_commitment = p.data.deltas.commit();
                                if !view_leader_key
                                    .validate(&p.signature, block_commitment.as_ref())
                                {
                                    warn!(?p.signature, "Could not verify proposal.");
                                    continue;
                                }
                                let vote_token = self.exchange.make_vote_token(self.cur_view);
                                match vote_token {
                                    Err(e) => {
                                        error!(
                                            "Failed to generate vote token for {:?} {:?}",
                                            self.cur_view, e
                                        );
                                    }
                                    Ok(None) => {
                                        info!(
                                            "We were not chosen for DA committee on {:?}",
                                            self.cur_view
                                        );
                                    }
                                    Ok(Some(vote_token)) => {
                                        info!(
                                            "We were chosen for DA committee on {:?}",
                                            self.cur_view
                                        );

                                        // Generate and send vote
                                        let message = self.exchange.create_da_message(
                                            block_commitment,
                                            self.cur_view,
                                            vote_token,
                                        );

                                        info!("Sending vote to the leader {:?}", message);

                                        let consensus = self.consensus.read().await;
                                        if self.api.send_direct_da_message::<CommitteeProposalType<TYPES, I>, CommitteeVote<TYPES, I>>(sender, SequencingMessage(Right(message))).await.is_err() {
                                            consensus.metrics.failed_to_send_messages.add(1);
                                            warn!("Failed to send vote to the leader");
                                        } else {
                                            consensus.metrics.outgoing_direct_messages.add(1);
                                        }
                                    }
                                }
                                break p.data.deltas;
                            }
                            ProcessedCommitteeConsensusMessage::DAVote(_, _) => {
                                // Should only be for DA leader, never member.
                                warn!("DA committee member receieved a vote message. This is not what the member expects. Skipping.");
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
            warn!("DA committee member did not receive successfully from channel.");
            return None;
        };
        Some(leaf)
    }

    /// Run one view of DA committee member.
    #[instrument(skip(self), fields(id = self.id, view = *self.cur_view), name = "DA Member Task", level = "error")]
    pub async fn run_view(self) -> QuorumCertificate<TYPES, SequencingLeaf<TYPES>> {
        info!("DA Committee Member task started!");
        let view_leader_key = self.exchange.get_leader(self.cur_view);

        let maybe_block = self.find_valid_msg(view_leader_key).await;

        if let Some(block) = maybe_block {
            let mut consensus = self.consensus.write().await;

            // Ensure this view is in the view map for garbage collection, but do not overwrite if
            // there is already a view there: the replica task may have inserted a `Leaf` view which
            // contains strictly more information.
            consensus.state_map.entry(self.cur_view).or_insert(View {
                view_inner: ViewInner::DA {
                    block: block.commit(),
                },
            });

            // Record the block we have promised to make available.
            consensus.saved_blocks.insert(block);
        };

        self.high_qc
    }
}
