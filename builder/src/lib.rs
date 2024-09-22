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
use espresso_types::{
    v0::traits::{PersistenceOptions, SequencerPersistence, StateCatchup},
    SeqTypes,
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
use hotshot_builder_api::v0_1::builder::{
    BuildError, Error as BuilderApiError, Options as HotshotBuilderApiOptions,
};
use hotshot_builder_core::service::{GlobalState, ProxyGlobalState};
use hotshot_orchestrator::{
    client::{OrchestratorClient, ValidatorArgs},
    config::NetworkConfig,
};
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
    L1Params, NetworkParams, Node, SequencerApiVersion,
};
use tide_disco::{app, method::ReadState, App, Url};
use vbs::version::{StaticVersion, StaticVersionType};

pub mod non_permissioned;
pub mod permissioned;

// It runs the api service for the builder
pub fn run_builder_api_service(url: Url, source: ProxyGlobalState<SeqTypes>) {
    // it is to serve hotshot
    let builder_api = hotshot_builder_api::v0_1::builder::define_api::<
        ProxyGlobalState<SeqTypes>,
        SeqTypes,
    >(&HotshotBuilderApiOptions::default())
    .expect("Failed to construct the builder APIs");

    // it enables external clients to submit txn to the builder's private mempool
    let private_mempool_api = hotshot_builder_api::v0_1::builder::submit_api::<
        ProxyGlobalState<SeqTypes>,
        SeqTypes,
        StaticVersion<0, 1>,
    >(&HotshotBuilderApiOptions::default())
    .expect("Failed to construct the builder API for private mempool txns");

    let mut app: App<ProxyGlobalState<SeqTypes>, BuilderApiError> = App::with_state(source);

    app.register_module("block_info", builder_api)
        .expect("Failed to register the builder API");

    app.register_module("txn_submit", private_mempool_api)
        .expect("Failed to register the private mempool API");

    async_spawn(app.serve(url, SequencerApiVersion::instance()));
}

#[cfg(test)]
pub mod testing {
    use core::num;
    use std::{
        collections::HashSet,
        num::NonZeroUsize,
        time::{Duration, Instant},
    };

    //use sequencer::persistence::NoStorage;
    use async_broadcast::{
        broadcast, Receiver as BroadcastReceiver, RecvError, Sender as BroadcastSender,
        TryRecvError,
    };
    use async_compatibility_layer::{
        art::{async_sleep, async_spawn},
        channel::{unbounded, UnboundedReceiver, UnboundedSender},
    };
    use async_lock::RwLock;
    use async_trait::async_trait;
    use committable::Committable;

    use espresso_types::{
        mock::MockStateCatchup, v0_3::ChainConfig, Event, FeeAccount, L1Client, NamespaceId,
        NodeState, PrivKey, PubKey, Transaction, ValidatedState,
    };
    use ethers::{
        types::spoof::State,
        utils::{Anvil, AnvilInstance},
    };
    use futures::{
        future::join_all,
        stream::{Stream, StreamExt},
    };
    use hotshot::{
        traits::{
            implementations::{MasterMap, MemoryNetwork},
            BlockPayload,
        },
        types::{
            EventType::{self, Decide},
            Message,
        },
    };
    use hotshot_builder_api::{
        v0_1::builder::{
            BuildError, Error as BuilderApiError, Options as HotshotBuilderApiOptions,
        },
        v0_2::block_info::{AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo},
    };
    use hotshot_builder_core::{
        builder_state::{BuildBlockInfo, BuilderState, MessageType, ResponseMessage},
        service::GlobalState,
    };
    use hotshot_events_service::{
        events::{Error as EventStreamApiError, Options as EventStreamingApiOptions},
        events_source::{EventConsumer, EventsStreamer},
    };
    use hotshot_types::{
        data::{fake_commitment, Leaf, ViewNumber},
        event::LeafInfo,
        light_client::StateKeyPair,
        traits::{
            block_contents::{vid_commitment, BlockHeader, GENESIS_VID_NUM_STORAGE_NODES},
            metrics::NoMetrics,
            network::Topic,
            node_implementation::{ConsensusTime, Versions},
            signature_key::BuilderSignatureKey as _,
        },
        ExecutionType, HotShotConfig, PeerConfig, ValidatorConfig,
    };
    use portpicker::pick_unused_port;
    use sequencer::{state_signature::StateSignatureMemStorage, SequencerApiVersion};
    use serde::{Deserialize, Serialize};
    use surf_disco::Client;
    use vbs::version::StaticVersion;

