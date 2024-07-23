use std::sync::Arc;

use super::{
    get_stake_table_from_sequencer, process_node_identity_url_stream,
    stream_leaves_from_hotshot_query_service, StateClientMessageSender, STATIC_VER_0_1,
};
use crate::service::{
    client_id::ClientId,
    client_message::InternalClientMessage,
    client_state::{
        process_distribute_block_detail_handling_stream,
        process_distribute_node_identity_handling_stream,
        process_distribute_voters_handling_stream, process_internal_client_message_stream,
        ClientThreadState,
    },
    data_state::{process_leaf_stream, process_node_identity_stream, DataState},
    server_message::ServerMessage,
};
use async_std::{stream::StreamExt, sync::RwLock, task::JoinHandle};
use futures::{
    channel::mpsc::{self, Sender},
    SinkExt,
};
use tide_disco::App;
use url::Url;

pub struct NodeValidatorAPIState {
    pub sender: Sender<InternalClientMessage<Sender<ServerMessage>>>,
}

impl StateClientMessageSender<Sender<ServerMessage>> for NodeValidatorAPIState {
    fn sender(&self) -> Sender<InternalClientMessage<Sender<ServerMessage>>> {
        self.sender.clone()
    }
}

pub struct NodeValidatorAPI {
    pub task_handles: Vec<JoinHandle<()>>,
}

pub struct NodeValidatorConfig {
    pub bind_address: String,
    pub stake_table_url_base: Url,
    pub initial_node_public_base_urls: Vec<Url>,
}

pub async fn create_node_validator_api(
    config: NodeValidatorConfig,
) -> (NodeValidatorAPI, JoinHandle<()>) {
    let node_validator_api_result = super::define_api::<NodeValidatorAPIState>();

    let node_validator_api = match node_validator_api_result {
        Ok(api) => api,
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    };

    let (server_message_sender, server_message_receiver) = mpsc::channel(32);
    let mut app: App<NodeValidatorAPIState, super::Error> =
        App::with_state(NodeValidatorAPIState {
            sender: server_message_sender,
        });
    let register_module_result = app.register_module("node-validator", node_validator_api);

    if let Err(e) = register_module_result {
        panic!("Error: {:?}", e);
    }

    let mut data_state = DataState::new(
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    );

    let client_thread_state = ClientThreadState::<Sender<ServerMessage>>::new(
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        ClientId::from_count(1),
    );

    let client = surf_disco::Client::new(
        // "https://query.cappuccino.testnet.espresso.network/v0"
        config.stake_table_url_base,
    );

    let get_stake_table_result = get_stake_table_from_sequencer(client.clone()).await;
    let stake_table = get_stake_table_result.unwrap();
    data_state.replace_stake_table(stake_table);

    let data_state = Arc::new(RwLock::new(data_state));
    let client_thread_state = Arc::new(RwLock::new(client_thread_state));
    let (block_detail_sender, block_detail_receiver) = mpsc::channel(32);
    let (leaf_sender, leaf_receiver) = mpsc::channel(32);
    let (node_identity_sender_1, node_identity_receiver_1) = mpsc::channel(32);
    let (node_identity_sender_2, node_identity_receiver_2) = mpsc::channel(32);
    let (voters_sender, voters_receiver) = mpsc::channel(32);
    let (mut url_sender, url_receiver) = mpsc::channel(32);

    let process_internal_client_message_handle =
        async_std::task::spawn(process_internal_client_message_stream(
            server_message_receiver,
            data_state.clone(),
            client_thread_state.clone(),
        ));

    let process_distribute_block_detail_handle =
        async_std::task::spawn(process_distribute_block_detail_handling_stream(
            client_thread_state.clone(),
            block_detail_receiver,
        ));

    let process_distribute_node_identity_handle =
        async_std::task::spawn(process_distribute_node_identity_handling_stream(
            client_thread_state.clone(),
            node_identity_receiver_2,
        ));

    let process_distribute_voters_handle = async_std::task::spawn(
        process_distribute_voters_handling_stream(client_thread_state.clone(), voters_receiver),
    );

    let process_leaf_stream_handle = async_std::task::spawn(process_leaf_stream(
        leaf_receiver,
        data_state.clone(),
        block_detail_sender,
        voters_sender,
    ));

    let process_node_identity_stream_handle = async_std::task::spawn(process_node_identity_stream(
        node_identity_receiver_1,
        data_state.clone(),
        node_identity_sender_2,
    ));

    let process_url_stream_handle = async_std::task::spawn(process_node_identity_url_stream(
        url_receiver,
        node_identity_sender_1,
    ));

    let leaf_retriever_handle = async_std::task::spawn(async move {
        // Alright, let's get some leaves, bro

        let client = client;

        let mut leaf_stream = stream_leaves_from_hotshot_query_service(None, client)
            .await
            .unwrap();

        let mut leaf_sender = leaf_sender;

        loop {
            let leaf_result = leaf_stream.next().await;
            let leaf = if let Some(Ok(leaf)) = leaf_result {
                leaf
            } else {
                tracing::info!("leaf stream closed");
                break;
            };

            let leaf_send_result = leaf_sender.send(leaf).await;
            if let Err(err) = leaf_send_result {
                tracing::info!("leaf sender closed: {}", err);
                break;
            }
        }
    });

    // send the original three node base urls
    // This is assuming that demo-native is running, as such those Urls
    // should be used / match
    {
        let urls = config.initial_node_public_base_urls;

        for url in urls {
            let send_result = url_sender.send(url).await;
            if let Err(err) = send_result {
                tracing::info!("url sender closed: {}", err);
                break;
            }
        }
    }

    let app_serve_handle = async_std::task::spawn(async move {
        let app_serve_result = app.serve("0.0.0.0:9000", STATIC_VER_0_1).await;
        tracing::info!("app serve result: {:?}", app_serve_result);
    });

    tracing::info!("listening on: {:?}", config.bind_address);

    (
        NodeValidatorAPI {
            task_handles: vec![
                process_internal_client_message_handle,
                process_distribute_block_detail_handle,
                process_distribute_node_identity_handle,
                process_distribute_voters_handle,
                process_leaf_stream_handle,
                process_node_identity_stream_handle,
                process_url_stream_handle,
                leaf_retriever_handle,
            ],
        },
        app_serve_handle,
    )
}

mod test {

    #[async_std::test]
    #[ignore]
    async fn test_full_setup_example() {
        let (node_validator_api, app_serve_handle) =
            super::create_node_validator_api(super::NodeValidatorConfig {
                bind_address: "0.0.0.0:9000".to_string(),
                stake_table_url_base: "http://localhost:24000/v0".parse().unwrap(),
                initial_node_public_base_urls: vec![
                    "http://localhost:24000/".parse().unwrap(),
                    "http://localhost:24001/".parse().unwrap(),
                    "http://localhost:24002/".parse().unwrap(),
                    "http://localhost:24003/".parse().unwrap(),
                    "http://localhost:24004/".parse().unwrap(),
                ],
            })
            .await;

        // We would like to wait until being signaled
        app_serve_handle.await;

        for handle in node_validator_api.task_handles {
            handle.cancel().await;
        }
    }
}
