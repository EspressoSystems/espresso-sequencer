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

use crate::{Block, Leaf, Resolvable, Transaction};
use bincode::Options;
use commit::{Commitment, Committable};
use hotshot_types::{
    simple_certificate::QuorumCertificate,
    traits::{
        self,
        node_implementation::{NodeImplementation, NodeType},
        signature_key::EncodedPublicKey,
    },
};
use hotshot_utils::bincode::bincode_opts;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::{ensure, Snafu};
use std::fmt::Debug;

pub type LeafHash<Types> = Commitment<Leaf<Types>>;
pub type BlockHash<Types> = Commitment<Block<Types>>;
pub type TransactionHash<Types> = Commitment<Transaction<Types>>;
pub type TransactionIndex<Types> = <Block<Types> as QueryableBlock>::TransactionIndex;
pub type TransactionInclusionProof<Types> = <Block<Types> as QueryableBlock>::InclusionProof;

pub type Timestamp = time::OffsetDateTime;

/// A block whose contents (e.g. individual transactions) can be examined.
///
/// Note to implementors: this trait has only a few required methods. The provided methods, for
/// querying transactions in various ways, are implemented in terms of the required
/// [`iter`](Self::iter) and [`transaction_with_proof`](Self::transaction_with_proof) methods, and
/// the default implementations may be inefficient (e.g. performing an O(n) search, or computing an
/// unnecessary inclusion proof). It is good practice to override these default implementations if
/// your block type supports more efficient implementations (e.g. sublinear indexing by hash).
pub trait QueryableBlock: traits::BlockPayload {
    /// An index which can be used to efficiently retrieve a transaction for the block.
    ///
    /// This is left abstract so that different block implementations can index transactions
    /// internally however they want (e.g. by position or by hash). Meanwhile, many high-level
    /// functions for querying transactions by different means can be implemented by returning a
    /// `TransactionIndex` and then finally using it to retrieve the desired transaction.
    type TransactionIndex: Clone
        + Debug
        + PartialEq
        + Eq
        + Ord
        + Serialize
        + DeserializeOwned
        + Send
        + Sync;

    /// Enumerate the transactions in this block.
    type Iter<'a>: Iterator<Item = Self::TransactionIndex>
    where
        Self: 'a;

    /// A proof that a certain transaction exists in the block.
    ///
    /// The proof system and the statement which is proved will vary by application, with different
    /// applications proving stronger or weaker statements depending on the trust assumptions at
    /// play. Some may prove a very strong statement (for example, a shared sequencer proving that
    /// the transaction belongs not only to the block but to a section of the block dedicated to a
    /// specific rollup), otherws may prove something substantially weaker (for example, a trusted
    /// query service may use `()` for the proof).
    type InclusionProof: Clone + Debug + PartialEq + Eq + Serialize + DeserializeOwned;

    /// The number of transactions in the block.
    fn len(&self) -> usize;

    /// Whether this block is empty of transactions.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// List the transaction indices in the block.
    fn iter(&self) -> Self::Iter<'_>;

