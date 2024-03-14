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
    implementations::{MemoryNetwork, MemoryStorage},
    NodeImplementation, ValidatedState,
};
use hotshot_example_types::{
    block_types::{TestBlockHeader, TestBlockPayload, TestTransaction},
    state_types::TestInstanceState,
};
use hotshot_types::data::{BlockError, Leaf};
use hotshot_types::traits::states::StateDelta;
use hotshot_types::{
    data::{QuorumProposal, ViewNumber},
    message::Message,
    signature_key::BLSPubKey,
    traits::node_implementation::NodeType,
};
use jf_primitives::merkle_tree::prelude::{Sha3Digest, Sha3Node};
use jf_primitives::merkle_tree::universal_merkle_tree::UniversalMerkleTree;
use jf_primitives::merkle_tree::ToTraversalPath;
use jf_primitives::merkle_tree::{MerkleTreeScheme, UniversalMerkleTreeScheme};
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
    type ValidatedState = MockValidatedState;
    type Membership = GeneralStaticCommittee<Self, BLSPubKey>;
}

pub type TestMerkleTree = UniversalMerkleTree<usize, Sha3Digest, usize, typenum::U8, Sha3Node>;

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct MockValidatedState {
    pub tree: TestMerkleTree,
    pub block_height: u64,
}

impl Default for MockValidatedState {
    fn default() -> Self {
        Self {
            tree: TestMerkleTree::new(20),
            block_height: 0,
        }
    }
}

impl MerklizedState<MockTypes> for TestMerkleTree {
    type Arity = typenum::U8;
    type Key = usize;

    fn state_type(&self) -> &'static str {
        "test_tree"
    }

    fn deltas(&self, _header: TestBlockHeader) -> Vec<Self::Key> {
        Vec::new()
    }

    fn header_state_commitment_field(&self) -> &'static str {
        "test_merkle_tree_root"
    }
}

#[async_trait]
impl UpdateStateStorage<MockTypes> for MockValidatedState {
    async fn update_storage(
        &self,
        storage: &mut impl crate::merklized_state::UpdateStateData,
        leaf: &Leaf<MockTypes>,
        delta: Arc<<<MockTypes as NodeType>::ValidatedState as ValidatedState<MockTypes>>::Delta>,
    ) -> anyhow::Result<()> {
        let tree = &self.tree;
        let block_number = &leaf.block_header.block_number;

        for key in delta.0.iter() {
            let (_, proof) = self.tree.lookup(key).expect_ok().unwrap();

            let traversal_path =
                <usize as ToTraversalPath<typenum::U8>>::to_traversal_path(key, tree.height());

            storage
                .insert_merkle_nodes(tree.state_type(), proof, traversal_path, *block_number)
                .await
                .unwrap()
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct MockStateDelta(pub Vec<usize>);
impl StateDelta for MockStateDelta {}

impl ValidatedState<MockTypes> for MockValidatedState {
    type Error = BlockError;

    type Instance = TestInstanceState;

    type Delta = MockStateDelta;

    type Time = ViewNumber;

    async fn validate_and_apply_header(
        &self,
        _instance: &Self::Instance,
        _parent_leaf: &Leaf<MockTypes>,
        _proposed_header: &<MockTypes as NodeType>::BlockHeader,
    ) -> Result<(Self, Self::Delta), Self::Error> {
        let mut tree = self.tree.clone();

        tree.update(
            self.block_height as usize,
            self.block_height as usize + 1000,
        )
        .unwrap();

        Ok((
            Self {
                tree,
                block_height: self.block_height + 1,
            },
            MockStateDelta(vec![(self.block_height).try_into().unwrap()]),
        ))
    }

    fn from_header(_block_header: &<MockTypes as NodeType>::BlockHeader) -> Self {
        Self {
            tree: TestMerkleTree::new(20),
            block_height: 0,
        }
    }

    fn on_commit(&self) {}

    fn genesis(_instance: &Self::Instance) -> (Self, Self::Delta) {
        (Self::default(), MockStateDelta(vec![]))
    }
}

pub type MockMembership = GeneralStaticCommittee<MockTypes, <MockTypes as NodeType>::SignatureKey>;

pub type MockQuorumProposal = QuorumProposal<MockTypes>;
pub type MockNetwork = MemoryNetwork<Message<MockTypes>, BLSPubKey>;

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct MockNodeImpl;

impl NodeImplementation<MockTypes> for MockNodeImpl {
    type Storage = MemoryStorage<MockTypes>;
    type QuorumNetwork = MockNetwork;
    type CommitteeNetwork = MockNetwork;
}
