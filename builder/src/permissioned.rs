use anyhow::Context;
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
    traits::{
        election::static_committee::{GeneralStaticCommittee, StaticElectionConfig},
        implementations::{
            derive_libp2p_peer_id, CombinedNetworks, KeyPair, Libp2pNetwork,
            NetworkingMetricsValue, PushCdnNetwork, WebServerNetwork, WrappedSignatureKey,
        },
    },
    types::{SignatureKey, SystemContextHandle},
    HotShotInitializer, Memberships, Networks, SystemContext,
};
use hotshot_orchestrator::{
    client::{OrchestratorClient, ValidatorArgs},
    config::NetworkConfig,
};
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    event::Event,
    light_client::StateKeyPair,
    signature_key::{BLSPrivKey, BLSPubKey},
    traits::{election::Membership, metrics::Metrics},
    utils::BuilderCommitment,
    HotShotConfig, PeerConfig, ValidatorConfig,
};
use std::fmt::Display;
// Should move `STAKE_TABLE_CAPACITY` in the sequencer repo when we have variate stake table support
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;

use async_broadcast::{
    broadcast, Receiver as BroadcastReceiver, RecvError, Sender as BroadcastSender, TryRecvError,
};
use async_compatibility_layer::{
    art::{async_sleep, async_spawn},
    channel::{unbounded, UnboundedReceiver, UnboundedSender},
};
use async_std::sync::RwLock;
use async_std::{
    sync::Arc,
    task::{spawn, JoinHandle},
};
use hotshot_builder_api::builder::{
    BuildError, Error as BuilderApiError, Options as HotshotBuilderApiOptions,
};
use hotshot_builder_core::{
    builder_state::{
        BuildBlockInfo, BuilderProgress, BuilderState, BuiltFromProposedBlock, MessageType,
        ResponseMessage,
    },
    service::{
        run_non_permissioned_standalone_builder_service,
        run_permissioned_standalone_builder_service, GlobalState,
    },
};
use hotshot_state_prover;
use jf_primitives::{
    merkle_tree::{namespaced_merkle_tree::NamespacedMerkleTreeScheme, MerkleTreeScheme},
    signatures::bls_over_bn254::VerKey,
};
use sequencer::state_signature::StakeTableCommitmentType;
use sequencer::{catchup::mock::MockStateCatchup, eth_signature_key::EthKeyPair, ChainConfig};
use sequencer::{
    catchup::StatePeers,
    context::{Consensus, SequencerContext},
    l1_client::L1Client,
    network,
    persistence::SequencerPersistence,
    state::FeeAccount,
    state::ValidatedState,
    state_signature::{static_stake_table_commitment, StateSigner},
    BuilderParams, L1Params, NetworkParams, Node, NodeState, PrivKey, PubKey, SeqTypes,
};
use std::{alloc::System, any, fmt::Debug, mem};
use std::{marker::PhantomData, net::IpAddr};
use std::{net::Ipv4Addr, thread::Builder};
use tide_disco::{app, method::ReadState, App, Url};
use vbs::version::StaticVersionType;

use hotshot_types::{
    constants::{Version01, STATIC_VER_0_1},
    data::{fake_commitment, Leaf, ViewNumber},
    traits::{
        block_contents::{vid_commitment, GENESIS_VID_NUM_STORAGE_NODES},
        node_implementation::{ConsensusTime, NodeType},
    },
};

use hotshot_events_service::{
    events::{Error as EventStreamApiError, Options as EventStreamingApiOptions},
    events_source::{BuilderEvent, EventConsumer, EventsStreamer},
};
type ElectionConfig = StaticElectionConfig;
use crate::run_builder_api_service;
use std::{num::NonZeroUsize, time::Duration};
use surf_disco::Client;

pub struct BuilderContext<
    N: network::Type,
    P: SequencerPersistence,
    Ver: StaticVersionType + 'static,
> {
    /// The consensus handle
    pub hotshot_handle: Consensus<N, P>,

    /// Index of this sequencer node
    pub node_index: u64,

    /// Context for generating state signatures.
    pub state_signer: Arc<StateSigner<Ver>>,

    /// An orchestrator to wait for before starting consensus.
    pub wait_for_orchestrator: Option<Arc<OrchestratorClient>>,

    /// global state
    pub global_state: Arc<RwLock<GlobalState<SeqTypes>>>,

    /// hotshot builder api url
    pub hotshot_builder_api_url: Url,
}

