use std::{marker::PhantomData, sync::Arc, time::Duration};

use async_broadcast::Sender;
use async_compatibility_layer::art::{async_sleep, async_spawn, async_timeout};
use async_lock::RwLock;
use hotshot_task::task::TaskState;
use hotshot_types::{
    consensus::Consensus,
    message::{CommitteeConsensusMessage, DataMessage, Message, MessageKind, SequencingMessage},
    traits::{
        election::Membership,
        network::{ConnectedNetwork, DataRequest, RequestKind, ResponseMessage},
        node_implementation::{NodeImplementation, NodeType},
        signature_key::SignatureKey,
    },
    vote::HasViewNumber,
};
use rand::{prelude::SliceRandom, thread_rng};
use sha2::{Digest, Sha256};
use tracing::{debug, error, info, instrument, warn};
use vbs::{version::StaticVersionType, BinarySerializer, Serializer};

use crate::{
    events::{HotShotEvent, HotShotTaskCompleted},
    helpers::broadcast_event,
};

/// Amount of time to try for a request before timing out.
const REQUEST_TIMEOUT: Duration = Duration::from_millis(500);

/// Long running task which will request information after a proposal is received.
/// The task will wait a it's `delay` and then send a request iteratively to peers
/// for any data they don't have related to the proposal.  For now it's just requesting VID
/// shares.
pub struct NetworkRequestState<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    Ver: StaticVersionType,
> {
    /// Network to send requests over
    pub network: Arc<I::QuorumNetwork>,
    /// Consensus shared state so we can check if we've gotten the information
    /// before sending a request
    pub state: Arc<RwLock<Consensus<TYPES>>>,
    /// Last seen view, we won't request for proposals before older than this view
    pub view: TYPES::Time,
    /// Delay before requesting peers
    pub delay: Duration,
    /// Committee
    pub da_membership: TYPES::Membership,
    /// Quorum
    pub quorum_membership: TYPES::Membership,
    /// This nodes public key
    pub public_key: TYPES::SignatureKey,
    /// This nodes private/signign key, used to sign requests.
    pub private_key: <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    /// Version discrimination
    pub _phantom: PhantomData<fn(&Ver)>,
    /// The node's id
    pub id: u64,
}

