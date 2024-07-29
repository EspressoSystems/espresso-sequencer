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

//! SQL transactions
//!
//! A transaction encapsulates all the mutable functionality provided by the SQL database, and
//! allows for mutable operations to be combined into complex updates that affect the main database
//! atomically. A transaction also provides all the immutable query functionality of a regular
//! database connection, so that the updated state of the database can be queried midway through a
//! transaction.

use super::{
    postgres::{types::BorrowToSql, ToStatement},
    query::{
        sql_param,
        state::{HashTableRow, Node, ParseError},
    },
    Client, Query,
};
use crate::{
    availability::{
        BlockQueryData, LeafQueryData, QueryableHeader, QueryablePayload, UpdateAvailabilityData,
        VidCommonQueryData,
    },
    data_source::update,
    merklized_state::{MerklizedState, UpdateStateData},
    types::HeightIndexed,
    Header, Payload, QueryError, VidShare,
};
use anyhow::{bail, ensure, Context};
use ark_serialize::CanonicalSerialize;
use async_std::{sync::MutexGuard, task::sleep};
use async_trait::async_trait;
use bit_vec::BitVec;
use committable::Committable;
use derivative::Derivative;
use derive_more::From;
use futures::{
    future::Future,
    stream::{StreamExt, TryStreamExt},
};
use hotshot_types::traits::{
    block_contents::BlockHeader, node_implementation::NodeType, EncodeBytes,
};
use itertools::{izip, Itertools};
use jf_merkle_tree::prelude::{MerkleNode, MerkleProof};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    time::Duration,
};

/// An atomic SQL transaction.
#[derive(Derivative, From)]
#[derivative(Debug)]
pub struct Transaction<'a> {
    inner: MutexGuard<'a, Client>,
    committed: bool,
}

impl<'a> Transaction<'a> {
    pub(super) async fn new(inner: MutexGuard<'a, Client>) -> anyhow::Result<Self> {
        inner.batch_execute("BEGIN").await?;
        Ok(Self {
            inner,
            committed: false,
        })
    }
}

impl<'a> update::Transaction for Transaction<'a> {
    async fn commit(mut self) -> anyhow::Result<()> {
        self.inner.batch_execute("COMMIT").await?;
        self.committed = true;
        Ok(())
    }
    fn revert(self) -> impl Future + Send {
        async move {
            self.inner.batch_execute("ROLLBACK").await.unwrap();
        }
    }
}

impl<'a> Drop for Transaction<'a> {
    fn drop(&mut self) {
        if !self.committed {
            async_std::task::block_on(self.inner.batch_execute("ROLLBACK")).unwrap();
        }
    }
}

