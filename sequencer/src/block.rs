use crate::{vm::Vm, Error, Transaction, TransactionNMT, MAX_NMT_DEPTH};
use commit::{Commitment, Committable};
use hotshot::traits::Block as HotShotBlock;
use hotshot_query_service::QueryableBlock;
use hotshot_types::traits::state::TestableBlock;
use jf_primitives::merkle_tree::{
    namespaced_merkle_tree::NamespacedMerkleTreeScheme, AppendableMerkleTreeScheme, LookupResult,
    MerkleCommitment, MerkleTreeScheme,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display};

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Block {
    #[serde(with = "nmt_serializer")]
    pub(crate) transaction_nmt: TransactionNMT,
}

mod nmt_serializer {
    use super::*;

    // Serialize the NMT as a compact Vec<Transaction>
    pub fn serialize<S>(nmt: &TransactionNMT, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let leaves = nmt.leaves().cloned().collect::<Vec<Transaction>>();
        leaves.serialize(s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<TransactionNMT, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de;

        let leaves = <Vec<Transaction>>::deserialize(deserializer)?;
        let nmt = TransactionNMT::from_elems(MAX_NMT_DEPTH, leaves)
            .map_err(|_| de::Error::custom("Failed to build NMT from serialized leaves"))?;
        Ok(nmt)
    }
}

impl QueryableBlock for Block {
    type TransactionIndex = u64;
    type InclusionProof = <TransactionNMT as MerkleTreeScheme>::MembershipProof;
    type Iter<'a> = Box<dyn Iterator<Item = u64>>;

    fn len(&self) -> usize {
        self.transaction_nmt.num_leaves() as usize
    }

    fn transaction_with_proof(
        &self,
        index: &Self::TransactionIndex,
    ) -> Option<(&Self::Transaction, Self::InclusionProof)> {
        match self.transaction_nmt.lookup(index) {
            LookupResult::Ok(txn, proof) => Some((txn, proof)),
            _ => None,
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        Box::new(0..self.len() as u64)
    }
}

impl HotShotBlock for Block {
    type Error = Error;

    type Transaction = Transaction;

    fn add_transaction_raw(
        &self,
        tx: &Self::Transaction,
    ) -> std::result::Result<Self, Self::Error> {
        let mut new = self.clone();
        new.transaction_nmt
            .push(tx.clone())
            .map_err(|e| Error::MerkleTreeError {
                error: e.to_string(),
            })?;
        Ok(new)
    }

    fn contained_transactions(&self) -> std::collections::HashSet<Commitment<Self::Transaction>> {
        self.transaction_nmt
            .leaves()
            .map(|tx| tx.commit())
            .collect()
    }

    fn new() -> Self {
        Self {
            transaction_nmt: TransactionNMT::from_elems(MAX_NMT_DEPTH, &[]).unwrap(),
        }
    }
}

#[cfg(any(test, feature = "testing"))]
impl TestableBlock for Block {
    fn genesis() -> Self {
        Block::genesis()
    }

    fn txn_count(&self) -> u64 {
        self.transaction_nmt.num_leaves()
    }
}

// Required for TestableBlock
#[cfg(any(test, feature = "testing"))]
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Committable for Block {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("Block Comm")
            .array_field(
                "txns",
                &self
                    .transaction_nmt
                    .leaves()
                    .map(|x| x.commit())
                    .collect::<Vec<_>>(),
            )
            .finalize()
    }
}

impl Block {
    pub fn genesis() -> Self {
        Self {
            transaction_nmt: TransactionNMT::from_elems(MAX_NMT_DEPTH, &[]).unwrap(),
        }
    }

    /// Visit all transactions in this block.
    pub fn transactions(&self) -> impl ExactSizeIterator<Item = &Transaction> + '_ {
        self.transaction_nmt.leaves()
    }

    /// Return namespace proof for a `V`, which can be used to extract the transactions for `V` in this block
    /// and the root of the NMT
    pub fn get_namespace_proof<V: Vm>(
        &self,
        vm: &V,
    ) -> (
        <TransactionNMT as MerkleTreeScheme>::NodeValue,
        <TransactionNMT as NamespacedMerkleTreeScheme>::NamespaceProof,
    ) {
        (
            self.transaction_nmt.commitment().digest(),
            self.transaction_nmt.get_namespace_proof(vm.id()),
        )
    }
}
