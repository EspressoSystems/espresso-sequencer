use std::{
    alloc::System,
    any,
    fmt::{Debug, Display},
    marker::PhantomData,
    mem,
    net::{IpAddr, Ipv4Addr},
    num::NonZeroUsize,
    str::FromStr,
    thread::Builder,
    time::Duration,
};

use anyhow::Context;
use async_broadcast::{
    broadcast, Receiver as BroadcastReceiver, RecvError, Sender as BroadcastSender, TryRecvError,
};
use async_compatibility_layer::{
    art::{async_sleep, async_spawn},
    channel::{unbounded, UnboundedReceiver, UnboundedSender},
};
use async_std::{
    sync::{Arc, RwLock},
    task::{spawn, JoinHandle},
};
use espresso_types::{
    eth_signature_key::EthKeyPair,
    v0::traits::{PersistenceOptions, SequencerPersistence, StateCatchup},
    FeeAmount, L1Client, NodeState, Payload, PubKey, SeqTypes, SolverAuctionResultsProvider,
    ValidatedState,
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
    traits::{
        election::static_committee::GeneralStaticCommittee,
        implementations::{
            derive_libp2p_peer_id, CdnMetricsValue, CdnTopic, CombinedNetworks, KeyPair,
            Libp2pNetwork, PushCdnNetwork, WrappedSignatureKey,
        },
        BlockPayload,
    },
    types::{SignatureKey, SystemContextHandle},
    HotShotInitializer, MarketplaceConfig, Memberships, SystemContext,
};
use hotshot_builder_api::v0_1::builder::{
    BuildError, Error as BuilderApiError, Options as HotshotBuilderApiOptions,
};
use hotshot_builder_core::{
    builder_state::{
        BuildBlockInfo, BuilderState, BuiltFromProposedBlock, MessageType, ResponseMessage,
    },
    service::{
        run_non_permissioned_standalone_builder_service,
        run_permissioned_standalone_builder_service, GlobalState, ProxyGlobalState,
        ReceivedTransaction,
    },
};
use hotshot_events_service::{
    events::{Error as EventStreamApiError, Options as EventStreamingApiOptions},
    events_source::{EventConsumer, EventsStreamer},
};
use hotshot_example_types::auction_results_provider_types::TestAuctionResultsProvider;
use hotshot_orchestrator::{
    client::{OrchestratorClient, ValidatorArgs},
    config::NetworkConfig,
};
// Should move `STAKE_TABLE_CAPACITY` in the sequencer repo when we have variate stake table support
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;
use hotshot_state_prover;
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    data::{fake_commitment, Leaf, ViewNumber},
    event::Event,
    light_client::StateKeyPair,
    signature_key::{BLSPrivKey, BLSPubKey},
    traits::{
        auction_results_provider::AuctionResultsProvider,
        block_contents::{vid_commitment, GENESIS_VID_NUM_STORAGE_NODES},
        election::Membership,
        metrics::Metrics,
        network::{ConnectedNetwork, Topic},
        node_implementation::{ConsensusTime, NodeType},
        EncodeBytes,
    },
    utils::BuilderCommitment,
    HotShotConfig, PeerConfig, ValidatorConfig,
};
use jf_merkle_tree::{namespaced_merkle_tree::NamespacedMerkleTreeScheme, MerkleTreeScheme};
use jf_signature::bls_over_bn254::VerKey;
use sequencer::{
    catchup::StatePeers,
    context::{Consensus, SequencerContext},
    genesis::L1Finalized,
    network,
    network::libp2p::split_off_peer_id,
    state_signature::{static_stake_table_commitment, StakeTableCommitmentType, StateSigner},
    Genesis, L1Params, NetworkParams, Node,
};
use surf_disco::Client;
use tide_disco::{app, method::ReadState, App, Url};
use vbs::version::StaticVersionType;

use crate::run_builder_api_service;

