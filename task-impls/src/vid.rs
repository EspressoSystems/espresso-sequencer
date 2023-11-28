use crate::{
    events::HotShotEvent,
    vote::{spawn_vote_accumulator, AccumulatorInfo},
};
use async_lock::RwLock;

use hotshot_task::{
    event_stream::{ChannelStream, EventStream},
    global_registry::GlobalRegistry,
    task::{HotShotTaskCompleted, TS},
    task_impls::HSTWithEvent,
};
use hotshot_types::traits::network::ConsensusIntentEvent;
use hotshot_types::{
    consensus::{Consensus, View},
    traits::{
        consensus_api::ConsensusApi,
        election::Membership,
        node_implementation::{NodeImplementation, NodeType},
        signature_key::SignatureKey,
        state::ConsensusTime,
    },
    utils::ViewInner,
};
use hotshot_types::{
    simple_vote::{VIDData, VIDVote},
    traits::network::CommunicationChannel,
    vote::HasViewNumber,
};

use snafu::Snafu;
use std::sync::Arc;
use tracing::{debug, error, instrument, warn};

#[derive(Snafu, Debug)]
/// Error type for consensus tasks
pub struct ConsensusTaskError {}

/// Tracks state of a VID task
pub struct VIDTaskState<
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
    /// Network for all nodes
    pub network: Arc<I::QuorumNetwork>,
    /// Membership for teh quorum
    pub membership: Arc<TYPES::Membership>,
    /// This Nodes Public Key
    pub public_key: TYPES::SignatureKey,
    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// The view and ID of the current vote collection task, if there is one.
    pub vote_collector: Option<(TYPES::Time, usize, usize)>,

    /// Global events stream to publish events
    pub event_stream: ChannelStream<HotShotEvent<TYPES>>,

    /// This state's ID
    pub id: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static>
    VIDTaskState<TYPES, I, A>
{
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "VID Main Task", level = "error")]
    pub async fn handle_event(
        &mut self,
        event: HotShotEvent<TYPES>,
    ) -> Option<HotShotTaskCompleted> {
        match event {
            HotShotEvent::VidVoteRecv(ref vote) => {
                let view = vote.get_view_number();
                if self.membership.get_leader(view) != self.public_key {
                    error!(
                        "We are not the VID leader for view {} are we leader for next view? {}",
                        *view,
                        self.membership.get_leader(view + 1) == self.public_key
                    );
                    return None;
                }
                let collection_view =
                    if let Some((collection_view, collection_id, _)) = &self.vote_collector {
                        // TODO: Is this correct for consecutive leaders?
                        if view > *collection_view {
                            // warn!("shutting down for view {:?}", collection_view);
                            self.registry.shutdown_task(*collection_id).await;
                        }
                        *collection_view
                    } else {
                        TYPES::Time::new(0)
                    };

                if view > collection_view {
                    debug!("Starting vote handle for view {:?}", vote.get_view_number());
                    let info = AccumulatorInfo {
                        public_key: self.public_key.clone(),
                        membership: self.membership.clone(),
                        view: vote.get_view_number(),
                        event_stream: self.event_stream.clone(),
                        id: self.id,
                        registry: self.registry.clone(),
                    };
                    let name = "Quorum Vote Collection";
                    self.vote_collector =
                        spawn_vote_accumulator(&info, vote.clone(), event, name.to_string()).await;
                } else if let Some((_, _, stream_id)) = self.vote_collector {
                    self.event_stream
                        .direct_message(stream_id, HotShotEvent::VidVoteRecv(vote.clone()))
                        .await;
                };
            }
            HotShotEvent::VidDisperseRecv(disperse, sender) => {
                // TODO copy-pasted from DAProposalRecv https://github.com/EspressoSystems/HotShot/issues/1690
                debug!(
                    "VID disperse received for view: {:?}",
                    disperse.data.get_view_number()
                );

                // stop polling for the received disperse
                self.network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVIDDisperse(
                        *disperse.data.view_number,
                    ))
                    .await;

                // ED NOTE: Assuming that the next view leader is the one who sends DA proposal for this view
                let view = disperse.data.get_view_number();

                // Allow VID disperse date that is one view older, in case we have updated the
                // view.
                // Adding `+ 1` on the LHS rather tahn `- 1` on the RHS, to avoid the overflow
                // error due to subtracting the genesis view number.
                if view + 1 < self.cur_view {
                    warn!("Throwing away VID disperse data that is more than one view older");
                    return None;
                }

                debug!("VID disperse data is fresh.");
                let payload_commitment = disperse.data.payload_commitment;

                // ED Is this the right leader?
                let view_leader_key = self.membership.get_leader(view);
                if view_leader_key != sender {
                    error!("VID proposal doesn't have expected leader key for view {} \n DA proposal is: [N/A for VID]", *view);
                    return None;
                }

                if !view_leader_key.validate(&disperse.signature, payload_commitment.as_ref()) {
                    error!("Could not verify VID proposal sig.");
                    return None;
                }

                if !self.membership.has_stake(&self.public_key) {
                    debug!(
                        "We were not chosen for consensus committee on {:?}",
                        self.cur_view
                    );
                    return None;
                }

                // Generate and send vote
                let vote = VIDVote::create_signed_vote(
                    VIDData {
                        payload_commit: payload_commitment,
                    },
                    view,
                    &self.public_key,
                    &self.private_key,
                );

                // ED Don't think this is necessary?
                // self.cur_view = view;

                debug!(
                    "Sending vote to the VID leader {:?}",
                    vote.get_view_number()
                );
                self.event_stream
                    .publish(HotShotEvent::VidVoteSend(vote))
                    .await;
                let mut consensus = self.consensus.write().await;

                // Ensure this view is in the view map for garbage collection, but do not overwrite if
                // there is already a view there: the replica task may have inserted a `Leaf` view which
                // contains strictly more information.
                consensus.state_map.entry(view).or_insert(View {
                    view_inner: ViewInner::DA {
                        block: payload_commitment,
                    },
                });

                // Record the block we have promised to make available.
                // TODO https://github.com/EspressoSystems/HotShot/issues/1692
                // consensus.saved_block_payloads.insert(proposal.data.block_payload);
            }
            HotShotEvent::VidCertRecv(cert) => {
                self.network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVIDCertificate(
                        *cert.view_number,
                    ))
                    .await;
            }
            HotShotEvent::ViewChange(view) => {
                if *self.cur_view >= *view {
                    return None;
                }

                if *view - *self.cur_view > 1 {
                    error!("View changed by more than 1 going to view {:?}", view);
                }
                self.cur_view = view;

                // Start polling for VID disperse for the new view
                self.network
                    .inject_consensus_info(ConsensusIntentEvent::PollForVIDDisperse(
                        *self.cur_view + 1,
                    ))
                    .await;

                self.network
                    .inject_consensus_info(ConsensusIntentEvent::PollForVIDCertificate(
                        *self.cur_view + 1,
                    ))
                    .await;

                // If we are not the next leader, we should exit
                if self.membership.get_leader(self.cur_view + 1) != self.public_key {
                    // panic!("We are not the DA leader for view {}", *self.cur_view + 1);
                    return None;
                }

                // Start polling for VID votes for the "next view"
                self.network
                    .inject_consensus_info(ConsensusIntentEvent::PollForVIDVotes(
                        *self.cur_view + 1,
                    ))
                    .await;

                return None;
            }

            HotShotEvent::Shutdown => {
                return Some(HotShotTaskCompleted::ShutDown);
            }
            _ => {
                error!("unexpected event {:?}", event);
            }
        }
        None
    }

    /// Filter the VID event.
    pub fn filter(event: &HotShotEvent<TYPES>) -> bool {
        matches!(
            event,
            HotShotEvent::Shutdown
                | HotShotEvent::VidDisperseRecv(_, _)
                | HotShotEvent::VidVoteRecv(_)
                | HotShotEvent::VidCertRecv(_)
                | HotShotEvent::ViewChange(_)
        )
    }
}

/// task state implementation for VID Task
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static> TS
    for VIDTaskState<TYPES, I, A>
{
}

/// Type alias for VID Task Types
pub type VIDTaskTypes<TYPES, I, A> = HSTWithEvent<
    ConsensusTaskError,
    HotShotEvent<TYPES>,
    ChannelStream<HotShotEvent<TYPES>>,
    VIDTaskState<TYPES, I, A>,
>;
