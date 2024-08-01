use std::collections::HashSet;

use async_trait::async_trait;
use espresso_types::v0_3::BidTxBody;

use espresso_types::v0_3::RollupRegistration;
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

/// Configurations for bid submission.
pub struct BidConfig {
    /// Namespace ID to filter and bid for.
    pub namespace_id: NamespaceId,
    /// Amount to bid.
    pub amount: FeeAmount,
}

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

/// Non-generic builder hooks for espresso sequencer.
///
/// Provides bidding and transaction filtering on top of base builder functionality.
pub(crate) struct EspressoNormalHooks {
    /// Bid configuraitons.
    pub(crate) bid_config: BidConfig,
    /// Base API to contact the solver
    pub(crate) solver_api_url: Url,
    /// Builder API base to include in the bid
    pub(crate) builder_api_base_url: Url,
    /// Keys for bidding
    pub(crate) bid_key_pair: EthKeyPair,
}

#[async_trait]
impl BuilderHooks<SeqTypes> for EspressoNormalHooks {
    #[inline(always)]
    async fn process_transactions(
        &mut self,
        mut transactions: Vec<<SeqTypes as NodeType>::Transaction>,
    ) -> Vec<<SeqTypes as NodeType>::Transaction> {
        transactions.retain(|txn| txn.namespace() == self.bid_config.namespace_id);
        transactions
    }

    #[inline(always)]
    async fn handle_hotshot_event(&mut self, event: &Event<SeqTypes>) {
        if let EventType::ViewFinished { view_number } = event.event {
            let bid_tx = match BidTxBody::new(
                self.bid_key_pair.fee_account(),
                self.bid_config.amount,
                view_number + 3, // We submit a bid three views in advance.
                vec![self.bid_config.namespace_id],
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

/// Generic builder hooks for espresso sequencer.
///
/// Provides transaction filtering on top of base builder functionality.
pub(crate) struct EspressoGenericHooks {
    /// Base API to contact the solver
    pub(crate) solver_api_url: Url,
}

#[async_trait]
impl BuilderHooks<SeqTypes> for EspressoGenericHooks {
    #[inline(always)]
    async fn process_transactions(
        &mut self,
        mut transactions: Vec<<SeqTypes as NodeType>::Transaction>,
    ) -> Vec<<SeqTypes as NodeType>::Transaction> {
        let Some(solver_client) = connect_to_solver(self.solver_api_url.clone()).await else {
            error!("Failed to connect to the solver service.");
            return Vec::new();
        };

        let mut registered_namespaces = Vec::new();
        let registrations: Vec<RollupRegistration> =
            match solver_client.get("rollup_registrations").send().await {
                Ok(registrations) => registrations,
                Err(e) => {
                    error!("Failed to get the registered rollups: {:?}.", e);
                    return Vec::new();
                }
            };
        for registration in registrations {
            registered_namespaces.push(registration.body.namespace_id);
        }

        transactions.retain(|txn| !registered_namespaces.contains(&txn.namespace()));
        transactions
    }

    #[inline(always)]
    async fn handle_hotshot_event(&mut self, _event: &Event<SeqTypes>) {}
}
