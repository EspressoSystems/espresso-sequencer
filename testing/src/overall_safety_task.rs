// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    sync::Arc,
};

use anyhow::Result;
use async_broadcast::Sender;
use async_lock::RwLock;
use async_trait::async_trait;
use hotshot::{traits::TestableNodeImplementation, HotShotError};
use hotshot_types::{
    data::Leaf2,
    error::RoundTimedoutState,
    event::{Event, EventType, LeafChain},
    simple_certificate::QuorumCertificate2,
    traits::{
        election::Membership,
        node_implementation::{ConsensusTime, NodeType, Versions},
    },
    vid::VidCommitment,
};
use thiserror::Error;
use tracing::error;

use crate::{
    test_runner::Node,
    test_task::{TestEvent, TestResult, TestTaskState},
};
/// convenience type alias for state and block
pub type StateAndBlock<S, B> = (Vec<S>, Vec<B>);

/// the status of a view
#[derive(Debug, Clone)]
pub enum ViewStatus<TYPES: NodeType> {
    /// success
    Ok,
    /// failure
    Failed,
    /// safety violation
    Err(OverallSafetyTaskErr<TYPES>),
    /// in progress
    InProgress,
}

/// possible errors
#[derive(Error, Debug, Clone)]
pub enum OverallSafetyTaskErr<TYPES: NodeType> {
    #[error("Mismatched leaf")]
    MismatchedLeaf,

    #[error("Inconsistent blocks")]
    InconsistentBlocks,

    #[error("Inconsistent number of transactions: {map:?}")]
    InconsistentTxnsNum { map: HashMap<u64, usize> },

    #[error("Not enough decides: got: {got}, expected: {expected}")]
    NotEnoughDecides { got: usize, expected: usize },

    #[error("Too many view failures: {0:?}")]
    TooManyFailures(HashSet<TYPES::View>),

    #[error("Inconsistent failed views: expected: {expected_failed_views:?}, actual: {actual_failed_views:?}")]
    InconsistentFailedViews {
        expected_failed_views: Vec<TYPES::View>,
        actual_failed_views: HashSet<TYPES::View>,
    },
    #[error(
        "Not enough round results: results_count: {results_count}, views_count: {views_count}"
    )]
    NotEnoughRoundResults {
        results_count: usize,
        views_count: usize,
    },

    #[error("View timed out")]
    ViewTimeout,
}