    use super::*;
    use crate::{
        non_permissioned::BuilderConfig,
        permissioned::{init_hotshot, BuilderContext},
    };

    #[derive(Clone)]
    pub struct HotShotTestConfig {
        pub config: HotShotConfig<PubKey>,
        priv_keys_staking_nodes: Vec<BLSPrivKey>,
        priv_keys_non_staking_nodes: Vec<BLSPrivKey>,
        staking_nodes_state_key_pairs: Vec<StateKeyPair>,
        non_staking_nodes_state_key_pairs: Vec<StateKeyPair>,
        non_staking_nodes_stake_entries: Vec<PeerConfig<hotshot_state_prover::QCVerKey>>,
        master_map: Arc<MasterMap<PubKey>>,
        anvil: Arc<AnvilInstance>,
    }

    impl Default for HotShotTestConfig {
        fn default() -> Self {
            let num_nodes_with_stake = Self::NUM_STAKED_NODES;
            let num_nodes_without_stake = Self::NUM_NON_STAKED_NODES;

            // first generate stake table entries for the staking nodes
            let (priv_keys_staking_nodes, staking_nodes_state_key_pairs, known_nodes_with_stake) =
                generate_stake_table_entries(num_nodes_with_stake as u64, 1);
            // Now generate the stake table entries for the non-staking nodes
            let (
                priv_keys_non_staking_nodes,
                non_staking_nodes_state_key_pairs,
                known_nodes_without_stake,
            ) = generate_stake_table_entries(num_nodes_without_stake as u64, 0);

            // get the pub key out of the stake table entry for the non-staking nodes
            // Only pass the pub keys to the hotshot config
            let known_nodes_without_stake_pub_keys = known_nodes_without_stake
                .iter()
                .map(|x| <BLSPubKey as SignatureKey>::public_key(&x.stake_table_entry))
                .collect::<Vec<_>>();

            let master_map = MasterMap::new();

            let builder_url = hotshot_builder_url();

            let config: HotShotConfig<PubKey> = HotShotConfig {
                execution_type: ExecutionType::Continuous,
                num_nodes_with_stake: NonZeroUsize::new(num_nodes_with_stake).unwrap(),
                known_da_nodes: known_nodes_with_stake.clone(),
                known_nodes_with_stake: known_nodes_with_stake.clone(),
                known_nodes_without_stake: known_nodes_without_stake_pub_keys,
                next_view_timeout: Duration::from_secs(5).as_millis() as u64,
                timeout_ratio: (10, 11),
                round_start_delay: Duration::from_millis(1).as_millis() as u64,
                start_delay: Duration::from_millis(1).as_millis() as u64,
                num_bootstrap: 1usize,
                da_staked_committee_size: num_nodes_with_stake,
                my_own_validator_config: Default::default(),
                data_request_delay: Duration::from_millis(200),
                view_sync_timeout: Duration::from_secs(5),
                fixed_leader_for_gpuvid: 0,
                builder_urls: vec1::vec1![builder_url],
                builder_timeout: Duration::from_secs(1),
                start_threshold: (
                    known_nodes_with_stake.clone().len() as u64,
                    known_nodes_with_stake.clone().len() as u64,
                ),
                start_proposing_view: 0,
                stop_proposing_view: 0,
                start_voting_view: 0,
                stop_voting_view: 0,
                start_proposing_time: 0,
                start_voting_time: 0,
                stop_proposing_time: 0,
                stop_voting_time: 0,
            };

            Self {
                config,
                priv_keys_staking_nodes,
                priv_keys_non_staking_nodes,
                staking_nodes_state_key_pairs,
                non_staking_nodes_state_key_pairs,
                non_staking_nodes_stake_entries: known_nodes_without_stake,
                master_map,
                anvil: Arc::new(Anvil::new().spawn()),
            }
        }
    }

    pub fn generate_stake_table_entries(
        num_nodes: u64,
        stake_value: u64,
    ) -> (Vec<BLSPrivKey>, Vec<StateKeyPair>, Vec<PeerConfig<PubKey>>) {
        // Generate keys for the nodes.
        let priv_keys = (0..num_nodes)
            .map(|_| PrivKey::generate(&mut rand::thread_rng()))
            .collect::<Vec<_>>();
        let pub_keys = priv_keys
            .iter()
            .map(PubKey::from_private)
            .collect::<Vec<_>>();
        let state_key_pairs = (0..num_nodes)
            .map(|_| StateKeyPair::generate())
            .collect::<Vec<_>>();

        let nodes_with_stake = pub_keys
            .iter()
            .zip(&state_key_pairs)
            .map(|(pub_key, state_key_pair)| PeerConfig::<PubKey> {
                stake_table_entry: pub_key.stake_table_entry(stake_value),
                state_ver_key: state_key_pair.ver_key(),
            })
            .collect::<Vec<_>>();

        (priv_keys, state_key_pairs, nodes_with_stake)
    }

