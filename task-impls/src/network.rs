// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use async_broadcast::{Receiver, Sender};
use async_compatibility_layer::art::async_spawn;
use async_lock::RwLock;
use async_trait::async_trait;
use hotshot_task::task::TaskState;
use hotshot_types::{
    consensus::Consensus,
    data::{VidDisperse, VidDisperseShare},
    event::{Event, EventType, HotShotAction},
    message::{
        DaConsensusMessage, DataMessage, GeneralConsensusMessage, Message, MessageKind, Proposal,
        SequencingMessage, UpgradeLock,
    },
    traits::{
        election::Membership,
        network::{
            BroadcastDelay, ConnectedNetwork, RequestKind, ResponseMessage, TransmitType,
            ViewMessage,
        },
        node_implementation::{ConsensusTime, NodeType, Versions},
        storage::Storage,
    },
    vote::{HasViewNumber, Vote},
};
use tracing::{error, instrument, warn};

use crate::{
    events::{HotShotEvent, HotShotTaskCompleted},
    helpers::broadcast_event,
};

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
    /// Handles a (deserialized) message from the network
    pub async fn handle_message(&mut self, message: Message<TYPES>) {
        tracing::trace!("Received message from network:\n\n{message:?}");

        // Match the message kind and send the appropriate event to the internal event stream
        let sender = message.sender;
        match message.kind {
            // Handle consensus messages
            MessageKind::Consensus(consensus_message) => {
                let event = match consensus_message {
                    SequencingMessage::General(general_message) => match general_message {
                        GeneralConsensusMessage::Proposal(proposal) => {
                            HotShotEvent::QuorumProposalRecv(proposal, sender)
                        }
                        GeneralConsensusMessage::ProposalRequested(req, sig) => {
                            HotShotEvent::QuorumProposalRequestRecv(req, sig)
                        }
                        GeneralConsensusMessage::LeaderProposalAvailable(proposal) => {
                            HotShotEvent::QuorumProposalResponseRecv(proposal)
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
                        GeneralConsensusMessage::ViewSyncCommitCertificate(view_sync_message) => {
                            HotShotEvent::ViewSyncCommitCertificate2Recv(view_sync_message)
                        }

                        GeneralConsensusMessage::ViewSyncFinalizeVote(view_sync_message) => {
                            HotShotEvent::ViewSyncFinalizeVoteRecv(view_sync_message)
                        }
                        GeneralConsensusMessage::ViewSyncFinalizeCertificate(view_sync_message) => {
                            HotShotEvent::ViewSyncFinalizeCertificate2Recv(view_sync_message)
                        }

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
                        DaConsensusMessage::DaVote(vote) => HotShotEvent::DaVoteRecv(vote.clone()),
                        DaConsensusMessage::DaCertificate(cert) => {
                            HotShotEvent::DaCertificateRecv(cert)
                        }
                        DaConsensusMessage::VidDisperseMsg(proposal) => {
                            HotShotEvent::VidShareRecv(sender, proposal)
                        }
                    },
                };
                broadcast_event(Arc::new(event), &self.internal_event_stream).await;
            }

            // Handle data messages
            MessageKind::Data(message) => match message {
                DataMessage::SubmitTransaction(transaction, _) => {
                    broadcast_event(
                        Arc::new(HotShotEvent::TransactionsRecv(vec![transaction])),
                        &self.internal_event_stream,
                    )
                    .await;
                }
                DataMessage::DataResponse(response) => {
                    if let ResponseMessage::Found(message) = response {
                        match message {
                            SequencingMessage::Da(da_message) => {
                                if let DaConsensusMessage::VidDisperseMsg(proposal) = da_message {
                                    broadcast_event(
                                        Arc::new(HotShotEvent::VidResponseRecv(sender, proposal)),
                                        &self.internal_event_stream,
                                    )
                                    .await;
                                }
                            }
                            SequencingMessage::General(_) => {}
                        }
                    }
                }
                DataMessage::RequestData(data) => {
                    let req_data = data.clone();
                    if let RequestKind::Vid(_view_number, _key) = req_data.request {
                        broadcast_event(
                            Arc::new(HotShotEvent::VidRequestRecv(data, sender)),
                            &self.internal_event_stream,
                        )
                        .await;
                    }
                }
            },

            // Handle external messages
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
        }
    }
}

