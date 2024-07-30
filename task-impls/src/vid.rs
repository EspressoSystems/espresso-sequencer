use std::{marker::PhantomData, sync::Arc};

use anyhow::Result;
use async_broadcast::{Receiver, Sender};
use async_trait::async_trait;
use hotshot_task::task::TaskState;
use hotshot_types::{
    consensus::OuterConsensus,
    data::{PackedBundle, VidDisperse, VidDisperseShare},
    message::Proposal,
    traits::{
        election::Membership,
        node_implementation::{NodeImplementation, NodeType},
        signature_key::SignatureKey,
        BlockPayload,
    },
};
use tracing::{debug, error, instrument, warn};

use crate::{
    events::{HotShotEvent, HotShotTaskCompleted},
    helpers::broadcast_event,
};

/// Tracks state of a VID task
pub struct VidTaskState<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// View number this view is executing in.
    pub cur_view: TYPES::Time,
    /// Reference to consensus. Leader will require a read lock on this.
    pub consensus: OuterConsensus<TYPES>,
    /// The underlying network
    pub network: Arc<I::Network>,
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

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> VidTaskState<TYPES, I> {
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view), name = "VID Main Task", level = "error", target = "VidTaskState")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
    ) -> Option<HotShotTaskCompleted> {
        match event.as_ref() {
            HotShotEvent::BlockRecv(packed_bundle) => {
                let PackedBundle::<TYPES> {
                    encoded_transactions,
                    metadata,
                    view_number,
                    sequencing_fees,
                    vid_precompute,
                    auction_result,
                    ..
                } = packed_bundle;
                let payload =
                    <TYPES as NodeType>::BlockPayload::from_bytes(encoded_transactions, metadata);
                let builder_commitment = payload.builder_commitment(metadata);
                let vid_disperse = VidDisperse::calculate_vid_disperse(
                    Arc::clone(encoded_transactions),
                    &Arc::clone(&self.membership),
                    *view_number,
                    vid_precompute.clone(),
                )
                .await;
                let payload_commitment = vid_disperse.payload_commitment;
                let shares = VidDisperseShare::from_vid_disperse(vid_disperse.clone());
                let mut consensus = self.consensus.write().await;
                for share in shares {
                    if let Some(disperse) = share.to_proposal(&self.private_key) {
                        consensus.update_vid_shares(*view_number, disperse);
                    }
                }
                drop(consensus);

                // send the commitment and metadata to consensus for block building
                broadcast_event(
                    Arc::new(HotShotEvent::SendPayloadCommitmentAndMetadata(
                        payload_commitment,
                        builder_commitment,
                        metadata.clone(),
                        *view_number,
                        sequencing_fees.clone(),
                        auction_result.clone(),
                    )),
                    &event_stream,
                )
                .await;

                // send the block to the VID dispersal function
                broadcast_event(
                    Arc::new(HotShotEvent::BlockReady(vid_disperse, *view_number)),
                    &event_stream,
                )
                .await;
            }

            HotShotEvent::BlockReady(vid_disperse, view_number) => {
                let view_number = *view_number;
                let Ok(signature) = TYPES::SignatureKey::sign(
                    &self.private_key,
                    vid_disperse.payload_commitment.as_ref(),
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

                // If we are not the next leader, we should exit
                if self.membership.leader(self.cur_view + 1) != self.public_key {
                    // panic!("We are not the DA leader for view {}", *self.cur_view + 1);
                    return None;
                }

                return None;
            }

            HotShotEvent::Shutdown => {
                return Some(HotShotTaskCompleted);
            }
            _ => {}
        }
        None
    }
}

#[async_trait]
/// task state implementation for VID Task
impl<TYPES: NodeType, I: NodeImplementation<TYPES>> TaskState for VidTaskState<TYPES, I> {
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

    async fn cancel_subtasks(&mut self) {}
}
