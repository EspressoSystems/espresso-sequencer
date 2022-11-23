use commit::{Commitment, Committable};
use hotshot::{
    traits::{
        election::static_committee::{StaticCommittee, StaticElectionConfig, StaticVoteToken},
        implementations::{CentralizedServerNetwork, MemoryStorage},
        Block as HotShotBlock, NodeImplementation, State as HotShotState,
    },
    types::ed25519::Ed25519Pub,
};
use hotshot_types::{
    data::ViewNumber,
    traits::{block_contents::Transaction as HotShotTransaction, node_implementation::NodeTypes},
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

#[derive(Debug, Clone)]
struct Node;

impl NodeImplementation<SeqTypes> for Node {
    type Storage = MemoryStorage<SeqTypes>;

    type Networking = CentralizedServerNetwork<SeqTypes>;

    type Election = StaticCommittee; // ???
}

#[derive(
    Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize,
)]
struct SeqTypes;

type SignatureKey = Ed25519Pub;

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
        tx: &Self::Transaction,
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
struct Transaction;

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

    fn validate_block(&self, block: &Self::BlockType, view_number: &Self::Time) -> bool {
        todo!()
    }

    fn append(
        &self,
        block: &Self::BlockType,
        view_number: &Self::Time,
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
