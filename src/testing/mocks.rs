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

use crate::{
    availability::{AvailabilityDataSource, QueryablePayload},
    data_source::{UpdateDataSource, VersionedDataSource},
    status::StatusDataSource,
};
use async_trait::async_trait;
use hotshot::{
    demo::DemoState,
    traits::{
        election::static_committee::{GeneralStaticCommittee, StaticElectionConfig},
        implementations::{MemoryCommChannel, MemoryStorage},
        NodeImplementation,
    },
};
use hotshot_signature_key::bn254::BLSPubKey;
use hotshot_types::{
    block_impl::{VIDBlockHeader, VIDBlockPayload, VIDTransaction},
    data::{QuorumProposal, ViewNumber},
    traits::node_implementation::{ChannelMaps, NodeType},
};
use serde::{Deserialize, Serialize};
use std::ops::Range;

pub type MockState = DemoState;
pub type MockHeader = VIDBlockHeader;
pub type MockPayload = VIDBlockPayload;
pub type MockTransaction = VIDTransaction;

pub fn mock_transaction(payload: Vec<u8>) -> MockTransaction {
    VIDTransaction(payload)
}

impl QueryablePayload for MockPayload {
    type TransactionIndex = usize;
    type Iter<'a> = Range<usize>;
    type InclusionProof = ();

    fn len(&self) -> usize {
        self.transactions.len()
    }

    fn iter(&self) -> Self::Iter<'_> {
        0..self.len()
    }

    fn transaction_with_proof(
        &self,
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
    type StateType = MockState;
    type Membership = GeneralStaticCommittee<Self, BLSPubKey>;
}

pub type MockMembership = GeneralStaticCommittee<MockTypes, <MockTypes as NodeType>::SignatureKey>;

pub type MockQuorumProposal = QuorumProposal<MockTypes>;
pub type MockQuorumNetwork = MemoryCommChannel<MockTypes>;
pub type MockDANetwork = MemoryCommChannel<MockTypes>;

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct MockNodeImpl;

impl NodeImplementation<MockTypes> for MockNodeImpl {
    type Storage = MemoryStorage<MockTypes>;
    type QuorumNetwork = MockQuorumNetwork;
    type CommitteeNetwork = MockDANetwork;

    fn new_channel_maps(
        start_view: ViewNumber,
    ) -> (ChannelMaps<MockTypes>, Option<ChannelMaps<MockTypes>>) {
        (ChannelMaps::new(start_view), None)
    }
}

#[async_trait]
pub trait TestableDataSource:
    AvailabilityDataSource<MockTypes>
    + StatusDataSource
    + UpdateDataSource<MockTypes>
    + VersionedDataSource
    + Send
    + Sync
    + Sized
    + 'static
{
    /// Backing storage for the data source.
    ///
    /// This can be used to connect to data sources to the same underlying data. It must be kept
    /// alive as long as the related data sources are open.
    type Storage: Send + Sync;

    async fn create(node_id: usize) -> Self::Storage;
    async fn connect(storage: &Self::Storage) -> Self;
}
