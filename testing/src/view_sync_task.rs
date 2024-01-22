use async_compatibility_layer::channel::UnboundedStream;
use futures::FutureExt;
use hotshot_task::task::{HotShotTaskCompleted, HotShotTaskTypes};
use hotshot_task::{
    event_stream::ChannelStream,
    task::{FilterEvent, HandleEvent, HandleMessage, TS},
    task_impls::{HSTWithEventAndMessage, TaskBuilder},
    MergeN,
};
use hotshot_task_impls::events::HotShotEvent;
use hotshot_types::traits::node_implementation::{NodeType, TestableNodeImplementation};
use snafu::Snafu;
use std::{collections::HashSet, sync::Arc};

use crate::{test_launcher::TaskGenerator, test_runner::Node, GlobalTestEvent};

/// `ViewSync` Task error
#[derive(Snafu, Debug, Clone)]
pub struct ViewSyncTaskErr {
    /// set of node ids that hit view sync
    hit_view_sync: HashSet<usize>,
}

/// `ViewSync` task state
pub struct ViewSyncTask<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// the node handles
    pub(crate) handles: Vec<Node<TYPES, I>>,
    /// nodes that hit view sync
    pub(crate) hit_view_sync: HashSet<usize>,
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TS for ViewSyncTask<TYPES, I> {}

/// `ViewSync` task types
pub type ViewSyncTaskTypes<TYPES, I> = HSTWithEventAndMessage<
    ViewSyncTaskErr,
    GlobalTestEvent,
    ChannelStream<GlobalTestEvent>,
    (usize, HotShotEvent<TYPES>),
    MergeN<UnboundedStream<HotShotEvent<TYPES>>>,
    ViewSyncTask<TYPES, I>,
>;

/// enum desecribing whether a node should hit view sync
#[derive(Clone, Debug, Copy)]
pub enum ShouldHitViewSync {
    /// the node should hit view sync
    Yes,
    /// the node should not hit view sync
    No,
    /// don't care if the node should hit view sync
    Ignore,
}

/// Description for a view sync task.
#[derive(Clone, Debug)]
pub enum ViewSyncTaskDescription {
    /// (min, max) number nodes that may hit view sync, inclusive
    Threshold(usize, usize),
}

impl ViewSyncTaskDescription {
    /// build a view sync task from its description
    /// # Panics
    /// if there is an violation of the view sync description
    #[must_use]
    pub fn build<TYPES: NodeType, I: TestableNodeImplementation<TYPES>>(
        self,
    ) -> TaskGenerator<ViewSyncTask<TYPES, I>> {
        Box::new(move |mut state, mut registry, test_event_stream| {
            async move {
                let event_handler =
                    HandleEvent::<ViewSyncTaskTypes<TYPES, I>>(Arc::new(move |event, state| {
                        let self_dup = self.clone();
                        async move {
                            match event {
                                GlobalTestEvent::ShutDown => match self_dup.clone() {
                                    ViewSyncTaskDescription::Threshold(min, max) => {
                                        let num_hits = state.hit_view_sync.len();
                                        if min <= num_hits && num_hits <= max {
                                            (Some(HotShotTaskCompleted::ShutDown), state)
                                        } else {
                                            (
                                                Some(HotShotTaskCompleted::Error(Box::new(
                                                    ViewSyncTaskErr {
                                                        hit_view_sync: state.hit_view_sync.clone(),
                                                    },
                                                ))),
                                                state,
                                            )
                                        }
                                    }
                                },
                            }
                        }
                        .boxed()
                    }));

                let message_handler = HandleMessage::<ViewSyncTaskTypes<TYPES, I>>(Arc::new(
                    // NOTE: could short circuit on entering view sync if we're not supposed to
                    // enter view sync. I opted not to do this just to gather more information
                    // (since we'll fail the test later anyway)
                    move |(id, msg), mut state| {
                        async move {
                            match msg {
                                // all the view sync events
                                HotShotEvent::ViewSyncTimeout(_, _, _)
                                | HotShotEvent::ViewSyncPreCommitVoteRecv(_)
                                | HotShotEvent::ViewSyncCommitVoteRecv(_)
                                | HotShotEvent::ViewSyncFinalizeVoteRecv(_)
                                | HotShotEvent::ViewSyncPreCommitVoteSend(_)
                                | HotShotEvent::ViewSyncCommitVoteSend(_)
                                | HotShotEvent::ViewSyncFinalizeVoteSend(_)
                                | HotShotEvent::ViewSyncPreCommitCertificate2Recv(_)
                                | HotShotEvent::ViewSyncCommitCertificate2Recv(_)
                                | HotShotEvent::ViewSyncFinalizeCertificate2Recv(_)
                                | HotShotEvent::ViewSyncPreCommitCertificate2Send(_, _)
                                | HotShotEvent::ViewSyncCommitCertificate2Send(_, _)
                                | HotShotEvent::ViewSyncFinalizeCertificate2Send(_, _)
                                | HotShotEvent::ViewSyncTrigger(_) => {
                                    state.hit_view_sync.insert(id);
                                }
                                _ => (),
                            }
                            (None, state)
                        }
                        .boxed()
                    },
                ));
                let mut streams = vec![];
                for handle in &mut state.handles {
                    let stream = handle
                        .handle
                        .get_internal_event_stream_known_impl(FilterEvent::default())
                        .await
                        .0;
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
            }
            .boxed()
        })
    }
}
