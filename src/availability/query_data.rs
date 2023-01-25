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

use crate::{Leaf, QuorumCertificate};
use bincode::Options;
use commit::{Commitment, Committable};
use hotshot_types::{
    data::LeafType,
    traits::{
        election::SignedCertificate,
        node_implementation::{NodeImplementation, NodeType},
        signature_key::EncodedPublicKey,
        Block,
    },
};
use hotshot_utils::bincode::bincode_opts;
use serde::{Deserialize, Serialize};

pub type LeafHash<Types, I> = Commitment<Leaf<Types, I>>;
pub type BlockHash<Types> = Commitment<<Types as NodeType>::BlockType>;
pub type TransactionHash<Types> =
    Commitment<<<Types as NodeType>::BlockType as Block>::Transaction>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct LeafQueryData<Types: NodeType, I: NodeImplementation<Types>> {
    leaf: Leaf<Types, I>,
    qc: QuorumCertificate<Types, I>,
}

impl<Types: NodeType, I: NodeImplementation<Types>> LeafQueryData<Types, I> {
    pub fn new(leaf: Leaf<Types, I>, qc: QuorumCertificate<Types, I>) -> Self {
        assert_eq!(qc.leaf_commitment(), leaf.commit());
        Self { leaf, qc }
    }

    pub fn leaf(&self) -> &Leaf<Types, I> {
        &self.leaf
    }

    pub fn qc(&self) -> QuorumCertificate<Types, I> {
        self.leaf.get_justify_qc()
    }

    pub fn height(&self) -> u64 {
        self.leaf.get_height()
    }

    pub fn hash(&self) -> LeafHash<Types, I> {
        self.leaf.commit()
    }

    pub fn block_hash(&self) -> BlockHash<Types> {
        self.leaf.get_deltas().commit()
    }

    pub fn proposer(&self) -> EncodedPublicKey {
        self.leaf.get_proposer_id()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct BlockQueryData<Types: NodeType> {
    block: Types::BlockType,
    hash: BlockHash<Types>,
    height: u64,
    size: u64,
    txn_hashes: Vec<TransactionHash<Types>>,
}

impl<Types: NodeType> BlockQueryData<Types> {
    pub fn new<I: NodeImplementation<Types>>(
        leaf: Leaf<Types, I>,
        qc: QuorumCertificate<Types, I>,
    ) -> Self
    where
        Types::BlockType: Serialize,
    {
        assert_eq!(qc.leaf_commitment(), leaf.commit());
        let block = leaf.get_deltas();
        Self {
            hash: block.commit(),
            height: leaf.get_height(),
            size: bincode_opts().serialized_size(&block).unwrap_or_default(),
            txn_hashes: block.contained_transactions().into_iter().collect(),
            block,
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

impl<Types: NodeType> IntoIterator for BlockQueryData<Types> {
    type Item = TransactionHash<Types>;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.txn_hashes.into_iter()
    }
}

impl<'a, Types: NodeType> IntoIterator for &'a BlockQueryData<Types> {
    type Item = TransactionHash<Types>;
    type IntoIter = std::iter::Copied<<&'a Vec<Self::Item> as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.txn_hashes.iter().copied()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct TransactionQueryData<Types: NodeType> {
    transaction: <Types::BlockType as Block>::Transaction,
    height: u64,
    index: u64,
    hash: TransactionHash<Types>,
}

impl<Types: NodeType> TransactionQueryData<Types> {
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
