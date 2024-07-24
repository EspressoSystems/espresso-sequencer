use std::sync::Arc;

use super::{
    get_stake_table_from_sequencer, process_node_identity_url_stream,
    stream_leaves_from_hotshot_query_service,
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
    channel::mpsc::{self, Receiver, Sender},
    SinkExt,
};
use url::Url;

pub struct NodeValidatorAPI {
    pub task_handles: Vec<JoinHandle<()>>,
}

pub struct NodeValidatorConfig {
    pub bind_address: String,
    pub stake_table_url_base: Url,
    pub initial_node_public_base_urls: Vec<Url>,
}

#[derive(Debug)]
pub enum CreateNodeValidatorProcessingError {
    FailedToGetStakeTable(hotshot_query_service::Error),
}

/**
 * create_node_validator_processing is a function that creates a node validator
 * processing environment.  This function will create a number of tasks that
 * will be responsible for processing the data streams that are coming in from
 * the various sources.  This function will also create the data state that
 * will be used to store the state of the network.
 */
pub async fn create_node_validator_processing(
    config: NodeValidatorConfig,
    server_message_receiver: Receiver<InternalClientMessage<Sender<ServerMessage>>>,
) -> Result<NodeValidatorAPI, CreateNodeValidatorProcessingError> {
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

    let client = surf_disco::Client::new(config.stake_table_url_base);

    let stake_table = get_stake_table_from_sequencer(client.clone())
        .await
        .map_err(CreateNodeValidatorProcessingError::FailedToGetStakeTable)?;

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
        // Alright, let's start processing leaves
        // TODO: We should move this into its own function that can respond
        //       and react appropriately when a service or sequencer does down
        //       so that it can gracefully re-establish the stream as necessary.

        let client = client;

        let mut leaf_stream = match stream_leaves_from_hotshot_query_service(None, client).await {
            Ok(leaf_stream) => leaf_stream,
            Err(err) => {
                tracing::info!("error getting leaf stream: {}", err);
                return;
            }
        };

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

    Ok(NodeValidatorAPI {
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
    })
}

#[cfg(test)]
mod test {
    use crate::{
        api::node_validator::v0::{StateClientMessageSender, STATIC_VER_0_1},
        service::{client_message::InternalClientMessage, server_message::ServerMessage},
    };
    use futures::channel::mpsc::{self, Sender};
    use tide_disco::App;

    struct TestState(Sender<InternalClientMessage<Sender<ServerMessage>>>);

    impl StateClientMessageSender<Sender<ServerMessage>> for TestState {
        fn sender(&self) -> Sender<InternalClientMessage<Sender<ServerMessage>>> {
            self.0.clone()
        }
    }

    #[async_std::test]
    #[ignore]
    async fn test_full_setup_example() {
        let (internal_client_message_sender, internal_client_message_receiver) = mpsc::channel(32);
        let state = TestState(internal_client_message_sender);
        // let state = Arc::new(state);

        let mut app: App<_, crate::api::node_validator::v0::Error> = App::with_state(state);
        let node_validator_api_result = super::super::define_api::<TestState>();
        let node_validator_api = match node_validator_api_result {
            Ok(node_validator_api) => node_validator_api,
            Err(err) => {
                panic!("error defining node validator api: {:?}", err);
            }
        };

        match app.register_module("node-validator", node_validator_api) {
            Ok(_) => {}
            Err(err) => {
                panic!("error registering node validator api: {:?}", err);
            }
        }

        let node_validator_task_state = match super::create_node_validator_processing(
            super::NodeValidatorConfig {
                bind_address: "0.0.0.0:9000".to_string(),
                stake_table_url_base: "http://localhost:24000/v0".parse().unwrap(),
                initial_node_public_base_urls: vec![
                    "http://localhost:24000/".parse().unwrap(),
                    "http://localhost:24001/".parse().unwrap(),
                    "http://localhost:24002/".parse().unwrap(),
                    "http://localhost:24003/".parse().unwrap(),
                    "http://localhost:24004/".parse().unwrap(),
                ],
            },
            internal_client_message_receiver,
        )
        .await
        {
            Ok(node_validator_task_state) => node_validator_task_state,

            Err(err) => {
                panic!("error defining node validator api: {:?}", err);
            }
        };

        // We would like to wait until being signaled
        let app_serve_handle = async_std::task::spawn(async move {
            let app_serve_result = app.serve("0.0.0.0:9000", STATIC_VER_0_1).await;
            tracing::info!("app serve result: {:?}", app_serve_result);
        });

        app_serve_handle.await;

        for handle in node_validator_task_state.task_handles {
            handle.cancel().await;
        }
    }
}
