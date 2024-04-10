use std::{collections::HashMap, sync::Arc};

use async_broadcast::{Receiver, Sender};
use async_lock::{RwLock, RwLockUpgradableReadGuard};
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use committable::Committable;
use hotshot_task::{
    dependency::{AndDependency, EventDependency},
    dependency_task::{DependencyTask, HandleDepOutput},
    task::{Task, TaskState},
};
use hotshot_types::{
    consensus::Consensus,
    data::Leaf,
    event::{Event, EventType},
    message::GeneralConsensusMessage,
    simple_vote::{QuorumData, QuorumVote},
    traits::{
        block_contents::BlockHeader,
        election::Membership,
        network::{ConnectedNetwork, ConsensusIntentEvent},
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::SignatureKey,
        storage::Storage,
        ValidatedState,
    },
    utils::{Terminator, View, ViewInner},
    vote::{Certificate, HasViewNumber},
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, error, instrument, warn};

use crate::{
    events::HotShotEvent,
    helpers::{broadcast_event, cancel_task},
};

/// Vote dependency types.
#[derive(Debug, PartialEq)]
enum VoteDependency {
    /// For the `QuorumProposalRecv` event.
    QuorumProposal,
    /// For the `DACertificateRecv` event.
    Dac,
    /// For the `VIDShareRecv` event.
    Vid,
}

/// Handler for the vote dependency.
struct VoteDependencyHandle<TYPES: NodeType> {
    /// Public key.
    pub public_key: TYPES::SignatureKey,
    /// Private Key.
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// View number to vote on.
    view_number: TYPES::Time,
    /// Event sender.
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
}
impl<TYPES: NodeType> HandleDepOutput for VoteDependencyHandle<TYPES> {
    type Output = Vec<Arc<HotShotEvent<TYPES>>>;
    async fn handle_dep_result(self, res: Self::Output) {
        let mut payload_commitment = None;
        let mut leaf = None;
        for event in res {
            match event.as_ref() {
                HotShotEvent::QuorumProposalValidated(proposal, parent_leaf) => {
                    let proposal_payload_comm = proposal.block_header.payload_commitment();
                    if let Some(comm) = payload_commitment {
                        if proposal_payload_comm != comm {
                            error!("Quorum proposal has inconsistent payload commitment with DAC or VID.");
                            return;
                        }
                    } else {
                        payload_commitment = Some(proposal_payload_comm);
                    }
                    let parent_commitment = parent_leaf.commit();
                    let proposed_leaf = Leaf::from_quorum_proposal(proposal);
                    if proposed_leaf.get_parent_commitment() != parent_commitment {
                        return;
                    }
                    leaf = Some(proposed_leaf);
                }
                HotShotEvent::DACertificateValidated(cert) => {
                    let cert_payload_comm = cert.get_data().payload_commit;
                    if let Some(comm) = payload_commitment {
                        if cert_payload_comm != comm {
                            error!("DAC has inconsistent payload commitment with quorum proposal or VID.");
                            return;
                        }
                    } else {
                        payload_commitment = Some(cert_payload_comm);
                    }
                }
                HotShotEvent::VIDShareValidated(share) => {
                    let vid_payload_commitment = share.payload_commitment;
                    if let Some(comm) = payload_commitment {
                        if vid_payload_commitment != comm {
                            error!("VID has inconsistent payload commitment with quorum proposal or DAC.");
                            return;
                        }
                    } else {
                        payload_commitment = Some(vid_payload_commitment);
                    }
                }
                _ => {}
            }
        }
        broadcast_event(
            Arc::new(HotShotEvent::QuorumVoteDependenciesValidated(
                self.view_number,
            )),
            &self.sender,
        )
        .await;

        // Create and send the vote.
        let Some(leaf) = leaf else {
            error!("Quorum proposal isn't validated, but it should be.");
            return;
        };
        let message = if let Ok(vote) = QuorumVote::<TYPES>::create_signed_vote(
            QuorumData {
                leaf_commit: leaf.commit(),
            },
            self.view_number,
            &self.public_key,
            &self.private_key,
        ) {
            GeneralConsensusMessage::<TYPES>::Vote(vote)
        } else {
            error!("Unable to sign quorum vote!");
            return;
        };
        if let GeneralConsensusMessage::Vote(vote) = message {
            debug!(
                "Sending vote to next quorum leader {:?}",
                vote.get_view_number() + 1
            );
            broadcast_event(Arc::new(HotShotEvent::QuorumVoteSend(vote)), &self.sender).await;
        }
    }
}

