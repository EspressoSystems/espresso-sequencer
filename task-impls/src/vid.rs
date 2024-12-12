// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{marker::PhantomData, sync::Arc};

use async_broadcast::{Receiver, Sender};
use async_trait::async_trait;
use hotshot_task::task::TaskState;
use hotshot_types::{
    consensus::OuterConsensus,
    data::{PackedBundle, VidDisperse, VidDisperseShare2},
    message::Proposal,
    traits::{
        election::Membership,
        node_implementation::{NodeImplementation, NodeType},
        signature_key::SignatureKey,
        BlockPayload,
    },
};
use tracing::{debug, error, info, instrument};
use utils::anytrace::Result;

use crate::{
    events::{HotShotEvent, HotShotTaskCompleted},
    helpers::broadcast_event,
};

/// Tracks state of a VID task
pub struct VidTaskState<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// View number this view is executing in.
    pub cur_view: TYPES::View,

    /// Epoch number this node is executing in.
    pub cur_epoch: TYPES::Epoch,

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

    /// This state's ID
    pub id: u64,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> VidTaskState<TYPES, I> {
    /// main task event handler
    #[instrument(skip_all, fields(id = self.id, view = *self.cur_view, epoch = *self.cur_epoch), name = "VID Main Task", level = "error", target = "VidTaskState")]
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
                let epoch = self.cur_epoch;
                if self.membership.leader(*view_number, epoch).ok()? != self.public_key {
                    tracing::debug!(
                        "We are not the leader in the current epoch. Do not send the VID dispersal."
                    );
                    return None;
                }
                let vid_disperse = VidDisperse::calculate_vid_disperse(
                    Arc::clone(encoded_transactions),
                    &Arc::clone(&self.membership),
                    *view_number,
                    epoch,
                    vid_precompute.clone(),
                )
                .await;
                let payload_commitment = vid_disperse.payload_commitment;
                let shares = VidDisperseShare2::from_vid_disperse(vid_disperse.clone());
                let mut consensus_writer = self.consensus.write().await;
                for share in shares {
                    if let Some(disperse) = share.to_proposal(&self.private_key) {
                        consensus_writer.update_vid_shares(*view_number, disperse);
                    }
                }
                drop(consensus_writer);

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

            HotShotEvent::ViewChange(view, epoch) => {
                if *epoch > self.cur_epoch {
                    self.cur_epoch = *epoch;
                }

                let view = *view;
                if (*view != 0 || *self.cur_view > 0) && *self.cur_view >= *view {
                    return None;
                }

                if *view - *self.cur_view > 1 {
                    info!("View changed by more than 1 going to view {:?}", view);
                }
                self.cur_view = view;

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

    fn cancel_subtasks(&mut self) {}
}
