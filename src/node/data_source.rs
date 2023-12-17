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

use super::query_data::LeafQueryData;
use crate::QueryResult;
use async_trait::async_trait;
use hotshot_types::traits::{node_implementation::NodeType, signature_key::EncodedPublicKey};
use std::error::Error;
use std::fmt::Debug;

#[async_trait]
pub trait NodeDataSource<Types: NodeType> {
    async fn block_height(&self) -> QueryResult<usize>;
    async fn get_proposals(
        &self,
        proposer: &EncodedPublicKey,
        limit: Option<usize>,
    ) -> QueryResult<Vec<LeafQueryData<Types>>>;
    async fn count_proposals(&self, proposer: &EncodedPublicKey) -> QueryResult<usize>;
}

#[async_trait]
pub trait UpdateNodeData<Types: NodeType> {
    type Error: Error + Debug + Send + Sync + 'static;
    async fn insert_leaf(&mut self, leaf: LeafQueryData<Types>) -> Result<(), Self::Error>;
}
