use crate::events::HotShotEvent;
use async_lock::RwLock;
use hotshot_task::{
    event_stream::ChannelStream,
    global_registry::GlobalRegistry,
    task::{HotShotTaskCompleted, TS},
    task_impls::HSTWithEvent,
};
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

use hotshot_task::event_stream::EventStream;
use snafu::Snafu;
use std::marker::PhantomData;
use std::sync::Arc;
use tracing::{debug, error, instrument};

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
    /// Global registry task for the state
    pub registry: GlobalRegistry,

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

    /// Global events stream to publish events
    pub event_stream: ChannelStream<HotShotEvent<TYPES>>,

    /// This state's ID
    pub id: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static>
    VIDTaskState<TYPES, I, A>
{
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "VID Main Task", level = "error")]
    pub async fn handle_event(
        &mut self,
        event: HotShotEvent<TYPES>,
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
                let vid = VidScheme::new(chunk_size, num_quorum_committee, &srs).unwrap();
                let vid_disperse = vid.disperse(encoded_transactions.clone()).unwrap();

                // send the commitment and metadata to consensus for block building
                self.event_stream
                    .publish(HotShotEvent::SendPayloadCommitmentAndMetadata(
                        vid_disperse.commit,
                        metadata,
                    ))
                    .await;

                // send the block to the VID dispersal function
                self.event_stream
                    .publish(HotShotEvent::BlockReady(
                        VidDisperse::from_membership(view_number, vid_disperse, &self.membership),
                        view_number,
                    ))
                    .await;
            }

            HotShotEvent::BlockReady(vid_disperse, view_number) => {
                debug!("publishing VID disperse for view {}", *view_number);
                self.event_stream
                    .publish(HotShotEvent::VidDisperseSend(
                        Proposal {
                            signature: TYPES::SignatureKey::sign(
                                &self.private_key,
                                &vid_disperse.payload_commitment,
                            ),
                            data: vid_disperse,
                            _pd: PhantomData,
                        },
                        self.public_key.clone(),
                    ))
                    .await;
            }

            HotShotEvent::ViewChange(view) => {
                if *self.cur_view >= *view {
                    return None;
                }

                if *view - *self.cur_view > 1 {
                    error!("View changed by more than 1 going to view {:?}", view);
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
                return Some(HotShotTaskCompleted::ShutDown);
            }
            _ => {
                error!("unexpected event {:?}", event);
            }
        }
        None
    }

    /// Filter the VID event.
    pub fn filter(event: &HotShotEvent<TYPES>) -> bool {
        matches!(
            event,
            HotShotEvent::Shutdown
                | HotShotEvent::TransactionsSequenced(_, _, _)
                | HotShotEvent::BlockReady(_, _)
                | HotShotEvent::ViewChange(_)
        )
    }
}

/// task state implementation for VID Task
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, A: ConsensusApi<TYPES, I> + 'static> TS
    for VIDTaskState<TYPES, I, A>
{
}

/// Type alias for VID Task Types
pub type VIDTaskTypes<TYPES, I, A> = HSTWithEvent<
    ConsensusTaskError,
    HotShotEvent<TYPES>,
    ChannelStream<HotShotEvent<TYPES>>,
    VIDTaskState<TYPES, I, A>,
>;
