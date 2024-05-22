use std::{collections::HashMap, sync::Arc};

use async_broadcast::{Receiver, Sender};
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use either::Either;
use hotshot_task::{
    dependency::{AndDependency, EventDependency, OrDependency},
    dependency_task::DependencyTask,
    task::{Task, TaskState},
};
use hotshot_types::{
    consensus::Consensus,
    event::Event,
    traits::{
        election::Membership,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::SignatureKey,
        storage::Storage,
    },
    vote::{Certificate, HasViewNumber},
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, instrument, warn};
use vbs::version::Version;

use crate::{
    events::HotShotEvent,
    helpers::{broadcast_event, cancel_task},
};

use self::{
    dependency_handle::{ProposalDependency, ProposalDependencyHandle},
    handlers::handle_quorum_proposal_validated,
};

mod dependency_handle;

/// Event handlers for [`QuorumProposalTaskState`].
mod handlers;

/// The state for the quorum proposal task.
pub struct QuorumProposalTaskState<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Latest view number that has been proposed for.
    pub latest_proposed_view: TYPES::Time,

    /// Table for the in-progress proposal depdencey tasks.
    pub propose_dependencies: HashMap<TYPES::Time, JoinHandle<()>>,

    /// Network for all nodes
    pub quorum_network: Arc<I::QuorumNetwork>,

    /// Network for DA committee
    pub da_network: Arc<I::DaNetwork>,

    /// Output events to application
    pub output_event_stream: async_broadcast::Sender<Event<TYPES>>,

    /// Immutable instance state
    pub instance_state: Arc<TYPES::InstanceState>,

    /// Membership for Timeout votes/certs
    pub timeout_membership: Arc<TYPES::Membership>,

    /// Membership for Quorum Certs/votes
    pub quorum_membership: Arc<TYPES::Membership>,

    /// Our public key
    pub public_key: TYPES::SignatureKey,

    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,

    /// View timeout from config.
    pub timeout: u64,

    /// Round start delay from config, in milliseconds.
    pub round_start_delay: u64,

    /// timeout task handle
    pub timeout_task: Option<JoinHandle<()>>,

    /// This node's storage ref
    pub storage: Arc<RwLock<I::Storage>>,

    /// Shared consensus task state
    pub consensus: Arc<RwLock<Consensus<TYPES>>>,

    /// The node's id
    pub id: u64,

    /// Current version of consensus
    pub version: Version,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> QuorumProposalTaskState<TYPES, I> {
    /// Create an event dependency
    #[instrument(skip_all, fields(id = self.id, latest_proposed_view = *self.latest_proposed_view), name = "Create event dependency", level = "info")]
    fn create_event_dependency(
        &self,
        dependency_type: ProposalDependency,
        view_number: TYPES::Time,
        event_receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
    ) -> EventDependency<Arc<HotShotEvent<TYPES>>> {
        EventDependency::new(
            event_receiver,
            Box::new(move |event| {
                let event = event.as_ref();
                let event_view = match dependency_type {
                    ProposalDependency::QC => {
                        if let HotShotEvent::UpdateHighQc(qc) = event {
                            qc.view_number() + 1
                        } else {
                            return false;
                        }
                    }
                    ProposalDependency::TimeoutCert => {
                        if let HotShotEvent::QcFormed(either::Right(timeout)) = event {
                            timeout.view_number() + 1
                        } else {
                            return false;
                        }
                    }
                    ProposalDependency::ViewSyncCert => {
                        if let HotShotEvent::ViewSyncFinalizeCertificate2Recv(view_sync_cert) =
                            event
                        {
                            view_sync_cert.view_number()
                        } else {
                            return false;
                        }
                    }

                    ProposalDependency::Proposal => {
                        if let HotShotEvent::QuorumProposalValidated(proposal, _) = event {
                            proposal.view_number() + 1
                        } else {
                            return false;
                        }
                    }
                    ProposalDependency::PayloadAndMetadata => {
                        if let HotShotEvent::SendPayloadCommitmentAndMetadata(
                            _payload_commitment,
                            _builder_commitment,
                            _metadata,
                            view_number,
                            _fee,
                        ) = event
                        {
                            *view_number
                        } else {
                            return false;
                        }
                    }
                    ProposalDependency::ProposeNow => {
                        if let HotShotEvent::ProposeNow(view_number, _) = event {
                            *view_number
                        } else {
                            return false;
                        }
                    }
                    ProposalDependency::VidShare => {
                        if let HotShotEvent::VidShareValidated(vid_share) = event {
                            vid_share.data.view_number()
                        } else {
                            return false;
                        }
                    }
                    ProposalDependency::ValidatedState => {
                        if let HotShotEvent::ValidatedStateUpdated(view_number, _) = event {
                            *view_number + 1
                        } else {
                            return false;
                        }
                    }
                };
                let valid = event_view == view_number;
                if valid {
                    debug!("Dependency {dependency_type:?} is complete for view {event_view:?}!",);
                }
                valid
            }),
        )
    }

    /// Creates the requisite dependencies for the Quorum Proposal task. It also handles any event forwarding.
    fn create_and_complete_dependencies(
        &self,
        view_number: TYPES::Time,
        event_receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
        event: Arc<HotShotEvent<TYPES>>,
    ) -> AndDependency<Vec<Vec<Arc<HotShotEvent<TYPES>>>>> {
        let mut proposal_dependency = self.create_event_dependency(
            ProposalDependency::Proposal,
            view_number,
            event_receiver.clone(),
        );

        let mut qc_dependency = self.create_event_dependency(
            ProposalDependency::QC,
            view_number,
            event_receiver.clone(),
        );

        let mut view_sync_dependency = self.create_event_dependency(
            ProposalDependency::ViewSyncCert,
            view_number,
            event_receiver.clone(),
        );

        let mut timeout_dependency = self.create_event_dependency(
            ProposalDependency::TimeoutCert,
            view_number,
            event_receiver.clone(),
        );

        let mut payload_commitment_dependency = self.create_event_dependency(
            ProposalDependency::PayloadAndMetadata,
            view_number,
            event_receiver.clone(),
        );

        let mut propose_now_dependency = self.create_event_dependency(
            ProposalDependency::ProposeNow,
            view_number,
            event_receiver.clone(),
        );

        let mut vid_share_dependency = self.create_event_dependency(
            ProposalDependency::VidShare,
            view_number,
            event_receiver.clone(),
        );

        let mut validated_state_update_dependency = self.create_event_dependency(
            ProposalDependency::ValidatedState,
            view_number,
            event_receiver,
        );

        match event.as_ref() {
            HotShotEvent::ProposeNow(..) => {
                propose_now_dependency.mark_as_completed(Arc::clone(&event));
            }
            HotShotEvent::SendPayloadCommitmentAndMetadata(..) => {
                payload_commitment_dependency.mark_as_completed(Arc::clone(&event));
            }
            HotShotEvent::QuorumProposalValidated(..) => {
                proposal_dependency.mark_as_completed(event);
            }
            HotShotEvent::QcFormed(quorum_certificate) => match quorum_certificate {
                Either::Right(_) => {
                    timeout_dependency.mark_as_completed(event);
                }
                Either::Left(_) => {
                    // qc_dependency.mark_as_completed(event);
                }
            },
            HotShotEvent::ViewSyncFinalizeCertificate2Recv(_) => {
                view_sync_dependency.mark_as_completed(event);
            }
            HotShotEvent::VidShareValidated(_) => {
                vid_share_dependency.mark_as_completed(event);
            }
            HotShotEvent::ValidatedStateUpdated(_, _) => {
                validated_state_update_dependency.mark_as_completed(event);
            }
            HotShotEvent::UpdateHighQc(_) => {
                qc_dependency.mark_as_completed(event);
            }
            _ => {}
        };

        // We have three cases to consider:
        let mut secondary_deps = vec![
            // 2. A timeout cert was received
            AndDependency::from_deps(vec![timeout_dependency]),
            // 3. A view sync cert was received.
            AndDependency::from_deps(vec![view_sync_dependency]),
        ];

        // 1. A QcFormed event and QuorumProposalValidated event
        if *view_number > 1 {
            secondary_deps.push(AndDependency::from_deps(vec![
                qc_dependency,
                proposal_dependency,
            ]));
        } else {
            secondary_deps.push(AndDependency::from_deps(vec![qc_dependency]));
        }

        AndDependency::from_deps(vec![OrDependency::from_deps(vec![
            AndDependency::from_deps(vec![AndDependency::from_deps(vec![propose_now_dependency])]),
            AndDependency::from_deps(vec![
                OrDependency::from_deps(vec![AndDependency::from_deps(vec![
                    payload_commitment_dependency,
                    vid_share_dependency,
                    validated_state_update_dependency,
                ])]),
                OrDependency::from_deps(secondary_deps),
            ]),
        ])])
    }

    /// Create and store an [`AndDependency`] combining [`EventDependency`]s associated with the
    /// given view number if it doesn't exist. Also takes in the received `event` to seed a
    /// dependency as already completed. This allows for the task to receive a proposable event
    /// without losing the data that it received, as the dependency task would otherwise have no
    /// ability to receive the event and, thus, would never propose.
    #[instrument(skip_all, fields(id = self.id, latest_proposed_view = *self.latest_proposed_view), name = "Create dependency task", level = "error")]
    fn create_dependency_task_if_new(
        &mut self,
        view_number: TYPES::Time,
        event_receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
        event_sender: Sender<Arc<HotShotEvent<TYPES>>>,
        event: Arc<HotShotEvent<TYPES>>,
    ) {
        // Don't even bother making the task if we are not entitled to propose anyay.
        if self.quorum_membership.leader(view_number) != self.public_key {
            tracing::trace!("We are not the leader of the next view");
            return;
        }

        // Don't try to propose twice for the same view.
        if view_number <= self.latest_proposed_view {
            tracing::trace!("We have already proposed for this view");
            return;
        }

        debug!("Attempting to make dependency task for view {view_number:?} and event {event:?}");
        if self.propose_dependencies.contains_key(&view_number) {
            debug!("Task already exists");
            return;
        }

        let dependency_chain =
            self.create_and_complete_dependencies(view_number, event_receiver, event);

        let dependency_task = DependencyTask::new(
            dependency_chain,
            ProposalDependencyHandle {
                latest_proposed_view: self.latest_proposed_view,
                view_number,
                sender: event_sender,
                quorum_membership: Arc::clone(&self.quorum_membership),
                public_key: self.public_key.clone(),
                private_key: self.private_key.clone(),
                round_start_delay: self.round_start_delay,
                instance_state: Arc::clone(&self.instance_state),
                consensus: Arc::clone(&self.consensus),
                version: self.version,
            },
        );
        self.propose_dependencies
            .insert(view_number, dependency_task.run());
    }

    /// Update the latest proposed view number.
    #[instrument(skip_all, fields(id = self.id, latest_proposed_view = *self.latest_proposed_view), name = "Update latest proposed view", level = "error")]
    async fn update_latest_proposed_view(&mut self, new_view: TYPES::Time) -> bool {
        if *self.latest_proposed_view < *new_view {
            debug!(
                "Updating latest proposed view from {} to {}",
                *self.latest_proposed_view, *new_view
            );

            // Cancel the old dependency tasks.
            for view in (*self.latest_proposed_view + 1)..=(*new_view) {
                if let Some(dependency) = self.propose_dependencies.remove(&TYPES::Time::new(view))
                {
                    cancel_task(dependency).await;
                }
            }

            self.latest_proposed_view = new_view;

            return true;
        }
        false
    }

    /// Handles a consensus event received on the event stream
    #[instrument(skip_all, fields(id = self.id, latest_proposed_view = *self.latest_proposed_view), name = "handle method", level = "error")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        event_receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
        event_sender: Sender<Arc<HotShotEvent<TYPES>>>,
    ) {
        match event.as_ref() {
            HotShotEvent::VersionUpgrade(version) => {
                self.version = *version;
            }
            HotShotEvent::ProposeNow(view_number, _) => {
                self.create_dependency_task_if_new(
                    *view_number,
                    event_receiver,
                    event_sender,
                    Arc::clone(&event),
                );
            }
            HotShotEvent::QcFormed(cert) => match cert.clone() {
                either::Right(timeout_cert) => {
                    let view_number = timeout_cert.view_number + 1;

                    self.create_dependency_task_if_new(
                        view_number,
                        event_receiver,
                        event_sender,
                        Arc::clone(&event),
                    );
                }
                either::Left(qc) => {
                    // Only update if the qc is from a newer view
                    let consensus_reader = self.consensus.read().await;
                    if qc.view_number <= consensus_reader.high_qc().view_number {
                        tracing::trace!(
                            "Received a QC for a view that was not > than our current high QC"
                        );
                    }

                    // We need to gate on this data actually existing in the consensus shared state.
                    // So we broadcast here and handle *before* we make the task.
                    broadcast_event(HotShotEvent::UpdateHighQc(qc).into(), &event_sender).await;
                }
            },
            HotShotEvent::SendPayloadCommitmentAndMetadata(
                _payload_commitment,
                _builder_commitment,
                _metadata,
                view_number,
                _fee,
            ) => {
                let view_number = *view_number;

                self.create_dependency_task_if_new(
                    view_number,
                    event_receiver,
                    event_sender,
                    Arc::clone(&event),
                );
            }
            HotShotEvent::ViewSyncFinalizeCertificate2Recv(certificate) => {
                if !certificate.is_valid_cert(self.quorum_membership.as_ref()) {
                    warn!(
                        "View Sync Finalize certificate {:?} was invalid",
                        certificate.date()
                    );
                    return;
                }

                let view_number = certificate.view_number;

                self.create_dependency_task_if_new(
                    view_number,
                    event_receiver,
                    event_sender,
                    event,
                );
            }
            HotShotEvent::QuorumProposalValidated(proposal, _) => {
                let new_view = proposal.view_number;

                // All nodes get the latest proposed view as a proxy of `cur_view` of olde.
                if !self.update_latest_proposed_view(new_view).await {
                    tracing::trace!("Failed to update latest proposed view");
                    return;
                }

                // Handle the event before creating the dependency task.
                if let Err(e) =
                    handle_quorum_proposal_validated(proposal, &event_sender, self).await
                {
                    debug!("Failed to handle QuorumProposalValidated event; error = {e:#}");
                }

                self.create_dependency_task_if_new(
                    new_view + 1,
                    event_receiver,
                    event_sender,
                    Arc::clone(&event),
                );
            }
            HotShotEvent::QuorumProposalSend(proposal, _) => {
                let view = proposal.data.view_number;
                if !self.update_latest_proposed_view(view).await {
                    tracing::trace!("Failed to update latest proposed view");
                    return;
                }
            }
            HotShotEvent::VidShareValidated(vid_share) => {
                let view_number = vid_share.data.view_number();

                // Update the vid shares map if we need to include the new value.
                let share = vid_share.clone();
                self.consensus
                    .write()
                    .await
                    .update_vid_shares(view_number, share.clone());

                self.create_dependency_task_if_new(
                    view_number,
                    event_receiver,
                    event_sender,
                    Arc::clone(&event),
                );
            }
            HotShotEvent::ValidatedStateUpdated(view_number, view) => {
                // Update the internal validated state map.
                self.consensus
                    .write()
                    .await
                    .update_validated_state_map(*view_number, view.clone());

                self.create_dependency_task_if_new(
                    *view_number + 1,
                    event_receiver,
                    event_sender,
                    Arc::clone(&event),
                );
            }
            HotShotEvent::UpdateHighQc(qc) => {
                // First, update the high QC.
                if let Err(e) = self.consensus.write().await.update_high_qc(qc.clone()) {
                    tracing::trace!("Failed to update high qc; error = {e}");
                }

                if let Err(e) = self.storage.write().await.update_high_qc(qc.clone()).await {
                    warn!("Failed to store High QC of QC we formed; error = {:?}", e);
                }

                let view_number = qc.view_number() + 1;
                self.create_dependency_task_if_new(
                    view_number,
                    event_receiver,
                    event_sender,
                    Arc::clone(&event),
                );
            }
            _ => {}
        }
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> TaskState
    for QuorumProposalTaskState<TYPES, I>
{
    type Event = Arc<HotShotEvent<TYPES>>;
    type Output = ();
    fn filter(&self, event: &Self::Event) -> bool {
        !matches!(
            event.as_ref(),
            HotShotEvent::QuorumProposalValidated(..)
                | HotShotEvent::QcFormed(_)
                | HotShotEvent::SendPayloadCommitmentAndMetadata(..)
                | HotShotEvent::ViewSyncFinalizeCertificate2Recv(_)
                | HotShotEvent::ProposeNow(..)
                | HotShotEvent::QuorumProposalSend(..)
                | HotShotEvent::VidShareValidated(_)
                | HotShotEvent::ValidatedStateUpdated(..)
                | HotShotEvent::UpdateHighQc(_)
                | HotShotEvent::Shutdown
        )
    }
    async fn handle_event(event: Self::Event, task: &mut Task<Self>) -> Option<()>
    where
        Self: Sized,
    {
        let receiver = task.subscribe();
        let sender = task.clone_sender();
        task.state_mut().handle(event, receiver, sender).await;
        None
    }
    fn should_shutdown(event: &Self::Event) -> bool {
        matches!(event.as_ref(), HotShotEvent::Shutdown)
    }
}
