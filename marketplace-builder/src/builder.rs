use std::{num::NonZeroUsize, time::Duration};

use anyhow::Context;
use async_broadcast::{
    broadcast, Receiver as BroadcastReceiver, RecvError, Sender as BroadcastSender, TryRecvError,
};
use async_compatibility_layer::{
    art::{async_sleep, async_spawn},
    channel::{unbounded, UnboundedReceiver, UnboundedSender},
};
use async_lock::RwLock;
use async_std::sync::Arc;
use espresso_types::{
    eth_signature_key::EthKeyPair, v0_3::ChainConfig, FeeAmount, L1Client, NamespaceId, NodeState,
    Payload, SeqTypes, ValidatedState,
};
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{coins_bip39::English, MnemonicBuilder, Signer as _, Wallet},
    types::{Address, U256},
};
use hotshot::traits::BlockPayload;
use hotshot_builder_api::v0_3::builder::{
    BuildError, Error as BuilderApiError, Options as HotshotBuilderApiOptions,
};
use hotshot_events_service::{
    events::{Error as EventStreamApiError, Options as EventStreamingApiOptions},
    events_source::{EventConsumer, EventsStreamer},
};
use hotshot_types::{
    data::{fake_commitment, Leaf, ViewNumber},
    traits::{
        block_contents::{vid_commitment, GENESIS_VID_NUM_STORAGE_NODES},
        node_implementation::{ConsensusTime, NodeType},
        EncodeBytes,
    },
    utils::BuilderCommitment,
};
use marketplace_builder_core::{
    builder_state::{
        BuildBlockInfo, BuilderState, BuiltFromProposedBlock, MessageType, ResponseMessage,
    },
    service::{
        run_non_permissioned_standalone_builder_service, BroadcastSenders, GlobalState,
        ProxyGlobalState, ReceivedTransaction,
    },
};
use sequencer::{catchup::StatePeers, L1Params, NetworkParams};
use surf::http::headers::ACCEPT;
use surf_disco::Client;
use tide_disco::{app, method::ReadState, App, Url};
use vbs::version::StaticVersionType;

use crate::{
    hooks::{self, BidConfig, EspressoFallbackHooks, EspressoReserveHooks},
    run_builder_api_service,
};

#[derive(Clone, Debug)]
pub struct BuilderConfig {
    pub global_state: Arc<RwLock<GlobalState<SeqTypes>>>,
    pub hotshot_events_api_url: Url,
    pub hotshot_builder_apis_url: Url,
}

pub fn build_instance_state<Ver: StaticVersionType + 'static>(
    chain_config: ChainConfig,
    l1_params: L1Params,
    state_peers: Vec<Url>,
    _: Ver,
) -> anyhow::Result<NodeState> {
    let l1_client = L1Client::new(l1_params.url, l1_params.events_max_block_range);
    let instance_state = NodeState::new(
        u64::MAX, // dummy node ID, only used for debugging
        chain_config,
        l1_client,
        Arc::new(StatePeers::<Ver>::from_urls(
            state_peers,
            Default::default(),
        )),
    );
    Ok(instance_state)
}

