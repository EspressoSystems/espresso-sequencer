use anyhow::Context;
use async_broadcast::{
    broadcast, Receiver as BroadcastReceiver, RecvError, Sender as BroadcastSender, TryRecvError,
};
use async_compatibility_layer::{
    art::{async_sleep, async_spawn},
    channel::{unbounded, UnboundedReceiver, UnboundedSender},
};
use async_std::sync::{Arc, RwLock};
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{coins_bip39::English, MnemonicBuilder, Signer as _, Wallet},
    types::{Address, U256},
};
use hotshot::traits::BlockPayload;
use hotshot_builder_api::builder::{
    BuildError, Error as BuilderApiError, Options as HotshotBuilderApiOptions,
};
use hotshot_builder_core::{
    builder_state::{
        BuildBlockInfo, BuilderProgress, BuilderState, BuiltFromProposedBlock, MessageType,
        ResponseMessage,
    },
    service::{run_non_permissioned_standalone_builder_service, GlobalState, ProxyGlobalState},
};

use hotshot_types::{
    constants::{Version01, STATIC_VER_0_1},
    data::{fake_commitment, Leaf, ViewNumber},
    traits::{
        block_contents::{vid_commitment, GENESIS_VID_NUM_STORAGE_NODES},
        node_implementation::{ConsensusTime, NodeType},
        EncodeBytes,
    },
    utils::BuilderCommitment,
};
use sequencer::{
    catchup::StatePeers, eth_signature_key::EthKeyPair, l1_client::L1Client, BuilderParams,
    ChainConfig, L1Params, NetworkParams, NodeState, Payload, PrivKey, PubKey, SeqTypes,
    ValidatedState,
};

use hotshot_events_service::{
    events::{Error as EventStreamApiError, Options as EventStreamingApiOptions},
    events_source::{BuilderEvent, EventConsumer, EventsStreamer},
};

use crate::run_builder_api_service;
use std::{num::NonZeroUsize, time::Duration};
use surf::http::headers::ACCEPT;
use surf_disco::Client;
use tide_disco::{app, method::ReadState, App, Url};
use vbs::version::StaticVersionType;

#[derive(Clone, Debug)]
pub struct BuilderConfig {
    pub global_state: Arc<RwLock<GlobalState<SeqTypes>>>,
    pub hotshot_events_api_url: Url,
    pub hotshot_builder_apis_url: Url,
}

pub fn build_instance_state<Ver: StaticVersionType + 'static>(
    l1_params: L1Params,
    state_peers: Vec<Url>,
    chain_config: ChainConfig,
    _: Ver,
) -> anyhow::Result<NodeState> {
    let l1_client = L1Client::new(l1_params.url, l1_params.events_max_block_range);
    let instance_state = NodeState::new(
        u64::MAX, // dummy node ID, only used for debugging
        chain_config,
        l1_client,
        Arc::new(StatePeers::<Ver>::from_urls(state_peers)),
    );
    Ok(instance_state)
}

