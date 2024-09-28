// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::sync::Arc;

use anyhow::Result;
use async_broadcast::{Receiver, Sender};
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use async_trait::async_trait;
use hotshot_task::task::TaskState;
use hotshot_types::{
    consensus::OuterConsensus,
    event::Event,
    message::UpgradeLock,
    simple_certificate::{QuorumCertificate, TimeoutCertificate},
    simple_vote::{QuorumVote, TimeoutVote},
    traits::{
        node_implementation::{NodeImplementation, NodeType, Versions},
        signature_key::SignatureKey,
    },
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::instrument;

use self::handlers::{
    handle_quorum_vote_recv, handle_timeout, handle_timeout_vote_recv, handle_view_change,
};
use crate::{events::HotShotEvent, vote_collection::VoteCollectorsMap};

/// Event handlers for use in the `handle` method.
mod handlers;

/// Task state for the Consensus task.
pub struct Consensus2TaskState<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> {
    /// Our public key
    pub public_key: TYPES::SignatureKey,

    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,

    /// Immutable instance state
    pub instance_state: Arc<TYPES::InstanceState>,

    /// The underlying network
    pub network: Arc<I::Network>,

    /// Membership for Timeout votes/certs
    pub timeout_membership: Arc<TYPES::Membership>,

    /// Membership for Quorum Certs/votes
    pub quorum_membership: Arc<TYPES::Membership>,

    /// Membership for DA committee Votes/certs
    pub committee_membership: Arc<TYPES::Membership>,

    /// A map of `QuorumVote` collector tasks.
    pub vote_collectors: VoteCollectorsMap<TYPES, QuorumVote<TYPES>, QuorumCertificate<TYPES>, V>,

    /// A map of `TimeoutVote` collector tasks.
    pub timeout_vote_collectors:
        VoteCollectorsMap<TYPES, TimeoutVote<TYPES>, TimeoutCertificate<TYPES>, V>,

    /// This node's storage ref
    pub storage: Arc<RwLock<I::Storage>>,

    /// The view number that this node is currently executing in.
    pub cur_view: TYPES::Time,

    /// Timestamp this view starts at.
    pub cur_view_time: i64,

    /// Output events to application
    pub output_event_stream: async_broadcast::Sender<Event<TYPES>>,

    /// Timeout task handle
    pub timeout_task: JoinHandle<()>,

    /// View timeout from config.
    pub timeout: u64,

    /// A reference to the metrics trait.
    pub consensus: OuterConsensus<TYPES>,

    /// The last decided view
    pub last_decided_view: TYPES::Time,

    /// The node's id
    pub id: u64,

    /// Lock for a decided upgrade
    pub upgrade_lock: UpgradeLock<TYPES, V>,
}
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> Consensus2TaskState<TYPES, I, V> {
    /// Handles a consensus event received on the event stream
    #[instrument(skip_all, fields(id = self.id, cur_view = *self.cur_view, last_decided_view = *self.last_decided_view), name = "Consensus replica task", level = "error", target = "Consensus2TaskState")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        sender: Sender<Arc<HotShotEvent<TYPES>>>,
    ) {
        match event.as_ref() {
            HotShotEvent::QuorumVoteRecv(ref vote) => {
                if let Err(e) =
                    handle_quorum_vote_recv(vote, Arc::clone(&event), &sender, self).await
                {
                    tracing::debug!("Failed to handle QuorumVoteRecv event; error = {e}");
                }
            }
            HotShotEvent::TimeoutVoteRecv(ref vote) => {
                if let Err(e) =
                    handle_timeout_vote_recv(vote, Arc::clone(&event), &sender, self).await
                {
                    tracing::debug!("Failed to handle TimeoutVoteRecv event; error = {e}");
                }
            }
            HotShotEvent::ViewChange(new_view_number) => {
                if let Err(e) = handle_view_change(*new_view_number, &sender, self).await {
                    tracing::trace!("Failed to handle ViewChange event; error = {e}");
                }
            }
            HotShotEvent::Timeout(view_number) => {
                if let Err(e) = handle_timeout(*view_number, &sender, self).await {
                    tracing::debug!("Failed to handle Timeout event; error = {e}");
                }
            }
            HotShotEvent::LastDecidedViewUpdated(view_number) => {
                if *view_number < self.last_decided_view {
                    tracing::debug!("New decided view is not newer than ours");
                } else {
                    self.last_decided_view = *view_number;
                    if let Err(e) = self
                        .consensus
                        .write()
                        .await
                        .update_last_decided_view(*view_number)
                    {
                        tracing::trace!("{e:?}");
                    }
                }
            }
            _ => {}
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, V: Versions> TaskState
    for Consensus2TaskState<TYPES, I, V>
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

    /// Joins all subtasks.
    async fn cancel_subtasks(&mut self) {}
}
