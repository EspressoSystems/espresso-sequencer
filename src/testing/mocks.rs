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

use crate::availability::QueryablePayload;
use hotshot::traits::{
    election::static_committee::{GeneralStaticCommittee, StaticElectionConfig},
    implementations::{MemoryNetwork, MemoryStorage},
    NodeImplementation,
};
use hotshot_example_types::{
    block_types::{TestBlockHeader, TestBlockPayload, TestTransaction},
    state_types::{TestInstanceState, TestValidatedState},
};
use hotshot_types::{
    data::{QuorumProposal, ViewNumber},
    message::Message,
    signature_key::BLSPubKey,
    traits::node_implementation::NodeType,
};
use serde::{Deserialize, Serialize};
use std::ops::Range;

pub type MockHeader = TestBlockHeader;
pub type MockPayload = TestBlockPayload;
pub type MockTransaction = TestTransaction;

pub fn mock_transaction(payload: Vec<u8>) -> MockTransaction {
    TestTransaction(payload)
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