/// Low-level, general database mutation.
impl<'a> Transaction<'a> {
    /// Execute a statement against the underlying database.
    ///
    /// The results of the statement will be reflected immediately in future statements made within
    /// this transaction, but will not be reflected in the underlying database until the transaction
    /// is committed with [`commit`](update::Transaction::commit).
    pub async fn execute<T, P>(&mut self, statement: &T, params: P) -> anyhow::Result<u64>
    where
        T: ?Sized + ToStatement,
        P: IntoIterator,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        Ok(self.inner.execute_raw(statement, params).await?)
    }

    /// Execute a statement that is expected to modify exactly one row.
    ///
    /// Returns an error if the database is not modified.
    pub async fn execute_one<T, P>(&mut self, statement: &T, params: P) -> anyhow::Result<()>
    where
        T: ?Sized + ToStatement + Display,
        P: IntoIterator,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        let nrows = self.execute_many(statement, params).await?;
        if nrows > 1 {
            // If more than one row is affected, we don't return an error, because clearly
            // _something_ happened and modified the database. So we don't necessarily want the
            // caller to retry. But we do log an error, because it seems the query did something
            // different than the caller intended.
            tracing::error!(
                %statement,
                "statement modified more rows ({nrows}) than expected (1)"
            );
        }
        Ok(())
    }

    /// Execute a statement that is expected to modify exactly one row.
    ///
    /// Returns an error if the database is not modified. Retries several times before failing.
    pub async fn execute_one_with_retries<T, P>(
        &mut self,
        statement: &T,
        params: P,
    ) -> anyhow::Result<()>
    where
        T: ?Sized + ToStatement + Display,
        P: IntoIterator + Clone,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        let interval = Duration::from_secs(1);
        let mut retries = 5;

        while let Err(err) = self.execute_one(statement, params.clone()).await {
            tracing::error!(
                %statement,
                "error in statement execution ({retries} tries remaining): {err}"
            );
            if retries == 0 {
                return Err(err);
            }
            retries -= 1;
            sleep(interval).await;
        }

        Ok(())
    }

    /// Execute a statement that is expected to modify at least one row.
    ///
    /// Returns an error if the database is not modified.
    pub async fn execute_many<T, P>(&mut self, statement: &T, params: P) -> anyhow::Result<u64>
    where
        T: ?Sized + ToStatement + Display,
        P: IntoIterator,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        let nrows = self.execute(statement, params).await?;
        ensure!(
            nrows > 0,
            "statement failed: 0 rows affected. Statement: {statement}"
        );
        Ok(nrows)
    }

    /// Execute a statement that is expected to modify at least one row.
    ///
    /// Returns an error if the database is not modified. Retries several times before failing.
    pub async fn execute_many_with_retries<T, P>(
        &mut self,
        statement: &T,
        params: P,
    ) -> anyhow::Result<u64>
    where
        T: ?Sized + ToStatement + Display,
        P: IntoIterator + Clone,
        P::IntoIter: ExactSizeIterator,
        P::Item: BorrowToSql,
    {
        let interval = Duration::from_secs(1);
        let mut retries = 5;

        loop {
            match self.execute_many(statement, params.clone()).await {
                Ok(nrows) => return Ok(nrows),
                Err(err) => {
                    tracing::error!(
                        %statement,
                        "error in statement execution ({retries} tries remaining): {err}"
                    );
                    if retries == 0 {
                        return Err(err);
                    }
                    retries -= 1;
                    sleep(interval).await;
                }
            }
        }
    }

    pub async fn upsert<const N: usize, P>(
        &mut self,
        table: &str,
        columns: [&str; N],
        pk: impl IntoIterator<Item = &str>,
        rows: impl IntoIterator<Item = [P; N]>,
    ) -> anyhow::Result<()>
    where
        P: BorrowToSql + Clone,
    {
        let set_columns = columns
            .iter()
            .map(|col| format!("{col} = excluded.{col}"))
            .join(",");
        let columns = columns.into_iter().join(",");
        let pk = pk.into_iter().join(",");

        let mut values = vec![];
        let mut params = vec![];
        let mut num_rows = 0;
        for (row, entries) in rows.into_iter().enumerate() {
            let start = row * N;
            let end = (row + 1) * N;
            let row_params = (start..end).map(|i| format!("${}", i + 1)).join(",");

            values.push(format!("({row_params})"));
            params.extend(entries);
            num_rows += 1;
        }

        if num_rows == 0 {
            tracing::warn!("trying to upsert 0 rows, this has no effect");
            return Ok(());
        }
        tracing::debug!("upserting {num_rows} rows");

        let values = values.into_iter().join(",");
        let stmt = format!(
            "INSERT INTO {table} ({columns})
                  VALUES {values}
             ON CONFLICT ({pk}) DO UPDATE SET {set_columns}"
        );
        let rows_modified = self.execute_many_with_retries(&stmt, params).await?;
        if rows_modified != num_rows {
            tracing::error!(
                stmt,
                "unexpected number of rows modified: expected {num_rows} but got {rows_modified}"
            );
        }
        Ok(())
    }
}

