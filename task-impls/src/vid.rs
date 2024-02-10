use crate::events::{HotShotEvent, HotShotTaskCompleted};
use crate::helpers::broadcast_event;
use async_broadcast::Sender;
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::spawn_blocking;

use hotshot_task::task::{Task, TaskState};
use hotshot_types::traits::network::CommunicationChannel;
use hotshot_types::{
    consensus::Consensus,
    data::VidDisperse,
    message::Proposal,
    traits::{
        consensus_api::ConsensusApi,
        election::Membership,
        node_implementation::{NodeImplementation, NodeType},
        signature_key::SignatureKey,
    },
};
use hotshot_types::{
    data::{test_srs, VidScheme, VidSchemeTrait},
    traits::network::ConsensusIntentEvent,
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::spawn_blocking;

use snafu::Snafu;
use std::marker::PhantomData;
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
        event: HotShotEvent<TYPES>,
        event_stream: Sender<HotShotEvent<TYPES>>,
    ) -> Option<HotShotTaskCompleted> {
        match event {
            HotShotEvent::TransactionsSequenced(encoded_transactions, metadata, view_number) => {
                // get the number of quorum committee members to be used for VID calculation
                let num_quorum_committee = self.membership.total_nodes();

                // TODO <https://github.com/EspressoSystems/HotShot/issues/1686>
                let srs = test_srs(num_quorum_committee);

                // calculate the last power of two
                // TODO change after https://github.com/EspressoSystems/jellyfish/issues/339
                // issue: https://github.com/EspressoSystems/HotShot/issues/2152
                let chunk_size = 1 << num_quorum_committee.ilog2();

                // calculate vid shares
                let vid_disperse = spawn_blocking(move || {
                    let vid = VidScheme::new(chunk_size, num_quorum_committee, &srs).unwrap();
                    vid.disperse(encoded_transactions.clone()).unwrap()
                })
                .await;

                #[cfg(async_executor_impl = "tokio")]
                // Unwrap here will just propogate any panic from the spawned task, it's not a new place we can panic.
                let vid_disperse = vid_disperse.unwrap();
                // send the commitment and metadata to consensus for block building
                broadcast_event(
                    HotShotEvent::SendPayloadCommitmentAndMetadata(
                        vid_disperse.commit,
                        metadata,
                        view_number,
                    ),
                    &event_stream,
                )
                .await;

                // send the block to the VID dispersal function
                broadcast_event(
                    HotShotEvent::BlockReady(
                        VidDisperse::from_membership(view_number, vid_disperse, &self.membership),
                        view_number,
                    ),
                    &event_stream,
                )
                .await;
            }

            HotShotEvent::BlockReady(vid_disperse, view_number) => {
                let Ok(signature) = TYPES::SignatureKey::sign(
                    &self.private_key,
                    vid_disperse.payload_commitment.as_ref().as_ref(),
                ) else {
                    error!("VID: failed to sign dispersal payload");
                    return None;
                };
                debug!("publishing VID disperse for view {}", *view_number);
                broadcast_event(
                    HotShotEvent::VidDisperseSend(
                        Proposal {
                            signature,
                            data: vid_disperse,
                            _pd: PhantomData,
                        },
                        self.public_key.clone(),
                    ),
                    &event_stream,
                )
                .await;
            }

            HotShotEvent::ViewChange(view) => {
                if *self.cur_view >= *view {
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
    type Event = HotShotEvent<TYPES>;

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
            event,
            HotShotEvent::Shutdown
                | HotShotEvent::TransactionsSequenced(_, _, _)
                | HotShotEvent::BlockReady(_, _)
                | HotShotEvent::ViewChange(_)
        )
    }
    fn should_shutdown(event: &Self::Event) -> bool {
        matches!(event, HotShotEvent::Shutdown)
    }
}
