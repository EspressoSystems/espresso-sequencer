mod block;
mod chain_variables;
mod state;
mod transaction;
mod vm;

use crate::{block::Block, state::State};
use ark_bls12_381::Parameters;
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
use hotshot_types::{data::ViewNumber, traits::node_implementation::NodeTypes};
use hotshot_types::{traits::metrics::NoMetrics, HotShotConfig};

use jf_primitives::signatures::BLSSignatureScheme;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::fmt::Debug;
use std::net::SocketAddr;
use transaction::SequencerTransaction;

#[derive(Debug, Clone)]
struct Node<N>(std::marker::PhantomData<fn(&N)>);

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
struct SeqTypes;

type Param381 = ark_bls12_381::Parameters;
type SignatureSchemeType = BLSSignatureScheme<Param381>;
type SignatureKeyType = JfPubKey<SignatureSchemeType>;

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

#[allow(dead_code)]
async fn init_node(
    addr: SocketAddr,
    nodes_pub_keys: Vec<PubKey>,
    genesis_block: Block,
    private_key: PrivKey,
) -> HotShotHandle<SeqTypes, Node<CentralizedServerNetwork<SeqTypes>>> {
    let (config, _, networking) =
        CentralizedServerNetwork::connect_with_server_config(NoMetrics::new(), addr).await;

    init_hotshot(
        nodes_pub_keys,
        genesis_block,
        config.node_index.try_into().unwrap(),
        private_key,
        networking,
        config.config,
    )
    .await
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use crate::{
        transaction::{ApplicationTransaction, Transaction},
        vm::{TestVm, Vm},
    };

    use super::*;
    use hotshot::{
        traits::implementations::{MasterMap, MemoryNetwork},
        types::EventType,
    };

    use hotshot_types::ExecutionType;

    use std::time::Duration;

    use jf_primitives::signatures::SignatureScheme; // This trait provides the `key_gen` method.
    use rand::thread_rng;

    #[async_std::test]
    async fn test_skeleton_instantiation() -> Result<(), ()> {
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
            min_transactions: 0,
            max_transactions: 2usize.try_into().unwrap(),
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

        for handle in handles.iter() {
            handle.start().await;
        }
        println!("Started");

        let event = handles[0].next_event().await;
        println!("Event: {:?}", event);

        let txn = ApplicationTransaction::new(vec![1, 2, 3]);

        handles[0]
            .submit_transaction(SequencerTransaction::Wrapped(Transaction::new(
                TestVm::id(),
                bincode::serialize(&txn).unwrap(),
            )))
            .await
            .expect("Failed to submit transaction");

        println!("Submitted: {:?}", txn);

        let event = handles[0].next_event().await.unwrap();
        println!("Event: {:?}", event);

        assert_eq!(*event.view_number, 1u64);
        assert!(matches!(event.event, EventType::ViewFinished { .. }));

        Ok(())
    }
}
