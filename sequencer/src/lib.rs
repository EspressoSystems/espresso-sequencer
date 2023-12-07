pub mod api;
mod block;
mod block2;
mod chain_variables;
pub mod hotshot_commitment;
pub mod options;
use url::Url;
mod l1_client;
mod state;
pub mod transaction;
mod vm;

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use derivative::Derivative;
use hotshot::{
    traits::{
        election::static_committee::{
            GeneralStaticCommittee, StaticElectionConfig, StaticVoteToken,
        },
        implementations::{MemoryCommChannel, MemoryStorage, WebCommChannel, WebServerNetwork},
        NodeImplementation,
    },
    types::{Message, SignatureKey, SystemContextHandle},
    HotShotInitializer, SystemContext,
};
use hotshot_signature_key::bn254::BN254Pub;
use hotshot_types::{
    certificate::ViewSyncCertificate,
    data::{DAProposal, QuorumProposal, SequencingLeaf, ViewNumber},
    message::SequencingMessage,
    traits::{
        election::{
            CommitteeExchange, Membership as MembershipTrait, QuorumExchange, ViewSyncExchange,
        },
        metrics::{Metrics, NoMetrics},
        network::CommunicationChannel,
        node_implementation::{ChannelMaps, ExchangesType, NodeType, SequencingExchanges},
    },
    vote::{DAVote, QuorumVote, ViewSyncVote},
    HotShotConfig,
};

