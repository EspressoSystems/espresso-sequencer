use std::sync::Arc;

use super::{
    get_stake_table_from_sequencer, ProcessNodeIdentityUrlStreamTask, ProcessProduceLeafStreamTask,
};
use crate::service::{
    client_id::ClientId,
    client_message::InternalClientMessage,
    client_state::{
        ClientThreadState, InternalClientMessageProcessingTask,
        ProcessDistributeBlockDetailHandlingTask, ProcessDistributeNodeIdentityHandlingTask,
        ProcessDistributeVotersHandlingTask,
    },
    data_state::{DataState, ProcessLeafStreamTask, ProcessNodeIdentityStreamTask},
    server_message::ServerMessage,
};
use async_std::{sync::RwLock, task::JoinHandle};
use espresso_types::{PubKey, SeqTypes};
use futures::{
    channel::mpsc::{self, Receiver, SendError, Sender},
    Sink, SinkExt, Stream, StreamExt,
};
use hotshot_types::event::{Event, EventType};
use serde::{Deserialize, Serialize};
use url::Url;

pub struct NodeValidatorAPI {
    pub process_internal_client_message_handle: Option<InternalClientMessageProcessingTask>,
    pub process_distribute_block_detail_handle: Option<ProcessDistributeBlockDetailHandlingTask>,
    pub process_distribute_node_identity_handle: Option<ProcessDistributeNodeIdentityHandlingTask>,
    pub process_distribute_voters_handle: Option<ProcessDistributeVotersHandlingTask>,
    pub process_leaf_stream_handle: Option<ProcessLeafStreamTask>,
    pub process_node_identity_stream_handle: Option<ProcessNodeIdentityStreamTask>,
    pub process_url_stream_handle: Option<ProcessNodeIdentityUrlStreamTask>,
    pub process_consume_leaves: Option<ProcessProduceLeafStreamTask>,
    pub hotshot_event_processing_task: Option<HotShotEventProcessingTask>,
}

pub struct NodeValidatorConfig {
    pub stake_table_url_base: Url,
    pub initial_node_public_base_urls: Vec<Url>,
}

#[derive(Debug)]
pub enum CreateNodeValidatorProcessingError {
    FailedToGetStakeTable(hotshot_query_service::Error),
}

/// An external message that can be sent to or received from a node
#[derive(Serialize, Deserialize, Clone)]
pub enum ExternalMessage {
    /// A request for a node to respond with its identifier
    /// Contains the public key of the node that is requesting the roll call
    RollCallRequest(PubKey),

    /// A response to a roll call request
    /// Contains the identifier of the node
    RollCallResponse(RollCallInfo),
}

/// Information about a node that is used in a roll call response
#[derive(Serialize, Deserialize, Clone)]
pub struct RollCallInfo {
    // The public API URL of the node
    pub public_api_url: Url,
}

pub struct HotShotEventProcessingTask {
    pub task_handle: Option<JoinHandle<()>>,
}

impl HotShotEventProcessingTask {
    pub fn new<S, K>(event_stream: S, url_sender: K) -> Self
    where
        S: Stream<Item = Event<SeqTypes>> + Send + Unpin + 'static,
        K: Sink<Url, Error = SendError> + Send + Unpin + 'static,
    {
        let task_handle = async_std::task::spawn(Self::process_messages(event_stream, url_sender));

        Self {
            task_handle: Some(task_handle),
        }
    }

    async fn process_messages<S, K>(event_receiver: S, url_sender: K)
    where
        S: Stream<Item = Event<SeqTypes>> + Send + Unpin + 'static,
        K: Sink<Url, Error = SendError> + Unpin,
    {
        let mut event_stream = event_receiver;
        let mut url_sender = url_sender;
        loop {
            let event_result = event_stream.next().await;
            let event = match event_result {
                Some(event) => event,
                None => {
                    tracing::info!("event stream closed");
                    break;
                }
            };

            let Event { event, .. } = event;

            let external_message_deserialize_result =
                if let EventType::ExternalMessageReceived(external_message_bytes) = event {
                    bincode::deserialize(&external_message_bytes)
                } else {
                    // Ignore all events that are not external messages
                    continue;
                };

            let external_message: ExternalMessage = match external_message_deserialize_result {
                Ok(external_message) => external_message,
                Err(err) => {
                    tracing::info!(
                        "failed to deserialize external message, unrecognized: {}",
                        err
                    );
                    continue;
                }
            };

            let public_api_url = match external_message {
                ExternalMessage::RollCallResponse(roll_call_response) => {
                    roll_call_response.public_api_url
                }
                _ => continue,
            };

            // Send the the discovered public url to the sink
            let send_result = url_sender.send(public_api_url).await;
            if let Err(err) = send_result {
                tracing::info!("url sender closed: {}", err);
                break;
            }
        }
    }
}

