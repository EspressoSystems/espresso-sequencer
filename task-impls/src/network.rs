use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use async_broadcast::{Receiver, Sender};
use async_compatibility_layer::art::async_spawn;
use async_lock::RwLock;
use async_trait::async_trait;
use hotshot_task::task::TaskState;
use hotshot_types::{
    data::{VidDisperse, VidDisperseShare},
    event::{Event, EventType, HotShotAction},
    message::{
        DaConsensusMessage, DataMessage, GeneralConsensusMessage, Message, MessageKind, Proposal,
        SequencingMessage, VersionedMessage,
    },
    simple_certificate::UpgradeCertificate,
    traits::{
        election::Membership,
        network::{BroadcastDelay, ConnectedNetwork, TransmitType, ViewMessage},
        node_implementation::{ConsensusTime, NodeType},
        storage::Storage,
    },
    vote::{HasViewNumber, Vote},
};
use tracing::{error, instrument, warn};

use crate::{
    events::{HotShotEvent, HotShotTaskCompleted},
    helpers::broadcast_event,
};

/// quorum filter
pub fn quorum_filter<TYPES: NodeType>(event: &Arc<HotShotEvent<TYPES>>) -> bool {
    !matches!(
        event.as_ref(),
        HotShotEvent::QuorumProposalSend(_, _)
            | HotShotEvent::QuorumVoteSend(_)
            | HotShotEvent::DacSend(_, _)
            | HotShotEvent::TimeoutVoteSend(_)
            | HotShotEvent::UpgradeDecided(_)
            | HotShotEvent::ViewChange(_)
    )
}

/// upgrade filter
pub fn upgrade_filter<TYPES: NodeType>(event: &Arc<HotShotEvent<TYPES>>) -> bool {
    !matches!(
        event.as_ref(),
        HotShotEvent::UpgradeProposalSend(_, _)
            | HotShotEvent::UpgradeVoteSend(_)
            | HotShotEvent::UpgradeDecided(_)
            | HotShotEvent::ViewChange(_)
    )
}

/// DA filter
pub fn da_filter<TYPES: NodeType>(event: &Arc<HotShotEvent<TYPES>>) -> bool {
    !matches!(
        event.as_ref(),
        HotShotEvent::DaProposalSend(_, _)
            | HotShotEvent::DaVoteSend(_)
            | HotShotEvent::UpgradeDecided(_)
            | HotShotEvent::ViewChange(_)
    )
}

/// vid filter
pub fn vid_filter<TYPES: NodeType>(event: &Arc<HotShotEvent<TYPES>>) -> bool {
    !matches!(
        event.as_ref(),
        HotShotEvent::VidDisperseSend(_, _)
            | HotShotEvent::UpgradeDecided(_)
            | HotShotEvent::ViewChange(_)
    )
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
            | HotShotEvent::UpgradeDecided(_)
            | HotShotEvent::ViewChange(_)
    )
}
/// the network message task state
#[derive(Clone)]
pub struct NetworkMessageTaskState<TYPES: NodeType> {
    /// Sender to send internal events this task generates to other tasks
    pub internal_event_stream: Sender<Arc<HotShotEvent<TYPES>>>,

    /// Sender to send external events this task generates to the event stream
    pub external_event_stream: Sender<Event<TYPES>>,
}

impl<TYPES: NodeType> NetworkMessageTaskState<TYPES> {
    #[instrument(skip_all, name = "Network message task", level = "trace")]
    /// Handle the message.
    pub async fn handle_messages(&mut self, messages: Vec<Message<TYPES>>) {
        // We will send only one event for a vector of transactions.
        let mut transactions = Vec::new();
        for message in messages {
            tracing::trace!("Received message from network:\n\n{message:?}");
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
                        SequencingMessage::Da(da_message) => match da_message {
                            DaConsensusMessage::DaProposal(proposal) => {
                                HotShotEvent::DaProposalRecv(proposal, sender)
                            }
                            DaConsensusMessage::DaVote(vote) => {
                                HotShotEvent::DaVoteRecv(vote.clone())
                            }
                            DaConsensusMessage::DaCertificate(cert) => {
                                HotShotEvent::DaCertificateRecv(cert)
                            }
                            DaConsensusMessage::VidDisperseMsg(proposal) => {
                                HotShotEvent::VidShareRecv(proposal)
                            }
                        },
                    };
                    // TODO (Keyao benchmarking) Update these event variants (similar to the
                    // `TransactionsRecv` event) so we can send one event for a vector of messages.
                    // <https://github.com/EspressoSystems/HotShot/issues/1428>
                    broadcast_event(Arc::new(event), &self.internal_event_stream).await;
                }
                MessageKind::Data(message) => match message {
                    DataMessage::SubmitTransaction(transaction, _) => {
                        transactions.push(transaction);
                    }
                    DataMessage::DataResponse(_) | DataMessage::RequestData(_) => {
                        warn!("Request and Response messages should not be received in the NetworkMessage task");
                    }
                },