/// network event task state
pub struct NetworkEventTaskState<
    TYPES: NodeType,
    V: Versions,
    NET: ConnectedNetwork<TYPES::SignatureKey>,
    S: Storage<TYPES>,
> {
    /// comm network
    pub network: Arc<NET>,
    /// view number
    pub view: TYPES::Time,
    /// quorum for the network
    pub quorum_membership: TYPES::Membership,
    /// da for the network
    pub da_membership: TYPES::Membership,
    /// Storage to store actionable events
    pub storage: Arc<RwLock<S>>,
    /// Shared consensus state
    pub consensus: Arc<RwLock<Consensus<TYPES>>>,
    /// Lock for a decided upgrade
    pub upgrade_lock: UpgradeLock<TYPES, V>,
}

#[async_trait]
impl<
        TYPES: NodeType,
        V: Versions,
        NET: ConnectedNetwork<TYPES::SignatureKey>,
        S: Storage<TYPES> + 'static,
    > TaskState for NetworkEventTaskState<TYPES, V, NET, S>
{
    type Event = HotShotEvent<TYPES>;

    async fn handle_event(
        &mut self,
        event: Arc<Self::Event>,
        _sender: &Sender<Arc<Self::Event>>,
        _receiver: &Receiver<Arc<Self::Event>>,
    ) -> Result<()> {
        self.handle(event).await;

        Ok(())
    }

    async fn cancel_subtasks(&mut self) {}
}

