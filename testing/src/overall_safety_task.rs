use commit::Commitment;
use either::Either;
use hotshot_task::{event_stream::EventStream, Merge};
use hotshot_task_impls::events::HotShotEvent;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    sync::Arc,
};

use async_compatibility_layer::channel::UnboundedStream;
use futures::FutureExt;
use hotshot::{
    traits::{NodeImplementation, TestableNodeImplementation},
    HotShotError,
};
use hotshot_task::{
    event_stream::ChannelStream,
    task::{FilterEvent, HandleEvent, HandleMessage, HotShotTaskCompleted, HotShotTaskTypes, TS},
    task_impls::{HSTWithEventAndMessage, TaskBuilder},
    MergeN,
};
use hotshot_types::{
    data::{LeafBlockPayload, LeafType},
    error::RoundTimedoutState,
    event::{Event, EventType},
    simple_certificate::QuorumCertificate2,
    traits::node_implementation::NodeType,
};
use snafu::Snafu;

use crate::{test_launcher::TaskGenerator, test_runner::Node};
pub type StateAndBlock<S, B> = (Vec<S>, Vec<B>);

use super::GlobalTestEvent;

/// the status of a view
#[derive(Debug, Clone)]
pub enum ViewStatus {
    /// success
    Ok,
    /// failure
    Failed,
    /// safety violation
    Err(OverallSafetyTaskErr),
    /// in progress
    InProgress,
}

/// possible errors
#[derive(Snafu, Debug, Clone)]
pub enum OverallSafetyTaskErr {
    /// inconsistent txn nums
    InconsistentTxnsNum { map: HashMap<u64, usize> },
    /// too many failed  views
    TooManyFailures {
        /// expected number of failures
        expected: usize,
        /// actual number of failures
        got: usize,
    },
    /// not enough decides
    NotEnoughDecides {
        /// expected number of decides
        expected: usize,
        /// acutal number of decides
        got: usize,
    },
    /// mismatched leaves for a view
    MismatchedLeaf,
    /// mismatched states for a view
    InconsistentStates,
    /// mismatched blocks for a view
    InconsistentBlocks,
}

/// Data availability task state
pub struct OverallSafetyTask<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// handles
    pub handles: Vec<Node<TYPES, I>>,
    /// ctx
    pub ctx: RoundCtx<TYPES, I>,
    /// event stream for publishing safety violations
    pub test_event_stream: ChannelStream<GlobalTestEvent>,
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TS for OverallSafetyTask<TYPES, I> {}

/// Result of running a round of consensus
#[derive(Debug)]
pub struct RoundResult<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> {
    /// Transactions that were submitted
    // pub txns: Vec<TYPES::Transaction>,

    /// Nodes that committed this round
    /// id -> (leaf, qc)
    // TODO GG: isn't it infeasible to store a Vec<LEAF>?
    #[allow(clippy::type_complexity)]
    success_nodes: HashMap<u64, (Vec<LEAF>, QuorumCertificate2<TYPES, LEAF>)>,

    /// Nodes that failed to commit this round
    pub failed_nodes: HashMap<u64, Vec<Arc<HotShotError<TYPES>>>>,

    /// whether or not the round succeeded (for a custom defn of succeeded)
    pub status: ViewStatus,

    /// NOTE: technically a map is not needed
    /// left one anyway for ease of viewing
    /// leaf -> # entries decided on that leaf
    pub leaf_map: HashMap<LEAF, usize>,

    /// block -> # entries decided on that block
    pub block_map: HashMap<Commitment<LeafBlockPayload<LEAF>>, usize>,

    /// state -> # entries decided on that state
    pub state_map: HashMap<<LEAF as LeafType>::MaybeState, usize>,

    pub num_txns_map: HashMap<u64, usize>,
}

impl<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> Default for RoundResult<TYPES, LEAF> {
    fn default() -> Self {
        Self {
            success_nodes: Default::default(),
            failed_nodes: Default::default(),
            leaf_map: Default::default(),
            block_map: Default::default(),
            state_map: Default::default(),
            num_txns_map: Default::default(),
            status: ViewStatus::InProgress,
        }
    }
}

/// smh my head I shouldn't need to implement this
/// Rust doesn't realize I doesn't need to implement default
impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> Default for RoundCtx<TYPES, I> {
    fn default() -> Self {
        Self {
            round_results: Default::default(),
            failed_views: Default::default(),
            successful_views: Default::default(),
        }
    }
}

