pub mod api;
mod block;
mod block2;
mod chain_variables;
pub mod context;
mod header;
pub mod hotshot_commitment;
pub mod options;
pub mod state_signature;
use context::SequencerContext;
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{coins_bip39::English, MnemonicBuilder, Wallet},
};
// Should move `STAKE_TABLE_CAPACITY` in the sequencer repo when we have variate stake table support
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;
use hotshot_types::light_client::StateKeyPair;
use state_signature::static_stake_table_commitment;
use url::Url;
mod l1_client;
pub mod persistence;
mod state;
pub mod transaction;
mod vm;

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use derivative::Derivative;
use hotshot::{
    traits::{
        election::static_committee::{GeneralStaticCommittee, StaticElectionConfig},
        implementations::{
            MemoryCommChannel, MemoryStorage, NetworkingMetricsValue, WebCommChannel,
            WebServerNetwork,
        },
        NodeImplementation,
    },
    types::{SignatureKey, SystemContextHandle},
    HotShotInitializer, Memberships, Networks, SystemContext,
};
use hotshot_orchestrator::client::{OrchestratorClient, ValidatorArgs};
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    data::ViewNumber,
    signature_key::BLSPubKey,
    traits::{
        metrics::Metrics, network::CommunicationChannel, node_implementation::NodeType,
        states::InstanceState,
    },
    HotShotConfig, ValidatorConfig,
};

use jf_primitives::merkle_tree::{
    namespaced_merkle_tree::NamespacedMerkleTreeScheme, MerkleTreeScheme,
};
use persistence::SequencerPersistence;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::net::Ipv4Addr;
use std::time::Duration;
use std::{fmt::Debug, sync::Arc};
use std::{marker::PhantomData, net::IpAddr};
use typenum::U2;

pub use block::Payload;
pub use chain_variables::ChainVariables;
pub use header::Header;
use jf_primitives::merkle_tree::{
    namespaced_merkle_tree::NMT,
    prelude::{Sha3Digest, Sha3Node},
};
pub use l1_client::L1BlockInfo;
pub use options::Options;
pub use state::ValidatedState;
pub use transaction::Transaction;
pub use vm::{Vm, VmId, VmTransaction};

// Supports 1K transactions
pub const MAX_NMT_DEPTH: usize = 10;

pub type TransactionNMT = NMT<Transaction, Sha3Digest, U2, VmId, Sha3Node>;
pub type NamespaceProofType = <TransactionNMT as NamespacedMerkleTreeScheme>::NamespaceProof;

