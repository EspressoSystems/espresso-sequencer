use std::{marker::PhantomData, sync::Arc};

use anyhow::Result;
use async_broadcast::{Receiver, Sender};
use async_lock::RwLock;
use async_trait::async_trait;
use committable::Committable;
use hotshot_task::task::TaskState;
use hotshot_types::{
    constants::{Base, Upgrade, UPGRADE_HASH},
    data::UpgradeProposal,
    event::{Event, EventType},
    message::Proposal,
    simple_certificate::UpgradeCertificate,
    simple_vote::{UpgradeProposalData, UpgradeVote},
    traits::{
        election::Membership,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::SignatureKey,
    },
    vote::HasViewNumber,
};
use tracing::{debug, error, info, instrument, warn};
use vbs::version::StaticVersionType;

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
pub struct UpgradeTaskState<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Output events to application
    pub output_event_stream: async_broadcast::Sender<Event<TYPES>>,

    /// View number this view is executing in.
    pub cur_view: TYPES::Time,

    /// Membership for Quorum Certs/votes
    pub quorum_membership: Arc<TYPES::Membership>,
    /// Network for all nodes
    pub quorum_network: Arc<I::QuorumNetwork>,

    /// The current vote collection task, if there is one.
    pub vote_collector:
        RwLock<VoteCollectorOption<TYPES, UpgradeVote<TYPES>, UpgradeCertificate<TYPES>>>,

    /// This Nodes public key
    pub public_key: TYPES::SignatureKey,

    /// This Nodes private key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,

    /// This state's ID
    pub id: u64,

    /// View to start proposing an upgrade
    pub start_proposing_view: u64,

    /// View to stop proposing an upgrade
    pub stop_proposing_view: u64,

    /// View to start voting on an upgrade
    pub start_voting_view: u64,

    /// View to stop voting on an upgrade
    pub stop_voting_view: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> UpgradeTaskState<TYPES, I> {
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

                if *proposal.data.view_number() < self.start_voting_view
                    || *proposal.data.view_number() >= self.stop_voting_view
                {
                    return None;
                }

                // If the proposal does not match our upgrade target, we immediately exit.
                if proposal.data.upgrade_proposal.new_version_hash != UPGRADE_HASH
                    || proposal.data.upgrade_proposal.old_version != Base::VERSION
                    || proposal.data.upgrade_proposal.new_version != Upgrade::VERSION
                {
                    return None;
                }

                // If we have an upgrade target, we validate that the proposal is relevant for the current view.
                info!(
                    "Upgrade proposal received for view: {:?}",
                    proposal.data.view_number()
                );

                let view = proposal.data.view_number();

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
                let view_leader_key = self.quorum_membership.leader(view);
                if &view_leader_key != sender {
                    error!("Upgrade proposal doesn't have expected leader key for view {} \n Upgrade proposal is: {:?}", *view, proposal.data.clone());
                    return None;
                }

                // At this point, we've checked that:
                //   * the proposal was expected,
                //   * the proposal is valid, and
                // so we notify the application layer
                broadcast_event(
                    Event {
                        view_number: self.cur_view,
                        event: EventType::UpgradeProposal {
                            proposal: proposal.clone(),
                            sender: sender.clone(),
                        },
                    },
                    &self.output_event_stream,
                )
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
                debug!("Sending upgrade vote {:?}", vote.view_number());
                broadcast_event(Arc::new(HotShotEvent::UpgradeVoteSend(vote)), &tx).await;
            }
            HotShotEvent::UpgradeVoteRecv(ref vote) => {
                debug!("Upgrade vote recv, Main Task {:?}", vote.view_number());

                // Check if we are the leader.
                {
                    let view = vote.view_number();
                    if self.quorum_membership.leader(view) != self.public_key {
                        error!(
                            "We are not the leader for view {} are we leader for next view? {}",
                            *view,
                            self.quorum_membership.leader(view + 1) == self.public_key
                        );
                        return None;
                    }
                }

                let mut collector = self.vote_collector.write().await;

                if collector.is_none() || vote.view_number() > collector.as_ref().unwrap().view {
                    debug!("Starting vote handle for view {:?}", vote.view_number());
                    let info = AccumulatorInfo {
                        public_key: self.public_key.clone(),
                        membership: Arc::clone(&self.quorum_membership),
                        view: vote.view_number(),
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
                        .handle_vote_event(Arc::clone(&event), &tx)
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
                // We try to form a certificate 5 views before we're leader.
                if *view >= self.start_proposing_view
                    && *view < self.stop_proposing_view
                    && self.quorum_membership.leader(view + 5) == self.public_key
                {
                    let upgrade_proposal_data = UpgradeProposalData {
                        old_version: Base::VERSION,
                        new_version: Upgrade::VERSION,
                        new_version_hash: UPGRADE_HASH.to_vec(),
                        // We schedule the upgrade to begin 15 views in the future
                        old_version_last_view: TYPES::Time::new(*view + 15),
                        // and end 20 views in the future
                        new_version_first_view: TYPES::Time::new(*view + 20),
                        decide_by: TYPES::Time::new(*view + 10),
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

                return None;
            }
            HotShotEvent::Shutdown => {
                error!("Shutting down because of shutdown signal!");
                return Some(HotShotTaskCompleted);
            }
            _ => {}
        }
        None
    }
}

#[async_trait]
/// task state implementation for the upgrade task
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> TaskState for UpgradeTaskState<TYPES, I> {
    type Event = HotShotEvent<TYPES>;

    async fn handle_event(
        &mut self,
        event: Arc<Self::Event>,
        sender: &Sender<Arc<Self::Event>>,
        _receiver: &Receiver<Arc<Self::Event>>,
    ) -> Result<()> {
        self.handle(event, sender.clone()).await;

        Ok(())
    }

    async fn cancel_subtasks(&mut self) {}
}