/// The state for the quorum vote task.
///
/// Contains all of the information for the quorum vote.
pub struct QuorumVoteTaskState<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Public key.
    pub public_key: TYPES::SignatureKey,

    /// Private Key.
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,

    /// Reference to consensus. The replica will require a write lock on this.
    pub consensus: Arc<RwLock<Consensus<TYPES>>>,

    /// Latest view number that has been voted for.
    pub latest_voted_view: TYPES::Time,

    /// Table for the in-progress dependency tasks.
    pub vote_dependencies: HashMap<TYPES::Time, JoinHandle<()>>,

    /// Network for all nodes
    pub quorum_network: Arc<I::QuorumNetwork>,

    /// Network for DA committee
    pub committee_network: Arc<I::CommitteeNetwork>,

    /// Membership for Quorum certs/votes.
    pub quorum_membership: Arc<TYPES::Membership>,

    /// Membership for DA committee certs/votes.
    pub da_membership: Arc<TYPES::Membership>,

    /// Output events to application
    pub output_event_stream: async_broadcast::Sender<Event<TYPES>>,

    /// The node's id
    pub id: u64,

    /// Reference to the storage.
    pub storage: Arc<RwLock<I::Storage>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> QuorumVoteTaskState<TYPES, I> {
    /// Create an event dependency.
    #[instrument(skip_all, fields(id = self.id, latest_voted_view = *self.latest_voted_view), name = "Quorum vote create event dependency", level = "error")]
    fn create_event_dependency(
        &self,
        dependency_type: VoteDependency,
        view_number: TYPES::Time,
        event_receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
    ) -> EventDependency<Arc<HotShotEvent<TYPES>>> {
        EventDependency::new(
            event_receiver.clone(),
            Box::new(move |event| {
                let event = event.as_ref();
                let event_view = match dependency_type {
                    VoteDependency::QuorumProposal => {
                        if let HotShotEvent::QuorumProposalValidated(proposal, _) = event {
                            proposal.view_number
                        } else {
                            return false;
                        }
                    }
                    VoteDependency::Dac => {
                        if let HotShotEvent::DACertificateValidated(cert) = event {
                            cert.view_number
                        } else {
                            return false;
                        }
                    }
                    VoteDependency::Vid => {
                        if let HotShotEvent::VIDShareValidated(disperse) = event {
                            disperse.view_number
                        } else {
                            return false;
                        }
                    }
                };
                if event_view == view_number {
                    debug!("Vote dependency {:?} completed", dependency_type);
                    return true;
                }
                false
            }),
        )
    }

    /// Create and store an [`AndDependency`] combining [`EventDependency`]s associated with the
    /// given view number if it doesn't exist.
    #[instrument(skip_all, fields(id = self.id, latest_voted_view = *self.latest_voted_view), name = "Quorum vote crete dependency task if new", level = "error")]
    fn create_dependency_task_if_new(
        &mut self,
        view_number: TYPES::Time,
        event_receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
        event_sender: &Sender<Arc<HotShotEvent<TYPES>>>,
    ) {
        if self.vote_dependencies.get(&view_number).is_some() {
            return;
        }
        let deps = vec![
            self.create_event_dependency(
                VoteDependency::QuorumProposal,
                view_number,
                event_receiver.clone(),
            ),
            self.create_event_dependency(VoteDependency::Dac, view_number, event_receiver.clone()),
            self.create_event_dependency(VoteDependency::Vid, view_number, event_receiver),
        ];
        let vote_dependency = AndDependency::from_deps(deps);
        let dependency_task = DependencyTask::new(
            vote_dependency,
            VoteDependencyHandle {
                public_key: self.public_key.clone(),
                private_key: self.private_key.clone(),
                view_number,
                sender: event_sender.clone(),
            },
        );
        self.vote_dependencies
            .insert(view_number, dependency_task.run());
    }

    /// Update the latest voted view number.
    #[instrument(skip_all, fields(id = self.id, latest_voted_view = *self.latest_voted_view), name = "Quorum vote update latest voted view", level = "error")]
    async fn update_latest_voted_view(&mut self, new_view: TYPES::Time) -> bool {
        if *self.latest_voted_view < *new_view {
            debug!(
                "Updating next vote view from {} to {} in the quorum vote task",
                *self.latest_voted_view, *new_view
            );

            // Cancel the old dependency tasks.
            for view in (*self.latest_voted_view + 1)..=(*new_view) {
                if let Some(dependency) = self.vote_dependencies.remove(&TYPES::Time::new(view)) {
                    cancel_task(dependency).await;
                    debug!("Vote dependency removed for view {:?}", view);
                }
            }

            self.latest_voted_view = new_view;

            return true;
        }
        false
    }

    /// Handle a vote dependent event received on the event stream
    #[instrument(skip_all, fields(id = self.id, latest_voted_view = *self.latest_voted_view), name = "Quorum vote handle", level = "error")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        event_receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
        event_sender: Sender<Arc<HotShotEvent<TYPES>>>,
    ) {
        match event.as_ref() {
            HotShotEvent::QuorumProposalRecv(proposal, sender) => {
                let view = proposal.data.view_number;
                debug!("Received Quorum Proposal for view {}", *view);
                if view <= self.latest_voted_view {
                    return;
                }

                // TODO (Keyao) Add validations for view change evidence and upgrade cert.

                // Vaildate the justify QC.
                let justify_qc = proposal.data.justify_qc.clone();
                if !justify_qc.is_valid_cert(self.quorum_membership.as_ref()) {
                    error!("Invalid justify_qc in proposal for view {}", *view);
                    let consensus = self.consensus.write().await;
                    consensus.metrics.invalid_qc.update(1);
                    return;
                }
                broadcast_event(Arc::new(HotShotEvent::ViewChange(view + 1)), &event_sender).await;

                let consensus = self.consensus.upgradable_read().await;
                // Get the parent leaf and state.
                let parent = {
                    match consensus
                        .saved_leaves
                        .get(&justify_qc.get_data().leaf_commit)
                        .cloned()
                    {
                        Some(leaf) => {
                            if let (Some(state), _) =
                                consensus.get_state_and_delta(leaf.get_view_number())
                            {
                                Some((leaf, state.clone()))
                            } else {
                                error!("Parent state not found! Consensus internally inconsistent");
                                return;
                            }
                        }
                        None => None,
                    }
                };
                let mut consensus = RwLockUpgradableReadGuard::upgrade(consensus).await;
                if justify_qc.get_view_number() > consensus.high_qc.view_number {
                    debug!("Updating high QC");

                    if let Err(e) = self
                        .storage
                        .write()
                        .await
                        .update_high_qc(justify_qc.clone())
                        .await
                    {
                        warn!("Failed to store High QC not voting. Error: {:?}", e);
                        return;
                    }

                    consensus.high_qc = justify_qc.clone();
                }
                // Justify qc's leaf commitment is not the same as the parent's leaf commitment, but it should be (in this case)
                let Some((parent_leaf, parent_state)) = parent else {
                    warn!(
                        "Proposal's parent missing from storage with commitment: {:?}, proposal view {:?}",
                        justify_qc.get_data().leaf_commit,
                        *view,
                    );

                    let leaf = Leaf::from_quorum_proposal(&proposal.data);

                    let state = Arc::new(
                        <TYPES::ValidatedState as ValidatedState<TYPES>>::from_header(
                            &proposal.data.block_header,
                        ),
                    );

                    consensus.validated_state_map.insert(
                        view,
                        View {
                            view_inner: ViewInner::Leaf {
                                leaf: leaf.commit(),
                                state,
                                delta: None,
                            },
                        },
                    );
                    consensus.saved_leaves.insert(leaf.commit(), leaf.clone());

                    if let Err(e) = self
                        .storage
                        .write()
                        .await
                        .update_undecided_state(
                            consensus.saved_leaves.clone(),
                            consensus.validated_state_map.clone(),
                        )
                        .await
                    {
                        warn!("Couldn't store undecided state.  Error: {:?}", e);
                    }
                    drop(consensus);

                    return;
                };

                drop(consensus);

                // Validate the state.
                let Ok((validated_state, state_delta)) = parent_state
                    .validate_and_apply_header(
                        &self.consensus.read().await.instance_state,
                        &parent_leaf,
                        &proposal.data.block_header.clone(),
                    )
                    .await
                else {
                    error!("Block header doesn't extend the proposal",);
                    return;
                };
                let state = Arc::new(validated_state);
                let delta = Arc::new(state_delta);
                let parent_commitment = parent_leaf.commit();
                let view = proposal.data.get_view_number();
                let proposed_leaf = Leaf::from_quorum_proposal(&proposal.data);
                if proposed_leaf.get_parent_commitment() != parent_commitment {
                    return;
                }

                // Validate the signature. This should also catch if `leaf_commitment`` does not
                // equal our calculated parent commitment.
                let view_leader_key = self.quorum_membership.get_leader(view);
                if view_leader_key != *sender {
                    warn!("Leader key does not match key in proposal");
                    return;
                }
                if !view_leader_key.validate(&proposal.signature, proposed_leaf.commit().as_ref()) {
                    error!(?proposal.signature, "Could not verify proposal.");
                    return;
                }

                // Liveness and safety checks.
                let consensus = self.consensus.upgradable_read().await;
                let liveness_check = justify_qc.get_view_number() > consensus.locked_view;
                // Check if proposal extends from the locked leaf.
                let outcome = consensus.visit_leaf_ancestors(
                    justify_qc.get_view_number(),
                    Terminator::Inclusive(consensus.locked_view),
                    false,
                    |leaf, _, _| {
                        // if leaf view no == locked view no then we're done, report success by
                        // returning true
                        leaf.get_view_number() != consensus.locked_view
                    },
                );
                let safety_check = outcome.is_ok();
                // Skip if both saftey and liveness checks fail.
                if !safety_check && !liveness_check {
                    error!("Failed safety and liveness check \n High QC is {:?}  Proposal QC is {:?}  Locked view is {:?}", consensus.high_qc, proposal.data.clone(), consensus.locked_view);
                    if let Err(e) = outcome {
                        broadcast_event(
                            Event {
                                view_number: view,
                                event: EventType::Error { error: Arc::new(e) },
                            },
                            &self.output_event_stream,
                        )
                        .await;
                    }
                    return;
                }

                // Stop polling for the received proposal.
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForProposal(*view))
                    .await;

                // Notify the application layer and other tasks.
                broadcast_event(
                    Event {
                        view_number: view,
                        event: EventType::QuorumProposal {
                            proposal: proposal.clone(),
                            sender: sender.clone(),
                        },
                    },
                    &self.output_event_stream,
                )
                .await;
                broadcast_event(
                    Arc::new(HotShotEvent::QuorumProposalValidated(
                        proposal.data.clone(),
                        parent_leaf,
                    )),
                    &event_sender,
                )
                .await;

                // Add to the storage.
                let mut consensus = RwLockUpgradableReadGuard::upgrade(consensus).await;
                consensus.validated_state_map.insert(
                    view,
                    View {
                        view_inner: ViewInner::Leaf {
                            leaf: proposed_leaf.commit(),
                            state: state.clone(),
                            delta: Some(delta.clone()),
                        },
                    },
                );
                consensus
                    .saved_leaves
                    .insert(proposed_leaf.commit(), proposed_leaf.clone());
                if let Err(e) = self
                    .storage
                    .write()
                    .await
                    .update_undecided_state(
                        consensus.saved_leaves.clone(),
                        consensus.validated_state_map.clone(),
                    )
                    .await
                {
                    warn!("Couldn't store undecided state.  Error: {:?}", e);
                }
                drop(consensus);

                self.create_dependency_task_if_new(view, event_receiver, &event_sender);
            }
            HotShotEvent::DACertificateRecv(cert) => {
                let view = cert.view_number;
                debug!("Received DAC for view {}", *view);
                if view <= self.latest_voted_view {
                    return;
                }

                // Validate the DAC.
                if !cert.is_valid_cert(self.da_membership.as_ref()) {
                    return;
                }

                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForDAC(*view))
                    .await;

                self.committee_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(*view))
                    .await;

                // Add to the storage.
                self.consensus
                    .write()
                    .await
                    .saved_da_certs
                    .insert(view, cert.clone());

                broadcast_event(
                    Arc::new(HotShotEvent::DACertificateValidated(cert.clone())),
                    &event_sender.clone(),
                )
                .await;
                self.create_dependency_task_if_new(view, event_receiver, &event_sender);
            }
            HotShotEvent::VIDShareRecv(disperse) => {
                let view = disperse.data.get_view_number();
                debug!("Received VID share for view {}", *view);
                if view <= self.latest_voted_view {
                    return;
                }

                // Validate the VID share.
                let payload_commitment = disperse.data.payload_commitment;
                // Check whether the data satisfies one of the following.
                // * From the right leader for this view.
                // * Calculated and signed by the current node.
                // * Signed by one of the staked DA committee members.
                if !self
                    .quorum_membership
                    .get_leader(view)
                    .validate(&disperse.signature, payload_commitment.as_ref())
                    && !self
                        .public_key
                        .validate(&disperse.signature, payload_commitment.as_ref())
                {
                    let mut validated = false;
                    for da_member in self.da_membership.get_staked_committee(view) {
                        if da_member.validate(&disperse.signature, payload_commitment.as_ref()) {
                            validated = true;
                            break;
                        }
                    }
                    if !validated {
                        return;
                    }
                }

                // stop polling for the received disperse after verifying it's valid
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVIDDisperse(
                        *disperse.data.view_number,
                    ))
                    .await;

                // Add to the storage.
                if let Err(e) = self.storage.write().await.append_vid(disperse).await {
                    error!("Failed to store VID share with error {:?}", e);
                    return;
                }
                self.consensus
                    .write()
                    .await
                    .vid_shares
                    .entry(view)
                    .or_default()
                    .insert(disperse.data.recipient_key.clone(), disperse.data.clone());

                broadcast_event(
                    Arc::new(HotShotEvent::VIDShareValidated(disperse.data.clone())),
                    &event_sender.clone(),
                )
                .await;
                self.create_dependency_task_if_new(view, event_receiver, &event_sender);
            }
            HotShotEvent::QuorumVoteDependenciesValidated(view) => {
                debug!("All vote dependencies verified for view {:?}", view);
                if !self.update_latest_voted_view(*view).await {
                    debug!("view not updated");
                    return;
                }
            }
            HotShotEvent::ViewChange(new_view) => {
                let new_view = *new_view;
                debug!(
                    "View Change event for view {} in quorum vote task",
                    *new_view
                );

                let old_voted_view = self.latest_voted_view;

                // Start polling for VID disperse for the new view
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::PollForVIDDisperse(
                        *old_voted_view + 1,
                    ))
                    .await;
            }
            _ => {}
        }
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> TaskState for QuorumVoteTaskState<TYPES, I> {
    type Event = Arc<HotShotEvent<TYPES>>;
    type Output = ();
    fn filter(&self, event: &Arc<HotShotEvent<TYPES>>) -> bool {
        !matches!(
            event.as_ref(),
            HotShotEvent::QuorumProposalRecv(_, _)
                | HotShotEvent::DACertificateRecv(_)
                | HotShotEvent::ViewChange(_)
                | HotShotEvent::VIDShareRecv(..)
                | HotShotEvent::QuorumVoteDependenciesValidated(_)
                | HotShotEvent::Shutdown,
        )
    }
    async fn handle_event(event: Self::Event, task: &mut Task<Self>) -> Option<()>
    where
        Self: Sized,
    {
        let receiver = task.subscribe();
        let sender = task.clone_sender();
        tracing::trace!("sender queue len {}", sender.len());
        task.state_mut().handle(event, receiver, sender).await;
        None
    }
    fn should_shutdown(event: &Self::Event) -> bool {
        matches!(event.as_ref(), HotShotEvent::Shutdown)
    }
}
