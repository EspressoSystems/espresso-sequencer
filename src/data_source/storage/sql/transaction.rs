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
    queries::{
        state::{build_hash_batch_insert, Node},
        DecodeError,
    },
    Database, Db,
};
use crate::{
    availability::{
        BlockQueryData, LeafQueryData, QueryableHeader, QueryablePayload, UpdateAvailabilityData,
        VidCommonQueryData,
    },
    data_source::{storage::pruning::PrunedHeightStorage, update},
    merklized_state::{MerklizedState, UpdateStateData},
    types::HeightIndexed,
    Header, Payload, QueryError, VidShare,
};
use anyhow::{bail, ensure, Context};
use ark_serialize::CanonicalSerialize;
use async_std::task::sleep;
use async_trait::async_trait;
use committable::Committable;
use derive_more::{Deref, DerefMut};
use futures::{future::Future, stream::TryStreamExt};
use hotshot_types::traits::{
    block_contents::BlockHeader, node_implementation::NodeType, EncodeBytes,
};
use itertools::Itertools;
use jf_merkle_tree::prelude::{MerkleNode, MerkleProof};
use sqlx::{pool::Pool, types::BitVec, Encode, Execute, FromRow, Type};
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    time::Duration,
};

pub use sqlx::Executor;

pub type Query<'q> = sqlx::query::Query<'q, Db, <Db as Database>::Arguments<'q>>;
pub type QueryAs<'q, T> = sqlx::query::QueryAs<'q, Db, T, <Db as Database>::Arguments<'q>>;

pub fn query(sql: &str) -> Query<'_> {
    sqlx::query(sql)
}

pub fn query_as<'q, T>(sql: &'q str) -> QueryAs<'q, T>
where
    T: for<'r> FromRow<'r, <Db as Database>::Row>,
{
    sqlx::query_as(sql)
}

/// Marker type indicating a transaction with read-write access to the database.
#[derive(Clone, Copy, Debug, Default)]
pub struct Write;

/// Marker type indicating a transaction with read-only access to the database.
#[derive(Clone, Copy, Debug, Default)]
pub struct Read;

/// Trait for marker types indicating what type of access a transaction has to the database.
pub trait TransactionMode: Send + Sync {
    fn begin(
        conn: &mut <Db as Database>::Connection,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl TransactionMode for Write {
    async fn begin(conn: &mut <Db as Database>::Connection) -> anyhow::Result<()> {
        conn.execute("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
            .await?;
        Ok(())
    }
}

impl TransactionMode for Read {
    async fn begin(conn: &mut <Db as Database>::Connection) -> anyhow::Result<()> {
        conn.execute("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE, READ ONLY, DEFERRABLE")
            .await?;
        Ok(())
    }
}

/// An atomic SQL transaction.
#[derive(Debug, Deref, DerefMut)]
pub struct Transaction<Mode> {
    #[deref]
    #[deref_mut]
    inner: sqlx::Transaction<'static, Db>,
    _mode: PhantomData<Mode>,
}

impl<Mode: TransactionMode> Transaction<Mode> {
    pub(super) async fn new(pool: &Pool<Db>) -> anyhow::Result<Self> {
        let mut tx = pool.begin().await?;
        Mode::begin(tx.as_mut()).await?;
        Ok(Self {
            inner: tx,
            _mode: Default::default(),
        })
    }
}

impl<Mode: TransactionMode> update::Transaction for Transaction<Mode> {
    async fn commit(self) -> anyhow::Result<()> {
        self.inner.commit().await?;
        Ok(())
    }
    fn revert(self) -> impl Future + Send {
        async move {
            self.inner.rollback().await.unwrap();
        }
    }
}

/// A collection of parameters which can be bound to a SQL query.
///
/// This trait allows us to carry around hetergenous lists of parameters (e.g. tuples) and bind them
/// to a query at the last moment before executing. This means we can manipulate the parameters
/// independently of the query before executing it. For example, by requiring a trait bound of
/// `Params<'p> + Clone`, we get a list (or tuple) of parameters which can be cloned and then bound
/// to a query, which allows us to keep a copy of the parameters around in order to retry the query
/// if it fails.
///
/// # Lifetimes
///
/// A SQL [`Query`] with lifetime `'q` borrows from both it's SQL statement (`&'q str`) and its
/// parameters (bound via `bind<'q>`). Sometimes, though, it is necessary for the statement and its
/// parameters to have different (but overlapping) lifetimes. For example, the parameters might be
/// passed in and owned by the caller, while the query string is constructed in the callee and its
/// lifetime is limited to the callee scope. (See for example the [`upsert`](Transaction::upsert)
/// function which does exactly this.)
///
/// We could rectify this situation with a trait bound like `P: for<'q> Params<'q>`, meaning `P`
/// must be bindable to a query with a lifetime chosen by the callee. However, when `P` is an
/// associated type, such as an element of an iterator, as in
/// `<I as IntoIter>::Item: for<'q> Params<'q>`, [a current limitation](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html#implied-static-requirement-from-higher-ranked-trait-bounds.)
/// in the Rust compiler then requires `P: 'static`, which we don't necessarily want: the caller
/// should be able to pass in a reference to avoid expensive cloning.
///
/// So, instead, we work around this by making it explicit in the [`Params`] trait that the lifetime
/// of the query we're binding to (`'q`) may be different than the lifetime of the parameters (`'p`)
/// as long as the parameters outlive the duration of the query (the `'p: 'q`) bound on the
/// [`bind`](Self::bind) function.
pub trait Params<'p> {
    fn bind<'q>(self, q: Query<'q>) -> Query<'q>
    where
        'p: 'q;
}

/// A collection of parameters with a statically known length.
///
/// This is a simple trick for enforcing at compile time that a list of parameters has a certain
/// length, such as matching the length of a list of column names. This can prevent easy mistakes
/// like leaving out a parameter. It is implemented for tuples up to length 8.
pub trait FixedLengthParams<'p, const N: usize>: Params<'p> {}

macro_rules! impl_tuple_params {
    ($n:literal, ($($t:ident,)+)) => {
        impl<'p, $($t),+> Params<'p> for ($($t,)+)
        where $(
            $t: 'p + for<'q> Encode<'q, Db> + Type<Db>
        ),+ {
            fn bind<'q>(self, q: Query<'q>) -> Query<'q>
            where
                'p: 'q
            {
                #[allow(non_snake_case)]
                let ($($t,)+) = self;
                q $(
                    .bind($t)
                )+
            }
        }

        impl<'p, $($t),+> FixedLengthParams<'p, $n> for ($($t,)+)
        where $(
            $t: 'p + for<'q> Encode<'q, Db> + Type<Db>
        ),+ {
        }
    };
}

