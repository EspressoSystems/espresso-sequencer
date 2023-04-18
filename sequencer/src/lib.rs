pub mod api;
mod block;
mod chain_variables;
pub mod hotshot_commitment;
mod state;
pub mod transaction;
mod vm;

use ark_bls12_381::Parameters;
use async_std::task::sleep;
use clap::{Parser, Subcommand};
use derivative::Derivative;
use hotshot::{
    traits::{
        election::{
            static_committee::{GeneralStaticCommittee, StaticElectionConfig, StaticVoteToken},
            vrf::JfPubKey,
        },
        implementations::{
            CentralizedCommChannel, CentralizedServerNetwork, MemoryCommChannel, MemoryStorage,
        },
        NodeImplementation,
    },
    types::{HotShotHandle, Message, SignatureKey},
    HotShot, HotShotInitializer,
};
use hotshot_commitment::HotShotContractOptions;
use hotshot_types::{
    data::{CommitmentProposal, DAProposal, SequencingLeaf, ViewNumber},
    traits::{
        election::{CommitteeExchange, ConsensusExchange, QuorumExchange},
        metrics::{Metrics, NoMetrics},
        network::CommunicationChannel,
        node_implementation::NodeType,
        state::SequencingConsensus,
    },
    vote::{DAVote, QuorumVote},
    HotShotConfig,
};

use jf_primitives::signatures::BLSSignatureScheme;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;
use transaction::SequencerTransaction;
use url::Url;

pub use block::Block;
pub use chain_variables::ChainVariables;
pub use state::State;
pub use transaction::{GenesisTransaction, Transaction};
pub use vm::{Vm, VmId, VmTransaction};

#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// Unique identifier for this instance of the sequencer network.
    #[clap(long, env = "ESPRESSO_SEQUENCER_CHAIN_ID", default_value = "0")]
    pub chain_id: u16,

    /// Port that the sequencer API will use.
    #[clap(long, env = "ESPRESSO_SEQUENCER_API_PORT")]
    pub port: u16,

    /// URL of the HotShot CDN.
    #[clap(short, long, env = "ESPRESSO_SEQUENCER_CDN_URL")]
    pub cdn_url: Url,

    /// Storage path for HotShot query service data.
    #[clap(long, env = "ESPRESSO_SEQUENCER_STORAGE_PATH")]
    pub storage_path: PathBuf,

    /// Create new query storage instead of opening existing one.
    #[clap(long, env = "ESPRESSO_SEQUENCER_RESET_STORE")]
    pub reset_store: bool,

    /// If specified, the sequencer will post hotshot commitments to the L1
    #[command(subcommand)]
    pub hotshot_contract_options: Option<HotShotContractCommand>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum HotShotContractCommand {
    HotShotContractOptions(HotShotContractOptions),
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
        >;
        type QuorumChannel<I: NodeImplementation<SeqTypes>>: CommunicationChannel<
            SeqTypes,
            Message<SeqTypes, I>,
            CommitmentProposal<SeqTypes, Leaf>,
            QuorumVote<SeqTypes, Leaf>,
            Membership,
        >;
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Centralized;

    impl Type for Centralized {
        type DAChannel<I: NodeImplementation<SeqTypes>> = CentralizedCommChannel<
            SeqTypes,
            I,
            DAProposal<SeqTypes>,
            DAVote<SeqTypes, Leaf>,
            Membership,
        >;
        type QuorumChannel<I: NodeImplementation<SeqTypes>> = CentralizedCommChannel<
            SeqTypes,
            I,
            CommitmentProposal<SeqTypes, Leaf>,
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
            CommitmentProposal<SeqTypes, Leaf>,
            QuorumVote<SeqTypes, Leaf>,
            Membership,
        >;
    }
}

/// The Sequencer node is generic over the hotshot CommChannel.
#[derive(Derivative)]
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

type Param381 = ark_bls12_381::Parameters;
pub type SignatureSchemeType = BLSSignatureScheme<Param381>;
pub type SignatureKeyType = JfPubKey<SignatureSchemeType>;
type ElectionConfig = StaticElectionConfig;

impl<N: network::Type> NodeImplementation<SeqTypes> for Node<N> {
    type Leaf = Leaf;
    type Storage = Storage;
    type QuorumExchange = QuorumExchange<
        SeqTypes,
        Leaf,
        CommitmentProposal<SeqTypes, Leaf>,
        Membership,
        N::QuorumChannel<Self>,
        Message<SeqTypes, Self>,
    >;
    type CommitteeExchange =
        CommitteeExchange<SeqTypes, Leaf, Membership, N::DAChannel<Self>, Message<SeqTypes, Self>>;
}

impl NodeType for SeqTypes {
    type ConsensusType = SequencingConsensus;
    type Time = ViewNumber;
    type BlockType = Block;
    type SignatureKey = SignatureKeyType;
    type VoteTokenType = StaticVoteToken<SignatureKeyType>;
    type Transaction = SequencerTransaction;
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
}

type PubKey = JfPubKey<BLSSignatureScheme<Parameters>>;
type PrivKey = <PubKey as SignatureKey>::PrivateKey;

