//! Provides the core consensus types

pub use crate::{
    traits::node_implementation::ViewQueue,
    utils::{View, ViewInner},
};
use displaydoc::Display;

use crate::{
    certificate::QuorumCertificate,
    data::LeafType,
    error::HotShotError,
    traits::{
        metrics::{Counter, Gauge, Histogram, Label, Metrics},
        node_implementation::NodeType,
        BlockPayload,
    },
    utils::Terminator,
};
use commit::Commitment;
use derivative::Derivative;
use std::{
    collections::{hash_map::Entry, BTreeMap, HashMap},
    sync::{Arc, Mutex},
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

    /// Map of leaf hash -> leaf
    /// - contains undecided leaves
    /// - includes the MOST RECENT decided leaf
    pub saved_leaves: CommitmentMap<LEAF>,

    /// Saved block payloads
    ///
    /// Contains the block payload for every leaf in `saved_leaves` if that payload is available.
    pub saved_block_payloads: BlockPayloadStore<TYPES::BlockPayload>,

    /// The `locked_qc` view number
    pub locked_view: TYPES::Time,

    /// the highqc per spec
    pub high_qc: QuorumCertificate<TYPES, Commitment<LEAF>>,

    /// A reference to the metrics trait
    pub metrics: Arc<ConsensusMetricsValue>,
}

/// Contains several `ConsensusMetrics` that we're interested in from the consensus interfaces
#[derive(Clone, Debug)]
pub struct ConsensusMetricsValue {
    /// The values that are being tracked
    pub values: Arc<Mutex<InnerConsensusMetrics>>,
    /// The number of last synced synced block height
    pub last_synced_block_height: Box<dyn Gauge>,
    /// The number of last decided view
    pub last_decided_view: Box<dyn Gauge>,
    /// The current view
    pub current_view: Box<dyn Gauge>,
    /// Number of views that are in-flight since the last decided view
    pub number_of_views_since_last_decide: Box<dyn Gauge>,
    /// Number of views that are in-flight since the last anchor view
    pub number_of_views_per_decide_event: Box<dyn Histogram>,
    /// Number of invalid QCs we've seen since the last commit.
    pub invalid_qc: Box<dyn Gauge>,
    /// Number of outstanding transactions
    pub outstanding_transactions: Box<dyn Gauge>,
    /// Memory size in bytes of the serialized transactions still outstanding
    pub outstanding_transactions_memory_size: Box<dyn Gauge>,
    /// Number of views that timed out
    pub number_of_timeouts: Box<dyn Counter>,
}

/// The wrapper with a string name for the networking metrics
#[derive(Clone, Debug)]
pub struct ConsensusMetrics {
    /// a prefix which tracks the name of the metric
    prefix: String,
    /// a map of values
    values: Arc<Mutex<InnerConsensusMetrics>>,
}

/// the set of counters and gauges for the networking metrics
#[derive(Clone, Debug, Default, Display)]
pub struct InnerConsensusMetrics {
    /// All the counters of the networking metrics
    pub counters: HashMap<String, usize>,
    /// All the gauges of the networking metrics
    pub gauges: HashMap<String, usize>,
    /// All the histograms of the networking metrics
    pub histograms: HashMap<String, Vec<f64>>,
    /// All the labels of the networking metrics
    pub labels: HashMap<String, String>,
}

impl ConsensusMetrics {
    #[must_use]
    /// For the creation and naming of gauge, counter, histogram and label.
    pub fn sub(&self, name: String) -> Self {
        let prefix = if self.prefix.is_empty() {
            name
        } else {
            format!("{}-{name}", self.prefix)
        };
        Self {
            prefix,
            values: Arc::clone(&self.values),
        }
    }
}

impl Metrics for ConsensusMetrics {
    fn create_counter(&self, label: String, _unit_label: Option<String>) -> Box<dyn Counter> {
        Box::new(self.sub(label))
    }

    fn create_gauge(&self, label: String, _unit_label: Option<String>) -> Box<dyn Gauge> {
        Box::new(self.sub(label))
    }

    fn create_histogram(&self, label: String, _unit_label: Option<String>) -> Box<dyn Histogram> {
        Box::new(self.sub(label))
    }

    fn create_label(&self, label: String) -> Box<dyn Label> {
        Box::new(self.sub(label))
    }

    fn subgroup(&self, subgroup_name: String) -> Box<dyn Metrics> {
        Box::new(self.sub(subgroup_name))
    }
}

impl Counter for ConsensusMetrics {
    fn add(&self, amount: usize) {
        *self
            .values
            .lock()
            .unwrap()
            .counters
            .entry(self.prefix.clone())
            .or_default() += amount;
    }
}

impl Gauge for ConsensusMetrics {
    fn set(&self, amount: usize) {
        *self
            .values
            .lock()
            .unwrap()
            .gauges
            .entry(self.prefix.clone())
            .or_default() = amount;
    }
    fn update(&self, delta: i64) {
        let mut values = self.values.lock().unwrap();
        let value = values.gauges.entry(self.prefix.clone()).or_default();
        let signed_value = i64::try_from(*value).unwrap_or(i64::MAX);
        *value = usize::try_from(signed_value + delta).unwrap_or(0);
    }
}