    impl HotShotTestConfig {
        pub const NUM_STAKED_NODES: usize = 4;
        pub const NUM_NON_STAKED_NODES: usize = 2;

        pub fn num_staked_nodes(&self) -> usize {
            self.priv_keys_staking_nodes.len()
        }
        pub fn num_non_staked_nodes(&self) -> usize {
            self.priv_keys_non_staking_nodes.len()
        }
        pub fn num_staking_non_staking_nodes(&self) -> usize {
            self.num_staked_nodes() + self.num_non_staked_nodes()
        }
        pub fn total_nodes() -> usize {
            Self::NUM_STAKED_NODES + Self::NUM_NON_STAKED_NODES
        }
        pub fn get_anvil(&self) -> Arc<AnvilInstance> {
            self.anvil.clone()
        }
        pub fn get_validator_config(
            &self,
            i: usize,
            is_staked: bool,
        ) -> ValidatorConfig<hotshot_state_prover::QCVerKey> {
            if is_staked {
                ValidatorConfig {
                    public_key: self.config.known_nodes_with_stake[i]
                        .stake_table_entry
                        .stake_key,
                    private_key: self.priv_keys_staking_nodes[i].clone(),
                    stake_value: self.config.known_nodes_with_stake[i]
                        .stake_table_entry
                        .stake_amount
                        .as_u64(),
                    state_key_pair: self.staking_nodes_state_key_pairs[i].clone(),
                    is_da: true,
                }
            } else {
                ValidatorConfig {
                    public_key: self.config.known_nodes_without_stake[i],
                    private_key: self.priv_keys_non_staking_nodes[i].clone(),
                    stake_value: 0,
                    state_key_pair: self.non_staking_nodes_state_key_pairs[i].clone(),
                    is_da: true,
                }
            }
        }

        pub async fn init_nodes<P: SequencerPersistence, V: Versions>(
            &self,
            bind_version: V,
            options: impl PersistenceOptions<Persistence = P>,
        ) -> Vec<(
            Arc<Consensus<network::Memory, P, V>>,
            Option<StateSigner<SequencerApiVersion>>,
        )> {
            let num_staked_nodes = self.num_staked_nodes();
            let mut is_staked = false;
            let stake_table_commit = static_stake_table_commitment(
                &self.config.known_nodes_with_stake,
                Self::total_nodes(),
            );

            join_all((0..self.num_staking_non_staking_nodes()).map(|i| {
                is_staked = i < num_staked_nodes;
                let options = options.clone();
                async move {
                    let persistence = options.create().await.unwrap();
                    let (hotshot_handle, state_signer) = self
                        .init_node(
                            i,
                            is_staked,
                            stake_table_commit,
                            &NoMetrics,
                            bind_version,
                            persistence,
                        )
                        .await;
                    // wrapped in some because need to take later
                    (Arc::new(hotshot_handle), Some(state_signer))
                }
            }))
            .await
        }

        pub async fn init_node<P: SequencerPersistence, V: Versions>(
            &self,
            i: usize,
            is_staked: bool,
            stake_table_commit: StakeTableCommitmentType,
            metrics: &dyn Metrics,
            bind_version: V,
            persistence: P,
        ) -> (
            Consensus<network::Memory, P, V>,
            StateSigner<SequencerApiVersion>,
        ) {
            let mut config = self.config.clone();

            let num_staked_nodes = self.num_staked_nodes();
            if is_staked {
                config.my_own_validator_config = self.get_validator_config(i, is_staked);
            } else {
                config.my_own_validator_config =
                    self.get_validator_config(i - num_staked_nodes, is_staked);
            }

            let network = Arc::new(MemoryNetwork::new(
                &config.my_own_validator_config.public_key,
                &self.master_map,
                &[Topic::Global, Topic::Da],
                None,
            ));

            let node_state = NodeState::new(
                i as u64,
                ChainConfig::default(),
                L1Client::new(self.anvil.endpoint().parse().unwrap(), 1),
                MockStateCatchup::default(),
                V::Base::VERSION,
            )
            .with_genesis(ValidatedState::default());

            tracing::info!("Before init hotshot");
            let handle = init_hotshot(
                config,
                Some(self.non_staking_nodes_stake_entries.clone()),
                node_state,
                network,
                metrics,
                i as u64,
                None,
                stake_table_commit,
                bind_version,
                persistence,
            )
            .await;

            tracing::info!("After init hotshot");
            handle
        }

