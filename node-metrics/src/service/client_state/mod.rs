use super::{
    client_id::ClientId, client_message::InternalClientMessage, data_state::DataState,
    server_message::ServerMessage,
};
use futures::{Stream, StreamExt};
use hotshot_query_service::explorer::{BlockDetail, ExplorerHistograms};
use sequencer::SeqTypes;
use std::sync::RwLockWriteGuard;
use std::{
    collections::{HashMap, HashSet},
    sync::{
        mpsc::{Receiver, SendError, Sender},
        Arc, RwLock,
    },
};

/// ClientState represents the service state of the connected clients.
/// It maintains and represents the connected clients, and their subscriptions.
// This state is meant to be managed in a separate thread that assists with
// processing and updating of individual client states.
pub struct ClientState {
    client_id: ClientId,
    sender: Sender<ServerMessage>,
}

impl ClientState {
    /// Create a new ClientState with the given client_id and receiver.
    pub fn new(client_id: ClientId, sender: Sender<ServerMessage>) -> Self {
        Self { client_id, sender }
    }

    /// Send a message to the client's consuming thread.
    pub fn send_message(&self, message: ServerMessage) -> Result<(), SendError<ServerMessage>> {
        self.sender.send(message)
    }
}

/// [ClientThreadState] represents the state of all of the active client
/// connections connected to the service. This state governs which clients
/// are connected, and what subscriptions they have setup.
pub struct ClientThreadState {
    clients: HashMap<ClientId, ClientState>,
    subscribed_latest_block: HashSet<ClientId>,
    subscribed_node_identity: HashSet<ClientId>,
    connection_id_counter: u128,
}

/// [drop_client_client_thread_state_write_guard] is a utility function for
/// cleaning up the [ClientThreadState]
fn drop_client_client_thread_state_write_guard(
    client_id: &ClientId,
    client_thread_state_write_guard: &mut RwLockWriteGuard<ClientThreadState>,
) -> Option<ClientState> {
    let client = client_thread_state_write_guard.clients.remove(client_id);
    client_thread_state_write_guard
        .subscribed_latest_block
        .remove(client_id);
    client_thread_state_write_guard
        .subscribed_node_identity
        .remove(client_id);

    client
}

/// [drop_client_no_lock_guard] is a utility function for cleaning up the [ClientThreadState]
/// when a client is detected as disconnected.
fn drop_client_no_lock_guard(
    client_id: &ClientId,
    client_thread_state: Arc<RwLock<ClientThreadState>>,
) -> Option<ClientState> {
    let mut client_thread_state_write_lock_guard = match client_thread_state.write() {
        Ok(lock) => lock,
        Err(_) => return None,
    };

    drop_client_client_thread_state_write_guard(
        client_id,
        &mut client_thread_state_write_lock_guard,
    )
}

/// [handle_client_message_connected] is a function that processes the client
/// message to connect a client to the service.
pub fn handle_client_message_connected(
    sender: Sender<ServerMessage>,
    client_thread_state: Arc<RwLock<ClientThreadState>>,
) -> Result<ClientId, ()> {
    let mut client_thread_state_write_lock_guard = match client_thread_state.write() {
        Ok(lock) => lock,
        Err(_) => return Err(()),
    };

    client_thread_state_write_lock_guard.connection_id_counter += 1;
    let client_id =
        ClientId::from_count(client_thread_state_write_lock_guard.connection_id_counter);

    client_thread_state_write_lock_guard.clients.insert(
        client_id.clone(),
        ClientState {
            client_id: client_id.clone(),
            sender: sender.clone(),
        },
    );

    // Explicitly unlock
    drop(client_thread_state_write_lock_guard);

    // Send the client their new id.
    if let Err(_) = sender.send(ServerMessage::YouAre(client_id.clone())) {
        // We need to remove drop the client now.
        drop_client_no_lock_guard(&client_id, client_thread_state.clone());
        return Err(());
    }

    Ok(client_id)
}

/// [handle_client_message_disconnected] is a function that processes the client
/// message to disconnect a client from the service.
pub fn handle_client_message_disconnected(
    client_id: ClientId,
    client_thread_state: Arc<RwLock<ClientThreadState>>,
) -> Result<(), ()> {
    // We might receive an implicit disconnect when attempting to
    // send a message, as the receiving channel might be closed.
    drop_client_no_lock_guard(&client_id, client_thread_state.clone());
    Ok(())
}

/// [handle_client_message_subscribe_latest_block] is a function that processes
/// the client message to subscribe to the latest block stream.
pub fn handle_client_message_subscribe_latest_block(
    client_id: ClientId,
    client_thread_state: Arc<RwLock<ClientThreadState>>,
) -> Result<(), ()> {
    let mut client_thread_state_write_lock_guard = match client_thread_state.write() {
        Ok(lock) => lock,
        Err(_) => return Err(()),
    };

    client_thread_state_write_lock_guard
        .subscribed_latest_block
        .insert(client_id);

    // Explicitly unlock
    drop(client_thread_state_write_lock_guard);
    Ok(())
}

