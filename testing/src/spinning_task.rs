use std::collections::HashMap;

use hotshot::traits::TestableNodeImplementation;

use crate::test_runner::HotShotTaskCompleted;
use crate::test_runner::LateStartNode;
use crate::test_runner::Node;
use hotshot_task::task::{Task, TaskState, TestTaskState};
use hotshot_types::traits::network::ConnectedNetwork;
use hotshot_types::{event::Event, traits::node_implementation::NodeType};
use snafu::Snafu;
use std::collections::BTreeMap;
/// convience type for state and block
pub type StateAndBlock<S, B> = (Vec<S>, Vec<B>);

use super::GlobalTestEvent;

/// error for the spinning task
#[derive(Snafu, Debug)]
pub struct SpinningTaskErr {}

/// Spinning task state
pub struct SpinningTask<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// handle to the nodes
    pub(crate) handles: Vec<Node<TYPES, I>>,
    /// late start nodes
    pub(crate) late_start: HashMap<u64, LateStartNode<TYPES, I>>,
    /// time based changes
    pub(crate) changes: BTreeMap<TYPES::Time, Vec<ChangeNode>>,
    /// most recent view seen by spinning task
    pub(crate) latest_view: Option<TYPES::Time>,
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TaskState for SpinningTask<TYPES, I> {
    type Event = GlobalTestEvent;

    type Output = HotShotTaskCompleted;

    async fn handle_event(event: Self::Event, _task: &mut Task<Self>) -> Option<Self::Output> {
        if matches!(event, GlobalTestEvent::ShutDown) {
            return Some(HotShotTaskCompleted::ShutDown);
        }
        None
    }

    fn should_shutdown(_event: &Self::Event) -> bool {
        false
    }
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TestTaskState
    for SpinningTask<TYPES, I>
{
    type Message = Event<TYPES>;

    type Output = HotShotTaskCompleted;

    type State = Self;

    async fn handle_message(
        message: Self::Message,
        _id: usize,
        task: &mut hotshot_task::task::TestTask<Self::State, Self>,
    ) -> Option<Self::Output> {
        let Event {
            view_number,
            event: _,
        } = message;

        let state = &mut task.state_mut();

        // if we have not seen this view before
        if state.latest_view.is_none() || view_number > state.latest_view.unwrap() {
            // perform operations on the nodes
            if let Some(operations) = state.changes.remove(&view_number) {
                for ChangeNode { idx, updown } in operations {
                    match updown {
                        UpDown::Up => {
                            if let Some(node) = state.late_start.remove(&idx.try_into().unwrap()) {
                                tracing::error!("Node {} spinning up late", idx);
                                let handle = node.context.run_tasks().await;
                                handle.hotshot.start_consensus().await;
                            }
                        }
                        UpDown::Down => {
                            if let Some(node) = state.handles.get_mut(idx) {
                                tracing::error!("Node {} shutting down", idx);
                                node.handle.shut_down().await;
                            }
                        }
                        UpDown::NetworkUp => {
                            if let Some(handle) = state.handles.get(idx) {
                                tracing::error!("Node {} networks resuming", idx);
                                handle.networks.0.resume();
                                handle.networks.1.resume();
                            }
                        }
                        UpDown::NetworkDown => {
                            if let Some(handle) = state.handles.get(idx) {
                                tracing::error!("Node {} networks pausing", idx);
                                handle.networks.0.pause();
                                handle.networks.1.pause();
                            }
                        }
                    }
                }
            }

            // update our latest view
            state.latest_view = Some(view_number);
        }

        None
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