impl BuilderConfig {
    #[allow(clippy::too_many_arguments)]
    pub async fn init(
        is_reserve: bool,
        builder_key_pair: EthKeyPair,
        bootstrapped_view: ViewNumber,
        tx_channel_capacity: NonZeroUsize,
        event_channel_capacity: NonZeroUsize,
        node_count: NonZeroUsize,
        instance_state: NodeState,
        validated_state: ValidatedState,
        hotshot_events_api_url: Url,
        hotshot_builder_apis_url: Url,
        max_api_timeout_duration: Duration,
        buffered_view_num_count: usize,
        maximize_txns_count_timeout_duration: Duration,
        base_fee: FeeAmount,
        bid_config: Option<BidConfig>,
        solver_api_url: Url,
    ) -> anyhow::Result<Self> {
        println!("here start init");
        tracing::info!(
            address = %builder_key_pair.fee_account(),
            ?bootstrapped_view,
            %tx_channel_capacity,
            %event_channel_capacity,
            ?max_api_timeout_duration,
            buffered_view_num_count,
            ?maximize_txns_count_timeout_duration,
            "initializing builder",
        );

        let (mut senders, receivers) =
            marketplace_builder_core::service::broadcast_channels(event_channel_capacity.get());

        senders.transactions.set_capacity(tx_channel_capacity.get());

        // builder api request channel
        let (req_sender, req_receiver) =
            broadcast::<MessageType<SeqTypes>>(event_channel_capacity.get());

        let (genesis_payload, genesis_ns_table) =
            Payload::from_transactions([], &validated_state, &instance_state)
                .await
                .expect("genesis payload construction failed");
            println!("here genesis");

        let builder_commitment = genesis_payload.builder_commitment(&genesis_ns_table);

        let vid_commitment = {
            let payload_bytes = genesis_payload.encode();
            vid_commitment(&payload_bytes, GENESIS_VID_NUM_STORAGE_NODES)
        };

        // create the global state
        let global_state: GlobalState<SeqTypes> = GlobalState::<SeqTypes>::new(
            req_sender,
            senders.transactions.clone(),
            vid_commitment,
            bootstrapped_view,
            bootstrapped_view,
            buffered_view_num_count as u64,
        );
        println!("here global state");

        let global_state = Arc::new(RwLock::new(global_state));
        let global_state_clone = global_state.clone();

        let builder_state = BuilderState::<SeqTypes>::new(
            BuiltFromProposedBlock {
                view_number: bootstrapped_view,
                vid_commitment,
                leaf_commit: fake_commitment(),
                builder_commitment,
            },
            &receivers,
            req_receiver,
            Vec::new() /* tx_queue */,
            global_state_clone,
            node_count,
            maximize_txns_count_timeout_duration,
            base_fee
                .as_u64()
                .context("the base fee exceeds the maximum amount that a builder can pay (defined by u64::MAX)")?,
            Arc::new(instance_state),
            Duration::from_secs(60),
            Arc::new(validated_state),
        );
        println!("here builder state");

        // spawn the builder event loop
        // Note: we don't do anything with the handle because BuilderState's
        // event loop is going to be spawning child BuilderStates and will exit
        // when the view it's building for is decided, so we don't care about
        // it eventually finishing.
        async_spawn(async move {
            builder_state.event_loop();
        });
        println!("here event loop");

        // create the proxy global state it will server the builder apis
        let proxy_global_state = ProxyGlobalState::new(
            global_state.clone(),
            (builder_key_pair.fee_account(), builder_key_pair.clone()),
        );

        // start the hotshot api service
        run_builder_api_service(hotshot_builder_apis_url.clone(), proxy_global_state);
        println!("here builder api");

        // spawn the builder service
        let events_url = hotshot_events_api_url.clone();
        tracing::info!("Running permissionless builder against hotshot events API at {events_url}",);

        if is_reserve {
            let Some(bid_config) = bid_config else {
                panic!("Missing bid config for the reserve builder.");
            };
            let hooks = hooks::EspressoReserveHooks {
                namespaces: bid_config.namespaces.into_iter().collect(),
                solver_api_url,
                builder_api_base_url: hotshot_builder_apis_url.clone(),
                bid_key_pair: builder_key_pair,
                bid_amount: bid_config.amount,
            };

            async_spawn(async move {
                let res =
                    run_non_permissioned_standalone_builder_service(hooks, senders, events_url)
                        .await;
                tracing::error!(?res, "Reserve builder service exited");
                if res.is_err() {
                    panic!("Reserve builder should restart.");
                }
            });

            tracing::info!("Reserve builder init finished");
        } else {
            let hooks = hooks::EspressoFallbackHooks { solver_api_url };

            async_spawn(async move {
                let res =
                    run_non_permissioned_standalone_builder_service(hooks, senders, events_url)
                        .await;
                tracing::error!(?res, "Fallback builder service exited");
                if res.is_err() {
                    panic!("Fallback builder should restart.");
                }
            });

            tracing::info!("Fallback builder init finished");
        }
        println!("here run_non_permissioned_standalone_builder_service");

        Ok(Self {
            global_state,
            hotshot_events_api_url,
            hotshot_builder_apis_url,
        })
    }
}

#[cfg(test)]
mod test {
    use std::{str::FromStr, time::{Duration, Instant}};