/// context for a round
/// TODO eventually we want these to just be futures
/// that we poll when things are event driven
/// this context will be passed around
#[derive(Debug)]
pub struct RoundCtx<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// results from previous rounds
    /// view number -> round result
    pub round_results:
        HashMap<TYPES::Time, RoundResult<TYPES, <I as NodeImplementation<TYPES>>::Leaf>>,
    /// during the run view refactor
    pub failed_views: HashSet<TYPES::Time>,
    /// successful views
    pub successful_views: HashSet<TYPES::Time>,
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> RoundCtx<TYPES, I> {
    /// inserts an error into the context
    pub fn insert_error_to_context(
        &mut self,
        view_number: TYPES::Time,
        error: Arc<HotShotError<TYPES>>,
    ) {
        match self.round_results.entry(view_number) {
            Entry::Occupied(mut o) => match o.get_mut().failed_nodes.entry(*view_number) {
                Entry::Occupied(mut o2) => {
                    o2.get_mut().push(error);
                }
                Entry::Vacant(v) => {
                    v.insert(vec![error]);
                }
            },
            Entry::Vacant(v) => {
                let mut round_result = RoundResult::default();
                round_result.failed_nodes.insert(*view_number, vec![error]);
                v.insert(round_result);
            }
        }
    }
}

impl<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> RoundResult<TYPES, LEAF> {
    /// insert into round result
    pub fn insert_into_result(
        &mut self,
        idx: usize,
        result: (Vec<LEAF>, QuorumCertificate2<TYPES, LEAF>),
        maybe_block_size: Option<u64>,
    ) -> Option<LEAF> {
        self.success_nodes.insert(idx as u64, result.clone());

        let maybe_leaf: Option<LEAF> = result.0.into_iter().last();
        if let Some(leaf) = maybe_leaf.clone() {
            match self.leaf_map.entry(leaf.clone()) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    *o.get_mut() += 1;
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(1);
                }
            }

            let (state, payload_commitment) = (leaf.get_state(), leaf.get_payload_commitment());

            match self.state_map.entry(state.clone()) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    *o.get_mut() += 1;
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(1);
                }
            }
            match self.block_map.entry(payload_commitment) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    *o.get_mut() += 1;
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(1);
                }
            }

            if let Some(num_txns) = maybe_block_size {
                match self.num_txns_map.entry(num_txns) {
                    Entry::Occupied(mut o) => {
                        *o.get_mut() += 1;
                    }
                    Entry::Vacant(v) => {
                        v.insert(1);
                    }
                }
            }
        }
        maybe_leaf
    }

    /// determines whether or not the round passes
    /// also do a safety check
    #[allow(clippy::too_many_arguments)]
    pub fn update_status(
        &mut self,
        threshold: usize,
        total_num_nodes: usize,
        key: LEAF,
        check_leaf: bool,
        check_state: bool,
        check_block: bool,
        transaction_threshold: u64,
    ) {
        let num_decided = self.success_nodes.len();
        let num_failed = self.failed_nodes.len();
        let remaining_nodes = total_num_nodes - (num_decided + num_failed);

        if check_leaf && self.leaf_map.len() != 1 {
            self.status = ViewStatus::Err(OverallSafetyTaskErr::MismatchedLeaf);
            return;
        }

        if check_state && self.state_map.len() != 1 {
            self.status = ViewStatus::Err(OverallSafetyTaskErr::InconsistentStates);
            return;
        }

        if check_block && self.block_map.len() != 1 {
            self.status = ViewStatus::Err(OverallSafetyTaskErr::InconsistentBlocks);
            return;
        }

        if transaction_threshold >= 1 {
            if self.num_txns_map.len() > 1 {
                self.status = ViewStatus::Err(OverallSafetyTaskErr::InconsistentTxnsNum {
                    map: self.num_txns_map.clone(),
                });
                return;
            }
            if *self.num_txns_map.iter().last().unwrap().0 < transaction_threshold {
                self.status = ViewStatus::Failed;
                return;
            }
        }

        // check for success
        if num_decided >= threshold {
            // decide on if we've succeeded.
            // if so, set state and return
            // if not, return error
            // if neither, continue through

            let state_key = key.get_state();
            let block_key = key.get_payload_commitment();

            if *self.block_map.get(&block_key).unwrap() == threshold
                && *self.state_map.get(&state_key).unwrap() == threshold
                && *self.leaf_map.get(&key).unwrap() == threshold
            {
                self.status = ViewStatus::Ok;
                return;
            }
        }

        let is_success_possible = remaining_nodes + num_decided >= threshold;
        if !is_success_possible {
            self.status = ViewStatus::Failed;
        }
    }

    /// generate leaves
    pub fn gen_leaves(&self) -> HashMap<LEAF, usize> {
        let mut leaves = HashMap::<LEAF, usize>::new();

        for (leaf_vec, _) in self.success_nodes.values() {
            let most_recent_leaf = leaf_vec.iter().last();
            if let Some(leaf) = most_recent_leaf {
                match leaves.entry(leaf.clone()) {
                    std::collections::hash_map::Entry::Occupied(mut o) => {
                        *o.get_mut() += 1;
                    }
                    std::collections::hash_map::Entry::Vacant(v) => {
                        v.insert(1);
                    }
                }
            }
        }
        leaves
    }
}

