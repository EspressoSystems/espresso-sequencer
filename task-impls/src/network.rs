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
        network::{BroadcastDelay, ConnectedNetwork, TransmitType, ViewMessage},
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

/// quorum filter
pub fn quorum_filter<TYPES: NodeType>(event: &Arc<HotShotEvent<TYPES>>) -> bool {
    !matches!(
        event.as_ref(),
        HotShotEvent::QuorumProposalSend(_, _)
            | HotShotEvent::QuorumVoteSend(_)
            | HotShotEvent::DacSend(_, _)
            | HotShotEvent::TimeoutVoteSend(_)
            | HotShotEvent::ViewChange(_)
    )
}

/// upgrade filter
pub fn upgrade_filter<TYPES: NodeType>(event: &Arc<HotShotEvent<TYPES>>) -> bool {
    !matches!(
        event.as_ref(),
        HotShotEvent::UpgradeProposalSend(_, _)
            | HotShotEvent::UpgradeVoteSend(_)
            | HotShotEvent::ViewChange(_)
    )
}

/// DA filter
pub fn da_filter<TYPES: NodeType>(event: &Arc<HotShotEvent<TYPES>>) -> bool {
    !matches!(
        event.as_ref(),
        HotShotEvent::DaProposalSend(_, _)
            | HotShotEvent::QuorumProposalRequestSend(..)
            | HotShotEvent::QuorumProposalResponseSend(..)
            | HotShotEvent::DaVoteSend(_)
            | HotShotEvent::ViewChange(_)
    )
}

