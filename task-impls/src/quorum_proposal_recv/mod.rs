#![allow(unused_imports)]
#![cfg(feature = "dependency-tasks")]

use std::{collections::BTreeMap, sync::Arc};

use anyhow::{bail, Result};
use async_broadcast::{broadcast, Receiver, Sender};
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use async_trait::async_trait;
use futures::future::join_all;
use hotshot_task::task::{Task, TaskState};
use hotshot_types::{
    consensus::{Consensus, OuterConsensus},
    data::{Leaf, ViewChangeEvidence},
    event::Event,
    simple_certificate::UpgradeCertificate,
    traits::{
        node_implementation::{NodeImplementation, NodeType},
        signature_key::SignatureKey,
    },
    vote::{HasViewNumber, VoteDependencyData},
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, error, info, instrument, warn};
use vbs::version::Version;

use self::handlers::handle_quorum_proposal_recv;
use crate::{
    events::{HotShotEvent, ProposalMissing},
    helpers::{broadcast_event, cancel_task, parent_leaf_and_state},
    quorum_proposal_recv::handlers::QuorumProposalValidity,
};

/// Event handlers for this task.
mod handlers;

/// The state for the quorum proposal task. Contains all of the information for
/// handling [`HotShotEvent::QuorumProposalRecv`] events.
pub struct QuorumProposalRecvTaskState<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Our public key
    pub public_key: TYPES::SignatureKey,

    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,

    /// Reference to consensus. The replica will require a write lock on this.
    pub consensus: OuterConsensus<TYPES>,

    /// View number this view is executing in.
    pub cur_view: TYPES::Time,

    /// Timestamp this view starts at.
    pub cur_view_time: i64,

    /// The underlying network
    pub network: Arc<I::Network>,

    /// Membership for Quorum Certs/votes
    pub quorum_membership: Arc<TYPES::Membership>,

    /// Membership for Timeout votes/certs
    pub timeout_membership: Arc<TYPES::Membership>,

    /// timeout task handle
    pub timeout_task: JoinHandle<()>,

    /// View timeout from config.
    pub timeout: u64,

    /// Round start delay from config, in milliseconds.
    pub round_start_delay: u64,

    /// Output events to application
    pub output_event_stream: async_broadcast::Sender<Event<TYPES>>,

    /// This node's storage ref
    pub storage: Arc<RwLock<I::Storage>>,

    /// last View Sync Certificate or Timeout Certificate this node formed.
    pub proposal_cert: Option<ViewChangeEvidence<TYPES>>,

    /// Spawned tasks related to a specific view, so we can cancel them when
    /// they are stale
    pub spawned_tasks: BTreeMap<TYPES::Time, Vec<JoinHandle<()>>>,

    /// Immutable instance state
    pub instance_state: Arc<TYPES::InstanceState>,

    /// The node's id
    pub id: u64,

    /// Globally shared reference to the current network version.
    pub version: Arc<RwLock<Version>>,

    /// An upgrade certificate that has been decided on, if any.
    pub decided_upgrade_certificate: Arc<RwLock<Option<UpgradeCertificate<TYPES>>>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> QuorumProposalRecvTaskState<TYPES, I> {
    /// Cancel all tasks the consensus tasks has spawned before the given view
    pub async fn cancel_tasks(&mut self, view: TYPES::Time) {
        let keep = self.spawned_tasks.split_off(&view);
        let mut cancel = Vec::new();
        while let Some((_, tasks)) = self.spawned_tasks.pop_first() {
            let mut to_cancel = tasks.into_iter().map(cancel_task).collect();
            cancel.append(&mut to_cancel);
        }
        self.spawned_tasks = keep;
        join_all(cancel).await;
    }

    /// Handles all consensus events relating to propose and vote-enabling events.
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "Consensus replica task", level = "error")]
    #[allow(unused_variables)]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    ) {
        #[cfg(feature = "dependency-tasks")]
        if let HotShotEvent::QuorumProposalRecv(proposal, sender) = event.as_ref() {
            match handle_quorum_proposal_recv(proposal, sender, &event_stream, self).await {
                Ok(QuorumProposalValidity::Fully) => {
                    self.cancel_tasks(proposal.data.view_number()).await;
                }
                Ok(QuorumProposalValidity::Liveness) => {
                    // Build the parent leaf since we didn't find it during the proposal check.
                    let parent_leaf = match parent_leaf_and_state(
                        proposal.data.view_number() + 1,
                        Arc::clone(&self.quorum_membership),
                        self.public_key.clone(),
                        OuterConsensus::new(Arc::clone(&self.consensus.inner_consensus)),
                    )
                    .await
                    {
                        Ok((parent_leaf, _ /* state */)) => parent_leaf,
                        Err(error) => {
                            warn!("Failed to get parent leaf and state during VoteNow data construction; error = {error:#}");
                            return;
                        }
                    };

                    let view_number = proposal.data.view_number();
                    self.cancel_tasks(view_number).await;
                    let consensus = self.consensus.read().await;
                    let Some(vid_shares) = consensus.vid_shares().get(&view_number) else {
                        debug!(
                                "We have not seen the VID share for this view {:?} yet, so we cannot vote.",
                                view_number
                            );
                        return;
                    };
                    let Some(vid_share) = vid_shares.get(&self.public_key) else {
                        error!("Did not get a VID share for our public key, aborting vote");
                        return;
                    };
                    let Some(da_cert) = consensus.saved_da_certs().get(&view_number) else {
                        debug!(
                            "Received VID share, but couldn't find DAC cert for view {:?}",
                            view_number
                        );
                        return;
                    };
                    broadcast_event(
                        Arc::new(HotShotEvent::VoteNow(
                            view_number,
                            VoteDependencyData {
                                quorum_proposal: proposal.data.clone(),
                                parent_leaf,
                                vid_share: vid_share.clone(),
                                da_cert: da_cert.clone(),
                            },
                        )),
                        &event_stream,
                    )
                    .await;
                }
                Err(e) => debug!(?e, "Failed to propose"),
            }
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> TaskState
    for QuorumProposalRecvTaskState<TYPES, I>
{
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

    async fn cancel_subtasks(&mut self) {
        while !self.spawned_tasks.is_empty() {
            let Some((_, handles)) = self.spawned_tasks.pop_first() else {
                break;
            };

            for handle in handles {
                #[cfg(async_executor_impl = "async-std")]
                handle.cancel().await;
                #[cfg(async_executor_impl = "tokio")]
                handle.abort();
            }
        }
    }
}