pub struct BuilderContext<
    N: ConnectedNetwork<PubKey>,
    P: SequencerPersistence,
    Ver: StaticVersionType + 'static,
> {
    /// The consensus handle
    pub hotshot_handle: Arc<Consensus<N, P>>,

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
    genesis: Genesis,
    network_params: NetworkParams,
    metrics: &dyn Metrics,
    l1_params: L1Params,
    hotshot_builder_api_url: Url,
    eth_key_pair: EthKeyPair,
    bootstrapped_view: ViewNumber,
    tx_channel_capacity: NonZeroUsize,
    event_channel_capacity: NonZeroUsize,
    bind_version: Ver,
    persistence: P,
    max_api_timeout_duration: Duration,
    buffered_view_num_count: usize,
    is_da: bool,
    maximize_txns_count_timeout_duration: Duration,
) -> anyhow::Result<BuilderContext<network::Production, P, Ver>> {
    // Orchestrator client
    let validator_args = ValidatorArgs {
        url: network_params.orchestrator_url,
        advertise_address: Some(network_params.libp2p_advertise_address),
        builder_address: None,
        network_config_file: None,
    };
    let orchestrator_client = OrchestratorClient::new(validator_args);
    let state_key_pair = StateKeyPair::from_sign_key(network_params.private_state_key);
    let my_config = ValidatorConfig {
        public_key: BLSPubKey::from_private(&network_params.private_staking_key),
        private_key: network_params.private_staking_key,
        stake_value: 1,
        state_key_pair,
        is_da,
    };

    // Derive our Libp2p public key from our private key
    let libp2p_public_key =
        derive_libp2p_peer_id::<<SeqTypes as NodeType>::SignatureKey>(&my_config.private_key)
            .with_context(|| "Failed to derive Libp2p peer ID")?;

    let mut config = NetworkConfig::get_complete_config(
        &orchestrator_client,
        my_config.clone(),
        // Register in our Libp2p advertise address and public key so other nodes
        // can contact us on startup
        Some(network_params.libp2p_advertise_address),
        Some(libp2p_public_key),
    )
    .await?
    .0;

    // If the `Libp2p` bootstrap nodes were supplied via the command line, override those
    // present in the config file.
    if let Some(bootstrap_nodes) = network_params.libp2p_bootstrap_nodes {
        if let Some(libp2p_config) = config.libp2p_config.as_mut() {
            // If the libp2p configuration is present, we can override the bootstrap nodes.

            // Split off the peer ID from the addresses
            libp2p_config.bootstrap_nodes = bootstrap_nodes
                .into_iter()
                .map(split_off_peer_id)
                .collect::<Result<Vec<_>, _>>()
                .with_context(|| "Failed to parse peer ID from bootstrap node")?;
        } else {
            // If not, don't try launching with them. Eventually we may want to
            // provide a default configuration here instead.
            tracing::warn!("No libp2p configuration found, ignoring bootstrap nodes");
        }
    }

    tracing::info!(
    node_id = config.node_index,
    stake_table = ?config.config.known_nodes_with_stake,
    "loaded config",
    );

    let node_index = config.node_index;

    // Initialize the push CDN network (and perform the initial connection)
    let cdn_network = PushCdnNetwork::new(
        network_params.cdn_endpoint,
        vec![CdnTopic::Global, CdnTopic::Da],
        KeyPair {
            public_key: WrappedSignatureKey(my_config.public_key),
            private_key: my_config.private_key.clone(),
        },
        CdnMetricsValue::new(metrics),
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
        hotshot::traits::implementations::Libp2pMetricsValue::new(metrics),
    )
    .await
    .with_context(|| "Failed to create libp2p network")?;

    // Combine the communication channels
    #[cfg(feature = "libp2p")]
    let network = Arc::new(CombinedNetworks::new(
        cdn_network,
        p2p_network,
        Some(Duration::from_secs(1)),
    ));

    #[cfg(not(feature = "libp2p"))]
    let network = Arc::from(cdn_network.clone());

    let base_fee = genesis.max_base_fee();
    let mut genesis_state = ValidatedState {
        chain_config: genesis.chain_config.into(),
        ..Default::default()
    };
    for (address, amount) in genesis.accounts {
        tracing::warn!(%address, %amount, "Prefunding account for demo");
        genesis_state.prefund_account(address, amount);
    }

    let l1_client = L1Client::new(l1_params.url, l1_params.events_max_block_range);
    let l1_genesis = match genesis.l1_finalized {
        Some(L1Finalized::Block(b)) => Some(b),
        Some(L1Finalized::Number { number }) => {
            Some(l1_client.wait_for_finalized_block(number).await)
        }
        None => None,
    };

    let instance_state = NodeState {
        chain_config: genesis.chain_config,
        l1_client,
        genesis_header: genesis.header,
        genesis_state: genesis_state.clone(),
        l1_genesis,
        peers: Arc::new(StatePeers::<Ver>::from_urls(
            network_params.state_peers,
            network_params.catchup_backoff,
        )),
        node_id: node_index,
        upgrades: Default::default(),
        current_version: Ver::VERSION,
    };

    let stake_table_commit =
        static_stake_table_commitment(&config.config.known_nodes_with_stake, STAKE_TABLE_CAPACITY);

    let (hotshot_handle, state_signer) = init_hotshot(
        config.config,
        None,
        instance_state.clone(),
        network,
        metrics,
        node_index,
        Some(network_params.state_relay_server_url),
        stake_table_commit,
        bind_version,
        persistence,
    )
    .await;

    let ctx = BuilderContext::init(
        Arc::new(hotshot_handle),
        state_signer,
        node_index,
        eth_key_pair,
        bootstrapped_view,
        tx_channel_capacity,
        event_channel_capacity,
        instance_state,
        genesis_state,
        hotshot_builder_api_url,
        max_api_timeout_duration,
        buffered_view_num_count,
        maximize_txns_count_timeout_duration,
        base_fee,
    )
    .await?;

    Ok(ctx)
}

