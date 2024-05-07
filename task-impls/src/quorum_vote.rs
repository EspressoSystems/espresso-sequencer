#[cfg(feature = "dependency-tasks")]
use crate::consensus::helpers::update_state_and_vote_if_able;
use crate::{
    events::HotShotEvent,
    helpers::{broadcast_event, cancel_task},
};
use async_broadcast::{Receiver, Sender};
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use committable::Committable;
use hotshot_task::{
    dependency::{AndDependency, EventDependency, OrDependency},
    dependency_task::{DependencyTask, HandleDepOutput},
    task::{Task, TaskState},
};
use hotshot_types::{
    consensus::Consensus,
    data::Leaf,
    event::Event,
    message::GeneralConsensusMessage,
    simple_vote::{QuorumData, QuorumVote},
    traits::{
        block_contents::BlockHeader,
        election::Membership,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::SignatureKey,
        storage::Storage,
    },
    vid::vid_scheme,
    vote::{Certificate, HasViewNumber},
};
use jf_primitives::vid::VidScheme;
#[cfg(feature = "dependency-tasks")]
use std::marker::PhantomData;
use std::{collections::HashMap, sync::Arc};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, error, instrument, trace, warn};

/// Vote dependency types.
#[derive(Debug, PartialEq)]
enum VoteDependency {
    /// For the `QuroumProposalValidated` event after validating `QuorumProposalRecv`.
    QuorumProposal,
    /// For the `DACertificateRecv` event.
    Dac,
    /// For the `VIDShareRecv` event.
    Vid,
    /// For the `VoteNow` event.
    VoteNow,
}

