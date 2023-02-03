pub mod api;
mod block;
mod chain_variables;
mod state;
mod transaction;
mod vm;

use ark_bls12_381::Parameters;
use async_std::task::sleep;
use hotshot::traits::implementations::CentralizedServerNetwork;
use hotshot::traits::NetworkingImplementation;
use hotshot::types::SignatureKey;
use hotshot::{
    traits::{
        election::{
            static_committee::{StaticCommittee, StaticElectionConfig, StaticVoteToken},
            vrf::JfPubKey,
        },
        implementations::MemoryStorage,
        NodeImplementation,
    },
    types::HotShotHandle,
};
use hotshot::{HotShot, HotShotInitializer};
use hotshot_types::traits::metrics::Metrics;
use hotshot_types::{data::ViewNumber, traits::node_implementation::NodeTypes};
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

#[derive(Debug, Clone)]
pub struct Node<N>(std::marker::PhantomData<fn(&N)>);

impl<N: Clone + Debug + NetworkingImplementation<SeqTypes>> NodeImplementation<SeqTypes>
    for Node<N>
{
    type Storage = MemoryStorage<SeqTypes>;

    type Networking = N;

    type Election = StaticCommittee<SeqTypes>;
}

#[derive(
    Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct SeqTypes;

type Param381 = ark_bls12_381::Parameters;
pub type SignatureSchemeType = BLSSignatureScheme<Param381>;
pub type SignatureKeyType = JfPubKey<SignatureSchemeType>;

impl NodeTypes for SeqTypes {
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

async fn init_hotshot<
    I: NodeImplementation<
        SeqTypes,
        Storage = MemoryStorage<SeqTypes>,
        Election = StaticCommittee<SeqTypes>,
    >,
>(
    nodes_pub_keys: Vec<PubKey>,
    genesis_block: Block,
    node_id: usize,
    private_key: PrivKey,
    networking: I::Networking,
    config: HotShotConfig<PubKey, StaticElectionConfig>,
) -> HotShotHandle<SeqTypes, I> {
    // Create public and private keys for the node.
    let public_key = PubKey::from_private(&private_key);

    let storage = MemoryStorage::<SeqTypes>::new();
    let election = StaticCommittee::<SeqTypes>::new(nodes_pub_keys.clone());
    let initializer = HotShotInitializer::<SeqTypes>::from_genesis(genesis_block.clone()).unwrap();
    let metrics = NoMetrics::new();

    let handle: HotShotHandle<SeqTypes, I> = HotShot::init(
        public_key,
        private_key,
        node_id as u64,
        config,
        networking,
        storage,
        election,
        initializer,
        metrics,
    )
    .await
    .unwrap();

    handle
}

pub async fn init_node(
    addr: SocketAddr,
    genesis_block: Block,
    metrics: Box<dyn Metrics>,
) -> HotShotHandle<SeqTypes, Node<CentralizedServerNetwork<SeqTypes>>> {
    let (config, _, networking) =
        CentralizedServerNetwork::connect_with_server_config(metrics, addr).await;

    // Generate public keys and this node's private key.
    let (pub_keys, priv_keys): (Vec<_>, Vec<_>) = (0..config.config.total_nodes.get())
        .into_iter()
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

    init_hotshot(
        pub_keys,
        genesis_block,
        config.node_index as usize,
        sk,
        networking,
        config.config,
    )
    .await
}

#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::*;
    use core::panic;
    use hotshot::{
        traits::implementations::{MasterMap, MemoryNetwork},
        types::{Event, EventType::Decide},
    };
    use hotshot_types::ExecutionType;
    use jf_primitives::signatures::SignatureScheme; // This trait provides the `key_gen` method.
    use rand::thread_rng;
    use std::sync::Arc;
    use std::time::Duration;

    pub async fn init_hotshot_handles(
    ) -> Vec<HotShotHandle<SeqTypes, Node<MemoryNetwork<SeqTypes>>>> {
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

        let mut handles: Vec<HotShotHandle<SeqTypes, Node<MemoryNetwork<SeqTypes>>>> = vec![];

        let master_map: Arc<MasterMap<SeqTypes>> = MasterMap::new();

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

        // Create HotShot instances.
        for (node_id, (sign_key, ver_key)) in nodes_key_pairs.iter().enumerate() {
            // Create public and private keys for the node.
            let public_key = JfPubKey::from_native(ver_key.clone());

            let network = MemoryNetwork::<SeqTypes>::new(
                public_key.clone(),
                NoMetrics::new(),
                master_map.clone(),
                None,
            );

            let private_key = (sign_key.clone(), ver_key.clone());

            let handle = init_hotshot(
                nodes_pub_keys.clone(),
                genesis_block.clone(),
                node_id,
                private_key,
                network,
                config.clone(),
            )
            .await;

            handles.push(handle);
        }
        handles
    }

    // Wait for decide event, make sure it matches submitted transaction
    pub async fn wait_for_decide_on_handle(
        mut handle: HotShotHandle<SeqTypes, Node<MemoryNetwork<SeqTypes>>>,
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
                    if let Some(non_empty_leaf) = leaf
                        .iter()
                        .find(|leaf| !leaf.deltas.transactions.is_empty())
                    {
                        // When we find a non-empty Decide, check that it only contains the target transaction
                        assert!(non_empty_leaf.deltas.transactions == vec![submitted_txn]);
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
    use hotshot::{
        traits::implementations::MemoryNetwork,
        types::{Event, EventType::Decide},
    };
    use testing::{init_hotshot_handles, wait_for_decide_on_handle};

    // Submit transaction to given handle, return clone of transaction
    async fn submit_txn_to_handle(
        handle: HotShotHandle<SeqTypes, Node<MemoryNetwork<SeqTypes>>>,
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
                assert!(leaf.len() == 1 && leaf[0].deltas == Block::genesis(Default::default()))
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
