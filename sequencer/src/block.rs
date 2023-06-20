use crate::{vm::Vm, Error, Transaction, VmId};
use commit::{Commitment, Committable};
use hotshot::traits::Block as HotShotBlock;
use hotshot_query_service::QueryableBlock;
use hotshot_types::traits::state::TestableBlock;
use jf_primitives::merkle_tree::{
    examples::{Sha3Digest, Sha3Node},
    namespaced_merkle_tree::NMT,
    AppendableMerkleTreeScheme, LookupResult, MerkleTreeScheme,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use typenum::U2;

type TransactionNMT = NMT<Transaction, Sha3Digest, U2, VmId, Sha3Node>;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Block {
    pub(crate) transaction_nmt: TransactionNMT,
}

// TODO(#345) implement
impl QueryableBlock for Block {
    type TransactionIndex = u64;
    type InclusionProof = ();
    type Iter<'a> = Box<dyn Iterator<Item = u64>>;

    fn len(&self) -> usize {
        self.transaction_nmt.num_leaves() as usize
    }

    fn transaction_with_proof(
        &self,
        _index: &Self::TransactionIndex,
    ) -> Option<(&Self::Transaction, Self::InclusionProof)> {
        unimplemented!()
    }

    fn transaction(&self, index: &Self::TransactionIndex) -> Option<&Self::Transaction> {
        match self.transaction_nmt.lookup(index) {
            LookupResult::Ok(val, _) => Some(val),
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
            transaction_nmt: TransactionNMT::from_elems(0, &[]).unwrap(),
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
            transaction_nmt: TransactionNMT::from_elems(0, &[]).unwrap(),
        }
    }

    /// Visit all transactions in this block.
    pub fn transactions(&self) -> impl ExactSizeIterator<Item = &Transaction> + '_ {
        self.transaction_nmt.leaves()
    }

    /// Visit the valid transactions for `V` in this block.
    pub fn vm_transactions<'a, V: Vm>(
        &'a self,
        vm: &'a V,
    ) -> impl Iterator<Item = V::Transaction> + 'a {
        self.transactions().filter_map(|txn| txn.as_vm(vm))
    }
}
