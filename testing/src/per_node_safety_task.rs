// // TODO rename this file to per-node
//
// use std::{ops::Deref, sync::Arc};
//
// use async_compatibility_layer::channel::UnboundedStream;
// use either::Either;
// use futures::{
//     future::{BoxFuture, LocalBoxFuture},
//     FutureExt,
// };
// use hotshot::traits::TestableNodeImplementation;
// use hotshot_task::{
//     event_stream::ChannelStream,
//     global_registry::{GlobalRegistry, HotShotTaskId},
//     task::{
//         FilterEvent, HandleEvent, HandleMessage, HotShotTaskCompleted, HotShotTaskTypes, TaskErr,
//         HST, TS,
//     },
//     task_impls::{HSTWithEvent, HSTWithEventAndMessage, TaskBuilder},
// };
// use hotshot_types::{
//     event::{Event, EventType},
//     traits::node_implementation::NodeType,
// };
// use nll::nll_todo::nll_todo;
// use snafu::Snafu;
// use tracing::log::warn;
//
// use crate::test_errors::ConsensusTestError;
//
// use super::{
//     completion_task::CompletionTask,
//     node_ctx::{NodeCtx, ViewFailed, ViewStatus, ViewSuccess},
//     GlobalTestEvent,
// };
//
// #[derive(Snafu, Debug)]
// pub enum PerNodeSafetyTaskErr {
//     // TODO make this more detailed
//     TooManyFailures,
//     NotEnoughDecides,
// }
// impl TaskErr for PerNodeSafetyTaskErr {}
//
// /// Data availability task state
// ///
// #[derive(Debug)]
// pub struct PerNodeSafetyTask<TYPES: NodeType, I: TestableNodeImplementation<TYPES::ConsensusType, TYPES>> {
//     pub(crate) ctx: NodeCtx<TYPES, I>,
// }
//
// impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES::ConsensusType, TYPES>> Default
//     for PerNodeSafetyTask<TYPES, I>
// {
//     fn default() -> Self {
//         Self {
//             ctx: Default::default(),
//         }
//     }
// }
//
// impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES::ConsensusType, TYPES>> TS
//     for PerNodeSafetyTask<TYPES, I>
// {
// }
//
// /// builder describing custom safety properties
// #[derive(Clone)]
// pub enum PerNodeSafetyTaskDescription<
//     TYPES: NodeType,
//     I: TestableNodeImplementation<TYPES::ConsensusType, TYPES>,
// > {
//     GenProperties(PerNodeSafetyPropertiesDescription),
//     CustomProperties(PerNodeSafetyFinisher<TYPES, I>),
// }
//
// /// properties used for gen
// #[derive(Clone, Debug)]
// pub struct PerNodeSafetyPropertiesDescription {
//     /// number failed views
//     pub num_failed_views: Option<usize>,
//     /// number decide events
//     pub num_decide_events: Option<usize>,
// }
//
// // basic consistency check for single node
// /// Exists for easier overriding
// /// runs at end of all tasks
// #[derive(Clone)]
// #[allow(clippy::type_complexity)]
// pub struct PerNodeSafetyFinisher<
//     TYPES: NodeType,
//     I: TestableNodeImplementation<TYPES::ConsensusType, TYPES>,
// >(
//     pub  Arc<
//         dyn for<'a> Fn(&'a mut NodeCtx<TYPES, I>) -> BoxFuture<'a, Result<(), PerNodeSafetyTaskErr>>
//             + Send
//             + 'static
//             + Sync,
//     >,
// );
//
// impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES::ConsensusType, TYPES>> Deref
//     for PerNodeSafetyFinisher<TYPES, I>
// {
//     type Target = dyn for<'a> Fn(&'a mut NodeCtx<TYPES, I>) -> BoxFuture<'a, Result<(), PerNodeSafetyTaskErr>>
//         + Send
//         + 'static
//         + Sync;
//
//     fn deref(&self) -> &Self::Target {
//         &*self.0
//     }
// }
//
// impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES::ConsensusType, TYPES>>
//     PerNodeSafetyTaskDescription<TYPES, I>
// {
//     fn gen_finisher(self) -> PerNodeSafetyFinisher<TYPES, I> {
//         match self {
//             PerNodeSafetyTaskDescription::CustomProperties(finisher) => finisher,
//             PerNodeSafetyTaskDescription::GenProperties(PerNodeSafetyPropertiesDescription {
//                 num_failed_views,
//                 num_decide_events,
//             }) => PerNodeSafetyFinisher(Arc::new(move |ctx: &mut NodeCtx<TYPES, I>| {
//                 async move {
//                     let mut num_failed = 0;
//                     let mut num_decided = 0;
//                     for (_view_num, view_status) in &ctx.round_results {
//                         match view_status {
//                             ViewStatus::InProgress(_) => {}
//                             ViewStatus::ViewFailed(_) => {
//                                 num_failed += 1;
//                             }
//                             ViewStatus::ViewSuccess(_) => {
//                                 num_decided += 1;
//                             }
//                         }
//                     }
//                     if let Some(num_failed_views) = num_failed_views {
//                         if num_failed >= num_failed_views {
//                             return Err(PerNodeSafetyTaskErr::TooManyFailures);
//                         }
//                     }
//
//                     if let Some(num_decide_events) = num_decide_events {
//                         if num_decided < num_decide_events {
//                             return Err(PerNodeSafetyTaskErr::NotEnoughDecides);
//                         }
//                     }
//                     Ok(())
//                 }
//                 .boxed()
//             })),
//         }
//     }
//
//     /// build
//     pub fn build(
//         self,
//         // registry: &mut GlobalRegistry,
//         // test_event_stream: ChannelStream<GlobalTestEvent>,
//         // hotshot_event_stream: UnboundedStream<Event<TYPES, I::Leaf>>,
//     ) -> TaskGenerator<
//         PerNodeSafetyTask<TYPES, I>
//     > {
//         Box::new(
//             move |state, mut registry, test_event_stream, hotshot_event_stream| {
//                 // TODO this is cursed, there's definitely a better way to do this
//                 let desc = self.clone();
//                 async move {
//                     let test_event_handler = HandleEvent::<PerNodeSafetyTaskTypes<TYPES, I>>(Arc::new(
//                         move |event, mut state| {
//                             let finisher = desc.clone().gen_finisher();
//                             async move {
//                                 match event {
//                                     GlobalTestEvent::ShutDown => {
//                                         let finished = finisher(&mut state.ctx).await;
//                                         let result = match finished {
//                                             Ok(()) => HotShotTaskCompleted::ShutDown,
//                                             Err(err) => HotShotTaskCompleted::Error(Box::new(err)),
//                                         };
//                                         return (Some(result), state);
//                                     }
//                                     _ => {
//                                         unimplemented!()
//                                     }
//                                 }
//                             }
//                             .boxed()
//                         },
//                     ));
//                     let message_handler = HandleMessage::<PerNodeSafetyTaskTypes<TYPES, I>>(Arc::new(
//                         move |msg, mut state| {
//                             async move {
//                                 let Event { view_number, event } = msg;
//                                 match event {
//                                     EventType::Error { error } => {
//                                         // TODO better warn with node idx
//                                         warn!("View {:?} failed for a replica", view_number);
//                                         state.ctx.round_results.insert(
//                                             view_number,
//                                             ViewStatus::ViewFailed(ViewFailed(error)),
//                                         );
//                                     }
//                                     EventType::Decide { leaf_chain, qc } => {
//                                         state.ctx.round_results.insert(
//                                             view_number,
//                                             ViewStatus::ViewSuccess(ViewSuccess {
//                                                 agreed_state:
//
//                                             }),
//                                         );
//                                     }
//                                     // these aren't failures
//                                     EventType::ReplicaViewTimeout { view_number }
//                                     | EventType::NextLeaderViewTimeout { view_number }
//                                     | EventType::ViewFinished { view_number } => todo!(),
//                                     _ => todo!(),
//                                 }
//                                 (None, state)
//                             }
//                             .boxed()
//                         },
//                     ));
//
//                     let builder = TaskBuilder::<PerNodeSafetyTaskTypes<TYPES, I>>::new(
//                         "Safety Check Task".to_string(),
//                     )
//                     .register_event_stream(test_event_stream, FilterEvent::default())
//                     .await
//                     .register_registry(&mut registry)
//                     .await
//                     .register_state(state)
//                     .register_event_handler(test_event_handler)
//                     .register_message_handler(message_handler)
//                     .register_message_stream(hotshot_event_stream);
//                     let task_id = builder.get_task_id().unwrap();
//                     (task_id, PerNodeSafetyTaskTypes::build(builder).launch())
//                 }
//                 .boxed()
//             },
//         )
//     }
// }
//
// // /// Data Availability task types
// pub type PerNodeSafetyTaskTypes<
//     TYPES: NodeType,
//     I: TestableNodeImplementation<TYPES::ConsensusType, TYPES>,
// > = HSTWithEventAndMessage<
//     PerNodeSafetyTaskErr,
//     GlobalTestEvent,
//     ChannelStream<GlobalTestEvent>,
//     Event<TYPES, I::Leaf>,
//     UnboundedStream<Event<TYPES, I::Leaf>>,
//     PerNodeSafetyTask<TYPES, I>,
// >;