/// [handle_client_message_subscribe_node_identity] is a function that processes
/// the client message to subscribe to the node identity stream.
pub fn handle_client_message_subscribe_node_identity(
    client_id: ClientId,
    client_thread_state: Arc<RwLock<ClientThreadState>>,
) -> Result<(), ()> {
    let mut client_thread_state_write_lock_guard = match client_thread_state.write() {
        Ok(lock) => lock,
        Err(_) => return Err(()),
    };

    client_thread_state_write_lock_guard
        .subscribed_node_identity
        .insert(client_id);

    // Explicitly unlock
    drop(client_thread_state_write_lock_guard);
    Ok(())
}

/// [handle_client_message_request_blocks_snapshot] is a function that processes
/// the client message request for a blocks snapshot.
pub fn handle_client_message_request_blocks_snapshot(
    client_id: ClientId,
    data_state: Arc<RwLock<DataState>>,
    client_thread_state: Arc<RwLock<ClientThreadState>>,
) -> Result<(), ()> {
    let client_thread_state_read_lock_guard = match client_thread_state.read() {
        Ok(lock) => lock,
        Err(_) => return Err(()),
    };

    let data_state_read_lock_guard = match data_state.read() {
        Ok(lock) => lock,
        Err(_) => {
            drop(client_thread_state_read_lock_guard);
            return Err(());
        }
    };

    let latest_blocks = data_state_read_lock_guard
        .latest_blocks
        .iter()
        .map(|block| BlockDetail {
            hash: block.hash,
            proposer_id: block.proposer_id,
            height: block.height,
            size: block.size,
            time: block.time,
            num_transactions: block.num_transactions,
            fee_recipient: block.fee_recipient,
            block_reward: block.block_reward.clone(),
        })
        .collect::<Vec<BlockDetail<SeqTypes>>>();

    if let Some(client) = client_thread_state_read_lock_guard.clients.get(&client_id) {
        if let Err(_) = client.send_message(ServerMessage::BlocksSnapshot(latest_blocks)) {
            drop_client_no_lock_guard(&client_id, client_thread_state.clone());
        }
    }

    drop(data_state_read_lock_guard);
    drop(client_thread_state_read_lock_guard);

    Ok(())
}

/// [handle_client_message_request_node_identity_snapshot] is a function that
/// processes the client message request for a node identity snapshot.
pub fn handle_client_message_request_node_identity_snapshot(
    client_id: ClientId,
    data_state: Arc<RwLock<DataState>>,
    client_thread_state: Arc<RwLock<ClientThreadState>>,
) -> Result<(), ()> {
    // Let's send the current Blocks Snapshot to the client
    let client_thread_state_read_lock_guard = match client_thread_state.read() {
        Ok(lock) => lock,
        Err(_) => return Err(()),
    };

    let data_state_read_lock_guard = match data_state.read() {
        Ok(lock) => lock,
        Err(_) => {
            drop(client_thread_state_read_lock_guard);
            return Err(());
        }
    };

    let client_result = client_thread_state_read_lock_guard.clients.get(&client_id);
    drop(data_state_read_lock_guard);
    if let Some(client) = client_result {
        if let Err(_) = client.send_message(ServerMessage::NodeIdentitySnapshot) {
            drop(client_thread_state_read_lock_guard);
            drop_client_no_lock_guard(&client_id, client_thread_state.clone());
            return Ok(());
        }

        drop_client_no_lock_guard(&client_id, client_thread_state.clone());
        return Ok(());
    }

    drop(client_thread_state_read_lock_guard);
    return Ok(());
}

