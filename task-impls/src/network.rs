use crate::{
    events::{HotShotEvent, HotShotTaskCompleted},
    helpers::broadcast_event,
};
use async_broadcast::Sender;
use async_compatibility_layer::art::async_spawn;
use async_lock::RwLock;
use hotshot_types::{event::HotShotAction, traits::storage::Storage};
use std::collections::HashMap;
use std::sync::Arc;

use hotshot_task::task::{Task, TaskState};
use hotshot_types::constants::{BASE_VERSION, STATIC_VER_0_1};
use hotshot_types::{
    data::{VidDisperse, VidDisperseShare},
    message::{
        CommitteeConsensusMessage, DataMessage, GeneralConsensusMessage, Message, MessageKind,
        Proposal, SequencingMessage,
    },
    traits::{
        election::Membership,
        network::{ConnectedNetwork, TransmitType, ViewMessage},
        node_implementation::{ConsensusTime, NodeType},
    },
    vote::{HasViewNumber, Vote},
};
use tracing::instrument;
use tracing::{debug, error, warn};
use versioned_binary_serialization::version::Version;

/// quorum filter
pub fn quorum_filter<TYPES: NodeType>(event: &Arc<HotShotEvent<TYPES>>) -> bool {
    !matches!(
        event.as_ref(),
        HotShotEvent::QuorumProposalSend(_, _)
            | HotShotEvent::QuorumVoteSend(_)
            | HotShotEvent::DACSend(_, _)
            | HotShotEvent::TimeoutVoteSend(_)
    )
}

/// upgrade filter
pub fn upgrade_filter<TYPES: NodeType>(event: &Arc<HotShotEvent<TYPES>>) -> bool {
    !matches!(
        event.as_ref(),
        HotShotEvent::UpgradeProposalSend(_, _) | HotShotEvent::UpgradeVoteSend(_)
    )
}

/// committee filter
pub fn committee_filter<TYPES: NodeType>(event: &Arc<HotShotEvent<TYPES>>) -> bool {
    !matches!(
        event.as_ref(),
        HotShotEvent::DAProposalSend(_, _) | HotShotEvent::DAVoteSend(_)
    )
}

/// vid filter
pub fn vid_filter<TYPES: NodeType>(event: &Arc<HotShotEvent<TYPES>>) -> bool {
    !matches!(event.as_ref(), HotShotEvent::VidDisperseSend(_, _))
}

/// view sync filter
pub fn view_sync_filter<TYPES: NodeType>(event: &Arc<HotShotEvent<TYPES>>) -> bool {
    !matches!(
        event.as_ref(),
        HotShotEvent::ViewSyncPreCommitCertificate2Send(_, _)
            | HotShotEvent::ViewSyncCommitCertificate2Send(_, _)
            | HotShotEvent::ViewSyncFinalizeCertificate2Send(_, _)
            | HotShotEvent::ViewSyncPreCommitVoteSend(_)
            | HotShotEvent::ViewSyncCommitVoteSend(_)
            | HotShotEvent::ViewSyncFinalizeVoteSend(_)
    )
}
/// the network message task state
#[derive(Clone)]
pub struct NetworkMessageTaskState<TYPES: NodeType> {
    /// Sender to send internal events this task generates to other tasks
    pub event_stream: Sender<Arc<HotShotEvent<TYPES>>>,
}

impl<TYPES: NodeType> TaskState for NetworkMessageTaskState<TYPES> {
    type Event = Vec<Message<TYPES>>;
    type Output = ();

    async fn handle_event(event: Self::Event, task: &mut Task<Self>) -> Option<()>
    where
        Self: Sized,
    {
        task.state_mut().handle_messages(event).await;
        None
    }

    fn filter(&self, _event: &Self::Event) -> bool {
        false
    }

    fn should_shutdown(_event: &Self::Event) -> bool {
        false
    }
}

