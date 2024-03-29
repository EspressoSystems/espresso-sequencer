use crate::{
    events::{HotShotEvent, HotShotTaskCompleted},
    helpers::broadcast_event,
    vote_collection::{create_vote_accumulator, AccumulatorInfo, VoteCollectionTaskState},
};
use async_broadcast::Sender;
use async_lock::RwLock;

use hotshot_task::task::TaskState;
use hotshot_types::{
    event::{Event, EventType},
    simple_certificate::UpgradeCertificate,
    simple_vote::{UpgradeProposalData, UpgradeVote},
    traits::{
        consensus_api::ConsensusApi,
        election::Membership,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::SignatureKey,
    },
    vote::HasViewNumber,
};

use crate::vote_collection::HandleVoteEvent;
use std::sync::Arc;
use tracing::{debug, error, instrument, warn};

/// Alias for Optional type for Vote Collectors
type VoteCollectorOption<TYPES, VOTE, CERT> = Option<VoteCollectionTaskState<TYPES, VOTE, CERT>>;

/// Tracks state of a DA task
pub struct UpgradeTaskState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    A: ConsensusApi<TYPES, I> + 'static,
> {
    /// The state's api
    pub api: A,
    /// View number this view is executing in.
    pub cur_view: TYPES::Time,

    /// Membership for Quorum Certs/votes
    pub quorum_membership: Arc<TYPES::Membership>,
    /// Network for all nodes
    pub quorum_network: Arc<I::QuorumNetwork>,

    /// Whether we should vote affirmatively on a given upgrade proposal (true) or not (false)
    pub should_vote: fn(UpgradeProposalData<TYPES>) -> bool,

    /// The current vote collection task, if there is one.
    pub vote_collector:
        RwLock<VoteCollectorOption<TYPES, UpgradeVote<TYPES>, UpgradeCertificate<TYPES>>>,

    /// This Nodes public key
    pub public_key: TYPES::SignatureKey,

    /// This Nodes private key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,

    /// This state's ID
    pub id: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static>
    UpgradeTaskState<TYPES, I, A>
{
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Upgrade Task", level = "error")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        tx: Sender<Arc<HotShotEvent<TYPES>>>,
    ) -> Option<HotShotTaskCompleted> {
        match event.as_ref() {
            HotShotEvent::UpgradeProposalRecv(proposal, sender) => {
                let should_vote = self.should_vote;
                // If the proposal does not match our upgrade target, we immediately exit.
                if !should_vote(proposal.data.upgrade_proposal.clone()) {
                    warn!("Received unexpected upgrade proposal:\n{:?}", proposal.data);
                    return None;
                }

                // If we have an upgrade target, we validate that the proposal is relevant for the current view.

                debug!(
                    "Upgrade proposal received for view: {:?}",
                    proposal.data.get_view_number()
                );
                // NOTE: Assuming that the next view leader is the one who sends an upgrade proposal for this view
                let view = proposal.data.get_view_number();

                // Allow an upgrade proposal that is one view older, in case we have voted on a quorum
                // proposal and updated the view.
                // `self.cur_view` should be at least 1 since there is a view change before getting
                // the `UpgradeProposalRecv` event. Otherwise, the view number subtraction below will
                // cause an overflow error.
                // TODO Come back to this - we probably don't need this, but we should also never receive a UpgradeCertificate where this fails, investigate block ready so it doesn't make one for the genesis block

                if self.cur_view != TYPES::Time::genesis() && view < self.cur_view - 1 {
                    warn!("Discarding old upgrade proposal; the proposal is for view {:?}, but the current view is {:?}.",
                      view,
                      self.cur_view
                    );
                    return None;
                }

                // We then validate that the proposal was issued by the leader for the view.
                let view_leader_key = self.quorum_membership.get_leader(view);
                if &view_leader_key != sender {
                    error!("Upgrade proposal doesn't have expected leader key for view {} \n Upgrade proposal is: {:?}", *view, proposal.data.clone());
                    return None;
                }

                // At this point, we've checked that:
                //   * the proposal was expected,
                //   * the proposal is valid, and
                //   * the proposal is recent,
                // so we notify the application layer
                self.api
                    .send_event(Event {
                        view_number: self.cur_view,
                        event: EventType::UpgradeProposal {
                            proposal: proposal.clone(),
                            sender: sender.clone(),
                        },
                    })
                    .await;

                // If everything is fine up to here, we generate and send a vote on the proposal.
                let Ok(vote) = UpgradeVote::create_signed_vote(
                    proposal.data.upgrade_proposal.clone(),
                    view,
                    &self.public_key,
                    &self.private_key,
                ) else {
                    error!("Failed to sign UpgradeVote!");
                    return None;
                };
                debug!("Sending upgrade vote {:?}", vote.get_view_number());
                broadcast_event(Arc::new(HotShotEvent::UpgradeVoteSend(vote)), &tx).await;
            }
            HotShotEvent::UpgradeVoteRecv(ref vote) => {
                debug!("Upgrade vote recv, Main Task {:?}", vote.get_view_number());
                // Check if we are the leader.
                let view = vote.get_view_number();
                if self.quorum_membership.get_leader(view) != self.public_key {
                    error!(
                        "We are not the leader for view {} are we leader for next view? {}",
                        *view,
                        self.quorum_membership.get_leader(view + 1) == self.public_key
                    );
                    return None;
                }
                let mut collector = self.vote_collector.write().await;

                if collector.is_none() || vote.get_view_number() > collector.as_ref().unwrap().view
                {
                    debug!("Starting vote handle for view {:?}", vote.get_view_number());
                    let info = AccumulatorInfo {
                        public_key: self.public_key.clone(),
                        membership: self.quorum_membership.clone(),
                        view: vote.get_view_number(),
                        id: self.id,
                    };
                    *collector = create_vote_accumulator::<
                        TYPES,
                        UpgradeVote<TYPES>,
                        UpgradeCertificate<TYPES>,
                    >(&info, vote.clone(), event, &tx)
                    .await;
                } else {
                    let result = collector
                        .as_mut()
                        .unwrap()
                        .handle_event(event.clone(), &tx)
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
                if *self.cur_view >= *view {
                    return None;
                }

                if *view - *self.cur_view > 1 {
                    warn!("View changed by more than 1 going to view {:?}", view);
                }
                self.cur_view = view;

                return None;
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

/// task state implementation for the upgrade task
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static> TaskState
    for UpgradeTaskState<TYPES, I, A>
{
    type Event = Arc<HotShotEvent<TYPES>>;

    type Output = HotShotTaskCompleted;

    async fn handle_event(
        event: Self::Event,
        task: &mut hotshot_task::task::Task<Self>,
    ) -> Option<Self::Output> {
        let sender = task.clone_sender();
        tracing::trace!("sender queue len {}", sender.len());
        task.state_mut().handle(event, sender).await
    }

    fn should_shutdown(event: &Self::Event) -> bool {
        matches!(event.as_ref(), HotShotEvent::Shutdown)
    }

    fn filter(&self, event: &Self::Event) -> bool {
        !matches!(
            event.as_ref(),
            HotShotEvent::UpgradeProposalRecv(_, _)
                | HotShotEvent::UpgradeVoteRecv(_)
                | HotShotEvent::Shutdown
                | HotShotEvent::ViewChange(_)
        )
    }
}
