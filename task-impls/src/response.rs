use std::collections::HashMap;
use std::sync::Arc;

use async_broadcast::Receiver;
use async_compatibility_layer::art::async_spawn;
use async_lock::RwLock;
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use bincode::config::Options;
use either::Either::Right;
use futures::{channel::mpsc, FutureExt, StreamExt};
use hotshot_task::dependency::{Dependency, EventDependency};
use hotshot_types::{
    consensus::Consensus,
    data::VidDisperseShare,
    message::{
        CommitteeConsensusMessage, DataMessage, Message, MessageKind, Proposal, SequencingMessage,
    },
    traits::{
        election::Membership,
        network::{DataRequest, RequestKind, ResponseChannel, ResponseMessage},
        node_implementation::NodeType,
        signature_key::SignatureKey,
    },
    utils::bincode_opts,
};
use sha2::{Digest, Sha256};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;

use crate::events::HotShotEvent;

/// Type alias for consensus state wrapped in a lock.
type LockedConsensusState<TYPES> = Arc<RwLock<Consensus<TYPES>>>;

/// Type alias for the channel that we receive requests from the network on.
pub type RequestReceiver<TYPES> = mpsc::Receiver<(Message<TYPES>, ResponseChannel<Message<TYPES>>)>;

/// Task state for the Network Request Task. The task is responsible for handling
/// requests sent to this node by the network.  It will validate the sender,
/// parse the request, and try to find the data request in the consensus stores.
pub struct NetworkResponseState<TYPES: NodeType> {
    /// Locked consensus state
    consensus: LockedConsensusState<TYPES>,
    /// Receiver for requests
    receiver: RequestReceiver<TYPES>,
    /// Quorum membership for checking if requesters have state
    quorum: TYPES::Membership,
    /// This replicas public key
    pub_key: TYPES::SignatureKey,
}

impl<TYPES: NodeType> NetworkResponseState<TYPES> {
    /// Create the network request state with the info it needs
    pub fn new(
        consensus: LockedConsensusState<TYPES>,
        receiver: RequestReceiver<TYPES>,
        quorum: TYPES::Membership,
        pub_key: TYPES::SignatureKey,
    ) -> Self {
        Self {
            consensus,
            receiver,
            quorum,
            pub_key,
        }
    }

    /// Run the request response loop until a `HotShotEvent::Shutdown` is received.
    /// Or the stream is closed.
    async fn run_loop(mut self, shutdown: EventDependency<Arc<HotShotEvent<TYPES>>>) {
        let mut shutdown = Box::pin(shutdown.completed().fuse());
        loop {
            futures::select! {
                req = self.receiver.next() => {
                    match req {
                        Some((msg, chan)) => self.handle_message(msg, chan).await,
                        None => return,
                    }
                },
                _ = shutdown => {
                    return;
                }
            }
        }
    }

    /// Handle an incoming message.  First validates the sender, then handles the contained request.
    /// Sends the response via `chan`
    async fn handle_message(&self, req: Message<TYPES>, chan: ResponseChannel<Message<TYPES>>) {
        let sender = req.sender.clone();
        if !self.valid_sender(&sender) {
            let _ = chan.0.send(self.make_msg(ResponseMessage::Denied));
            return;
        }

        match req.kind {
            MessageKind::Data(DataMessage::RequestData(req)) => {
                if !valid_signature(&req, &sender) {
                    let _ = chan.0.send(self.make_msg(ResponseMessage::Denied));
                    return;
                }
                let response = self.handle_request(req).await;
                let _ = chan.0.send(response);
            }
            msg => tracing::error!(
                "Received message that wasn't a DataRequest in the request task.  Message: {:?}",
                msg
            ),
        }
    }

    /// Handle the request contained in the message. Returns the response we should send
    /// First parses the kind and passes to the appropriate handler for the specific type
    /// of the request.
    async fn handle_request(&self, req: DataRequest<TYPES>) -> Message<TYPES> {
        match req.request {
            RequestKind::VID(view, pub_key) => {
                let state = self.consensus.read().await;
                let Some(proposals_map) = state.vid_shares.get(&view) else {
                    return self.make_msg(ResponseMessage::NotFound);
                };
                self.handle_vid(proposals_map, &pub_key)
            }
            // TODO impl for DA Proposal: https://github.com/EspressoSystems/HotShot/issues/2651
            RequestKind::DAProposal(_view) => self.make_msg(ResponseMessage::NotFound),
        }
    }

    /// Handle a vid request by looking up the the share for the key.  If a share is found
    /// build the response and return it
    fn handle_vid(
        &self,
        proposals_map: &HashMap<TYPES::SignatureKey, Proposal<TYPES, VidDisperseShare<TYPES>>>,
        key: &TYPES::SignatureKey,
    ) -> Message<TYPES> {
        if !proposals_map.contains_key(key) {
            return self.make_msg(ResponseMessage::NotFound);
        }
        let seq_msg = SequencingMessage(Right(CommitteeConsensusMessage::VidDisperseMsg(
            proposals_map.get(key).unwrap().clone(),
        )));
        self.make_msg(ResponseMessage::Found(seq_msg))
    }

    /// Helper to turn a `ResponseMessage` into a `Message` by filling
    /// in the surrounding feilds and creating the `MessageKind`
    fn make_msg(&self, msg: ResponseMessage<TYPES>) -> Message<TYPES> {
        Message {
            sender: self.pub_key.clone(),
            kind: MessageKind::Data(DataMessage::DataResponse(msg)),
        }
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
    let Ok(data) = bincode_opts().serialize(&req.request) else {
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
) -> JoinHandle<()> {
    let dep = EventDependency::new(
        event_stream,
        Box::new(|e| matches!(e.as_ref(), HotShotEvent::Shutdown)),
    );
    async_spawn(task_state.run_loop(dep))
}
