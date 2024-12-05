use std::{fmt::Display, sync::Arc};

use anyhow::Context;
use async_broadcast::{broadcast, Receiver, Sender};
use async_lock::RwLock;
use clap::Parser;
use committable::{Commitment, Committable};
use derivative::Derivative;
use espresso_types::{
    parse_duration,
    v0::traits::{EventConsumer as PersistenceEventConsumer, SequencerPersistence},
    NodeState, PubKey, Transaction, ValidatedState,
};
use futures::{
    future::{join_all, Future},
    stream::{Stream, StreamExt},
};
use hotshot::{
    types::{Event, EventType, SystemContextHandle},
    MarketplaceConfig, Memberships, SystemContext,
};
use hotshot_events_service::events_source::{EventConsumer, EventsStreamer};
use parking_lot::Mutex;
use tokio::{
    spawn,
    task::JoinHandle,
    time::{sleep, timeout},
};

use hotshot_orchestrator::client::OrchestratorClient;
use hotshot_query_service::Leaf;
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    data::{EpochNumber, ViewNumber},
    network::NetworkConfig,
    traits::{
        metrics::Metrics,
        network::ConnectedNetwork,
        node_implementation::{ConsensusTime, NodeType, Versions},
        ValidatedState as _,
    },
    utils::{View, ViewInner},
    PeerConfig, ValidatorConfig,
};
use std::{fmt::Debug, time::Duration};
use tracing::{Instrument, Level};
use url::Url;

use crate::{
    external_event_handler::{self, ExternalEventHandler},
    state_signature::StateSigner,
    static_stake_table_commitment, Node, SeqTypes, SequencerApiVersion,
};

/// The consensus handle
pub type Consensus<N, P, V> = SystemContextHandle<SeqTypes, Node<N, P>, V>;

#[derive(Clone, Copy, Debug, Parser)]
pub struct ProposalFetcherConfig {
    #[clap(
        long = "proposal-fetcher-num-workers",
        env = "ESPRESSO_SEQUENCER_PROPOSAL_FETCHER_NUM_WORKERS",
        default_value = "2"
    )]
    pub num_workers: usize,

    #[clap(
        long = "proposal-fetcher-channel-capacity",
        env = "ESPRESSO_SEQUENCER_PROPOSAL_FETCHER_CHANNEL_CAPACITY",
        default_value = "100"
    )]
    pub channel_capacity: usize,

    #[clap(
        long = "proposal-fetcher-fetch-timeout",
        env = "ESPRESSO_SEQUENCER_PROPOSAL_FETCHER_FETCH_TIMEOUT",
        default_value = "2s",
        value_parser = parse_duration,
    )]
    pub fetch_timeout: Duration,
}

impl Default for ProposalFetcherConfig {
    fn default() -> Self {
        Self::parse_from(std::iter::empty::<String>())
    }
}

/// The sequencer context contains a consensus handle and other sequencer specific information.
#[derive(Derivative, Clone)]
#[derivative(Debug(bound = ""))]
pub struct SequencerContext<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, V: Versions> {
    /// The consensus handle
    #[derivative(Debug = "ignore")]
    handle: Arc<RwLock<Consensus<N, P, V>>>,

    /// Context for generating state signatures.
    state_signer: Arc<StateSigner<SequencerApiVersion>>,

    /// An orchestrator to wait for before starting consensus.
    #[derivative(Debug = "ignore")]
    wait_for_orchestrator: Option<Arc<OrchestratorClient>>,

    /// Background tasks to shut down when the node is dropped.
    tasks: TaskList,

    /// events streamer to stream hotshot events to external clients
    events_streamer: Arc<RwLock<EventsStreamer<SeqTypes>>>,

    detached: bool,

    node_state: NodeState,

    network_config: NetworkConfig<PubKey>,

    #[derivative(Debug = "ignore")]
    validator_config: ValidatorConfig<<SeqTypes as NodeType>::SignatureKey>,
}

