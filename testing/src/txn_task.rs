use crate::test_runner::Node;
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
use hotshot_types::{
    message::SequencingMessage,
    traits::node_implementation::{NodeImplementation, NodeType},
};
use rand::thread_rng;
use snafu::Snafu;
use std::{sync::Arc, time::Duration};

use super::{test_launcher::TaskGenerator, GlobalTestEvent};

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
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TS for TxnTask<TYPES, I> {}

/// types for task that deices when things are completed
pub type TxnTaskTypes<TYPES, I> = HSTWithEventAndMessage<
    TxnTaskErr,
    GlobalTestEvent,
    ChannelStream<GlobalTestEvent>,
    (),
    GeneratedStream<()>,
    TxnTask<TYPES, I>,
>;

/// build the transaction task
#[derive(Clone, Debug)]
pub enum TxnTaskDescription {
    /// submit transactions in a round robin style using
    /// every `Duration` seconds
    RoundRobinTimeBased(Duration),
    /// TODO
    DistributionBased, // others?
}

impl TxnTaskDescription {
    /// build a task
    pub fn build<TYPES: NodeType, I: TestableNodeImplementation<TYPES>>(
        self,
    ) -> TaskGenerator<TxnTask<TYPES, I>>
    where
        TYPES: NodeType,
        I: NodeImplementation<TYPES, ConsensusMessage = SequencingMessage<TYPES, I>>,
    {
        Box::new(move |state, mut registry, test_event_stream| {
            async move {
                // consistency check
                match self {
                    TxnTaskDescription::RoundRobinTimeBased(_) => {
                        assert!(state.next_node_idx.is_some())
                    }
                    TxnTaskDescription::DistributionBased => assert!(state.next_node_idx.is_none()),
                }
                // TODO we'll possibly want multiple criterion including:
                // - certain number of txns committed
                // - anchor of certain depth
                // - some other stuff? probably?
                let event_handler =
                    HandleEvent::<TxnTaskTypes<TYPES, I>>(Arc::new(move |event, state| {
                        async move {
                            match event {
                                GlobalTestEvent::ShutDown => {
                                    (Some(HotShotTaskCompleted::ShutDown), state)
                                }
                            }
                        }
                        .boxed()
                    }));
                let message_handler =
                    HandleMessage::<TxnTaskTypes<TYPES, I>>(Arc::new(move |_, mut state| {
                        async move {
                            if let Some(idx) = state.next_node_idx {
                                // submit to idx handle
                                // increment state
                                state.next_node_idx = Some((idx + 1) % state.handles.len());
                                match state.handles.get(idx) {
                                    None => {
                                        // should do error
                                        unimplemented!()
                                    }
                                    Some(node) => {
                                        // use rand::seq::IteratorRandom;
                                        // we're assuming all nodes have the same leaf.
                                        // If they don't match, this is probably fine since
                                        // it should be caught by an assertion (and the txn will be rejected anyway)
                                        let leaf = node.handle.get_decided_leaf().await;
                                        let txn = I::leaf_create_random_transaction(
                                            &leaf,
                                            &mut thread_rng(),
                                            0,
                                        );
                                        node.handle
                                            .submit_transaction(txn.clone())
                                            .await
                                            .expect("Could not send transaction");
                                        (None, state)
                                    }
                                }
                            } else {
                                // TODO make an issue
                                // in the case that this is random
                                // which I haven't implemented yet
                                unimplemented!()
                            }
                        }
                        .boxed()
                    }));
                let stream_generator = match self {
                    TxnTaskDescription::RoundRobinTimeBased(duration) => {
                        GeneratedStream::new(Arc::new(move || {
                            let fut = async move {
                                async_sleep(duration).await;
                            };
                            Some(boxed_sync(fut))
                        }))
                    }
                    TxnTaskDescription::DistributionBased => unimplemented!(),
                };
                let builder = TaskBuilder::<TxnTaskTypes<TYPES, I>>::new(
                    "Test Transaction Submission Task".to_string(),
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
                (task_id, TxnTaskTypes::build(builder).launch())
            }
            .boxed()
        })
    }
}
