#![allow(clippy::panic)]

mod common;
use crate::common::print_connections;
use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::RwLock;
use common::{test_bed, HandleSnafu, HandleWithState, TestError};
use hotshot_types::constants::{Version01, STATIC_VER_0_1};
use libp2p_networking::network::{NetworkEvent, NetworkNodeHandleError};
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::{fmt::Debug, sync::Arc, time::Duration};
use tracing::{debug, error, info, instrument, warn};
use versioned_binary_serialization::{BinarySerializer, Serializer};

#[cfg(async_executor_impl = "async-std")]
use async_std::prelude::StreamExt;
#[cfg(async_executor_impl = "tokio")]
use tokio_stream::StreamExt;
#[cfg(not(any(async_executor_impl = "async-std", async_executor_impl = "tokio")))]
compile_error! {"Either config option \"async-std\" or \"tokio\" must be enabled for this crate."}

pub type CounterState = u32;

const NUM_ROUNDS: usize = 100;

const TOTAL_NUM_PEERS_COVERAGE: usize = 10;
const NUM_OF_BOOTSTRAP_COVERAGE: usize = 5;
const TIMEOUT_COVERAGE: Duration = Duration::from_secs(120);

const TOTAL_NUM_PEERS_STRESS: usize = 100;
const NUM_OF_BOOTSTRAP_STRESS: usize = 25;
const TIMEOUT_STRESS: Duration = Duration::from_secs(60);

const DHT_KV_PADDING: usize = 1024;

/// Message types. We can either
/// - increment the Counter
/// - request a counter value
/// - reply with a counter value
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum CounterMessage {
    IncrementCounter {
        from: CounterState,
        to: CounterState,
    },
    AskForCounter,
    MyCounterIs(CounterState),
    Noop,
}

/// Given a slice of handles assumed to be larger than 0,
/// chooses one
/// # Panics
/// panics if handles is of length 0
fn get_random_handle<S: Debug + Default + Send + Clone>(
    handles: &[HandleWithState<S>],
    rng: &mut dyn rand::RngCore,
) -> HandleWithState<S> {
    handles.iter().choose(rng).unwrap().clone()
}

/// event handler for events from the swarm
/// - updates state based on events received
/// - replies to direct messages
#[instrument]
pub async fn counter_handle_network_event(
    event: NetworkEvent,
    handle: HandleWithState<CounterState>,
) -> Result<(), NetworkNodeHandleError> {
    use CounterMessage::*;
    use NetworkEvent::*;
    match event {
        IsBootstrapped | NetworkEvent::ResponseRequested(..) => {}
        GossipMsg(m) | DirectResponse(m, _) => {
            if let Ok(msg) = Serializer::<Version01>::deserialize::<CounterMessage>(&m) {
                match msg {
                    // direct message only
                    MyCounterIs(c) => {
                        handle.state.modify(|s| *s = c).await;
                    }
                    // gossip message only
                    IncrementCounter { from, to, .. } => {
                        handle
                            .state
                            .modify(|s| {
                                if *s == from {
                                    *s = to;
                                }
                            })
                            .await;
                    }
                    // only as a response
                    AskForCounter | Noop => {}
                }
            } else {
                error!("FAILED TO DESERIALIZE MSG {:?}", m);
            }
        }
        DirectRequest(m, _, chan) => {
            if let Ok(msg) = Serializer::<Version01>::deserialize::<CounterMessage>(&m) {
                match msg {
                    // direct message request
                    IncrementCounter { from, to, .. } => {
                        handle
                            .state
                            .modify(|s| {
                                if *s == from {
                                    *s = to;
                                }
                            })
                            .await;
                        handle
                            .handle
                            .direct_response(chan, &CounterMessage::Noop, STATIC_VER_0_1)
                            .await?;
                    }
                    // direct message response
                    AskForCounter => {
                        let response = MyCounterIs(handle.state.copied().await);
                        handle
                            .handle
                            .direct_response(chan, &response, STATIC_VER_0_1)
                            .await?;
                    }
                    MyCounterIs(_) => {
                        handle
                            .handle
                            .direct_response(chan, &CounterMessage::Noop, STATIC_VER_0_1)
                            .await?;
                    }
                    Noop => {
                        handle
                            .handle
                            .direct_response(chan, &CounterMessage::Noop, STATIC_VER_0_1)
                            .await?;
                    }
                }
            }
        }
    };
    Ok(())
}

