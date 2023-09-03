use async_lock::RwLock;
use either::Either;
use futures::{future::BoxFuture, FutureExt};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Deref,
    sync::Arc,
};

use crate::task_state::{TaskState, TaskStatus};

/// function to shut down gobal registry
#[derive(Clone)]
pub struct ShutdownFn(pub Arc<dyn Fn() -> BoxFuture<'static, ()> + Sync + Send>);

// TODO this might cleaner as `run()`
// but then this pattern should change everywhere
impl Deref for ShutdownFn {
    type Target = dyn Fn() -> BoxFuture<'static, ()> + Sync + Send;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

/// id of task. Usize instead of u64 because
/// used for primarily for indexing
pub type HotShotTaskId = usize;

/// the global registry provides a place to:
/// - inquire about the state of various tasks
/// - gracefully shut down tasks
#[derive(Debug, Clone)]
pub struct GlobalRegistry {
    /// up-to-date shared list of statuses
    /// only used if `state_cache` is out of date
    /// or if appending
    state_list: Arc<RwLock<BTreeMap<HotShotTaskId, (TaskState, String)>>>,
    /// possibly stale read version of state
    /// NOTE: must include entire state in order to
    /// support both incrementing and reading.
    /// Writing to the status should gracefully shut down the task
    state_cache: BTreeMap<HotShotTaskId, (TaskState, String)>,
}

/// function to modify state
#[allow(clippy::type_complexity)]
struct Modifier(Box<dyn Fn(&TaskState) -> Either<TaskStatus, bool> + Send>);

impl Default for GlobalRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalRegistry {
    /// create new registry
    #[must_use]
    pub fn new() -> Self {
        Self {
            state_list: Arc::new(RwLock::new(BTreeMap::default())),
            state_cache: BTreeMap::default(),
        }
    }

    /// register with the global registry
    /// return a function to the caller (task) that can be used to deregister
    /// returns a function to call to shut down the task
    /// and the unique identifier of the task
    pub async fn register(&mut self, name: &str, status: TaskState) -> (ShutdownFn, HotShotTaskId) {
        let mut list = self.state_list.write().await;
        let next_id = list
            .last_key_value()
            .map(|(k, _v)| k)
            .copied()
            .unwrap_or_default()
            + 1;
        let new_entry = (status.clone(), name.to_string());
        let new_entry_dup = new_entry.0.clone();
        list.insert(next_id, new_entry.clone());

        self.state_cache.insert(next_id, new_entry);

        let shutdown_fn = ShutdownFn(Arc::new(move || {
            new_entry_dup.set_state(TaskStatus::Completed);
            async move {}.boxed()
        }));
        (shutdown_fn, next_id)
    }

    /// update the cache
    async fn update_cache(&mut self) {
        // NOTE: this can be done much more cleverly
        // avoid one intersection by comparing max keys (constant time op vs O(n + m))
        // and debatable how often the other op needs to be run
        // probably much much less often
        let list = self.state_list.read().await;
        let list_keys: BTreeSet<usize> = list.keys().copied().collect();
        let cache_keys: BTreeSet<usize> = self.state_cache.keys().copied().collect();
        // bleh not as efficient
        let missing_key_list = list_keys.difference(&cache_keys);
        let expired_key_list = cache_keys.difference(&list_keys);

        for expired_key in expired_key_list {
            self.state_cache.remove(expired_key);
        }

        for key in missing_key_list {
            // technically shouldn't be possible for this to be none since
            // we have a read lock
            // nevertheless, this seems easier
            if let Some(val) = list.get(key) {
                self.state_cache.insert(*key, val.clone());
            }
        }
    }

    /// internal function to run `modifier` on `uid`
    /// if it exists
    async fn operate_on_task(
        &mut self,
        uid: HotShotTaskId,
        modifier: Modifier,
    ) -> Either<TaskStatus, bool> {
        // the happy path
        if let Some(ele) = self.state_cache.get(&uid) {
            modifier.0(&ele.0)
        }
        // the sad path
        else {
            self.update_cache().await;
            if let Some(ele) = self.state_cache.get(&uid) {
                modifier.0(&ele.0)
            } else {
                Either::Right(false)
            }
        }
    }

    /// set `uid`'s state to paused
    /// returns true upon success and false if `uid` is not registered
    pub async fn pause_task(&mut self, uid: HotShotTaskId) -> bool {
        let modifier = Modifier(Box::new(|state| {
            state.set_state(TaskStatus::Paused);
            Either::Right(true)
        }));
        match self.operate_on_task(uid, modifier).await {
            Either::Left(_) => unreachable!(),
            Either::Right(b) => b,
        }
    }

    /// set `uid`'s state to running
    /// returns true upon success and false if `uid` is not registered
    pub async fn run_task(&mut self, uid: HotShotTaskId) -> bool {
        let modifier = Modifier(Box::new(|state| {
            state.set_state(TaskStatus::Running);
            Either::Right(true)
        }));
        match self.operate_on_task(uid, modifier).await {
            Either::Left(_) => unreachable!(),
            Either::Right(b) => b,
        }
    }

    /// if the `uid` is registered with the global registry
    /// return its task status
    /// this is a way to subscribe to state changes from the taskstatus
    /// since `HotShotTaskStatus` implements stream
    pub async fn get_task_state(&mut self, uid: HotShotTaskId) -> Option<TaskStatus> {
        let modifier = Modifier(Box::new(|state| Either::Left(state.get_status())));
        match self.operate_on_task(uid, modifier).await {
            Either::Left(state) => Some(state),
            Either::Right(false) => None,
            Either::Right(true) => unreachable!(),
        }
    }

    /// shut down a task from a different thread
    /// returns true if succeeded
    /// returns false if the task does not exist
    pub async fn shutdown_task(&mut self, uid: usize) -> bool {
        let modifier = Modifier(Box::new(|state| {
            state.set_state(TaskStatus::Completed);
            Either::Right(true)
        }));
        let result = match self.operate_on_task(uid, modifier).await {
            Either::Left(_) => unreachable!(),
            Either::Right(b) => b,
        };
        let mut list = self.state_list.write().await;
        list.remove(&uid);
        result
    }

    /// checks if all registered tasks have completed
    pub async fn is_shutdown(&mut self) -> bool {
        let task_list = self.state_list.read().await;
        for task in (*task_list).values() {
            if task.0.get_status() != TaskStatus::Completed {
                return false;
            }
        }
        true
    }

    /// shut down all tasks in registry
    pub async fn shutdown_all(&mut self) {
        let mut task_list = self.state_list.write().await;
        while let Some((_uid, task)) = task_list.pop_last() {
            task.0.set_state(TaskStatus::Completed);
        }
    }
}
