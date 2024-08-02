use std::{collections::HashMap, sync::Arc};

use anyhow::{bail, ensure, Context, Result};
use async_broadcast::{Receiver, Sender};
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use async_trait::async_trait;
use committable::Committable;
use hotshot_task::{
    dependency::{AndDependency, Dependency, EventDependency, OrDependency},
    dependency_task::{DependencyTask, HandleDepOutput},
    task::TaskState,
};
use hotshot_types::{
    consensus::OuterConsensus,
    data::{Leaf, VidDisperseShare, ViewNumber},
    event::Event,
    message::Proposal,
    simple_certificate::{version, UpgradeCertificate},
    simple_vote::{QuorumData, QuorumVote},
    traits::{
        block_contents::BlockHeader,
        election::Membership,
        node_implementation::{ConsensusTime, NodeImplementation, NodeType},
        signature_key::SignatureKey,
        storage::Storage,
        ValidatedState,
    },
    utils::{View, ViewInner},
    vid::vid_scheme,
    vote::{Certificate, HasViewNumber},
};
use jf_vid::VidScheme;
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::{
    events::HotShotEvent,
    helpers::{broadcast_event, cancel_task, fetch_proposal},
    quorum_vote::handlers::handle_quorum_proposal_validated,
};

/// Event handlers for `QuorumProposalValidated`.
mod handlers;

/// Vote dependency types.
#[derive(Debug, PartialEq)]
enum VoteDependency {
    /// For the `QuroumProposalValidated` event after validating `QuorumProposalRecv`.
    QuorumProposal,
    /// For the `DaCertificateRecv` event.
    Dac,
    /// For the `VidShareRecv` event.
    Vid,
    /// For the `VoteNow` event.
    VoteNow,
}

