use std::{marker::PhantomData, sync::Arc};

use async_broadcast::Sender;
use async_compatibility_layer::art::async_spawn;
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::spawn_blocking;
use hotshot_task::task::{Task, TaskState};
use hotshot_types::{
    consensus::{Consensus, View},
    data::DaProposal,
    event::{Event, EventType},
    message::Proposal,
    simple_certificate::DaCertificate,
    simple_vote::{DaData, DaVote},
    traits::{
        block_contents::vid_commitment,
        consensus_api::ConsensusApi,
        election::Membership,
        network::ConnectedNetwork,
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
pub struct DaTaskState<
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
    pub da_network: Arc<I::DaNetwork>,

    /// The current vote collection task, if there is one.
    pub vote_collector: RwLock<VoteCollectorOption<TYPES, DaVote<TYPES>, DaCertificate<TYPES>>>,

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
    DaTaskState<TYPES, I, A>
{
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "DA Main Task", level = "error")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    ) -> Option<HotShotTaskCompleted> {
        match event.as_ref() {
            HotShotEvent::DaProposalRecv(proposal, sender) => {
                let sender = sender.clone();
                debug!(
                    "DA proposal received for view: {:?}",
                    proposal.data.view_number()
                );
                // ED NOTE: Assuming that the next view leader is the one who sends DA proposal for this view
                let view = proposal.data.view_number();

                // Allow a DA proposal that is one view older, in case we have voted on a quorum
                // proposal and updated the view.
                // `self.cur_view` should be at least 1 since there is a view change before getting
                // the `DaProposalRecv` event. Otherwise, the view number subtraction below will
                // cause an overflow error.
                // TODO ED Come back to this - we probably don't need this, but we should also never receive a DAC where this fails, investigate block ready so it doesn't make one for the genesis block
                if self.cur_view != TYPES::Time::genesis() && view < self.cur_view - 1 {
                    warn!("Throwing away DA proposal that is more than one view older");
                    return None;
                }

                if self
                    .consensus
                    .read()
                    .await
                    .saved_payloads()
                    .contains_key(&view)
                {
                    warn!("Received DA proposal for view {:?} but we already have a payload for that view.  Throwing it away", view);
                    return None;
                }

                let encoded_transactions_hash = Sha256::digest(&proposal.data.encoded_transactions);

                // ED Is this the right leader?
                let view_leader_key = self.da_membership.leader(view);
                if view_leader_key != sender {
                    error!("DA proposal doesn't have expected leader key for view {} \n DA proposal is: {:?}", *view, proposal.data.clone());
                    return None;
                }

                if !view_leader_key.validate(&proposal.signature, &encoded_transactions_hash) {
                    error!("Could not verify proposal.");
                    return None;
                }

                broadcast_event(
                    Arc::new(HotShotEvent::DaProposalValidated(proposal.clone(), sender)),
                    &event_stream,
                )
                .await;
            }
            HotShotEvent::DaProposalValidated(proposal, sender) => {
                let curr_view = self.consensus.read().await.cur_view();
                if curr_view > proposal.data.view_number() + 1 {
                    tracing::debug!("Validated DA proposal for prior view but it's too old now Current view {:?}, DA Proposal view {:?}", curr_view, proposal.data.view_number());
                    return None;
                }
                // Proposal is fresh and valid, notify the application layer
                self.api
                    .send_event(Event {
                        view_number: self.cur_view,
                        event: EventType::DaProposal {
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
                let txns = Arc::clone(&proposal.data.encoded_transactions);
                let num_nodes = self.quorum_membership.total_nodes();
                let payload_commitment =
                    spawn_blocking(move || vid_commitment(&txns, num_nodes)).await;
                #[cfg(async_executor_impl = "tokio")]
                let payload_commitment = payload_commitment.unwrap();

                let view = proposal.data.view_number();
                // Generate and send vote
                let Ok(vote) = DaVote::create_signed_vote(
                    DaData {
                        payload_commit: payload_commitment,
                    },
                    view,
                    &self.public_key,
                    &self.private_key,
                ) else {
                    error!("Failed to sign DA Vote!");
                    return None;
                };

                debug!("Sending vote to the DA leader {:?}", vote.view_number());

                broadcast_event(Arc::new(HotShotEvent::DaVoteSend(vote)), &event_stream).await;
                let mut consensus = self.consensus.write().await;

                // Ensure this view is in the view map for garbage collection, but do not overwrite if
                // there is already a view there: the replica task may have inserted a `Leaf` view which
                // contains strictly more information.
                if !consensus.validated_state_map().contains_key(&view) {
                    consensus.update_validated_state_map(
                        view,
                        View {
                            view_inner: ViewInner::Da { payload_commitment },
                        },
                    );
                }

                // Record the payload we have promised to make available.
                if let Err(e) = consensus
                    .update_saved_payloads(view, Arc::clone(&proposal.data.encoded_transactions))
                {
                    tracing::trace!("{e:?}");
                }
                // Optimistically calculate and update VID if we know that the primary network is down.
                if self.da_network.is_primary_down() {
                    let consensus = Arc::clone(&self.consensus);
                    let membership = Arc::clone(&self.quorum_membership);
                    let pk = self.private_key.clone();
                    async_spawn(async move {
                        consensus
                            .write()
                            .await
                            .calculate_and_update_vid(view, membership, &pk)
                            .await;
                    });
                }
            }
            HotShotEvent::DaVoteRecv(ref vote) => {
                debug!("DA vote recv, Main Task {:?}", vote.view_number());
                // Check if we are the leader and the vote is from the sender.
                let view = vote.view_number();
                if self.da_membership.leader(view) != self.public_key {
                    error!("We are not the DA committee leader for view {} are we leader for next view? {}", *view, self.da_membership.leader(view + 1) == self.public_key);
                    return None;
                }
                let mut collector = self.vote_collector.write().await;

                if collector.is_none() || vote.view_number() > collector.as_ref().unwrap().view {
                    debug!("Starting vote handle for view {:?}", vote.view_number());
                    let info = AccumulatorInfo {
                        public_key: self.public_key.clone(),
                        membership: Arc::clone(&self.da_membership),
                        view: vote.view_number(),
                        id: self.id,
                    };
                    *collector = create_vote_accumulator::<
                        TYPES,
                        DaVote<TYPES>,
                        DaCertificate<TYPES>,
                    >(&info, vote.clone(), event, &event_stream)
                    .await;
                } else {
                    let result = collector
                        .as_mut()
                        .unwrap()
                        .handle_event(Arc::clone(&event), &event_stream)
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

                // If we are not the next leader (DA leader for this view) immediately exit
                if self.da_membership.leader(self.cur_view + 1) != self.public_key {
                    return None;
                }
                debug!("Polling for DA votes for view {}", *self.cur_view + 1);

                return None;
            }
            HotShotEvent::BlockRecv(encoded_transactions, metadata, view, _fee, _vid_precomp) => {
                let view = *view;

                // quick hash the encoded txns with sha256
                let encoded_transactions_hash = Sha256::digest(encoded_transactions);

                // sign the encoded transactions as opposed to the VID commitment
                let Ok(signature) =
                    TYPES::SignatureKey::sign(&self.private_key, &encoded_transactions_hash)
                else {
                    error!("Failed to sign block payload!");
                    return None;
                };

                let data: DaProposal<TYPES> = DaProposal {
                    encoded_transactions: Arc::clone(encoded_transactions),
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
                    Arc::new(HotShotEvent::DaProposalSend(
                        message.clone(),
                        self.public_key.clone(),
                    )),
                    &event_stream,
                )
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
    for DaTaskState<TYPES, I, A>
{
    type Event = Arc<HotShotEvent<TYPES>>;

    type Output = HotShotTaskCompleted;

    fn filter(&self, event: &Arc<HotShotEvent<TYPES>>) -> bool {
        !matches!(
            event.as_ref(),
            HotShotEvent::DaProposalRecv(_, _)
                | HotShotEvent::DaVoteRecv(_)
                | HotShotEvent::Shutdown
                | HotShotEvent::BlockRecv(_, _, _, _, _)
                | HotShotEvent::ViewChange(_)
                | HotShotEvent::DaProposalValidated(_, _)
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
