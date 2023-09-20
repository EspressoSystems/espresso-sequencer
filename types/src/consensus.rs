//! Provides the core consensus types

pub use crate::traits::node_implementation::ViewQueue;
pub use crate::utils::{View, ViewInner};
use async_compatibility_layer::async_primitives::subscribable_rwlock::SubscribableRwLock;
use std::collections::HashSet;

use crate::utils::Terminator;
use crate::{
    certificate::QuorumCertificate,
    data::LeafType,
    error::HotShotError,
    traits::{
        metrics::{Counter, Gauge, Histogram, Metrics},
        node_implementation::NodeType,
    },
};
use commit::{Commitment, Committable};
use derivative::Derivative;
use std::{
    collections::{hash_map::Entry, BTreeMap, HashMap},
    sync::Arc,
};
use tracing::error;

/// A type alias for `HashMap<Commitment<T>, T>`
type CommitmentMap<T> = HashMap<Commitment<T>, T>;

/// A reference to the consensus algorithm
///
/// This will contain the state of all rounds.
#[derive(custom_debug::Debug)]
pub struct Consensus<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> {
    /// The phases that are currently loaded in memory
    // TODO(https://github.com/EspressoSystems/hotshot/issues/153): Allow this to be loaded from `Storage`?
    pub state_map: BTreeMap<TYPES::Time, View<TYPES, LEAF>>,

    /// cur_view from pseudocode
    pub cur_view: TYPES::Time,

    /// last view had a successful decide event
    pub last_decided_view: TYPES::Time,

    /// A list of undecided transactions
    pub transactions: Arc<SubscribableRwLock<CommitmentMap<TYPES::Transaction>>>,

    /// A list of transactions we've seen decided, but didn't receive
    pub seen_transactions: HashSet<Commitment<TYPES::Transaction>>,

    /// Map of leaf hash -> leaf
    /// - contains undecided leaves
    /// - includes the MOST RECENT decided leaf
    pub saved_leaves: CommitmentMap<LEAF>,

    /// Saved blocks
    ///
    /// Contains the full block for every leaf in `saved_leaves` if that block is available.
    pub saved_blocks: BlockStore<TYPES::BlockType>,

    /// The `locked_qc` view number
    pub locked_view: TYPES::Time,

    /// the highqc per spec
    pub high_qc: QuorumCertificate<TYPES, LEAF>,

    /// A reference to the metrics trait
    #[debug(skip)]
    pub metrics: Arc<ConsensusMetrics>,

    /// Amount of invalid QCs we've seen since the last commit
    /// Used for metrics.  This resets to 0 on every decide event.
    pub invalid_qc: usize,
}

/// The metrics being collected for the consensus algorithm
#[derive(Debug)]
pub struct ConsensusMetrics {
    /// The current view
    pub current_view: Box<dyn Gauge>,
    /// The duration to collect votes in a view (only applies when this insance is the leader)
    pub vote_validate_duration: Box<dyn Histogram>,
    /// The duration we waited for txns before building the proposal
    pub proposal_wait_duration: Box<dyn Histogram>,
    /// The duration to build the proposal
    pub proposal_build_duration: Box<dyn Histogram>,
    /// The duration of each view, in seconds
    pub view_duration: Box<dyn Histogram>,
    /// Number of views that are in-flight since the last committed view
    pub number_of_views_since_last_commit: Box<dyn Gauge>,
    /// Number of views that are in-flight since the last anchor view
    pub number_of_views_per_decide_event: Box<dyn Histogram>,
    /// Number of invalid QCs between anchors
    pub invalid_qc_views: Box<dyn Histogram>,
    /// Number of views that were discarded since from one achor to the next
    pub discarded_views_per_decide_event: Box<dyn Histogram>,
    /// Views where no proposal was seen from one anchor to the next
    pub empty_views_per_decide_event: Box<dyn Histogram>,
    /// Number of rejected transactions
    pub rejected_transactions: Box<dyn Counter>,
    /// Number of outstanding transactions
    pub outstanding_transactions: Box<dyn Gauge>,
    /// Memory size in bytes of the serialized transactions still outstanding
    pub outstanding_transactions_memory_size: Box<dyn Gauge>,
    /// Number of views that timed out
    pub number_of_timeouts: Box<dyn Counter>,
    /// Total direct messages this node sent out
    pub outgoing_direct_messages: Box<dyn Counter>,
    /// Total broadcasts sent
    pub outgoing_broadcast_messages: Box<dyn Counter>,
    /// Total messages received
    pub direct_messages_received: Box<dyn Counter>,
    /// Total broadcast messages received
    pub broadcast_messages_received: Box<dyn Counter>,
    /// Total number of messages which couldn't be sent
    pub failed_to_send_messages: Box<dyn Counter>,
}