impl<TYPES: NodeType> NetworkMessageTaskState<TYPES> {
    /// Handle the message.
    pub async fn handle_messages(&mut self, messages: Vec<Message<TYPES>>) {
        // We will send only one event for a vector of transactions.
        let mut transactions = Vec::new();
        for message in messages {
            let sender = message.sender;
            match message.kind {
                MessageKind::Consensus(consensus_message) => {
                    let event = match consensus_message {
                        SequencingMessage::General(general_message) => match general_message {
                            GeneralConsensusMessage::Proposal(proposal) => {
                                HotShotEvent::QuorumProposalRecv(proposal, sender)
                            }
                            GeneralConsensusMessage::Vote(vote) => {
                                HotShotEvent::QuorumVoteRecv(vote.clone())
                            }
                            GeneralConsensusMessage::ViewSyncPreCommitVote(view_sync_message) => {
                                HotShotEvent::ViewSyncPreCommitVoteRecv(view_sync_message)
                            }
                            GeneralConsensusMessage::ViewSyncPreCommitCertificate(
                                view_sync_message,
                            ) => HotShotEvent::ViewSyncPreCommitCertificate2Recv(view_sync_message),

                            GeneralConsensusMessage::ViewSyncCommitVote(view_sync_message) => {
                                HotShotEvent::ViewSyncCommitVoteRecv(view_sync_message)
                            }
                            GeneralConsensusMessage::ViewSyncCommitCertificate(
                                view_sync_message,
                            ) => HotShotEvent::ViewSyncCommitCertificate2Recv(view_sync_message),

                            GeneralConsensusMessage::ViewSyncFinalizeVote(view_sync_message) => {
                                HotShotEvent::ViewSyncFinalizeVoteRecv(view_sync_message)
                            }
                            GeneralConsensusMessage::ViewSyncFinalizeCertificate(
                                view_sync_message,
                            ) => HotShotEvent::ViewSyncFinalizeCertificate2Recv(view_sync_message),

                            GeneralConsensusMessage::TimeoutVote(message) => {
                                HotShotEvent::TimeoutVoteRecv(message)
                            }
                            GeneralConsensusMessage::UpgradeProposal(message) => {
                                HotShotEvent::UpgradeProposalRecv(message, sender)
                            }
                            GeneralConsensusMessage::UpgradeVote(message) => {
                                error!("Received upgrade vote!");
                                HotShotEvent::UpgradeVoteRecv(message)
                            }
                        },
                        SequencingMessage::Committee(committee_message) => {
                            match committee_message {
                                CommitteeConsensusMessage::DAProposal(proposal) => {
                                    HotShotEvent::DAProposalRecv(proposal, sender)
                                }
                                CommitteeConsensusMessage::DAVote(vote) => {
                                    HotShotEvent::DAVoteRecv(vote.clone())
                                }
                                CommitteeConsensusMessage::DACertificate(cert) => {
                                    HotShotEvent::DACertificateRecv(cert)
                                }
                                CommitteeConsensusMessage::VidDisperseMsg(proposal) => {
                                    HotShotEvent::VidDisperseRecv(proposal)
                                }
                            }
                        }
                    };
                    // TODO (Keyao benchmarking) Update these event variants (similar to the
                    // `TransactionsRecv` event) so we can send one event for a vector of messages.
                    // <https://github.com/EspressoSystems/HotShot/issues/1428>
                    broadcast_event(Arc::new(event), &self.event_stream).await;
                }
                MessageKind::Data(message) => match message {
                    DataMessage::SubmitTransaction(transaction, _) => {
                        transactions.push(transaction);
                    }
                    DataMessage::DataResponse(_) | DataMessage::RequestData(_) => {
                        warn!("Request and Response messages should not be received in the NetworkMessage task");
                    }
                },
            };
        }
        if !transactions.is_empty() {
            broadcast_event(
                Arc::new(HotShotEvent::TransactionsRecv(transactions)),
                &self.event_stream,
            )
            .await;
        }
    }
}

/// network event task state
pub struct NetworkEventTaskState<
    TYPES: NodeType,
    COMMCHANNEL: ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>,
    S: Storage<TYPES>,
> {
    /// comm channel
    pub channel: Arc<COMMCHANNEL>,
    /// view number
    pub view: TYPES::Time,
    /// version
    pub version: Version,
    /// membership for the channel
    pub membership: TYPES::Membership,
    // TODO ED Need to add exchange so we can get the recipient key and our own key?
    /// Filter which returns false for the events that this specific network task cares about
    pub filter: fn(&Arc<HotShotEvent<TYPES>>) -> bool,
    /// Storage to store actionable events
    pub storage: Arc<RwLock<S>>,
}

impl<
        TYPES: NodeType,
        COMMCHANNEL: ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>,
        S: Storage<TYPES> + 'static,
    > TaskState for NetworkEventTaskState<TYPES, COMMCHANNEL, S>
{
    type Event = Arc<HotShotEvent<TYPES>>;

    type Output = HotShotTaskCompleted;

    async fn handle_event(
        event: Self::Event,
        task: &mut Task<Self>,
    ) -> Option<HotShotTaskCompleted> {
        let membership = task.state_mut().membership.clone();
        task.state_mut().handle_event(event, &membership).await
    }

    fn should_shutdown(event: &Self::Event) -> bool {
        if matches!(event.as_ref(), HotShotEvent::Shutdown) {
            error!("Network Task received Shutdown event");
            return true;
        }
        false
    }

    fn filter(&self, event: &Self::Event) -> bool {
        (self.filter)(event)
            && !matches!(
                event.as_ref(),
                HotShotEvent::VersionUpgrade(_)
                    | HotShotEvent::ViewChange(_)
                    | HotShotEvent::Shutdown
            )
    }
}

