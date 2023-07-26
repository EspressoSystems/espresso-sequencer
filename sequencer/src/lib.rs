pub mod api;
mod block;
mod chain_variables;
pub mod hotshot_commitment;
pub mod options;
use url::Url;
mod state;
pub mod transaction;
mod vm;

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use derivative::Derivative;
use hotshot::{
    traits::{
        election::{
            static_committee::{GeneralStaticCommittee, StaticElectionConfig, StaticVoteToken},
            vrf::JfPubKey,
        },
        implementations::{MemoryCommChannel, MemoryStorage, WebCommChannel, WebServerNetwork},
        NodeImplementation,
    },
    types::{HotShotHandle, Message, SignatureKey},
    HotShot, HotShotInitializer,
};
use hotshot_types::{
    data::{DAProposal, QuorumProposal, SequencingLeaf, ViewNumber},
    message::SequencingMessage,
    traits::{
        consensus_type::sequencing_consensus::SequencingConsensus,
        election::{CommitteeExchange, Membership as MembershipTrait, QuorumExchange},
        metrics::{Metrics, NoMetrics},
        network::CommunicationChannel,
        node_implementation::{ExchangesType, NodeType, SequencingExchanges},
    },
    vote::{DAVote, QuorumVote},
    HotShotConfig,
};

use jf_primitives::{
    aead::KeyPair,
    merkle_tree::{namespaced_merkle_tree::NamespacedMerkleTreeScheme, MerkleTreeScheme},
    signatures::BLSSignatureScheme,
};
use rand::{rngs::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::net::Ipv4Addr;
use std::time::Duration;
use std::{fmt::Debug, sync::Arc};
use std::{marker::PhantomData, net::IpAddr};
use typenum::U2;

pub use block::Block;
pub use chain_variables::ChainVariables;
use jf_primitives::merkle_tree::{
    examples::{Sha3Digest, Sha3Node},
    namespaced_merkle_tree::NMT,
};
pub use options::Options;
pub use state::State;
pub use transaction::Transaction;
pub use vm::{Vm, VmId, VmTransaction};

use hotshot_orchestrator::client::{OrchestratorClient, ValidatorArgs};

// Supports 1K transactions
pub const MAX_NMT_DEPTH: usize = 10;

pub type TransactionNMT = NMT<Transaction, Sha3Digest, U2, VmId, Sha3Node>;
pub type NamespaceProofType = <TransactionNMT as NamespacedMerkleTreeScheme>::NamespaceProof;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
                DAVote<SeqTypes, Leaf>,
                Membership,
            > + Debug;
        type QuorumChannel<I: NodeImplementation<SeqTypes>>: CommunicationChannel<
                SeqTypes,
                Message<SeqTypes, I>,
                QuorumProposal<SeqTypes, Leaf>,
                QuorumVote<SeqTypes, Leaf>,
                Membership,
            > + Debug;
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Web;

    impl Type for Web {
        type DAChannel<I: NodeImplementation<SeqTypes>> =
            WebCommChannel<SeqTypes, I, DAProposal<SeqTypes>, DAVote<SeqTypes, Leaf>, Membership>;
        type QuorumChannel<I: NodeImplementation<SeqTypes>> = WebCommChannel<
            SeqTypes,
            I,
            QuorumProposal<SeqTypes, Leaf>,
            QuorumVote<SeqTypes, Leaf>,
            Membership,
        >;
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Memory;

    impl Type for Memory {
        type DAChannel<I: NodeImplementation<SeqTypes>> = MemoryCommChannel<
            SeqTypes,
            I,
            DAProposal<SeqTypes>,
            DAVote<SeqTypes, Leaf>,
            Membership,
        >;
        type QuorumChannel<I: NodeImplementation<SeqTypes>> = MemoryCommChannel<
            SeqTypes,
            I,
            QuorumProposal<SeqTypes, Leaf>,
            QuorumVote<SeqTypes, Leaf>,
            Membership,
        >;
    }
}

/// The Sequencer node is generic over the hotshot CommChannel.
#[derive(Derivative, Serialize, Deserialize)]
#[derivative(
    Clone(bound = ""),
    Copy(bound = ""),
    Debug(bound = ""),
    Default(bound = "")
)]
pub struct Node<N: network::Type>(PhantomData<fn(&N)>);

#[derive(
    Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct SeqTypes;

pub type Leaf = SequencingLeaf<SeqTypes>;
pub type Membership = GeneralStaticCommittee<SeqTypes, Leaf, SignatureKeyType>;
pub type Storage = MemoryStorage<SeqTypes, Leaf>;

pub type SignatureSchemeType = BLSSignatureScheme;
pub type SignatureKeyType = JfPubKey<SignatureSchemeType>;
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
    >;
}

