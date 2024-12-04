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

use async_lock::RwLock;
use async_trait::async_trait;
use espresso_types::{
    eth_signature_key::EthKeyPair,
    v0::traits::{PersistenceOptions, SequencerPersistence, StateCatchup},
    v0_99::BidTxBody,
    FeeVersion, MarketplaceVersion, SeqTypes, SequencerVersions,
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
    traits::election::static_committee::StaticCommittee,
    types::{SignatureKey, SystemContextHandle},
    HotShotInitializer, SystemContext,
};
use hotshot_builder_api::v0_99::builder::{
    BuildError, Error as BuilderApiError, Options as HotshotBuilderApiOptions,
};
use hotshot_orchestrator::client::{OrchestratorClient, ValidatorArgs};
use hotshot_types::network::NetworkConfig;
use marketplace_builder_core::service::{GlobalState, ProxyGlobalState};
use std::sync::Arc;
use tokio::{spawn, task::JoinHandle};
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
    L1Params, NetworkParams, Node, SequencerApiVersion,
};
use surf_disco::Client;
use tide_disco::{app, method::ReadState, App, Url};
use tracing::error;
use vbs::version::{StaticVersion, StaticVersionType};

pub mod builder;

pub mod hooks;
