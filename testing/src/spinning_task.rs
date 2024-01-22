use std::{collections::HashMap, sync::Arc};

use async_compatibility_layer::channel::UnboundedStream;
use futures::FutureExt;
use hotshot::{traits::TestableNodeImplementation, SystemContext};
use hotshot_task::{
    event_stream::ChannelStream,
    task::{FilterEvent, HandleEvent, HandleMessage, HotShotTaskCompleted, HotShotTaskTypes, TS},
    task_impls::{HSTWithEventAndMessage, TaskBuilder},
    MergeN,
};
use hotshot_types::traits::network::CommunicationChannel;
use hotshot_types::{event::Event, traits::node_implementation::NodeType};
use snafu::Snafu;

use crate::{test_launcher::TaskGenerator, test_runner::Node};
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
    pub(crate) late_start: HashMap<u64, SystemContext<TYPES, I>>,
    /// time based changes
    pub(crate) changes: HashMap<TYPES::Time, Vec<ChangeNode>>,
    /// most recent view seen by spinning task
    pub(crate) latest_view: Option<TYPES::Time>,
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TS for SpinningTask<TYPES, I> {}

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

impl SpinningTaskDescription {
    /// build a task
    /// # Panics
    /// If there is no latest view
    /// or if the node id is over `u32::MAX`
    #[must_use]
    pub fn build<TYPES: NodeType, I: TestableNodeImplementation<TYPES>>(
        self,
    ) -> TaskGenerator<SpinningTask<TYPES, I>> {
        Box::new(move |mut state, mut registry, test_event_stream| {
            async move {
                let event_handler =
                    HandleEvent::<SpinningTaskTypes<TYPES, I>>(Arc::new(move |event, state| {
                        async move {
                            match event {
                                GlobalTestEvent::ShutDown => {
                                    (Some(HotShotTaskCompleted::ShutDown), state)
                                }
                            }
                        }
                        .boxed()
                    }));

                let message_handler = HandleMessage::<SpinningTaskTypes<TYPES, I>>(Arc::new(
                    move |msg, mut state| {
                        async move {
                            let Event {
                                view_number,
                                event: _,
                            } = msg.1;

                            // if we have not seen this view before
                            if state.latest_view.is_none()
                                || view_number > state.latest_view.unwrap()
                            {
                                // perform operations on the nodes
                                if let Some(operations) = state.changes.remove(&view_number) {
                                    for ChangeNode { idx, updown } in operations {
                                        match updown {
                                            UpDown::Up => {
                                                if let Some(node) = state
                                                    .late_start
                                                    .remove(&idx.try_into().unwrap())
                                                {
                                                    tracing::error!(
                                                        "Node {} spinning up late",
                                                        idx
                                                    );
                                                    let handle = node.run_tasks().await;
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
                                                    tracing::error!(
                                                        "Node {} networks resuming",
                                                        idx
                                                    );
                                                    handle.networks.0.resume();
                                                    handle.networks.1.resume();
                                                }
                                            }
                                            UpDown::NetworkDown => {
                                                if let Some(handle) = state.handles.get(idx) {
                                                    tracing::error!(
                                                        "Node {} networks pausing",
                                                        idx
                                                    );
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

                            (None, state)
                        }
                        .boxed()
                    },
                ));

                let mut streams = vec![];
                for handle in &mut state.handles {
                    let s1 = handle
                        .handle
                        .get_event_stream_known_impl(FilterEvent::default())
                        .await
                        .0;
                    streams.push(s1);
                }
                let builder = TaskBuilder::<SpinningTaskTypes<TYPES, I>>::new(
                    "Test Spinning Task".to_string(),
                )
                .register_event_stream(test_event_stream, FilterEvent::default())
                .await
                .register_registry(&mut registry)
                .await
                .register_message_handler(message_handler)
                .register_message_stream(MergeN::new(streams))
                .register_event_handler(event_handler)
                .register_state(state);
                let task_id = builder.get_task_id().unwrap();
                (task_id, SpinningTaskTypes::build(builder).launch())
            }
            .boxed()
        })
    }
}

/// types for safety task
pub type SpinningTaskTypes<TYPES, I> = HSTWithEventAndMessage<
    SpinningTaskErr,
    GlobalTestEvent,
    ChannelStream<GlobalTestEvent>,
    (usize, Event<TYPES>),
    MergeN<UnboundedStream<Event<TYPES>>>,
    SpinningTask<TYPES, I>,
>;
