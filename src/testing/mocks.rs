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
    availability::AvailabilityDataSource,
    data_source::{UpdateDataSource, VersionedDataSource},
    status::StatusDataSource,
    QueryableBlock,
};
use async_std::sync::Arc;
use async_trait::async_trait;
use commit::{Commitment, Committable, RawCommitmentBuilder};
use derive_more::{Display, Index, IndexMut};
use hotshot::traits::{
    election::static_committee::{GeneralStaticCommittee, StaticElectionConfig},
    implementations::{MemoryCommChannel, MemoryStorage},
    NodeImplementation,
};
use hotshot_signature_key::bn254::BLSPubKey;
use hotshot_types::{
    data::{QuorumProposal, ViewNumber},
    traits::{
        block_contents::{BlockHeader, Transaction},
        node_implementation::{ChannelMaps, NodeType},
        state::{State, TestableBlock, TestableState},
        BlockPayload,
    },
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::collections::BTreeSet;
use std::ops::Range;

#[derive(Clone, Debug, Snafu)]
pub enum MockError {
    DoubleSpend { nonce: u64 },
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
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

#[derive(Clone, Debug, Display, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[display(fmt = "{:?}", self)]
pub struct MockState {
    pub last_header: Commitment<MockHeader>,
    pub spent: Arc<BTreeSet<u64>>,
}

impl Default for MockState {
    fn default() -> Self {
        Self {
            last_header: MockHeader::default().commit(),
            spent: Default::default(),
        }
    }
}

impl Committable for MockState {
    fn commit(&self) -> Commitment<Self> {
        RawCommitmentBuilder::new("MockState")
            .field("last_block", self.last_header)
            .var_size_bytes(&bincode::serialize(&self.spent).unwrap())
            .finalize()
    }

    fn tag() -> String {
        "MOCKSTATE".to_string()
    }
}

impl MockState {
    fn validate(&self, header: &MockHeader) -> Result<(), MockError> {
        if let Some(txn) = header
            .metadata()
            .transactions
            .iter()
            .find(|txn| self.spent.contains(&txn.nonce))
        {
            return Err(DoubleSpendSnafu { nonce: txn.nonce }.build());
        }
        Ok(())
    }
}

impl State for MockState {
    type Error = MockError;
    type Time = ViewNumber;
    type BlockHeader = MockHeader;
    type BlockPayload = MockBlock;

    fn validate_block(&self, header: &Self::BlockHeader, view_number: &Self::Time) -> bool {
        self.validate(header).is_ok()
    }

    fn append(
        &self,
        header: &Self::BlockHeader,
        _view_number: &Self::Time,
    ) -> Result<Self, Self::Error> {
        self.validate(header)?;

        let mut spent = (*self.spent).clone();
        for txn in header.metadata().transactions.iter() {
            spent.insert(txn.nonce);
        }
        Ok(Self {
            last_header: header.commit(),
            spent: Arc::new(spent),
        })
    }

    fn on_commit(&self) {}

    fn initialize() -> Self {
        todo!()
    }
}

impl TestableState for MockState {
    fn create_random_transaction(
        state: Option<&Self>,
        rng: &mut dyn RngCore,
        _padding: u64,
    ) -> <Self::BlockPayload as BlockPayload>::Transaction {
        loop {
            let nonce = rng.next_u64();
            if let Some(state) = state {
                if state.spent.contains(&nonce) {
                    continue;
                }
            }
            break MockTransaction { nonce };
        }
    }
}

#[derive(
    PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug, Default, Display, Index, IndexMut,
)]
#[display(fmt = "{:?}", self)]
pub struct MockBlock {
    #[index]
    #[index_mut]
    pub transactions: Vec<MockTransaction>,
}

impl Committable for MockBlock {
    fn commit(&self) -> Commitment<Self> {
        RawCommitmentBuilder::new("MockBlock")
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
    pub fn genesis() -> Self {
        Self::default()
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

impl QueryableBlock for MockBlock {
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
        Some((self.transactions.get(*index)?.clone(), ()))
    }
}

impl IntoIterator for MockBlock {
    type Item = MockTransaction;
    type IntoIter = std::vec::IntoIter<MockTransaction>;

    fn into_iter(self) -> Self::IntoIter {
        self.transactions.into_iter()
    }
}

#[derive(Clone, Debug, Default, Display, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct MockHeader;

impl BlockHeader for MockHeader {
    type Payload = MockBlock;

    fn new(
        payload_commitment: hotshot_types::data::VidCommitment,
        metadata: <Self::Payload as hotshot_types::traits::BlockPayload>::Metadata,
        parent_header: &Self,
    ) -> Self {
        todo!()
    }

    fn genesis() -> (
        Self,
        Self::Payload,
        <Self::Payload as hotshot_types::traits::BlockPayload>::Metadata,
    ) {
        todo!()
    }

    fn block_number(&self) -> u64 {
        todo!()
    }

    fn payload_commitment(&self) -> hotshot_types::data::VidCommitment {
        todo!()
    }

    fn metadata(&self) -> <Self::Payload as hotshot_types::traits::BlockPayload>::Metadata {
        todo!()
    }
}

impl Committable for MockHeader {
    fn commit(&self) -> Commitment<Self> {
        todo!()
    }
}

/// Use the metadata to store mock transactions so we have something to validate.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct MockMetadata {
    pub transactions: Vec<MockTransaction>,
}

impl BlockPayload for MockBlock {
    type Error = MockError;
    type Transaction = MockTransaction;
    type Metadata = MockMetadata;
    type Encode<'a> = <Vec<u8> as IntoIterator>::IntoIter;

    fn from_transactions(
        transactions: impl IntoIterator<Item = Self::Transaction>,
    ) -> Result<(Self, Self::Metadata), Self::Error> {
        todo!()
    }

    fn from_bytes<I>(encoded_transactions: I, metadata: Self::Metadata) -> Self
    where
        I: Iterator<Item = u8>,
    {
        todo!()
    }

    fn genesis() -> (Self, Self::Metadata) {
        todo!()
    }

    fn encode(&self) -> Result<Self::Encode<'_>, Self::Error> {
        todo!()
    }

    fn transaction_commitments(&self) -> Vec<Commitment<Self::Transaction>> {
        todo!()
    }
}

impl TestableBlock for MockBlock {
    fn genesis() -> Self {
        Self::genesis()
    }

    fn txn_count(&self) -> u64 {
        self.transactions.len() as u64
    }
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct MockTypes;

impl NodeType for MockTypes {
    type Time = ViewNumber;
    type BlockHeader = MockHeader;
    type BlockPayload = MockBlock;
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
