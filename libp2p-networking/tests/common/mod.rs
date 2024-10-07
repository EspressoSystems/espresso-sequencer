// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    num::NonZeroUsize,
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use async_compatibility_layer::{
    art::{async_sleep, async_spawn},
    async_primitives::subscribable_mutex::SubscribableMutex,
    channel::bounded,
    logging::{setup_backtrace, setup_logging},
};
use futures::{future::join_all, Future, FutureExt};
use hotshot_types::traits::{network::NetworkError, signature_key::SignatureKey};
use libp2p::Multiaddr;
use libp2p_identity::PeerId;
use libp2p_networking::network::{
    spawn_network_node, NetworkEvent, NetworkNodeConfigBuilder, NetworkNodeHandle,
    NetworkNodeReceiver,
};
use thiserror::Error;
use tracing::{instrument, warn};

#[derive(Clone, Debug)]
pub(crate) struct HandleWithState<S: Debug + Default + Send, K: SignatureKey + 'static> {
    pub(crate) handle: Arc<NetworkNodeHandle<K>>,
    pub(crate) state: Arc<SubscribableMutex<S>>,
}

/// Spawn a handler `F` that will be notified every time a new [`NetworkEvent`] arrives.
///
/// # Panics
///
/// Will panic if a handler is already spawned
pub fn spawn_handler<F, RET, S, K: SignatureKey + 'static>(
    handle_and_state: HandleWithState<S, K>,
    mut receiver: NetworkNodeReceiver,
    cb: F,
) -> impl Future
where
    F: Fn(NetworkEvent, HandleWithState<S, K>) -> RET + Sync + Send + 'static,
    RET: Future<Output = Result<(), NetworkError>> + Send + 'static,
    S: Debug + Default + Send + Clone + 'static,
{
    async_spawn(async move {
        let Some(mut kill_switch) = receiver.take_kill_switch() else {
            tracing::error!(
                "`spawn_handle` was called on a network handle that was already closed"
            );
            return;
        };
        let mut next_msg = receiver.recv().boxed();
        let mut kill_switch = kill_switch.recv().boxed();
        loop {
            match futures::future::select(next_msg, kill_switch).await {
                futures::future::Either::Left((incoming_message, other_stream)) => {
                    let incoming_message = match incoming_message {
                        Ok(msg) => msg,
                        Err(e) => {
                            tracing::warn!(?e, "NetworkNodeHandle::spawn_handle was unable to receive more messages");
                            return;
                        }
                    };
                    if let Err(e) = cb(incoming_message, handle_and_state.clone()).await {
                        tracing::error!(?e, "NetworkNodeHandle::spawn_handle returned an error");
                        return;
                    }

                    // re-set the `kill_switch` for the next loop
                    kill_switch = other_stream;
                    // re-set `receiver.recv()` for the next loop
                    next_msg = receiver.recv().boxed();
                }
                futures::future::Either::Right(_) => {
                    return;
                }
            }
        }
    })
}

/// General function to spin up testing infra
/// perform tests by calling `run_test`
/// then cleans up tests
/// # Panics
/// Panics if unable to:
/// - Initialize logging
/// - Initialize network nodes
/// - Kill network nodes
/// - A test assertion fails
pub async fn test_bed<
    S: 'static + Send + Default + Debug + Clone,
    F,
    FutF,
    G,
    FutG,
    K: SignatureKey + 'static,
>(
    run_test: F,
    client_handler: G,
    num_nodes: usize,
    timeout: Duration,
) where
    FutF: Future<Output = ()>,
    FutG: Future<Output = Result<(), NetworkError>> + 'static + Send + Sync,
    F: FnOnce(Vec<HandleWithState<S, K>>, Duration) -> FutF,
    G: Fn(NetworkEvent, HandleWithState<S, K>) -> FutG + 'static + Send + Sync + Clone,
{
    setup_logging();
    setup_backtrace();

    let mut kill_switches = Vec::new();
    // NOTE we want this to panic if we can't spin up the swarms.
    // that amounts to a failed test.
    let handles_and_receivers = spin_up_swarms::<S, K>(num_nodes, timeout).await.unwrap();

    let (handles, receivers): (Vec<_>, Vec<_>) = handles_and_receivers.into_iter().unzip();
    let mut handler_futures = Vec::new();
    for (i, mut rx) in receivers.into_iter().enumerate() {
        let (kill_tx, kill_rx) = bounded(1);
        let handle = &handles[i];
        kill_switches.push(kill_tx);
        rx.set_kill_switch(kill_rx);
        let handler_fut = spawn_handler(handle.clone(), rx, client_handler.clone());
        handler_futures.push(handler_fut);
    }

    run_test(handles.clone(), timeout).await;

    // cleanup
    for handle in handles {
        handle.handle.shutdown().await.unwrap();
    }
    for switch in kill_switches {
        let _ = switch.send(()).await;
    }

    for fut in handler_futures {
        fut.await;
    }
}