/// Alias for a signature
type Signature<TYPES> =
    <<TYPES as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType;

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, Ver: StaticVersionType + 'static> TaskState
    for NetworkRequestState<TYPES, I, Ver>
{
    type Event = Arc<HotShotEvent<TYPES>>;

    type Output = HotShotTaskCompleted;

    async fn handle_event(
        event: Self::Event,
        task: &mut hotshot_task::task::Task<Self>,
    ) -> Option<Self::Output> {
        match event.as_ref() {
            HotShotEvent::QuorumProposalValidated(proposal, _) => {
                let state = task.state();
                let prop_view = proposal.get_view_number();
                if prop_view >= state.view {
                    state
                        .spawn_requests(prop_view, task.clone_sender(), Ver::instance())
                        .await;
                }
                None
            }
            HotShotEvent::ViewChange(view) => {
                let view = *view;
                if view > task.state().view {
                    task.state_mut().view = view;
                }
                None
            }
            HotShotEvent::Shutdown => Some(HotShotTaskCompleted),
            _ => None,
        }
    }

    fn should_shutdown(event: &Self::Event) -> bool {
        matches!(event.as_ref(), HotShotEvent::Shutdown)
    }

    fn filter(&self, event: &Self::Event) -> bool {
        !matches!(
            event.as_ref(),
            HotShotEvent::Shutdown
                | HotShotEvent::QuorumProposalValidated(..)
                | HotShotEvent::ViewChange(_)
        )
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, Ver: StaticVersionType + 'static>
    NetworkRequestState<TYPES, I, Ver>
{
    /// Spawns tasks for a given view to retrieve any data needed.
    async fn spawn_requests(
        &self,
        view: TYPES::Time,
        sender: Sender<Arc<HotShotEvent<TYPES>>>,
        bind_version: Ver,
    ) {
        let requests = self.build_requests(view, bind_version).await;
        if requests.is_empty() {
            return;
        }
        requests
            .into_iter()
            .for_each(|r| self.run_delay(r, sender.clone(), view, bind_version));
    }

    /// Creates the srequest structures for all types that are needed.
    async fn build_requests(&self, view: TYPES::Time, _: Ver) -> Vec<RequestKind<TYPES>> {
        let mut reqs = Vec::new();
        if !self.state.read().await.vid_shares.contains_key(&view) {
            reqs.push(RequestKind::VID(view, self.public_key.clone()));
        }
        // TODO request other things
        reqs
    }

    /// run a delayed request task for a request.  The first response
    /// received will be sent over `sender`
    #[instrument(skip_all, fields(id = self.id, view = *self.view), name = "NetworkRequestState run_delay", level = "error")]
    fn run_delay(
        &self,
        request: RequestKind<TYPES>,
        sender: Sender<Arc<HotShotEvent<TYPES>>>,
        view: TYPES::Time,
        _: Ver,
    ) {
        let mut recipients: Vec<_> = self
            .da_membership
            .get_whole_committee(view)
            .into_iter()
            .collect();
        // Randomize the recipients so all replicas don't overload the same 1 recipients
        // and so we don't implicitly rely on the same replica all the time.
        recipients.shuffle(&mut thread_rng());
        let requester = DelayedRequester::<TYPES, I> {
            network: Arc::clone(&self.network),
            state: Arc::clone(&self.state),
            sender,
            delay: self.delay,
            recipients,
        };
        let Ok(data) = Serializer::<Ver>::serialize(&request) else {
            tracing::error!("Failed to serialize request!");
            return;
        };
        let Ok(signature) = TYPES::SignatureKey::sign(&self.private_key, &Sha256::digest(data))
        else {
            error!("Failed to sign Data Request");
            return;
        };
        debug!("Requesting data: {:?}", request);
        async_spawn(requester.run::<Ver>(request, signature));
    }
}

/// A short lived task that waits a delay and starts trying peers until it completes
/// a request.  If at any point the requested info is seen in the data stores or
/// the view has moved beyond the view we are requesting, the task will completed.
struct DelayedRequester<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Network to send requests
    network: Arc<I::QuorumNetwork>,
    /// Shared state to check if the data go populated
    state: Arc<RwLock<Consensus<TYPES>>>,
    /// Channel to send the event when we receive a response
    sender: Sender<Arc<HotShotEvent<TYPES>>>,
    /// Duration to delay sending the first request
    delay: Duration,
    /// The peers we will request in a random order
    recipients: Vec<TYPES::SignatureKey>,
}

/// Wrapper for the info in a VID request
struct VidRequest<TYPES: NodeType>(TYPES::Time, TYPES::SignatureKey);

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> DelayedRequester<TYPES, I> {
    /// Wait the delay, then try to complete the request.  Iterates over peers
    /// until the request is completed, or the data is no longer needed.
    async fn run<Ver: StaticVersionType + 'static>(
        mut self,
        request: RequestKind<TYPES>,
        signature: Signature<TYPES>,
    ) {
        // Do the delay only if primary is up and then start sending
        if !self.network.is_primary_down() {
            async_sleep(self.delay).await;
        }
        match request {
            RequestKind::VID(view, key) => {
                self.do_vid::<Ver>(VidRequest(view, key), signature).await;
            }
            RequestKind::DAProposal(..) => {}
        }
    }

    /// Handle sending a VID Share request, runs the loop until the data exists
    async fn do_vid<Ver: StaticVersionType + 'static>(
        &mut self,
        req: VidRequest<TYPES>,
        signature: Signature<TYPES>,
    ) {
        let message = make_vid(&req, signature);

        while !self.recipients.is_empty() && !self.cancel_vid(&req).await {
            match async_timeout(
                REQUEST_TIMEOUT,
                self.network.request_data::<TYPES, Ver>(
                    message.clone(),
                    self.recipients.pop().unwrap(),
                    Ver::instance(),
                ),
            )
            .await
            {
                Ok(Ok(response)) => {
                    match response {
                        ResponseMessage::Found(data) => {
                            self.handle_response_message(data).await;
                            // keep trying, but expect the map to be populated, or view to increase
                            async_sleep(REQUEST_TIMEOUT).await;
                        }
                        ResponseMessage::NotFound => {
                            info!("Peer Responded they did not have the data");
                        }
                        ResponseMessage::Denied => {
                            error!("Request for data was denied by the receiver");
                        }
                    }
                }
                Ok(Err(e)) => {
                    warn!("Error Sending request.  Error: {:?}", e);
                }
                Err(_) => {
                    warn!("Request to other node timed out");
                }
            }
        }
    }
    /// Returns true if we got the data we wanted, or the view has moved on.
    async fn cancel_vid(&self, req: &VidRequest<TYPES>) -> bool {
        let view = req.0;
        let state = self.state.read().await;
        state.vid_shares.contains_key(&view) && state.cur_view() > view
    }

    /// Transform a response into a `HotShotEvent`
    async fn handle_response_message(&self, message: SequencingMessage<TYPES>) {
        let event = match message {
            SequencingMessage::Committee(CommitteeConsensusMessage::VidDisperseMsg(prop)) => {
                HotShotEvent::VIDShareRecv(prop)
            }
            _ => return,
        };
        broadcast_event(Arc::new(event), &self.sender).await;
    }
}

/// Make a VID Request Message to send
fn make_vid<TYPES: NodeType>(
    req: &VidRequest<TYPES>,
    signature: Signature<TYPES>,
) -> Message<TYPES> {
    let kind = RequestKind::VID(req.0, req.1.clone());
    let data_request = DataRequest {
        view: req.0,
        request: kind,
        signature,
    };
    Message {
        sender: req.1.clone(),
        kind: MessageKind::Data(DataMessage::RequestData(data_request)),
    }
}
