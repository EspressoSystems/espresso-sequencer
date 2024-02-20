pub mod api;
mod block;
mod chain_variables;
pub mod context;
mod header;
pub mod hotshot_commitment;
pub mod options;
pub mod state_signature;
use block::entry::TxTableEntryWord;
use context::SequencerContext;
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{coins_bip39::English, MnemonicBuilder, Wallet},
};
// Should move `STAKE_TABLE_CAPACITY` in the sequencer repo when we have variate stake table support
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;
use state::FeeAccount;
use state_signature::static_stake_table_commitment;
use url::Url;
mod l1_client;
pub mod persistence;
mod state;
pub mod transaction;
mod vm;

use derivative::Derivative;
use hotshot::traits::implementations::MemoryNetwork;
use hotshot::{
    traits::{
        election::static_committee::{GeneralStaticCommittee, StaticElectionConfig},
        implementations::{MemoryStorage, NetworkingMetricsValue, WebServerNetwork},
    },
    types::{SignatureKey, SystemContextHandle},
    HotShotInitializer, Memberships, Networks, SystemContext,
};
use hotshot_types::traits::network::ConnectedNetwork;
use jf_primitives::signatures::bls_over_bn254::VerKey;

use hotshot_orchestrator::{
    client::{OrchestratorClient, ValidatorArgs},
    config::NetworkConfig,
};
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    data::ViewNumber,
    light_client::StateKeyPair,
    signature_key::{BLSPrivKey, BLSPubKey},
    traits::{
        metrics::Metrics,
        node_implementation::{NodeImplementation, NodeType},
        states::InstanceState,
    },
    HotShotConfig,
};

use persistence::SequencerPersistence;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::net::Ipv4Addr;
use std::time::Duration;
use std::{fmt::Debug, sync::Arc};
use std::{marker::PhantomData, net::IpAddr};

pub use block::payload::Payload;
pub use chain_variables::ChainVariables;
pub use header::Header;
pub use l1_client::L1BlockInfo;
pub use options::Options;
pub use state::ValidatedState;
pub use transaction::Transaction;
pub use vm::{Vm, VmId, VmTransaction};

/// Initialize the static variables for the sequencer
///
/// Calling it early on startup makes it easier to catch errors.
pub fn init_static() {
    lazy_static::initialize(&header::L1_CLIENT);
}

pub mod network {
    use hotshot_types::message::Message;

    use super::*;

    pub trait Type: 'static {
        type DAChannel: ConnectedNetwork<Message<SeqTypes>, PubKey> + Debug;
        type QuorumChannel: ConnectedNetwork<Message<SeqTypes>, PubKey> + Debug;
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Web;

    impl Type for Web {
        type DAChannel = WebServerNetwork<SeqTypes>;
        type QuorumChannel = WebServerNetwork<SeqTypes>;
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Memory;

