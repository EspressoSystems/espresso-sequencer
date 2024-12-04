use std::collections::{btree_map, BTreeMap};

use hotshot_types::{
    drb::{compute_drb_result, DrbResult, DrbSeedInput},
    traits::node_implementation::{ConsensusTime, NodeType},
};
use tokio::{spawn, task::JoinHandle};

/// Number of previous results and seeds to keep
pub const KEEP_PREVIOUS_RESULT_COUNT: u64 = 8;

/// Helper struct to track state of DRB computations
pub struct DrbComputations<TYPES: NodeType> {
    /// Stored results from computations
    results: BTreeMap<TYPES::Epoch, DrbResult>,

    /// Currently live computation
    task: Option<(TYPES::Epoch, JoinHandle<DrbResult>)>,

    /// Stored inputs to computations
    seeds: BTreeMap<TYPES::Epoch, DrbSeedInput>,
}

impl<TYPES: NodeType> DrbComputations<TYPES> {
    #[must_use]
    /// Create a new DrbComputations
    pub fn new() -> Self {
        Self {
            results: BTreeMap::new(),
            task: None,
            seeds: BTreeMap::new(),
        }
    }

    /// If a task is currently live AND has finished, join it and save the result.
    /// If the epoch for the calculation was the same as the provided epoch, return true
    /// If a task is currently live and NOT finished, abort it UNLESS the task epoch is the same as
    /// cur_epoch, in which case keep letting it run and return true.
    /// Return false if a task should be spawned for the given epoch.
    async fn join_or_abort_old_task(&mut self, epoch: TYPES::Epoch) -> bool {
        if let Some((task_epoch, join_handle)) = &mut self.task {
            if join_handle.is_finished() {
                match join_handle.await {
                    Ok(result) => {
                        self.results.insert(*task_epoch, result);
                        let result = *task_epoch == epoch;
                        self.task = None;
                        result
                    }
                    Err(e) => {
                        tracing::error!("error joining DRB computation task: {e:?}");
                        false
                    }
                }
            } else if *task_epoch == epoch {
                true
            } else {
                join_handle.abort();
                self.task = None;
                false
            }
        } else {
            false
        }
    }

    /// Stores a seed for a particular epoch for later use by start_task_if_not_running, called from handle_quorum_proposal_validated_drb_calculation_start
    pub fn store_seed(&mut self, epoch: TYPES::Epoch, drb_seed_input: DrbSeedInput) {
        self.seeds.insert(epoch, drb_seed_input);
    }

    /// Starts a new task. Cancels a current task if that task is not for the provided epoch. Allows a task to continue
    /// running if it was already started for the given epoch. Avoids running the task if we already have a result for
    /// the epoch.
    pub async fn start_task_if_not_running(&mut self, epoch: TYPES::Epoch) {
        // If join_or_abort_task returns true, then we either just completed a task for this epoch, or we currently
        // have a running task for the epoch.
        if self.join_or_abort_old_task(epoch).await {
            return;
        }

        // In case we somehow ended up processing this epoch already, don't start it again
        if self.results.contains_key(&epoch) {
            return;
        }

        if let btree_map::Entry::Occupied(entry) = self.seeds.entry(epoch) {
            let drb_seed_input = *entry.get();
            let new_drb_task = spawn(async move { compute_drb_result::<TYPES>(drb_seed_input) });
            self.task = Some((epoch, new_drb_task));
            entry.remove();
        }
    }

    /// Retrieves the result for a given epoch
    pub fn get_result(&self, epoch: TYPES::Epoch) -> Option<DrbResult> {
        self.results.get(&epoch).copied()
    }

    /// Retrieves the seed for a given epoch
    pub fn get_seed(&self, epoch: TYPES::Epoch) -> Option<DrbSeedInput> {
        self.seeds.get(&epoch).copied()
    }

    /// Garbage collects internal data structures
    pub fn garbage_collect(&mut self, epoch: TYPES::Epoch) {
        if epoch.u64() < KEEP_PREVIOUS_RESULT_COUNT {
            return;
        }

        let retain_epoch = epoch - KEEP_PREVIOUS_RESULT_COUNT;
        // N.B. x.split_off(y) returns the part of the map where key >= y

        // Remove result entries older than EPOCH
        self.results = self.results.split_off(&retain_epoch);

        // Remove result entries older than EPOCH+1
        self.seeds = self.seeds.split_off(&(retain_epoch + 1));
    }
}

impl<TYPES: NodeType> Default for DrbComputations<TYPES> {
    fn default() -> Self {
        Self::new()
    }
}