impl<
        TYPES: NodeType,
        V: Versions,
        NET: ConnectedNetwork<TYPES::SignatureKey>,
        S: Storage<TYPES> + 'static,
    > NetworkEventTaskState<TYPES, V, NET, S>
{
    /// Handle the given event.
    ///
    /// Returns the completion status.
    #[instrument(skip_all, fields(view = *self.view), name = "Network Task", level = "error")]
    pub async fn handle(&mut self, event: Arc<HotShotEvent<TYPES>>) {
        let mut maybe_action = None;
        if let Some((sender, message_kind, transmit)) =
            self.parse_event(event, &mut maybe_action).await
        {
            self.spawn_transmit_task(message_kind, maybe_action, transmit, sender);
        };
    }

    /// handle `VidDisperseSend`
    async fn handle_vid_disperse_proposal(
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
                )),
            };
            let serialized_message = match self.upgrade_lock.serialize(&message).await {
                Ok(serialized) => serialized,
                Err(e) => {
                    error!("Failed to serialize message: {}", e);
                    continue;
                }
            };

            messages.insert(recipient, serialized_message);
        }

        let net = Arc::clone(&self.network);
        let storage = Arc::clone(&self.storage);
        let state = Arc::clone(&self.consensus);
        async_spawn(async move {
            if NetworkEventTaskState::<TYPES, V, NET, S>::maybe_record_action(
                Some(HotShotAction::VidDisperse),
                storage,
                state,
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
        state: Arc<RwLock<Consensus<TYPES>>>,
        view: <TYPES as NodeType>::Time,
    ) -> Result<(), ()> {
        if let Some(action) = maybe_action {
            if !state.write().await.update_action(action, view) {
                warn!("Already actioned {:?} in view {:?}", action, view);
                return Err(());
            }
            match storage.write().await.record_action(view, action).await {
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

    /// Parses a `HotShotEvent` and returns a tuple of: (sender's public key, `MessageKind`, `TransmitType`)
    /// which will be used to create a message and transmit on the wire.
    /// Returns `None` if the parsing result should not be sent on the wire.
    /// Handles the `VidDisperseSend` event separately using a helper method.
    #[allow(clippy::too_many_lines)]
    async fn parse_event(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        maybe_action: &mut Option<HotShotAction>,
    ) -> Option<(
        <TYPES as NodeType>::SignatureKey,
        MessageKind<TYPES>,
        TransmitType<TYPES>,
    )> {
        match event.as_ref().clone() {
            HotShotEvent::QuorumProposalSend(proposal, sender) => {
                *maybe_action = Some(HotShotAction::Propose);
                Some((
                    sender,
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::Proposal(proposal),
                    )),
                    TransmitType::Broadcast,
                ))
            }

            // ED Each network task is subscribed to all these message types.  Need filters per network task
            HotShotEvent::QuorumVoteSend(vote) => {
                *maybe_action = Some(HotShotAction::Vote);
                Some((
                    vote.signing_key(),
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::Vote(vote.clone()),
                    )),
                    TransmitType::Direct(self.quorum_membership.leader(vote.view_number() + 1)),
                ))
            }
            HotShotEvent::QuorumProposalRequestSend(req, signature) => Some((
                req.key.clone(),
                MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                    GeneralConsensusMessage::ProposalRequested(req.clone(), signature),
                )),
                TransmitType::Broadcast,
            )),
            HotShotEvent::QuorumProposalResponseSend(sender_key, proposal) => Some((
                sender_key.clone(),
                MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                    GeneralConsensusMessage::LeaderProposalAvailable(proposal),
                )),
                TransmitType::Direct(sender_key),
            )),
            HotShotEvent::VidDisperseSend(proposal, sender) => {
                self.handle_vid_disperse_proposal(proposal, &sender).await;
                None
            }
            HotShotEvent::DaProposalSend(proposal, sender) => {
                *maybe_action = Some(HotShotAction::DaPropose);
                Some((
                    sender,
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::Da(
                        DaConsensusMessage::DaProposal(proposal),
                    )),
                    TransmitType::DaCommitteeBroadcast,
                ))
            }
            HotShotEvent::DaVoteSend(vote) => {
                *maybe_action = Some(HotShotAction::DaVote);
                Some((
                    vote.signing_key(),
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::Da(
                        DaConsensusMessage::DaVote(vote.clone()),
                    )),
                    TransmitType::Direct(self.quorum_membership.leader(vote.view_number())),
                ))
            }
            HotShotEvent::DacSend(certificate, sender) => {
                *maybe_action = Some(HotShotAction::DaCert);
                Some((
                    sender,
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::Da(
                        DaConsensusMessage::DaCertificate(certificate),
                    )),
                    TransmitType::Broadcast,
                ))
            }
            HotShotEvent::ViewSyncPreCommitVoteSend(vote) => Some((
                vote.signing_key(),
                MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                    GeneralConsensusMessage::ViewSyncPreCommitVote(vote.clone()),
                )),
                TransmitType::Direct(
                    self.quorum_membership
                        .leader(vote.view_number() + vote.date().relay),
                ),
            )),
            HotShotEvent::ViewSyncCommitVoteSend(vote) => Some((
                vote.signing_key(),
                MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                    GeneralConsensusMessage::ViewSyncCommitVote(vote.clone()),
                )),
                TransmitType::Direct(
                    self.quorum_membership
                        .leader(vote.view_number() + vote.date().relay),
                ),
            )),
            HotShotEvent::ViewSyncFinalizeVoteSend(vote) => Some((
                vote.signing_key(),
                MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                    GeneralConsensusMessage::ViewSyncFinalizeVote(vote.clone()),
                )),
                TransmitType::Direct(
                    self.quorum_membership
                        .leader(vote.view_number() + vote.date().relay),
                ),
            )),
            HotShotEvent::ViewSyncPreCommitCertificate2Send(certificate, sender) => Some((
                sender,
                MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                    GeneralConsensusMessage::ViewSyncPreCommitCertificate(certificate),
                )),
                TransmitType::Broadcast,
            )),
            HotShotEvent::ViewSyncCommitCertificate2Send(certificate, sender) => Some((
                sender,
                MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                    GeneralConsensusMessage::ViewSyncCommitCertificate(certificate),
                )),
                TransmitType::Broadcast,
            )),
            HotShotEvent::ViewSyncFinalizeCertificate2Send(certificate, sender) => Some((
                sender,
                MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                    GeneralConsensusMessage::ViewSyncFinalizeCertificate(certificate),
                )),
                TransmitType::Broadcast,
            )),
            HotShotEvent::TimeoutVoteSend(vote) => {
                *maybe_action = Some(HotShotAction::Vote);
                Some((
                    vote.signing_key(),
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::TimeoutVote(vote.clone()),
                    )),
                    TransmitType::Direct(self.quorum_membership.leader(vote.view_number() + 1)),
                ))
            }
            HotShotEvent::UpgradeProposalSend(proposal, sender) => Some((
                sender,
                MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                    GeneralConsensusMessage::UpgradeProposal(proposal),
                )),
                TransmitType::Broadcast,
            )),
            HotShotEvent::UpgradeVoteSend(vote) => {
                error!("Sending upgrade vote!");
                Some((
                    vote.signing_key(),
                    MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                        GeneralConsensusMessage::UpgradeVote(vote.clone()),
                    )),
                    TransmitType::Direct(self.quorum_membership.leader(vote.view_number())),
                ))
            }
            HotShotEvent::ViewChange(view) => {
                self.view = view;
                self.network
                    .update_view::<TYPES>(self.view.u64(), &self.quorum_membership)
                    .await;
                None
            }
            HotShotEvent::VidRequestSend(req, sender, to) => Some((
                sender,
                MessageKind::Data(DataMessage::RequestData(req)),
                TransmitType::Direct(to),
            )),
            HotShotEvent::VidResponseSend(sender, to, proposal) => {
                let da_message = DaConsensusMessage::VidDisperseMsg(proposal);
                let sequencing_msg = SequencingMessage::Da(da_message);
                let response_message = ResponseMessage::Found(sequencing_msg);
                Some((
                    sender,
                    MessageKind::Data(DataMessage::DataResponse(response_message)),
                    TransmitType::Direct(to),
                ))
            }
            _ => None,
        }
    }

    /// Creates a network message and spawns a task that transmits it on the wire.
    fn spawn_transmit_task(
        &self,
        message_kind: MessageKind<TYPES>,
        maybe_action: Option<HotShotAction>,
        transmit: TransmitType<TYPES>,
        sender: TYPES::SignatureKey,
    ) {
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
        let committee_topic = self.quorum_membership.committee_topic();
        let da_committee = self.da_membership.committee_members(view);
        let net = Arc::clone(&self.network);
        let storage = Arc::clone(&self.storage);
        let state = Arc::clone(&self.consensus);
        let upgrade_lock = self.upgrade_lock.clone();
        async_spawn(async move {
            if NetworkEventTaskState::<TYPES, V, NET, S>::maybe_record_action(
                maybe_action,
                Arc::clone(&storage),
                state,
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

            let serialized_message = match upgrade_lock.serialize(&message).await {
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
                    net.da_broadcast_message(serialized_message, da_committee, broadcast_delay)
                        .await
                }
                TransmitType::DaCommitteeAndLeaderBroadcast(recipient) => {
                    if let Err(e) = net
                        .direct_message(serialized_message.clone(), recipient)
                        .await
                    {
                        error!("Failed to send message from network task: {e:?}");
                    }

                    // Otherwise, send the next message.
                    net.da_broadcast_message(serialized_message, da_committee, broadcast_delay)
                        .await
                }
            };

            match transmit_result {
                Ok(()) => {}
                Err(e) => error!("Failed to send message from network task: {:?}", e),
            }
        });
    }
}

