// pub mod api;
mod block;
mod chain_variables;
mod state;
mod transaction;
mod vm;

use ark_bls12_381::Parameters;
use async_std::task::sleep;
use hotshot::traits::election::static_committee::GeneralStaticCommittee;
use hotshot::traits::implementations::{
    CentralizedCommChannel, CentralizedServerNetwork, MemoryCommChannel,
};
use hotshot::types::{Message, SignatureKey};
use hotshot::{
    traits::{
        election::{
            static_committee::{StaticElectionConfig, StaticVoteToken},
            vrf::JfPubKey,
        },
        implementations::MemoryStorage,
        NodeImplementation,
    },
    types::HotShotHandle,
};
use hotshot::{HotShot, HotShotInitializer};
use hotshot_types::data::{CommitmentProposal, DAProposal, SequencingLeaf};
use hotshot_types::traits::election::{CommitteeExchange, ConsensusExchange, QuorumExchange};
use hotshot_types::traits::metrics::Metrics;
use hotshot_types::traits::state::SequencingConsensus;
use hotshot_types::vote::{DAVote, QuorumVote};
use hotshot_types::{data::ViewNumber, traits::node_implementation::NodeType};
use hotshot_types::{traits::metrics::NoMetrics, HotShotConfig};
use jf_primitives::signatures::BLSSignatureScheme;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::time::Duration;
use transaction::SequencerTransaction;

pub use block::Block;
pub use chain_variables::ChainVariables;
pub use state::State;
pub use transaction::{GenesisTransaction, Transaction};
pub use vm::{Vm, VmId, VmTransaction};

/// The Sequencer node is generic over the hotshot CommChannel.
#[derive(Debug, Clone)]
pub struct Node;

#[derive(
    Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct SeqTypes;

type Leaf = SequencingLeaf<SeqTypes>;
type Membership = GeneralStaticCommittee<SeqTypes, Leaf, SignatureKeyType>;
type Storage = MemoryStorage<SeqTypes, Leaf>;

type DAChannel<I> =
    MemoryCommChannel<SeqTypes, I, DAProposal<SeqTypes>, DAVote<SeqTypes, Leaf>, Membership>;
type QuorumChannel<I> = MemoryCommChannel<
    SeqTypes,
    I,
    CommitmentProposal<SeqTypes, Leaf>,
    QuorumVote<SeqTypes, Leaf>,
    Membership,
>;

type Param381 = ark_bls12_381::Parameters;
pub type SignatureSchemeType = BLSSignatureScheme<Param381>;
pub type SignatureKeyType = JfPubKey<SignatureSchemeType>;

type QuorumExchangeType<I> = QuorumExchange<
    SeqTypes,
    Leaf,
    CommitmentProposal<SeqTypes, Leaf>,
    GeneralStaticCommittee<SeqTypes, Leaf, SignatureKeyType>,
    QuorumChannel<I>,
    Message<SeqTypes, I>,
>;

type CommitteeExchangeType<I> = CommitteeExchange<
    SeqTypes,
    Leaf,
    GeneralStaticCommittee<SeqTypes, Leaf, SignatureKeyType>,
    DAChannel<I>,
    Message<SeqTypes, I>,
>;

impl NodeImplementation<SeqTypes> for Node {
    type Leaf = Leaf;
    type Storage = Storage;
    type QuorumExchange = QuorumExchangeType<Self>;
    type CommitteeExchange = CommitteeExchangeType<Self>;
}