/// `requester_handle` asks for `requestee_handle`'s state,
/// and then `requester_handle` updates its state to equal `requestee_handle`.
/// # Panics
/// on error
#[allow(clippy::similar_names)]
async fn run_request_response_increment<'a>(
    requester_handle: HandleWithState<CounterState>,
    requestee_handle: HandleWithState<CounterState>,
    timeout: Duration,
) -> Result<(), TestError<CounterState>> {
    async move {
        let new_state = requestee_handle.state.copied().await;

        // set up state change listener
        #[cfg(async_executor_impl = "async-std")]
        let mut stream = requester_handle.state.wait_timeout_until_with_trigger(timeout, move |state| *state == new_state);
        #[cfg(async_executor_impl = "tokio")]
        let mut stream = Box::pin(
            requester_handle.state.wait_timeout_until_with_trigger(timeout, move |state| *state == new_state),
        );
        #[cfg(not(any(async_executor_impl = "async-std", async_executor_impl = "tokio")))]
        compile_error! {"Either config option \"async-std\" or \"tokio\" must be enabled for this crate."}

        let requestee_pid = requestee_handle.handle.peer_id();

        match stream.next().await.unwrap() {
            Ok(()) => {}
            Err(e) => {error!("timed out waiting for {requestee_pid:?} to update state: {e}");
            std::process::exit(-1)},
        }
        requester_handle.handle
            .direct_request(requestee_pid, &CounterMessage::AskForCounter, STATIC_VER_0_1)
            .await
            .context(HandleSnafu)?;
        match stream.next().await.unwrap() {
            Ok(()) => {}
            Err(e) => {error!("timed out waiting for {requestee_pid:?} to update state: {e}");
            std::process::exit(-1)},        }

        let s1 = requester_handle.state.copied().await;

        // sanity check
        if s1 == new_state {
            Ok(())
        } else {
            Err(TestError::State {
                id: requester_handle.handle.id(),
                expected: new_state,
                actual: s1,
            })
        }
    }
    .await
}

/// broadcasts `msg` from a randomly chosen handle
/// then asserts that all nodes match `new_state`
async fn run_gossip_round(
    handles: &[HandleWithState<CounterState>],
    msg: CounterMessage,
    new_state: CounterState,
    timeout_duration: Duration,
) -> Result<(), TestError<CounterState>> {
    let mut rng = rand::thread_rng();
    let msg_handle = get_random_handle(handles, &mut rng);
    msg_handle.state.modify(|s| *s = new_state).await;

    let mut futs = Vec::new();

    let len = handles.len();
    for handle in handles {
        // already modified, so skip msg_handle
        if handle.handle.peer_id() != msg_handle.handle.peer_id() {
            let stream = handle
                .state
                .wait_timeout_until_with_trigger(timeout_duration, |state| *state == new_state);
            futs.push(Box::pin(stream));
        }
    }

    #[cfg(async_executor_impl = "async-std")]
    let mut merged_streams = futures::stream::select_all(futs);
    #[cfg(async_executor_impl = "tokio")]
    let mut merged_streams = Box::pin(futures::stream::select_all(futs));
    #[cfg(not(any(async_executor_impl = "async-std", async_executor_impl = "tokio")))]
    compile_error! {"Either config option \"async-std\" or \"tokio\" must be enabled for this crate."}

    // make sure all are ready/listening
    for i in 0..len - 1 {
        // unwrap is okay because stream must have 2 * (len - 1) elements
        match merged_streams.next().await.unwrap() {
            Ok(()) => {}
            Err(e) => {
                error!("timed out waiting for handle {i:?} to subscribe to state events: {e}");
                std::process::exit(-1)
            }
        }
    }

    msg_handle
        .handle
        .gossip("global".to_string(), &msg, STATIC_VER_0_1)
        .await
        .context(HandleSnafu)?;

    for _ in 0..len - 1 {
        // wait for all events to finish
        // then check for failures
        let _ = merged_streams.next().await;
    }

    let mut failing = Vec::new();
    for handle in handles {
        let handle_state = handle.state.copied().await;
        if handle_state != new_state {
            failing.push(handle.handle.id());
            println!("state: {handle_state:?}, expected: {new_state:?}");
        }
    }
    if !failing.is_empty() {
        let nodes = handles
            .iter()
            .cloned()
            .map(|h| h.handle)
            .collect::<Vec<_>>();
        print_connections(nodes.as_slice()).await;
        return Err(TestError::GossipTimeout { failing });
    }

    Ok(())
}

