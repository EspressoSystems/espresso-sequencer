use std::collections::HashSet;

use async_trait::async_trait;
use espresso_types::v0_3::BidTxBody;

use espresso_types::SeqTypes;
use hotshot::types::EventType;

use hotshot::types::Event;

use marketplace_builder_core::service::BuilderHooks;

use espresso_types::FeeAmount;

use espresso_types::eth_signature_key::EthKeyPair;

use espresso_types::NamespaceId;

use hotshot_types::traits::node_implementation::NodeType;

use surf_disco::Client;

use tide_disco::Url;
use tracing::error;
use tracing::info;

pub async fn connect_to_solver(
    solver_api_url: Url,
) -> Option<Client<hotshot_events_service::events::Error, <SeqTypes as NodeType>::Base>> {
    let client = Client::<hotshot_events_service::events::Error, <SeqTypes as NodeType>::Base>::new(
        solver_api_url.join("v0/marketplace-solver/").unwrap(),
    );

    if !(client.connect(None).await) {
        return None;
    }

    tracing::info!("Builder client connected to the solver api");

    Some(client)
}

/// Builder hooks for espresso sequencer
/// Provides bidding and transaction filtering on top of base builder functionality
pub(crate) struct EspressoHooks {
    /// IDs of namespaces to filter and bid for
    pub(crate) namespaces: HashSet<NamespaceId>,
    /// Base API to contact the solver
    pub(crate) solver_api_url: Url,
    /// Builder API base to include in the bid
    pub(crate) builder_api_base_url: Url,
    /// Keys for bidding
    pub(crate) bid_key_pair: EthKeyPair,
    /// Bid amount
    pub(crate) bid_amount: FeeAmount,
}

#[async_trait]
impl BuilderHooks<SeqTypes> for EspressoHooks {
    #[inline(always)]
    async fn process_transactions(
        &mut self,
        mut transactions: Vec<<SeqTypes as NodeType>::Transaction>,
    ) -> Vec<<SeqTypes as NodeType>::Transaction> {
        transactions.retain(|txn| self.namespaces.contains(&txn.namespace()));
        transactions
    }

    #[inline(always)]
    async fn handle_hotshot_event(&mut self, event: &Event<SeqTypes>) {
        if let EventType::ViewFinished { view_number } = event.event {
            let bid_tx = match BidTxBody::new(
                self.bid_key_pair.fee_account(),
                self.bid_amount,
                view_number + 3, // We submit a bid three views in advance.
                self.namespaces.iter().cloned().collect(),
                self.builder_api_base_url.clone(),
            )
            .signed(&self.bid_key_pair)
            {
                Ok(bid) => bid,
                Err(e) => {
                    error!("Failed to sign the bid txn: {:?}.", e);
                    return;
                }
            };

            let Some(solver_client) = connect_to_solver(self.solver_api_url.clone()).await else {
                error!("Failed to connect to the solver service.");
                return;
            };

            if let Err(e) = solver_client
                .post::<()>("submit_bid")
                .body_json(&bid_tx)
                .unwrap()
                .send()
                .await
            {
                error!("Failed to submit the bid: {:?}.", e);
            }

            info!("Submitted bid for view {}", *view_number);
        }
    }
}
