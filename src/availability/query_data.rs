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

use crate::{Header, Payload, Transaction};
use commit::{Commitment, Committable};
use hotshot_types::{
    data::Leaf,
    simple_certificate::QuorumCertificate,
    traits::{
        self,
        block_contents::{BlockHeader, BlockPayload},
        node_implementation::NodeType,
        signature_key::EncodedPublicKey,
    },
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::{ensure, Snafu};
use std::fmt::Debug;

pub type LeafHash<Types> = Commitment<Leaf<Types>>;
/// A block hash is the hash of the block header.
///
/// A block consists of a header and a payload. But the header itself contains a commitment to the
/// payload, so we can commit to the entire block simply by hashing the header.
pub type BlockHash<Types> = Commitment<Header<Types>>;
pub type TransactionHash<Types> = Commitment<Transaction<Types>>;
pub type TransactionIndex<Types> = <Payload<Types> as QueryablePayload>::TransactionIndex;
pub type TransactionInclusionProof<Types> = <Payload<Types> as QueryablePayload>::InclusionProof;

pub type Timestamp = time::OffsetDateTime;

/// A block payload whose contents (e.g. individual transactions) can be examined.
///
/// Note to implementors: this trait has only a few required methods. The provided methods, for
/// querying transactions in various ways, are implemented in terms of the required
/// [`iter`](Self::iter) and [`transaction_with_proof`](Self::transaction_with_proof) methods, and
/// the default implementations may be inefficient (e.g. performing an O(n) search, or computing an
/// unnecessary inclusion proof). It is good practice to override these default implementations if
/// your block type supports more efficient implementations (e.g. sublinear indexing by hash).
pub trait QueryablePayload: traits::BlockPayload {
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
#[snafu(display("QC references leaf {qc_leaf}, but expected {leaf}"))]
pub struct InconsistentLeafError<Types: NodeType> {
    pub leaf: LeafHash<Types>,
    pub qc_leaf: LeafHash<Types>,
}

impl<Types: NodeType> LeafQueryData<Types> {
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
            qc.data.leaf_commit == leaf.commit(),
            InconsistentLeafSnafu {
                leaf: leaf.commit(),
                qc_leaf: qc.data.leaf_commit
            }
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
        self.leaf.block_header.block_number()
    }

    pub fn hash(&self) -> LeafHash<Types> {
        self.leaf.commit()
    }

    pub fn block_hash(&self) -> BlockHash<Types> {
        self.leaf.block_header.commit()
    }

    pub fn proposer(&self) -> EncodedPublicKey {
        self.leaf.get_proposer_id()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct BlockQueryData<Types: NodeType> {
    pub(crate) header: Header<Types>,
    pub(crate) payload: Payload<Types>,
    pub(crate) hash: BlockHash<Types>,
    pub(crate) size: u64,
}

impl<Types: NodeType> BlockQueryData<Types> {
    /// Collect information about a [`Block`].
    ///
    /// Returns a new [`BlockQueryData`] object populated from `leaf`, `qc`, and `block`.
    ///
    /// # Errors
    ///
    /// Fails with an [`InconsistentLeafError`] if `qc` and `leaf` do not correspond to the same
    /// block. If `payload` does not correspond to the same block as `leaf`, the behavior is
    /// unspecified. In debug builds, the call may panic. However, this consistency check is quite
    /// expensive, and may be omitted in optimized builds. The responsibility of ensuring
    /// consistency between `leaf` and `payload` ultimately falls on the caller.
    pub fn new(
        leaf: Leaf<Types>,
        qc: QuorumCertificate<Types>,
        payload: Payload<Types>,
    ) -> Result<Self, InconsistentLeafError<Types>> {
        ensure!(
            qc.data.leaf_commit == leaf.commit(),
            InconsistentLeafSnafu {
                leaf: leaf.commit(),
                qc_leaf: qc.data.leaf_commit
            },
        );
        debug_assert!(
            leaf.block_header.payload_commitment()
                == hotshot_types::traits::block_contents::vid_commitment(
                    &payload.encode().unwrap().collect()
                )
        );
        Ok(Self {
            hash: leaf.block_header.commit(),
            header: leaf.block_header,
            size: payload_size::<Types>(&payload),
            payload,
        })
    }

    pub fn header(&self) -> &Header<Types> {
        &self.header
    }

    pub fn payload(&self) -> &Payload<Types> {
        &self.payload
    }

    pub fn hash(&self) -> BlockHash<Types> {
        self.hash
    }

    pub fn height(&self) -> u64 {
        self.header.block_number()
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn len(&self) -> usize
    where
        Payload<Types>: QueryablePayload,
    {
        self.payload.len()
    }

    pub fn is_empty(&self) -> bool
    where
        Payload<Types>: QueryablePayload,
    {
        self.len() == 0
    }

    pub fn transaction(&self, i: &TransactionIndex<Types>) -> Option<TransactionQueryData<Types>>
    where
        Payload<Types>: QueryablePayload,
    {
        let (transaction, proof) = self.payload.transaction_with_proof(i)?;
        Some(TransactionQueryData {
            transaction: transaction.clone(),
            block_hash: self.hash(),
            proof,
            height: self.height(),
            hash: transaction.commit(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct TransactionQueryData<Types: NodeType>
where
    Payload<Types>: QueryablePayload,
{
    transaction: Transaction<Types>,
    block_hash: BlockHash<Types>,
    proof: TransactionInclusionProof<Types>,
    height: u64,
    hash: TransactionHash<Types>,
}

impl<Types: NodeType> TransactionQueryData<Types>
where
    Payload<Types>: QueryablePayload,
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

pub(crate) fn payload_size<Types: NodeType>(payload: &Payload<Types>) -> u64 {
    match payload.encode() {
        Ok(iter) => iter.count() as u64,
        Err(_) => 0,
    }
}
