use std::{collections::VecDeque, num::NonZeroUsize, time::Duration};

use anyhow::Context;
use async_broadcast::{
    broadcast, Receiver as BroadcastReceiver, RecvError, Sender as BroadcastSender, TryRecvError,
};
use async_compatibility_layer::{
    art::{async_sleep, async_spawn},
    channel::{unbounded, UnboundedReceiver, UnboundedSender},
};
use async_std::sync::{Arc, RwLock};
use espresso_types::{
    eth_signature_key::EthKeyPair, v0_3::ChainConfig, FeeAmount, L1Client, MockSequencerVersions,
    NodeState, Payload, SeqTypes, SequencerVersions, ValidatedState,
};
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{coins_bip39::English, MnemonicBuilder, Signer as _, Wallet},
    types::{Address, U256},
};
use hotshot::traits::BlockPayload;
use hotshot_builder_api::v0_1::builder::{
    BuildError, Error as BuilderApiError, Options as HotshotBuilderApiOptions,
};
use hotshot_builder_core::{
    builder_state::{
        BuildBlockInfo, BuilderState, BuiltFromProposedBlock, MessageType, ResponseMessage,
    },
    service::{
        run_non_permissioned_standalone_builder_service, GlobalState, ProxyGlobalState,
        ReceivedTransaction,
    },
};
use hotshot_events_service::{
    events::{Error as EventStreamApiError, Options as EventStreamingApiOptions},
    events_source::{EventConsumer, EventsStreamer},
};
use hotshot_types::{
    data::{fake_commitment, Leaf, ViewNumber},
    traits::{
        block_contents::{vid_commitment, GENESIS_VID_NUM_STORAGE_NODES},
        node_implementation::{ConsensusTime, NodeType, Versions},
        EncodeBytes,
    },
    utils::BuilderCommitment,
};
use sequencer::{catchup::StatePeers, L1Params, NetworkParams, SequencerApiVersion};
use surf::http::headers::ACCEPT;
use surf_disco::Client;
use tide_disco::{app, method::ReadState, App, Url};
use vbs::version::{StaticVersionType, Version};

use crate::run_builder_api_service;

#[derive(Clone, Debug)]
pub struct BuilderConfig {
    pub global_state: Arc<RwLock<GlobalState<SeqTypes>>>,
    pub hotshot_events_api_url: Url,
    pub hotshot_builder_apis_url: Url,
}

pub async fn build_instance_state<V: Versions>(
    chain_config: ChainConfig,
    l1_params: L1Params,
    state_peers: Vec<Url>,
    _versions: V,
) -> anyhow::Result<NodeState> {
    let l1_client = L1Client::new(l1_params.url, l1_params.events_max_block_range);
    let instance_state = NodeState::new(
        u64::MAX, // dummy node ID, only used for debugging
        chain_config,
        l1_client,
        Arc::new(StatePeers::<SequencerApiVersion>::from_urls(
            state_peers,
            Default::default(),
        )),
        V::Base::VERSION,
    );
    Ok(instance_state)
}

impl BuilderConfig {
    #[allow(clippy::too_many_arguments)]
    pub async fn init(
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
    ) -> anyhow::Result<Self> {
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
            VecDeque::new() /* tx_queue */,
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

        // spawn the builder event loop
        async_spawn(async move {
            builder_state.event_loop();
        });

        // create the proxy global state it will server the builder apis
        let proxy_global_state = ProxyGlobalState::new(
            global_state.clone(),
            (builder_key_pair.fee_account(), builder_key_pair),
            max_api_timeout_duration,
        );

        // start the hotshot api service
        run_builder_api_service(hotshot_builder_apis_url.clone(), proxy_global_state);

        // spawn the builder service
        let events_url = hotshot_events_api_url.clone();
        let global_state_clone = global_state.clone();
        tracing::info!("Running permissionless builder against hotshot events API at {events_url}",);
        async_spawn(async move {
            let res = run_non_permissioned_standalone_builder_service::<_, SequencerApiVersion>(
                da_sender,
                qc_sender,
                decide_sender,
                events_url,
                global_state_clone,
            )
            .await;
            tracing::error!(?res, "builder service exited");
            if res.is_err() {
                panic!("Builder should restart.");
            }
        });

        tracing::info!("Builder init finished");
        Ok(Self {
            global_state,
            hotshot_events_api_url,
            hotshot_builder_apis_url,
        })
    }
}

#[cfg(test)]
mod test {
    use std::time::{Duration, Instant};

    use async_compatibility_layer::{
        art::{async_sleep, async_spawn},
        logging::{setup_backtrace, setup_logging},
    };
    use async_lock::RwLock;
    use async_std::{stream::StreamExt, task};
    use espresso_types::{Event, FeeAccount, NamespaceId, Transaction};
    use ethers::utils::Anvil;
    use futures::Stream;
    use hotshot::types::EventType;
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
    use portpicker::pick_unused_port;
    use sequencer::{
        api::{
            options::HotshotEvents,
            test_helpers::{TestNetwork, TestNetworkConfigBuilder},
            Options,
        },
        persistence::{
            self,
            no_storage::{self, NoStorage},
        },
        testing::TestConfigBuilder,
    };
    use sequencer_utils::test_utils::setup_test;
    use surf_disco::Client;
    use tempfile::TempDir;
    use vbs::version::StaticVersion;

    use super::*;
    use crate::testing::{
        hotshot_builder_url, test_builder_impl, HotShotTestConfig, NonPermissionedBuilderTestConfig,
    };

    /// Test the non-permissioned builder core
    /// It creates a memory hotshot network and launches the hotshot event streaming api
    /// Builder subscrived to this api, and server the hotshot client request and the private mempool tx submission
    #[async_std::test]
    async fn test_non_permissioned_builder() {
        setup_test();

        let query_port = pick_unused_port().expect("No ports free");

        let event_port = pick_unused_port().expect("No ports free");
        let event_service_url: Url = format!("http://localhost:{event_port}").parse().unwrap();

        let builder_port = pick_unused_port().expect("No ports free");
        let builder_api_url: Url = format!("http://localhost:{builder_port}").parse().unwrap();

        // Set up and start the network
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();

        let tmpdir = TempDir::new().unwrap();

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
        let network = TestNetwork::new(config, MockSequencerVersions::new()).await;

        let builder_config = NonPermissionedBuilderTestConfig::init_non_permissioned_builder::<
            MockSequencerVersions,
        >(
            event_service_url.clone(),
            builder_api_url.clone(),
            network.cfg.num_nodes(),
        )
        .await;

        let events_service_client = Client::<
            hotshot_events_service::events::Error,
            SequencerApiVersion,
        >::new(event_service_url.clone());
        events_service_client.connect(None).await;

        let subscribed_events = events_service_client
            .socket("hotshot-events/events")
            .subscribe::<hotshot_types::event::Event<SeqTypes>>()
            .await
            .unwrap()
            .map(|item| item.expect("Failed to get event from event service"));

        test_builder_impl(
            builder_api_url,
            subscribed_events,
            builder_config.fee_account,
        )
        .await;
    }
}