impl Histogram for ConsensusMetrics {
    fn add_point(&self, point: f64) {
        self.values
            .lock()
            .unwrap()
            .histograms
            .entry(self.prefix.clone())
            .or_default()
            .push(point);
    }
}

impl Label for ConsensusMetrics {
    fn set(&self, value: String) {
        *self
            .values
            .lock()
            .unwrap()
            .labels
            .entry(self.prefix.clone())
            .or_default() = value;
    }
}

impl ConsensusMetricsValue {
    /// Create a new instance of this [`ConsensusMetricsValue`] struct, setting all the counters and gauges
    #[must_use]
    pub fn new() -> Self {
        let values = Arc::default();
        let metrics: Box<dyn Metrics> = Box::new(ConsensusMetrics {
            prefix: String::new(),
            values: Arc::clone(&values),
        });
        Self {
            values,
            last_synced_block_height: metrics
                .create_gauge(String::from("last_synced_block_height"), None),
            last_decided_view: metrics.create_gauge(String::from("last_decided_view"), None),
            current_view: metrics.create_gauge(String::from("current_view"), None),
            number_of_views_since_last_decide: metrics
                .create_gauge(String::from("number_of_views_since_last_decide"), None),
            number_of_views_per_decide_event: metrics
                .create_histogram(String::from("number_of_views_per_decide_event"), None),
            invalid_qc: metrics.create_gauge(String::from("invalid_qc"), None),
            outstanding_transactions: metrics
                .create_gauge(String::from("outstanding_transactions"), None),
            outstanding_transactions_memory_size: metrics
                .create_gauge(String::from("outstanding_transactions_memory_size"), None),
            number_of_timeouts: metrics.create_counter(String::from("number_of_timeouts"), None),
        }
    }
}

impl Default for ConsensusMetricsValue {
    fn default() -> Self {
        Self::new()
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
    /// right now, this removes from both the `saved_block_payloads`
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
            .filter_map(|(_view_number, view)| view.get_payload_commitment())
            .for_each(|block| {
                self.saved_block_payloads.remove(block);
            });
        self.state_map
            .range(old_anchor_view..new_anchor_view)
            .filter_map(|(_view_number, view)| view.get_leaf_commitment())
            .for_each(|leaf| {
                if let Some(removed) = self.saved_leaves.remove(&leaf) {
                    self.saved_block_payloads
                        .remove(removed.get_payload_commitment());
                }
            });
        self.state_map = self.state_map.split_off(&new_anchor_view);
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

/// Mapping from block payload commitments to the payloads.
///
/// Entries in this mapping are reference-counted, so multiple consensus objects can refer to the
/// same block, and the block will only be deleted after _all_ such objects are garbage collected.
/// For example, multiple leaves may temporarily reference the same block on different branches,
/// before all but one branch are ultimately garbage collected.
#[derive(Clone, Debug, Derivative)]
#[derivative(Default(bound = ""))]
pub struct BlockPayloadStore<PAYLOAD: BlockPayload>(HashMap<Commitment<PAYLOAD>, (PAYLOAD, u64)>);

impl<PAYLOAD: BlockPayload> BlockPayloadStore<PAYLOAD> {
    /// Save payload commitment for later retrieval.
    ///
    /// After calling this function, and before the corresponding call to [`remove`](Self::remove),
    /// `self.get(payload_commitment)` will return `Some(payload)`.
    ///
    /// This function will increment a reference count on the saved payload commitment, so that
    /// multiple calls to [`insert`](Self::insert) for the same payload commitment result in
    /// multiple owning references to the payload commitment. [`remove`](Self::remove) must be
    /// called once for each reference before the payload commitment will be deallocated.
    pub fn insert(&mut self, payload: PAYLOAD) {
        self.0
            .entry(payload.commit())
            .and_modify(|(_, refcount)| *refcount += 1)
            .or_insert((payload, 1));
    }

    /// Get a saved set of transaction commitments, if available.
    ///
    /// If a set of transaction commitments has been saved with [`insert`](Self::insert), this
    /// function will retrieve it. It may return [`None`] if a block with the given commitment has
    /// not been saved or if the block has been dropped with [`remove`](Self::remove).
    #[must_use]
    pub fn get(&self, payload_commitment: Commitment<PAYLOAD>) -> Option<&PAYLOAD> {
        self.0.get(&payload_commitment).map(|(payload, _)| payload)
    }

    /// Drop a reference to a saved set of transaction commitments.
    ///
    /// If the set exists and this call drops the last reference to it, the set will be returned,
    /// Otherwise, the return value is [`None`].
    pub fn remove(&mut self, payload_commitment: Commitment<PAYLOAD>) -> Option<PAYLOAD> {
        if let Entry::Occupied(mut e) = self.0.entry(payload_commitment) {
            let (_, refcount) = e.get_mut();
            *refcount -= 1;
            if *refcount == 0 {
                let (payload, _) = e.remove();
                return Some(payload);
            }
        }
        None
    }
}
