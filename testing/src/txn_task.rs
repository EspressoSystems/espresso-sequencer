use std::time::Duration;

use async_broadcast::{Receiver, TryRecvError};
use async_compatibility_layer::art::{async_sleep, async_spawn};
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use hotshot::traits::TestableNodeImplementation;
use hotshot_types::traits::node_implementation::NodeType;
use rand::thread_rng;
use snafu::Snafu;
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;

use super::GlobalTestEvent;
use crate::test_runner::{HotShotTaskCompleted, Node};

// the obvious idea here is to pass in a "stream" that completes every `n` seconds
// the stream construction can definitely be fancier but that's the baseline idea

/// Data Availability task error
#[derive(Snafu, Debug)]
pub struct TxnTaskErr {}

/// state of task that decides when things are completed
pub struct TxnTask<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    // TODO should this be in a rwlock? Or maybe a similar abstraction to the registry is in order
    /// Handles for all nodes.
    pub handles: Vec<Node<TYPES, I>>,
    /// Optional index of the next node.
    pub next_node_idx: Option<usize>,
    /// time to wait between txns
    pub duration: Duration,
    /// Receiver for the shutdown signal from the testing harness
    pub shutdown_chan: Receiver<GlobalTestEvent>,
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TxnTask<TYPES, I> {
    pub fn run(mut self) -> JoinHandle<HotShotTaskCompleted> {
        async_spawn(async move {
            async_sleep(Duration::from_millis(100)).await;
            loop {
                async_sleep(self.duration).await;
                match self.shutdown_chan.try_recv() {
                    Ok(_event) => {
                        return HotShotTaskCompleted::ShutDown;
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(_) => {
                        return HotShotTaskCompleted::StreamsDied;
                    }
                }
                self.submit_tx().await;
            }
        })
    }
    async fn submit_tx(&mut self) {
        if let Some(idx) = self.next_node_idx {
            // submit to idx handle
            // increment state
            self.next_node_idx = Some((idx + 1) % self.handles.len());
            match self.handles.get(idx) {
                None => {
                    tracing::error!("couldn't get node in txn task");
                    // should do error
                    unimplemented!()
                }
                Some(node) => {
                    // use rand::seq::IteratorRandom;
                    // we're assuming all nodes have the same leaf.
                    // If they don't match, this is probably fine since
                    // it should be caught by an assertion (and the txn will be rejected anyway)
                    let leaf = node.handle.get_decided_leaf().await;
                    let txn = I::leaf_create_random_transaction(&leaf, &mut thread_rng(), 0);
                    node.handle
                        .submit_transaction(txn.clone())
                        .await
                        .expect("Could not send transaction");
                }
            }
        }
    }
}

/// build the transaction task
#[derive(Clone, Debug)]
pub enum TxnTaskDescription {
    /// submit transactions in a round robin style using
    /// every `Duration` seconds
    RoundRobinTimeBased(Duration),
    /// TODO
    DistributionBased, // others?
}
