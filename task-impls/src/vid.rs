use crate::events::{HotShotEvent, HotShotTaskCompleted};
use crate::helpers::broadcast_event;
use async_broadcast::Sender;
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::spawn_blocking;

use futures::future::join_all;
use hotshot_task::task::{Task, TaskState};
use hotshot_types::{
    consensus::Consensus,
    data::{VidDisperse, VidDisperseShare},
    message::Proposal,
    traits::{
        consensus_api::ConsensusApi,
        election::Membership,
        network::{ConnectedNetwork, ConsensusIntentEvent},
        node_implementation::{NodeImplementation, NodeType},
        signature_key::SignatureKey,
    },
    vid::vid_scheme,
};
use jf_primitives::vid::VidScheme;
#[cfg(async_executor_impl = "tokio")]
use tokio::task::spawn_blocking;

use std::marker::PhantomData;
use std::sync::Arc;
use tracing::{debug, error, instrument, warn};

/// Tracks state of a VID task
pub struct VIDTaskState<
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
    /// Network for all nodes
    pub network: Arc<I::QuorumNetwork>,
    /// Membership for the quorum
    pub membership: Arc<TYPES::Membership>,
    /// This Nodes Public Key
    pub public_key: TYPES::SignatureKey,
    /// Our Private Key
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// The view and ID of the current vote collection task, if there is one.
    pub vote_collector: Option<(TYPES::Time, usize, usize)>,
    /// This state's ID
    pub id: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static>
    VIDTaskState<TYPES, I, A>
{
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "VID Main Task", level = "error")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    ) -> Option<HotShotTaskCompleted> {
        match event.as_ref() {
            HotShotEvent::TransactionsSequenced(encoded_transactions, metadata, view_number) => {
                let encoded_transactions = encoded_transactions.clone();
                // get the number of quorum committee members to be used for VID calculation
                let num_storage_nodes = self.membership.total_nodes();

                // calculate vid shares
                let vid_disperse = spawn_blocking(move || {
                    #[allow(clippy::panic)]
                    vid_scheme(num_storage_nodes).disperse(&encoded_transactions).unwrap_or_else(|err|panic!("VID disperse failure:\n\t(num_storage nodes,payload_byte_len)=({num_storage_nodes},{})\n\terror: : {err}", encoded_transactions.len()))
                })
                .await;

                #[cfg(async_executor_impl = "tokio")]
                // Unwrap here will just propagate any panic from the spawned task, it's not a new place we can panic.
                let vid_disperse = vid_disperse.unwrap();
                // send the commitment and metadata to consensus for block building
                broadcast_event(
                    Arc::new(HotShotEvent::SendPayloadCommitmentAndMetadata(
                        vid_disperse.commit,
                        metadata.clone(),
                        *view_number,
                    )),
                    &event_stream,
                )
                .await;

                // send the block to the VID dispersal function
                broadcast_event(
                    Arc::new(HotShotEvent::BlockReady(
                        VidDisperse::from_membership(*view_number, vid_disperse, &self.membership),
                        *view_number,
                    )),
                    &event_stream,
                )
                .await;
            }

            HotShotEvent::BlockReady(vid_disperse, view_number) => {
                let view_number = *view_number;
                let Ok(signature) = TYPES::SignatureKey::sign(
                    &self.private_key,
                    vid_disperse.payload_commitment.as_ref().as_ref(),
                ) else {
                    error!("VID: failed to sign dispersal payload");
                    return None;
                };
                debug!("publishing VID disperse for view {}", *view_number);
                broadcast_event(
                    Arc::new(HotShotEvent::VidDisperseSend(
                        Proposal {
                            signature,
                            data: vid_disperse.clone(),
                            _pd: PhantomData,
                        },
                        self.public_key.clone(),
                    )),
                    &event_stream,
                )
                .await;
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

                // Start polling for VID disperse for the new view
                self.network
                    .inject_consensus_info(ConsensusIntentEvent::PollForVIDDisperse(
                        *self.cur_view + 1,
                    ))
                    .await;

                // If we are not the next leader, we should exit
                if self.membership.get_leader(self.cur_view + 1) != self.public_key {
                    // panic!("We are not the DA leader for view {}", *self.cur_view + 1);
                    return None;
                }

                return None;
            }

            HotShotEvent::DAProposalValidated(proposal, _sender) => {
                let txns = proposal.data.encoded_transactions.clone();
                let num_nodes = self.membership.total_nodes();
                let vid_disperse = spawn_blocking(move || {
                    #[allow(clippy::panic)]
                    vid_scheme(num_nodes).disperse(&txns).unwrap_or_else(|err|panic!("VID disperse failure:\n\t(num_storage nodes,payload_byte_len)=({num_nodes},{})\n\terror: : {err}", txns.len()))
                })
                .await;
                #[cfg(async_executor_impl = "tokio")]
                let vid_disperse = vid_disperse.unwrap();

                let vid_disperse = VidDisperse::from_membership(
                    proposal.data.view_number,
                    vid_disperse,
                    &self.membership,
                );

                let vid_disperse_tasks = VidDisperseShare::from_vid_disperse(vid_disperse)
                    .into_iter()
                    .filter_map(|vid_share| {
                        Some(broadcast_event(
                            Arc::new(HotShotEvent::VidDisperseRecv(
                                vid_share.to_proposal(&self.private_key)?,
                            )),
                            &event_stream,
                        ))
                    });

                join_all(vid_disperse_tasks).await;
            }

            HotShotEvent::Shutdown => {
                return Some(HotShotTaskCompleted);
            }
            _ => {
                error!("unexpected event {:?}", event);
            }
        }
        None
    }
}

/// task state implementation for VID Task
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static> TaskState
    for VIDTaskState<TYPES, I, A>
{
    type Event = Arc<HotShotEvent<TYPES>>;

    type Output = HotShotTaskCompleted;

    async fn handle_event(
        event: Self::Event,
        task: &mut Task<Self>,
    ) -> Option<HotShotTaskCompleted> {
        let sender = task.clone_sender();
        task.state_mut().handle(event, sender).await;
        None
    }
    fn filter(&self, event: &Self::Event) -> bool {
        !matches!(
            event.as_ref(),
            HotShotEvent::Shutdown
                | HotShotEvent::TransactionsSequenced(_, _, _)
                | HotShotEvent::BlockReady(_, _)
                | HotShotEvent::ViewChange(_)
                | HotShotEvent::DAProposalValidated(_, _)
        )
    }
    fn should_shutdown(event: &Self::Event) -> bool {
        matches!(event.as_ref(), HotShotEvent::Shutdown)
    }
}
