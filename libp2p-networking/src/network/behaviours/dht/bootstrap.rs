use std::time::Duration;

use async_compatibility_layer::{art, channel::UnboundedSender};
use futures::{channel::mpsc, StreamExt};

use crate::network::ClientRequest;

/// Internal bootstrap events
pub enum InputEvent {
    /// Start bootstrap
    StartBootstrap,
    /// Bootstrap has finished
    BootstrapFinished,
    /// Shutdown bootstrap
    ShutdownBootstrap,
}
/// Bootstrap task's state
pub struct DHTBootstrapTask {
    /// Task's receiver
    rx: mpsc::Receiver<InputEvent>,
    /// Task's sender
    network_tx: UnboundedSender<ClientRequest>,
    /// Field indicating progress state
    in_progress: bool,
}

impl DHTBootstrapTask {
    /// Run bootstrap task
    pub fn run(rx: mpsc::Receiver<InputEvent>, tx: UnboundedSender<ClientRequest>) {
        art::async_spawn(async move {
            let state = Self {
                rx,
                network_tx: tx,
                in_progress: false,
            };
            state.run_loop().await;
        });
    }
    /// Task's loop
    async fn run_loop(mut self) {
        loop {
            tracing::debug!("looping bootstrap");
            if self.in_progress {
                match self.rx.next().await {
                    Some(InputEvent::BootstrapFinished) => {
                        tracing::debug!("Bootstrap finished");
                        self.in_progress = false;
                    }
                    Some(InputEvent::ShutdownBootstrap) => {
                        tracing::debug!("ShutdownBootstrap received, shutting down");
                        break;
                    }
                    Some(_) => {}
                    None => {
                        tracing::debug!("Bootstrap channel closed, exiting loop");
                        break;
                    }
                }
            } else if let Ok(maybe_event) =
                art::async_timeout(Duration::from_secs(120), self.rx.next()).await
            {
                match maybe_event {
                    Some(InputEvent::StartBootstrap) => {
                        tracing::debug!("Start bootstrap in bootstrap task");
                        self.bootstrap().await;
                    }
                    Some(InputEvent::ShutdownBootstrap) => {
                        tracing::debug!("ShutdownBootstrap received, shutting down");
                        break;
                    }
                    Some(_) => {}
                    None => {
                        tracing::debug!("Bootstrap channel closed, exiting loop");
                        break;
                    }
                }
            } else {
                tracing::debug!("Start bootstrap in bootstrap task after timeout");
                self.bootstrap().await;
            }
        }
    }
    /// Start bootstrap
    async fn bootstrap(&mut self) {
        self.in_progress = true;
        let _ = self.network_tx.send(ClientRequest::BeginBootstrap).await;
    }
}
