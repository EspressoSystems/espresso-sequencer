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

use ark_serialize::CanonicalDeserialize;
use derive_more::{Display, From};

use async_trait::async_trait;
use commit::Commitment;
use derivative::Derivative;
use hotshot_types::{data::Leaf, traits::node_implementation::NodeType};
use jf_primitives::{
    circuit::merkle_tree::MembershipProof,
    merkle_tree::{
        prelude::{MerkleNode, MerklePath},
        Element, Index, NodeValue, ToTraversalPath,
    },
};

use serde::de::DeserializeOwned;
use serde_json::Value;
use typenum::Unsigned;

use std::cmp::Ordering;

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

#[async_trait]
pub trait UpdateStateStorage<
    Types: NodeType,
    Proof: MembershipProof<E, I, T> + Send + Sync + 'static,
    E: Element + Send + Sync,
    I: Index + Send + Sync,
    T: NodeValue + Send + Sync,
>
{
    type Error: std::error::Error + std::fmt::Debug + Send + Sync + 'static;
    async fn insert_nodes(
        &mut self,
        _name: String,
        _proof: Proof,
        _path: Vec<usize>,
        _leaf: Leaf<Types>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
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

// These tests run the `postgres` Docker image, which doesn't work on Windows.
#[cfg(all(test, not(target_os = "windows")))]
mod test {

    use crate::merklized_state::MerklizedState;
    use jf_primitives::merkle_tree::prelude::LightWeightSHA3MerkleTree;
    use typenum::U3;

    type TestMerkleTree = LightWeightSHA3MerkleTree<usize>;

    impl MerklizedState for TestMerkleTree {
        type Arity = U3;
        fn state_type(&self) -> &'static str {
            "test_tree"
        }

        fn header_state_commitment_field(&self) -> &'static str {
            "block_merkle_tree_root"
        }
    }
}
