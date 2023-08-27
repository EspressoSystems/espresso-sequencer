use atomic_enum::atomic_enum;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    sync::{atomic::Ordering, Arc},
};

/// Nit: wish this was for u8 but sadly no
/// Represents the status of a hotshot task
#[atomic_enum]
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskStatus {
    /// the task hasn't started running
    NotStarted = 0,
    /// the task is running
    Running = 1,
    /// NOTE: not useful generally, but VERY useful for byzantine nodes
    /// and testing malfunctions
    /// we'll have a granular way to, from the registry, stop a task momentarily
    /// and inspect/modify its state
    Paused = 2,
    /// the task completed
    Completed = 3,
}

/// The state of a task
/// `AtomicTaskStatus` + book keeping to notify btwn tasks
#[derive(Clone)]
pub struct TaskState {
    /// previous status
    prev: Arc<AtomicTaskStatus>,
    /// next status
    next: Arc<AtomicTaskStatus>,
    // using `std::sync::mutex` here because it's faster than async's version
    // wakers: Arc<Mutex<Vec<Waker>>>,
}

impl Debug for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskState")
            .field("status", &self.get_status())
            .finish()
    }
}
impl Default for TaskState {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskState {
    /// create a new state
    #[must_use]
    pub fn new() -> Self {
        Self {
            prev: Arc::new(TaskStatus::NotStarted.into()),
            next: Arc::new(TaskStatus::NotStarted.into()),
            // wakers: Arc::default(),
        }
    }

    /// create a task state from a task status
    #[must_use]
    pub fn from_status(state: Arc<AtomicTaskStatus>) -> Self {
        let prev_state = AtomicTaskStatus::new(state.load(Ordering::SeqCst));
        Self {
            prev: Arc::new(prev_state),
            next: state,
            // wakers: Arc::default(),
        }
    }

    /// sets the state
    /// # Panics
    /// should never panic unless internally a lock poison happens
    /// this should NOT be possible
    pub fn set_state(&self, state: TaskStatus) {
        self.next.swap(state, Ordering::SeqCst);
        // no panics, so can never be poisoned.
        // let mut wakers = self.wakers.lock().unwrap();

        // drain the wakers
        // for waker in wakers.drain(..) {
        //     waker.wake();
        // }
    }
    /// gets a possibly stale version of the state
    #[must_use]
    pub fn get_status(&self) -> TaskStatus {
        self.next.load(Ordering::SeqCst)
    }
}

// GNARLY bug @jbearer found
// cx gets *really* large in some cases
// impl Stream for TaskState {
//     type Item = TaskStatus;
//
//     #[unstable]
//     fn poll_next(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Option<Self::Item>> {
//         let next = self.next.load(Ordering::SeqCst);
//         let prev = self.prev.swap(next, Ordering::SeqCst);
//         // a new value has been set
//         if prev == next {
//             // no panics, so impossible to be poisoned
//             self.wakers.lock().unwrap().push(cx.waker().clone());
//
//             // no value has been set, poll again later
//             std::task::Poll::Pending
//         } else {
//             std::task::Poll::Ready(Some(next))
//         }
//     }
// }

impl TaskState {
    /// Try to get the next task status.
    #[must_use]
    pub fn try_next(self: std::pin::Pin<&mut Self>) -> Option<TaskStatus> {
        let next = self.next.load(Ordering::SeqCst);
        let prev = self.prev.swap(next, Ordering::SeqCst);
        // a new value has been set
        if prev == next {
            None
        } else {
            // drain the wakers to wake up the stream.
            // we did change value
            // let mut wakers = self.wakers.lock().unwrap();
            // for waker in wakers.drain(..) {
            //     waker.wake();
            // }
            Some(next)
        }
    }
}

#[cfg(test)]
pub mod test {

    // #[cfg(test)]
    // #[cfg_attr(
    //     feature = "tokio-executor",
    //     tokio::test(flavor = "multi_thread", worker_threads = 2)
    // )]
    // #[cfg_attr(feature = "async-std-executor", async_std::test)]
    // async fn test_state_stream() {
    //     setup_logging();
    //
    //     let mut task = crate::task_state::TaskState::new();
    //
    //     let task_dup = task.clone();
    //
    //     async_spawn(async move {
    //         async_sleep(std::time::Duration::from_secs(1)).await;
    //         task_dup.set_state(crate::task_state::TaskStatus::Running);
    //         async_sleep(std::time::Duration::from_secs(1)).await;
    //         task_dup.set_state(crate::task_state::TaskStatus::Paused);
    //         async_sleep(std::time::Duration::from_secs(1)).await;
    //         task_dup.set_state(crate::task_state::TaskStatus::Completed);
    //     });
    //
    //     // spawn new task that sleeps then increments
    //
    //     assert_eq!(
    //         task.try_next().unwrap(),
    //         crate::task_state::TaskStatus::Running
    //     );
    //     assert_eq!(
    //         task.next().unwrap(),
    //         crate::task_state::TaskStatus::Paused
    //     );
    //     assert_eq!(
    //         task.next().unwrap(),
    //         crate::task_state::TaskStatus::Completed
    //     );
    // }
    // TODO test global registry using either global + lazy_static
    // or passing around global registry
}
