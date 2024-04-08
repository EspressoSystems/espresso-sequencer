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
use tracing::{debug, error, info, instrument, warn};

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
                info!("Received upgrade proposal: {:?}", proposal);

                // If the proposal does not match our upgrade target, we immediately exit.
                if !(self.should_vote)(proposal.data.upgrade_proposal.clone()) {
                    info!("Received unexpected upgrade proposal:\n{:?}", proposal.data);
                    return None;
                }

                // If we have an upgrade target, we validate that the proposal is relevant for the current view.
                info!(
                    "Upgrade proposal received for view: {:?}",
                    proposal.data.get_view_number()
                );

                let view = proposal.data.get_view_number();

                // At this point, we could choose to validate
                // that the proposal was issued by the correct leader
                // for the indiciated view.
                //
                // We choose not to, because we don't gain that much from it.
                // The certificate itself is only useful to the leader for that view anyway,
                // and from the node's perspective it doesn't matter who the sender is.
                // All we'd save is the cost of signing the vote, and we'd lose some flexibility.

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
                {
                    let view = vote.get_view_number();
                    if self.quorum_membership.get_leader(view) != self.public_key {
                        error!(
                            "We are not the leader for view {} are we leader for next view? {}",
                            *view,
                            self.quorum_membership.get_leader(view + 1) == self.public_key
                        );
                        return None;
                    }
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
            HotShotEvent::VersionUpgrade(version) => {
                error!("The network was upgraded to {:?}. This instance of HotShot did not expect an upgrade.", version);
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

                #[cfg(feature = "example-upgrade")]
                {
                    use committable::Committable;
                    use std::marker::PhantomData;

                    use hotshot_types::{
                        data::UpgradeProposal, message::Proposal,
                        traits::node_implementation::ConsensusTime,
                    };
                    use vbs::version::Version;

                    if *view == 5 && self.quorum_membership.get_leader(view + 5) == self.public_key
                    {
                        let upgrade_proposal_data = UpgradeProposalData {
                            old_version: Version { major: 0, minor: 1 },
                            new_version: Version { major: 1, minor: 0 },
                            new_version_hash: vec![1, 1, 0, 0, 1],
                            old_version_last_block: TYPES::Time::new(15),
                            new_version_first_block: TYPES::Time::new(18),
                        };

                        let upgrade_proposal = UpgradeProposal {
                            upgrade_proposal: upgrade_proposal_data.clone(),
                            view_number: view + 5,
                        };

                        let signature = TYPES::SignatureKey::sign(
                            &self.private_key,
                            upgrade_proposal_data.commit().as_ref(),
                        )
                        .expect("Failed to sign upgrade proposal commitment!");

                        let message = Proposal {
                            data: upgrade_proposal,
                            signature,
                            _pd: PhantomData,
                        };

                        broadcast_event(
                            Arc::new(HotShotEvent::UpgradeProposalSend(
                                message,
                                self.public_key.clone(),
                            )),
                            &tx,
                        )
                        .await;
                    }
                }

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
                | HotShotEvent::VersionUpgrade(_)
        )
    }
}