/// Query service specific mutations.
impl<'a> Transaction<'a> {
    /// Delete a batch of data for pruning.
    pub(super) async fn delete_batch(&mut self, height: u64) -> anyhow::Result<()> {
        self.execute("DELETE FROM header WHERE height <= $1", &[&(height as i64)])
            .await?;
        self.save_pruned_height(height).await?;
        Ok(())
    }

    /// Record the height of the latest pruned header.
    pub(super) async fn save_pruned_height(&mut self, height: u64) -> anyhow::Result<()> {
        // id is set to 1 so that there is only one row in the table.
        // height is updated if the row already exists.
        self.upsert(
            "pruned_height",
            ["id", "last_height"],
            ["id"],
            [[sql_param(&(1_i32)), sql_param(&(height as i64))]],
        )
        .await
    }
}

#[async_trait]
impl<'a, Types> UpdateAvailabilityData<Types> for Transaction<'a>
where
    Types: NodeType,
    Payload<Types>: QueryablePayload<Types>,
    Header<Types>: QueryableHeader<Types>,
{
    async fn insert_leaf(&mut self, leaf: LeafQueryData<Types>) -> anyhow::Result<()> {
        // While we don't necessarily have the full block for this leaf yet, we can initialize the
        // header table with block metadata taken from the leaf.
        let header_json = serde_json::to_value(leaf.leaf().block_header())
            .context("failed to serialize header")?;
        self.upsert(
            "header",
            ["height", "hash", "payload_hash", "data", "timestamp"],
            ["height"],
            [[
                sql_param(&(leaf.height() as i64)),
                sql_param(&leaf.block_hash().to_string()),
                sql_param(&leaf.leaf().block_header().payload_commitment().to_string()),
                sql_param(&header_json),
                sql_param(&(leaf.leaf().block_header().timestamp() as i64)),
            ]],
        )
        .await?;

        // Similarly, we can initialize the payload table with a null payload, which can help us
        // distinguish between blocks that haven't been produced yet and blocks we haven't received
        // yet when answering queries.
        self.upsert("payload", ["height"], ["height"], [[leaf.height() as i64]])
            .await?;

        // Finally, we insert the leaf itself, which references the header row we created.
        // Serialize the full leaf and QC to JSON for easy storage.
        let leaf_json = serde_json::to_value(leaf.leaf()).context("failed to serialize leaf")?;
        let qc_json = serde_json::to_value(leaf.qc()).context("failed to serialize QC")?;
        self.upsert(
            "leaf",
            ["height", "hash", "block_hash", "leaf", "qc"],
            ["height"],
            [[
                sql_param(&(leaf.height() as i64)),
                sql_param(&leaf.hash().to_string()),
                sql_param(&leaf.block_hash().to_string()),
                sql_param(&leaf_json),
                sql_param(&qc_json),
            ]],
        )
        .await?;

        Ok(())
    }

    async fn insert_block(&mut self, block: BlockQueryData<Types>) -> anyhow::Result<()> {
        // The header and payload tables should already have been initialized when we inserted the
        // corresponding leaf. All we have to do is add the payload itself and its size.
        let payload = block.payload.encode();
        self.upsert(
            "payload",
            ["height", "data", "size"],
            ["height"],
            [[
                sql_param(&(block.height() as i64)),
                sql_param(&payload.as_ref()),
                sql_param(&(block.size() as i32)),
            ]],
        )
        .await?;

        // Index the transactions in the block. For each transaction, collect, separately, its hash,
        // height, and index. These items all have different types, so we collect them into
        // different vecs.
        let mut tx_hashes = vec![];
        let mut tx_block_heights = vec![];
        let mut tx_indexes = vec![];
        for (txn_ix, txn) in block.enumerate() {
            let txn_ix =
                serde_json::to_value(&txn_ix).context("failed to serialize transaction index")?;
            tx_hashes.push(txn.commit().to_string());
            tx_block_heights.push(block.height() as i64);
            tx_indexes.push(txn_ix);
        }
        if !tx_hashes.is_empty() {
            self.upsert(
                "transaction",
                ["hash", "block_height", "index"],
                ["block_height", "index"],
                // Now that we have the transaction hashes, block heights, and indexes collected in
                // memory, we can combine them all into a single vec using type erasure: all the
                // values get converted to `&dyn ToSql`. The references all borrow from one of
                // `tx_hashes`, `tx_block_heights`, or `tx_indexes`, which all outlive this function
                // call, so the lifetimes work out.
                izip!(
                    tx_hashes.iter().map(sql_param),
                    tx_block_heights.iter().map(sql_param),
                    tx_indexes.iter().map(sql_param),
                )
                .map(|(hash, height, index)| [hash, height, index]),
            )
            .await?;
        }

        Ok(())
    }

    async fn insert_vid(
        &mut self,
        common: VidCommonQueryData<Types>,
        share: Option<VidShare>,
    ) -> anyhow::Result<()> {
        let common_data =
            bincode::serialize(common.common()).context("failed to serialize VID common data")?;
        if let Some(share) = share {
            let share_data = bincode::serialize(&share).context("failed to serialize VID share")?;
            self.upsert(
                "vid",
                ["height", "common", "share"],
                ["height"],
                [[
                    sql_param(&(common.height() as i64)),
                    sql_param(&common_data),
                    sql_param(&share_data),
                ]],
            )
            .await
        } else {
            // Don't touch the `share` column at all if we don't have a share to insert. It's
            // possible that this column already exists, and we are just upserting the common data,
            // in which case we don't want to overwrite the share with NULL.
            self.upsert(
                "vid",
                ["height", "common"],
                ["height"],
                [[
                    sql_param(&(common.height() as i64)),
                    sql_param(&common_data),
                ]],
            )
            .await
        }
    }
}

