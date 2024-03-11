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
    availability::QueryableHeader,
    data_source::storage::{sql::*, SqlStorage},
    Header, QueryError, QueryResult,
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use async_std::stream::StreamExt;
use bit_vec::BitVec;
use derive_more::Display;
use derive_more::From;

use async_trait::async_trait;
use commit::{Commitment, Committable};
use derivative::Derivative;
use hotshot_types::{data::Leaf, traits::node_implementation::NodeType};
use jf_primitives::{
    circuit::merkle_tree::MembershipProof,
    merkle_tree::{
        prelude::{MerkleNode, MerklePath},
        Element, Index, NodeValue, ToTraversalPath,
    },
};

use postgres::Row;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use typenum::Unsigned;

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Display,
    sync::Arc,
};

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
        state_type: &'static str,
        tree_height: usize,
        header_state_commitment_field: &'static str,
        snapshot: Snapshot<Types>,
        key: Value,
    ) -> Result<MerklePath<E, I, T>, Self::Error>;
}

#[async_trait]
pub trait UpdateStateStorage<
    Types: NodeType,
    Proof: MembershipProof<E, I, T> + Send + Sync,
    E: Element + Send + Sync,
    I: Index + Send + Sync,
    T: NodeValue + Send + Sync,
>
{
    type Error: std::error::Error + std::fmt::Debug + Send + Sync + 'static;
    async fn insert_nodes(
        &mut self,
        name: String,
        proof: Proof,
        path: Vec<usize>,
        elem: E,
        leaf: Leaf<Types>,
    ) -> Result<(), Self::Error>;
}
#[async_trait]
impl<Types, Proof, E, I, T> UpdateStateStorage<Types, Proof, E, I, T> for SqlStorage
where
    Proof: MembershipProof<E, I, T> + Send + Sync + 'static,
    E: Element + Send + Sync + Display + Serialize + 'static,
    I: Index + Send + Sync + Display + Serialize + 'static,
    T: NodeValue + Send + Sync + CanonicalSerialize + 'static,
    Types: NodeType,
    Header<Types>: QueryableHeader<Types>,
{
    type Error = QueryError;
    async fn insert_nodes(
        &mut self,
        name: String,
        proof: Proof,
        traversal_path: Vec<usize>,
        elem: E,
        leaf: Leaf<Types>,
    ) -> QueryResult<()> {
        // Use height from proof and first entry in proof is always leaf so remove leaf param?
        let created = leaf.get_height() as i64;
        let pos = proof.index().clone();
        let commit = leaf.commit();
        let commit: &[u8; 32] = commit.as_ref();
        let ltree_path = LTree::from(traversal_path.iter());

        let mut txn = self.transaction().await?;

        let insert_hash = txn
            .query_one(
                "INSERT INTO hash(value) VALUES ($1) 
                ON CONFLICT (value) DO UPDATE SET value = excluded.value 
                RETURNING id",
                &[&commit.to_vec()],
            )
            .await?;
        let hash_id: i32 = insert_hash.get(0);

        txn.upsert(
            &name,
            ["pos", "created", "hash_id", "index", "entry"],
            ["pos", "created"],
            [[
                sql_param(&ltree_path),
                sql_param(&(created)),
                sql_param(&hash_id),
                sql_param(&serde_json::to_value(&pos).map_err(ParseError::Serde)?),
                sql_param(&serde_json::to_value(&elem).map_err(ParseError::Serde)?),
            ]],
        )
        .await?;

        for (index, node) in proof.merkle_path().iter().enumerate() {
            match node {
                MerkleNode::Branch { value, children } => {
                    // Get hash
                    let mut writer = [0_u8; 32];
                    value
                        .serialize_compressed(&mut writer[..])
                        .map_err(ParseError::Serialize)?;

                    let row = txn
                        .query_one(
                            "INSERT INTO hash(value) VALUES ($1) 
                            ON CONFLICT (value) DO UPDATE SET value = excluded.value
                            RETURNING id",
                            &[&writer.to_vec()],
                        )
                        .await?;
                    let hash_id: i32 = row.get(0);

                    // Get children
                    let mut children_values = Vec::new();
                    let mut children_bitmap = BitVec::new();
                    for child in children {
                        let child = child.as_ref();
                        match child {
                            MerkleNode::Empty => {
                                children_bitmap.push(false);
                            }
                            MerkleNode::Branch { .. } => (),
                            // Insert leaf too just in case?
                            MerkleNode::Leaf { .. } => (),
                            MerkleNode::ForgettenSubtree { value } => {
                                let mut writer = [0_u8; 32];
                                value
                                    .serialize_compressed(&mut writer[..])
                                    .map_err(ParseError::Serialize)?;

                                let r = txn
                                    .query_one(
                                        "INSERT INTO hash(value) VALUES ($1) 
                                        ON CONFLICT (value) DO UPDATE SET value = excluded.value 
                                        RETURNING id",
                                        &[&writer.to_vec()],
                                    )
                                    .await?;
                                let hash_id: i32 = r.get(0);

                                children_values.push(hash_id);
                                children_bitmap.push(true);
                            }
                        }
                    }

                    // insert internal node
                    txn.upsert(
                        "node",
                        ["pos", "created", "hash_id", "children", "children_bitmap"],
                        ["pos", "created"],
                        [[
                            sql_param(&LTree::from(traversal_path.iter().skip(index + 1))),
                            sql_param(&(created)),
                            sql_param(&hash_id),
                            sql_param(&children_values),
                            sql_param(&children_bitmap),
                        ]],
                    )
                    .await?;
                }

                _ => continue,
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! parse_field {
    ($row:expr, $pos:expr) => {{
        let result = $row
            .try_get::<_, Value>($pos)
            .map_err(|err| QueryError::Error {
                message: format!("error extracting {} from query results: {}", $pos, err),
            })
            .and_then(|val| {
                serde_json::from_value(val).map_err(|err| QueryError::Error {
                    message: format!("malformed {}: {}", $pos, err),
                })
            })?;

        result
    }};
}

#[derive(Debug, Clone)]
pub struct Node {
    pub pos: String,
    pub created: i64,
    pub hash_id: i32,
    pub children: Option<Vec<i32>>,
    pub children_bitmap: Option<BitVec>,
    pub index: Value,
    pub entry: Value,
}

impl TryFrom<Row> for Node {
    type Error = QueryError;
    fn try_from(row: Row) -> QueryResult<Self> {
        Ok(Self {
            pos: parse_field!(row, "pos"),
            created: parse_field!(row, "created"),
            hash_id: parse_field!(row, "hash_id"),
            children: parse_field!(row, "children"),
            children_bitmap: parse_field!(row, "children_bitmap"),
            index: parse_field!(row, "index"),
            entry: parse_field!(row, "entry"),
        })
    }
}

pub struct Hash {
    pub id: i32,
    pub value: [u8; 32],
}

impl TryFrom<Row> for Hash {
    type Error = QueryError;
    fn try_from(row: Row) -> QueryResult<Self> {
        Ok(Self {
            id: parse_field!(row, "id"),
            value: parse_field!(row, "value"),
        })
    }
}

#[async_trait]
impl<Types: NodeType> MerklizedStateDataSource<Types> for SqlStorage {
    type Error = QueryError;

    async fn get_path<
        E: Element + Send + DeserializeOwned,
        I: Index + Send + ToTraversalPath<A> + DeserializeOwned,
        A: Unsigned,
        T: NodeValue + Send + CanonicalDeserialize,
    >(
        &self,
        state_type: &'static str,
        tree_height: usize,
        header_state_commitment_field: &'static str,
        snapshot: Snapshot<Types>,
        key: Value,
    ) -> QueryResult<MerklePath<E, I, T>> {
        let index: I = serde_json::from_value(key).map_err(ParseError::Serde)?;
        let path = I::to_traversal_path(&index, tree_height)
            .into_iter()
            .map(|x| x as i64)
            .collect::<Vec<_>>();

        let created = match snapshot {
            Snapshot::<Types>::Commit(commit) => {
                let commit: &[u8; 32] = commit.as_ref();

                let query = self
                    .query_one(
                        &format!(
                            "SELECT n.created FROM {state_type} n INNER JOIN Header h on 
                            n.created = h.height where h.data->>{header_state_commitment_field}"
                        ),
                        &[&commit.to_vec()],
                    )
                    .await?;

                query.get(0)
            }

            Snapshot::<Types>::Index(created) => created as i64,
        };

        self.query_one(
            "SELECT 1 from HEADER where height = $1",
            [sql_param(&created)],
        )
        .await
        .map_err(|err| QueryError::Error {
            message: format!("Header for {created} not found! {err:?}"),
        })?;

        let nodes = self
            .query(
                "SELECT DISTINCT ON (pos) *
                    FROM $1
                    WHERE pos @> $2 and created <= $3
                    ORDER BY pos, created;",
                [
                    sql_param(&state_type),
                    sql_param(&LTree::from(path.iter())),
                    sql_param(&created),
                ],
            )
            .await?;

        let nodes = nodes
            .map(|res| Node::try_from(res?))
            .collect::<Result<Vec<_>, _>>()
            .await?;

        let mut hash_ids = HashSet::new();

        nodes.iter().for_each(|n| {
            hash_ids.insert(n.hash_id);
            if let Some(children) = &n.children {
                hash_ids.extend(children);
            }
        });

        let hashes_query = self
            .query(
                "SELECT value, id FROM hash WHERE id = ANY($1)",
                [sql_param(&hash_ids.into_iter().collect::<Vec<i32>>())],
            )
            .await?;

        let hashes: HashMap<i32, [u8; 32]> = hashes_query
            .map(|row| Hash::try_from(row?).map(|h| (h.id, h.value)))
            .collect::<QueryResult<HashMap<_, _>>>()
            .await?;

        let mut proof_path = Vec::new();

        for Node {
            hash_id,
            children,
            children_bitmap,
            index,
            entry,
            ..
        } in nodes.iter()
        {
            {
                let value = hashes.get(hash_id).ok_or(QueryError::NotFound)?;
                if let (Some(children), Some(children_bitmap)) = (children, children_bitmap) {
                    let child_nodes = children
                        .iter()
                        .zip(children_bitmap.iter())
                        .map(|(child, bit)| {
                            if bit {
                                let value = hashes.get(child).ok_or(QueryError::NotFound)?;
                                Ok(Arc::new(MerkleNode::ForgettenSubtree {
                                    value: T::deserialize_compressed(value.as_slice())
                                        .map_err(ParseError::Deserialize)?,
                                }))
                            } else {
                                Ok(Arc::new(MerkleNode::Empty))
                            }
                        })
                        .collect::<QueryResult<Vec<_>>>()?;
                    proof_path.push(MerkleNode::Branch {
                        value: T::deserialize_compressed(value.as_slice())
                            .map_err(ParseError::Deserialize)?,
                        children: child_nodes,
                    });
                } else {
                    proof_path.push(MerkleNode::<E, I, T>::Leaf {
                        value: T::deserialize_compressed(value.as_slice())
                            .map_err(ParseError::Deserialize)?,
                        pos: serde_json::from_value(index.clone()).map_err(ParseError::Serde)?,
                        elem: serde_json::from_value(entry.clone()).map_err(ParseError::Serde)?,
                    });
                }
            }
        }

        Ok(proof_path)
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

enum ParseError {
    Serde(serde_json::Error),
    Deserialize(SerializationError),
    Serialize(SerializationError),
}

impl From<ParseError> for QueryError {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::Serde(err) => Self::Error {
                message: format!("failed to parse value {err:?}"),
            },
            ParseError::Deserialize(err) => Self::Error {
                message: format!("failed to failed to deserialize {err:?}"),
            },
            ParseError::Serialize(err) => Self::Error {
                message: format!("failed to failed to serialize {err:?}"),
            },
        }
    }
}

// These tests run the `postgres` Docker image, which doesn't work on Windows.
#[cfg(all(test, not(target_os = "windows")))]
mod test {

    use hotshot_example_types::state_types::TestInstanceState;
    use jf_primitives::merkle_tree::{
        prelude::LightWeightSHA3MerkleTree, AppendableMerkleTreeScheme, MerkleTreeScheme,
    };
    use typenum::U3;

    use super::{testing::TmpDb, *};
    use crate::{
        data_source::VersionedDataSource,
        node::LeafQueryData,
        testing::{mocks::MockTypes, setup_test},
    };
    #[async_std::test]
    async fn test_insert_nodes() {
        setup_test();

        let db = TmpDb::init().await;
        let port = db.port();
        let host = &db.host();

        let migration = vec![Migration::unapplied(
            "V11__create_node_table.sql",
            "CREATE TABLE hash
                    (
                        id SERIAL PRIMARY KEY,
                        value BYTEA  NOT NULL UNIQUE
                    );

                    CREATE TABLE node
                    (
                        pos LTREE NOT NULL, 
                        created BIGINT NOT NULL,
                        hash_id INT NOT NULL REFERENCES hash (id),
                        children INT[],
                        children_bitmap BIT(3),
                        index JSONB,
                        entry JSONB 
                    );
                    ALTER TABLE node ADD CONSTRAINT node_pk PRIMARY KEY (pos, created);
                    CREATE INDEX node_path ON node USING GIST (pos);",
        )
        .unwrap()];
        let cfg = Config::default()
            .user("postgres")
            .password("password")
            .host(host)
            .port(port)
            .migrations(migration);

        let mut storage = SqlStorage::connect(cfg).await.unwrap();
        let leaf = LeafQueryData::<MockTypes>::genesis(&TestInstanceState {});
        let leaf = leaf.leaf();

        let mut block_merkle_tree = LightWeightSHA3MerkleTree::<usize>::new(3);

        for i in 0..8 {
            block_merkle_tree.push(i).unwrap();
        }

        let index = 7;
        let (elem, proof) = block_merkle_tree.lookup(index).expect_ok().unwrap();
        let traversal_path = <u64 as ToTraversalPath<U3>>::to_traversal_path(&index, 3);

        tracing::info!("Proof {:?}", &proof);

        storage
            .insert_nodes(
                "node".to_owned(),
                proof,
                traversal_path,
                elem.clone(),
                leaf.clone(),
            )
            .await
            .expect("failed to insert nodes");

        storage.commit().await.expect("failed to commit");
    }
}
