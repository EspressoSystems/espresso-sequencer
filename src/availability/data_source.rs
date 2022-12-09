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

use super::query_data::{BlockHash, BlockQueryData, LeafHash, LeafQueryData, TransactionHash};
use hotshot_types::traits::{node_implementation::NodeTypes, signature_key::EncodedPublicKey};
use std::error::Error;
use std::fmt::Debug;

pub trait AvailabilityDataSource<Types: NodeTypes> {
    type LeafIterType<'a>: 'a + Iterator<Item = Option<LeafQueryData<Types>>>
    where
        Self: 'a;
    type BlockIterType<'a>: 'a + Iterator<Item = Option<BlockQueryData<Types>>>
    where
        Self: 'a;
    fn get_nth_leaf_iter(&self, n: usize) -> Self::LeafIterType<'_>;
    fn get_nth_block_iter(&self, n: usize) -> Self::BlockIterType<'_>;
    fn get_leaf_index_by_hash(&self, hash: LeafHash<Types>) -> Option<u64>;
    fn get_block_index_by_hash(&self, hash: BlockHash<Types>) -> Option<u64>;
    fn get_txn_index_by_hash(&self, hash: TransactionHash<Types>) -> Option<(u64, u64)>;
    fn get_leaf_ids_by_proposer_id(&self, id: &EncodedPublicKey) -> Vec<u64>;
}

pub trait UpdateAvailabilityData<Types: NodeTypes> {
    type Error: Error + Debug;
    fn insert_leaf(&mut self, leaf: LeafQueryData<Types>) -> Result<(), Self::Error>;
    fn insert_block(&mut self, block: BlockQueryData<Types>) -> Result<(), Self::Error>;
}