impl<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, V: Versions> SequencerContext<N, P, V> {
    #[tracing::instrument(skip_all, fields(node_id = instance_state.node_id))]
    #[allow(clippy::too_many_arguments)]
    pub async fn init(
        network_config: NetworkConfig<PubKey>,
        validator_config: ValidatorConfig<<SeqTypes as NodeType>::SignatureKey>,
        memberships: Memberships<SeqTypes>,
        instance_state: NodeState,
        persistence: P,
        network: Arc<N>,
        state_relay_server: Option<Url>,
        metrics: &dyn Metrics,
        stake_table_capacity: u64,
        public_api_url: Option<Url>,
        event_consumer: impl PersistenceEventConsumer + 'static,
        _: V,
        marketplace_config: MarketplaceConfig<SeqTypes, Node<N, P>>,
        proposal_fetcher_cfg: ProposalFetcherConfig,
    ) -> anyhow::Result<Self> {
        let config = &network_config.config;
        let pub_key = validator_config.public_key;
        tracing::info!(%pub_key, "initializing consensus");

        // Stick our node ID in `metrics` so it is easily accessible via the status API.
        metrics
            .create_gauge("node_index".into(), None)
            .set(instance_state.node_id as usize);

        // Start L1 client if it isn't already.
        instance_state.l1_client.spawn_tasks().await;

        // Load saved consensus state from storage.
        let (initializer, anchor_view) = persistence
            .load_consensus_state::<V>(instance_state.clone())
            .await?;

        let stake_table_commit = static_stake_table_commitment(
            &config.known_nodes_with_stake,
            stake_table_capacity
                .try_into()
                .context("stake table capacity out of range")?,
        );
        let state_key_pair = validator_config.state_key_pair.clone();

        let event_streamer = Arc::new(RwLock::new(EventsStreamer::<SeqTypes>::new(
            config.known_nodes_with_stake.clone(),
            0,
        )));

        let persistence = Arc::new(persistence);

        let handle = SystemContext::init(
            validator_config.public_key,
            validator_config.private_key.clone(),
            instance_state.node_id,
            config.clone(),
            memberships,
            network.clone(),
            initializer,
            ConsensusMetricsValue::new(metrics),
            persistence.clone(),
            marketplace_config,
        )
        .await?
        .0;

        let mut state_signer = StateSigner::new(state_key_pair, stake_table_commit);
        if let Some(url) = state_relay_server {
            state_signer = state_signer.with_relay_server(url);
        }

        // Create the roll call info we will be using
        let roll_call_info = external_event_handler::RollCallInfo { public_api_url };

        // Create the external event handler
        let mut tasks = TaskList::default();
        let external_event_handler =
            ExternalEventHandler::new(&mut tasks, network, roll_call_info, pub_key)
                .await
                .with_context(|| "Failed to create external event handler")?;

        Ok(Self::new(
            handle,
            persistence,
            state_signer,
            external_event_handler,
            event_streamer,
            instance_state,
            network_config,
            validator_config,
            event_consumer,
            anchor_view,
            proposal_fetcher_cfg,
        )
        .with_task_list(tasks))
    }

    /// Constructor
    #[allow(clippy::too_many_arguments)]
    fn new(
        handle: Consensus<N, P, V>,
        persistence: Arc<P>,
        state_signer: StateSigner<SequencerApiVersion>,
        external_event_handler: ExternalEventHandler<V>,
        event_streamer: Arc<RwLock<EventsStreamer<SeqTypes>>>,
        node_state: NodeState,
        network_config: NetworkConfig<PubKey>,
        validator_config: ValidatorConfig<<SeqTypes as NodeType>::SignatureKey>,
        event_consumer: impl PersistenceEventConsumer + 'static,
        anchor_view: Option<ViewNumber>,
        proposal_fetcher_cfg: ProposalFetcherConfig,
    ) -> Self {
        let events = handle.event_stream();

        let node_id = node_state.node_id;
        let mut ctx = Self {
            handle: Arc::new(RwLock::new(handle)),
            state_signer: Arc::new(state_signer),
            tasks: Default::default(),
            detached: false,
            wait_for_orchestrator: None,
            events_streamer: event_streamer.clone(),
            node_state,
            network_config,
            validator_config,
        };

        // Spawn proposal fetching tasks.
        let (send, recv) = broadcast(proposal_fetcher_cfg.channel_capacity);
        ctx.spawn("proposal scanner", scan_proposals(ctx.handle.clone(), send));
        for i in 0..proposal_fetcher_cfg.num_workers {
            ctx.spawn(
                format!("proposal fetcher {i}"),
                fetch_proposals(
                    ctx.handle.clone(),
                    persistence.clone(),
                    recv.clone(),
                    proposal_fetcher_cfg.fetch_timeout,
                ),
            );
        }

        // Spawn event handling loop.
        ctx.spawn(
            "event handler",
            handle_events(
                node_id,
                events,
                persistence,
                ctx.state_signer.clone(),
                external_event_handler,
                Some(event_streamer.clone()),
                event_consumer,
                anchor_view,
            ),
        );

        ctx
    }

    /// Wait for a signal from the orchestrator before starting consensus.
    pub fn wait_for_orchestrator(mut self, client: OrchestratorClient) -> Self {
        self.wait_for_orchestrator = Some(Arc::new(client));
        self
    }