impl<
        TYPES: NodeType,
        COMMCHANNEL: ConnectedNetwork<Message<TYPES>, TYPES::SignatureKey>,
        S: Storage<TYPES> + 'static,
    > NetworkEventTaskState<TYPES, COMMCHANNEL, S>
{
    /// Handle the given event.
    ///
    /// Returns the completion status.
    #[allow(clippy::too_many_lines)] // TODO https://github.com/EspressoSystems/HotShot/issues/1704
    #[instrument(skip_all, fields(view = *self.view), name = "Network Task", level = "error")]
    pub async fn handle_event(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        membership: &TYPES::Membership,
    ) -> Option<HotShotTaskCompleted> {
        let mut maybe_action = None;
        let (sender, message_kind, transmit): (_, _, TransmitType<TYPES>) =
            match event.as_ref().clone() {
                HotShotEvent::QuorumProposalSend(proposal, sender) => {
                    maybe_action = Some(HotShotAction::Propose);
                    (
                        sender,
                        MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                            GeneralConsensusMessage::Proposal(proposal),
                        )),
                        TransmitType::Broadcast,
                    )
                }

                // ED Each network task is subscribed to all these message types.  Need filters per network task
                HotShotEvent::QuorumVoteSend(vote) => {
                    maybe_action = Some(HotShotAction::Vote);
                    (
                        vote.get_signing_key(),
                        MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                            GeneralConsensusMessage::Vote(vote.clone()),
                        )),
                        TransmitType::Direct(membership.get_leader(vote.get_view_number() + 1)),
                    )
                }
                HotShotEvent::VidDisperseSend(proposal, sender) => {
                    return self.handle_vid_disperse_proposal(proposal, &sender);
                }
                HotShotEvent::DAProposalSend(proposal, sender) => {
                    maybe_action = Some(HotShotAction::DAPropose);
                    (
                        sender,
                        MessageKind::<TYPES>::from_consensus_message(SequencingMessage::Committee(
                            CommitteeConsensusMessage::DAProposal(proposal),
                        )),
                        TransmitType::DACommitteeBroadcast,
                    )
                }
                HotShotEvent::DAVoteSend(vote) => {
                    maybe_action = Some(HotShotAction::DAVote);
                    (
                        vote.get_signing_key(),
                        MessageKind::<TYPES>::from_consensus_message(SequencingMessage::Committee(
                            CommitteeConsensusMessage::DAVote(vote.clone()),
                        )),
                        TransmitType::Direct(membership.get_leader(vote.get_view_number())),
                    )
                }
                // ED NOTE: This needs to be broadcasted to all nodes, not just ones on the DA committee
                HotShotEvent::DACSend(certificate, sender) => {
                    maybe_action = Some(HotShotAction::DACert);
                    (
                        sender,
                        MessageKind::<TYPES>::from_consensus_message(SequencingMessage::Committee(
                            CommitteeConsensusMessage::DACertificate(certificate),
                        )),
                        TransmitType::Broadcast,
                    )
                }
                HotShotEvent::ViewSyncPreCommitVoteSend(vote) => (
                    vote.get_signing_key(),
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::ViewSyncPreCommitVote(vote.clone()),
                    )),
                    TransmitType::Direct(
                        membership.get_leader(vote.get_view_number() + vote.get_data().relay),
                    ),
                ),
                HotShotEvent::ViewSyncCommitVoteSend(vote) => (
                    vote.get_signing_key(),
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::ViewSyncCommitVote(vote.clone()),
                    )),
                    TransmitType::Direct(
                        membership.get_leader(vote.get_view_number() + vote.get_data().relay),
                    ),
                ),
                HotShotEvent::ViewSyncFinalizeVoteSend(vote) => (
                    vote.get_signing_key(),
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::ViewSyncFinalizeVote(vote.clone()),
                    )),
                    TransmitType::Direct(
                        membership.get_leader(vote.get_view_number() + vote.get_data().relay),
                    ),
                ),
                HotShotEvent::ViewSyncPreCommitCertificate2Send(certificate, sender) => (
                    sender,
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::ViewSyncPreCommitCertificate(certificate),
                    )),
                    TransmitType::Broadcast,
                ),
                HotShotEvent::ViewSyncCommitCertificate2Send(certificate, sender) => (
                    sender,
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::ViewSyncCommitCertificate(certificate),
                    )),
                    TransmitType::Broadcast,
                ),
                HotShotEvent::ViewSyncFinalizeCertificate2Send(certificate, sender) => (
                    sender,
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::ViewSyncFinalizeCertificate(certificate),
                    )),
                    TransmitType::Broadcast,
                ),
                HotShotEvent::TimeoutVoteSend(vote) => (
                    vote.get_signing_key(),
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::TimeoutVote(vote.clone()),
                    )),
                    TransmitType::Direct(membership.get_leader(vote.get_view_number() + 1)),
                ),
                HotShotEvent::UpgradeProposalSend(proposal, sender) => (
                    sender,
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::UpgradeProposal(proposal),
                    )),
                    TransmitType::Broadcast,
                ),
                HotShotEvent::UpgradeVoteSend(vote) => {
                    error!("Sending upgrade vote!");
                    (
                        vote.get_signing_key(),
                        MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                            GeneralConsensusMessage::UpgradeVote(vote.clone()),
                        )),
                        TransmitType::Direct(membership.get_leader(vote.get_view_number())),
                    )
                }
                HotShotEvent::ViewChange(view) => {
                    self.view = view;
                    self.channel.update_view(self.view.get_u64());
                    return None;
                }
                HotShotEvent::VersionUpgrade(version) => {
                    debug!("Updating internal version in network task to {:?}", version);
                    self.version = version;
                    return None;
                }
                HotShotEvent::Shutdown => {
                    error!("Networking task shutting down");
                    return Some(HotShotTaskCompleted);
                }
                event => {
                    error!("Receieved unexpected message in network task {:?}", event);
                    return None;
                }
            };
        let message = Message {
            sender,
            kind: message_kind,
        };
        let view = message.kind.get_view_number();
        let committee = membership.get_whole_committee(view);
        let net = self.channel.clone();
        let storage = self.storage.clone();
        let version = self.version;
        async_spawn(async move {
            if NetworkEventTaskState::<TYPES, COMMCHANNEL, S>::maybe_record_action(
                maybe_action,
                storage,
                view,
            )
            .await
            .is_err()
            {
                return;
            }

            let transmit_result = if version == BASE_VERSION {
                match transmit {
                    TransmitType::Direct(recipient) => {
                        net.direct_message(message, recipient, STATIC_VER_0_1).await
                    }
                    TransmitType::Broadcast => {
                        net.broadcast_message(message, committee, STATIC_VER_0_1)
                            .await
                    }
                    TransmitType::DACommitteeBroadcast => {
                        net.da_broadcast_message(message, committee, STATIC_VER_0_1)
                            .await
                    }
                }
            } else {
                error!("The network has upgraded to {:?}, which is not implemented in this instance of HotShot.", version);
                return;
            };

            match transmit_result {
                Ok(()) => {}
                Err(e) => error!("Failed to send message from network task: {:?}", e),
            }
        });

        None
    }

    /// handle `VidDisperseSend`
    fn handle_vid_disperse_proposal(
        &self,
        vid_proposal: Proposal<TYPES, VidDisperse<TYPES>>,
        sender: &<TYPES as NodeType>::SignatureKey,
    ) -> Option<HotShotTaskCompleted> {
        let view = vid_proposal.data.view_number;
        let vid_share_proposals = VidDisperseShare::to_vid_share_proposals(vid_proposal);
        let messages: HashMap<_, _> = vid_share_proposals
            .into_iter()
            .map(|proposal| {
                (
                    proposal.data.recipient_key.clone(),
                    Message {
                        sender: sender.clone(),
                        kind: MessageKind::<TYPES>::from_consensus_message(
                            SequencingMessage::Committee(
                                CommitteeConsensusMessage::VidDisperseMsg(proposal),
                            ),
                        ), // TODO not a CommitteeConsensusMessage https://github.com/EspressoSystems/HotShot/issues/1696
                    },
                )
            })
            .collect();

        let net = self.channel.clone();
        let storage = self.storage.clone();
        async_spawn(async move {
            if NetworkEventTaskState::<TYPES, COMMCHANNEL, S>::maybe_record_action(
                Some(HotShotAction::VidDisperse),
                storage,
                view,
            )
            .await
            .is_err()
            {
                return;
            }
            match net.vid_broadcast_message(messages, STATIC_VER_0_1).await {
                Ok(()) => {}
                Err(e) => error!("Failed to send message from network task: {:?}", e),
            }
        });

        None
    }

    /// Record `HotShotAction` if available
    async fn maybe_record_action(
        maybe_action: Option<HotShotAction>,
        storage: Arc<RwLock<S>>,
        view: <TYPES as NodeType>::Time,
    ) -> Result<(), ()> {
        if let Some(action) = maybe_action {
            match storage
                .write()
                .await
                .record_action(view, action.clone())
                .await
            {
                Ok(()) => Ok(()),
                Err(e) => {
                    warn!("Not Sending {:?} because of storage error: {:?}", action, e);
                    Err(())
                }
            }
        } else {
            Ok(())
        }
    }
}
