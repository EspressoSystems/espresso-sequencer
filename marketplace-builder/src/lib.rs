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
use espresso_types::{
    eth_signature_key::EthKeyPair,
    v0::traits::{PersistenceOptions, SequencerPersistence, StateCatchup},
    v0_3::BidTxBody,
    SeqTypes, SequencerVersions,
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
    types::{SignatureKey, SystemContextHandle},
    HotShotInitializer, Memberships, SystemContext,
};
use hotshot_builder_api::v0_3::builder::{
    BuildError, Error as BuilderApiError, Options as HotshotBuilderApiOptions,
};
use hotshot_orchestrator::{
    client::{OrchestratorClient, ValidatorArgs},
    config::NetworkConfig,
};
use marketplace_builder_core::service::{GlobalState, ProxyGlobalState};
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
        node_implementation::{ConsensusTime, NodeType, Versions},
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
use surf_disco::Client;
use tide_disco::{app, method::ReadState, App, Url};
use tracing::error;
use vbs::version::StaticVersionType;

pub mod builder;

pub mod hooks;

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
        <SequencerVersions as Versions>::Base,
    >(&HotshotBuilderApiOptions::default())
    .expect("Failed to construct the builder API for private mempool txns");

    let mut app: App<ProxyGlobalState<SeqTypes>, BuilderApiError> = App::with_state(source);

    app.register_module("block_info", builder_api)
        .expect("Failed to register the builder API");

    app.register_module("txn_submit", private_mempool_api)
        .expect("Failed to register the private mempool API");

    async_spawn(app.serve(url, <SequencerVersions as Versions>::Base::instance()));
}