    /// Add a list of tasks to the given context.
    pub(crate) fn with_task_list(mut self, tasks: TaskList) -> Self {
        self.tasks.extend(tasks);
        self
    }

    /// Return a reference to the consensus state signer.
    pub fn state_signer(&self) -> Arc<StateSigner<SequencerApiVersion>> {
        self.state_signer.clone()
    }

    /// Stream consensus events.
    pub async fn event_stream(&self) -> impl Stream<Item = Event<SeqTypes>> {
        self.handle.read().await.event_stream()
    }

    pub async fn submit_transaction(&self, tx: Transaction) -> anyhow::Result<()> {
        self.handle.read().await.submit_transaction(tx).await?;
        Ok(())
    }

    /// get event streamer
    pub fn event_streamer(&self) -> Arc<RwLock<EventsStreamer<SeqTypes>>> {
        self.events_streamer.clone()
    }

    /// Return a reference to the underlying consensus handle.
    pub fn consensus(&self) -> Arc<RwLock<Consensus<N, P, V>>> {
        Arc::clone(&self.handle)
    }

    pub async fn shutdown_consensus(&self) {
        self.handle.write().await.shut_down().await
    }

    pub async fn decided_leaf(&self) -> Leaf<SeqTypes> {
        self.handle.read().await.decided_leaf().await
    }

    pub async fn state(&self, view: ViewNumber) -> Option<Arc<ValidatedState>> {
        self.handle.read().await.state(view).await
    }

    pub async fn decided_state(&self) -> Arc<ValidatedState> {
        self.handle.read().await.decided_state().await
    }

    pub fn node_id(&self) -> u64 {
        self.node_state.node_id
    }

    pub fn node_state(&self) -> NodeState {
        self.node_state.clone()
    }

    /// Start participating in consensus.
    pub async fn start_consensus(&self) {
        if let Some(orchestrator_client) = &self.wait_for_orchestrator {
            tracing::warn!("waiting for orchestrated start");
            let peer_config = PeerConfig::to_bytes(&self.validator_config.public_config()).clone();
            orchestrator_client
                .wait_for_all_nodes_ready(peer_config)
                .await;
        } else {
            tracing::error!("Cannot get info from orchestrator client");
        }
        tracing::warn!("starting consensus");
        self.handle.read().await.hotshot.start_consensus().await;
    }

    /// Spawn a background task attached to this context.
    ///
    /// When this context is dropped or [`shut_down`](Self::shut_down), background tasks will be
    /// cancelled in the reverse order that they were spawned.
    pub fn spawn(&mut self, name: impl Display, task: impl Future<Output: Debug> + Send + 'static) {
        self.tasks.spawn(name, task);
    }

    /// Spawn a short-lived background task attached to this context.
    ///
    /// When this context is dropped or [`shut_down`](Self::shut_down), background tasks will be
    /// cancelled in the reverse order that they were spawned.
    ///
    /// The only difference between a short-lived background task and a [long-lived](Self::spawn)
    /// one is how urgently logging related to the task is treated.
    pub fn spawn_short_lived(
        &mut self,
        name: impl Display,
        task: impl Future<Output: Debug> + Send + 'static,
    ) {
        self.tasks.spawn_short_lived(name, task);
    }

    /// Stop participating in consensus.
    pub async fn shut_down(&mut self) {
        tracing::info!("shutting down SequencerContext");
        self.handle.write().await.shut_down().await;
        self.tasks.shut_down();
        self.node_state.l1_client.shut_down_tasks().await;

        // Since we've already shut down, we can set `detached` so the drop
        // handler doesn't call `shut_down` again.
        self.detached = true;
    }

    /// Wait for consensus to complete.
    ///
    /// Under normal conditions, this function will block forever, which is a convenient way of
    /// keeping the main thread from exiting as long as there are still active background tasks.
    pub async fn join(mut self) {
        self.tasks.join().await;
    }

    /// Allow this node to continue participating in consensus even after it is dropped.
    pub fn detach(&mut self) {
        // Set `detached` so the drop handler doesn't call `shut_down`.
        self.detached = true;
    }

    /// Get the network config
    pub fn network_config(&self) -> NetworkConfig<PubKey> {
        self.network_config.clone()
    }
}

