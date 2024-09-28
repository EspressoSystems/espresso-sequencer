// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{sync::Arc, time::Duration};

use async_broadcast::{Receiver, Sender};
use async_compatibility_layer::art::{async_sleep, async_spawn};
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use committable::Committable;
use hotshot_types::{
    consensus::{Consensus, LockedConsensusState, OuterConsensus},
    data::VidDisperseShare,
    message::Proposal,
    traits::{
        election::Membership, network::DataRequest, node_implementation::NodeType,
        signature_key::SignatureKey,
    },
};
use sha2::{Digest, Sha256};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::instrument;

use crate::{events::HotShotEvent, helpers::broadcast_event};
/// Time to wait for txns before sending `ResponseMessage::NotFound`
const TXNS_TIMEOUT: Duration = Duration::from_millis(100);

/// Task state for the Network Request Task. The task is responsible for handling
/// requests sent to this node by the network.  It will validate the sender,
/// parse the request, and try to find the data request in the consensus stores.
pub struct NetworkResponseState<TYPES: NodeType> {
    /// Locked consensus state
    consensus: LockedConsensusState<TYPES>,
    /// Quorum membership for checking if requesters have state
    quorum: Arc<TYPES::Membership>,
    /// This replicas public key
    pub_key: TYPES::SignatureKey,
    /// This replicas private key
    private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// The node's id
    id: u64,
}

impl<TYPES: NodeType> NetworkResponseState<TYPES> {
    /// Create the network request state with the info it needs
    pub fn new(
        consensus: LockedConsensusState<TYPES>,
        quorum: Arc<TYPES::Membership>,
        pub_key: TYPES::SignatureKey,
        private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
        id: u64,
    ) -> Self {
        Self {
            consensus,
            quorum,
            pub_key,
            private_key,
            id,
        }
    }

    /// Process request events or loop until a `HotShotEvent::Shutdown` is received.
    async fn run_response_loop(
        self,
        mut receiver: Receiver<Arc<HotShotEvent<TYPES>>>,
        event_sender: Sender<Arc<HotShotEvent<TYPES>>>,
    ) {
        loop {
            match receiver.recv_direct().await {
                Ok(event) => {
                    // break loop when false, this means shutdown received
                    match event.as_ref() {
                        HotShotEvent::VidRequestRecv(request, sender) => {
                            // Verify request is valid
                            if !self.valid_sender(sender)
                                || !valid_signature::<TYPES>(request, sender)
                            {
                                continue;
                            }
                            if let Some(proposal) =
                                self.get_or_calc_vid_share(request.view, sender).await
                            {
                                broadcast_event(
                                    HotShotEvent::VidResponseSend(
                                        self.pub_key.clone(),
                                        sender.clone(),
                                        proposal,
                                    )
                                    .into(),
                                    &event_sender,
                                )
                                .await;
                            }
                        }
                        HotShotEvent::QuorumProposalRequestRecv(req, signature) => {
                            // Make sure that this request came from who we think it did
                            if !req.key.validate(signature, req.commit().as_ref()) {
                                tracing::warn!("Invalid signature key on proposal request.");
                                return;
                            }

                            if let Some(quorum_proposal) = self
                                .consensus
                                .read()
                                .await
                                .last_proposals()
                                .get(&req.view_number)
                            {
                                broadcast_event(
                                    HotShotEvent::QuorumProposalResponseSend(
                                        req.key.clone(),
                                        quorum_proposal.clone(),
                                    )
                                    .into(),
                                    &event_sender,
                                )
                                .await;
                            }
                        }
                        HotShotEvent::Shutdown => {
                            return;
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to receive event. {:?}", e);
                }
            }
        }
    }

    /// Get the VID share from consensus storage, or calculate it from the payload for
    /// the view, if we have the payload.  Stores all the shares calculated from the payload
    /// if the calculation was done
    #[instrument(skip_all, target = "NetworkResponseState", fields(id = self.id))]
    async fn get_or_calc_vid_share(
        &self,
        view: TYPES::Time,
        key: &TYPES::SignatureKey,
    ) -> Option<Proposal<TYPES, VidDisperseShare<TYPES>>> {
        let contained = self
            .consensus
            .read()
            .await
            .vid_shares()
            .get(&view)
            .is_some_and(|m| m.contains_key(key));
        if !contained {
            if Consensus::calculate_and_update_vid(
                OuterConsensus::new(Arc::clone(&self.consensus)),
                view,
                Arc::clone(&self.quorum),
                &self.private_key,
            )
            .await
            .is_none()
            {
                // Sleep in hope we receive txns in the meantime
                async_sleep(TXNS_TIMEOUT).await;
                Consensus::calculate_and_update_vid(
                    OuterConsensus::new(Arc::clone(&self.consensus)),
                    view,
                    Arc::clone(&self.quorum),
                    &self.private_key,
                )
                .await?;
            }
            return self
                .consensus
                .read()
                .await
                .vid_shares()
                .get(&view)?
                .get(key)
                .cloned();
        }
        self.consensus
            .read()
            .await
            .vid_shares()
            .get(&view)?
            .get(key)
            .cloned()
    }

    /// Makes sure the sender is allowed to send a request.
    fn valid_sender(&self, sender: &TYPES::SignatureKey) -> bool {
        self.quorum.has_stake(sender)
    }
}

/// Check the signature
fn valid_signature<TYPES: NodeType>(
    req: &DataRequest<TYPES>,
    sender: &TYPES::SignatureKey,
) -> bool {
    let Ok(data) = bincode::serialize(&req.request) else {
        return false;
    };
    sender.validate(&req.signature, &Sha256::digest(data))
}

/// Spawn the network response task to handle incoming request for data
/// from other nodes.  It will shutdown when it gets `HotshotEvent::Shutdown`
/// on the `event_stream` arg.
pub fn run_response_task<TYPES: NodeType>(
    task_state: NetworkResponseState<TYPES>,
    event_stream: Receiver<Arc<HotShotEvent<TYPES>>>,
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
) -> JoinHandle<()> {
    async_spawn(task_state.run_response_loop(event_stream, sender))
}
