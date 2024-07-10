use anyhow::Context;
use async_std::{
    sync::{Arc, RwLock},
    task::{spawn, JoinHandle},
};
use derivative::Derivative;
use futures::{
    future::{join_all, Future},
    stream::{Stream, StreamExt},
};
use hotshot::{
    traits::election::static_committee::GeneralStaticCommittee,
    types::{Event, SystemContextHandle},
    Memberships, SystemContext,
};
use hotshot_example_types::auction_results_provider_types::TestAuctionResultsProvider;
use hotshot_orchestrator::{client::OrchestratorClient, config::NetworkConfig};
use hotshot_query_service::Leaf;
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    data::ViewNumber,
    traits::{election::Membership, metrics::Metrics, network::ConnectedNetwork},
    HotShotConfig,
};
use std::fmt::Display;
use url::Url;
use vbs::version::StaticVersionType;

#[cfg(feature = "benchmarking")]
use hotshot::{traits::BlockPayload, types::EventType};
#[cfg(feature = "benchmarking")]
use hotshot_orchestrator::client::BenchResults;
#[cfg(feature = "benchmarking")]
use hotshot_types::traits::{block_contents::BlockHeader, node_implementation::ConsensusTime};
#[cfg(feature = "benchmarking")]
use std::time::Instant;

use crate::{
    persistence::SequencerPersistence, state_signature::StateSigner, static_stake_table_commitment,
    Node, NodeState, PubKey, SeqTypes, Transaction, ValidatedState,
};
use hotshot_events_service::events_source::{EventConsumer, EventsStreamer};
/// The consensus handle
pub type Consensus<N, P> = SystemContextHandle<SeqTypes, Node<N, P>>;

/// The sequencer context contains a consensus handle and other sequencer specific information.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct SequencerContext<
    N: ConnectedNetwork<PubKey>,
    P: SequencerPersistence,
    Ver: StaticVersionType + 'static,
> {
    /// The consensus handle
    #[derivative(Debug = "ignore")]
    handle: Arc<RwLock<Consensus<N, P>>>,

    /// Context for generating state signatures.
    state_signer: Arc<StateSigner<Ver>>,

    /// An orchestrator to wait for before starting consensus.
    #[derivative(Debug = "ignore")]
    wait_for_orchestrator: Option<Arc<OrchestratorClient>>,

    /// Background tasks to shut down when the node is dropped.
    tasks: TaskList,

    /// events streamer to stream hotshot events to external clients
    events_streamer: Arc<RwLock<EventsStreamer<SeqTypes>>>,

    detached: bool,

    node_state: NodeState,
}

