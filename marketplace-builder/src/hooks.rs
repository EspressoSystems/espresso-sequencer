use std::collections::HashSet;

use async_trait::async_trait;
use espresso_types::v0_3::BidTxBody;

use espresso_types::v0_3::RollupRegistration;

use espresso_types::SeqTypes;
use hotshot::types::EventType;

use hotshot::types::Event;

use hotshot_types::traits::node_implementation::Versions;
use marketplace_builder_core::service::BuilderHooks;

use espresso_types::FeeAmount;

use espresso_types::eth_signature_key::EthKeyPair;

use espresso_types::NamespaceId;

use hotshot_types::traits::node_implementation::NodeType;

use marketplace_solver::SolverError;
use sequencer::SequencerApiVersion;
use surf_disco::Client;

use tide_disco::Url;
use tracing::error;
use tracing::info;

/// Configurations for bid submission.
pub struct BidConfig {
    /// Namespace IDs to filter and bid for.
    pub namespaces: Vec<NamespaceId>,
    /// Amount to bid.
    pub amount: FeeAmount,
}

pub async fn connect_to_solver(
    solver_api_url: Url,
) -> Option<Client<SolverError, SequencerApiVersion>> {
    let client = Client::<SolverError, SequencerApiVersion>::new(
        solver_api_url.join("marketplace-solver/").unwrap(),
    );

    if !(client.connect(None).await) {
        return None;
    }

    tracing::info!(
        %solver_api_url,
        "Builder client connected to the solver api"
    );

    Some(client)
}

/// Reserve builder hooks for espresso sequencer.
///
/// Provides bidding and transaction filtering on top of base builder functionality.
pub(crate) struct EspressoReserveHooks {
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
impl BuilderHooks<SeqTypes> for EspressoReserveHooks {
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
                view_number + 3, // We submit a bid 3 views in advance.
                self.namespaces.iter().cloned().collect(),
                self.builder_api_base_url.clone(),
                Default::default(),
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
                return;
            }

            info!("Submitted bid for view {}", *view_number);
        }
    }
}

/// Fallback builder hooks for espresso sequencer.
///
/// Provides transaction filtering on top of base builder functionality for unregistered rollups.
pub(crate) struct EspressoFallbackHooks {
    /// Base API to contact the solver
    pub(crate) solver_api_url: Url,
}

#[async_trait]
impl BuilderHooks<SeqTypes> for EspressoFallbackHooks {
    #[inline(always)]
    async fn process_transactions(
        &mut self,
        mut transactions: Vec<<SeqTypes as NodeType>::Transaction>,
    ) -> Vec<<SeqTypes as NodeType>::Transaction> {
        let Some(solver_client) = connect_to_solver(self.solver_api_url.clone()).await else {
            error!("Failed to connect to the solver service.");
            return Vec::new();
        };

        let mut namespaces_to_skip = HashSet::new();
        let registrations: Vec<RollupRegistration> =
            match solver_client.get("rollup_registrations").send().await {
                Ok(registrations) => registrations,
                Err(e) => {
                    error!("Failed to get the registered rollups: {:?}.", e);
                    return Vec::new();
                }
            };
        for registration in registrations {
            // Rollups that have reserve builders or aren't active shouldn't be served by fallback builder
            if registration.body.reserve_url.is_some() || !registration.body.active {
                namespaces_to_skip.insert(registration.body.namespace_id);
            }
        }

        transactions.retain(|txn| !namespaces_to_skip.contains(&txn.namespace()));
        transactions
    }

    #[inline(always)]
    async fn handle_hotshot_event(&mut self, _event: &Event<SeqTypes>) {}
}
