use std::{sync::Arc, collections::{HashSet, HashMap}};
use futures::FutureExt;
use hotshot_task::task::HotShotTaskTypes;
use async_compatibility_layer::channel::UnboundedStream;
use hotshot_task::{task_impls::{TaskBuilder, HSTWithEventAndMessage}, task::{FilterEvent, HandleMessage, HandleEvent, TS}, MergeN, event_stream::ChannelStream};
use hotshot_task_impls::events::HotShotEvent;
use hotshot_types::traits::node_implementation::{TestableNodeImplementation, NodeType};
use snafu::Snafu;

use crate::{test_launcher::TaskGenerator, test_runner::Node, GlobalTestEvent};

/// ViewSync Task error
#[derive(Snafu, Debug)]
pub struct ViewSyncTaskErr {}

/// ViewSync task state
pub struct ViewSyncTask<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// the global event stream
    pub(crate) test_event_stream: ChannelStream<GlobalTestEvent>,
    /// the node handles
    pub(crate) handles: Vec<Node<TYPES, I>>,
    /// nodes that hit view sync
    pub(crate) hit_view_sync: HashMap<usize, <TYPES as NodeType>::Time>
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TS for ViewSyncTask<TYPES, I> {}

/// ViewSync task types
pub type ViewSyncTaskTypes<TYPES, I> = HSTWithEventAndMessage<
    ViewSyncTaskErr,
    GlobalTestEvent,
    ChannelStream<GlobalTestEvent>,
    (usize, HotShotEvent<TYPES>),
    MergeN<UnboundedStream<HotShotEvent<TYPES>>>,
    ViewSyncTask<TYPES, I>,
>;

#[derive(Clone, Debug, Copy)]
pub enum ShouldHitViewSync {
    /// the node should hit view sync
    Yes,
    /// the node should not hit view sync
    No,
    /// don't care if the node should hit view sync
    DontCare
}

/// Description for a view sync task.
#[derive(Clone, Debug)]
pub enum ViewSyncTaskDescription {
    /// (min, max) number nodes that may hit view sync, inclusive
    Threshold(usize, usize),
    /// node idx -> whether or not the node should hit view sync
    /// if node not in map, assumed to be `ShouldHItViewSync::DontCare`
    Precise(HashMap<usize, ShouldHitViewSync>)
}

impl ViewSyncTaskDescription {
    pub fn build<TYPES: NodeType, I: TestableNodeImplementation<TYPES>>(
        self,
    ) -> TaskGenerator<ViewSyncTask<TYPES, I>> {
        Box::new(move |mut state, mut registry, test_event_stream| {
            async move {

                let event_handler = HandleEvent::<ViewSyncTaskTypes<TYPES, I>>(Arc::new(move |event, state| {
                    async move {
                        match event {
                            GlobalTestEvent::ShutDown => {
                                todo!()
                                    // logic checking stuff
                            }
                        }
                    }.boxed()

                }));

                let message_handler = HandleMessage::<ViewSyncTaskTypes<TYPES, I>>(Arc::new(
                        move |msg, mut state| {
                            todo!()
                        }
                        ));
                let mut streams = vec![];
                for handle in &mut state.handles {
                    let stream = handle.handle.get_internal_event_stream_known_impl(FilterEvent::default()).await.0;
                    streams.push(stream);
                }

                let builder = TaskBuilder::<ViewSyncTaskTypes<TYPES, I>>::new(
                    "Test Completion Task".to_string(),
                    )
                    .register_event_stream(test_event_stream, FilterEvent::default())
                    .await
                    .register_registry(&mut registry)
                    .await
                    .register_state(state)
                    .register_event_handler(event_handler)
                    .register_message_handler(message_handler)
                    .register_message_stream(MergeN::new(streams));
                let task_id = builder.get_task_id().unwrap();
                (task_id, ViewSyncTaskTypes::build(builder).launch())
            }.boxed()
        })

        // match self {
        //     ViewSyncTaskDescription::Threshold(threshold) => {
        //
        //     },
        //     ViewSyncTaskDescription::Precise(map) => {
        //
        //     }
        // }
    }

}
