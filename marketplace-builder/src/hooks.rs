use std::collections::HashSet;

use async_trait::async_trait;
use espresso_types::v0_3::BidTxBody;

use espresso_types::v0_3::RollupRegistration;

use espresso_types::SeqTypes;
use ethers::abi::Hash;
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

/// Get the registered rollups from the solver.
async fn rollup_registrations(solver_api_url: Url) -> Vec<RollupRegistration> {
    let Some(solver_client) = connect_to_solver(solver_api_url).await else {
        error!("Failed to connect to the solver service.");
        return Vec::new();
    };

    match solver_client.get("rollup_registrations").send().await {
        Ok(registrations) => registrations,
        Err(e) => {
            error!("Failed to get the registered rollups: {:?}.", e);
            Vec::new()
        }
    }
}

/// Reserve builder hooks for espresso sequencer.
///
/// Provides bidding and transaction filtering on top of base builder functionality.
pub(crate) struct EspressoReserveHooks {
    /// Base API to contact the solver
    pub(crate) solver_api_url: Url,
    /// Builder API base to include in the bid
    pub(crate) builder_api_base_url: Url,
    /// Keys for bidding
    pub(crate) bid_key_pair: EthKeyPair,
    /// Bid amount
    pub(crate) bid_amount: FeeAmount,
}

impl EspressoReserveHooks {
    /// Namespaces to submit transactions and bid for.
    async fn namespaces(&self) -> HashSet<NamespaceId> {
        let mut namespaces = HashSet::new();
        for registration in rollup_registrations(self.solver_api_url.clone()).await {
            // Rollups that have reserve builders should be served by the reserve builder.
            if registration.body.reserve_url.is_some() {
                namespaces.insert(registration.body.namespace_id);
            }
        }

        namespaces
    }
}

#[async_trait]
impl BuilderHooks<SeqTypes> for EspressoReserveHooks {
    #[inline(always)]
    async fn process_transactions(
        &mut self,
        mut transactions: Vec<<SeqTypes as NodeType>::Transaction>,
    ) -> Vec<<SeqTypes as NodeType>::Transaction> {
        let namespaces_to_include = self.namespaces().await;

        transactions.retain(|txn| namespaces_to_include.contains(&txn.namespace()));
        transactions
    }

    #[inline(always)]
    async fn handle_hotshot_event(&mut self, event: &Event<SeqTypes>) {
        if let EventType::ViewFinished { view_number } = event.event {
            let bid_tx = match BidTxBody::new(
                self.bid_key_pair.fee_account(),
                self.bid_amount,
                view_number + 3, // We submit a bid 3 views in advance.
                self.namespaces().await.iter().cloned().collect(),
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
        let mut namespaces_to_skip = HashSet::new();
        for registration in rollup_registrations(self.solver_api_url.clone()).await {
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
