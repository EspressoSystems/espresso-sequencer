use core::time::Duration;
use std::sync::Arc;

use anyhow::{ensure, Result};
use async_broadcast::Sender;
use async_compatibility_layer::art::{async_sleep, async_spawn};
use async_lock::{RwLock, RwLockUpgradableReadGuard};
#[cfg(async_executor_impl = "async-std")]
use async_std::task::JoinHandle;
use hotshot_types::{
    consensus::Consensus,
    event::{Event, EventType},
    traits::node_implementation::{ConsensusTime, NodeType},
};
#[cfg(async_executor_impl = "tokio")]
use tokio::task::JoinHandle;
use tracing::{debug, error};

use crate::{
    events::HotShotEvent,
    helpers::{broadcast_event, cancel_task},
};

/// Constant which tells [`update_view`] to send a view change event when called.
pub(crate) const SEND_VIEW_CHANGE_EVENT: bool = true;

/// Constant which tells [`update_view`] to not send a view change event when called.
pub(crate) const DONT_SEND_VIEW_CHANGE_EVENT: bool = false;

/// Update the view if it actually changed, takes a mutable reference to the `cur_view` and the
/// `timeout_task` which are updated during the operation of the function.
///
/// # Errors
/// Returns an [`anyhow::Error`] when the new view is not greater than the current view.
/// TODO: Remove args when we merge dependency tasks.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn update_view<TYPES: NodeType>(
    new_view: TYPES::Time,
    event_stream: &Sender<Arc<HotShotEvent<TYPES>>>,
    timeout: u64,
    consensus: Arc<RwLock<Consensus<TYPES>>>,
    cur_view: &mut TYPES::Time,
    timeout_task: &mut JoinHandle<()>,
    output_event_stream: &Sender<Event<TYPES>>,
    send_view_change_event: bool,
) -> Result<()> {
    ensure!(
        new_view > *cur_view,
        "New view is not greater than our current view"
    );

    let old_view = *cur_view;

    debug!("Updating view from {} to {}", *old_view, *new_view);

    if *old_view / 100 != *new_view / 100 {
        // TODO (https://github.com/EspressoSystems/HotShot/issues/2296):
        // switch to info! when INFO logs become less cluttered
        error!("Progress: entered view {:>6}", *new_view);
    }

    *cur_view = new_view;

    // The next view is just the current view + 1
    let next_view = *cur_view + 1;

    if send_view_change_event {
        futures::join! {
            broadcast_event(Arc::new(HotShotEvent::ViewChange(new_view)), event_stream),
            broadcast_event(
                Event {
                    view_number: old_view,
                    event: EventType::ViewFinished {
                        view_number: old_view,
                    },
                },
                output_event_stream,
            )
        };
    }

    // Spawn a timeout task if we did actually update view
    let new_timeout_task = async_spawn({
        let stream = event_stream.clone();
        // Nuance: We timeout on the view + 1 here because that means that we have
        // not seen evidence to transition to this new view
        let view_number = next_view;
        let timeout = Duration::from_millis(timeout);
        async move {
            async_sleep(timeout).await;
            broadcast_event(
                Arc::new(HotShotEvent::Timeout(TYPES::Time::new(*view_number))),
                &stream,
            )
            .await;
        }
    });

    // cancel the old timeout task
    cancel_task(std::mem::replace(timeout_task, new_timeout_task)).await;

    let consensus = consensus.upgradable_read().await;
    consensus
        .metrics
        .current_view
        .set(usize::try_from(cur_view.u64()).unwrap());

    // Do the comparison before the subtraction to avoid potential overflow, since
    // `last_decided_view` may be greater than `cur_view` if the node is catching up.
    if usize::try_from(cur_view.u64()).unwrap()
        > usize::try_from(consensus.last_decided_view().u64()).unwrap()
    {
        consensus.metrics.number_of_views_since_last_decide.set(
            usize::try_from(cur_view.u64()).unwrap()
                - usize::try_from(consensus.last_decided_view().u64()).unwrap(),
        );
    }
    let mut consensus = RwLockUpgradableReadGuard::upgrade(consensus).await;
    if let Err(e) = consensus.update_view(new_view) {
        tracing::trace!("{e:?}");
    }
    tracing::trace!("View updated successfully");

    Ok(())
}
