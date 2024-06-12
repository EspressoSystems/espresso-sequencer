use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use anyhow::Result;
use async_lock::RwLock;
use async_trait::async_trait;
use futures::future::Either::{Left, Right};
use hotshot::{traits::TestableNodeImplementation, types::EventType, HotShotInitializer};
use hotshot_example_types::{
    state_types::{TestInstanceState, TestValidatedState},
    storage_types::TestStorage,
};
use hotshot_types::{
    data::Leaf,
    event::Event,
    simple_certificate::QuorumCertificate,
    traits::{
        network::ConnectedNetwork,
        node_implementation::{NodeImplementation, NodeType},
    },
    vote::HasViewNumber,
    ValidatorConfig,
};
use snafu::Snafu;

use crate::{
    test_runner::{LateStartNode, Node, TestRunner},
    test_task::{TestResult, TestTaskState},
};

/// convience type for state and block
pub type StateAndBlock<S, B> = (Vec<S>, Vec<B>);

/// error for the spinning task
#[derive(Snafu, Debug)]
pub struct SpinningTaskErr {}

/// Spinning task state
pub struct SpinningTask<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// handle to the nodes
    pub(crate) handles: Arc<RwLock<Vec<Node<TYPES, I>>>>,
    /// late start nodes
    pub(crate) late_start: HashMap<u64, LateStartNode<TYPES, I>>,
    /// time based changes
    pub(crate) changes: BTreeMap<TYPES::Time, Vec<ChangeNode>>,
    /// most recent view seen by spinning task
    pub(crate) latest_view: Option<TYPES::Time>,
    /// Last decided leaf that can be used as the anchor leaf to initialize the node.
    pub(crate) last_decided_leaf: Leaf<TYPES>,
    /// Highest qc seen in the test for restarting nodes
    pub(crate) high_qc: QuorumCertificate<TYPES>,
}

#[async_trait]
impl<
        TYPES: NodeType<InstanceState = TestInstanceState, ValidatedState = TestValidatedState>,
        I: TestableNodeImplementation<TYPES>,
        N: ConnectedNetwork<TYPES::SignatureKey>,
    > TestTaskState for SpinningTask<TYPES, I>
where
    I: TestableNodeImplementation<TYPES>,
    I: NodeImplementation<TYPES, QuorumNetwork = N, DaNetwork = N, Storage = TestStorage<TYPES>>,
{
    type Event = Event<TYPES>;

    async fn handle_event(&mut self, (message, _id): (Self::Event, usize)) -> Result<()> {
        let Event { view_number, event } = message;

        if let EventType::Decide {
            leaf_chain,
            qc: _,
            block_size: _,
        } = event
        {
            let leaf = leaf_chain.first().unwrap().leaf.clone();
            if leaf.view_number() > self.last_decided_leaf.view_number() {
                self.last_decided_leaf = leaf;
            }
        } else if let EventType::QuorumProposal {
            proposal,
            sender: _,
        } = event
        {
            if proposal.data.justify_qc.view_number() > self.high_qc.view_number() {
                self.high_qc = proposal.data.justify_qc.clone();
            }
        }

        // if we have not seen this view before
        if self.latest_view.is_none() || view_number > self.latest_view.unwrap() {
            // perform operations on the nodes
            if let Some(operations) = self.changes.remove(&view_number) {
                for ChangeNode { idx, updown } in operations {
                    match updown {
                        UpDown::Up => {
                            let node_id = idx.try_into().unwrap();
                            if let Some(node) = self.late_start.remove(&node_id) {
                                tracing::error!("Node {} spinning up late", idx);
                                let node_id = idx.try_into().unwrap();
                                let context = match node.context {
                                    Left(context) => context,
                                    // Node not initialized. Initialize it
                                    // based on the received leaf.
                                    Right((storage, memberships, config)) => {
                                        let initializer = HotShotInitializer::<TYPES>::from_reload(
                                            self.last_decided_leaf.clone(),
                                            TestInstanceState {},
                                            None,
                                            view_number,
                                            BTreeMap::new(),
                                            self.high_qc.clone(),
                                            Vec::new(),
                                            BTreeMap::new(),
                                        );
                                        // We assign node's public key and stake value rather than read from config file since it's a test
                                        let validator_config =
                                            ValidatorConfig::generated_from_seed_indexed(
                                                [0u8; 32],
                                                node_id,
                                                1,
                                                // For tests, make the node DA based on its index
                                                node_id < config.da_staked_committee_size as u64,
                                            );
                                        TestRunner::add_node_with_config(
                                            node_id,
                                            node.networks.clone(),
                                            memberships,
                                            initializer,
                                            config,
                                            validator_config,
                                            storage,
                                        )
                                        .await
                                    }
                                };

                                let handle = context.run_tasks().await;

                                // Create the node and add it to the state, so we can shut them
                                // down properly later to avoid the overflow error in the overall
                                // safety task.
                                let node = Node {
                                    node_id,
                                    networks: node.networks,
                                    handle,
                                };
                                node.handle.hotshot.start_consensus().await;

                                self.handles.write().await.push(node);
                            }
                        }
                        UpDown::Down => {
                            if let Some(node) = self.handles.write().await.get_mut(idx) {
                                tracing::error!("Node {} shutting down", idx);
                                node.handle.shut_down().await;
                            }
                        }
                        UpDown::NetworkUp => {
                            if let Some(handle) = self.handles.write().await.get(idx) {
                                tracing::error!("Node {} networks resuming", idx);
                                handle.networks.0.resume();
                                handle.networks.1.resume();
                            }
                        }
                        UpDown::NetworkDown => {
                            if let Some(handle) = self.handles.write().await.get(idx) {
                                tracing::error!("Node {} networks pausing", idx);
                                handle.networks.0.pause();
                                handle.networks.1.pause();
                            }
                        }
                    }
                }
            }

            // update our latest view
            self.latest_view = Some(view_number);
        }

        Ok(())
    }

    fn check(&self) -> TestResult {
        TestResult::Pass
    }
}

/// Spin the node up or down
#[derive(Clone, Debug)]
pub enum UpDown {
    /// spin the node up
    Up,
    /// spin the node down
    Down,
    /// spin the node's network up
    NetworkUp,
    /// spin the node's network down
    NetworkDown,
}

/// denotes a change in node state
#[derive(Clone, Debug)]
pub struct ChangeNode {
    /// the index of the node
    pub idx: usize,
    /// spin the node or node's network up or down
    pub updown: UpDown,
}

/// description of the spinning task
/// (used to build a spinning task)
#[derive(Clone, Debug)]
pub struct SpinningTaskDescription {
    /// the changes in node status, time -> changes
    pub node_changes: Vec<(u64, Vec<ChangeNode>)>,
}
