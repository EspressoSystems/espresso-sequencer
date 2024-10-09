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

//! Merklized state storage implementation for a database query engine.

use super::{
    postgres::{types::ToSql, Row},
    sql_param, Transaction,
};
use crate::{
    merklized_state::{
        MerklizedState, MerklizedStateDataSource, MerklizedStateHeightPersistence, Snapshot,
    },
    QueryError, QueryResult,
};
use ark_serialize::{CanonicalDeserialize, SerializationError};
use async_std::sync::Arc;
use async_trait::async_trait;
use bit_vec::BitVec;
use futures::stream::{StreamExt, TryStreamExt};
use hotshot_types::traits::node_implementation::NodeType;
use itertools::Itertools;
use jf_merkle_tree::{
    prelude::{MerkleNode, MerkleProof},
    DigestAlgorithm, MerkleCommitment, ToTraversalPath,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{self, Display, Formatter},
};

#[async_trait]
impl<'a, Types, State, const ARITY: usize> MerklizedStateDataSource<Types, State, ARITY>
    for Transaction<'a>
where
    Types: NodeType,
    State: MerklizedState<Types, ARITY> + 'static,
{
    /// Retreives a Merkle path from the database
    async fn get_path(
        &self,
        snapshot: Snapshot<Types, State, ARITY>,
        key: State::Key,
    ) -> QueryResult<MerkleProof<State::Entry, State::Key, State::T, ARITY>> {
        let state_type = State::state_type();
        let tree_height = State::tree_height();

        // Get the traversal path of the index
        let traversal_path = State::Key::to_traversal_path(&key, tree_height);
        let (created, merkle_commitment) = self.snapshot_info(snapshot).await?;

        // Get all the nodes in the path to the index.
        // Order by pos DESC is to return nodes from the leaf to the root

        let (params, stmt) = build_get_path_query(state_type, traversal_path.clone(), created);

        let nodes = self.query(&stmt, params).await?;

        let nodes: Vec<_> = nodes.map(|res| Node::try_from(res?)).try_collect().await?;

        // insert all the hash ids to a hashset which is used to query later
        // HashSet is used to avoid duplicates
        let mut hash_ids = HashSet::new();
        nodes.iter().for_each(|n| {
            hash_ids.insert(n.hash_id);
            if let Some(children) = &n.children {
                hash_ids.extend(children);
            }
        });

        // Find all the hash values and create a hashmap
        // Hashmap will be used to get the hash value of the nodes children and the node itself.
        let hashes_query = self
            .query(
                "SELECT * FROM hash WHERE id = ANY( $1)",
                [sql_param(&hash_ids.into_iter().collect::<Vec<i32>>())],
            )
            .await?;
        let hashes: HashMap<_, _> = hashes_query
            .map(|row| HashTableRow::try_from(row?).map(|h| (h.id, h.value)))
            .try_collect()
            .await?;

        let mut proof_path = VecDeque::with_capacity(State::tree_height());
        for Node {
            hash_id,
            children,
            children_bitvec,
            index,
            entry,
            ..
        } in nodes.iter()
        {
            {
                let value = hashes.get(hash_id).ok_or(QueryError::Error {
                    message: format!("node's value references non-existent hash {hash_id}"),
                })?;
                match (children, children_bitvec, index, entry) {
                    // If the row has children then its a branch
                    (Some(children), Some(children_bitvec), None, None) => {
                        let mut children = children.iter();

                        // Reconstruct the Children MerkleNodes from storage.
                        // Children bit_vec is used to create forgotten  or empty node
                        let child_nodes = children_bitvec
                            .iter()
                            .map(|bit| {
                                if bit {
                                    let hash_id = children.next().ok_or(QueryError::Error {
                                        message: "node has fewer children than set bits".into(),
                                    })?;
                                    let value = hashes.get(hash_id).ok_or(QueryError::Error {
                                        message: format!(
                                            "node's child references non-existent hash {hash_id}"
                                        ),
                                    })?;
                                    Ok(Arc::new(MerkleNode::ForgettenSubtree {
                                        value: State::T::deserialize_compressed(value.as_slice())
                                            .map_err(ParseError::Deserialize)?,
                                    }))
                                } else {
                                    Ok(Arc::new(MerkleNode::Empty))
                                }
                            })
                            .collect::<QueryResult<Vec<_>>>()?;
                        // Use the Children merkle nodes to reconstruct the branch node
                        proof_path.push_back(MerkleNode::Branch {
                            value: State::T::deserialize_compressed(value.as_slice())
                                .map_err(ParseError::Deserialize)?,
                            children: child_nodes,
                        });
                    }
                    // If it has an entry, it's a leaf
                    (None, None, Some(index), Some(entry)) => {
                        proof_path.push_back(MerkleNode::Leaf {
                            value: State::T::deserialize_compressed(value.as_slice())
                                .map_err(ParseError::Deserialize)?,
                            pos: serde_json::from_value(index.clone())
                                .map_err(ParseError::Serde)?,
                            elem: serde_json::from_value(entry.clone())
                                .map_err(ParseError::Serde)?,
                        });
                    }
                    // Otherwise, it's empty.
                    (None, None, Some(_), None) => {
                        proof_path.push_back(MerkleNode::Empty);
                    }
                    _ => {
                        return Err(QueryError::Error {
                            message: "Invalid type of merkle node found".to_string(),
                        });
                    }
                }
            }
        }

        // Reconstruct the merkle commitment from the path
        let init = if let Some(MerkleNode::Leaf { value, .. }) = proof_path.front() {
            *value
        } else {
            // If the path ends in a branch (or, as a special case, if the path and thus the entire
            // tree is empty), we are looking up an entry that is not present in the tree. We always
            // store all the nodes on all the paths to all the entries in the tree, so the only
            // nodes we could be missing are empty nodes from unseen entries. Thus, we can
            // reconstruct what the path should be by prepending empty nodes.
            while proof_path.len() <= State::tree_height() {
                proof_path.push_front(MerkleNode::Empty);
            }
            State::T::default()
        };
        let commitment_from_path = traversal_path
            .iter()
            .zip(proof_path.iter().skip(1))
            .try_fold(init, |val, (branch, node)| -> QueryResult<State::T> {
                match node {
                    MerkleNode::Branch { value: _, children } => {
                        let data = children
                            .iter()
                            .map(|node| match node.as_ref() {
                                MerkleNode::ForgettenSubtree { value } => Ok(*value),
                                MerkleNode::Empty => Ok(State::T::default()),
                                _ => Err(QueryError::Error {
                                    message: "Invalid child node".to_string(),
                                }),
                            })
                            .collect::<QueryResult<Vec<_>>>()?;

                        if data[*branch] != val {
                            // This can only happen if data is missing: we have an old version of
                            // one of the nodes in the path, which is why it is not matching up with
                            // its parent.
                            tracing::warn!(
                                ?key,
                                parent = ?data[*branch],
                                child = ?val,
                                branch = %*branch,
                                %created,
                                %merkle_commitment,
                                "missing data in merklized state; parent-child mismatch",
                            );
                            return Err(QueryError::Missing);
                        }

                        State::Digest::digest(&data).map_err(|err| QueryError::Error {
                            message: format!("failed to update digest: {err:#}"),
                        })
                    }
                    MerkleNode::Empty => Ok(init),
                    _ => Err(QueryError::Error {
                        message: "Invalid type of Node in the proof".to_string(),
                    }),
                }
            })?;

        if commitment_from_path != merkle_commitment.digest() {
            return Err(QueryError::Error {
                message:
                    format!("Commitment calcuated from merkle path ({commitment_from_path:?}) does not match the commitment in the header ({:?})", merkle_commitment.digest()),
            });
        }

        Ok(MerkleProof {
            pos: key,
            proof: proof_path.into(),
        })
    }
}