        // url for the hotshot event streaming api
        pub fn hotshot_event_streaming_api_url() -> Url {
            // spawn the event streaming api
            let port = portpicker::pick_unused_port()
                .expect("Could not find an open port for hotshot event streaming api");

            let hotshot_events_streaming_api_url =
                Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

            hotshot_events_streaming_api_url
        }

        // start the server for the hotshot event streaming api
        pub fn run_hotshot_event_streaming_api(
            url: Url,
            source: Arc<RwLock<EventsStreamer<SeqTypes>>>,
        ) {
            // Start the web server.
            let hotshot_events_api = hotshot_events_service::events::define_api::<
                Arc<RwLock<EventsStreamer<SeqTypes>>>,
                SeqTypes,
                StaticVersion<0, 1>,
            >(&EventStreamingApiOptions::default())
            .expect("Failed to define hotshot eventsAPI");

            let mut app = App::<_, EventStreamApiError>::with_state(source);

            app.register_module("hotshot-events", hotshot_events_api)
                .expect("Failed to register hotshot events API");

            async_spawn(app.serve(url, SequencerApiVersion::instance()));
        }
        // enable hotshot event streaming
        pub fn enable_hotshot_node_event_streaming<P: SequencerPersistence, V: Versions>(
            hotshot_events_api_url: Url,
            known_nodes_with_stake: Vec<PeerConfig<VerKey>>,
            num_non_staking_nodes: usize,
            hotshot_context_handle: Arc<Consensus<network::Memory, P, V>>,
        ) {
            // create a event streamer
            let events_streamer = Arc::new(RwLock::new(EventsStreamer::new(
                known_nodes_with_stake,
                num_non_staking_nodes,
            )));

            // serve the hotshot event streaming api with events_streamer state
            Self::run_hotshot_event_streaming_api(hotshot_events_api_url, events_streamer.clone());

            // send the events to the event streaming state
            async_spawn({
                async move {
                    let mut hotshot_event_stream = hotshot_context_handle.event_stream();
                    loop {
                        let event = hotshot_event_stream.next().await.unwrap();
                        tracing::debug!("Before writing in event streamer: {event:?}");
                        events_streamer.write().await.handle_event(event).await;
                        tracing::debug!("Event written to the event streamer");
                    }
                }
            });
        }
    }

    // Wait for decide event, make sure it matches submitted transaction. Return the block number
    // containing the transaction.
    pub async fn wait_for_decide_on_handle(
        events: &mut (impl Stream<Item = Event> + Unpin),
        submitted_txn: &Transaction,
    ) -> u64 {
        let commitment = submitted_txn.commit();

        // Keep getting events until we see a Decide event
        loop {
            let event = events.next().await.unwrap();
            tracing::info!("Received event from handle: {event:?}");

            if let Decide { leaf_chain, .. } = event.event {
                if let Some(height) = leaf_chain.iter().find_map(|LeafInfo { leaf, .. }| {
                    if leaf
                        .block_payload()
                        .as_ref()?
                        .transaction_commitments(leaf.block_header().metadata())
                        .contains(&commitment)
                    {
                        Some(leaf.block_header().block_number())
                    } else {
                        None
                    }
                }) {
                    return height;
                }
            } else {
                // Keep waiting
            }
        }
    }

    pub struct NonPermissionedBuilderTestConfig {
        pub config: BuilderConfig,
        pub fee_account: FeeAccount,
    }

    impl NonPermissionedBuilderTestConfig {
        pub const SUBSCRIBED_DA_NODE_ID: usize = 5;