/// [handle_client_message_request_histogram_snapshot] is a function that
/// processes the client message request for a histogram snapshot.
pub fn handle_client_message_request_histogram_snapshot(
    client_id: ClientId,
    data_state: Arc<RwLock<DataState>>,
    client_thread_state: Arc<RwLock<ClientThreadState>>,
) -> Result<(), ()> {
    // Let's send the current histogram data snapshot to the client
    let client_thread_state_read_lock_guard = match client_thread_state.read() {
        Ok(lock) => lock,
        Err(_) => return Err(()),
    };

    let data_state_read_lock_guard = match data_state.read() {
        Ok(lock) => lock,
        Err(_) => {
            drop(client_thread_state_read_lock_guard);
            return Err(());
        }
    };

    let histogram_data = ExplorerHistograms {
        block_size: data_state_read_lock_guard
            .latest_blocks
            .iter()
            .skip(1)
            .map(|block| block.size)
            .collect(),
        block_time: data_state_read_lock_guard
            .latest_blocks
            .iter()
            .skip(1)
            .zip(data_state_read_lock_guard.latest_blocks.iter())
            .map(|(block_i, block_i_sub_1)| {
                (block_i.time.0 - block_i_sub_1.time.0).whole_seconds() as u64
            })
            .collect(),
        block_transactions: data_state_read_lock_guard
            .latest_blocks
            .iter()
            .skip(1)
            .map(|block| block.num_transactions)
            .collect(),
        block_heights: data_state_read_lock_guard
            .latest_blocks
            .iter()
            .skip(1)
            .map(|block| block.height)
            .collect(),
    };
    drop(data_state_read_lock_guard);

    if let Some(client) = client_thread_state_read_lock_guard.clients.get(&client_id) {
        if let Err(_) = client.send_message(ServerMessage::HistogramSnapshot(histogram_data)) {
            drop(client_thread_state_read_lock_guard);
            drop_client_no_lock_guard(&client_id, client_thread_state.clone());
            return Ok(());
        }

        drop_client_no_lock_guard(&client_id, client_thread_state.clone());
        return Ok(());
    }

    drop(client_thread_state_read_lock_guard);
    Ok(())
}

/// [process_client_message] is a function that processes the client message
/// and processes the message accordingly.
///
/// The [DataState] is provided and is used only as a Read lock to distribute
/// the current state of the system to the clients upon request.
///
/// The [ClientThreadState] is provided as it needs to be updated with new
/// subscriptions / new connections depending on the incoming
/// [InternalClientMessage]
pub fn process_client_message(
    message: InternalClientMessage,
    data_state: Arc<RwLock<DataState>>,
    client_thread_state: Arc<RwLock<ClientThreadState>>,
) -> Result<(), ()> {
    match message {
        InternalClientMessage::Connected(sender) => {
            handle_client_message_connected(sender, client_thread_state).map(|_| ())
        }

        InternalClientMessage::Disconnected(client_id) => {
            handle_client_message_disconnected(client_id, client_thread_state)
        }

        InternalClientMessage::SubscribeLatestBlock(client_id) => {
            handle_client_message_subscribe_latest_block(client_id, client_thread_state)
        }

        InternalClientMessage::SubscribeNodeIdentity(client_id) => {
            handle_client_message_subscribe_node_identity(client_id, client_thread_state)
        }

        InternalClientMessage::RequestBlocksSnapshot(client_id) => {
            handle_client_message_request_blocks_snapshot(
                client_id,
                data_state,
                client_thread_state,
            )
        }

        InternalClientMessage::RequestNodeIdentitySnapshot(client_id) => {
            handle_client_message_request_node_identity_snapshot(
                client_id,
                data_state,
                client_thread_state,
            )
        }

        InternalClientMessage::RequestHistogramSnapshot(client_id) => {
            handle_client_message_request_histogram_snapshot(
                client_id,
                data_state,
                client_thread_state,
            )
        }
    }
}

/// [clone_block_detail] is a utility function that clones a [BlockDetail]
/// instance.
fn clone_block_detail(input: &BlockDetail<SeqTypes>) -> BlockDetail<SeqTypes> {
    BlockDetail {
        hash: input.hash.clone(),
        proposer_id: input.proposer_id.clone(),
        height: input.height,
        size: input.size,
        time: input.time,
        num_transactions: input.num_transactions,
        fee_recipient: input.fee_recipient.clone(),
        block_reward: input.block_reward.clone(),
    }
}

/// [handle_received_block_detail] is a function that processes received Block
/// details and will attempt to distribute the message to all of the clients
/// that are subscribed to the latest block stream.
fn handle_received_block_detail(
    client_thread_state: Arc<RwLock<ClientThreadState>>,
    block_detail: BlockDetail<SeqTypes>,
) -> Result<(), ()> {
    let client_thread_state_read_lock_guard = match client_thread_state.read() {
        Ok(lock) => lock,
        Err(_) => return Err(()),
    };

    // These are the clients who are subscribed to the latest blocks, that
    // have an active ClientState within the system.
    let latest_block_subscribers = client_thread_state_read_lock_guard
        .subscribed_latest_block
        .iter()
        .map(|client_id| {
            (
                client_id,
                (&client_thread_state_read_lock_guard)
                    .clients
                    .get(client_id),
            )
        })
        .filter(|(_, client)| client.is_some());

    // We collect the results of sending the latest block to the clients.
    let client_send_results = latest_block_subscribers.map(|(client_id, client)| {
        // This is guaranteed to be a some now
        let client = client.unwrap();
        let send_result = client.send_message(ServerMessage::LatestBlock(clone_block_detail(
            &block_detail,
        )));
        (client_id, send_result)
    });

    // These are the clients we failed to send the message to.  We copy these
    // here so we can drop our read lock.
    let failed_client_sends = client_send_results
        .filter(|(_, send_result)| send_result.is_err())
        .map(|(client_id, _)| client_id.clone())
        .collect::<Vec<_>>();

    // Explicitly Drop the read lock.
    drop(client_thread_state_read_lock_guard);

    if failed_client_sends.is_empty() {
        return Ok(());
    }

    // Let's acquire our write lock
    let mut client_thread_state_write_lock_guard = match client_thread_state.write() {
        Ok(lock) => lock,
        Err(_) => return Err(()),
    };

    // We want to drop all of the failed clients.
    // There's an optimization to be had here
    for client_id in failed_client_sends {
        drop_client_client_thread_state_write_guard(
            &client_id,
            &mut client_thread_state_write_lock_guard,
        );
    }

    drop(client_thread_state_write_lock_guard);

    Ok(())
}