async fn run_intersperse_many_rounds(
    handles: Vec<HandleWithState<CounterState>>,
    timeout: Duration,
) {
    for i in 0..u32::try_from(NUM_ROUNDS).unwrap() {
        if i % 2 == 0 {
            run_request_response_increment_all(&handles, timeout).await;
        } else {
            run_gossip_rounds(&handles, 1, i, timeout).await;
        }
    }
    for h in handles {
        assert_eq!(h.state.copied().await, u32::try_from(NUM_ROUNDS).unwrap());
    }
}

async fn run_dht_many_rounds(handles: Vec<HandleWithState<CounterState>>, timeout: Duration) {
    run_dht_rounds(&handles, timeout, 0, NUM_ROUNDS).await;
}

async fn run_dht_one_round(handles: Vec<HandleWithState<CounterState>>, timeout: Duration) {
    run_dht_rounds(&handles, timeout, 0, 1).await;
}

async fn run_request_response_many_rounds(
    handles: Vec<HandleWithState<CounterState>>,
    timeout: Duration,
) {
    for _i in 0..NUM_ROUNDS {
        run_request_response_increment_all(&handles, timeout).await;
    }
    for h in handles {
        assert_eq!(h.state.copied().await, u32::try_from(NUM_ROUNDS).unwrap());
    }
}

/// runs one round of request response
/// # Panics
/// on error
async fn run_request_response_one_round(
    handles: Vec<HandleWithState<CounterState>>,
    timeout: Duration,
) {
    run_request_response_increment_all(&handles, timeout).await;
    for h in handles {
        assert_eq!(h.state.copied().await, 1);
    }
}

/// runs multiple rounds of gossip
/// # Panics
/// on error
async fn run_gossip_many_rounds(handles: Vec<HandleWithState<CounterState>>, timeout: Duration) {
    run_gossip_rounds(&handles, NUM_ROUNDS, 0, timeout).await;
}

/// runs one round of gossip
/// # Panics
/// on error
async fn run_gossip_one_round(handles: Vec<HandleWithState<CounterState>>, timeout: Duration) {
    run_gossip_rounds(&handles, 1, 0, timeout).await;
}

/// runs many rounds of dht
/// # Panics
/// on error
async fn run_dht_rounds(
    handles: &[HandleWithState<CounterState>],
    timeout: Duration,
    starting_val: usize,
    num_rounds: usize,
) {
    let mut rng = rand::thread_rng();
    for i in 0..num_rounds {
        debug!("begin round {}", i);
        let msg_handle = get_random_handle(handles, &mut rng);
        let mut key = vec![0; DHT_KV_PADDING];
        let inc_val = u8::try_from(starting_val + i).unwrap();
        key.push(inc_val);
        let mut value = vec![0; DHT_KV_PADDING];
        value.push(inc_val);

        // put the key
        msg_handle
            .handle
            .put_record(&key, &value, STATIC_VER_0_1)
            .await
            .unwrap();

        // get the key from the other nodes
        for handle in handles {
            let result: Result<Vec<u8>, NetworkNodeHandleError> = handle
                .handle
                .get_record_timeout(&key, timeout, STATIC_VER_0_1)
                .await;
            match result {
                Err(e) => {
                    error!("DHT error {e:?} during GET");
                    std::process::exit(-1);
                }
                Ok(v) => {
                    assert_eq!(v, value);
                }
            }
        }
    }
}