        pub async fn init_non_permissioned_builder<V: Versions>(
            hotshot_events_streaming_api_url: Url,
            hotshot_builder_api_url: Url,
            num_nodes: usize,
        ) -> Self {
            // generate builder keys
            let seed = [201_u8; 32];
            let (fee_account, key_pair) = FeeAccount::generated_from_seed_indexed(seed, 2011_u64);

            // channel capacity for the builder states
            let tx_channel_capacity = NonZeroUsize::new(500).unwrap();
            let event_channel_capacity = NonZeroUsize::new(20).unwrap();
            // bootstrapping view number
            // A new builder can use this view number to start building blocks from this view number
            let bootstrapped_view = ViewNumber::new(0);

            let node_count = NonZeroUsize::new(num_nodes).unwrap();

            let builder_config = BuilderConfig::init(
                key_pair,
                bootstrapped_view,
                tx_channel_capacity,
                event_channel_capacity,
                node_count,
                NodeState::default().with_current_version(V::Base::VERSION),
                ValidatedState::default(),
                hotshot_events_streaming_api_url,
                hotshot_builder_api_url,
                Duration::from_millis(2000),
                15,
                Duration::from_millis(500),
                ChainConfig::default().base_fee,
            )
            .await
            .unwrap();

            Self {
                config: builder_config,
                fee_account,
            }
        }
    }

    pub struct PermissionedBuilderTestConfig<P: SequencerPersistence, V: Versions> {
        pub builder_context: BuilderContext<network::Memory, P, V>,
        pub fee_account: FeeAccount,
    }

    impl<P: SequencerPersistence, V: Versions> PermissionedBuilderTestConfig<P, V> {
        pub async fn init_permissioned_builder(
            hotshot_handle: Arc<Consensus<network::Memory, P, V>>,
            node_id: u64,
            state_signer: Arc<StateSigner<SequencerApiVersion>>,
            hotshot_builder_api_url: Url,
        ) -> Self {
            // setup the instance state
            let node_state = NodeState::default().with_current_version(V::Base::VERSION);

            // generate builder keys
            let seed = [201_u8; 32];
            let (fee_account, key_pair) = FeeAccount::generated_from_seed_indexed(seed, 2011_u64);

            // channel capacity for the builder states
            let tx_channel_capacity = NonZeroUsize::new(20).unwrap();
            let event_channel_capacity = NonZeroUsize::new(500).unwrap();
            // bootstrapping view number
            // A new builder can use this view number to start building blocks from this view number
            let bootstrapped_view = ViewNumber::new(0);

            let builder_context = BuilderContext::init(
                Arc::clone(&hotshot_handle),
                state_signer,
                node_id,
                key_pair,
                bootstrapped_view,
                tx_channel_capacity,
                event_channel_capacity,
                node_state,
                ValidatedState::default(),
                hotshot_builder_api_url,
                Duration::from_millis(2000),
                15,
                Duration::from_millis(500),
                ChainConfig::default().base_fee,
            )
            .await
            .unwrap();

            Self {
                builder_context,
                fee_account,
            }
        }
    }

    pub fn hotshot_builder_url() -> Url {
        // spawn the builder api
        let port =
            portpicker::pick_unused_port().expect("Could not find an open port for builder api");

        let hotshot_builder_api_url =
            Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

        hotshot_builder_api_url
    }

