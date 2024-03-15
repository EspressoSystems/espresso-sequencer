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

//! Data for the [`merklized_state`](super) API.
//!
//! This module facilitates storing the state of a Merkle Tree at a specific point in time
//! and provides methods for querying and reconstructing the snapshot.
//!

use async_trait::async_trait;
use derivative::Derivative;
use derive_more::Display;
use hotshot::traits::ValidatedState;
use hotshot_types::{data::Leaf, traits::node_implementation::NodeType};
use jf_primitives::merkle_tree::{prelude::MerklePath, Index, MerkleTreeScheme};
use serde::Serialize;
use std::fmt::Debug;

use typenum::Unsigned;

use std::{cmp::Ordering, sync::Arc};

use crate::QueryResult;

// This trait defines methods that a data source should implement
// It enables retrieval of the membership path for a leaf node, which can be used to reconstruct the Merkle tree state.
#[async_trait]
pub trait MerklizedStateDataSource<Types, State>
where
    Types: NodeType,
    State: MerklizedState<Types>,
{
    async fn get_path(
        &self,
        _state_type: &'static str,
        _tree_height: usize,
        _header_state_commitment_field: &'static str,
        _snapshot: Snapshot<State>,
        _key: String,
    ) -> QueryResult<MerklePath<State::Element, State::Index, State::NodeValue>>;
}

// This trait defines methods for updating the storage with the merkle tree state.
#[async_trait]
pub trait UpdateStateData<Types: NodeType, State: MerklizedState<Types>>: Send + Sync
where
    State::Element: Serialize,
    State::Index: Serialize,
{
    async fn insert_merkle_nodes(
        &mut self,
        _name: &'static str,
        _path: MerklePath<State::Element, State::Index, State::NodeValue>,
        _traversal_path: Vec<usize>,
        _block_number: u64,
    ) -> QueryResult<()>;
}

// This trait should be implemented by types that represent validated states.
// It enables services using the module to implement update functionality.
// Some services may manage multiple Merkle trees within their validated state,
// requiring updates for all state types with each event.
// Therefore, the `insert_merkle_nodes` method of `UpdateStateData` should be called
// for all state types within the validated state.
#[async_trait]
pub trait UpdateStateStorage<Types: NodeType, D> {
    async fn update_storage(
        &self,
        storage: &mut D,
        leaf: &Leaf<Types>,
        delta: Arc<<<Types as NodeType>::ValidatedState as ValidatedState<Types>>::Delta>,
    ) -> anyhow::Result<()>;
}

type MerkleCommitment<T> = <T as MerkleTreeScheme>::Commitment;
#[derive(Derivative, Display)]
#[derivative(Ord = "feature_allow_slow_enum")]
#[derivative(
    Debug(bound = "T::Commitment: Debug"),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Ord(bound = ""),
    Hash(bound = "")
)]
// Snapshot can be queried by block height (index) or merkle tree commitment
pub enum Snapshot<T: MerkleTreeScheme> {
    #[display(bound = "T::Commitment: Display ", fmt = "{_0}")]
    Commit(MerkleCommitment<T>),
    #[display(fmt = "{_0}")]
    Index(u64),
}

impl<T: MerkleTreeScheme> PartialOrd for Snapshot<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// This trait should be implemented by the MerkleTree that the module is initialized for.
// It defines methods utilized by the module.
pub trait MerklizedState<Types>: MerkleTreeScheme
where
    Types: NodeType,
{
    type Arity: Unsigned;
    type Key: Index;

    /// Retrieves the name of the state being queried.
    fn state_type(&self) -> &'static str;

    /// Determines the entries in the Merkle tree affected by the provided header.
    fn deltas(&self, header: <Types as NodeType>::BlockHeader) -> Vec<Self::Key>;

    /// Retrieves the field in the header containing the Merkle tree commitment
    /// for the state implementing this trait.
    fn header_state_commitment_field(&self) -> &'static str;
}