/// runs `num_rounds` of message broadcast, incrementing the state of all nodes each broadcast
async fn run_gossip_rounds(
    handles: &[HandleWithState<CounterState>],
    num_rounds: usize,
    starting_state: CounterState,
    timeout: Duration,
) {
    let mut old_state = starting_state;
    for i in 0..num_rounds {
        info!("running gossip round {}", i);
        let new_state = old_state + 1;
        let msg = CounterMessage::IncrementCounter {
            from: old_state,
            to: new_state,
        };
        run_gossip_round(handles, msg, new_state, timeout)
            .await
            .unwrap();
        old_state = new_state;
    }
}

/// chooses a random handle from `handles`
/// increments its state by 1,
/// then has all other peers request its state
/// and update their state to the recv'ed state
#[allow(clippy::similar_names)]
async fn run_request_response_increment_all(
    handles: &[HandleWithState<CounterState>],
    timeout: Duration,
) {
    let mut rng = rand::thread_rng();
    let requestee_handle = get_random_handle(handles, &mut rng);
    requestee_handle.state.modify(|s| *s += 1).await;
    info!("RR REQUESTEE IS {:?}", requestee_handle.handle.peer_id());
    let mut futs = Vec::new();
    for handle in handles {
        if handle
            .handle
            .lookup_pid(requestee_handle.handle.peer_id())
            .await
            .is_err()
        {
            error!("ERROR LOOKING UP REQUESTEE ADDRS");
        }
        // NOTE uncomment if debugging
        // let _ = h.print_routing_table().await;
        // skip `requestee_handle`
        if handle.handle.peer_id() != requestee_handle.handle.peer_id() {
            let requester_handle = handle.clone();
            futs.push(run_request_response_increment(
                requester_handle,
                requestee_handle.clone(),
                timeout,
            ));
        }
    }

    // NOTE this was originally join_all
    // but this is simpler.
    let results = Arc::new(RwLock::new(vec![]));

    let len = futs.len();

    for _ in 0..futs.len() {
        let fut = futs.pop().unwrap();
        let results = results.clone();
        async_spawn(async move {
            let res = fut.await;
            results.write().await.push(res);
        });
    }
    loop {
        let l = results.read().await.iter().len();
        if l >= len {
            break;
        }
        info!("NUMBER OF RESULTS for increment all is: {}", l);
        async_sleep(Duration::from_secs(1)).await;
    }

    if results.read().await.iter().any(Result::is_err) {
        let nodes = handles
            .iter()
            .cloned()
            .map(|h| h.handle)
            .collect::<Vec<_>>();
        print_connections(nodes.as_slice()).await;
        let mut states = vec![];
        for handle in handles {
            states.push(handle.state.copied().await);
        }
        error!("states: {states:?}");
        std::process::exit(-1);
    }
}

/// simple case of direct message
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn test_coverage_request_response_one_round() {
    Box::pin(test_bed(
        run_request_response_one_round,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_COVERAGE,
        NUM_OF_BOOTSTRAP_COVERAGE,
        TIMEOUT_COVERAGE,
    ))
    .await;
}

/// stress test of direct message
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn test_coverage_request_response_many_rounds() {
    Box::pin(test_bed(
        run_request_response_many_rounds,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_COVERAGE,
        NUM_OF_BOOTSTRAP_COVERAGE,
        TIMEOUT_COVERAGE,
    ))
    .await;
}