#[allow(clippy::too_many_arguments)]
pub async fn init_hotshot<
    N: ConnectedNetwork<PubKey>,
    P: SequencerPersistence,
    Ver: StaticVersionType + 'static,
>(
    config: HotShotConfig<PubKey>,
    stake_table_entries_for_non_voting_nodes: Option<
        Vec<PeerConfig<hotshot_state_prover::QCVerKey>>,
    >,
    instance_state: NodeState,
    networks: Arc<N>,
    metrics: &dyn Metrics,
    node_id: u64,
    state_relay_server: Option<Url>,
    stake_table_commit: StakeTableCommitmentType,
    _: Ver,
    persistence: P,
) -> (SystemContextHandle<SeqTypes, Node<N, P>>, StateSigner<Ver>) {
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

    let quorum_membership = GeneralStaticCommittee::create_election(
        combined_known_nodes_with_stake.clone(),
        combined_known_nodes_with_stake.clone(),
        Topic::Global,
        0,
    );
    let da_membership = GeneralStaticCommittee::create_election(
        combined_known_nodes_with_stake.clone(),
        combined_known_nodes_with_stake,
        Topic::Da,
        0,
    );
    let memberships = Memberships {
        quorum_membership: quorum_membership.clone(),
        da_membership: da_membership.clone(),
        vid_membership: quorum_membership.clone(),
        view_sync_membership: quorum_membership,
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
        HotShotInitializer::from_genesis(instance_state)
            .await
            .unwrap(),
        ConsensusMetricsValue::new(metrics),
        da_storage,
        MarketplaceConfig {
            auction_results_provider: Arc::new(SolverAuctionResultsProvider(
                Url::from_str("https://some.solver").unwrap(),
            )),
            generic_builder_url: Url::from_str("https://some.builder").unwrap(),
        },
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

impl<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, Ver: StaticVersionType + 'static>
    BuilderContext<N, P, Ver>
{
    /// Constructor
    #[allow(clippy::too_many_arguments)]
    pub async fn init(
        hotshot_handle: Arc<Consensus<N, P>>,
        state_signer: StateSigner<Ver>,
        node_index: u64,
        eth_key_pair: EthKeyPair,
        bootstrapped_view: ViewNumber,
        tx_channel_capacity: NonZeroUsize,
        event_channel_capacity: NonZeroUsize,
        instance_state: NodeState,
        validated_state: ValidatedState,
        hotshot_builder_api_url: Url,
        max_api_timeout_duration: Duration,
        buffered_view_num_count: usize,
        maximize_txns_count_timeout_duration: Duration,
        base_fee: FeeAmount,
    ) -> anyhow::Result<Self> {
        // tx channel
        let (mut tx_sender, tx_receiver) =
            broadcast::<Arc<ReceivedTransaction<SeqTypes>>>(tx_channel_capacity.get());
        tx_sender.set_overflow(true);

        // da channel
        let (da_sender, da_receiver) =
            broadcast::<MessageType<SeqTypes>>(event_channel_capacity.get());

        // qc channel
        let (qc_sender, qc_receiver) =
            broadcast::<MessageType<SeqTypes>>(event_channel_capacity.get());

        // decide channel
        let (decide_sender, decide_receiver) =
            broadcast::<MessageType<SeqTypes>>(event_channel_capacity.get());

        // builder api request channel
        let (req_sender, req_receiver) =
            broadcast::<MessageType<SeqTypes>>(event_channel_capacity.get());

        let (genesis_payload, genesis_ns_table) =
            Payload::from_transactions([], &validated_state, &instance_state)
                .await
                .expect("genesis payload construction failed");

        let builder_commitment = genesis_payload.builder_commitment(&genesis_ns_table);

        let vid_commitment = {
            // TODO we should not need to collect payload bytes just to compute vid_commitment
            let payload_bytes = genesis_payload.encode();
            vid_commitment(&payload_bytes, GENESIS_VID_NUM_STORAGE_NODES)
        };

        // create the global state
        let global_state: GlobalState<SeqTypes> = GlobalState::<SeqTypes>::new(
            req_sender,
            tx_sender.clone(),
            vid_commitment,
            bootstrapped_view,
            bootstrapped_view,
            buffered_view_num_count as u64,
        );

        let global_state = Arc::new(RwLock::new(global_state));

        let global_state_clone = global_state.clone();

        let builder_state = BuilderState::<SeqTypes>::new(
            BuiltFromProposedBlock {
                view_number: bootstrapped_view,
                vid_commitment,
                leaf_commit: fake_commitment(),
                builder_commitment,
            },
            decide_receiver,
            da_receiver,
            qc_receiver,
            req_receiver,
            tx_receiver,
            Vec::new() /* tx_queue */,
            global_state_clone,
            NonZeroUsize::new(1).unwrap(),
            maximize_txns_count_timeout_duration,
                              base_fee
                .as_u64()
                .context("the base fee exceeds the maximum amount that a builder can pay (defined by u64::MAX)")?,
            Arc::new(instance_state),
            Duration::from_secs(60),
            Arc::new(validated_state),
        );

        let hotshot_handle_clone = Arc::clone(&hotshot_handle);
        // spawn the builder service
        async_spawn(async move {
            run_permissioned_standalone_builder_service(
                tx_sender,
                da_sender,
                qc_sender,
                decide_sender,
                hotshot_handle_clone,
            )
            .await;
        });

        // spawn the builder event loop
        async_spawn(async move {
            builder_state.event_loop();
        });

        // create the proxy global state it will server the builder apis
        let proxy_global_state = ProxyGlobalState::new(
            global_state.clone(),
            (eth_key_pair.fee_account(), eth_key_pair),
            max_api_timeout_duration,
        );

        // start the builder api service
        run_builder_api_service(hotshot_builder_api_url.clone(), proxy_global_state);

        let ctx = Self {
            hotshot_handle: Arc::clone(&hotshot_handle),
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
    use std::time::Duration;

    use async_compatibility_layer::art::{async_sleep, async_spawn};
    use async_lock::RwLock;
    use async_std::task;

    use espresso_types::{FeeAccount, NamespaceId, Transaction};
    use hotshot_builder_api::v0_1::{
        block_info::{AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo},
        builder::BuildError,
    };
    use hotshot_builder_core::service::{
        run_non_permissioned_standalone_builder_service,
        run_permissioned_standalone_builder_service,
    };
    use hotshot_events_service::{
        events::{Error as EventStreamApiError, Options as EventStreamingApiOptions},
        events_source::{EventConsumer, EventsStreamer},
    };
    use hotshot_types::{
        signature_key::BLSPubKey,
        traits::{
            block_contents::{BlockPayload, GENESIS_VID_NUM_STORAGE_NODES},
            node_implementation::NodeType,
            signature_key::SignatureKey,
        },
    };
    use sequencer::persistence::no_storage::{self, NoStorage};
    use sequencer_utils::test_utils::setup_test;
    use surf_disco::Client;
    use vbs::version::StaticVersion;

    use super::*;
    use crate::{
        non_permissioned,
        testing::{
            hotshot_builder_url, HotShotTestConfig, NonPermissionedBuilderTestConfig,
            PermissionedBuilderTestConfig,
        },
    };

    #[async_std::test]
    async fn test_permissioned_builder() {
        setup_test();

        let ver = StaticVersion::<0, 1>::instance();

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
        let hotshot_context_handle = Arc::clone(&handles[node_id].0);
        let state_signer = handles[node_id].1.take().unwrap();

        // builder api url
        let hotshot_builder_api_url = hotshot_config.config.builder_urls[0].clone();
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
        let builder_client =
            Client::<hotshot_builder_api::v0_1::builder::Error, StaticVersion<0, 1>>::new(
                hotshot_builder_api_url.clone(),
            );
        assert!(builder_client.connect(Some(Duration::from_secs(60))).await);

        let seed = [207_u8; 32];

        // Hotshot client Public, Private key
        let (hotshot_client_pub_key, hotshot_client_private_key) =
            BLSPubKey::generated_from_seed_indexed(seed, 2011_u64);

        let parent_commitment = vid_commitment(&[], GENESIS_VID_NUM_STORAGE_NODES);

        // sign the parent_commitment using the client_private_key
        let encoded_signature = <SeqTypes as NodeType>::SignatureKey::sign(
            &hotshot_client_private_key,
            parent_commitment.as_ref(),
        )
        .expect("Claim block signing failed");

        let test_view_num = 0;
        // test getting available blocks
        tracing::info!(
                "block_info/availableblocks/{parent_commitment}/{test_view_num}/{hotshot_client_pub_key}/{encoded_signature}"
            );
        // sleep and wait for builder service to startup
        async_sleep(Duration::from_millis(3000)).await;
        let available_block_info = match builder_client
            .get::<Vec<AvailableBlockInfo<SeqTypes>>>(&format!(
                "block_info/availableblocks/{parent_commitment}/{test_view_num}/{hotshot_client_pub_key}/{encoded_signature}"
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
                "block_info/claimblock/{builder_commitment}/{test_view_num}/{hotshot_client_pub_key}/{encoded_signature}"
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
                "block_info/claimheaderinput/{builder_commitment}/{test_view_num}/{hotshot_client_pub_key}/{encoded_signature}"
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

        let txn = Transaction::new(NamespaceId::from(1_u32), vec![1, 2, 3]);
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