/// [process_client_handling_thread] is a function that processes the client
/// handling thread. This thread is responsible for managing the state of the
/// connected clients, and their subscriptions.
pub fn process_client_handling_thread(
    receiver: Receiver<InternalClientMessage>,
    data_state: Arc<RwLock<DataState>>,
    client_thread_state: Arc<RwLock<ClientThreadState>>,
) {
    while let Ok(message) = receiver.recv() {
        if let Err(_) =
            process_client_message(message, data_state.clone(), client_thread_state.clone())
        {
            break;
        }
    }
}

/// [process_client_handling_stream] is a function that processes the client
/// handling stream. This stream is responsible for managing the state of the
/// connected clients, and their subscriptions.
pub async fn process_client_handling_stream<S>(
    mut stream: S,
    data_state: Arc<RwLock<DataState>>,
    client_thread_state: Arc<RwLock<ClientThreadState>>,
) where
    S: Stream<Item = InternalClientMessage> + Unpin,
{
    while let Some(message) = stream.next().await {
        if let Err(_) =
            process_client_message(message, data_state.clone(), client_thread_state.clone())
        {
            break;
        }
    }
}

/// [process_distribute_client_handling_thread] is a function that processes the
/// the [Receiver] of incoming [BlockDetail] and distributes them to all
/// subscribed clients.
pub fn process_distribute_client_handling_thread(
    client_thread_state: Arc<RwLock<ClientThreadState>>,
    block_detail_receiver: Receiver<BlockDetail<SeqTypes>>,
) {
    while let Ok(block_detail) = block_detail_receiver.recv() {
        if let Err(_) = handle_received_block_detail(client_thread_state.clone(), block_detail) {
            break;
        }
    }
}

/// [process_distribute_client_handling_stream] is a function that processes the
/// the [Stream] of incoming [BlockDetail] and distributes them to all
/// subscribed clients.
pub async fn process_distribute_client_handling_stream<S>(
    client_thread_state: Arc<RwLock<ClientThreadState>>,
    mut stream: S,
) where
    S: Stream<Item = BlockDetail<SeqTypes>> + Unpin,
{
    while let Some(block_detail) = stream.next().await {
        if let Err(_) = handle_received_block_detail(client_thread_state.clone(), block_detail) {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{process_client_handling_thread, ClientThreadState};
    use crate::service::{
        client_id::ClientId, client_message::InternalClientMessage, data_state::DataState,
        server_message::ServerMessage,
    };
    use circular_buffer::CircularBuffer;
    use std::{
        collections::{HashMap, HashSet},
        sync::{mpsc, Arc, RwLock},
        thread,
    };

    #[test]
    fn test_process_client_handling_stream() {
        // Woo hoo
        let (server_message_sender, server_message_receiver) = mpsc::channel();
        let (internal_client_message_sender, internal_client_message_receiver) = mpsc::channel();

        let mut clients = HashMap::with_capacity(1);

        let mut subscribed_node_identity = HashSet::with_capacity(1);
        let mut subscribed_latest_block = HashSet::with_capacity(1);

        let client_thread_state = ClientThreadState {
            clients,
            subscribed_latest_block,
            subscribed_node_identity,
            connection_id_counter: 1,
        };

        let client_thread_state = Arc::new(RwLock::new(client_thread_state));
        let data_state = Arc::new(RwLock::new(DataState {
            latest_voters: CircularBuffer::new(),
            latest_blocks: CircularBuffer::new(),
            stake_table: Default::default(),
            node_identity: vec![],
        }));

        thread::spawn(move || {
            process_client_handling_thread(
                internal_client_message_receiver,
                data_state.clone(),
                client_thread_state.clone(),
            )
        });

        // Send a Connected Message to the server
        assert_eq!(
            internal_client_message_sender
                .send(InternalClientMessage::Connected(server_message_sender)),
            Ok(())
        );

        assert_eq!(
            server_message_receiver.recv(),
            Ok(ServerMessage::YouAre(ClientId::from_count(2))),
        );
    }
}
