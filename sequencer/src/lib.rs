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
    types::HotShotHandle,
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

#[derive(Debug, Clone)]
struct Node;

// Name to be decided. Adapted from Espresso Validator.
#[async_trait]
trait Api {
    type Error: std::error::Error + Debug;
    async fn submit<V: Vm>(&mut self, txn: V::Transaction) -> Result<(), Self::Error>;
    // async fn next_event(&mut self) -> Result<ConsensusEvent, Self::Error>;
}

#[async_trait]
impl<N> Api for HotShotHandle<SeqTypes, N>
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

type SignatureKey = JfPubKey<BLSSignatureScheme<ark_bls12_381::Parameters>>;

impl NodeTypes for SeqTypes {
    type Time = ViewNumber;

    type BlockType = Block;

    type SignatureKey = SignatureKey;

    type VoteTokenType = StaticVoteToken<SignatureKey>;

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