use jf_primitives::merkle_tree::{
    namespaced_merkle_tree::NamespacedMerkleTreeScheme, MerkleTreeScheme,
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::net::Ipv4Addr;
use std::time::Duration;
use std::{fmt::Debug, sync::Arc};
use std::{marker::PhantomData, net::IpAddr};
use typenum::U2;

pub use block::{Header, Payload};
pub use chain_variables::ChainVariables;
use jf_primitives::merkle_tree::{
    examples::{Sha3Digest, Sha3Node},
    namespaced_merkle_tree::NMT,
};
pub use l1_client::L1BlockInfo;
pub use options::Options;
pub use state::State;
pub use transaction::Transaction;
pub use vm::{Vm, VmId, VmTransaction};

use hotshot_orchestrator::client::{OrchestratorClient, ValidatorArgs};

// Supports 1K transactions
pub const MAX_NMT_DEPTH: usize = 10;

pub type TransactionNMT = NMT<Transaction, Sha3Digest, U2, VmId, Sha3Node>;
pub type NamespaceProofType = <TransactionNMT as NamespacedMerkleTreeScheme>::NamespaceProof;

/// Initialize the static variables for the sequencer
///
/// Calling it early on startup makes it easier to catch errors.
pub fn init_static() {
    lazy_static::initialize(&state::L1_CLIENT);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
        type DAChannel<I: NodeImplementation<SeqTypes>>: CommunicationChannel<
                SeqTypes,
                Message<SeqTypes, I>,
                DAProposal<SeqTypes>,
                DAVote<SeqTypes>,
                Membership,
            > + Debug;
        type QuorumChannel<I: NodeImplementation<SeqTypes>>: CommunicationChannel<
                SeqTypes,
                Message<SeqTypes, I>,
                QuorumProposal<SeqTypes, Leaf>,
                QuorumVote<SeqTypes, Leaf>,
                Membership,
            > + Debug;
        type ViewSyncChannel<I: NodeImplementation<SeqTypes>>: CommunicationChannel<
                SeqTypes,
                Message<SeqTypes, I>,
                ViewSyncCertificate<SeqTypes>,
                ViewSyncVote<SeqTypes>,
                Membership,
            > + Debug;
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Web;

    impl Type for Web {
        type DAChannel<I: NodeImplementation<SeqTypes>> =
            WebCommChannel<SeqTypes, I, DAProposal<SeqTypes>, DAVote<SeqTypes>, Membership>;
        type QuorumChannel<I: NodeImplementation<SeqTypes>> = WebCommChannel<
            SeqTypes,
            I,
            QuorumProposal<SeqTypes, Leaf>,
            QuorumVote<SeqTypes, Leaf>,
            Membership,
        >;
        type ViewSyncChannel<I: NodeImplementation<SeqTypes>> = WebCommChannel<
            SeqTypes,
            I,
            ViewSyncCertificate<SeqTypes>,
            ViewSyncVote<SeqTypes>,
            Membership,
        >;
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Memory;

    impl Type for Memory {
        type DAChannel<I: NodeImplementation<SeqTypes>> =
            MemoryCommChannel<SeqTypes, I, DAProposal<SeqTypes>, DAVote<SeqTypes>, Membership>;
        type QuorumChannel<I: NodeImplementation<SeqTypes>> = MemoryCommChannel<
            SeqTypes,
            I,
            QuorumProposal<SeqTypes, Leaf>,
            QuorumVote<SeqTypes, Leaf>,
            Membership,
        >;
        type ViewSyncChannel<I: NodeImplementation<SeqTypes>> = MemoryCommChannel<
            SeqTypes,
            I,
            ViewSyncCertificate<SeqTypes>,
            ViewSyncVote<SeqTypes>,
            Membership,
        >;
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

pub type Leaf = SequencingLeaf<SeqTypes>;
pub type Membership = GeneralStaticCommittee<SeqTypes, Leaf, SignatureKeyType>;
pub type Storage = MemoryStorage<SeqTypes, Leaf>;
pub type Event = hotshot::types::Event<SeqTypes, Leaf>;

pub type SignatureKeyType = BN254Pub;
type ElectionConfig = StaticElectionConfig;

impl<N: network::Type> NodeImplementation<SeqTypes> for Node<N> {
    type Leaf = Leaf;
    type Storage = Storage;
    type ConsensusMessage = SequencingMessage<SeqTypes, Self>;
    type Exchanges = SequencingExchanges<
        SeqTypes,
        Message<SeqTypes, Self>,
        QuorumExchange<
            SeqTypes,
            Leaf,
            QuorumProposal<SeqTypes, Leaf>,
            Membership,
            N::QuorumChannel<Self>,
            Message<SeqTypes, Self>,
        >,
        CommitteeExchange<SeqTypes, Membership, N::DAChannel<Self>, Message<SeqTypes, Self>>,
        ViewSyncExchange<
            SeqTypes,
            ViewSyncCertificate<SeqTypes>,
            Membership,
            N::ViewSyncChannel<Self>,
            Message<SeqTypes, Self>,
        >,
    >;

    fn new_channel_maps(
        start_view: ViewNumber,
    ) -> (
        ChannelMaps<SeqTypes, Self>,
        Option<ChannelMaps<SeqTypes, Self>>,
    ) {
        (ChannelMaps::new(start_view), None)
    }
}

impl NodeType for SeqTypes {
    type Time = ViewNumber;
    type BlockType = Block;
    type SignatureKey = SignatureKeyType;
    type VoteTokenType = StaticVoteToken<SignatureKeyType>;
    type Transaction = Transaction;
    type ElectionConfigType = ElectionConfig;
    type StateType = State;
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
}

type PubKey = BN254Pub;
type PrivKey = <PubKey as SignatureKey>::PrivateKey;

struct CommChannels<N: network::Type> {
    da: N::DAChannel<Node<N>>,
    quorum: N::QuorumChannel<Node<N>>,
    view_sync: N::ViewSyncChannel<Node<N>>,
}

impl CommChannels<network::Web> {
    fn web(network_params: NetworkParams, pub_key: PubKey) -> Self {
        let wait_time = Duration::from_millis(100);
        let da_network = Arc::new(WebServerNetwork::create(
            &network_params.da_server_url.host().unwrap().to_string(),
            network_params
                .da_server_url
                .port_or_known_default()
                .unwrap(),
            wait_time,
            pub_key,
            true,
        ));
        let consensus_network = Arc::new(WebServerNetwork::create(
            &network_params
                .consensus_server_url
                .host()
                .unwrap()
                .to_string(),
            network_params
                .consensus_server_url
                .port_or_known_default()
                .unwrap(),
            wait_time,
            pub_key,
            false,
        ));
        Self {
            da: WebCommChannel::new(da_network),
            quorum: WebCommChannel::new(consensus_network.clone()),
            view_sync: WebCommChannel::new(consensus_network),
        }
    }
}

async fn init_hotshot<N: network::Type>(
    nodes_pub_keys: Vec<PubKey>,
    known_nodes_with_stake: Vec<<PubKey as SignatureKey>::StakeTableEntry>,
    genesis_block: Block,
    node_id: usize,
    priv_key: PrivKey,
    channels: CommChannels<N>,
    config: HotShotConfig<PubKey, <PubKey as SignatureKey>::StakeTableEntry, ElectionConfig>,
) -> SystemContextHandle<SeqTypes, Node<N>> {
    let storage = Storage::empty();
    let initializer = HotShotInitializer::<SeqTypes, SequencingLeaf<SeqTypes>>::from_genesis(
        genesis_block.clone(),
    )
    .unwrap();
    let metrics = Box::<NoMetrics>::default();
    let num_nodes = nodes_pub_keys.len() as u64;
    let election_config = Membership::default_election_config(num_nodes);

    let exchanges = SequencingExchanges::create(
        known_nodes_with_stake,
        nodes_pub_keys.clone(),
        (election_config.clone(), election_config),
        (channels.quorum, channels.da, channels.view_sync),
        nodes_pub_keys[node_id],
        nodes_pub_keys[node_id].get_stake_table_entry(1u64),
        priv_key.clone(),
    );

    SystemContext::init(
        nodes_pub_keys[node_id],
        priv_key,
        node_id as u64,
        config,
        storage,
        exchanges,
        initializer,
        metrics,
    )
    .await
    .unwrap()
    .0
}

pub struct NetworkParams {
    pub da_server_url: Url,
    pub consensus_server_url: Url,
    pub orchestrator_url: Url,
}

pub async fn init_node(
    network_params: NetworkParams,
    genesis_block: Block,
    _metrics: Box<dyn Metrics>,
) -> (SystemContextHandle<SeqTypes, Node<network::Web>>, u64) {
    // Orchestrator client
    let validator_args = ValidatorArgs {
        host: network_params.orchestrator_url.host().unwrap().to_string(),
        port: network_params
            .orchestrator_url
            .port_or_known_default()
            .unwrap(),
        public_ip: None,
    };
    let orchestrator_client = OrchestratorClient::connect_to_orchestrator(validator_args).await;

    // This "public" IP only applies to libp2p network configurations, so we can supply any value here
    let public_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let node_index: u16 = orchestrator_client
        .identify_with_orchestrator(public_ip.to_string())
        .await;

    let config = orchestrator_client
        .get_config_from_orchestrator::<SeqTypes>(node_index)
        .await;

    // Generate public keys and this node's private keys.
    //
    // These are deterministic keys suitable *only* for testing and demo purposes.
    let num_nodes = config.config.total_nodes.get();
    let (pub_keys, priv_keys): (Vec<_>, Vec<_>) = (0..num_nodes)
        .map(|i| SignatureKeyType::generated_from_seed_indexed(config.seed, i as u64))
        .unzip();
    let priv_key = priv_keys[node_index as usize].clone();
    let known_nodes_with_stake: Vec<<BN254Pub as SignatureKey>::StakeTableEntry> = (0..num_nodes)
        .map(|id| pub_keys[id].get_stake_table_entry(1u64))
        .collect();

    // Wait for other nodes to connect.
    orchestrator_client
        .wait_for_all_nodes_ready(node_index.into())
        .await;

    (
        init_hotshot(
            pub_keys.clone(),
            known_nodes_with_stake.clone(),
            genesis_block,
            node_index as usize,
            priv_key,
            CommChannels::web(network_params, pub_keys[node_index as usize]),
            config.config,
        )
        .await,
        node_index.into(),
    )
}

#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::*;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use either::Either;
    use futures::{Stream, StreamExt};
    use hotshot::traits::implementations::{MasterMap, MemoryNetwork};
    use hotshot::types::EventType::Decide;
    use hotshot_signature_key::bn254::BN254Priv;
    use hotshot_types::{
        data::LeafType,
        traits::{
            election::ConsensusExchange,
            network::{TestableChannelImplementation, TestableNetworkingImplementation},
            node_implementation::TestableExchange,
        },
        ExecutionType,
    };
    use std::time::Duration;

    impl TestableExchange<SeqTypes, Leaf, Message<SeqTypes, Node<network::Memory>>>
        for <Node<network::Memory> as NodeImplementation<SeqTypes>>::Exchanges
    {
        fn gen_comm_channels(
            expected_node_count: usize,
            num_bootstrap: usize,
            da_committee_size: usize,
        ) -> Box<
            dyn Fn(
                    u64,
                ) -> (
                    <Self::QuorumExchange as ConsensusExchange<
                        SeqTypes,
                        Message<SeqTypes, Node<network::Memory>>,
                    >>::Networking,
                    <Self::CommitteeExchange as ConsensusExchange<
                        SeqTypes,
                        Message<SeqTypes, Node<network::Memory>>,
                    >>::Networking,
                    <Self::ViewSyncExchange as ConsensusExchange<
                        SeqTypes,
                        Message<SeqTypes, Node<network::Memory>>,
                    >>::Networking,
                ) + 'static,
        > {
            #[allow(clippy::arc_with_non_send_sync)]
            let network_generator = Arc::new(<MemoryNetwork<
                Message<SeqTypes, Node<network::Memory>>,
                <SeqTypes as NodeType>::SignatureKey,
            > as TestableNetworkingImplementation<
                SeqTypes,
                Message<SeqTypes, Node<network::Memory>>,
            >>::generator(
                expected_node_count,
                num_bootstrap,
                0,
                da_committee_size,
                false,
            ));
            #[allow(clippy::arc_with_non_send_sync)]
            let network_da_generator = Arc::new(<MemoryNetwork<
                Message<SeqTypes, Node<network::Memory>>,
                <SeqTypes as NodeType>::SignatureKey,
            > as TestableNetworkingImplementation<
                SeqTypes,
                Message<SeqTypes, Node<network::Memory>>,
            >>::generator(
                expected_node_count,
                num_bootstrap,
                1,
                da_committee_size,
                true,
            ));
            Box::new(move |id| {
                let network = Arc::new(network_generator(id));
                let network_da = Arc::new(network_da_generator(id));
                let quorum_chan = <<Self::QuorumExchange as hotshot_types::traits::election::ConsensusExchange<SeqTypes, Message<SeqTypes, Node<network::Memory>>>>::Networking as TestableChannelImplementation<_, _, _, _, _, _>>::generate_network()(network.clone());
                let committee_chan = <<Self::CommitteeExchange as hotshot_types::traits::election::ConsensusExchange<SeqTypes, Message<SeqTypes, Node<network::Memory>>>>::Networking as TestableChannelImplementation<_, _, _, _, _, _>>::generate_network()(network_da);
                let view_sync_chan = <<Self::ViewSyncExchange as hotshot_types::traits::election::ConsensusExchange<SeqTypes, Message<SeqTypes, Node<network::Memory>>>>::Networking as TestableChannelImplementation<_, _, _, _, _, _>>::generate_network()(network);

                (quorum_chan, committee_chan, view_sync_chan)
            })
        }
    }

    impl CommChannels<network::Memory> {
        fn memory(
            master_map: Arc<MasterMap<Message<SeqTypes, Node<network::Memory>>, PubKey>>,
            pub_key: PubKey,
        ) -> Self {
            let network = Arc::new(MemoryNetwork::new(
                pub_key,
                Box::<NoMetrics>::default(),
                master_map,
                None,
            ));
            Self {
                da: MemoryCommChannel::new(network.clone()),
                quorum: MemoryCommChannel::new(network.clone()),
                view_sync: MemoryCommChannel::new(network),
            }
        }
    }

    pub async fn init_hotshot_handles() -> Vec<SystemContextHandle<SeqTypes, Node<network::Memory>>>
    {
        setup_logging();
        setup_backtrace();

        let genesis_block = Block::genesis();

        let num_nodes = 5;

        // Generate keys for the nodes.
        let priv_keys = (0..num_nodes)
            .map(|_| BN254Priv::generate())
            .collect::<Vec<_>>();
        let pub_keys = priv_keys
            .iter()
            .map(BN254Pub::from_private)
            .collect::<Vec<_>>();
        let known_nodes_with_stake: Vec<<BN254Pub as SignatureKey>::StakeTableEntry> = (0
            ..num_nodes)
            .map(|id| pub_keys[id].get_stake_table_entry(1u64))
            .collect();

        let mut handles = vec![];

        let master_map = MasterMap::new();

        let config: HotShotConfig<_, _, _> = HotShotConfig {
            execution_type: ExecutionType::Continuous,
            total_nodes: num_nodes.try_into().unwrap(),
            known_nodes: pub_keys.clone(),
            min_transactions: 1,
            max_transactions: 10000.try_into().unwrap(),
            known_nodes_with_stake: known_nodes_with_stake.clone(),
            next_view_timeout: Duration::from_secs(60).as_millis() as u64,
            timeout_ratio: (10, 11),
            round_start_delay: Duration::from_millis(1).as_millis() as u64,
            start_delay: Duration::from_millis(1).as_millis() as u64,
            num_bootstrap: 1usize,
            propose_min_round_time: Duration::from_secs(1),
            propose_max_round_time: Duration::from_secs(1),
            election_config: Some(Membership::default_election_config(num_nodes as u64)),
            da_committee_size: num_nodes,
        };

        // Create HotShot instances.
        for node_id in 0..num_nodes {
            let handle = init_hotshot(
                pub_keys.clone(),
                known_nodes_with_stake.clone(),
                genesis_block.clone(),
                node_id,
                priv_keys[node_id].clone(),
                CommChannels::memory(master_map.clone(), pub_keys[node_id]),
                config.clone(),
            )
            .await;

            handles.push(handle);
        }
        handles
    }

    // Wait for decide event, make sure it matches submitted transaction
    pub async fn wait_for_decide_on_handle(
        events: &mut (impl Stream<Item = Event> + Unpin),
        submitted_txn: Transaction,
    ) -> Result<(), ()> {
        // Keep getting events until we see a Decide event
        loop {
            let event = events.next().await;
            tracing::info!("Received event from handle: {event:?}");

            if let Some(Event {
                event: Decide {
                    leaf_chain: leaf, ..
                },
                ..
            }) = event
            {
                if leaf.iter().any(|leaf| match leaf.get_deltas() {
                    Either::Left(block) => block
                        .transaction_nmt
                        .leaves()
                        .any(|txn| txn == &submitted_txn),
                    Either::Right(_) => false,
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
    use hotshot_testing::test_builder::TestMetadata;
    use testing::{init_hotshot_handles, wait_for_decide_on_handle};

    // Run a hotshot test with our types
    #[async_std::test]
    async fn hotshot_test() {
        setup_logging();
        setup_backtrace();

        TestMetadata::default()
            .gen_launcher::<SeqTypes, Node<network::Memory>>()
            .launch()
            .run_test()
            .await;
    }

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

        wait_for_decide_on_handle(&mut events, submitted_txn).await
    }
}