/// cross node safety properties
#[derive(Clone)]
pub struct OverallSafetyPropertiesDescription {
    /// required number of successful views
    pub num_successful_views: usize,
    /// whether or not to check the leaf
    pub check_leaf: bool,
    /// whether or not to check the state
    pub check_state: bool,
    /// whether or not to check the block
    pub check_block: bool,
    /// whether or not to check that we have threshold amounts of transactions each block
    /// if 0: don't check
    /// if n > 0, check that at least n transactions are decided upon if such information
    /// is available
    pub transaction_threshold: u64,
    /// num of total rounds allowed to fail
    pub num_failed_views: usize,
    /// threshold calculator. Given number of live and total nodes, provide number of successes
    /// required to mark view as successful
    pub threshold_calculator: Arc<dyn Fn(usize, usize) -> usize + Send + Sync>,
}

impl std::fmt::Debug for OverallSafetyPropertiesDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OverallSafetyPropertiesDescription")
            .field("num successful views", &self.num_successful_views)
            .field("check leaf", &self.check_leaf)
            .field("check_state", &self.check_state)
            .field("check_block", &self.check_block)
            .field("num_failed_rounds_total", &self.num_failed_views)
            .finish()
    }
}

impl Default for OverallSafetyPropertiesDescription {
    fn default() -> Self {
        Self {
            num_successful_views: 50,
            check_leaf: false,
            check_state: true,
            check_block: true,
            num_failed_views: 0,
            transaction_threshold: 0,
            // very strict
            threshold_calculator: Arc::new(|_num_live, num_total| 2 * num_total / 3 + 1),
        }
    }
}

