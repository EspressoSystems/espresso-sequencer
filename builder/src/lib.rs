#![allow(unused_imports)]
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
        implementations::{NetworkingMetricsValue, WebServerNetwork},
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
    constants::{Version01, STATIC_VER_0_1},
    event::Event,
    light_client::StateKeyPair,
    signature_key::{BLSPrivKey, BLSPubKey},
    traits::election::Membership,
    traits::metrics::Metrics,
    HotShotConfig, PeerConfig, ValidatorConfig,
};
use std::fmt::Display;
// Should move `STAKE_TABLE_CAPACITY` in the sequencer repo when we have variate stake table support
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;

use async_std::sync::{Arc, RwLock};
use async_std::task::{spawn, JoinHandle};

use async_compatibility_layer::art::{async_sleep, async_spawn};
use hotshot_builder_api::builder::{
    BuildError, Error as BuilderApiError, Options as HotshotBuilderApiOptions,
};
use hotshot_builder_core::service::{GlobalState, ProxyGlobalState};
use jf_primitives::{
    merkle_tree::{namespaced_merkle_tree::NamespacedMerkleTreeScheme, MerkleTreeScheme},
    signatures::bls_over_bn254::VerKey,
};
use sequencer::catchup::mock::MockStateCatchup;
use sequencer::state_signature::StakeTableCommitmentType;
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

pub mod non_permissioned;
pub mod permissioned;

// It runs the api service for the builder
pub fn run_builder_api_service(url: Url, source: Arc<RwLock<ProxyGlobalState<SeqTypes>>>) {
    // it is to serve hotshot
    let builder_api = hotshot_builder_api::builder::define_api::<
        Arc<RwLock<ProxyGlobalState<SeqTypes>>>,
        SeqTypes,
        Version01,
    >(&HotshotBuilderApiOptions::default())
    .expect("Failed to construct the builder APIs");

    // it enables external clients to submit txn to the builder's private mempool
    let private_mempool_api = hotshot_builder_api::builder::submit_api::<
        Arc<RwLock<ProxyGlobalState<SeqTypes>>>,
        SeqTypes,
        Version01,
    >(&HotshotBuilderApiOptions::default())
    .expect("Failed to construct the builder API for private mempool txns");

    let mut app: App<Arc<RwLock<ProxyGlobalState<SeqTypes>>>, BuilderApiError> =
        App::with_state(source);

    app.register_module("block_info", builder_api)
        .expect("Failed to register the builder API");

    app.register_module("txn_submit", private_mempool_api)
        .expect("Failed to register the private mempool API");

    async_spawn(app.serve(url, STATIC_VER_0_1));
}

#[cfg(test)]
pub mod testing {
    use super::*;
    use committable::Committable;
    use core::num;
    use ethers::{
        types::spoof::State,
        utils::{Anvil, AnvilInstance},
    };
    use futures::{
        future::join_all,
        stream::{Stream, StreamExt},
    };
    use hotshot::traits::{
        implementations::{MasterMap, MemoryNetwork},
        BlockPayload,
    };
    use hotshot::types::{EventType::Decide, Message};
    use hotshot_types::{
        light_client::StateKeyPair,
        traits::{
            block_contents::BlockHeader, metrics::NoMetrics,
            signature_key::BuilderSignatureKey as _,
        },
        ExecutionType, HotShotConfig, PeerConfig, ValidatorConfig,
    };
    use portpicker::pick_unused_port;
    //use sequencer::persistence::NoStorage;
    use async_broadcast::{
        broadcast, Receiver as BroadcastReceiver, RecvError, Sender as BroadcastSender,
        TryRecvError,
    };
    use async_compatibility_layer::channel::unbounded;
    use async_compatibility_layer::{
        art::{async_sleep, async_spawn},
        channel::{UnboundedReceiver, UnboundedSender},
    };
    use async_lock::RwLock;
    use hotshot_builder_core::{
        builder_state::{BuildBlockInfo, BuilderState, MessageType, ResponseMessage},
        service::GlobalState,
    };
    use hotshot_types::event::LeafInfo;
    use hotshot_types::{
        data::{fake_commitment, Leaf, ViewNumber},
        traits::{
            block_contents::{vid_commitment, GENESIS_VID_NUM_STORAGE_NODES},
            node_implementation::ConsensusTime,
        },
    };
    use sequencer::{
        catchup::StateCatchup, eth_signature_key::EthKeyPair, persistence::PersistenceOptions,
        state_signature::StateSignatureMemStorage, ChainConfig,
    };
    use sequencer::{Event, Transaction};
    use std::{num::NonZeroUsize, time::Duration};