#[async_trait]
impl<'a> MerklizedStateHeightPersistence for Transaction<'a> {
    async fn get_last_state_height(&self) -> QueryResult<usize> {
        let row = self
            .query_opt_static("SELECT * from last_merklized_state_height")
            .await?;

        let height = row.map(|r| r.get::<_, i64>("height") as usize);

        Ok(height.unwrap_or(0))
    }
}

impl<'a> Transaction<'a> {
    /// Get information identifying a [`Snapshot`].
    ///
    /// If the given snapshot is known to the database, this function returns
    /// * The block height at which the snapshot was created
    /// * A digest of the Merkle commitment to the snapshotted state
    async fn snapshot_info<Types, State, const ARITY: usize>(
        &self,
        snapshot: Snapshot<Types, State, ARITY>,
    ) -> QueryResult<(i64, State::Commit)>
    where
        Types: NodeType,
        State: MerklizedState<Types, ARITY>,
    {
        let header_state_commitment_field = State::header_state_commitment_field();

        let (created, commit) = match snapshot {
            Snapshot::Commit(commit) => {
                // Get the block height using the merkle commitment. It is possible that multiple
                // headers will have the same state commitment. In this case we don't care which
                // height we get, since any query against equivalent states will yield equivalent
                // results, regardless of which block the state is from. Thus, we can make this
                // query fast with `LIMIT 1` and no `ORDER BY`.
                let query = self
                    .query_one(
                        &format!(
                            "SELECT height
                           FROM header
                          WHERE {header_state_commitment_field} = $1
                          LIMIT 1"
                        ),
                        &[&commit.to_string()],
                    )
                    .await?;

                (query.get(0), commit)
            }
            Snapshot::Index(created) => {
                let created = created as i64;
                let row = self
                    .query_one(
                        &format!(
                            "SELECT {header_state_commitment_field} AS root_commmitment
                         FROM header
                         WHERE height = $1
                         LIMIT 1"
                        ),
                        [sql_param(&created)],
                    )
                    .await?;
                let commit: String = row.get(0);
                let commit = serde_json::from_value(commit.into()).map_err(ParseError::Serde)?;
                (created, commit)
            }
        };

        // Make sure the requested snapshot is up to date.
        let height = self.get_last_state_height().await?;
        if height < (created as usize) {
            return Err(QueryError::NotFound);
        }

        Ok((created, commit))
    }
}

/// Represents a Hash table row
pub(crate) struct HashTableRow {
    /// Hash id to be used by the state table to save space
    id: i32,
    /// hash value
    value: Vec<u8>,
}

impl HashTableRow {
    // TODO: create a generic upsert function with retries that returns the column
    pub(crate) fn build_batch_insert(hashes: &[Vec<u8>]) -> (Vec<&(dyn ToSql + Sync)>, String) {
        let len = hashes.len();
        let params: Vec<_> = hashes
            .iter()
            .flat_map(|c| [c as &(dyn ToSql + Sync)])
            .collect();
        let stmt = format!(
        "INSERT INTO hash(value) values {} ON CONFLICT (value) DO UPDATE SET value = EXCLUDED.value returning *",
        (1..len+1)
            .format_with(", ", |v, f| { f(&format_args!("(${v})")) }),
    );

        (params, stmt)
    }
}

