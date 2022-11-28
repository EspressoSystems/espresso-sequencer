use async_trait::async_trait;
use commit::{Commitment, Committable};
use hotshot::{
    traits::{
        election::{
            static_committee::{StaticCommittee, StaticElectionConfig, StaticVoteToken},
            vrf::JfPubKey,
        },
        implementations::{CentralizedServerNetwork, MemoryStorage},
        Block as HotShotBlock, NodeImplementation, State as HotShotState,
    },
    types::{EventType, HotShotHandle},
    HotShotError,
};
use hotshot_types::{
    data::ViewNumber,
    traits::{block_contents::Transaction as HotShotTransaction, node_implementation::NodeTypes},
};
use jf_primitives::signatures::BLSSignatureScheme;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::Snafu;
use std::fmt::Debug;

type ConsensusEvent = EventType<SeqTypes>;

#[derive(Debug, Clone)]
struct Node;

// Name to be decided. Adapted from Espresso Validator.
#[async_trait]
trait ValidatorDataSource {
    type Error: std::error::Error + Debug;
    async fn submit<V: Vm>(&mut self, txn: V::Transaction) -> Result<(), Self::Error>;
    async fn next_event(&mut self) -> Result<ConsensusEvent, Self::Error>;
}

#[async_trait]
impl<N> ValidatorDataSource for HotShotHandle<SeqTypes, N>
where
    N: NodeImplementation<SeqTypes>,
{
    type Error = HotShotError<SeqTypes>;

    async fn submit<V: Vm>(&mut self, txn: V::Transaction) -> Result<(), Self::Error> {
        self.submit_transaction(Transaction {
            vm: V::id(),
            payload: bincode::serialize(&txn).unwrap(),
        })
        .await
    }

    async fn next_event(&mut self) -> Result<ConsensusEvent, Self::Error> {
        self.next_event().await.map(|e| e.event)
    }
}

impl NodeImplementation<SeqTypes> for Node {
    type Storage = MemoryStorage<SeqTypes>;

    type Networking = CentralizedServerNetwork<SeqTypes>;

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

    type Transaction = Transaction;

    type ElectionConfigType = StaticElectionConfig;

    type StateType = State;
}

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
struct Block;

impl HotShotBlock for Block {
    type Error = Error;

    type Transaction = Transaction;

    fn add_transaction_raw(
        &self,
        _tx: &Self::Transaction,
    ) -> std::result::Result<Self, Self::Error> {
        todo!()
    }

    fn contained_transactions(&self) -> std::collections::HashSet<Commitment<Self::Transaction>> {
        todo!()
    }
}

impl Committable for Block {
    fn commit(&self) -> Commitment<Self> {
        todo!()
    }
}

#[derive(Clone, Debug, Snafu, Deserialize, Serialize)]
pub enum Error {
    Error,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
struct VmId(u64);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
struct Transaction {
    vm: VmId,
    payload: Vec<u8>,
}

trait Vm {
    type Transaction: DeserializeOwned + Serialize + Sync + Send;
    fn id() -> VmId;
}

impl HotShotTransaction for Transaction {}

impl Committable for Transaction {
    fn commit(&self) -> Commitment<Self> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Hash, PartialEq, Eq)]
struct State;

impl HotShotState for State {
    type Error = Error;

    type BlockType = Block;

    type Time = ViewNumber;

    fn next_block(&self) -> Self::BlockType {
        todo!()
    }

    fn validate_block(&self, _block: &Self::BlockType, _view_number: &Self::Time) -> bool {
        todo!()
    }

    fn append(
        &self,
        _block: &Self::BlockType,
        _view_number: &Self::Time,
    ) -> Result<Self, Self::Error> {
        todo!()
    }

    fn on_commit(&self) {
        todo!()
    }
}

impl Committable for State {
    fn commit(&self) -> Commitment<Self> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hotshot::{HotShot, HotShotInitializer};
    use hotshot_types::{traits::metrics::NoMetrics, ExecutionType, HotShotConfig};
    use jf_primitives::signatures::SignatureScheme; // This trait provides the `key_gen` method.
    use rand::thread_rng;
    use std::{net::SocketAddr, time::Duration};

    #[ignore] // Currently hangs, probably waiting for centralized server that isn't running.
    #[async_std::test]
    async fn test_skeleton_instatiation() -> Result<(), ()> {
        let num_nodes = 3usize;

        // Generate keys for the nodes.
        let nodes_key_pairs = (0..num_nodes)
            .map(|_| SignatureSchemeType::key_gen(&(), &mut thread_rng()).unwrap())
            .collect::<Vec<_>>();

        // Convert public keys to JfPubKey
        let nodes_pub_keys = nodes_key_pairs
            .iter()
            .map(|(_sign_key, ver_key)| JfPubKey::from_native(ver_key.clone()))
            .collect::<Vec<_>>();

        // Create public and private keys for the first node.
        let (sign_key, ver_key) = nodes_key_pairs[0].clone();
        let public_key = JfPubKey::from_native(ver_key.clone());

        let node_id = 0u64;

        let config: HotShotConfig<_, _> = HotShotConfig {
            execution_type: ExecutionType::Incremental,
            total_nodes: num_nodes.try_into().unwrap(),
            min_transactions: 0,
            max_transactions: 5usize.try_into().unwrap(),
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

        let centralized_server_address: SocketAddr = "127.0.0.1:11223".parse().unwrap();
        // TODO: run the centralized server
        let (_network_config, _run, network) =
            CentralizedServerNetwork::<SeqTypes>::connect_with_server_config(
                NoMetrics::new(),
                centralized_server_address,
            )
            .await;
        let storage = MemoryStorage::<SeqTypes>::new();
        let election = StaticCommittee::<SeqTypes>::new(nodes_pub_keys);
        let genesis_block = Block {};
        let initializer = HotShotInitializer::<SeqTypes>::from_genesis(genesis_block).unwrap();
        let metrics = NoMetrics::new();

        let _hotshot: HotShotHandle<SeqTypes, Node> = HotShot::init(
            public_key,
            (sign_key, ver_key),
            node_id,
            config,
            network,
            storage,
            election,
            initializer,
            metrics,
        )
        .await
        .unwrap();

        Ok(())
    }
}
