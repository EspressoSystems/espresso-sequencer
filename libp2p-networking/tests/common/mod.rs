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
    channel::{bounded, RecvError},
    logging::{setup_backtrace, setup_logging},
};
use futures::{future::join_all, Future, FutureExt};
use libp2p::{identity::Keypair, Multiaddr};
use libp2p_identity::PeerId;
use libp2p_networking::network::{
    network_node_handle_error::NodeConfigSnafu, spawn_network_node, NetworkEvent,
    NetworkNodeConfigBuilder, NetworkNodeHandle, NetworkNodeHandleError, NetworkNodeReceiver,
    NetworkNodeType,
};
use snafu::{ResultExt, Snafu};
use tracing::{info, instrument, warn};

#[derive(Clone, Debug)]
pub(crate) struct HandleWithState<S: Debug + Default + Send> {
    pub(crate) handle: Arc<NetworkNodeHandle>,
    pub(crate) state: Arc<SubscribableMutex<S>>,
}

/// Spawn a handler `F` that will be notified every time a new [`NetworkEvent`] arrives.
///
/// # Panics
///
/// Will panic if a handler is already spawned
pub fn spawn_handler<F, RET, S>(
    handle_and_state: HandleWithState<S>,
    mut receiver: NetworkNodeReceiver,
    cb: F,
) -> impl Future
where
    F: Fn(NetworkEvent, HandleWithState<S>) -> RET + Sync + Send + 'static,
    RET: Future<Output = Result<(), NetworkNodeHandleError>> + Send + 'static,
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
pub async fn test_bed<S: 'static + Send + Default + Debug + Clone, F, FutF, G: Clone, FutG>(
    run_test: F,
    client_handler: G,
    num_nodes: usize,
    num_of_bootstrap: usize,
    timeout: Duration,
) where
    FutF: Future<Output = ()>,
    FutG: Future<Output = Result<(), NetworkNodeHandleError>> + 'static + Send + Sync,
    F: FnOnce(Vec<HandleWithState<S>>, Duration) -> FutF,
    G: Fn(NetworkEvent, HandleWithState<S>) -> FutG + 'static + Send + Sync,
{
    setup_logging();
    setup_backtrace();

    let mut kill_switches = Vec::new();
    // NOTE we want this to panic if we can't spin up the swarms.
    // that amounts to a failed test.
    let handles_and_receivers = spin_up_swarms::<S>(num_nodes, timeout, num_of_bootstrap)
        .await
        .unwrap();

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

fn gen_peerid_map(handles: &[Arc<NetworkNodeHandle>]) -> HashMap<PeerId, usize> {
    let mut r_val = HashMap::new();
    for handle in handles {
        r_val.insert(handle.peer_id(), handle.id());
    }
    r_val
}

/// print the connections for each handle in `handles`
/// useful for debugging
pub async fn print_connections(handles: &[Arc<NetworkNodeHandle>]) {
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
pub async fn spin_up_swarms<S: Debug + Default + Send>(
    num_of_nodes: usize,
    timeout_len: Duration,
    num_bootstrap: usize,
) -> Result<Vec<(HandleWithState<S>, NetworkNodeReceiver)>, TestError<S>> {
    let mut handles = Vec::new();
    let mut bootstrap_addrs = Vec::<(PeerId, Multiaddr)>::new();
    let mut connecting_futs = Vec::new();
    // should never panic unless num_nodes is 0
    let replication_factor = NonZeroUsize::new(num_of_nodes - 1).unwrap();

    for i in 0..num_bootstrap {
        let mut config = NetworkNodeConfigBuilder::default();
        let identity = Keypair::generate_ed25519();
        // let start_port = 5000;
        // NOTE use this if testing locally and want human readable ports
        // as opposed to random ports. These are harder to track
        // especially since the "listener"/inbound connection sees a different
        // port
        // let addr = Multiaddr::from_str(&format!("/ip4/127.0.0.1/udp/{}/quic-v1", start_port + i)).unwrap();

        let addr = Multiaddr::from_str("/ip4/127.0.0.1/udp/0/quic-v1").unwrap();
        config
            .identity(identity)
            .replication_factor(replication_factor)
            .node_type(NetworkNodeType::Bootstrap)
            .to_connect_addrs(HashSet::default())
            .bound_addr(Some(addr))
            .ttl(None)
            .republication_interval(None)
            .server_mode(true);
        let config = config
            .build()
            .context(NodeConfigSnafu)
            .context(HandleSnafu)?;
        let (rx, node) = spawn_network_node(config.clone(), i).await.unwrap();
        let node = Arc::new(node);
        let addr = node.listen_addr();
        info!("listen addr for {} is {:?}", i, addr);
        bootstrap_addrs.push((node.peer_id(), addr));
        connecting_futs.push({
            let node = node.clone();
            async move {
                node.begin_bootstrap().await?;
                node.lookup_pid(PeerId::random()).await
            }
            .boxed_local()
        });
        let node_with_state = HandleWithState {
            handle: node.clone(),
            state: Arc::default(),
        };
        handles.push((node_with_state, rx));
    }

    for j in 0..(num_of_nodes - num_bootstrap) {
        let addr = Multiaddr::from_str("/ip4/127.0.0.1/udp/0/quic-v1").unwrap();
        // NOTE use this if testing locally and want human readable ports
        // let addr = Multiaddr::from_str(&format!(
        //     "/ip4/127.0.0.1/udp/{}/quic-v1",
        //     start_port + num_bootstrap + j
        // )).unwrap();
        let regular_node_config = NetworkNodeConfigBuilder::default()
            .node_type(NetworkNodeType::Regular)
            .replication_factor(replication_factor)
            .bound_addr(Some(addr.clone()))
            .to_connect_addrs(HashSet::default())
            .server_mode(true)
            .build()
            .context(NodeConfigSnafu)
            .context(HandleSnafu)?;
        let (rx, node) = spawn_network_node(regular_node_config.clone(), j + num_bootstrap)
            .await
            .unwrap();

        let node = Arc::new(node);
        connecting_futs.push({
            let node = node.clone();
            async move {
                node.begin_bootstrap().await?;
                node.lookup_pid(PeerId::random()).await
            }
            .boxed_local()
        });
        let node_with_state = HandleWithState {
            handle: node.clone(),
            state: Arc::default(),
        };
        handles.push((node_with_state, rx));
    }
    info!("BSADDRS ARE: {:?}", bootstrap_addrs);

    info!(
        "known nodes: {:?}",
        bootstrap_addrs
            .iter()
            .map(|(a, b)| (Some(*a), b.clone()))
            .collect::<Vec<_>>()
    );

    for (handle, _) in &handles[0..num_of_nodes] {
        let to_share = bootstrap_addrs.clone();
        handle
            .handle
            .add_known_peers(to_share)
            .await
            .context(HandleSnafu)?;
    }

    let res = join_all(connecting_futs.into_iter()).await;
    let mut failing_nodes = Vec::new();
    for (idx, a_node) in res.iter().enumerate() {
        if a_node.is_err() {
            failing_nodes.push(idx);
        }
    }
    if !failing_nodes.is_empty() {
        return Err(TestError::SpinupTimeout { failing_nodes });
    }

    for (handle, _) in &handles {
        handle
            .handle
            .subscribe("global".to_string())
            .await
            .context(HandleSnafu)?;
    }

    async_sleep(Duration::from_secs(5)).await;

    Ok(handles)
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum TestError<S: Debug> {
    #[snafu(display("Channel error {source:?}"))]
    Recv {
        source: RecvError,
    },
    #[snafu(display(
        "Timeout while running direct message round. Timed out when {requester} dmed {requestee}"
    ))]
    DirectTimeout {
        requester: usize,
        requestee: usize,
    },
    #[snafu(display("Timeout while running gossip round. Timed out on {failing:?}."))]
    GossipTimeout {
        failing: Vec<usize>,
    },
    #[snafu(display(
        "Inconsistent state while running test. Expected {expected:?}, got {actual:?} on node {id}"
    ))]
    State {
        id: usize,
        expected: S,
        actual: S,
    },
    #[snafu(display("Handler error while running test. {source:?}"))]
    Handle {
        source: NetworkNodeHandleError,
    },
    #[snafu(display("Failed to spin up nodes. Hit timeout instead. {failing_nodes:?}"))]
    SpinupTimeout {
        failing_nodes: Vec<usize>,
    },
    DHTTimeout,
}
