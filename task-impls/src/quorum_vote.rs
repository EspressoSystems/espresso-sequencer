use crate::{
    events::HotShotEvent,
    helpers::{broadcast_event, cancel_task},
};
use async_broadcast::{Receiver, Sender};
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use hotshot_task::{
    dependency::{AndDependency, EventDependency},
    dependency_task::{DependencyTask, HandleDepOutput},
    task::{Task, TaskState},
};
use hotshot_types::{
    data::{QuorumProposal, VidDisperseShare},
    event::Event,
    message::Proposal,
    simple_certificate::DACertificate,
    traits::{
        block_contents::BlockHeader,
        network::{ConnectedNetwork, ConsensusIntentEvent},
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
    },
    vote::{Certificate, HasViewNumber},
};
use std::collections::HashMap;
use std::sync::Arc;
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, error, instrument, warn};

/// Vote dependency types.
#[derive(Debug, PartialEq)]
enum VoteDependency {
    /// For the `QuorumProposalRecv` event.
    QuorumProposal,
    /// For the `DACertificateRecv` event.
    Dac,
    /// For the `VidDisperseRecv` event.
    Vid,
}

/// Validate the quorum proposal.
// TODO: Complete the dependency implementation.
// <https://github.com/EspressoSystems/HotShot/issues/2710>
#[allow(clippy::needless_pass_by_value)]
fn validate_quorum_proposal<TYPES: NodeType>(
    _quorum_proposal: Proposal<TYPES, QuorumProposal<TYPES>>,
    _event_sender: Sender<Arc<HotShotEvent<TYPES>>>,
) -> bool {
    true
}

/// Validate the DAC.
// TODO: Complete the dependency implementation.
// <https://github.com/EspressoSystems/HotShot/issues/2710>
#[allow(clippy::needless_pass_by_value)]
fn validate_dac<TYPES: NodeType>(
    _dac: DACertificate<TYPES>,
    _event_sender: Sender<Arc<HotShotEvent<TYPES>>>,
) -> bool {
    true
}

/// Validate the VID share.
// TODO: Complete the dependency implementation.
// <https://github.com/EspressoSystems/HotShot/issues/2710>
#[allow(clippy::needless_pass_by_value)]
fn validate_vid<TYPES: NodeType>(
    _disperse: Proposal<TYPES, VidDisperseShare<TYPES>>,
    _event_sender: Sender<Arc<HotShotEvent<TYPES>>>,
) -> bool {
    true
}

/// Handler for the vote dependency.
struct VoteDependencyHandle<TYPES: NodeType> {
    /// View number to vote on.
    view_number: TYPES::Time,
    /// Event sender.
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
}
impl<TYPES: NodeType> HandleDepOutput for VoteDependencyHandle<TYPES> {
    type Output = Vec<Arc<HotShotEvent<TYPES>>>;
    async fn handle_dep_result(self, res: Self::Output) {
        // Add this commitment check to test if the handler works, but this isn't the only thing
        // that we'll need to check. E.g., we also need to check that VID commitment matches
        // `payload_commitment`.
        // TODO: Complete the dependency implementation.
        // <https://github.com/EspressoSystems/HotShot/issues/2710>
        let mut payload_commitment = None;
        for event in res {
            match event.as_ref() {
                HotShotEvent::QuorumProposalValidated(proposal) => {
                    let proposal_payload_comm = proposal.block_header.payload_commitment();
                    if let Some(comm) = payload_commitment {
                        if proposal_payload_comm != comm {
                            error!("Quorum proposal and DAC have inconsistent payload commitment.");
                            return;
                        }
                    } else {
                        payload_commitment = Some(proposal_payload_comm);
                    }
                }
                HotShotEvent::DACertificateValidated(cert) => {
                    let cert_payload_comm = cert.get_data().payload_commit;
                    if let Some(comm) = payload_commitment {
                        if cert_payload_comm != comm {
                            error!("Quorum proposal and DAC have inconsistent payload commitment.");
                            return;
                        }
                    } else {
                        payload_commitment = Some(cert_payload_comm);
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
        broadcast_event(
            Arc::new(HotShotEvent::DummyQuorumVoteSend(self.view_number)),
            &self.sender,
        )
        .await;
    }
}

/// The state for the quorum vote task.
///
/// Contains all of the information for the quorum vote.
pub struct QuorumVoteTaskState<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Latest view number that has been voted for.
    pub latest_voted_view: TYPES::Time,

    /// Table for the in-progress dependency tasks.
    pub vote_dependencies: HashMap<TYPES::Time, JoinHandle<()>>,

    /// Network for all nodes
    pub quorum_network: Arc<I::QuorumNetwork>,

    /// Network for DA committee
    pub committee_network: Arc<I::CommitteeNetwork>,

    /// Output events to application
    pub output_event_stream: async_broadcast::Sender<Event<TYPES>>,

    /// The node's id
    pub id: u64,
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
                        if let HotShotEvent::QuorumProposalValidated(proposal) = event {
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
            HotShotEvent::QuorumProposalRecv(proposal, _sender) => {
                let view = proposal.data.view_number;
                debug!("Received Quorum Proposal for view {}", *view);
                if view <= self.latest_voted_view {
                    return;
                }

                // stop polling for the received proposal
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForProposal(*view))
                    .await;
                broadcast_event(Arc::new(HotShotEvent::ViewChange(view + 1)), &event_sender).await;
                if !validate_quorum_proposal(proposal.clone(), event_sender.clone()) {
                    return;
                }
                broadcast_event(
                    Arc::new(HotShotEvent::QuorumProposalValidated(proposal.data.clone())),
                    &event_sender.clone(),
                )
                .await;
                self.create_dependency_task_if_new(view, event_receiver, &event_sender);
            }
            HotShotEvent::DACertificateRecv(cert) => {
                let view = cert.view_number;
                debug!("Received DAC for view {}", *view);
                if view <= self.latest_voted_view {
                    return;
                }

                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForDAC(*view))
                    .await;

                self.committee_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVotes(*view))
                    .await;

                if !validate_dac(cert.clone(), event_sender.clone()) {
                    return;
                }
                broadcast_event(
                    Arc::new(HotShotEvent::DACertificateValidated(cert.clone())),
                    &event_sender.clone(),
                )
                .await;
                self.create_dependency_task_if_new(view, event_receiver, &event_sender);
            }
            HotShotEvent::VidDisperseRecv(disperse) => {
                let view = disperse.data.get_view_number();
                debug!("Received VID share for view {}", *view);
                if view <= self.latest_voted_view {
                    return;
                }

                // stop polling for the received disperse after verifying it's valid
                self.quorum_network
                    .inject_consensus_info(ConsensusIntentEvent::CancelPollForVIDDisperse(
                        *disperse.data.view_number,
                    ))
                    .await;

                if !validate_vid(disperse.clone(), event_sender.clone()) {
                    return;
                }
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
                | HotShotEvent::VidDisperseRecv(..)
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