#[allow(clippy::too_many_arguments)]
pub async fn init_node<P: SequencerPersistence, Ver: StaticVersionType + 'static>(
    network_params: NetworkParams,
    metrics: &dyn Metrics,
    builder_params: BuilderParams,
    l1_params: L1Params,
    hotshot_builder_api_url: Url,
    eth_key_pair: EthKeyPair,
    bootstrapped_view: ViewNumber,
    channel_capacity: NonZeroUsize,
    bind_version: Ver,
    persistence: P,
) -> anyhow::Result<BuilderContext<network::Production, P, Ver>> {
    // Orchestrator client
    let validator_args = ValidatorArgs {
        url: network_params.orchestrator_url,
        advertise_address: Some(network_params.libp2p_advertise_address),
        network_config_file: None,
    };
    let orchestrator_client = OrchestratorClient::new(validator_args);
    let state_key_pair = StateKeyPair::from_sign_key(network_params.private_state_key);
    let my_config = ValidatorConfig {
        public_key: BLSPubKey::from_private(&network_params.private_staking_key),
        private_key: network_params.private_staking_key,
        stake_value: 1,
        state_key_pair,
    };

    // Derive our Libp2p public key from our private key
    let libp2p_public_key =
        derive_libp2p_peer_id::<<SeqTypes as NodeType>::SignatureKey>(&my_config.private_key)
            .with_context(|| "Failed to derive Libp2p peer ID")?;

    let config = NetworkConfig::get_complete_config(
        &orchestrator_client,
        None,
        my_config.clone(),
        // Register in our Libp2p advertise address and public key so other nodes
        // can contact us on startup
        Some(network_params.libp2p_advertise_address),
        Some(libp2p_public_key),
    )
    .await?
    .0;

    tracing::info!(
    node_id = config.node_index,
    stake_table = ?config.config.known_nodes_with_stake,
    "loaded config",
    );

    let node_index = config.node_index;

    // Initialize the push CDN network (and perform the initial connection)
    let cdn_network = PushCdnNetwork::new(
        network_params.cdn_endpoint,
        vec!["Global".into(), "DA".into()],
        KeyPair {
            public_key: WrappedSignatureKey(my_config.public_key),
            private_key: my_config.private_key.clone(),
        },
    )
    .with_context(|| "Failed to create CDN network")?;

    // Initialize the Libp2p network (if enabled)
    #[cfg(feature = "libp2p")]
    let p2p_network = Libp2pNetwork::from_config::<SeqTypes>(
        config.clone(),
        network_params.libp2p_bind_address,
        &my_config.public_key,
        // We need the private key so we can derive our Libp2p keypair
        // (using https://docs.rs/blake3/latest/blake3/fn.derive_key.html)
        &my_config.private_key,
    )
    .await
    .with_context(|| "Failed to create libp2p network")?;

    // Combine the communication channels
    #[cfg(feature = "libp2p")]
    let (da_network, quorum_network) = {
        (
            Arc::from(CombinedNetworks::new(
                cdn_network.clone(),
                p2p_network.clone(),
                Duration::from_secs(1),
            )),
            Arc::from(CombinedNetworks::new(
                cdn_network,
                p2p_network,
                Duration::from_secs(1),
            )),
        )
    };

    #[cfg(not(feature = "libp2p"))]
    let (da_network, quorum_network) = { (Arc::from(cdn_network.clone()), Arc::from(cdn_network)) };

    // Convert to the sequencer-compatible type
    let networks = Networks {
        da_network,
        quorum_network,
        _pd: Default::default(),
    };

    // The web server network doesn't have any metrics. By creating and dropping a
    // `NetworkingMetricsValue`, we ensure the networking metrics are created, but just not
    // populated, so that monitoring software built to work with network-related metrics doesn't
    // crash horribly just because we're not using the P2P network yet.
    let _ = NetworkingMetricsValue::new(metrics);

    // creating the instance state without any builder mnemonic
    let wallet = MnemonicBuilder::<English>::default()
        .phrase::<&str>(&builder_params.mnemonic)
        .index(builder_params.eth_account_index)?
        .build()?;
    tracing::info!("Builder account address {:?}", wallet.address());

    let mut genesis_state = ValidatedState::default();
    for address in builder_params.prefunded_accounts {
        tracing::warn!("Prefunding account {:?} for demo", address);
        genesis_state.prefund_account(address.into(), U256::max_value().into());
    }

    let l1_client = L1Client::new(l1_params.url, Address::default());

    let instance_state = NodeState::new(
        ChainConfig::default(),
        l1_client,
        wallet,
        Arc::new(StatePeers::<Ver>::from_urls(network_params.state_peers)),
    );

    let stake_table_commit =
        static_stake_table_commitment(&config.config.known_nodes_with_stake, STAKE_TABLE_CAPACITY);

    let (hotshot_handle, state_signer) = init_hotshot(
        config.config,
        None,
        instance_state.clone(),
        networks,
        metrics,
        node_index,
        Some(network_params.state_relay_server_url),
        stake_table_commit,
        bind_version,
        persistence,
    )
    .await;

    let ctx = BuilderContext::init(
        hotshot_handle,
        state_signer,
        node_index,
        eth_key_pair,
        bootstrapped_view,
        channel_capacity,
        instance_state,
        hotshot_builder_api_url,
    )
    .await?;

    Ok(ctx)
}