impl_tuple_params!(1, (T,));
impl_tuple_params!(2, (T1, T2,));
impl_tuple_params!(3, (T1, T2, T3,));
impl_tuple_params!(4, (T1, T2, T3, T4,));
impl_tuple_params!(5, (T1, T2, T3, T4, T5,));
impl_tuple_params!(6, (T1, T2, T3, T4, T5, T6,));
impl_tuple_params!(7, (T1, T2, T3, T4, T5, T6, T7,));
impl_tuple_params!(8, (T1, T2, T3, T4, T5, T6, T7, T8,));

impl<'p, T> Params<'p> for Vec<T>
where
    T: Params<'p>,
{
    fn bind<'q>(self, mut q: Query<'q>) -> Query<'q>
    where
        'p: 'q,
    {
        for params in self {
            q = params.bind(q);
        }
        q
    }
}

/// Low-level, general database queries and mutation.
impl Transaction<Write> {
    /// Execute a statement that is expected to modify exactly one row.
    ///
    /// Returns an error if the database is not modified.
    pub async fn execute_one<'q, E>(&mut self, statement: E) -> anyhow::Result<()>
    where
        E: 'q + Execute<'q, Db>,
    {
        let nrows = self.execute_many(statement).await?;
        if nrows > 1 {
            // If more than one row is affected, we don't return an error, because clearly
            // _something_ happened and modified the database. So we don't necessarily want the
            // caller to retry. But we do log an error, because it seems the query did something
            // different than the caller intended.
            tracing::error!("statement modified more rows ({nrows}) than expected (1)");
        }
        Ok(())
    }