                MessageKind::External(data) => {
                    // Send the external message to the external event stream so it can be processed
                    broadcast_event(
                        Event {
                            view_number: TYPES::Time::new(1),
                            event: EventType::ExternalMessageReceived(data),
                        },
                        &self.external_event_stream,
                    )
                    .await;
                }
            };
        }
        if !transactions.is_empty() {
            broadcast_event(
                Arc::new(HotShotEvent::TransactionsRecv(transactions)),
                &self.internal_event_stream,
            )
            .await;
        }
    }
}

/// network event task state
pub struct NetworkEventTaskState<
    TYPES: NodeType,
    COMMCHANNEL: ConnectedNetwork<TYPES::SignatureKey>,
    S: Storage<TYPES>,
> {
    /// comm channel
    pub channel: Arc<COMMCHANNEL>,
    /// view number
    pub view: TYPES::Time,
    /// membership for the channel
    pub membership: TYPES::Membership,
    // TODO ED Need to add exchange so we can get the recipient key and our own key?
    /// Filter which returns false for the events that this specific network task cares about
    pub filter: fn(&Arc<HotShotEvent<TYPES>>) -> bool,
    /// Storage to store actionable events
    pub storage: Arc<RwLock<S>>,
    /// Decided upgrade certificate
    pub decided_upgrade_certificate: Option<UpgradeCertificate<TYPES>>,
}

#[async_trait]
impl<
        TYPES: NodeType,
        COMMCHANNEL: ConnectedNetwork<TYPES::SignatureKey>,
        S: Storage<TYPES> + 'static,
    > TaskState for NetworkEventTaskState<TYPES, COMMCHANNEL, S>
{
    type Event = HotShotEvent<TYPES>;

    async fn handle_event(
        &mut self,
        event: Arc<Self::Event>,
        _sender: &Sender<Arc<Self::Event>>,
        _receiver: &Receiver<Arc<Self::Event>>,
    ) -> Result<()> {
        let membership = self.membership.clone();

        if !(self.filter)(&event) {
            self.handle(event, &membership).await;
        }

        Ok(())
    }

    async fn cancel_subtasks(&mut self) {}
}