/// vid filter
pub fn vid_filter<TYPES: NodeType>(event: &Arc<HotShotEvent<TYPES>>) -> bool {
    !matches!(
        event.as_ref(),
        HotShotEvent::VidDisperseSend(_, _) | HotShotEvent::ViewChange(_)
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
                            HotShotEvent::VidShareRecv(proposal)
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
                DataMessage::DataResponse(_) | DataMessage::RequestData(_) => {
                    warn!("Request and Response messages should not be received in the NetworkMessage task");
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
    /// Shared consensus state
    pub consensus: Arc<RwLock<Consensus<TYPES>>>,
    /// Lock for a decided upgrade
    pub upgrade_lock: UpgradeLock<TYPES, V>,
}

#[async_trait]
impl<
        TYPES: NodeType,
        V: Versions,
        COMMCHANNEL: ConnectedNetwork<TYPES::SignatureKey>,
        S: Storage<TYPES> + 'static,
    > TaskState for NetworkEventTaskState<TYPES, V, COMMCHANNEL, S>
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
        V: Versions,
        COMMCHANNEL: ConnectedNetwork<TYPES::SignatureKey>,
        S: Storage<TYPES> + 'static,
    > NetworkEventTaskState<TYPES, V, COMMCHANNEL, S>
{
    /// Handle the given event.
    ///
    /// Returns the completion status.
    #[instrument(skip_all, fields(view = *self.view), name = "Network Task", level = "error")]
    pub async fn handle(
        &mut self,
        event: Arc<HotShotEvent<TYPES>>,
        membership: &TYPES::Membership,
    ) {
        let mut maybe_action = None;
        if let Some((sender, message_kind, transmit)) =
            self.parse_event(event, &mut maybe_action, membership).await
        {
            self.spawn_transmit_task(message_kind, membership, maybe_action, transmit, sender);
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

        let net = Arc::clone(&self.channel);
        let storage = Arc::clone(&self.storage);
        let state = Arc::clone(&self.consensus);
        async_spawn(async move {
            if NetworkEventTaskState::<TYPES, V, COMMCHANNEL, S>::maybe_record_action(
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
        membership: &TYPES::Membership,
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
                    TransmitType::Direct(membership.leader(vote.view_number() + 1)),
                ))
            }
            HotShotEvent::QuorumProposalRequestSend(req, signature) => Some((
                req.key.clone(),
                MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                    GeneralConsensusMessage::ProposalRequested(req.clone(), signature),
                )),
                TransmitType::DaCommitteeAndLeaderBroadcast(membership.leader(req.view_number)),
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
                    TransmitType::Direct(membership.leader(vote.view_number())),
                ))
            }
            // ED NOTE: This needs to be broadcasted to all nodes, not just ones on the DA committee
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
                TransmitType::Direct(membership.leader(vote.view_number() + vote.date().relay)),
            )),
            HotShotEvent::ViewSyncCommitVoteSend(vote) => Some((
                vote.signing_key(),
                MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                    GeneralConsensusMessage::ViewSyncCommitVote(vote.clone()),
                )),
                TransmitType::Direct(membership.leader(vote.view_number() + vote.date().relay)),
            )),
            HotShotEvent::ViewSyncFinalizeVoteSend(vote) => Some((
                vote.signing_key(),
                MessageKind::<TYPES>::from_consensus_message(SequencingMessage::General(
                    GeneralConsensusMessage::ViewSyncFinalizeVote(vote.clone()),
                )),
                TransmitType::Direct(membership.leader(vote.view_number() + vote.date().relay)),
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
                    TransmitType::Direct(membership.leader(vote.view_number() + 1)),
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
                    TransmitType::Direct(membership.leader(vote.view_number())),
                ))
            }
            HotShotEvent::ViewChange(view) => {
                self.view = view;
                self.channel
                    .update_view::<TYPES>(self.view.u64(), membership)
                    .await;
                None
            }
            _ => None,
        }
    }

    /// Creates a network message and spawns a task that transmits it on the wire.
    fn spawn_transmit_task(
        &self,
        message_kind: MessageKind<TYPES>,
        membership: &TYPES::Membership,
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
        let committee = membership.committee_members(view);
        let committee_topic = membership.committee_topic();
        let net = Arc::clone(&self.channel);
        let storage = Arc::clone(&self.storage);
        let state = Arc::clone(&self.consensus);
        let upgrade_lock = self.upgrade_lock.clone();
        async_spawn(async move {
            if NetworkEventTaskState::<TYPES, V, COMMCHANNEL, S>::maybe_record_action(
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
                    net.da_broadcast_message(serialized_message, committee, broadcast_delay)
                        .await
                }
                TransmitType::DaCommitteeAndLeaderBroadcast(recipient) => {
                    // Short-circuit exit from this call if we get an error during the direct leader broadcast.
                    // NOTE: An improvement to this is to check if the leader is in the DA committee but it's
                    // just a single extra message to the leader, so it's not an optimization that we need now.
                    if let Err(e) = net
                        .direct_message(serialized_message.clone(), recipient)
                        .await
                    {
                        error!("Failed to send message from network task: {e:?}");
                        return;
                    }

                    // Otherwise, send the next message.
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
        COMMCHANNEL: ConnectedNetwork<TYPES::SignatureKey>,
        S: Storage<TYPES>,
    > {
        /// The real `NetworkEventTaskState`
        pub network_event_task_state: NetworkEventTaskState<TYPES, V, COMMCHANNEL, S>,
        /// A function that takes the result of `NetworkEventTaskState::parse_event` and
        /// changes it before transmitting on the network.
        pub modifier: Arc<ModifierClosure<TYPES>>,
    }

    impl<
            TYPES: NodeType,
            V: Versions,
            COMMCHANNEL: ConnectedNetwork<TYPES::SignatureKey>,
            S: Storage<TYPES> + 'static,
        > NetworkEventTaskStateModifier<TYPES, V, COMMCHANNEL, S>
    {
        /// Handles the received event modifying it before sending on the network.
        pub async fn handle(
            &mut self,
            event: Arc<HotShotEvent<TYPES>>,
            membership: &TYPES::Membership,
        ) {
            let mut maybe_action = None;
            if let Some((mut sender, mut message_kind, mut transmit)) =
                self.parse_event(event, &mut maybe_action, membership).await
            {
                // Modify the values acquired by parsing the event.
                (self.modifier)(&mut sender, &mut message_kind, &mut transmit, membership);
                self.spawn_transmit_task(message_kind, membership, maybe_action, transmit, sender);
            }
        }
    }

    #[async_trait]
    impl<
            TYPES: NodeType,
            V: Versions,
            COMMCHANNEL: ConnectedNetwork<TYPES::SignatureKey>,
            S: Storage<TYPES> + 'static,
        > TaskState for NetworkEventTaskStateModifier<TYPES, V, COMMCHANNEL, S>
    {
        type Event = HotShotEvent<TYPES>;

        async fn handle_event(
            &mut self,
            event: Arc<Self::Event>,
            _sender: &Sender<Arc<Self::Event>>,
            _receiver: &Receiver<Arc<Self::Event>>,
        ) -> Result<()> {
            let membership = self.network_event_task_state.membership.clone();

            if !(self.network_event_task_state.filter)(&event) {
                self.handle(event, &membership).await;
            }

            Ok(())
        }

        async fn cancel_subtasks(&mut self) {}
    }

    impl<
            TYPES: NodeType,
            V: Versions,
            COMMCHANNEL: ConnectedNetwork<TYPES::SignatureKey>,
            S: Storage<TYPES>,
        > Deref for NetworkEventTaskStateModifier<TYPES, V, COMMCHANNEL, S>
    {
        type Target = NetworkEventTaskState<TYPES, V, COMMCHANNEL, S>;

        fn deref(&self) -> &Self::Target {
            &self.network_event_task_state
        }
    }

    impl<
            TYPES: NodeType,
            V: Versions,
            COMMCHANNEL: ConnectedNetwork<TYPES::SignatureKey>,
            S: Storage<TYPES>,
        > DerefMut for NetworkEventTaskStateModifier<TYPES, V, COMMCHANNEL, S>
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.network_event_task_state
        }
    }
}