impl NodeType for SeqTypes {
    type ConsensusType = SequencingConsensus;
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

type PubKey = JfPubKey<SignatureSchemeType>;
type PrivKey = <PubKey as SignatureKey>::PrivateKey;

#[allow(clippy::too_many_arguments)]
async fn init_hotshot<N: network::Type>(
    nodes_pub_keys: Vec<PubKey>,
    genesis_block: Block,
    node_id: usize,
    priv_key: PrivKey,
    enc_key: KeyPair,
    da_channel: N::DAChannel<Node<N>>,
    quorum_channel: N::QuorumChannel<Node<N>>,
    config: HotShotConfig<PubKey, ElectionConfig>,
) -> HotShotHandle<SeqTypes, Node<N>> {
    // Create public and private keys for the node.
    let public_key = PubKey::from_private(&priv_key);

    let storage = Storage::empty();
    let initializer = HotShotInitializer::<SeqTypes, SequencingLeaf<SeqTypes>>::from_genesis(
        genesis_block.clone(),
    )
    .unwrap();
    let metrics = Box::<NoMetrics>::default();
    let num_nodes = nodes_pub_keys.len() as u64;
    let election_config = Membership::default_election_config(num_nodes);

    let exchanges = SequencingExchanges::create(
        nodes_pub_keys.clone(),
        (election_config.clone(), election_config),
        (quorum_channel, da_channel),
        public_key.clone(),
        priv_key.clone(),
        enc_key,
    );

    HotShot::init(
        public_key,
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
) -> (HotShotHandle<SeqTypes, Node<network::Web>>, u64) {
    // Orchestrator client
    let validator_args = ValidatorArgs {
        host: network_params.orchestrator_url.host().unwrap().to_string(),
        port: network_params.orchestrator_url.port().unwrap(),
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
    let (pub_keys, priv_keys): (Vec<_>, Vec<_>) = (0..config.config.total_nodes.get())
        .map(|i| SignatureKeyType::generated_from_seed_indexed(config.seed, i as u64))
        .unzip();
    let priv_key = priv_keys[node_index as usize].clone();
    let enc_key = KeyPair::generate(&mut StdRng::seed_from_u64(config.node_index));

    // Wait for other nodes to connect.
    orchestrator_client
        .wait_for_all_nodes_ready(node_index.into())
        .await;
    let wait_time = Duration::from_millis(100);
    let da_network = WebServerNetwork::create(
        &network_params.da_server_url.host().unwrap().to_string(),
        network_params.da_server_url.port().unwrap(),
        wait_time,
        pub_keys[node_index as usize].clone(),
        pub_keys.clone(),
    );
    let consensus_network = WebServerNetwork::create(
        &network_params
            .consensus_server_url
            .host()
            .unwrap()
            .to_string(),
        network_params.consensus_server_url.port().unwrap(),
        wait_time,
        pub_keys[node_index as usize].clone(),
        pub_keys.clone(),
    );
    let da_channel = WebCommChannel::new(Arc::new(da_network));
    let quorum_channel = WebCommChannel::new(Arc::new(consensus_network));

    (
        init_hotshot(
            pub_keys,
            genesis_block,
            node_index as usize,
            priv_key,
            enc_key,
            da_channel,
            quorum_channel,
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
    use core::panic;
    use either::Either;
    use hotshot::{
        traits::implementations::{MasterMap, MemoryNetwork},
        types::{Event, EventType::Decide},
    };
    use hotshot_types::{data::LeafType, ExecutionType};
    use jf_primitives::signatures::SignatureScheme; // This trait provides the `key_gen` method.
    use rand::thread_rng;
    use std::{num::NonZeroUsize, sync::Arc, time::Duration};

    pub async fn init_hotshot_handles() -> Vec<HotShotHandle<SeqTypes, Node<network::Memory>>> {
        setup_logging();
        setup_backtrace();

        let genesis_block = Block::genesis();

        let num_nodes = 5;

        // Generate keys for the nodes.
        let nodes_key_pairs = (0..num_nodes)
            .map(|_| {
                (
                    SignatureSchemeType::key_gen(&(), &mut thread_rng()).unwrap(),
                    KeyPair::generate(&mut thread_rng()),
                )
            })
            .collect::<Vec<_>>();

        // Convert public keys to JfPubKey
        let nodes_pub_keys: Vec<PubKey> = nodes_key_pairs
            .iter()
            .map(|((_, ver_key), _)| JfPubKey::from_native(*ver_key))
            .collect::<Vec<_>>();

        let mut handles = vec![];

        let master_map = MasterMap::new();

        let config: HotShotConfig<_, _> = HotShotConfig {
            execution_type: ExecutionType::Continuous,
            total_nodes: num_nodes.try_into().unwrap(),
            min_transactions: 1,
            max_transactions: 10000.try_into().unwrap(),
            known_nodes: nodes_pub_keys.clone(),
            next_view_timeout: Duration::from_secs(60).as_millis() as u64,
            timeout_ratio: (10, 11),
            round_start_delay: Duration::from_millis(1).as_millis() as u64,
            start_delay: Duration::from_millis(1).as_millis() as u64,
            num_bootstrap: 1usize,
            propose_min_round_time: Duration::from_secs(1),
            propose_max_round_time: Duration::from_secs(1),
            election_config: Some(Membership::default_election_config(num_nodes as u64)),
            da_committee_size: NonZeroUsize::new(num_nodes).unwrap(),
        };

        // Create HotShot instances.
        for (node_id, ((sign_key, ver_key), enc_key)) in nodes_key_pairs.iter().enumerate() {
            let priv_key = (sign_key.clone(), *ver_key);
            let public_key = JfPubKey::from_native(*ver_key);

            let network = MemoryNetwork::new(
                public_key.clone(),
                Box::<NoMetrics>::default(),
                master_map.clone(),
                None,
            );

            let da_channel = MemoryCommChannel::new(Arc::new(network.clone()));
            let quorum_channel = MemoryCommChannel::new(Arc::new(network));

            let handle = init_hotshot(
                nodes_pub_keys.clone(),
                genesis_block.clone(),
                node_id,
                priv_key,
                enc_key.clone(),
                da_channel,
                quorum_channel,
                config.clone(),
            )
            .await;

            handles.push(handle);
        }
        handles
    }

    // Wait for decide event, make sure it matches submitted transaction
    pub async fn wait_for_decide_on_handle<N: network::Type>(
        mut handle: HotShotHandle<SeqTypes, Node<N>>,
        submitted_txn: Transaction,
    ) -> Result<(), ()> {
        let start_view = handle.get_current_view().await;

        // Keep getting events until we see a Decide event
        loop {
            let event = handle.next_event().await;
            tracing::info!("Received event from handle: {event:?}");

            match event {
                Ok(Event {
                    event:
                        Decide {
                            leaf_chain: leaf, ..
                        },
                    ..
                }) => {
                    if leaf.iter().any(|leaf| match leaf.get_deltas() {
                        Either::Left(block) => block
                            .transaction_nmt
                            .leaves()
                            .any(|txn| txn == &submitted_txn),
                        Either::Right(_) => false,
                    }) {
                        return Ok(());
                    }
                }
                Ok(Event {
                    view_number: vn, ..
                }) => {
                    // Don't wait too long; 20 views should more more than enough
                    if *vn > *start_view + 20u64 {
                        panic!();
                    }
                } // Keep waiting
                _ => panic!(), // Error
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        transaction::{ApplicationTransaction, Transaction},
        vm::{TestVm, Vm},
    };
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use core::panic;
    use either::Either;
    use hotshot::types::{Event, EventType::Decide};
    use hotshot_testing::test_builder::{TestBuilder, TestMetadata};
    use testing::{init_hotshot_handles, wait_for_decide_on_handle};

    // Submit transaction to given handle, return clone of transaction
    async fn submit_txn_to_handle<I: NodeImplementation<SeqTypes>>(
        handle: HotShotHandle<SeqTypes, I>,
        txn: &ApplicationTransaction,
    ) -> Transaction {
        let tx = Transaction::new(TestVm {}.id(), bincode::serialize(txn).unwrap());

        handle
            .submit_transaction(tx.clone())
            .await
            .expect("Failed to submit transaction");

        tx
    }

    // Run a hotshot test with our types
    #[async_std::test]
    async fn hotshot_test() {
        let builder = TestBuilder {
            metadata: TestMetadata {
                total_nodes: 10,
                start_nodes: 10,
                num_succeeds: 10,
                min_transactions: 1,
                failure_threshold: 3,
                da_committee_size: 4,
                ..Default::default()
            },
            ..Default::default()
        };

        builder
            .build::<SeqTypes, Node<network::Memory>>()
            .launch()
            .run_test()
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn test_skeleton_instantiation() -> Result<(), ()> {
        setup_logging();
        setup_backtrace();

        let mut handles = init_hotshot_handles().await;
        for handle in handles.iter() {
            handle.start().await;
        }

        let event = handles[0].next_event().await;
        tracing::info!("Received event from handle: {event:?}");

        // Should immediately get genesis block decide event
        match event {
            Ok(Event {
                event: Decide {
                    leaf_chain: leaf, ..
                },
                ..
            }) => {
                // Exactly one leaf, and it contains the genesis block
                assert_eq!(leaf.len(), 1);
                assert_eq!(leaf[0].deltas, Either::Left(Block::genesis()));
            }
            _ => panic!(),
        }

        // Submit target transaction to handle
        let txn = ApplicationTransaction::new(vec![1, 2, 3]);
        let submitted_txn = submit_txn_to_handle(handles[0].clone(), &txn).await;
        tracing::info!("Submitted transaction to handle: {txn:?}");

        wait_for_decide_on_handle(handles[0].clone(), submitted_txn).await
    }
}
