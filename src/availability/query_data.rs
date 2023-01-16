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

use bincode::Options;
use commit::{Commitment, Committable};
use hotshot::{
    data::{Leaf, QuorumCertificate},
    traits::Block,
};
use hotshot_types::traits::{node_implementation::NodeTypes, signature_key::EncodedPublicKey};
use hotshot_utils::bincode::bincode_opts;
use serde::{Deserialize, Serialize};

pub type LeafHash<Types> = Commitment<Leaf<Types>>;
pub type BlockHash<Types> = Commitment<<Types as NodeTypes>::BlockType>;
pub type TransactionHash<Types> =
    Commitment<<<Types as NodeTypes>::BlockType as Block>::Transaction>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(bound = "")]
pub struct LeafQueryData<Types: NodeTypes> {
    leaf: Leaf<Types>,
    qc: QuorumCertificate<Types>,
}

impl<Types: NodeTypes> LeafQueryData<Types> {
    pub fn new(leaf: Leaf<Types>, qc: QuorumCertificate<Types>) -> Self {
        assert_eq!(qc.leaf_commitment, leaf.commit());
        Self { leaf, qc }
    }

    pub fn leaf(&self) -> &Leaf<Types> {
        &self.leaf
    }

    pub fn qc(&self) -> &QuorumCertificate<Types> {
        &self.qc
    }

    pub fn height(&self) -> u64 {
        self.leaf.height
    }

    pub fn hash(&self) -> LeafHash<Types> {
        self.qc.leaf_commitment
    }

    pub fn block_hash(&self) -> BlockHash<Types> {
        self.qc.block_commitment
    }

    pub fn proposer(&self) -> &EncodedPublicKey {
        &self.leaf.proposer_id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct BlockQueryData<Types: NodeTypes> {
    block: Types::BlockType,
    hash: BlockHash<Types>,
    height: u64,
    size: u64,
    txn_hashes: Vec<TransactionHash<Types>>,
}

impl<Types: NodeTypes> BlockQueryData<Types> {
    pub fn new(leaf: Leaf<Types>, qc: QuorumCertificate<Types>) -> Self
    where
        Types::BlockType: Serialize,
    {
        assert_eq!(qc.block_commitment, leaf.deltas.commit());
        Self {
            hash: qc.block_commitment,
            height: leaf.height,
            size: bincode_opts()
                .serialized_size(&leaf.deltas)
                .unwrap_or_default(),
            txn_hashes: leaf.deltas.contained_transactions().into_iter().collect(),
            block: leaf.deltas,
        }
    }

    pub fn block(&self) -> &Types::BlockType {
        &self.block
    }

    pub fn hash(&self) -> BlockHash<Types> {
        self.hash
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn iter(&self) -> impl Iterator<Item = TransactionHash<Types>> + '_ {
        self.txn_hashes.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.txn_hashes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn transaction(&self, i: usize) -> Option<TransactionQueryData<Types>> {
        if i >= self.len() {
            return None;
        }
        #[allow(unreachable_code)]
        Some(TransactionQueryData {
            transaction: unimplemented!(), // TODO the block trait should expose some way of getting the `i`th transaction
            height: self.height,
            index: i as u64,
            hash: self.txn_hashes[i],
        })
    }
}

impl<Types: NodeTypes> IntoIterator for BlockQueryData<Types> {
    type Item = TransactionHash<Types>;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.txn_hashes.into_iter()
    }
}

impl<'a, Types: NodeTypes> IntoIterator for &'a BlockQueryData<Types> {
    type Item = TransactionHash<Types>;
    type IntoIter = std::iter::Copied<<&'a Vec<Self::Item> as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.txn_hashes.iter().copied()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct TransactionQueryData<Types: NodeTypes> {
    transaction: <Types::BlockType as Block>::Transaction,
    height: u64,
    index: u64,
    hash: TransactionHash<Types>,
}

impl<Types: NodeTypes> TransactionQueryData<Types> {
    pub fn transaction(&self) -> &<Types::BlockType as Block>::Transaction {
        &self.transaction
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn index(&self) -> u64 {
        self.index
    }

    pub fn hash(&self) -> TransactionHash<Types> {
        self.hash
    }
}