impl<
        TYPES: NodeType,
        COMMCHANNEL: ConnectedNetwork<TYPES::SignatureKey>,
        S: Storage<TYPES> + 'static,
    > NetworkEventTaskState<TYPES, COMMCHANNEL, S>
{
    /// Handle the given event.
    ///
    /// Returns the completion status.
    #[allow(clippy::too_many_lines)] // TODO https://github.com/EspressoSystems/HotShot/issues/1704
    #[instrument(skip_all, fields(view = *self.view), name = "Network Task", level = "error")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        membership: &TYPES::Membership,
    ) {
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
                        vote.signing_key(),
                        MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                            GeneralConsensusMessage::Vote(vote.clone()),
                        )),
                        TransmitType::Direct(membership.leader(vote.view_number() + 1)),
                    )
                }
                HotShotEvent::VidDisperseSend(proposal, sender) => {
                    self.handle_vid_disperse_proposal(proposal, &sender);
                    return;
                }
                HotShotEvent::DaProposalSend(proposal, sender) => {
                    maybe_action = Some(HotShotAction::DaPropose);
                    (
                        sender,
                        MessageKind::<TYPES>::from_consensus_message(SequencingMessage::Da(
                            DaConsensusMessage::DaProposal(proposal),
                        )),
                        TransmitType::DaCommitteeBroadcast,
                    )
                }
                HotShotEvent::DaVoteSend(vote) => {
                    maybe_action = Some(HotShotAction::DaVote);
                    (
                        vote.signing_key(),
                        MessageKind::<TYPES>::from_consensus_message(SequencingMessage::Da(
                            DaConsensusMessage::DaVote(vote.clone()),
                        )),
                        TransmitType::Direct(membership.leader(vote.view_number())),
                    )
                }
                // ED NOTE: This needs to be broadcasted to all nodes, not just ones on the DA committee
                HotShotEvent::DacSend(certificate, sender) => {
                    maybe_action = Some(HotShotAction::DaCert);
                    (
                        sender,
                        MessageKind::<TYPES>::from_consensus_message(SequencingMessage::Da(
                            DaConsensusMessage::DaCertificate(certificate),
                        )),
                        TransmitType::Broadcast,
                    )
                }
                HotShotEvent::ViewSyncPreCommitVoteSend(vote) => (
                    vote.signing_key(),
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::ViewSyncPreCommitVote(vote.clone()),
                    )),
                    TransmitType::Direct(membership.leader(vote.view_number() + vote.date().relay)),
                ),
                HotShotEvent::ViewSyncCommitVoteSend(vote) => (
                    vote.signing_key(),
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::ViewSyncCommitVote(vote.clone()),
                    )),
                    TransmitType::Direct(membership.leader(vote.view_number() + vote.date().relay)),
                ),
                HotShotEvent::ViewSyncFinalizeVoteSend(vote) => (
                    vote.signing_key(),
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::ViewSyncFinalizeVote(vote.clone()),
                    )),
                    TransmitType::Direct(membership.leader(vote.view_number() + vote.date().relay)),
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
                    vote.signing_key(),
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::TimeoutVote(vote.clone()),
                    )),
                    TransmitType::Direct(membership.leader(vote.view_number() + 1)),
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
                        vote.signing_key(),
                        MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                            GeneralConsensusMessage::UpgradeVote(vote.clone()),
                        )),
                        TransmitType::Direct(membership.leader(vote.view_number())),
                    )
                }
                HotShotEvent::ViewChange(view) => {
                    self.view = view;
                    self.channel
                        .update_view::<TYPES>(self.view.u64(), membership)
                        .await;
                    return;
                }
                HotShotEvent::UpgradeDecided(cert) => {
                    self.decided_upgrade_certificate = Some(cert.clone());
                    return;
                }
                _ => {
                    return;
                }
            };
        let broadcast_delay = match &message_kind {
            MessageKind::Consensus(
                SequencingMessage::General(GeneralConsensusMessage::Vote(_))
                | SequencingMessage::Da(_),
            ) => BroadcastDelay::View(*message_kind.view_number()),
            _ => BroadcastDelay::None,
        };
        let message = Message {
            sender,
            kind: message_kind,
        };
        let view = message.kind.view_number();
        let committee = membership.whole_committee(view);
        let committee_topic = membership.committee_topic();
        let net = Arc::clone(&self.channel);
        let storage = Arc::clone(&self.storage);
        let decided_upgrade_certificate = self.decided_upgrade_certificate.clone();
        async_spawn(async move {
            if NetworkEventTaskState::<TYPES, COMMCHANNEL, S>::maybe_record_action(
                maybe_action,
                Arc::clone(&storage),
                view,
            )
            .await
            .is_err()
            {
                return;
            }
            if let MessageKind::Consensus(SequencingMessage::General(
                GeneralConsensusMessage::Proposal(prop),
            )) = &message.kind
            {
                if storage.write().await.append_proposal(prop).await.is_err() {
                    return;
                }
            }

            let serialized_message = match message.serialize(&decided_upgrade_certificate) {
                Ok(serialized) => serialized,
                Err(e) => {
                    error!("Failed to serialize message: {}", e);
                    return;
                }
            };

            let transmit_result = match transmit {
                TransmitType::Direct(recipient) => {
                    net.direct_message(serialized_message, recipient).await
                }
                TransmitType::Broadcast => {
                    net.broadcast_message(serialized_message, committee_topic, broadcast_delay)
                        .await
                }
                TransmitType::DaCommitteeBroadcast => {
                    net.da_broadcast_message(serialized_message, committee, broadcast_delay)
                        .await
                }
            };

            match transmit_result {
                Ok(()) => {}
                Err(e) => error!("Failed to send message from network task: {:?}", e),
            }
        });
    }

    /// handle `VidDisperseSend`
    fn handle_vid_disperse_proposal(
        &self,
        vid_proposal: Proposal<TYPES, VidDisperse<TYPES>>,
        sender: &<TYPES as NodeType>::SignatureKey,
    ) -> Option<HotShotTaskCompleted> {
        let view = vid_proposal.data.view_number;
        let vid_share_proposals = VidDisperseShare::to_vid_share_proposals(vid_proposal);
        let mut messages = HashMap::new();

        for proposal in vid_share_proposals {
            let recipient = proposal.data.recipient_key.clone();
            let message = Message {
                sender: sender.clone(),
                kind: MessageKind::<TYPES>::from_consensus_message(SequencingMessage::Da(
                    DaConsensusMessage::VidDisperseMsg(proposal),
                )), // TODO not a DaConsensusMessage https://github.com/EspressoSystems/HotShot/issues/1696
            };
            let serialized_message = match message.serialize(&self.decided_upgrade_certificate) {
                Ok(serialized) => serialized,
                Err(e) => {
                    error!("Failed to serialize message: {}", e);
                    continue;
                }
            };

            messages.insert(recipient, serialized_message);
        }

        let net = Arc::clone(&self.channel);
        let storage = Arc::clone(&self.storage);
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
            match net.vid_broadcast_message(messages).await {
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
