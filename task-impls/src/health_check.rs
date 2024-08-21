// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{
    collections::{hash_map::Entry, HashMap},
    marker::PhantomData,
    sync::Arc,
    time::Instant,
};

use anyhow::Result;
use async_broadcast::{Receiver, Sender};
use async_lock::Mutex;
use async_trait::async_trait;
use hotshot_task::task::TaskState;
use hotshot_types::traits::node_implementation::NodeType;

use crate::events::{HotShotEvent, HotShotTaskCompleted};

/// Health event task, recieve heart beats from other tasks
pub struct HealthCheckTaskState<TYPES: NodeType> {
    /// Node id
    pub node_id: u64,
    /// Map of the task id to timestamp of last heartbeat
    pub task_ids_heartbeat_timestamp: Mutex<HashMap<String, Instant>>,
    /// Specify the time we start logging when no heartbeat received
    pub heartbeat_timeout_duration_in_secs: u64,
    /// phantom
    pub _phantom: PhantomData<TYPES>,
}

impl<TYPES: NodeType> HealthCheckTaskState<TYPES> {
    /// Create a new instance of task state with task ids pre populated
    #[must_use]
    pub fn new(
        node_id: u64,
        task_ids: Vec<String>,
        heartbeat_timeout_duration_in_secs: u64,
    ) -> Self {
        let time = Instant::now();
        let mut task_ids_heartbeat_timestamp: HashMap<String, Instant> = HashMap::new();
        for task_id in task_ids {
            task_ids_heartbeat_timestamp.insert(task_id, time);
        }

        HealthCheckTaskState {
            node_id,
            task_ids_heartbeat_timestamp: Mutex::new(task_ids_heartbeat_timestamp),
            heartbeat_timeout_duration_in_secs,
            _phantom: std::marker::PhantomData,
        }
    }
    /// Handles only HeartBeats and updates the timestamp for a task
    pub async fn handle(
        &mut self,
        event: &Arc<HotShotEvent<TYPES>>,
    ) -> Option<HotShotTaskCompleted> {
        match event.as_ref() {
            HotShotEvent::HeartBeat(task_id) => {
                let mut task_ids_heartbeat_timestamp =
                    self.task_ids_heartbeat_timestamp.lock().await;
                match task_ids_heartbeat_timestamp.entry(task_id.clone()) {
                    Entry::Occupied(mut heartbeat_timestamp) => {
                        *heartbeat_timestamp.get_mut() = Instant::now();
                    }
                    Entry::Vacant(_) => {
                        // On startup of this task we populate the map with all task ids
                    }
                }
            }
            HotShotEvent::Shutdown => {
                return Some(HotShotTaskCompleted);
            }
            _ => {}
        }
        None
    }
}

#[async_trait]
impl<TYPES: NodeType> TaskState for HealthCheckTaskState<TYPES> {
    type Event = HotShotEvent<TYPES>;

    async fn handle_event(
        &mut self,
        event: Arc<Self::Event>,
        _sender: &Sender<Arc<Self::Event>>,
        _receiver: &Receiver<Arc<Self::Event>>,
    ) -> Result<()> {
        self.handle(&event).await;

        Ok(())
    }

    async fn cancel_subtasks(&mut self) {}

    async fn periodic_task(&self, _sender: &Sender<Arc<Self::Event>>, _task_id: String) {
        let current_time = Instant::now();

        let task_ids_heartbeat = self.task_ids_heartbeat_timestamp.lock().await;
        for (task_id, heartbeat_timestamp) in task_ids_heartbeat.iter() {
            if current_time.duration_since(*heartbeat_timestamp).as_secs()
                > self.heartbeat_timeout_duration_in_secs
            {
                tracing::error!(
                    "Node Id {} has not received a heartbeat for task id {} for {} seconds",
                    self.node_id,
                    task_id,
                    self.heartbeat_timeout_duration_in_secs
                );
            }
        }
    }

    fn get_task_name(&self) -> &'static str {
        std::any::type_name::<HealthCheckTaskState<TYPES>>()
    }
}