    pub async fn test_builder_impl(
        hotshot_builder_api_url: Url,
        mut subscribed_events: impl Stream<Item = Event> + Unpin,
        builder_pub_key: FeeAccount,
    ) {
        // Start a builder api client
        let builder_client =
            Client::<hotshot_builder_api::v0_1::builder::Error, StaticVersion<0, 1>>::new(
                hotshot_builder_api_url.clone(),
            );
        assert!(builder_client.connect(Some(Duration::from_secs(60))).await);

        // Test submitting transactions
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
            }
            Err(e) => {
                panic!("Error submitting private transaction {:?}", e);
            }
        }

        let seed = [207_u8; 32];

        // Hotshot client Public, Private key
        let (hotshot_client_pub_key, hotshot_client_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed(seed, 2011_u64);

        let start = Instant::now();
        let (available_block_info, view_num) = loop {
            if start.elapsed() > Duration::from_secs(10) {
                panic!("Didn't get a quorum proposal in 10 seconds");
            }

            let event = subscribed_events.next().await.unwrap();
            tracing::warn!("Event: {:?}", event.event);
            if let EventType::QuorumProposal { proposal, .. } = event.event {
                let parent_view_number = *proposal.data.view_number;
                let parent_commitment =
                    Leaf::from_quorum_proposal(&proposal.data).payload_commitment();
                let encoded_signature = <SeqTypes as NodeType>::SignatureKey::sign(
                    &hotshot_client_private_key,
                    parent_commitment.as_ref(),
                )
                .expect("Claim block signing failed");
                let available_blocks = builder_client
                        .get::<Vec<AvailableBlockInfo<SeqTypes>>>(&format!(
                            "block_info/availableblocks/{parent_commitment}/{parent_view_number}/{hotshot_client_pub_key}/{encoded_signature}"
                        ))
                        .send()
                        .await.expect("Error getting available blocks");
                assert!(!available_blocks.is_empty());
                break (available_blocks, parent_view_number);
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
        let available_block_data = match builder_client
                .get::<AvailableBlockData<SeqTypes>>(&format!(
                    "block_info/claimblock/{builder_commitment}/{view_num}/{hotshot_client_pub_key}/{encoded_signature}"
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

        assert_eq!(
            available_block_data
                .block_payload
                .transactions(&available_block_data.metadata)
                .collect::<Vec<_>>(),
            vec![txn]
        );

        // Test claiming block header input
        let _available_block_header = match builder_client
                .get::<AvailableBlockHeaderInput<SeqTypes>>(&format!(
                    "block_info/claimheaderinput/{builder_commitment}/{view_num}/{hotshot_client_pub_key}/{encoded_signature}"
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
    }
}

#[cfg(test)]
mod test {
    use async_std::stream::IntoStream;
    use clap::builder;
    use espresso_types::{Header, MockSequencerVersions, NodeState, Payload, ValidatedState};
    use ethers::providers::Quorum;
    use futures::StreamExt;
    use hotshot::types::EventType::Decide;
    use hotshot_builder_api::v0_1::block_info::AvailableBlockData;
    use hotshot_builder_core::service::GlobalState;
    use sequencer::{
        empty_builder_commitment,
        persistence::{
            no_storage::{self, NoStorage},
            sql,
        },
    };
    use sequencer_utils::test_utils::setup_test;
    use testing::{wait_for_decide_on_handle, HotShotTestConfig};
    use vbs::version::StaticVersion;

    use super::*;

    // Test that a non-voting hotshot node can participate in consensus and reach a certain height.
    // It is enabled by keeping the node(s) in the stake table, but with a stake of 0.
    // This is useful for testing that the builder(permissioned node) can participate in consensus without voting.
    #[ignore]
    #[async_std::test]
    async fn test_non_voting_hotshot_node() {
        setup_test();

        let success_height = 5;
        // Assign `config` so it isn't dropped early.
        let config = HotShotTestConfig::default();
        tracing::debug!("Done with hotshot test config");
        let handles = config
            .init_nodes(MockSequencerVersions::new(), no_storage::Options)
            .await;
        tracing::debug!("Done with init nodes");
        let total_nodes = HotShotTestConfig::total_nodes();

        // try to listen on non-voting node handle as it is the last handle
        let mut events = handles[total_nodes - 1].0.event_stream();
        for (handle, ..) in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        let genesis_state = NodeState::mock();
        let validated_state = ValidatedState::default();
        let mut parent = {
            // TODO refactor repeated code from other tests
            let (genesis_payload, genesis_ns_table) =
                Payload::from_transactions([], &validated_state, &genesis_state)
                    .await
                    .expect("unable to create genesis payload");
            let builder_commitment = genesis_payload.builder_commitment(&genesis_ns_table);
            let genesis_commitment = {
                // TODO we should not need to collect payload bytes just to compute vid_commitment
                let payload_bytes = genesis_payload.encode();
                vid_commitment(&payload_bytes, GENESIS_VID_NUM_STORAGE_NODES)
            };
            Header::genesis(
                &genesis_state,
                genesis_commitment,
                builder_commitment,
                genesis_ns_table,
            )
        };

        loop {
            let event = events.next().await.unwrap();
            tracing::debug!("Received event from handle: {event:?}");
            let Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            tracing::info!("Got decide {leaf_chain:?}");

            // Check that each successive header satisfies invariants relative to its parent: all
            // the fields which should be monotonic are.
            for LeafInfo { leaf, .. } in leaf_chain.iter().rev() {
                let header = leaf.block_header().clone();
                if header.height() == 0 {
                    parent = header;
                    continue;
                }
                assert_eq!(header.height(), parent.height() + 1);
                assert!(header.timestamp() >= parent.timestamp());
                assert!(header.l1_head() >= parent.l1_head());
                assert!(header.l1_finalized() >= parent.l1_finalized());
                parent = header;
            }

            if parent.height() >= success_height {
                break;
            }
        }
    }
}