#[allow(clippy::too_many_arguments)]
pub async fn init_hotshot<
    N: network::Type,
    P: SequencerPersistence,
    Ver: StaticVersionType + 'static,
>(
    config: HotShotConfig<PubKey, ElectionConfig>,
    stake_table_entries_for_non_voting_nodes: Option<
        Vec<PeerConfig<hotshot_state_prover::QCVerKey>>,
    >,
    instance_state: NodeState,
    networks: Networks<SeqTypes, Node<N, P>>,
    metrics: &dyn Metrics,
    node_id: u64,
    state_relay_server: Option<Url>,
    stake_table_commit: StakeTableCommitmentType,
    _: Ver,
    persistence: P,
) -> (SystemContextHandle<SeqTypes, Node<N, P>>, StateSigner<Ver>) {
    let election_config = GeneralStaticCommittee::<SeqTypes, PubKey>::default_election_config(
        config.num_nodes_with_stake.get() as u64,
        config.num_nodes_without_stake as u64,
    );
    let combined_known_nodes_with_stake = match stake_table_entries_for_non_voting_nodes {
        Some(stake_table_entries) => {
            let combined_entries = config
                .known_nodes_with_stake
                .iter()
                .cloned()
                .chain(stake_table_entries.into_iter())
                .collect();
            combined_entries
        }
        None => config.known_nodes_with_stake.clone(),
    };
    let membership = GeneralStaticCommittee::create_election(
        combined_known_nodes_with_stake,
        election_config,
        0,
    );
    let memberships = Memberships {
        quorum_membership: membership.clone(),
        da_membership: membership.clone(),
        vid_membership: membership.clone(),
        view_sync_membership: membership,
    };
    let state_key_pair = config.my_own_validator_config.state_key_pair.clone();

    let da_storage = Arc::new(RwLock::new(persistence));
    tracing::debug!("Before hotshot handle initialisation");
    let hotshot_handle = SystemContext::init(
        config.my_own_validator_config.public_key,
        config.my_own_validator_config.private_key.clone(),
        node_id,
        config,
        memberships,
        networks,
        HotShotInitializer::from_genesis(instance_state).unwrap(),
        ConsensusMetricsValue::new(metrics),
        da_storage,
    )
    .await
    .unwrap()
    .0;

    tracing::debug!("Hotshot handle initialized");

    let mut state_signer: StateSigner<Ver> = StateSigner::new(state_key_pair, stake_table_commit);

    if let Some(url) = state_relay_server {
        state_signer = state_signer.with_relay_server(url);
    }
    (hotshot_handle, state_signer)
}

