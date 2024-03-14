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
use ark_serialize::CanonicalDeserialize;
use derive_more::{Display, From};

use async_trait::async_trait;
use commit::Commitment;
use derivative::Derivative;
use hotshot::traits::ValidatedState;
use hotshot_types::{data::Leaf, traits::node_implementation::NodeType};
use jf_primitives::{
    circuit::merkle_tree::MembershipProof,
    merkle_tree::{
        prelude::{MerkleNode, MerklePath},
        Element, Index, MerkleTreeScheme, NodeValue, ToTraversalPath,
    },
};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use typenum::Unsigned;

use std::{cmp::Ordering, sync::Arc};

// This trait defines methods that a data source should implement
// It enables retrieval of the membership path for a leaf node, which can be used to reconstruct the Merkle tree state.
#[async_trait]
pub trait MerklizedStateDataSource<Types>
where
    Types: NodeType,
{
    type Error: std::error::Error + std::fmt::Debug + Send + Sync + 'static;
    async fn get_path<
        E: Element + Send + DeserializeOwned,
        I: Index + Send + ToTraversalPath<A> + DeserializeOwned,
        A: Unsigned,
        T: NodeValue + Send + CanonicalDeserialize,
    >(
        &self,
        _state_type: &'static str,
        _tree_height: usize,
        _header_state_commitment_field: &'static str,
        _snapshot: Snapshot<Types>,
        _key: Value,
    ) -> Result<MerklePath<E, I, T>, Self::Error> {
        Ok(vec![MerkleNode::Empty])
    }
}

// This trait defines methods for updating the storage with the merkle tree state.
#[async_trait]
pub trait UpdateStateData: Send + Sync {
    type Error: std::error::Error + std::fmt::Debug + Send + Sync + 'static;
    async fn insert_merkle_nodes<
        Proof: MembershipProof<E, I, T> + Send + Sync + std::fmt::Debug + 'static,
        E: Element + Send + Sync + Serialize,
        I: Index + Send + Sync + Serialize,
        T: NodeValue + Send + Sync,
    >(
        &mut self,
        _name: &'static str,
        _proof: Proof,
        _path: Vec<usize>,
        _block_number: u64,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

// This trait should be implemented by types that represent validated states.
// It enables services using the module to implement update functionality.
// Some services may manage multiple Merkle trees within their validated state,
// requiring updates for all state types with each event.
// Therefore, the `insert_merkle_nodes` method of `UpdateStateData` should be called
// for all state types within the validated state.
#[async_trait]
pub trait UpdateStateStorage<Types: NodeType> {
    async fn update_storage(
        &self,
        storage: &mut impl UpdateStateData,
        leaf: &Leaf<Types>,
        delta: Arc<<<Types as NodeType>::ValidatedState as ValidatedState<Types>>::Delta>,
    ) -> anyhow::Result<()>;
}

type MerkleCommitment<Types> = Commitment<Leaf<Types>>;

#[derive(Derivative, From, Display)]
#[derivative(Ord = "feature_allow_slow_enum")]
#[derivative(
    Copy(bound = ""),
    Debug(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Ord(bound = ""),
    Hash(bound = "")
)]

// Snapshot can be queried by block height (index) or merkle tree commitment
pub enum Snapshot<Types: NodeType> {
    #[display(fmt = "{_0}")]
    Commit(MerkleCommitment<Types>),
    #[display(fmt = "{_0}")]
    Index(u64),
}

impl<Types: NodeType> Clone for Snapshot<Types> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Types: NodeType> PartialOrd for Snapshot<Types> {
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