impl OverallSafetyPropertiesDescription {
    /// build a task
    pub fn build<TYPES: NodeType, I: TestableNodeImplementation<TYPES>>(
        self,
    ) -> TaskGenerator<OverallSafetyTask<TYPES, I>> {
        let Self {
            check_leaf,
            check_state,
            check_block,
            num_failed_views: num_failed_rounds_total,
            num_successful_views,
            threshold_calculator,
            transaction_threshold,
        }: Self = self;

        Box::new(move |mut state, mut registry, test_event_stream| {
            async move {
                let event_handler = HandleEvent::<OverallSafetyTaskTypes<TYPES, I>>(Arc::new(
                    move |event, state| {
                        async move {
                            match event {
                                GlobalTestEvent::ShutDown => {
                                    let num_incomplete_views = state.ctx.round_results.len()
                                        - state.ctx.successful_views.len()
                                        - state.ctx.failed_views.len();

                                    if state.ctx.successful_views.len() < num_successful_views {
                                        return (
                                            Some(HotShotTaskCompleted::Error(Box::new(
                                                OverallSafetyTaskErr::NotEnoughDecides {
                                                    got: state.ctx.successful_views.len(),
                                                    expected: num_successful_views,
                                                },
                                            ))),
                                            state,
                                        );
                                    }

                                    if state.ctx.failed_views.len() + num_incomplete_views
                                        >= num_failed_rounds_total
                                    {
                                        return (
                                            Some(HotShotTaskCompleted::Error(Box::new(
                                                OverallSafetyTaskErr::TooManyFailures {
                                                    got: state.ctx.failed_views.len(),
                                                    expected: num_failed_rounds_total,
                                                },
                                            ))),
                                            state,
                                        );
                                    }
                                    // TODO check if we got enough successful views
                                    (Some(HotShotTaskCompleted::ShutDown), state)
                                }
                            }
                        }
                        .boxed()
                    },
                ));

                let message_handler = HandleMessage::<OverallSafetyTaskTypes<TYPES, I>>(Arc::new(
                    move |msg, mut state| {
                        let threshold_calculator = threshold_calculator.clone();
                        async move {

                            let (idx, maybe_event ) : (usize, Either<_, _>)= msg;
                            if let Either::Left(Event { view_number, event }) = maybe_event {
                                let key = match event {
                                    EventType::Error { error } => {
                                        state.ctx.insert_error_to_context(view_number, error);
                                        None
                                    }
                                    EventType::Decide {
                                        leaf_chain,
                                        qc,
                                        block_size: maybe_block_size,
                                    } => {
                                        let paired_up = (leaf_chain.to_vec(), (*qc).clone());
                                        match state.ctx.round_results.entry(view_number) {
                                            Entry::Occupied(mut o) => o.get_mut().insert_into_result(
                                                idx,
                                                paired_up,
                                                maybe_block_size,
                                                ),
                                            Entry::Vacant(v) => {
                                                let mut round_result = RoundResult::default();
                                                let key = round_result.insert_into_result(
                                                    idx,
                                                    paired_up,
                                                    maybe_block_size,
                                                    );
                                                v.insert(round_result);
                                                key
                                            }
                                        }
                                    }
                                    // TODO Emit this event in the consensus task once canceling the timeout task is implemented
                                    EventType::ReplicaViewTimeout { view_number } => {
                                        let error = Arc::new(HotShotError::<TYPES>::ViewTimeoutError {
                                            view_number,
                                            state: RoundTimedoutState::TestCollectRoundEventsTimedOut,
                                        });
                                        state.ctx.insert_error_to_context(view_number, error);
                                        None
                                    }
                                    _ => return (None, state),
                                };

                                // update view count
                                let threshold =
                                    (threshold_calculator)(state.handles.len(), state.handles.len());

                                let view = state.ctx.round_results.get_mut(&view_number).unwrap();

                                if let Some(key) = key {
                                    view.update_status(
                                        threshold,
                                        state.handles.len(),
                                        key,
                                        check_leaf,
                                        check_state,
                                        check_block,
                                        transaction_threshold,
                                        );
                                    match view.status.clone() {
                                        ViewStatus::Ok => {
                                            state.ctx.successful_views.insert(view_number);
                                            if state.ctx.successful_views.len()
                                                >= self.num_successful_views
                                                {
                                                    state
                                                        .test_event_stream
                                                        .publish(GlobalTestEvent::ShutDown)
                                                        .await;
                                                    return (Some(HotShotTaskCompleted::ShutDown), state);
                                                }
                                            return (None, state);
                                        }
                                        ViewStatus::Failed => {
                                            state.ctx.failed_views.insert(view_number);
                                            if state.ctx.failed_views.len() >= self.num_failed_views {
                                                state
                                                    .test_event_stream
                                                    .publish(GlobalTestEvent::ShutDown)
                                                    .await;
                                                return (
                                                    Some(HotShotTaskCompleted::Error(Box::new(
                                                                OverallSafetyTaskErr::TooManyFailures {
                                                                    got: state.ctx.failed_views.len(),
                                                                    expected: num_failed_rounds_total,
                                                                },
                                                                ))),
                                                                state,
                                                                );
                                            }
                                            return (None, state);
                                        }
                                        ViewStatus::Err(e) => {
                                            return (
                                                Some(HotShotTaskCompleted::Error(Box::new(e))),
                                                state,
                                                );
                                        }
                                        ViewStatus::InProgress => {
                                            return (None, state);
                                        }
                                    }
                                }

                            }

                            (None, state)
                        }
                        .boxed()
                    },
                ));

                let mut streams = vec![];
                for handle in &mut state.handles {
                    let s1 =
                        handle
                            .handle
                            .get_event_stream_known_impl(FilterEvent::default())
                            .await
                            .0;
                    let s2 =
                        handle
                            .handle
                            .get_internal_event_stream_known_impl(FilterEvent::default())
                            .await
                            .0;
                    streams.push(
                        Merge::new(s1, s2)
                    );
                }
                let builder = TaskBuilder::<OverallSafetyTaskTypes<TYPES, I>>::new(
                    "Test Overall Safety Task".to_string(),
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
                (task_id, OverallSafetyTaskTypes::build(builder).launch())
            }
            .boxed()
        })
    }
}

/// overall types for safety task
pub type OverallSafetyTaskTypes<TYPES, I> = HSTWithEventAndMessage<
    OverallSafetyTaskErr,
    GlobalTestEvent,
    ChannelStream<GlobalTestEvent>,
    (
        usize,
        Either<Event<TYPES, <I as NodeImplementation<TYPES>>::Leaf>, HotShotEvent<TYPES, I>>,
    ),
    MergeN<
        Merge<
            UnboundedStream<Event<TYPES, <I as NodeImplementation<TYPES>>::Leaf>>,
            UnboundedStream<HotShotEvent<TYPES, I>>,
        >,
    >,
    OverallSafetyTask<TYPES, I>,
>;