impl<N: network::Type, P: SequencerPersistence, Ver: StaticVersionType + 'static>
    BuilderContext<N, P, Ver>
{
    /// Constructor
    #[allow(clippy::too_many_arguments)]
    pub async fn init(
        hotshot_handle: Consensus<N, P>,
        state_signer: StateSigner<Ver>,
        node_index: u64,
        eth_key_pair: EthKeyPair,
        bootstrapped_view: ViewNumber,
        channel_capacity: NonZeroUsize,
        instance_state: NodeState,
        hotshot_builder_api_url: Url,
    ) -> anyhow::Result<Self> {
        // tx channel
        let (tx_sender, tx_receiver) = broadcast::<MessageType<SeqTypes>>(channel_capacity.get());

        // da channel
        let (da_sender, da_receiver) = broadcast::<MessageType<SeqTypes>>(channel_capacity.get());

        // qc channel
        let (qc_sender, qc_receiver) = broadcast::<MessageType<SeqTypes>>(channel_capacity.get());

        // decide channel
        let (decide_sender, decide_receiver) =
            broadcast::<MessageType<SeqTypes>>(channel_capacity.get());

        // builder api request channel
        let (req_sender, req_receiver) = broadcast::<MessageType<SeqTypes>>(channel_capacity.get());

        // builder api response channel
        let (res_sender, res_receiver) = unbounded();

        // create the global state
        let global_state: GlobalState<SeqTypes> = GlobalState::<SeqTypes>::new(
            (eth_key_pair.fee_account(), eth_key_pair),
            req_sender,
            res_receiver,
            tx_sender.clone(),
            instance_state.clone(),
            BuilderCommitment::from_bytes([]),
        );

        let global_state = Arc::new(RwLock::new(global_state));

        let global_state_clone = global_state.clone();

        let builder_state = BuilderState::<SeqTypes>::new(
            BuiltFromProposedBlock {
                view_number: bootstrapped_view,
                vid_commitment: vid_commitment(&vec![], GENESIS_VID_NUM_STORAGE_NODES),
                leaf_commit: fake_commitment(),
                builder_commitment: BuilderCommitment::from_bytes([]),
            },
            tx_receiver,
            decide_receiver,
            da_receiver,
            qc_receiver,
            req_receiver,
            global_state_clone,
            res_sender,
            NonZeroUsize::new(1).unwrap(),
            bootstrapped_view,
        );

        let hotshot_handle_clone = hotshot_handle.clone();
        // spawn the builder service
        async_spawn(async move {
            run_permissioned_standalone_builder_service(
                tx_sender,
                da_sender,
                qc_sender,
                decide_sender,
                hotshot_handle,
                instance_state,
            )
            .await;
        });

        // spawn the builder event loop
        async_spawn(async move {
            builder_state.event_loop();
        });

        run_builder_api_service(hotshot_builder_api_url.clone(), global_state.clone());

        let ctx = Self {
            hotshot_handle: hotshot_handle_clone,
            node_index,
            state_signer: Arc::new(state_signer),
            wait_for_orchestrator: None,
            global_state,
            hotshot_builder_api_url,
        };

        Ok(ctx)
    }

    /// Start participating in consensus.
    pub async fn start_consensus(&self) {
        if let Some(orchestrator_client) = &self.wait_for_orchestrator {
            tracing::info!("waiting for orchestrated start");
            orchestrator_client
                .wait_for_all_nodes_ready(self.node_index)
                .await;
        }
        self.hotshot_handle.hotshot.start_consensus().await;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::non_permissioned;
    use crate::testing::{HotShotTestConfig, NonPermissionedBuilderTestConfig};
    use async_compatibility_layer::art::{async_sleep, async_spawn};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_std::task;
    use hotshot_builder_api::{
        block_info::{AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo},
        builder::BuildError,
    };
    use hotshot_builder_core::builder_state::BuilderProgress;
    use hotshot_builder_core::service::{
        run_non_permissioned_standalone_builder_service,
        run_permissioned_standalone_builder_service,
    };
    use hotshot_types::constants::{Version01, STATIC_VER_0_1};
    use hotshot_types::traits::{
        block_contents::GENESIS_VID_NUM_STORAGE_NODES, node_implementation::NodeType,
    };
    use hotshot_types::{signature_key::BLSPubKey, traits::signature_key::SignatureKey};
    use sequencer::persistence::no_storage::{self, NoStorage};
    use sequencer::transaction::Transaction;
    use std::time::Duration;
    use surf_disco::Client;

    use crate::testing::{hotshot_builder_url, PermissionedBuilderTestConfig};
    use async_lock::RwLock;
    use es_version::SequencerVersion;
    use hotshot_events_service::{
        events::{Error as EventStreamApiError, Options as EventStreamingApiOptions},
        events_source::{BuilderEvent, EventConsumer, EventsStreamer},
    };
    #[async_std::test]
    async fn test_permissioned_builder() {
        setup_logging();
        setup_backtrace();

        let ver = SequencerVersion::instance();

        // Hotshot Test Config
        let hotshot_config = HotShotTestConfig::default();

        // Get the handle for all the nodes, including both the non-builder and builder nodes
        let mut handles = hotshot_config.init_nodes(ver, no_storage::Options).await;

        // start consensus for all the nodes
        for (handle, ..) in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        let total_nodes = HotShotTestConfig::total_nodes();

        let node_id = total_nodes - 1;
        // non-staking node handle
        let hotshot_context_handle = handles[node_id].0.clone();
        let state_signer = handles[node_id].1.take().unwrap();

        // builder api url
        let hotshot_builder_api_url = hotshot_config.config.builder_url.clone();
        let builder_config = PermissionedBuilderTestConfig::init_permissioned_builder(
            hotshot_config,
            hotshot_context_handle,
            node_id as u64,
            state_signer,
            hotshot_builder_api_url.clone(),
        )
        .await;

        let builder_pub_key = builder_config.fee_account;

        // Start a builder api client
        let builder_client = Client::<hotshot_builder_api::builder::Error, Version01>::new(
            hotshot_builder_api_url.clone(),
        );
        assert!(builder_client.connect(Some(Duration::from_secs(60))).await);

        let seed = [207_u8; 32];

        // Hotshot client Public, Private key
        let (hotshot_client_pub_key, hotshot_client_private_key) =
            BLSPubKey::generated_from_seed_indexed(seed, 2011_u64);

        let parent_commitment = vid_commitment(&vec![], GENESIS_VID_NUM_STORAGE_NODES);

        // sign the parent_commitment using the client_private_key
        let encoded_signature = <SeqTypes as NodeType>::SignatureKey::sign(
            &hotshot_client_private_key,
            parent_commitment.as_ref(),
        )
        .expect("Claim block signing failed");

        // test getting available blocks
        let available_block_info = match builder_client
            .get::<Vec<AvailableBlockInfo<SeqTypes>>>(&format!(
                "block_info/availableblocks/{parent_commitment}/{hotshot_client_pub_key}/{encoded_signature}"
            ))
            .send()
            .await
        {
            Ok(response) => {
                tracing::info!("Received Available Blocks: {:?}", response);
                assert!(!response.is_empty());
                response
            }
            Err(e) => {
                panic!("Error getting available blocks {:?}", e);
            }
        };

        let builder_commitment = available_block_info[0].block_hash.clone();

        // sign the builder_commitment using the client_private_key
        let encoded_signature = <SeqTypes as NodeType>::SignatureKey::sign(
            &hotshot_client_private_key,
            builder_commitment.as_ref(),
        )
        .expect("Claim block signing failed");

        // Test claiming blocks
        let _available_block_data = match builder_client
            .get::<AvailableBlockData<SeqTypes>>(&format!(
                "block_info/claimblock/{builder_commitment}/{hotshot_client_pub_key}/{encoded_signature}"
            ))
            .send()
            .await
        {
            Ok(response) => {
                tracing::info!("Received Block Data: {:?}", response);
                response
            }
            Err(e) => {
                panic!("Error while claiming block {:?}", e);
            }
        };

        // Test claiming block header input
        let _available_block_header = match builder_client
            .get::<AvailableBlockHeaderInput<SeqTypes>>(&format!(
                "block_info/claimheaderinput/{builder_commitment}/{hotshot_client_pub_key}/{encoded_signature}"
            ))
            .send()
            .await
        {
            Ok(response) => {
                tracing::info!("Received Block Header : {:?}", response);
                response
            }
            Err(e) => {
                panic!("Error getting claiming block header {:?}", e);
            }
        };

        // test getting builder key
        match builder_client
            .get::<FeeAccount>("block_info/builderaddress")
            .send()
            .await
        {
            Ok(response) => {
                tracing::info!("Received Builder Key : {:?}", response);
                assert_eq!(response, builder_pub_key);
            }
            Err(e) => {
                panic!("Error getting builder key {:?}", e);
            }
        }

        let txn = Transaction::new(Default::default(), vec![1, 2, 3]);
        match builder_client
            .post::<()>("txn_submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
        {
            Ok(response) => {
                tracing::info!("Received txn submitted response : {:?}", response);
                return;
            }
            Err(e) => {
                panic!("Error submitting private transaction {:?}", e);
            }
        }
    }
}