/**
 * create_node_validator_processing is a function that creates a node validator
 * processing environment.  This function will create a number of tasks that
 * will be responsible for processing the data streams that are coming in from
 * the various sources.  This function will also create the data state that
 * will be used to store the state of the network.
 */
pub async fn create_node_validator_processing<M, K>(
    config: NodeValidatorConfig,
    internal_client_message_receiver: Receiver<InternalClientMessage<Sender<ServerMessage>>>,
    public_key: PubKey,
    event_stream: Option<M>,
    external_message_sink: Option<K>,
) -> Result<NodeValidatorAPI, CreateNodeValidatorProcessingError>
where
    M: Stream<Item = Event<SeqTypes>> + Send + Unpin + 'static,
    K: Sink<ExternalMessage, Error = SendError> + Send + Unpin + 'static,
{
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

    let client_stake_table = surf_disco::Client::new(config.stake_table_url_base.clone());
    let client_leaf_stream = surf_disco::Client::new(config.stake_table_url_base);

    let stake_table = get_stake_table_from_sequencer(client_stake_table)
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

    let process_internal_client_message_handle = InternalClientMessageProcessingTask::new(
        internal_client_message_receiver,
        data_state.clone(),
        client_thread_state.clone(),
    );

    let process_distribute_block_detail_handle = ProcessDistributeBlockDetailHandlingTask::new(
        client_thread_state.clone(),
        block_detail_receiver,
    );

    let process_distribute_node_identity_handle = ProcessDistributeNodeIdentityHandlingTask::new(
        client_thread_state.clone(),
        node_identity_receiver_2,
    );

    let process_distribute_voters_handle =
        ProcessDistributeVotersHandlingTask::new(client_thread_state.clone(), voters_receiver);

    let process_leaf_stream_handle = ProcessLeafStreamTask::new(
        leaf_receiver,
        data_state.clone(),
        block_detail_sender,
        voters_sender,
    );

    let process_node_identity_stream_handle = ProcessNodeIdentityStreamTask::new(
        node_identity_receiver_1,
        data_state.clone(),
        node_identity_sender_2,
    );

    let process_url_stream_handle =
        ProcessNodeIdentityUrlStreamTask::new(url_receiver, node_identity_sender_1);

    let process_consume_leaves = ProcessProduceLeafStreamTask::new(client_leaf_stream, leaf_sender);

    let hotshot_event_processing_task = match (event_stream, external_message_sink) {
        (Some(event_stream), Some(mut external_message_sink)) => {
            let hotshot_event_processing_task =
                HotShotEventProcessingTask::new(event_stream, url_sender.clone());

            let send_roll_call_result = external_message_sink
                .send(ExternalMessage::RollCallRequest(public_key))
                .await;

            if let Err(err) = send_roll_call_result {
                tracing::info!("external message sink closed: {}", err);
            }

            Some(hotshot_event_processing_task)
        }
        _ => {
            // It doesn't make sne to send out a RollCall message if we don't
            // have the ability to receive the response.
            None
        }
    };

    // Send any initial URLS to the url sender for immediate processing.
    // These urls are supplied by the configuration of this function
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
        process_internal_client_message_handle: Some(process_internal_client_message_handle),
        process_distribute_block_detail_handle: Some(process_distribute_block_detail_handle),
        process_distribute_node_identity_handle: Some(process_distribute_node_identity_handle),
        process_distribute_voters_handle: Some(process_distribute_voters_handle),
        process_leaf_stream_handle: Some(process_leaf_stream_handle),
        process_node_identity_stream_handle: Some(process_node_identity_stream_handle),
        process_url_stream_handle: Some(process_url_stream_handle),
        process_consume_leaves: Some(process_consume_leaves),
        hotshot_event_processing_task,
    })
}

#[cfg(test)]
mod test {
    use crate::{
        api::node_validator::v0::{StateClientMessageSender, STATIC_VER_0_1},
        service::{client_message::InternalClientMessage, server_message::ServerMessage},
    };
    use espresso_types::PubKey;
    use futures::channel::mpsc::{self, Sender};
    use hotshot_types::traits::signature_key::BuilderSignatureKey;
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

        let public_key = PubKey::generated_from_seed_indexed([0; 32], 0).0;
        let (external_message_sender, _external_message_receiver) = mpsc::channel(10);
        let (_event_sender, event_receiver) = mpsc::channel(10);

        let node_validator_task_state = match super::create_node_validator_processing(
            super::NodeValidatorConfig {
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
            public_key,
            Some(event_receiver),
            Some(external_message_sender),
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
        tracing::info!("now listening on port 9000");

        app_serve_handle.await;

        drop(node_validator_task_state);
    }
}
