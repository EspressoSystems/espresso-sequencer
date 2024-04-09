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

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use async_trait::async_trait;
use derivative::Derivative;
use derive_more::Display;
use hotshot::traits::ValidatedState;
use hotshot_types::{data::Leaf, traits::node_implementation::NodeType};

use jf_primitives::merkle_tree::DigestAlgorithm;
use jf_primitives::merkle_tree::{
    prelude::MerklePath, Element, Index, MerkleCommitment, MerkleTreeScheme, NodeValue,
    ToTraversalPath,
};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;
use std::{fmt::Debug, str::FromStr};
use tagged_base64::TaggedBase64;

use std::{cmp::Ordering, sync::Arc};

use crate::QueryResult;

/// This trait defines methods that a data source should implement
/// It enables retrieval of the membership path for a leaf node, which can be used to reconstruct the Merkle tree state.
#[async_trait]
pub trait MerklizedStateDataSource<Types, State, const ARITY: usize>
where
    Types: NodeType,
    State: MerklizedState<Types, ARITY>,
{
    async fn get_path(
        &self,
        snapshot: Snapshot<Types, State, ARITY>,
        key: State::Key,
    ) -> QueryResult<MerklePath<State::Entry, State::Key, State::T>>;
}

/// This trait defines methods for updating the storage with the merkle tree state.
#[async_trait]
pub trait UpdateStateData<Types: NodeType, State: MerklizedState<Types, ARITY>, const ARITY: usize>:
    Send + Sync
{
    async fn insert_merkle_nodes(
        &mut self,
        path: MerklePath<State::Entry, State::Key, State::T>,
        traversal_path: Vec<usize>,
        block_number: u64,
    ) -> QueryResult<()>;
}

/// This trait should be implemented by types that represent validated states.
/// It enables services using the module to implement update functionality.
/// Some services may manage multiple Merkle trees within their validated state,
/// requiring updates for all state types with each event.
/// Therefore, the `insert_merkle_nodes` method of `UpdateStateData` should be called
/// for all state types within the validated state.
#[async_trait]
pub trait UpdateStateStorage<Types: NodeType, D> {
    async fn update_storage(
        &self,
        storage: &mut D,
        leaf: &Leaf<Types>,
        delta: Arc<<<Types as NodeType>::ValidatedState as ValidatedState<Types>>::Delta>,
    ) -> anyhow::Result<()>;
}

type StateCommitment<Types, T, const ARITY: usize> = <T as MerklizedState<Types, ARITY>>::Commit;
#[derive(Derivative, Display)]
#[derivative(Ord = "feature_allow_slow_enum")]
#[derivative(
    Debug(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Ord(bound = ""),
    Hash(bound = "")
)]
// Snapshot can be queried by block height (index) or merkle tree commitment
pub enum Snapshot<Types: NodeType, T: MerklizedState<Types, ARITY>, const ARITY: usize> {
    #[display(fmt = "{_0}")]
    Commit(StateCommitment<Types, T, ARITY>),
    #[display(fmt = "{_0}")]
    Index(u64),
}

impl<T: MerklizedState<Types, ARITY>, Types: NodeType, const ARITY: usize> PartialOrd
    for Snapshot<Types, T, ARITY>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// This trait should be implemented by the MerkleTree that the module is initialized for.
/// It defines methods utilized by the module.
pub trait MerklizedState<Types, const ARITY: usize>:
    MerkleTreeScheme + Send + Sync + Clone + 'static
where
    Types: NodeType,
{
    type Key: Index
        + Send
        + Sync
        + Serialize
        + ToTraversalPath<ARITY>
        + FromStr
        + DeserializeOwned
        + Display
        + CanonicalSerialize
        + CanonicalDeserialize;
    type Entry: Element
        + Send
        + Sync
        + Serialize
        + DeserializeOwned
        + CanonicalSerialize
        + CanonicalDeserialize;
    type T: NodeValue + Send;
    type Commit: MerkleCommitment<Self::T>
        + Send
        + for<'a> TryFrom<&'a TaggedBase64>
        + Display
        + Debug
        + Into<TaggedBase64>;
    type Digest: DigestAlgorithm<Self::Entry, Self::Key, Self::T>;

    /// Retrieves the name of the state being queried.
    fn state_type() -> &'static str;

    /// Retrieves the field in the header containing the Merkle tree commitment
    /// for the state implementing this trait.
    fn header_state_commitment_field() -> &'static str;

    /// Get the height of the tree
    fn tree_height() -> usize;
}