impl NodeType for SeqTypes {
    type ConsensusType = SequencingConsensus;
    type Time = ViewNumber;
    type BlockType = Block;
    type SignatureKey = SignatureKeyType;
    type VoteTokenType = StaticVoteToken<SignatureKeyType>;
    type Transaction = SequencerTransaction;
    type ElectionConfigType = StaticElectionConfig;
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

async fn init_hotshot(
    nodes_pub_keys: Vec<PubKey>,
    genesis_block: Block,
    node_id: usize,
    private_key: PrivKey,
    da_channel: DAChannel<Node>,
    quorum_channel: QuorumChannel<Node>,
    config: HotShotConfig<PubKey, StaticElectionConfig>,
) -> HotShotHandle<SeqTypes, Node> {
    // Create public and private keys for the node.
    let public_key = PubKey::from_private(&private_key);

    let storage = Storage::new();
    let initializer = HotShotInitializer::<SeqTypes, SequencingLeaf<SeqTypes>>::from_genesis(
        genesis_block.clone(),
    )
    .unwrap();
    let metrics = NoMetrics::new();

    let quorum_exchange = QuorumExchangeType::create(
        nodes_pub_keys.clone(),
        StaticElectionConfig {},
        quorum_channel,
        public_key.clone(),
        private_key.clone(),
    );

    let committee_exchange = CommitteeExchangeType::create(
        nodes_pub_keys,
        StaticElectionConfig {},
        da_channel,
        public_key.clone(),
        private_key.clone(),
    );

    let handle: HotShotHandle<SeqTypes, Node> = HotShot::init(
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
    .unwrap();

    handle
}

// pub async fn init_node(
//     addr: SocketAddr,
//     genesis_block: Block,
//     metrics: Box<dyn Metrics>,
// ) -> HotShotHandle<SeqTypes, Node> {
//     let (config, _, networking) =
//         CentralizedServerNetwork::connect_with_server_config(metrics, addr).await;
//     let da_channel = CentralizedCommChannel::new(networking.clone());
//     let quorum_channel = CentralizedCommChannel::new(networking.clone());

//     // Generate public keys and this node's private key.
//     let (pub_keys, priv_keys): (Vec<_>, Vec<_>) = (0..config.config.total_nodes.get())
//         .map(|i| SignatureKeyType::generated_from_seed_indexed(config.seed, i as u64))
//         .unzip();
//     let sk = priv_keys[config.node_index as usize].clone();

//     // Wait for other nodes to connect.
//     while !networking.run_ready() {
//         let connected = networking.get_connected_client_count().await;
//         tracing::info!(
//             "waiting for start signal ({}/{} connected)",
//             connected,
//             config.config.total_nodes,
//         );
//         sleep(Duration::from_secs(1)).await;
//     }

//     init_hotshot(
//         pub_keys,
//         genesis_block,
//         config.node_index as usize,
//         sk,
//         da_channel,
//         quorum_channel,
//         config.config,
//     )
//     .await
// }

#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::*;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use core::panic;
    use either::Either;
    use hotshot::{
        traits::implementations::MasterMap,
        types::{Event, EventType::Decide},
    };
    use hotshot_types::{traits::network::TestableNetworkingImplementation, ExecutionType};
    use jf_primitives::signatures::SignatureScheme; // This trait provides the `key_gen` method.
    use rand::thread_rng;
    use std::time::Duration;

    pub async fn init_hotshot_handles() -> Vec<HotShotHandle<SeqTypes, Node>> {
        setup_logging();
        setup_backtrace();

        let genesis_block = Block::genesis(Default::default());

        let num_nodes = 4;

        // Generate keys for the nodes.
        let nodes_key_pairs = (0..num_nodes)
            .map(|_| SignatureSchemeType::key_gen(&(), &mut thread_rng()).unwrap())
            .collect::<Vec<_>>();

        // Convert public keys to JfPubKey
        let nodes_pub_keys: Vec<PubKey> = nodes_key_pairs
            .iter()
            .map(|(_sign_key, ver_key)| JfPubKey::from_native(ver_key.clone()))
            .collect::<Vec<_>>();

        let mut handles: Vec<HotShotHandle<SeqTypes, Node>> = vec![];

        // let master_map = MasterMap::<Message<SeqTypes, Node>, SignatureKeyType>::new();

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
            election_config: Some(StaticElectionConfig {}),
        };

        let da_channel_gen = MemoryCommChannel::generator(4, 4, 1);
        let quorum_channel_gen = MemoryCommChannel::generator(4, 4, 2);

        // Create HotShot instances.
        for (node_id, (sign_key, ver_key)) in nodes_key_pairs.iter().enumerate() {
            // TODO: How to get the private key?
            let private_key = (sign_key.clone(), ver_key.clone());

            let handle = init_hotshot(
                nodes_pub_keys.clone(),
                genesis_block.clone(),
                node_id,
                private_key,
                da_channel_gen(node_id as u64),
                quorum_channel_gen(node_id as u64),
                config.clone(),
            )
            .await;

            handles.push(handle);
        }
        handles
    }

    // Wait for decide event, make sure it matches submitted transaction
    pub async fn wait_for_decide_on_handle(
        mut handle: HotShotHandle<SeqTypes, Node>,
        submitted_txn: SequencerTransaction,
    ) -> Result<(), ()> {
        // Keep getting events until we see a Decide event
        loop {
            let event = handle.next_event().await;
            println!("Event: {event:?}\n");

            match event {
                Ok(Event {
                    event:
                        Decide {
                            leaf_chain: leaf, ..
                        },
                    ..
                }) => {
                    if let Some(transactions) = leaf.iter().find_map(|leaf| match &leaf.deltas {
                        Either::Left(block) => {
                            if !block.transactions.is_empty() {
                                Some(block.transactions.clone())
                            } else {
                                None
                            }
                        }
                        Either::Right(_) => None,
                    }) {
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
    use hotshot::{
        traits::implementations::MemoryNetwork,
        types::{Event, EventType::Decide},
    };
    use testing::{init_hotshot_handles, wait_for_decide_on_handle};

    // Submit transaction to given handle, return clone of transaction
    async fn submit_txn_to_handle(
        handle: HotShotHandle<SeqTypes, Node>,
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

    #[async_std::test]
    async fn test_skeleton_instantiation() -> Result<(), ()> {
        let mut handles = init_hotshot_handles().await;
        for handle in handles.iter() {
            handle.start().await;
        }

        let event = handles[0].next_event().await;
        println!("Event: {event:?}\n");

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
        println!("Submitted: {txn:?}");

        wait_for_decide_on_handle(handles[0].clone(), submitted_txn).await
    }
}