    /// Enumerate the transactions in the block with their indices.
    fn enumerate(
        &self,
    ) -> Box<dyn '_ + Iterator<Item = (Self::TransactionIndex, Self::Transaction)>> {
        Box::new(self.iter().map(|ix| {
            // `self.transaction` should always return `Some` if we are using an index which was
            // yielded by `self.iter`.
            let tx = self.transaction(&ix).unwrap();
            (ix, tx)
        }))
    }

    /// Get a transaction by its block-specific index, along with an inclusion proof.
    fn transaction_with_proof(
        &self,
        index: &Self::TransactionIndex,
    ) -> Option<(Self::Transaction, Self::InclusionProof)>;

    /// Get a transaction by its block-specific index.
    fn transaction(&self, index: &Self::TransactionIndex) -> Option<Self::Transaction> {
        Some(self.transaction_with_proof(index)?.0)
    }

    /// Get an inclusion proof for a transaction with a given index.
    fn proof(&self, index: &Self::TransactionIndex) -> Option<Self::InclusionProof> {
        Some(self.transaction_with_proof(index)?.1)
    }

    /// Get the index of the `nth` transaction.
    fn nth(&self, n: usize) -> Option<Self::TransactionIndex> {
        self.iter().nth(n)
    }

    /// Get the `nth` transaction.
    fn nth_transaction(&self, n: usize) -> Option<Self::Transaction> {
        self.transaction(&self.nth(n)?)
    }

    /// Get the `nth` transaction, along with an inclusion proof.
    fn nth_transaction_with_proof(
        &self,
        n: usize,
    ) -> Option<(Self::Transaction, Self::InclusionProof)> {
        self.transaction_with_proof(&self.nth(n)?)
    }

    /// Get the index of the transaction with a given hash, if it is in the block.
    fn by_hash(&self, hash: Commitment<Self::Transaction>) -> Option<Self::TransactionIndex> {
        self.iter().find(|i| {
            if let Some(tx) = self.transaction(i) {
                tx.commit() == hash
            } else {
                false
            }
        })
    }

    /// Get the transaction with a given hash, if it is in the block.
    fn transaction_by_hash(
        &self,
        hash: Commitment<Self::Transaction>,
    ) -> Option<Self::Transaction> {
        self.transaction(&self.by_hash(hash)?)
    }

    /// Get the transaction with a given hash, if it is in the block, along with an inclusion proof.
    fn transaction_by_hash_with_proof(
        &self,
        hash: Commitment<Self::Transaction>,
    ) -> Option<(Self::Transaction, Self::InclusionProof)> {
        self.transaction_with_proof(&self.by_hash(hash)?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct LeafQueryData<Types: NodeType> {
    pub(crate) leaf: Leaf<Types>,
    pub(crate) qc: QuorumCertificate<Types>,
}

#[derive(Clone, Debug, Snafu)]
#[snafu(display("QC references leaf {}, but expected {}", qc.leaf_commitment(), leaf.commit()))]
pub struct InconsistentLeafError<Types: NodeType> {
    pub leaf: Leaf<Types>,
    pub qc: QuorumCertificate<Types>,
}

impl<Types: NodeType, I: NodeImplementation<Types>> LeafQueryData<Types> {
    /// Collect information about a [`Leaf`].
    ///
    /// Returns a new [`LeafQueryData`] object populated from `leaf` and `qc`.
    ///
    /// # Errors
    ///
    /// Fails with an [`InconsistentLeafError`] if `qc` does not reference `leaf`.
    pub fn new(
        leaf: Leaf<Types>,
        qc: QuorumCertificate<Types>,
    ) -> Result<Self, InconsistentLeafError<Types>> {
        ensure!(
            qc.leaf_commitment() == leaf.commit(),
            InconsistentLeafSnafu { leaf, qc }
        );
        Ok(Self { leaf, qc })
    }

    pub fn leaf(&self) -> &Leaf<Types> {
        &self.leaf
    }

    pub fn qc(&self) -> &QuorumCertificate<Types> {
        &self.qc
    }

    pub fn height(&self) -> u64 {
        leaf_height(&self.leaf)
    }

    pub fn timestamp(&self) -> Timestamp {
        parse_timestamp(self.leaf.get_timestamp())
    }

    pub fn hash(&self) -> LeafHash<Types> {
        self.leaf.commit()
    }

    pub fn block_hash(&self) -> BlockHash<Types> {
        self.leaf.get_deltas().commitment()
    }

    pub fn proposer(&self) -> EncodedPublicKey {
        self.leaf.get_proposer_id()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct BlockQueryData<Types: NodeType>
where
    Block<Types>: QueryableBlock,
{
    pub(crate) block: Block<Types>,
    pub(crate) hash: BlockHash<Types>,
    pub(crate) height: u64,
    pub(crate) timestamp: i128,
    pub(crate) size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct BlockHeaderQueryData<Types: NodeType> {
    hash: BlockHash<Types>,
    height: u64,
    size: u64,
}

#[derive(Clone, Debug, Snafu)]
pub enum InconsistentBlockError<Types: NodeType>
where
    Block<Types>: Serialize,
{
    #[snafu(display("QC references leaf {}, but expected {}", qc.leaf_commitment(), leaf.commit()))]
    InconsistentQc {
        qc: QuorumCertificate<Types>,
        leaf: Leaf<Types>,
    },
    #[snafu(display("Leaf {} references block {}, but expected {}",
        leaf.commit(), block.commit(), leaf.get_deltas().commitment()))]
    InconsistentBlock {
        leaf: Leaf<Types>,
        block: Block<Types>,
    },
}

impl<Types: NodeType> BlockQueryData<Types>
where
    Block<Types>: QueryableBlock,
{
    /// Collect information about a [`Block`].
    ///
    /// Returns a new [`BlockQueryData`] object populated from `leaf`, `qc`, and `block`.
    ///
    /// # Errors
    ///
    /// Fails with an [`InconsistentBlockError`] if `qc`, `leaf`, and `block` do not all correspond
    /// to the same block.
    pub fn new<I: NodeImplementation<Types>>(
        leaf: Leaf<Types>,
        qc: QuorumCertificate<Types>,
        block: Block<Types>,
    ) -> Result<Self, InconsistentBlockError<Types>> {
        ensure!(
            qc.leaf_commitment() == leaf.commit(),
            InconsistentQcSnafu { qc, leaf }
        );
        ensure!(
            leaf.get_deltas().commitment() == block.commit(),
            InconsistentBlockSnafu { leaf, block }
        );
        Ok(Self {
            hash: block.commit(),
            height: leaf_height(&leaf),
            timestamp: round_timestamp(leaf.get_timestamp()),
            size: bincode_opts().serialized_size(&block).unwrap_or_default(),
            block,
        })
    }

    pub fn block(&self) -> &Types::BlockType {
        &self.block
    }

    pub fn hash(&self) -> BlockHash<Types> {
        self.hash
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn timestamp(&self) -> Timestamp {
        parse_timestamp(self.timestamp)
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn len(&self) -> usize {
        self.block().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn header(&self) -> BlockHeaderQueryData<Types> {
        BlockHeaderQueryData {
            hash: self.hash,
            height: self.height,
            size: self.size,
        }
    }

    pub fn transaction(&self, i: &TransactionIndex<Types>) -> Option<TransactionQueryData<Types>> {
        let (transaction, proof) = self.block.transaction_with_proof(i)?;
        Some(TransactionQueryData {
            transaction: transaction.clone(),
            block_hash: self.hash(),
            proof,
            height: self.height(),
            hash: transaction.commit(),
        })
    }
}

impl<Types: NodeType> BlockHeaderQueryData<Types> {
    pub fn hash(&self) -> BlockHash<Types> {
        self.hash
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct TransactionQueryData<Types: NodeType>
where
    Block<Types>: QueryableBlock,
{
    transaction: Transaction<Types>,
    block_hash: BlockHash<Types>,
    proof: TransactionInclusionProof<Types>,
    height: u64,
    hash: TransactionHash<Types>,
}

impl<Types: NodeType> TransactionQueryData<Types>
where
    Block<Types>: QueryableBlock,
{
    pub fn transaction(&self) -> &Transaction<Types> {
        &self.transaction
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn hash(&self) -> TransactionHash<Types> {
        self.hash
    }

    pub fn block_hash(&self) -> BlockHash<Types> {
        self.block_hash
    }
}

fn parse_timestamp(ns: i128) -> Timestamp {
    Timestamp::from_unix_timestamp_nanos(ns).expect("HotShot timestamp out of range")
}

fn round_timestamp(ns: i128) -> i128 {
    // HotShot gives us the timestamp with nanosecond precision, which is far more than necessary
    // and can't be stored accurately in Postgres. Round down to microsecond precision.
    (ns / 1000) * 1000
}

fn leaf_height<T: NodeType>(leaf: &Leaf<T>) -> u64 {
    // HotShot generates a genesis leaf with height 0, but we don't see it in the event stream.
    // Therefore, the first leaf we see has height 1. But to clients, the first leaf should have
    // height 0, since there is nothing before it, so we adjust the height here.
    leaf.get_height() - 1
}