impl<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, V: Versions> Drop
    for SequencerContext<N, P, V>
{
    fn drop(&mut self) {
        if !self.detached {
            // Spawn a task to shut down the context
            let handle_clone = self.handle.clone();
            let tasks_clone = self.tasks.clone();
            let node_state_clone = self.node_state.clone();

            spawn(async move {
                tracing::info!("shutting down SequencerContext");
                handle_clone.write().await.shut_down().await;
                tasks_clone.shut_down();
                node_state_clone.l1_client.shut_down_tasks().await;
            });

            // Set `detached` so the drop handler doesn't call `shut_down` again.
            self.detached = true;
        }
    }
}

#[tracing::instrument(skip_all, fields(node_id))]
#[allow(clippy::too_many_arguments)]
async fn handle_events<V: Versions>(
    node_id: u64,
    mut events: impl Stream<Item = Event<SeqTypes>> + Unpin,
    persistence: Arc<impl SequencerPersistence>,
    state_signer: Arc<StateSigner<SequencerApiVersion>>,
    external_event_handler: ExternalEventHandler<V>,
    events_streamer: Option<Arc<RwLock<EventsStreamer<SeqTypes>>>>,
    event_consumer: impl PersistenceEventConsumer + 'static,
    anchor_view: Option<ViewNumber>,
) {
    if let Some(view) = anchor_view {
        // Process and clean up any leaves that we may have persisted last time we were running but
        // failed to handle due to a shutdown.
        if let Err(err) = persistence
            .append_decided_leaves(view, vec![], &event_consumer)
            .await
        {
            tracing::warn!(
                "failed to process decided leaves, chain may not be up to date: {err:#}"
            );
        }
    }

    while let Some(event) = events.next().await {
        tracing::debug!(node_id, ?event, "consensus event");

        // Store latest consensus state.
        persistence.handle_event(&event, &event_consumer).await;

        // Generate state signature.
        state_signer.handle_event(&event).await;

        // Handle external messages
        if let EventType::ExternalMessageReceived { data, .. } = &event.event {
            if let Err(err) = external_event_handler.handle_event(data).await {
                tracing::warn!("Failed to handle external message: {:?}", err);
            };
        }

        // Send the event via the event streaming service
        if let Some(events_streamer) = events_streamer.as_ref() {
            events_streamer.write().await.handle_event(event).await;
        }
    }
}

#[tracing::instrument(skip_all)]
async fn scan_proposals<N, P, V>(
    consensus: Arc<RwLock<Consensus<N, P, V>>>,
    fetcher: Sender<(ViewNumber, Commitment<Leaf<SeqTypes>>)>,
) where
    N: ConnectedNetwork<PubKey>,
    P: SequencerPersistence,
    V: Versions,
{
    let mut events = consensus.read().await.event_stream();
    while let Some(event) = events.next().await {
        let EventType::QuorumProposal { proposal, .. } = event.event else {
            continue;
        };
        // Whenever we see a quorum proposal, ensure we have the chain of proposals stretching back
        // to the anchor. This allows state replay from the decided state.
        let parent_view = proposal.data.justify_qc.view_number;
        let parent_leaf = proposal.data.justify_qc.data.leaf_commit;
        fetcher
            .broadcast_direct((parent_view, parent_leaf))
            .await
            .ok();
    }
}