    /// Execute a statement that is expected to modify exactly one row.
    ///
    /// Returns an error if the database is not modified. Retries several times before failing.
    pub async fn execute_one_with_retries<'q>(
        &mut self,
        statement: &'q str,
        params: impl Params<'q> + Clone,
    ) -> anyhow::Result<()> {
        let interval = Duration::from_secs(1);
        let mut retries = 5;

        while let Err(err) = self
            .execute_one(params.clone().bind(query(statement)))
            .await
        {
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
    pub async fn execute_many<'q, E>(&mut self, statement: E) -> anyhow::Result<u64>
    where
        E: 'q + Execute<'q, Db>,
    {
        let nrows = self.execute(statement).await?.rows_affected();
        ensure!(nrows > 0, "statement failed: 0 rows affected");
        Ok(nrows)
    }

    /// Execute a statement that is expected to modify at least one row.
    ///
    /// Returns an error if the database is not modified. Retries several times before failing.
    pub async fn execute_many_with_retries<'q, 'p>(
        &mut self,
        statement: &'q str,
        params: impl Params<'p> + Clone,
    ) -> anyhow::Result<u64>
    where
        'p: 'q,
    {
        let interval = Duration::from_secs(1);
        let mut retries = 5;

        loop {
            match self
                .execute_many(params.clone().bind(query(statement)))
                .await
            {
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

    pub async fn upsert<'p, const N: usize, R>(
        &mut self,
        table: &str,
        columns: [&str; N],
        pk: impl IntoIterator<Item = &str>,
        rows: R,
    ) -> anyhow::Result<()>
    where
        R: IntoIterator,
        R::Item: 'p + FixedLengthParams<'p, N> + Clone,
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
            params.push(entries);
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
impl Transaction<Write> {
    /// Delete a batch of data for pruning.
    pub(super) async fn delete_batch(&mut self, height: u64) -> anyhow::Result<()> {
        self.execute(query("DELETE FROM header WHERE height <= $1").bind(height as i64))
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
            [(1i32, height as i64)],
        )
        .await
    }
}

#[async_trait]
impl<Types> UpdateAvailabilityData<Types> for Transaction<Write>
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
            [(
                leaf.height() as i64,
                leaf.block_hash().to_string(),
                leaf.leaf().block_header().payload_commitment().to_string(),
                header_json,
                leaf.leaf().block_header().timestamp() as i64,
            )],
        )
        .await?;

        // Similarly, we can initialize the payload table with a null payload, which can help us
        // distinguish between blocks that haven't been produced yet and blocks we haven't received
        // yet when answering queries.
        self.upsert("payload", ["height"], ["height"], [(leaf.height() as i64,)])
            .await?;

        // Finally, we insert the leaf itself, which references the header row we created.
        // Serialize the full leaf and QC to JSON for easy storage.
        let leaf_json = serde_json::to_value(leaf.leaf()).context("failed to serialize leaf")?;
        let qc_json = serde_json::to_value(leaf.qc()).context("failed to serialize QC")?;
        self.upsert(
            "leaf",
            ["height", "hash", "block_hash", "leaf", "qc"],
            ["height"],
            [(
                leaf.height() as i64,
                leaf.hash().to_string(),
                leaf.block_hash().to_string(),
                leaf_json,
                qc_json,
            )],
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
            [(block.height() as i64, payload.as_ref(), block.size() as i32)],
        )
        .await?;

        // Index the transactions in the block.
        let mut rows = vec![];
        for (txn_ix, txn) in block.enumerate() {
            let txn_ix =
                serde_json::to_value(&txn_ix).context("failed to serialize transaction index")?;
            rows.push((txn.commit().to_string(), block.height() as i64, txn_ix));
        }
        if !rows.is_empty() {
            self.upsert(
                "transaction",
                ["hash", "block_height", "index"],
                ["block_height", "index"],
                rows,
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
                [(common.height() as i64, common_data, share_data)],
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
                [(common.height() as i64, common_data)],
            )
            .await
        }
    }
}

#[async_trait]
impl<Types: NodeType, State: MerklizedState<Types, ARITY>, const ARITY: usize>
    UpdateStateData<Types, State, ARITY> for Transaction<Write>
{
    async fn set_last_state_height(&mut self, height: usize) -> anyhow::Result<()> {
        self.upsert(
            "last_merklized_state_height",
            ["id", "height"],
            ["id"],
            [(1i32, height as i64)],
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
                    let index = serde_json::to_value(pos.clone())
                        .decode_error("malformed merkle position")?;
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
                        .decode_error("malformed merkle leaf commitment")?;

                    let path = traversal_path.clone().rev().collect();

                    let index = serde_json::to_value(pos.clone())
                        .decode_error("malformed merkle position")?;
                    let entry =
                        serde_json::to_value(elem).decode_error("malformed merkle element")?;

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
                        .decode_error("malformed merkle branch hash")?;

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
                                    .decode_error("malformed merkle node hash")?;

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
        let (query, sql) = build_hash_batch_insert(&hashes)?;
        let nodes_hash_ids: HashMap<Vec<u8>, i32> = query
            .query_as(&sql)
            .fetch(self.as_mut())
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
        Node::upsert(name, nodes.into_iter().map(|(n, _, _)| n), self).await?;
        Ok(())
    }
}

#[async_trait]
impl<Mode: TransactionMode> PrunedHeightStorage for Transaction<Mode> {
    async fn load_pruned_height(&mut self) -> anyhow::Result<Option<u64>> {
        let Some((height,)) =
            query_as::<(i64,)>("SELECT last_height FROM pruned_height ORDER BY id DESC LIMIT 1")
                .fetch_optional(self.as_mut())
                .await?
        else {
            return Ok(None);
        };
        Ok(Some(height as u64))
    }
}
