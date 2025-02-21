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
    FeeVersion, MarketplaceVersion, SeqTypes, SequencerVersions,
    eth_signature_key::EthKeyPair,
    v0::traits::{PersistenceOptions, SequencerPersistence, StateCatchup},
    v0_99::BidTxBody,
};
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{MnemonicBuilder, Signer as _, Wallet, coins_bip39::English},
    types::{Address, U256},
};
use futures::{
    future::{Future, join_all},
    stream::{Stream, StreamExt},
};
use hotshot::{
    HotShotInitializer, SystemContext,
    traits::election::static_committee::StaticCommittee,
    types::{SignatureKey, SystemContextHandle},
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
    HotShotConfig, PeerConfig, ValidatorConfig,
    consensus::ConsensusMetricsValue,
    event::LeafInfo,
    light_client::StateKeyPair,
    signature_key::{BLSPrivKey, BLSPubKey},
    traits::{
        block_contents::{
            BlockHeader, BlockPayload, EncodeBytes, GENESIS_VID_NUM_STORAGE_NODES, vid_commitment,
        },
        election::Membership,
        metrics::Metrics,
        node_implementation::{ConsensusTime, NodeType, Versions},
    },
    utils::BuilderCommitment,
};
use jf_merkle_tree::{MerkleTreeScheme, namespaced_merkle_tree::NamespacedMerkleTreeScheme};
use jf_signature::bls_over_bn254::VerKey;
use sequencer::{
    L1Params, NetworkParams, Node, SequencerApiVersion,
    catchup::StatePeers,
    context::{Consensus, SequencerContext},
    network,
    state_signature::{StakeTableCommitmentType, StateSigner, static_stake_table_commitment},
};
use surf_disco::Client;
use tide_disco::{App, Url, app, method::ReadState};
use tracing::error;
use vbs::version::{StaticVersion, StaticVersionType};

pub mod builder;

pub mod hooks;