    use crate::non_permissioned::BuilderConfig;
    use crate::permissioned::{init_hotshot, BuilderContext};
    use async_trait::async_trait;
    use hotshot_builder_api::builder::Options as HotshotBuilderApiOptions;
    use hotshot_builder_api::builder::{BuildError, Error as BuilderApiError};
    use hotshot_events_service::{
        events::{Error as EventStreamApiError, Options as EventStreamingApiOptions},
        events_source::{EventConsumer, EventsStreamer},
    };
    use hotshot_types::constants::{Version01, STATIC_VER_0_1};
    use serde::{Deserialize, Serialize};
    type ElectionConfig = StaticElectionConfig;
    use snafu::{guide::feature_flags, *};

    #[derive(Clone)]
    pub struct HotShotTestConfig {
        pub config: HotShotConfig<PubKey, ElectionConfig>,
        priv_keys_staking_nodes: Vec<BLSPrivKey>,
        priv_keys_non_staking_nodes: Vec<BLSPrivKey>,
        staking_nodes_state_key_pairs: Vec<StateKeyPair>,
        non_staking_nodes_state_key_pairs: Vec<StateKeyPair>,
        non_staking_nodes_stake_entries: Vec<PeerConfig<hotshot_state_prover::QCVerKey>>,
        master_map: Arc<MasterMap<Message<SeqTypes>, PubKey>>,
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
                .map(|x| <BLSPubKey as SignatureKey>::get_public_key(&x.stake_table_entry))
                .collect::<Vec<_>>();

            let master_map = MasterMap::new();

            let builder_url = hotshot_builder_url();

