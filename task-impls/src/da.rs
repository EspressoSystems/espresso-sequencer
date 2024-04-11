use std::{marker::PhantomData, sync::Arc};

use async_broadcast::Sender;
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::spawn_blocking;
use hotshot_task::task::{Task, TaskState};
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
        network::{ConnectedNetwork, ConsensusIntentEvent},
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::SignatureKey,
        storage::Storage,
    },
    utils::ViewInner,
    vote::HasViewNumber,
};
use sha2::{Digest, Sha256};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::spawn_blocking;
use tracing::{debug, error, instrument, warn};

use crate::{
    events::{HotShotEvent, HotShotTaskCompleted},
    helpers::broadcast_event,
    vote_collection::{
        create_vote_accumulator, AccumulatorInfo, HandleVoteEvent, VoteCollectionTaskState,
    },
};

/// Alias for Optional type for Vote Collectors
type VoteCollectorOption<TYPES, VOTE, CERT> = Option<VoteCollectionTaskState<TYPES, VOTE, CERT>>;

/// Tracks state of a DA task
pub struct DATaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    A: ConsensusApi<TYPES, I> + 'static,
> {
    /// The state's api
    pub api: A,

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

    /// This Nodes public key
    pub public_key: TYPES::SignatureKey,

    /// This Nodes private key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,

    /// This state's ID
    pub id: u64,

    /// This node's storage ref
    pub storage: Arc<RwLock<I::Storage>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static>
    DATaskState<TYPES, I, A>
{
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "DA Main Task", level = "error")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    ) -> Option<HotShotTaskCompleted> {
        match event.as_ref() {
            HotShotEvent::DAProposalRecv(proposal, sender) => {
                let sender = sender.clone();
                debug!(
                    "DA proposal received for view: {:?}",
                    proposal.data.get_view_number()
                );
                // ED NOTE: Assuming that the next view leader is the one who sends DA proposal for this view
                let view = proposal.data.get_view_number();

                // Allow a DA proposal that is one view older, in case we have voted on a quorum
                // proposal and updated the view.
                // `self.cur_view` should be at least 1 since there is a view change before getting
                // the `DAProposalRecv` event. Otherwise, the view number subtraction below will
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

                if self
                    .consensus
                    .read()
                    .await
                    .saved_payloads
                    .contains_key(&view)
                {
                    warn!("Received DA proposal for view {:?} but we already have a payload for that view.  Throwing it away", view);
                    return None;
                }

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

                broadcast_event(
                    Arc::new(HotShotEvent::DAProposalValidated(proposal.clone(), sender)),
                    &event_stream,
                )
                .await;
            }
            HotShotEvent::DAProposalValidated(proposal, sender) => {
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
                if let Err(e) = self.storage.write().await.append_da(proposal).await {
                    error!(
                        "Failed to store DA Proposal with error {:?}, aborting vote",
                        e
                    );
                    return None;
                }
                let txns = proposal.data.encoded_transactions.clone();
                let num_nodes = self.quorum_membership.total_nodes();
                let payload_commitment =
                    spawn_blocking(move || vid_commitment(&txns, num_nodes)).await;
                #[cfg(async_executor_impl = "tokio")]
                let payload_commitment = payload_commitment.unwrap();

                let view = proposal.data.get_view_number();
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

                debug!("Sending vote to the DA leader {:?}", vote.get_view_number());

                broadcast_event(Arc::new(HotShotEvent::DAVoteSend(vote)), &event_stream).await;
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
                    .insert(view, proposal.data.encoded_transactions.clone());
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

                if collector.is_none() || vote.get_view_number() > collector.as_ref().unwrap().view
                {
                    debug!("Starting vote handle for view {:?}", vote.get_view_number());
                    let info = AccumulatorInfo {
                        public_key: self.public_key.clone(),
                        membership: self.da_membership.clone(),
                        view: vote.get_view_number(),
                        id: self.id,
                    };
                    *collector = create_vote_accumulator::<
                        TYPES,
                        DAVote<TYPES>,
                        DACertificate<TYPES>,
                    >(&info, vote.clone(), event, &event_stream)
                    .await;
                } else {
                    let result = collector
                        .as_mut()
                        .unwrap()
                        .handle_event(event.clone(), &event_stream)
                        .await;

                    if result == Some(HotShotTaskCompleted) {
                        *collector = None;
                        // The protocol has finished
                        return None;
                    }
                }
            }
            HotShotEvent::ViewChange(view) => {
                let view = *view;
                if (*view != 0 || *self.cur_view > 0) && *self.cur_view >= *view {
                    return None;
                }

                if *view - *self.cur_view > 1 {
                    warn!("View changed by more than 1 going to view {:?}", view);
                }
                self.cur_view = view;

                // Inject view info into network
                let is_da = self
                    .da_membership
                    .get_whole_committee(self.cur_view + 1)
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
                    return None;
                }
                debug!("Polling for DA votes for view {}", *self.cur_view + 1);

                // Start polling for DA votes for the "next view"
                self.da_network
                    .inject_consensus_info(ConsensusIntentEvent::PollForVotes(*self.cur_view + 1))
                    .await;

                return None;
            }
            HotShotEvent::BlockRecv(encoded_transactions, metadata, view) => {
                let view = *view;
                self.da_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForTransactions(*view))
                    .await;

                // quick hash the encoded txns with sha256
                let encoded_transactions_hash = Sha256::digest(encoded_transactions);

                // sign the encoded transactions as opposed to the VID commitment
                let Ok(signature) =
                    TYPES::SignatureKey::sign(&self.private_key, &encoded_transactions_hash)
                else {
                    error!("Failed to sign block payload!");
                    return None;
                };

                let data: DAProposal<TYPES> = DAProposal {
                    encoded_transactions: encoded_transactions.clone(),
                    metadata: metadata.clone(),
                    // Upon entering a new view we want to send a DA Proposal for the next view -> Is it always the case that this is cur_view + 1?
                    view_number: view,
                };

                let message = Proposal {
                    data,
                    signature,
                    _pd: PhantomData,
                };

                broadcast_event(
                    Arc::new(HotShotEvent::DAProposalSend(
                        message.clone(),
                        self.public_key.clone(),
                    )),
                    &event_stream,
                )
                .await;
            }

            HotShotEvent::Timeout(view) => {
                self.da_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(**view))
                    .await;
            }

            HotShotEvent::Shutdown => {
                error!("Shutting down because of shutdown signal!");
                return Some(HotShotTaskCompleted);
            }
            _ => {
                error!("unexpected event {:?}", event);
            }
        }
        None
    }
}

/// task state implementation for DA Task
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static> TaskState
    for DATaskState<TYPES, I, A>
{
    type Event = Arc<HotShotEvent<TYPES>>;

    type Output = HotShotTaskCompleted;

    fn filter(&self, event: &Arc<HotShotEvent<TYPES>>) -> bool {
        !matches!(
            event.as_ref(),
            HotShotEvent::DAProposalRecv(_, _)
                | HotShotEvent::DAVoteRecv(_)
                | HotShotEvent::Shutdown
                | HotShotEvent::BlockRecv(_, _, _)
                | HotShotEvent::Timeout(_)
                | HotShotEvent::ViewChange(_)
                | HotShotEvent::DAProposalValidated(_, _)
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