/// A module with test helpers
pub mod test {
    use std::ops::{Deref, DerefMut};

    use async_trait::async_trait;

    use super::{
        Arc, ConnectedNetwork, HotShotEvent, MessageKind, NetworkEventTaskState, NodeType,
        Receiver, Result, Sender, Storage, TaskState, TransmitType, Versions,
    };

    /// A dynamic type alias for a function that takes the result of `NetworkEventTaskState::parse_event`
    /// and changes it before transmitting on the network.
    pub type ModifierClosure<TYPES> = dyn Fn(
            &mut <TYPES as NodeType>::SignatureKey,
            &mut MessageKind<TYPES>,
            &mut TransmitType<TYPES>,
            &<TYPES as NodeType>::Membership,
        ) + Send
        + Sync;

    /// A helper wrapper around `NetworkEventTaskState` that can modify its behaviour for tests
    pub struct NetworkEventTaskStateModifier<
        TYPES: NodeType,
        V: Versions,
        NET: ConnectedNetwork<TYPES::SignatureKey>,
        S: Storage<TYPES>,
    > {
        /// The real `NetworkEventTaskState`
        pub network_event_task_state: NetworkEventTaskState<TYPES, V, NET, S>,
        /// A function that takes the result of `NetworkEventTaskState::parse_event` and
        /// changes it before transmitting on the network.
        pub modifier: Arc<ModifierClosure<TYPES>>,
    }