    use async_compatibility_layer::{
        art::{async_sleep, async_spawn},
        logging::{setup_backtrace, setup_logging},
    };
    use async_lock::RwLock;
    use async_std::{stream::StreamExt, task,prelude::FutureExt};
    use committable::Commitment;
    use committable::Committable;
    use espresso_types::{
        mock::MockStateCatchup, v0_3::{RollupRegistration, RollupRegistrationBody}, FeeAccount, NamespaceId, PubKey, SeqTypes, Transaction
    };
    use ethers::{solc::resolver::print, utils::Anvil};
    use hotshot::types::BLSPrivKey;
    use hotshot_builder_api::v0_3::builder::BuildError;
    use hotshot_events_service::{
        events::{Error as EventStreamApiError, Options as EventStreamingApiOptions},
        events_source::{EventConsumer, EventsStreamer},
    };
    use hotshot_example_types::block_types::TestTransaction;
    use hotshot_query_service::availability::LeafQueryData;
    use hotshot_types::{
        bundle::Bundle,
        constants::MarketplaceVersion,
        light_client::StateKeyPair,
        signature_key::BLSPubKey,
        traits::{
            block_contents::{BlockPayload, GENESIS_VID_NUM_STORAGE_NODES},
            node_implementation::NodeType,
            signature_key::{BuilderSignatureKey, SignatureKey},
        },
    };
    use marketplace_builder_core::{
        builder_state::{self, RequestMessage, TransactionSource},
        service::{
            run_non_permissioned_standalone_builder_service,
            run_permissioned_standalone_builder_service,
        }, utils::BuilderStateId,
    };
    use marketplace_solver::{testing::MockSolver, SolverError};
    use portpicker::pick_unused_port;
    use sequencer::{
        api::test_helpers::TestNetworkConfigBuilder,
        persistence::no_storage::{self, NoStorage},
        testing::TestConfigBuilder,
    };
    use sequencer::{
        api::{fs::DataSource, options::HotshotEvents, test_helpers::TestNetwork, Options},
        persistence,
    };
    use sequencer_utils::test_utils::setup_test;
    use surf_disco::Client;
    use tempfile::TempDir;
    use tide_disco::error::ServerError;

    use super::*;

