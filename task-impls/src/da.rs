use crate::{
    events::HotShotEvent,
    vote::{create_vote_accumulator, AccumulatorInfo, VoteCollectionTaskState},
};
use async_lock::RwLock;

use hotshot_task::{
    event_stream::{ChannelStream, EventStream},
    global_registry::GlobalRegistry,
    task::{HotShotTaskCompleted, TS},
    task_impls::HSTWithEvent,
};
use hotshot_types::{
    consensus::{Consensus, View},
    data::DAProposal,
    event::{Event, EventType},
    message::Proposal,
    simple_certificate::DACertificate,
    simple_vote::{DAData, DAVote},
    traits::{
        block_contents::vid_commitment,
        consensus_api::ConsensusApi,
        election::Membership,
        network::{CommunicationChannel, ConsensusIntentEvent},
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::SignatureKey,
    },
    utils::ViewInner,
    vote::HasViewNumber,
};
use sha2::{Digest, Sha256};

use crate::vote::HandleVoteEvent;
use snafu::Snafu;
use std::{marker::PhantomData, sync::Arc};
use tracing::{debug, error, instrument, warn};

/// Alias for Optional type for Vote Collectors
type VoteCollectorOption<TYPES, VOTE, CERT> = Option<VoteCollectionTaskState<TYPES, VOTE, CERT>>;

#[derive(Snafu, Debug)]
/// Error type for consensus tasks
pub struct ConsensusTaskError {}