    impl<
            TYPES: NodeType,
            V: Versions,
            NET: ConnectedNetwork<TYPES::SignatureKey>,
            S: Storage<TYPES> + 'static,
        > NetworkEventTaskStateModifier<TYPES, V, NET, S>
    {
        /// Handles the received event modifying it before sending on the network.
        pub async fn handle(&mut self, event: Arc<HotShotEvent<TYPES>>) {
            let mut maybe_action = None;
            if let Some((mut sender, mut message_kind, mut transmit)) =
                self.parse_event(event, &mut maybe_action).await
            {
                // Modify the values acquired by parsing the event.
                (self.modifier)(
                    &mut sender,
                    &mut message_kind,
                    &mut transmit,
                    &self.quorum_membership,
                );
                self.spawn_transmit_task(message_kind, maybe_action, transmit, sender);
            }
        }
    }

    #[async_trait]
    impl<
            TYPES: NodeType,
            V: Versions,
            NET: ConnectedNetwork<TYPES::SignatureKey>,
            S: Storage<TYPES> + 'static,
        > TaskState for NetworkEventTaskStateModifier<TYPES, V, NET, S>
    {
        type Event = HotShotEvent<TYPES>;

        async fn handle_event(
            &mut self,
            event: Arc<Self::Event>,
            _sender: &Sender<Arc<Self::Event>>,
            _receiver: &Receiver<Arc<Self::Event>>,
        ) -> Result<()> {
            self.handle(event).await;

            Ok(())
        }

        async fn cancel_subtasks(&mut self) {}
    }

    impl<
            TYPES: NodeType,
            V: Versions,
            NET: ConnectedNetwork<TYPES::SignatureKey>,
            S: Storage<TYPES>,
        > Deref for NetworkEventTaskStateModifier<TYPES, V, NET, S>
    {
        type Target = NetworkEventTaskState<TYPES, V, NET, S>;

        fn deref(&self) -> &Self::Target {
            &self.network_event_task_state
        }
    }

    impl<
            TYPES: NodeType,
            V: Versions,
            NET: ConnectedNetwork<TYPES::SignatureKey>,
            S: Storage<TYPES>,
        > DerefMut for NetworkEventTaskStateModifier<TYPES, V, NET, S>
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.network_event_task_state
        }
    }
}