async fn init_hotshot<N: network::Type>(
    nodes_pub_keys: Vec<PubKey>,
    genesis_block: Block,
    node_id: usize,
    private_key: PrivKey,
    da_channel: N::DAChannel<Node<N>>,
    quorum_channel: N::QuorumChannel<Node<N>>,
    config: HotShotConfig<PubKey, ElectionConfig>,
) -> HotShotHandle<SeqTypes, Node<N>> {
    // Create public and private keys for the node.
    let public_key = PubKey::from_private(&private_key);

    let storage = Storage::empty();
    let initializer = HotShotInitializer::<SeqTypes, SequencingLeaf<SeqTypes>>::from_genesis(
        genesis_block.clone(),
    )
    .unwrap();
    let metrics = Box::<NoMetrics>::default();

    let quorum_exchange = QuorumExchange::create(
        nodes_pub_keys.clone(),
        ElectionConfig {},
        quorum_channel,
        public_key.clone(),
        private_key.clone(),
    );

    let committee_exchange = CommitteeExchange::create(
        nodes_pub_keys,
        ElectionConfig {},
        da_channel,
        public_key.clone(),
        private_key.clone(),
    );

    HotShot::init(
        public_key,
        private_key,
        node_id as u64,
        config,
        storage,
        quorum_exchange,
        committee_exchange,
        initializer,
        metrics,
    )
    .await
    .unwrap()
}

pub async fn init_node(
    addr: SocketAddr,
    genesis_block: Block,
    metrics: Box<dyn Metrics>,
) -> (HotShotHandle<SeqTypes, Node<network::Centralized>>, u64) {
    let (config, _, networking) =
        CentralizedServerNetwork::connect_with_server_config(metrics, addr).await;
    let da_channel = CentralizedCommChannel::new(networking.clone());
    let quorum_channel = CentralizedCommChannel::new(networking.clone());

    // Generate public keys and this node's private key.
    let (pub_keys, priv_keys): (Vec<_>, Vec<_>) = (0..config.config.total_nodes.get())
        .map(|i| SignatureKeyType::generated_from_seed_indexed(config.seed, i as u64))
        .unzip();
    let sk = priv_keys[config.node_index as usize].clone();

    // Wait for other nodes to connect.
    while !networking.run_ready() {
        let connected = networking.get_connected_client_count().await;
        tracing::info!(
            "waiting for start signal ({}/{} connected)",
            connected,
            config.config.total_nodes,
        );
        sleep(Duration::from_secs(1)).await;
    }

    (
        init_hotshot(
            pub_keys,
            genesis_block,
            config.node_index as usize,
            sk,
            da_channel,
            quorum_channel,
            config.config,
        )
        .await,
        config.node_index,
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
    use std::time::Duration;

    pub async fn init_hotshot_handles() -> Vec<HotShotHandle<SeqTypes, Node<network::Memory>>> {
        setup_logging();
        setup_backtrace();

        let genesis_block = Block::genesis(Default::default());

        let num_nodes = 5;

        // Generate keys for the nodes.
        let nodes_key_pairs = (0..num_nodes)
            .map(|_| SignatureSchemeType::key_gen(&(), &mut thread_rng()).unwrap())
            .collect::<Vec<_>>();

        // Convert public keys to JfPubKey
        let nodes_pub_keys: Vec<PubKey> = nodes_key_pairs
            .iter()
            .map(|(_sign_key, ver_key)| JfPubKey::from_native(ver_key.clone()))
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
            propose_max_round_time: Duration::from_secs(30),
            election_config: Some(ElectionConfig {}),
        };

        // Create HotShot instances.
        for (node_id, (sign_key, ver_key)) in nodes_key_pairs.iter().enumerate() {
            let private_key = (sign_key.clone(), ver_key.clone());
            let public_key = JfPubKey::from_native(ver_key.clone());

            let network = MemoryNetwork::new(
                public_key.clone(),
                Box::<NoMetrics>::default(),
                master_map.clone(),
                None,
            );

            let da_channel = MemoryCommChannel::new(network.clone());
            let quorum_channel = MemoryCommChannel::new(network);

            let handle = init_hotshot(
                nodes_pub_keys.clone(),
                genesis_block.clone(),
                node_id,
                private_key,
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
        submitted_txn: SequencerTransaction,
    ) -> Result<(), ()> {
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
                    if let Some(transactions) =
                        leaf.iter().find_map(|leaf| match leaf.get_deltas() {
                            Either::Left(block) => {
                                if !block.transactions.is_empty() {
                                    Some(block.transactions)
                                } else {
                                    None
                                }
                            }
                            Either::Right(_) => None,
                        })
                    {
                        // When we find a non-empty Decide, check that it only contains the target transaction
                        assert_eq!(transactions, vec![submitted_txn]);
                        return Ok(());
                    } else {
                        // Empty Decide event
                        continue;
                    }
                }
                Ok(Event {
                    view_number: vn, ..
                }) => {
                    // Don't wait too long; 20 views should more more than enough
                    if *vn > 20u64 {
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
    use core::panic;
    use either::Either;
    use hotshot::types::{Event, EventType::Decide};
    use hotshot_testing::test_description::GeneralTestDescriptionBuilder;
    use testing::{init_hotshot_handles, wait_for_decide_on_handle};

    // Submit transaction to given handle, return clone of transaction
    async fn submit_txn_to_handle<I: NodeImplementation<SeqTypes>>(
        handle: HotShotHandle<SeqTypes, I>,
        txn: &ApplicationTransaction,
    ) -> SequencerTransaction {
        let tx = SequencerTransaction::Wrapped(Transaction::new(
            TestVm::default().id(),
            bincode::serialize(txn).unwrap(),
        ));

        handle
            .submit_transaction(tx.clone())
            .await
            .expect("Failed to submit transaction");

        tx
    }

    // Run a hotshot test with our types
    #[async_std::test]
    async fn general_hotshot_test() {
        let builder = GeneralTestDescriptionBuilder {
            num_succeeds: 3,
            ..Default::default()
        };
        builder
            .build::<SeqTypes, Node<network::Memory>>()
            .execute()
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn test_skeleton_instantiation() -> Result<(), ()> {
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
                assert_eq!(
                    leaf[0].deltas,
                    Either::Left(Block::genesis(Default::default()))
                );
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
