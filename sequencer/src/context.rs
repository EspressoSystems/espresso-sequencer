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
    Memberships, Networks, SystemContext,
};
use hotshot_orchestrator::client::OrchestratorClient;
// Should move `STAKE_TABLE_CAPACITY` in the sequencer repo when we have variate stake table support
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    traits::{election::Membership, metrics::Metrics},
    HotShotConfig,
};
use std::fmt::Display;
use url::Url;
use versioned_binary_serialization::version::StaticVersionType;

use crate::{
    network, persistence::SequencerPersistence, state_signature::StateSigner,
    static_stake_table_commitment, ElectionConfig, Node, NodeState, PubKey, SeqTypes, Transaction,
};

/// The consensus handle
pub type Consensus<N, P> = SystemContextHandle<SeqTypes, Node<N, P>>;

/// The sequencer context contains a consensus handle and other sequencer specific information.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct SequencerContext<
    N: network::Type,
    P: SequencerPersistence,
    Ver: StaticVersionType + 'static,
> {
    /// The consensus handle
    #[derivative(Debug = "ignore")]
    handle: Consensus<N, P>,

    /// Index of this sequencer node
    #[allow(dead_code)]
    node_index: u64,

    /// Context for generating state signatures.
    state_signer: Arc<StateSigner<Ver>>,

    /// An orchestrator to wait for before starting consensus.
    #[derivative(Debug = "ignore")]
    wait_for_orchestrator: Option<Arc<OrchestratorClient>>,

    /// Background tasks to shut down when the node is dropped.
    tasks: Vec<(String, JoinHandle<()>)>,

    detached: bool,
}

impl<N: network::Type, P: SequencerPersistence, Ver: StaticVersionType + 'static>
    SequencerContext<N, P, Ver>
{
    #[allow(clippy::too_many_arguments)]
    pub async fn init(
        config: HotShotConfig<PubKey, ElectionConfig>,
        instance_state: NodeState,
        persistence: P,
        networks: Networks<SeqTypes, Node<N, P>>,
        state_relay_server: Option<Url>,
        metrics: &dyn Metrics,
        node_id: u64,
        _: Ver,
    ) -> anyhow::Result<Self> {
        // Load saved consensus state from storage.
        let initializer = persistence.load_consensus_state(instance_state).await?;

        let election_config = GeneralStaticCommittee::<SeqTypes, PubKey>::default_election_config(
            config.num_nodes_with_stake.get() as u64,
            0,
        );
        let membership = GeneralStaticCommittee::create_election(
            config.known_nodes_with_stake.clone(),
            election_config,
            0,
        );
        let memberships = Memberships {
            quorum_membership: membership.clone(),
            da_membership: membership.clone(),
            vid_membership: membership.clone(),
            view_sync_membership: membership,
        };

        let stake_table_commit =
            static_stake_table_commitment(&config.known_nodes_with_stake, STAKE_TABLE_CAPACITY);
        let state_key_pair = config.my_own_validator_config.state_key_pair.clone();

        let persistence = Arc::new(RwLock::new(persistence));

        let handle = SystemContext::init(
            config.my_own_validator_config.public_key,
            config.my_own_validator_config.private_key.clone(),
            node_id,
            config,
            memberships,
            networks,
            initializer,
            ConsensusMetricsValue::new(metrics),
            persistence.clone(),
        )
        .await?
        .0;

        let mut state_signer = StateSigner::new(state_key_pair, stake_table_commit);
        if let Some(url) = state_relay_server {
            state_signer = state_signer.with_relay_server(url);
        }
        Ok(Self::new(handle, persistence, node_id, state_signer))
    }

    /// Constructor
    fn new(
        handle: Consensus<N, P>,
        persistence: Arc<RwLock<P>>,
        node_index: u64,
        state_signer: StateSigner<Ver>,
    ) -> Self {
        let events = handle.get_event_stream();
        let mut ctx = Self {
            handle,
            node_index,
            state_signer: Arc::new(state_signer),
            tasks: vec![],
            detached: false,
            wait_for_orchestrator: None,
        };
        ctx.spawn(
            "main event handler",
            handle_events(events, persistence, ctx.state_signer.clone()),
        );
        ctx
    }

    /// Wait for a signal from the orchestrator before starting consensus.
    pub fn wait_for_orchestrator(mut self, client: OrchestratorClient) -> Self {
        self.wait_for_orchestrator = Some(Arc::new(client));
        self
    }

    /// Return a reference to the consensus state signer.
    pub fn state_signer(&self) -> Arc<StateSigner<Ver>> {
        self.state_signer.clone()
    }

    /// Stream consensus events.
    pub fn get_event_stream(&self) -> impl Stream<Item = Event<SeqTypes>> {
        self.handle.get_event_stream()
    }

    pub async fn submit_transaction(&self, tx: Transaction) -> anyhow::Result<()> {
        self.handle.submit_transaction(tx).await?;
        Ok(())
    }

    /// Return a reference to the underlying consensus handle.
    pub fn consensus(&self) -> &Consensus<N, P> {
        &self.handle
    }

    /// Return a mutable reference to the underlying consensus handle.
    pub fn consensus_mut(&mut self) -> &mut Consensus<N, P> {
        &mut self.handle
    }

    /// Start participating in consensus.
    pub async fn start_consensus(&self) {
        if let Some(orchestrator_client) = &self.wait_for_orchestrator {
            tracing::info!("waiting for orchestrated start");
            orchestrator_client
                .wait_for_all_nodes_ready(self.node_index)
                .await;
        }
        self.handle.hotshot.start_consensus().await;
    }

    /// Spawn a background task attached to this context.
    ///
    /// When this context is dropped or [`shut_down`](Self::shut_down), background tasks will be
    /// cancelled in the reverse order that they were spawned.
    pub fn spawn(&mut self, name: impl Display, task: impl Future + Send + 'static) {
        let name = name.to_string();
        let task = {
            let name = name.clone();
            spawn(async move {
                task.await;
                tracing::info!(name, "background task exited");
            })
        };
        self.tasks.push((name, task));
    }

    /// Stop participating in consensus.
    pub async fn shut_down(&mut self) {
        tracing::info!("shutting down SequencerContext");
        self.handle.shut_down().await;
        for (name, task) in self.tasks.drain(..).rev() {
            tracing::info!(name, "cancelling background task");
            task.cancel().await;
        }
    }

    /// Wait for consensus to complete.
    ///
    /// Under normal conditions, this function will block forever, which is a convenient way of
    /// keeping the main thread from exiting as long as there are still active background tasks.
    pub async fn join(mut self) {
        join_all(self.tasks.drain(..).map(|(_, task)| task)).await;
    }

    /// Allow this node to continue participating in consensus even after it is dropped.
    pub fn detach(&mut self) {
        // Set `detached` so the drop handler doesn't call `shut_down`.
        self.detached = true;
    }
}

impl<N: network::Type, P: SequencerPersistence, Ver: StaticVersionType + 'static> Drop
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
    }
}