#[async_trait]
impl<'a, Types: NodeType, State: MerklizedState<Types, ARITY>, const ARITY: usize>
    UpdateStateData<Types, State, ARITY> for Transaction<'a>
{
    async fn set_last_state_height(&mut self, height: usize) -> anyhow::Result<()> {
        self.upsert(
            "last_merklized_state_height",
            ["id", "height"],
            ["id"],
            [[sql_param(&(1_i32)), sql_param(&(height as i64))]],
        )
        .await?;

        Ok(())
    }

    async fn insert_merkle_nodes(
        &mut self,
        proof: MerkleProof<State::Entry, State::Key, State::T, ARITY>,
        traversal_path: Vec<usize>,
        block_number: u64,
    ) -> anyhow::Result<()> {
        let pos = proof.pos;
        let path = proof.proof;

        let name = State::state_type();
        let block_number = block_number as i64;

        let mut traversal_path = traversal_path.iter().map(|n| *n as i32);

        // All the nodes are collected here, They depend on the hash ids which are returned after
        // hashes are upserted in the db
        let mut nodes = Vec::new();
        let mut hashset = HashSet::new();

        for node in path.iter() {
            match node {
                MerkleNode::Empty => {
                    let index = serde_json::to_value(pos.clone()).map_err(ParseError::Serde)?;
                    // The node path represents the sequence of nodes from the root down to a specific node.
                    // Therefore, the traversal path needs to be reversed
                    // The root node path is an empty array.
                    let node_path = traversal_path.clone().rev().collect();
                    nodes.push((
                        Node {
                            path: node_path,
                            index: Some(index),
                            ..Default::default()
                        },
                        None,
                        [0_u8; 32].to_vec(),
                    ));
                    hashset.insert([0_u8; 32].to_vec());
                }
                MerkleNode::ForgettenSubtree { .. } => {
                    bail!("Node in the Merkle path contains a forgetten subtree");
                }
                MerkleNode::Leaf { value, pos, elem } => {
                    let mut leaf_commit = Vec::new();
                    // Serialize the leaf node hash value into a vector
                    value
                        .serialize_compressed(&mut leaf_commit)
                        .map_err(ParseError::Serialize)?;

                    let path = traversal_path.clone().rev().collect();

                    let index = serde_json::to_value(pos.clone()).map_err(ParseError::Serde)?;
                    let entry = serde_json::to_value(elem).map_err(ParseError::Serde)?;

                    nodes.push((
                        Node {
                            path,
                            index: Some(index),
                            entry: Some(entry),
                            ..Default::default()
                        },
                        None,
                        leaf_commit.clone(),
                    ));

                    hashset.insert(leaf_commit);
                }
                MerkleNode::Branch { value, children } => {
                    // Get hash
                    let mut branch_hash = Vec::new();
                    value
                        .serialize_compressed(&mut branch_hash)
                        .map_err(ParseError::Serialize)?;

                    // We only insert the non-empty children in the children field of the table
                    // BitVec is used to separate out Empty children positions
                    let mut children_bitvec = BitVec::new();
                    let mut children_values = Vec::new();
                    for child in children {
                        let child = child.as_ref();
                        match child {
                            MerkleNode::Empty => {
                                children_bitvec.push(false);
                            }
                            MerkleNode::Branch { value, .. }
                            | MerkleNode::Leaf { value, .. }
                            | MerkleNode::ForgettenSubtree { value } => {
                                let mut hash = Vec::new();
                                value
                                    .serialize_compressed(&mut hash)
                                    .map_err(ParseError::Serialize)?;

                                children_values.push(hash);
                                // Mark the entry as 1 in bitvec to indiciate a non-empty child
                                children_bitvec.push(true);
                            }
                        }
                    }

                    // insert internal node
                    let path = traversal_path.clone().rev().collect();
                    nodes.push((
                        Node {
                            path,
                            children: None,
                            children_bitvec: Some(children_bitvec),
                            ..Default::default()
                        },
                        Some(children_values.clone()),
                        branch_hash.clone(),
                    ));
                    hashset.insert(branch_hash);
                    hashset.extend(children_values);
                }
            }

            // advance the traversal path for the internal nodes at each iteration
            // The final node would be the Root node where this iterator is exhausted
            traversal_path.next();
        }
        // We build a hashset to avoid duplicate entries
        let hashes = hashset.into_iter().collect::<Vec<Vec<u8>>>();

        // insert all the hashes into database
        // It returns all the ids inserted in the order they were inserted
        // We use the hash ids to insert all the nodes
        let (params, batch_hash_insert_stmt) = HashTableRow::build_batch_insert(&hashes);

        // Batch insert all the hashes
        let nodes_hash_ids: HashMap<Vec<u8>, i32> = self
            .inner
            .query_raw(&batch_hash_insert_stmt, params)
            .await?
            .map_ok(|r| (r.get(1), r.get(0)))
            .try_collect()
            .await?;

        // Updates the node fields
        for (node, children, hash) in &mut nodes {
            node.created = block_number;
            node.hash_id = *nodes_hash_ids.get(&*hash).ok_or(QueryError::Error {
                message: "Missing node hash".to_string(),
            })?;

            if let Some(children) = children {
                let children_hashes = children
                    .iter()
                    .map(|c| nodes_hash_ids.get(c).copied())
                    .collect::<Option<Vec<i32>>>()
                    .ok_or(QueryError::Error {
                        message: "Missing child hash".to_string(),
                    })?;

                node.children = Some(children_hashes);
            }
        }
        let nodes = nodes.into_iter().map(|(n, _, _)| n).collect::<Vec<_>>();
        let (params, batch_stmt) = Node::build_batch_insert(name, &nodes);

        // Batch insert all the child hashes
        let rows_inserted = self.inner.query_raw(&batch_stmt, params).await?;

        if rows_inserted.count().await != path.len() {
            bail!("failed to insert all merkle nodes");
        }

        Ok(())
    }
}

/// Query the underlying SQL database.
///
/// The results will reflect the state after the statements thus far added to this transaction have
/// been applied, even though those effects have not been committed to the database yet.
#[async_trait]
impl<'a> Query for Transaction<'a> {
    async fn client(&self) -> &Client {
        &*self.inner
    }
}