/// Initialize the static variables for the sequencer
///
/// Calling it early on startup makes it easier to catch errors.
pub fn init_static() {
    lazy_static::initialize(&header::L1_CLIENT);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NMTRoot {
    #[serde(with = "nmt_root_serializer")]
    root: <TransactionNMT as MerkleTreeScheme>::NodeValue,
}

mod nmt_root_serializer {
    use serde::{Deserializer, Serializer};

    use super::*;

    pub fn serialize<A, S>(a: &A, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        A: CanonicalSerialize,
    {
        let mut bytes = vec![];
        a.serialize_with_mode(&mut bytes, ark_serialize::Compress::Yes)
            .map_err(serde::ser::Error::custom)?;
        s.serialize_bytes(&bytes)
    }

    pub fn deserialize<'de, D, A>(deserializer: D) -> Result<A, D::Error>
    where
        D: Deserializer<'de>,
        A: CanonicalDeserialize,
    {
        let s: Vec<u8> = serde::de::Deserialize::deserialize(deserializer)?;
        let a = A::deserialize_with_mode(s.as_slice(), Compress::Yes, Validate::Yes);
        a.map_err(serde::de::Error::custom)
    }
}

impl NMTRoot {
    pub fn root(&self) -> <TransactionNMT as MerkleTreeScheme>::NodeValue {
        self.root
    }
}

pub mod network {
    use super::*;

    pub trait Type: 'static {
        type DAChannel: CommunicationChannel<SeqTypes> + Debug;
        type QuorumChannel: CommunicationChannel<SeqTypes> + Debug;
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Web;

    impl Type for Web {
        type DAChannel = WebCommChannel<SeqTypes>;
        type QuorumChannel = WebCommChannel<SeqTypes>;
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Memory;

    impl Type for Memory {
        type DAChannel = MemoryCommChannel<SeqTypes>;
        type QuorumChannel = MemoryCommChannel<SeqTypes>;
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
        let phrase = "test test test test test test test test test test test junk";
        let wallet = MnemonicBuilder::<English>::default()
            .phrase::<&str>(phrase)
            .build()
            .unwrap();

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
    type BlockPayload = Payload;
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

    let (mut config, wait_for_orchestrator) = match persistence.load_config().await? {
        Some(config) => {
            tracing::info!("loaded network config from storage, rejoining existing network");
            (config, false)
        }
        None => {
            tracing::info!("loading network config from orchestrator");
            let config = orchestrator_client.get_config(public_ip.to_string()).await;
            tracing::info!("loaded config, we are node {}", config.node_index);
            persistence.save_config(&config).await?;
            (config, true)
        }
    };
    let node_index = config.node_index;

    // Generate public keys and this node's private keys.
    //
    // These are deterministic keys suitable *only* for testing and demo purposes.
    config.config.my_own_validator_config =
        ValidatorConfig::<PubKey>::generated_from_seed_indexed(config.seed, node_index, 1);
    let priv_key = config.config.my_own_validator_config.private_key.clone();
    let num_nodes = config.config.total_nodes.get();
    let pub_keys = (0..num_nodes)
        .map(|i| PubKey::generated_from_seed_indexed(config.seed, i as u64).0)
        .collect::<Vec<_>>();
    let known_nodes_with_stake: Vec<<PubKey as SignatureKey>::StakeTableEntry> = (0..num_nodes)
        .map(|id| pub_keys[id].get_stake_table_entry(1u64))
        .collect();
    let state_ver_keys = (0..num_nodes)
        .map(|i| StateKeyPair::generate_from_seed_indexed(config.seed, i as u64).ver_key())
        .collect::<Vec<_>>();
    let state_key_pair = StateKeyPair::generate_from_seed_indexed(config.seed, node_index);

    // Initialize networking.
    let networks = Networks {
        da_network: WebCommChannel::new(Arc::new(WebServerNetwork::create(
            network_params.da_server_url,
            network_params.webserver_poll_interval,
            pub_keys[node_index as usize],
            true,
        ))),
        quorum_network: WebCommChannel::new(Arc::new(WebServerNetwork::create(
            network_params.consensus_server_url,
            network_params.webserver_poll_interval,
            pub_keys[node_index as usize],
            false,
        ))),
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
        priv_key,
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
    use hotshot::traits::{
        implementations::{MasterMap, MemoryNetwork},
        BlockPayload,
    };
    use hotshot::types::EventType::Decide;

    use hotshot_types::{
        traits::{block_contents::BlockHeader, metrics::NoMetrics},
        ExecutionType, ValidatorConfig,
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
                state_key_pair: Default::default(),
            };

            let network = Arc::new(MemoryNetwork::new(
                pub_keys[node_id],
                NetworkingMetricsValue::new(metrics),
                master_map.clone(),
                None,
            ));
            let networks = Networks {
                da_network: MemoryCommChannel::new(network.clone()),
                quorum_network: MemoryCommChannel::new(network),
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

    // Wait for decide event, make sure it matches submitted transaction
    pub async fn wait_for_decide_on_handle(
        events: &mut (impl Stream<Item = Event> + Unpin),
        submitted_txn: &Transaction,
    ) -> Result<(), ()> {
        let commitment = submitted_txn.commit();

        // Keep getting events until we see a Decide event
        loop {
            let event = events.next().await;
            tracing::info!("Received event from handle: {event:?}");

            if let Some(Event {
                event: Decide { leaf_chain, .. },
                ..
            }) = event
            {
                if leaf_chain.iter().any(|leaf| match &leaf.block_payload {
                    Some(ref block) => block
                        .transaction_commitments(leaf.get_block_header().metadata())
                        .contains(&commitment),
                    None => false,
                }) {
                    return Ok(());
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
    async fn test_skeleton_instantiation() -> Result<(), ()> {
        setup_logging();
        setup_backtrace();

        let mut handles = init_hotshot_handles().await;
        let mut events = handles[0].get_event_stream(Default::default()).await.0;
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

        wait_for_decide_on_handle(&mut events, &submitted_txn).await
    }

    #[async_std::test]
    async fn test_header_invariants() {
        setup_logging();
        setup_backtrace();

        let success_height = 30;

        let mut handles = init_hotshot_handles().await;
        let mut events = handles[0].get_event_stream(Default::default()).await.0;
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
            for (i, leaf) in leaf_chain.iter().rev().enumerate() {
                let header = leaf.block_header.clone();
                if i == 0 {
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