            let config: HotShotConfig<PubKey, ElectionConfig> = HotShotConfig {
                execution_type: ExecutionType::Continuous,
                num_nodes_with_stake: NonZeroUsize::new(num_nodes_with_stake).unwrap(),
                num_nodes_without_stake,
                min_transactions: 1,
                max_transactions: 10000.try_into().unwrap(),
                known_nodes_with_stake,
                known_nodes_without_stake: known_nodes_without_stake_pub_keys,
                next_view_timeout: Duration::from_secs(5).as_millis() as u64,
                timeout_ratio: (10, 11),
                round_start_delay: Duration::from_millis(1).as_millis() as u64,
                start_delay: Duration::from_millis(1).as_millis() as u64,
                num_bootstrap: 1usize,
                propose_min_round_time: Duration::from_secs(0),
                propose_max_round_time: Duration::from_secs(1),
                election_config: None,
                da_staked_committee_size: num_nodes_with_stake,
                da_non_staked_committee_size: num_nodes_without_stake,
                my_own_validator_config: Default::default(),
                data_request_delay: Duration::from_millis(200),
                view_sync_timeout: Duration::from_secs(5),
                fixed_leader_for_gpuvid: 0,
                builder_url,
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
                stake_table_entry: pub_key.get_stake_table_entry(stake_value),
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
                }
            } else {
                ValidatorConfig {
                    public_key: self.config.known_nodes_without_stake[i],
                    private_key: self.priv_keys_non_staking_nodes[i].clone(),
                    stake_value: 0,
                    state_key_pair: self.non_staking_nodes_state_key_pairs[i].clone(),
                }
            }
        }

        pub async fn init_nodes<P: SequencerPersistence, Ver: StaticVersionType + 'static>(
            &self,
            bind_version: Ver,
            options: impl PersistenceOptions<Persistence = P>,
        ) -> Vec<(
            SystemContextHandle<SeqTypes, Node<network::Memory, P>>,
            Option<StateSigner<Ver>>,
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
                    (hotshot_handle, Some(state_signer))
                }
            }))
            .await
        }

        pub async fn init_node<P: SequencerPersistence, Ver: StaticVersionType + 'static>(
            &self,
            i: usize,
            is_staked: bool,
            stake_table_commit: StakeTableCommitmentType,
            metrics: &dyn Metrics,
            bind_version: Ver,
            persistence: P,
        ) -> (
            SystemContextHandle<SeqTypes, Node<network::Memory, P>>,
            StateSigner<Ver>,
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
                config.my_own_validator_config.public_key,
                NetworkingMetricsValue::new(metrics),
                self.master_map.clone(),
                None,
            ));
            let networks = Networks {
                da_network: network.clone(),
                quorum_network: network,
                _pd: Default::default(),
            };

            let key = Self::builder_key(i);
            tracing::info!("node {i} is builder {:x}", key.address());
            let node_state = NodeState::new(
                ChainConfig::default(),
                L1Client::new(self.anvil.endpoint().parse().unwrap(), Address::default()),
                key,
                MockStateCatchup::default(),
            )
            .with_genesis(ValidatedState::default());

            tracing::info!("Before init hotshot");
            let handle = init_hotshot(
                config,
                Some(self.non_staking_nodes_stake_entries.clone()),
                node_state,
                networks,
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

        pub fn builder_key(i: usize) -> EthKeyPair {
            EthKeyPair::from_mnemonic(
                "test test test test test test test test test test test junk",
                i as u32,
            )
            .unwrap()
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
                Version01,
            >(&EventStreamingApiOptions::default())
            .expect("Failed to define hotshot eventsAPI");

            let mut app = App::<_, EventStreamApiError>::with_state(source);

            app.register_module("hotshot-events", hotshot_events_api)
                .expect("Failed to register hotshot events API");

            async_spawn(app.serve(url, STATIC_VER_0_1));
        }
        // enable hotshot event streaming
        pub fn enable_hotshot_node_event_streaming<P: SequencerPersistence>(
            hotshot_events_api_url: Url,
            known_nodes_with_stake: Vec<PeerConfig<VerKey>>,
            num_non_staking_nodes: usize,
            hotshot_context_handle: SystemContextHandle<SeqTypes, Node<network::Memory, P>>,
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
                    let mut hotshot_event_stream = hotshot_context_handle.get_event_stream();
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
                        .get_block_payload()
                        .as_ref()?
                        .transaction_commitments(leaf.get_block_header().metadata())
                        .contains(&commitment)
                    {
                        Some(leaf.get_block_header().block_number())
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

        pub async fn init_non_permissioned_builder(
            hotshot_test_config: &HotShotTestConfig,
            hotshot_events_streaming_api_url: Url,
            hotshot_builder_api_url: Url,
        ) -> Self {
            // setup the instance state
            let key = HotShotTestConfig::builder_key(Self::SUBSCRIBED_DA_NODE_ID);
            tracing::info!(
                "node {} is builder {:x}",
                Self::SUBSCRIBED_DA_NODE_ID,
                key.address()
            );
            let node_state = NodeState::new(
                ChainConfig::default(),
                L1Client::new(
                    hotshot_test_config.get_anvil().endpoint().parse().unwrap(),
                    Address::default(),
                ),
                key,
                MockStateCatchup::default(),
            )
            .with_genesis(ValidatedState::default());

            // generate builder keys
            let seed = [201_u8; 32];
            let (fee_account, key_pair) = FeeAccount::generated_from_seed_indexed(seed, 2011_u64);

            // channel capacity for the builder states
            let channel_capacity = NonZeroUsize::new(100).unwrap();
            // bootstrapping view number
            // A new builder can use this view number to start building blocks from this view number
            let bootstrapped_view = ViewNumber::new(0);

            let builder_config = BuilderConfig::init(
                key_pair,
                bootstrapped_view,
                channel_capacity,
                node_state,
                hotshot_events_streaming_api_url,
                hotshot_builder_api_url,
                Duration::from_millis(1000),
                15,
            )
            .await
            .unwrap();

            Self {
                config: builder_config,
                fee_account,
            }
        }
    }

    pub struct PermissionedBuilderTestConfig<
        P: SequencerPersistence,
        Ver: StaticVersionType + 'static,
    > {
        pub builder_context: BuilderContext<network::Memory, P, Ver>,
        pub fee_account: FeeAccount,
    }

    impl<P: SequencerPersistence, Ver: StaticVersionType + 'static>
        PermissionedBuilderTestConfig<P, Ver>
    {
        pub async fn init_permissioned_builder(
            hotshot_test_config: HotShotTestConfig,
            hotshot_handle: SystemContextHandle<SeqTypes, Node<network::Memory, P>>,
            node_id: u64,
            state_signer: StateSigner<Ver>,
            hotshot_builder_api_url: Url,
        ) -> Self {
            // setup the instance state
            let key = HotShotTestConfig::builder_key(HotShotTestConfig::NUM_STAKED_NODES);
            tracing::info!(
                "node {} is builder {:x}",
                HotShotTestConfig::NUM_STAKED_NODES,
                key.address()
            );
            let node_state = NodeState::new(
                ChainConfig::default(),
                L1Client::new(
                    hotshot_test_config.get_anvil().endpoint().parse().unwrap(),
                    Address::default(),
                ),
                key,
                MockStateCatchup::default(),
            )
            .with_genesis(ValidatedState::default());

            // generate builder keys
            let seed = [201_u8; 32];
            let (fee_account, key_pair) = FeeAccount::generated_from_seed_indexed(seed, 2011_u64);

            // channel capacity for the builder states
            let channel_capacity = NonZeroUsize::new(100).unwrap();
            // bootstrapping view number
            // A new builder can use this view number to start building blocks from this view number
            let bootstrapped_view = ViewNumber::new(0);

            let builder_context = BuilderContext::init(
                hotshot_handle,
                state_signer,
                node_id,
                key_pair,
                bootstrapped_view,
                channel_capacity,
                node_state,
                hotshot_builder_api_url,
                Duration::from_millis(1000),
                15,
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
}

#[cfg(test)]
mod test {
    //use self::testing::mock_node_state;

    use super::*;
    //use super::{transaction::ApplicationTransaction, vm::TestVm, *};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};

    use async_std::stream::IntoStream;
    use clap::builder;
    use ethers::providers::Quorum;
    use futures::StreamExt;
    use hotshot::types::EventType::Decide;

    use hotshot_builder_api::block_info::AvailableBlockData;
    use hotshot_builder_core::service::GlobalState;
    use hotshot_types::event::LeafInfo;
    use hotshot_types::traits::block_contents::{
        vid_commitment, BlockHeader, BlockPayload, GENESIS_VID_NUM_STORAGE_NODES,
    };
    use hotshot_types::utils::BuilderCommitment;
    use sequencer::block::payload::Payload;
    use sequencer::persistence::no_storage::{self, NoStorage};
    use sequencer::persistence::sql;
    use sequencer::{empty_builder_commitment, Header};
    use testing::{wait_for_decide_on_handle, HotShotTestConfig};

    use es_version::SequencerVersion;

    // Test that a non-voting hotshot node can participate in consensus and reach a certain height.
    // It is enabled by keeping the node(s) in the stake table, but with a stake of 0.
    // This is useful for testing that the builder(permissioned node) can participate in consensus without voting.
    #[ignore]
    #[async_std::test]
    async fn test_non_voting_hotshot_node() {
        setup_logging();
        setup_backtrace();

        let ver = SequencerVersion::instance();

        let success_height = 5;
        // Assign `config` so it isn't dropped early.
        let config = HotShotTestConfig::default();
        tracing::debug!("Done with hotshot test config");
        let handles = config.init_nodes(ver, no_storage::Options).await;
        tracing::debug!("Done with init nodes");
        let total_nodes = HotShotTestConfig::total_nodes();

        // try to listen on non-voting node handle as it is the last handle
        let mut events = handles[total_nodes - 1].0.get_event_stream();
        for (handle, ..) in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        let mut parent = {
            // TODO refactor repeated code from other tests
            let (genesis_payload, genesis_ns_table) = Payload::genesis();
            let builder_commitment = genesis_payload.builder_commitment(&genesis_ns_table);
            let genesis_commitment = {
                // TODO we should not need to collect payload bytes just to compute vid_commitment
                let payload_bytes = genesis_payload
                    .encode()
                    .expect("unable to encode genesis payload")
                    .collect();
                vid_commitment(&payload_bytes, GENESIS_VID_NUM_STORAGE_NODES)
            };
            let genesis_state = NodeState::mock();
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
                let header = leaf.get_block_header().clone();
                if header.height == 0 {
                    parent = header;
                    continue;
                }
                assert_eq!(header.height, parent.height + 1);
                assert!(header.timestamp >= parent.timestamp);
                assert!(header.l1_head >= parent.l1_head);
                assert!(header.l1_finalized >= parent.l1_finalized);
                parent = header;
            }

            if parent.height >= success_height {
                break;
            }
        }
    }
}