    #[async_std::test]
    async fn test_marketplace_fallback_builder() {
        setup_test();

        let query_port = pick_unused_port().expect("No ports free");
        let query_api_url: Url = format!("http://localhost:{query_port}").parse().unwrap();

        let event_port = pick_unused_port().expect("No ports free");
        let event_service_url: Url = format!("http://localhost:{event_port}").parse().unwrap();

        let builder_port = pick_unused_port().expect("No ports free");
        let builder_api_url: Url = format!("http://localhost:{builder_port}").parse().unwrap();

        // Set up and start the network
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();

        let tmpdir = TempDir::new().unwrap();
        println!("here after tmpdir");

        let config = TestNetworkConfigBuilder::default()
            .api_config(
                Options::with_port(query_port)
                    .submit(Default::default())
                    .query_fs(
                        Default::default(),
                        persistence::fs::Options::new(tmpdir.path().to_owned()),
                    )
                    .hotshot_events(HotshotEvents {
                        events_service_port: event_port,
                    }),
            )
            .network_config(network_config)
            .build();
        let network = TestNetwork::new(config, <SeqTypes as NodeType>::Base::instance()).await;

        let mock_solver = MockSolver::init().await;
        let solver_api = mock_solver.solver_api();
        println!("here after solver api");
        let client =
        surf_disco::Client::<SolverError, <SeqTypes as NodeType>::Base>::new(solver_api.clone());

    // Create a list of signature keys for rollup registration data
    let mut signature_keys = Vec::new();

    for _ in 0..10 {
        let private_key =
            <BLSPubKey as SignatureKey>::PrivateKey::generate(&mut rand::thread_rng());
        signature_keys.push(BLSPubKey::from_private(&private_key))
    }

    let private_key =
        <BLSPubKey as SignatureKey>::PrivateKey::generate(&mut rand::thread_rng());
    let signature_key = BLSPubKey::from_private(&private_key);

    signature_keys.push(signature_key);

                // Initialize a rollup registration with namespace id = 1
                let reg_ns_1_body = RollupRegistrationBody {
                    namespace_id: 1_u64.into(),
                    reserve_url: Url::from_str("http://localhost").unwrap(),
                    reserve_price: 200.into(),
                    active: true,
                    signature_keys: signature_keys.clone(),
                    text: "test".to_string(),
                    signature_key,
                };
        
                // Sign the registration body
                let reg_signature = <SeqTypes as NodeType>::SignatureKey::sign(
                    &private_key,
                    reg_ns_1_body.commit().as_ref(),
                )
                .expect("failed to sign");
        
                let reg_ns_1 = RollupRegistration {
                    body: reg_ns_1_body.clone(),
                    signature: reg_signature,
                };
        
                // registering a rollup
                let result: RollupRegistration = client
                    .post("register_rollup")
                    .body_json(&reg_ns_1)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();


        // Start the builder
        let init = BuilderConfig::init(
            false,
            FeeAccount::test_key_pair(),
            ViewNumber::genesis(),
            NonZeroUsize::new(1024).unwrap(),
            NonZeroUsize::new(1024).unwrap(),
            NonZeroUsize::new(network.cfg.num_nodes()).unwrap(),
            NodeState::default(),
            ValidatedState::default(),
            event_service_url.clone(),
            builder_api_url.clone(),
            Duration::from_secs(2),
            5,
            Duration::from_secs(2),
            FeeAmount::from(10),
            None,
            solver_api,
        );
        println!("here after init");
        let builder_config = init.await.unwrap();
        println!("here after await");

        // Wait for at least one empty block to be sequenced (after consensus starts VID).
        let sequencer_client: Client<ServerError, <SeqTypes as NodeType>::Base> =
            Client::new(query_api_url);
        sequencer_client.connect(None).await;
        sequencer_client
            .socket("availability/stream/leaves/0")
            .subscribe::<LeafQueryData<SeqTypes>>()
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap();

        //  Connect to builder
        let builder_client: Client<ServerError, MarketplaceVersion> =
            Client::new(builder_api_url.clone());
        builder_client.connect(None).await;

        //  TODO(AG): workaround for version mismatch between bundle and submit APIs
        let submission_client: Client<ServerError, <SeqTypes as NodeType>::Base> =
            Client::new(builder_api_url);
        submission_client.connect(None).await;

        // Test getting a bundle
let _bundle = builder_client
    .get::<Bundle<SeqTypes>>("block_info/bundle/1")
    .send()
    .await
    .unwrap();

            println!("here after bundle");
        // Test submitting transactions
        let tx_registered = Transaction::new(1u32.into(), vec![1, 1, 1, 1]);
        let tx_unregistered = Transaction::new(50u32.into(), vec![1, 1, 1, 1]);
        let global_state = builder_config.global_state.read().await;
        println!("here global_state {:?}",global_state);

        global_state
            .tx_sender
            .try_broadcast(Arc::new(ReceivedTransaction {
                tx: tx_registered.clone(),
                source: TransactionSource::External,
                commit: tx_registered.commit(),
                time_in: Instant::now(),
            }))
            .unwrap();
        println!("here after global_state {:?}",global_state.tx_sender);
        drop(global_state);

        let (response_sender, response_receiver) = unbounded();
        let request_message = MessageType::<SeqTypes>::RequestMessage(RequestMessage {
            requested_view_number: ViewNumber::new(0),
            response_channel: response_sender,
        });
        let transactions = Vec::new();
        let encoded_transactions = TestTransaction::encode(&transactions);
        let block_vid_commitment = vid_commitment(&encoded_transactions, GENESIS_VID_NUM_STORAGE_NODES);
        let req_msg = (
            response_receiver,
            BuilderStateId {
                parent_commitment: block_vid_commitment,
                view: ViewNumber::genesis(),
            },
            request_message,
        );

        // give builder state time to fork
        async_sleep(Duration::from_millis(100)).await;

        // get the builder state for parent view we've just simulated
        builder_config.global_state
            .read_arc()
            .await
            .spawned_builder_states
            .get(&req_msg.1)
            .expect("Failed to get channel for matching builder")
            .broadcast(req_msg.2.clone())
            .await
            .unwrap();

        // get response
        let res_msg = req_msg
            .0
            .recv()
            .timeout(Duration::from_secs(10))
            .await
            .unwrap()
            .unwrap();
        println!("here after res_msg {:?}",res_msg);

        let global_state = builder_config.global_state.read().await;
        println!("here after global_state {:?}",global_state.tx_sender);
        println!("here after blocks {:?}",global_state.blocks);
        assert_eq!(global_state.blocks.len(), 0);

        global_state
            .tx_sender
            .try_broadcast(Arc::new(ReceivedTransaction {
                tx: tx_unregistered.clone(),
                source: TransactionSource::External,
                commit: tx_unregistered.commit(),
                time_in: Instant::now(),
            }))
            .unwrap();

            println!("here after global_state {:?}",global_state.tx_sender);
            println!("here after global_state {:?}",global_state.spawned_builder_states);

        // assert_eq!(global_state.blocks.len(), 1);
        // println!("here after tx unregistered");

    }
}