/// Handler for the vote dependency.
#[allow(dead_code)]
struct VoteDependencyHandle<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Public key.
    pub public_key: TYPES::SignatureKey,
    /// Private Key.
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// Reference to consensus. The replica will require a write lock on this.
    consensus: Arc<RwLock<Consensus<TYPES>>>,
    /// Immutable instance state
    instance_state: Arc<TYPES::InstanceState>,
    /// Membership for Quorum certs/votes.
    quorum_membership: Arc<TYPES::Membership>,
    /// Reference to the storage.
    pub storage: Arc<RwLock<I::Storage>>,
    /// View number to vote on.
    view_number: TYPES::Time,
    /// Event sender.
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES> + 'static> HandleDepOutput
    for VoteDependencyHandle<TYPES, I>
{
    type Output = Vec<Arc<HotShotEvent<TYPES>>>;
    #[allow(clippy::too_many_lines)]
    async fn handle_dep_result(self, res: Self::Output) {
        #[allow(unused_variables)]
        let mut cur_proposal = None;
        let mut payload_commitment = None;
        let mut leaf = None;
        let mut disperse_share = None;
        for event in res {
            match event.as_ref() {
                #[allow(unused_assignments)]
                HotShotEvent::QuorumProposalValidated(proposal, parent_leaf) => {
                    cur_proposal = Some(proposal.clone());
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
                        warn!("Proposed leaf parent commitment does not match parent leaf payload commitment. Aborting vote.");
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
                    let vid_payload_commitment = share.data.payload_commitment;
                    disperse_share = Some(share.clone());
                    if let Some(comm) = payload_commitment {
                        if vid_payload_commitment != comm {
                            error!("VID has inconsistent payload commitment with quorum proposal or DAC.");
                            return;
                        }
                    } else {
                        payload_commitment = Some(vid_payload_commitment);
                    }
                }
                HotShotEvent::VoteNow(_, vote_dependency_data) => {
                    leaf = Some(vote_dependency_data.parent_leaf.clone());
                    disperse_share = Some(vote_dependency_data.disperse_share.clone());
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

        #[cfg(feature = "dependency-tasks")]
        {
            let Some(proposal) = cur_proposal else {
                error!("No proposal received, but it should be.");
                return;
            };
            // For this vote task, we'll update the state in storage without voting in this function,
            // then vote later.
            update_state_and_vote_if_able::<TYPES, I>(
                self.view_number,
                proposal,
                self.public_key.clone(),
                self.consensus,
                Arc::clone(&self.storage),
                self.quorum_membership,
                self.instance_state,
                PhantomData,
            )
            .await;
        }

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
            // Add to the storage.
            let Some(disperse) = disperse_share else {
                return;
            };
            if let Err(e) = self.storage.write().await.append_vid(&disperse).await {
                error!("Failed to store VID share with error {:?}", e);
                return;
            }
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

    /// Immutable instance state
    pub instance_state: Arc<TYPES::InstanceState>,

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
                            disperse.data.view_number
                        } else {
                            return false;
                        }
                    }
                    VoteDependency::VoteNow => {
                        if let HotShotEvent::VoteNow(view, _) = event {
                            *view
                        } else {
                            return false;
                        }
                    }
                };
                if event_view == view_number {
                    trace!("Vote dependency {:?} completed", dependency_type);
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
        event: Option<Arc<HotShotEvent<TYPES>>>,
    ) {
        if self.vote_dependencies.contains_key(&view_number) {
            return;
        }

        let mut quorum_proposal_dependency = self.create_event_dependency(
            VoteDependency::QuorumProposal,
            view_number,
            event_receiver.clone(),
        );
        let dac_dependency =
            self.create_event_dependency(VoteDependency::Dac, view_number, event_receiver.clone());
        let vid_dependency =
            self.create_event_dependency(VoteDependency::Vid, view_number, event_receiver.clone());
        let mut vote_now_dependency = self.create_event_dependency(
            VoteDependency::VoteNow,
            view_number,
            event_receiver.clone(),
        );

        // If we have an event provided to us
        if let Some(event) = event {
            match event.as_ref() {
                HotShotEvent::VoteNow(..) => {
                    vote_now_dependency.mark_as_completed(event);
                }
                HotShotEvent::QuorumProposalValidated(..) => {
                    quorum_proposal_dependency.mark_as_completed(event);
                }
                _ => {}
            }
        }

        let deps = vec![quorum_proposal_dependency, dac_dependency, vid_dependency];
        let dependency_chain = OrDependency::from_deps(vec![
            // Either we fulfull the dependencies individiaully.
            AndDependency::from_deps(deps),
            // Or we fulfill the single dependency that contains all the info that we need.
            AndDependency::from_deps(vec![vote_now_dependency]),
        ]);

        let dependency_task = DependencyTask::new(
            dependency_chain,
            VoteDependencyHandle::<TYPES, I> {
                public_key: self.public_key.clone(),
                private_key: self.private_key.clone(),
                consensus: Arc::clone(&self.consensus),
                instance_state: Arc::clone(&self.instance_state),
                quorum_membership: Arc::clone(&self.quorum_membership),
                storage: Arc::clone(&self.storage),
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
            HotShotEvent::VoteNow(view, ..) => {
                self.create_dependency_task_if_new(
                    *view,
                    event_receiver,
                    &event_sender,
                    Some(event),
                );
            }
            HotShotEvent::QuorumProposalValidated(proposal, _leaf) => {
                // This task simultaneously does not rely on the state updates of the `handle_quorum_proposal_validated`
                // function and that function does not return an `Error` unless the propose or vote fails, in which case
                // the other would still have been attempted regardless. Therefore, we pass this through as a task and
                // eschew validation in lieu of the `QuorumProposal` task doing it for us and updating the internal state.
                self.create_dependency_task_if_new(
                    proposal.view_number,
                    event_receiver,
                    &event_sender,
                    Some(Arc::clone(&event)),
                );
            }
            HotShotEvent::DACertificateRecv(cert) => {
                let view = cert.view_number;
                trace!("Received DAC for view {}", *view);
                if view <= self.latest_voted_view {
                    return;
                }

                // Validate the DAC.
                if !cert.is_valid_cert(self.da_membership.as_ref()) {
                    return;
                }

                // Add to the storage.
                self.consensus
                    .write()
                    .await
                    .update_saved_da_certs(view, cert.clone());

                broadcast_event(
                    Arc::new(HotShotEvent::DACertificateValidated(cert.clone())),
                    &event_sender.clone(),
                )
                .await;
                self.create_dependency_task_if_new(view, event_receiver, &event_sender, None);
            }
            HotShotEvent::VIDShareRecv(disperse) => {
                let view = disperse.data.get_view_number();
                trace!("Received VID share for view {}", *view);
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
                if vid_scheme(self.quorum_membership.total_nodes())
                    .verify_share(
                        &disperse.data.share,
                        &disperse.data.common,
                        &payload_commitment,
                    )
                    .is_err()
                {
                    debug!("Invalid VID share.");
                    return;
                }

                self.consensus
                    .write()
                    .await
                    .update_vid_shares(view, disperse.clone());

                if disperse.data.recipient_key != self.public_key {
                    debug!("Got a Valid VID share but it's not for our key");
                    return;
                }

                broadcast_event(
                    Arc::new(HotShotEvent::VIDShareValidated(disperse.clone())),
                    &event_sender.clone(),
                )
                .await;
                self.create_dependency_task_if_new(view, event_receiver, &event_sender, None);
            }
            HotShotEvent::QuorumVoteDependenciesValidated(view) => {
                debug!("All vote dependencies verified for view {:?}", view);
                if !self.update_latest_voted_view(*view).await {
                    debug!("view not updated");
                    return;
                }
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
            HotShotEvent::DACertificateRecv(_)
                | HotShotEvent::VIDShareRecv(..)
                | HotShotEvent::QuorumVoteDependenciesValidated(_)
                | HotShotEvent::VoteNow(..)
                | HotShotEvent::QuorumProposalValidated(..)
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