// Parse a row to a HashTableRow
impl TryFrom<Row> for HashTableRow {
    type Error = QueryError;
    fn try_from(row: Row) -> QueryResult<Self> {
        Ok(Self {
            id: row.try_get(0).map_err(|e| QueryError::Error {
                message: format!("failed to get column id {e}"),
            })?,
            value: row.try_get(1).map_err(|e| QueryError::Error {
                message: format!("failed to get column value {e}"),
            })?,
        })
    }
}

// parsing errors
#[derive(Debug)]
pub(crate) enum ParseError {
    Serde(serde_json::Error),
    Deserialize(SerializationError),
    Serialize(SerializationError),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Serde(err) => {
                write!(f, "failed to parse value {err:?}")
            }
            Self::Deserialize(err) => {
                write!(f, "failed to deserialize {err:?}")
            }
            Self::Serialize(err) => {
                write!(f, "failed to serialize {err:?}")
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl From<ParseError> for QueryError {
    fn from(value: ParseError) -> Self {
        Self::Error {
            message: value.to_string(),
        }
    }
}

// Represents a row in a state table
#[derive(Debug, Default, Clone)]
pub(crate) struct Node {
    pub(crate) path: Vec<i32>,
    pub(crate) created: i64,
    pub(crate) hash_id: i32,
    pub(crate) children: Option<Vec<i32>>,
    pub(crate) children_bitvec: Option<BitVec>,
    pub(crate) index: Option<serde_json::Value>,
    pub(crate) entry: Option<serde_json::Value>,
}

impl Node {
    pub(crate) fn build_batch_insert<'a>(
        name: &'a str,
        nodes: &'a [Self],
    ) -> (Vec<&'a (dyn ToSql + Sync)>, String) {
        let params: Vec<&(dyn ToSql + Sync)> = nodes
            .iter()
            .flat_map(|n| {
                [
                    &n.path as &(dyn ToSql + Sync),
                    &n.created,
                    &n.hash_id,
                    &n.children,
                    &n.children_bitvec,
                    &n.index,
                    &n.entry,
                ]
            })
            .collect();

        let stmt = format!(
                "INSERT INTO {name} (path, created, hash_id, children, children_bitvec, index, entry) values {} ON CONFLICT (path, created)
                DO UPDATE SET hash_id = EXCLUDED.hash_id, children = EXCLUDED.children, children_bitvec = EXCLUDED.children_bitvec,
                index = EXCLUDED.index, entry = EXCLUDED.entry RETURNING path",
                (1..params.len()+1)
                .tuples()
                    .format_with(", ", |(path, created, id, children, bitmap, i, e), f|
                    { f(&format_args!("(${path}, ${created}, ${id}, ${children}, ${bitmap}, ${i}, ${e})")) }),
            );

        (params, stmt)
    }
}

// Parse a Row to a Node
impl TryFrom<Row> for Node {
    type Error = QueryError;
    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            path: row.try_get(0).map_err(|e| QueryError::Error {
                message: format!("failed to get column path: {e}"),
            })?,
            created: row.try_get(1).map_err(|e| QueryError::Error {
                message: format!("failed to get column created: {e}"),
            })?,
            hash_id: row.try_get(2).map_err(|e| QueryError::Error {
                message: format!("failed to get column hash_id: {e}"),
            })?,
            children: row.try_get(3).map_err(|e| QueryError::Error {
                message: format!("failed to get column children: {e}"),
            })?,
            children_bitvec: row.try_get(4).map_err(|e| QueryError::Error {
                message: format!("failed to get column children bitmap: {e}"),
            })?,
            index: row.try_get(5).map_err(|e| QueryError::Error {
                message: format!("failed to get column index: {e}"),
            })?,
            entry: row.try_get(6).map_err(|e| QueryError::Error {
                message: format!("failed to get column entry: {e}"),
            })?,
        })
    }
}

fn build_get_path_query(
    table: &'static str,
    traversal_path: Vec<usize>,
    created: i64,
) -> (Vec<Box<(dyn ToSql + Send + Sync)>>, String) {
    let mut traversal_path = traversal_path.into_iter().map(|x| x as i32);

    // Since the 'created' parameter is common to all queries,
    // we place it at the first position in the 'params' vector.
    // We iterate through the path vector skipping the first element after each iteration
    let mut params: Vec<Box<(dyn ToSql + Send + Sync)>> = vec![Box::new(created)];
    let len = traversal_path.len();
    let mut queries = Vec::new();

    for i in 0..=len {
        let node_path = traversal_path.clone().rev().collect::<Vec<_>>();

        let query = format!(
            "(SELECT * FROM {table} WHERE path = ${} AND created <= $1 ORDER BY created DESC LIMIT 1)",
            i + 2
        );

        queries.push(query);
        params.push(Box::new(node_path));
        traversal_path.next();
    }

    let mut final_query: String = queries.join(" UNION ");
    final_query.push_str("ORDER BY path DESC");
    (params, final_query)
}

#[cfg(test)]
mod test {
    use jf_merkle_tree::{
        universal_merkle_tree::UniversalMerkleTree, LookupResult, MerkleTreeScheme,
        UniversalMerkleTreeScheme,
    };
    use rand::{seq::IteratorRandom, RngCore};

