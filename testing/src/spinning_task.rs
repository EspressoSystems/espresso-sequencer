use std::{
    sync::{atomic::AtomicUsize, Arc},
    time::Duration,
};

use async_compatibility_layer::art::async_sleep;
use futures::FutureExt;
use hotshot::traits::TestableNodeImplementation;
use hotshot_task::{
    boxed_sync,
    event_stream::ChannelStream,
    task::{FilterEvent, HandleEvent, HandleMessage, HotShotTaskCompleted, HotShotTaskTypes, TS},
    task_impls::{HSTWithEventAndMessage, TaskBuilder},
    GeneratedStream,
};
use hotshot_types::traits::node_implementation::NodeType;
use snafu::Snafu;

use crate::{test_launcher::TaskGenerator, test_runner::Node, GlobalTestEvent};
#[derive(Snafu, Debug)]
pub struct SpinningTaskErr {}

/// Completion task types
pub type SpinningTaskTypes<TYPES, I> = HSTWithEventAndMessage<
    SpinningTaskErr,
    GlobalTestEvent,
    ChannelStream<GlobalTestEvent>,
    (),
    GeneratedStream<()>,
    SpinningTask<TYPES, I>,
>;

pub struct SpinningTask<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    pub(crate) handles: Vec<Node<TYPES, I>>,
    pub(crate) changes: Vec<Vec<ChangeNode>>,
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TS for SpinningTask<TYPES, I> {}

/// Spin the node up or down
#[derive(Clone, Debug)]
pub enum UpDown {
    /// spin the node up
    Up,
    /// spin the node down
    Down,
}

/// denotes a change in node state
#[derive(Clone, Debug)]
pub struct ChangeNode {
    /// the index of the node
    pub idx: usize,
    /// spin the node up or down
    pub updown: UpDown,
}

#[derive(Clone, Debug)]
pub struct SpinningTaskDescription {
    pub node_changes: Vec<(Duration, Vec<ChangeNode>)>,
}

impl SpinningTaskDescription {
    pub fn build<TYPES: NodeType, I: TestableNodeImplementation<TYPES>>(
        self,
    ) -> TaskGenerator<SpinningTask<TYPES, I>> {
        Box::new(move |state, mut registry, test_event_stream| {
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
                let atomic_idx = Arc::new(AtomicUsize::new(0));
                let sleep_durations = Arc::new(
                    self.node_changes
                        .clone()
                        .into_iter()
                        .map(|(d, _)| d)
                        .collect::<Vec<_>>(),
                );
                let stream_generator = GeneratedStream::new(Arc::new(move || {
                    let atomic_idx = atomic_idx.clone();
                    let sleep_durations = sleep_durations.clone();
                    let atomic_idx = atomic_idx.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    sleep_durations.get(atomic_idx).copied().map(|duration| {
                        let fut = async move {
                            async_sleep(duration).await;
                        };
                        boxed_sync(fut)
                    })
                }));
                let message_handler = HandleMessage::<SpinningTaskTypes<TYPES, I>>(Arc::new(
                    move |_msg, mut state| {
                        async move {
                            if let Some(nodes_to_change) = state.changes.pop() {
                                for ChangeNode { idx, updown } in nodes_to_change {
                                    match updown {
                                        UpDown::Up => {
                                            // TODO... we don't need this right now anyway. We haven't
                                            // implemented catchup
                                        }
                                        UpDown::Down => {
                                            if let Some(node) = state.handles.get_mut(idx) {
                                                node.handle.shut_down().await;
                                            }
                                        }
                                    }
                                }
                            }
                            (None, state)
                        }
                        .boxed()
                    },
                ));
                let builder = TaskBuilder::<SpinningTaskTypes<TYPES, I>>::new(
                    "Test Spinning Task".to_string(),
                )
                .register_event_stream(test_event_stream, FilterEvent::default())
                .await
                .register_registry(&mut registry)
                .await
                .register_state(state)
                .register_event_handler(event_handler)
                .register_message_handler(message_handler)
                .register_message_stream(stream_generator);
                let task_id = builder.get_task_id().unwrap();
                (task_id, SpinningTaskTypes::build(builder).launch())
            }
            .boxed()
        })
    }
}