fn gen_peerid_map<K: SignatureKey + 'static>(
    handles: &[Arc<NetworkNodeHandle<K>>],
) -> HashMap<PeerId, usize> {
    let mut r_val = HashMap::new();
    for handle in handles {
        r_val.insert(handle.peer_id(), handle.id());
    }
    r_val
}

/// print the connections for each handle in `handles`
/// useful for debugging
pub async fn print_connections<K: SignatureKey + 'static>(handles: &[Arc<NetworkNodeHandle<K>>]) {
    let m = gen_peerid_map(handles);
    warn!("PRINTING CONNECTION STATES");
    for handle in handles {
        warn!(
            "peer {}, connected to {:?}",
            handle.id(),
            handle
                .connected_pids()
                .await
                .unwrap()
                .iter()
                .map(|pid| m.get(pid).unwrap())
                .collect::<Vec<_>>()
        );
    }
}

/// Spins up `num_of_nodes` nodes, connects them to each other
/// and waits for connections to propagate to all nodes.
#[allow(clippy::type_complexity)]
#[instrument]
pub async fn spin_up_swarms<S: Debug + Default + Send, K: SignatureKey + 'static>(
    num_of_nodes: usize,
    timeout_len: Duration,
) -> Result<Vec<(HandleWithState<S, K>, NetworkNodeReceiver)>, TestError<S>> {
    let mut handles = Vec::new();
    let mut node_addrs = Vec::<(PeerId, Multiaddr)>::new();
    let mut connecting_futs = Vec::new();
    // should never panic unless num_nodes is 0
    let replication_factor = NonZeroUsize::new(num_of_nodes - 1).unwrap();

    for i in 0..num_of_nodes {
        // Get an unused port
        let port = portpicker::pick_unused_port().expect("Failed to get an unused port");

        // Use the port to create a Multiaddr
        let addr =
            Multiaddr::from_str(format!("/ip4/127.0.0.1/udp/{port}/quic-v1").as_str()).unwrap();

        let config = NetworkNodeConfigBuilder::default()
            .replication_factor(replication_factor)
            .bind_address(Some(addr.clone()))
            .to_connect_addrs(HashSet::default())
            .build()
            .map_err(|e| TestError::ConfigError(format!("failed to build network node: {e}")))?;

        let (rx, node) = spawn_network_node(config.clone(), i).await.unwrap();

        // Add ourselves to the list of node addresses to connect to
        node_addrs.push((node.peer_id(), addr));

        let node = Arc::new(node);
        connecting_futs.push({
            let node = Arc::clone(&node);
            async move {
                node.begin_bootstrap().await?;
                node.lookup_pid(PeerId::random()).await
            }
            .boxed_local()
        });
        let node_with_state = HandleWithState {
            handle: Arc::clone(&node),
            state: Arc::default(),
        };
        handles.push((node_with_state, rx));
    }

    for (handle, _) in &handles[0..num_of_nodes] {
        let to_share = node_addrs.clone();
        handle
            .handle
            .add_known_peers(to_share)
            .await
            .map_err(|e| TestError::HandleError(format!("failed to add known peers: {e}")))?;
    }

    let res = join_all(connecting_futs.into_iter()).await;
    let mut failing_nodes = Vec::new();
    for (idx, a_node) in res.iter().enumerate() {
        if a_node.is_err() {
            failing_nodes.push(idx);
        }
    }
    if !failing_nodes.is_empty() {
        return Err(TestError::Timeout(failing_nodes, "spinning up".to_string()));
    }

    for (handle, _) in &handles {
        handle
            .handle
            .subscribe("global".to_string())
            .await
            .map_err(|e| TestError::HandleError(format!("failed to subscribe: {e}")))?;
    }

    async_sleep(Duration::from_secs(5)).await;

    Ok(handles)
}

#[derive(Debug, Error)]
pub enum TestError<S: Debug> {
    #[error("Error with network node handle: {0}")]
    HandleError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("The following nodes timed out: {0:?} while {1}")]
    Timeout(Vec<usize>, String),

    #[error(
        "Inconsistent state while running test. Expected {expected:?}, got {actual:?} on node {id}"
    )]
    InconsistentState { id: usize, expected: S, actual: S },
}
