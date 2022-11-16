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

use commit::Commitment;
use hotshot::{data::Leaf, traits::Block};
use hotshot_types::traits::node_implementation::NodeTypes;
use serde::{Deserialize, Serialize};

pub type LeafHash<Types> = Commitment<Leaf<Types>>;
pub type BlockHash<Types> = Commitment<<Types as NodeTypes>::BlockType>;
pub type TransactionHash<Types> =
    Commitment<<<Types as NodeTypes>::BlockType as Block>::Transaction>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(bound = "")]
pub struct LeafQueryData<Types: NodeTypes> {
    pub leaf: Leaf<Types>,
    pub hash: LeafHash<Types>,
    pub height: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct BlockQueryData<Types: NodeTypes> {
    pub block: Types::BlockType,
    pub hash: BlockHash<Types>,
    pub height: u64,
    pub size: u64,
    pub txn_hashes: Vec<TransactionHash<Types>>,
}

impl<Types: NodeTypes> BlockQueryData<Types> {
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
        Some(TransactionQueryData {
            transaction: unimplemented!(), // TODO the block trait should expose some way of getting the `i`th transaction
            height: self.height,
            index: i as u64,
            hash: self.txn_hashes[i],
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct TransactionQueryData<Types: NodeTypes> {
    pub transaction: <Types::BlockType as Block>::Transaction,
    pub height: u64,
    pub index: u64,
    pub hash: TransactionHash<Types>,
}
