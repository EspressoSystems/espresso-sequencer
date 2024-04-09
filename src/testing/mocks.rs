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

use crate::merklized_state::{MerklizedState, UpdateStateStorage};
use crate::{
    availability::{QueryableHeader, QueryablePayload},
    types::HeightIndexed,
};
use async_trait::async_trait;
use hotshot::traits::{
    election::static_committee::{GeneralStaticCommittee, StaticElectionConfig},
    implementations::MemoryNetwork,
    NodeImplementation, ValidatedState,
};
use hotshot_example_types::{
    block_types::{TestBlockHeader, TestBlockPayload, TestTransaction},
    state_types::{TestInstanceState, TestValidatedState},
    storage_types::TestStorage,
};
use hotshot_types::data::Leaf;
use hotshot_types::{
    data::{QuorumProposal, ViewNumber},
    message::Message,
    signature_key::BLSPubKey,
    traits::node_implementation::NodeType,
};

use jf_primitives::merkle_tree::{
    prelude::{Sha3Digest, Sha3Node},
    universal_merkle_tree::UniversalMerkleTree,
};
use serde::{Deserialize, Serialize};
use std::{ops::Range, sync::Arc};

pub type MockHeader = TestBlockHeader;
pub type MockPayload = TestBlockPayload;
pub type MockTransaction = TestTransaction;

pub fn mock_transaction(payload: Vec<u8>) -> MockTransaction {
    TestTransaction(payload)
}

impl QueryableHeader<MockTypes> for MockHeader {
    fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

impl HeightIndexed for MockHeader {
    fn height(&self) -> u64 {
        self.block_number
    }
}

impl QueryablePayload for MockPayload {
    type TransactionIndex = usize;
    type Iter<'a> = Range<usize>;
    type InclusionProof = ();

    fn len(&self, _meta: &Self::Metadata) -> usize {
        self.transactions.len()
    }

    fn iter(&self, meta: &Self::Metadata) -> Self::Iter<'_> {
        0..self.len(meta)
    }

    fn transaction_with_proof(
        &self,
        _meta: &Self::Metadata,
        index: &Self::TransactionIndex,
    ) -> Option<(Self::Transaction, Self::InclusionProof)> {
        self.transactions.get(*index).cloned().map(|tx| (tx, ()))
    }
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct MockTypes;

impl NodeType for MockTypes {
    type Time = ViewNumber;
    type BlockHeader = MockHeader;
    type BlockPayload = MockPayload;
    type SignatureKey = BLSPubKey;
    type Transaction = MockTransaction;
    type ElectionConfigType = StaticElectionConfig;
    type InstanceState = TestInstanceState;
    type ValidatedState = TestValidatedState;
    type Membership = GeneralStaticCommittee<Self, BLSPubKey>;
}

#[async_trait]
impl<D> UpdateStateStorage<MockTypes, D> for TestValidatedState {
    async fn update_storage(
        &self,
        _storage: &mut D,
        _leaf: &Leaf<MockTypes>,
        _delta: Arc<<<MockTypes as NodeType>::ValidatedState as ValidatedState<MockTypes>>::Delta>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

pub type MockMembership = GeneralStaticCommittee<MockTypes, <MockTypes as NodeType>::SignatureKey>;
pub type MockQuorumProposal = QuorumProposal<MockTypes>;
pub type MockNetwork = MemoryNetwork<Message<MockTypes>, BLSPubKey>;

pub type MockStorage = TestStorage<MockTypes>;

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct MockNodeImpl;

impl NodeImplementation<MockTypes> for MockNodeImpl {
    type QuorumNetwork = MockNetwork;
    type CommitteeNetwork = MockNetwork;
    type Storage = MockStorage;
}

pub type MockMerkleTree = UniversalMerkleTree<usize, Sha3Digest, usize, 8, Sha3Node>;

impl MerklizedState<MockTypes, 8> for MockMerkleTree {
    type Key = usize;
    type Entry = usize;
    type T = Sha3Node;
    type Commit = Self::Commitment;
    type Digest = Sha3Digest;

    fn state_type() -> &'static str {
        "test_tree"
    }

    fn header_state_commitment_field() -> &'static str {
        "test_merkle_tree_root"
    }

    fn tree_height() -> usize {
        3
    }
}