/// stress test of broadcast + direct message
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn test_coverage_intersperse_many_rounds() {
    Box::pin(test_bed(
        run_intersperse_many_rounds,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_COVERAGE,
        NUM_OF_BOOTSTRAP_COVERAGE,
        TIMEOUT_COVERAGE,
    ))
    .await;
}

/// stress teset that we can broadcast a message out and get counter increments
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn test_coverage_gossip_many_rounds() {
    Box::pin(test_bed(
        run_gossip_many_rounds,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_COVERAGE,
        NUM_OF_BOOTSTRAP_COVERAGE,
        TIMEOUT_COVERAGE,
    ))
    .await;
}

/// simple case of broadcast message
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn test_coverage_gossip_one_round() {
    Box::pin(test_bed(
        run_gossip_one_round,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_COVERAGE,
        NUM_OF_BOOTSTRAP_COVERAGE,
        TIMEOUT_COVERAGE,
    ))
    .await;
}

/// simple case of direct message
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
#[ignore]
async fn test_stress_request_response_one_round() {
    Box::pin(test_bed(
        run_request_response_one_round,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_STRESS,
        NUM_OF_BOOTSTRAP_STRESS,
        TIMEOUT_STRESS,
    ))
    .await;
}

/// stress test of direct messsage
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
#[ignore]
async fn test_stress_request_response_many_rounds() {
    Box::pin(test_bed(
        run_request_response_many_rounds,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_STRESS,
        NUM_OF_BOOTSTRAP_STRESS,
        TIMEOUT_STRESS,
    ))
    .await;
}

/// stress test of broadcast + direct message
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
#[ignore]
async fn test_stress_intersperse_many_rounds() {
    Box::pin(test_bed(
        run_intersperse_many_rounds,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_STRESS,
        NUM_OF_BOOTSTRAP_STRESS,
        TIMEOUT_STRESS,
    ))
    .await;
}

/// stress teset that we can broadcast a message out and get counter increments
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
#[ignore]
async fn test_stress_gossip_many_rounds() {
    Box::pin(test_bed(
        run_gossip_many_rounds,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_STRESS,
        NUM_OF_BOOTSTRAP_STRESS,
        TIMEOUT_STRESS,
    ))
    .await;
}

/// simple case of broadcast message
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
#[ignore]
async fn test_stress_gossip_one_round() {
    Box::pin(test_bed(
        run_gossip_one_round,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_STRESS,
        NUM_OF_BOOTSTRAP_STRESS,
        TIMEOUT_STRESS,
    ))
    .await;
}

/// simple case of one dht publish event
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
#[ignore]
async fn test_stress_dht_one_round() {
    Box::pin(test_bed(
        run_dht_one_round,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_STRESS,
        NUM_OF_BOOTSTRAP_STRESS,
        TIMEOUT_STRESS,
    ))
    .await;
}

/// many dht publishing events
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
#[ignore]
async fn test_stress_dht_many_rounds() {
    Box::pin(test_bed(
        run_dht_many_rounds,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_STRESS,
        NUM_OF_BOOTSTRAP_STRESS,
        TIMEOUT_STRESS,
    ))
    .await;
}

/// simple case of one dht publish event
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn test_coverage_dht_one_round() {
    Box::pin(test_bed(
        run_dht_one_round,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_COVERAGE,
        NUM_OF_BOOTSTRAP_COVERAGE,
        TIMEOUT_COVERAGE,
    ))
    .await;
}

/// many dht publishing events
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn test_coverage_dht_many_rounds() {
    Box::pin(test_bed(
        run_dht_many_rounds,
        counter_handle_network_event,
        TOTAL_NUM_PEERS_COVERAGE,
        NUM_OF_BOOTSTRAP_COVERAGE,
        TIMEOUT_COVERAGE,
    ))
    .await;
}