impl BuilderConfig {
    #[allow(clippy::too_many_arguments)]
    pub async fn init(
        builder_key_pair: EthKeyPair,
        bootstrapped_view: ViewNumber,
        channel_capacity: NonZeroUsize,
        node_count: NonZeroUsize,
        instance_state: NodeState,
        validated_state: ValidatedState,
        hotshot_events_api_url: Url,
        hotshot_builder_apis_url: Url,
        max_api_timeout_duration: Duration,
        buffered_view_num_count: usize,
        maximize_txns_count_timeout_duration: Duration,
    ) -> anyhow::Result<Self> {
        tracing::info!(
            address = %builder_key_pair.fee_account(),
            ?bootstrapped_view,
            %channel_capacity,
            ?max_api_timeout_duration,
            buffered_view_num_count,
            ?maximize_txns_count_timeout_duration,
            "initializing builder",
        );

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
            tx_receiver,
            decide_receiver,
            da_receiver,
            qc_receiver,
            req_receiver,
            global_state_clone,
            node_count,
            maximize_txns_count_timeout_duration,
            instance_state
                .chain_config()
                .base_fee
                .as_u64()
                .context("the base fee exceeds the maximum amount that a builder can pay (defined by u64::MAX)")?,
            Arc::new(instance_state),
            Arc::new(validated_state),
            //????
            Duration::from_secs(60),
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
        tracing::info!("Running permissionless builder against hotshot events API at {events_url}",);
        async_spawn(async move {
            let res = run_non_permissioned_standalone_builder_service(
                tx_sender,
                da_sender,
                qc_sender,
                decide_sender,
                events_url,
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
    use super::*;
    use crate::testing::{
        hotshot_builder_url, HotShotTestConfig, NonPermissionedBuilderTestConfig,
    };
    use async_compatibility_layer::art::{async_sleep, async_spawn};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_lock::RwLock;
    use async_std::task;
    use es_version::SequencerVersion;
    use hotshot_builder_api::{
        block_info::{AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo},
        builder::BuildError,
    };
    use hotshot_builder_core::builder_state::BuilderProgress;
    use hotshot_builder_core::service::{
        run_non_permissioned_standalone_builder_service,
        run_permissioned_standalone_builder_service,
    };
    use hotshot_events_service::{
        events::{Error as EventStreamApiError, Options as EventStreamingApiOptions},
        events_source::{BuilderEvent, EventConsumer, EventsStreamer},
    };
    use hotshot_types::{
        constants::{Version01, STATIC_VER_0_1},
        traits::{
            block_contents::{BlockPayload, GENESIS_VID_NUM_STORAGE_NODES},
            node_implementation::NodeType,
        },
    };
    use hotshot_types::{signature_key::BLSPubKey, traits::signature_key::SignatureKey};
    use sequencer::{
        persistence::{
            no_storage::{self, NoStorage},
            PersistenceOptions,
        },
        state::FeeAccount,
        transaction::Transaction,
        Payload,
    };
    use std::time::Duration;
    use surf_disco::Client;

    /// Test the non-permissioned builder core
    /// It creates a memory hotshot network and launches the hotshot event streaming api
    /// Builder subscrived to this api, and server the hotshot client request and the private mempool tx submission
    #[async_std::test]
    async fn test_non_permissioned_builder() {
        setup_logging();
        setup_backtrace();

        let ver = SequencerVersion::instance();
        // Hotshot Test Config
        let hotshot_config = HotShotTestConfig::default();

        // Get the handle for all the nodes, including both the non-builder and builder nodes
        let handles = hotshot_config.init_nodes(ver, no_storage::Options).await;

        // start consensus for all the nodes
        for (handle, ..) in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        // get the required stuff for the election config
        let known_nodes_with_stake = hotshot_config.config.known_nodes_with_stake.clone();

        // get count of non-staking nodes
        let num_non_staking_nodes = hotshot_config.config.num_nodes_without_stake;

        // non-staking node handle
        let hotshot_context_handle = handles
            [NonPermissionedBuilderTestConfig::SUBSCRIBED_DA_NODE_ID]
            .0
            .clone();

        // hotshot event streaming api url
        let hotshot_events_streaming_api_url = HotShotTestConfig::hotshot_event_streaming_api_url();

        // enable a hotshot node event streaming
        HotShotTestConfig::enable_hotshot_node_event_streaming::<NoStorage>(
            hotshot_events_streaming_api_url.clone(),
            known_nodes_with_stake,
            num_non_staking_nodes,
            hotshot_context_handle,
        );

        // builder api url
        let hotshot_builder_api_url = hotshot_config.config.builder_url.clone();

        let builder_config = NonPermissionedBuilderTestConfig::init_non_permissioned_builder(
            &hotshot_config,
            hotshot_events_streaming_api_url.clone(),
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

        let parent_commitment = vid_commitment(&[], GENESIS_VID_NUM_STORAGE_NODES);

        // sign the parent_commitment using the client_private_key
        let encoded_signature = <SeqTypes as NodeType>::SignatureKey::sign(
            &hotshot_client_private_key,
            parent_commitment.as_ref(),
        )
        .expect("Claim block signing failed");

        // sleep and wait for builder service to startup
        async_sleep(Duration::from_millis(3000)).await;

        let test_view_num = 0;

        // test getting available blocks
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