/// Data availability task state
pub struct OverallSafetyTask<TYPES: NodeType, I: TestableNodeImplementation<TYPES>, V: Versions> {
    /// handles
    pub handles: Arc<RwLock<Vec<Node<TYPES, I, V>>>>,
    /// ctx
    pub ctx: RoundCtx<TYPES>,
    /// configure properties
    pub properties: OverallSafetyPropertiesDescription<TYPES>,
    /// error
    pub error: Option<Box<OverallSafetyTaskErr<TYPES>>>,
    /// sender to test event channel
    pub test_sender: Sender<TestEvent>,
    /// Number of blocks in an epoch, zero means there are no epochs
    pub epoch_height: u64,
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>, V: Versions>
    OverallSafetyTask<TYPES, I, V>
{
    async fn handle_view_failure(&mut self, num_failed_views: usize, view_number: TYPES::View) {
        let expected_views_to_fail = &mut self.properties.expected_views_to_fail;

        self.ctx.failed_views.insert(view_number);
        if self.ctx.failed_views.len() > num_failed_views {
            let _ = self.test_sender.broadcast(TestEvent::Shutdown).await;
            self.error = Some(Box::new(OverallSafetyTaskErr::<TYPES>::TooManyFailures(
                self.ctx.failed_views.clone(),
            )));
        } else if !expected_views_to_fail.is_empty() {
            match expected_views_to_fail.entry(view_number) {
                Entry::Occupied(mut view_seen) => {
                    *view_seen.get_mut() = true;
                }
                Entry::Vacant(_v) => {
                    let _ = self.test_sender.broadcast(TestEvent::Shutdown).await;
                    self.error = Some(Box::new(
                        OverallSafetyTaskErr::<TYPES>::InconsistentFailedViews {
                            expected_failed_views: expected_views_to_fail.keys().cloned().collect(),
                            actual_failed_views: self.ctx.failed_views.clone(),
                        },
                    ));
                }
            }
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>, V: Versions> TestTaskState
    for OverallSafetyTask<TYPES, I, V>
{
    type Event = Event<TYPES>;

    /// Handles an event from one of multiple receivers.
    async fn handle_event(&mut self, (message, id): (Self::Event, usize)) -> Result<()> {
        let OverallSafetyPropertiesDescription::<TYPES> {
            check_leaf,
            check_block,
            num_failed_views,
            num_successful_views,
            transaction_threshold,
            ..
        }: OverallSafetyPropertiesDescription<TYPES> = self.properties.clone();
        let Event { view_number, event } = message;
        let key = match event {
            EventType::Error { error } => {
                self.ctx
                    .insert_error_to_context(view_number, id, error.clone());
                None
            }
            EventType::Decide {
                leaf_chain,
                qc,
                block_size: maybe_block_size,
            } => {
                // Skip the genesis leaf.
                if leaf_chain.last().unwrap().leaf.view_number() == TYPES::View::genesis() {
                    return Ok(());
                }
                let paired_up = (leaf_chain.to_vec(), (*qc).clone());
                match self.ctx.round_results.entry(view_number) {
                    Entry::Occupied(mut o) => {
                        let entry = o.get_mut();
                        entry.insert_into_result(id, paired_up, maybe_block_size)
                    }
                    Entry::Vacant(v) => {
                        let mut round_result = RoundResult::default();
                        let key = round_result.insert_into_result(id, paired_up, maybe_block_size);
                        v.insert(round_result);
                        key
                    }
                }
            }
            EventType::ReplicaViewTimeout { view_number } => {
                let error = Arc::new(HotShotError::<TYPES>::ViewTimedOut {
                    view_number,
                    state: RoundTimedoutState::TestCollectRoundEventsTimedOut,
                });
                self.ctx.insert_error_to_context(view_number, id, error);
                None
            }
            _ => return Ok(()),
        };

        if let Some(ref key) = key {
            match (
                key.epoch(self.epoch_height).map(|x| *x),
                self.ctx.latest_epoch,
            ) {
                (Some(key_epoch), Some(latest_epoch)) => {
                    if key_epoch > latest_epoch {
                        self.ctx.latest_epoch = Some(key_epoch);
                    }
                }
                (Some(key_epoch), None) => {
                    self.ctx.latest_epoch = Some(key_epoch);
                }
                _ => {}
            }
        }

        let epoch = self.ctx.latest_epoch.map(TYPES::Epoch::new);
        let memberships_arc = Arc::clone(
            &self
                .handles
                .read()
                .await
                .first()
                .unwrap()
                .handle
                .memberships,
        );
        let memberships_reader = memberships_arc.read().await;
        let len = memberships_reader.total_nodes(epoch);

        // update view count
        let threshold = memberships_reader.success_threshold(epoch).get() as usize;
        drop(memberships_reader);
        drop(memberships_arc);

        let view = self.ctx.round_results.get_mut(&view_number).unwrap();
        if let Some(key) = key {
            view.update_status(
                threshold,
                len,
                &key,
                check_leaf,
                check_block,
                transaction_threshold,
            );
            match view.status.clone() {
                ViewStatus::Ok => {
                    self.ctx.successful_views.insert(view_number);
                    // if a view succeeds remove it from the failed views
                    self.ctx.failed_views.remove(&view_number);
                    if self.ctx.successful_views.len() >= num_successful_views {
                        let _ = self.test_sender.broadcast(TestEvent::Shutdown).await;
                    }
                    return Ok(());
                }
                ViewStatus::Failed => {
                    self.handle_view_failure(num_failed_views, view_number)
                        .await;

                    return Ok(());
                }
                ViewStatus::Err(e) => {
                    let _ = self.test_sender.broadcast(TestEvent::Shutdown).await;
                    self.error = Some(Box::new(e));
                    return Ok(());
                }
                ViewStatus::InProgress => {
                    return Ok(());
                }
            }
        } else if view.check_if_failed(threshold, len) {
            view.status = ViewStatus::Failed;
            self.handle_view_failure(num_failed_views, view_number)
                .await;
            return Ok(());
        }
        Ok(())
    }

    async fn check(&self) -> TestResult {
        if let Some(e) = &self.error {
            return TestResult::Fail(e.clone());
        }

        let OverallSafetyPropertiesDescription::<TYPES> {
            check_leaf: _,
            check_block: _,
            num_failed_views: num_failed_rounds_total,
            num_successful_views,
            threshold_calculator: _,
            transaction_threshold: _,
            expected_views_to_fail,
        }: OverallSafetyPropertiesDescription<TYPES> = self.properties.clone();

        let views_count = self.ctx.failed_views.len() + self.ctx.successful_views.len();
        let results_count = self.ctx.round_results.len();

        // This can cause tests to crash if we do the subtracting to get `num_incomplete_views` below
        // So lets fail return an error instead
        // Use this check instead of saturating_sub as that could hide a real problem
        if views_count > results_count {
            return TestResult::Fail(Box::new(
                OverallSafetyTaskErr::<TYPES>::NotEnoughRoundResults {
                    results_count,
                    views_count,
                },
            ));
        }
        let num_incomplete_views = results_count - views_count;

        if self.ctx.successful_views.len() < num_successful_views {
            return TestResult::Fail(Box::new(OverallSafetyTaskErr::<TYPES>::NotEnoughDecides {
                got: self.ctx.successful_views.len(),
                expected: num_successful_views,
            }));
        }

        if self.ctx.failed_views.len() + num_incomplete_views > num_failed_rounds_total {
            return TestResult::Fail(Box::new(OverallSafetyTaskErr::<TYPES>::TooManyFailures(
                self.ctx.failed_views.clone(),
            )));
        }

        if !expected_views_to_fail
            .values()
            .all(|&view_failed| view_failed)
        {
            return TestResult::Fail(Box::new(
                OverallSafetyTaskErr::<TYPES>::InconsistentFailedViews {
                    actual_failed_views: self.ctx.failed_views.clone(),
                    expected_failed_views: expected_views_to_fail.keys().cloned().collect(),
                },
            ));
        }

        // We should really be able to include a check like this:
        //
        //        if self.ctx.failed_views.len() < num_failed_rounds_total {
        //            return TestResult::Fail(Box::new(OverallSafetyTaskErr::<TYPES>::NotEnoughFailures {
        //                expected: num_failed_rounds_total,
        //                failed_views: self.ctx.failed_views.clone(),
        //            }));
        //        }
        //
        // but we have several tests where it's not possible to fail pin down an exact number of failures (just from async timing issues, if nothing else). Ideally, we should refactor some of the failure count logic for this.

        TestResult::Pass
    }
}

/// Result of running a round of consensus
#[derive(Debug)]
pub struct RoundResult<TYPES: NodeType> {
    /// Transactions that were submitted
    // pub txns: Vec<TYPES::Transaction>,

    /// Nodes that committed this round
    /// id -> (leaf, qc)
    success_nodes: HashMap<u64, (LeafChain<TYPES>, QuorumCertificate2<TYPES>)>,

    /// Nodes that failed to commit this round
    pub failed_nodes: HashMap<u64, Arc<HotShotError<TYPES>>>,

    /// whether or not the round succeeded (for a custom defn of succeeded)
    pub status: ViewStatus<TYPES>,

    /// NOTE: technically a map is not needed
    /// left one anyway for ease of viewing
    /// leaf -> # entries decided on that leaf
    pub leaf_map: HashMap<Leaf2<TYPES>, usize>,

    /// block -> # entries decided on that block
    pub block_map: HashMap<VidCommitment, usize>,

    /// number of transactions -> number of nodes reporting that number
    pub num_txns_map: HashMap<u64, usize>,
}

impl<TYPES: NodeType> Default for RoundResult<TYPES> {
    fn default() -> Self {
        Self {
            success_nodes: HashMap::default(),
            failed_nodes: HashMap::default(),
            leaf_map: HashMap::default(),
            block_map: HashMap::default(),
            num_txns_map: HashMap::default(),
            status: ViewStatus::InProgress,
        }
    }
}

/// smh my head I shouldn't need to implement this
/// Rust doesn't realize I doesn't need to implement default
impl<TYPES: NodeType> Default for RoundCtx<TYPES> {
    fn default() -> Self {
        Self {
            round_results: HashMap::default(),
            failed_views: HashSet::default(),
            successful_views: HashSet::default(),
            latest_epoch: None,
        }
    }
}

/// context for a round
/// TODO eventually we want these to just be futures
/// that we poll when things are event driven
/// this context will be passed around
#[derive(Debug)]
pub struct RoundCtx<TYPES: NodeType> {
    /// results from previous rounds
    /// view number -> round result
    pub round_results: HashMap<TYPES::View, RoundResult<TYPES>>,
    /// during the run view refactor
    pub failed_views: HashSet<TYPES::View>,
    /// successful views
    pub successful_views: HashSet<TYPES::View>,
    /// latest epoch, updated when a leaf with a higher epoch is seen
    pub latest_epoch: Option<u64>,
}

impl<TYPES: NodeType> RoundCtx<TYPES> {
    /// inserts an error into the context
    pub fn insert_error_to_context(
        &mut self,
        view_number: TYPES::View,
        idx: usize,
        error: Arc<HotShotError<TYPES>>,
    ) {
        match self.round_results.entry(view_number) {
            Entry::Occupied(mut o) => match o.get_mut().failed_nodes.entry(idx as u64) {
                Entry::Occupied(mut o2) => {
                    *o2.get_mut() = error;
                }
                Entry::Vacant(v) => {
                    v.insert(error);
                }
            },
            Entry::Vacant(v) => {
                let mut round_result = RoundResult::default();
                round_result.failed_nodes.insert(idx as u64, error);
                v.insert(round_result);
            }
        }
    }
}

impl<TYPES: NodeType> RoundResult<TYPES> {
    /// insert into round result
    #[allow(clippy::unit_arg)]
    pub fn insert_into_result(
        &mut self,
        idx: usize,
        result: (LeafChain<TYPES>, QuorumCertificate2<TYPES>),
        maybe_block_size: Option<u64>,
    ) -> Option<Leaf2<TYPES>> {
        self.success_nodes.insert(idx as u64, result.clone());

        let maybe_leaf = result.0.first();
        if let Some(leaf_info) = maybe_leaf {
            let leaf = &leaf_info.leaf;
            match self.leaf_map.entry(leaf.clone()) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    *o.get_mut() += 1;
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(1);
                }
            }

            let payload_commitment = leaf.payload_commitment();

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
            return Some(leaf.clone());
        }
        None
    }

    /// check if the test failed due to not enough nodes getting through enough views
    pub fn check_if_failed(&mut self, threshold: usize, total_num_nodes: usize) -> bool {
        let num_failed = self.failed_nodes.len();
        total_num_nodes - num_failed < threshold
    }
    /// determines whether or not the round passes
    /// also do a safety check
    #[allow(clippy::too_many_arguments, clippy::let_unit_value)]
    pub fn update_status(
        &mut self,
        threshold: usize,
        total_num_nodes: usize,
        key: &Leaf2<TYPES>,
        check_leaf: bool,
        check_block: bool,
        transaction_threshold: u64,
    ) {
        let num_decided = self.success_nodes.len();
        let num_failed = self.failed_nodes.len();

        if check_leaf && self.leaf_map.len() != 1 {
            let (quorum_leaf, count) = self
                .leaf_map
                .iter()
                .max_by(|(_, v), (_, other_val)| v.cmp(other_val))
                .unwrap();
            if *count >= threshold {
                for leaf in self.leaf_map.keys() {
                    if leaf.view_number() > quorum_leaf.view_number() {
                        error!("LEAF MAP (that is mismatched) IS: {:?}", self.leaf_map);
                        self.status = ViewStatus::Err(OverallSafetyTaskErr::MismatchedLeaf);
                        return;
                    }
                }
            }
        }

        if check_block && self.block_map.len() != 1 {
            self.status = ViewStatus::Err(OverallSafetyTaskErr::InconsistentBlocks);
            error!("Check blocks failed.  Block map IS: {:?}", self.block_map);
            return;
        }

        if transaction_threshold >= 1 {
            if self.num_txns_map.len() > 1 {
                self.status = ViewStatus::Err(OverallSafetyTaskErr::InconsistentTxnsNum {
                    map: self.num_txns_map.clone(),
                });
                return;
            }
            if let Some((n_txn, _)) = self.num_txns_map.iter().last() {
                if *n_txn < transaction_threshold {
                    tracing::error!("not enough transactions for view {:?}", key.view_number());
                    self.status = ViewStatus::Failed;
                    return;
                }
            }
        }

        // check for success
        if num_decided >= threshold {
            // decide on if we've succeeded.
            // if so, set state and return
            // if not, return error
            // if neither, continue through

            let block_key = key.payload_commitment();

            if *self.block_map.get(&block_key).unwrap() == threshold
                && *self.leaf_map.get(key).unwrap() == threshold
            {
                self.status = ViewStatus::Ok;
                return;
            }
        }

        let is_success_possible = total_num_nodes - num_failed >= threshold;
        if !is_success_possible {
            self.status = ViewStatus::Failed;
        }
    }

    /// generate leaves
    #[must_use]
    pub fn gen_leaves(&self) -> HashMap<Leaf2<TYPES>, usize> {
        let mut leaves = HashMap::<Leaf2<TYPES>, usize>::new();

        for (leaf_vec, _) in self.success_nodes.values() {
            let most_recent_leaf = leaf_vec.iter().last();
            if let Some(leaf_info) = most_recent_leaf {
                match leaves.entry(leaf_info.leaf.clone()) {
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
pub struct OverallSafetyPropertiesDescription<TYPES: NodeType> {
    /// required number of successful views
    pub num_successful_views: usize,
    /// whether or not to check the leaf
    pub check_leaf: bool,
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
    /// pass in the views that we expect to fail
    pub expected_views_to_fail: HashMap<TYPES::View, bool>,
}

impl<TYPES: NodeType> std::fmt::Debug for OverallSafetyPropertiesDescription<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OverallSafetyPropertiesDescription")
            .field("num successful views", &self.num_successful_views)
            .field("check leaf", &self.check_leaf)
            .field("check_block", &self.check_block)
            .field("num_failed_rounds_total", &self.num_failed_views)
            .field("transaction_threshold", &self.transaction_threshold)
            .field("expected views to fail", &self.expected_views_to_fail)
            .finish_non_exhaustive()
    }
}

impl<TYPES: NodeType> Default for OverallSafetyPropertiesDescription<TYPES> {
    fn default() -> Self {
        Self {
            num_successful_views: 50,
            check_leaf: false,
            check_block: true,
            num_failed_views: 0,
            transaction_threshold: 0,
            // very strict
            threshold_calculator: Arc::new(|_num_live, num_total| 2 * num_total / 3 + 1),
            expected_views_to_fail: HashMap::new(),
        }
    }
}