impl<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, Ver: StaticVersionType + 'static>
    SequencerContext<N, P, Ver>
{
    #[tracing::instrument(skip_all, fields(node_id = instance_state.node_id))]
    #[allow(clippy::too_many_arguments)]
    pub async fn init(
        config: HotShotConfig<PubKey>,
        instance_state: NodeState,
        persistence: P,
        network: Arc<N>,
        state_relay_server: Option<Url>,
        metrics: &dyn Metrics,
        stake_table_capacity: u64,
        _: Ver,
    ) -> anyhow::Result<Self> {
        let pub_key = config.my_own_validator_config.public_key;
        tracing::info!(%pub_key, "initializing consensus");

        // Stick our node ID in `metrics` so it is easily accessible via the status API.
        metrics
            .create_gauge("node_index".into(), None)
            .set(instance_state.node_id as usize);

        // Load saved consensus state from storage.
        let initializer = persistence
            .load_consensus_state(instance_state.clone())
            .await?;

        let committee_membership = GeneralStaticCommittee::create_election(
            config.known_nodes_with_stake.clone(),
            config.known_nodes_with_stake.clone(),
            0,
        );

        let da_membership = GeneralStaticCommittee::create_election(
            config.known_nodes_with_stake.clone(),
            config.known_da_nodes.clone(),
            0,
        );

        let memberships = Memberships {
            quorum_membership: committee_membership.clone(),
            da_membership,
            vid_membership: committee_membership.clone(),
            view_sync_membership: committee_membership.clone(),
        };

        let stake_table_commit = static_stake_table_commitment(
            &config.known_nodes_with_stake,
            stake_table_capacity
                .try_into()
                .context("stake table capacity out of range")?,
        );
        let state_key_pair = config.my_own_validator_config.state_key_pair.clone();

        let event_streamer = Arc::new(RwLock::new(EventsStreamer::<SeqTypes>::new(
            config.known_nodes_with_stake.clone(),
            config.num_nodes_without_stake,
        )));

        let persistence = Arc::new(RwLock::new(persistence));

        let handle = SystemContext::init(
            config.my_own_validator_config.public_key,
            config.my_own_validator_config.private_key.clone(),
            instance_state.node_id,
            config,
            memberships,
            network,
            initializer,
            ConsensusMetricsValue::new(metrics),
            persistence.clone(),
            TestAuctionResultsProvider::default(),
        )
        .await?
        .0;

        let mut state_signer = StateSigner::new(state_key_pair, stake_table_commit);
        if let Some(url) = state_relay_server {
            state_signer = state_signer.with_relay_server(url);
        }

        Ok(Self::new(
            handle,
            persistence,
            state_signer,
            event_streamer,
            instance_state,
        ))
    }

    /// Constructor
    fn new(
        handle: Consensus<N, P>,
        persistence: Arc<RwLock<P>>,
        state_signer: StateSigner<Ver>,
        event_streamer: Arc<RwLock<EventsStreamer<SeqTypes>>>,
        node_state: NodeState,
    ) -> Self {
        let events = handle.event_stream();

        let mut ctx = Self {
            handle: Arc::new(RwLock::new(handle)),
            state_signer: Arc::new(state_signer),
            tasks: Default::default(),
            detached: false,
            wait_for_orchestrator: None,
            events_streamer: event_streamer.clone(),
            node_state,
        };
        ctx.spawn(
            "main event handler",
            handle_events(
                events,
                persistence,
                ctx.state_signer.clone(),
                Some(event_streamer.clone()),
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
    pub fn state_signer(&self) -> Arc<StateSigner<Ver>> {
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
    pub fn consensus(&self) -> Arc<RwLock<Consensus<N, P>>> {
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

    pub fn node_state(&self) -> NodeState {
        self.node_state.clone()
    }

    /// Start participating in consensus.
    pub async fn start_consensus(&self) {
        #[cfg(feature = "benchmarking")]
        let mut has_orchestrator_client = false;
        #[cfg(feature = "benchmarking")]
        let mut network_config: NetworkConfig<PubKey> = Default::default();
        if let Some(orchestrator_client) = &self.wait_for_orchestrator {
            tracing::warn!("waiting for orchestrated start");
            orchestrator_client
                .wait_for_all_nodes_ready(self.node_state.node_id)
                .await;
            #[cfg(feature = "benchmarking")]
            {
                has_orchestrator_client = true;
                network_config = orchestrator_client.get_config_after_collection().await;
            }
        } else {
            tracing::error!("Cannot get info from orchestrator client");
        }
        tracing::warn!("starting consensus");
        self.handle.read().await.hotshot.start_consensus().await;

        #[cfg(feature = "benchmarking")]
        if has_orchestrator_client {
            // start_round is the number of rounds for warm up, which will not be counted in for benchmarking phase
            let start_round: usize = 20;
            let end_round: usize = start_round + network_config.rounds;
            let mut event_stream = self.event_stream().await;
            let mut num_successful_commits = 0;
            let mut total_transactions_committed = 0;
            let mut total_throughput = 0;
            let node_index: u64 = self.node_state().node_id;
            let mut start: Instant = Instant::now(); // will be re-assign once has_started turned to true
            let mut has_started: bool = false;
            loop {
                match event_stream.next().await {
                    None => {
                        tracing::error!(
                            "Error in Benchmarking! Event stream completed before consensus ended."
                        );
                    }
                    Some(Event { event, .. }) => {
                        match event {
                            EventType::Error { error } => {
                                tracing::error!("Error in consensus: {:?}", error);
                            }
                            EventType::Decide {
                                leaf_chain,
                                qc: _,
                                block_size,
                            } => {
                                if let Some(leaf_info) = leaf_chain.first() {
                                    let leaf = &leaf_info.leaf;
                                    tracing::info!(
                                        "Decide event for leaf: {}",
                                        *leaf.view_number()
                                    );
                                    num_successful_commits += leaf_chain.len();

                                    // only count in the info after warm up
                                    if num_successful_commits >= start_round {
                                        if !has_started {
                                            start = Instant::now();
                                            has_started = true;
                                        }

                                        // iterate all the decided transactions
                                        if let Some(block_payload) = &leaf.block_payload() {
                                            for tx in block_payload
                                                .transactions(leaf.block_header().metadata())
                                            {
                                                let payload_length = tx.into_payload().len();
                                                // Transaction = NamespaceId(u64) + payload(Vec<u8>)
                                                let tx_sz = payload_length * std::mem::size_of::<u8>() // size of payload
                                                    + std::mem::size_of::<u64>() // size of the namespace
                                                    + std::mem::size_of::<Transaction>(); // size of the struct wrapper
                                                total_throughput += tx_sz;
                                            }
                                        }
                                    }
                                }

                                if num_successful_commits >= start_round {
                                    if let Some(size) = block_size {
                                        total_transactions_committed += size;
                                    }
                                }

                                if num_successful_commits >= end_round {
                                    let total_time_elapsed = start.elapsed(); // in seconds
                                    let consensus_lock =
                                        self.handle.read().await.hotshot.consensus();
                                    let consensus = consensus_lock.read().await;
                                    let total_num_views =
                                        usize::try_from(consensus.locked_view().u64()).unwrap();
                                    let failed_num_views = total_num_views - num_successful_commits;
                                    let bench_results = if total_transactions_committed != 0 {
                                        let throughput_bytes_per_sec = (total_throughput as u64)
                                            / std::cmp::max(total_time_elapsed.as_secs(), 1u64);
                                        BenchResults {
                                            partial_results: "Unset".to_string(),
                                            // latency will be reported in another struct
                                            avg_latency_in_sec: 0,
                                            num_latency: 1,
                                            minimum_latency_in_sec: 0,
                                            maximum_latency_in_sec: 0,
                                            throughput_bytes_per_sec,
                                            total_transactions_committed,
                                            transaction_size_in_bytes: (total_throughput as u64)
                                                / total_transactions_committed, // refer to `submit-transactions.rs` for the range of transaction size
                                            total_time_elapsed_in_sec: total_time_elapsed.as_secs(),
                                            total_num_views,
                                            failed_num_views,
                                        }
                                    } else {
                                        BenchResults::default()
                                    };
                                    tracing::info!("[{node_index}]: {total_transactions_committed} committed from round {start_round} to {end_round} in {total_time_elapsed:?}, total number of views = {total_num_views}.");
                                    if let Some(orchestrator_client) = &self.wait_for_orchestrator {
                                        orchestrator_client.post_bench_results(bench_results).await;
                                    }
                                    break;
                                }

                                if leaf_chain.len() > 1 {
                                    tracing::warn!(
                                        "Leaf chain is greater than 1 with len {}",
                                        leaf_chain.len()
                                    );
                                }
                            }
                            _ => {} // mostly DA proposal
                        }
                    }
                }
            }
        }
    }

    /// Spawn a background task attached to this context.
    ///
    /// When this context is dropped or [`shut_down`](Self::shut_down), background tasks will be
    /// cancelled in the reverse order that they were spawned.
    pub fn spawn(&mut self, name: impl Display, task: impl Future + Send + 'static) {
        self.tasks.spawn(name, task);
    }

    /// Stop participating in consensus.
    pub async fn shut_down(&mut self) {
        tracing::info!("shutting down SequencerContext");
        self.handle.write().await.shut_down().await;
        self.tasks.shut_down().await;
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
}

impl<N: ConnectedNetwork<PubKey>, P: SequencerPersistence, Ver: StaticVersionType + 'static> Drop
    for SequencerContext<N, P, Ver>
{
    fn drop(&mut self) {
        if !self.detached {
            async_std::task::block_on(self.shut_down());
        }
    }
}

async fn handle_events<Ver: StaticVersionType>(
    mut events: impl Stream<Item = Event<SeqTypes>> + Unpin,
    persistence: Arc<RwLock<impl SequencerPersistence>>,
    state_signer: Arc<StateSigner<Ver>>,
    events_streamer: Option<Arc<RwLock<EventsStreamer<SeqTypes>>>>,
) {
    while let Some(event) = events.next().await {
        tracing::debug!(?event, "consensus event");

        {
            let mut p = persistence.write().await;
            // Store latest consensus state.
            p.handle_event(&event).await;
        }
        // Generate state signature.
        state_signer.handle_event(&event).await;

        // Send the event via the event streaming service
        if let Some(events_streamer) = events_streamer.as_ref() {
            events_streamer.write().await.handle_event(event).await;
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct TaskList(Vec<(String, JoinHandle<()>)>);

impl TaskList {
    /// Spawn a background task attached to this [`TaskList`].
    ///
    /// When this [`TaskList`] is dropped or [`shut_down`](Self::shut_down), background tasks will
    /// be cancelled in the reverse order that they were spawned.
    pub fn spawn(&mut self, name: impl Display, task: impl Future + Send + 'static) {
        let name = name.to_string();
        let task = {
            let name = name.clone();
            spawn(async move {
                task.await;
                tracing::info!(name, "background task exited");
            })
        };
        self.0.push((name, task));
    }

    /// Stop all background tasks.
    pub async fn shut_down(&mut self) {
        for (name, task) in self.0.drain(..).rev() {
            tracing::info!(name, "cancelling background task");
            task.cancel().await;
        }
    }

    /// Wait for all background tasks to complete.
    pub async fn join(&mut self) {
        join_all(self.0.drain(..).map(|(_, task)| task)).await;
    }

    pub fn extend(&mut self, mut tasks: TaskList) {
        self.0.extend(std::mem::take(&mut tasks.0));
    }
}

impl Drop for TaskList {
    fn drop(&mut self) {
        async_std::task::block_on(self.shut_down());
    }
}