    use super::*;
    use crate::{
        data_source::{
            storage::sql::{query::sql_param, testing::TmpDb, *},
            VersionedDataSource,
        },
        merklized_state::UpdateStateData,
        testing::{
            mocks::{MockMerkleTree, MockTypes},
            setup_test,
        },
    };

    #[async_std::test]
    async fn test_merklized_state_storage() {
        // In this test we insert some entries into the tree and update the database
        // Each entry's merkle path is compared with the path from the tree
        setup_test();

        let db = TmpDb::init().await;
        let storage = SqlStorage::connect(db.config()).await.unwrap();

        // define a test tree
        let mut test_tree: UniversalMerkleTree<_, _, _, 8, _> =
            MockMerkleTree::new(MockMerkleTree::tree_height());
        let block_height = 1;

        // insert some entries into the tree and the header table
        // Header table is used the get_path query to check if the header exists for the block height.
        let mut tx = storage.write().await.unwrap();
        for i in 0..27 {
            test_tree.update(i, i).unwrap();

            // data field of the header
            let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()});
            tx
                .query_opt(
                    "INSERT INTO HEADER(height, hash, payload_hash, timestamp, data) VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                    [
                        sql_param(&(block_height as i64)),
                        sql_param(&format!("randomHash{i}")),
                        sql_param(&test_data),
                    ],
                )
                .await
                .unwrap();
            // proof for the index from the tree
            let (_, proof) = test_tree.lookup(i).expect_ok().unwrap();
            // traversal path for the index.
            let traversal_path =
                <usize as ToTraversalPath<8>>::to_traversal_path(&i, test_tree.height());

            UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
                &mut tx,
                proof.clone(),
                traversal_path.clone(),
                block_height as u64,
            )
            .await
            .expect("failed to insert nodes");
        }
        // update saved state height
        UpdateStateData::<_, MockMerkleTree, 8>::set_last_state_height(&mut tx, block_height)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        //Get the path and check if it matches the lookup
        for i in 0..27 {
            // Query the path for the index
            let tx = storage.read().await.unwrap();
            let merkle_path = tx
                .get_path(
                    Snapshot::<_, MockMerkleTree, 8>::Index(block_height as u64),
                    i,
                )
                .await
                .unwrap();

            let (_, proof) = test_tree.lookup(i).expect_ok().unwrap();

            tracing::info!("merkle path {:?}", merkle_path);

            // merkle path from the storage should match the path from test tree
            assert_eq!(merkle_path, proof.clone(), "merkle paths mismatch");
        }

        // Get the proof of index 0 with bh = 1
        let (_, proof_bh_1) = test_tree.lookup(0).expect_ok().unwrap();
        // Inserting Index 0 again with created (bh) = 2
        // Our database should then have 2 versions of this leaf node
        // Update the node so that proof is also updated
        test_tree.update(0, 99).unwrap();
        // Also update the merkle commitment in the header

        // data field of the header
        let mut tx = storage.write().await.unwrap();
        let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()});
        tx
            .query_opt(
                "INSERT INTO HEADER(height, hash, payload_hash, timestamp, data) VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                [
                    sql_param(&(2_i64)),
                    sql_param(&"randomstring"),
                    sql_param(&test_data),
                ],
            )
            .await
            .unwrap();
        let (_, proof_bh_2) = test_tree.lookup(0).expect_ok().unwrap();
        // traversal path for the index.
        let traversal_path =
            <usize as ToTraversalPath<8>>::to_traversal_path(&0, test_tree.height());
        // Update storage to insert a new version of this code

        UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
            &mut tx,
            proof_bh_2.clone(),
            traversal_path.clone(),
            2,
        )
        .await
        .expect("failed to insert nodes");
        // update saved state height
        UpdateStateData::<_, MockMerkleTree, 8>::set_last_state_height(&mut tx, 2)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        let node_path = traversal_path
            .into_iter()
            .rev()
            .map(|n| n as i32)
            .collect::<Vec<_>>();

        // Find all the nodes of Index 0 in table
        let rows = storage
            .read()
            .await
            .unwrap()
            .query(
                "SELECT * from test_tree where path = $1 ORDER BY created",
                [sql_param(&node_path)],
            )
            .await
            .unwrap();

        let nodes: Vec<_> = rows
            .map(|res| Node::try_from(res.unwrap()))
            .try_collect()
            .await
            .unwrap();
        // There should be only 2 versions of this node
        assert!(nodes.len() == 2, "incorrect number of nodes");
        assert_eq!(nodes[0].created, 1, "wrong block height");
        assert_eq!(nodes[1].created, 2, "wrong block height");

        // Now we can have two snapshots for Index 0
        // One with created = 1 and other with 2
        // Query snapshot with created = 2

        let path_with_bh_2 = storage
            .read()
            .await
            .unwrap()
            .get_path(Snapshot::<_, MockMerkleTree, 8>::Index(2), 0)
            .await
            .unwrap();

        assert_eq!(path_with_bh_2, proof_bh_2);
        let path_with_bh_1 = storage
            .read()
            .await
            .unwrap()
            .get_path(Snapshot::<_, MockMerkleTree, 8>::Index(1), 0)
            .await
            .unwrap();
        assert_eq!(path_with_bh_1, proof_bh_1);
    }

    #[async_std::test]
    async fn test_merklized_state_non_membership_proof() {
        // This test updates the Merkle tree with a new entry and inserts the corresponding Merkle nodes into the database with created = 1.
        // A Merkle node is then deleted from the tree.
        // The database is then updated to reflect the deletion of the entry with a created (block height) of 2
        // As the leaf node becomes a non-member, we do a universal lookup to obtain its non-membership proof path.
        // It is expected that the path retrieved from the tree matches the path obtained from the database.
        setup_test();

        let db = TmpDb::init().await;
        let storage = SqlStorage::connect(db.config()).await.unwrap();

        // define a test tree
        let mut test_tree = MockMerkleTree::new(MockMerkleTree::tree_height());
        let block_height = 1;
        //insert an entry into the tree
        test_tree.update(0, 0).unwrap();
        let commitment = test_tree.commitment();

        let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(commitment).unwrap()});
        // insert the header with merkle commitment
        let mut tx = storage.write().await.unwrap();
        tx
                .query_opt(
                    "INSERT INTO HEADER(height, hash, payload_hash, timestamp, data) VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                    [
                        sql_param(&(block_height as i64)),
                        sql_param(&"randomString"),
                        sql_param(&test_data),
                    ],
                )
                .await
                .unwrap();
        // proof for the index from the tree
        let (_, proof_before_remove) = test_tree.lookup(0).expect_ok().unwrap();
        // traversal path for the index.
        let traversal_path =
            <usize as ToTraversalPath<8>>::to_traversal_path(&0, test_tree.height());
        // insert merkle nodes
        UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
            &mut tx,
            proof_before_remove.clone(),
            traversal_path.clone(),
            block_height as u64,
        )
        .await
        .expect("failed to insert nodes");
        // update saved state height
        UpdateStateData::<_, MockMerkleTree, 8>::set_last_state_height(&mut tx, block_height)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        // the path from the db and and tree should match
        let merkle_path = storage
            .read()
            .await
            .unwrap()
            .get_path(
                Snapshot::<_, MockMerkleTree, 8>::Index(block_height as u64),
                0,
            )
            .await
            .unwrap();

        // merkle path from the storage should match the path from test tree
        assert_eq!(
            merkle_path,
            proof_before_remove.clone(),
            "merkle paths mismatch"
        );

        //Deleting the index 0
        test_tree.remove(0).expect("failed to delete index 0 ");

        // Update the database with the proof
        // Created = 2 in this case
        let proof_after_remove = test_tree.universal_lookup(0).expect_not_found().unwrap();

        let mut tx = storage.write().await.unwrap();
        UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
            &mut tx,
            proof_after_remove.clone(),
            traversal_path.clone(),
            2_u64,
        )
        .await
        .expect("failed to insert nodes");
        // Insert the new header
        tx
        .query_opt(
            "INSERT INTO HEADER(height, hash, payload_hash, timestamp, data) VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
            [
                sql_param(&2_i64),
                sql_param(&"randomString2"),
                sql_param(&serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()})),
            ],
        )
        .await
        .unwrap();
        // update saved state height
        UpdateStateData::<_, MockMerkleTree, 8>::set_last_state_height(&mut tx, 2)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        // Get non membership proof
        let non_membership_path = storage
            .read()
            .await
            .unwrap()
            .get_path(Snapshot::<_, MockMerkleTree, 8>::Index(2_u64), 0)
            .await
            .unwrap();
        // Assert that the paths from the db and the tree are equal
        assert_eq!(
            non_membership_path, proof_after_remove,
            "merkle paths dont match"
        );

        // Query the membership proof i.e proof with created = 1
        // This proof should be equal to the proof before deletion
        // Assert that the paths from the db and the tree are equal

        let proof_bh_1 = storage
            .read()
            .await
            .unwrap()
            .get_path(Snapshot::<_, MockMerkleTree, 8>::Index(1_u64), 0)
            .await
            .unwrap();
        assert_eq!(proof_bh_1, proof_before_remove, "merkle paths dont match");
    }

    #[async_std::test]
    async fn test_merklized_state_non_membership_proof_unseen_entry() {
        setup_test();

        let db = TmpDb::init().await;
        let storage = SqlStorage::connect(db.config()).await.unwrap();

        // define a test tree
        let mut test_tree = MockMerkleTree::new(MockMerkleTree::tree_height());

        // For each case (where the root is empty, a leaf, and a branch) test getting a
        // non-membership proof for an entry node the database has never seen.
        for i in 0..=2 {
            tracing::info!(i, ?test_tree, "testing non-membership proof");
            let mut tx = storage.write().await.unwrap();

            // Insert a dummy header
            tx
                .query_opt(
                    "INSERT INTO HEADER(height, hash, payload_hash, timestamp, data) VALUES ($1, $2, 't', 0, $3)",
                    [
                        sql_param(&(i as i64)),
                        sql_param(&format!("hash{i}")),
                        sql_param(&serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()})),
                    ],
                )
                .await
                .unwrap();
            // update saved state height
            UpdateStateData::<_, MockMerkleTree, 8>::set_last_state_height(&mut tx, i)
                .await
                .unwrap();
            tx.commit().await.unwrap();

            // get a non-membership proof for a never-before-seen node.
            let proof = storage
                .read()
                .await
                .unwrap()
                .get_path(
                    Snapshot::<MockTypes, MockMerkleTree, 8>::Index(i as u64),
                    100,
                )
                .await
                .unwrap();
            assert_eq!(proof.elem(), None);
            assert!(test_tree.non_membership_verify(100, proof).unwrap());

            // insert an additional node into the tree.
            test_tree.update(i, i).unwrap();
            let (_, proof) = test_tree.lookup(i).expect_ok().unwrap();
            let traversal_path = ToTraversalPath::<8>::to_traversal_path(&i, test_tree.height());
            let mut tx = storage.write().await.unwrap();
            UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
                &mut tx,
                proof,
                traversal_path,
                (i + 1) as u64,
            )
            .await
            .expect("failed to insert nodes");
            tx.commit().await.unwrap();
        }
    }

    #[async_std::test]
    async fn test_merklized_storage_with_commit() {
        // This test insert a merkle path into the database and queries the path using the merkle commitment
        setup_test();

        let db = TmpDb::init().await;
        let storage = SqlStorage::connect(db.config()).await.unwrap();

        // define a test tree
        let mut test_tree = MockMerkleTree::new(MockMerkleTree::tree_height());
        let block_height = 1;
        //insert an entry into the tree
        test_tree.update(0, 0).unwrap();
        let commitment = test_tree.commitment();

        let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(commitment).unwrap()});
        // insert the header with merkle commitment
        let mut tx = storage.write().await.unwrap();
        tx
                .query_opt(
                    "INSERT INTO HEADER(height, hash, payload_hash, timestamp, data) VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                    [
                        sql_param(&(block_height as i64)),
                        sql_param(&"randomString"),
                        sql_param(&test_data),
                    ],
                )
                .await
                .unwrap();
        // proof for the index from the tree
        let (_, proof) = test_tree.lookup(0).expect_ok().unwrap();
        // traversal path for the index.
        let traversal_path =
            <usize as ToTraversalPath<8>>::to_traversal_path(&0, test_tree.height());
        // insert merkle nodes
        UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
            &mut tx,
            proof.clone(),
            traversal_path.clone(),
            block_height as u64,
        )
        .await
        .expect("failed to insert nodes");
        // update saved state height
        UpdateStateData::<_, MockMerkleTree, 8>::set_last_state_height(&mut tx, block_height)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        let merkle_proof = storage
            .read()
            .await
            .unwrap()
            .get_path(Snapshot::<_, MockMerkleTree, 8>::Commit(commitment), 0)
            .await
            .unwrap();

        let (_, proof) = test_tree.lookup(0).expect_ok().unwrap();

        assert_eq!(merkle_proof, proof.clone(), "merkle paths mismatch");
    }
    #[async_std::test]
    async fn test_merklized_state_missing_state() {
        // This test checks that header commitment matches the root hash.
        // For this, the header merkle root commitment field is not updated, which should result in an error
        // The full merkle path verification is also done by recomputing the root hash
        // An index and its corresponding merkle nodes with created (bh) = 1 are inserted.
        // The entry of the index is updated, and the updated nodes are inserted with created (bh) = 2.
        // A node which is in the traversal path with bh = 2 is deleted, so the get_path should return an error as an older version of one of the nodes is used.
        setup_test();

        let db = TmpDb::init().await;
        let storage = SqlStorage::connect(db.config()).await.unwrap();

        // define a test tree
        let mut test_tree = MockMerkleTree::new(MockMerkleTree::tree_height());
        let block_height = 1;
        //insert an entry into the tree

        let mut tx = storage.write().await.unwrap();
        for i in 0..27 {
            test_tree.update(i, i).unwrap();
            let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()});
            // insert the header with merkle commitment
            tx
                    .query_opt(
                        "INSERT INTO HEADER(height, hash, payload_hash, timestamp, data) VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                        [
                            sql_param(&(block_height as i64)),
                            sql_param(&format!("randomString{i}")),
                            sql_param(&test_data),
                        ],
                    )
                    .await
                    .unwrap();
            // proof for the index from the tree
            let (_, proof) = test_tree.lookup(i).expect_ok().unwrap();
            // traversal path for the index.
            let traversal_path =
                <usize as ToTraversalPath<8>>::to_traversal_path(&i, test_tree.height());
            // insert merkle nodes
            UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
                &mut tx,
                proof.clone(),
                traversal_path.clone(),
                block_height as u64,
            )
            .await
            .expect("failed to insert nodes");
            // update saved state height
            UpdateStateData::<_, MockMerkleTree, 8>::set_last_state_height(&mut tx, block_height)
                .await
                .unwrap();
        }

        test_tree.update(1, 100).unwrap();
        //insert updated merkle path without updating the header
        let traversal_path =
            <usize as ToTraversalPath<8>>::to_traversal_path(&1, test_tree.height());
        let (_, proof) = test_tree.lookup(1).expect_ok().unwrap();

        // insert merkle nodes
        UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
            &mut tx,
            proof.clone(),
            traversal_path.clone(),
            block_height as u64,
        )
        .await
        .expect("failed to insert nodes");
        tx.commit().await.unwrap();

        let merkle_path = storage
            .read()
            .await
            .unwrap()
            .get_path(
                Snapshot::<_, MockMerkleTree, 8>::Index(block_height as u64),
                1,
            )
            .await;
        assert!(merkle_path.is_err());

        let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()});
        // insert the header with merkle commitment
        let tx = storage.write().await.unwrap();
        tx
                .query_opt(
                    "INSERT INTO HEADER(height, hash, payload_hash, timestamp, data) VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                    [
                        sql_param(&(block_height as i64)),
                        sql_param(&"randomStringgg"),
                        sql_param(&test_data),
                    ],
                )
                .await
                .unwrap();
        tx.commit().await.unwrap();
        // Querying the path again
        let merkle_proof = storage
            .read()
            .await
            .unwrap()
            .get_path(
                Snapshot::<_, MockMerkleTree, 8>::Index(block_height as u64),
                1,
            )
            .await
            .unwrap();
        assert_eq!(merkle_proof, proof, "path dont match");

        // Update the tree again for index 0 with created (bh) = 2
        // Delete one of the node in the traversal path
        test_tree.update(1, 200).unwrap();

        let (_, proof) = test_tree.lookup(1).expect_ok().unwrap();
        let test_data = serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()});

        // insert the header with merkle commitment
        let mut tx = storage.write().await.unwrap();
        tx
                .query_opt(
                    "INSERT INTO HEADER(height, hash, payload_hash, timestamp, data) VALUES ($1, $2, 't', 0, $3) ON CONFLICT(height) DO UPDATE set data = excluded.data",
                    [
                        sql_param(&(2_i64)),
                        sql_param(&"randomHashString"),
                        sql_param(&test_data),
                    ],
                )
                .await
                .unwrap();
        UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
            &mut tx,
            proof.clone(),
            traversal_path.clone(),
            2_u64,
        )
        .await
        .expect("failed to insert nodes");

        // Deleting one internal node
        let node_path = traversal_path
            .iter()
            .skip(1)
            .rev()
            .map(|n| *n as i32)
            .collect::<Vec<_>>();
        tx.execute_one(
            &format!(
                "DELETE FROM {} WHERE created = 2 and path = $1",
                MockMerkleTree::state_type()
            ),
            [sql_param(&node_path)],
        )
        .await
        .expect("failed to delete internal node");
        tx.commit().await.unwrap();

        let merkle_path = storage
            .read()
            .await
            .unwrap()
            .get_path(Snapshot::<_, MockMerkleTree, 8>::Index(2_u64), 1)
            .await;

        assert!(merkle_path.is_err());
    }

    #[async_std::test]
    async fn test_merklized_state_snapshot() {
        setup_test();

        let db = TmpDb::init().await;
        let storage = SqlStorage::connect(db.config()).await.unwrap();

        // Define a test tree
        let mut test_tree = MockMerkleTree::new(MockMerkleTree::tree_height());

        // We will sample random keys as u32. This is a value that is not a valid u32 and thus is a
        // key we will never insert into the tree.
        const RESERVED_KEY: usize = (u32::MAX as usize) + 1;

        // Randomly insert and delete some entries. For each entry we insert, we also keep track of
        // whether the entry should be in the tree using a HashMap.
        #[tracing::instrument(skip(tree, expected))]
        fn randomize(tree: &mut MockMerkleTree, expected: &mut HashMap<usize, Option<usize>>) {
            let mut rng = rand::thread_rng();
            tracing::info!("randomizing tree");

            for _ in 0..50 {
                // We flip a coin to decide whether to insert or delete, unless the tree is empty,
                // in which case we can only insert.
                if !expected.values().any(|v| v.is_some()) || rng.next_u32() % 2 == 0 {
                    // Insert.
                    let key = rng.next_u32() as usize;
                    let val = rng.next_u32() as usize;
                    tracing::info!(key, val, "inserting");

                    tree.update(key, val).unwrap();
                    expected.insert(key, Some(val));
                } else {
                    // Delete.
                    let key = expected
                        .iter()
                        .filter_map(|(k, v)| if v.is_some() { Some(k) } else { None })
                        .choose(&mut rng)
                        .unwrap();
                    tracing::info!(key, "deleting");

                    tree.remove(key).unwrap();
                    expected.insert(*key, None);
                }
            }
        }

        // Commit the tree to storage.
        #[tracing::instrument(skip(storage, tree, expected))]
        async fn store(
            storage: &SqlStorage,
            tree: &MockMerkleTree,
            expected: &HashMap<usize, Option<usize>>,
            block_height: u64,
        ) {
            tracing::info!("persisting tree");
            let mut tx = storage.write().await.unwrap();

            for key in expected.keys() {
                let proof = match tree.universal_lookup(key) {
                    LookupResult::Ok(_, proof) => proof,
                    LookupResult::NotFound(proof) => proof,
                    LookupResult::NotInMemory => panic!("failed to find key {key}"),
                };
                let traversal_path = ToTraversalPath::<8>::to_traversal_path(key, tree.height());
                UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
                    &mut tx,
                    proof,
                    traversal_path,
                    block_height,
                )
                .await
                .unwrap();
            }
            // insert the header with merkle commitment
            tx
            .upsert("header", ["height", "hash", "payload_hash", "timestamp", "data"], ["height"],
                [[
                    sql_param(&(block_height as i64)),
                    sql_param(&format!("hash{block_height}")),
                    sql_param(&"hash"),
                    sql_param(&0i64),
                    sql_param(&serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(tree.commitment()).unwrap()})),
                ]],
            )
            .await
            .unwrap();
            UpdateStateData::<MockTypes, MockMerkleTree, 8>::set_last_state_height(
                &mut tx,
                block_height as usize,
            )
            .await
            .unwrap();
            tx.commit().await.unwrap();
        }

        #[tracing::instrument(skip(storage, tree, expected))]
        async fn validate(
            storage: &SqlStorage,
            tree: &MockMerkleTree,
            expected: &HashMap<usize, Option<usize>>,
            block_height: u64,
        ) {
            tracing::info!("validating snapshot");

            // Check that we can get a correct path for each key that we touched.
            let snapshot = Snapshot::<_, MockMerkleTree, 8>::Index(block_height);

            for (key, val) in expected {
                let proof = match tree.universal_lookup(key) {
                    LookupResult::Ok(_, proof) => proof,
                    LookupResult::NotFound(proof) => proof,
                    LookupResult::NotInMemory => panic!("failed to find key {key}"),
                };
                assert_eq!(
                    proof,
                    storage
                        .read()
                        .await
                        .unwrap()
                        .get_path(snapshot, *key)
                        .await
                        .unwrap()
                );
                assert_eq!(val.as_ref(), proof.elem());
                // Check path is valid for test_tree
                if val.is_some() {
                    MockMerkleTree::verify(tree.commitment().digest(), key, proof)
                        .unwrap()
                        .unwrap();
                } else {
                    assert!(tree.non_membership_verify(key, proof).unwrap());
                }
            }

            // Check that we can even get a non-membership proof for a key that we never touched.
            let proof = match tree.universal_lookup(RESERVED_KEY) {
                LookupResult::Ok(_, proof) => proof,
                LookupResult::NotFound(proof) => proof,
                LookupResult::NotInMemory => panic!("failed to find reserved key {RESERVED_KEY}"),
            };
            assert_eq!(
                proof,
                storage
                    .read()
                    .await
                    .unwrap()
                    .get_path(snapshot, RESERVED_KEY)
                    .await
                    .unwrap()
            );
            assert_eq!(proof.elem(), None);
            // Check path is valid for test_tree
            assert!(tree.non_membership_verify(RESERVED_KEY, proof).unwrap());
        }

        // Create a randomized Merkle tree.
        let mut expected = HashMap::<usize, Option<usize>>::new();
        randomize(&mut test_tree, &mut expected);

        // Commit the randomized tree to storage.
        store(&storage, &test_tree, &expected, 1).await;
        validate(&storage, &test_tree, &expected, 1).await;

        // Make random edits and commit another snapshot.
        let mut expected2 = expected.clone();
        let mut test_tree2 = test_tree.clone();
        randomize(&mut test_tree2, &mut expected2);
        store(&storage, &test_tree2, &expected2, 2).await;
        validate(&storage, &test_tree2, &expected2, 2).await;

        // Ensure the original snapshot is still valid.
        validate(&storage, &test_tree, &expected, 1).await;
    }

    #[async_std::test]
    async fn test_merklized_state_missing_leaf() {
        // Check that if a leaf is missing but its ancestors are present/key is in the tree, we
        // catch it rather than interpreting the entry as an empty node by default. Note that this
        // scenario should be impossible in normal usage, since we never store or delete partial
        // paths. But we should never return an invalid proof even in extreme cases like database
        // corruption.
        setup_test();

        for tree_size in 1..=3 {
            let db = TmpDb::init().await;
            let storage = SqlStorage::connect(db.config()).await.unwrap();

            // Define a test tree
            let mut test_tree = MockMerkleTree::new(MockMerkleTree::tree_height());
            for i in 0..tree_size {
                test_tree.update(i, i).unwrap();
            }

            let mut tx = storage.write().await.unwrap();

            // Insert a header with the tree commitment.
            tx
                .query_opt(
                    "INSERT INTO HEADER(height, hash, payload_hash, timestamp, data) VALUES (0, 'hash', 'hash', 0, $1)",
                    [
                        sql_param(&serde_json::json!({ MockMerkleTree::header_state_commitment_field() : serde_json::to_value(test_tree.commitment()).unwrap()})),
                    ],
                )
                .await
                .unwrap();

            // Insert Merkle nodes.
            for i in 0..tree_size {
                let proof = test_tree.lookup(i).expect_ok().unwrap().1;
                let traversal_path =
                    ToTraversalPath::<8>::to_traversal_path(&i, test_tree.height());
                UpdateStateData::<_, MockMerkleTree, 8>::insert_merkle_nodes(
                    &mut tx,
                    proof,
                    traversal_path,
                    0,
                )
                .await
                .unwrap();
            }
            UpdateStateData::<_, MockMerkleTree, 8>::set_last_state_height(&mut tx, 0)
                .await
                .unwrap();
            tx.commit().await.unwrap();

            // Test that we can get all the entries.
            let snapshot = Snapshot::<MockTypes, MockMerkleTree, 8>::Index(0);
            for i in 0..tree_size {
                let proof = test_tree.lookup(i).expect_ok().unwrap().1;
                assert_eq!(
                    proof,
                    storage
                        .read()
                        .await
                        .unwrap()
                        .get_path(snapshot, i)
                        .await
                        .unwrap()
                );
                assert_eq!(*proof.elem().unwrap(), i);
            }

            // Now delete the leaf node for the last entry we inserted, corrupting the database.
            let index = serde_json::to_value(tree_size - 1).unwrap();
            let mut tx = storage.write().await.unwrap();
            tx.execute_one_with_retries(
                &format!(
                    "DELETE FROM {} WHERE index = $1",
                    MockMerkleTree::state_type()
                ),
                [index],
            )
            .await
            .unwrap();
            tx.commit().await.unwrap();

            // Test that we can still get the entries we didn't delete.
            for i in 0..tree_size - 1 {
                let proof = test_tree.lookup(i).expect_ok().unwrap().1;
                assert_eq!(
                    proof,
                    storage
                        .read()
                        .await
                        .unwrap()
                        .get_path(snapshot, i)
                        .await
                        .unwrap()
                );
                assert_eq!(*proof.elem().unwrap(), i);
            }

            // Looking up the entry we deleted fails, rather than return an invalid path.
            let err = storage
                .read()
                .await
                .unwrap()
                .get_path(snapshot, tree_size - 1)
                .await
                .unwrap_err();
            assert!(matches!(err, QueryError::Missing));
        }
    }
}
