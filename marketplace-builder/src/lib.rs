#![allow(unused_imports)]
use std::{
    alloc::System,
    any,
    fmt::{Debug, Display},
    marker::PhantomData,
    mem,
    net::{IpAddr, Ipv4Addr},
    thread::Builder,
};

use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_std::{
    sync::{Arc, RwLock},
    task::{spawn, JoinHandle},
};
use async_trait::async_trait;
use bid::{connect_to_solver, BidConfig};
use espresso_types::{
    v0::traits::{PersistenceOptions, SequencerPersistence, StateCatchup},
    NamespaceId, SeqTypes,
};
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{coins_bip39::English, MnemonicBuilder, Signer as _, Wallet},
    types::{Address, U256},
};
use futures::{
    future::{join_all, Future},
    stream::{Stream, StreamExt},
};
use hotshot::{
    traits::election::static_committee::GeneralStaticCommittee,
    types::{Event, EventType, SignatureKey, SystemContextHandle},
    HotShotInitializer, Memberships, SystemContext,
};
use hotshot_builder_api::v0_3::builder::{
    BuildError, Error as BuilderApiError, Options as HotshotBuilderApiOptions,
};
use hotshot_orchestrator::{
    client::{OrchestratorClient, ValidatorArgs},
    config::NetworkConfig,
};
use marketplace_builder_core::service::{BuilderHooks, GlobalState, ProxyGlobalState};
// Should move `STAKE_TABLE_CAPACITY` in the sequencer repo when we have variate stake table support
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    event::LeafInfo,
    light_client::StateKeyPair,
    signature_key::{BLSPrivKey, BLSPubKey},
    traits::{
        block_contents::{
            vid_commitment, BlockHeader, BlockPayload, EncodeBytes, GENESIS_VID_NUM_STORAGE_NODES,
        },
        election::Membership,
        metrics::Metrics,
        node_implementation::NodeType,
    },
    utils::BuilderCommitment,
    HotShotConfig, PeerConfig, ValidatorConfig,
};
use jf_merkle_tree::{namespaced_merkle_tree::NamespacedMerkleTreeScheme, MerkleTreeScheme};
use jf_signature::bls_over_bn254::VerKey;
use sequencer::{
    catchup::StatePeers,
    context::{Consensus, SequencerContext},
    network,
    state_signature::{static_stake_table_commitment, StakeTableCommitmentType, StateSigner},
    L1Params, NetworkParams, Node,
};
use tide_disco::{app, method::ReadState, App, Url};
use tracing::error;
use vbs::version::StaticVersionType;

pub mod bid;
pub mod non_permissioned;

/// Builder hooks for espresso sequencer
/// Provides bidding and transaction filtering on top of base builder functionality
struct EspressoHooks {
    /// ID of namespace to filter and bid for
    namespace_id: NamespaceId,
    /// Base API to contact the solver
    solver_api_url: Url,
    /// Builder API base to include in the bid
    builder_api_base_url: Url,
    /// Configuration for bidding
    bid_config: BidConfig,
}

#[async_trait]
impl BuilderHooks<SeqTypes> for EspressoHooks {
    #[inline(always)]
    async fn process_transactions(
        &mut self,
        mut transactions: Vec<<SeqTypes as NodeType>::Transaction>,
    ) -> Vec<<SeqTypes as NodeType>::Transaction> {
        transactions.retain(|txn| txn.namespace() == self.namespace_id);
        transactions
    }

    #[inline(always)]
    async fn handle_hotshot_event(&mut self, event: &Event<SeqTypes>) {
        if let EventType::ViewFinished { view_number } = event.event {
            // We submit a bid three views in advance.
            let bid_tx = match bid::from_bid_config(
                &self.bid_config,
                view_number + 3,
                self.builder_api_base_url.clone(),
                self.namespace_id,
            ) {
                Ok(bid) => bid,
                Err(e) => {
                    error!("Failed to construct the bid txn: {:?}.", e);
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
        }
    }
}

// It runs the api service for the builder
pub fn run_builder_api_service(url: Url, source: ProxyGlobalState<SeqTypes>) {
    // it is to serve hotshot
    let builder_api = hotshot_builder_api::v0_3::builder::define_api::<
        ProxyGlobalState<SeqTypes>,
        SeqTypes,
    >(&HotshotBuilderApiOptions::default())
    .expect("Failed to construct the builder APIs");

    // it enables external clients to submit txn to the builder's private mempool
    let private_mempool_api = hotshot_builder_api::v0_3::builder::submit_api::<
        ProxyGlobalState<SeqTypes>,
        SeqTypes,
        <SeqTypes as NodeType>::Base,
    >(&HotshotBuilderApiOptions::default())
    .expect("Failed to construct the builder API for private mempool txns");

    let mut app: App<ProxyGlobalState<SeqTypes>, BuilderApiError> = App::with_state(source);

    app.register_module("block_info", builder_api)
        .expect("Failed to register the builder API");

    app.register_module("txn_submit", private_mempool_api)
        .expect("Failed to register the private mempool API");

    async_spawn(app.serve(url, <SeqTypes as NodeType>::Base::instance()));
}
