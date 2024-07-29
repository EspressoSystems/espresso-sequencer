use std::fs;

use espresso_types::{
    v0_3::{BidTx, BidTxBody},
    FeeAccount, FeeAmount, NamespaceId, SeqTypes,
};
use ethers::types::U256;
use hotshot_builder_api::v0_3::builder::BuildError;
use hotshot_types::traits::{
    node_implementation::ConsensusTime, signature_key::BuilderSignatureKey,
};
use hotshot_types::{data::ViewNumber, traits::node_implementation::NodeType};
use serde::Deserialize;
use surf_disco::Client;
use toml::from_str;
use url::Url;

/// Configurations for the bid construction.
///
/// See `bid-config.rs` for an example.
#[derive(Clone, Debug, Deserialize)]
pub struct BidConfig {
    pub account_seed: [u8; 32],
    pub account_index: u64,
    pub bid_amount: U256,
}

/// Read the bid configuration file.
pub fn read_bid_config_file(file_path: &str) -> Result<BidConfig, toml::de::Error> {
    let contents = fs::read_to_string(file_path).expect("Failed to open or read bid config file");
    from_str(&contents)
}

/// Construct a bid transaction from bid configurations.
///
/// Bid configurations can be found via `read_bid_config_file`.
pub fn from_bid_config(
    bid_config: &BidConfig,
    view_number: ViewNumber,
    bid_base_url: Url,
    namespace: NamespaceId,
) -> Result<BidTx, BuildError> {
    let (account, key) =
        FeeAccount::generated_from_seed_indexed(bid_config.account_seed, bid_config.account_index);
    let bid_amount = FeeAmount(bid_config.bid_amount);
    let namespaces = vec![namespace];

    BidTxBody::new(
        account,
        bid_amount,
        hotshot_types::data::ViewNumber::new(*view_number),
        namespaces,
        bid_base_url,
    )
    .signed(&key)
    .map_err(|e| BuildError::Error {
        message: format!("Failed to sign the bid: {:?}", e),
    })
}

pub async fn connect_to_solver(
    solver_api_url: Url,
) -> Option<Client<hotshot_events_service::events::Error, <SeqTypes as NodeType>::Base>> {
    let client = Client::<hotshot_events_service::events::Error, <SeqTypes as NodeType>::Base>::new(
        solver_api_url,
    );

    if !(client.connect(None).await) {
        return None;
    }

    tracing::info!("Builder client connected to the solver api");

    Some(client)
}
