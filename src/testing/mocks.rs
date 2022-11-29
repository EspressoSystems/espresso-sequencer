// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the HotShot Query Service library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

use ark_serialize::*;
use commit::{Commitment, Committable, RawCommitmentBuilder};
use derive_more::{Index, IndexMut};
use hotshot::traits::{
    election::static_committee::{StaticCommittee, StaticElectionConfig, StaticVoteToken},
    implementations::{CentralizedServerNetwork, MemoryStorage},
    Block, NodeImplementation,
};
use hotshot_types::{
    data::ViewNumber,
    traits::{
        block_contents::Transaction, node_implementation::NodeTypes,
        signature_key::ed25519::Ed25519Pub, State,
    },
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::collections::HashSet;

#[derive(Clone, Debug, Snafu)]
pub enum MockError {
    InvalidBlockParent,
}

#[derive(
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    CanonicalSerialize,
    CanonicalDeserialize,
    Clone,
    Debug,
)]
pub struct MockTransaction {
    pub nonce: u64,
}

impl Committable for MockTransaction {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("MockTransaction")
            .u64_field("nonce", self.nonce)
            .finalize()
    }

    fn tag() -> String {
        "MOCKTXN".to_string()
    }
}

impl Transaction for MockTransaction {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct MockState {
    pub last_block: Commitment<MockBlock>,
}

impl Default for MockState {
    fn default() -> Self {
        Self {
            last_block: MockBlock::genesis().parent,
        }
    }
}

impl Committable for MockState {
    fn commit(&self) -> Commitment<Self> {
        RawCommitmentBuilder::new("MockState").finalize()
    }

    fn tag() -> String {
        "MOCKSTATE".to_string()
    }
}

impl State for MockState {
    type Error = MockError;
    type BlockType = MockBlock;
    type Time = ViewNumber;

    fn next_block(&self) -> Self::BlockType {
        MockBlock::new(self.last_block)
    }

    fn validate_block(&self, block: &Self::BlockType, _view_number: &Self::Time) -> bool {
        block.parent == self.last_block
    }

    fn append(
        &self,
        block: &Self::BlockType,
        _view_number: &Self::Time,
    ) -> Result<Self, Self::Error> {
        if block.parent == self.last_block {
            Ok(Self {
                last_block: block.commit(),
            })
        } else {
            Err(MockError::InvalidBlockParent)
        }
    }

    fn on_commit(&self) {}
}

#[derive(
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    CanonicalSerialize,
    CanonicalDeserialize,
    Clone,
    Debug,
    Index,
    IndexMut,
)]
pub struct MockBlock {
    pub parent: Commitment<MockBlock>,
    #[index]
    #[index_mut]
    pub transactions: Vec<MockTransaction>,
}

impl Committable for MockBlock {
    fn commit(&self) -> Commitment<Self> {
        RawCommitmentBuilder::new("MockBlock")
            .field("parent", self.parent)
            .array_field(
                "transactions",
                &self
                    .transactions
                    .iter()
                    .map(|txn| txn.commit())
                    .collect::<Vec<_>>(),
            )
            .finalize()
    }

    fn tag() -> String {
        "MOCKBLOCK".to_string()
    }
}

impl MockBlock {
    pub fn new(parent: Commitment<MockBlock>) -> Self {
        Self {
            parent,
            transactions: Default::default(),
        }
    }

    pub fn genesis() -> Self {
        Self::new(RawCommitmentBuilder::new("GenesisMockBlock").finalize())
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &MockTransaction> {
        self.transactions.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut MockTransaction> {
        self.transactions.iter_mut()
    }
}

impl IntoIterator for MockBlock {
    type Item = MockTransaction;
    type IntoIter = std::vec::IntoIter<MockTransaction>;

    fn into_iter(self) -> Self::IntoIter {
        self.transactions.into_iter()
    }
}

impl Block for MockBlock {
    type Transaction = MockTransaction;
    type Error = MockError;

    fn add_transaction_raw(&self, tx: &Self::Transaction) -> Result<Self, Self::Error> {
        let mut block = self.clone();
        block.transactions.push(tx.clone());
        Ok(block)
    }
    fn contained_transactions(&self) -> HashSet<Commitment<Self::Transaction>> {
        self.transactions.iter().map(|tx| tx.commit()).collect()
    }
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct MockTypes;

impl NodeTypes for MockTypes {
    type Time = ViewNumber;
    type BlockType = MockBlock;
    type SignatureKey = Ed25519Pub;
    type VoteTokenType = StaticVoteToken;
    type Transaction = MockTransaction;
    type ElectionConfigType = StaticElectionConfig;
    type StateType = MockState;
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct MockNodeImpl;

impl NodeImplementation<MockTypes> for MockNodeImpl {
    type Storage = MemoryStorage<MockTypes>;
    type Networking = CentralizedServerNetwork<MockTypes>;
    type Election = StaticCommittee<MockTypes>;
}