#[tracing::instrument(skip_all)]
async fn fetch_proposals<N, P, V>(
    consensus: Arc<RwLock<Consensus<N, P, V>>>,
    persistence: Arc<impl SequencerPersistence>,
    mut scanner: Receiver<(ViewNumber, Commitment<Leaf<SeqTypes>>)>,
    fetch_timeout: Duration,
) where
    N: ConnectedNetwork<PubKey>,
    P: SequencerPersistence,
    V: Versions,
{
    let sender = scanner.new_sender();
    while let Some((view, leaf)) = scanner.next().await {
        let span = tracing::warn_span!("fetch proposal", ?view, %leaf);
        let res: anyhow::Result<()> = async {
            let anchor_view = load_anchor_view(&*persistence).await;
            if view <= anchor_view {
                tracing::debug!(?anchor_view, "skipping already-decided proposal");
                return Ok(());
            }

            match persistence.load_quorum_proposal(view).await {
                Ok(proposal) => {
                    // If we already have the proposal in storage, keep traversing the chain to its
                    // parent.
                    let view = proposal.data.justify_qc.view_number;
                    let leaf = proposal.data.justify_qc.data.leaf_commit;
                    sender.broadcast_direct((view, leaf)).await.ok();
                    return Ok(());
                }
                Err(err) => {
                    tracing::info!("proposal missing from storage; fetching from network: {err:#}");
                }
            }

            let future =
                consensus
                    .read()
                    .await
                    .request_proposal(view, EpochNumber::genesis(), leaf)?;
            let proposal = timeout(fetch_timeout, future)
                .await
                .context("timed out fetching proposal")?
                .context("error fetching proposal")?;
            persistence
                .append_quorum_proposal(&proposal)
                .await
                .context("error saving fetched proposal")?;

            // Add the fetched leaf to HotShot state, so consensus can make use of it.
            let leaf = Leaf::from_quorum_proposal(&proposal.data);
            let handle = consensus.read().await;
            let consensus = handle.consensus();
            let mut consensus = consensus.write().await;
            if matches!(
                consensus.validated_state_map().get(&view),
                None | Some(View {
                    // Replace a Da-only view with a Leaf view, which has strictly more information.
                    view_inner: ViewInner::Da { .. }
                })
            ) {
                let v = View {
                    view_inner: ViewInner::Leaf {
                        leaf: Committable::commit(&leaf),
                        state: Arc::new(ValidatedState::from_header(leaf.block_header())),
                        delta: None,
                    },
                };
                if let Err(err) = consensus.update_validated_state_map(view, v) {
                    tracing::warn!("unable to update validated state map: {err:#}");
                }
                consensus
                    .update_saved_leaves(leaf, &handle.hotshot.upgrade_lock)
                    .await;
                tracing::debug!("added view to validated state map view proposal fetcher");
            }

            Ok(())
        }
        .instrument(span)
        .await;
        if let Err(err) = res {
            tracing::warn!("failed to fetch proposal: {err:#}");

            // Avoid busy loop when operations are failing.
            sleep(Duration::from_secs(1)).await;

            // If we fail fetching the proposal, don't let it clog up the fetching task. Just push
            // it back onto the queue and move onto the next proposal.
            sender.broadcast_direct((view, leaf)).await.ok();
        }
    }
}

async fn load_anchor_view(persistence: &impl SequencerPersistence) -> ViewNumber {
    loop {
        match persistence.load_anchor_view().await {
            Ok(view) => break view,
            Err(err) => {
                tracing::warn!("error loading anchor view: {err:#}");
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
#[allow(clippy::type_complexity)]
pub(crate) struct TaskList(Arc<Mutex<Vec<(String, JoinHandle<()>)>>>);

macro_rules! spawn_with_log_level {
    ($this:expr, $lvl:expr, $name:expr, $task: expr) => {
        let name = $name.to_string();
        let task = {
            let name = name.clone();
            let span = tracing::span!($lvl, "background task", name);
            spawn(
                async move {
                    tracing::event!($lvl, "spawning background task");
                    let res = $task.await;
                    tracing::event!($lvl, ?res, "background task exited");
                }
                .instrument(span),
            )
        };
        $this.0.lock().push((name, task));
    };
}

impl TaskList {
    /// Spawn a background task attached to this [`TaskList`].
    ///
    /// When this [`TaskList`] is dropped or [`shut_down`](Self::shut_down), background tasks will
    /// be cancelled in the reverse order that they were spawned.
    pub fn spawn(&mut self, name: impl Display, task: impl Future<Output: Debug> + Send + 'static) {
        spawn_with_log_level!(self, Level::INFO, name, task);
    }

    /// Spawn a short-lived background task attached to this [`TaskList`].
    ///
    /// When this [`TaskList`] is dropped or [`shut_down`](Self::shut_down), background tasks will
    /// be cancelled in the reverse order that they were spawned.
    ///
    /// The only difference between a short-lived background task and a [long-lived](Self::spawn)
    /// one is how urgently logging related to the task is treated.
    pub fn spawn_short_lived(
        &mut self,
        name: impl Display,
        task: impl Future<Output: Debug> + Send + 'static,
    ) {
        spawn_with_log_level!(self, Level::DEBUG, name, task);
    }

    /// Stop all background tasks.
    pub fn shut_down(&self) {
        let tasks: Vec<(String, JoinHandle<()>)> = self.0.lock().drain(..).collect();
        for (name, task) in tasks.into_iter().rev() {
            tracing::info!(name, "cancelling background task");
            task.abort();
        }
    }

    /// Wait for all background tasks to complete.
    pub async fn join(&mut self) {
        let tasks: Vec<(String, JoinHandle<()>)> = self.0.lock().drain(..).collect();
        join_all(tasks.into_iter().map(|(_, task)| task)).await;
    }

    pub fn extend(&mut self, tasks: TaskList) {
        self.0.lock().extend(
            tasks
                .0
                .lock()
                .drain(..)
                .collect::<Vec<(String, JoinHandle<()>)>>(),
        );
    }
}

impl Drop for TaskList {
    fn drop(&mut self) {
        self.shut_down()
    }
}