/// Tracks state of a DA task
pub struct DATaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    A: ConsensusApi<TYPES, I> + 'static,
> {
    /// The state's api
    pub api: A,
    /// Global registry task for the state
    pub registry: GlobalRegistry,

    /// View number this view is executing in.
    pub cur_view: TYPES::Time,

    /// Reference to consensus. Leader will require a read lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES>>>,

    /// Membership for the DA committee
    pub da_membership: Arc<TYPES::Membership>,

    /// Membership for the quorum committee
    /// We need this only for calculating the proper VID scheme
    /// from the number of nodes in the quorum.
    pub quorum_membership: Arc<TYPES::Membership>,

    /// Network for DA
    pub da_network: Arc<I::CommitteeNetwork>,

    /// The current vote collection task, if there is one.
    pub vote_collector: RwLock<VoteCollectorOption<TYPES, DAVote<TYPES>, DACertificate<TYPES>>>,

    /// Global events stream to publish events
    pub event_stream: ChannelStream<HotShotEvent<TYPES>>,

    /// This Nodes public key
    pub public_key: TYPES::SignatureKey,

    /// This Nodes private key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,

    /// This state's ID
    pub id: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static>
    DATaskState<TYPES, I, A>
{
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "DA Main Task", level = "error")]
    pub async fn handle_event(
        &mut self,
        event: HotShotEvent<TYPES>,
    ) -> Option<HotShotTaskCompleted> {
        match event {
            HotShotEvent::DAProposalRecv(proposal, sender) => {
                debug!(
                    "DA proposal received for view: {:?}",
                    proposal.data.get_view_number()
                );
                // ED NOTE: Assuming that the next view leader is the one who sends DA proposal for this view
                let view = proposal.data.get_view_number();

                // Allow a DA proposal that is one view older, in case we have voted on a quorum
                // proposal and updated the view.
                // `self.cur_view` should be at least 1 since there is a view change before getting
                // the `DAProposalRecv` event. Otherewise, the view number subtraction below will
                // cause an overflow error.
                // TODO ED Come back to this - we probably don't need this, but we should also never receive a DAC where this fails, investigate block ready so it doesn't make one for the genesis block

                // stop polling for the received proposal
                self.da_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForProposal(
                        *proposal.data.view_number,
                    ))
                    .await;

                if self.cur_view != TYPES::Time::genesis() && view < self.cur_view - 1 {
                    warn!("Throwing away DA proposal that is more than one view older");
                    return None;
                }

                let payload_commitment = vid_commitment(
                    &proposal.data.encoded_transactions,
                    self.quorum_membership.total_nodes(),
                );
                let encoded_transactions_hash = Sha256::digest(&proposal.data.encoded_transactions);

                // ED Is this the right leader?
                let view_leader_key = self.da_membership.get_leader(view);
                if view_leader_key != sender {
                    error!("DA proposal doesn't have expected leader key for view {} \n DA proposal is: {:?}", *view, proposal.data.clone());
                    return None;
                }

                if !view_leader_key.validate(&proposal.signature, &encoded_transactions_hash) {
                    error!("Could not verify proposal.");
                    return None;
                }

                // Proposal is fresh and valid, notify the application layer
                self.api
                    .send_event(Event {
                        view_number: self.cur_view,
                        event: EventType::DAProposal {
                            proposal: proposal.clone(),
                            sender: sender.clone(),
                        },
                    })
                    .await;

                if !self.da_membership.has_stake(&self.public_key) {
                    debug!(
                        "We were not chosen for consensus committee on {:?}",
                        self.cur_view
                    );
                    return None;
                }
                // Generate and send vote
                let Ok(vote) = DAVote::create_signed_vote(
                    DAData {
                        payload_commit: payload_commitment,
                    },
                    view,
                    &self.public_key,
                    &self.private_key,
                ) else {
                    error!("Failed to sign DA Vote!");
                    return None;
                };

                // ED Don't think this is necessary?
                // self.cur_view = view;

                debug!("Sending vote to the DA leader {:?}", vote.get_view_number());
                self.event_stream
                    .publish(HotShotEvent::DAVoteSend(vote))
                    .await;
                let mut consensus = self.consensus.write().await;

                // Ensure this view is in the view map for garbage collection, but do not overwrite if
                // there is already a view there: the replica task may have inserted a `Leaf` view which
                // contains strictly more information.
                consensus.validated_state_map.entry(view).or_insert(View {
                    view_inner: ViewInner::DA { payload_commitment },
                });

                // Record the payload we have promised to make available.
                consensus
                    .saved_payloads
                    .insert(view, proposal.data.encoded_transactions);
            }
            HotShotEvent::DAVoteRecv(ref vote) => {
                debug!("DA vote recv, Main Task {:?}", vote.get_view_number());
                // Check if we are the leader and the vote is from the sender.
                let view = vote.get_view_number();
                if self.da_membership.get_leader(view) != self.public_key {
                    error!("We are not the committee leader for view {} are we leader for next view? {}", *view, self.da_membership.get_leader(view + 1) == self.public_key);
                    return None;
                }
                let mut collector = self.vote_collector.write().await;

                let maybe_task = collector.take();

                if maybe_task.is_none()
                    || vote.get_view_number() > maybe_task.as_ref().unwrap().view
                {
                    debug!("Starting vote handle for view {:?}", vote.get_view_number());
                    let info = AccumulatorInfo {
                        public_key: self.public_key.clone(),
                        membership: self.da_membership.clone(),
                        view: vote.get_view_number(),
                        event_stream: self.event_stream.clone(),
                        id: self.id,
                        registry: self.registry.clone(),
                    };
                    *collector = create_vote_accumulator::<
                        TYPES,
                        DAVote<TYPES>,
                        DACertificate<TYPES>,
                    >(&info, vote.clone(), event)
                    .await;
                } else {
                    let result = maybe_task.unwrap().handle_event(event.clone()).await;

                    if result.0 == Some(HotShotTaskCompleted::ShutDown) {
                        // The protocol has finished
                        return None;
                    }
                    *collector = Some(result.1);
                }
            }
            HotShotEvent::ViewChange(view) => {
                if *self.cur_view >= *view {
                    return None;
                }

                if *view - *self.cur_view > 1 {
                    warn!("View changed by more than 1 going to view {:?}", view);
                }
                self.cur_view = view;

                // Inject view info into network
                let is_da = self
                    .da_membership
                    .get_committee(self.cur_view + 1)
                    .contains(&self.public_key);

                if is_da {
                    debug!("Polling for DA proposals for view {}", *self.cur_view + 1);
                    self.da_network
                        .inject_consensus_info(ConsensusIntentEvent::PollForProposal(
                            *self.cur_view + 1,
                        ))
                        .await;
                }
                if self.da_membership.get_leader(self.cur_view + 3) == self.public_key {
                    debug!("Polling for transactions for view {}", *self.cur_view + 3);
                    self.da_network
                        .inject_consensus_info(ConsensusIntentEvent::PollForTransactions(
                            *self.cur_view + 3,
                        ))
                        .await;
                }

                // If we are not the next leader (DA leader for this view) immediately exit
                if self.da_membership.get_leader(self.cur_view + 1) != self.public_key {
                    // panic!("We are not the DA leader for view {}", *self.cur_view + 1);
                    return None;
                }
                debug!("Polling for DA votes for view {}", *self.cur_view + 1);

                // Start polling for DA votes for the "next view"
                self.da_network
                    .inject_consensus_info(ConsensusIntentEvent::PollForVotes(*self.cur_view + 1))
                    .await;

                return None;
            }
            HotShotEvent::TransactionsSequenced(encoded_transactions, metadata, view) => {
                self.da_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForTransactions(*view))
                    .await;

                // quick hash the encoded txns with sha256
                let encoded_transactions_hash = Sha256::digest(&encoded_transactions);

                // sign the encoded transactions as opposed to the VID commitment
                let Ok(signature) =
                    TYPES::SignatureKey::sign(&self.private_key, &encoded_transactions_hash)
                else {
                    error!("Failed to sign block payload!");
                    return None;
                };

                let data: DAProposal<TYPES> = DAProposal {
                    encoded_transactions,
                    metadata: metadata.clone(),
                    // Upon entering a new view we want to send a DA Proposal for the next view -> Is it always the case that this is cur_view + 1?
                    view_number: view,
                };
                debug!("Sending DA proposal for view {:?}", data.view_number);

                let message = Proposal {
                    data,
                    signature,
                    _pd: PhantomData,
                };

                self.event_stream
                    .publish(HotShotEvent::DAProposalSend(
                        message.clone(),
                        self.public_key.clone(),
                    ))
                    .await;
            }

            HotShotEvent::Timeout(view) => {
                self.da_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(*view))
                    .await;
            }

            HotShotEvent::Shutdown => {
                error!("Shutting down because of shutdown signal!");
                return Some(HotShotTaskCompleted::ShutDown);
            }
            _ => {
                error!("unexpected event {:?}", event);
            }
        }
        None
    }

    /// Filter the DA event.
    pub fn filter(event: &HotShotEvent<TYPES>) -> bool {
        matches!(
            event,
            HotShotEvent::DAProposalRecv(_, _)
                | HotShotEvent::DAVoteRecv(_)
                | HotShotEvent::Shutdown
                | HotShotEvent::TransactionsSequenced(_, _, _)
                | HotShotEvent::Timeout(_)
                | HotShotEvent::ViewChange(_)
        )
    }
}

/// task state implementation for DA Task
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static> TS
    for DATaskState<TYPES, I, A>
{
}

/// Type alias for DA Task Types
pub type DATaskTypes<TYPES, I, A> = HSTWithEvent<
    ConsensusTaskError,
    HotShotEvent<TYPES>,
    ChannelStream<HotShotEvent<TYPES>>,
    DATaskState<TYPES, I, A>,
>;