impl ConsensusMetrics {
    /// Create a new instance of this [`ConsensusMetrics`] struct, setting all the counters and gauges
    #[must_use]
    pub fn new(metrics: &dyn Metrics) -> Self {
        Self {
            current_view: metrics.create_gauge(String::from("current_view"), None),
            vote_validate_duration: metrics.create_histogram(
                String::from("vote_validate_duration"),
                Some(String::from("seconds")),
            ),
            proposal_build_duration: metrics.create_histogram(
                String::from("proposal_build_duration"),
                Some(String::from("seconds")),
            ),
            proposal_wait_duration: metrics.create_histogram(
                String::from("proposal_wait_duration"),
                Some(String::from("seconds")),
            ),
            view_duration: metrics
                .create_histogram(String::from("view_duration"), Some(String::from("seconds"))),
            number_of_views_since_last_commit: metrics
                .create_gauge(String::from("number_of_views_since_last_commit"), None),
            number_of_views_per_decide_event: metrics
                .create_histogram(String::from("number_of_views_per_decide_event"), None),
            invalid_qc_views: metrics.create_histogram(String::from("invalid_qc_views"), None),
            discarded_views_per_decide_event: metrics
                .create_histogram(String::from("discarded_views_per_decide_event"), None),
            empty_views_per_decide_event: metrics
                .create_histogram(String::from("empty_views_per_decide_event"), None),
            rejected_transactions: metrics
                .create_counter(String::from("rejected_transactions"), None),
            outstanding_transactions: metrics
                .create_gauge(String::from("outstanding_transactions"), None),
            outstanding_transactions_memory_size: metrics
                .create_gauge(String::from("outstanding_transactions_memory_size"), None),
            outgoing_direct_messages: metrics
                .create_counter(String::from("outgoing_direct_messages"), None),
            outgoing_broadcast_messages: metrics
                .create_counter(String::from("outgoing_broadcast_messages"), None),
            direct_messages_received: metrics
                .create_counter(String::from("direct_messages_received"), None),
            broadcast_messages_received: metrics
                .create_counter(String::from("broadcast_messages_received"), None),
            failed_to_send_messages: metrics
                .create_counter(String::from("failed_to_send_messages"), None),
            number_of_timeouts: metrics
                .create_counter(String::from("number_of_views_timed_out"), None),
        }
    }
}

impl<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> Consensus<TYPES, LEAF> {
    /// increment the current view
    /// NOTE may need to do gc here
    pub fn increment_view(&mut self) -> TYPES::Time {
        self.cur_view += 1;
        self.cur_view
    }

    /// gather information from the parent chain of leafs
    /// # Errors
    /// If the leaf or its ancestors are not found in storage
    pub fn visit_leaf_ancestors<F>(
        &self,
        start_from: TYPES::Time,
        terminator: Terminator<TYPES::Time>,
        ok_when_finished: bool,
        mut f: F,
    ) -> Result<(), HotShotError<TYPES>>
    where
        F: FnMut(&LEAF) -> bool,
    {
        let mut next_leaf = if let Some(view) = self.state_map.get(&start_from) {
            view.get_leaf_commitment()
                .ok_or_else(|| HotShotError::InvalidState {
                    context: format!(
                        "Visited failed view {start_from:?} leaf. Expected successfuil leaf"
                    ),
                })?
        } else {
            return Err(HotShotError::InvalidState {
                context: format!("View {start_from:?} leaf does not exist in state map "),
            });
        };

        while let Some(leaf) = self.saved_leaves.get(&next_leaf) {
            if let Terminator::Exclusive(stop_before) = terminator {
                if stop_before == leaf.get_view_number() {
                    if ok_when_finished {
                        return Ok(());
                    }
                    break;
                }
            }
            next_leaf = leaf.get_parent_commitment();
            if !f(leaf) {
                return Ok(());
            }
            if let Terminator::Inclusive(stop_after) = terminator {
                if stop_after == leaf.get_view_number() {
                    if ok_when_finished {
                        return Ok(());
                    }
                    break;
                }
            }
        }
        Err(HotShotError::LeafNotFound {})
    }