/// Handler for the vote dependency.
pub struct VoteDependencyHandle<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Public key.
    pub public_key: TYPES::SignatureKey,
    /// Private Key.
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// Reference to consensus. The replica will require a write lock on this.
    pub consensus: OuterConsensus<TYPES>,
    /// Immutable instance state
    pub instance_state: Arc<TYPES::InstanceState>,
    /// Membership for Quorum certs/votes.
    pub quorum_membership: Arc<TYPES::Membership>,
    /// Reference to the storage.
    pub storage: Arc<RwLock<I::Storage>>,
    /// View number to vote on.
    pub view_number: TYPES::Time,
    /// Event sender.
    pub sender: Sender<Arc<HotShotEvent<TYPES>>>,
    /// Event receiver.
    pub receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
    /// An upgrade certificate that has been decided on, if any.
    pub decided_upgrade_certificate: Arc<RwLock<Option<UpgradeCertificate<TYPES>>>>,
    /// The node's id
    pub id: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES> + 'static> VoteDependencyHandle<TYPES, I> {
    /// Updates the shared consensus state with the new voting data.
    #[instrument(skip_all, target = "VoteDependencyHandle", fields(id = self.id, view = *self.view_number))]
    async fn update_shared_state(
        &self,
        proposed_leaf: &Leaf<TYPES>,
        vid_share: &Proposal<TYPES, VidDisperseShare<TYPES>>,
    ) -> Result<()> {
        let justify_qc = &proposed_leaf.justify_qc();

        // Justify qc's leaf commitment should be the same as the parent's leaf commitment.
        let mut maybe_parent = self
            .consensus
            .read()
            .await
            .saved_leaves()
            .get(&justify_qc.date().leaf_commit)
            .cloned();
        maybe_parent = match maybe_parent {
            Some(p) => Some(p),
            None => fetch_proposal(
                justify_qc.view_number(),
                self.sender.clone(),
                Arc::clone(&self.quorum_membership),
                OuterConsensus::new(Arc::clone(&self.consensus.inner_consensus)),
            )
            .await
            .ok(),
        };
        let parent = maybe_parent.context(format!(
            "Proposal's parent missing from storage with commitment: {:?}, proposal view {:?}",
            justify_qc.date().leaf_commit,
            proposed_leaf.view_number(),
        ))?;
        let consensus_reader = self.consensus.read().await;

        let (Some(parent_state), _) = consensus_reader.state_and_delta(parent.view_number()) else {
            bail!("Parent state not found! Consensus internally inconsistent")
        };

        drop(consensus_reader);

        let version = version(
            self.view_number,
            &self.decided_upgrade_certificate.read().await.clone(),
        )?;
        let (validated_state, state_delta) = parent_state
            .validate_and_apply_header(
                &self.instance_state,
                &parent,
                &proposed_leaf.block_header().clone(),
                vid_share.data.common.clone(),
                version,
            )
            .await
            .context("Block header doesn't extend the proposal!")?;

        let state = Arc::new(validated_state);
        let delta = Arc::new(state_delta);

        // Now that we've rounded everyone up, we need to update the shared state and broadcast our events.
        // We will defer broadcast until all states are updated to avoid holding onto the lock during a network call.
        let mut consensus_writer = self.consensus.write().await;

        let view = View {
            view_inner: ViewInner::Leaf {
                leaf: proposed_leaf.commit(),
                state: Arc::clone(&state),
                delta: Some(Arc::clone(&delta)),
            },
        };
        if let Err(e) =
            consensus_writer.update_validated_state_map(proposed_leaf.view_number(), view.clone())
        {
            tracing::trace!("{e:?}");
        }
        consensus_writer.update_saved_leaves(proposed_leaf.clone());

        // Kick back our updated structures for downstream usage.
        let new_leaves = consensus_writer.saved_leaves().clone();
        let new_state = consensus_writer.validated_state_map().clone();
        drop(consensus_writer);

        // Broadcast now that the lock is dropped.
        broadcast_event(
            HotShotEvent::ValidatedStateUpdated(proposed_leaf.view_number(), view).into(),
            &self.sender,
        )
        .await;

        // Send the new state up to the sequencer.
        self.storage
            .write()
            .await
            .update_undecided_state(new_leaves, new_state)
            .await?;

        Ok(())
    }

    /// Submits the `QuorumVoteSend` event if all the dependencies are met.
    #[instrument(skip_all, fields(id = self.id, name = "Submit quorum vote", level = "error"))]
    async fn submit_vote(
        &self,
        leaf: Leaf<TYPES>,
        vid_share: Proposal<TYPES, VidDisperseShare<TYPES>>,
    ) -> Result<()> {
        ensure!(
            self.quorum_membership.has_stake(&self.public_key),
            format!(
                "We were not chosen for quorum committee on {:?}",
                self.view_number
            ),
        );

        // Create and send the vote.
        let vote = QuorumVote::<TYPES>::create_signed_vote(
            QuorumData {
                leaf_commit: leaf.commit(),
            },
            self.view_number,
            &self.public_key,
            &self.private_key,
        )
        .context("Failed to sign vote")?;
        debug!(
            "sending vote to next quorum leader {:?}",
            vote.view_number() + 1
        );
        // Add to the storage.
        self.storage
            .write()
            .await
            .append_vid(&vid_share)
            .await
            .context("Failed to store VID share")?;
        broadcast_event(Arc::new(HotShotEvent::QuorumVoteSend(vote)), &self.sender).await;

        Ok(())
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES> + 'static> HandleDepOutput
    for VoteDependencyHandle<TYPES, I>
{
    type Output = Vec<Arc<HotShotEvent<TYPES>>>;

    #[allow(clippy::too_many_lines)]
    async fn handle_dep_result(self, res: Self::Output) {
        let high_qc_view_number = self.consensus.read().await.high_qc().view_number;
        // The validated state of a non-genesis high QC should exist in the state map.
        if *high_qc_view_number != *ViewNumber::genesis()
            && !self
                .consensus
                .read()
                .await
                .validated_state_map()
                .contains_key(&high_qc_view_number)
        {
            // Block on receiving the event from the event stream.
            EventDependency::new(
                self.receiver.clone(),
                Box::new(move |event| {
                    let event = event.as_ref();
                    if let HotShotEvent::ValidatedStateUpdated(view_number, _) = event {
                        *view_number == high_qc_view_number
                    } else {
                        false
                    }
                }),
            )
            .completed()
            .await;
        }

        let mut payload_commitment = None;
        let mut leaf = None;
        let mut vid_share = None;
        for event in res {
            match event.as_ref() {
                #[allow(unused_assignments)]
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
                    if proposed_leaf.parent_commitment() != parent_commitment {
                        warn!("Proposed leaf parent commitment does not match parent leaf payload commitment. Aborting vote.");
                        return;
                    }
                    leaf = Some(proposed_leaf);
                }
                HotShotEvent::DaCertificateValidated(cert) => {
                    let cert_payload_comm = cert.date().payload_commit;
                    if let Some(comm) = payload_commitment {
                        if cert_payload_comm != comm {
                            error!("DAC has inconsistent payload commitment with quorum proposal or VID.");
                            return;
                        }
                    } else {
                        payload_commitment = Some(cert_payload_comm);
                    }
                }
                HotShotEvent::VidShareValidated(share) => {
                    let vid_payload_commitment = share.data.payload_commitment;
                    vid_share = Some(share.clone());
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
                    vid_share = Some(vote_dependency_data.vid_share.clone());
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

        let Some(vid_share) = vid_share else {
            error!(
                "We don't have the VID share for this view {:?}, but we should, because the vote dependencies have completed.",
                self.view_number
            );
            return;
        };

        let Some(leaf) = leaf else {
            error!(
                "We don't have the leaf for this view {:?}, but we should, because the vote dependencies have completed.",
                self.view_number
            );
            return;
        };

        // Update internal state
        if let Err(e) = self.update_shared_state(&leaf, &vid_share).await {
            error!("Failed to update shared consensus state; error = {e:#}");
            return;
        }

        if let Err(e) = self.submit_vote(leaf, vid_share).await {
            debug!("Failed to vote; error = {e:#}");
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
    pub consensus: OuterConsensus<TYPES>,

    /// Immutable instance state
    pub instance_state: Arc<TYPES::InstanceState>,

    /// Latest view number that has been voted for.
    pub latest_voted_view: TYPES::Time,

    /// Table for the in-progress dependency tasks.
    pub vote_dependencies: HashMap<TYPES::Time, JoinHandle<()>>,

    /// The underlying network
    pub network: Arc<I::Network>,

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

    /// An upgrade certificate that has been decided on, if any.
    pub decided_upgrade_certificate: Arc<RwLock<Option<UpgradeCertificate<TYPES>>>>,
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
                        if let HotShotEvent::DaCertificateValidated(cert) = event {
                            cert.view_number
                        } else {
                            return false;
                        }
                    }
                    VoteDependency::Vid => {
                        if let HotShotEvent::VidShareValidated(disperse) = event {
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
        if view_number <= self.latest_voted_view {
            tracing::trace!("We have already voted for this view");
            return;
        }

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
                consensus: OuterConsensus::new(Arc::clone(&self.consensus.inner_consensus)),
                instance_state: Arc::clone(&self.instance_state),
                quorum_membership: Arc::clone(&self.quorum_membership),
                storage: Arc::clone(&self.storage),
                view_number,
                sender: event_sender.clone(),
                receiver: event_receiver.clone(),
                decided_upgrade_certificate: Arc::clone(&self.decided_upgrade_certificate),
                id: self.id,
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
            for view in *self.latest_voted_view..(*new_view) {
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
    #[instrument(skip_all, fields(id = self.id, latest_voted_view = *self.latest_voted_view), name = "Quorum vote handle", level = "error", target = "QuorumVoteTaskState")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        event_receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
        event_sender: Sender<Arc<HotShotEvent<TYPES>>>,
    ) {
        match event.as_ref() {
            HotShotEvent::VoteNow(view, ..) => {
                info!("Vote NOW for view {:?}", *view);
                self.create_dependency_task_if_new(
                    *view,
                    event_receiver,
                    &event_sender,
                    Some(event),
                );
            }
            HotShotEvent::QuorumProposalValidated(proposal, _leaf) => {
                trace!("Received Proposal for view {}", *proposal.view_number());

                // Handle the event before creating the dependency task.
                if let Err(e) =
                    handle_quorum_proposal_validated(proposal, &event_sender, self).await
                {
                    debug!("Failed to handle QuorumProposalValidated event; error = {e:#}");
                }

                self.create_dependency_task_if_new(
                    proposal.view_number,
                    event_receiver,
                    &event_sender,
                    Some(Arc::clone(&event)),
                );
            }
            HotShotEvent::DaCertificateRecv(cert) => {
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
                    Arc::new(HotShotEvent::DaCertificateValidated(cert.clone())),
                    &event_sender.clone(),
                )
                .await;
                self.create_dependency_task_if_new(view, event_receiver, &event_sender, None);
            }
            HotShotEvent::VidShareRecv(disperse) => {
                let view = disperse.data.view_number();
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
                    .leader(view)
                    .validate(&disperse.signature, payload_commitment.as_ref())
                    && !self
                        .public_key
                        .validate(&disperse.signature, payload_commitment.as_ref())
                {
                    let mut validated = false;
                    for da_member in self.da_membership.staked_committee(view) {
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
                    Arc::new(HotShotEvent::VidShareValidated(disperse.clone())),
                    &event_sender.clone(),
                )
                .await;
                self.create_dependency_task_if_new(view, event_receiver, &event_sender, None);
            }
            HotShotEvent::QuorumVoteDependenciesValidated(view_number) => {
                debug!("All vote dependencies verified for view {:?}", view_number);
                if !self.update_latest_voted_view(*view_number).await {
                    debug!("view not updated");
                    return;
                }
            }
            _ => {}
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> TaskState for QuorumVoteTaskState<TYPES, I> {
    type Event = HotShotEvent<TYPES>;

    async fn handle_event(
        &mut self,
        event: Arc<Self::Event>,
        sender: &Sender<Arc<Self::Event>>,
        receiver: &Receiver<Arc<Self::Event>>,
    ) -> Result<()> {
        self.handle(event, receiver.clone(), sender.clone()).await;

        Ok(())
    }

    async fn cancel_subtasks(&mut self) {
        for handle in self.vote_dependencies.drain().map(|(_view, handle)| handle) {
            #[cfg(async_executor_impl = "async-std")]
            handle.cancel().await;
            #[cfg(async_executor_impl = "tokio")]
            handle.abort();
        }
    }
}