    impl Type for Memory {
        type DAChannel = MemoryNetwork<Message<SeqTypes>, PubKey>;
        type QuorumChannel = MemoryNetwork<Message<SeqTypes>, PubKey>;
    }
}

/// The Sequencer node is generic over the hotshot CommChannel.
#[derive(Derivative, Serialize, Deserialize)]
#[derivative(
    Copy(bound = ""),
    Debug(bound = ""),
    Default(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
pub struct Node<N: network::Type>(PhantomData<fn(&N)>);

// Using derivative to derive Clone triggers the clippy lint
// https://rust-lang.github.io/rust-clippy/master/index.html#/incorrect_clone_impl_on_copy_type
impl<N: network::Type> Clone for Node<N> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(
    Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct SeqTypes;

pub type Leaf = hotshot_types::data::Leaf<SeqTypes>;
pub type Storage = MemoryStorage<SeqTypes>;
pub type Event = hotshot::types::Event<SeqTypes>;

pub type PubKey = BLSPubKey;
pub type PrivKey = <PubKey as SignatureKey>::PrivateKey;

type ElectionConfig = StaticElectionConfig;

impl<N: network::Type> NodeImplementation<SeqTypes> for Node<N> {
    type Storage = Storage;
    type QuorumNetwork = N::QuorumChannel;
    type CommitteeNetwork = N::DAChannel;
}

#[derive(Clone, Debug)]
pub struct NodeState {
    genesis_state: ValidatedState,
    builder_address: Wallet<SigningKey>,
}

impl Default for NodeState {
    fn default() -> Self {
        let wallet = FeeAccount::test_wallet();

        Self {
            genesis_state: ValidatedState::default(),
            builder_address: wallet,
        }
    }
}

impl InstanceState for NodeState {}

impl NodeType for SeqTypes {
    type Time = ViewNumber;
    type BlockHeader = Header;
    type BlockPayload = Payload<TxTableEntryWord>;
    type SignatureKey = PubKey;
    type Transaction = Transaction;
    type ElectionConfigType = ElectionConfig;
    type InstanceState = NodeState;
    type ValidatedState = ValidatedState;
    type Membership = GeneralStaticCommittee<Self, PubKey>;
}

#[derive(Clone, Debug, Snafu, Deserialize, Serialize)]
pub enum Error {
    // TODO: Can we nest these errors in a `ValidationError` to group them?

    // Parent state commitment of block doesn't match current state commitment
    IncorrectParent,

    // New view number isn't strictly after current view
    IncorrectView,

    // Genesis block either has zero or more than one transaction
    GenesisWrongSize,

    // Genesis transaction not present in genesis block
    MissingGenesis,

    // Genesis transaction in non-genesis block
    UnexpectedGenesis,

    // Merkle tree error
    MerkleTreeError { error: String },

    BlockBuilding,
}

#[allow(clippy::too_many_arguments)]
async fn init_hotshot<N: network::Type>(
    nodes_pub_keys: Vec<PubKey>,
    known_nodes_with_stake: Vec<<PubKey as SignatureKey>::StakeTableEntry>,
    node_id: usize,
    priv_key: PrivKey,
    networks: Networks<SeqTypes, Node<N>>,
    config: HotShotConfig<PubKey, ElectionConfig>,
    metrics: &dyn Metrics,
    instance_state: &NodeState,
) -> SystemContextHandle<SeqTypes, Node<N>> {
    let membership = GeneralStaticCommittee::new(&nodes_pub_keys, known_nodes_with_stake.clone());
    let memberships = Memberships {
        quorum_membership: membership.clone(),
        da_membership: membership.clone(),
        vid_membership: membership.clone(),
        view_sync_membership: membership,
    };

    SystemContext::init(
        nodes_pub_keys[node_id],
        priv_key,
        node_id as u64,
        config,
        Storage::empty(),
        memberships,
        networks,
        HotShotInitializer::from_genesis(instance_state).unwrap(),
        ConsensusMetricsValue::new(metrics),
    )
    .await
    .unwrap()
    .0
}

pub struct NetworkParams {
    pub da_server_url: Url,
    pub consensus_server_url: Url,
    pub orchestrator_url: Url,
    pub state_relay_server_url: Url,
    pub webserver_poll_interval: Duration,
    pub private_staking_key: BLSPrivKey,
}

pub async fn init_node(
    network_params: NetworkParams,
    metrics: &dyn Metrics,
    persistence: &mut impl SequencerPersistence,
    builder_mnemonic: String,
) -> anyhow::Result<SequencerContext<network::Web>> {
    // Orchestrator client
    let validator_args = ValidatorArgs {
        url: network_params.orchestrator_url,
        public_ip: None,
        network_config_file: None,
    };
    // This "public" IP only applies to libp2p network configurations, so we can supply any value here
    let public_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let orchestrator_client = OrchestratorClient::new(validator_args, public_ip.to_string());

    let private_staking_key = network_params.private_staking_key;
    let public_staking_key = BLSPubKey::from_private(&private_staking_key);

    let (config, wait_for_orchestrator) = match persistence.load_config().await? {
        Some(config) => {
            tracing::info!("loaded network config from storage, rejoining existing network");
            (config, false)
        }
        None => {
            tracing::info!("loading network config from orchestrator");
            let mut config: NetworkConfig<VerKey, StaticElectionConfig> =
                orchestrator_client.get_config(public_ip.to_string()).await;
            let node_index = config.node_index;

            // Get updated config from orchestrator containing all peer's public keys
            config = orchestrator_client
                .post_and_wait_all_public_keys(config.node_index, public_staking_key)
                .await;

            // `post_and_wait_all_public_keys` does not set `config.node_index` properly, so we fix
            // it by using the `node_index` from `get_config`. This will be fixed by
            // https://github.com/EspressoSystems/HotShot/issues/2618, after which we can remove
            // this line.
            config.node_index = node_index;

            config.config.my_own_validator_config.private_key = private_staking_key.clone();
            config.config.my_own_validator_config.public_key = public_staking_key;

            tracing::info!("loaded config, we are node {}", config.node_index);
            persistence.save_config(&config).await?;
            (config, true)
        }
    };
    let node_index = config.node_index;
    let num_nodes = config.config.total_nodes.get();

    let known_nodes_with_stake: Vec<<PubKey as SignatureKey>::StakeTableEntry> =
        config.config.known_nodes_with_stake.clone();

    let pub_keys = known_nodes_with_stake
        .iter()
        .map(|entry| entry.stake_key)
        .collect::<Vec<_>>();

    // TODO: fetch from orchestrator?
    let state_ver_keys = (0..num_nodes)
        .map(|i| StateKeyPair::generate_from_seed_indexed(config.seed, i as u64).ver_key())
        .collect::<Vec<_>>();

    let state_key_pair = config.config.my_own_validator_config.state_key_pair.clone();

    // Initialize networking.
    let networks = Networks {
        da_network: Arc::new(WebServerNetwork::create(
            network_params.da_server_url,
            network_params.webserver_poll_interval,
            pub_keys[node_index as usize],
            true,
        )),
        quorum_network: Arc::new(WebServerNetwork::create(
            network_params.consensus_server_url,
            network_params.webserver_poll_interval,
            pub_keys[node_index as usize],
            false,
        )),
        _pd: Default::default(),
    };

    // The web server network doesn't have any metrics. By creating and dropping a
    // `NetworkingMetricsValue`, we ensure the networking metrics are created, but just not
    // populated, so that monitoring software built to work with network-related metrics doesn't
    // crash horribly just because we're not using the P2P network yet.
    let _ = NetworkingMetricsValue::new(metrics);

    let wallet = MnemonicBuilder::<English>::default()
        .phrase::<&str>(&builder_mnemonic)
        .build()
        .unwrap();

    let instance_state = NodeState {
        builder_address: wallet,
        genesis_state: ValidatedState::default(),
    };
    let hotshot = init_hotshot(
        pub_keys.clone(),
        known_nodes_with_stake.clone(),
        node_index as usize,
        private_staking_key,
        networks,
        config.config,
        metrics,
        &instance_state,
    )
    .await;
    let mut ctx = SequencerContext::new(
        hotshot,
        node_index,
        state_key_pair,
        static_stake_table_commitment(
            &known_nodes_with_stake,
            &state_ver_keys,
            STAKE_TABLE_CAPACITY,
        ),
    )
    .with_state_relay_server(network_params.state_relay_server_url);
    if wait_for_orchestrator {
        ctx = ctx.wait_for_orchestrator(orchestrator_client);
    }
    Ok(ctx)
}

#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::*;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use commit::Committable;
    use futures::{Stream, StreamExt};
    use hotshot::traits::implementations::{MasterMap, MemoryNetwork};
    use hotshot::types::EventType::Decide;
    use hotshot_types::traits::block_contents::{BlockHeader, BlockPayload};

    use hotshot_types::{
        light_client::StateKeyPair, traits::metrics::NoMetrics, ExecutionType, ValidatorConfig,
    };
    use std::time::Duration;

    pub async fn init_hotshot_handles() -> Vec<SystemContextHandle<SeqTypes, Node<network::Memory>>>
    {
        init_hotshot_handles_with_metrics(&NoMetrics).await
    }

    pub async fn init_hotshot_handles_with_metrics(
        metrics: &dyn Metrics,
    ) -> Vec<SystemContextHandle<SeqTypes, Node<network::Memory>>> {
        setup_logging();
        setup_backtrace();

        let num_nodes = 5;

        // Generate keys for the nodes.
        let priv_keys = (0..num_nodes)
            .map(|_| PrivKey::generate(&mut rand::thread_rng()))
            .collect::<Vec<_>>();
        let pub_keys = priv_keys
            .iter()
            .map(PubKey::from_private)
            .collect::<Vec<_>>();
        let known_nodes_with_stake: Vec<<PubKey as SignatureKey>::StakeTableEntry> = (0..num_nodes)
            .map(|id| pub_keys[id].get_stake_table_entry(1u64))
            .collect();

        let mut handles = vec![];

        let master_map = MasterMap::new();

        let config: HotShotConfig<_, _> = HotShotConfig {
            execution_type: ExecutionType::Continuous,
            total_nodes: num_nodes.try_into().unwrap(),
            min_transactions: 1,
            max_transactions: 10000.try_into().unwrap(),
            known_nodes_with_stake: known_nodes_with_stake.clone(),
            next_view_timeout: Duration::from_secs(5).as_millis() as u64,
            timeout_ratio: (10, 11),
            round_start_delay: Duration::from_millis(1).as_millis() as u64,
            start_delay: Duration::from_millis(1).as_millis() as u64,
            num_bootstrap: 1usize,
            propose_min_round_time: Duration::from_secs(0),
            propose_max_round_time: Duration::from_secs(1),
            election_config: None,
            da_committee_size: num_nodes,
            my_own_validator_config: Default::default(),
        };

        // Create HotShot instances.
        for node_id in 0..num_nodes {
            let metrics = if node_id == 0 { metrics } else { &NoMetrics };

            let mut config = config.clone();
            config.my_own_validator_config = ValidatorConfig {
                public_key: pub_keys[node_id],
                private_key: priv_keys[node_id].clone(),
                stake_value: known_nodes_with_stake[node_id].stake_amount.as_u64(),
                state_key_pair: StateKeyPair::generate(),
            };

            let network = Arc::new(MemoryNetwork::new(
                pub_keys[node_id],
                NetworkingMetricsValue::new(metrics),
                master_map.clone(),
                None,
            ));
            let networks = Networks {
                da_network: network.clone(),
                quorum_network: network,
                _pd: Default::default(),
            };
            let instance_state = &NodeState::default();
            let handle = init_hotshot(
                pub_keys.clone(),
                known_nodes_with_stake.clone(),
                node_id,
                priv_keys[node_id].clone(),
                networks,
                config,
                metrics,
                instance_state,
            )
            .await;

            handles.push(handle);
        }
        handles
    }

    // Wait for decide event, make sure it matches submitted transaction. Return the block number
    // containing the transaction.
    pub async fn wait_for_decide_on_handle(
        events: &mut (impl Stream<Item = Event> + Unpin),
        submitted_txn: &Transaction,
    ) -> u64 {
        let commitment = submitted_txn.commit();

        // Keep getting events until we see a Decide event
        loop {
            let event = events.next().await.unwrap();
            tracing::info!("Received event from handle: {event:?}");

            if let Decide { leaf_chain, .. } = event.event {
                if let Some(height) = leaf_chain.iter().find_map(|(leaf, _)| {
                    if leaf
                        .block_payload
                        .as_ref()?
                        .transaction_commitments(leaf.get_block_header().metadata())
                        .contains(&commitment)
                    {
                        Some(leaf.get_block_header().block_number())
                    } else {
                        None
                    }
                }) {
                    return height;
                }
            } else {
                // Keep waiting
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::{transaction::ApplicationTransaction, vm::TestVm, *};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use futures::StreamExt;
    use hotshot::types::EventType::Decide;

    use hotshot_types::traits::block_contents::BlockHeader;
    use testing::{init_hotshot_handles, wait_for_decide_on_handle};

    #[async_std::test]
    async fn test_skeleton_instantiation() {
        setup_logging();
        setup_backtrace();

        let handles = init_hotshot_handles().await;
        let mut events = handles[0].get_event_stream();
        for handle in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        // Submit target transaction to handle
        let txn = ApplicationTransaction::new(vec![1, 2, 3]);
        let submitted_txn = Transaction::new(TestVm {}.id(), bincode::serialize(&txn).unwrap());
        handles[0]
            .submit_transaction(submitted_txn.clone())
            .await
            .expect("Failed to submit transaction");
        tracing::info!("Submitted transaction to handle: {txn:?}");

        wait_for_decide_on_handle(&mut events, &submitted_txn).await;
    }

    #[async_std::test]
    async fn test_header_invariants() {
        setup_logging();
        setup_backtrace();

        let success_height = 30;

        let handles = init_hotshot_handles().await;
        let mut events = handles[0].get_event_stream();
        for handle in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        let mut parent = Header::genesis(&NodeState::default()).0;

        loop {
            let event = events.next().await.unwrap();
            let Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            tracing::info!("Got decide {leaf_chain:?}");

            // Check that each successive header satisfies invariants relative to its parent: all
            // the fields which should be monotonic are.
            for (leaf, _) in leaf_chain.iter().rev() {
                let header = leaf.block_header.clone();
                if header.height == 0 {
                    parent = header;
                    continue;
                }
                dbg!(header.height, parent.height);
                assert_eq!(header.height, parent.height + 1);
                assert!(header.timestamp >= parent.timestamp);
                assert!(header.l1_head >= parent.l1_head);
                assert!(header.l1_finalized >= parent.l1_finalized);
                parent = header;
            }

            if parent.height >= success_height {
                break;
            }
        }
    }
}