    /// garbage collects based on state change
    /// right now, this removes from both the `saved_blocks`
    /// and `state_map` fields of `Consensus`
    /// # Panics
    /// On inconsistent stored entries
    #[allow(clippy::unused_async)] // async for API compatibility reasons
    pub async fn collect_garbage(
        &mut self,
        old_anchor_view: TYPES::Time,
        new_anchor_view: TYPES::Time,
    ) {
        // state check
        let anchor_entry = self
            .state_map
            .iter()
            .next()
            .expect("INCONSISTENT STATE: anchor leaf not in state map!");
        if *anchor_entry.0 != old_anchor_view {
            error!(
                "Something about GC has failed. Older leaf exists than the previous anchor leaf."
            );
        }
        // perform gc
        self.state_map
            .range(old_anchor_view..new_anchor_view)
            .filter_map(|(_view_number, view)| view.get_block_commitment())
            .for_each(|block| {
                self.saved_blocks.remove(block);
            });
        self.state_map
            .range(old_anchor_view..new_anchor_view)
            .filter_map(|(_view_number, view)| view.get_leaf_commitment())
            .for_each(|leaf| {
                if let Some(removed) = self.saved_leaves.remove(&leaf) {
                    self.saved_blocks.remove(removed.get_deltas_commitment());
                }
            });
        self.state_map = self.state_map.split_off(&new_anchor_view);
    }

    /// return a clone of the internal storage of unclaimed transactions
    #[must_use]
    pub fn get_transactions(&self) -> Arc<SubscribableRwLock<CommitmentMap<TYPES::Transaction>>> {
        self.transactions.clone()
    }

    /// Gets the last decided state
    /// # Panics
    /// if the last decided view's state does not exist in the state map
    /// this should never happen.
    #[must_use]
    pub fn get_decided_leaf(&self) -> LEAF {
        let decided_view_num = self.last_decided_view;
        let view = self.state_map.get(&decided_view_num).unwrap();
        let leaf = view
            .get_leaf_commitment()
            .expect("Decided state not found! Consensus internally inconsistent");
        self.saved_leaves.get(&leaf).unwrap().clone()
    }
}

/// Mapping from block commitments to full blocks.
///
/// Entries in this mapping are reference-counted, so multiple consensus objects can refer to the
/// same block, and the block will only be deleted after _all_ such objects are garbage collected.
/// For example, multiple leaves may temporarily reference the same block on different branches,
/// before all but one branch are ultimately garbage collected.
#[derive(Clone, Debug, Derivative)]
#[derivative(Default(bound = ""))]
pub struct BlockStore<BLOCK: Committable>(HashMap<Commitment<BLOCK>, (BLOCK, u64)>);

impl<BLOCK: Committable> BlockStore<BLOCK> {
    /// Save `block` for later retrieval.
    ///
    /// After calling this function, and before the corresponding call to [`remove`](Self::remove),
    /// `self.get(block.commit())` will return `Some(block)`.
    ///
    /// This function will increment a reference count on the saved block, so that multiple calls to
    /// [`insert`](Self::insert) for the same block result in multiple owning references to the
    /// block. [`remove`](Self::remove) must be called once for each reference before the block will
    /// be deallocated.
    pub fn insert(&mut self, block: BLOCK) {
        self.0
            .entry(block.commit())
            .and_modify(|(_, refcount)| *refcount += 1)
            .or_insert((block, 1));
    }

    /// Get a saved block, if available.
    ///
    /// If a block has been saved with [`insert`](Self::insert), this function will retrieve it. It
    /// may return [`None`] if a block with the given commitment has not been saved or if the block
    /// has been dropped with [`remove`](Self::remove).
    #[must_use]
    pub fn get(&self, block: Commitment<BLOCK>) -> Option<&BLOCK> {
        self.0.get(&block).map(|(block, _)| block)
    }

    /// Drop a reference to a saved block.
    ///
    /// If the block exists and this call drops the last reference to it, the block will be
    /// returned. Otherwise, the return value is [`None`].
    pub fn remove(&mut self, block: Commitment<BLOCK>) -> Option<BLOCK> {
        if let Entry::Occupied(mut e) = self.0.entry(block) {
            let (_, refcount) = e.get_mut();
            *refcount -= 1;
            if *refcount == 0 {
                let (block, _) = e.remove();
                return Some(block);
            }
        }
        None
    }
}
