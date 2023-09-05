use futures::future::{join_all, BoxFuture};

use crate::{
    global_registry::{GlobalRegistry, HotShotTaskId},
    task::HotShotTaskCompleted,
};

// TODO use genericarray + typenum to make this use the number of tasks as a parameter
/// runner for tasks
/// `N` specifies the number of tasks to ensure that the user
/// doesn't forget how many tasks they wished to add.
pub struct TaskRunner
// <
//     const N: usize,
// >
{
    /// internal set of tasks to launch
    tasks: Vec<(
        HotShotTaskId,
        String,
        BoxFuture<'static, HotShotTaskCompleted>,
    )>,
    /// global registry
    pub registry: GlobalRegistry,
}

impl Default for TaskRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskRunner /* <N> */ {
    /// create new runner
    #[must_use]
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            registry: GlobalRegistry::new(),
        }
    }

    // `name` is for logging purposes only and may be duplicated or inconsistent.
    /// to support builder pattern
    #[must_use]
    pub fn add_task(
        mut self,
        id: HotShotTaskId,
        name: String,
        task: BoxFuture<'static, HotShotTaskCompleted>,
    ) -> TaskRunner {
        self.tasks.push((id, name, task));
        self
    }

    /// returns a `Vec` because type isn't known
    pub async fn launch(self) -> Vec<(String, HotShotTaskCompleted)> {
        let names = self
            .tasks
            .iter()
            .map(|(_id, name, _)| name.clone())
            .collect::<Vec<_>>();
        let result = join_all(self.tasks.into_iter().map(|(_, _, task)| task)).await;

        names.into_iter().zip(result).collect::<Vec<_>>()
    }
}
