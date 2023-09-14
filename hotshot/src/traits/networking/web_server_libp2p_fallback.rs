//! Networking Implementation that has a primary and a fallback newtork.  If the primary
//! Errors we will use the backup to send or receive
use super::NetworkError;
use crate::{
    traits::implementations::{Libp2pNetwork, WebServerNetwork},
    NodeImplementation,
};

use async_trait::async_trait;

use futures::join;

use async_compatibility_layer::channel::UnboundedSendError;
use hotshot_task::{boxed_sync, BoxSyncFuture};
use hotshot_types::{
    data::{ProposalType, ViewNumber},
    message::Message,
    traits::{
        election::Membership,
        network::{
            CommunicationChannel, ConnectedNetwork, ConsensusIntentEvent,
            TestableChannelImplementation, TestableNetworkingImplementation, TransmitType,
            ViewMessage,
        },
        node_implementation::NodeType,
    },
};
use std::{marker::PhantomData, sync::Arc};
use tracing::error;
/// A communication channel with 2 networks, where we can fall back to the slower network if the
/// primary fails
#[derive(Clone, Debug)]
pub struct WebServerWithFallbackCommChannel<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    MEMBERSHIP: Membership<TYPES>,
> {
    /// The two networks we'll use for send/recv
    networks: Arc<CombinedNetworks<TYPES, I, MEMBERSHIP>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    WebServerWithFallbackCommChannel<TYPES, I, MEMBERSHIP>
{
    /// Constructor
    #[must_use]
    pub fn new(networks: Arc<CombinedNetworks<TYPES, I, MEMBERSHIP>>) -> Self {
        Self { networks }
    }

    /// Get a ref to the primary network
    #[must_use]
    pub fn network(&self) -> &WebServerNetwork<Message<TYPES, I>, TYPES::SignatureKey, TYPES> {
        &self.networks.0
    }

    /// Get a ref to the backup network
    #[must_use]
    pub fn fallback(&self) -> &Libp2pNetwork<Message<TYPES, I>, TYPES::SignatureKey> {
        &self.networks.1
    }
}

/// Wrapper for the tuple of `WebServerNetwork` and `Libp2pNetwork`
/// We need this so we can impl `TestableNetworkingImplementation`
/// on the tuple
#[derive(Debug, Clone)]
pub struct CombinedNetworks<
    TYPES: NodeType,
    I: NodeImplementation<TYPES>,
    MEMBERSHIP: Membership<TYPES>,
>(
    pub WebServerNetwork<Message<TYPES, I>, TYPES::SignatureKey, TYPES>,
    pub Libp2pNetwork<Message<TYPES, I>, TYPES::SignatureKey>,
    pub PhantomData<MEMBERSHIP>,
);

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    TestableNetworkingImplementation<TYPES, Message<TYPES, I>>
    for CombinedNetworks<TYPES, I, MEMBERSHIP>
{
    fn generator(
        expected_node_count: usize,
        num_bootstrap: usize,
        network_id: usize,
        da_committee_size: usize,
        is_da: bool,
    ) -> Box<dyn Fn(u64) -> Self + 'static> {
        let generators = (
            <WebServerNetwork<
                Message<TYPES, I>,
                TYPES::SignatureKey,
                TYPES,
            > as TestableNetworkingImplementation<_, _>>::generator(
                expected_node_count,
                num_bootstrap,
                network_id,
                da_committee_size,
                is_da
            ),
            <Libp2pNetwork<Message<TYPES, I>, TYPES::SignatureKey> as TestableNetworkingImplementation<_, _>>::generator(
                expected_node_count,
                num_bootstrap,
                network_id,
                da_committee_size,
                is_da
            )
        );
        Box::new(move |node_id| {
            CombinedNetworks(generators.0(node_id), generators.1(node_id), PhantomData)
        })
    }

    /// Get the number of messages in-flight.
    ///
    /// Some implementations will not be able to tell how many messages there are in-flight. These implementations should return `None`.
    fn in_flight_message_count(&self) -> Option<usize> {
        None
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    TestableNetworkingImplementation<TYPES, Message<TYPES, I>>
    for WebServerWithFallbackCommChannel<TYPES, I, MEMBERSHIP>
{
    fn generator(
        expected_node_count: usize,
        num_bootstrap: usize,
        network_id: usize,
        da_committee_size: usize,
        is_da: bool,
    ) -> Box<dyn Fn(u64) -> Self + 'static> {
        let generator = <CombinedNetworks<
            TYPES,
            I,
            MEMBERSHIP,
        > as TestableNetworkingImplementation<_, _>>::generator(
            expected_node_count,
            num_bootstrap,
            network_id,
            da_committee_size,
            is_da
        );
        Box::new(move |node_id| Self {
            networks: generator(node_id).into(),
        })
    }

    /// Get the number of messages in-flight.
    ///
    /// Some implementations will not be able to tell how many messages there are in-flight. These implementations should return `None`.
    fn in_flight_message_count(&self) -> Option<usize> {
        None
    }
}

#[async_trait]
impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    CommunicationChannel<TYPES, Message<TYPES, I>, MEMBERSHIP>
    for WebServerWithFallbackCommChannel<TYPES, I, MEMBERSHIP>
{
    type NETWORK = CombinedNetworks<TYPES, I, MEMBERSHIP>;

    async fn wait_for_ready(&self) {
        join!(
            self.network().wait_for_ready(),
            self.fallback().wait_for_ready()
        );
    }

    async fn is_ready(&self) -> bool {
        self.network().is_ready().await && self.fallback().is_ready().await
    }

    fn shut_down<'a, 'b>(&'a self) -> BoxSyncFuture<'b, ()>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            join!(self.network().shut_down(), self.fallback().shut_down());
        };
        boxed_sync(closure)
    }

    async fn broadcast_message(
        &self,
        message: Message<TYPES, I>,
        election: &MEMBERSHIP,
    ) -> Result<(), NetworkError> {
        let recipients =
            <MEMBERSHIP as Membership<TYPES>>::get_committee(election, message.get_view_number());
        let fallback = self
            .fallback()
            .broadcast_message(message.clone(), recipients.clone());
        let network = self.network().broadcast_message(message, recipients);
        match join!(fallback, network) {
            (Err(e1), Err(e2)) => {
                error!(
                    "Both network broadcasts failed primary error: {}, fallback error: {}",
                    e1, e2
                );
                Err(e1)
            }
            (Err(e), _) => {
                error!("Failed primary broadcast with error: {}", e);
                Ok(())
            }
            (_, Err(e)) => {
                error!("Failed backup broadcast with error: {}", e);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    async fn direct_message(
        &self,
        message: Message<TYPES, I>,
        recipient: TYPES::SignatureKey,
    ) -> Result<(), NetworkError> {
        match self
            .network()
            .direct_message(message.clone(), recipient.clone())
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(
                    "Falling back on direct message, error on primary network: {}",
                    e
                );
                self.fallback().direct_message(message, recipient).await
            }
        }
    }

    fn recv_msgs<'a, 'b>(
        &'a self,
        transmit_type: TransmitType,
    ) -> BoxSyncFuture<'b, Result<Vec<Message<TYPES, I>>, NetworkError>>
    where
        'a: 'b,
        Self: 'b,
    {
        let closure = async move {
            match self.network().recv_msgs(transmit_type).await {
                Ok(msgs) => Ok(msgs),
                Err(e) => {
                    error!(
                        "Falling back on recv message, error on primary network: {}",
                        e
                    );
                    self.fallback().recv_msgs(transmit_type).await
                }
            }
        };
        boxed_sync(closure)
    }

    async fn queue_node_lookup(
        &self,
        view_number: ViewNumber,
        pk: TYPES::SignatureKey,
    ) -> Result<(), UnboundedSendError<Option<(ViewNumber, TYPES::SignatureKey)>>> {
        self.network()
            .queue_node_lookup(view_number, pk.clone())
            .await?;
        self.fallback().queue_node_lookup(view_number, pk).await?;

        Ok(())
    }

    async fn inject_consensus_info(&self, event: ConsensusIntentEvent<TYPES::SignatureKey>) {
        <WebServerNetwork<_, _, _> as ConnectedNetwork<
            Message<TYPES, I>,
            TYPES::SignatureKey,
        >>::inject_consensus_info(self.network(), event.clone())
        .await;

        <Libp2pNetwork<_, _> as ConnectedNetwork<
        Message<TYPES, I>,
        TYPES::SignatureKey,
        >>::inject_consensus_info(self.fallback(), event)
    .await;
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>, MEMBERSHIP: Membership<TYPES>>
    TestableChannelImplementation<
        TYPES,
        Message<TYPES, I>,
        MEMBERSHIP,
        CombinedNetworks<TYPES, I, MEMBERSHIP>,
    > for WebServerWithFallbackCommChannel<TYPES, I, MEMBERSHIP>
{
    fn generate_network() -> Box<dyn Fn(Arc<Self::NETWORK>) -> Self + 'static> {
        Box::new(move |network| WebServerWithFallbackCommChannel::new(network))
    }
}
